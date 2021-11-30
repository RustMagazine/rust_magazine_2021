# 官方动态

聚焦 Rust 官方活动与新闻

---

##  Rust 1.56.1 版本发布

Rust1.56.1引入了两个新的lints，以减轻最近披露的一个安全问题的影响，`CVE-2021-42574`。我们建议所有用户立即升级，以确保其代码库不受安全问题的影响。

如果已经安装过 Rust,使用如下命令升级:

```
rustup update stable
```

- [原文链接](https://blog.rust-lang.org/2021/11/01/Rust-1.56.1.html)
- [CVE-2021-42574 相关说明](https://blog.rust-lang.org/2021/11/01/cve-2021-42574.html)
- 关于该漏洞详细文章： [特洛伊之源｜ 在 Rust 代码中隐藏无形的漏洞](./trojan-source.md)

## 本月新加入 Rust 基金会的公司

涉及领域： 游戏、芯片、汽车、AI 安全

- 动视（activision） 游戏公司 ： [https://www.activision.com/](https://www.activision.com/)
- ARM 芯片公司 ： [https://www.arm.com/](https://www.arm.com/)
- 丰田汽车（TOYOTA）: [https://www.toyotaconnected.com/](https://www.toyotaconnected.com/)
- ZAMA : AI 端到端加密,用于保护云中的 AI 应用程序 [https://github.com/zama-ai/concrete](https://github.com/zama-ai/concrete)
- Spectral:  代码安全  [https://spectralops.io/](https://spectralops.io/)
- 1Password ： 密码管理软件

Rust 基金会正在推动 Rust 在各个领域广泛应用

[https://foundation.rust-lang.org/members/](https://foundation.rust-lang.org/members/)

## Rust 基金会任命 Rebecca Rumbul 为执行董事兼首席执行官

2021 年 11 月 17 日，[Rust 基金会](https://foundation.rust-lang.org/)宣布任命[Rebecca Rumbul](https://foundation.rust-lang.org/posts/2021-11-17-introducing-rebecca-rumbul/)为该组织的执行董事兼首席执行官。

Rumbul 来到 Rust 基金会，她在国际非营利管理方面拥有深厚的专业知识，并且是数字民主和信息权利的领先全球倡导者。Rumbul 最近担任 mySociety 的研究和参与总监，在那里她致力于为世界各地的政府、非政府组织和商业企业的治理和议会系统带来透明度。

Rumbul 拥有在执行和学术委员会工作的广泛背景，曾担任广告标准局的理事会成员、Hansard Society 的受托人以及 Privacy Collective 的英国代表索赔人。她拥有开放大学的政治、公共行政和项目治理博士学位、公共行政硕士学位和人文科学学士学位。

“Rust 社区正在使用 Rust 编程语言做重要的、鼓舞人心的工作，我很高兴能成为对世界产生重大影响的合作的一部分，”Rumbul 说。“Rust 的核心功能使其具有全球影响力，包括支持安全和可持续性计划的进步。很荣幸成为其中的一员，我期待加入这个变革性的社区。”

“Rebecca 在非营利组织和数字治理方面带来了丰富的领导经验，”执行董事搜索委员会主席、Rust 基金会董事会成员和 Mozilla 杰出工程师 Bobby Holley 说。“她是一位聪明、善于协作的领导者，拥有建立 Rust 基金会的远见和勇气，并产生我们在这里要产生的影响。”

除 Holley 外，执行董事搜寻委员会还包括红帽项目总监兼首席软件工程师 Shane Miller、Josh Stone、核心团队项目总监兼 Rust 基金会秘书 Mark Rousskov 以及项目总监兼高级软件工程师 Tyler Mandry谷歌工程师。

[https://foundation.rust-lang.org/news/2021-11-17-news-announcing-rebecca-rumbul-executive-director-ceo/](https://foundation.rust-lang.org/news/2021-11-17-news-announcing-rebecca-rumbul-executive-director-ceo/)

## Rustc 变得更快了！

作者在 Mozilla 从事 Rust 的兼职工作数年，那段时间里，作者定期总结编译器的性能是如何改进的。 例如：2017-11-12 至 2019-07-24。进行的最后一次比较是在 2020 年 8 月，从 2020 年末开始暂停 Rust 工作，直到本周我成为 Futurewei Technologies 的 Rust 团队的全职成员，作者很想知道那段时间的性能是如何提高的，好消息！

从 2020-08-05 到 2021-11-11，编译器基准套件的结果有 459 项改进，只有 18 项回归，如下面（很长）的屏幕截图所示，因为截图过长印象阅读体验，请移步原文查看。

- [https://nnethercote.github.io/2021/11/12/the-rust-compiler-has-gotten-faster-again.html](https://nnethercote.github.io/2021/11/12/the-rust-compiler-has-gotten-faster-again.html)

## `std::simd` nightly 可用

可以移植的 SIMD 模块.

该模块提供了一个可移植的不绑定于任何硬件架构的 SIMD 操作的跨平台抽象。目前 nightly-only.

[https://doc.rust-lang.org/nightly/std/simd/index.html](https://doc.rust-lang.org/nightly/std/simd/index.html)

### 内联汇编语法快稳定了

跟踪贴在这里：https://github.com/rust-lang/rust/issues/72016#issuecomment-964186287

