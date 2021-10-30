# 推荐项目 ｜ 基础工具库

## sentinel-rust ： 阿里的 sentinel 出了 Rust 版本 

目前是 WIP

Sentinel 是 阿里之前出的微服务限流熔断框架。

Sentinel 的主要工作机制如下：

- 对主流框架提供适配或者显示的 API，来定义需要保护的资源，并提供设施对资源进行实时统计和调用链路分析。
- 根据预设的规则，结合对资源的实时统计信息，对流量进行控制。同时，Sentinel 提供开放的接口，方便您定义及改变规则。
- Sentinel 提供实时的监控系统，方便您快速了解目前系统的状态。

以上参考 ： [https://zhuanlan.zhihu.com/p/64786381](https://zhuanlan.zhihu.com/p/64786381)

[https://github.com/sentinel-group/sentinel-rust](https://github.com/sentinel-group/sentinel-rust)

## apkeep： 从多个源下载APK 文件的命令行工具

[https://github.com/EFForg/apkeep](https://github.com/EFForg/apkeep)

## 远程桌面软件 RustDesk 1.1.8 发布

- 修复各种兼容性问题：白屏，闪退（有可能还会出现，没法测试所有系统）
-  修复鼠标/键盘异常和快捷键问题
- 修复Linux剪贴板同步问题
- 支持32位Windows，>=Win7
- 添加iOS客户端
- 手机端增加触屏模式，优化鼠标拖拽/选取
- 启用dxgi，如果失败就退回到gdi
- 升级tokio到v1，升级vpx到1.10
- 实验实现Wayland，可用性还是比较低，暂时放弃
- 默认只提供Windows安装程序，但是可以在安装界面选择无安装运行

作者：RustDesk
链接：[https://zhuanlan.zhihu.com/p/402963916](https://zhuanlan.zhihu.com/p/402963916)
来源：知乎
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

[https://github.com/rustdesk/rustdesk/releases/tag/1.1.8](https://github.com/rustdesk/rustdesk/releases/tag/1.1.8)

## MegFlow ： 旷视开源的满足长尾需求的高效机器学习解决方案

MegFlow 提供快速视觉应用落地流程，最快 15 分钟搭建起视频分析服务。

基于 Rust 实现。  

[https://github.com/MegEngine/MegFlow](https://github.com/MegEngine/MegFlow)

## Novus : Rust实现的 Windows 包管理器

性能比 Chocolatey 高 3~10倍。

[https://github.com/novus-package-manager/novus](https://github.com/novus-package-manager/novus)

## pilka ： 用 Rust 写成跨平台实时编码工具

Pilka是一种用于创建着色器（shader）演示的跨平台实时编码工具，类似于 Bonzomatic 或 KodeLife 。支持热重载，能够在后台检查和更新资源。

[https://github.com/pudnax/pilka](https://github.com/pudnax/pilka)

## pixels 0.6.0 发布

pixels 是用于简单软件侧光栅化的板条箱。它可以提供一个像素缓冲区，用于插入颜色（在 CPU 端完成）。缓冲区作为纹理上载到GPU，所有缩放和剪裁都由默认着色器处理。对于其他控件，可以添加自己的自定义着色器以进行预处理和后处理。

[https://github.com/parasyte/pixels](https://github.com/parasyte/pixels)

## NoProto：灵活、快速和紧凑的序列化和rpc

特点：

- 零依赖
- 支持no_std，WASM
- 最紧凑的非编译存储格式
- 稳定

[https://github.com/only-cliches/NoProto](https://github.com/only-cliches/NoProto)

## gradient-rs: 用于玩颜色渐变的命令行工具

Features:

- 许多预设渐变。
- 自定义渐变。
- 从 SVG 和 GIMP 渐变 (ggr) 文件中读取渐变 ...

[https://github.com/mazznoer/gradient-rs](https://github.com/mazznoer/gradient-rs)

## Relm4 v0.1 发布


在第一个测试版发布大约一个月后，经过无数个小时的工作，作者高兴地宣布Relm4的第一个稳定版本正式发布！

关于Relm4：

Relm4是一个受Elm启发并基于gtk4-rs的惯用GUI库。它是一个从头开始构建的relm的新版本，并且兼容gtk4和libadwaita。Relm4的主要目标是生产效率、灵活性、简单性和可维护性。

功能特性

- 支持libadwaita;
- 配套书籍GUI development with Relm4 已完结;
- 新增支持非阻塞IO的消息句柄;
- 更多的可复用组件;
- 许多其他的改进和修复;
- 完整的ChangeLog可以参见： [https://github.com/AaronErhardt/relm4/blob/main/CHANGES.md](https://github.com/AaronErhardt/relm4/blob/main/CHANGES.md)

更多信息：

- 项目地址：[https://github.com/AaronErhardt/relm4](https://github.com/AaronErhardt/relm4)
- 项目文档：[https://aaronerhardt.github.io/docs/relm4/relm4/](https://aaronerhardt.github.io/docs/relm4/relm4/)
- 参考书籍：[GUI development with Relm4](https://aaronerhardt.github.io/relm4-book/book/)

## Skiff: 一门用Rust编写的逐渐类型化的函数式编程语言

Skiff，是一门用Rust编写的逐渐类型化的函数式编程语言。所谓逐渐类型化是指作者计划下一步通过添加类型化关键字来区分完全类型函数和部分类型函数。

Skiff受`Elm/Pyret/Python`语言启发，并受`Rust/Javascript/Typescript/Haskell/OCaml/Lua`等语言影响，当前语言功能还在持续完善中，作者提供了一个由wasm!驱动的网页编辑器可供读者学习使用，更多信息请访问项目主页的Readme。

更多信息：

- 项目地址：[https://github.com/P-bibs/skiff/](https://github.com/P-bibs/skiff/)
- 网页编辑器：[https://skiff.paulbiberstein.me/](https://skiff.paulbiberstein.me/)

## htmlq

像 jq，但用于 HTML。使用 CSS 选择器从 HTML 文件中提取部分内容。

> jq 就像用于 JSON 数据的 sed - 您可以使用它来切片、过滤、映射和转换结构化数据，就像 sed、awk、grep 和朋友让您处理文本一样轻松。

htmlq 则用于处理 html 数据。

[https://github.com/mgdm/htmlq](https://github.com/mgdm/htmlq)

## zerocopy 0.6.0刚刚发布，带来了很多新的功能!

其中包括：`simd`和`simd-nightly`特性使分别支持 Stable 和 Unstable 的SIMD。

[https://docs.rs/zerocopy/0.6.0/zerocopy/](https://docs.rs/zerocopy/0.6.0/zerocopy/)

## Tabled 发布v0.3

Tabled 是一个易于使用的库，用于美化 Rust 结构和枚举的输出。

[https://github.com/zhiburt/tabled](https://github.com/zhiburt/tabled)

## ferros : 为 seL4 开发添加额外保证的 Rust 库

seL4 是 L4 微内核家族的成员，它为系统中运行的应用之间的隔离提供了最高级别保障，可以遏制系统某一部分的危害，并防止损害系统中其它可能更关键的部分。 据介绍，seL4 是世界上第一个通过数学方法被证明安全的操作系统内核，并且在安全的基础上还强调高性能，是世界上最快、最先进的OS 微内核。

ferros 围绕 seL4 功能提供智能类型安全包装器，重点是编译时资源跟踪。

ferros 建立在 selfe-sys 库之上。

[https://github.com/auxoncorp/ferros](https://github.com/auxoncorp/ferros)

## Matchbox: Rust wasm 中的 p2p 网络解决方案

Matchbox 的诞生是因为作者在rust 中制作了一款多人网页游戏，遇到了以下问题:

如何使用不可靠的、无序的 p2p connection 连接 N 个web浏览器?

[https://johanhelsing.studio/posts/introducing-matchbox](https://johanhelsing.studio/posts/introducing-matchbox)

## Sycamore: v0.6.0 版本发布了

Sycamore是一个用 Rust 和 WebAssembly 构建同构web应用程序的库. 目前发布了 0.6.0 版本了.

- 静态生成
- 服务端渲染
- 重验证
- 增量构建
- 开放构建矩阵
- CLI利用，让您轻松和自信地构建应用程序
- 充分利用 Fluent 开箱即用的 i18n 支持

- [https://sycamore-rs.netlify.app/news/announcing-v0.6.0](https://sycamore-rs.netlify.app/news/announcing-v0.6.0)
- [https://github.com/sycamore-rs/sycamore](https://github.com/sycamore-rs/sycamore)

## compact_str: 一种内存高效的不可变 string 类型

CompactStr 是一种内存效率更高的不可变字符串类型，它可以在堆栈上存储较小的字符串，并透明地在堆上存储更长的字符串。它们大多可以用作String的替换，在解析、反序列化或任何其他可能有较小字符串的应用程序中特别有用。

[https://github.com/ParkMyCar/compact_str](https://github.com/ParkMyCar/compact_str)

## 使用 Tarpaulin 进行 Rust 工程测试率覆盖

Tarpaulin是一个用于货物构建系统的代码覆盖率报告工具，它的名字来源于船上用来覆盖货物的防水布。目前，Tarpaulin提供的是工作线覆盖率，虽然相当可靠，但在结果中仍可能包含小的不准确之处。

Tarpaulin只支持运行Linux的x86_64处理器。这是因为在可执行文件中设置断点并跟踪其执行需要处理器和操作系统的特定代码。当达到更高的稳定性时，它的目标是增加更广泛的系统支持，然而这足以让Tarpaulin在Travis等流行的CI工具上运行。

它也可以在Docker中运行，这对你不使用Linux但又想在本地运行它的时候很有用，比如在开发期间。

Allen Wyma 与软件工程师 Daniel McKenna，也是 Tarpaulin 覆盖测试工具的作者的访谈节目。欢迎收听。

- [https://rustacean-station.org/episode/037-daniel-mckenna/](https://rustacean-station.org/episode/037-daniel-mckenna/)
- [https://github.com/xd009642/tarpaulin](https://github.com/xd009642/tarpaulin)

## Trunk - 一个 Rust 的 WASM web 应用打包器

Trunk 会打包 WASM，JS 代码片断，静态资源（images, css, scss 等）。它的配置使用 HTML 文件。

Trunk 支持所有基于 wasm-bindgen 的框架，包括但不仅限于 Yew 和 Seed。

官网：[https://trunkrs.dev/](https://trunkrs.dev/)

代码仓库：[https://github.com/thedodd/trunk](https://github.com/thedodd/trunk)

## Composing Studio - 协作编曲工具

Composing Studio 是一款支持在线实时协作的音乐编辑器，使用 Rust、WebAssembly 和 TypeScript 构建，允许任何人创建简单的音乐作品。

Composing Studio 使用一种名为 ABC 的文本格式来完成对音乐的编辑，可以用于转录简单的歌曲 + 吉他和弦，以及其他一些作品，如合唱和民间音乐。同时提供一个友好直观的 Web 界面，具有语法突出显示、实时预览、音频播放和实时协作等功能。

- [https://github.com/ekzhang/composing.studio](https://github.com/ekzhang/composing.studio)
- [https://composing.studio/productive-animal-5688](https://composing.studio/productive-animal-5688)

## termusic - 终端音乐播放器

termusic 是一款用 Rust 开发的终端音乐播放器，目前支持 mp3, m4a, flac 和 ogg/vorbis 多种格式。作者曾经是 GOMU 的贡献者，由于在开发时遇到像数据竞争这样的严重问题，所以使用 Rust 进行了重写。

[https://github.com/tramhao/termusic](https://github.com/tramhao/termusic)

## eztd：让 Rust 更易学习

项目致力于『可学习性和可控制』，目标包括：

- 低语法噪声
- 对 Python 开发者熟悉
- 允许优化内循环
- 与 Rust 生态互操作

- [https://github.com/epage/eztd#about](https://github.com/epage/eztd#about)
- [https://epage.github.io/blog/2021/09/learning-rust/](https://epage.github.io/blog/2021/09/learning-rust/)

## rustls 0.20 发布了

russtls 是一个 Rust 编写的现代的 TLS库。它使用ring进行加密，使用libwebpki进行证书验证。

目前已发布 0.20 版本.

[https://github.com/rustls/rustls](https://github.com/rustls/rustls)

## dune: 一个 Rust 写的 shell

[https://github.com/adam-mcdaniel/dune](https://github.com/adam-mcdaniel/dune)