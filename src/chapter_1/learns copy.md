# 本月简报：学习资源

- 来源：[Rust日报](https://rustcc.cn/section?id=f4703117-7e6b-4caf-aa22-a3ad3db6898f)
- 作者：Rust 日报小组


## Rust 书籍宝库

glynnormington 整理了网络上大部分有关 rust 的 mdbook，有官方的，也有非官方的。值得注意的一点是大家关注的 Rust 宏小册很多人以为一直没有更新，但是其实有另一个团队重新在原来的基础上，更新了新的版本，目前已收录到该书库中。

[原文链接](https://www.reddit.com/r/rust/comments/kwiwb8/the_little_book_of_rust_books/)

[项目地址](https://lborb.github.io/book/title-page.html)

## 使用 Rust 创建一个模拟器: part 1

这个系列中, 作者会通过 神经网络 和 遗传算法 制作一个 进化模拟器.

作者首先会介绍 神经网络和遗传算法 是如何工作的, 然后会使用 Rust 来实现他们, 并且编译成 WebAssembly. 下图是一个预览图.

教程地址： [https://pwy.io/en/posts/learning-to-fly-pt1/](https://pwy.io/en/posts/learning-to-fly-pt1/)

## Rust陷阱: repr(transparent)

repr(transparent) 可以让类似 struct Foo(i32) 和 i32 有同样的内存分布方式. 他作用范围非常具体,只能有一个非 0 size 的字段.

本文章介绍了如何使用 repr(transparent) 以及一些陷阱.

原文链接：[https://jack.wrenn.fyi/blog/semver-snares-transparent/](https://jack.wrenn.fyi/blog/semver-snares-transparent/)

## Unsafe Rust：该如何或何时使用它

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

## Mozilla: 如何导出 Rust 组件给 Kotlin 

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

## 【译】Async/Await（二）—— Futures

新的文章翻译来啦。

来自：公众号：「Rust 碎碎念」，翻译 by：Praying

- 翻译链接： [https://mp.weixin.qq.com/s/OL7_usSmY_gAZzYYydyr8A](https://mp.weixin.qq.com/s/OL7_usSmY_gAZzYYydyr8A)
- 原文链接：[https://os.phil-opp.com/async-await/#multitasking](https://os.phil-opp.com/async-await/#multitasking)


## LibHunt: 根据reddit 被提及状态展示 rust 库的热度

LibHunt 根据 reddit 上大家提及到库的热度来排序出一些热门的 rust 库.

对于调研阶段的同学来说,是一个很好的工具.

libhunt的主页地址: [https://www.libhunt.com/lang/rust](https://www.libhunt.com/lang/rust)

## 用 Rust 实现一个 Rest Client

这是 << Zero To Production In Rust>> 的 这本书中的一个示例。在本文, 作者演示了:

- 如何使用 reqwests 来写一个 REST API client.
- 如何来使用 wiremock 来进行测试.

原文链接: [https://www.lpalmieri.com/posts/how-to-write-a-rest-client-in-rust-with-reqwest-and-wiremock/](https://www.lpalmieri.com/posts/how-to-write-a-rest-client-in-rust-with-reqwest-and-wiremock/)

## 太素OS：基于 RISCV 架构的 Rust 系统内核实现（中文）教程和源码

构建于QEMU 之上，适合学习

- 源码： [https://github.com/belowthetree/TisuOS](https://github.com/belowthetree/TisuOS)
- 教程： [https://www.zhihu.com/column/c_1118934193425629184](https://www.zhihu.com/column/c_1118934193425629184)

