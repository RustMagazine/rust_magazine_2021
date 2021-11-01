# 学习资源

聚焦学习 Rust 的网络资源

---

## toml-edit 优化之旅

`toml_edit` 是一个保留格式（在修改用户的Cargo.toml时，能保留格式）的TOML crate，允许用户修改.toml文件。

优化之前

```
toml_edit 8.7us 271us
toml_edit::easy 20.7us 634us
```

优化之后

```
toml_edit 4.0us 149us
toml_edit::easy 5.0us 179us
```

上下文：

该作者是 cargo-edit 的核心贡献者，现在正致力于将 cargo-add 合并到 cargo 的工作中，其中对 toml 的修改要用到 toml_edit 这个库，他们已经把相关的工作都做完了，只剩下最后都性能优化。这篇文章就是 作者对 toml_edit 性能优化的记录。

性能之旅：

1. 要确定你的优化目标。 作者之前并不确定可以从 toml_edit 中可以挤出多少性能，但是Alex Crichton帮助他确定了这个目标，他特别指出Cargo的解析器，是一个影响用户的瓶颈，这个结果是在`toml-rs` 分析时展示出来的。因此，toml_edit 至少应该和 `toml_rs`有同样的速度，他们还想进一步优化`toml_rs`。
2.  依靠性能之书（https://nnethercote.github.io/perf-book/profiling.html）来帮助开始profile，选择了callgrind和kcachegrind作为可视化工具。
3. 使用 [kstring](https://github.com/cobalt-org/kstring)，来优化字符串key。 

默认情况下，`kstring` ：

- 内联字符串存储总共 16个字节
-  max_inline 特性选择存储总共 23 个字节，对于小字符串来说会很慢
-  以`Box<str>`的形式存储堆字符串
-  使用`Arc<str>`，它用Ref-counting的成本代替了分配的成本

4. 要注意第三方解析便利性背后的成本

解析器组合器，如`nom`或`combined`，使语法转换为代码变得容易，但也很容易隐藏大量的成本：

- 不必要的分配
- 缺少批处理

他优化解析器的 [https://github.com/ordian/toml_edit/pull/209 ](https://github.com/ordian/toml_edit/pull/209 )

5.  将字符和字符串的操作换成按字节操作，并且根据情况在某些安全的情况下，使用 uncheck 的方法来避免 utf-8 校验。 pr 在这里 [https://github.com/ordian/toml_edit/pull/219/](https://github.com/ordian/toml_edit/pull/219/)

6. 良好错误处理背后的代价。

toml_edit 之前的解析器用的是 combine，它的特点是错误处理非常细致，将检查每个选择并合并错误。它优先考虑在字符串中最早出现的错误，并合并发生在相同点的错误。这样做的问题是，即使没有面向用户的错误发生，错误处理的逻辑也会让你付出代价。

作者要优化他还有很多选择，比如放弃 combine，使用nom或者手写parse  (性能优化效果将最大)，但是他选择继续使用 combine，但是用 dispatch! （可以迅速实施改变）来代替map。这样的选择算是小步迈进。PR: [https://github.com/ordian/toml_edit/pull/222 ](https://github.com/ordian/toml_edit/pull/222 )

7. serde的隐藏成本

serde对toml文件中每个字符串进行解析，看它是否可能是一个日期。没有Datetime的文件必须为它的存在付出代价。所以作者将使用一个专门的结构体来优化这种情况。pr：[https://github.com/ordian/toml_edit/pull/226 ](https://github.com/ordian/toml_edit/pull/226 )

默认情况下，serde检查每个未标记的枚举的变体，看它是否有效。作者使用 手动实现 serde::Deserialize  trait 来优化这种情况，而避免 derive 自动实现。pr ：[https://github.com/ordian/toml_edit/pull/227](https://github.com/ordian/toml_edit/pull/227)

有哪些优化是失败的呢？

不是所有的修复都是成功的。不幸的是，失败的尝试通常不会被很好地记录下来，因为它们可能不会被记录到PR中。

这里作者凭记忆罗列了一些失败的尝试：

1. 批处理优化，收益比较小
2. 想进一步优化KString，反而变慢了
3. 尝试将 combine 迁移到 nom，目前正在努力

最后，发现了这句话： 感谢 Futurewei 对这项工作的赞助 

[https://epage.github.io/blog/2021/09/optimizing-toml-edit/](https://epage.github.io/blog/2021/09/optimizing-toml-edit/)

## 在 Rust 项目中编写 dockerfile 的建议

当你写一个Rust项目时，也许你想建立一个小型的运行容器（基于alpine和distroless/cc-debian），然后你可以在 k8s 或其他你喜欢的地方运行它。

文章中介绍了步骤以及最终的三个示例：

1. [https://windsoilder.github.io/writing_dockerfile_in_rust_project.html#6-final-dockerfile](https://windsoilder.github.io/writing_dockerfile_in_rust_project.html#6-final-dockerfile)
2. [https://windsoilder.github.io/writing_dockerfile_in_rust_project.html#7-additional-dockerfile](https://windsoilder.github.io/writing_dockerfile_in_rust_project.html#7-additional-dockerfile)
3. [https://windsoilder.github.io/writing_dockerfile_in_rust_project.html#not-vendor-based-dockerfile-based-on-cargo-chef](https://windsoilder.github.io/writing_dockerfile_in_rust_project.html#not-vendor-based-dockerfile-based-on-cargo-chef)


[https://windsoilder.github.io/writing_dockerfile_in_rust_project.html](https://windsoilder.github.io/writing_dockerfile_in_rust_project.html)

## 面向有多门语言编程经验的开发者的 Rust 入门书

[https://www.chiark.greenend.org.uk/~ianmdlvl/rust-polyglot/index.html](https://www.chiark.greenend.org.uk/~ianmdlvl/rust-polyglot/index.html)

## 使用Rust编写嵌入式固件

本文探讨了Rust在嵌入式上的能力，并为用它编写固件提供了一个起点。本文包含关于嵌入式编程和Rust的介绍性信息。本文描述为什么你应该考虑在新项目中使用Rust，提供常用库的概述，并提供涵盖最重要部分的代码示例。其中的一些主题与官方的Rust嵌入式和Rust嵌入式发现书籍有重叠。

这些都是很好的入门资源，并为刚接触Rust的有经验的嵌入式工程师和刚接触嵌入式的人分别进行了详细介绍。本文的重点是实用固件的架构，并介绍了在这些书之后发布的工具和库。

作者使用Rust编写了水监测器固件，并计划在未来的设备中也这样做，因为它的内存安全和人体工程学。

[https://www.anyleaf.org/blog/writing-embedded-firmware-using-rust](https://www.anyleaf.org/blog/writing-embedded-firmware-using-rust)

## 使用 Rust Cloudflare Workers 构建 ServerLess

文章带我们了解怎么用 Rust 编写 Cloudflare Workers ServerLess 代码。

然后将展示构建一个完整的 ServerLess 功能，并可以使用它来验证带有 hCaptcha 的前端 Web 用户。

[https://dev.to/askrodney/using-rust-cloudflare-workers-serverless-hcaptcha-358g](https://dev.to/askrodney/using-rust-cloudflare-workers-serverless-hcaptcha-358g)

## Rust游戏 -  "自走棋"

由 Rust ([bevy](https://github.com/topics/bevy)) 开发的自走棋游戏。

[https://github.com/yopox/LD49](https://github.com/yopox/LD49)

## 【系列】Rust 插件开发 | 深入理解动态加载

[https://nullderef.com/blog/plugin-dynload/](https://nullderef.com/blog/plugin-dynload/)

## Rust中的 Phantom 类型 

Phantom 类型是在运行时中从不使用的类型，但有助于在编译时强制执行某些约束。对其感兴趣的同学可以通过本文一探究竟.

[https://www.greyblake.com/blog/2021-10-11-phantom-types-in-rust/](https://www.greyblake.com/blog/2021-10-11-phantom-types-in-rust/)

## 使用 Rust 进行 PIC32 单片机编程

> 作者: 这篇文章是给新手 开始使用PIC32 和 Rust进行编程的。

所以本文的步骤非常的详尽, 感兴趣的小伙伴可以跟着动手试试.

[https://gill.net.in/posts/pic32-blink-led-rust/](https://gill.net.in/posts/pic32-blink-led-rust/)

## 使用 Rust 编写高性能的 javascript API

> WasmEdge 集Rust的性能和JavaScript的易用性于一身

将Rust函数合并到JavaScript api中，使得开发人员可以用“纯JavaScript”编写程序，同时还可以利用Rust的高性能功能。使用WasmEdge Runtime，您可以做到这一点。

[https://www.secondstate.io/articles/embed-rust-in-javascript/](https://www.secondstate.io/articles/embed-rust-in-javascript/)

## Java 开发者分享的 Rust 学习笔记

初学 Rust 的 Java 开发者 的学习笔记  

[https://github.com/sumeetdas/succinct-rust](https://github.com/sumeetdas/succinct-rust)

## 【油管视频】 Rust 学习视频两则

- [Rust中的类型驱动API设计 视频讲解](https://www.youtube.com/watch?v=bnnacleqg6k)
- [Rust数据类型的内存布局可视化]([https://youtube.com/watch?v=rDoqT-a6UFg](https://youtube.com/watch?v=rDoqT-a6UFg&feature=share))

## 在java和rust中返回Optional的开销对比

一些编程语言，如 Java 或 Scala，提供了不止一种方式来表达“lack of value”的概念。传统上，一个特殊的null值用于表示根本不引用任何值的引用。 然而，随着时间的推移，我们了解到使用null 可能非常容易出错，并且会导致许多麻烦，例如 NullPointerException在最意想不到的时刻出现错误使程序崩溃。因此，现代编程风格建议null尽可能避免使用更好的Option,Optional或Maybe数据类型（在很多语言中叫法不同，但概念是一样的）。不幸的是，人们认为 Java 中的可选值可能会带来性能损失。在这篇博文中，我将尝试回答是否属实，如果性能惩罚真的存在，那么严重程度如何。

[https://pkolaczk.github.io/overhead-of-optional/](https://pkolaczk.github.io/overhead-of-optional/)

## 在Rust中使用建造者模式

由于Rust不支持函数的可选参数和具名参数，也不支持函数重载，建造者模式在Rust中被广泛使用。 以下是一个使用建造者模式创建一个`User`结构体的代码示例：

```Rust
#[derive(Debug)]
struct User {
    id: String,
    email: String,
    first_name: Option<String>,
    last_name: Option<String>
}

struct UserBuilder {
    id: String,
    email: String,
    first_name: Option<String>,
    last_name: Option<String>
}


impl UserBuilder {
    fn new(id: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            email: email.into(),
            first_name: None,
            last_name: None,
        }
    }

    fn first_name(mut self, first_name: impl Into<String>) -> Self {
        self.first_name = Some(first_name.into());
        self
    }

    fn last_name(mut self, last_name: impl Into<String>) -> Self {
        self.last_name = Some(last_name.into());
        self
    }

    fn build(self) -> User {
        let Self { id, email, first_name, last_name } = self;
        User { id, email, first_name, last_name }
    }
}

impl User {
    fn builder(id: impl Into<String>, email: impl Into<String>) -> UserBuilder {
        UserBuilder::new(id, email)
    }
}

fn main() {
    let greyblake = User::builder("13","greyblake@example.com")
        .first_name("Sergey")
        .build();
    dbg!(greyblake);
}
```

要点：

1. 使用`UserBuilder`来创建`User`结构体；
2. 结构体的必要字段通过必选位置显示传入建造者的`new`方法，可选参数通过`setter`来设置；
3. 最后通过`build`函数返回完整的结构体实例；

[Builder pattern in Rust](https://www.greyblake.com/blog/2021-10-19-builder-pattern-in-rust/)

## Rust中的容器运行时 - 第二部分

克隆容器进程和父子通信。

本系列的[第一部分](https://penumbra23.medium.com/container-runtime-in-rust-part-i-7bd9a434c50a)描述了文件系统布局以及运行时如何将容器进程囚禁在容器的根文件系统中。

第二部分更深入地探讨了实现，并展示了运行时如何创建子进程以及它们如何通信，直到用户定义的进程启动。它还将描述如何设置伪终端并展示其重要性Unix 套接字。

到本部分结束时，我们应该有一个可与 Docker 互操作的基本运行时。

[https://itnext.io/container-runtime-in-rust-part-ii-9c88e99d8cbc](https://itnext.io/container-runtime-in-rust-part-ii-9c88e99d8cbc)

## 如何实现一个跨平台的Rust库

[oso](https://www.osohq.com/what-is-oso) 项目团队新写的文章。`oso` 支持六种编程语言，是基于 Rust 编写的 核心库来完成多语言支持的。

这篇文章就是介绍了 `oso` 如何做到通过一个 Rust 核心库支持多语言。

[https://www.osohq.com/post/cross-platform-rust-libraries](https://www.osohq.com/post/cross-platform-rust-libraries)

## 一个很酷的 Rust 优化故事

在 Quickwit，我们正在为大数据构建最具成本效益的搜索引擎。我们的[整个搜索引擎](https://github.com/quickwit-inc/quickwit)是用 rust 开发的，搜索的核心是由一个名为[tantivy](https://github.com/quickwit-inc/tantivy)的库提供的。

人们经常问为什么[tantivy](https://github.com/quickwit-inc/tantivy) 在[基准测试中的](https://tantivy-search.github.io/bench/)表现优于[Lucene](https://lucene.apache.org/)，这是一个复杂的问题。许多人认为这是 Rust 比 Java 更快的故事之一，真相要复杂得多。

[https://quickwit.io/blog/search-a-sorted-block/](https://quickwit.io/blog/search-a-sorted-block/)

## `rust-motd`  无运行时命令行可配置界面美观的 MOTD 生成工具

在[类Unix系统](https://zh.wikipedia.org/wiki/类Unix系统)中，`/etc/motd`是一个包含“今日消息”（**message of the day**）的文件。

[https://github.com/rust-motd/rust-motd](https://github.com/rust-motd/rust-motd)

## block-ciphers：用纯 Rust 编写的分组密码算法集合

支持多种算法。

[https://github.com/RustCrypto/block-ciphers](https://github.com/RustCrypto/block-ciphers)

## darkfi：匿名的 DeFi 网络

DarkFi 是一个匿名的 DeFi 网络。它的目标是提供灵活的私有原语，可以用来创建任何类型的应用程序。 DarkFi 使用零知识密码学的进步，并创建了一种合同语言和开发人员工具包，旨在使匿名工程对开发人员具有高度的可访问性。

[https://github.com/darkrenaissance/darkfi](https://github.com/darkrenaissance/darkfi)

## tunneler：隧道工具

Rust 实现的，通过 TCP、（相互）TLS 或 DNS（权威服务器或直接连接）隧道传输 TCP 或 UDP 流量。

每个可执行文件包含 2 个组件，通过客户端 stream 通道（字节读取器和写入器元组）进行通信：

- 客户端监听器绑定套接字并将传入和传出的流量转为新的流。
- 客户端 tunneler 将流读取器和写入器转为隧道协议。
- 服务器 untunneler 根据隧道协议绑定套接字并将隧道流量转换回原始流。
- 服务器转发器将流写入器和读取器转换回流量。

基于 TCP 的流量被简单地转换为流。 基于 UDP 的流量转换取决于隧道协议。基于 UDP 的流量还需要一种方法来识别现有客户端以继续其会话。解决方案是内存中的客户端缓存，它将客户端的标识符映射到其对应的流。

[https://github.com/dlemel8/tunneler](https://github.com/dlemel8/tunneler)

## rust-kernel-barebones：Rust 内核和配置脚本

一个最小的 64 位 Rust 内核和一堆配置脚本，可用于使用 Nightly-Rust 编译器引导操作系统开发。使用 Rust OsDev 社区构建的工具如 xbuild、bootimage、bootloader crates，并将所有这些工具配置为协同工作。从而开发人员不必担心 toochain 的配置。工具链构建并配置项目以使用 `x86_64-unknown-none` 目标。 一些功能包括：

- 配置整个环境的脚本。
- 构建内核并使用 Qemu 模拟的脚本。
- VS Code RLS 配置。

[https://github.com/Narasimha1997/rust-kernel-barebones](https://github.com/Narasimha1997/rust-kernel-barebones)

## 使用 Rust 实现健康检查 API 模式

通过添加一个`/health`API 端点来满足[**kubelet**](https://kubernetes.io/docs/reference/command-line-tools-reference/kubelet/)探测以进行[**liveness**](https://kubernetes.io/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/#define-startup-probes)和[**readiness**](https://kubernetes.io/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/#define-readiness-probes)检查，该端点根据您的服务的当前状态以`Ok`或`ServiceUnavailable`HTTP 状态进行响应。

此`/health`API 端点解决方案是[**Health Check API 模式的实现**](https://microservices.io/patterns/observability/health-check-api.html)，该模式用于检查 API 服务的健康状况。在像[**Spring**](https://spring.io/)这样的 web 框架中，像[**Spring Actuator**](https://docs.spring.io/spring-boot/docs/current/reference/html/actuator.html)这样的[**嵌入式**](https://docs.spring.io/spring-boot/docs/current/reference/html/actuator.html)解决方案可供您集成到您的 Spring 项目中。但是，在许多 Web 框架中，您必须自己构建此 Health Check API 行为。

在这篇博文中，使用[**actix-web**](https://actix.rs/) Web 框架实现健康检查 API 模式，该框架使用[**sqlx**](https://github.com/launchbadge/sqlx)连接到本地 PostgreSQL 数据库实例。

[https://itnext.io/implementing-the-health-check-api-pattern-with-rust-eaef04cb4d2d](https://itnext.io/implementing-the-health-check-api-pattern-with-rust-eaef04cb4d2d)

## 使用 Rust 构建 Emacs lisp VM

这是编写 Emacs lisp vm 的介绍（非教程），相关项目: [rune](https://github.com/CeleritasCelery/rune)

[https://coredumped.dev/2021/10/21/building-an-emacs-lisp-vm-in-rust/](https://coredumped.dev/2021/10/21/building-an-emacs-lisp-vm-in-rust/)

## 【系列】使用 `tracing-subscriber` 和 `tracing` 自定义 日志

该系列文章有两篇，介绍了如何使用  [`tracing`](https://docs.rs/tracing/0.1)  和  [`tracing-subscriber`](https://docs.rs/tracing-subscriber/0.3)  来构建日志系统。

- [https://burgers.io/custom-logging-in-rust-using-tracing](https://burgers.io/custom-logging-in-rust-using-tracing)
- [https://burgers.io/custom-logging-in-rust-using-tracing-part-2](https://burgers.io/custom-logging-in-rust-using-tracing-part-2)

##  Iced.rs 教程：如何构建一个简单的 Rust 前端 Web 应用程序

本文使用 Iced 和 Rust 构建一个非常基本的前端应用程序，它使用[JSONPlaceholder](https://jsonplaceholder.typicode.com/)来获取数据。获取帖子并将它们显示在一个列表中，每个帖子都有一个详细链接，引导用户阅读带有评论的完整帖子。

[Iced.rs](https://iced.rs/)和[Yew](https://yew.rs/)最大的区别在于，虽然 Yew 纯粹是用于构建 Web 应用程序，但 Iced 的重点实际上是跨平台应用程序；Web 只是您可以为其构建应用程序的多个平台之一。

另一件需要注意的事情是 Iced.rs 处于早期和积极的开发阶段。虽然使用它构建基本应用程序是绝对可能的，但生态系统还不是特别成熟。除了[文档](https://docs.rs/iced/latest/iced/)和[示例之外](https://github.com/iced-rs/iced/tree/master/examples)，在这个早期阶段，开始有点困难，特别是如果你试图构建一些复杂的东西。

[https://dev.to/logrocket/icedrs-tutorial-how-to-build-a-simple-rust-frontend-web-app-2pg7](https://dev.to/logrocket/icedrs-tutorial-how-to-build-a-simple-rust-frontend-web-app-2pg7)https://fasterthanli.me/articles/my-ideal-rust-workflow)

## 使用 Rust 进行嵌入式开发

本文基于开源的[RT-Thread 操作系统](https://github.com/RT-Thread/rt-thread)来简单演示如何使用 Rust 进行嵌入式开发。

> RT-Thread诞生于2006年，是一款以开源、中立、社区化发展起来的物联网操作系统。 RT-Thread主要采用 C 语言编写，浅显易懂，且具有方便移植的特性（可快速移植到多种主流 MCU 及模组芯片上）。RT-Thread把面向对象的设计方法应用到实时系统设计中，使得代码风格优雅、架构清晰、系统模块化并且可裁剪性非常好。

本文介绍了以下内容：

- 如何在 C 中调用 Rust
- 在 Rust 中调用 C

[https://opensource.com/article/21/10/rust-embedded-development](https://opensource.com/article/21/10/rust-embedded-development)

## 【系列】在 Raspberry Pi Pico 上开始使用 Rust

这是探索 Raspberry Pi Foundation 的第一个微控制器 (RP2040) + 开发板 (Pico) 的高效且令人兴奋的世界的系列文章中的第一篇，其中固件是用 Rust 编写的。

本指南涵盖使用两块 Raspberry Pi Pico 板，一块作为目标板，另一块作为硬件编程器和调试器。

[https://reltech.substack.com/p/getting-started-with-rust-on-a-raspberry](https://reltech.substack.com/p/getting-started-with-rust-on-a-raspberry)
