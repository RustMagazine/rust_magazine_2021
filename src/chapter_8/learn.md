# 学习资源

编辑： 张汉东

---

## 《Rust In Action》书籍第一版发布

作何 Tim McNamara 是一位经验丰富的程序员，对自然语言处理、文本挖掘以及更广泛的机器学习和人工智能形式有着浓厚的兴趣。 他在包括新西兰开源协会在内的开源社区中非常活跃。Rust in Action 是使用 Rust 进行系统编程的实践指南，它是为具有好奇心的程序员编写的，提供了远远超出语法和结构的实际用例。

国外最近的Rust的书籍，除了《Rust In Action》还有另外两本，《Refactor to Rust》和 《Rust Servers, Services, and Apps》。

国内翻译版也在路上了。

[Amazon](https://www.amazon.com/dp/1617294551/ref=cm_sw_r_cp_awdb_imm_VJ4HZ4859SDB7K5B7VQK)

## 使用 Rust 进行端到端加密

《End-to-End Encryption with Rust》是一本`ockam-network/ockam`实践指南， 在本指南中，我们将创建两个名为 Alice 和 Bob 的小型 Rust 程序。 Alice 和 Bob 将通过云服务通过网络相互发送消息。 在我们的代码示例中，Alice 和 Bob 将相互进行身份验证，并将获得加密保证，以确保其消息的完整性、真实性和机密性得到端到端的保护。

网络上的中间云服务和攻击者将无法看到或更改途中消息的内容。 在后面的示例中，我们还将看到即使当 Alice 和 Bob 之间的通信路径更复杂 - 具有多个传输连接、各种传输协议和许多中介时，我们如何才能实现这种端到端保护。

[https://github.com/ockam-network/ockam/tree/develop/documentation/use-cases/end-to-end-encryption-with-rust#readme](https://github.com/ockam-network/ockam/tree/develop/documentation/use-cases/end-to-end-encryption-with-rust#readme)

## 两张图展示当前 Rust Web 生态

微信： [https://mp.weixin.qq.com/s/eIOMI0JvpOkdmiTqJfWkRg](https://mp.weixin.qq.com/s/eIOMI0JvpOkdmiTqJfWkRg)
知乎： [https://zhuanlan.zhihu.com/p/398232138](https://zhuanlan.zhihu.com/p/398232138)


## 创意！用 Rust crate 作为自己的简历 

如果你觉得学习 Rust 不知道该做些什么好？那不如从做自己简历开始。

[https://yozhgoor.github.io/yohan_boogaert_1995/](https://yozhgoor.github.io/yohan_boogaert_1995/)

## Mini Lust 系列教程：

好奇如何从零造出来一个 RPC 框架？本教程将带你一步一步写出来一个 Rust 版 Thrift RPC 框架。 

1.前言部分，RPC 相关概念介绍
2. Thrift IDL 介绍
3. 序列化/反序列化的抽象
4. Codec 和 Transport 抽象
5. 客户端和服务端实现
6. Thrift IDL 解析和代码生成
7. 基于 tower 的服务发现和负载均衡
8. 中间件支持

[https://github.com/mini-lust/tutorials](https://github.com/mini-lust/tutorials)

## Rust 公开课 | 《 Rust 异步编程二: Tokio 入门运行时介绍》|Vol. 6

这节课预计 9.5 号晚上8点，感兴趣的可以去听听。

该系列课程大纲

1、回顾 Rust 异步编程模型.
2、谈谈对 Rust 异步框架的认识 ( futures-rs、async-std、tokio ) .
3、Tokio 介绍
4、Tokio 里的 Executor、Reactor、Future 如何使用.
5、使用 Tokio 实现一个简单的服务端与客户端程序.

[https://mp.weixin.qq.com/s/23YDZdwJNOAu15AIBDnWuQ](https://mp.weixin.qq.com/s/23YDZdwJNOAu15AIBDnWuQ)

## Clippy 1.54 增加 `disallowed-methods` 配置

允许你在 `clippy.toml` 中配置不允许的方法：

```rust
# clippy.toml
disallowed-methods = ["std::vec::Vec::leak", "std::time::Instant::now"]
```

不良代码：

```rust
// 该代码将要被警告

let xs = vec![1, 2, 3, 4];
xs.leak(); // Vec::leak is disallowed in the config.

let _now = Instant::now(); // Instant::now is disallowed in the config.
```

应该用此代替：

```rust
// Example code which does not raise clippy warning
let mut xs = Vec::new(); // Vec::new is _not_ disallowed in the config.
xs.push(123); // Vec::push is _not_ disallowed in the config.
```

## 5000倍速度提升的 CRDT

CRDT 全称 Conflict-Free Replicated Data types. 主要用于在线合作文档编辑等方面. 

作者详细介绍了如何提升相关实现和算法的一些过程,并且最终使得提升了 5000 倍的速度.

[https://josephg.com/blog/crdts-go-brrr/](https://josephg.com/blog/crdts-go-brrr/)

## 如何写出运行缓慢的 Rust 代码

用Rust写代码并不意味着你的代码会快得不得了。你很容易犯错并获得相当慢的性能。正如这篇博文所显示的，你甚至可能需要付出相当多的汗水才能打败Common Lisp和Java。

作者分享了自己如何使用 Rust 重写自己的 Lisp 代码, 如何成功的写出更慢的代码 并且 修复他们的故事.

[https://renato.athaydes.com/posts/how-to-write-slow-rust-code.html](https://renato.athaydes.com/posts/how-to-write-slow-rust-code.html)

## RustCast: Rust 系列教学视频

一系列 Rust 学习系列视频，希望能坚持下去。

[https://www.youtube.com/channel/UCZSy_LFJOtOPPcsE64KxDkw](https://www.youtube.com/channel/UCZSy_LFJOtOPPcsE64KxDkw)

## 用Rust重写我的手机游戏，并且编译到 wasm

作者的游戏之前是用 C++ 写的。这篇文章详细记录了他决心使用rust重写的心路历程和一些idea的发展。

推荐阅读：

[https://itnext.io/rewriting-my-mobile-game-in-rust-targeting-wasm-1f9f82751830](https://itnext.io/rewriting-my-mobile-game-in-rust-targeting-wasm-1f9f82751830)

## 使用 Rust 从头开始​​实现 Base64

文章仔细研究 Base64 算法，并使用 Rust 编程语言从头开始实现编码器和解码器。

[https://dev.to/tiemen/implementing-base64-from-scratch-in-rust-kb1](https://dev.to/tiemen/implementing-base64-from-scratch-in-rust-kb1)

## Async Rust 从头开始​​：一个简单的 Web 服务器

[https://ibraheem.ca/writings/a-simple-web-server/](https://ibraheem.ca/writings/a-simple-web-server/)

## 一个网络应用程序，可以学习使用 AI（遗传算法）构建车辆，使用Rust编写

它在你的浏览器中运行，使用人工智能（具体来说：遗传算法）来尝试制造越来越好的车辆。车辆必须克服障碍路线，从一些小山坡开始，然后是陡峭的山坡，最后是一些跳跃。车辆由面板和轮子制成，连接在一起，类似于Besiege游戏。

[https://github.com/Bauxitedev/vehicle_evolver_deluxe](https://github.com/Bauxitedev/vehicle_evolver_deluxe)

## 当零成本抽象不再是零成本
Rust 是围绕着“零成本抽象”的概念构建的。其理念是，您可以编写人机友好的高级代码，而编译器将为您提供至少与您自己编写的任何优化的低级别代码一样好的性能。使用零成本抽象，您不再需要在可维护性和性能之间进行权衡。

不幸的是，很难确保零成本抽象是真正的零成本，并且在实践中Rust经常不能满足这个崇高的理想。在这篇文章中，我将展示两个例子，在这两个例子中，即使看似简单的零成本抽象实际上也不是零成本。

[https://blog.polybdenum.com/2021/08/09/when-zero-cost-abstractions-aren-t-zero-cost.html](https://blog.polybdenum.com/2021/08/09/when-zero-cost-abstractions-aren-t-zero-cost.html)

## 【系列】Rust 每周一模块

这是一个系列博客，目前只发了两篇文章，每周讲一个模块：

比如第二周：Rust 标准库中`std::fs`模块

`std::fs` 是Rust标准库中操作文件系统的模块，包括创建、读取、更新、删除等常见操作。由于不同操作系统支持的API不尽相同，本文仅展示了与平台无关的一些例子：

- 通过修改时间(mtime)来聚合相同年份、月份乃至日期的文件；
- 硬链接(hard link)一个路径至另一个路径；
- 递归创建目录；
- 递归删除文件夹；
- 拷贝文件；

[https://motw.rs/](https://motw.rs/)

## 【书籍】Black Hat Rust 早期访问版

Black Hat Rust 是一本深入研究使用 Rust 编程语言的进攻性安全（Offensive Security）的书籍，支持PDF，Kindle 和 Epub。

这本书是一项正在进行的工作。它可以在早期访问计划的背景下使用，这意味着各章节将在写完后立即发送给你，我们非常感谢你的反馈。当前状态：

可访问页数：250+ 代码进度：~90% [https://github.com/skerkour/black-hat-rust](https://github.com/skerkour/black-hat-rust) 预计最终出版：Q3 2021 估计的页数：~320

备注：作者为感谢所有帮助其完成这本书的人，所有早期访问的买家还将获得以下奖励：一个高级恶意软件分析的策划清单。在开发自己的攻击性工具时，会在里面找到巨大的灵感。

[https://academy.kerkour.com/black-hat-rust?coupon=BLOG](https://academy.kerkour.com/black-hat-rust?coupon=BLOG)

## 如何写出高效的 Rust 代码

该文作者对如何写出高效 Rust 代码给出了一些建议，内容还比较长，感兴趣可以看看。

[https://renato.athaydes.com/posts/how-to-write-fast-rust-code.html](https://renato.athaydes.com/posts/how-to-write-fast-rust-code.html)

## 理解 `#[derive(Clone)]` 宏

你可能不知道这个宏背后发生的事，这篇文章带你探索一下。

[https://stegosaurusdormant.com/understanding-derive-clone/](https://stegosaurusdormant.com/understanding-derive-clone/)