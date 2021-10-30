# 社区热点

编辑： 张汉东

---

## Rust for Linux 项目相关进展

Google 赞助 且 由 ISRG 组织雇佣 Miguel Ojeda，让他全职从事Rust for Linux和其他安全工作，为期一年。希望能通过让他全职从事这项工作，为支持数字基础设施尽一份力。 

[https://www.memorysafety.org/blog/supporting-miguel-ojeda-rust-in-linux/](https://www.memorysafety.org/blog/supporting-miguel-ojeda-rust-in-linux/)

## RustConf 2021 CFP 开放申请

RustConf 2021 大会计划于 9月14号举行， 提案开放时间为太平洋时间6月13日至7月11日晚上11:59分。

[https://cfp.rustconf.com/events/rustconf-2021](https://cfp.rustconf.com/events/rustconf-2021)

## 国内新的  crates.io 和 rustup 的国内镜像

字节跳动 的小伙伴搞的，希望能帮助建设国内的 rust 生态，感兴趣的话可以试用。

不限速（实际上是 1000Gbps），优质 CDN 分发，欢迎大家使用~ 

有问题可以直接在评论区反馈。

[https://rsproxy.cn/](https://rsproxy.cn/)

##  2021 Rust China Conf 的调研

朋友们，请抽出一分钟完成一下 2021 Rust China Conf 的调研，只有3个问题 。

[https://wj.qq.com/s2/8683119/cbf3/](https://wj.qq.com/s2/8683119/cbf3/)

## 【现场实录】20210619-北京-Rust区块链开发者活动

[https://www.bilibili.com/video/BV1Jh411h7pp](https://www.bilibili.com/video/BV1Jh411h7pp)

## RustSBI组织宣布成立

RustSBI是RISC-V平台下的引导程序实现，它完全由Rust编写，并已经被录入RISC-V SBI国际标准。6月3日，RustSBI已经在GitHub上成立了组织，并提交了它对多个RISC-V平台的支持示例软件包。

RISC-V处理器架构中，存在着定义于操作系统之下的环境，它被称作SBI标准。这个环境除了引导系统启动，还将常驻后台，在内核运行时提供处理器功能。RustSBI就是这样的一种环境，它是一个扩展性较好的库，允许开发者自由地添加需要的功能模块，以支持芯片研发企业、应用厂商和板卡厂商提供自己芯片的SBI支持环境。

根据它的GitHub主页介绍，RustSBI项目组已经根据Mulan-PSL v2协议开源了它对K210和QEMU平台的支持，更多平台如FU540、C906等的支持也在筹划中。RustSBI组织以模块化开发、较好的支持嵌入式Rust生态，期望以这一形式，使厂商无需担忧是否合并到主分支，并鼓励开发者尝试多种多样的设计，来丰富引导程序方面的RISC-V生态环境。

“未来的RISC-V引导程序可能包含很多种可能性，这包括已经用在无盘工作站、安全网络等引导程序中的下载和验证机制，也可用于DIY爱好者的主板诊断。甚至用它做调试器也是可以的，它事实上提供了完全可定制的硬件兼容层，”RustSBI的维护者洛佳这样告诉《Rust日报》，“实践证明，它可以完成硬件到硬件的兼容性，从而延长硬件的生命周期，降低管理和迭代更新成本。”

“RustSBI已经能引导启动rCore等类Unix系统，”维护者继续说，“它是开源开放的，我们期待它被更进一步的被科研、教学和生产界广泛使用。”

RustSBI组织首页：[https://github.com/rustsbi](https://github.com/rustsbi)

##  惊叹！ Rust 在艺术领域的商业应用！

[观看视频](https://t.me/rust_daily_news/4863)

该艺术项目使用486步进电机，86,000个LED和5通道颗粒式合成引擎，控制软件系统使用 Rust 实现，利用的是 nannou 和 koto。

该项目背后是一家德国公司：[mindbuffer.net](mindbuffer.net)

- [https://nannou.cc/](https://nannou.cc/)
- [https://github.com/koto-lang/koto](https://github.com/koto-lang/koto)

## 2021年开源操作系统夏令营 欢迎报名

**任何** 对Rust 和 RISC-V写操作系统**感兴趣的均可报名**

鹏城实验室和清华大学组织的2020年开源操作系统夏令营收到同学们的积极响应，参与夏令营的同学在今年的“2021全国大学生计算机系统能力大赛 - 操作系统赛”的初赛中都有不错的表现( [https://mp.weixin.qq.com/s/Cb2SaonAAHDVNDli_80Bpw](https://mp.weixin.qq.com/s/Cb2SaonAAHDVNDli_80Bpw) )，今年启元实验室、清华大学和CSDN等将继续组织“2021年开源操作系统夏令营”，希望能把对开源操作系统开发有兴趣的朋友团结在一起，形成一个活跃的开源社区。欢迎对开源操作系统有兴趣的任何人报名（需填写下面的报名问卷）参加。

**2021年开源操作系统夏令营**

- 第一阶段： [https://github.com/rcore-os/rCore/wiki/os-tutorial-summer-of-code-2021](https://github.com/rcore-os/rCore/wiki/os-tutorial-summer-of-code-2021)
- 第二阶段： [https://github.com/rcore-os/rCore/wiki/zcore-summer-of-code-2021](https://github.com/rcore-os/rCore/wiki/zcore-summer-of-code-2021)
- 报名问卷： [http://oscourse2019.mikecrm.com/vzZqxgM](http://oscourse2019.mikecrm.com/vzZqxgM)

## GDC 2021 峰会 ： Treyarch 公司致力于 游戏工具库的 Rust 应用

GDC (Game Developers Conference 2021) 峰会将于 7月19～23日举行。 Treyarch 公司（《使命召唤系列》游戏公司）将在该峰会发表 Rust 相关议题。

> Treyarch是美国的一家电子游戏开发商，总部位于加利福尼亚州的圣莫尼卡。 Treyarch成立于1996年，2001年被美国动视收购。其代表作为《使命召唤系列》。

Rust编程语言已经悄悄地在科技界掀起风暴，但在游戏工作室中的采用却比较缓慢。自2018年以来，Treyarch一直在逐步将Rust整合到我们的工具和管道中。本次会议将利用这一经验，探讨Rust可以给游戏工具程序员带来的机遇和挑战，并研究Rust可以成为游戏工具库的有力补充的方式。

Treyarch 在 GDC  大会的议题：

-  The Rust Programming Language for Game Tooling
- Boots on the Ground: The Terrain of 'Call of Duty'
- Shadows of Cold War: A Scalable Approach to Shadowing

[https://schedule.gdconf.com/session/tools-summit-the-rust-programming-language-for-game-tooling/880599](https://schedule.gdconf.com/session/tools-summit-the-rust-programming-language-for-game-tooling/880599)

## 在curl中使用hyper如何帮助使互联网更安全

> 作者：Sean McArthur（hyper作者）

**大概翻译了一下重点摘要：**

今年2月，互联网安全研究小组的Josh Aas、curl的Daniel Stenberg和我（来自hyper和Amazon Web Services）联合举办了一场网络研讨会，讨论内存安全和互联网，以及在curl中使用hyper如何帮助使互联网更安全。由于curl是开源和许可的，从物联网设备到卫星，以及大多数Linux发行版中都可以找到它。Curl是那些我们认为理所当然的基础库之一，但它影响着我们的网络生活。（视频回顾：https://www.youtube.com/watch?v=okGUxW_i9yk）

**不太安全的互联网**

内存安全是编程语言或系统的一个属性，它可以保护程序不会错误地访问它不应该访问的内存，例如认为缓冲区比它大，或者从一个指针解释数据，而这个指针后来被清理并用于不同的值。缺乏内存安全是对互联网基础设施的一个严重的持续威胁，并对个人和组织都造成了重大的、有意义的损害。比如说。

- 微软估计，在过去十年中，他们的产品中70%的漏洞都是由缺乏内存安全造成的。
- 谷歌发现，Chrome浏览器70%的严重安全漏洞是内存安全问题。
- Android团队报告说，他们90%的漏洞是内存安全问题。
- Mozilla指出，Firefox在其风格组件中的74%的安全漏洞是内存安全漏洞。
- 最近的一项研究表明，60-70%的iOS和macOS漏洞都与内存安全有关。
- Project Zero的一项分析发现，被利用的0-day中，有超过80%是由于缺乏内存安全。

这些漏洞可能导致现实生活中的隐私被侵犯、财务损失、公共服务被剥夺，以及人权受到影响。

造成这种漏洞泛滥的一个重要原因是，许多工具是用编程语言编写的，而这些语言对内存安全漏洞的保护作用不大，甚至没有。尽管 "内存安全 "语言已经存在了很长时间，但由于性能或互操作性要求，它们经常被忽视。

通常说到内存安全就要提到 Rust 。Rust是一种较新的语言，它执行内存安全，但具有与C类似的性能和互操作性。然而，这篇博文并不是呼吁 "用Rust重写（所有）"。

重写所有的东西是不现实的，每一段代码对人类的影响也是不一样的。相反，让我们简单地走过一个实际的努力，把内存安全带到互联网的关键部分。这种努力鼓励项目用内存安全库来取代库或模块化功能，而不是着手进行基础重写。它将工作分解成可管理的部分，并逐步提供价值。


**curl无处不在**

curl是开始这项工作的理想人选。潜在的影响是巨大的，因为curl无处不在。根据curl网站的说法。

curl在命令行或脚本中被用来传输数据。curl也被用于汽车、电视机、路由器、打印机、音频设备、手机、平板电脑、机顶盒、媒体播放器中，并且是超过100亿套软件应用程序的互联网传输引擎。

curl也是用C语言编写的。

在最近的一篇博文中，Daniel Stenberg指出，curl有一半的漏洞是C语言的错误--换句话说，与内存安全有关。这些错误包括缓冲区超读、缓冲区溢出、使用后释放和双重释放。尽管curl处理了一大堆协议，但HTTP是内存安全漏洞的第二大领域。

我们能让curl更安全吗？

这样做会使互联网更安全，所以值得努力。而这正是我们要做的。Curl的API和ABI是稳定的 "装甲门"，不会损坏。但是curl已经熟悉了为其内部实现细节选择不同的 "后端"。在此之前，curl可以被配置为内部支持TLS、DNS、压缩和其他组件的后端。我们只需要提供一个更安全的HTTP后端选项。

**"此时 Hyper 加入了群聊"**

Hyper是一个安全、正确、快速的Rust语言的HTTP库。Hyper是开源的，并被AWS、Buoyant、Discord、谷歌、微软、Mozilla等公司使用。它有客户端和服务器端的API，并提供对HTTP/1和HTTP/2的支持。

Rust库不会带来对新运行时的依赖，在C语言中调用Rust函数也没有开销，反之亦然。它只是需要工程师的工作来暴露一个与C语言兼容的API。

Hyper开发人员立即知道这是需要做的事情。考虑到curl的使用量，这是一个让互联网更安全的机会。而hyper的Rust用户也会受益，因为这项工作所处理的任何bug修复或边缘案例也会为他们修复。

我们为hyper设计了一个C语言API。大部分的工作是围绕着识别Rust和C语言之间的假设差异。在这些部分被解决后，API就开始工作了。Hyper添加了几个选项，让curl对他们的用户来说几乎没有区别。

**目前状态：**

curl和hyper的代码都已经合并到了各自的主开发分支。Curl 可以被配置为以 hyper 作为其 HTTP 后台进行编译，尽管它将处于实验状态，直到 curl 的所有 HTTP 支持与它的内部 C 后台一样工作，并且被更广泛地试用。大部分的代码已经工作了，剩下的问题是更复杂的HTTP功能，仍然需要更新以支持不同的后端。跟踪工作的最好方法是通过curl的广泛测试套件。

从数量上看，在800个左右的HTTP单元测试中，有95%的测试被移植并通过了配置为后端的hyper。这意味着 curl 的很多标准功能已经在工作了。使用hyper后端，curl支持HTTP/1和HTTP/2。它可以使用任何 TLS 后台的 HTTPS，甚至可以使用 HTTP(S) 代理。无论使用哪种后端，电线上的HTTP请求都是相同的。

**下一步是什么？**

还需要在curl中加入一些功能，以使整个测试套件通过。此外，我们还希望改善对恐慌和内存不足的处理，以及其他关于从Rust中暴露C语言库的细节。此外，开发人员还需要调整或修复curl在完成单元测试时注意到的任何问题。

[https://aws.amazon.com/blogs/opensource/how-using-hyper-in-curl-can-help-make-the-internet-safer/](https://aws.amazon.com/blogs/opensource/how-using-hyper-in-curl-can-help-make-the-internet-safer/)

## TensorBase | RISC-V 芯片上执行 SQL

TensorBase 是第一个在真正的 RISC-V 芯片上运行的 SQL 数据库

[SQL on RISC-V Chip in Rust (tensorbase.io)](https://tensorbase.io/2021/06/08/sql_on_riscv_in_rust.html)

## AWS 添加了 9 项关于rust sdk的新服务

[https://github.com/awslabs/aws-sdk-rust/releases/tag/v0.0.7-alpha](https://github.com/awslabs/aws-sdk-rust/releases/tag/v0.0.7-alpha)

## Reddit 讨论：Rust语言在项目管理上的优势

Rust在技术/项目管理上有什么优势吗？足以支撑你说服自己和其他人在公司内部开始尝试使用Rust？ 

作者提到了两点：

1. 安全
    - 理由："70%的安全问题都和内存安全有关"
    - 证据：
        - [Microsoft: 70 percent of all security bugs are memory safety issues](https://www.zdnet.com/article/microsoft-70-percent-of-all-security-bugs-are-memory-safety-issues/#:~:text=Microsoft%3A%2070%20percent%20of%20all%20security%20bugs%20are%20memory%20safety%20issues,-Percentage%20of%20memory&text=Around%2070%20percent%20of%20all,week%20at%20a%20security%20conference)
        - [Google: chromium memory-safety problem](https://www.chromium.org/Home/chromium-security/memory-safety)
2. 维护
    - 理由：软件设计、研发到生产过程中的维护复杂度会逐步上升
    - 证据：[Relative cost to fix, based on time of detection](https://www.whitesourcesoftware.com/wp-content/media/2021/04/graph2.jpg)

或者你有其他任何观点和证据来说服别人使用rust吗？

[https://www.reddit.com/r/rust/comments/o0a61h/rusts_advantages_in_13_slides_on_management_level/%5D](https://www.reddit.com/r/rust/comments/o0a61h/rusts_advantages_in_13_slides_on_management_level/%5D)

## 在 Facebook Rust 是如何被用来构建 Linux 系统工具、库和服务的

这篇是一篇采访纪录，@NavyataBawa邀请到了 Facebook 工程师 Daniel Xu，分享他在 Facebook 是如何使用 Rust 来构建 Linux 系统工具、库和服务的。

[https://developers.facebook.com/blog/post/2021/06/24/meet-rustaceans-daniel-xu/](https://developers.facebook.com/blog/post/2021/06/24/meet-rustaceans-daniel-xu/)