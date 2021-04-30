# GitHub 趋势榜

编辑：张汉东

---

盘点一下 本月 [GitHub 趋势榜上榜的 Rust 项目](https://github.com/trending/rust?since=daily)。

**从趋势榜中看得出来，「Rust 嵌入式/ WebAssembly/ 网络服务/ 性能遥测/ Web/ 云计算/ 游戏/ GUI」是 Rust 相对受关注比较多的领域。**

## Top 1: [rust-embedded / rust-raspberrypi-OS-tutorials](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials)

该项目是 Rust 嵌入式工作组维护的一个教程项目，使用 Rust 实现一个基于 树莓派的操作系统。这个月热度很高，是因为最近该项目活跃度又提升了，加了不少更新。

## Top 2: [TheAlgorithms/Rust](https://github.com/TheAlgorithms/Rust)

该项目是教育为目的的 Rust 算法实现库。近两个月更新不太活跃，但是却上了本月的趋势榜，可能在某些技术媒体做了宣传？

## Top 3: [bytecodealliance/wasmtime](https://github.com/bytecodealliance/wasmtime)

该项目为字节码联盟成员 Fastly 公司的 WebAssembly 的编译器和运行时，基于 Cranelift 。最近字节码联盟比较活跃，成立了新的开源组织，吸收了新成员，所以受关注比较多。

## Top 4: [LemmyNet/lemmy](https://github.com/LemmyNet/lemmy)

Lemmy 是基于 actix-web 开发的 仿 reddit 开源论坛。最近发布了新版本。

## Top 5: [rustdesk/rustdesk](https://github.com/rustdesk/rustdesk)

RustDesk 是国内一位开发者发布的开源项目，号称最好的远程桌面客户端，基于 Rust 和 [sciter](https://sciter.com/)实现。我邀请过 RustDesk 作者写文分享这个产品的开发心得（主要想了解 Windows 开发方面的心得 ），但是遭到作者以不会写文章为由推辞（汗），但是该库代码是完全开源的，想学习的朋友可以看源码学习吧。

## Top 6: [firecracker-microvm/firecracker](https://github.com/firecracker-microvm/firecracker)

Firecracker 是 AWS 开源的 用于 severless 计算的 microVMs。

## Top 7: [yewstack/yew](https://github.com/yewstack/yew)

允许你使用 Rust 和 WebAssembly 构建 Web App 的客户端。最近又有了活跃的更新。

## Top 8: [dani-garcia/vaultwarden](https://github.com/dani-garcia/vaultwarden)

用Rust 实现的非官方 Bitwarden 兼容服务器。并且与上游Bitwarden客户端*兼容，非常适合自托管部署，在这种情况下，运行官方资源密集型服务可能不理想。延伸阅读：[登录信息就该自己掌握：基于私有云的 Bitwarden 迁移指南](https://sspai.com/post/61976)

## Top 9: [novifinancial/winterfell](https://github.com/novifinancial/winterfell)

构建分布式 STARK 证明程序的实验项目。这是和 零知识证明 相关的库。

警告：这是一个研究项目。 它未经审核，可能包含错误和安全漏洞。 此实现尚未准备好用于生产。

## Top 10: [KOBA789/relly](https://github.com/KOBA789/relly)

一个小型的 关系数据库管理系统(RDBMS)  实现，以了解RDBMS的工作方式。

## Top 11: [extrawurst/gitui](https://github.com/extrawurst/gitui)

Rust 实现的高性能终端 Git UI。刚发布了新版本。

## Top 12: [mozilla/glean](https://github.com/mozilla/glean)

Mozilla 出的现代高性能跨平台遥测（Telemetry）库，Glean有两种实现，总共支持5种不同的编程语言。 

[Glean Book](https://mozilla.github.io/glean/book/index.html)

## Top 13: [meilisearch/MeiliSearch](https://github.com/meilisearch/MeiliSearch)

MeiliSearch是功能强大，快速，开源，易于使用和部署的搜索引擎。 搜索和索引编制都是高度可定制的。 开箱即用的功能包括拼写错误，过滤器和同义词。这有一篇 [MeiliSearch CEO 访谈](./meili_search.md)

## Top 14: [timberio/vector](https://github.com/timberio/vector)

Vector是高性能的端到端（代理和聚合器）可观察性数据管道，可让开发者控制可观察性数据。 开源，并且比其他任何方式快10倍。这也是一个非常优秀的开源项目，国内豆瓣在用它。[QuickStart](https://vector.dev/docs/setup/quickstart/)。

## Top 15: [serenity-rs/serenity](https://github.com/serenity-rs/serenity)

这是 Discord API 的 Rust 库，可以用它开发 Discord Bot 。

## Top 16: [EmbarkStudios/rust-gpu](https://github.com/EmbarkStudios/rust-gpu)

EmbarkStudios 出品的 GPU 库，目标是让 Rust 成为 GPU 编码的一流语言和生态。EmbarkStudios 公司也是 Rust Game 工作组的成员，为 Rust 游戏领域贡献不少库。最近该公司也加入了字节码联盟。

## Top 17: [solana-labs/solana](https://github.com/solana-labs/solana)

网络级（Web-Scale）区块链，用于快速，安全，可扩展，去中心化的应用程序和市场。去年 Solana 还赞助了 RustChinaConf。

## Top 18: [Kethku/neovide](https://github.com/Kethku/neovide)

一个 Neovim 简单图形用户界面。

## Top 19: [hyperium/tonic](https://github.com/hyperium/tonic)

tonic是基于HTTP/2 的 gRPC 实施，专注于高性能，互操作性和灵活性。 创建该库是为了对async / await提供一流的支持，并充当用Rust编写的生产系统的核心构建块。 最近几天在努力为发新版（0.4.3）做准备。

## Top 20: [tonarino/innernet](https://github.com/tonarino/innernet)

允许你创建使用 WireGuard 的专用网络系统。 WireGuard 是一款新型虚拟专用网络，旨在替代 IPSec 和 OpenVPN。它的设计目标就是简单而且安全，并且充分利用[噪声协议框架（Noise Protocol Framework）](https://noiseprotocol.org/)等新技术。

内部网的目标与Slack的 [nebula](https://github.com/slackhq/nebula) 或 [Tailscale](https://tailscale.com/) 类似，但采取了不同的方法。它旨在利用现有的网络概念（如CIDR）和WireGuard的安全属性，将计算机的基本IP网络转变为功能更强大的ACL原语。

innernet不是官方的WireGuard项目，WireGuard是Jason A. Donenfeld的注册商标。

该软件尚未接受独立的安全审核，因此应在其生命周期的早期阶段视为试验性软件。

## Top 21: [tauri-apps/tauri](https://github.com/tauri-apps/tauri)

允许开发者使用Web前端构建更小，更快和更安全的桌面应用程序。最近发布了 1.0-beta 版。[知乎：如何评价 tauri？](https://www.zhihu.com/question/396199869)

## Top 22: [linebender/druid](https://github.com/linebender/druid)

Druid是一个实验性的Rust-native UI工具箱。 其主要目标是提供完善的用户体验。最近也是开发比较活跃。

## Top 23: [bevyengine/bevy](https://github.com/bevyengine/bevy)

Bevy 可以说是当下最火的 Rust 游戏引擎了。最近发布了0.5版本，对其 ecs 引擎升级到了 V2。

## Top 24: [gfx-rs/wgpu-rs](https://github.com/gfx-rs/wgpu-rs)

 wgpu 的 Rust 绑定库。 它的设计适合于Rust社区的通用图形和计算需求。wgpu-rs可以直接针对本地支持的后端和WASM。正在准备发布新版(0.8)。


