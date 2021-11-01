# 推荐项目 ｜ 基础工具库

聚焦 Rust 生态库

---

## SwapQueue： 一个高度优化和轻量级的`crossbeam::deque`替代品

一个无锁线程拥有的队列，其中任务由线程安全窃取者通过缓冲区交换而不是任务弹出来完成。

只为通过缓冲区交换来获取整个队列这一单一的使用情况而设计。通过放弃弹出单个任务的能力，可以做出一个更适合于完全批处理的数据结构。

[https://github.com/Bajix/swap-queue-rs](https://github.com/Bajix/swap-queue-rs)

## Goose：Rust 实现的负载测试框架

Goose是受Locust启发的Rust负载测试工具。用户行为是用标准的 Rust 代码定义的。负载测试是依赖于Goose 库的应用程序。Web 请求是使用Reqwest HTTP 客户端发出的。

Goose 每 CPU 核产生的流量至少是 Locust 的 11 倍，对于更复杂的负载测试（例如使用第三方库抓取表单内容的负载测试），收益甚至更大。

- [https://github.com/tag1consulting/goose](https://github.com/tag1consulting/goose)
- [Goose Book](https://book.goose.rs/)

## Flowistry：强大的 Rust IDE 工具

`Flowistry` 是一个 `VSCode` 扩展，可帮助您理解 Rust 程序。`Flowistry` 使用数据流分析和指针分析在比类型所能提供的更深层次上分析 Rust 程序（参见 `rust-analyzer`）。

Flowistry 是` alpha `软件（请参阅限制），正在寻找早期采用者来尝试并提供反馈！ 

[https://github.com/willcrichton/flowistry](https://github.com/willcrichton/flowistry)

## doku: 0.10 发布

Doku是一个框架，可以直接从代码中构建美观的、人类可读的文档;它允许您轻松地为配置文件、HTTP端点等生成文档。

告别陈旧的手写文档——使用Doku，代码就是文档!。

[https://github.com/anixe/doku](https://github.com/anixe/doku)

## Mimic: 一款用Rust语言编写的命令行Gameboy模拟器

Mimic是一款用Rust编写的开源Gameboy模拟器，可以使用命令行界面作为屏幕和输入设备。该项目试图为Gameboy制作一个易于使用和学习的模拟器，该模拟器可用于解释模拟一个系统所需的概念，而不会让读者感到不知所措。模拟器的核心逻辑完全使用safe rust 编写，没有JIT编译器，屏幕/IO逻辑与模拟器核心功能保持分离以降低复杂性。因此，它的性能并不理想，但Gameboy是一个旧系统，因此理想的性能对于全速运行游戏来说不是必要的。

- [Mimic: A Gameboy emulator written in Rust that can be played on the command line](https://www.reddit.com/r/rust/comments/pzq52u/mimic_a_gameboy_emulator_written_in_rust_that_can/)
- [https://github.com/jawline/Mimic](https://github.com/jawline/Mimic)

## Boa发布v0.13

Boa是一个用Rust语言编写的实验性Javascript词法分析器、解析器和编译器。它可以相当容易地嵌入到Rust项目中，也可以从命令行使用。Boa的存在也是为了作为EcmaScript规范的Rust实现，我们可以在某些领域利用Rust及其奇妙的生态系统来制造一个快速、并行和安全的引擎。

Boa由Jason Williams在2019年JSConf欧盟大会上首次介绍，目前官方已经实现了该语言的一部分特性。在这个版本中，Boa与ECMAScript标准的一致性已经增长到官方ECMAScript测试套件（Test262）的41.97%；已经解决了40个问题，并且合并了105个请求。此版本带来了一些新特性，例如支持从JavaScript调用Rust闭包，以提高JS和Rust之间的更好互操作性。

[https://github.com/boa-dev/boa](https://github.com/boa-dev/boa)

## Hurl  命令行工具，简单**纯文本格式**定义HTTP请求

**Hurl**是一个 HTTP 客户端，它执行以简单纯文本格式定义的 HTTP 请求。

Hurl 非常好用，它可以发送 HTTP 请求，从 HTTP 响应中提取数据。

```
$ hurl session.hurl
```

如果未指定输入文件，则从标准输入读取输入。

```
$ echo GET http://httpbin.org/get | hurl
    {
      "args": {},
      "headers": {
        "Accept": "*/*",
        "Accept-Encoding": "gzip",
        "Content-Length": "0",
        "Host": "httpbin.org",
        "User-Agent": "hurl/0.99.10",
        "X-Amzn-Trace-Id": "Root=1-5eedf4c7-520814d64e2f9249ea44e0"
      },
      "origin": "1.2.3.4",
      "url": "http://httpbin.org/get"
    }
```

默认输出到标准输出。要输出到文件，请使用 -o 选项：

```
$ hurl -o output input.hurl
```

默认情况下，Hurl 执行所有 HTTP 请求并输出最后一次 HTTP 调用的响应体。

[https://hurl.dev/docs/man-page.html](https://hurl.dev/docs/man-page.html)

## Bronze: Rust 垃圾回收器

Rust 没有使用垃圾回收器，而是借助精密、复杂的类型系统，这样做使得 Rust 很高效，但相对难以学习和使用。本文作者们为 Rust 设计了一个可选的基于库的垃圾回收器。为了验证效果，文章对来自 633 人班级的志愿者进行了一项随机对照试验，总共收集了 428 名学生的数据。结果发现，对于需要管理复杂别名的任务，使用垃圾回收器的用户更有可能在可用时间内完成任务，而完成任务的用户只需要大约三分之一的时间（4 小时与 12 小时）。

将来希望扩展 Bronze 跟踪器以跟踪可能传递地包含对 GC 对象引用的任意对象。还希望调查使用 GC 的影响，不仅是针对复杂的别名场景，而且是为了减轻总体所有权的影响；也许这样做可以使学习曲线变平，并且帮助用户更积极地使用 Rust。

很有意思的想法，详细可阅读下方论文。

作者主页：[Michael Coblenz](http://www.cs.umd.edu/~mcoblenz/)

Paper：[Does the Bronze Garbage Collector Make Rust Easier to Use? A Controlled Experiment](https://arxiv.org/pdf/2110.01098.pdf)

GitHub：[mcoblenz/Bronze](https://github.com/mcoblenz/Bronze/)

## abi_stable: 动态加载

Rust-to-Rust ffi，标准类型的 ffi-safe 等价物，并创s建在启动时加载的库。对于 Rust-to-Rust ffi，重点是创建在程序启动时加载的库，以及加载时类型检查。这个库允许定义可以在运行时加载的 Rust 库，即使它们是使用与依赖它的 crate 不同的 Rust 版本构建的。

相关文章：[Plugins in Rust: Diving into Dynamic Loading | NullDeref](https://nullderef.com/blog/plugin-dynload/)

GitHub：[rodrimati1992/abi_stable_crates: Rust-to-Rust ffi,ffi-safe equivalents of std types,and creating libraries loaded at startup.](https://github.com/rodrimati1992/abi_stable_crates/)

## rust-ci-release-template: Rust 二进制 CI 发布模板

每次开发时自动执行：

- 带缓存的 CI：检查、测试、代码风格

推送新标签时自动执行：

- 构建多平台二进制文件
- 创建 GitHub 版本
- 更新 Homebrew

[SpectralOps/rust-ci-release-template: A Github Actions based CI release template for Rust binaries](https://github.com/SpectralOps/rust-ci-release-template)

## crabz: 跨平台、快速压缩和解压缩工具

支持以下格式：

- Gzip
- Zlib
- Mgzip
- BGZF
- Raw Deflate
- Snap

[sstadick/crabz: Like pigz, but rust](https://github.com/sstadick/crabz)

## Dart/Flutter <–> Rust binding 开源了

想要结合Flutter 和 Rust 之间的优点吗?这里来了!

特性:

- 内存安全
- 类型支持
- 零拷贝
- 异步编程
- 易于使用
- 轻量
- 易于代码评审
- Prue-Dart 兼容

[https://github.com/fzyzcjy/flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge)

## redact: 构建去中心化的、端到端加密的网站的工具

值得一提的是，这个项目并没有用到区块链，但是其号称实现了零信任 zero-trust，相关领域的同学可以关注一下。

它怎样工作的：[https://redact.ws/how-it-works](https://redact.ws/how-it-works)

仓库：[https://github.com/pauwels-labs/redact-client](https://github.com/pauwels-labs/redact-client)

## lnx: 一个基于 tantivy 的搜索引擎

它是 MeiliSearch 和 ElasticSearch 的竞品。其基于 tokio-rs，hyper 和 tantivy 进行开发。提供 REST 接口。现已发布 v0.6 版。持续关注。

[https://github.com/lnx-search/lnx](https://github.com/lnx-search/lnx)

## rs-merkle：一个用Rust编写的高级散列树库

rs-merkle是一个高级的Rust merkle树库。基本功能包括构建Merkle树、创建和验证单个和多个元素的Merkle证明，即多重证明。高级功能包括对树进行事务性更改，并回滚到以前提交的任何树状态，类似于Git。 该库有两个主要结构。第一个是MerkleTree，它构建了一棵树，可用于验证数据完整性并生成Merkle证明。第二种是MerkleProof，可用于验证集合中是否包含项目。 这个库是高度可定制的。哈希算法和树的构建方式可以通过`Hasher` trait进行配置。

关于Merkle树

Merkle树，也称为散列树，用于验证两个或多个参与方是否拥有相同的数据，而无需交换整个数据集合。 Merkle树被用于Git、Mercurial、ZFS、IPFS、比特币、以太坊、Cassandra等许多领域。例如，在Git中，Merkle树用于查找本地和远程存储库状态之间的增量，以便通过网络仅传输它们之间的差异。在比特币中，Merkle树用于验证交易是否包含在区块中，而无需下载整个区块内容。ZFS使用Merkle树快速验证数据完整性，提供保护，防止幻象写入、磁盘固件中的错误、电源浪涌和其他原因导致的静默数据损坏。

[https://github.com/antouhou/rs-merkle](https://github.com/antouhou/rs-merkle)

## Rust 的动态类型 `dyn_struct`

这个库可以安全地初始化动态大小类型 (DST)。

```Rust
#[repr(C)]
#[derive(DynStruct)]
struct MyDynamicType {
    pub awesome: bool,
    pub number: u32,
    pub dynamic: [u32],
}

// the `new` function is generated by the `DynStruct` macro.
let foo: Box<MyDynamicType> = MyDynamicType::new(true, 123, [4, 5, 6, 7]);
assert_eq!(foo.awesome, true);
assert_eq!(foo.number, 123);
assert_eq!(&foo.dynamic, &[4, 5, 6, 7]);
```

[https://github.com/nolanderc/dyn_struct](https://github.com/nolanderc/dyn_struct)

## elasticsearch-dsl-rs - 用 Rust 写就的 Elasticsearch DSL

elasticsearch-dsl-rs 是 Elasticsearch DSL 的 Rust 实现，特性如下：

- 强类型查询
- 强类型聚合
- 自动跳过空查询
- 不依赖 elasticsearch-rs，可以作为独立库来方便 HTTP 客户端调用 ElasticSearch

[https://github.com/vinted/elasticsearch-dsl-rs](https://github.com/vinted/elasticsearch-dsl-rs)

## ROAPI：静态资源服务器

ROAPI是一个 API 服务器，用户无需编写任何代码即可公开 CSV、JSON 和 Parquet 文件。ROAPI 由 4K 行 Rust 组成。

[https://tech.marksblogg.com/roapi-rust-data-api.html](https://tech.marksblogg.com/roapi-rust-data-api.html)

## Infinitree  嵌入式数据库

具有 3 层缓存的可扩展，加密嵌入式数据库

[https://github.com/symmetree-labs/infinitree](https://github.com/symmetree-labs/infinitree)

## Rust Web Local Storage API

这里有一个 Rust 实现的 Web LocalStorage API，用于非浏览器环境。

[https://github.com/richardanaya/web-local-storage-api](https://github.com/richardanaya/web-local-storage-api)

## autograph：Rust的机器学习库

这是在SPIR-V compute shaders上重建的autograph的第一个版本，可以使用rust-gpu从Rust源代码编译！

要在 crate 中使用autograph，请将其添加为 Cargo.toml 中的依赖项：

```
[dependencies]
autograph = { git = https://github.com/charles-r-earp/autograph }
```

- [https://github.com/charles-r-earp/autograph/tree/v0.1.0](https://github.com/charles-r-earp/autograph/tree/v0.1.0)
- [https://www.reddit.com/r/rust/comments/qiwtet/autograph_v010/](https://www.reddit.com/r/rust/comments/qiwtet/autograph_v010/)

## Cooptex -无死锁 Mutexes

这应该是可用的crate版本，旨在提供不会死锁的互斥锁。这个crate使用wait-die scheme方案来实现这一点。

如果Mutex:：lock调用可能会死锁，它将返回一个Err(Retry)，请求调用方删除所有持有的锁并再次尝试获取它们。这由retry_循环函数处理。

[https://crates.io/crates/cooptex](https://crates.io/crates/cooptex)

[https://www.reddit.com/r/rust/comments/qis8zy/cooptex_deadlockfree_mutexes/](https://www.reddit.com/r/rust/comments/qis8zy/cooptex_deadlockfree_mutexes/)

## Rusterizer：用Rust编写的简单3D渲染器

该项目实现了一个基本的OpenGL渲染pipeline。没有使用依赖项，从零开始就完全使用Rust。你可以在这里在线试用

[https://github.com/dzharvis/rusterizer#readme](https://github.com/dzharvis/rusterizer#readme)

[https://www.reddit.com/r/rust/comments/qixyuw/simple_3d_renderer_written_in_rust/](https://www.reddit.com/r/rust/comments/qixyuw/simple_3d_renderer_written_in_rust/)

## 【系列】用 Rust 实现文本编辑器

分为 7 个部分：从设置 到[**读取按键和进入“原始模式”**](https://medium.com/@otukof/build-your-text-editor-with-rust-part-2-74e03daef237)、[**在屏幕上绘图和移动光标**](https://medium.com/@otukof/build-your-text-editor-with-rust-part-3-b030670fa815)、[**显示文本文件（使我们的程序成为文本视图）**](https://medium.com/@otukof/build-your-text-editor-with-rust-part-4-fd4a8b8641f8)、[**编辑文本文件和保存更改**](https://medium.com/@otukof/build-your-text-editor-with-rust-part-5-e363c16f542b)，[**实现一个很酷的搜索功能**](https://medium.com/@otukof/build-your-text-editor-with-rust-part-6-3cff61dc2de5)，最后[**添加语法突出显示**](https://medium.com/@otukof/build-your-text-editor-with-rust-final-part-4c841a649900)**。**

[https://medium.com/@otukof/build-your-text-editor-with-rust-678a463f968b](https://medium.com/@otukof/build-your-text-editor-with-rust-678a463f968b)

## Rust 开源气象站

weather-station，运行和监控自己的开源气象站所需的一切。

气象站将测量值发送到 API 服务器进行收集，然后 API 服务器将这些数据提供给用户加载到其计算机或移动设备上的 Web 应用程序 (PWA)。

[https://github.com/codi-hacks/weather-station](https://github.com/codi-hacks/weather-station)，
