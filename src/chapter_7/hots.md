# 社区热点

编辑：张汉东 

---

## RustChinaConf 2021 议题征集开放申请

[申请通道详情](./rust_china_conf_2021.md)

## 为Linux内核添加Rust支持的最新补丁已经出现

Miguel Ojeda一直在领导 "Rust for Linux"的工作--现在这个项目得到了Google的资助以使这种编程语言能够在内核中使用。虽然5.14内核合并窗口目前正在进行，但这并没有被标记为拉动请求阶段，估计要到下一个或者更晚的周期才会登陆。这是继4月份发出的 "征求意见"补丁之后的又一进展。

Rust for Linux的启用现在已经达到了33000多行代码，部分原因是他们目前在树中包括了Rust的 "alloc"标准库的一个子集，为内核目的进行了修改。最终，开发者们将尝试把变化放到上游的alloc crate中，但现在是在内核树中进行，其他需要的库也是在内核树中进行的。

这些新补丁的另一个变化是，之前的版本需要使用Rust编译器的每夜版本，而现在内核可以用beta和稳定版的Rustc编译。然而，内核支持确实需要一些Rust编译器的功能，目前被上游视为不稳定的来源。

除了AArch64、PowerPC和x86_64之外，ARM 32位和RISC-V体系现在也被Rust for Linux所支持。

除了Linux内核中的Rust管道外，Rust支持的最初用户是Rust中的Android Binder IPC实现，这仍被认为是一项正在进行的工作。

这些最新的Rust for Linux内核补丁可以在内核邮件列表中找到：

[https://lore.kernel.org/lkml/20210704202756.29107-1-ojeda@kernel.org/](https://lore.kernel.org/lkml/20210704202756.29107-1-ojeda@kernel.org/)

此消息来自: [https://www.cnbeta.com/articles/tech/1148741.htm](https://www.cnbeta.com/articles/tech/1148741.htm)

## Facebook 从 Java 到 Rust | Buck 的未来

Buck 是 Facebook 在 2013 年的Facebook Mobile DevCon上亮相的一个快速的 Android 构建系统。从那时起，它已经扩展到支持用15种以上的语言编写的应用程序，目标平台包括手机、服务器、智能设备和VR头盔等等。

不过，随着时间的推移，Buck并没有跟上这种功能和灵活性的增长，没有进行必要的架构改变和改进来管理这种复杂性。随着Facebook内部和外部开发新的功能，发现这导致了巨大的实施复杂性，对核心和语言规则进行修改的挑战越来越大。虽然该团队在近4年前就开始了一项举措，逐步对 Buck 进行一些这样的跨领域的架构改进，但这些改变是非常困难的。

所以，从2020年开始，该团队开始构思，如果从零开始构建 Buck 会怎么样？怎样才能写出一个可以在未来 10 年或 20 年中继续扩展的构建系统？

在考虑这次重写的同时，我们利用这个机会实验并验证了Rust作为构建系统的技术栈。Facebook对Rust编程语言的投资越来越大，许多项目都在使用它，并取得了巨大的成功。我们发现这种语言非常合适，原因有很多：

1. Rust的`async/await`语法使得编写异步代码非常顺畅，而且Rust有助于正确处理复杂的并发性细节。Buck的Java计算向并发计算的每一次迁移都是一个长达数月的艰难过渡，而且那里仍然存在着明显的单线程瓶颈。
2. Rust有很多高级语言特性，使开发更容易，更愉快。这些都是像枚举、模式匹配、特质（trait）、过程宏和所有其他的功能，这些功能都受到 Rust开发者的广泛喜欢。
3. Rust对内存分配提供了更大的控制。GC语言（即便是分代收集）在处理像Buck这样的增量计算时也有挑战。
4. Rust是高性能的。我们已经看到了将一些程序用Rust重写后的显著加速。

在接下来的几个月里，你可能会看到Buck的进展较少，但请放心，我们将继续努力为社区提供最好的构建系统。我们认识到，分享过程的一个重要部分是为Buck的用户定义一个平稳的过渡，并确保社区可以和我们一起前进。我们计划在2022年1月前公开这个方案，届时会有更多关于当前Buck用户过渡的细节。

[https://developers.facebook.com/blog/post/2021/07/01/future-of-buck](https://developers.facebook.com/blog/post/2021/07/01/future-of-buck)

## 知乎近期 Rust 相关问题摘录

-  [2021年了，Rust在偏底层的某些领域是替代C++的一个好的选择吗？](https://www.zhihu.com/question/451687128)
- [相比Rust，现代C++有什么难度吗?](https://www.zhihu.com/question/447731745)
- [如何看待 Rust 的应用前景？](https://www.zhihu.com/question/30407715)
- [在2021 年，Rust 将会比 C++ 强在哪里？](https://www.zhihu.com/question/437987252)
- [Rust 的优点是什么？](https://www.zhihu.com/question/463506409)
- [就高频量化交易系统而言，据说rust作为主要面向安全的高性能计算编程语言，比c++要强，这个是真的吗？](https://www.zhihu.com/question/390738348)

## Rust + Copilot 什么效果？

近日 Discord 工程师尝试用 copilot 来辅助开发 Rust 项目。效果不是很好。

视频观看：[https://t.me/rust_daily_news/4914](https://t.me/rust_daily_news/4914)


## Rust GameDev #23 

这一期游戏开发报告中包含了一些很有创意的游戏。写 Rust 累了，可以玩一玩，都是开源的。

1.  吃尾蛇。支持wasm，可以网页玩耍。关卡设计的很有心。基于 bevy 0.5 实现。

- [https://github.com/szunami/taileater/](https://github.com/szunami/taileater/)
- [https://szunami.itch.io/taileater](https://szunami.itch.io/taileater)

2. Egregoria，模拟城市建设者，试图复制现代社会以及尽可能复制。基于  Legion ecs 实现。

[https://github.com/Uriopass/Egregoria](https://github.com/Uriopass/Egregoria)


3. Blightmud ，是一款 命令行终端的 mud 客户端，可以支持很多 mud server，比如 bat.org 等。

[https://github.com/Blightmud/Blightmud](https://github.com/Blightmud/Blightmud)

4.  Dango， 多人物理沙盒游戏。基于 bevy , Nphysics 物理引擎， CrystalObs 网络库等。Dango 目前在浏览器中基于wasm 运行 server，其他玩家通过 webrtc 加入。

[https://github.com/ErnWong/dango-tribute](https://github.com/ErnWong/dango-tribute)

5. hyper-farmer ： 基于bevy实现，游戏虽然简单但是挺有创意，锻炼双手协调

- [https://wilsk.itch.io/hyper-farmer](https://wilsk.itch.io/hyper-farmer) 在线玩
-  [https://github.com/will-hart/cloud-surfer ](https://github.com/will-hart/cloud-surfer )

6. fish-game，基于 macroquad 游戏引擎实现，支持wasm

- [https://fedorgames.itch.io/fish-game](https://fedorgames.itch.io/fish-game) 在线玩
- [https://github.com/heroiclabs/fishgame-macroquad](https://github.com/heroiclabs/fishgame-macroquad) 源码
-  [https://github.com/not-fl3/macroquad](https://github.com/not-fl3/macroquad) 游戏引擎

[https://gamedev.rs/news/023/](https://gamedev.rs/news/023/)

## CNCF 又多了俩 Rust 项目

Good to see both wasmCloud and Krustlet submitted to the CNCF Sandbox at the same time:

- wasmCloud: [https://github.com/cncf/toc/issues/693](https://github.com/cncf/toc/issues/693)
- Krustlet: [https://github.com/cncf/toc/issues/690](https://github.com/cncf/toc/issues/690)

## Zenoh 性能提升的故事｜ 漫游在 Rust 异步仙境

在 Rust Maginze 月刊第四期中介绍过 Zenoh :  [开源产品 | eclipse zenoh 助力雾计算和边缘计算]( [https://rustmagazine.github.io/rust_magazine_2021/chapter_4/zenoh.html](https://rustmagazine.github.io/rust_magazine_2021/chapter_4/zenoh.html) ) 

eclipse zenoh （读： /zeno/ ） ，提供了零开销的Pub/Sub、Store/Query 和 计算。

zenoh 统一了 动态/静止/使用中的数据并提供计算结果。它颇有分寸地将传统的Pub/Sub与地理分布的存储、查询和计算融合在一起，同时保留了远远超出任何主流协议栈的时间和空间效率水平。

官网是 zenoh.io 。

GitHub代码仓库 `eclipse-zenoh/zenoh` 。

2020 年 12 月 Eclipse Edge Native 工作组启动，并将 Zenoh 引入 Eclipse 。并用 Rust 对 zenoh 进行重写。

在本文中，Zenoh 团队剖析了他们如何改进让异步性能提升一倍。

- 8字节payload 时超过3.5M msg/s
- 1Mb payload 时超过 45Gb/s
- 在 backlogged 场景下，延迟低至 35 微秒

该团队如何做到的呢？

一：准备工作

1. 准备测试环境，以便获得可复现的结果。因为很多外部因素可能会影响代码性能，这是为了排除这些干扰。这有个非常棒的指南：https://easyperf.net/blog/2019/08/02/Perf-measurement-environment-on-Linux
2. 彻底阅读 [《Rust 性能手册》](https://nnethercote.github.io/perf-book/title-page.html)。我们发现它对Rust的性能技巧和诀窍以及剖析技术都很有见地。另外，还有一篇关于[如何在Rust中编写高性能代码](http://likebike.com/posts/How_To_Write_Fast_Rust_Code.html)的博客也是不错的参考。

二：寻找性能热点（hotspots）

1.  我们先使用 [flamegraph](https://github.com/flamegraph-rs/flamegraph) 来生成火焰图，打算寻找 zenoh 中的异步性能热点。然而，异步使得火焰图相当难以阅读，因为异步调度器和future执行器基本上出现在火焰图中每一个地方。所以改变了性能剖析工具，开始使用 [perf](https://perf.wiki.kernel.org/index.php/Main_Page) ，可以提供更清晰的热点图，尤其是序列化和反序列化方面。
2. 改进了序列化/反序列化相关实现，性能直接提升 100% 。但是这种改进在吞吐量测试中没有反映出来。

三： 堆分配还是栈分配？

zenoh 团队 一直避免在关键环节进行堆分配。用 valgrind 仔细检查后发现，并没有不必要的堆分配，缓存未命中率也不高。因此该团队开始检查 栈分配的问题，利用 Rust 编译器的一个 flag （仅在 Rust Nightly 可用）来验证一个数据结构多大以及它的内存对齐方式。

rust
$ RUSTFLAGS=-Zprint-type-sizes cargo build --release
用这种方式来编译 zenoh 即可。输出：

```rust
print-type-size type: `net::protocol::proto::msg::Data`: 304 bytes, alignment: 8 bytes
print-type-size     field `.key`: 40 bytes
print-type-size     field `.data_info`: 168 bytes
print-type-size     field `.payload`: 96 bytes
```

异步数据结构也会这样打印出来。然后该团队发现了一个痛苦的事实：

1. 异步 future，一旦被编译，就会在栈中占用几十 KB 的空间。每次有消息需要通过网络发送，就会调用这些 futures。
2. 因为zenoh广泛使用异步，所以现在导致 栈太深太大，给内存带来很大压力。
3. 经过思考，该团队将 异步代码隔离在特定的部分，尤其是网络交互部分，而其他部分则转为使用同步。由此来平衡 同步和异步，汲取了两个世界的优点。大幅减少了栈内存的压力，带了巨大的性能提升。

四： 性能测试结果

该团队性能测试环境为：AMD Ryzen 5800x，32GB内存，通过100Gb以太网连接，根据前面所说的性能测试环境配置指南配置好。

具体的性能测试图表，可以进一步查看文章。也可以关注 zenoh 团队的博客，因为他们性能优化还会继续。

[https://zenoh.io/blog/2021-07-13-zenoh-performance-async/](https://zenoh.io/blog/2021-07-13-zenoh-performance-async/)

## 清华90后校友、MIT助理教授范楚楚获ACM博士论文奖，Rust社区Ralf Jung荣誉提名


Ralf Jung 的博士论文为《Understanding and Evolving the Rust Programming Language (https://people.mpi-sws.org/~jung/phd/thesis-screen.pdf)》

RalfJung 研究的这个，是对  Safe Rust 安全模型做了形式化验证，也为 Unsafe Rust UB 检查建立了模型。实至名归。

[https://mp.weixin.qq.com/s/wkjexOyXXpEC-nYEWxpkWQ](https://mp.weixin.qq.com/s/wkjexOyXXpEC-nYEWxpkWQ)

## Tokio 支持 io-uring

Tokio 为 Linux 上的 io-uring 系统 API 提供支持。此版本提供异步文件操作，将在后续版本中添加对更多操作的支持。

tokio-uring API可能构成Tokio 2.0版本的基础，2024 年 可能会发布 tokio 2.0 ，然后集成这个库。

[https://tokio.rs/blog/2021-07-tokio-uring](https://tokio.rs/blog/2021-07-tokio-uring)

## 同一功能实现下 Rust vs Elixir 代码量比较 案例 : Ockam

Ockam 是一套 用于相互身份验证和分布式应用程序之间的端到端加密消息的工具。 目前提供 Rust 和 Elixir 两种实现。

从 Rust  和 Elixir 的比例来看:

- Rust 69.3%
- Elixir 30.7%

因为是同一种功能特性实现（Rust 比 Elixir 多两个特性实现），但是否可以从 代码量得出开发效率 elixir 是 Rust  的两倍呢？ 

我认为不尽然。也需要看场景。 这个项目设计到分布式消息通信，这方面因为elixir 的默认支持 的生产力可能更高。 而 Rust 可能还需要多做一些基础工作。 

这就好比，很多人说 rust 编译helloworld比 c 的大，但其实 c 的一些基础库操作系统都是默认支持的，不需要另外携带，rust 则需要。

[https://github.com/ockam-network/ockam](https://github.com/ockam-network/ockam)

## CNCF宣布Linkerd毕业

Linkerd 是第一个加入 CNCF 沙箱的项目，沙箱当时被称为 inception，现在是第一个获得毕业地位的服务网格项目。

Linkerd 创始人、Buoyant 首席技术官 Oliver Gould 说。“我们的使命是为服务网格领域带来简单性和用户同理心，我们不得不孜孜不倦地努力消除普遍认为服务网格复杂而笨重的说法。虽然我们做出了有争议的技术决策——采用 Rust 而不是 C++，构建特定于服务网格的'微代理'而不是使用通用代理，专注于 Kubernetes 而不是构建抽象层——这些决策已经得到验证，而我们的全球运营者社区再次对这一愿景下注。”

- [https://linkerd.io/2021/07/28/announcing-cncf-graduation/](https://linkerd.io/2021/07/28/announcing-cncf-graduation/)
- 中文 [https://mp.weixin.qq.com/s/P5dQjVe0jidguNhZ0KzGTg](https://mp.weixin.qq.com/s/P5dQjVe0jidguNhZ0KzGTg)
- [https://linkerd.io/](https://linkerd.io/)

## Rust Search Extension 1.3.0-rc 版发布！

想提前体验的朋友欢迎在这个页面下载并帮忙测测，如果没啥 bug 了周末就正式发布！

- [https://github.com/huhu/rust-search-extension/wiki/V1.3.0-Release-Candidate-(zh_CN) ](https://github.com/huhu/rust-search-extension/wiki/V1.3.0-Release-Candidate-(zh_CN) )
- [https://rust.extension.sh/changelog/](https://rust.extension.sh/changelog/)

## Arti: 一个纯Rust实现的Tor

今天，我很高兴地宣布 Tor 的新时代实现。

在过去一年左右的时间里，我们一直致力于“Arti”，这是一个用 Rust 重写 Tor 的项目。感谢 Zcash Open Major Grants (ZOMG) 的资助，我们终于可以将 Arti 项目列入我们的优先事项列表，并投入更多时间。

下面我将谈谈我们为什么要做这个项目，它对 Tor 用户和运营商意味着什么，它的未来发展方向，以及人们可以如何提供帮助。

- [https://blog.torproject.org/announcing-arti](https://blog.torproject.org/announcing-arti) 
- [https://gitlab.torproject.org/tpo/core/arti/](https://gitlab.torproject.org/tpo/core/arti/)

## Scott Mabin 全职加入 乐鑫科技

Scott Mabin 全职加入 Espressif，将为其所有芯片提供 Rust 支持，改善其芯片的 Rust 生态系统。

乐鑫科技（股票代码：688018）是一家全球化的无晶圆厂半导体公司，成立于2008 年，在中国、捷克、印度、新加坡和巴西均设有办公地，团队来自20 多个国家和地区。

组织

支持esp的分叉编译器已经被移到 esp-rs组织中，同时还有一些idf支持板块--很快就会有更多的内容。在esp-rs/rust资源库中的讨论已经开始，在过去的一个月中，Espressif每周都会举办社区会议，重点是推动Rust支持的发展。

Espressif芯片上的Rust标准库

@ivmarkov 一直在努力将Rust STD库移植到esp-idf上，esp-idf是Espressif芯片的基于C的开发环境，esp-idf有一个newlib环境，可以用来构建Rust库。@ivmarkov把rust-esp32-std-hello演示版本放在一起，以展示其功能，其中包括WiFi、线程和使用esp-idf的驱动来驱动显示器。非常酷的东西! 有一些初期的问题，你可以在这里追踪，但希望我们很快就能开始向上游提供这些补丁。

为了补充Espressif芯片的标准库，esp-idf-hal已经被更新并移到esp-rs组织中。这个 crate 有API来使用标准库中没有抽象的外围设备和硬件，例如SPI外围设备。与HAL一起，esp-idf-svc也被加入，它是Espressif芯片的嵌入式svc特性的实现。embedded-svc背后的目标是提供围绕更高级别的嵌入式服务的抽象，WiFi、Ping、HTTPD、NVS等。

[https://mabez.dev/blog/posts/esp-rust-espressif/](https://mabez.dev/blog/posts/esp-rust-espressif/)

## Lemmy 发布 v0.11.3

Lemmy 类似于Reddit、Hacker News等网站。你订阅你感兴趣的论坛，发布链接和讨论，然后投票，并对其进行评论。在背后它却是非常不同的；任何人都可以很容易地运行一个服务器，所有这些服务器是联合的（类似电子邮件），并连接到同一个宇宙，称为Fediverse。对于一个链接聚合器来说，这意味着在一个服务器上注册的用户可以订阅任何其他服务器上的论坛，并可以与其他地方注册的用户进行讨论。

总体目标是创建一个容易自主托管的、分散的替代Reddit和其他链接聚合器的网站，不受公司控制和干涉。

- [https://github.com/LemmyNet/lemmy](https://github.com/LemmyNet/lemmy)
- [https://lemmy.ml/post/75818](https://lemmy.ml/post/75818)