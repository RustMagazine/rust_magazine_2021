# 推荐项目 ｜ 基础工具库

编辑： 张汉东

---

## pyre-http：用 Rust 实现的 Python HTTP Server

目前还未到达生产可用级别。

pyre-http 几乎完全用 Rust 实现，为了兼容 python 的 ASGI/Asyncio生态系统，还使用了 PyO3。

具体来说，基本上用纯 Rust 重写了整个 asyncio 服务器和协议API，将Python调用减少到平均每个请求只有1-2个调用。

作者的一些心得：

> Rust和Python在一起工作很好，但有些地方开销很大。
>
> 在Rust中为异步Python写异步代码，需要从完全不同的角度来看待你在Python中写的东西，你最好以事件驱动的状态机风格来写代码，而不是Python中正常的基于回调的Future。
> 
> 编写异步Python并不意味着编写高级的异步Rust，公平地说，在大多数情况下Python也是如此；通常你会期望看到大多数服务器代码是完全同步的。

[https://github.com/Project-Dream-Weaver/pyre-http](https://github.com/Project-Dream-Weaver/pyre-http)

## Fang: 后台任务处理库

作者处理后台任务的心路历程：

1. 最初的方法（天真烂漫）

在同一个线程中执行tokio任务中的每个同步工作。在并发量大的时候，出现了问题：有一些同步任务根本没有执行。开发者对tokio任务没有任何控制权，所以没有办法检查任务的总数和当前正在执行的任务等。

有趣的是，作者在2020年还为此写过一篇文章：[在 Rust 里，你不需要后台任务框架](https://www.badykov.com/rust/2020/06/28/you-dont-need-background-job-library/)

然而，现在作者萌生了实现简单后台任务处理库的想法。

2. Fang 的方案（简单，但不天真）

    a. 任务被存储在 Postgres 数据库中。
    b. Fang 启动指定数量的 Worker，每个 Worker 就是一个独立线程，用于执行任务。
    c. 如果数据库中没有剩余的任务，Worker就会在指定的秒数内休眠。
    d. 如果任何 worker 在任务执行过程中出现故障，它将被重新启动。

Fang 的方案确实简单粗暴，并且还对 Postgres 数据库绑定很深。因为 需要保证每个 Worker 必须对任务只处理一次，所以 Fang 依赖了 Postgres 数据库的 `Locks(FOR UPDATE SKIP LOCKED)`。

个人观点：

这种方案其实比较传统，比如 Ruby 的 delayed_job ，或者 Sidekiq（依赖于 redis）。

其实在 Rust 社区，也有不少后台任务库，但很多就不维护了，可能和 Rust 异步生态不太稳定有关系。

有一个目前还维护（但不积极）的语言无关的后台任务库：[ocypod](https://github.com/davechallis/ocypod)，是基于 redis 的，值得关注。但依赖的还是 tokio 0.2 。

Jonhoo 实现了一个 [faktory-rs](https://github.com/jonhoo/faktory-rs)，是 高性能任务系统 Faktory 的 Rust 客户端和 worker 实现。[Faktory](https://github.com/contribsys/faktory) 是 Sidekiq 作者（实现财富自由以后）的新产品(go语言实现)，支持异构系统，可以用任意语言做生产者和消费者。虽然褒贬不一，但毕竟作者已经有一个非常成功的Sidekiq实现了。

Faktory 特点：

- Faktory server（not worker）支持 retry 等特性
- 不特定依赖 redis，内置RocksDB做消息存储
- 提供类似 Sidekiq 的 WebUI
- 支持异构系统，并且保留了从 Sidekiq 积累下的一些好用的特性
- Faktory 的接口格式很简单，核心是 queue、 jobtype 和 args
- 目前国内云服务商不提供 Faktory 相关服务，需要自己维护
- 已经发布1.5.1 版本

Fang 相关链接：

- [https://github.com/ayrat555/fang](https://github.com/ayrat555/fang)
- [介绍文章](https://www.badykov.com/rust/2021/06/27/fang/)


## enarx : 在可信执行环境的应用部署系统

据说是红帽和一些知名企业一起搞的。

Enarx是独立于CPU架构的，使相同的应用程序代码可以在多个目标上部署，抽象出诸如交叉编译和硬件供应商之间不同的认证机制等问题。目前，支持 AMD SEV和英特尔SGX的工作正在进行中。

Enarx 使用 WebAssembly 作为其隔离和可移植性的基础。


[https://github.com/enarx/enarx](https://github.com/enarx/enarx)

## cargo-c ：构建并安装C-ABI兼容的动态和静态库。

它生成并安装正确的PKG-Config文件，静态库和动态库，以及任何C（和C兼容）软件使用的C头。

[https://github.com/lu-zero/cargo-c](https://github.com/lu-zero/cargo-c)

## macroquad游戏引擎分离出音频系统

macroquad 游戏引擎在最近将其音频系统给单独提取出来，作为一个更通用的Rust包，该crate是对多个平台的多个音频后端的统一抽象，目前完成度如下：

- Web: WebAudio
- Android: OpenSLES
- Linux: Alsa
- Mac: CoreAudio
- Windows: Wasapi
- IOS: CoreAudio(?)

[https://github.com/not-fl3/quad-snd](https://github.com/not-fl3/quad-snd)

## Gloo v0.3.0 发布

Gloo 团队很高兴地宣布一个新的、姗姗来迟的 Gloo 版本：v0.3.0。Gloo 是一个模块化工具包，用于使用 Rust 和 WASM 构建快速、可靠的 Web 应用程序和库。

[https://gloo-rs.web.app/blog/release-0.3.0](https://gloo-rs.web.app/blog/release-0.3.0)

## Throne ： 用于游戏原型设计和故事逻辑的脚本语言

@tobmansf 一直在研究用于游戏原型设计和故事逻辑的脚本语言。它可以编译成 WebAssembly，可以在 [https://t-mw.github.io/throne-playground/](https://t-mw.github.io/throne-playground/) 上试一试。

[https://github.com/t-mw/throne](https://github.com/t-mw/throne)

## 零成本反序列化框架 rkyv 发布 0.7.1 版本

[https://github.com/djkoloski/rkyv](https://github.com/djkoloski/rkyv)

## 一个 Rust 和 TypeScript 实现的 体素（Voxel）引擎

基于 actix-web 实现

- [https://github.com/ian13456/mine.js](https://github.com/ian13456/mine.js)
- 在线玩：[https://mine.iantheearl.io/?world=terrains](https://mine.iantheearl.io/?world=terrains)

## 慢啃 Rust 系列   | Gazebo 库 之 Dupe

原文标题：Rust Nibbles - Gazebo : Dupe 

「Rust Nibbles」 翻译成 「慢啃 Rust 」 没毛病吧 ？ 

这是 Facebook for Develpers 网站出的Rust Nibbles系列文章，介绍 facebook 开源的各种 Rust 库。

Gazebo 是 facebook 工程师 编写的基础库，Gazebo以独立模块的形式包含了一系列经过测试的Rust实用程序。这篇文章是介绍了 Gazebo 中的 Dupe trait 。

在Rust中，有两个用于 "复制 "一个值的相关特性--Copy和Clone。

在Gazebo中引入了第三个类似的trait，称之为Dupe，它可以在Gazebo Prelude中使用。（dupe 有复制物品/复制底片的意思）。

Copy 是 编译器的自动行为，复制成本也不高。而 Clone 则不然。为了降低 Clone  的成本，一般可以使用 Arc，但是 Arc 使得代码阅读成本提升。比如 `let xs = ys.clone();`，你可能需要查看大量上下文来弄清是 调用了 Clone 还是 Arc 。当然你可以使用 `let xs = Arc::clone(ys)`来提升可读性，但缺点是，它破坏了抽象。

所以，Gazebo 中引入了 Dupe trait, `let xs = ys.dupe()`。

```rust
use gazebo::prelude::*;
#[derive(Clone, Dupe)]
struct MyArc(Arc<String>);
```

看了一下实现源码：[https://github.com/facebookincubator/gazebo/blob/master/gazebo/src/dupe.rs](https://github.com/facebookincubator/gazebo/blob/master/gazebo/src/dupe.rs)

```rust
pub trait Dupe: Clone {
    fn dupe(&self) -> Self {
        self.clone()
    }
}
```

看上去和 Clone 很像，但它仅在 常量时或零分配下可用，比如 Arc。因为 Dupe 只给这些类型实现了。

[https://developers.facebook.com/blog/post/2021/07/06/rust-nibbles-gazebo-dupe/](https://developers.facebook.com/blog/post/2021/07/06/rust-nibbles-gazebo-dupe/)

## 想用 Rust  写脚本吗？ 

rust-script ，可以在没有任何设置或编译步骤的情况下运行rust 文件和表达式。

```rust

$ echo 'println!("Hello, World!");' > hello.rs
$ rust-script hello.rs
Hello, World!
```

也支持 依赖 crate

```rust

#!/usr/bin/env rust-script
//! This is a regular crate doc comment, but it also contains a partial
//! Cargo manifest.  Note the use of a *fenced* code block, and the
//! `cargo` "language".
//!
//! ```cargo
//! [dependencies]
//! time = "0.1.25"
//! 
fn main() {
    println!("{}", time::now().rfc822z());
}

```

```rust
$ rust-script now
Wed, 28 Oct 2020 00:38:45 +0100
```

- [https://rust-script.org/](https://rust-script.org/)
- [https://github.com/fornwall/rust-script](https://github.com/fornwall/rust-script)

## Quilkin : 一个用于游戏服务器的开源UDP代理

由embark 和 Google Cloud 共同推出，目标是为任何游戏工作室提供和巨头同等的网络功能。

[https://medium.com/embarkstudios/say-hi-to-quilkin-an-open-source-udp-proxy-88577c795204](https://medium.com/embarkstudios/say-hi-to-quilkin-an-open-source-udp-proxy-88577c795204)

## franzplot ： Rust 实现的教学软件

米兰理工大学的一名研究助理，担任了 “设计的曲线和表面”课程的助教，这门课主要是为设计专业的学生解释三维数学概念。 因为没有趁手的教学工具，所以这位助教自己用 Rust 实现了一个。

第一个版本是 cpp 实现的。然后新版本用 Rust 重写了，为什么呢？

1. 他在cpp版本内亏欠的技术债务太多，不利于开源协同
2. OpenGL 已经被苹果废弃
3. 想让工具变得更加强大

所以，现在用 WebGPU + Rust 重写了这个工具。基于 [https://github.com/gfx-rs/wgpu](https://github.com/gfx-rs/wgpu)

FranzPlot目前是闭源的，未来可能会开源。因为尽管重新写了软件，也还需要处理一些技术债务。另外想完全使用 WGSL 而抛弃 GLSL ，还想将 界面替换为 纯 Rust 实现，比如 使用egui框架。现在是用了 imgui-rs。

[https://gfx-rs.github.io/stories/franzplot](https://gfx-rs.github.io/stories/franzplot)

## delicate 一个轻量的分布式的任务调度平台

特色大概有几点：

1. 丰富Rust生态。是一个中小型项目（代码量3W+）， 涉及的交互端包括（Front-end , Server ,  agent）  js & rust 的交织， 可以让新同学作为参考实现一个Rust的工程。
2. 里面的面对C端的交互都尽量设计的符合用户习惯，除了性能之外 项目很关心人使用的舒适度。
3. 里面有一些新定义的概念比如绑定，任务不直接关联机器而是关联机器的一个抽象（绑定），当有成百上千的任务需要从  A 机器迁移到 B 机器 ，只需要修改一次关联关系，任务就自动完成了迁移。

[https://github.com/BinChengZhao/delicate](https://github.com/BinChengZhao/delicate)

## 一个安全的可用于 `#[no_std]` ASN.1 的 解码器框架

[https://github.com/XAMPPRocky/rasn](https://github.com/XAMPPRocky/rasn)

## Rust 实现的 Windows 上的下一代包管理器，非常快

现在只发布了 Alpha 版本，但是已经比目前 Windows 自带的包管理器快 5 倍了。

[https://github.com/novus-package-manager/novus](https://github.com/novus-package-manager/novus)

## Loadstone 发布，一个安全的裸金属的 bootloader
Rust 开始写 bootloader 了。看起来这是一个相当严肃的项目，目前已经发布了 1.0.0。提供了如下功能：

- 多镜像操作：存储，拷贝，升级，验证和启动。可灵活配置
- 支持外部 flash 芯片
- Golden image rollbacks （怎么翻译？）
- 自动或应用触发的升级

搞底层的同学，请严重关注一下此项目。

[https://github.com/absw/loadstone](https://github.com/absw/loadstone)

## 一个研究型的 OS：NrOS

Node Replicated Kernel (NRK) 最早是在 VMware Research 里面的一个原型研究型 Os 项目，现在已经做出了一些学术成果。主要特点是要探索未来操作系统的结构。

[https://nrkernel.systems/](https://nrkernel.systems/)

## Sycamore v0.5.0发布

Sycamore 是一个用于在 Rust 和 WebAssembly 中构建同构 Web 应用程序的库。v0.5.0 版本是Sycamore迄今为止最大的版本，包含大量新功能和错误修复。其中这个版本还引入了功能齐全的路由系统

文章链接：[https://sycamore-rs.netlify.app/news/announcing-v0.5.0](https://sycamore-rs.netlify.app/news/announcing-v0.5.0)

## nanorand v0.6 发布

nanorand 是一个快速、轻量、高效的随机数生成器，其提供的 nanorand::WyRand 在 M1 Macbook Air 上速度可达 16.4 GB/s。

目前 nanorand 发布了 v0.6 ，RandomGen 支持带符号整数和浮点数，RandomRange 支持带符号整数。

[https://github.com/Absolucy/nanorand-rs](https://github.com/Absolucy/nanorand-rs)

## Hora 0.1.0

Hora，Rust 实现的近似最邻近搜索（Approximate Nearest Neighbor Search, ANNS）算法库。先发布了 v0.1.0，专注于近似最邻近搜索领域，已经实现了 HNSW（Hierarchical Navigable Small World Graph Index）索引，SSG（Satellite System Graph）索引，PQIVF（Product Quantization Inverted File）索引，BruteForceIndex，其他索引也即将推出。

Hora 可以部署在任何操作系统平台上，已经支持的 PC 操作系统 Linux，Mac OS，Windows，将支持移动设备操作系统 IOS 和Android，以及将来支持嵌入式系统（no_std），并将支持多语言绑定，包括 Python，Javascript，Java，Ruby，Swift 和 R。

相关链接信息：[https://github.com/hora-search/hora](https://github.com/hora-search/hora)