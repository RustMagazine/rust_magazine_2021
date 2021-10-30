---
pub_date: Tue, 31 May 2021 21:00:00 GMT
description: Rust FAQ

---

# 常见问题汇总

> 说明：
>
> 常见问题汇总来自于：
> - [StackOverflow - Rust](https://stackoverflow.com/questions/tagged/rust?tab=Votes)
> - [知乎 - Rust](https://www.zhihu.com/topic/19674381/hot)
> - [https://rust-zh.github.io/faq/](https://rust-zh.github.io/faq/)


---

## 为什么go仅仅160M的安装包就可以编译程序？而rust却还需要几个G的msvc才能编译？

> 来源：[知乎](https://www.zhihu.com/question/458838401)

> 在windows下，我下载了一个go1.16.zip版，仅仅150M，解压后，都不需要安装，就可以用命令行编译程序，非常小巧。但是，为什么rust，在安装了自身安装包后，却还得要vstudio build，安装得好几个G！为什么都同样是编译程序，差距如此之大？我的疑问是差距在哪里，在编译方面？难道rust不能设计成象go这样，不依赖msvc？

### 回答：

**什么是 MSVC ？**

the Microsoft Visual C++ compiler and libraries toolset 。微软 Visual C++ 编译器和库工具集。

**为什么要有 MSVC ？**

MSVC的目标是成为Windows上针对Windows的最佳编译器选择，无论您选择使用哪种编辑器或IDE。 我们旨在通过在以下关键领域上进行持续投资来实现我们的目标：完全符合C ++，更好的错误和警告，运行时性能，可靠性，构建吞吐量和最佳安全性。

**Rust 为什么要支持 MSVC 以及如何支持 ？**

[Windows - The rustup book](https://link.zhihu.com/?target=https%3A//rust-lang.github.io/rustup/installation/windows.html)

因为 Rust 要支持 Windows 开发，而 Windows 上面有两类 ABI ：

- Visual Studio使用的本机（MSVC）ABI
- GCC工具链使用的GNU ABI。

你需要哪种版本的Rust很大程度上取决于您要与哪些C / C ++库进行互操作：

- 要与Visual Studio生产的软件互操作，请使用Rust的MSVC版本；
- 要与使用MinGW / MSYS2工具链构建的GNU软件互操作，请使用GNU构建。

以MSVC ABI为目标时，Rust还需要安装Visual Studio 2013（或更高版本）或Visual C ++ Build Tools 2019，以便rustc可以使用其链接器。 对于Visual Studio，请确保选中“ C ++工具”和“ Windows 10 SDK”选项。 基本使用GNU构建不需要额外的软件安装。

如果你计划仅使用 Rust 库和可以与 MinGW 一起构建和使用的各种开源库，则可以完全避免使用Microsoft 的工具。 只需将 Rust 切换到x86_64-pc-windows-gnu目标即可。

但是，如果你打算使用本地Windows库进行Windows开发，则需要Microsoft的链接器，并且应该使用 Rust 的 x86_64-pc-windows-msvc 目标与之兼容。 由于Windows上的大多数人都对Windows开发感兴趣，因此这是默认设置。

**而 Rust却还需要几个G的 msvc 才能编译？**

因为找不到人实现必要的功能，不得不依赖Windows sdk 。

**go 为什么不需要 MSVC ？**

因为go根本没做 msvc 的支持（不是默认支持，需要你自己手工再做处理），你用 cgo 只支持用 GCC。 （不晓得现在支持 msvc 没有）。

**为什么 Rust 不能像 go 那样，不依赖 msvc ？**

所以，你觉得呢？

这正是 Rust 和 Go 设计目标的差别了。

Rust 语言是一个通用的系统级语言，Go 语言则不是这个目标。所以 Go 可以不依赖 MSVC ，Rust 则不可以。

未来，Rust 将越来越倾向于使用 LLVM LLD 链接器，而不是平台本机链接器（Microsoft或GNU binutils）。 LLD 是通用的，旨在支持所有平台和ABI。 因此，未来预计将不需要任何其他工具。

---

## 为什么Rust中的String不能用整数下标进行切片？

> 来源：[知乎](https://www.zhihu.com/question/458788810)

    ```rust
    fn main() {
        let name: String = "ABCD".to_string();
        println!("{}", &name[2..3]);
    }
    // 为什么不能直接用 name[2] 或者 &name[2] ？
    ```

> 另外，为什么切片还要加&？

### 回答：

[std::string::String - Rust](https://doc.rust-lang.org/std/string/struct.String.html#utf-8)

文档里说的很清楚了。

Rust 里 String 总是按 UTF-8 编码的。而索引旨在进行恒定时间操作，但是UTF-8编码不允许我们执行此操作。 另外，索引应返回哪种类型呢？字节，码点 还是 字素簇(grapheme cluster)。

实际上String 文档里帮你定义基于 字节和字符 处理的两类基本方法。

切片为啥加 & ，因为 Rust 里 切片 是一个 DST，必须放 & 后面。

---

## 错误处理推荐使用什么库？

目前一般认为对于应用程序推荐使用 [anyhow]，而对于库推荐使用 [thiserror]。

anyhow 提供了一个基于[特质对象]的错误类型，可以很容易地将不同来源的错误统一到单一来源，并可以方便地为错误添加上下文，以及就地创建新的错误。

thiserror 则提供了一个 derive 宏，方便为自定义的错误类型实现 [`Error` 特质][error-trait]。

[anyhow]: https://crates.io/crates/anyhow
[thiserror]: https://crates.io/crates/thiserror
[error-trait]: https://doc.rust-lang.org/std/error/trait.Error.html


## # `fn()` 类型与 `Fn()` 等特质的关系和区别是什么？

在 Rust 中，每一个函数，无论是由 `fn` 关键字定义的一般函数，还是由闭包表达式定义的闭包，都有一个各自独立的匿名类型。为了能间接地使用函数，Rust 准备了两种方式，即 [`fn()`][fn] 类型与 [`Fn()`][Fn-trait]、[`FnMut()`][FnMut-trait] 和 [`FnOnce()`][FnOnce-trait] 等[特质]。

要表达不同的类型，最常见的方法即是使用特质（作为类型约束，即 `T: Fn()` 和 `impl Fn()`，或者使用[特质对象]，即 `dyn Fn()`），`Fn()` 一族就是用于表达函数类型的特质。

`fn()` 不是一个特质，而是一个具体的类型，表示一个函数指针。功能上它与特质对象类似，可以近似地看作 `&'static dyn Fn()`。但 `fn()` 与 `Fn()` 不同，它不包含对上下文的引用，因而只有一般函数或没有捕获任何上下文的闭包能够被转换成 `fn()`。因此它也与 `&dyn Fn()` 不同，不需要使用[胖指针]。它的大小与普通的指针一致。

因为 `fn()` 是一个函数指针，通过它调用函数与通过特质对象一样是间接调用，而使用 `Fn()` 等特质约束的泛型则是通过[单态化]来直接调用的。


[fn]: https://doc.rust-lang.org/std/primitive.fn.html
[Fn-trait]: https://doc.rust-lang.org/std/ops/trait.Fn.html
[FnMut-trait]: https://doc.rust-lang.org/std/ops/trait.FnMut.html
[FnOnce-trait]: https://doc.rust-lang.org/std/ops/trait.FnOnce.html

## # Rust 的 `Future` 是基于轮询的，这种方式不会有性能问题吗？

`Future` 的轮询是带通知机制的轮询，与传统意义上的轮询不完全一样。

当[执行器](<> "executor")调用 `Future` 的 [`poll`][poll] 方法时会传入一个 [`Waker`][waker]，而 `Future` 可以将这个 `Waker` 保存起来，当自己的状态有所变化时，通过其通知执行器可以再次对自己进行轮询。通过这个机制，执行器可以避免反复轮询一个未准备好的 `Future`，避免了传统轮询带来的性能问题。


[poll]: https://doc.rust-lang.org/std/future/trait.Future.html#tymethod.poll
[waker]: https://doc.rust-lang.org/std/task/struct.Waker.html

## 标准库的 `Future`、futures crate、tokio 和 async-std 等之间的关系是什么？

标准库的 [`Future`][future] [特质]以及相关的 [`Context`][context]、[`Pin`][pin]、[`Waker`][waker] 等是核心。由于编译器编译[异步函数]需要依赖它们的定义，因而它们必须被包含在标准库里。

[futures] 是 `Future` 的扩展，提供了许多虽不必进入标准库但依然重要的基础性的东西，比如 [`FutureExt`][future-ext]、[`StreamExt`][stream-ext] 等扩展特质和基础的[通道][channel]、[执行器][executor]实现等。

[tokio] 和 [async-std] 是同一个层次的，主要提供异步运行时的实现，都依赖 futures 提供的元语，但因为处理的层次不同，所以可以看到一些自定义的与 futures 差不多的模块。

此外，虽然目前 [`Stream`][stream] 是由 futures 提供的，但未来如果编译器要实现[异步生成器][generator]，这个特质也很可能会进入标准库，因而对其的扩展也依然放进了独立的 `StreamExt` 里。


[future]: https://doc.rust-lang.org/std/future/trait.Future.html
[context]: https://doc.rust-lang.org/std/task/struct.Context.html
[pin]: https://doc.rust-lang.org/std/pin/struct.Pin.html
[waker]: https://doc.rust-lang.org/std/task/struct.Waker.html

[futures]: https://crates.io/crates/futures
[future-ext]: https://docs.rs/futures/0.3/futures/future/trait.FutureExt.html
[stream]: https://docs.rs/futures/0.3/futures/stream/trait.Stream.html
[stream-ext]: https://docs.rs/futures/0.3/futures/stream/trait.StreamExt.html
[channel]: https://docs.rs/futures/0.3/futures/channel/index.html
[executor]: https://docs.rs/futures/0.3/futures/executor/index.html

[tokio]: https://crates.io/crates/tokio
[async-std]: https://crates.io/crates/async-std

[generator]: https://rust-lang.github.io/rfcs/2394-async_await.html#generators-and-streams "async generator"


