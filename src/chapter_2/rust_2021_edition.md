---
pub_date: Sat, 27 Feb 2021 16:00:00 GMT
description: Interpreting the Rust 2021 Edition RFC

---

# 解读 Rust 2021 Edition RFC 

作者/编辑：张汉东

---

目前 Rust 2021 Edition 正在讨论中，[RFC 3085](https://github.com/rust-lang/rfcs/pull/3085) 目前已经取代了 [RFC 2052](https://github.com/rust-lang/rfcs/blob/master/text/2052-epochs.md) 成为新的 RFC。

`Edition` 在RFC 2052中提出，Rust在2018年发布了第一个 Edition版本。这项工作在许多方面都是成功的，但也带来了一些困难的教训。 RFC 3085 为 2021 Edition 提出了不同的模型。 需要注意的是，目前该 RFC 还未合并。

### 「2021 Edition 模型」讨论的关键点包括：

- `Edition` 用于将语言引入更改，否则可能会破坏现有代码，例如引入新关键字。
- `Edition` 永远不允许分裂生态系统。 我们只允许不同版本的 crate 进行互操作的更改。
- `Edition` 以其出现的年份命名（例如，Rust 2015，Rust 2018，Rust 2021）。
- 发布新 `Edition` 时，我们还会发布工具以自动执行 crate 的迁移。 可能需要进行一些手动操作，但是这种情况很少见。
- Nightly 工具链提供对即将发布的 `Edition` 的“预览”访问权限，以便我们可以随时进行针对将来 `Edition` 的工作。
- 我们维护一个《`Edition` 迁移指南》，其中提供了有关如何迁移到下一 `Edition` 的指南。
- 只要有可能，都应使新功能适用于所有 `Edition` 。

该RFC旨在确立 `Edition` 的高级用途，并描述RFC对最终用户的感觉。 它有意避免进行详细的策略讨论，这些讨论将由相应的子团队（编译器，lang，开发工具等）来解决。

### 目标与设计原则

顺序代表优先级

1. `Edition`不能分裂生态系统。

最重要的一条规则是：一个`Edition`中的 crate 可以与其他`Edition`中编译的 crate 无缝地互操作。不管`Edition`如何，所有 Rust 代码最终都会在编译器中编译为相同的内部 IR。

2. `Edition` 迁移应该很方便且尽最大可能自动化完成。

在发布新`Edition`的同时也会发布一些工具帮助自动升级`Edition`。并且维护《`Edition`迁移指南》以便手动迁移之需。

3. 由用户来控制何时使用新的`Edition`

4. `Edition` 注定是要被使用的。目标是看到所有Rust用户都采用新`Edition`。

5. Rust 应该感觉像是一种语言，而非被 `Edition` 分割为多种“方言”。

`Edition`向 Rust 引入了向后不兼容的更改，从而又增加了 Rust 开始感觉像具有多种方言的语言的风险。 我们想要避免人们进入 Rust 项目的经历，并对给定的代码含义或可以使用的功能种类感到不确定。 这就是为什么我们更喜欢基于年份的版本（例如Rust 2018，Rust 2021），这些版本将许多更改组合在一起，而不是细粒度的选择加入; 可以简洁地描述基于年份的版本，并确保当您进入代码库时，相对容易地确定可以使用哪些功能。


### 一些背景

Rust 2018版在 RFC 2052中被描述为一个“集结点”，不仅引入了一些迁移，而且还是许多其他更改（例如更新本书，实现连贯的新API集等）的目标。这在很多方面都很有帮助，但在其他方面却是有害的。 例如，在是否有必要升级到新`Edition`以使用其功能方面存在一定的困惑（尚不清楚该困惑是否具有除困惑之外的其他负面影响）。 这也是组织本身将所有内容整合在一起的压力。 它与「火车模型」相反，后者旨在确保我们具有“低压力”发布。

相反，2021版故意是“低调”事件，其重点仅在于介绍已进行了一段时间的一些迁移，惯用法lint和其他工作。 我们没有将其与其他无关的更改进行协调。 这并不是说我们永远不应该再发布“集结点”。 但是，目前，我们在工作中并没有一整套协调一致的变化，我们需要将这些变化汇总在一起。

但是，由于此更改，Rust 2018的一项好处可能会丢失。 有一定比例的潜在Rust用户可能对Rust感兴趣，但兴趣不足以跟进每个`Edition`并跟踪发生了什么变化。 对于这些用户，一篇博客文章列出了Rust 2018以来发生的所有令人振奋的事情，足以说服他们尝试一下Rust。 我们可以通过发布回顾过去几年的回顾来解决这个问题。 但是，我们不必将此回顾与`Edition`联系在一起，因此，此RFC中未对此进行描述。

### 小结

通过以上内容，我想你应该对目前官方的 Rust 2021 Edition 工作内容有所了解。目前该 RFC 还在持续且激烈的讨论中，更多内容可以移步[该 RFC 的 PR](https://github.com/rust-lang/rfcs/pull/3085)中参看。

在官方的 [Edition Guide](https://doc.rust-lang.org/edition-guide/rust-next/index.html) 文档中，已经增加了 [Next Edition](https://doc.rust-lang.org/edition-guide/rust-next/index.html#the-next-edition) 可能发布的功能集合，感兴趣可以自行关注。