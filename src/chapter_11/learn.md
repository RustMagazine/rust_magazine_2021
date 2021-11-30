# 学习资源

聚焦学习 Rust 的网络资源

---

## 使用 Rust 处理信号

信号是 linux 上进程生命周期的重要组成部分，但它们非常复杂，要使用它们需要非常小心。这篇文章介绍了信号处理程序中有挑战的问题之一：信号处理程序的限制。

Unix 信号难以处理的原因总结：

- 它们是一种全局资源。如果一个库想设置自己的信号处理程序，它就有可能干扰其他库。有可能将以前的信号处理程序串联起来，但这样就不可能以任何实际的方式将旧的信号处理程序从串联链中移除。
- 它们可以从任何线程中被调用，需要同步。另外，由于它们可以在任何时候中断（interrupt）一个线程，使得大多数处理方式都容易发生竞争。
- 根据 POSIX 标准，一个信号处理程序中可以调用的函数被限制在很少的函数集中。要强调的是，互斥（或其他锁定机制）以及内存分配和删除是不允许的。

本文还推荐了一个安全正确处理 Unix 信号的库： [signal-hook](https://github.com/vorner/signal-hook)，用来解决上面的问题。

[https://www.jameselford.com/blog/working-with-signals-in-rust-pt1-whats-a-signal/](https://www.jameselford.com/blog/working-with-signals-in-rust-pt1-whats-a-signal/)

##  Gitoxide 作者编写的 Rust 学习视频教程

[gitoxide](https://github.com/Byron/gitoxide) 是纯 Rust 实现的 Git 。

视频教程 : [Learn Rust with Gitoxide](https://www.youtube.com/watch?v=LDlBTbO8oQ4&list=PLMHbQxe1e9Mk5kOHrm9v20-umkE2ck_gE) ，源码地址： [https://github.com/Byron/learning-rust-with-gitoxide](https://github.com/Byron/learning-rust-with-gitoxide) 

## Horcrux：在Rust中实现Shamir密钥共享（第1部分）

[Horcrux: Implementing Shamir's Secret Sharing in Rust (part 1)](https://gendignoux.com/blog/2021/11/01/horcrux-1-math.html)

> 就像我在之前一篇博客中提到过的那样，归因于内存安全、强类型、简单易用的单元测试和高性能等特性，我认为Rust是一门极好的用来实现加密算法的编程语言。

Horcruxs是一个用来展示如何用Rust来实现Shamir共享密钥算法的程序例子，目前它还不能被用于生产中。

从这篇博客开始，作者将从基本的数学原理开始讲起，并带领读者学习如何用Rust来实现Shamir共享密钥算法（关于Rust实现部分的讲解需要等到下一篇博客，有兴趣的读者可以关注作者的动态）。

项目地址： https://github.com/gendx/horcrux

## PodCast: 和 错误处理工作组的负责人 Jane 聊聊 Rust 错误处理

Rustacean Station 播客之家 是一个社区项目，提供 PodCast。（音频联系听力也不错）。

- [https://rustacean-station.org/episode/047-jane-lusby/](https://rustacean-station.org/episode/047-jane-lusby/)
- [https://github.com/rustacean-station/](https://github.com/rustacean-station/)

## 实验性的 CXX 应用案例 : TouchDesigner  的 Rust 绑定

>  **TouchDesigner** 可谓是一款真正意义上定义在新媒体交互领域的可视化节点式创作工具。一个重要特色有众多的**API接口**（一些预先设定好的函数，简单理解原本是不同厂商生产的硬件或软件可以通过API来配合TouchDesigner使用*）使它可以和市面几乎所有的软硬件实现便捷的信息交换) 。TouchDeisgner 可以实现控制机械、声音、灯光，还有实时视觉的特效渲染都有十分错的表现。

 目前还是实验性的库，但是作为学习案例还是不错的。

[https://github.com/tychedelia/td-rs](https://github.com/tychedelia/td-rs)

## 【系列】Rust 中的循环引用结构

Rust 中一共有两类自引用问题： 结构体内部的自引用 和 结构体外的自引用 。 这个系列的文章讨论的是后者，即循环引用。

- [https://arunanshub.hashnode.dev/self-referential-structs-in-rust](https://arunanshub.hashnode.dev/self-referential-structs-in-rust) 
- [https://arunanshub.hashnode.dev/self-referential-structs-in-rust-part-2](https://arunanshub.hashnode.dev/self-referential-structs-in-rust-part-2)
