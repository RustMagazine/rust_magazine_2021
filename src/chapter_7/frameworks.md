# 推荐项目 |  框架引擎

编辑： 张汉东

---

## Axum: tokio 出品的 Web 框架

这个框架一出，意味着 tokio 生态正在强势建立！ 算的上是Rust Web 开发领域的里程碑事件！让其他 web 框架顿时失色！当然，我并不是否认其他 web 框架的优秀！ 

该框架全面整合 tokio 生态，并且由tokio 团队自己维护。这一点就很占优势了！

查了一下，Axum 这名字 有 king of kings 之意  😂，[https://en.wikipedia.org/wiki/Kingdom_of_Aksum](https://en.wikipedia.org/wiki/Kingdom_of_Aksum)

[https://tokio.rs/blog/2021-07-announcing-axum](https://tokio.rs/blog/2021-07-announcing-axum)

## MeiliSearch 搜索引擎合并了前半年的重构工作

改进摘要：

1. 改进索引系统。
2. 设计了更快的过滤系统版本。
3. 提供了一个新的Facet系统。
4. 提供一个新的 Dashboard。
5. 其他。

- [twitter thread](https://twitter.com/Kerollmops/status/1410613829147324424?s=20)
- [MeiliSearch](https://github.com/meilisearch/MeiliSearch)

## Fluvio：现代化可编程流处理平台

[https://www.infinyon.com/blog/2021/06/introducing-fluvio/#fluvio-programmable-platform-for-data-in-motion](https://www.infinyon.com/blog/2021/06/introducing-fluvio/#fluvio-programmable-platform-for-data-in-motion)

## Quickwit ： 极具成本效益（cost-efficient ）的搜索引擎 诞生了 

从成本角度看，当前有两类搜索模型：

1. 公共搜索引擎。比如 Google/ Twitter/ Wiki /GitHub/ Reddit 之类
2. 私有搜索引擎。 有限的搜索集。比如你的服务日志 / 邮件

如果你运营的是一家成功的电商，你付出昂贵的硬件成本在搜索上面，你的投资回报率可能比较高，所以你也不会在意这点成本。

但是当你还没有达到足够的投资回报率，而你的日志搜索就占了你硬件成本的很大部分，那你就该关心这个成本了。

Quickwit 搜索引擎主要是为了解决这个问题，基于 tantivy ，类似Lucene，的一个分布式搜索引擎。

这是我们的第一个版本，它为一个具有成本效益的搜索引擎奠定了基础。

具体来说，Quickwit 0.1采用了一个命令行界面的形式来创建、索引和提供搜索集群，其主要特点如下。

- 由Tantivy支持的快速索引
- 改良的索引数据结构布局，可以直接在对象存储上打开并读取索引
- 具有无状态实例的分布式搜索
- 基于SWIM协议（Scalable Weakly-consistent Infection-style Process Group Membership Protocol， 可伸缩的弱一致性传染式进程组成员协议，由 gossip 协议发展而来）的集群形成，不用担心 leader 或 consensus
- 可配置的映射
- 自然查询语言

Quickwit 如何节省成本：

1. 使用段复制方式优化索引，降低 cpu 使用时间
2. 分离存储与计算，减少 cpu 空闲时间
3. 使用 热启动 bundle，减少每次查询打开索引的花费
4. 采用并发请求来解决 s3的低吞吐量问题
5. 使用自定义索引格式，减少搜索关键路径

[https://quickwit.io/blog/quickwit-first-release/](https://quickwit.io/blog/quickwit-first-release/)

## 🌱🦀🌱   trillium : 构建 异步 web 应用的 组件库

- trillium 意思是 延龄草 ，或者三叶草。
- trillium 整体架构受 Elixir Plug的启发，并打算成为plug和 tide 的混合体。
- trillium 的特点是 组件化，一切都是 opt-in 的。
- trillium 的核心概念是： Handlers/ Conn/ Adapters 
- Trillium 遵守[12 factor](https://12factor.net/config)的配置方法，尽可能接受环境中的配置。

- [https://trillium.rs/](https://trillium.rs/)
- [https://github.com/trillium-rs/trillium](https://github.com/trillium-rs/trillium)

