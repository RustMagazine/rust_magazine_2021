
# 本月简报：学习资源

- 来源：[Rust日报](https://rustcc.cn/section?id=f4703117-7e6b-4caf-aa22-a3ad3db6898f)
- 作者：Rust 日报小组


## 🎈Rust Design Patterns Book

非官方好书系列, 再次安利! Rust Design Patterns Book. 作者最近更新了很多东西。

看下翻译的中文引言吧。

#### 引言

#### 设计模式

在开发程序中，我们必须解决许多问题。一个程序可以看作是一个问题的解决方案。它也可以被看作是许多不同问题的解决方案的集合。所有这些解决方案共同解决一个更大的问题。

#### 在Rust中的设计模式

有许多问题的形式是相同的，由于事实上，rust不是面向对象设计，模式不同于其他面向对象程序设计语言，虽然细节是不同的，因为他们有相同的形式，他们可以解决使用相同的基本方法。

[设计模式](https://rust-unofficial.github.io/patterns/patterns/index.html)是解决编写软件时常见问题的方法。

[反模式](https://rust-unofficial.github.io/patterns/anti_patterns/index.html)是解决这些相同问题的方法。

然而，尽管设计模式给我们带来了好处，反模式却带来了更多的问题。

[惯用法](https://rust-unofficial.github.io/patterns/idioms/index.html)，是编码是要遵守的指南，他们是社区的社区规范，你可以破他们，但如果你这样做，你应该有一个很好的理由。

TODO: 说明为什么Rust是一个有点特殊功能要素，类型系统，借用检查。

[book 链接](https://rust-unofficial.github.io/patterns/)


## 🎈异步书翻译更新啦

这次翻译新增了 第八章-关于生态的叙述 （[@EthanYuan](https://github.com/EthanYuan)) 以及 第九章 http服务器项目（[@huangjj27](https://github.com/huangjj27)), 欢迎来指正错误或贡献~

[english book link](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html)

[中文翻译链接](https://huangjj27.github.io/async-book/index.html)

​
## 🎈Manning的Rust新书《Refactoring to Rust》

这本书正在MEAP阶段，目前才更新了3章，感兴趣的同学可以看看。

[Refactoring to Rust](https://www.manning.com/books/refactoring-to-rust)


## 🎈Rust 书籍宝库

[glynnormington](https://www.reddit.com/user/glynnormington/)整理了网络上大部分有关rust的mdbook，有官方的，也有非官方的。值得注意的一点是大家关注的rust宏小册很多人以为一直没有更新，但是其实有另一个团队重新在原来的基础上，更新了新的版本，目前已收录到该书库中。

[Rust 书籍宝库](https://lborb.github.io/book/title-page.html)

[Read More on reddit](https://www.reddit.com/r/rust/comments/kwiwb8/the_little_book_of_rust_books/)


## 🎈使用Rust 编写一门语言

有关使用[Rust](https://rust-lang.org/)编程语言制作称为[Eldiro](https://github.com/arzg/eldiro)的编程语言的系列文章。

[原文链接](https://arzg.github.io/lang/)


## Rust 错误处理: python 同学专用

本文是 python 同学专用, 介绍了 python 日常中的错误处理以及如何在 rust 中达到类似效果和最佳实践.

[原文链接](https://theomn.com/rust-error-handling-for-pythonistas/)

## 🎈其他语言调用Rust - C++

作者选择Rust作为运行时库的实现语言，并且希望使同一库可用于不同的编程语言。

最初，选择从对三种语言的支持开始：

- **Rust**：因为这是我们的实现语言。
- **C ++**：这是我们熟悉的低级语言，仍然是嵌入式设备领域中最成熟的语言之一。
- **JavaScript / TypeScript**：因为它是一种非常流行的动态语言。

![img](https://sixtyfps.io/blog/expose-rust-library-to-other-languages/diagrams.png)

Rust库（也称为板条箱）分为两部分，共享实现板条箱和精简惯用的API条板箱。

对于JavaScript，我们使用[Neon](https://github.com/neon-bindings/neon)公开API。Neon使我们能够方便地编写JavaScript API和创建NPM包。

C ++部分更具挑战性。

[原文链接](https://sixtyfps.io/blog/expose-rust-library-to-other-languages.html)

## 🎈使用 Rust 创建一个模拟器: part 1

这个系列中, 作者会通过 神经网络 和 遗传算法 制作一个 进化模拟器.

作者首先会介绍 神经网络和遗传算法 是如何工作的, 然后会使用 Rust 来实现他们, 并且编译成 WebAssembly. 下图是一个预览图.

教程地址： [https://pwy.io/en/posts/learning-to-fly-pt1/](https://pwy.io/en/posts/learning-to-fly-pt1/)

## 🎈Rust陷阱: repr(transparent)

repr(transparent) 可以让类似 struct Foo(i32) 和 i32 有同样的内存分布方式. 他作用范围非常具体,只能有一个非 0 size 的字段.

本文章介绍了如何使用 repr(transparent) 以及一些陷阱.

原文链接：[https://jack.wrenn.fyi/blog/semver-snares-transparent/](https://jack.wrenn.fyi/blog/semver-snares-transparent/)

## 🎈Unsafe Rust：该如何或何时使用它

本文包含了以下内容：

- 关于 Unsafe Rust 的五点迷思
- 什么时候不该用 Unsafe 的代码
- 处理未初始化的内存
- 内部可变性
- 内在动机
- 内联汇编
- FFi
- 编写Unsafe Rust时候应该使用的工具

原文链接：[https://blog.logrocket.com/unsafe-rust-how-and-when-not-to-use-it/](https://blog.logrocket.com/unsafe-rust-how-and-when-not-to-use-it/)

## 🎈Mozilla: 如何导出 Rust 组件给 Kotlin 

Mozilla 应用服务平台这个仓库中提供了一个 login 组件可以很好地展示这个示例。

概要：

假设你已经的组件在./src/目录下编写了一个不错的Rust核心代码。

首先，你需要将 Rust API 扁平化为一组 FFI 绑定，通常是在 `./ffi/ `目录下。使用 `ffi_support` crate 来帮助实现这个功能，这将涉及到在核心 Rust 代码中实现一些特性。

接下来，你需要编写消耗`FFI`的`Kotlin`代码，通常是在`./android/`目录下。这段代码应该使用`JNA`通过共享库加载编译后的`Rust`代码，并将其作为一个漂亮的安全且易于使用的`Kotlin API`暴露出来。

似乎我们很可能在这里提供一个有用的模板来让你入门。但我们还没有这样做。

最后，将你的包添加到`android-components repo`中。

文章还回答了一些导出过程中的问题。

原文链接：[https://github.com/mozilla/application-services/blob/020a3eb831da8cd9d21978e3d1fb7af3a6ffcfea/docs/howtos/exposing-rust-components-to-kotlin.md](https://github.com/mozilla/application-services/blob/020a3eb831da8cd9d21978e3d1fb7af3a6ffcfea/docs/howtos/exposing-rust-components-to-kotlin.md)

## 入门教程：用Rust写一个todo应用

在这篇教程里，作者依照javscript的传统，教你用Rust写一个todo应用。 你会学到：

- Rust中的错误处理
- Option的使用
- Struct和impl
- 终端输入输出
- 文件操作
- 所有权和借用
- 模式匹配
- 迭代器和闭包
- 使用外部crate

链接：[https://www.freecodecamp.org/news/how-to-build-a-to-do-app-with-rust/](https://www.freecodecamp.org/news/how-to-build-a-to-do-app-with-rust/)


## 🎈LibHunt: 根据reddit 被提及状态展示 rust 库的热度

LibHunt 根据 reddit 上大家提及到库的热度来排序出一些热门的 rust 库.

对于调研阶段的同学来说,是一个很好的工具.

libhunt的主页地址: [https://www.libhunt.com/lang/rust](https://www.libhunt.com/lang/rust)

## 🎈用 Rust 实现一个 Rest Client

这是 << Zero To Production In Rust>> 的 这本书中的一个示例。在本文, 作者演示了:

- 如何使用 reqwests 来写一个 REST API client.
- 如何来使用 wiremock 来进行测试.

原文链接: [https://www.lpalmieri.com/posts/how-to-write-a-rest-client-in-rust-with-reqwest-and-wiremock/](https://www.lpalmieri.com/posts/how-to-write-a-rest-client-in-rust-with-reqwest-and-wiremock/)

## 🎈太素OS：基于 RISCV 架构的 Rust 系统内核实现（中文）教程和源码

构建于QEMU 之上，适合学习

- 源码： [https://github.com/belowthetree/TisuOS](https://github.com/belowthetree/TisuOS)
- 教程： [https://www.zhihu.com/column/c_1118934193425629184](https://www.zhihu.com/column/c_1118934193425629184)

## 【译】Async/Await（二）—— Futures

新的文章翻译来啦。

来自：公众号：「Rust 碎碎念」，翻译 by：Praying

- 翻译链接： [https://mp.weixin.qq.com/s/OL7_usSmY_gAZzYYydyr8A](https://mp.weixin.qq.com/s/OL7_usSmY_gAZzYYydyr8A)
- 原文链接：[https://os.phil-opp.com/async-await/#multitasking](https://os.phil-opp.com/async-await/#multitasking)



## Rust Programming Language: The Ultimate Guide

这篇文章中作者从伪代码出发，一步步教你实现一个爱情计算器。

作者称这是线上最通俗易懂的Rust入门指南，你怎么认为呢？快来试试吧。

链接：[https://masteringbackend.com/posts/rust-programming-the-ultimate-guide](https://masteringbackend.com/posts/rust-programming-the-ultimate-guide)



## Rust: Initial thoughts

作者分享了自己刚开始学Rust的一些想法和与其它语言的对比。

[原文链接](https://dev.to/hb/rust-initial-thoughts-4jka)


## 关于Future::join设计的思考

这篇文章中作者分享了关于如何将`Future::{try_}join`和`{try_}join!`以一种更一致的形式加入标准库中的思考，以及对于const-eval可能起到的作用的讨论。

[原文链接](https://blog.yoshuawuyts.com/future-join-and-const-eval/)


### Rust 教程: 从头开始学 Rust

Rust 越来越被更多的人喜爱, 很多小伙伴也想入坑. 这篇教程可以帮助零基础的小伙伴了解 Rust.

![img](https://www.educative.io/api/page/5328531525992448/image/download/6018549143830528)

[原文链接](https://www.educative.io/blog/rust-tutorial-from-scratch)



### ref vs & in variables

帖子讨论了ref 和& 的使用，哪个使用更好。

[原文链接](https://www.reddit.com/r/rust/comments/l451ux/ref_vs_in_variables/)

### 在Rust中包装错误

在开发时错误处理是必须，有时错误处理非常糟糕，文章中提高了warp Error 提高体验。

[原文链接](https://edgarluque.com/blog/wrapping-errors-in-rust)

[on reddit](https://www.reddit.com/r/rust/comments/l3x6p0/wrapping_errors_in_rust/)
