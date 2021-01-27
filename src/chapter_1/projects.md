# 本月简报 | 推荐项目

## 「微软」Rust for Windows

- [仓库链接](https://github.com/microsoft/windows-rs)
- [文档链接](https://microsoft.github.io/windows-docs-rs/doc/bindings/windows)
- [crate 链接](https://crates.io/crates/windows)

这个仓库是 1 月 20 日微软发布的官方 Win32 API crate。

过去用 rust 为 Windows 开发应用程序时，若要调用 Win32 API，必须使用 [winapi-rs](https://github.com/retep998/winapi-rs) 这样的 wrapper 库，此类库需要社区去人工维护和 Win32 API 的绑定。
为了改善这点，微软通过 [win32metadata](https://github.com/microsoft/win32metadata) 项目来加强对 C/C++ 以外的编程语言的支持（[相关链接](https://blogs.windows.com/windowsdeveloper/2021/01/21/making-win32-apis-more-accessible-to-more-languages/)），
其中就包括对 rust 的支持。

现在已经有使用该库实现的[扫雷](https://github.com/robmikh/minesweeper-rs)程序, 除此之外，也有微软工程师发布了一些[示例项目](https://github.com/kennykerr/samples-rs)。

## Czkawka

- [仓库链接](https://github.com/qarmin/czkawka)
- [reddit 讨论](https://www.reddit.com/r/linux/comments/kjcbva/czkawka_200_multithread_support_similar_images/)

*Czkawka* 是一个多平台的空间清理应用，可用于找出系统中的重复的文件、空文件夹、临时文件等。

项目采用 gtk3/gtk-rs 开发 GUI 部分, 同时也提供 CLI 程序。

![czkawka](https://user-images.githubusercontent.com/41945903/103371136-fb9cae80-4ace-11eb-8d72-7b4c8ac44260.png)


## Artichoke

- [项目主页](https://www.artichokeruby.org/)
- [推特主页](https://twitter.com/artichokeruby)
- [仓库链接](https://github.com/artichoke/artichoke)
- [rubyconf 2019 上的相关演讲](https://www.youtube.com/watch?v=QMni48MBqFw&list=PLE7tQUdRKcyZDE8nFrKaqkpd-XK4huygU&index=37)

*Artichoke* 是一个由 rust 开发的 ruby 实现，可以将 ruby 代码编译至 WebAssembly。

当前 Artichoke 依然依赖于 mruby backend，在与 mruby 进行 FFI 交互的同时，改进某些 Kernel 和库函数的实现。例如 [regex](https://github.com/artichoke/artichoke/tree/trunk/artichoke-backend/src/extn/core/regexp) 部分就是由 rust 实现的。

作者表示在未来会开发出一个纯 rust 的实现。

## linfa

- [仓库链接](https://github.com/rust-ml/linfa)
- [文档链接](https://docs.rs/linfa/0.3.0/linfa/)
- [reddit 讨论](https://www.reddit.com/r/rust/comments/e4wh8c/linfa_taking_ml_to_production_with_rust_a_25x/)

*linfa* 是一个机器学习的框架和工具集，其设计参照了 python 的 `scikit-learn` 库。

关于 rust 在机器学习方面的生态系统，可以参考 [arewelearningyet](http://www.arewelearningyet.com/)。

## async-trait-static

- [仓库链接](https://github.com/tiannian/async-trait-static)
- [文档链接](https://docs.rs/async-trait-static/0.1.4/async_trait_static/)

*async-trait-static* 是一个用于在 trait 中声明 async 方法的库，可以在 `no_std` 下使用。

由于 rustc 的限制，[要在 trait 中写出 async 方法是很困难的](https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/)。
针对这个问题，dtolnay 实现了 [async-trait](https://github.com/dtolnay/async-trait)，将 `async fn` 的返回类型转化为 `Pin<Box<dyn Future>>`。

async-trait-static 则采用了 GAT 来实现这个功能，无需用到 trait object。

当前 rust 的 GAT 依然不够完善，因此该库还是有些功能是缺失的。

## regexm

- [仓库链接](https://github.com/TaKO8Ki/regexm)
- [文档链接](https://docs.rs/regexm/0.1.0-beta.1/regexm/)
- [示例](https://github.com/TaKO8Ki/regexm/tree/main/examples)

*regexm* 是一个用于对正则表达式进行模式匹配的库：

```rust
fn main() {
    let text1 = "2020-01-01";
    regexm::regexm!(match text1 {
        r"^\d{4}$" => println!("y"),
        r"^\d{4}-\d{2}$" => println!("y-m"),
        // block
        r"^\d{4}-\d{2}-\d{2}$" => {
            let y_m_d = "y-m-d";
            println!("{}", y_m_d);
        }
        _ => println!("default"),
    });
}
```

## swc

- [项目主页](https://swc.rs/)
- [仓库链接](https://github.com/swc-project/swc)

*swc* 是一个 typescript/javascript 的 transpiler，在运行速度上，单核比 babel 快 4 倍，4 核比 babel 快 70 倍，同时也具有 treeshaking 的功能。

*swc* 被用于 deno 项目中，用于类型擦除。 swc 的作者是一名 97 年的大二学生，如今已经获得了 Deno 官方的顾问合同。

## rlink-rs

*国产项目*

- [仓库链接](https://github.com/rlink-rs/rlink-rs)

rlink-rs是基于rust实现的流式计算引擎，用来作为Apache Flink的替代方案。

相对于在线业务，rlink-rs更关注海量数据的离线流式处理场景，提升吞吐能力、降低资源消耗。其特点是针对exactly once提供计算和输出两种语义；基于特殊的exactly once输出语义，结合rust内存管理模型，实现大部分场景的全内存计算，解决state和checkpoint引起的重量级IO操作。

rlink-rs的目标是成为一个计算驱动引擎，允许基于DAG定制你自己的计算流程、实现自己的计算语义。

目前状态：主要针对flink流计算这块做对比。已经实现基本窗口计算流程。

希望能从社区得到关于流引擎设计方面的帮助：

1.因为rust语言不如Java动态语言可以反射，在用户api上不那么优雅。
2.只是想在语义上实现类似flink的api，实现上还是想走一条新的路线，毕竟flink有历史包袱，它的实现我们不需要100%参考。

## Rapier 2021的路线图

Rapier 是一个完全免费的开源物理引擎，可用于游戏，动画和机器人，完全使用 Rust 编程语言编写。 它着重于性能，可移植性和跨平台确定性（可选）。

Rapier 团队希望到2021年年底，Rapier 具有游戏物理引擎所期望的所有功能，实现流行的 C++ 物理引擎，比如：Box2d，Bullet Physics 和 PhysX 等同等的功能， 但是不打算在 GPU 上支持运行物理仿真。

2021 路线图链接：[https://www.dimforge.com/blog/2021/01/01/physics-simulation-with-rapier-2021-roadmap/](https://www.dimforge.com/blog/2021/01/01/physics-simulation-with-rapier-2021-roadmap/)

## Psst：使用Rust和Druid构建的第三方Spotify客户端

- [仓库链接](https://github.com/jpochyla/psst)

Psst 是一款GUI的快速Spotify客户端，不带Electron，内置Rust。

[Druid](https://www.reddit.com/r/rust/comments/ksgtk7/druid_v070/)是一个原生Rust GUI库，支持Windows，macOS，Linux，之前是xi-editor的一部分。

## slotmap: 1.0 released

- [仓库链接](https://github.com/orlp/slotmap)

slotmap 提供了三种 map 的实现, SlotMap, HopSlotMap 和 DenseSlotMap.

增加,删除,查询均为O(1)复杂度,而且额外开销非常低. 非常适合存储需要稳定和安全引用的 objects, 例如游戏中的 entities, graph 中的 nodes.

## Rust 的 WebDriver库

- [仓库链接](https://github.com/stevepryde/thirtyfour)

Thirtyfour是一个用于Rust的Selenium / WebDriver库，用于自动化网站UI测试。

它支持完整的W3C WebDriver规范。经过Chrome和Firefox的测试，尽管任何与W3C兼容的WebDriver都可以使用。


## webrtc.rs

- [官网](https://webrtc.rs/)
- [仓库链接](https://github.com/webrtc-rs/webrtc)

用 Rust 重写 Pion WebRTC (http://Pion.ly)。目前 v1.0 仍然处于开发中，欢迎开源贡献者提PR。

## Rust中的科学计算

- [文章链接](https://aftix.xyz/home/bacon/)
- [仓库链接](https://github.com/aftix/bacon)

这篇文章中作者分享了在课余时间用Rust重写生物膜仿真过程中遇到的问题。

由于crates.io上找不到SciPy的代替品，作者自己实现了一个bacon-sci。



## shadow-rs 0.5.14 支持自定义钩子

- [仓库链接](https://github.com/baoyachi/shadow-rs)

shadow-rs是一个使得程序能在运行时读取到编译过程中信息的库，这些信息包括：

- Cargo.toml 中的项目版本
- 依赖信息
- git commit
- 编译中用到的Rust工具链
- build类型，debug版还是release版

之前想要增加加自定义信息会很麻烦，在0.5.14支持了自定义钩子后就容易多啦。



## Ballista：分布式计算平台

- [仓库链接](https://github.com/ballista-compute/ballista)

Ballista 用 Rust 实现的概念验证分布式计算平台，使用 Apache Arrow 作为内存模型。它建立在一种体系结构之上，这种体系结构允许将其他编程语言作为一级公民进行支持，而不需要为序列化付出代价。


## 德国亚琛工业大学研究项目：RustyHermit 介绍

- [RustyHermit 介绍文章](https://rust-osdev.com/showcase/rusty-hermit/)

相关链接：

- [Phil-Opp OS 教程](https://os.phil-opp.com/)
- [libhermit-rs](https://github.com/hermitcore/libhermit-rs)
- [rusty-hermit](https://github.com/hermitcore/rusty-hermit)

RustyHermit 是一个 Unikernel（我理解这就是  Unique-Kernel 的缩写，独立内核？）。 Unikernel 被认为是有可能改变未来云生态格局的技术。

Unikernel是使用libOS(library os)构建的具有专门用途的单地址空间机器镜像。为了支撑程序的运行，开发者从模块栈中选择最小的类库集合，构建对应的OS。类库和应用代码、配置文件一起构建成固定用途的镜像，可以直接运行在hypervisor或者硬件上而无需Linux或者Windows这样的操作系统。所以，也有人称它为下一代容器技术。

Unikernel 其最大的卖点就是在，没有用户空间与内核空间之分，只有一个连续的地址空间。这样使得 Unikernel 中只能运行一个应用，而且对于运行的应用而言，没有硬件抽象可言，所有的逻辑，包括应用逻辑和操作硬件的逻辑，都在一个地址空间中。

但是目前 Unikernel 仍然出于研究阶段。

RustyHermit 是依赖于 libhermit-rs（库操作系统）实现的。

这两个项目都出自 亚琛工大，有意思的是，它们都是基于著名的 Rust实现操作系统教程phil-opp 衍生实现的。


## 用 Rust 编写现代操作系统

[仓库链接](https://github.com/theseus-os/Theseus)

Theseus 是从Rust编写的新操作系统，尝试使用新颖的OS结构，更好的状态管理以及如何将OS职责（如资源管理）转移到编译器中。

我们一直在不断改进操作系统，包括其故障恢复能力，以提供更高的系统可用性而没有冗余，以及更轻松，更随意的实时演进和运行时灵活性。尽管仍然是一个不完整的原型，但我们认为These修斯将对高端嵌入式系统或边缘数据中心环境很有用。请参阅我们的已发表论文，以获取有关These修斯的设计原理和实现理念的更多信息，以及我们避免状态泄漏现象或尽可能减轻其影响的目标。

## Evcxr: A Rust REPL 的解决方案

并且它还包含了 Jupyter Kernel 指南

该项目挂在 Google 的 GitHub 组织下。

- [仓库链接](https://github.com/google/evcxr)
- [Jupyter Kernel 指南](https://github.com/google/evcxr/blob/master/evcxr_jupyter/samples/evcxr_jupyter_tour.ipynb)

## Findomain: 可提供子域监视服务

- [仓库链接](https://github.com/Findomain/Findomain)

该服务可提供：目录模糊处理/端口扫描/漏洞发现（使用Nuclei），等等。 

允许您使用多个顶级工具（OWASP Amass，Sublist3r，Assetfinder和Subfinder）监视目标域，并在出现新的子域时将警报发送到Discord，Slack，Telegram，电子邮件或推送通知（Android / iOS / Smart Watch / Desktop）。 

您唯一要做的就是使用您的电子邮件地址（如果适用）或/和webhooks / Telegram聊天信息配置文件，然后将域放入另一个文件中。

一旦完成，您便拥有了一个完全自动化的子域监视服务，可以让您 包含最新发现的新子域，主机IP，HTTP状态，HTTP网站的屏幕快照，开放端口，子域CNAME等。 您所有的数据都安全地保存在关系数据库中，您可以随时请求转储数据。

## Weylus：让你的平板电脑用作电脑上的图形平板/触摸屏

特点：
- 用平板电脑控制鼠标
- 将屏幕镜像到平板电脑上

上述功能在所有操作系统上都可以使用，但`Weylus`在`Linux`上效果最好。`Linux`上的其他功能有：
- 支持手写笔/笔（支持压力和倾斜）。
- 多点触控。用支持多点触控的软件试试，- 比如Krita，你就会知道了。
- 捕捉特定的窗口，并只对其进行绘制。
- 更快的屏幕镜像
- 硬件加速视频编码
- 平板电脑作为第二屏幕

- [仓库链接](https://github.com/H-M-H/Weylus)