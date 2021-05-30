---
pub_date: Sun， 30 May 2021 10:00:00 GMT
description: hots

---

# 社区热点

编辑：李冬杰

---

## Rust 核心团队进展

早在去年8月，核心团队就写了一篇题为《为 Rust的未来奠定基础》的博客文章，从那时起，核心团队一直在做大量的工作来帮助打下基础，并为项目上因这些事件而发生的变化做好准备。这种工作主要集中在内部，而不是真正从外部可见的东西，即使您在 Rust 团队中也是如此。 由于这些努力，Rust 基金会现在已经存在，并开始致力于其使命，这也意味着核心团队是时候转移工作重点了。

除了基金会的努力之外，Rust核心团队还开展了一些您可能知道也可能不知道的举措。

**[The 2021 Roadmap](https://github.com/rust-lang/rfcs/pull/3037)**

2021年 Roadmap RFC 于1月合并，这为今年的核心团队设定了目标，与往年有些不同，Rust为整个项目制定了目标，2021年决定将重点放在 Rust Core 上，并为团队设定自己的目标留出空间，而我们则专注于总体组织健康。

今年晚些时候，Rust也将启动明年的流程，目前还没有积极考虑这个问题，但理想情况下年度 Roadmap 将在12月合并，而不是在1月，因此团队希望能尽早开始，以便在 2022 年按时完成目标。

**[Team Charters](https://github.com/rust-lang/rfcs/blob/master/text/1068-rust-governance.md)**

作为该工作的一部分，Rust项目已经开始了为每个团队提供正式章程的流程的第一步。 早在 RFC 1068 中，初始团队的范围就已经列出。 虽然多年来这对Rust很有帮助，但随着团队的组建、关闭和变化，Rust项目在明确每个团队的责任界限方面并不总是做得很好。 Rust 治理结构的神奇之处在于，每个团队都被赋予了重要的权力，可以按照他们认为合适的方式做事，但这也意味着Rust项目必须意识到范围。 随着该过程的继续展开将有更多关于此过程的报告，但最终目标已在路线图中说明：

> Rust 团队将与核心团队合作，在一年中为每个 Rust 团队制定章程，目的是定义，特别是目的和成员要求。我们的目标是，到 2022 年，Rust 项目中的所有活跃团体都将拥有明确定义的章程和成员资格。

**Audit of packages owned by the project**

Rust核心团队一直在努力澄清Rust团队在crates.io拥有的包的状态，目前核心团队正在对这些程序包进行全面审核，以确保它们是项目应该拥有的东西，确保它们有适当的权限，并确保有人维护它们。

[Read More](https://blog.rust-lang.org/inside-rust/2021/05/04/core-team-update.html)

## 在安卓上运行 Rust

作者决定对目前一个客户的产品使用Rust，这个决定背后有两个原因，除了技术优点之外，还有一个无可争辩的事实，即 Rust 仍然是一种相对较新的语言，花哨和时髦。当你是一家初创公司时，使用前十年出现的任何技术只会让自己失败，如果不使用创新技术，公司如何进行创新呢？最快的成功方式是搭乘炒作列车。因此作者开始研究如何让Rust在安卓上运行起来，并提供了一个最小运行程序的模板。

[Read More](https://blog.svgames.pl/article/running-rust-on-android)
[MVP template Repo](https://github.com/suve/rust-on-android/)

## Rust for Windows v0.9

Rust for Windows v0.9最近已发布，其中包括全面的消费支持以及其他几个更新！有了完整的使用支持，您现在可以使用Rust语言来调用任何Windows API（过去，现在和将来）。 Rust开发人员可以以一种惯用的语言访问整个Windows API接口，从而使他们可以轻松利用 Windows 开发的强大功能和广度。

[Read More](https://blogs.windows.com/windowsdeveloper/2021/05/06/announcing-rust-for-windows-v0-9/)

## 一个 Rust 的新 AWS SDK：alpha 发布

我们很兴奋地宣布 Rust 的新 AWS SDK 的 alpha 版本发布。在 Rust 中开发的 AWS 客户想要一个本地的 Rust SDK，这样他们就可以使用他们习惯的语言结构，而 Rust 的新客户想要一个与他们在其他语言环境中使用的 SDK 行为类似的 SDK。在这个 alpha 版本中，客户可以在客户端试用7种AWS服务，并提供可用性方面的反馈。

[Read More](https://aws.amazon.com/cn/blogs/developer/a-new-aws-sdk-for-rust-alpha-launch/)

## Rust 六周年 🎉

2021年5月15日是 Rust 六岁生日（从2015年 1.0 版本算起），在这过去的六年里发生了许多变化，但 Rust 项目没有什么不同，依然没有基金会，没有 Const 泛型，许多组织仍然怀疑 Rust 是否已经准备好投入生产环境。

这篇文章将会回顾一下过去一年中的一些重大改进，社区如何在生产中使用Rust，最后展望目前正在进行的一些改进工作，这些改进和改进了Rust在小型和小型企业中的使用。明年的大型项目，让我们开始用Rust吧！

[Read More](https://blog.rust-lang.org/2021/05/15/six-years-of-rust.html)

## SpaceX 开始使用 Rust 了

考虑到 Rust 的安全性、高性能、现代化的工具集，SpaceX 可以在嵌入式系统、模拟器、工具集、web开发使用统一的语言，SpaceX 已经在开发一些原型工具，这仅仅是这个长途旅行的开始！

![space use Rust](./image/spacex-use-rust.jpeg)

[Read More](https://www.reddit.com/r/rust/comments/ndm4ne/spacex_about_the_rust_programming_language/)

## 【Rust 安全案例】Rust 也能写出漏洞，但都是逻辑漏洞

2021年05月18日，openSUSE 邮件列表里收到一份安全报告，主题如下：

[oss-security] please: CVE-2021-31153，CVE-2021-31154，CVE-2021-31155: local root exploit and further

please 是一个 Rust 实现的替代 sudo 的工具，该库作者向 SUSE 团队提出 setuid-root 的代码安全审核，然后就被发现存在很多安全问题，并且包含一个比较严重的本地 root 漏洞，允许执行命令。

报告摘录如下：

- 可以进行任意文件存在测试，并且可以以root用户身份打开任意文件
- 通过search_path（）函数进行任意文件存在性测试
- 通过-d开关进行任意文件存在性测试
- 使用不可信的umask创建令牌目录“ / var / run / pleaser / token”
- 允许通过pleaseedit编辑任何文件，允许任意文件覆盖和所有权更改

结论：

- 哪怕是 Rust 这样的现代语言，要实现 setuid-root 二进制文件也是一个很大的挑战。
- please库中几乎没有unsafe（只有几行 libc/nix调用），所以基本已经不存在内存安全类的漏洞了，但是像这种逻辑漏洞是无法避免的，除非你不写代码。
- setuid 二进制文件是在非 root 用户 shell 程序中运行的 root 权限二进制文件

[Read More](https://marc.info/?l=oss-security&m=162133298513412&w=2)
[Read More](https://bugzilla.suse.com/show_bug.cgi?id=1183669)

## Facebook 将进一步深化 Rust 使用和支持

Rust 在 Facebook 一共经历过 3 个阶段：

- 2016-2017：早期源码控制中的使用，创建了 eden 项目 以增加项目代码的最大提交率。
- 2017-2019：接受采纳阶段，不少 Python 和 JS 的后端程序员由于性能和编译时错误检测开始使用 Rust。
- 2019-2020：专门支持阶段，2019 年 Rust 的开发人数呈指数增长，一个显著的例子是 Rust 作为 Diem 区块链（FaceBook 的电子钱包 Novi 是 Diem 组织的一员）的主语言。于是一个小的 Rust 开发小组被创建，专门致力于工具和集成方面的挑战。

未来，短期内主要会聚焦以下四个领域：

- 从语言和工具链的角度支持内部用户。
- 在 FaceBook 以外的社区中做出积极贡献。
- Rust 与 C++ 简单安全的互操作性。
- 积极支持和参与 Rust 基金会。

[Read More](https://engineering.fb.com/2021/04/29/developer-tools/rust/?utm_campaign=Learning%20Posts&utm_content=166528802&utm_medium=social&utm_source=twitter&hss_channel=tw-1359556530618646530)
