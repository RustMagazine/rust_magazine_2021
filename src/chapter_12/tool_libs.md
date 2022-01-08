

# 推荐项目 ｜ 基础工具库

聚焦 Rust 生态库

---

## SeaORM 0.4.0 发布

SeaORM 是一个 异步 ORM 框架。0.4 版本升级到了 Rust 2021 Edition。也新增了很多特性。

详情： [https://www.sea-ql.org/SeaORM/blog/2021-11-19-whats-new-in-0.4.0/](https://www.sea-ql.org/SeaORM/blog/2021-11-19-whats-new-in-0.4.0/)

## Zoxide 新版本发布： 现代化的智能 cd 命令

跨平台。它会记住`cd`进入的目录，因此只需按几下键就可以导航到任何地方。新版本最近添加了 shell 补全。

[https://github.com/ajeetdsouza/zoxide](https://github.com/ajeetdsouza/zoxide)

## wraft: 使用 Rust 和 WebRTC 为浏览器实现 Raft

这是一个 Raft 共识算法的玩具版实现。

WRaft 使用WebRTC 数据通道来建立浏览器窗口之间的通信。遗憾的是，WebRTC 并不是纯粹的点对点，因此有一个单独的基于 WebSocket 的服务 ( webrtc-introducer)，它可以在集群启动之前将浏览器窗口相互“介绍”。浏览器窗口可以在同一台计算机上，也可以在 LAN 上的不同机器上（理论上，或者不同的网络，但我还没有测试过）。Firefox 和 Chrome（或两者的任意组合）似乎可以工作；Safari 似乎无法正常工作。

- [https://wraft0.eevans.co/](https://wraft0.eevans.co/) 
- [https://eevans.co/blog/wraft/](https://eevans.co/blog/wraft/)
- [https://github.com/shosti/wraft](https://github.com/shosti/wraft)

## toql: 一个异步 ORM

目前只支持 MySQL。

[https://github.com/roy-ganz/toql](https://github.com/roy-ganz/toql)

## Rust Cuda: 0.2 版本发布

Rust CUDA 项目是一个旨在使Rust成为使用CUDA工具包进行极其快速的GPU计算的一级语言的项目。它提供了将Rust编译为极其快速的 PTX 代码的工具，以及使用现有CUDA库的库。

目前发布了 0.2 版本.

[https://github.com/Rust-GPU/Rust-CUDA/releases/tag/0.2](https://github.com/Rust-GPU/Rust-CUDA/releases/tag/0.2)

## monoio: 基于 io-uring 的异步运行时

Monoio 是字节跳动服务框架组开源的基于 io-uring 的 thread-per-core 模型高性能 Rust Runtime，旨在为高性能网络中间件等场景提供必要的运行时。

与 Golang 不同，Rust 语言中标准库并没有提供异步运行时(Runtime)，只提供了必要的结构抽象。Runtime 负责与操作系统打交道，并对齐标准库的 Future 和 Waker 等定义，用户可以自主选择 Runtime。

当前被广泛使用的 Runtime 是 Tokio，它提供了类似 Golang 调度器的实现，用户的 Task 可以在多个线程之间被调度，较为有效地利用了多核心的性能。

但问题也随之而来：在部分强依赖高性能低延迟的场景下，调度带来的开销反而是用户不希望看到的。在核心数较多的情况下，调度开销反而会抵消调度带来的好处。

Nginx 和 Envoy 这类组件往往使用 thread-per-core 模型，即多少核心就运行多少线程，一个任务一旦被一个线程所接收，它后续的处理则都在该线程上。这种做法下几乎没有跨线程的开销，提升了 CPU 利用率，可以较好地保持系统的线性扩展性。此外，由于没有跨线程，处理逻辑也可以尽可能地利用 thread local 的便利，多数时候无需加锁便可操作共享数据。

面向这类场景，Monoio 基于 io-uring 致力于提供最佳的性能；另外，我们还定义了一个更适合 io-uring 的 IO trait。

[https://github.com/bytedance/monoio/blob/master/README-zh.md](https://github.com/bytedance/monoio/blob/master/README-zh.md)

## Hurl 1.5.0：一个运行和测试HTTP请求的命令行工具

Hurl 是一个简单的HTTP命令行工具，建立在libcurl和Rust之上。

Hurl允许运行以纯文本格式定义的HTTP请求。它可以用来获取数据，或者模拟一个场景(请求序列)，并在过程中对响应断言。它的文本格式既适合devops，也适合开发人员。

[https://github.com/Orange-OpenSource/hurl](https://github.com/Orange-OpenSource/hurl)

## amdgpud: AMD GPU tools: 1.0.8 版本发布

该仓库包含几个 AMD 图形相关的工具:

- amdfand - fan speed daemon
- amdvold - voltage and overclocking tool
- amdmond - monitor daemon

目前 1.0.8 版本发布了.

[https://github.com/Eraden/amdgpud](https://github.com/Eraden/amdgpud)

## yew: 0.19.3 版本发布

Yew是一个现代的Rust框架，用于使用WebAssembly创建多线程前端web应用。

目前 0.19.3 版本已经发布.

[https://github.com/yewstack/yew/releases/tag/yew-v0.19.3](https://github.com/yewstack/yew/releases/tag/yew-v0.19.3)

## Lapce: 用Rust编写的闪电般快速且功能强大的代码编辑器

Lapce完全是用Rust编写的，它的UI使用Druid，它使用Xi编辑器的Rope Science技术进行文本编辑，并使用Wgpu进行渲染。

特性：

- 作为一等公民的模态编辑(类似Vim)支持（也可以关闭）；
- 内置LSP(语言服务器协议)支持；
- 内置远程开发支持（受VSCode远程开发启发）；
- 插件可以用任何能编译成WASI格式的编程语言编写；
- 内置终端；

[https://github.com/lapce/lapce](https://github.com/lapce/lapce)

## geoping - shodan开发的多地ping工具

从世界各地的多个位置向指定地址发送 ping 请求。

[https://gitlab.com/shodan-public/geonet-rs](https://gitlab.com/shodan-public/geonet-rs)

## Goose: 由 locust 框架启发的性能测试框架

熟悉小编的朋友知道小编最近又转到了测试岗位的工作，在进行性能测试的时候由于 Jmeter 多线程模型占用了极大量的资源才能 打出并发量比较高的测试压力，于是找到了 Python 的 Locust（蝗虫）框架，用起来还算顺手，但资源占用对于在办公室里的笔记本电脑来说实在是弱了些。另外，由于 Python 的 GIL 原因，使得Locust 在单机上只能使用单核单进程，要想利用多核只能开启 Locust 的分布式配置

于是，在 @PotatoTooLarge 的指点下，发现 [Goose](https://github.com/tag1consulting/goose)这个 Rust 的框架，它由如下特点：

- 文档齐全，一如 Rust 社区的作用
- 基于 Reqwest，可以异步发起请求
- Locust 报告里有的指标，Goose 的报告里面基本都有
- **资源消耗极小**，适合在没有资源的情况下发起测试压力

但是，该框架仍有以下不足：

- 没有配置文件（以及profile）
- 没有 locust 的实时监控 WebUI

## YAAR，异步运行时

YAAR，**Y**et **A**nother **A**synchronous **R**untime 专注于`#![forbid(unsafe_code)]`和扩展性，目前这个 crate 正在开发中。

作者决定挑战自己，为 Rust 编写一个不使用`unsafe`的异步运行时，但仍然会依赖 `parking_lot`, `crossbeam`, `arc-swap`。目标是做出一个`#![forbid(unsafe_code)]`在性能上与 tokio 有竞争力的异步运行时。这首先是一个研究项目，其次才是一个库。

[https://github.com/kprotty/yaar/tree/forbid_unsafe_std3](https://github.com/kprotty/yaar/tree/forbid_unsafe_std3)

## Spiderfire: JS运行时

Spiderfire 是一个用 Mozilla 的 SpiderMonkey 引擎和 Rust 构建的 JavaScript 运行时，Spiderfire 的目标是颠覆服务器端的 JS 运行环境。

[https://github.com/Redfire75369/spiderfire](https://github.com/Redfire75369/spiderfire)

## foundry：以太坊应用程序开发工具包

一个用 Rust 编写的用于以太坊应用程序开发的快速、便携和模块化的工具包。包括：

- Forge：以太坊测试框架（如 Truffle、Hardhat 和 Dapptools）。
- Cast：瑞士军刀，用于与 EVM 智能合约交互、发送交易和获取链数据。

特点：

- 快速、灵活的编译管道
  - Solidity 编译器版本自动检测安装
  - 增量编译和缓存：仅重新编译更改的文件
  - 并行编译
  - 非标准目录结构支持
- 测试是用 Solidity 编写的
- 使用收缩输入和打印反例的快速模糊测试
- 快速远程 RPC forking 模式利用 Rust 的异步基础架构
- 灵活的 Debug 日志
  - Dapptools 风格，使用 DsTest 发出的日志
  - Hardhat 风格，使用流行的 console.sol 合约
- 便携 (5-10MB) & 易于安装静态链接的二进制文件，无需 Nix 或任何其他包管理器
- 抽象的 EVM 实现（目前支持：Sputnik、EvmOdin）

[https://github.com/gakonst/foundry](https://github.com/gakonst/foundry)

## Zetro: 从 schema 中生成高效的 API

作者从微软的一些 API 中获得灵感, 开了一个 可以 从 schema 文件生成 类型化和极其高效的api的库.

[https://github.com/muscache/zetro](https://github.com/muscache/zetro)

## codasai: 使用 git 的历史记录来创建编程指南

codasai 可以让你使用 git 的历史记录来创建编程指南，这样读者就可以在任何给定的时间点查看程序的状态。

[https://github.com/emi2k01/codasai](https://github.com/emi2k01/codasai)

## Robyn 发布 v0.10 版本

Robyn 是一个用 Rust 编写的快速异步 Web Python 框架。 作者从今年 5 月开始编写 Robyn，今天发布了 v0.10.0。 根据他的基准测试，它是最快的 Python 框架之一。

目前已经添加了 WebSockets、MultiCore 扩展（以及更多）等功能！欢迎大家尝试和贡献！

[https://github.com/sansyrox/robyn](https://github.com/sansyrox/robyn) 

## gitoxide - 一个体验和性能更好的git工具套件

它不但是一个命令行工具，还是一批 crates，你可以基于它开发自己的 git 工具。git 本身可以作为一套版本管理系统或数据库存在，二次开发打开了一道新大门。

[https://github.com/Byron/gitoxide](https://github.com/Byron/gitoxide)

## podman-api Rust的podman api

Podman 是一个容器引擎，用于在 Linux 上开发，管理和运行 OCI 容器。

[https://github.com/vv9k/podman-api-rs](https://github.com/vv9k/podman-api-rs)

## Hello, youki!

Youki 是一个用Rust编写的底层容器运行时，实现了OCI运行时规范。简单地说，youki是一个可以创建容器的命令行工具。您可能听说过的其他同类运行时包括 runc 和 crrun。当您使用Docker或Podman创建容器时，实际的容器创建被委托给 youki 这样的工具。

[https://www.utam0k.jp/en/blog/2021/12/27/youki_first_release/](https://www.utam0k.jp/en/blog/2021/12/27/youki_first_release/)

## clap v3.0 发布

clap 3.0，一个 Rust CLI 参数解析器

[https://epage.github.io/blog/2021/12/clap3/](https://epage.github.io/blog/2021/12/clap3/)

## SIMD 加速操作

支持 SSE4.1、AVX2、ARM NEON、Aarch64 NEON 和 WASM SIMD128！

比 fast-hex、base64 和 radix64 更快！

- [https://github.com/Nugine/simd](https://github.com/Nugine/simd)

## pixels v0.9 发布

Pixels 是一个小的硬件加速像素帧缓冲区。它广泛用于模拟器、软件渲染器、2D 像素艺术游戏和桌面实用程序。

0.9 版本带来了一些重大变化。 值得注意的是 wgpu 已更新到 0.12，我们现在需要 2021 版。

- [https://github.com/parasyte/pixels](https://github.com/parasyte/pixels)
- [https://github.com/parasyte/pixels/releases/tag/0.9.0](https://github.com/parasyte/pixels/releases/tag/0.9.0)

## Kira, 一个用于游戏开发的音频库

Kira 是用 Rust 编写的用于游戏开发的音频库，并且具有不太常见的功能，例如平滑的补间参数和声音的精确定时，作者正在用它来制作一个动态生成的音乐游戏。

[github.com/tesselode/kira](github.com/tesselode/kira)

## mmids: Rust编写的多媒体收发系统

mmids (multimedia Ingestion and Distribution System)是一个功能强大、用户友好、开源的实时视频工作流服务器。

目前 mmids 能做什么?

- 通过RTMP 接收音频/视频
- 提供RTMP 音频/视频服务
- 从外部源接收音频和视频
- 直播视频转码
- 生成视频的HLS流
- 将实时视频推到外部RTMP服务器。

[https://github.com/KallDrexx/mmids/](https://github.com/KallDrexx/mmids/)


## LLML: Low Level Math Library

具有高级前端和低级后端的基本数学数据类型的实现

- Complex number
- Quaternion
- Affine Transform 2D
- Vector 2
- Vector 3
- Vector 4
- Matrix 2
- Matrix 3

[https://github.com/Aandreba/llml](https://github.com/Aandreba/llml)

## Dioxus v0.1 发布

Dioxus 是一个新的 Rust GUI 工具包，用于 Web、桌面、移动、SSR、TUI，关注开发人员体验，有下面几点优势：

- 熟悉：提供类似 React 的心智模型和 API
- 健壮：借助 Rust，将规则和错误处理移入类型系统来避免运行时错误
- 高性能：可扩展到最大的应用程序
- 高效：全面的文档、快速重新编译和深度集成的工具
- 可扩展：适用于所有平台的可重用 hook 和组件

[https://dioxuslabs.com/blog/introducing-dioxus/](https://dioxuslabs.com/blog/introducing-dioxus/)

## zbus 2.0 发布

简要介绍一下 zbus 是什么：

D-Bus 是一种在 Linux（尤其是桌面和嵌入式系统）上非常流行的进程间通信 (IPC) 机制。 而 zbus 是一个纯粹的 Rust 库，旨在使 D-Bus 处理尽可能简单，许多服务（例如 systemd、NetworkManager、Geoclue 等）都使用它，所以如果你发现自己在 Rust 中与它们交互，你可以使用 zbus。

而大家期待已久的2.0 稳定版发布了！ 😎 虽然 1.x 版本很受欢迎，但缺少异步 API。 2.0 使用了全新的设计，将异步 API 作为主要的 API，阻塞 API 只是一个包装器。

[docs.rs 链接](https://docs.rs/zbus/latest/zbus/)
[zbus book 链接](https://dbus.pages.freedesktop.org/zbus/)

## LibAFL - 高性能 Rust fuzzer

Fuzzing 可以完全自动化地发现对安全至关重要的错误。

而 LibAFL 是从零开始用 Rust 编写的 fuzzing 框架，可以创建几乎所有的模糊测试器。

在谷歌的 fuzzbench 基准测试中，它已经超过了许多著名的覆盖引导模糊器，包括 honggfuzz、libfuzzer/entropic 以及 old-skool afl。最重要的是，LibAFL 可以更好地跨内核和机器扩展。

您还可以使用它在 Windows、Android、macOS 和 LibAFL 上对纯二进制目标进行模糊测试，也完全与 no_std 兼容。

[https://github.com/AFLplusplus/LibAFL](https://github.com/AFLplusplus/LibAFL)

## git-smart-checkout: git 智能 checkout 工具

[https://github.com/craciuncezar/git-smart-checkout/](https://github.com/craciuncezar/git-smart-checkout/)

