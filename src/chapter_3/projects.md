# 本月简报 | 推荐项目

- 来源：[Rust日报](https://rustcc.cn/section?id=f4703117-7e6b-4caf-aa22-a3ad3db6898f)
- 作者：`Rust`日报小组

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

