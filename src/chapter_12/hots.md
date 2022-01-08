# 社区热点

聚焦 Rust 生态热点新闻

---

## Rust for Linux 补丁更新到 V2 版本

2022 年，我们很可能会看到 Linux 内核中的实验性 Rust 编程语言支持成为主流。2021.12.6 早上发出了更新的补丁，介绍了在内核中处理 Rust 的初始支持和基础设施。

这次更新的内容包括：

1. 升级到了最新 Stable 编译器和 Rust 2021 edition 。因此可以摆脱了 `const_fn_transmute`，`const_panic`、`const_unreachable_unchecked`、`core_panic` 和`try_reserve` 这几个之前未稳定的特性。[未稳定特性心愿单]( https://github.com/Rust-for-Linux/linux/issues/2)。
2. 自定义 `core` 和 `alloc`。为 `alloc` 添加了更加模块化的选项，以便禁用一些他们不需要的功能：`no_rc` 和 `no_sync`，主要是为上游 Rust 项目添加。
3.  更严格的代码、文档和新的  `lint`。
4. 抽象和驱动程序更新。添加了序列锁、电源管理回调的抽象，io 内存（`readX`/`writeX`）、irq 芯片和高级流处理程序，gpio 芯片（包括 irq 芯片）、设备、amba 设备和驱动程序以及证书。此外，也改进并简化了 `Ref`（`refcount_t` 支持）对象并用它替换了 Rust 的 `Arc` 的所有实例。完全地从 `alloc` crate 中删除了 `Arc` 和 `Rc`。

从现在开始，Rust for linux 团队将开始定期提交补丁，每两周左右。

除了来自 Arm、Google 和 Microsoft 的支持外，这次该团队又收到一封来自红帽的信：红帽对 Rust 用于内核的工作也非常感兴趣（There is interest in using Rust for kernel work that Red Hat  is considering）。

- [v2 补丁：https://lore.kernel.org/lkml/20211206140313.5653-1-ojeda@kernel.org/](https://lore.kernel.org/lkml/20211206140313.5653-1-ojeda@kernel.org/)
- [https://www.phoronix.com/scan.php?page=news_item&px=Rust-For-Linux-v2](https://www.phoronix.com/scan.php?page=news_item&px=Rust-For-Linux-v2)
- [kernel  crate 文档](https://rust-for-linux.github.io/docs/kernel/)

## Hubris ： OXide公司出品的新的开源嵌入式 OS

Hubris  没有运行时创建或销毁任务的操作，没有动态资源分配，没有以特权模式运行的驱动程序代码，系统中也没有C代码。通过这种构造，消除了许多通常存在于类似系统中的攻击面。

OXide 公司在今年 OSFF Mini Summit 2021 会议上分享了  [即将到来的固件革命](https://www.youtube.com/watch?v=XbBzSSvT_P0) 中提到，Rust 将会是即将到来的固件革命的一部分。所以，他们重新审视嵌入式操作系统并用 Rust 开发了 Hubris。 Hubris 目前只支持 Arm Cortex M 平台。

Hubris vs TockOS ：

- Tock 使用动态加载，Hubris是静态的

- Tock 是非常异步的，Hubris是严格同步的

- Tock 的驱动程序与内核在同一保护区，Hubris 的驱动程序位于不同的投影域中


- [https://oxide.computer/blog/hubris-and-humility](https://oxide.computer/blog/hubris-and-humility) 
- [https://github.com/oxidecomputer/hubris](https://github.com/oxidecomputer/hubris)
- [https://github.com/oxidecomputer/humility](https://github.com/oxidecomputer/humility)


## Mixxx 工程师正在探索用 Rust 实现新的 DJ 软件

Mixxx 是起源于 2001 年的一款开源数字DJ系统。一名在 Mixxx 工作近7年的工程师现在决定要用 Rust 来实现新的 DJ 软件。但他认为，世界上已经不在需要另一个 Mixxx 了，他想做一些更具开创性的工作。

Mixxx 太古老了，当初设计的时候也存在一些技术缺陷，而且它使用 Qt 实现，现在正在经历 Qt5 到 Qt6 升级的障碍，技术债务巨大。Mixxx 的 GUI 渲染非常复杂，效率低下，并且围绕着一个已弃用的API 16在 Qt6 中被删除。迁移到 Qt 的较新 API，尽管工作了几个月，但仍然没有成功。Mixxx 唯一可行的前进道路是使用 QML 而不是 QWidgets 从头开始​​重写整个 GUI。在这方面已经取得了进展，但仍有很长的路要走。许多 Mixxx 与 GUI 耦合，包括整个音乐库数据库系统和 Auto DJ 功能，因此必须为 Qt6 完全重写。实际上，该工程师预计 Mixxx 需要 1 到 2 年才能发布 Qt6。

该工程师现在正在考虑将这些年投入到一个建立在 Rust 更好基础上的新应用程序上，而不是继续逐个系统地重写 Mixxx。我一直在探索现有的 Rust 音频生态系统，看起来有一些不错的库可供入门。特别是[creek](https://github.com/RustyDAW/creek)和[Symphonia](https://github.com/pdeljanov/Symphonia)看起来很有用。[dasp](https://github.com/RustAudio/dasp)看起来像是一个很好的基础，我希望它被广泛采用，这样我们就可以建立一个易于互操作的 crate 生态系统。[cpal](https://github.com/RustAudio/cpal)也不错，但他感觉该库还不够成熟。还有 [Meadowlark](https://github.com/MeadowlarkDAW/Meadowlark)旨在成为适用于 Linux、Mac 和 Windows 的免费和开源 DAW（数字音频工作站）。

对于 GUI ，该工程师决定使用 [SixtyFPS](https://github.com/sixtyfpsui/sixtyfps)，因为 SixtyFPS 更有前景，两位创始人都是 Qt 多年经验的人，其中一位更是 QML 的首席工程师。

[https://rust-audio.discourse.group/t/a-new-dj-application-in-rust/484/12](https://rust-audio.discourse.group/t/a-new-dj-application-in-rust/484/12)

## Wasmer 2.1.0 发布

经过几个月的工作，WASMER 发布了 2.1 版本。包含一些新功能和错误修复，包括：

- Wasmer Js.
- 虚拟文件系统
- iOS 支持
- Windows 下支持 Singlepass 编译
- LLVM ARM64 支持 & LLVM 13
- 更快的 Singlepass 编译
- 可重复和确定性构建
- 新语言集成：LISP 和 Crystal

Wasmer 2.1: [https://wasmer.io/posts/wasmer-2.1](https://wasmer.io/posts/wasmer-2.1)

##  GitHub 新的代码搜索引擎使用 Rust 实现

[https://github.blog/2021-12-08-improving-github-code-search/](https://github.blog/2021-12-08-improving-github-code-search/)

## 分析 Rust 生态系统对 IoT 的准备情况

本文是斯德哥尔摩 KTH 皇家理工学院 ID2012 普适计算课程的一部分。

任务是对普适计算或物联网领域的某个主题进行分析概述。作者选择 Rust 的物联网应用程序是出于我个人对 Rust 的兴趣以及之前深入物联网的经验，特别是使用 Rust 的嵌入式设备。

这篇文章于2021 年 5 月完成并提交，今天回顾该领域已经显示出一些细微的变化。因此请注意，该领域正在迅速发展，并且可能看起来更好，具体取决于您何时阅读本文。

Rust 完全有能力在嵌入式计算等更高级别的物联网领域完成特定任务，例如边缘轻量级计算和后端服务的实现。然而，它表明 Rust 是一种相当新的语言。由于其生态系统的这些部分，与物联网相关，仍在不断发展，甚至缺乏一些重要的基础，而且远非稳定。对于早期采用者和普遍好奇的人来说，Rust 仍然为构建自己的解决方案提供了基础，尽管没有提供数十年开发的成熟度。从好的方面来说，我们看到像 Drogue、Ferrous Systems 和其他独立集团这样的几家公司正在这样做。至关重要的基础正在积极开发中，并为 Rust 带来更光明的未来。

Rust 展示了成为未来首选物联网平台的潜力，提供速度、人体工程学和安全性，但没有显示出成熟的成熟度，可以在没有经过全面考虑的情况下轻松使用。

[https://blog.ysndr.de/posts/essays/2021-12-12-rust-for-iot/](https://blog.ysndr.de/posts/essays/2021-12-12-rust-for-iot/)

## ## Deno加入ECMA国际组织的TC39工作组

[Deno Joins TC39](https://www.reddit.com/r/rust/comments/rfem07/deno_joins_tc39/)

Deno 是 `Ryan Dahl` 在2017年创立的(`Ryan Dahl` 同时也是 `Node.js` 的作者)，旨在为`JavaScript`和`TypeScript`构建一个简单、现代、安全的运行时，Deno是用Rust实现的，内置`V8`引擎。

Deno基本上是为现代JavaScript构建的：`Promises`、`async/await`、`ES模块`和`异步迭代器`等在Deno中都是一等公民。

为了确保JavaScript的未来发展将继续适用于Deno和服务器端JavaScript运行时，Deno公司已加入负责JavaScript标准（`ECMA-262`）的ECMA国际标准组织，并参与到`TC39`工作组的工作中，`Luca Casonato`(卢卡·卡索纳托)将是Deno在TC39的主要代表。

在TC39工作组中，Deno将与其他ECMA成员和更广泛的JS社区合作开发下一代JavaScript，并将推动该语言的功能和改进，使每个人受益，尤其是服务器端JavaScript的用户。

> TC39是指第39号技术委员会。它是ECMA的一部分，该机构根据“ECMAScript”规范标准化JavaScript语言。

参考资料：

- [Deno joins TC39](https://deno.com/blog/deno-joins-tc39)
- [Deno Land](https://deno.land/)
- [tc39.es](https://tc39.es/)
- [TC39, ECMAScript, and the Future of JavaScript](https://ponyfoo.com/articles/tc39-ecmascript-proposals-future-of-javascript)
- [Deno 运行时入门教程：Node.js 的替代品](https://www.ruanyifeng.com/blog/2020/01/deno-intro.html)

## ## cbor4ii：一种新的`CBOR`格式序列化库

[cbor4ii: A new CBOR serialization crate](https://www.reddit.com/r/rust/comments/rg2qgg/cbor4ii_a_new_cbor_serialization_crate/)

**CBOR**：

简明二进制对象表示法（CBOR）是一种数据格式([RFC8949](https://www.rfc-editor.org/rfc/rfc8949.html))，其设计目标包括实现极小的代码大小、相当小的消息大小和无需版本协商的可扩展性。

cbor4ii是一个新的用Rust语言实现的`CBOR`格式序列化库，它没有针对性能进行专门优化，但基准测试表明，它的性能略优于[`serde_cbor`](https://github.com/pyfisch/cbor)。

而且它支持零拷贝反序列化和反序列化任何支持`deserialize_ignored_any`的`serde`，因此在某些情况下，它的性能可能比不支持此功能的`serde`要好。

## Cratesinquire: 搜索并分析`crates.io`

[Cratesinquire: crates.io insight and explorer.](https://www.reddit.com/r/rust/comments/rfk25y/cratesinquire_cratesio_insight_and_explorer/)

[www.cratesinquire.com](https://www.cratesinquire.com/)是一个crates.io的辅助功能网站，以`Bundlephobia`为灵感，搜索crate包并检查其细节。

**它能做什么？**

- 向[`crates.io`](https://crates.io/)的API发送请求，并处理数据；
- 显示从[`crates.io`](https://crates.io/)扩展的额外数据；
- 显示数据比较图表；
- 深入显示crate的依赖；
- 洞察crate的特征。

立刻体验，请访问这里：[www.cratesinquire.com](https://www.cratesinquire.com/)


## 现代链接器 mold v1.0 发布

mold 是一个更加现代的链接器，它是由 C++ 编写的，但能够与 Rust / Cargo 生态完美协作，提供更快地链接过程。

你可以使用 `mold --run cargo build` 来尝试它，或者将下述内容添加到 `~/.cargo/config.toml`：

```
[target.x86_64-unknown-linux-gnu] 
linker = "clang" 
rustflags = ["-C", "link-arg=-fuse-ld=/PATH/TO/mold"]
```

[GitHub - mold](https://github.com/rui314/mold)

## tokio-rs/console 发布 v0.1

一个分析与调试的工具，为异步 Rust 程序设计。分析套件包括几个组件：

- 一个 wire protocol，为了串流来自被分析应用到分析工具的分析数据。这个格式是用了 gPRC 和 protocol buffer 定义的，为了高效地在 wire 上传送，以及为了保证不同数据生产者和消费者之间的互操作性。 `console-api` 库包含了 wire format 的自动生成代码，用的是 tonic gPRC。此外，使用其他 gPRC 代码生成器的项目（包括使用其他语言实现的！）能够自行依赖 protobuf 定义。
- 从进程收集分析数据并通过 wire format 暴露它的instrumentation。 仓库中的 `console-subscriber` 库包含了作为tracing-subscriber 层的 instrumentation 端API，给用 Tokio 和 tracing 的项目使用。
- 用来展现和探索分析数据的工具，使用 console wire protocol 实现成了gPRC 客户端。`console` 库实现了一个非交互式的命令行工具来消费这些山上，但是其他实现，例如图形化的或者基于web 的工具，也是可用的。

[https://github.com/tokio-rs/console](https://github.com/tokio-rs/console)

## 一个由Atom开发者用Rust编写的代码编辑器

目前官方透露的信息很少，给了一个简陋的官网：[https://zed.dev/](https://zed.dev/)

同时官方在该帖下透露了一些有趣的信息。

在正式介绍相关信息之前，我想多说几句，为什么都过了一周了，又把这个信息扒出来，之前的时候我以为这个信息热度挺高的，一定会发日报吧，没仔细看，知道后面想看看大家对这件事的评论的时候，才发现居然没发日报。索性今天我值班，就由我来给大家说说这个旧闻。

Atom大家应该有听说过，是GitHub开发的一个代码编辑器，早年我还在大学的时候折腾过各种编辑器，对这个的印象实在是不怎么好，现在回想起来，第一个感觉就是卡，真卡。同时，由于该团队开发Atom的时候，需要一个前端框架，于是有了大名鼎鼎的Electron，早先的名字叫做Atom Shell，名气上应该是要大于Atom本身的。对于这个框架，我的感受，还是卡，因为有用过Linux各种桌面发行版的经历，很多软件为了跨平台支持，都是Electron开发的，很卡。

现在这个团队，要用Rust开发一个代码编辑器，和前段时间的Fleet不一样的地方在于，前端的部分，也是Rust写的，同时因为Rust的GUI生态很薄软，他们直接开发了一个Rust前端框架，叫做GPUI，直接放原文：

> 我们最初计划使用Electron作为为Zed提供跨平台GUI的便捷手段，同时用Rust构建应用程序的核心。但每当这时，我们发现网络技术是实现惊人性能的瓶颈。最后，我们决定建立一个由GPU驱动的可完全控制的并简单的UI框架，以满足我们的需求。
>
> 我们称它为GPUI。
>
> 我们从Mozilla的Webrender项目中得到了很多灵感。关键的见解是，现代图形硬件可以以高帧率渲染复杂的3D图形，那么为什么不利用它来渲染相对简单的2D用户界面，并采用即时模式架构呢？
>
> Rust的所有权模型要求我们重新思考我们在其他UI编程范式中所学到的很多东西，但其结果是一个框架，使用起来很有成效，而且非常容易推理。
>
> 控制每一个像素是一种解放，而以闪电般的速度推动这些像素则是一种冲动。(It’s liberating to control every pixel, and it’s a rush to push those pixels at lightning speed.)

单单是上述官方的博文，仍然无法确定这个GUI框架是用Rust写的，但是该帖子下有这样的回复：

> Rust迫切需要一个好的UI框架，让每个人都觉得用起来很舒服......现在有一些很棒的独立解决方案，但没有一个听起来适合每个人的。

这样大概率是Rust的UI框架解决方案了。

相对于这个编辑器本身，我对该UI框架的兴趣更为浓厚，也正是有了很大的期望，所以如果最后一些偏差，比如该框架不是给Rust做的、比如该框架不是像宣传所说的很简单易用，那会让我很失望，索性把这个旧闻翻出来，让对Rust进行GUI开发感兴趣的同志们一齐期待，比我最后一个人可能失望要好的多。

[Read More](https://libreddit.spike.codes/r/rust/comments/rgyss8/an_code_editor_written_in_rust_by_the_atom_devs/)

## Rust有什么是Zig所没有的？

Reddit 上有个讨论很有意思，同属于更好的C的类型的语言，Zig提供了很多有趣的设计思路，甚至于Rust语言团队也需要从中吸取经验，用以改进Rust，这部分讨论收录在这里：

> 我还没有深入研究过Zig，但与Rust相比，它似乎是一种非常简单的语言，就系统编程语言而言，它是相当有能力的。Rust有什么是Zig所没有的？

高赞回复：

> ```
> Rust有什么是Zig所没有的？
> ```
>
> 编写代码时不需要经常考虑内存管理问题的能力。
>
> Zig让内存管理变得非常明确，比如强迫你处理分配失败的情况，这很好......但是，这并不是我在PC平台上做游戏时真正要处理的问题。
>
> 一旦Rust的基本借用规则成为第二天性，代码就几乎不存在内存管理部分了......一切都会自动地做 "正确的事情"。
>
> 在使用Rust这么长时间后，我觉得要传递内存分配器，要存储指向内存分配器的指针都是很麻烦的。
>
> 不要误会我的意思，它是一个非常棒的项目（就像Jai、Odin和所有其他即将推出的"更好的C"一样），但是不断有走错路的危险，我觉得很难再回去了。
>
> 而且这还没有触及多线程的问题

[Read More](https://libreddit.spike.codes/r/rust/comments/rlj9zl/what_do_you_think_about_zig/)

