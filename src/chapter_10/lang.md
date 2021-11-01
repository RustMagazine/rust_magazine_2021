# 官方动态

聚焦 Rust 官方活动与新闻

---

## Rust 1.56.0 版本和 Rust 2021 版次发布，迈向 Rust 广泛应用的征程

###  Rust 1.56.0 和 Rust 2021 的升级或安装

如果你已通过 rustup 安装了 Rust 的早期版本，那么更新到 Rust 1.56.0 相当容易：

```bash
$ rustup update stable
```

如果您还未安装过 Rust，可以从 Rust 官网页面[获取 `rustup`](https://www.rust-lang.org/install.html)。

### 新特性一览

Rust 1.56.0 版本和 Rust 2021 版次的升级改进并不算大，新特性大抵如下：

- [闭包捕获的改进](https://doc.rust-lang.org/edition-guide/rust-2021/disjoint-capture-in-closures.html)：直接参考如下示例。

```rust
// 2015 or 2018 edition code
let a = SomeStruct::new();

// Move out of one field of the struct
drop(a.x);

// Ok: Still use another field of the struct
println!("{}", a.y);

// Error: Before 2021 edition, tries to capture all of `a`
let c = || println!("{}", a.y);
c();
```

- [数组迭代器 `IntoIterator`](https://doc.rust-lang.org/edition-guide/rust-2021/IntoIterator-for-arrays.html): `array.into_iter()` 现在是按项值遍历，替代了原来的按引用遍历。
- [宏中的 Or 模式](https://doc.rust-lang.org/edition-guide/rust-2021/or-patterns-macro-rules.html) 即 `:pat` 中的 `A|B`。
- [Cargo 新的默认解析特性 resolver](https://doc.rust-lang.org/edition-guide/rust-2021/default-cargo-resolver.html) 现在默认值为 2。即 Rust 1.51.0 版本后，显式设定在 `Cargo.toml` 中的 `resolver = "2"`，可以删除了。
- [prelude 的补录](https://doc.rust-lang.org/edition-guide/rust-2021/prelude.html)：默认增补 `TryInto`、`TryFrom`，以及 `FromIterator`。
- [Panic 宏](https://doc.rust-lang.org/edition-guide/rust-2021/panic-macro-consistency.html) 期望的输出为字符串格式，就像 `println!()`。
- [预留语法](https://doc.rust-lang.org/edition-guide/rust-2021/reserving-syntax.html)：`ident#`、`ident"..."`，以及 `ident'...'`.
- [warnings -> errors](https://doc.rust-lang.org/edition-guide/rust-2021/warnings-promoted-to-error.html)：主要涉及 `bare_trait_objects` 和 `ellipsis_inclusive_range_patterns`。

###  将已有项目迁移到 Rust 2021 版次

1. 运行 `cargo fix --edition`
2. 编辑 `Cargo.toml`，设定 `edition = "2021"`。如：

```toml
[package]
name = "..."
version = "0.0.1"
...
edition = "2021"
...
```

[https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html](https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html)



## Rust 基金会 新成员 聚焦 |  为什么要加入 Rust 基金会 

Rust 基金会官网成员聚焦栏目，会定期采访一些新加入基金会的成员，这里是对这些采访内容的纪要。

详情见： [Rust 基金会 新成员 聚焦 |  为什么要加入 Rust 基金会](./rust-foundation.md)

## 一个实用的 rustdoc 改进刚刚在 nightly 中合并

`#[cfg()] `信息现在将出现在文档中（默认情况下不再需要使用 `#[doc(cfg())]`！）

[https://github.com/rust-lang/rust/pull/89596](https://github.com/rust-lang/rust/pull/89596)

**P.S** Rustdoc team leader Guillaume Gomez 将在 RustChinaConf 2021 大会上有精彩分享

## Rust Lang 团队 10 月更新

本周 lang 团队召开了 10 月的计划会议，涉及以下内容：

- [Async](https://rust-lang.github.io/async-fundamentals-initiative/updates/2021-oct.html) ，计划稳定 trait 中的[异步函数 MVP ](https://rust-lang.github.io/async-fundamentals-initiative/roadmap/mvp.html)版本。
- [Impl trait](https://rust-lang.github.io/impl-trait-initiative/updates/2021-oct.html) ,   `type Foo = impl Trait` 功能取得一些进展
- [Dyn upcasting](https://rust-lang.github.io/dyn-upcasting-coercion-initiative/updates/2021-oct.html) ， 取得了不错的进展，需要解决一个[健全性的问题](https://github.com/rust-lang/lang-team/issues/119)然后合并。
- [泛型关联类型](https://rust-lang.github.io/generic-associated-types-initiative/updates/2021-oct.html)，目前也有一些进展。
- 其他

- [https://blog.rust-lang.org/inside-rust/2021/10/08/Lang-team-Oct-update.html](https://blog.rust-lang.org/inside-rust/2021/10/08/Lang-team-Oct-update.html)



## Rust 生态的可持续增长和可见性

Rust 生态系统的长期健康非常重要，确保它不仅需要我们随着时间的推移而增长，还需要确保我们能够适当地扩展。实现这一目标意味着确保我们生态系统的关键部分“人员充足”并且能够自我维护，并且不会有任何维护人员精疲力竭。

至关重要的是，人*不是计算机*。对一个人来说，那种令人无法接受的承诺会很快耗尽他们的精力，而其他人则可能会因此茁壮成长。不同的代码库对“处于健康状态”的含义有不同的需求。但是，拥有更多信息、明确数字来引导我们提出更有针对性的问题，这将有助于我们*提前*发现精疲力竭的人，帮助我们更早地发现问题（以及进展顺利的事情！）。

所以官方创建了工具 [optopodi](https://github.com/optopodi/optopodi) 。 optopodi 是一个实验项目，用于开始收集有关 github 组织和存储库的指标。

目标是了解各种“开源健康”风格的指标，例如：

- 哪些存储库最活跃？
- 谁在为他们做出贡献？
- 获得 review 需要多长时间？

最终，我们希望使用这些数据来帮助项目和贡献者取得更大的成功。

官方的长期目标：

> 是让这个工具成为一个全面的自动化服务，为公共仪表板提供服务。我希望能够一目了然地检查生态系统的状况，并且让每个人都能看到我正在看的相同数据。我希望能够知道贡献者何时面临倦怠的风险并加以预防。我想确保贡献者的 "服务质量 "很高，如果它没有得到解决，并能够验证我们的改变是否达到了预期效果。我希望我们能够清楚地看到我们的行动所产生的效果。我希望我们不要怀疑我们是否在做正确的事情，我希望我们能有具体的证据。

> 有了这些信息，不仅可以让项目指导我们的工作，还可以让我们尽早做到这一点，因为我们可以在维护者感受到这些问题之前发现潜在的问题。增加参与关键库的知识贡献者的数量有助于这些项目的每个贡献者。公关和问题将有更快的周转时间。而将这些信息公之于众，也可以让其他人独立评估我们的健康状况。

[https://estebank.github.io/sustainable-growth-and-visibility.html](https://estebank.github.io/sustainable-growth-and-visibility.html)
