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



## Rust 优秀博客推荐 第一期 | https://fasterthanli.me/

Rust 社区现在有很多优秀的博客，我感觉做这么一个栏目挺好，给大家推荐一些优秀的Rust 学习博客，也作为一个学习资源的备份。

第一期的主角是 fasterthanli.me ，作者是 Amos 。

> 弹钢琴、玩乐高长大的90 后游戏 Boy ，之前用 Java，现在用 Rust。关于他更详细的介绍可以参考 : [https://fasterthanli.me/about](https://fasterthanli.me/about)

为什么第一期就推荐他呢？ 因为他的文章写的好。他的博客文章写的系统且细腻入微，引导你思考。

如果你学习 Rust ，那么强烈推荐 Amos 的博客：[ https://fasterthanli.me/]( https://fasterthanli.me/)

如果你觉得他的文章对你有很大帮助，可以适当支持一下他的写作：[https://www.patreon.com/fasterthanlime](https://www.patreon.com/fasterthanlime)

## Python中调用 Rust

PyO3 使从 Python 调用Rust 代码变得很容易。您可以编写一个Rust库，并依赖PyO3和 maturin (PyO3生态系统的一个支持工具)的组合来编译Rust库，并将其直接作为Python模块安装。除此之外，PyO3可以在Python和Rust之间转换类型，还可以通过一组宏方便地将Rust函数导出到Python。

[http://saidvandeklundert.net/learn/2021-11-18-calling-rust-from-python-using-pyo3/](http://saidvandeklundert.net/learn/2021-11-18-calling-rust-from-python-using-pyo3/) 

# Life simulation

Life simulation, 一个 Rust 编写的 模拟器.

生物可以通过突变和自然选择进化。生物有一个简单的基因组，赋予它们独特的特征.

[demo地址](https://joelthelion.github.io/life_web/demo/)

[github地址](https://github.com/joelthelion/life_web)

##  How to write idiomatic Rust

想写出更多 idiomatic Rust 吗？[@matthiasendler](https://twitter.com/matthiasendler) 维护了一个经过同行评审的项目，他包括文章、演讲、repos，它们都使用了 Rust。

- Repo [https://github.com/mre/idiomatic-rust](https://github.com/mre/idiomatic-rust)



##  Redox OS（以及类似的项目）计划如何处理缺乏动态链接支持的问题？

> 几天前，我问是否有人对C FFI之外的二进制库发行有什么建议，人们不断地给我指出各种 "hack "解决方案，对此我非常感激（尽管这些hack的方法在我的特定情况下没有帮助，C FFI看起来仍然是最不坏的解决方案）。
>
> 然而，有一件事让我很吃惊，那就是动态链接（或者说是可预测的ABI，我认为动态链接需要ABI）的想法（也许只是被认为）受到了反对。还有一个事实是，几乎没有人关心在我看来是系统语言设计中的一个巨大漏洞。
>
> TL;DR：
>
> 这让我想到：Redox OS或类似的项目（比如今天早些时候[u/dptzippy发布的项目](https://www.reddit.com/r/rust/comments/qr2kyb/libertyos_another_opensource_operating_system/)）打算如何解决这个问题？他们肯定不会期望人们在某个核心库/缓存中的错误修复后重新编译整个操作系统，或者他们会怎样做？还是我错过了什么？
>
> *我说的是 "可预测 "而不是 "稳定"，因为我们不需要一个完全稳定的ABI。即使只是保证如果我用同一版本的编译器编译东西，如果公共API保持不变，公共ABI也会有很大的帮助。

今日reddit上的Rust板块的热帖，贴一下高赞回复：

> 你好，我是[`Theseus`](https://github.com/theseus-os/Theseus)操作系统的作者。很好的帖子，真的和我心底的想法一致。
>
> 缺乏一个稳定的ABI是我们尚未完全解决的一个大障碍。这个问题对于像 `Theseus` 这样基于安全语言的 SAS/SPL 操作系统来说特别重要，考虑到我们的目标是让编译器对系统各层的所有代码都能可见（`visibility`）/自我反省（`instrospection`），没有盲点，就更重要了。我个人认为，对于使用系统调用作为用户应用程序和内核之间的 "稳定 "接口的其他操作系统来说，这不是一个问题，或者在更广泛的意义上，作为两个独立编译的代码片断之间的 "稳定 "连接层，它们在执行时相互依赖。 我记录了我在实现针对现有 `Theseus` 编译实例为了编译`out-of-tree`这个`crate`（以及一个各种启动器 libc 骨架）的工具时所经历的历程和思考过程。这涉及到对`cargo`的hack式滥用，我没必要推荐其他用例 -- 在我们有限的事例下，它是有效的，因为我们可以控制一切的编译 -- 但相当难以处理。它很可能无法扩展到支持闭源应用程序或库的分发。显然，我意识到，在运行时动态地重新链接单独编译的二进制文件中的符号/依赖关系是非常愚蠢和乏味的，这些符号/依赖关系在构建时是静态链接的，与实际运行的系统实例中存在的这些依赖关系的版本相一致。请注意，我们还没有时间去探索更好的选择。我发这个帖子只是为了说明人们为了解决稳定的ABI问题而可能需要做的事情。
>
> 最好的 "替代方案 "基本上是其他人在你上面链接的u/dptzippy的帖子中所建议的—使用某种`bindgen`来创建一个薄的FFI层，将稳定的C ABI函数作为Rust函数的存根来暴露。然而，这有一个主要的缺点，那就是到处引入不必要的`unsafe`因素，这有点违反了我上面提到的`Theseus`的整个目标，因为`unsafe`因素是一个盲点。
>
> 不管怎么说，我只是觉得应该把我的想法说出来，在这里分享我的经验，让对话进行下去，看看其他人有什么意见。

[Read More](https://www.reddit.com/r/rust/comments/qr5pyo/how_is_redox_os_and_similar_projects_planning_to/): [https://www.reddit.com/r/rust/comments/qr5pyo/how_is_redox_os_and_similar_projects_planning_to/](https://www.reddit.com/r/rust/comments/qr5pyo/how_is_redox_os_and_similar_projects_planning_to/)

## Rust 中使用 opencv

作者做了一个关于如何在 Rust 中使用 opencv 的视频。

[油管视频](https://www.youtube.com/watch?v=zcfixnuJFXg)

##  kotlin 可以从 Rust 身上学到哪些？

虽然是站在 kotlin 立场上来看的，比较研究也比较有价值。

[https://www.beust.com/weblog/2021/11/09/what-kotlin-could-learn-from-rust/](https://www.beust.com/weblog/2021/11/09/what-kotlin-could-learn-from-rust/)

### 好文推荐：如何改进限制过多的 Rust 库 API

文章非常深入地讨论基于 Rust 中的泛型、dyn trait, slice 等设施的更有宽容度的上层设计。属于进阶文章，值得学习。

https://blog.logrocket.com/improving-overconstrained-rust-library-apis/

## 使用纯 Rust 开发 ios 应用

属于验证型的项目，做 ios 开发的童鞋可以研究研究。

[https://github.com/wooden-worm/ios-app-rs](https://github.com/wooden-worm/ios-app-rs)

##  使用 Prometheus 和 Grafana 监控 Rust Web 应用程序

主要就是一些配置和对应的代码嵌入，写得非常详细。推荐。

[https://romankudryashov.com/blog/2021/11/monitoring-rust-web-application/](https://romankudryashov.com/blog/2021/11/monitoring-rust-web-application/)

##  Neon - 使用 Rust 创建内存和类型安全的 Node.js 模块

Neon除了内存和类型安全之外，应该在 Node.js 中使用 Rust Embedding 的原因还有很多。

- 并行编程和线程
- 性能更强
- 访问操作系统特定的库
- 通过 Cargo 访问 Rust 的生态系统

[https://levelup.gitconnected.com/create-memory-and-type-safe-node-js-modules-with-rust-2c10bba92013](https://levelup.gitconnected.com/create-memory-and-type-safe-node-js-modules-with-rust-2c10bba92013)

## Stack-safety for free?

一篇有意思的文章，演示了如何使用 Rust 的生成器将任何递归函数转换为迭代函数，而几乎无需更改代码。

[Stack-safety for free?](https://hurryabit.github.io/blog/stack-safety-for-free/)

## IDE 和 宏

我们日常中享用着各种 IDE 的便利性，但是很少人会去实现相关的功能。

在本文中，我们将讨论语言服务器在支持宏时所面临的挑战。这很有趣，因为对于Rust 的 analyzer 来说，宏是最难破解的。

[原文链接](https://rust-analyzer.github.io/blog/2021/11/21/ides-and-macros.html)

## Inx 如何做到使用 SymSpell 使 模糊搜索提速超过5倍

本文介绍了 SymSpell这个难以置信的算法, 以及大致了解我们如何在lnx中实现它。

[https://chillfish8.ghost.io/fuzzy-searching-5x-faster-with-symspell/](https://chillfish8.ghost.io/fuzzy-searching-5x-faster-with-symspell/)

## 三个Rust代码库的故事

现在是使用Rust的好时机了吗？

Convex的创始团队（从DropBox分离出来的）有使用Rust开发Magic Pocket（Dropbox的地理分布式数据存储系统），Nucleus（重写的Dropbox的同步引擎），Convex（0配置，无限扩容的为响应式应用开发需求设计的后端）。它们是目前世界上负载最大的基于Rust语言的系统之一。

Convex的创始团队分享了使用Rust的好处和一些取舍心得体会。十分推荐。

[https://blog.convex.dev/a-tale-of-three-codebases/](https://blog.convex.dev/a-tale-of-three-codebases/)

## 使用 Rust 和机器学习的无人机摄影测量尝试

文章作者用无人机拍了一张防水布的照片。使用基本的摄影测量法估计了图片的面积，使用机器学习对图像中的防水布进行了分割，得到的防水布面积为 3.86 m2，而实际面积为 3.96 m2（误差约为 4%）整个预估算法由 Rust 实现。

- [http://cmoran.xyz/writing/adventures_in_photogrammetry](http://cmoran.xyz/writing/adventures_in_photogrammetry)

##  使用 Bors 合并队列

合并队列是一个与您的版本控制系统（本文将重点关注git和 GitHub）集成的应用程序，它要求以原子方式合并代码更改，从而确保主分支始终包含经过全面测试的代码版本。 许多工程团队和开源项目正在引入合并队列作为其工作流程的一部分。这篇文章探讨了使用合并队列的几个原因，并描述了如何设置 Bors，Rust 语言项目使用的合并队列实现。

- [https://kflansburg.com/posts/merge-queues/](https://kflansburg.com/posts/merge-queues/)
