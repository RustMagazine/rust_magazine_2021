# 学习资源

编辑： 张汉东

---

## 《Rust In Action》书籍第一版发布

作何 Tim McNamara 是一位经验丰富的程序员，对自然语言处理、文本挖掘以及更广泛的机器学习和人工智能形式有着浓厚的兴趣。 他在包括新西兰开源协会在内的开源社区中非常活跃。Rust in Action 是使用 Rust 进行系统编程的实践指南，它是为具有好奇心的程序员编写的，提供了远远超出语法和结构的实际用例。

国外最近的Rust的书籍，除了《Rust In Action》还有另外两本，《Refactor to Rust》和 《Rust Servers, Services, and Apps》。

国内翻译版也在路上了。

[Amazon](https://www.amazon.com/dp/1617294551/ref=cm_sw_r_cp_awdb_imm_VJ4HZ4859SDB7K5B7VQK)

## 使用 Rust 进行端到端加密

《End-to-End Encryption with Rust》是一本`ockam-network/ockam`实践指南， 在本指南中，我们将创建两个名为 Alice 和 Bob 的小型 Rust 程序。 Alice 和 Bob 将通过云服务通过网络相互发送消息。 在我们的代码示例中，Alice 和 Bob 将相互进行身份验证，并将获得加密保证，以确保其消息的完整性、真实性和机密性得到端到端的保护。

网络上的中间云服务和攻击者将无法看到或更改途中消息的内容。 在后面的示例中，我们还将看到即使当 Alice 和 Bob 之间的通信路径更复杂 - 具有多个传输连接、各种传输协议和许多中介时，我们如何才能实现这种端到端保护。

[https://github.com/ockam-network/ockam/tree/develop/documentation/use-cases/end-to-end-encryption-with-rust#readme](https://github.com/ockam-network/ockam/tree/develop/documentation/use-cases/end-to-end-encryption-with-rust#readme)

## 两张图展示当前 Rust Web 生态

微信： [https://mp.weixin.qq.com/s/eIOMI0JvpOkdmiTqJfWkRg](https://mp.weixin.qq.com/s/eIOMI0JvpOkdmiTqJfWkRg)
知乎： [https://zhuanlan.zhihu.com/p/398232138](https://zhuanlan.zhihu.com/p/398232138)


## 创意！用 Rust crate 作为自己的简历 

如果你觉得学习 Rust 不知道该做些什么好？那不如从做自己简历开始。

[https://yozhgoor.github.io/yohan_boogaert_1995/](https://yozhgoor.github.io/yohan_boogaert_1995/)

## Mini Lust 系列教程：

好奇如何从零造出来一个 RPC 框架？本教程将带你一步一步写出来一个 Rust 版 Thrift RPC 框架。 

1.前言部分，RPC 相关概念介绍
2. Thrift IDL 介绍
3. 序列化/反序列化的抽象
4. Codec 和 Transport 抽象
5. 客户端和服务端实现
6. Thrift IDL 解析和代码生成
7. 基于 tower 的服务发现和负载均衡
8. 中间件支持

[https://github.com/mini-lust/tutorials](https://github.com/mini-lust/tutorials)

## Rust 公开课 | 《 Rust 异步编程二: Tokio 入门运行时介绍》|Vol. 6

这节课预计 9.5 号晚上8点，感兴趣的可以去听听。

该系列课程大纲

1、回顾 Rust 异步编程模型.
2、谈谈对 Rust 异步框架的认识 ( futures-rs、async-std、tokio ) .
3、Tokio 介绍
4、Tokio 里的 Executor、Reactor、Future 如何使用.
5、使用 Tokio 实现一个简单的服务端与客户端程序.

[https://mp.weixin.qq.com/s/23YDZdwJNOAu15AIBDnWuQ](https://mp.weixin.qq.com/s/23YDZdwJNOAu15AIBDnWuQ)

## Clippy 1.54 增加 `disallowed-methods` 配置

允许你在 `clippy.toml` 中配置不允许的方法：

```rust
# clippy.toml
disallowed-methods = ["std::vec::Vec::leak", "std::time::Instant::now"]
```

不良代码：

```rust
// 该代码将要被警告

let xs = vec![1, 2, 3, 4];
xs.leak(); // Vec::leak is disallowed in the config.

let _now = Instant::now(); // Instant::now is disallowed in the config.
```

应该用此代替：

```rust
// Example code which does not raise clippy warning
let mut xs = Vec::new(); // Vec::new is _not_ disallowed in the config.
xs.push(123); // Vec::push is _not_ disallowed in the config.
```

## 5000倍速度提升的 CRDT

CRDT 全称 Conflict-Free Replicated Data types. 主要用于在线合作文档编辑等方面. 

作者详细介绍了如何提升相关实现和算法的一些过程,并且最终使得提升了 5000 倍的速度.

[https://josephg.com/blog/crdts-go-brrr/](https://josephg.com/blog/crdts-go-brrr/)

## 如何写出运行缓慢的 Rust 代码

用Rust写代码并不意味着你的代码会快得不得了。你很容易犯错并获得相当慢的性能。正如这篇博文所显示的，你甚至可能需要付出相当多的汗水才能打败Common Lisp和Java。

作者分享了自己如何使用 Rust 重写自己的 Lisp 代码, 如何成功的写出更慢的代码 并且 修复他们的故事.

[https://renato.athaydes.com/posts/how-to-write-slow-rust-code.html](https://renato.athaydes.com/posts/how-to-write-slow-rust-code.html)

## RustCast: Rust 系列教学视频

一系列 Rust 学习系列视频，希望能坚持下去。

[https://www.youtube.com/channel/UCZSy_LFJOtOPPcsE64KxDkw](https://www.youtube.com/channel/UCZSy_LFJOtOPPcsE64KxDkw)

## 用Rust重写我的手机游戏，并且编译到 wasm

作者的游戏之前是用 C++ 写的。这篇文章详细记录了他决心使用rust重写的心路历程和一些idea的发展。

推荐阅读：

[https://itnext.io/rewriting-my-mobile-game-in-rust-targeting-wasm-1f9f82751830](https://itnext.io/rewriting-my-mobile-game-in-rust-targeting-wasm-1f9f82751830)

## 使用 Rust 从头开始​​实现 Base64

文章仔细研究 Base64 算法，并使用 Rust 编程语言从头开始实现编码器和解码器。

[https://dev.to/tiemen/implementing-base64-from-scratch-in-rust-kb1](https://dev.to/tiemen/implementing-base64-from-scratch-in-rust-kb1)

## Async Rust 从头开始​​：一个简单的 Web 服务器

[https://ibraheem.ca/writings/a-simple-web-server/](https://ibraheem.ca/writings/a-simple-web-server/)

## 一个网络应用程序，可以学习使用 AI（遗传算法）构建车辆，使用Rust编写

它在你的浏览器中运行，使用人工智能（具体来说：遗传算法）来尝试制造越来越好的车辆。车辆必须克服障碍路线，从一些小山坡开始，然后是陡峭的山坡，最后是一些跳跃。车辆由面板和轮子制成，连接在一起，类似于Besiege游戏。

[https://github.com/Bauxitedev/vehicle_evolver_deluxe](https://github.com/Bauxitedev/vehicle_evolver_deluxe)

## 当零成本抽象不再是零成本
Rust 是围绕着“零成本抽象”的概念构建的。其理念是，您可以编写人机友好的高级代码，而编译器将为您提供至少与您自己编写的任何优化的低级别代码一样好的性能。使用零成本抽象，您不再需要在可维护性和性能之间进行权衡。

不幸的是，很难确保零成本抽象是真正的零成本，并且在实践中Rust经常不能满足这个崇高的理想。在这篇文章中，我将展示两个例子，在这两个例子中，即使看似简单的零成本抽象实际上也不是零成本。

[https://blog.polybdenum.com/2021/08/09/when-zero-cost-abstractions-aren-t-zero-cost.html](https://blog.polybdenum.com/2021/08/09/when-zero-cost-abstractions-aren-t-zero-cost.html)

## 【系列】Rust 每周一模块

这是一个系列博客，目前只发了两篇文章，每周讲一个模块：

比如第二周：Rust 标准库中`std::fs`模块

`std::fs` 是Rust标准库中操作文件系统的模块，包括创建、读取、更新、删除等常见操作。由于不同操作系统支持的API不尽相同，本文仅展示了与平台无关的一些例子：

- 通过修改时间(mtime)来聚合相同年份、月份乃至日期的文件；
- 硬链接(hard link)一个路径至另一个路径；
- 递归创建目录；
- 递归删除文件夹；
- 拷贝文件；

[https://motw.rs/](https://motw.rs/)

## 【书籍】Black Hat Rust 早期访问版

Black Hat Rust 是一本深入研究使用 Rust 编程语言的进攻性安全（Offensive Security）的书籍，支持PDF，Kindle 和 Epub。

这本书是一项正在进行的工作。它可以在早期访问计划的背景下使用，这意味着各章节将在写完后立即发送给你，我们非常感谢你的反馈。当前状态：

可访问页数：250+ 代码进度：~90% [https://github.com/skerkour/black-hat-rust](https://github.com/skerkour/black-hat-rust) 预计最终出版：Q3 2021 估计的页数：~320

备注：作者为感谢所有帮助其完成这本书的人，所有早期访问的买家还将获得以下奖励：一个高级恶意软件分析的策划清单。在开发自己的攻击性工具时，会在里面找到巨大的灵感。

[https://academy.kerkour.com/black-hat-rust?coupon=BLOG](https://academy.kerkour.com/black-hat-rust?coupon=BLOG)

## 如何写出高效的 Rust 代码

该文作者对如何写出高效 Rust 代码给出了一些建议，内容还比较长，感兴趣可以看看。

[https://renato.athaydes.com/posts/how-to-write-fast-rust-code.html](https://renato.athaydes.com/posts/how-to-write-fast-rust-code.html)

## 理解 `#[derive(Clone)]` 宏

你可能不知道这个宏背后发生的事，这篇文章带你探索一下。

[https://stegosaurusdormant.com/understanding-derive-clone/](https://stegosaurusdormant.com/understanding-derive-clone/)

## 使用 Rust 进行即时分词

文章思路基于 Rust 实现 Python wordsegment 库，目的是为了实现更快的即时域搜索。文章将分词实现问题分为两点：实现一种估计句子在现实世界中出现的概率的方法以及实现一种对输入字符串的所有可能分段进行评分的有效方法。让我们来讨论这两者是如何工作的：

[https://instantdomainsearch.com/engineering/instant-word-segmentation-with-rust](https://instantdomainsearch.com/engineering/instant-word-segmentation-with-rust)

## 使用 WebAssembly 实现的 2048 游戏

2048 这个游戏相比大家都不陌生，它是一个以合成 2048 数字为目的的游戏，游戏玩法只有左右上下，通过将两个 2 合并成 4，将两个 4 合并成 8，以此类推合并出 2048。

[https://2048.nishchith.com/](https://2048.nishchith.com/)

## Pin,Unpin为什么Rust需要它们

又是一篇讲Pin的blog，是作者本人在学习Rust异步过程中做的一些总结和理解，方便大家在学习异步时遇到相关疑惑可以查阅。

[https://blog.adamchalmers.com/pin-unpin/](https://blog.adamchalmers.com/pin-unpin/)

## Typing the technical interview 从Haskell翻译到Rust

Typing the technical interview是一篇将计算机知识拟作魔法的小说？原文提到的相关代码都是使用Haskell写的，现在社区里有人将其用Rust重新实现了一遍：

Github: [https://github.com/insou22/typing-the-technical-interview-rust/](https://github.com/insou22/typing-the-technical-interview-rust/)

同时，如果对这篇原文感兴趣的，链接也在这里：

Read More: [https://aphyr.com/posts/342-typing-the-technical-interview](https://aphyr.com/posts/342-typing-the-technical-interview)

## 关于Futures和运行时如何工作的心智模型

这一部分的主要目标是建立一个高层次的心理模型，说明我们在前一章中读到的不同部分是如何一起工作的。我希望这将使我们在接下来的几章中深入研究特质对象和生成器等主题之前，更容易理解高层次的概念。

这并不是创建一个异步系统模型的唯一方法，因为我们要对运行时的具体情况进行假设，而这些情况可能会有很大的不同。这是我认为最容易建立的方式，而且对于理解你在异步生态系统中发现的很多真实的实现也很有意义。

最后，请注意，由于需要简洁明了，代码本身是 "伪的"。

[https://cfsamson.github.io/books-futures-explained/2_a_mental_model_for_futures.html](https://cfsamson.github.io/books-futures-explained/2_a_mental_model_for_futures.html)

## 【系列】Embedded Rust 第一步：选择一块板子

内容整理自 [robyoung (Rob Young) ](https://github.com/robyoung)的文章：First steps with Embedded Rust: Selecting a board

有这么多令人眼花缭乱的微控制器和项目，对于嵌入式经验很少的人来说应该从哪里开始？

我们在开发板中想要什么？

- 良好的架构支持
- 良好的芯片支持
- 活跃的社区
- 内置调试器

我们需要什么架构？

拥有最完整库、最详尽指南和最大社区的架构是 ARM Cortex-M。 ARM Cortex-M 是面向微控制器应用的低功耗、低成本处理器。 查看 crates.io 上的下载量虽说不是一个完美的指标，但可以让我们了解规模上的差异。在过去的 90 天内，cortex-m 的下载量超过 250k。 RISC-V、AVR 或 Xtensa 最多有 3k 次下载，cortex-a 有大约 18k 次下载。ARM Cortex-M 独树一帜。

- AVR：AVR 是用于嵌入式系统的 8 位微控制器系列。在 Rust 生态系统中，它们并没有得到很好的支持。直到最近，还需要使用 rustc 的一个分支来构建 AVR。 现在有几个不同的选择，awesome-avr-rust 是一个很好的起点。
- ARM Cortex-A：更强大的多核 ARM 处理器，专为运行更大的东西而设计。 通常会在它们上运行完整的操作系统。 例如这是大多数智能手机和掌上游戏机中使用的架构。查看 cortex-a - crates.io: Rust Package Registry 了解更多。
- RISC-V：似乎是机器架构的新热点，它是一种免费且开放的指令集架构 (ISA)。 它也从一开始就被设计成模块化的，这意味着芯片设计人员可以创建各种各样的专用芯片，虽然目前开发板的范围很小。有一个活跃的 Rust RISC-V 社区，SiFive 或 www.riscv.org 都是不错的起点，Rust 方面，可以查看 riscv crate。
- Xtensa：最受欢迎的主板组是来自 Espressif 的 ESP32 系列芯片。它们是小型、廉价、支持 WiFi 的电路板。 需要注意的是，并非所有 ESP32 开发板都使用 Xtensa 芯片，新的 ESP32-C3 是基于 RISC-V 的。在 Xtensa 芯片上使用 Rust 的最大障碍可能是 llvm 不支持它，因此需要构建 Rust 的 fork：esp-rs/rust。

我们需要什么芯片？

因此，我们将使用 ARM Cortex-M。 这缩小了搜索范围，但仍有很多选择。如果我们查看 cortex-m crate 的依赖项，我们会看到有两组芯片比其他任何一组都使用得更多； STM32 系列芯片和 nRF5 系列，这是我们要重点搜索的地方。

STM32：STM32 系列芯片可能是应用最广泛的嵌入式 Rust ARM Cortex-M 芯片。两种最受欢迎的 STM32 板是 Blue Pill 和 Black Pill。主要的缺点是没有板载调试器。如果想要带有调试器的基于 STM32 的电路板，那么获得 STMicroelectronics 官方套件是一个不错的选择（STM32F3 或 STM32F4 是不错的选择）。Rust Embedded Discovery 书的原始版本是针对 STM32F3 板编写的，因此有非常高质量的初学者文档，可以从那里开始。

nRF5：用于嵌入式 Rust 的第二个最广泛使用的 ARM Cortex-M 芯片系列是 Nordic Semiconductor 的 nRF5 系列。官方开发套件 (DK) 是很棒的入门板。 Ferrous Systems 的 Knurling-rs 会议使用 nRF52840 开发套件。Knurling 课程质量非常高，手把手指导，通过有趣好玩的项目教授嵌入 Rust，是使用 Rust 进行嵌入式开发的最佳切入点。另一个很棒的基于 nRF 的开发板是 BBC micro:bit。它配备了板载调试器和一系列有趣的板载外围设备，如板上的 LED 显示屏、按钮和传感器。BBC micro:bit 被设计为一个教育平台，因此硬件在他们的开发者社区中以非常适合初学者的方式进行记录，并且互联网上有大量项目创意。

RP2040：RP2040 于 2020 年底发布，是 Raspberry Pi 基金会首次尝试设计自己的芯片。由于如此新，Rust 对它的支持仍在开发中。与 BBC micro:bit 一样，RP2040 旨在成为一个教育平台，因此硬件文档是一流的，并且有大量初学者友好的代码示例和其他编程语言的库（没有多少适合初学者的嵌入式 Rust 文档）。这是一个非常令人兴奋的平台，并且在 Embedded Rust 社区中围绕它进行了大量活动，所以一定要密切关注，但它可能不适合作为入门第一块板。
板载调试器？

在主机上运行程序时，可以在 shell 中运行它并查看打印输出。这在嵌入式目标上更加困难，调试器填补了这一空白。除了允许单步调试、断点调试外，它还允许将程序加载到设备上并轻松查看输出。不过有一个问题，它通常是连接到主机然后连接到目标设备的单独设备。第一次开始时，这是一笔不可忽视的费用，也是必须正确设置的另一件事。幸运的是，有些设备带有内置调试器，将它们直接插入主机并在瞬间探测运行的代码（通常需要在主机上进行一些设置才能使调试器正常工作，ferrous 有一个很好的设置指南）。

结论

以下这些板都有很棒的 HAL 和 BSP crate、活跃友好的社区和板载调试器。

- BBC micro:bit（约 13 英镑）：它是新版 Rust Embedded Discovery 书中使用的板。
- nRF52840 开发套件（约 35 英镑）； 它是 Ferrous Systems 在 Kunrling 会议和培训中使用的板。
- STM32F3 探索套件（约 14 英镑）； 它是 Rust Embedded Discovery 书的第一版中使用的板。

密切关注：

- Raspberry Pi Pico（约 6 英镑，带预焊引脚）； ARM Cortex-M 但没有内置调试器，HAL 仍在开发中。不过目前有很多活动，进展很快。
- HiFive1 Rev B（约 50 英镑）； RISC-V 是新的热点。 Rust 中似乎有很多围绕它的活动，但它目前还没有 ARM Cortex-M 的支持。 其他需要关注的开发板是 Logan Nano 和 ESP32-C3。

部分内容略有轻微调整，更多可阅读原文：[Rob Young | digital](https://github.com/robyoung)

## 如何来看待 unwrap

unwrap 方法可能会让新手感到困惑。一些建议:

- 可以使用 Expect (&str) 而不是 unwrap() 为 panic 提供上下文。
- 使用 unwrap 和 expect 类似于断言。如果他们 panic，那只有在不可挽回的情况下才会发生。
- 避免在库代码中使用。

[https://owengage.com/writing/2021-08-30-how-to-think-of-unwrap/](https://owengage.com/writing/2021-08-30-how-to-think-of-unwrap/)

## Learning Rust: Interfacing with C

通过本文学习如何使用 Rust 调用 C 方法以及如何在 C 中调用 Rust 方法.

[https://piware.de/post/2021-08-27-rust-and-c/](https://piware.de/post/2021-08-27-rust-and-c/)

## Unsafe Rust 的超能力

该文是 knoldus 团队的系列文章之一，该团队博客还有很多 Rust 相关文章可以学习。

[https://blog.knoldus.com/superpowers-of-unsafe-rust/](https://blog.knoldus.com/superpowers-of-unsafe-rust/)

## Rust 密码学生态系统概述

根据密码库漏洞的实证研究（Jenny Blessing、Michael A. Specter、Daniel J. Weitzner - MIT），密码库中 37.2% 的漏洞是内存安全问题，而只有 27.2% 是密码问题。

现在是时候从C语言作为实现加密原语的现实中走出来了！

这篇文章摘自 Black Hat Rust 一书：

由于其具有低级控制、无垃圾收集器、可移植性和易于嵌入的高级性质，Rust 是我们替换当今最著名的加密库的最佳选择：OpenSSL、BoringSSL 和 libsodium，它们都是用 C 编写的 .

这肯定需要时间，但在 2019 年，根据任务的不同，rustls（我们稍后会看到的一个库）的基准测试比 OpenSSL 快 5% 到 70%。 看到广泛采用的最重要的事情之一（今天缺少）？ 认证（如 FIPS）。

事不宜迟，这里是 2021 年对 Rust 密码学生态系统的调查。

[https://kerkour.com/blog/rust-cryptography-ecosystem/](https://kerkour.com/blog/rust-cryptography-ecosystem/)