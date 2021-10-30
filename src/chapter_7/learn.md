# 学习资源

编辑：张汉东

---

## Rand 之书

- [英文：The Rust Rand Book](https://github.com/rust-random/book/)
- [中文：Rand 之书](https://github.com/zjp-CN/Rust-Rand-Book-zh)

## Rust 新书 ：《 Hands-on Rust 》

PragProg 出版社出的一本新书 《Hands-on Rust》，以游戏开发为主题学习 Rust 。 

[https://pragprog.com/titles/hwrust/hands-on-rust/](https://pragprog.com/titles/hwrust/hands-on-rust/)

视频介绍：[https://www.youtube.com/watch?v=DvcWrd5VJ2I](https://www.youtube.com/watch?v=DvcWrd5VJ2I)

## 《Rust for Rustaceans》 样章试译 | 第二章 Rust 基础

本文是对 Jon Gjengset 写的新书 《Rust for Rustaceans》样章第二章的中文试译初稿。出于对 Jon 的尊敬，以及想了解 Jon 眼中的 Rust ，我打算翻译一下这本书。发出来让大家看看翻译效果，欢迎指正。

[https://mp.weixin.qq.com/s/PW7UQ1QpolXeXQTgJGblog](https://mp.weixin.qq.com/s/PW7UQ1QpolXeXQTgJGblog)

## 曼宁新书推荐：  《Refactoring to Rust》

这本书试图教会你，如何把现有项目用 Rust 语言重构。 比较适合想在公司里推广 Rust 的朋友。

[https://www.manning.com/books/refactoring-to-rust](https://www.manning.com/books/refactoring-to-rust)

## 在 R 语言中调用 Rust 

[https://extendr.github.io/rextendr/](https://extendr.github.io/rextendr/)

## TezEdge: 使用 nom 加速二进制解析

Tezos 是一个开源去中心化区块链网络，为智能合约和数字资产提供平台。 之前 Tezos 节点使用 serde 来序列化/反序列化二进制，但是这样始终维护着一个中间结构，占用了 CPU 和 内存。所以他们使用 nom 直接对二进制流进行解析，就消除来这个中间结构，提升了性能。

- [https://medium.com/tezedge/speeding-up-incoming-message-parsing-by-3-to-10-times-by-switching-from-serde-to-the-nom-library-a74b04391bb4](https://medium.com/tezedge/speeding-up-incoming-message-parsing-by-3-to-10-times-by-switching-from-serde-to-the-nom-library-a74b04391bb4)
- [https://github.com/tezedge/tezedge](https://github.com/tezedge/tezedge)

## Rust 概念解惑 | Deref vs AsRef vs Borrow vs Cow 

- 英文：[https://dev.to/zhanghandong/rust-concept-clarification-deref-vs-asref-vs-borrow-vs-cow-13g6](https://dev.to/zhanghandong/rust-concept-clarification-deref-vs-asref-vs-borrow-vs-cow-13g6)
- 中文：[Rust 概念解惑 | Deref vs AsRef vs Borrow vs Cow ](https://mp.weixin.qq.com/s/OdcLb5U8QCeYH08feThN7w)

## arithmetic-parser: 一个多功能的算术表达式解析器

此项目可以作为 Rust 学习案例

[https://github.com/slowli/arithmetic-parser](https://github.com/slowli/arithmetic-parser)

## Bevy 引擎游戏开发指南系列视频

该指南是利用 Bevy 引擎开发一个好玩的类似小蜜蜂的游戏。

[https://www.youtube.com/channel/UCiT_r1GD7JSftnbViKHcOtQ](https://www.youtube.com/channel/UCiT_r1GD7JSftnbViKHcOtQ)

## Rust 如何和 FFI 的数据打交道

该博客主要是简单展示如何通过 FFi 进行数据交互

[https://blog.guillaume-gomez.fr/articles/2021-07-29+Interacting+with+data+from+FFI](https://blog.guillaume-gomez.fr/articles/2021-07-29+Interacting+with+data+from+FFI)

## 与c相比，如何改善Rust巨大的二进制大小？

如果你想尽可能的减少Rust生成的二进制文件大小，请看这里：

- [https://github.com/johnthagen/min-sized-rust](https://github.com/johnthagen/min-sized-rust)
- [https://blog.mgattozzi.dev/rusts-runtime/](https://blog.mgattozzi.dev/rusts-runtime/)

## 【讨论】为什么说 OpenSSL 是 Rust 开发的痛点？

来自 reddit 的讨论，摘录一段评论：


在Node.JS和Python中，TLS的实现是来自运行时的。有人已经为你处理了这个令人头痛的问题。缺点是，当你写一个NodeJS或Python程序时，你需要确保你的用户也安装了该运行时。

Rust没有这样的运行时。当你运行你的Rust程序时，该程序必须知道如何以某种方式处理TLS。

那么`openssl-sys`与其他 crate，如`serde`、`clap`等有什么不同？

你从 crates.io 上找到的大多数crates都是纯粹的Rust crates，它们是用Rust工具链编译的，（通常）静态链接到你的可执行文件中。这是一个相对简单的过程，只依赖于Rust编译工具，反正你已经设置好了。

`openssl-sys`（像大多数其他的-sys板块）不是一个纯粹的Rust crate。它依赖于实际的OpenSSL库，它是一个C/C++库。这意味着还有许多其他方面需要考虑。

OpenSSL是静态链接还是动态链接到最终可执行文件？

在哪里找到/如何为链接器编译必要的文件？

这里没有适用于所有用户的开箱即用的解决方案。通常你需要下载带有必要文件的OpenSSL二进制分布，并以某种方式通知`openssl-sys`在哪里提取这些文件，或者你需要设置OpenSSL构建工具链（C/C++编译器等），然后让`openssl-sys`为你从头编译 OpenSSL。

[https://www.reddit.com/r/rust/comments/oto406/why_is_ssl_such_a_pain/](https://www.reddit.com/r/rust/comments/oto406/why_is_ssl_such_a_pain/)

## Local Native系列教程完结了

Local Native是一个跨平台的Rust写的桌面应用程序，使用的UI框架是iced，教程记录了如何搭建整个程序的过程，因为本人水平有待提高，因此教程肯定会有不少问题，希望对此感兴趣的朋友可以提出问题，我会尽可能解决。这个项目会长期维护，知道iced等重要依赖达到1.0稳定版本。

上次收到的反馈有注释过长之类的，在新版本中有了改正，非常感谢大家的反馈。

- 项目总结：[https://localnative.app/blog/2021/07/28/localnative-2021-soc-retrospective-blog](https://localnative.app/blog/2021/07/28/localnative-2021-soc-retrospective-blog)
- 0-9章：[https://localnative.app/docs/tutorial0](https://localnative.app/docs/tutorial0)

## Prechelt 论文 | 比较Rust、Java、Lisp、C/C++ 和脚本语言

文章主要介绍了关于编程语言对生产力和程序效率的影响的讨论。

[https://renato.athaydes.com/posts/revisiting-prechelt-paper-comparing-languages.html](https://renato.athaydes.com/posts/revisiting-prechelt-paper-comparing-languages.html)

## Rust 实现 esp8266 驱动程序

通过串口与esp8266 模块通信。通过这个模块，可以加入现有的接入点或创建自己的接入点。创建网络后，该模块既可以侦听传入的 TCP 连接，也可以连接到其他套接字。

[https://github.com/alekseysidorov/esp8266-wifi-serial](https://github.com/alekseysidorov/esp8266-wifi-serial)

## Rust 中那些超棒的 Unstable 特性

本文讲解了Rust编译器的一些超棒的未稳定特性，并且通过例子来使读者了解这些未稳定特性的基本用法和情况。

[https://lazy.codes/posts/awesome-unstable-rust-features/](https://lazy.codes/posts/awesome-unstable-rust-features/)

## 深入理解 Rust 的 Features

[https://fasterthanli.me/articles/understanding-rust-futures-by-going-way-too-deep](https://fasterthanli.me/articles/understanding-rust-futures-by-going-way-too-deep)

## 如何在 Rust 中实现工作池

单纯看到 Rust 所有权模型，工作池模式会不太适合 Rust。但是相反，拥抱函数式编程和不可变数据使 Rust 拥有了更简单易用和更优雅的工具：并行迭代器和流。作者提供了 #计算密集型# 和 #I/O 密集型# 两种作业的工作池使用方式。

[https://kerkour.com/blog/rust-worker-pool/](https://kerkour.com/blog/rust-worker-pool/)

## Rust 和 JVM

通过 jni 让 rust 和 Java 结合起来，可以在 Java 中使用 rust function

[https://blog.frankel.ch/start-rust/7](https://blog.frankel.ch/start-rust/7)

## 如何 在 yew 框架 和 Trunk 中使用 Tailwind CSS

- [https://dev.to/arctic_hen7/how-to-set-up-tailwind-css-with-yew-and-trunk-il9](https://dev.to/arctic_hen7/how-to-set-up-tailwind-css-with-yew-and-trunk-il9)
- [https://github.com/thedodd/trunk](https://github.com/thedodd/trunk)
- [https://github.com/yewstack/yew](https://github.com/yewstack/yew)
- [https://tailwindcss.com/](https://tailwindcss.com/)

## Rust + Tauri + Svelte 指南

[https://jbarszczewski.com/rust-tauri-svelte-tutorial](https://jbarszczewski.com/rust-tauri-svelte-tutorial)

## 系列： 在树莓派上跑 wasm 模块

[https://blog.knoldus.com/host-a-wasm-module-on-raspberry-pi-easily-part-1/](https://blog.knoldus.com/host-a-wasm-module-on-raspberry-pi-easily-part-1/)

## Fluvio SmartStreams:： 为应用程序日志编写 wasm 过滤器

[https://www.infinyon.com/blog/2021/06/smartstream-filters/](https://www.infinyon.com/blog/2021/06/smartstream-filters/)

## 用 Rust 重写 go 项目，最开始慢 4～5 倍，然后经过Rust 社区网友支招优化以后，反过来比 go 项目快且稳定

这个帖子下涉及一些异步优化技巧可以看看

[https://www.reddit.com/r/rust/comments/oje3w7/rewrote_golang_project_in_rust_its_4x_times/](https://www.reddit.com/r/rust/comments/oje3w7/rewrote_golang_project_in_rust_its_4x_times/)

## Discourse 论坛 ： 使用 Rust 、WebAssembly 和  MozJPEG 加速上传


[https://blog.discourse.org/2021/07/faster-user-uploads-on-discourse-with-rust-webassembly-and-mozjpeg](https://blog.discourse.org/2021/07/faster-user-uploads-on-discourse-with-rust-webassembly-and-mozjpeg)

## Rust 实现的轻量级浏览器，用于教学目标

[https://github.com/lmt-swallow/puppy-browser/](https://github.com/lmt-swallow/puppy-browser/)

## 如何在 Windows 上进行 Rust 开发

微软官方指南，在 Windows 上使用 Rust 进行开发，包括开发环境的设置、Windows 的 Rust 和代码示例。

[https://docs.microsoft.com/en-us/windows/dev-environment/rust/](https://docs.microsoft.com/en-us/windows/dev-environment/rust/)

## hyperfunctions: PostgreSQL 中的函数，用于简化时序数据的工作
Rust 实现的 postgresql 扩展。能简化开发工作。

其中提到一个理念：创建新的 SQL 函数，而不是重新发明语法。值得品味。

[https://blog.timescale.com/blog/introducing-hyperfunctions-new-sql-functions-to-simplify-working-with-time-series-data-in-postgresql/](https://blog.timescale.com/blog/introducing-hyperfunctions-new-sql-functions-to-simplify-working-with-time-series-data-in-postgresql/)

## 并不总是iCache的原因

摘要： 

众所周知，内联是一项很有用的代码优化技术。但有时候我们也常听到类似下面这样的观点：

内联也会使代码变慢，因为内联会增加代码的大小，使指令缓存变大并导致缓存未命中。

对于内联导致代码运行速度变慢的原因，不少人归结为内联使得iCache命中率降低，造成指令流水线在取指的过程中耗费更多的时间。

本文作者通过使用若干有效的工具(perf/valgrind)对比了是否启用内联特性的代码性能和汇编指令，提出了内联可能造成代码变慢的另一种解释：

试考虑内联技术将函数S内联展开于函数C中：

1. 内联使得C占用了更多的寄存器。由于函数S的代码直接在函数C的函数体中展开，造成函数C在程序上下文切换过程中加入了更多的push/pop指令，并且函数C的运行时栈的空间进一步膨胀。与内联版本中每次调用函数C都意味着这些新增的push/pop指令都会运行不同，未内联版本的push/pop指令只存在于函数S的上下文中，并且只有当函数C确实调用函数S时，这些指令才会被运行；
2. 基于第一点的基本认识，现在设想函数S在流程控制语句中被调用（循环或条件分支等），编译器可能会提升函数S中的某些指令到条件分支之外，造成这些指令从冷路径变为热路径（冷热路径：因为条件分支可能不会执行，但是位于条件分支之外的代码总会执行，是为热路径）；
3. 在上述场景中，随着外层函数C的栈中局部变量和流程控制语句增多，编译器的优化反而使得热路径执行效率降低。

- [https://matklad.github.io/2021/07/10/its-not-always-icache.html]
- [https://matklad.github.io/2021/07/09/inline-in-rust.html](https://matklad.github.io/2021/07/09/inline-in-rust.html)
- [https://www.scylladb.com/2017/07/06/scyllas-approach-improve-performance-cpu-bound-workloads/](https://www.scylladb.com/2017/07/06/scyllas-approach-improve-performance-cpu-bound-workloads/)

## 一个 Rust 编写的 k8s controller

作者为了学习 k8s 的 controller 以及 Rust, 用 Rust 编写了一个 controller.

[https://blog.frankel.ch/start-rust/6/](https://blog.frankel.ch/start-rust/6/)

## Rust 在前端中的使用

这其实是一个手把手教你如何使用 WebAssembly 的教程.

[https://blog.frankel.ch/start-rust/5/](https://blog.frankel.ch/start-rust/5/)