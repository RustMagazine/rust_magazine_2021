---
pub_date: Thu, 30 Apr 2021 18:00:00 GMT
description: Learn

---

# 学习资源

编辑：张汉东

---

## Rust Cheat Sheet 速查

 内容包含：The Book BK, Rust by Example EX, Std Docs STD, Nomicon NOM, Reference REF。

[https://cheats.rs/](https://cheats.rs/)


## 微软出 Rust 新手教程了

微软给想学习 Rust 的朋友提供了一份新手教程, 手把手带你走出 Rust 第一步.

[https://docs.microsoft.com/en-us/learn/paths/rust-first-steps/](https://docs.microsoft.com/en-us/learn/paths/rust-first-steps/)

## Rust 标准库的 trait 引导大全

Rust 标准库提供了大量的 Trait，每个 Trait 的功能是什么？怎样区分功能有点类似的 Trait 的使用场景？这篇博客非常详细的介绍了标准库中各种内置 Trait，非常值得阅读。

[https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md](https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md)

## 面向 Rust 初学者的错误处理指南

[https://dev.to/seanchen1991/a-beginner-s-guide-to-handling-errors-in-rust-40k2](https://dev.to/seanchen1991/a-beginner-s-guide-to-handling-errors-in-rust-40k2)

## Easy Rust - learn to program in Rust with simple English

Easy Rust 的作者用简单的英语词汇录制了一系列 Rust 教程的视频，发布在了 YouTube。每集 5 到 10 分钟，总共 186 集，23 个小时。

[https://www.youtube.com/playlist?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk](https://www.youtube.com/playlist?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk)

## Rust中的异步流（Part 1）- Future，缓冲和难理解的编译错误

[https://gendignoux.com/blog/2021/04/01/rust-async-streams-futures-part1.html](https://gendignoux.com/blog/2021/04/01/rust-async-streams-futures-part1.html)

## Kafka和Rust入门 Part 1

[https://dev.to/abhirockzz/getting-started-with-kafka-and-rust-part-1-4hkb](https://dev.to/abhirockzz/getting-started-with-kafka-and-rust-part-1-4hkb)

## 如何构建最精简的 Rust Docker 镜像

- [https://kerkour.com/blog/rust-small-docker-image/](https://kerkour.com/blog/rust-small-docker-image/)
- [译文](https://blog.budshome.com/budshome/gou-jian-zui-jing-jian-de-rust-docker-jing-xiang)

## First-Class I/O

即执行 I/O 的函数，可作为参数或返回值在程序中传递，如 File。

> 纯函数式编程是一种巧妙的技巧，用以证明你无需可变即可编码，而 Rust 则是一种更加巧妙的技巧，用以表明你可以进行可变。

First-Class I/O是广泛应用的有用概念，例如面向能力的安全性如何帮助实现[无共享链接](https://github.com/WebAssembly/interface-types/blob/master/proposals/interface-types/Explainer.md#creating-maximally-reusable-modules)。 增量应用也很有用，例如io-stream 或cap-std 库如何帮助程序的各个部分高效地和惯用地进行协作。

[https://blog.sunfishcode.online/first-class-io/](https://blog.sunfishcode.online/first-class-io/)

## 盘点使用 Safe Rust 实现的 标记 GC 

本文盘点了近几年基于 Safe Rust 实现的 跟踪/标记类 GC 库。

[https://manishearth.github.io/blog/2021/04/05/a-tour-of-safe-tracing-gc-designs-in-rust/](https://manishearth.github.io/blog/2021/04/05/a-tour-of-safe-tracing-gc-designs-in-rust/)

## Rust 机器学习列表

有点 awesome-rust-ml 的意思。

[https://github.com/e-tony/best-of-ml-rust](https://github.com/e-tony/best-of-ml-rust)

## 使用 Rust + Lunatic + WebAssembly 构建 TelNet Chat Server  

[https://www.hackernoon.com/how-i-used-rust-lunatic-to-build-a-telnet-chat-server-with-webassembly-rb3l33cg](https://www.hackernoon.com/how-i-used-rust-lunatic-to-build-a-telnet-chat-server-with-webassembly-rb3l33cg)

## 将 Actor 和 Async/Await 连接起来 Part 1

该文作者受官方异步基础组愿景文档影响，也写了一篇关于如何将他实现的 Stakker Actor 库和异步连接起来的一些思考。

Stakker 被设计为分层放置在用户喜欢使用的任何事件循环之上。 它旨在最大程度地利用Rust的编译时检查和优化。

- [https://github.com/uazu/stakker](https://github.com/uazu/stakker)
- [https://uazu.github.io/blog/20210406.html](https://uazu.github.io/blog/20210406.html)

## 【系列】使用 Warp 进行 REST Api 开发

[https://dev.to/rogertorres/series/12179](https://dev.to/rogertorres/series/12179)

## Rust 库的错误管理

如何提供易于理解，易于管理且足够具体的错误，以使用户能够易于处理？文章里给出一种方案。

[http://www.tglman.com/posts/rust_lib_error_management.html](http://www.tglman.com/posts/rust_lib_error_management.html)

## libp2p 教程: 使用 Rust 构建一个点对点应用

在这个教程里,作者会使用 libp2p 来构建一个简单的 peer-to-peer 菜谱应用.

在这个菜谱应用会有以下基本功能:

- 创建菜谱.
- 发布菜谱.
- 列出本地菜谱.
- 列出网络上发现的其他菜谱.
- 列出一个节点上的菜谱.
- 列出所有已知节点上所有的菜谱.

[https://blog.logrocket.com/libp2p-tutorial-build-a-peer-to-peer-app-in-rust/](https://blog.logrocket.com/libp2p-tutorial-build-a-peer-to-peer-app-in-rust/)

## 编写 `*-sys` crate 帮助 Rust 程序使用 C 库

这篇文章简要解释了什么是 `*-sys` crate ，以及如何利用构建脚本以一次性完成 `*-sys` crate 的创建。

[https://kornel.ski/rust-sys-crate](https://kornel.ski/rust-sys-crate)

## 为什么Rust字符串看起来很难

对于常规语言来说，字符串的理解对于大多数人来说不是难事。当新手开始接触Rust时,字符串&str,String会让他们丈二和尚摸不着头脑。本文详细介绍了Rust中字符串使用困难的原因。

[https://www.brandons.me/blog/why-rust-strings-seem-hard](https://www.brandons.me/blog/why-rust-strings-seem-hard)

## 用Rust构建小型滴灌系统

作者用Rust编写了一个给绿植的灌溉功能的小型系统。非常有意思的是，用的还是国内厂商的电磁阀。如果你对树莓派、硬件电路感兴趣的话，不要错过它。

[https://github.com/kitallis/WAP](https://github.com/kitallis/WAP)

## 嵌入式系统中使用 std

估计很多有人也有这个疑问, 我们能在嵌入式系统编程中使用标准库 std 吗?

作者同样有这个疑问, 但是他找到了自己的答案, 有同样困扰的小伙伴可以参考一下.

[http://blog.timhutt.co.uk/std-embedded-rust/index.html](http://blog.timhutt.co.uk/std-embedded-rust/index.html)

## [Rust] Github Actions 最好的实践

在一个相当大的项目中，作者用 GitHub Actions 来完成一些重复性的工作，主要的两个改进就是：合并多个任务，使用 sccahe 提高测试速度

- GitHub workflows 矩阵
- 包含和排除规则
- 使用 sccahe 优化Rust编译速度
- 检验 sccache 结果

[https://www.fluvio.io/blog/2021/04/github-actions-best-practices/](https://www.fluvio.io/blog/2021/04/github-actions-best-practices/)

## Rust 实现的一款有意思的迷宫游戏

挺好玩的 。。。

基于 Rust 实现，并可以导出 wasm 

[https://ldjam.com/events/ludum-dare/48/$242669](https://ldjam.com/events/ludum-dare/48/$242669)

- 源码：[https://github.com/Healthire/ld48](https://github.com/Healthire/ld48)
- 浏览器play：[https://healthire.github.io/ld48/](https://healthire.github.io/ld48/)

你可以关注 https://github.com/Healthire ，他不止用 Rust 做了这一个游戏

## Rust 中如何使用 gRPC

本文介绍了 如何在Rust中创建gRPC服务器和客户端。 出于可见性目的，客户端还将是Telegram机器人。

[https://romankudryashov.com/blog/2021/04/grpc-rust/](https://romankudryashov.com/blog/2021/04/grpc-rust/)

## 一个 actix-web 中使用 tokio-tracing 的示例

[https://github.com/LukeMathWalker/tracing-actix-web](https://github.com/LukeMathWalker/tracing-actix-web)

## Rust 中可能实现 移动构造函数（Move Constructors） 吗？

移动构造函数（Move Constructors） 是 Cpp 11 引入的一个特性，它允许开发者通过拿其他已存在对象来构造新的对象，从而避免深拷贝导致的效率问题。

如果 Rust 里也支持，那么也可以同样减少深拷贝效率问题，实现就地构造对象。 本文作者探讨了如何安全地实现这个特性，作者为此也实现了一个库 [moveit](https://crates.io/crates/moveit)。

moveit 中充分利用了 Pin/Unpin 来实现该特性，除了 Rust 还支持 Cpp 的移动构造函数，以备 FFi 时候方便。虽然实现了 Unsafe trait ，但作者指明了 Safety 边界。

- [https://mcyoung.xyz/2021/04/26/move-ctors/](https://mcyoung.xyz/2021/04/26/move-ctors/)
