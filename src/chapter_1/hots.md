# 本月简报 | 社区热点

- 来源：[Rust日报](https://rustcc.cn/section?id=f4703117-7e6b-4caf-aa22-a3ad3db6898f)
- 作者：Rust 日报小组

## Deno in 2020

一直很火热的 Deno 官方最近发布了 Deno 的[大事记表](https://deno.land/posts/deno-in-2020)。
其中 1 月份进行了将 libdeno 替换成 rusty_v8 的工作。之前是使用 libdeno（C++ 写的） 来进行绑定 V8 的操作。现在替换成 Rust 原生实现的 rusty_v8。并且 rusty_v8 是一个单独的 Rust crate。

## The Rust on Raspberry Pi Pico Charity Live Stream

在树莓派上写 Rust 是一种怎样的体验？最近一位国外友人就尝试这么做了，并且进行了直播。具体详情可以[戳此链接](https://www.reddit.com/r/rust/comments/l39jdo/the_rust_on_raspberry_pi_pico_charity_live_stream/)。
Rust 在嵌入式开发领域还是有非常大的潜力的。

想要看更多关于 Rust 的流媒体视频，可以关注这个项目 [awesome-rust-streaming](https://github.com/jamesmunns/awesome-rust-streaming)

## Sequoia PGP 发布 1.0 版本

2018 年，三位 GnuPG 开发者开始着手开发 Sequoia，这是 OpenPGP 在 Rust 中的实现版本。OpenPGP 是一种非专有协议，为加密消息、签名、私钥和用于交换公钥的证书定义了统一标准。

通过[官方博客](https://sequoia-pgp.org/blog/2020/12/16/202012-1.0/)可以看出团队对当前版本对于安全性的思考和对未来下一步的规划。

## Rustup 宣布发布 1.23.0 版本

官方发布 1.23.0 版本，其中最激动人心的改变就是支持 Apple M1 设备。大家可以安心的买 M1 了！

[原文链接](https://blog.rust-lang.org/2020/11/27/Rustup-1.23.0.html)

## Firecracker

Firecracker 是一种开源虚拟化技术，专门用于创建和管理安全的，多租户容器和基于功能的服务。

[项目地址](https://github.com/firecracker-microvm/firecracker)

## Rust GUI 编程介绍

Rust GUI 方面的介绍以及目前 Rust GUI 库的现阶段状况

[原文链接](https://dev.to/davidedelpapa/rust-gui-introduction-a-k-a-the-state-of-rust-gui-libraries-as-of-january-2021-40gl#comments)

## Facebook 使用 Rust 的简单介绍

该 twitter 快速的介绍了 Rust 在 facebook 中的使用历程:

2017 年开始应用于一个资源控制项目，后来证明性能和稳定性都比 C++好。
之后，更多的工程师开始使用 Rust 在各种项目中，例如 Diem，Hack，Mononoke。
在 dev tools 中证明 Rust 可行之后, 开始在后端和手机应用中使用 Rust
很多工程师来自 python 和 javascript 语言，Rust 的强类型和高性能让这些工程师不再挣扎于运行时的 bug。
为了让 Rust 更广泛的使用，设立了一个专门的 Rust 小组来支持其他的工程师在不同的项目中使用 Rust。 该小组同时在 Rust 社区中也非常活跃，贡献代码。

[原文链接](https://twitter.com/alexvoica/status/1350049393471324161)

## Rust 要上太空了！RocketLab 招聘 Rust 工程师

Rocket Lab 是小型卫星发射领域的全球领导者。团队有 500 人，而且每周都在增加。

当然，这是在美国的工作。期待国内也会有！

[原文链接](https://www.rocketlabusa.com/careers/positions/flight-software-engineer-ii-auckland-new-zealand-3653845/)

## Rust 书籍宝库

glynnormington 整理了网络上大部分有关 rust 的 mdbook，有官方的，也有非官方的。值得注意的一点是大家关注的 Rust 宏小册很多人以为一直没有更新，但是其实有另一个团队重新在原来的基础上，更新了新的版本，目前已收录到该书库中。

[原文链接](https://www.reddit.com/r/rust/comments/kwiwb8/the_little_book_of_rust_books/)

[项目地址](https://lborb.github.io/book/title-page.html)

## 时隔一年 tower 终于发布新版本啦

Tower 是一个模块化和可重用组件库，用于构建健壮的网络客户端和服务器。上一个版本 0.3.1 版本是 2020 年 1 月 17 发布的，新版本 0.4.0 是 2021 年 1 月 7 号发布的，这个版本包含了大量改动，包括使用了 tokio 1.0，将所有的中间件转移到了 tower crate，改造，添加了中间件 API。

不过这次变更并没有核心 Service 或者 Layer trait，所以新版本还是依赖着 tower- service 0.3 和 tower- layer 0.3，因此新版本是兼容使用这两个 crate 的库的。更多发布细节请移步下面的链接。

[原文链接](https://github.com/tower-rs/tower/releases/tag/tower-0.4.0)

[项目地址](https://crates.io/crates/tower/0.4.0)

## Rust Search Extension 1.1.0 发布

[Rust Search Extension](https://rust.extension.sh/) 发布了最新版，同时也突破了 500 个 star，感谢大家的支持！这个版本主要功能如下：

- ! 搜索改成了 docs.rs，!! 改成了 crates.io。
- 给 Rust 仓库的 release 页面增加了目录菜单。
- Rust 标准库文档页面和源码页面所有 "since" 和 "issue" 标签分别会链接到仓库的 release 页面对应的版本和 GitHub 对应的 issue 页。

## 为什么 2021 年将成为系统程序员的 Rust 年？

Gartner 今天的一篇博文报道了“Rust”：近年来，Rust 获得了很多粉丝，并且有充分的理由。Rust 旨在成为满足系统编程需求的 C++ 的可靠替代品。

[原文链接](https://blogs.gartner.com/manjunath-bhat/2021/01/03/why-2021-will-be-a-rusty-year-for-system-programmers/)

