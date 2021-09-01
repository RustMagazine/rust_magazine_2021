# Trait Upcasting 系列 ｜ 如何把子 trait 转成父 trait ？

作者： CrLF0710

> 本文由 Trait Upcasting 贡献者 CrLF0710（猫老师）来介绍一下特质向上类型转换(Trait upcasting coercion) 这个功能。

---

此功能对应的MCP是[Trait Upcasting · Issue #98 · rust-lang/lang-team (github.com)](https://link.zhihu.com/?target=https%3A//github.com/rust-lang/lang-team/issues/98)。

现在是2021年8月，主体功能已经完成了，在nightly上可以试用。

目前还剩两个边角情况没支持：

1. 在新的vtable设计方案下，trait upcasting 在多重继承存在的情况下有时需要实际访问vtable 内容。因为原生指针(raw pointer)的metadata是否必须有效是未解决问题，因而对原生指针的upcasting 应该是unsafe操作。这又涉及到了CoerceUnsized 相关的一些问题。（本条为这个功能目前为incomplete_features的主要原因）

2. 在多重继承下，trait存在列表中可能具有相同父trait使用不同泛型参数的情况。这里的类型推导有一些实现上的问题，目前尚未实现。（这种情况下会提示无法转换）

接下来我会继续推动这个功能完成，然后根据实现经验撰写一篇RFC。如果有必要的话，我也许会再看看在chalk下怎么实现这个功能。

接下来简单介绍下这个功能怎么用，其实还蛮自然的：

1需要转换的地方，标注转换的目标类型（函数参数、返回值之类的地方已经有标注了），然后如果编译器认为可以（认定的方法可以参见下面的详细设计），转换就会成功了。

以下是详细设计：

1. 对于实现了CoerceUnsized特质的类型可以在两个类型(暂时称为T, U)间进行尺寸擦除类型转换(unsizing coercion)，最常用的有从 &T -> &U, Box<T> 到 Box<U>等等。类型转换的场景(coercion site) 简单说就是转换目标有类型标注的情况，包括let上的类型标注，函数参数的标注，返回值的标注，as表达式等等。
2. 特质对象的语法是 dyn PrincipalTrait [+ AutoTrait]* [+ 'lifetime]* 或 dyn AutoTrait [+ AutoTrait]* [+ 'lifetime]* 其中方括号表示可选，星号表示0个或任意个。
这里的PrincipalTrait是一个Rustc内部术语，指的是占据特质对象主导地位的那个特质。（Rust目前不支持多个）。特质对象上面的方法/关联函数由这个特质和它的所有的祖先特质决定。
3. 特质对象之间的尺寸擦除类型转换原本就存在，主要包括三种：
    a. 减少AutoTrait部分
    b. 在subtyping规则允许的范围下调整lifetime
    c. 本次新增：将PrincipalTrait置换为它的任意一个祖先特质。
4. 置换的方法是调整trait object的metadata，也就是vtable的指针。我们本次重新设计了vtable的结构：
    对于 
    ```text
    A
    / \
    B C
    \ /
    D
    ```

这样的菱形继承结构，我们保障最左边一列 A- B 特质所需的vtable恰好是D的vtable的前缀（从而对单继承优化），并为这一列之外的特质(C)在vtable中存储一个vtable指针。
实际转换时，对于最左边一列的向上转换，是no-op，不会做任何事。对于这一列之外的转换，从vtable中取出对应的vtable指针替换即可。

目前官方 T-lang下推动这件事的活动(initiative)，有反馈和讨论都会放这个仓库里：[https://github.com/rust-lang/dyn-upcasting-coercion-initiative/](https://github.com/rust-lang/dyn-upcasting-coercion-initiative/)，感兴趣的可以关注。

