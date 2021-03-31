# 社区热点

- 来源：[Rust日报](https://rustcc.cn/section?id=f4703117-7e6b-4caf-aa22-a3ad3db6898f)
- 作者：`Rust`日报小组
- 后期编辑：张汉东

---

## 华为 | openEuler 社区在 3 月 29 日正式成立了 Rust SIG


近日，openEuler 社区正式成立了 Rust SIG 组织。在维护 openEuler 操作系统内 Rust 工具链相关软件包的同时，也致力于将上游 Rust 社区优秀开源项目引入到 openEuler 操作系统中。openEuler 社区未来会持续和国内 Rust 社区和相关厂商通力合作，推动 Rust 语言在国内的发展，吸引更多的开发者关注和使用 Rust 语言。

欢迎订阅 rust@openeuler.org 邮件列表，参与到社区讨论中。

订阅方法：

https://openeuler.org/zh/community/mailing-list/ 在这个页面找到 Rust SIG，然后进去有 SubScribe 按钮，输入你到邮箱订阅。然后会收到一封邮件，你回复这封邮件即可。

## Niko | 我们的 AWS Rust 团队将如何为 Rust 未来的成功做出贡献

自今年年初以来，AWS Rust 团队一直在起草我们的章程和宗旨。
章程和宗旨是 AWS 团队用来定义我们的范围和优先事项的框架。
章程告诉你的团队该做什么，宗旨告诉你的团队将如何做到这一点。
由于我们的团队宗旨一直是公开和透明运作的，我们想与您分享我们的章程和宗旨，我们希望您知道我们在做什么。

起草我们的章程很容易。
这只是一句话：AWS Rust 团队致力于让 Rust 为其所有用户提供高效、可靠的服务。
说得够多了！
然而，撰写这些宗旨需要更多的工作。

等等，AWS 有个 Rust 小组？

是的! 事实上，至少从 2017 年开始，AWS 就在多项服务中使用 Rust。
例如，用 Rust 编写的 Firecracker 于 2018 年推出，提供支持 AWS Lambda 和其他无服务器产品的开源虚拟化技术。
最近，AWS 发布了用 Rust 编写的基于 Linux 的容器操作系统 Bottlerocket ，Amazon Elastic Compute Cloud(Amazon EC2) 团队使用 Rust 作为新的 AWS Nitro 系统组件(包括 Nitro Enclaves 等敏感应用程序)的首选语言。
随着在 AWS 中采用 Rust 的增长，我们对 Rust 项目和社区的投资也在增加。
2019年，AWS 宣布赞助 Rust 项目。
2020年，AWS 开始打造 Rust 维护者和贡献者团队，2021年，AWS 联合其他 Rust 用户和 Rust 项目发起了 Rust 基金会。
AWS Rust 团队首先找出了如何最好地与 AWS 和更广泛的开源社区建立联系。
我们知道，我们希望在公开的环境下运作，并成为整个社会的一份子。
与此同时，我们知道我们想要充分利用在 AWS 工作的机会。
起草章程和宗旨是我们找到两者兼顾的方法和过程的一部分。


[点此阅读该文的中文翻译](./how-our-aws-rust-team-will-contribute-to-rusts-future-successes.md)


### Rust for Linux 相关

**linux-next 分支现在已被合并。**

Linus Torvalds 讨论 Rust 适合Linux 的场景

关键内容：

1.  Coreutils  的 Rust 版本已经被 Mozilla 主管 Sylvestre Ledru 移植到了 Linux 。有了这些，Ledru启动了Linux并运行了最受欢迎的Debian软件包。
2. Linux Rust的支持者并不是： “提议将Linux内核重写为Rust“ 。 他们只是专注于向可能编写新代码的世界迈进。 
3. Rust支持的三个潜在方面是：利用内核中的现有API，体系结构支持，以及处理Rust和C之间的应用程序二进制接口（ABI）兼容性。
4. Linus 目前到态度是： 等待和观察。 他对 Rust for Linux 是感兴趣的，就个人而言，他绝不会排挤 Rust，但Linus 认为目前这个事情是那些对 Rust 抱有强烈兴趣的人推动的（Linus比较冷静），他想看看最终 Rust 在实践中如何发挥作用。
5. linux 认为 Rust 可能的场景：Rust的主要首要目标似乎是驱动程序，仅是因为在那里可以找到许多不同的可能目标，并且内核的这些各个部分相当小且独立。这可能不是一个非常有趣的目标。对某些人来说，但这是显而易见的。
6. Kroah-Hartman 的观点：“一切都归结为如何将用C编写的内核核心结构和生存期规则之间的交互映射到Rust结构和生存期规则中”
7. 尽管几乎可以肯定不会很快看到Linux从C 迁移到Rust，但是接下来的几年估计会非常有趣： 引入基于 Rust 的用户空间程序/ 驱动程序/  基于 Rust 的 内核迁移到 Linux 等。

相关链接合集，排序规则：最上面的是最新的

Linux Kernel's Preliminary Rust Code Seeing 64-bit POWER Support
[https://www.phoronix.com/scan.php?page=news_item&px=Linux-Kernel-Rust-PPC64LE](https://www.phoronix.com/scan.php?page=news_item&px=Linux-Kernel-Rust-PPC64LE)

[https://www.phoronix.com/scan.php?page=news_item&px=Rust-Hits-Linux-Next](https://www.phoronix.com/scan.php?page=news_item&px=Rust-Hits-Linux-Next)

[https://www.zdnet.com/article/linus-torvalds-on-where-rust-will-fit-into-linux/](https://www.zdnet.com/article/linus-torvalds-on-where-rust-will-fit-into-linux/)

[https://lore.kernel.org/rust-for-linux/CANiq72nbNxpps+p4wYp03ncrbGH9FFoTfHQZwg_vGdPO41eGmQ@mail.gmail.com/t/](https://lore.kernel.org/rust-for-linux/CANiq72nbNxpps+p4wYp03ncrbGH9FFoTfHQZwg_vGdPO41eGmQ@mail.gmail.com/t/)

[https://git.kernel.org/pub/scm/linux/kernel/git/next/linux-next.git/commit/rust?id=c77c8025525c36c9d2b9d82e4539403701276a1d](https://git.kernel.org/pub/scm/linux/kernel/git/next/linux-next.git/commit/rust?id=c77c8025525c36c9d2b9d82e4539403701276a1d)


## Linux 基金会 和 RISCV 基金会 共同推出的 免费 RISCV 课程 

课程发布在 edx.org 上，包括两个课程： 

-  Introduction to RISC-V (LFD110x)
- Building a RISC-V CPU Core (LFD111x) 

[https://www.zdnet.com/article/linux-foundation-risc-v-international-launch-free-risc-v-training-classes/](https://www.zdnet.com/article/linux-foundation-risc-v-international-launch-free-risc-v-training-classes/)


## Rust and LLVM in 2021

作者是 Rust 的核心团队成员, 之前就职于 Mozilla, 现就职于 Facebook. 写过最初的基于 LLVM 的 Rust 代码生成器, 以及很多 Rust 相关的工作.

该 keynote 讲述的是 Rust 中 LLVM 相关工作:

新的特性. 将LLVM 的提升带到 Rust 中.
LLVM 相关的提升和修复.
未来的挑战.
对于 Rust 编译器层面感兴趣的小伙伴可以深入了解.

[keynote地址](https://www.icloud.com/keynote/09ZXbPfbCKm8vCtAnWdfi2xIg#rust-llvm-cgo-2021)

## Rust版coreutils现在可以用来跑Debian啦

现在可以用Rust版的Coreutils (cp, chmod, ls, rm, tail, install..) 来运行Debian啦。

- [GitHub项目地址](https://github.com/uutils/coreutils/)
- [阅读更多](https://sylvestre.ledru.info/blog/2021/03/09/debian-running-on-rust-coreutils)

## `curl` 工具一半的漏洞都是关于 C 语言的错误

作者对这一问题进行了分析，并提到一个观点，如果用 Rust 来写 curl 的话，这些漏洞会减少一半。

[链接](https://daniel.haxx.se/blog/2021/03/09/half-of-curls-vulnerabilities-are-c-mistakes/)

## Rust 和 C 速度比较

Rust 和 C 的编程风格差异很大，但两者开发的程序在运行速度和内存使用情况上大致相同。语言在理论上可以实现什么，但在实践中如何使用它们之间有很大的区别。作者总结了Rust 和 C 各自在哪些地方会更快。

简而言之

- Rust 可以在必要时以足够底层的方式对其进行优化，使其达到与 C 一样的性能；
- Rust 拥有更高层次的抽象，便捷的内存管理和丰富的第三方库；
- Rust 最大的潜力在于无畏并发（fearless concurrency）能力。

[文章链接](https://kornel.ski/rust-c-speed)

## GitHub Action 将 Rust warning 转为 review comments

Rust Action 可以在出发执行后，将 Rust check 的 warning 转为 code review 的 comments。

[ReadMore](https://twitter.com/greyblake/status/1370117541436219393?s=20)

## INTELLIJ RUST CHANGELOG #143

为类似函数的程序宏提供初步支持。现在，插件可以扩展这种程序性宏调用；因此，它们自动获得声明性宏已经具备的一些功能：高亮显示、名称解析、有限的代码完成、意图等。

![gif](https://intellij-rust.github.io/assets/posts/changelog-143/function-like-proc-macro-support.gif)

## Veloren 0.9

一款开源多人RPG游戏，今天发布了!会在3月20日格林威治时间18:00发布在公共服务器上!

《Veloren》是一款多人体素RPG游戏。它的灵感来自《魔方世界》、《塞尔达传说:荒野之息》、《矮人要塞》和《我的世界》等游戏。

Veloren是完全开源的，使用GPL 3授权。它使用原始图形，音乐和其他资产社区创建的资产。它的开发社区和用户社区都是受贡献者驱动的:开发者、玩家、艺术家和音乐家一起开发游戏。

[文章链接](https://veloren.net/)

## Actix Actor Framework v0.11 出來了

`~40%` 的效能改善，升级到 Tokio v1 

[Actix 0.11 Change Log](https://github.com/actix/actix/blob/master/actix/CHANGES.md)


## 知乎| 搜索引擎研发（Rust) 工程师

岗位职责

- 负责搜索引擎平台架构建设，优化系统稳定性，设计良好的架构支持业务快速迭代
- 抽象通用的搜索引擎部署方案，用于快速支持各大垂直搜索引擎
- 参与知乎搜索业务优化

任职要求：

- 有扎实的编程能力，有良好的数据结构和算法基础
- 良好的团队合作精神，较强的沟通能力
- 熟悉 Linux 开发环境，熟悉 Go/Rust 语言，熟悉网络编程、多线程编程
- 熟悉搜索引擎，对 Elasticsearch、Kubernetes 有使用经验者优先
- 有高可靠分布式系统架构设计经验者优先

知乎搜索Rust 开源项目： https://github.com/zhihu/rucene

联系邮箱:

[zl 【at】 zhihu 点 com](zhihu.com)


## 蚂蚁集团校招开启：Rust 实习生看过来 

@2021.11.1～2022.10.31毕业的应届生可看 ，要推荐的可以找我咨询 ，也可直接联系。

招聘部门：

- 蚂蚁智能监控团队JD（内有联系方式）： [https://mp.weixin.qq.com/s/mi5woh-btWEEsc8ruSww7Q](https://mp.weixin.qq.com/s/mi5woh-btWEEsc8ruSww7Q)

- 蚂蚁机密计算部门: 直接联系方式：微信32713933， email shoumeng.ysm@antgroup.com

部门相关信息看下面链接：

[https://mp.weixin.qq.com/s/9t6_RrgSujrosDVphlzebg](https://mp.weixin.qq.com/s/9t6_RrgSujrosDVphlzebg)



## 3.27 号 深圳 Rust Meetup 视频和资料

活动PPT和现场视频链接：  

- [https://disk.solarfs.io/sd/6e7b909b-133c-49f7-be0f-a51f65559665](https://disk.solarfs.io/sd/6e7b909b-133c-49f7-be0f-a51f65559665)

B 站：

- [https://b23.tv/aKsBq3](https://b23.tv/aKsBq3)
- [https://b23.tv/UR07IW](https://b23.tv/UR07IW)
