---
pub_date: Tue, 31 May 2021 21:00:00 GMT
description: Rust Learn

---

# 学习资源

编辑: Matrixtang

---

## RESTful API in Sync & Async Rust

[原文](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md)
来源是 github 上的 `pretzelhammer`, 他在 github 上开源了自己的 rust 博客, 上面的文章质量很高。
本文介绍了如何在 Rust 中为一个虚构的看板风格的项目管理应用程序实现一个 RESTful API 服务器。

## The Rust Borrow Checker—A Deep Dive

[视频链接](https://www.youtube.com/watch?v=Ys7ma3au5m0)
本视频由 `MS` 的工程师 ` Nell Shamrell-Harrington` 带来。对 `Borrow Checker` 工作方式感兴趣的同学可以看一下这个视频。

##　 Naming Your Lifetimes
[原文](https://www.possiblerust.com/pattern/naming-your-lifetimes)

有效地命名生命周期可以提高代码的可读性，本文对此进行了详细介绍。

## Rust 中最让我讨厌的东西

[原文](https://blog.yossarian.net/2020/05/20/Things-I-hate-about-rust)

Rust 是作者最喜欢的语言，但是其中一些东西也让作者喜欢不起来：比如太多的字符串类型 `(&str, String, &OsStr, OsString, AsRef)`，再比如 `impl<T> for Trait for T where T: OtherTrait` 这样的写法太过麻烦等

## Oxidizing the technical interview

[原文](https://blog.mgattozzi.dev/oxidizing-the-technical-interview/)

为 Rust 面试提供了一些新思路

## Rucredstash release & Rust experience from a Haskeller

[原文](https://psibi.in/posts/2021-05-22-credstash.html)

Rust 身上有函数式编程的影子, 因此也吸引了不少喜欢函数式爱好者的关注。本文讲述了一个 Haskell 程序员的 Rust 学习之路。

## Why and how we wrote a compiler in Rust - (blog post series 1/X): the context

[原文](https://bnjjj.medium.com/why-and-how-we-wrote-a-compiler-in-rust-blog-post-series-1-x-the-context-e2f83b10edb9)

这篇博客文章是该系列的第一篇文章，讲述了为何要使用 Rust 来编写编译器, 以及如何实现 一个用 Rust 编写的编译器 demo。

## Scylla Developer Hackathon: Rust Driver

[原文](https://www.scylladb.com/2021/02/17/scylla-developer-hackathon-rust-driver/)

Scylla 的开发者们在黑客马拉松上使用 Rust 来编写驱动,一起来看看他们是如何使用 Rust 的吧。

## How we utilized fuzzing to improve security in the TezEdge node and created an open-source CI tool for Rust code fuzzing.

[原文](https://medium.com/tezedge/how-we-utilized-fuzzing-to-improve-security-in-the-tezedge-node-and-created-an-open-source-ci-tool-92ffbd804db1)

本文讲述了作者使用 Fuzz 工具和开源 CI 工具来提高项目安全性的经历。 Fuzz 已经成为开源基础设施的必选项了。

## Verifying vectorized Rust revisited

[原文](https://project-oak.github.io/rust-verification-tools/2021/05/15/verifying-vectorized-code2.html)

实践出真知, 作者将带领读者一探 rustc 如何处理 `vector instructions`

## Writing Pythonic Rust

[原文](https://www.cmyr.net/blog/rust-python-learnings.html)

想必大家对 Python 都不会陌生, Python 语法简单, 其风格被成为 `Pythonic`。在 Rust 中也可以写出 Pythonic 的代码。

## Routes to Discovering Rust

[原文](https://blog.abor.dev/p/timclicks)

作者在文中讲述了自己的 Rust 探索之路。

## Rust Verification Workshop 2021

[video](https://youtu.be/iAs0gZ8o0oQ) Rust Verification Workshop 2021 - Ferrite: A Rust EDSL for Message-passing Protocol Verification

[video](https://www.youtube.com/watch?v=iAs0gZ8o0oQ) Rust Verification Workshop 2021 - RustBelt: A Quick Dive into the Abyss

[video](https://www.youtube.com/watch?v=H54VDCuT0J0) Rust Verification Workshop 2021 - Polonius

[video](https://youtu.be/_DM36e2A9dg) Rust Verification Workshop 2021 - Rust Interest in safety- and mission-critical environments

[video](https://youtu.be/0DcIn7kiNxM) Rust Verification Workshop 2021 - Leveraging Compiler Intermediate Representation for Multi- and Cross-Language Verification

Rust Verification Workshop 2021 年的技术分享

## 宏小本的最新中文翻译

[GitHub page](https://zjp-cn.github.io/tlborm/)

这是对 Daniel Keep 撰写的书 的续写， 自 2016 年初夏以来，那本书就一直没再更新。本书的续写者为 Veykril。译者是 zjp-cn。

## An Incomplete Explanation of the Proc Macro That Saved Me 4000 Lines of Rust

[原文](https://mbuffett.com/posts/incomplete-macro-walkthrough/)

本文中作者使用了 Proc Macro 解决了代码复用问题。

## The most underrated but useful Rust standard library type

[原文](https://dev.to/thepuzzlemaker/the-most-underrated-but-useful-rust-standard-library-type-59b1)

Rust 标准库充满了许多有用的类型，特征和抽象。 作者介绍了一个被我们经常忽略的一个类型 `Cow`

## Compilers as Teachers

[原文](https://ferrous-systems.com/blog/compilers-as-teachers/)

Rust 程序员经常被编译器 "折磨" 的失去活来, 其实不妨把编译器看作一个教导自己的导师。

##　 Is it possible to write overhead-free cyclic data-structures in safe, stable Rust?
[原讨论地址 readdit](https://www.reddit.com/r/rust/comments/n420cg/is_it_possible_to_write_overheadfree_cyclic/)

是否可以在稳定的 Rust 中编写无开销的循环数据结构？

## 构建 Rust 异步 GraphQL 服务：基于 tide + async-graphql + mongodb

[原文](<https://blog.budshome.com/budshome/gou-jian-rust-yi-bu-graphql-fu-wu-:ji-yu-tide-+-async-graphql-+-mongodb(3)--zhong-gou>)

基于 actix-web + async-graphql + rbatis + postgresql / mysql 构建异步 Rust GraphQL 服务, 本文是系列文章。

## Crust of Rust: Dispatch and Fat Pointers

[video](https://www.youtube.com/watch?v=xcygqF5LVmM)

视频来源于 Youtube 上一个著名的 Youtuber, `Jon Gjengset`。对静态分发和动态分发背后感兴趣的同学可以参考一下。

## Using GDB and defmt to debug embedded programs

[原文](https://ferrous-systems.com/blog/gdb-and-defmt/)

能直接在嵌入式系统中使用 gdb 当然是 "坠" 好的了。

## How Rust makes Rayon's data parallelism magical

[原文](https://developers.redhat.com/blog/2021/04/30/how-rust-makes-rayons-data-parallelism-magical)

`Rayon` 是用于 Rust 编程语言的数据并行性库。使用 `Rayon` 在 Rust 中开启并行之路。

## Linux 基金会提供的免费 WebAssembly 课程

此在线课程是为已经在构建微服务和云本机应用程序开发方面有经验的开发人员设计的。 简而言之，这不是WebAssembly的介绍。 但是，如果您准备在本地构建，试验和测试功能即服务（FaaS），则该类适合您。

具体来说，您将学习如何使用Rust创建和托管WebAssembly模块。 您还将学习如何将JavaScript WebAssembly API用于浏览器，以及有关WebAssembly的替代性非Web主机运行时的信息。

您将了解到可以使用社区工具和开源项目添加到基本WebAssembly规范中。 学完这些，您将清楚地了解如何构建基于WebAssembly的应用程序以及它们的实际工作方式。

[https://www.zdnet.com/article/linux-foundation-offers-free-webassembly-online-class/](https://www.zdnet.com/article/linux-foundation-offers-free-webassembly-online-class/)
