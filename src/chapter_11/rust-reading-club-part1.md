# Rust 源码阅读俱乐部 |  第一期

说明： 这不是线上沙龙的文字记录，而是我本人会后的学习记录。

## 引子

最近 Rust 官方发起了 Rust 源码阅读俱乐部 活动，参见 [Rust Code Reading Club](https://mojosd.medium.com/rust-code-reading-club-8fe356287049) 。

此活动目的就是为了让更多人为  Rust 编译器做贡献。由 语言工作组 Leader Niko 亲自来讲解 Rust 编译器中的源码。

这次活动不会有任何回放，只能实时参与。

参与人员条件：

1. 听得懂英文
2. 对 Rust 有一定了解，不必要了解 Rust 的所有角落
3. 不一定需要编译器背景知识，但是需要阅读 [Rustc Dev guide ](https://rustc-dev-guide.rust-lang.org/)  (中文版也需要你的贡献： [https://github.com/RustcRustc/rustc-dev-guide-zh](https://github.com/RustcRustc/rustc-dev-guide-zh))

第一期活动：

- 时间： 2021-11-04 12:00 - 13:30 US Eastern time ([see in your time zone](https://everytimezone.com/s/a287d2e5))
- 方式 zoom
- 内容： [compiler/rustc_resolve/src](https://github.com/rust-lang/rust/tree/master/compiler/rustc_resolve/src) 
- Slides： [https://hackmd.io/@rustc-reading-club/S1xsDveDK#/1](https://hackmd.io/@rustc-reading-club/S1xsDveDK#/1)

参与前准备：

1. 阅读 《the-programmers-brain》 这本书
2. 阅读 Rustc dev guide ，了解下 Rust 编译过程
3. 阅读 [compiler/rustc_resolve/src](https://github.com/rust-lang/rust/tree/master/compiler/rustc_resolve/src)  文档相关，了解该库



###  Rust 飞书群 Rust 源码阅读俱乐部

为了响应官方的活动，我在飞书 Rust 中文社群也准备每周六或日，也组织一次源码阅读在线沙龙。

内容就是跟随官方内容，共同学习。

并且会把每周源码学习记录下来，行为文字输出，分享给大家。

[如何加入飞书的Rust 中文社群?](https://github.com/ZhangHanDong/rust-code-reading-club/issues/1)



###  准备工作

#### 书籍推荐《 The Programmer's Brain》

Rust 官方推荐了《The Programmer's Brain》 这本书，该书好像没有引入中文版。在[ Manning 的官网](https://www.manning.com/books/the-programmers-brain)可以免费看这本书的在线版本。如果要翻译书名的话，我觉得《编程大脑》这个名字还行？ 听说这本书已经被国内出版社引进。

这本书分为四部分：

1. 如何更好地阅读代码
2. 关于代码的思考
3. 编写更好的代码
4. 关于代码协作

这本书的特点在于，它会介绍不同类型代码和大脑认知思维之间的联系，而不是纯技巧罗列。

既然 Rust 官方推荐这本书，那说明它的内容还是有一定价值，感兴趣可以去看看。

##### 关于编程过程中的三类困扰

这一期分享这本书第一章中提到的编程过程中的三类困扰开发者的问题：

1. 缺乏知识（Lack of knowledge，对应长期记忆  long-term memory ， LTM）。指开发者缺乏基本的编程语言的知识，无法使用或理解基本的语法，而造成的困扰。
2. 缺乏信息（Lack of information，对应短期记忆， short-term memory， STM）。指开发者缺乏程序要处理的问题领域信息的了解，而造成的困扰。
3. 缺乏处理能力（Lack of processing power ，对应工作记忆，working memory，WM）。指开发者缺乏对整个编程执行过程的处理能力，而造成的困扰。

这三类问题，不仅仅困扰开发者编写新的程序，还对开发者阅读既有代码造成困扰。

所以，当我们在阅读别人写的源码时，要确保对这三类问题中缺乏的知识有一个预备补充。

#### 我的源码阅读习惯

我阅读源码和读书的方式一致，都是从整体结构到细节。

首先要确保自己对 `rustc_resolve` 这个库的上下文信息有所了解，也就是上面提到的 编程过程中的三类困扰中的第二类问题要做信息补充。第一类和第三类问题，相信对于 非 Rust 新手应该是可以避开了。一般阅读 Rust 源码最常见的问题就是第二类问题，缺乏对程序要处理问题领域的信息的了解。

#### 官方建议的阅读方法

在  [官方给出的 Rustc 源码阅读第一期 Slides](https://hackmd.io/@rustc-reading-club/S1xsDveDK#/1) 中，建议采用一种 广-深-广( `Broad - deep - broad`) 的三段阅读方法。

具体来说：

1. 广（`Broad`）： 整体了解模块。
2. 深（`Deep`）： 聚焦于某个函数或小片区域（你感兴趣的，或有疑问的）。
3. 广（`Broad`）： 回到整个模块中。

按上述三段阅读方法来执行几轮，整个代码就读完了。



##  Rustc 编译器架构

在 [Rustc Dev Guide](https://rustc-dev-guide.rust-lang.org/overview.html) 中介绍了 Rust 编译器（Rustc） 的 整体架构。

Rustc 编译器架构不同于传统的编译器架构。传统编译器架构是 **基于 遍历** (`pass-based`) 而设计，Rust 编译器架构则是 **基于 按需驱动**（`demand-driven`）而设计。

### 基于遍历 的编译器架构

 所谓 遍历（`Pass`） ，就是对 代码 / `AST ` 扫描并进行处理。

早期的编译器一般都是 `Single Pass` 的，后来又出现 `Multi Pass` ，并且分为编译前端和后端。前端负责生成  `AST` ，而后端用于生成机器码。编译流程的每一步都被抽象为 `Pass`，这个称呼最早由 `LLVM` 采用，进而扩展到整个编译原理领域。

遍历 分为两类：

- 分析（analysis）遍历，负责收集信息供其他 Pass 使用，辅助调试或使程序可视化
- 变换 （transform）遍历，用于改变程序的数据流或控制流，比如优化等

这两类遍历流程，也对应着编译器的两大阶段：分析阶段和综合阶段。前者从给定的源码文本创建出一个中间表示，后者从中间表示创建等效的目标程序。

编译器前端一般对应于 分析阶段，编译器后端对应于综合阶段。

编译器前端又包括以下几部分：

1. 词法分析器
2. 语法分析器
3. 语义分析器
4. 中间代码生成器
5. 代码优化器

而目标代码生成则由后端完成。

在 词法分析、语法分析和语义分析阶段，编译器会创建和维护一个重要的数据结构，用于跟踪变量的语义，即它会存储有关的信息和名称的绑定信息等，叫做 符号表（`Symbol Table`）。在中间代码生成和目标代码生成过程中会使用它。

传统的基于遍历的编译器架构大概就是这样。

 ###  按需驱动的编译器架构

Rust 编译器执行过程：

- `rustc`  命令执行编译
- `rustc_driver` 来解析命令行参数，相关编译配置被记录于 `rustc_interface::Config`
- `rustc_lexer` 用于词法解析，将源代码文本输出为 词条流 (`Token Stream `)
- `rustc_parse` 为编译过程下一阶段做准备。包含了词法分析的一部分，通过 内置的  [` StringBuffer`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_parse/lexer/struct.StringReader.html) 结构体对文本字符串进行验证，以及将字符串进行符号（Symbol）化。 符号化是一种叫做 [`String interning`](https://en.wikipedia.org/wiki/String_interning) 的技术，将字符串的值存储一份不可变的副本。
- `rustc_parse` 另一部分就是语法解析，使用递归下降（自顶向下）方法进行语法分析，将 词条流转换为 抽象语法树（`AST`）。入口点是 [`rustc_parse::parser::Parser`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_parse/parser/struct.Parser.html) 结构体的 [`Parser::parse_crate_mod()`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_parse/parser/struct.Parser.html#method.parse_crate_mod)和[`Parser::parse_mod()`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_parse/parser/struct.Parser.html#method.parse_mod) 关联方法。外部模块解析入口点是[`rustc_expand::module::parse_external_mod`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_expand/module/fn.parse_external_mod.html)。宏解析器入口点是[`Parser::parse_nonterminal()`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_parse/parser/struct.Parser.html#method.parse_nonterminal)。
- 宏展开、`AST`验证、名称解析，以及 early lint 发生在编译过程的词法分析和语法分析阶段。
- 此后，将 [`AST `转为` HIR`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_hir/index.html)，  使用` HIR` 进行 类型推断](https://rustc-dev-guide.rust-lang.org/type-inference.html)（自动检测表达式类型的过程）、[特质求解](https://rustc-dev-guide.rust-lang.org/traits/resolution.html)（将` impl `与对特质的每个引用配对的过程）和[类型检查](https://rustc-dev-guide.rust-lang.org/type-checking.html)（转换类型的过程）。
- 随后，[将](https://rustc-dev-guide.rust-lang.org/mir/index.html)`HIR`[降级到中级中级代表 (`MIR`)](https://rustc-dev-guide.rust-lang.org/mir/index.html)。在此过程中，也构建了 `THIR`，这是一个更加脱糖的` HIR`。`THIR (Typed HIR)` 用于模式和穷举检查。转换成` MIR` 也比` HIR` 更方便。
- `MIR` 用于[借用检查](https://rustc-dev-guide.rust-lang.org/borrow_check.html)，它基本上是一个控制流图 (`CFG`)。此外 ， `MIR` 还用于 优化、增量编译、Unsafe Rust UB 检查等。
- 最后，进行 代码生成 （`Codegen`）。 将 `MIR` 转换为 `LLVM IR`，然后将` LLVM IR `传递给` LLVM` 生成目标机器代码。 

另一件需要注意的事情是编译器中的许多值都是`intern` 的。这是一种性能和内存优化，我们在称为`Arena`的特殊分配器中分配值。

在 Rust 编译器中，上面说的过程主要步骤都被组织成一堆相互调用的查询。

Rust 编译器使用的是 查询系统（`Query System`），而非大多数编译原理教科书那种遍历式编译器（基于遍历 的编译器架构 ）。 Rust 使用查询系统是为了实现 增量编译功能，即按需编译。

Rust 编译器最初并不是基于查询系统实现的，所以现在整个编译器还在改造为查询系统过程中，上面的整个编译过程都将被改造为基于查询系统。但是截至到 2021年 11月，目前仅是在` HIR` 到` LLVM IR` 这个过程是基于查询的。

### 编译器源码结构

Rust 语言项目本身由三个主要目录组成：

- `compiler/`，包含源代码`rustc`。它由许多 `crate` 组成，这些 `crate` 共同构成了编译器。
- `library/`，包含标准库 ( `core`, `alloc`, `std`, `proc_macro`, `test`) 以及 Rust 运行时 ( `backtrace`, `rtstartup`, `lang_start`)。
- `src/` ，包含 `rustdoc`、`clippy`、`cargo`、构建系统、语言文档等的源代码。

该`compiler/`包装箱所有名称以`rustc_*`。这些是大约 50 个相互依存的`crate`的集合，大小不等。还有`rustc` `crate` 是实际的二进制文件（即 `main`函数）；除了调用`rustc_driver` `crate`之外，它实际上并没有做任何事情。

Rust 编译器之所以区分这么多 `crate` ，主要是以下两个因素考虑：

1. 便于组织代码。编译器是一个巨大的代码库，拆分为多个 `crate`，更利于组织。
2. 加速编译时间。多个 `crate` 有利于增量和并行编译。

但是因为 查询系统是在 `rustc_middle` 中定义的，而其他很多 `crate`  都依赖于它，而它又很大，导致编译时间很长。但是将其拆分的工作又没那么简单。

整个编译器 依赖树的顶部是[`rustc_interface`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_interface/index.html)和 [`rustc_driver`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_driver/index.html)板条箱。[`rustc_interface`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_interface/index.html)是围绕查询系统的未稳定包装器，有助于驱动编译的各个阶段。

### 查询： 按需驱动编译

什么叫查询？ 比如有一个查询叫 `type_of(def_id)`，只要给定某个`Item`  的 `def-id` （标识符定义的索引值 `rustc_middle/src/hir/def_id.rs` ），就可以得到该` Item `的类型。查询执行是被缓存的，这也是增量编译的机制。

```rust
let ty = tcx.type_of(some_def_id);
```

但是，如果查询 不在缓存中，则编译器将尝试找到合适的**提供程序(provider)**。提供程序是一个已定义并链接到编译器某处的函数，该函数包含用于计算查询结果的代码。

由 Rust 编译器的查询系统还衍生出一个通用的按需增量计算框架 [Salsa](https://github.com/salsa-rs/salsa)。你可以通过 [`Salsa BOOK`](https://salsa-rs.github.io/salsa/about_salsa.html) 进一步了解查询系统工作机制。

## 源码阅读：名称解析组件 `rustc_resolve` 

第一期 源码阅读 的内容聚焦在  `rustc_resolve` 库，它和 名称解析 相关。

经过前面关于 Rust 编译器架构背景相关了解，我们知道， `rustc_resolve` 名称解析是发生在 语法分析阶段，为生成最终 抽象语法树而服务，所以，这个库并没有使用到 查询系统。

这也是源码阅读第一期指定这个库的原因吧，不会上来就涉及相对比较复杂的查询系统。

`crate` 的模块在这里构建，宏的路径、模块导入、表达式、类型、模式、标签（`label`）和生命周期 都是在这里解析的

类型相关的名称解析（方法、字段、关联项）发生在`rustc_typeck` 上。

### Rust 中的名称解析

经过查阅名称解析相关的资料，了解到 Rust 编译器在 2016 年引入 [RFC 1560](https://github.com/rust-lang/rfcs/blob/master/text/1560-name-resolution.md) 来改进名称解析的处理过程。

在这之前，名称解析在编译器的早期被处理，在 AST 降级到 HIR 之后。AST 会被遍历三遍，第一遍用于构建 `简化图（reduce_graph）`，第二遍用于解析名称，第三遍来检查未使用的名称。简化图是程序中所有定义和导入的记录。

RFC 1560 将名称解析分成两个阶段：第一个阶段是与宏展开同时发生，并且会解析导入，来定义一个作用域范围内名称到定义的映射。第二阶段是从整个映射中根据一个名称来查找定义。这样做的目的是解耦。

当前 RFC 1560 已经被实现，在宏扩展期间不会做 全名解析，只解析导入和宏。当整个 AST 被构建以后，才会做全名解析，以便解析整个 crate 中所有的名称。

来看一个示例：

```rust

#![allow(unused)]
fn main() {
    type x = u32;
    let x: x = 1;
    let y: x = 2;
}

```

上面代码是可以合法编译的。其中 `x` 即是类型的命名，也是一个变量的命名。 Rust 如何进行名称解析来让两个同名的标识符共存呢？

因为 Rust 有不同的命名空间。不同类型的符号存在于不同的命名空间中，比如类型和变量不会发生冲突。每个命名空间都会有自己的独立的 `rib` (编译器内部引入的抽象作用域概念，比如 let绑定、花括号定义范围、宏定义范围等都是一个 rib )栈。

接下来，我们现在来采用官方建议的三段阅读方法来阅读这个库的源码。

### rustc_resolve  的整体模块结构

包括在阅读 `rustc_resolve` 这个库的时候，我先从其文档着手。一个`crate` 的文档可以非常清晰的展现出这个 `crate` 的整体结构。

[https://doc.rust-lang.org/stable/nightly-rustc/rustc_resolve/index.html](https://doc.rust-lang.org/stable/nightly-rustc/rustc_resolve/index.html)

#### 模块

- [`build_reduced_graph`](https://doc.rust-lang.org/stable/nightly-rustc/rustc_resolve/build_reduced_graph/index.html)  从宏中获取 AST 片段后，此模块中的代码有助于将该片段集成到已经部分构建的模块结构中。
- [`check_unused`](https://doc.rust-lang.org/stable/nightly-rustc/rustc_resolve/check_unused/index.html)，顾名思义，检测 unused 结构体、枚举和函数
- [`def_collector`](https://doc.rust-lang.org/stable/nightly-rustc/rustc_resolve/def_collector/index.html)， 给 AST 的节点创建 DefId（定义标识ID）
- [`diagnostics`](https://doc.rust-lang.org/stable/nightly-rustc/rustc_resolve/diagnostics/index.html)，失败时候的诊断信息
- [`imports`](https://doc.rust-lang.org/stable/nightly-rustc/rustc_resolve/imports/index.html)，一揽子和解析 导入 相关的方法和结构
- [`late`](https://doc.rust-lang.org/stable/nightly-rustc/rustc_resolve/late/index.html)，“后期求解（late resolution）”  是除 导入 和 宏之前大多数名称求解的过程。它在 crate 完全展开并且模块结构完全构建时运行。所以，它只是遍历crate 并解析所有表达式、类型等。为什么没有对应的 `early`，因为它被分散到  `build_reduced_graph.rs`，`macros.rs`和`imports.rs` 中。
- [`macros`](https://doc.rust-lang.org/stable/nightly-rustc/rustc_resolve/macros/index.html)， 一揽子和 解析 宏 相关的方法和结构

#### 结构体

错误类型

- `AmbiguityError`，歧义错误
- `BindingError`， 绑定错误
- `PrivacyError`，可见性错误
- `UseError`， use 错误

数据类型

- `DeriveData`， Derive 相关数据
- `ExpandHasher` ，展开 Hasher
- `ModuleData`，模块树某个节点的数据
- `ExternPreludeEnty` ，处理 Extern、Prelude 相关
- `NameBinding`， 记录可能是私有的值、类型或模块定义
- `UsePlacementFinder`，use 相关

命名空间和作用域

- `PerNS`，每个命名空间的单独结构，辅助类型
- `ParentScope`， 记录scope 访问者的起点
- `Segment`，path 段最小呈现

解析器相关

- `Resolver` 主要的解析器类型
- `ResolverArenas`，为 crate其他部分提供内存，Arena 模型

#### 枚举

这里就不罗列了，和结构体分类类似的一些枚举类型。

#### Traits

- [`ToNameBinding`](https://doc.rust-lang.org/stable/nightly-rustc/rustc_resolve/trait.ToNameBinding.html)，用来转换areans 引用为 NameBinding 引用

#### 函数

 一些辅助函数

#### 类型别名

记录了一些类型别名

#### 依赖crate

在 `rustc_resolve` 的  `Cargo.toml` 中可以看到一些依赖 `crate`:

-  `rustc_ast `， 该库中定义了 Rust 内部用的 AST 数据结构
- `rustc_arean`，编译器内部全局内存池，使用它来分配内存，被分配的内存生命周期为 `'tcx`
- `rustc_middle`，Rust 编译器的 main 库，包含了其他 库中使用的通用类型定义
- `rustc_attr`，和编译器内建属性相关
- `rustc_data_structures`，定义了很多 编译器内部使用的数据结构，包括一些并行编译需要的线程安全的数据结构
- `rustc_errors`，定义了 编译器常用的报告错误的实用工具
- `rustc_expand`，用于宏展开。
- `rustc_feature`，定义了编译器中的 features gate 
- `rustc_hir`，定义了 HIR 相关数据类型
- `rustc_index`， 对 `usize`的一个 NewType 包装，用于编译器内部索引
- `rustc_metadata`，关于 Rust 静态库、动态库相关的一些链接元信息
- `rustc_query_system`，Rust 查询系统
- `rustc_session`，编译器编译过程中错误处理和内建lint相关
- `rustc_span`，定义源代码位置相关的数据类型，也包括宏卫生相关信息。

以上只是列出一些主要的依赖。截止今天（2021.11.13），看到 名称解析库也加入了 查询系统。

接下来我们看一下 [`lib.rs`](https://github.com/rust-lang/rust/blob/master/compiler/rustc_resolve/src/lib.rs) 中定义了什么内容。

看得出来，在 `lib.rs` 中定义的基本都是 上面文档中展示的那些 用于在名称解析过程中使用的结构体或枚举类型。

这里罗列有几个比较容易懂的类型：

**`Scope` 枚举类型:**

```rust
// 用于查找名称的特定作用域，只能用于 early 解析过程，比如 导入 和 宏，而不能用于 late 解析。
/// A specific scope in which a name can be looked up.
/// This enum is currently used only for early resolution (imports and macros),
/// but not for late resolution yet.
#[derive(Clone, Copy)]
enum Scope<'a> {
    DeriveHelpers(LocalExpnId),
    DeriveHelpersCompat,
    MacroRules(MacroRulesScopeRef<'a>),
    CrateRoot,
    // The node ID is for reporting the `PROC_MACRO_DERIVE_RESOLUTION_FALLBACK`
    // lint if it should be reported.
    Module(Module<'a>, Option<NodeId>),
    RegisteredAttrs,
    MacroUsePrelude,
    BuiltinAttrs,
    ExternPrelude,
    ToolPrelude,
    StdLibPrelude,
    BuiltinTypes,
}
```

**`Segment` 结构体：**

```rust
// path 的最小化呈现 ： 段
// 比如  std::sync::Arc  这就是一个 path，其中 `::` 分开的就是段
/// A minimal representation of a path segment. We use this in resolve because we synthesize 'path
/// segments' which don't have the rest of an AST or HIR `PathSegment`.
#[derive(Clone, Copy, Debug)]
pub struct Segment {
    ident: Ident,
    id: Option<NodeId>,
    /// Signals whether this `PathSegment` has generic arguments. Used to avoid providing
    /// nonsensical suggestions.
    has_generic_args: bool,
}
```

`**LexicalScopeBinding` 枚举：**

```rust
// Item，整个块中可见
// Res，只在定义的地方可见
/// An intermediate resolution result.
///
/// This refers to the thing referred by a name. The difference between `Res` and `Item` is that
/// items are visible in their whole block, while `Res`es only from the place they are defined
/// forward.
#[derive(Debug)]
enum LexicalScopeBinding<'a> {
    Item(&'a NameBinding<'a>),
    Res(Res),
}
```

**`ModuleKind` 枚举**

```rust
#[derive(Debug)]
enum ModuleKind {
    // 比较有意思的是，我们发现内部模块的分类，还有一种是 匿名模块，一个 block 就是一个匿名模块
    /// An anonymous module; e.g., just a block.
    ///
    /// ```
    /// fn main() {
    ///     fn f() {} // (1)
    ///     { // This is an anonymous module
    ///         f(); // This resolves to (2) as we are inside the block.
    ///         fn f() {} // (2)
    ///     }
    ///     f(); // Resolves to (1)
    /// }
    /// ```
    Block(NodeId),
    /// Any module with a name.
    ///
    /// This could be:
    ///
    /// * A normal module – either `mod from_file;` or `mod from_block { }` –
    ///   or the crate root (which is conceptually a top-level module).
    ///   Note that the crate root's [name][Self::name] will be [`kw::Empty`].
    /// * A trait or an enum (it implicitly contains associated types, methods and variant
    ///   constructors).
    Def(DefKind, DefId, Symbol),
}
```

`AmbiguityKind` 枚举

```rust
// 歧义类型
#[derive(Clone, Copy, PartialEq, Debug)]
enum AmbiguityKind {
    Import,  //  多个导入源
    BuiltinAttr, // 内建属性命名冲突
    DeriveHelper, //  derive 内命名冲突
    MacroRulesVsModularized, //   宏名和非宏名冲突
    GlobVsOuter, 
    GlobVsGlob,
    GlobVsExpanded,
    MoreExpandedVsOuter,
}
```

**`Resolver<'a'>` 结构体**

```rust
// 这是主要用于解析的结构体，这是一个很大的结构体，包含了名称解析过程需要的数据信息
/// The main resolver class.
///
/// This is the visitor that walks the whole crate.
pub struct Resolver<'a> {
    session: &'a Session,

    definitions: Definitions,

    graph_root: Module<'a>,

    prelude: Option<Module<'a>>,
    extern_prelude: FxHashMap<Ident, ExternPreludeEntry<'a>>,
    // ...
}

// 用于 Resolver 库里的内存分配
pub struct ResolverArenas<'a> {
    modules: TypedArena<ModuleData<'a>>,
    local_modules: RefCell<Vec<Module<'a>>>,
    imports: TypedArena<Import<'a>>,
    name_resolutions: TypedArena<RefCell<NameResolution<'a>>>,
    ast_paths: TypedArena<ast::Path>,
    dropless: DroplessArena,
}

```

接下来就是一些函数，包括 `report_errors` /  `report_conflict` / `add_suggestion_for_rename_of_use` 等一些用于编译器诊断信息等函数。

### 聚焦于问题

我们现在对 名称解析 功能相关背景有了足够且系统的认识。让我们来看一些代码细节。

根据官方阅读源码的建议，现在这一步，应该是 `Deep` ，聚焦于某些感兴趣或有疑问的函数。 

我对 Rustc 如何检查未使用变量比较感兴趣，就让我们聚焦 `check_unused.rs` 模块中的相关功能。

该模块注释中写道，检查未使用的导入主要分为三步：

第一步： `UnusedImportCheckVisitor` 来遍历 AST 以查找`UseTree`内所有未使用的导入，并且记录它们的 `use`分组和 `NodeId` 信息。

对于 unused trait 方法，则在  `rustc_typeck/check_unused.rs` 中检查。

我们从前面背景资料已经知道，`check_unused` 发生在第三遍 AST 遍历，经过前面两遍遍历，已经构建出了 `UseTree`，只需要走查 `Unused NodeId` 即可：

```rust
struct UnusedImport<'a> {
    use_tree: &'a ast::UseTree,
    use_tree_id: ast::NodeId,
    item_span: Span,
    unused: FxHashSet<ast::NodeId>,  // 内部的 快速 HashSet 存储 NodeId 信息
}

impl<'a> UnusedImport<'a> {
    fn add(&mut self, id: ast::NodeId) {
        self.unused.insert(id);
    }
}

struct UnusedImportCheckVisitor<'a, 'b> {
    r: &'a mut Resolver<'b>,
    /// All the (so far) unused imports, grouped path list
    unused_imports: NodeMap<UnusedImport<'a>>,
    base_use_tree: Option<&'a ast::UseTree>,
    base_id: ast::NodeId,
    item_span: Span,
}

impl<'a, 'b> UnusedImportCheckVisitor<'a, 'b> {
    // We have information about whether `use` (import) items are actually
    // used now. If an import is not used at all, we signal a lint error.
    fn check_import(&mut self, id: ast::NodeId) {
        /* do something */
    }
    
}

// 实现 rustc_ast 中 定义 的 Visitor trait， 这是访问者模式在 Rust 编译器中的应用
// Visitor trait 中定义了 AST Node的访问钩子方法，这样具体的访问者就可以实现 Visitor 的特定方法来进行具体的访问
// 这里具体的访问者就是 UnusedImportCheckVisitor
impl<'a, 'b> Visitor<'a> for UnusedImportCheckVisitor<'a, 'b> {
      fn visit_item(&mut self, item: &'a ast::Item) { /* do something */ }
      fn visit_use_tree(&mut self, use_tree: &'a ast::UseTree, id: ast::NodeId, nested: bool) { /* do something */ }
}
```



第二步： `calc_unused_spans` ，遍历上一步收集的 `NodeId` 关联的 `Span `

```rust
fn calc_unused_spans(
    unused_import: &UnusedImport<'_>,
    use_tree: &ast::UseTree,
    use_tree_id: ast::NodeId,
) -> UnusedSpanResult {
    /* do something */
    match use_tree.kind {
        ast::UseTreeKind::Simple(..) | ast::UseTreeKind::Glob => { /* do something */ }
        ast::UseTreeKind::Nested(ref nested) => {/* do something */}
    }
    /* do something */
}
```

第三步： `check_crate`，根据生成的数据发出诊断信息

```rust
impl Resolver<'_> {
    // 为 Resolver 实现 check_unused 方法
    crate fn check_unused(&mut self, krate: &ast::Crate) {
        /* do something */
        // 检查导入源
        for import in self.potentially_unused_imports.iter() {
            match import.kind {
                ImportKind::MacroUse => { /* do something */ }
                ImportKind::ExternCrate { .. } =>  { /* do something */ }
            }
        }
        let mut visitor = UnusedImportCheckVisitor {
            r: self,
            unused_imports: Default::default(),
            base_use_tree: None,
            base_id: ast::DUMMY_NODE_ID,
            item_span: DUMMY_SP,
        };
        visit::walk_crate(&mut visitor, krate);
        for unused in visitor.unused_imports.values() {
             let mut fixes = Vec::new(); // 为 cargo fix 记录
             /* do something */
             // 计算 unused 位置信息
             let mut spans = match calc_unused_spans(unused, unused.use_tree, unused.use_tree_id) {
             	/* do something */
             }
             /* do something */
             // 发出诊断消息
             visitor.r.lint_buffer.buffer_lint_with_diagnostic(
                UNUSED_IMPORTS,
                unused.use_tree_id,
                ms,
                &msg,
                BuiltinLintDiagnostics::UnusedImports(fix_msg.into(), fixes),
            );
        }
    }
}
```

通过阅读这部分代码，我们大概了解了 `rustc_resolve` 库的组织结构：

- `lib.rs` 中定义主要的 `Resolver`相关类型和方法
- 在不同的 `Resolver` 功能模块中实现具体的 解析方法，比如 `check_unused`

### 回到整体模块

然后，我们再回到整体模块中来了解其他部分的代码。

我们知道第一遍 AST 遍历会构建 简化图 （ `reduced graph`），那么这个过程肯定是对应于[ `build_reduced_graph.rs` ](https://github.com/rust-lang/rust/blob/master/compiler/rustc_resolve/src/build_reduced_graph.rs) 模块。

 我们可以看到该模块引入了 `rustc_ast` / `rustc_expand`/ `rustc_data_structures::sync::Lrc (等价于 Arc)/ rustc_hir::def_id` 等相关组件，可想而知，它是和宏展开相关，并且也支持并行编译。

```rust
impl<'a> Resolver<'a> {
    crate fn define<T>(&mut self, parent: Module<'a>, ident: Ident, ns: Namespace, def: T) where
        T: ToNameBinding<'a>,
    {
        let binding = def.to_name_binding(self.arenas);
        let key = self.new_key(ident, ns);
        // https://github.com/rust-lang/rust/blob/master/compiler/rustc_resolve/src/imports.rs#L490
        // try_define 定义于 imports 模块，解析导入的时候用于检查绑定的名称
        if let Err(old_binding) = self.try_define(parent, key, binding) {
            // 如果命名有冲突，这里会调用 report_conflict 来发出错误报告
            self.report_conflict(parent, ident, ns, old_binding, &binding);
        }
    }
    fn get_nearest_non_block_module(&mut self, mut def_id: DefId) -> Module<'a>  {/* do something */}
    crate fn get_module(&mut self, def_id: DefId) -> Option<Module<'a>>  {/* do something */}
    crate fn expn_def_scope(&mut self, expn_id: ExpnId) -> Module<'a>  {/* do something */}
    crate fn build_reduced_graph(
        &mut self,
        fragment: &AstFragment,
        parent_scope: ParentScope<'a>,
    ) -> MacroRulesScopeRef<'a>  {/* do something */}
    
}
```

实现了构建 简化图需要的  `Resolver` 相关方法。 具体细节我们就不再看了，了解整体流程即可。

## 总结

官方的源码阅读俱乐部活动，旨在带动 Rust 编译器贡献者踊跃去对 Rust 编译器做贡献。但是具体的源码阅读过程，猜测不会太详细，还有很多东西是需要私下去了解的。

本文作为一次源码阅读学习的记录分享，旨在抛砖引玉，如果文中有发现错误，欢迎反馈。































## 参考链接

- [https://github.com/rust-lang/rustc-reading-club](https://github.com/rust-lang/rustc-reading-club) 
- [https://www.manning.com/books/the-programmers-brain](https://www.manning.com/books/the-programmers-brain)
- [代码阅读着色工具（代码要使用永久链接）](https://annotate.code-reading.org/#/)
- [https://courses.cs.washington.edu/courses/cse401/07au/CSE401-07sem.pdf](https://courses.cs.washington.edu/courses/cse401/07au/CSE401-07sem.pdf)
- [https://github.com/rust-lang/rfcs/blob/master/text/1560-name-resolution.md](https://github.com/rust-lang/rfcs/blob/master/text/1560-name-resolution.md)