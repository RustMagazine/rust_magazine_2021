# 学习资源

## 2021 春季全球进阶类 OS 课程汇总信息

Advanced OS Course (2021 Spring)  

本仓库由清华大学陈渝教授维护

[https://github.com/chyyuu/aos_course_info](https://github.com/chyyuu/aos_course_info)

Rust based OS/Hypervisor/VMM/Firmwire ： 

[https://github.com/chyyuu/aos_course_info/blob/master/oslist.md](https://github.com/chyyuu/aos_course_info/blob/master/oslist.md)

## 结合 Axum、Hyper、Tonic 和 Tower 用于混合 Web/gRPC 应用程序

由 FPComplete 公司写的系列教程：

- [https://www.fpcomplete.com/blog/axum-hyper-tonic-tower-part1/](https://www.fpcomplete.com/blog/axum-hyper-tonic-tower-part1/)
- [https://www.fpcomplete.com/blog/axum-hyper-tonic-tower-part2/](https://www.fpcomplete.com/blog/axum-hyper-tonic-tower-part2/)
- [https://www.fpcomplete.com/blog/axum-hyper-tonic-tower-part3/](https://www.fpcomplete.com/blog/axum-hyper-tonic-tower-part3/)
- [https://www.fpcomplete.com/blog/axum-hyper-tonic-tower-part4/](https://www.fpcomplete.com/blog/axum-hyper-tonic-tower-part4/)

## FishFight： macroquad 游戏框架实现的一款2D 闯关游戏

刚刚开源

- [https://github.com/fishfight/FishFight](https://github.com/fishfight/FishFight)
- [https://fishfight.itch.io/ff/devlog/291737/fish-fight-is-open-source](https://fishfight.itch.io/ff/devlog/291737/fish-fight-is-open-source)

## 一个深度学习研究员学习Rust的经验

作为深度学习研究员，处于深度学习领域的最前沿，自然用 py 和现成的框架是不够的。而 Rust 是一个非常好的替代 c++ 的底层算法实现选择。

Reddit 下面有相关的讨论值得了解：[https://www.reddit.com/r/rust/comments/pft9n9/i_wanted_to_share_my_experience_of_rust_as_a_deep/](https://www.reddit.com/r/rust/comments/pft9n9/i_wanted_to_share_my_experience_of_rust_as_a_deep/)

## 编写高性能 Rust 代码的几个简单技巧

请记住，下面的建议并不能取代实际的分析和优化 ：

1. 配置 release profile：

```toml
[profile.release]
lto = "fat"
codegen-units = 1
panic = "abort"

```

2. 设置 `-C target-cpu=native`
3. 选择  jemalloc 或  mimalloc 分配器
4. 阅读 [Profile Guided Optimization](https://doc.rust-lang.org/1.41.1/rustc/profile-guided-optimization.html)
5. 使用性能测试工具。

[https://deterministic.space/high-performance-rust.html](https://deterministic.space/high-performance-rust.html)

## 使用 Rust 和 SIMD加速来创建世界上最快的 tac 

tac是对Coreutils的GNU tac工具的一个高性能、模拟加速、跨平台的重写，在与BSD兼容的（MIT）许可证下发布。tac从文件（或从stdin）中读取输入，然后逐行逆向打印。

tac 倒过来就是 cat 。

这个tac的实现使用simd-acceleration进行新行检测（在这里阅读更多关于它的内容），并在所有支持的操作系统上利用内存映射的文件。此外，它是用rust编写的，以获得最大的完整性和安全性。

作者说道：“在2021年，在tac中动态地启用AVX2支持要容易得多！”。

如果你想学习 SIMD 相关内容可以参考这篇文章。

- [Readmore](https://neosmart.net/blog/2021/using-simd-acceleration-in-rust-to-create-the-worlds-fastest-tac/)
- [tac](https://github.com/neosmart/tac/)

## hebi : 由 Bevy 引擎驱动的贪吃蛇游戏

hebi 是一个高度可定制的贪吃蛇游戏复刻，使用 Rust 写就，由 Bevy 引擎驱动，命名源于日语中的“蛇”。

该项目有助于学习 Bevy 引擎。

[https://github.com/ElnuDev/hebi](https://github.com/ElnuDev/hebi)

## 【PodCast】cURL 中的 Rust

Allen Wyma 与 cURL 的原作者 Daniel 谈论在 cURL 中使用 Rust。

- cURL 是一个命令行工具和库，用于通过 URL 传输数据。
- cURL 及其数据传输核心 libcurl 都是用 C 编写的，众所周知，这不是内存安全的。
- 虽然几乎不可能将其重写为另一种语言，但提供一个用 Rust 编写的第三方库可能会更进一步。

[https://rustacean-station.org/episode/035-daniel-stenberg/](https://rustacean-station.org/episode/035-daniel-stenberg/)

## 加快 Rust 的编译

众所周知，Rust代码编译起来很慢。但我有一种强烈的直觉，大多数Rust代码的编译速度比它本可以的要慢得多。

例如, Rust 的 `rust-analyzer` CI 在 GitHub 上操作需要8分钟。这是一个相当大和复杂的项目，有20万行自己的代码和100万行依赖。

跟随作者, 让我们进一步了解如何使编译时间保持在合理的范围内!

[https://matklad.github.io/2021/09/04/fast-rust-builds.html](https://matklad.github.io/2021/09/04/fast-rust-builds.html)

## 对 Rust 异步语法的重新思考

该文作者对 async trait 相关问题做了深刻思考之后得出一个结论：现在的 `async fn` 语法设计是错误的。

```rust
// this really returns an `impl Future<Output = usize>`, but that's hidden
async fn foo() -> usize { 1 }
```

这是当前的异步语法，它隐藏了真正的返回类型：`impl Future<Output = usize>`。

该文作者认为，这个返回类型不应该被隐藏：

```rust
async fn foo() -> impl Future<Output = usize> { 1 }
```

这篇文章就是介绍这种显式的返回有哪些好处。文章很长，可以抽个安静的时间，跟随作者的思考去看此文。


[https://ibraheem.ca/writings/an-alternative-async-fn-syntax/](https://ibraheem.ca/writings/an-alternative-async-fn-syntax/)

## Rust 插件开发系列文章

这系列文章来自于 Google代码之夏中 Tremor 项目的 issues： [Plugin Development Kit ( PDK )](https://github.com/tremor-rs/tremor-runtime/issues/791)

> 简而言之，Tremor 是一个事件处理系统。 它最初是为了替代 Logstash 或 Telegraf 等软件而设计的。 然而，通过支持更复杂的工作流（例如聚合、汇总、ETL 语言和查询语言），tremor 已经超出了这个单一用例的范围。

为 Tremor 插件开发一个通用接口，使上述库变得更加模块化，并减少核心依赖集。

这将大大减少 Tremor 的核心大小，这意味着该库的编译速度将更快，二进制大小也更小。最重要的是，它将把Tremor的架构转变为完全模块化，其中插件可以根据需要进行配置并以语言无关的方式独立开发。


- [https://nullderef.com/series/rust-plugins/](https://nullderef.com/series/rust-plugins/)
- [https://github.com/tremor-rs/tremor-runtime](https://github.com/tremor-rs/tremor-runtime)

## 为什么 Rust 需要 Pin 和 unpin

- 什么是Futures
- 什么是自引用类型
- 为什么他们不安全
- Pin/Unpin 如何使它们安全
- 使用 Pin/Unpin 编写复杂的futures

[https://blog.cloudflare.com/pin-and-unpin-in-rust/](https://blog.cloudflare.com/pin-and-unpin-in-rust/)

## Rust 的 Logging 推荐

内容整理自 Reddit 的讨论：[What is the current recommendation for logging in Rust? : rust](https://www.reddit.com/r/rust/comments/pmdh6a/what_is_the_current_recommendation_for_logging_in/)。

问题简述：除了标准的 log，还有不少选择：env_logger，tracing，slog，simplelog 等等，最佳实践是什么？

来自 Koxiaet 的答复：

通常有两类与日志相关的 crate：日志接口和日志消费者。接口提供了想要记录某些东西时调用的函数，消费者处理将结构化日志数据格式化到某个地方（stderr 或文件）。两个主要的接口是 log 和 tracing，后者功能更强大因为它支持结构化日志记录，但前者更普遍。还有另一个结构化日志接口 slog，比 tracing 更古老但用的较少。每个日志接口都有自己生态系统，可以根据自己的需要选择。如果在写一个库，log 是个不错的选择，因为所有的日志记录接口都与它兼容。但如果你确实需要结构化日志记录，则可以改用 tracing，这取决于你的需求，比如你是需要写到文件还是只是终端。

其他网友的推荐：

- File Logging：[emabee/flexi_logger](https://github.com/emabee/flexi_logger): A flexible logger for rust programs that can write to stderr or to log files。（来自 cfsamson）
- tracing 的接口：[tracing_log](https://docs.rs/tracing-log/0.1.2/tracing_log/) - Rust，有多个同时操作交错日志消息时特别方便，可以按某些属性对它们进行分组并单独查看它们。（来自 class_two_perversion）
- [estk/log4rs](https://github.com/estk/log4rs): A highly configurable logging framework for Rust，log4rs 是一个高度可配置的日志框架，以 Java 的 Logback 和 log4j 库为模型。通过 Yaml 配置，到 sdout 和文件，带有文件大小限制选项，还可以配置不同级别的日志。（来自 tms102）
- [tracing-appender](https://crates.io/crates/tracing-appender) - crates.io: Rust Package Registry，推荐者所知道的唯一线程外日志记录解决方案，不仅适用于异步应用程序。（来自 Pand9）
- [daboross/fern](https://github.com/daboross/fern): Simple, efficient logging for Rust，像 Python 的 logging 和 JS 的 Winston。（来自 RapBeautician）

## Rust 全栈

本文是一篇博客翻译，来自：[Full Stack Rust - Blog](https://www.justinm.one/blog/2021/09/11/fullstackrust/)。

一年前，我的首选语言如下：

- Python 用于高级代码快速原型设计，或用于需要第三方功能的代码
- C/C++ 用于长期的 low-level 项目

当时只听过 Rust 并简单使用过，我的经验来自用 Rust 写了一个处理大文件（>4GB）的事务并从中挖掘一些统计信息的小工具。我用了一个库将文件映射到内存，缤瑞按照顺序对其进行分析。有一些很酷的概念，比如编译器静态地强制内存映射在它被取消映射后无法访问——如果你不小心，C++ 中可能就会发生这种错误。

不过当时并没有真正吸引我，因为那只是一个小新奇。当我向 [pdblister](https://github.com/DrChat/pdblister) 添加新功能以并行获取数千个 PDB 文件时诀窍来了。由于 GIL，在 CPython 中几乎不可能，而在 C/C++ 中做到不面临并行错误是极其困难的。然而 Rust 让这变得容易。我添加了 tokio 驱动的异步，使用 tokio::spawn 生成新任务来下载 PDB，并修复了编译器报的错误，它可以正常工作了。Rust 编译器输出一个二进制文件，它可以在任何地方运行，没有运行时依赖。

取代 Python

这是第一点，Rust 是 Python 作为中长期工具语言的绝佳替代品。Python 的好处是庞大的库和生态系统，通过 pip 可以直接拿到，想要快速制作与 API 交互的原型，可以使用 requests，只要 import requests 就可以使用了。Rust 的 reqwest 也是如此，只要输入 `cargo add reqwest` 就可以在代码中使用它。

然而当进入更长期的生命周期时，Python 就显示出劣势，requests 是程序的依赖，用户需要后去后才能使用。此外，由于弱类型和错误处理能力（与 Rust 比），Python 变得更加劣势。这一点上，我可以使用 Rust 比使用 Python 更快地编写原型工具，并且我可以自信地知道我的工具比等效的 Python 更易于维护且寿命更长。但是，对于短期工具，Python 可能仍然更好，因为它不需要启动项目即可在 VSCode 中获得智能感知支持。 Rust 的 cargo-script 接近将 Rust 推入脚本语言的领域，但不幸的是，我还没有在 VSCode 中找到与之集成的插件。

取代 C

Rust 也是 C 的直接替代品，它在各方面都更好，并且可以与遗留 C 代码原生互操作以进行增量替换。Rust 最大的改进是生态系统：如上所述，利用 Rust 生态中已有的库是很容易的。如果你从未使用过 C，那很幸运，实际上 C 中使用高级功能的最佳方法是自己写。

C 生态系统是支离破碎的，而且很脆弱。ABI 或构建系统没有一致的标准：

- 由于缺乏 ABI 一致性，你不能跨平台或操作系统使用相同的二进制文件。 所以你必须从源代码构建。
- 由于缺乏一致的构建系统，你不能简单地和应用程序一起构建 C 库，必须修补或重写要使其与你的库兼容的库的构建系统。
- C 库很少跨平台兼容，因为它们缺乏可以依赖的共享抽象。

然后还有 Rust 最特色的安全改进——我就不展开了。但根据我的经验 - 安全性在很大程度上是一种工具，可以让第三方库开发人员更容易强迫我正确使用他们的库，这是 C 库不能做的事情。

全栈 Rust

总而言之，在过去的一年中，我一直在堆栈的所有部分使用 Rust，而我之前使用过其他语言。我已经使用 Rust 来实现引导加载程序：[xenia-project/xell-rs: Xell Bootloader, rewritten in Rust because ¯_(ツ)_/¯](https://github.com/xenia-project/xell-rs)，我已经使用它通过 pdblister 和 panamax 中的高级 HTTP/HTTPS 和其他技术来镜像文件。我利用并贡献了优秀的 gdbstub 库，用于控制由自定义 VMM 运行的 VM。这些项目都是在堆栈的不同级别完成的，而 Rust 非常适合所有级别。 我已经开始在我的个人项目中专门使用 Rust，并在适合的时候推动它在我的工作中使用。

## tagged_cell：快速、可初始化和线程安全的静态变量

通过 `TaggedCell` 和 `Tag` 类型实现，为了安全操作，`TaggedCell` 的每个实例都必须是唯一的。然后必须通过 `TaggedCell::init()` 初始化 `TaggedCell`，它使用用户提供的函数或闭包初始化底层数据，然后返回一个特殊的零大小的 `Init<Tag>` 用于访问 `Cell` 的数据。为了确保每个单元格使用唯一的标签类型，`tagged_cell!` 提供宏。该宏根据变量的名称创建一个新的标记类型，并将其应用到声明中。

```rust
use tagged_cell::tagged_cell;
tagged_cell!{
   static BAR: TaggedCell<Vec<usize>, _> = TaggedCell::new();
}

let tag = BAR.init(|| vec![0, 10, 20]);
let vec = BAR.get(tag);

assert_eq!(vec[2], 20);
```

为了允许跨线程使用，只有第一次调用` TaggedCell::init `才会初始化` Cell` 的数据。所有未来的 `TaggedCell::init` 调用都将返回一个新标签。未确定哪个线程将初始化 `Cell` 的数据。

```rust
use std::thread;
use tagged_cell::tagged_cell;

tagged_cell!{
    static TABLE: TaggedCell<Vec<usize>, _> = TaggedCell::new();
}

thread::spawn(move || {
    let tag = TABLE.init(|| vec![0, 10, 20]);
    let table = TABLE.get(tag);
    assert_eq!(table[2], 20);
});

thread::spawn(move || {
    let tag = TABLE.init(|| vec![0, 10, 20]);
    let table = TABLE.get(tag);
    assert_eq!(table[1], 10);
});
```

[https://github.com/Dasch0/tagged_cell](https://github.com/Dasch0/tagged_cell)

## Learn Wgpu 更新了

wgrpu 是 WebGPU API spec 的 Rust 实现, 目前这个教程已经更新到了 0.10 版本, 有大量的原理和代码示例讲解.

[https://sotrh.github.io/learn-wgpu/beginner/tutorial2-surface/](https://sotrh.github.io/learn-wgpu/beginner/tutorial2-surface/)

## 使用 Rust 写 shellcode 代码

漏洞利用中必不可缺的部分就是shellcode，shellcode是用来发送到服务器利用特定漏洞的代码，它能在极小的空间内完成一些基本而重要的工作。

Shellcode编写方式基本有3种：

1. 直接编写十六进制操作码（不现实）；
2. 采用像C这样的高级语言编写程序，编译后，进行反汇编以获取汇编指令和十六进制操作码。
3. 编译汇编程序，将该程序汇编，然后从二进制中提取十六进制操作码。

《Black Hat Rust》 一书作者的文章介绍了如何使用 Rust 编写 shellcode 。

[https://kerkour.com/blog/shellcode-in-rust/](https://kerkour.com/blog/shellcode-in-rust/)

## 一个使用 axum 框架开发的二维码生成服务

axum 的一个实战项目示例，是一个真正在跑的服务：

[https://github.com/sayanarijit/qrcode.show](https://github.com/sayanarijit/qrcode.show)

## Rust CI/CD: github action 使用

和任何其他语言一样, 在我们掌握语法之外, 我们往往还有 CI/CD 的需求:

- 需要哪些组件来组成CI管道，以确保我的代码是健康的？
- 如何部署？
- 我需要编写自定义工具还是有社区资源可用？

作者会用三篇文章来讲解 Rust在 github 中如何使用 action 来完成 CI/CD.

[https://www.homeops.dev/continuous-integration-with-github-actions-and-rust/](https://www.homeops.dev/continuous-integration-with-github-actions-and-rust/)


## 使用 100 行 Rust 代码构建一个静态站点生成器

原理很简单，通过解析 markdown 文件，填充一个 html 模板。完成。

[https://kerkour.com/blog/rust-static-site-generator/](https://kerkour.com/blog/rust-static-site-generator/)


## 使用 Rust 和 Godot 实现 RPG 游戏

油管上有个系列视频 [Godot Action RPG Series](https://www.youtube.com/playlist?list=PL9FzW-m48fn2SlrW0KoLT4n5egNdX-W9a)，学习在Godot引擎3.2中制作一个动作RPG。

> Godot是一个出色的免费和开源的游戏引擎，是为独立游戏设计的。

但是油管那个视频教程不是 Rust 的，于是有人用 Rust 实现了一遍，公开了出来：

[https://github.com/Nejat/godot-action-rpg-tutorial-rs](https://github.com/Nejat/godot-action-rpg-tutorial-rs)


## 使用Rust和tokio构建可扩展服务

使用Tokio，很容易实现网络服务器应用。然而，你经常在指南书或此类书籍中发现的典型代码在多个CPU上的扩展性很差。我将解释为什么，以及如何通过改变几行来解决这个问题。

[https://medium.com/@fujita.tomonori/scalable-server-design-in-rust-with-tokio-4c81a5f350a3](https://medium.com/@fujita.tomonori/scalable-server-design-in-rust-with-tokio-4c81a5f350a3)