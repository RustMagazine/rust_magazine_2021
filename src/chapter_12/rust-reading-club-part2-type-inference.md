# Rust 源码阅读俱乐部 |  第二期 : 类型推断

说明： 这不是线上沙龙的文字记录，而是我本人会后的学习记录。

## 引子

没有等到官方的源码分享，但是我们中文社群线上沙龙继续进行第二期活动。

- 14:00 ~ 15:30 rustc 源码阅读之类型推断  分享者： 字节跳动工程师 刘翼飞
- 16:00 ～ 17:30  Rust 异步运行时 Tokio 源码阅读系列之一   分享者： detenlord 联合创始人 施继成

如何加入飞书的Rust 中文社群? [https://github.com/ZhangHanDong/rust-code-reading-club/issues/1](https://github.com/ZhangHanDong/rust-code-reading-club/issues/1)

本文是关于 类型推导 相关内容的学习记录。

## 类型表示 与 静态分析

在源码阅读俱乐部活动第一期中，我们了解了名称解析。名称解析是编译器内部关于类型表示的内容之一。

如果把 Rust 编译器看作是一本书的话，它大概包含两部分内容：类型表示 和 静态分析。

类型表示的内容有很多，比如 从词法分析到 `AST` ，再到 `HIR/MIR/ LLVM IR` 等，不管什么形式，都是编译器的类型表示。 

那么 Rust 编译器的类型系统，包括类型检查、类型推导、所有权语义、借用检查等等，这些都是利用不同的类型表示，进行静态分析而得到的效果。

所以，我们源码阅读俱乐部，对 Rust 编译器源码的内容，大概也就是围绕这两部分内容来展开。

当然，Rust 语言项目本身除了编译器之外，还有标准库源码，也可以阅读，但这个是后话了。


## 静态分析

静态程序分析是在不实际运行计算机程序的情况下对其行为进行推理的艺术。

在 [Static Program Analysis](https://cs.au.dk/~amoeller/spa/spa.pdf) 一书中，介绍了 Hindley-Milner 类型推断算法的一个变种。很多编程语言，包括 ML/ OCaml/ Haskell/ Rust 的类型系统都是 HM 算法为基础。


## 类型推断

第二期活动，直接跳到了静态分析部分相关内容。我不太能猜到官方的第二期会围绕什么内容开展，但我估计应该不会是静态分析部分，我感觉这部分内容有点跨度太大，不过好在中文社群里有朋友可以引导大家来学习这部分内容。

### 什么叫类型推断

类型推导是自动检测表达式类型的过程。它允许 Rust 使用更少或不使用类型注解，从而使用户更容易编写代码：

```rust
fn main() {
    let mut things = vec![]; // 这里就可以省略掉 `Vec<&str>` 的类型注解
    things.push("thing");
}
```

类型推断基于标准的 Hindley-Milner (HM) 类型推断算法，但以各种方式扩展以适应子类型、区域推断和更高级别的类型。




## 参考资料

- [Static Program Analysis 静态分析英文书籍](https://cs.au.dk/~amoeller/spa/spa.pdf)
- [南京大学-沉浸式《程序分析》教材](https://zhuanlan.zhihu.com/p/417187798)