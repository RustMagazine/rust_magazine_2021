---
pub_date: Sat, 30 Jan 2021 16:00:00 GMT
description: think about io_uring and Rust

---

# 关于 io_uring 与 Rust 的思考

作者：王徐旸 

---

io_uring 是 Linux 5.x 时代加入的一套全新的异步机制，被钦定为 Linux 异步的未来。

本文将探讨在 Rust 中安全封装 io_uring 的一系列设计问题，并提出一些可能的解决方案。

## io_uring 的工作方式

io_uring 分为两个队列，提交队列 SQ (Submission Queue) 和完成队列 CQ (Completion Queue)。提交队列存放正在等待执行的异步任务，完成队列存放完成事件。

io_uring 的结构由内核分配，用户态通过 mmap 拿到相关结构的内存访问权限，这样就能让内核态与用户态共享内存，绕过系统调用双向传递数据。

概念工作流程具有三个阶段

1. 准备：应用程序获取一些提交队列项 SQE (Submission Queue Entry)，将每个异步任务分别设置到每个 SQE 中，用操作码、参数初始化。
2. 提交：应用程序向 SQ 中推入一些需要提交的 SQE，通过一次系统调用告诉内核有新的任务，或者让内核不停轮询来获取任务。
3. 收割：应用程序从 CQ 中取得一些完成队列事件 CQE (Completion Queue Event)，通过 user_data 识别并唤醒应用程序中的线程/协程，传递返回值。

epoll 是 Reactor 模型的实现，而 io_uring 是 Proactor 模型的实现。

这意味着基于 epoll 设计的程序难以直接迁移到 io_uring。

**问题 1**: 改变异步模型并不是一件容易的事，除非以部分性能为代价抹平差异。

**问题 2**: io_uring 需要较高版本的内核，现阶段，应用程序不得不考虑在没有 io_uring 高版本特性时要怎么回退 (fallback)。

## io_uring 的约束

在阻塞同步模型和非阻塞同步模型(如 epoll)中，用户态 IO 操作是一锤子买卖，无需担心生存期。

但 io_uring 是 Proactor，是非阻塞异步模型，对资源的生存期有所约束。

以 read 为例，它有 fd 和 buf 两个资源参数，当准备 IO 操作时，我们需要把 fd、buf 指针和 count 填入 SQE，并且**保证在内核完成或取消该任务之前，fd 和 buf 都必须有效**。

### fd 意外替换

```c
fd = 6, buf = 0x5678;
准备 SQE;
close fd = 6;
open -> fd = 6;
提交 SQE;
内核执行 IO;
```

在提交 SQE 之前，应用程序“不小心”关闭又打开了文件，这将导致 IO 操作意外地被执行到一个完全无关的文件上。

### 栈内存 UAF

```c
char stack_buf[1024];
fd = 6, buf = &stack_buf;
准备 SQE;
提交 SQE;
函数返回;
内核执行 IO;
```

内核执行的 IO 会操作已被释放的栈上内存，出现“释放后使用”(use-after-free) 漏洞。

### 堆内存 UAF

```c
char* heap_buf = malloc(1024);
fd = 6, buf = heap_buf;
准备 SQE;
提交 SQE;
执行其他代码出错;
free(heap_buf);
函数返回错误码;
内核执行 IO;
```

内核执行的 IO 会使用已被释放的堆上内存，又一个 UAF 漏洞。

### 移动后使用

```rust
struct Buf<T>(T);
let mut buf1: Buf<[u8;1024]> = Buf([0;1024]);
fd = 6, buf = buf1.0.as_mut_ptr();
unsafe {
    准备 SQE;
}
提交 SQE;
let buf2 = Box::new(buf1);
内核执行 IO;
```

当内核执行 IO 时，buf1 已被移动，指针失效。出现“移动后使用”的漏洞，本文称为 UAM 漏洞。

### 取消后使用

```rust
async fn foo() -> io::Result<()> {
    let mut buf1: [u8;1024] = [0;1024];
    fd = 6, buf = buf1.as_mut_ptr();
    unsafe {
        准备 SQE;
    }
    提交 SQE;
    bar().await
}
```

Rust 的 async 函数会生成无栈协程，栈变量保存在一个结构体中。如果这个结构体被析构，底层的叶 Future 就会被析构，同时取消异步操作。

然而析构函数是同步的，**当协程析构时，内核仍然可能正在占用缓冲区来执行 IO**。如果不做处理，就会出现 UAF 漏洞。


### 关闭后使用

```rust
准备 SQE;
提交 SQE;
io_uring_queue_exit(&ring)
???
```

内核在 io_uring_queue_exit 之后会立即取消正在执行的 IO 吗？

// TODO: 找到答案

如果会立即取消，那么用户态程序也无法得到取消事件，无法唤醒任务或释放资源。

如果不会立即取消，那么内核对资源的占用会超出 io_uring 实例的生存期，带来更加麻烦的问题。

这似乎说明 io_uring 实例必须为 static 生存期，与线程本身活得一样长。或者采取某种引用计数的方式，推迟 exit 时机。

## 具有 Rust 特色的 io_uring

Rust 的底线是内存安全，不允许出现内存安全漏洞或数据竞争。Rust 的所有权规则为此提供了很好的保障。

### 迁移所有权

“迁移所有权” 是本文中自行创造的概念，它表示要进行某个操作就必须放弃对参数的所有权，把参数的所有权“迁移”到其他地方。

当使用 io_uring 时，相当于内核持有资源的所有权。用户态必须放弃对资源的控制权，除非它可以安全地并发操作。IO 操作完成或取消时，内核占用的所有资源会被返还给用户态。

但内核不可能真的去持有所有权，实际上是由异步运行时来存储这些资源，并模拟出“迁移所有权”的模型。

`BufRead` trait 表示一个包含内部缓冲区的可读取类型。`BufReader<File>` 是一个典型用法。

`BufReader<File>` 可以匹配 io_uring 的工作模式。

```
准备 fd, buf
准备 SQE
提交 SQE
等待唤醒
拿到返回值
回收 fd, buf
暴露 buf 的共享引用
```

**问题 3**: 当 Future 被取消时，buf 仍然被内核占用，`BufReader<File>` 处于无效状态。再次进行 IO 时，它只能选择死亡。

想象这样一个底层 Future

```rust
pub struct Read<F, B>
where
    F: AsRawFd + 'static,
    B: AsMut<[u8]> + 'static,
{
    fd: F,
    buf: B,
    ...
}
```

buf 可以是 `[u8; N]`，也满足 `AsMut<[u8]> + 'static`，但它不能被取指针传递给 io_uring。

buf 在这个 Future 被析构时失效，不满足 io_uring 的约束。

修复方案有两种：在准备 SQE 之前就把 fd 和 buf 都移动到堆上，或者限制 buf 为可安全逃逸的缓冲区类型。

### 堆分配

如果要在准备 SQE 之前确保 fd 和 buf 不会被析构，只能堆分配了。

这样 fd 和 buf 在 IO 操作完成或取消之前就不会被移动或析构，保证了有效性。

```rust
pub struct Read<F, B>
where
    F: AsRawFd + 'static,
    B: AsMut<[u8]> + 'static,
{
    state: ManualDrop<Box<State<F, B>>>
}
```

然而，大部分时候 buf 都是指向堆上动态大小缓冲区的智能指针，为指针本身去堆分配是不太值得的，要提高效率必须以某种方式实现自定义分配器。


### 逃逸

通常的“逃逸分析”是分析对象的动态范围，如果对象有可能离开函数作用域，就把它分配到堆上。

本文提出的“逃逸”是指让结构体成员逃脱析构，转移到一个稳定的地方。

可安全逃逸的缓冲区类型在移动时不会改变缓冲区的内存地址。

`[u8;N]` 在移动时完全改变了缓冲区的地址范围，而 `Box<[u8]>` 和 `Vec<u8>` 不会改变。

`SmallVec<[u8;N]>` 在容量不大于 N 时会把数据存储在栈上，过大时存储在堆上。

`Box<[u8]>` 和 `Vec<u8>` 作为缓冲区可以安全逃逸，`[u8;N]` 和 `SmallVec<[u8;N]>` 不可以。

如果限制 buf 为可安全逃逸的缓冲区类型，那么在最理想的情况下，进行 IO 操作时不需要系统调用，不需要额外的堆分配，缓冲区由调用者控制，几乎完美。

**问题 4**: 如何在不传染 unsafe 的情况下表达这种约束？

定义一个 unsafe trait 自然省事，但无法对所有符合条件的缓冲区通用，还可能受孤儿规则影响，让用户必须去写 newtype 或 unsafe。

可以意识到，这里的“安全逃逸”和 `Pin` 的概念有某种相关，有没有办法联系起来？


### Send

io_uring 的收割可以由本线程做，也可以由一个专门的驱动线程做。

目前 SQ 不支持多线程提交，全局共享需要上锁。io_uring 更匹配每个线程自带一个 ring 的实现。

考虑这样一个 Future，当它析构时，里面的资源会逃逸到堆上。

```rust
pub struct Read<F, B>
where
    F: AsRawFd + 'static,
    B: EscapedBufMut + 'static,
{
    fd: F,
    buf: B,
    ...
}
```

如果由全局驱动线程做最终析构，那么资源就会从当前线程转移到驱动线程，这需要资源满足 Send。

如果由本线程做最终析构，那么资源不需要转移，可以不满足 Send。

**问题 5**: 收割和析构策略也会影响 API 的泛型约束，如何设计合适的 API？

### 拷贝

缓冲区必须能在 Future 析构之后保持有效，这意味着我们无法把临时的 `&mut [u8]` 或 `&[u8]` 传入 io_uring，无法做原地读取或写入。

而 epoll 可以等待 fd 可读或可写后，再原地读取或写入。

无论如何，把缓冲区放在堆上这一步是不可避免的，区别在于缓冲区是由异步类型本身来控制还是由调用者来控制。

让调用者来控制缓冲区，能避免额外拷贝，但会加大安全审查的难度，必须限制传入的缓冲区具有良好的行为。

异步类型内置缓冲区，会增加额外拷贝，但安全性由库的作者保证，减小了出现漏洞的可能性。

**问题6**: io_uring 加大了实现用户态零拷贝的难度。

## 生态

[uring-sys](https://github.com/ringbahn/uring-sys)： liburing 的绑定。

[iou](https://github.com/ringbahn/iou)：Rust 风格的低层 io_uring 接口。

[ringbahn](https://github.com/ringbahn/ringbahn)：实验性的 io_uring 高层封装

[maglev](https://github.com/ringbahn/maglev)：实验性的 io_uring 异步驱动/运行时

## 总结

划个重点

**问题 1**: epoll 是 Reactor 模型的实现，而 io_uring 是 Proactor 模型的实现。改变异步模型并不是一件容易的事，除非以性能为代价抹平差异。

**问题 2**: io_uring 需要较高版本的内核，现阶段，应用程序不得不考虑在没有 io_uring 高版本特性时要怎么回退 (fallback)。

**问题 3**: 当 Future 被取消时，buf 仍然被内核占用，异步类型可能处于无效状态。再次进行 IO 时，它只能选择死亡。

**问题 4**: 如果选择限制 buf 为可安全逃逸的缓冲区类型，如何在不传染 unsafe 的情况下表达这种约束？

**问题 5**: 收割和析构策略也会影响 API 的泛型约束，如何设计合适的 API？

**问题 6**: io_uring 加大了实现用户态零拷贝的难度。

如果不考虑最高性能，我们有各种方案来封装一个能用的 io_uring 库。

如果不考虑通用，我们可以在自己的程序中谨慎地用 io_uring，锁死类型。

Rust 对安全、性能、通用的追求给封装 io_uring 带来了较高的难度。

[ringbahn](https://github.com/ringbahn/ringbahn) 的设计思路是其中一种可能的方向。社区还需要探索什么才是最完美的设计。

## 扩展阅读

[Efficient IO with io_uring](http://kernel.dk/io_uring.pdf)

[AIO 的新归宿：io_uring](https://zhuanlan.zhihu.com/p/62682475)

[Go 与异步 IO - io_uring 的思考](http://icebergu.com/archives/go-iouring)

[Notes on io-uring](https://without.boats/blog/io-uring/)

[Ringbahn: a safe, ergonomic API for io-uring in Rust](https://without.boats/blog/ringbahn/)

[Ringbahn II: the central state machine](https://without.boats/blog/ringbahn-ii/)

[Ringbahn III: A deeper dive into drivers](https://without.boats/blog/ringbahn-iii/)

[feature requests: submit requests from any thread](https://github.com/axboe/liburing/issues/109)

---

本文首发于知乎专栏 「[Rust 日常](https://zhuanlan.zhihu.com/p/346219893)」

作者简介：

王徐旸，大三学生，2018 年开始学习和使用 Rust 语言，造轮子爱好者。

GitHub ID: [Nugine](https://github.com/Nugine)
