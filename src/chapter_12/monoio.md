# 字节跳动 ｜ Rust 异步运行时的设计与实现

作者：茬海/徐帅

---

本文主要介绍如何设计和实现一个基于 io-uring 的 Thread-per-core 模型的 Runtime。

我们的 Runtime **Monoio** 现已开源，你可以在 [github.com/bytedance/monoio](https://github.com/bytedance/monoio) 找到它。

下面我们会通过两个章节来介绍：
- `科普篇` 简单介绍一下异步 IO 以及 rust runtime 中需要的主要抽象。
- `实现篇` 本章节我们会介绍到 Monoio 的实现细节。

# 科普篇
## epoll & io-uring
为了做到异步并发，我们需要内核提供相关的能力，来做到在某个 IO 处于阻塞状态时能够处理其他任务。

### epoll
讲 epoll 的文章多如牛毛，在此简单地提一下：epoll 是 linux 下较好的 IO 事件通知机制，可以同时监听多个 fd 的 ready 状态。

它主要包含 3 个 syscall：
1. `epoll_create`
    创建 epoll fd。
2. `epoll_ctl`
    向 epoll fd 添加、修改或删除其监听的 fd event。
3. `epoll_wait`
    等待监听 fd event，任何一个 event 发生时即返回；同时也支持传入一个 timeout，这样即便是没有 ready event，也可以在超时后返回。

如果你不使用 epoll 这类，而是直接做 syscall，那么你需要让 fd 处于阻塞模式（默认），这样当你想从 fd 上 read 时，read 就会阻塞住直到有数据可读。

使用 epoll 的时候，需要设置 fd 为非阻塞模式。当 read 时，在没有数据的情况下，read 也会立刻返回 `WOULD_BLOCK`。这时需要做的事情是将这个 fd 注册到 epoll fd 上，并设置 `EPOLLIN` 事件。

之后在没事做的时候（所有任务都卡在 IO 了），陷入 syscall epoll_wait。当有 event 返回时，再对对应的 fd 做 read（取决于注册时设置的触发模式，可能还要做一些其他事情，确保下次读取正常）。

总的来说，这个机制十分简单：设置 fd 为非阻塞模式，并在需要的时候注册到 epoll fd 上，然后 epoll fd 的事件触发时，再对 fd 进行操作。这样将多个 fd 的阻塞问题转变为单个 fd 的阻塞。

### io-uring
与 epoll 不同，io-uring 不是一个事件通知机制，它是一个真正的异步 syscall 机制。你并不需要在它通知后再手动 syscall，因为它已经帮你做好了。

io-uring 主要由两个 ring 组成（SQ 和 CQ），SQ 用于提交任务，CQ 用于接收任务的完成通知。任务（Op）往往可以对应到一个 syscall（如 read 对应 ReadOp），也会指定这次 syscall 的参数和 flag 等。

在 submit 时，内核会消费掉所有 SQE，并注册 callback。之后等有数据时，如网卡中断发生，数据通过驱动读入，内核就会触发这些 callback，做 Op 想做的事情，如拷贝某个 fd 的数据到 buffer（这个 buffer 是用户指定的 buffer）。相比 epoll，io-uring 是纯同步的。

> 注：本节涉及的 io-uring 相关是对带 FAST_POLL 的描述。
>
> https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/commit/?id=d7718a9d25a61442da8ee8aeeff6a0097f0ccfd6

总结一下，io-uring 和 epoll 在使用上其实差不多，一般使用方式是：直接将想做的事情丢到 SQ 中（如果 SQ 满了就要先 submit 一下），然后在没事干（所有任务都卡在 IO 了）的时候 `submit_and_wait(1)`（`submit_and_wait` 和 `submit` 并不是 syscall，它们是 liburing 对 `enter` 的封装）；返回后消费 CQ，即可拿到 syscall 结果。如果你比较在意延迟，你可以更激进地做 `submit`，尽早将任务推入可以在数据 ready 后尽早返回，但与此同时也要付出 syscall 增多的代价。

## 异步任务的执行流
常规的编程方式下，我们的代码控制流是对应线程的。就像你在写 C 时理解的那样，你的代码会直接编译到汇编指令，然后会由操作系统提供的“线程”去执行，在这其中没有多余的插入的逻辑。

以 epoll 为例，基于 epoll 的异步本质上是对线程的多路复用。那么常规方式下的类似下面的代码就无法在这种场景下使用：
```
for connection = listener.accept():
    do_something(connection)
```
因为这段代码中的 accept 是需要等待 IO 的，直接阻塞在这里会导致线程阻塞，这样就无法执行其他任务了。

### 面向 Callback 编程
在 C/C++ 中常被使用的 libevent 就是这种模型。用户代码不掌握主动权（因为线程的掌控权就一个，而用户任务千千万万），而是通过将 Callback 注册在 libevent 上，关联某个事件。当事件发生时，libevent 会调用用户的回调函数，并将事件的参数传递给用户。用户在初始化好一些 callback 后，将线程的主动权交给 libevent。其内部会帮忙处理和 epoll 的交互，并在 ready 时执行 callback。

这种方式较为高效，但是写起来却令人头大。举例来说，如果你想做一次 HTTP 请求，那么你需要将这段代码拆成多个同步的函数，并通过 callback 将他们串起来：
![callback](https://i.imgur.com/nTiCXaX.png)

本来一次可以内聚在一个一个函数里的行为，被拆成了一堆函数。相比面向过程，面向状态编程十分混乱，且容易因为编码者遗忘一些细节而出问题。

### 有栈协程
如果我们能在用户代码和最终产物之间插入一些逻辑呢？像 Golang 那样，用户代码实际上只对应到可被调度的 goroutine，实际拥有线程控制权的是 go runtime。goroutine 可以被 runtime 调度，在执行过程中也可以被抢占。

当 goroutine 需要被中断然后切换到另一个 goroutine 时，runtime 只需要修改当前的 stack frame 即可。每个 goroutine 对应的栈其实是存在堆上的，所以可以随时打断随时复原。

网络库也是配合这一套 runtime 的。syscall 都是非阻塞的，并可以自动地挂在 netpoll 上。

有栈协程配合 runtime，解耦了 Task 级的用户代码和线程的对应关系。

### 基于 Future 的无栈协程
有栈协程的上下文切换开销不可忽视。因为可以被随时打断，所以我们有必要保存当时的寄存器上下文，否则恢复回去时就不能还原现场了。

无栈协程没有这个问题，这种模式非常符合 Rust 的 Zero Cost Abstraction 的理念。Rust 中的 `async + await` 本质上是代码的自动展开，`async + await` 代码会基于 llvm generator 自动展开成状态机，状态机实现 Future 通过 poll 和 runtime 交互（具体细节可以参考[这篇文章](https://hsqstephenzhang.github.io/2021/11/24/rust/future-explained0/)）。

## Rust 异步机制原理
Rust 的异步机制设计较为复杂，标准库的接口和 Runtime 实现是解耦的。

Rust 的异步依赖于 Future trait。
```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

Future 是如何被执行的？根据上述 trait 定义很显然，是通过 `poll` 方法。返回的结果是 `Poll`，要么 `Pending` 要么 `Ready(T)`。

那么 poll 方法是谁来调用的？
1. 用户。用户可以手动实现 Future，如用户对某个 Future 做包装，显然它需要实现 `poll` 并调用 `inner.poll`。
2. Runtime。Runtime 是最终 `poll` 的调用者。

作为 Future 的实现者，需要保证的是，一旦返回了 `Poll::Pending`，就要在未来该 Future 依赖的 IO 就绪后能够唤醒它。唤醒一个 Future 是通过 Context 内的 Waker 做到的。至于说唤醒之后要做什么，这个由 Runtime 所提供的 cx 自行实现（如将这个 Task 加入到待执行队列）。

所以任何会产生事件的东西都要负责存储 Waker 并在 Ready 后唤醒它；而提供 cx 的东西会接收到这次唤醒操作，要负责重新调度它。这两个概念分别对应 Reactor 和 Executor，这两套东西靠 Waker 和 Future 解耦。

![simple-runtime](https://i.imgur.com/A7ApXSS.png)

### Reactor
举例来说，你甚至可以在 Tokio 之上再实现一套自己的 Reactor（约等于自己做了一套多路复用）。事实上 Tokio-uring 就是这么做的：它本身注册在 Tokio(mio) 上了一个 uring fd，而基于这个 fd 和一套自己的 Pending Op 管理系统又对外作为 Reactor 暴露了事件源的能力。在 tokio-tungstenite 中，也通过 `WakerProxy` 来解决了读写唤醒问题。

另一个例子是计时器驱动。显然 IO 事件的信号来自于 epoll/io-uring 等，但计时器并不是，其内部维护了时间轮或 N 叉堆之类的结构，所以唤醒 Waker 一定是 Time Driver 的职责，所以它一定需要存储 Waker。Time Driver 在这个意义上是一个 Reactor 实现。

### Executor
说完了 Reactor 我们来科普一下 Executor。Executor 负责任务调度和执行。以 Thread-per-core 场景为例（为啥用这个例子？没有跨线程调度写起来多简单啊），完全可以实现成一个 `VecDeque`，Executor 做的事就是从里面拿任务，调用它的 poll 方法。

### IO 组件
看到这里你可能会好奇，既然 Reactor 负责唤醒对应 Task，Executor 负责执行唤醒的 Task，那我们看一下源头，是谁负责将 IO 注册到 Reactor 上的呢？

IO 库（如 TcpStream 实现）会负责将未能立刻完成的 IO 注册到 Reactor 上。这也是为什么你在使用 Tokio 的时候会非要用 `Tokio::net::TcpStream` 而不能用标准库的原因之一；以及你想异步 sleep 时也需要用 Runtime 提供的 sleep 方法。

## 实现一个极简的 Runtime
出于简便目的，我们使用 epoll 来写这个 Runtime。这并不是我们的最终产品，只是为了演示如何实现最简单的 Runtime。

本小节的完整代码在 [github.com/ihciah/mini-rust-runtime](https://github.com/ihciah/mini-rust-runtime) 。

对应前文的三部组成部分：Reactor、Executor 和 IO 组件，我们分别实现。我们从 Reactor 入手吧。

### Reactor
由于裸操作 epoll 的体验有点差，并且本文的重点也并不是 Rust Binding，所以这里使用 polling 这个 crate 来完成 epoll 操作。

polling crate 的基础用法是创建 `Poller`，以及向 `Poller` 添加或修改 fd 和 interest。这个包默认使用 oneshot 模式（`EPOLLONESHOT`），需要在 event 触发后重新注册。在多线程场景下这可能是有用的，不过在我们的单线程的最简版本中似乎没有必要这么做，因为会带来额外的 `epoll_ctl` syscall 开销。不过处于简便起见，我们仍旧使用它。

作为 Reactor，我们在向 `Poller` 注册 interest 时，需要提供一个其对应的标识符，这个标识符在很多其他地方会被叫做 Token 或 UserData 或 Key。当 Event Ready 后，这个标志符会被原样返回。

所以我们需要做的事情大概是这样的：
1. 创建 `Poller`
2. 当需要关注某个 fd 的 Readable 或 Writable 时，向 `Poller` 添加 interest Event，并将 Waker 存下来
3. 添加 interest Event 前需要分配一个 Token，这样当 Event Ready 后我们才知道这个 Event 对应的 Waker 在哪。

于是我们可以将 Reactor 设计为：
```rust
pub struct Reactor {
    poller: Poller,
    waker_mapping: rustc_hash::FxHashMap<u64, Waker>,
}
```
在其他 Runtime 实现中往往会使用 slab，slab 同时处理了 Token 分配和 Waker 的存储。

简便起见，这里保存 Token 和 Waker 的关系时直接使用了 HashMap。Waker 的存储比较 trick 的方式解决：由于我们只关心读和写，所以我们将读对应的 MapKey 定义为 `fd * 2`，写对应的 MapKey 定义为 `fd * 2 + 1`（因为 TCP 连接是全双工的，同一个 fd 上的读写无关，可以在不同的 Task 上，有各自独立的 Waker）；而 Event 的 UserData（Token）仍旧使用 fd 本身。

```rust
impl Reactor {
    pub fn modify_readable(&mut self, fd: RawFd, cx: &mut Context) {
        println!("[reactor] modify_readable fd {} token {}", fd, fd * 2);

        self.push_completion(fd as u64 * 2, cx);
        let event = polling::Event::readable(fd as usize);
        self.poller.modify(fd, event);
    }

    pub fn modify_writable(&mut self, fd: RawFd, cx: &mut Context) {
        println!("[reactor] modify_writable fd {}, token {}", fd, fd * 2 + 1);

        self.push_completion(fd as u64 * 2 + 1, cx);
        let event = polling::Event::writable(fd as usize);
        self.poller.modify(fd, event);
    }

    fn push_completion(&mut self, token: u64, cx: &mut Context) {
        println!("[reactor token] token {} waker saved", token);

        self.waker_mapping.insert(token, cx.waker().clone());
    }
}
```

要将 fd 挂在 Poller 上或摘掉也十分简单：
```rust
impl Reactor {
    pub fn add(&mut self, fd: RawFd) {
        println!("[reactor] add fd {}", fd);

        let flags =
            nix::fcntl::OFlag::from_bits(nix::fcntl::fcntl(fd, nix::fcntl::F_GETFL).unwrap())
                .unwrap();
        let flags_nonblocking = flags | nix::fcntl::OFlag::O_NONBLOCK;
        nix::fcntl::fcntl(fd, nix::fcntl::F_SETFL(flags_nonblocking)).unwrap();
        self.poller
            .add(fd, polling::Event::none(fd as usize))
            .unwrap();
    }

    pub fn delete(&mut self, fd: RawFd) {
        println!("[reactor] delete fd {}", fd);

        self.completion.remove(&(fd as u64 * 2));
        println!("[reactor token] token {} completion removed", fd as u64 * 2);
        self.completion.remove(&(fd as u64 * 2 + 1));
        println!(
            "[reactor token] token {} completion removed",
            fd as u64 * 2 + 1
        );
    }
}
```

一个注意事项是，在挂上去之前要设置为 Nonblocking 的，否则在做 syscall 时，如果出现误唤醒（epoll 并没有保证不会误唤醒）会导致线程阻塞。

然后我们会面临一个问题：什么时候 `epoll_wait`？答案是没有任务的时候。如果所有任务都等待 IO 了，那么我们可以安全地陷入 syscall。所以我们的 Reactor 需要暴露一个 wait 接口来供 Executor 在没有任务时等待。
```rust
pub struct Reactor {
    poller: Poller,
    waker_mapping: rustc_hash::FxHashMap<u64, Waker>,

    buffer: Vec<Event>,
}

impl Reactor {
    pub fn wait(&mut self) {
        println!("[reactor] waiting");
        self.poller.wait(&mut self.buffer, None);
        println!("[reactor] wait done");

        for i in 0..self.buffer.len() {
            let event = self.buffer.swap_remove(0);
            if event.readable {
                if let Some(waker) = self.waker_mapping.remove(&(event.key as u64 * 2)) {
                    println!(
                        "[reactor token] fd {} read waker token {} removed and woken",
                        event.key,
                        event.key * 2
                    );
                    waker.wake();
                }
            }
            if event.writable {
                if let Some(waker) = self.waker_mapping.remove(&(event.key as u64 * 2 + 1)) {
                    println!(
                        "[reactor token] fd {} write waker token {} removed and woken",
                        event.key,
                        event.key * 2 + 1
                    );
                    waker.wake();
                }
            }
        }
    }
}
```
接收 syscall 结果时需要提供预分配好的 buffer（`Vec<Event>`），为了避免每次都分配，我们直接在结构体中保存下来这个 buffer，通过 `Option` 包装可以让我们临时地拿到它的所有权。

`wait` 需要做的事情是：
1. 陷入 syscall
2. syscall 返回后，处理所有就绪的 Event。如果事件是 readable 或者 writable 的，那么就分别从 HashMap 里找到并删除其对应的 Completion，然后唤醒它（这个 fd 和 Map Key 的对应规则我们前面也说到了，readable 对应 `fd * 2`，writable 对应 `fd * 2 + 1`）。

最后将 Reactor 的创建函数补齐：
```rust
impl Reactor {
    pub fn new() -> Self {
        Self {
            poller: Poller::new().unwrap(),
            waker_mapping: Default::default(),

            buffer: Vec::with_capacity(2048),
        }
    }
}

impl Default for Reactor {
    fn default() -> Self {
        Self::new()
    }
}
```
这时我们的 Reactor 就写完了。总的来说就是包装了 epoll，同时额外做了 Waker 的存储和唤醒。

### Executor
Executor 需要存储 Task 并执行。

#### Task
Task 是什么？Task 其实是 Future，但因为 Task 需要共享所有权，所以这里我们使用 Rc 来存储；并且我们只知道用户丢进来一个 Future，并不知道它的具体类型，所以我们需要把它 Box 起来，这里使用 `LocalBoxFuture`。再加上内部可变性，所以 Task 的定义如下：
```rust
pub struct Task {
    future: RefCell<LocalBoxFuture<'static, ()>>,
}
```

#### TaskQueue
设计 Task 的存储结构，简便起见直接使用 VecDeque。
```rust
pub struct TaskQueue {
    queue: RefCell<VecDeque<Rc<Task>>>,
}
```

这个 TaskQueue 需要能够 push 和 pop 任务：
```rust
impl TaskQueue {
    pub(crate) fn push(&self, runnable: Rc<Task>) {
        println!("add task");
        self.queue.borrow_mut().push_back(runnable);
    }

    pub(crate) fn pop(&self) -> Option<Rc<Task>> {
        println!("remove task");
        self.queue.borrow_mut().pop_front()
    }
}
```

#### Waker
Executor 需要提供 Context，其中包含 Waker。Waker 需要能够在被 wake 时将任务推入执行队列等待执行。

```rust
pub struct Waker {
    waker: RawWaker,
}

pub struct RawWaker {
    data: *const (),
    vtable: &'static RawWakerVTable,
}
```

Waker 通过自行指针和 vtable 自行实现动态分发。所以我们要做的事情有两个：
1. 拿到 Task 结构的指针并维护它的引用计数
2. 生成类型对应的 vtable

我们可以这么定义 vtable：
```rust
struct Helper;

impl Helper {
    const VTABLE: RawWakerVTable = RawWakerVTable::new(
        Self::clone_waker,
        Self::wake,
        Self::wake_by_ref,
        Self::drop_waker,
    );

    unsafe fn clone_waker(data: *const ()) -> RawWaker {
        increase_refcount(data);
        let vtable = &Self::VTABLE;
        RawWaker::new(data, vtable)
    }

    unsafe fn wake(ptr: *const ()) {
        let rc = Rc::from_raw(ptr as *const Task);
        rc.wake_();
    }

    unsafe fn wake_by_ref(ptr: *const ()) {
        let rc = mem::ManuallyDrop::new(Rc::from_raw(ptr as *const Task));
        rc.wake_by_ref_();
    }

    unsafe fn drop_waker(ptr: *const ()) {
        drop(Rc::from_raw(ptr as *const Task));
    }
}

unsafe fn increase_refcount(data: *const ()) {
    let rc = mem::ManuallyDrop::new(Rc::<Task>::from_raw(data as *const Task));
    let _rc_clone: mem::ManuallyDrop<_> = rc.clone();
}
```

手动管理引用计数：我们会通过 `Rc::into_raw` 来取得 `Rc<Task>` 的裸指针并使用它和 vtable 构建 RawTask 然后构建 Task。在 vtable 实现中，我们需要小心地手动管理引用计数：如 `clone_waker` 时，虽然我们只 clone 一个指针，但它在含义上有了一次拷贝，所以我们需要手动让其引用计数加一。

Task 实现 `wake_` 和 `wake_by_ref_` 来重新调度任务。重新调度任务做的事情只是简单地从 thread local storage 拿到 executor 然后向 TaskQueue push。
```rust
impl Task {
    fn wake_(self: Rc<Self>) {
        Self::wake_by_ref_(&self)
    }

    fn wake_by_ref_(self: &Rc<Self>) {
        EX.with(|ex| ex.local_queue.push(self.clone()));
    }
}
```

#### Executor
有了前面这些组件以后，构建 Executor 是非常简单的。
```rust
scoped_tls::scoped_thread_local!(pub(crate) static EX: Executor);

pub struct Executor {
    local_queue: TaskQueue,
    pub(crate) reactor: Rc<RefCell<Reactor>>,

    /// Make sure the type is `!Send` and `!Sync`.
    _marker: PhantomData<Rc<()>>,
}
```

当用户 spawn Task 的时候：
```rust
impl Executor {
    pub fn spawn(fut: impl Future<Output = ()> + 'static) {
        let t = Rc::new(Task {
            future: RefCell::new(fut.boxed_local()),
        });
        EX.with(|ex| ex.local_queue.push(t));
    }
}
```
其实做的事情就是将传入的 Future Box 起来然后构建 `Rc<Task>` 之后丢到执行队列里。

那么 Executor 的主循环在哪呢？我们可以放在 `block_on` 里。
```rust
impl Executor {
    pub fn block_on<F, T, O>(&self, f: F) -> O
    where
        F: Fn() -> T,
        T: Future<Output = O> + 'static,
    {
        let _waker = waker_fn::waker_fn(|| {});
        let cx = &mut Context::from_waker(&_waker);

        EX.set(self, || {
            let fut = f();
            pin_utils::pin_mut!(fut);
            loop {
                // return if the outer future is ready
                if let std::task::Poll::Ready(t) = fut.as_mut().poll(cx) {
                    break t;
                }

                // consume all tasks
                while let Some(t) = self.local_queue.pop() {
                    let future = t.future.borrow_mut();
                    let w = waker(t.clone());
                    let mut context = Context::from_waker(&w);
                    let _ = Pin::new(future).as_mut().poll(&mut context);
                }

                // no task to execute now, it may ready
                if let std::task::Poll::Ready(t) = fut.as_mut().poll(cx) {
                    break t;
                }

                // block for io
                self.reactor.borrow_mut().wait();
            }
        })
    }
}
```
这段有点复杂，可以分成以下几个步骤：
1. 创建一个 dummy_waker，这个 waker 其实啥事不做。
2. (in loop)poll 传入的 future，检查是否 ready，如果 ready 就返回，结束 block_on。
3. (in loop)循环处理 TaskQueue 中的所有 Task：构建它对应的 Waker 然后 poll 它。
4. (in loop)这时已经没有待执行的任务了，可能主 future 已经 ready 了，也可能都在等待 IO。所以再次检查主 future，如果 ready 就返回。
5. (in loop)既然所有人都在等待 IO，那就 `reactor.wait()`。这时 reactor 会陷入 syscall 等待至少一个 IO 可执行，然后唤醒对应 Task，会向 TaskQueue 里推任务。

至此 Executor 基本写完了。

### IO 组件
IO 组件要在 WouldBlock 时将其挂在 Reactor 上。以 TcpStream 为例，我们需要为其实现 `tokio::io::AsyncRead`。
```rust
pub struct TcpStream {
    stream: StdTcpStream,
}
```

在创建 TcpStream 时需要将其添加到 Poller 上，销毁时需要摘下：
```rust
impl From<StdTcpStream> for TcpStream {
    fn from(stream: StdTcpStream) -> Self {
        let reactor = get_reactor();
        reactor.borrow_mut().add(stream.as_raw_fd());
        Self { stream }
    }
}

impl Drop for TcpStream {
    fn drop(&mut self) {
        println!("drop");
        let reactor = get_reactor();
        reactor.borrow_mut().delete(self.stream.as_raw_fd());
    }
}
```

在实现 AsyncRead 时，对其做 read syscall。因为在添加到 Poller 时已经设置 fd 为非阻塞，所以这里 syscall 是安全的。
```rust
impl tokio::io::AsyncRead for TcpStream {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let fd = self.stream.as_raw_fd();
        unsafe {
            let b = &mut *(buf.unfilled_mut() as *mut [std::mem::MaybeUninit<u8>] as *mut [u8]);
            println!("read for fd {}", fd);
            match self.stream.read(b) {
                Ok(n) => {
                    println!("read for fd {} done, {}", fd, n);
                    buf.assume_init(n);
                    buf.advance(n);
                    Poll::Ready(Ok(()))
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    println!("read for fd {} done WouldBlock", fd);
                    // modify reactor to register interest
                    let reactor = get_reactor();
                    reactor
                        .borrow_mut()
                        .modify_readable(self.stream.as_raw_fd(), cx);
                    Poll::Pending
                }
                Err(e) => {
                    println!("read for fd {} done err", fd);
                    Poll::Ready(Err(e))
                }
            }
        }
    }
}
```
read syscall 可能会返回正确的结果，也可能会返回错误。其中错误中有一个错误需要特殊处理，就是 `WouldBlock`。当 `WouldBlock` 时，我们便需要将其挂在 Reactor 上，这里通过我们前面定义的函数 `modify_readable` 表示我们对 readabe 是关心的。在挂 Reactor 动作完成后，我们可以放心地返回 `Poll::Pending`，因为我们知道，它后续会被唤醒。

# 实现篇
## Monoio
### Motivation
我在字节参与 Mesh Proxy（基于 Envoy）的研发过程中，我感觉我们不得不因为 C++ 的问题而采取非常不优雅的代码组织和设计。

因此我尝试调研了基于 Linkerd2-proxy（一个基于 Rust + Tokio 的 Mesh Proxy）来替代现有版本。压测数据显示，在 HTTP 场景性能提升只有 10% 左右；而 Envoy 压测数据显示非常多的 CPU 消耗是在 syscall 上。

我们可以利用 Rust 泛型编程消除 C++ 中的基于动态分发的抽象带来的运行时开销；在 IO 上，我们考虑利用 io-uring 来替代 epoll。

### 前期调研
项目前期我们对比了几种方案下的性能：1. Tokio 2. Glommio 3. 裸 epoll 4. 裸 io-uring。之后发现裸 io-uring 性能上确实领先，但基于 io-uring 的 Glommio 的表现并不如人意。我们尝试 fork Glommio 的代码并做优化，发现它的项目本身存在较大问题，比如创建 uring 的 flag 似乎都没有正确理解；同时，它的 Task 实现相比 Tokio 的也性能较差。

### 自己造轮子
最终我们决定自己造一套 Runtime 来满足内部需求，提供极致的性能。

该项目是我和 @dyxushuai 两人共同完成的。在我们实现过程中，我们大量参考了 Tokio、Tokio-uring 等项目，并且尝试了一些自己的设计。

## 模型讨论
不同的设计模型会有各自擅长的应用场景。

Tokio 使用了公平调度模型，其内部的调度逻辑类似 Golang，任务可以在线程之间转移，这样能尽可能充分地利用多核心的性能。

Glommio 也是一个 Rust Runtime，它基于 io-uring 实现，调度逻辑相比 Tokio 更加简单，是 thread-per-core 模型。

两种模型各有优劣，前者更加灵活，通用型更强，但代价也并不小：
1. 在多核机器上的性能表现不佳。

    在我的 1K Echo 的测试中(2021-11-26 latest version)，Tokio 4 Core 下性能只是 1 Core 下性能的 2.2 倍左右。而我们自己的 Monoio 可以基本保持线性。

    1 Core                     |  4 Cores
    :-------------------------:|:-------------------------:
    ![1core](https://i.imgur.com/e7qtpqm.png)  |  ![4cores](https://i.imgur.com/fjLXAYf.png)

    详细的测试报告在[这里](https://github.com/bytedance/monoio/blob/master/docs/zh/benchmark.md)。

2. 对 Task 本身的约束也不能忽视。如果 Task 可以在 Thread 之间调度，那么它必须实现 `Send + Sync`。这对用户代码是一个不小的限制。

    举例来说，如果要实现一个 cache 服务，基于公平调度模型的话，cache 对应的 map 就要通过 Atomic 或 Mutex 等来确保 `Send + Sync`；而如果实现成 thread-per-core 模型，直接使用 thread local 就可以了。以及，nginx 和 envoy 也是基于这种模型。

但是 thread-per-core 并不是银弹。例如，在业务系统中，不同的请求可能处理起来的逻辑是不同的，有的长连接需要做大量的运算，有的则几乎不怎么消耗 CPU。如果基于这种模型，那么很可能导致 CPU 核心之间出现不均衡，某个核心已经被满载，而另一个核心又非常空闲。

## 事件驱动
这里主要讨论 io-uring 和 epoll。

epoll 只是通知机制，本质上事情还是通过用户代码直接 syscall 来做的，如 read。这样在高频 syscall 的场景下，频繁的用户态内核态切换会消耗较多资源。io-uring 可以做异步 syscall，即便是不开 `SQ_POLL` 也可以大大减少 syscall 次数。

io-uring 的问题在于下面几点：
1. 兼容问题。平台兼容就不说了，linux only（epoll 在其他平台上有类似的存在，可以基于已经十分完善的 mio 做无缝兼容）。linux 上也会对 kernel 版本有一定要求，且不同版本的实现性能还有一定差距。大型公司一般还会有自己修改的内核版本，所以想持续跟进 backport 也是一件头疼事。同时对于 Mac/Windows 用户，在开发体验上也会带来一定困难。
2. Buffer 生命周期问题。io-uring 是全异步的，Op push 到 SQ 后就不能移动 buffer，一定要保证其有效，直到 syscall 完成或 Cancel Op 执行完毕。无论是在 C/C++ 还是 Rust 中，都会面临 buffer 生命周期管理问题。epoll 没有这个问题，因为 syscall 就是用户做的，陷入 syscall 期间本来就无法操作 buffer，所以可以保证其持续有效直到 syscall 返回。

## 生命周期、IO 接口与 GAT
前一小节提到了 io-uring 的这个问题：需要某种机制保证 buffer 在 Op 执行期间是有效的。

考虑下面这种情况：
1. 用户创建了 Buffer
2. 用户拿到了 buffer 的引用（不管是 `&` 还是 `&mut`）来做 read 和 write。
3. Runtime 返回了 Future，但用户直接将其 Drop 了。
4. 现在没有人持有 buffer 的引用了，用户可以直接将其 Drop 掉。
5. 但是，buffer 的地址和长度已经被提交给内核，它可能即将被处理，也可能已经在处理中了。我们可以推入一个 `CancelOp` 进去，但是我们也不能保证 `CancelOp` 被立刻消费。
6. Kernel 这时已经在操作错误的内存啦，如果这块内存被用户程序复用，会导致内存破坏。

如果 Rust 实现了 Async Drop，这件事还能做——以正常的方式拿引用来使用 buffer；然鹅木有，我们不能保证及时取消掉内核对 buffer 的读写。

所以，我们很难在不拿所有权的情况下保证 buffer 的有效性。这样就对 IO 接口有个新的挑战：常规的 IO 接口只需要给 `&self` 或 `&mut self`，而我们必须要给所有权。

这部分设计我们参考了 tokio-uring，并把它定义为了 trait。这个 Trait 必须启动 GAT。

```rust
/// AsyncReadRent: async read with a ownership of a buffer
pub trait AsyncReadRent {
    /// The future of read Result<size, buffer>
    type ReadFuture<'a, T>: Future<Output = BufResult<usize, T>>
    where
        Self: 'a,
        T: 'a;
    /// The future of readv Result<size, buffer>
    type ReadvFuture<'a, T>: Future<Output = BufResult<usize, T>>
    where
        Self: 'a,
        T: 'a;

    /// Same as read(2)
    fn read<T: IoBufMut>(&self, buf: T) -> Self::ReadFuture<'_, T>;
    /// Same as readv(2)
    fn readv<T: IoVecBufMut>(&self, buf: T) -> Self::ReadvFuture<'_, T>;
}

/// AsyncWriteRent: async write with a ownership of a buffer
pub trait AsyncWriteRent {
    /// The future of write Result<size, buffer>
    type WriteFuture<'a, T>: Future<Output = BufResult<usize, T>>
    where
        Self: 'a,
        T: 'a;
    /// The future of writev Result<size, buffer>
    type WritevFuture<'a, T>: Future<Output = BufResult<usize, T>>
    where
        Self: 'a,
        T: 'a;

    /// Same as write(2)
    fn write<T: IoBuf>(&self, buf: T) -> Self::WriteFuture<'_, T>;

    /// Same as writev(2)
    fn writev<T: IoVecBuf>(&self, buf_vec: T) -> Self::WritevFuture<'_, T>;
}
```

类似 Tokio 的做法，我们还提供了一个带默认实现的 Ext：
```rust
pub trait AsyncReadRentExt<T: 'static> {
    /// The future of Result<size, buffer>
    type Future<'a>: Future<Output = BufResult<usize, T>>
    where
        Self: 'a,
        T: 'a;

    /// Read until buf capacity is fulfilled
    fn read_exact(&self, buf: T) -> <Self as AsyncReadRentExt<T>>::Future<'_>;
}

impl<A, T> AsyncReadRentExt<T> for A
where
    A: AsyncReadRent,
    T: 'static + IoBufMut,
{
    type Future<'a>
    where
        A: 'a,
    = impl Future<Output = BufResult<usize, T>>;

    fn read_exact(&self, mut buf: T) -> Self::Future<'_> {
        async move {
            let len = buf.bytes_total();
            let mut read = 0;
            while read < len {
                let slice = buf.slice(read..len);
                let (r, slice_) = self.read(slice).await;
                buf = slice_.into_inner();
                match r {
                    Ok(r) => {
                        read += r;
                        if r == 0 {
                            return (Err(std::io::ErrorKind::UnexpectedEof.into()), buf);
                        }
                    }
                    Err(e) => return (Err(e), buf),
                }
            }
            (Ok(read), buf)
        }
    }
}

pub trait AsyncWriteRentExt<T: 'static> {
    /// The future of Result<size, buffer>
    type Future<'a>: Future<Output = BufResult<usize, T>>
    where
        Self: 'a,
        T: 'a;

    /// Write all
    fn write_all(&self, buf: T) -> <Self as AsyncWriteRentExt<T>>::Future<'_>;
}

impl<A, T> AsyncWriteRentExt<T> for A
where
    A: AsyncWriteRent,
    T: 'static + IoBuf,
{
    type Future<'a>
    where
        A: 'a,
    = impl Future<Output = BufResult<usize, T>>;

    fn write_all(&self, mut buf: T) -> Self::Future<'_> {
        async move {
            let len = buf.bytes_init();
            let mut written = 0;
            while written < len {
                let slice = buf.slice(written..len);
                let (r, slice_) = self.write(slice).await;
                buf = slice_.into_inner();
                match r {
                    Ok(r) => {
                        written += r;
                        if r == 0 {
                            return (Err(std::io::ErrorKind::WriteZero.into()), buf);
                        }
                    }
                    Err(e) => return (Err(e), buf),
                }
            }
            (Ok(written), buf)
        }
    }
}
```
开启 GAT 可以让我们很多事情变得方便。

我们在 trait 中的关联类型 Future 上定义了生命周期，这样它就可以捕获 `&self` 而不是非要 `Clone self` 中的部分成员，或者单独定义一个带生命周期标记的结构体。

### 定义 Future
如何定义一个 Future？常规我们需要定义一个结构体，并为它实现 Future trait。这里的关键在于要实现 `poll` 函数。这个函数接收 `Context` 并同步地返回 `Poll`。要实现 `poll` 我们一般需要手动管理状态，写起来十分困难且容易出错。

这时你可能会说，直接 `async` 和 `await` 不能用吗？事实上 `async` 块确实生成了一个状态机，和你手写的差不多。但是问题是，这个生成结构并没有名字，所以如果你想把这个 Future 的类型用作关联类型就难了。这时候可以开启 `type_alias_impl_trait` 然后使用 opaque type 作为关联类型；也可以付出一些运行时开销，使用 `Box<dyn Future>`。

### 生成 Future
除了使用 `async` 块外，常规的方式就是手动构造一个实现了 `Future` 的结构体。这种 Future 有两种：
1. 带有所有权的额 Future，不需要额外写生命周期标记。这种 `Future` 和其他所有结构体都没有关联，如果你需要让它依赖一些不 `Copy` 的数据，那你可以考虑使用 `Rc` 或 `Arc` 之类的共享所有权的结构。
2. 带有引用的 Future，这种结构体本身上就带有生命周期标记。例如，Tokio 中的 `AsyncReadExt`，`read` 的签名是 `fn read<'a>(&'a mut self, buf: &'a mut [u8]) -> Read<'a, Self>`。这里构造的 `Read<'a, Self>` 捕获了 self 和 buf 的引用，相比共享所有权，这是没有运行时开销的。但是这种 Future 不好作为 trait 的 type alias，只能开启 `generic_associated_types` 和 `type_alias_impl_trait`，然后使用 opaque type。

### 定义 IO trait
通常，我们的 IO 接口要以 `poll` 形式定义（如 `poll_read`），任何对 IO 的包装都应当基于这个 trait 来做（我们暂时称之为基础 trait）。

但是为了用户友好的接口，一般会提供一个额外的 `Ext` trait，主要使用其默认实现。`Ext` trait 为所有实现了基础 trait 的类自动实现。例如，`read` 返回一个 Future，显然基于这个 future 使用 `await` 要比手动管理状态和 `poll` 更容易。

那为什么基础 trait 使用 `poll` 形式定义呢？不能直接一步到位搞 Future 吗？因为 poll 形式是同步的，不需要捕获任何东西，容易定义且较为通用。如果直接一步到位定义了 Future，那么，要么类似 `Ext` 一样直接把返回 Future 类型写死（这样会导致无法包装和用户自行实现，就失去了定义 trait 的意义），要么把 Future 类型作为关联类型（前面说了，不开启 GAT 没办法带生命周期，即必须 static）。

所以总结一下就是，在目前的 Rust 稳定版本中，只能使用 poll 形式的基础 trait + future 形式的 Ext trait 来定义 IO 接口。

在开启 GAT 后这件事就能做了。我们可以直接在 trait 的关联类型中定义带生命周期的 Future，就可以捕获 self 了。

这是银弹吗？不是。唯一的问题在于，如果使用了 GAT 这一套模式，就要总是使用它。如果你在 `poll` 形式和 GAT 形式之间反复横跳，那你会十分痛苦。基于 `poll` 形式接口自行维护状态，确实可以实现 Future（最简单的实现如 `poll_fn`）；但反过来就很难受了：你很难存储一个带生命周期的 Future。虽然使用一些 unsafe 的 hack 可以做(也有 cost)这件事，但是仍旧，限制很多且并不推荐这么做。`monoio-compat` 基于 GAT 的 future 实现了 Tokio 的 `AsyncRead` 和 `AsyncWrite`，如果你非要试一试，可以参考它。

## Buffer 管理
Buffer 管理参考了 tokio-uring 设计。

Buffer 由用户提供所有权，并在 Future 完成时将所有权返回回去。
利用 Slab 维护一个全局状态，当 Op drop 时转移内部 Buffer 所有权至全局状态，并在 CQE 时做真正销毁。正常完成时丢回所有权。

## Time Driver 设计
很多场景需要计时器，如超时，需要 select 两个 future，其中一个是 timeout。作为 Runtime 必须支持异步 sleep。

### 计时器管理与唤醒
Glommio 内部这部分实现较为简单，直接使用了 BTreeMap 维护 Instant -> Waker 的映射，每次拿当前时间去 split_off 拿到所有过期的 timer 唤醒并计算下次 wake 间隔时间，然后在 driver park 的时候作为参数传入。Tokio 中类似，也有自己的时间轮实现，更复杂，但效率也更高（精确度上不如 Glommio 的实现方案）。

考虑到我们性能优先的实现初衷，我们选择类似 Tokio 的时间轮方案。

### 和 Driver 集成
在 epoll 下，在我们做 wait 前，需要检查当前最近的计时器。如果有，那么必须将它的超时事件作为 wait 的参数传入，否则如果没有任何 IO 在这段时间里就绪，我们就会错过这次计时器的预期唤醒时间，如用户要 timeout 100ms，结果可能 200ms 了才唤醒，这已经失去意义了。

Tokio 内部基于 EPOLL 和 时间轮 做了这件事。EPOLL 作为 IO Driver，并在这个之上封装了 Timer Driver。在 Timer Driver 陷入 syscall 之前，计算时间轮中最近的事件超时间隔，并作为 epoll wait 的参数。

为啥不直接使用 TimerFd 直接利用 epoll 能力？因为这么搞有点重：timerfd 的创建、使用 epoll_ctl 添加和移除都是 syscall，而且不能做粗粒度的合并（时间轮的可以）。

然而，io-uring 的 enter 并不支持传入一个超时时间。我们只能向 SQ 推 TimeoutOp 来做到这件事。

#### 方案1
在插入 element 到时间轮空格子的时候，推 TimeoutOp；并在该格子取消至数量为 0 时推 TimeoutRemoveOp（取消这部分也可以不推，只是要额外付出一次误唤醒的 cost）。

例如，我们会创建 5 个 10ms 的超时，它们会被插入到时间轮的同一个格子。在这个格子中数量从 0 变 1 的时机，我们向 SQ 推一个 10ms 的 TimeoutOp。

#### 方案 2
每次 wait 前计算最近超时时间，推入 SQ 然后 wait；TimeoutOp 中设置 offset = 1。

这里解释一下 offset 参数的含义，简单来说就是当有 $offset 个 CQ 完成时，或超时发生时会完成。

> This command will register a timeout operation. The addr field must contain a pointer to a struct timespec64 structure, len must contain 1 to signify one timespec64 structure, timeout_flags may contain IORING_TIMEOUT_ABS for an absolute timeout value, or 0 for a relative timeout. off may contain a completion event count. A timeout will trigger a wakeup event on the completion ring for anyone waiting for events. A timeout condition is met when either the specified timeout expires, or the specified number of events have completed. Either condition will trigger the event. If set to 0, completed events are not counted, which effectively acts like a timer. io_uring timeouts use the CLOCK_MONOTONIC clock source. The request will complete with -ETIME if the timeout got completed through expiration of the timer, or 0 if the timeout got completed through requests completing on their own. If the timeout was cancelled before it expired, the request will complete with -ECANCELED. Available since 5.4.

这样需要在每次 wait 前推 SQ 进去，好处是不需要 remove（因为每次返回时就已经被消费掉了），没有误唤醒问题；并且实现简单，不需要维护 Op 的 user_data 字段用来推 TimeoutRemoveOp。
![wait](https://i.imgur.com/oPlmYRN.png)

#### 方案 3
类似方案 2，只不过 TimeoutOp 中的 offset 设置为 0。

这样实现起来较为麻烦：因为 offset = 0 表示它是一个纯粹的计时器，与 CQ 完成个数无关，它只会在实际超时时完成。这样就意味着，我们需要推 TimeoutRemoveOp 或者承担误唤醒开销（Glommio 实现类似这种方案，它的 cost 选择了后者）。
![wait](https://i.imgur.com/56vsyAw.png)

#### 讨论
在插入 TimeoutOp 时，我们应当尽可能晚地插入，因为它可能会被 cancel。所以方案 1 会在 wait 前 0->1->0->1 变化时插入 2 次 TimeoutOp 和 2 次TiemoutRemoveOp，而这是不必要的，方案 1 基本不可取。

方案 2 和 3 在执行时机上和 EPOLL 场景下的 Tokio 以及 Glommio 是一样的。细节上的差别是：
- 方案 2 下，任何一个 CQ 完成时顺便把 TimeoutOp 给完成掉，这样就不需要 Remove，也就是说不需要维护 user_data，实现上会非常简单，也省了推 TimeoutRemoveOp 以及内核处理的开销。
- 方案 3 相对 2 的好处是，当 wait 次数很多时，方案 2 每次都要推一个 TimeoutOp 进去，而方案 3 可以检查 TimeoutOp 是否被消耗掉，省一些推入次数；当然，对比方案 2 也有缺点，就是当超时取消时得推 TimeoutRemove 进去。

在我们的实际业务场景中，时间事件绝大多数都是作为超时，少部分是定时轮询用。
超时场景中往往是注册超时并移除超时，真正的超时并非热路径：所以我们这里初步决定是使用方案 2。同时，方案 2 实现简单，后续即便是要优化也成本不大。

## 跨线程通信
虽然是 thread per core 的 runtime，但是如果没有跨线程通信能力，很多事情是做不了的。比如常见的 case：单个线程拉取配置并分发到每个线程的 thread local 上。

如果只是期望有跨线程通信能力，那么不需要任何 runtime 支持。无论是使用无锁的数据结构，或是跨线程锁，都可以做到。

但我们希望可以在 runtime 层面集成。举例来说，A 线程有一个 channel rx，B 线程有一个 tx，我们通过 B 发送数据，A 可以 await 在 rx 上。这里的实现难点在于，A 线程上的 reactor 可能已经陷入内核进入等待 uring CQ 状态了，我们需要在任务被唤醒时额外唤醒其所在 thread。

### Unpark 能力
所以我们需要在 Driver trait 上额外添加一个 Unpark 接口用于跨线程主动唤醒。

在 epoll 下，tokio 内部实现是注册上去一个 eventfd。因为 tokio 本身的调度模型就依赖于跨线程唤醒，所以无论你是否使用 tokio 提供的一些 sync 数据结构，它都会在 epoll 上挂上这么一个 eventfd；而我们的实现主体是不依赖这个的，只有在用到我们实现的 channel 的时候才会依赖，所以我们这里通过条件编译，开启 “sync” feature 才插入相关代码，尽可能做到 zero cost。

在 iouring 下怎么插入 eventfd 呢？同 time driver 中我们实现 park_timeout 做的事情差不多，可以直接推一个 ReadOp 进去读 8 byte，fd 就是 eventfd 的 fd。eventfd 读写都是 8 byte（u64）。

> 注：文档里提到了两个 syscall flag（IORING_REGISTER_EVENTFD, IORING_REGISTER_EVENTFD_ASYNC），不是做这个事的。
> Ref: https://unixism.net/loti/ref-iouring/io_uring_register.html

在什么时机我们需要推入 eventfd 呢？我们可以在内部维护一个状态标记当前 ring 里是否已存在 eventfd。在 sleep 前，如果已存在则直接 sleep，不存在则推一个进去并标记已存在。

当消费 CQ 的时候，遇到 eventfd 对应的 userdata，则标记为不存在，这样下次 sleep 前会重新插入。

当我们需要 unpark 线程时，只需要拿到它对应的 eventfd，并向其中写入 1u64，则这个 fd 就会可读，触发 ring 从 syscall 返回。

我们将 UnparkHandle 维护在一个全局 Map 中便于每个线程都能够唤醒其他线程。在线程创建时，我们向全局注册自己的 UnparkHandle 的 Weak 引用。

当需要跨线程唤醒时，我们只需要从全局 Map 里拿到这个 UnparkHandle 并尝试 upgrade，然后写入数据即可。为了减少对全部 Map 的访问，我们在每个线程中缓存这个映射。

参考 Eventfd 的实现，kernel 内部一来有锁，二来会保证这 8 byte 的 u64 是一口气写完的，不存在乱序问题。所以目前实现改为了直接走 libc::write。（wow so unsafe!）

### 集成 Waker
在纯本线程下，我们的唤醒逻辑是这样的：
1. 我们想等待一个在本线程执行 future
2. 因为事件源是 uring，所以我们在 future 被 poll 时将 task 的 waker 存储在 op 关联的存储区
3. Uring 产生了事件，唤醒 waker
4. Waker 执行时将任务重新塞回本线程的执行队列

在 uring driver 下，我们的事件源是 uring，所以 uring 负责存储并唤醒 waker；在 time driver 下，我们的事件源是 time wheel，所以也由其负责存储和唤醒 waker。

现在我们的事件源是其他线程。以 oneshot channel 为例，当 rx poll 时，需要将 waker 存储在 channel 的共享存储区内；在 tx send 后，需要从 channel 共享存储区拿到 waker 并唤醒。waker 的唤醒逻辑不再是无脑把任务加到本地队列，而是需要调度到其所在线程的队列中。

所以这样我们需要为每个 Executor 添加一个 shared_queue 用于共享地存储远程推入的 waker。当非本地 waker 被 wake 时，会将自己添加到目标线程的 queue 中。

Glommio 中的另一种参考实现：

前面说的方案是跨线程传递 waker，可以通用支持 channel、mutex 等数据结构。

还可以不传递 waker，poll 的时候将 waker 加入本线程的数据结构，然后发送端有数据后并不是直接唤醒接收端的 waker，而是直接唤醒它所在的线程，由对端线程轮询所有存在 waker 的 channel。

这种轮询的方式在某些场景下不够高效，且方案并不通用。

## Executor 设计
Executor 在 thread per core 下按理说应该非常简单：
1. 直接做一个 Queue 然后从一端推任务，从另一端消费任务
2. 在没任务可做时，陷入 syscall 并等待至少一个任务完成
3. 拿到完成任务后逐个处理，将 syscall 结果应用到 buffer 上，并 wake 对应任务。

在 epoll 下可能确实是这个逻辑；但是在 io-uring 下可能可以做一些额外的优化。

### 低延迟 or 减少 syscall
在我们推入 SQE 之后，我们可以当场 `submit()` 以尽快完成 syscall，降低延迟；也可以先等等，等到无事可做时再 `submit_and_wait(1)`。考虑到尽可能高性能，我们选择第二种方案（Glommio 和 Tokio-uring）——测试数据反映事实上延迟并不高，相比 Glommio 延迟有时反而因为 CPU 利用率降低而更低。在负载相对较低时，也可以采用一些动态的方式决定是否更激进地 `submit`。

### 饥饿问题
在这个 case 下，饥饿问题往往是用户代码写出了问题导致的。考虑下面几个场景：
1. 用户的 Future 每次 `poll` 都会 spawn 一个新任务，然后返回 Ready。
2. 用户的 Future 每次都会立刻 wake。
3. 用户的 Future 中的状态转换太多了，或者状态转换出现了死循环。

如果我们选择在没有任务时再处理 IO（包括提交、等待和收割），那么这几个场景下，依赖 IO 的任务都无法得到处理，因为任务队列永远不会为空，或者任务永远执行不完。

对于问题 1 和 2，我们[提出了一个做法](https://github.com/bytedance/monoio/pull/3)，与其执行完所有任务，我们设置一个执行上限，当达到上限时强制做一次提交和收割。

对于问题 3，可以类似 Tokio 做一个 coop 机制，限制递归 `poll` 执行的次数来做到限制状态转换的目的。

# 打个广告
[Monoio](https://github.com/bytedance/monoio) 目前仍处于非常不完善的阶段，期待你的贡献：）

另外，我们还搭建了一个国内的 crates.io 和 rustup 的镜像，欢迎使用 [RsProxy](https://rsproxy.cn/) ！


