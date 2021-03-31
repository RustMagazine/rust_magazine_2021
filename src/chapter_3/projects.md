# 本月简报 | 推荐项目

- 来源：[Rust日报](https://rustcc.cn/section?id=f4703117-7e6b-4caf-aa22-a3ad3db6898f)
- 作者：`Rust`日报小组
- 后期编辑：杨楚天

## lens-rs

- [仓库链接](https://github.com/TOETOE55/lens-rs)
- [文章链接](https://zhuanlan.zhihu.com/p/358678675)

lens-rs 是一个 lens 的 rust 实现。

## tinyvm

- [仓库链接](https://github.com/mkhan45/tinyvm)

tinyvm 是一个堆栈字节码虚拟机的最小化可用实现。这个 VM  可以运行一个简单的图灵完备的指令集。核心代码只有 250 行，并且有大量注释。

## maple

- [仓库链接](https://github.com/lukechu10/maple)

maple 是一个用 WASM 实现的响应式 DOM 库，没用到虚拟 DOM，而是在渲染过程中细粒度响应式地更新 DOM。

## byo-linker

- [仓库链接](https://github.com/andrewhalle/byo-linker)

byo-linker 是一个极简的链接器，用于帮助理解链接器的实现方法。

## rs_pbrt

- [仓库链接](https://github.com/wahn/rs_pbrt)
- [文档链接](https://www.janwalter.org/doc/rust/pbrt/index.html)
- [文章链接](https://www.rs-pbrt.org/blog/v0-9-0-release-notes/)

rs_pbrt 是经典书籍 *Physically Based Rendering: From Theory to Implementation* 的 rust 实现。

## flume

- [仓库链接](https://github.com/zesterer/flume)

flume 是一个 mpmc 的 channel 库，其用法和 `std::sync::mpsc` 基本一致，代码里没包含任何 `unsafe`。

## ferris-fetch

- [仓库链接](https://github.com/irevenko/ferris-fetch)

ferris-fetch 可以用于获取 rust 工具链以及操作系统的信息。

## Station Iapetus

- [仓库链接](https://github.com/mrDIMAS/StationIapetus)
- [文章链接](https://www.reddit.com/r/rust/comments/m8suco/station_iapetus_the_game_written_in_rust_based_on/)

Station Iapetus 是一个用 [rg3d](https://github.com/mrDIMAS/rg3d) 开发的第三人称射击游戏，仍处于早期开发阶段。

## Veloren

- [仓库链接](https://github.com/veloren/veloren)
- [主页链接](https://veloren.net/)

Veloren 是一个像素风的多人 RPG 游戏，其灵感来自《魔方世界》、《塞尔达传说:荒野之息》、《矮人要塞》和《我的世界》等游戏。

## mlc

- [仓库链接](https://github.com/becheran/mlc)

mlc 可以用于检查 html 和 markdown 中的无效链接。

## Kamu

- [仓库链接](https://github.com/kamu-data/kamu-cli)

![img](https://raw.githubusercontent.com/kamu-data/kamu-cli/master/docs/readme_files/dataset_graph.png)

Kamu 是 [Open Data Fabric](https://github.com/kamu-data/open-data-fabric) 的 rust 实现。

## MiniWASM

- [仓库链接](https://github.com/thedjinn/MiniWASM)

MiniWASM 是一个极简的 Rust WebAssembly 的项目模版。

## rkyv

- [仓库链接](https://github.com/djkoloski/rkyv)
- [文章链接](https://davidkoloski.me/blog/rkyv-is-faster-than/)

rkyv 是一个反序列框架，作者号称框架的速度比 serde_json 还要快。

## ter

- [仓库链接](https://github.com/schulke-214/ter)

ter 是一个 cli 程序，可以用类似自然语言的命令去做一些文字处理工作，例如过滤或者替换。

## ipipe 

- [仓库链接](https://github.com/Eolu/ipipe) 
- [文章链接](https://www.reddit.com/r/rust/comments/m0rh4p/im_making_a_crossplatform_namedpipe_api_in_rust/)

ipipe 是一个跨平台的命名管道库。

## Gloo

- [仓库链接](https://github.com/rustwasm/gloo)

Gloo 是一个模块化的工具箱库，可以用于 Wasm 项目的开发。

## aws-lambda-rust-runtime

- [仓库链接](https://github.com/awslabs/aws-lambda-rust-runtime)

aws-lambda-rust-runtime 是一个AWS Lambda Functions 的 runtime。

其中包括：

- `lambda-runtime` crate 用于提供 AWS Lambda 的 runtime
- `lambda-http` crate 用来写 AWS Lambda 的 API 网关代理事件

## synth

- [仓库链接](https://github.com/openquery-io/synth)

synth 是一个声明式的数据生成器，其主要特性有：

- 数据即代码
- 导入已有数据
- 数据自动推导
- 不限定特定数据库
- 语义化数据类型

## TiFS

- [仓库链接](https://github.com/Hexilee/tifs)

TiFS 是一个基于 TiKV 的分布式 POSIX 文件系统，具有分区容限和严格的一致性。

## 一个基于 wasm+rust+simd 技术栈实现的音乐合成器

[链接](https://notes.ameo.design/fm.html)

## InfluxDB IOx： 基于Apache Arrow 开发的新的 InfluxDB 核心 

- [仓库链接](https://github.com/influxdata/influxdb_iox)

InfluxDB是一个开源时间序列数据库

目前频繁开发中，正处于项目早期，感兴趣的可以及早关注

## Speedy2D: 兼容 OpenGL (ES) 2.0+ 的图像库

- [仓库链接](https://github.com/QuantumBadger/Speedy2D)

Speedy2D 是一个拥有硬件加速, 简单易上手的 API的图像库, 可以方便的绘制 各种形状, 图像 和 文本.

目标:

- 最简单的 Rust API 来创建 window, 渲染图像和文本, 处理输入.
- 兼容任意带有 OpenGL 2.0+ 和 OpenGL ES 2.0+ 的设备
- 非常快


## idcard-cn v0.0.1

- [仓库链接](https://github.com/huangjj27/idcard-cn)

过去的一周时间小编翻看了一些基于 Rust 的身份证识别库（如 https://crates.io/crates/rust-idcard ),基本上只提供了身份证证件号码和其他文本信息的读取，而缺少其他根据《中华人民共和国身份证法》需要提供的指纹和照片信息的读取。于是小编决定将这些信息结构化，并且统一为信息完全的特质库，并且提供了一些相应符合生活常识的类型对读取的身份信息进行处理


## Qovery Engine - Rust库,可在云服务上自动化部署应用程序

- [仓库链接](https://github.com/Qovery/engine)

Qovery Engine是一个开源抽象层库，仅需几分钟，它就可以轻松地在AWS，GCP，Azure和其他云提供商上部署应用程序。Qovery引擎是用Rust编写的，并利用Terraform，Helm，Kubectl和Docker来管理资源。

- 零基础架构管理： Qovery Engine为您初始化，配置和管理您的Cloud帐户。
- 支持多个云：Qovery Engine可以在AWS，GCP，Azure和任何云提供商上使用。
- 在Kubernetes之上： Qovery Engine在更高的抽象级别上利用了Kubernetes的功能。
- Terraform和Helm： Qovery Engine使用Terraform和Helm文件来管理基础结构和应用程序部署。
- 强大的CLI：使用提供的Qovery Engine CLI在您的Cloud帐户上无缝部署您的应用程序。
- Web界面： Qovery通过qovery.com提供Web界面。


## Linfa : Rust写的统计学习综合工具箱

- [官网](https://rust-ml.github.io/linfa/)
- [仓库地址](https://github.com/rust-ml/linfa)

## cargo-quickinstall 0.2.0版本发布

[cargo-quickinstall (https://crates.io/crates/cargo-quickinstall)] 有点类似于Homebrew的Bottles（二进制包）概念，但用于 Rust。

示例：

```
cargo quickinstall ripgrep
```

作者认为：在此之前，通常将二进制文件托管在Bintray（homebrew serves）上，但是该服务正在逐步淘汰，因此需要更换它。



## Rust编写的清理应用程序的 Czkawka 3.0.0发布

- [仓库地址](https://github.com/qarmin/czkawka)

完全用Safe Rust 和 gtk 实现，且跨平台，多功能应用程序，可查找重复项，空文件夹，相似图像等。


## GraphGate 0.3.2 发布.

- [仓库地址](https://github.com/async-graphql/graphgate)

用 Rust 实现的GraphQL API网关。

为什么要用Rust来实现？

​Rust是我最喜欢的编程语言。它既安全又快速，很适合开发API网关。

## libretranslate-rs

- [仓库地址](https://github.com/DefunctLizard/libretranslate-rs/)

一个可以替代谷歌翻译的自由/开源软件(Rust编写!)，使用的是 [libretranslate.com](https://libretranslate.com/)的 API。

## tide-acme：通過Let's Encrypt自動獲得HTTPS證書

- [仓库地址](https://github.com/http-rs/tide-acme)

Let's Encrypt 是個很常用的免費ssl證書服務

作者結合了tide與Let's Encrypt做了一個自動取得證書給tide使用的範例

## CleanIt: Rust实现的可以通过 gRPC 控制 Roomba 系列扫地机器人的框架

- [仓库地址](https://github.com/Sollimann/CleanIt)

还在开发中。

发现 [Roomba系列机器人吸尘器](https://www.irobot.cn/roomba/) 是中国广东的公司。

## task-stream 一个能运行在no_std的全局异步任务spawner

- [仓库地址](https://crates.io/crates/task-stream)

task-stream是一个全局任务spawner，可以在no_std中运行。

它提供了用于异步任务的spawner，以及异步延迟函数。

它是为库作者设计的。 在第三方库中，可以生成子任务，而无需关心执行程序主程序使用的子任务。

## Shipyard 0.5了

- [仓库地址](https://crates.io/crates/shipyard)

這是一個ECS框架 速度比上一版增加快了2倍

