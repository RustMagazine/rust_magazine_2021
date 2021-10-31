# 社区热点

聚焦 Rust 生态热点新闻

---

## Linkerd 2.11 现在包含一个用 Rust 编写的 Kubernetes 控制器





## 【Rust 生态观察】 Rust 实现的事件处理引擎 tremor-runtime 已经在 美国最大家具电商公司 Wayfair 生产环境跑了三年

深挖了一下 tremor-runtime 项目背后的公司，原来是 Wayfair 。

Wayfair 是美国最大的家具电商，2017 年市值就达58亿美元，前身是早在2002年就成立的CNSStores。亚马逊都吃不下它。

Tremor 应该是 Wayfair 公司旗下的开源项目，已经进入 CNCF 。今年九月份还召开了一次小型的线上的 [Tremor Conf](https://community.cncf.io/events/details/cncf-tremor-community-presents-tremor-con-2021)

去年（2020）3月份的一次分享：Rust 如何为 Wayfair 省掉数千个核心和TB级的内存的成本 ：[2020-03-31-RustAndTellBerlin-functions](https://www.tremor.rs/slides/2020-03-31-RustAndTellBerlin-functions.pdf)

从2018年开始， tremor 就是跑在了 wayfair生产环境中，每天处理10兆字节的数据，或每分钟100亿条消息，每秒1000万个指标。tremor 降低了成本，减少了复杂性，巩固和简化了操作环境，以激发SRE的乐趣，减少NOC的工作量，并降低运营成本。

最近有一个 Rust 插件开发系列文章，也是出自 tremor 项目的 GSoC 挑战：[rust-plugins](https://nullderef.com/series/rust-plugins/) ，已经发布了四篇。

[tremor-runtime](https://github.com/tremor-rs/tremor-runtime)

## 使用Rust进行内核开发

在2021年的Linux Plumbers大会上，Linux的Rust开发者们都在那里进行了许多富有成效的讨论。在维护者峰会上，Miguel Ojeda从Plumbers中走出来，在一个不同的场合谈论Rust。要怎样才能让Rust补丁被合并？他得到的答案是令人鼓舞的，即使不是完全承诺的。

[https://lwn.net/Articles/870555/](https://lwn.net/Articles/870555/)

### 【官方】安卓团队正式介绍 Android Rust

Android平台提供了对用Rust开发本地操作系统组件的支持。

- Android Rust 模块 ： [https://source.android.com/setup/build/rust/building-rust-modules/android-rust-modules](https://source.android.com/setup/build/rust/building-rust-modules/android-rust-modules)
- hello Rust example: [https://source.android.com/setup/build/rust/building-rust-modules/hello-rust-example](https://source.android.com/setup/build/rust/building-rust-modules/hello-rust-example)
- Android Rust 模式: [https://source.android.com/setup/build/rust/building-rust-modules/android-rust-patterns](https://source.android.com/setup/build/rust/building-rust-modules/android-rust-patterns)

还有其他模块介绍，详细请看：[https://source.android.com/setup/build/rust/building-rust-modules/overview](https://source.android.com/setup/build/rust/building-rust-modules/overview)

## OpenSUSE 2021 Rust Survey的结果

从9月8日到10月7日，OpenSUSE帮助我主持了一个关于开发人员如何在他们的环境中使用Rust的调查。作为SUSE和OpenSUSE中Rust包的维护者，对我来说，更好地了解人们如何使用Rust是很重要的，这样我们才能做出符合社区工作方式的决定。

所有的数据都可以在这里找到: [https://fy.blackhats.net.au/blog/html/2021/10/08/results_from_the_opensuse_2021_rust_survey.html](https://fy.blackhats.net.au/blog/html/2021/10/08/results_from_the_opensuse_2021_rust_survey.html)

## Pest 项目找维护人

pest 是著名的 Rust 解析器框架，现在作者好像停止维护了。需要有人接手。有意者请参与讨论：

[https://github.com/pest-parser/pest/discussions/547](https://github.com/pest-parser/pest/discussions/547)

