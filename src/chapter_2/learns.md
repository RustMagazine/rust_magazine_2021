# 本月简报 | 学习资源

- 来源：[Rust日报](https://rustcc.cn/section?id=f4703117-7e6b-4caf-aa22-a3ad3db6898f)
- 作者：`Rust`日报小组
- 后期编辑：苏胤榕（DaviRain）


## 使用 Rust 创建一个模拟器

这是 Learning to Fly: Let's create a simulation in Rust!.

在这一系列的文章中,作者会从头到尾带领大家使用 Rust 实现一个基本 feed-forward 的神经网络.

[链接](https://pwy.io/en/posts/learning-to-fly-pt2/)

## 使用Rust和WebAssembly创建爆炸性的Markdown编辑器

> 摘录： 让我们快速准备WebAssembly的开发环境

Rust通常cargo使用命令构建，但是WebAssembly有一个叫做wasm-pack的工具，它可以很方便地完成很多事情，所以让我们安装它。

[链接](https://zenn.dev/beijaflor/articles/da789ea779c005)

## Improving texture atlas allocation in WebRender

作者花费大量篇幅解读了如何改进WebRender中Texture atlas分配的问题。

[链接](https://nical.github.io/posts/etagere.html)

## 新书：《Black Hat Rust》

《Black Hat Rust》是一本基于Rust编程语言深入研究攻击性、安全性的书。最终出版预计2021年7月，书篇预估320页。如果你是一名安全的从业者，应该会对此书非常感兴趣。

[链接](https://academy.kerkour.com/black-hat-rust)

## Emacs 配置 Rust 开发环境

喜欢使用 Emacs 的小伙伴如果想开发 Rust, 可以参考这篇文章进行详细的设置.

[链接](https://robert.kra.hn/posts/2021-02-07_rust-with-emacs/)

## Rust 知识精炼

该文是作者将自己的所学的 Rust 知识整理到这里. 感兴趣的同学的可以看一下.

[链接](https://www.greyblake.com/blog/2021-02-07-rust-knowledge-refinement/)

## exercism[.]io：一个在线编程训练的平台

exercism[.]io 是一个在线编程训练平台支持Rust语言。

[链接](https://exercism.io)


## 【视频】1Password 开发者炉边谈话：介绍 Rust 宏

[视频链接](https://youtu.be/Lh262L63asA)

## 比较 Rust async 与 Linux 线程上下文切换时间

作者写了一些代码，试图比较 Linux 线程上下文切换所需时间和Rust async任务调度切换所需时间及其各自在使用时的内存使用总量，并且还做出了总结。

[Github](https://github.com/jimblandy/context-switch)

## 使用 Tokio 直接构建 Actors

本文使用Tokio直接构建 Actors, 而不是使用任何现有的 actor 库.

感兴趣的同学可以阅读一下.

[链接](https://ryhl.io/blog/actors-with-tokio/)

Rust 从零到生产: 可维护的测试套件的骨架和原则

## 这是 <<Rust 从零到生产>> 系列的第七章 part 1.

该章节主要侧重于测试. 整个书基本上都是使用 test-driven的方式来编写新的功能. 当代码变的庞大之后, 一个良好的测试框架可以更好的支撑更复杂的特性和日渐增多的测试用例.

[链接](https://www.lpalmieri.com/posts/skeleton-and-principles-for-a-maintainable-test-suite/)

## For the Love of Macros

宏是一种超越 more power的存在, 他赋予了我们超越 源代码 的抽象能力, 但是,同时,你也会放弃表层语法. 例如, 在一个拥有强大的宏的语言中, 重命名 基本上是不太可能 100% 工作的.

本文尽力探索Rust 中宏的使用方式, 目的是为了找到一种不放弃源代码推断的解决方案.

[链接](https://matklad.github.io/2021/02/14/for-the-love-of-macros.html)

## 使用Rust从零重写一个SQLite

作者计划使用Rust重新复制一个SQLite数据库，目前正在进行中。

SQLite有很完善的文档，代码质量非常高，而且有非常丰富的单元测试用例，终于有人尝试使用Rust重写一个SQLite了，感兴趣的朋友可以一起参与！

[Github](https://github.com/joaoh82/rust_sqlite)

[链接](https://medium.com/the-polyglot-programmer/what-would-sqlite-look-like-if-written-in-rust-part-1-4a84196c217d)

## 微软的员工发布的Windows用户Rust视频

主要介绍怎样在Windows平台使用windows-rs这个crate构建Rust程序。

[链接](https://kennykerr.ca/2021/02/18/rust-for-windows-getting-started/)

## 如何使用 webassembly 构建一个 telnet 聊天服务器

相信有大批的人喜欢 terminals这种审美, 作者也是其中之一.

作者使用 webassembly + Rust 构建了一个 telnet 聊天服务器. 你可以使用下面的命令来尝试一下.
```
# US
> telnet lunatic.chat
# EU
> telnet eu.lunatic.chat
```

[链接](https://lunatic.solutions/blog/lunatic-chat/)

## EasyRust 现在有视频了

EasyRust 是一个非常好的 Rust 入门教程,现在,他不仅有文档,还有视频了.

下面是第一期视频,未来至少还有 70 期. 想学习的小伙伴可以跟着视频了解一下.

[EasyRust地址](https://dhghomon.github.io/easy_rust/Chapter_0.html)

[油管视频](https://www.youtube.com/watch?v=-lYeJeQ11OI&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk)

## 经典 Rust 面试题六道

在电报群由 @wayslog 提出的六道面试题目，wayslog 老师称之为“经典六道”：

1. RwLock<T> 对想要在多线程下正确使用，T的约束是？
2. 如下代码：
    ```
    trait A{ fn foo(&self) -> Self; }
    Box<Vec<dyn A>>;
    ```
    是否可以通过编译？为什么？ 

3. Clone与 Copy 的区别是什么？ 
4. deref 的被调用过程？ 
5. Rust里如何实现在函数入口和出口自动打印一行日志？ 
6. Box<dyn (Fn() + Send +'static)> 是什么意思?

@wayslog 提供的答案：

1. The type parameter T represents the data that this lock protects. It is required that T satisfies Send to be shared across threads and Sync to allow concurrent access through readers.
2. 不可以，参考object safe 三条规则。
3. Copy是marker trait，告诉编译器需要move的时候copy。Clone表示拷贝语义，有函数体。不正确的实现Clone可能会导致Copy出BUG。
4. Deref 是一个trait，由于rust在调用的时候会自动加入正确数量的 * 表示解引用。则，即使你不加入*也能调用到Deref。
5. 调用处宏调用、声明时用宏声明包裹、proc_macro包裹函数、邪道一点用compiler plugin、llvm插桩等形式进行。（Go:我用snippet也行）
6. 一个可以被Send到其他线程里的没有参数和返回值的callable对象，即 Closure，同时是 ownershiped，带有static的生命周期，也就说明没有对上下文的引用。

读者们又会几道呢~

[讨论链接](https://rustcc.cn/article?id=0b0afa3e-db03-428e-9fc5-b06347997d41)

## Rust for web development

本篇blog作者是今年七月要出的rust新书Black Hat Rust的作者，在两年前作者就已经开始尝试用Rust去进行web开发，这篇blog谈的是他开发的一些感受，一些经验，同时提到了他开发中用到了哪些crate。

[链接](https://kerkour.com/blog/rust-for-web-development-2-years-later/)

## 笨方法学习Rust所有权机制

为了真正了解Rust，我们需要了解其关键的区别于其它语言的特性: 所有权。本篇blog用了笨方法的方式来讲解Rust的所有权。

[链接](https://chrismorgan.info/blog/rust-ownership-the-hard-way/)


## 好文推荐：《Rust和LoRa》

Drogue IoT 是一个试图将可重用和高效的组件引入嵌入式Rust的团队，本文讲述了“如何在Rust中开始使用LoRa“。

ps: LoRa是一种低功率远程无线协议

阅读原文: [https://blog.drogue.io/rust-and-lora/](https://blog.drogue.io/rust-and-lora/)

Repo: [https://github.com/drogue-iot/drogue-device](https://github.com/drogue-iot/drogue-device)

## Rust 循环优化

![1](https://rustcc-1252416178.cos.ap-nanjing.myqcloud.com/rust_loop_opt.jpeg)
![2](https://rustcc-1252416178.cos.ap-nanjing.myqcloud.com/rust_loop_opt_2.jpeg)
![3](https://rustcc-1252416178.cos.ap-nanjing.myqcloud.com/rust_loop_opt_2.jpeg)

## Cranelift 代码生成入门

Cranelift 是用 Rust 编程语言编写的代码生成器，旨在成为快速的代码生成器，其输出以合理速度运行的机器代码。如今，它被用于包括 Wasmtime 和 Wasmer 在内的几种不同的 WebAssembly 运行时中，并且还可以作为 Rust 调试编译的替代后端。

更多见博客原文：[https://blog.benj.me/2021/02/17/cranelift-codegen-primer/](https://blog.benj.me/2021/02/17/cranelift-codegen-primer/)

Cranelift 仓库地址：[https://github.com/bytecodealliance/wasmtime/tree/main/cranelift#cranelift-code-generator](https://github.com/bytecodealliance/wasmtime/tree/main/cranelift#cranelift-code-generator)

## Rtic book

[RTIC 框架](https://github.com/rtic-rs/cortex-m-rtic) 是中断驱动的异步实时系统，完全针对应用使用Rust的宏语法生成，拥有极高的效率。

RTIC Book ：[https://rtic.rs/0.5/book/en/by-example.html](https://rtic.rs/0.5/book/en/by-example.html)

## 国外 Rust 咨询公司 Ferrous System 的嵌入式课程资料

链接：[https://embedded-trainings.ferrous-systems.com/preparations.html](https://embedded-trainings.ferrous-systems.com/preparations.html)