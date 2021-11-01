# 外刊评论 ｜ 知名项目进展报告

编辑：张汉东 

> 编者按：国外的 Rust 资源非常丰富，将其都翻译过来也不现实，所以想到这样一个栏目。外刊评论，主要是收集优秀的 Rust 相关文章，取其精华，浓缩为一篇简单评论。
>
> 欢迎大家贡献：[https://github.com/RustMagazine/rust_magazine_2021/discussions/129](https://github.com/RustMagazine/rust_magazine_2021/discussions/129)

---

## 各大开源项目进展报告

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



