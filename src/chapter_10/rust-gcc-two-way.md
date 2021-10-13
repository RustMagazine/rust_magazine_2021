# 【Linux Plumbers 大会总结】Rust 和 GCC 整合的两种方式

> 原文 [https://lwn.net/SubscriberLink/871283/c437c1364397e70e/](https://lwn.net/SubscriberLink/871283/c437c1364397e70e/)
>
> 作者：  **Jonathan Corbet** 

在`C`或`C++`等语言中工作的开发者可以使用两种相互竞争的编译器: `GCC`和`LLVM`。 它们中的任何一种通常都可以完成工作。不过，`Rust` 的开发者目前只能使用基于`LLVM`的`rustc`编译器。虽然`rustc`工作得很好，但开发者也有合理的理由希望有一个替代品。事实证明，有两种不同的方法可以使用`GCC`编译`Rust`，虽然目前都还没有准备好。这两种方法的开发者都来到了[2021年的 Linux Plumbers 大会](https://linuxplumbersconf.org/)，介绍他们的工作状况。

> `rustc_codegen_gcc` 是为 Rust 编译器添加 GCC 后端；`Native GCC` ，是为 `GCC` 前端 添加` Rust `语言实现。 

## rustc_codegen_gcc

首先是 Antoni Boucher，他讲述了 [rustc_codegen_gcc](https://github.com/antoyo/rustc_codegen_gcc) 项目。他开始说，`rustc`编译器是基于`LLVM`的，除此之外，这意味着它不支持`GCC`所支持的所有体系结构。`LLVM`有的，`rustc`也有，但它有一个`API`，可以插入一个替代的代码生成器。这个`API`可以用来通过[`libgccjit`](https://gcc.gnu.org/onlinedocs/jit/)插入`GCC`的代码生成机制。这就是 `rustc_codegen_gcc` 所采取的方法。

为什么这会是一件有用的事情呢？Boucher说，`Rust` 语言越来越受欢迎，但它需要对`LLVM`所能提供的更多架构的支持。特别是`Rust for Linux`的工作突出了这个问题，但也有很多其他用户在那里。嵌入式系统的开发者会从更好的架构支持中受益，`Firefox`浏览器也是如此。

`rustc_codegen_gcc`现在支持许多`Rust`特性，包括基本和聚合类型、变量、函数、原子类型、线程本地存储、内联汇编、许多内部函数等等。该编译器在 [Compiler Explorer](https://godbolt.org/) 中得到了支持。`libcore`测试通过了，大部分的用户界面测试也通过了。作为一个实验，这个编译器已经被用来为`m68k`架构构建`Rust`代码；这项工作仍处于早期阶段，Boucher 说，但它表明确实有可能为`LLVM`不支持的平台构建`Rust`程序。

仍然有一些问题需要解决。一些属性仍然需要支持，调试信息的生成也是如此。生成的代码质量并不总是最好的。必须做更多的工作来支持新的体系结构。还不支持链接时优化（`LTO`），等等。这项工作也需要对`libgccjit`进行一系列的修改，其中大部分仍在审查中。

还有一些其他的问题，包括需要使用`GCC`的补丁版本，直到所有的修改都被合并到上游。即便如此，也需要回传这些补丁，以允许使用较早的`GCC`版本，这对于编译内核是很重要的。

即便如此，这个项目似乎已经取得了合理的进展。Boucher 指出，曾经有一个[活跃的Pull Request](https://github.com/rust-lang/rust/pull/87260)，要把`rustc_codegen_gcc`添加到rustc编译器中， 它已经在 9月29日被合并了。

## Native GCC

Philip Herron 接着谈到了 `Rust`的本地`GCC`前端，也就是`gccrs`。这个编译器不是`LLVM`和`GCC`的混合体，而是`GNU`工具链中`Rust`语言的完整实现。这项工作是用`C++`编写的（更容易启动，他说），并打算成为主线`GCC`的一部分。它使用现有的`binutils`，并重新使用官方的`Rust`库（如libcore、libstd和libproc）。

演讲者再次提出了 "为什么？"的问题。他说，他喜欢大项目，所以这个项目看起来很有吸引力。它与`LLVM`中的问题解决方式形成了有趣的对比，也是一个很好的机会，可以看到`GCC`是如何处理一种现代的高级语言。一旦工作完成，在代码大小、寄存器分配和能源效率方面对结果进行比较将会很有帮助。

他说，有一个独立的`Rust`实现，会有很多好处。与`GCC`的紧密结合将对一些项目很有帮助，这些项目也将能够从`GCC`插件中受益。基于`GCC`的`Rust`编译器将使`rustc`在新平台上的启动更加容易。对链接时优化的支持应该得到改善，这种优化在混合编译器的情况下往往效果不佳。当然，`GCC`也会带来对更多目标架构的支持。

Herron 说，`GCC`支持`Rust`的工作早在 2014 年就开始了，但后来停滞不前；该语言的发展速度太快，`GCC`的开发者无法跟上它。这项工作后来在 2019 年重新启动；最近对`Rust`用于内核的兴趣正在帮助推动这个项目。包括开源安全和Embecosm 在内的多家公司正在支持以`GCC`为基础的`Rust`编译器的开发。有一个详细的计划，即在 2022 年底前发布一个 "最小可行产品(MVP) "编译器。

到目前为止，对核心数据结构和大多数控制流都有工作支持，尽管一些控制流的工作仍在进行中。泛型和特质解析工作。未来的工作包括宏、导入、未稳定的特性和内部函数。有趣的是，目前的编译器可以构建 "hello world"，但它需要使用Unsafe 的代码；缺乏宏意味着`println!()`不可用，必须调用`C`语言的`printf()`函数来代替。

计划在未来进一步开展的工作包括借用检查器，这将与[`Polonius`](https://github.com/rust-lang/polonius)项目合作完成。增量编译也在列表中，还有将前端移植到旧的`GCC`版本。从长远来看，希望这项工作能够帮助推动`Rust`编译器的兼容性测试。

这些会谈的视频可以在[YouTube](https://www.youtube.com/watch?v=ORwYx5_zmZo&t=1h27m48s)上找到。

