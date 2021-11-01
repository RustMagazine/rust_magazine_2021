# GitHub 趋势榜

编辑：张汉东

> 说明：记录一下本月在 GitHub 趋势榜单出现的 新的项目。
>
> 排名不分先后

---

## keyscope: 用Rust构建的 `key/secret` 工作流工具

由`service_policy_kit`提供支持。

[https://github.com/SpectralOps/keyscope](https://github.com/SpectralOps/keyscope)

## universal-android-debloater: 用Rust编写的跨平台 Debloat GUI

使用ADB对未root的android设备进行debloat。改善你的隐私，安全和你的设备的电池寿命。

免责声明：该软件仍处于开发的早期阶段，使用它需要您自担风险。对于您手机上可能发生的任何事情，该库作者概不负责。

[https://github.com/0x192/universal-android-debloater](https://github.com/0x192/universal-android-debloater)

## ruffle:  Rust 实现的 Adob​​e Flash Player 模拟器

基于 Rust 和 WebAssembly 实现。

[https://github.com/ruffle-rs/ruffle](https://github.com/ruffle-rs/ruffle)

## chumsky: 一个友好的解析器组合器

使编写具有错误恢复和部分解析功能的`LL(k)`解析器变得容易。

[https://github.com/zesterer/chumsky](https://github.com/zesterer/chumsky)

## onefetch: 一个用命令行 Git 信息工具

可以直接在终端上显示本地 Git 存储库的项目信息和代码统计信息。该工具完全离线，不需要网络访问。

[https://github.com/o2sh/onefetch](https://github.com/o2sh/onefetch)

## solana-program-library： Solana 维护的链上程序的集合

Solana 程序库 (SPL) 是一系列针对 [Sealevel 并行运行时](https://medium.com/solana-labs/sealevel-parallel-processing-thousands-of-smart-contracts-d814b378192)的链上程序。这些程序针对 Solana 的 Sealevel 实现、solana-runtime 进行了测试，并部署到其主网上。

[https://github.com/solana-labs/solana-program-library](https://github.com/solana-labs/solana-program-library)

## aliyundrive-webdav: 阿里云盘 WebDAV 服务

阿里云盘 WebDAV 服务，主要使用场景为配合支持 WebDAV 协议的客户端 App 如 Infuse、nPlayer 等实现在电视上直接观看云盘视频内容， 支持上传文件，但受限于 WebDAV 协议不支持文件秒传。

[https://github.com/messense/aliyundrive-webdav](https://github.com/messense/aliyundrive-webdav)

## metaboss: Solana Metaplex NFT“瑞士军刀”工具

这是一个年轻生态系统的实验性软件。使用风险自负。作者不对软件的滥用或在用于生产 NFT 之前未能测试特定命令负责。

[https://github.com/samuelvanderwaal/metaboss](https://github.com/samuelvanderwaal/metaboss)

## nearcore: NEAR 协议的参考客户端

NEAR的组件之一是 NEAR 协议，这是一种用于无服务器应用程序和由区块链提供支持的智能合约的基础设施。NEAR 协议旨在以以太坊等区块链收费的一小部分价格提供 Firebase 等现代 PaaS 的可用性和可扩展性。

[https://github.com/near/nearcore](https://github.com/near/nearcore)

## mullvadvpn-app: 适用于桌面和移动设备的 Mullvad VPN 客户端应用程序

此存储库包含该应用程序的桌面和移动版本的所有源代码。对于桌面，这包括系统服务/守护程序 ( mullvad-daemon)、图形用户界面 ( GUI ) 和命令行界面 ( CLI )。Android 应用程序对隧道和安全使用相同的支持系统服务，但在`android/` 中有一个专用前端。iOS 由一个完全独立的实现组成，它驻留在`ios/` 中。

- [Mullvad VPN](https://mullvad.net/en/)
- [https://github.com/mullvad/mullvadvpn-app](https://github.com/mullvad/mullvadvpn-app)

## hyperfine: 命令行基准测试工具 

[https://github.com/sharkdp/hyperfine](https://github.com/sharkdp/hyperfine)

## lighthouse :  Ethereum 2.0 客户端

用 Rust 编写并由 Sigma Prime 维护

- [https://github.com/sigp/lighthouse](https://github.com/sigp/lighthouse)
- [Lighthouse Book](https://lighthouse-book.sigmaprime.io/)

## Just: 是保存和运行项目的特定命令的工具

命令存储在一个名为justfile，语法启发自make:

```
build:
    cc *.c -o main

# test everything
test-all: build
    ./test --all

# run a specific test
test TEST: build
    ./test --test {{TEST}}
```

然后可以使用just <COMMAND>运行它们:

```
$ just test-all
cc *.c -o main
./test --all
Yay, all your tests passed!
```

[https://github.com/casey/just](https://github.com/casey/just)