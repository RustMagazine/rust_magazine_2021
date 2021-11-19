# 【我读】Rust  插件开发

> 原文： [Plugin Development Kit in Rust 系列](https://nullderef.com/series/)

本文是我阅读 Rust插件开发系列文章的学习记录，也包含了关于事件处理系统的相关内容。

---

## 文前

这系列文章来自于 Google代码之夏中 Tremor 项目的 issues： [Plugin Development Kit ( PDK )](https://github.com/tremor-rs/tremor-runtime/issues/791)

> 简而言之，Tremor 是一个事件处理系统。 它最初是为了替代 Logstash 或 Telegraf 等软件而设计的。 然而，通过支持更复杂的工作流（例如聚合、汇总、ETL 语言和查询语言），tremor 已经超出了这个单一用例的范围。

为 Tremor 插件开发一个通用接口，使上述库变得更加模块化，并减少核心依赖集。

这将大大减少 Tremor 的核心大小，这意味着该库的编译速度将更快，二进制大小也更小。最重要的是，它将把Tremor的架构转变为完全模块化，其中插件可以根据需要进行配置并以语言无关的方式独立开发。

Tremor 每年 365 天 24x7 运行，并使用 Rust 编程语言实现。

> 深挖了一下 tremor-runtime 项目背后的公司，原来是 Wayfair 。Wayfair 是美国最大的家具电商，2017 年市值就达58亿美元，前身是早在2002年就成立的CNSStores。亚马逊都吃不下它。
>
> Tremor 应该是 Wayfair 公司旗下的开源项目，已经进入 CNCF 。今年九月份还召开了一次小型的线上的 [Tremor Conf](https://community.cncf.io/events/details/cncf-tremor-community-presents-tremor-con-2021)
>
> 去年（2020）3月份的一次分享：Rust 如何为 Wayfair 省掉数千个核心和TB级的内存的成本 ：[2020-03-31-RustAndTellBerlin-functions](https://www.tremor.rs/slides/2020-03-31-RustAndTellBerlin-functions.pdf)
>
> 从2018年开始， tremor 就是跑在了 wayfair生产环境中，每天处理10兆字节的数据，或每分钟100亿条消息，每秒1000万个指标。tremor 降低了成本，减少了复杂性，巩固和简化了操作环境，以激发SRE的乐趣，减少NOC的工作量，并降低运营成本。

## 什么是事件处理系统

可以参考我这篇文章：[Rust 生态观察 | 事件处理系统]() 





## 参考

- [Tremor Docs](https://www.tremor.rs/docs/index)
- [Event processing systems](https://ifs.host.cs.st-andrews.ac.uk/Books/SE9/Web/Architecture/AppArch/EventProc.html)
- [大数据原则与范式: 实时分析](https://www.sciencedirect.com/science/article/pii/B9780128053942000027)
- [天文地球观测大数据中的知识发现: 天文学中的实时流处理](https://www.sciencedirect.com/science/article/pii/B9780128191545000199)
- [https://nullderef.com/series/rust-plugins/](https://nullderef.com/series/rust-plugins/)
- [https://github.com/tremor-rs/tremor-runtime](https://github.com/tremor-rs/tremor-runtime)

