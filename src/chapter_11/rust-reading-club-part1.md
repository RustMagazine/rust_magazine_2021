# Rust 源码阅读俱乐部 | 第一期



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

参与前准备：

1. 阅读 《the-programmers-brain》 这本书
2. 阅读 Rustc dev guide ，了解下 Rust 编译过程
3. 阅读 [compiler/rustc_resolve/src](https://github.com/rust-lang/rust/tree/master/compiler/rustc_resolve/src)  文档相关，了解该库



##  Rust 飞书群 Rust 源码阅读俱乐部

为了响应官方的活动，我在飞书 Rust 中文社群也准备每周六或日，也组织一次源码阅读在线沙龙。

内容就是跟随官方内容，共同学习。

并且会把每周源码学习记录下来，行为文字输出，分享给大家。



## 学前准备



### The Programmer's Brain

Rust 官方推荐了《The Programmer's Brain》 这本书，该书好像没有引入中文版。在 Manning 的官网可以免费看这本书的在线版本。如果要翻译书名的话，我觉得《编程大脑》这个名字还行。

这本书分为四部分：

1. 如何更好地阅读代码
2. 关于代码的思考
3. 编写更好的代码
4. 关于代码协作

这本书的特点在于，它会介绍不同类型代码和大脑认知思维之间的联系，而不是纯技巧罗列。

既然 Rust 官方推荐这本书，那说明它的内容还是有一定价值，感兴趣可以去看看。



## Rustc Resolve 

第一期的内容聚焦在 `rustc_resolve` 这个库上。

这个库主要是负责名称解析，这个过程不需要类型检查。

crate 的模块在这里构建，宏的路径、模块导入、表达式、类型、模式都是在这里解析的。标签（label）和生命周期也都是在这里求解的。

类型相关的名称解析（方法、字段、关联项）发生在`rustc_typeck` 上。

### 我的源码阅读习惯

我阅读源码和读书的方式一致，都是从整体结构到细节。

包括在阅读 rustc_resolve 这个库的时候，我先从其文档着手。一个crate 的文档可以非常清晰的展现出这个 crate 的整体结构。

### rustc_resolve  的整体结构

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
- 

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



### 编译过程

Rust 编译过程大体分为：`词法分析 ->  语法分析 ->  HIR ->  MIR ->  LLVM IR` 这几个过程。

名称解析过程应该发生在 语法分析的过程中，并且宏也展开完毕。 

严格来说，词法分析、语法分析基本都是同时进行的，只是为了方便介绍和理解而分了先后顺序。



## 学习记录

### 2021.11.04 官方第一期



































## 参考链接

- [https://github.com/rust-lang/rustc-reading-club](https://github.com/rust-lang/rustc-reading-club) 
- [https://www.manning.com/books/the-programmers-brain](https://www.manning.com/books/the-programmers-brain)
- [代码阅读着色工具（代码要使用永久链接）](https://annotate.code-reading.org/#/)