# 社区热点

## 【视频】Miguel Ojeda 分享 Rust for Linux  

Linux 内核维护者 Miguel Ojeda 近日在 Linaro Connect 虚拟会议上介绍了 Rust for Linux 的工作进展。Miguel Ojeda 受雇于谷歌，专门负责 Rust for Linux 的开发工作，也是主要开发者之一。

据介绍，现在已经有 RFC 补丁用于添加 Rust 基础设施支持和至少一个基本的虚拟驱动程序，但到目前为止，这项工作还没有真正落地。

Miguel 先是介绍了 Rust 的相关背景和特性，以及它为 Linux 内核提供的好处。后面才进入了主题 —— Rust 在内核中的支持情况。

他还提到了三个编译器后端的进展情况，`rustc_codegen_gcc` 进展最快，目前已通过了部分的 `rustc` 测试，`rustc_codegen_llvm` 是目前的主要开发项目，`Rust GCC` 预计在 1~2 年内完成。

- youtube ：[https://www.youtube.com/watch?v=VlSkZYBeK8Q](https://www.youtube.com/watch?v=VlSkZYBeK8Q)
- slides : [https://static.linaro.org/connect/lvc21f/presentations/LVC21F-317.pdf](https://static.linaro.org/connect/lvc21f/presentations/LVC21F-317.pdf)

## Rust for Linux 研讨会 9.13 ～ 9.15 

9.13 分享： 

1.  High Velocity Kernel Modules in Rust with Bento —— Samantha Miller

Bento 是  Safe Rust 实现的 linux 内核 文件系统。 通过实现安全的 API 并使用安全的内核函数包装器，文件系统是用安全的 Rust 编写的。 这些安全接口尽可能接近现有的用户空间（主要是标准库）接口，因此只需将 Bento 包含更改为用户空间 Rust 库和/或 bento_utils 包含，就可以将文件系统重新编译为 FUSE 文件系统。更多信息：[https://github.com/smiller123/bento](https://github.com/smiller123/bento)

这次分享主要讨论 Bento ，用于加速 Linux 内核开发的框架。目前已经为文件系统模块实现了Bento框架，并利用它实现了一个性能类似于ext4的文件系统，可以在不卸载的情况下进行升级，而且只需要15ms的停机时间。我们目前正在努力扩展Bento，以支持自定义的TCP/IP堆栈。

2.  How can we formally verify Rust for Linux? —— Alastair Reid

在Linux中使用Rust的目的是创建更稳固、更安全的代码：通过利用Rust的语言特性和设计一个安全的API供驱动程序使用，避免内存安全问题和并发问题。该分享研究了我们如何/是否可以利用自动形式验证工具走得更远。

9.14 分享：

1.  Rust key concepts for the Linux kernel (I) —— Miguel Ojeda

介绍   在 Linux  内核领域应用 Rust 的一些关键概念，第一部分 

2.  The Thread wrapper for Rust in Linux kernel — Boqun Feng （冯博群）

内核线程是内核中最重要的组件之一，它也是实现内核中其他核心子系统的必要环节。本专题将分享如何在Linux内核中实现Rust类线程的封装器的学习过程，以及目前的状况和未来的工作。

3. Implementing the Iterator trait for seq_file —— Adam Bratschi-Kaye

内核中的seq_file接口允许通过实现一个迭代可以打印的值的接口来轻松创建虚拟文件。这似乎应该直接转化为Rust的Iterator特性，其中Item实现了Display，但当然，魔鬼在细节中。该分享将展示如何为Rust代码提供seq_file的接口。

9.15 分享

1. Rust key concepts for the Linux kernel (II) —— Wedson Almeida Filho

介绍   在 Linux  内核领域应用 Rust 的一些关键概念，第二部分

2.  Writing an embedded SPI-based Linux driver in Rust ——  Arthur Cohen, Esteban Blanc, Martin Schmidt


虽然Linux主要不是一个以嵌入式为重点的操作系统，但它仍然被用于诸如Raspberry Pi这样的平台。在这些平台上，内核模块提供了一种有用的方式，可以在内核层面与各种设备进行交互，这些设备通常使用低级协议进行通信，如SPI或I2C。

在这种工作负载中使用Rust有很多优势，虽然这些协议的内核API已经被尝试和测试了很长时间，但目前还没有Rust的抽象。

在该分享中，将谈论在ARM64平台上为Linux带来一个安全的Rust的SPI协议的抽象，以及如何使用它来实现一个简单的设备驱动程序。该分享将与C语言的原始实现进行比较，后者提供了同样多的功能。最后，将深入探讨所使用的技术和他们使用Rust-for-Linux的经验。

以上三天的研讨会，应该是线下的，因为并没有提供线上参与链接。

对此话题感兴趣的可以关注：[https://github.com/Rust-for-Linux/linux](https://github.com/Rust-for-Linux/linux)，

也可以登记参加在线讨论： [https://rust-for-linux.zulipchat.com ](https://rust-for-linux.zulipchat.com )


研讨会官网： [https://kangrejos.com/](https://kangrejos.com/)

## rcore tutorial  book 更新了

  rCore-Tutorial-Book 在暑假期间又进行了一轮修改，算是从0.35版进化到0.50版了。请对学习用Rust写OS感兴趣的朋友看看。如果有问题、建议，发现了bug，请直接在每节下方的交互窗口留言。如果想一起来参与写作，请直接联系 陈渝或吴一凡。谢谢！ 本书定位是以尽量简单的编程和尽量少的OS/CPU知识来逐步设计实现一个一个的小OS，让学生知道操作系统的概念的实际体现和操作系统的全貌。经过我们讨论，虽然这本书是基于单处理器讲解的，但觉得还是要加入OS的同步互斥支持，与传统方式不同，这一章主要讲解操作系统如何支持用户态线程的同步互斥操作。所以，目前还缺的是关于同步互斥的一章，各种图，相关OS历史的介绍，相关知识点的进一步补充。争取本月完成。

[https://rcore-os.github.io/rCore-Tutorial-Book-v3/](https://rcore-os.github.io/rCore-Tutorial-Book-v3/)

## tokio-console 开发日志 #1 

tokio-console 是一个 Rust 异步调试工具，它的目标是可以让开发者更好地了解异步任务的行为方式。它基于 tracing events和spans，和运行时没有关系。

目前该库只是概念原型阶段。 官方这篇文章介绍了 console 目前开发相关进展。

[https://tokio.rs/blog/2021-09-console-dev-diary-1](https://tokio.rs/blog/2021-09-console-dev-diary-1)

carllerche’s tokio-console thoughts:

[https://hackmd.io/a4P75rj0RP6qettxAVzv8w](https://hackmd.io/a4P75rj0RP6qettxAVzv8w)

## Rust  实现的开放沙盒游戏 veloren 发布 0.11 新版本

- [https://veloren.net/release-0-11/](https://veloren.net/release-0-11/)
- [B站视频：20W+行Rust构建的游戏居然这么好玩！](https://www.bilibili.com/video/BV1mL411x7rF)

## Chrome 中 Rust 和 C++ 的两手准备


月初，Chrome 团队写了一篇文章，介绍 Rust 和 Cpp 的互操性。

Chrome的工程师们正在尝试使用Rust。在可预见的未来，C++是我们代码库中的统治者，任何对Rust的使用都需要与C++相适应，而不是相反。这似乎给C++/Rust的互操作性带来了一些挑战，而这些挑战是其他人没有遇到过的。

由于C++是统治者，该团队主要关注的是新的Rust代码能够调用现有的C++代码，而不是C++对Rust的调用。

cxx是目前最先进的在C++和Rust之间安全交换数据的方法，基本能符合该团队对 安全调用cpp的标准。但对于开发体验方面还有很多要做的事，

目前，Chrome在Rust的投资仍将是一个背景调查（主要针对这些工具和技术的原型）。如果确信这种互操作性是可能的，该团队就会重新考虑在Chrome中广泛使用Rust，到那时，他们将计划通过强大的生产质量解决方案努力实现这一目标。

[https://www.chromium.org/Home/chromium-security/memory-safety/rust-and-c-interoperability](https://www.chromium.org/Home/chromium-security/memory-safety/rust-and-c-interoperability)

九月份，谷歌发布了一篇名为在Chrome中更安全地使用C++，它有一些关于这些安全问题如何困扰Chrome和Android代码库的安全报告的链接，这部分链接内提到了Rust。

[https://docs.google.com/document/d/e/2PACX-1vRZr-HJcYmf2Y76DhewaiJOhRNpjGHCxliAQTBhFxzv1QTae9o8mhBmDl32CRIuaWZLt5kVeH9e9jXv/pub](https://docs.google.com/document/d/e/2PACX-1vRZr-HJcYmf2Y76DhewaiJOhRNpjGHCxliAQTBhFxzv1QTae9o8mhBmDl32CRIuaWZLt5kVeH9e9jXv/pub)

## Kraken 使用 Rust 改进基础设施

Kraken 一家总部位于旧金山的公司，是世界上最大的基于欧元交易量和流动性的全球数字资产交易所。

Kraken 的后端服务早期是使用 PHP 实现的，在最近两年时间，使用 Rust 改造原有的 PHP 服务并开发新的产品和功能，并使用 Rust 支撑了不断扩大的加密货币交易业务。

Kraken 发布了关于使用 Rust 改进基础建设的一篇文章，并希望对于考虑使用 Rust 构建产品的公司和想要投入时间学习该语言的开发人员来说是一个有用的资源。同时，Kraken 还表示，为了感谢 RustAnalyzer 的出色工作，将会向该项目捐赠 50K EUR 。

[https://blog.kraken.com/post/7964/oxidizing-kraken-improving-kraken-infrastructure-using-rust/](https://blog.kraken.com/post/7964/oxidizing-kraken-improving-kraken-infrastructure-using-rust/)

## WGPU 与 Deno 在 CTS 上的成功合作

三月份的时候，Deno 1.8 就已经使用 wgpu 来提供了初始的 WebGPU 支持。Deno 团队花了更多的时间将一致性测试套件（CTS）连接到 Deno WebGPU 运行，并报告了 wgpu 上第一个 CTS 结果/问题。

现在该工作已经与 wgpu CI 集成，作为 wgpu 基础设施和生态系统的一部分。

[https://gfx-rs.github.io/2021/09/16/deno-webgpu.html](https://gfx-rs.github.io/2021/09/16/deno-webgpu.html)


## Linux Plumbers Conference 2021 有 9 个 topic 与 Rust 相关

Linux Plumbers Conference 是一个面向开源社区的开发者会议，将以下场景的顶级开发者汇聚在一起：内核子系统，核心库， 窗口系统等等。

Linux Plumbers Conference 2021 有 9 个 topic 与 Rust 相关:

- [Rust in the Linux ecosystem - Miguel Ojeda](https://youtu.be/ORwYx5_zmZo?t=1749)
- [rustc_codegen_gcc: A gcc codegen for the Rust compiler - Antoni Boucher](https://youtu.be/ORwYx5_zmZo?t=5259)
- [GCC Front-End for Rust - Philip Herron](https://youtu.be/ORwYx5_zmZo?t=8902)
- [Rust for Linux - Miguel Ojeda](https://youtu.be/mF10hgVIx9o?t=12247)
- [Improving the eBPF Developer Experience with Rust - Alessandro Decina, Dave Tucker](https://youtu.be/xj0PBFjLm1U?t=8701)
- [Testing in-kernel Rust code - Miguel Ojeda](https://youtu.be/Y_minEhZNm8?t=13094)
- [Android drivers in Rust - Wedson Almeida Filho, Miguel Ojeda](https://youtu.be/O_lCFGinFPM?t=11002)
- [The Rust toolchain in the kernel -Miguel Ojeda](https://youtu.be/txIgZ31-RHI?t=2089)
- [Writing Grub2 modules in Rust - Daniel Axtens](https://youtu.be/JwU1_hyOzMI?t=2126)


