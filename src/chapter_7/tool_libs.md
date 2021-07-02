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