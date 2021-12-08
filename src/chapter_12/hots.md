# 社区热点

聚焦 Rust 生态热点新闻

---

## Rust for Linux 补丁更新到 V2 版本

2022 年，我们很可能会看到 Linux 内核中的实验性 Rust 编程语言支持成为主流。2021.12.6 早上发出了更新的补丁，介绍了在内核中处理 Rust 的初始支持和基础设施。

这次更新的内容包括：

1. 升级到了最新 Stable 编译器和 Rust 2021 edition 。因此可以摆脱了 `const_fn_transmute`，`const_panic`、`const_unreachable_unchecked`、`core_panic` 和`try_reserve` 这几个之前未稳定的特性。[未稳定特性心愿单]( https://github.com/Rust-for-Linux/linux/issues/2)。
2. 自定义 `core` 和 `alloc`。为 `alloc` 添加了更加模块化的选项，以便禁用一些他们不需要的功能：`no_rc` 和 `no_sync`，主要是为上游 Rust 项目添加。
3.  更严格的代码、文档和新的  `lint`。
4. 抽象和驱动程序更新。添加了序列锁、电源管理回调的抽象，io 内存（`readX`/`writeX`）、irq 芯片和高级流处理程序，gpio 芯片（包括 irq 芯片）、设备、amba 设备和驱动程序以及证书。此外，也改进并简化了 `Ref`（`refcount_t` 支持）对象并用它替换了 Rust 的 `Arc` 的所有实例。完全地从 `alloc` crate 中删除了 `Arc` 和 `Rc`。

从现在开始，Rust for linux 团队将开始定期提交补丁，每两周左右。

除了来自 Arm、Google 和 Microsoft 的支持外，这次该团队又收到一封来自红帽的信：红帽对 Rust 用于内核的工作也非常感兴趣（There is interest in using Rust for kernel work that Red Hat  is considering）。

- [v2 补丁：https://lore.kernel.org/lkml/20211206140313.5653-1-ojeda@kernel.org/](https://lore.kernel.org/lkml/20211206140313.5653-1-ojeda@kernel.org/)
- [https://www.phoronix.com/scan.php?page=news_item&px=Rust-For-Linux-v2](https://www.phoronix.com/scan.php?page=news_item&px=Rust-For-Linux-v2)
- [kernel  crate 文档](https://rust-for-linux.github.io/docs/kernel/)

