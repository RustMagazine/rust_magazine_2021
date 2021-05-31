# Rust 发布六周年

作者：The Rust Team / 译者：杨楚天

---

今天是 [Rust] 自 2015 年发布 1.0 以来的诞生日六周年。六年以来发生了许多事情，去年尤其如是，Rust 也不例外。2020 年伊始，我们还没有 Rust 基金会，没有常量泛型。许多组织依然对在生产环境中使用 Rust 保持观望态度。

在新冠病毒爆发时期，分散在世界各地的上百名 Rust 的组织成员和志愿者为我们带来了 9 个 Rust 稳定版本，以及若干个 bug 修复版本。如今，我们不会再质疑 Rust 是否适用于生产环境。新成立的 Rust 基金会包含许多成员，不仅重视在实际产品中使用 Rust，也在持续地为 Rust 开源生态做出支持与贡献。

现在，我们来总结一下去年发生的各种重大进展，回顾社区是如何在生产环境中使用 Rust 的，并展望 Rust 社区当下的工作，会如何在未来一年继续提升这门语言的使用体验。

[rust]: https://www.rust-lang.org

## 近期工作

Rust 语言在去年取得了非常大的进展，带来了许多新的特性。这些功能谈不上给语言带来了质变，却也让我们能在更多领域方便地使用和维护 Rust。

- Rust 1.52.0 升级到了 LLVM 12，许多和 forward progress 有关的安全问题（例如无限循环的处理）得到了解决。这是 Rust 团队和 LLVM 长期以来合作的结果，也是 Rust 可以为更广阔的编程语言生态带来进步的例证。

- Rust 为更多生态系统提供了支持，包括 Tier 1 中对 64 位 ARM Linux 的支持，Tier 2 中对 ARM macOS 和 ARM Windows 的支持等。这使得我们可以在更多的架构中使用 Rust 构建自己的项目。

- Rust 在编译期的表达能力也获得了极大的提升。基础类型作为常量泛型的特性进入了稳定版本，同时 `const fn` 中也能更宽松地使用控制流，过程宏也能被用于更多的情景当中，这些都让我们能构造出更强力的 API，写出更好的 crate。

除了 rustc 以外，以下工具也有巨大的改进：

- Cargo 稳定化了最新的特性分解机制，让我们可以在不同的 target 中更方便地管理依赖。

- Rustdoc 稳定化了 "文档内部链接" 的功能，使得文档里的文本能自动地链接对应的类型或函数。

- Clippy 现在使用单独的缓存，不再与 Cargo 共享，使其行为更加稳定。


## 生产环境中的 Rust 

Rust 每年都能以难以置信的速度获得社区与业界的接纳，去年也不例外。2020 年 Rust 再一次成为了 StackOverflow 年度[最受喜爱的编程语言][stackoverflow]。感谢社区里所有人的支持，是你们才有 Rust 的今天。

随着[基金会][rust foundation]的成立，Rust 社区得以在一个更合适的位置上，构建出一个可持续的开源生态系统，帮助我们构建高效可靠的软件。许多公司成立了专门维护和促进 Rust 项目的小组，包括 [AWS](https://aws.amazon.com/blogs/opensource/how-our-aws-rust-team-will-contribute-to-rusts-future-successes/)、[Facebook](https://engineering.fb.com/2021/04/29/developer-tools/rust/) 和微软等。

不仅 Rust 自身正在进步，许多大公司也开始允许在自己的产品中使用 Rust，或提供 Rust API 的官方支持。

- 微软和亚马逊分别发布了 [Windows] 和 [AWS] 的官方 Rust 库。官方的海量 API 支持让 Rust 用户更能在项目中开发出想要的功能。

- 在 cURL 项目发布的新版本中，默认采用了由 Rust 实现的库，用于处理 [HTTP/s] 和 [TLS] 通信。ISRG、Hyper & Rustls 团队以及 cURL 项目团队之间密切地跨社区合作，为 cURL 这个被广泛使用的工具打造出了一个内存安全的后端。

- Tokio （一个 rust 的异步运行时）发布了 [1.0 版本][tokio-1.0] ，并承诺了三年的稳定维护。这个项目可以为我们实现可靠且高效的网络应用提供了坚实的基础。

[stackoverflow]: https://stackoverflow.blog/2020/06/05/why-the-developers-who-use-rust-love-it-so-much/
[tokio-1.0]: https://tokio.rs/blog/2020-12-tokio-1-0
[http/s]: https://daniel.haxx.se/blog/2020/10/09/rust-in-curl-with-hyper/
[tls]: https://daniel.haxx.se/blog/2021/02/09/curl-supports-rustls/
[rust foundation]: https://foundation.rust-lang.org/posts/2021-02-08-hello-world/
[windows]:https://github.com/microsoft/windows-rs
[aws]: https://github.com/awslabs/aws-sdk-rust

## 未来工作

当然，这一切仅仅是开始，我们可以看到现在很多组织在开展各种开创性的工作，尝试将 Rust 带入全新的领域。

- Critical Section 公司旗下的 Ferrous Systems 启动了 [Ferrocene] 项目，让 Rust 得以用于安全系统和关键业务系统中。
- Embark Studios 发布了一个 [`rust-gpu`] 的原型，借助于这个编译器后端，Rust 也可以用于实现 GPU 中的图形着色器。
- Linux 社区正在围绕是否[将 Rust 作为内核开发的另一选择][linux-rust]做出讨论，Rust 可以帮助 Linux 社区写出更安全的驱动以及内核代码。
- Google 宣布[已在 Android OS 中支持采用 Rust 开发底层组件][android-rust]，并正在着手使用 Rust 重写其中的蓝牙模块。

Right now the Rust teams are planning and coordinating the 2021 edition of Rust. Much like this past year, a lot of themes of the changes are around improving quality of life. You can check out our recent post about ["The Plan for the Rust 2021 Edition"][edition-plan] to see what the changes the teams are planning.

目前 Rust 团队正在围绕新版本 Rust 2021 进行相关的计划与协调工作。和去年一样，我们很多的改动还是聚焦在改善开发体验上。大家可以看一下我们最近的[一篇文章][edition-plan]，讲述 Rust 团队当前的工作计划。

这些只是冰山一角，当今的 Rust 社区，每时每刻在发生新的变化，每天都有激动人心的项目诞生。我们迫不及待地想看到新的一年，大家又用 Rust 打造出了什么样的创作。

---

过去一年 Rust 的那项改进最让你感到振奋？哪个项目最让你激动？是刚准备尝试使用 Rust 么？还是想参与 2021 版本的工作？欢迎大家来到我们的 [Discourse] 论坛和 [Zulip] 聊天室，随意介绍自己，然后加入我们的讨论吧。这里有安全友好的环境，无论你的性别取向身体状况宗教信仰如何，我们都欢迎你的到来。

[ferrocene]: https://ferrous-systems.com/ferrocene
[`rust-gpu`]: https://github.com/EmbarkStudios/rust-gpu
[linux-rust]: https://lore.kernel.org/lkml/CANiq72khBa2GcB6-PHM3A44Y90d6vzYAS=BVpk3nT4B6u+NVDw@mail.gmail.com/T/#mb5e524dae9d5a5815c6e68eb36b9bde4e87c861d
[edition-plan]: https://blog.rust-lang.org/2021/05/11/edition-2021.html
[discourse]: https://users.rust-lang.org/
[zulip]: https://rust-lang.zulipchat.com/
[android-rust]: https://security.googleblog.com/2021/04/rust-in-android-platform.html
