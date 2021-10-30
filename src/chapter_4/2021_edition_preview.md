---
pub_date: Thu, 30 Apr 2021 18:00:00 GMT
description: Rust 2021 Edition Plan

---


# 【官宣】Rust 2021 Edition 计划

翻译/编辑：张汉东

原文: [The Plan for the Rust 2021 Edition](https://github.com/m-ou-se/blog.rust-lang.org/blob/1cbd1ee944b1c1f3e369ad70f0d8fc4181f0e4f2/posts/2021-04-30-edition-2021.md)

---

我们很高兴地宣布，Rust语言的第三版次（Edition） Rust 2021 edition 计划于今年10月发布。 Rust 2021 包含许多细微的变化，但仍有望在实践中对 Rust 产生很大的影响。

> 译注：
>
> 这里把 Edition 译为 「版次」，是特意为了在中文中和 「版本」区别开来。
>
> 一般情况下，「版次」代表 Edition，而「版本」特指语义化版本。如果不做这样的区分，都用「版本」就会很乱。
>
> 为什么不用「版」呢，因为「版」对应发行版本，对应 Stable/Nightly/Beta 发行版。

## 什么是「版次（ Edition）」？

Rust 1.0 的发布确立了 [“无停滞的稳定性”](https://blog.rust-lang.org/2014/10/30/Stability.html) 作为 Rust 交付的核心。从 1.0 发行版开始，Rust 的规则是，一旦某个功能在稳定版(Stable)上发布，我们将致力于在所有将来的发行版中都支持该功能。

但是，有时候在 Rust 的语法层面中进行一些小的更改，版次是有用的，否则这些更改将无法向后兼容。最明显的例子是引入一个新的关键字，它会使变量等现有名称无效。即使这样的更改不会“感觉到”向后不兼容，它们仍然有可能破坏现有代码。如果要进行此类更改，人们会很快发现现有程序停止编译。

版次（Edition）是我们用来把这种不可能变成可能的机制。当我们希望发布一个向后不兼容的功能时，我们会将其作为新的Rust 版次的一部分发布。版次是可选的（Opt-in），因此，现有的 Crate 除非将其明确迁移到新版次，否则不会看到这些更改。Cargo 创建的新的 Crate 始终默认使用最新版次。

## 版次不会分裂生态系统

版次的最重要规则是，一个版次中的 Crate 可以与其他版次中编译的 Crate 无缝地互操作。这确保了迁移到较新版次的决定是 Crate 可以做出的“私人”决定，而不影响其他人，除了它影响所需的 rustc 语义版本（version）之外（类似于使用任何新功能）。

Crate 互操作性的要求对我们在一个版次中可以进行的更改种类有一定的限制。通常，一个版次中发生的更改往往是“很薄的一层”。不管版次如何，所有 Rust 代码最终都会在编译器中编译为相同的内部表示形式。

> 译注：版次(Edition)之间的差异，最终会在 MIR 层面消除。

## 版次迁移很容易，而且很大程度上是自动化的

我们的目标是使 Crate 轻松升级到新版次。每当我们发布新版次时，我们也会发布工具来自动进行迁移。工具不一定是完美的：它可能无法涵盖所有​​极端情况，并且仍然可能需要手动更改。该工具尽力避免对语义的更改，这些更改可能影响代码的正确性或性能。

除工具外，我们还维护一个[《版次迁移指南(Edition Migration Guide)》](https://doc.rust-lang.org/edition-guide/)，其中涵盖了版次中的更改。该指南将描述更改，并提供指向人们可以在其中了解更多信息的指南。它还将涵盖人们应注意的任何极端情况或细节。该指南既可以作为该版次的概述，也可以作为人们在使用自动化工具时遇到问题的快速疑难解答参考。最终版次列表将成为 Rust 2021 的一部分。所有这些摘要总结如下。

## Rust 2021 计划进行哪些更改？

在过去的几个月中，Rust 2021工作组已经就新版次中包含的内容提出了许多建议。 我们很高兴宣布最终候选名单。每个功能都必须满足两个条件才能进入此清单。 

首先，它们必须得到相应 Rust 团队的批准。 

第二，它们的实现必须考虑周全，以使我们确信，它们能按计划的里程碑及时完成。

### 增补 Prelude 

[标准库的 Prelude ](https://doc.rust-lang.org/stable/std/prelude/index.html)是一个模块，该模块包含了标准库中其他每个模块必须自动导入的所有内容。它包含了常用的**语言项**（Item)，比如 `Option`、`Vec`、`drop` 和 `Clone`。

Rust编译器会优先处理任何手动导入的项（Item），使其优先于 Prelude 中的项（Item），以确保在 Prelude 中添加的内容不会破坏任何现有代码。例如，如果您有一个名为 `example` 的 Crate 或 模块，其中包含`pub struct Option ;`，则使用`example::*;`。这样就能明确引用 `example` 中的`Option`，而不是标准库中的`Option`。

但是，在 Prelude 中添加 trait 可以以微妙的方式破坏现有代码。比如，`x.try_into()` ，在使用`MyTryInto` trait 中的方法进行调用时，如果还导入了`std`的`TryInto`，则这个调用可能会变得模棱两可，并且无法编译，因为它提供了具有相同名称的方法。这就是我们尚未将`TryInto`添加到 Prelude 的原因，因为有很多代码会破坏这种方式。

作为解决方案，Rust 2021 将使用新的 Prelude。除了以下三个新增功能外，其余与当前的功能相同：

- [std::convert::TryInto](https://doc.rust-lang.org/stable/std/convert/trait.TryInto.html)
- [std::convert::TryFrom](https://doc.rust-lang.org/stable/std/convert/trait.TryFrom.html)
- [std::iter::FromIterator](https://doc.rust-lang.org/stable/std/iter/trait.FromIterator.html)

仍然需要等待库团队（Library team）来批准这三条，但应该很快批准。

### 默认 Cargo Feature 解析器（Resolver）

从Rust 1.51.0开始，Cargo 支持了[可选的新的 Feature 解析器](https://doc.rust-lang.org/cargo/reference/resolver.html#feature-resolver-version-2)，可以通过`Cargo.toml`中的`resolver ="2"` 激活该功能。

从 Rust 2021 开始，这将是默认设置。 也就是说，在`Cargo.toml`中写入`edition ="2021"` 会暗含 `resolver ="2"`。

新的 Feature 解析器不再合并所有请求的功能，这些功能将以多种方式依赖于 Crate。 有关详细信息，请参见[ `Rust 1.51` 的公告](https://blog.rust-lang.org/2021/03/25/Rust-1.51.0.html#cargos-new-feature-resolver)。

### 数组（Array）支持 `IntoIterator`

在`Rust 1.53`之前，只有对数组的引用才实现 `IntoIterator`。 这意味着您可以遍历`＆[1、2、3]`和`＆mut [1、2、3]`，但不能直接遍历`[1、2、3]`。

```rust
for &e in &[1, 2, 3] {} // Ok :)

for e in [1, 2, 3] {} // Error :(
```

这是一个长期存在的问题，但是解决方案并不像看起来那样简单。仅添加`trait`实现会破坏现有代码。 `array.into_iter()`现在已可编译，由于方法调用语法的工作原理，该函数隐式调用`(＆array).into_iter()`。添加`trait`实现将改变含义。

通常，我们将这种类型的破坏(breakage)（添加`trait`实现）分类为“轻微(minor)”和“可接受(acceptable)”。但是在这种情况下，有太多的代码会被它破坏。

多次建议“仅在 Rust 2021 中为数组实现IntoIterator”。但是，这根本不可能。您不能在一个版次中存在`trait`实现，而在另一个版次中则不能存在，因为版次可以混合使用。

因此，我们决定在所有版次中添加`trait`实现（从Rust 1.53.0开始），但添加一个小技巧以避免在Rust 2021之前损坏。在 Rust 2015 和 2018 代码中，编译器仍将解析`array.into_iter()`为`(&array).into_iter()`，就好像`trait`实现不存在一样。这仅适用于`.into_iter()`方法调用语法。它不会影响任何其他语法，例如`[1、2、3]`中的`e`或`iter.zip([1、2、3])`。这些将开始在所有版次中使用。

遗憾的是，这需要上述小技巧以避免破损，但我们对这种如何将两个版次之间的差异保持在最低限度的解决方案感到非常满意。

### 闭包中不相关的捕获

[闭包(Closure) ](https://doc.rust-lang.org/book/ch13-01-closures.html)会自动从上下文捕获其引用的任何内容。 例如，`|| a + 1`会自动从周围的上下文中捕获对`a`的引用。

当前，即使仅使用一个字段，也将影响整个结构。 例如，`|| a.x +1`捕获对`a`的引用，而不仅仅是`a.x`。 在某些情况下，这是一个问题。 当结构的某个字段已被借用（可变）或移出时，其他字段将无法再用于闭包中，因为这将捕获整个结构，而该结构不再可用。

```rust
let a = SomeStruct::new();

drop(a.x); // Move out of one field of the struct

println!("{}", a.y); // Ok: Still use another field of the struct

let c = || println!("{}", a.y); // Error: Tries to capture all of `a`
c();
```

从 Rust 2021 开始，闭包将仅捕获其使用的字段。 因此，以上示例在 Rust 2021 中可以很好地进行编译。

此新行为仅在新版次中才被激活，**因为它可以更改字段的 drop 顺序**。 对于所有版次更改，都可以进行自动迁移。 `Cargo fix --edition`将能够更新与此相关的闭包。 也可以通过在闭包插入 `let _ =＆a; ` 来强制闭包像以前一样捕获整个结构。

### Panic 宏的一致性

`panic!()`宏是 Rust 中最常见的宏之一。 但是，它有一些[微妙的惊喜](https://github.com/rust-lang/rfcs/blob/master/text/3007-panic-plan.md)，我们不能仅仅因为向后兼容而进行更改。

```rust
panic!("{}", 1); // Ok, panics with the message "1"
panic!("{}"); // Ok, panics with the message "{}"
```

`panic!()`宏仅在使用多个参数调用时才使用字符串格式。当使用单个参数调用时，它甚至不会查看该参数。

```rust
let a = "{";
println!(a); // Error: First argument must be a format string literal
panic!(a); // Ok: The panic macro doesn't care
```

（它甚至接受诸如`panic!(123)`之类的非字符串，这是罕见的，很少有用。）

当[隐式格式参数](https://rust-lang.github.io/rfcs/2795-format-args-implicit-identifiers.html)将被稳定时，这尤其是一个问题。 该功能将使`println!("hello {name}")` 成为 `println!(" hello {}"，name)`的简写形式。 但是，`panic!("hello {name}")`不能按预期工作，因为`panic!()`不会将单个参数作为格式字符串处理。

为了避免这种混乱的情况，Rust 2021 提供了更一致的`panic!()`宏。 新的`panic!()`宏将不再接受任意表达式作为唯一参数。 就像`println!()`一样，它将始终将第一个参数作为格式字符串处理。

另外，Rust 2021 中的`core::panic!()`和`std::panic!()`相同。当前，这两者之间存在一些历史差异，当打开或关闭`＃！[no_std]`时，这是很明显的。 

### 保留语法

为了将来为某些新语法腾出空间，我们决定为前缀的标识符和文字保留语法：`prefix#identifier`，`prefix" string"`，`prefix'c'`和`prefix#123`，其中`prefix`可以是任何标识符。 （除了已经具有含义的含义，例如`b''`和`r“”`。）

这是一个重大变化，因为宏当前可以接受`hello"world"`，它们将被视为两个单独的标记：`hello`和`"world"`。 （自动）修复很简单。 只需插入一个空格：`hello "world"`。

除了将它们转换为标记化错误外，RFC 尚未将含义附加到任何前缀。 为特定的前缀分配含义留给将来的建议，由于现在保留了这些前缀，因此不会破坏更改。

这些是您将来可能会看到的一些新前缀：

`f""`是格式字符串的简写形式。 例如，`f"hello {name}"`是等效的`format_args!()`调用的简写形式。

`c""`或`z""`用于以`N`结尾的`C`字符串。

`k#keyword`允许编写当前版次中尚不存在的关键字。 例如，虽然`async`在 `2015 edition` 中不是关键字，但使用此前缀可以使我们在`2015 edition`中接受`k#async`，而不必等待`2018 edition`将`async`保留为关键字。

### 代码质量检查（Lint）

使用 Rust 2021，许多现有的 Lint 正成为 Crate 中的硬错误，在旧版次中，这些 Lint 将仍然是警告。

- `bare_trait_objects`：在Rust 2021中，必须使用`dyn`关键字来标识“ trait 对象”。
- `ellipsis_inclusive_range_patterns`：Rust 2021中包含范围模式的`...`语法将是一个硬错误； 新语法为`..=`，与表达式一致。

我们可能会在此列表中添加更多Lint。

### `macro_rules` 中的 或（Or） 模式

从 Rust 1.53.0 开始，[模式(pattern)](https://doc.rust-lang.org/stable/reference/patterns.html)被扩展以支持`|`用于嵌套在模式中的任何位置。例如，现在可以写`Some(1 | 2)`代替`Some(1) | Some(2)`。由于以前根本不允许这样做，所以这不是一个重大变化。

但是，此更改也会影响[`macro_rules`宏](https://doc.rust-lang.org/stable/reference/macros-by-example.html)。这样的宏可以使用`:pat`片段说明符接受模式。当前，`:pat`不匹配`|`，因为在 Rust 1.53 之前，并非所有模式（在所有嵌套级别）都可以包含`|`。接受像`A | B`这样的模式的宏，例如[`match!()`](https://doc.rust-lang.org/1.51.0/std/macro.matches.html)使用类似`$($_:pat)|+`的东西。因为我们不想破坏任何现有的宏，所以我们没有将 Rust 1.53.0 中的`:pat`的含义更改为包括`|`。

相反，我们将在 Rust 2021 中进行该更改。在新版本中，`:pat`片段说明符将匹配`A | B`。

由于有时仍然希望匹配不带`|`的单个模式变量，因此添加了指定的片段`:pat_param`以保留较旧的行为。该名称旨在表示使用这种模式的主要用于闭合参数。

就是说，到目前为止，我们的工作如期进行，许多困难的部分已经解决，这要归功于所有为 Rust 2021 做出贡献的人们。

## 接下来是什么？

2021 版次的计划里程碑如下：

- ✅ 今天：功能集已最终确定。
- 🚧 5月17日：在 Nightly 中完成实现，包括迁移（正在进行中）
- ⌛ 6月15日：《版次迁移指南》和其他文档完成
- ⌛ 7月1日：呼吁进行公开测试
- ⌛ 9月1日：先在 Nightly 中稳定 2021 版次
- ⌛ 10月21日：随着Rust 1.56.0的发布，稳定 2021 版次。

在撰写本文时，我们正在按时完成这些截止日期，并且不会预见任何问题。 但是，Rust是一个由志愿者运行的项目。 我们优先考虑在 Rust 上工作的每个人的个人福祉，而不是我们设定的任何截止日期和期望。 这可能意味着如果需要的话，会延迟版次的发布，或者放弃一项事实证明过于困难或压力太大而无法及时完成的功能。

如果您想继续，可以在[ Rust 2021 Edition 电子表格](https://docs.google.com/spreadsheets/d/1chZ2SL9T444nvU9al1kQ7TJMwC3IVQQV2xIv1HWGQ_k/edit#gid=1034375760)或[项目板](https://github.com/orgs/rust-lang/projects/7)上跟踪状态。

感谢阅读！