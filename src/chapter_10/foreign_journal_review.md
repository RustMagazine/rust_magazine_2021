# 外刊评论

编辑：张汉东 

> 编者按：国外的 Rust 资源非常丰富，将其都翻译过来也不现实，所以想到这样一个栏目。外刊评论，主要是收集优秀的 Rust 相关文章，取其精华，浓缩为一篇简单评论。
>
> 欢迎大家贡献：[https://github.com/RustMagazine/rust_magazine_2021/discussions/129](https://github.com/RustMagazine/rust_magazine_2021/discussions/129)

---

## 目录

- [各大开源项目进展概述](#各大开源项目进展概述)
- [嵌入式异步的现在与未来](#嵌入式异步的现在与未来)

---

## 各大开源项目进展概述

### GCC Rust

取得了里程碑式进展。通过阅读 rustc 的测试套件代码， 找出了测试用例的不少问题，正在取得稳定进展。

### Black Hat Rust  book 更新

- 100% 的[随书代码](https://github.com/skerkour/black-hat-rust)已上传 GitHub
- [内容](https://academy.kerkour.com/black-hat-rust?coupon=BETA) 完成度 90%，进入 beta 阶段
- 所有的插图都经过修改

为什么要写这本书？从安全角度来学习 Rust。

### SixtyFPS

[SixtyFPS ](https://github.com/sixtyfpsui/sixtyfps)是一个工具包，可以为任何显示器高效开发流畅的图形用户界面：嵌入式设备和桌面应用程序。

- SixtyFPS 0.1.4 发布
- cargo UI 0.3   发布

## Fluvio

[Fluvio ](https://www.fluvio.io/) 是一个开源数据流平台，可聚合、关联并将可编程智能应用于动态数据。Fluvio 由 Rust 提供支持，在云原生架构上提供低延迟、高性能的可编程流。

- 我们为 MQTT 协议提供了一个新的连接器
- 即将推出 table 功能，将启用具有结构化` JSON/YAML/TOML` 数据的物化视图 。

### Databend

[Databend](https://github.com/datafuselabs/databend) 旨在成为一个开源的**弹性**和**可靠的**云仓库，它提供极快的查询和联合弹性，简单，云的低成本，内置使数据云容易。

新增了一些特性：

- 新增 `system.metrics`  表
- 为查询新增了简单的 REST API
- 命令行新特性： 支持 admin 和 sql 模式；可通过 http 查询。

另外还做了一些改进和 修复了一些 Bug。

### Rust Analyzer

新增了一些新功能：

- 为关联方法自动增加类型前缀
- 方便为多个方法创建模块

修复了一些 Bug。

### Intellij Rust

新功能：为属性宏提供初始支持。可以展开属性宏了。

修复了一些 Bug。

### Gloo

[Gloo ](https://github.com/rustwasm/gloo) 0.4.0 版本发布。Gloo 是使用 Rust 和 Wasm 构建快速、可靠的 Web 应用程序和库的工具包。

### gfx-rs

gfx-rs 是一个为 Rust 带来高效跨平台图形的项目，

- wgpu 建立在 wgpu-hal 和 naga 之上，为图形应用程序提供安全性、可访问性和便携性。
- naga 在语言之间翻译着色器程序，包括 WGSL，提供着色器验证和转换，确保在 GPU 上运行的用户代码安全高效。

现发布 wgpu v0.11 和 naga v0.7。

[https://gfx-rs.github.io/2021/10/07/release-0.11.html](https://gfx-rs.github.io/2021/10/07/release-0.11.html)



---

## 嵌入式异步的现在与未来

该文作者写过三篇嵌入式下Rust异步的系列文章：

- [Async and asleep: designing our future embedded applications](https://tweedegolf.nl/blog/58/async-and-asleep-designing-our-future-embedded-applications)
- [Measuring power consumption: sync vs. async](https://tweedegolf.nl/blog/62/measuring-power-consumption-sync-vs-async)
- [Async on Embedded: Present & Future](https://tweedegolf.nl/blog/63/async-on-embedded-present-and-future)

 在前面的两篇中，作者介绍了 嵌入式下使用 Rust 异步的好处：更好的任务管理和节省能耗。

但是实际的测试结果发现，同步和异步的能耗其实没差多少。使用 embassy 嵌入式异步运行时库可以工作的更好。

但是目前异步还有限制，比如 trait 不能包含 async 方法。幸运的是，Rust 的 泛型关联类型 （GAT）可能在今年稳定，届时可以解决这个限制。

Rust Nightly 已经实现了 trait 中支持异步方法的特性。作者试用了这个Nightly特性。

```rust
pub trait I2c<A: AddressMode = SevenBitAddress> {
    /// Error type
    type Error;
    // 这里用到了生命周期参数，需要 Nightly 下 GAT 的支持
    type ReadFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    fn read<'a>(&'a mut self, addr: A, bs: &'a mut [u8]) -> Self::ReadFuture<'a>;
}
```

trait 里虽然不让出现异步函数，但是可以在函数实现中使用 异步块来解决：

```rust
fn read<'a>(&'a mut self, addr: u8, bs: &'a mut [u8]) -> Self::ReadFuture<'a> {
    async move {
        // implementation
    }
}
```



总之，对于嵌入式需求来说，稳定目前存在于 nightly 上的GAT 就足以为编写 async驱动 crate 提供基础，有效地推动了生态系统的发展。有了这些，嵌入式上的异步将成为一种非常有效的技术。

在准备过程中，现在可能是一个很好的时机，可以开始在夜间玩玩异步特性。一旦GAT 稳定下来，我们就可以一起建立 Rust Embedded 异步生态系统🦀❤️🦀。

