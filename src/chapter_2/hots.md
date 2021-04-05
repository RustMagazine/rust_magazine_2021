---
pub_date: Sat, 27 Feb 2021 16:00:00 GMT
description: Hot community news in February

---

# 本月简报 |社区热点

- 来源：[Rust日报](https://rustcc.cn/section?id=f4703117-7e6b-4caf-aa22-a3ad3db6898f)
- 作者：`Rust`日报小组
- 后期编辑：张汉东

## CURL 支持 RUSTLS

Rustls 是一个用 Rust 写的现代 TLS（安全传输协议） 库。目前已经被纳入了为 CURL 的一个 backend

curl 对以下这些 features 都有一些可替换的 backends ：

- International Domain Names
- Name resolving
- TLS
- SSH
- HTTP/3
- HTTP content encoding
- HTTP

[https://daniel.haxx.se/blog/2021/02/09/curl-supports-rustls/](https://daniel.haxx.se/blog/2021/02/09/curl-supports-rustls/)


## Rust  全栈框架 MoonZoon 计划

- [仓库链接](https://github.com/MoonZoon/MoonZoon)

这是 Seed  作者新开的一个项目，目标是做一个纯 Rust 的全栈框架。

- NO Javascript
- NO CSS
- NO HTML
- NO REST
- NO GraphQL
- NO SQL
- NO Analysis Paralysis
- NO Wheel Reinventing
- NO Passwords*

目标比较大，目前是草案阶段，感兴趣的可以关注参与。

个人看法：Rust 其实并不需要全栈框架。对于上面的一堆 NO XXX ，个人理解应该是指这个框架不太限定用户去使用什么，想用啥可以用啥，给予最大自由。

## VSCode 修补了关于 Rust 工作流中的一个怪异的 bug

最新的VSCode版本中有一个 PR，以防止提示弹出窗口过度滚动。 以前你将鼠标悬停在符号上来阅读相应文档，如果继续向下滚动至底部，则滚动将继续并将从文档窗口弹出。 现在，此问题已得到解决。🎉

[https://www.reddit.com/r/rust/comments/lgccv5/ysk_vscodes_most_recent_update_fixed_a_quirk_in/](https://www.reddit.com/r/rust/comments/lgccv5/ysk_vscodes_most_recent_update_fixed_a_quirk_in/)

## Google资助项目以使用新的Rust组件保护Apache Web服务器的安全

根据ZDNet报道，由Google资助并由Internet Security Research Group领导的Apache Web服务器将设置为接收新的基于Rust的mod_ssl模块（以将Apache HTTP Web服务器项目的关键组件从容易出错的C编程语言移植到一种更安全的替代品Rust中），该模块将基于 Rustls ; 开发了Rust开源库，以 替代基于C的OpenSSL项目。

阅读原文: [https://www.zdnet.com/article/google-funds-project-to-secure-apache-web-server-project-with-new-rust-component/](https://www.zdnet.com/article/google-funds-project-to-secure-apache-web-server-project-with-new-rust-component/)

## rust-analyzer 内部体系结构文档更新！

rust-analyzer是一个用于IDE的实验性Rust编译器前端。

阅读原文: [https://github.com/rust-analyzer/rust-analyzer/blob/master/docs/dev/architecture.md](https://github.com/rust-analyzer/rust-analyzer/blob/master/docs/dev/architecture.md)

## 微软的Rust课程将在下月开课

据几天前的消息微软正在组建一支Rust团队。现在，微软 Reactor 将在3月份将举办两次Rust课程，以下是课程预告。

课前准备：

不需要具有 Rust 经验，但是如果您有使用其他编程语言的经验会更佳。

适合人群：

该研讨会面向想要学习 Rust 的开发人员。不需要具有 Rust 经验，不过如果您有使用其他编程语言的经验会帮助你更快的学习 Rust 语言。

参与本次分享，你将收获：

如果您想更熟悉更多的 Rust 相关知识，包括：变量，数据类型，函数，集合类型和控制流，则应该参加此研讨会。

主办方：

微软 Reactor 上海 是微软为构建开发者社区而提供的一个社区空间。

原文：[https://mp.weixin.qq.com/s/TS3R8MNF_t09HmYNHMMHTg](https://mp.weixin.qq.com/s/TS3R8MNF_t09HmYNHMMHTg)

## CoreOS 的rpm-ostree用Rust重写部分功能

rpm-ostree 是一个CoreOS上的包管理器，最近使用Rust重写部分功能。该团队说更多氧化项目（比如/etc/{passwd,group}）正在进行中。

链接：[https://github.com/coreos/rpm-ostree/releases/tag/v2021.2](https://github.com/coreos/rpm-ostree/releases/tag/v2021.2)

## 《Rust用于web开发的2年后感悟》

原文地址：[https://kerkour.com/blog/rust-for-web-development-2-years-later/](https://kerkour.com/blog/rust-for-web-development-2-years-later/)

大约2年前，我开始使用Rust开发Web服务（JSON API），我认为是时候可以摆脱先入为主的观念并分享我学到的知识了。

偏见:

- Rust代码很丑陋：Rust是显式的。不可否认。但是，当我编写代码时，我的IDE可以帮到我很多，而不必按下那么多键。当我阅读代码时，这种明确性真是太棒了！没有隐藏的惊喜，没有奇怪的事情。
- 内存管理令人分心：实际上呢，没有。我没有使用那么多的词法生命周期，而是使用了智能指针。是的，因此我理解了Box，Rc和Arc之间的差异，与之同时和Node.JS、Golang语言相比，我的生产率没有因此受到影响。
- 编译器很麻烦：一开始是的。但是几个月后，我能够立即理解错误，并能立刻解决这些错误。今天，我真的没有花太多时间在编译器上。相反，它成为了我最好的朋友，尤其是在重构大部分代码或升级依赖项时。
- 缓慢的编译时间:我给这个说明。在Node.JS或Golang中，一个中等大小的服务的Docker image大约需要3到10分钟来构建和部署，在Rust中大约需要30分钟。
- 生态系统还不存在：不可否认，的确是这样。缺少一些组件，例如官方的Stripe和AWS开发工具包，但是社区确实很活跃，并构建了所有这些缺少的组件。

我特别值得点赞的几件事

- 静态链接非常简单：创建小的Docker images 一件令人愉快的事情。。
- Rust会让你成为一个更好的程序员：Rust很复杂，如果你不了解它的详细工作原理，它不会放过你。掌握它需要时间和耐心，但是一旦你这样做了，你就会学到很多你永远不会像以前那样接近编程的东西。在学习Tokio的工作原理时，我了解了Golang的运行时是如何工作的。(心智模型学习)
- 一旦它编译，通常它就可以正常工作:这是关于Rust我最喜欢的地方。当我的程序编译时，它按我的计划工作。注意：只要记住不要阻塞事件循环，编译器就会处理剩下的事情。您不再需要花时间为语言的怪癖编写测试。
- Rust具有很高的生产力：由于Rust是多种范式，因此在编写复杂的业务逻辑时，由于其功能方面，它的确非常出色。

当前我正在使用的一些crates

- actix-web 用于HTTP层.
- sqlx 用于数据库PostgreSQL.
- rusoto AWS接口服务（S3、SQS、SES）
- tera 用于电子邮件模板
- thiserror 用于错误类型处理
- sentry 用于错误监控

结论

Rust非常适合用于web开发，在此我强烈建议尝试一下。

取得成功是一次漫长的旅程，但完全值得，即使您不是每天都在使用它，也一定会通过学习它而成为一名更好的程序员，如果失去了，那就重新去发现编程的乐趣🤗。

一句话总结：Rust生而平静。凌晨3点不再有不好的惊喜，因为依赖项更新了它的API使得不再有bug。没有更多恼人的配置自动缩放或什么。而且响应时间非常短，您的用户因此会爱上您的产品。



