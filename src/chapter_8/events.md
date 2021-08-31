# 活动回顾

后期编辑：张汉东

> 编者按：
>
> 总结了本月的活动，包括线上和线下。
>
> 线上： 《Rust 唠嗑室》和 《RustFriday 飞书群线上沙龙》

---

# 【线上】Rust 唠嗑室本月汇总

- 来源：[Rust 唠嗑室](https://space.bilibili.com/25566598/video)
- 主持人：MikeTang
- 后期编辑：高宪凤

### 《Rust 唠嗑室》第 30 期 - 一起来围观 axum

**时间**: 2021/08/03 20:30-21:30

**主讲人**：Mike Tang

**题目**：一起来围观 Axum

**内容**： 就刚不久，tokio 团队推出了 Web 开发框架 axum，引起社区轰动。我们今晚一起来感受一下这个新框架。

参考资料：

1. https://docs.rs/axum/0.2.3/axum/

【回放】

- [https://www.bilibili.com/video/BV1MU4y1J79Y](https://www.bilibili.com/video/BV1MU4y1J79Y)

---

### 《Rust 唠嗑室》第 31 期 - Rust FFI 实践：分布式机器学习

**时间**: 2021/08/17 20:30-21:30

**主讲人**：Alice

**题目**：Rust FFI 实践：分布式机器学习

**内容**：本主题讲述如何通过 Rust 连接起深度学习已有的 Python, C++, CUDA 生态，构建大规模分布式训练工具。同时享受到 Rust 需要带来的高安全性、高开发效率。

参考资料：

1. 源代码：https://github.com/BaguaSys/bagua

【回放】

- [https://www.bilibili.com/video/BV1Gv411N7Z7](https://www.bilibili.com/video/BV1Gv411N7Z7)


---

<center> 🔥🔥🔥🔥 <strong>RustFriday 飞书群线上沙龙</strong> 🔥🔥🔥🔥 </center>

# 【线上】RustFriday 飞书群线上沙龙

每周五晚八点，限定两个主题：语言特性和开源项目，在线讨论。

Rust 中文社群 飞书群 邀请你加入：

对话群： [https://applink.feishu.cn/TeLAcbDR](https://applink.feishu.cn/TeLAcbDR)

话题群：[https://applink.feishu.cn/TeLD868w](https://applink.feishu.cn/TeLD868w)

视频来源：[https://space.bilibili.com/24917186](https://space.bilibili.com/24917186)

## 第十三期 讨论主题：如何在面试中考察一个人 Rust 水平？

1. 如何面试/或准备应聘一场 Rust 面试。
2. 实际面试经验分享 （@ huangjj ）
3. vscode + ra 使用小技巧分享 。

【回放】

- [https://www.bilibili.com/video/BV1ZV411p7Y3](https://www.bilibili.com/video/BV1ZV411p7Y3)

## 第十七期 ｜ Axum 异步 Web 框架

分享者：张汉东

【讨论主题】

1. axum 的中间件是直接使用 tower 的抽象，这样的好处就是：
   a. 使用了统一 的 Service 和 Layer 抽象标准，方便大家来繁荣生态
   b. 复用 tokio / hyper/ tonic 生态

2. axum 的路由机制是提供了简单的 DSL （链式调用）。路由是基于迭代和正则表达式来匹配的

3. 也提供了方便的 提取器 ，只要实现 FromRequest 就是一个提取器，实现起来也非常方便。

【参考资料】

1. 源代码：https://github.com/tokio-rs/axum

【回放】

- [https://www.bilibili.com/video/BV1Kb4y16742](https://www.bilibili.com/video/BV1Kb4y16742)


## 第十八期 ｜ 如何用 Rust 实现 RPC 框架

分享者：张汉东

【讨论主题】

1.  教程作者 茌海 分享了 Lust 框架的思路。为什么要用rust呢？因为go 服务的性能已经到了存量优化的一个瓶颈，不得不考虑使用rust重新实现从根本上消除go系统所带来的问题。
2.  大家一起跟随教程学习 如果构建一个 rpc 框架。 该教程虽然比较简短，但其实内容很丰富，并且融合了很多生产实践的思考。
    - 从第二章到第三章是介绍如果抽象消息和协议，
    -  第四章和第五章，介绍如何结合tokio codec 和 transport 来进一步整合消息和协议，以及使用tower 增加中间件支持，复用tokio生态里的工具。
    -  第六章 则介绍了如何使用nom来解析 thrift idl，以及通过过程宏来自动生成代码。
    -  第七和第八章则进一步基于tower来增加服务发现/负债均衡/自定义中间件等功能。麻雀虽小，五脏俱全，非常值得学习。

【参考资料】

 [https://github.com/mini-lust/tutorials](https://github.com/mini-lust/tutorials)

【回放】

- [https://www.bilibili.com/video/BV1Pg411V7FM/](https://www.bilibili.com/video/BV1Pg411V7FM/)


---

<center> 🔥🔥🔥🔥 <strong>北京-杭州MeetUp-20210808</strong> 🔥🔥🔥🔥 </center>

视频集地址：[https://www.bilibili.com/video/BV1tM4y157UP](https://www.bilibili.com/video/BV1tM4y157UP)

大会 PPT 链接：https://pan.baidu.com/s/1FKVQRcTk5YfXvo9lOuKe2Q

提取码：2ivv

## Rust+Tokio 在又拍云的实践

**演讲者**：夏功勋

## Tokio Internals

**演讲者**：王福音

## Rust 生态的性能调试器 pprof-rs

**演讲者**：杨可奥

参考资料：

1. https://github.com/tikv/pprof-rs

## 深挖 move 语义

**演讲者**：丁绍顺

## Rust Web 开发见解

**演讲者**：赵春霖

## ARM 上的 Cloud-Hypervisor

**演讲者**：李枫

参考资料：

1. http://en.wikipedia.org/wiki/
2. http://www.slideshare.net/
3. https://en.wikipedia.org/wiki/Comparison_of_application_virtualization_software
4. https://www.sciencedirect.com/topics/computer-science/assisted-virtualization
5. https://en.wikipedia.org/wiki/Systems_programming
6. https://docs.01.org/clearlinux/latest/tutorials/kata.html
7. https://www.redhat.com/sysadmin/selinux-kata-containers

## cps 变化在 Rust 语言中尾递归应用及其性能分析

**演讲者**：常开颜

---

<center> 🔥🔥🔥🔥 <strong>20210807-Hello Web3.0 Meetup 全国行首次 online 分享会</strong> 🔥🔥🔥🔥 </center>

视频集地址：[https://www.bilibili.com/video/BV1nq4y1Q73F](https://www.bilibili.com/video/BV1nq4y1Q73F)

## Web3.0 导论简述

**演讲者**：Mike

关于 Web3.0 的几种观点：

1. 语义互联网
2. 按请填写交互性来分代
3. 价值互联网
4. Coinbase 的观点

## 面向 Web3.0 的智能合约开发

**演讲者**：Robert

本次主题主要讲解了基于 Near 智能合约开发。为什么 Near 会成为智能合约开发选项呢？本次 Topic 中进一步分析。

参考资料：

1. https://near.org/

## Web3.0 应用链开发

**演讲者**：Lester

对于 Web3.0 的概述：

1. Web3.0 一个更开放、更公平更安全的互联网新阶段。
2. Web3.0 应用【社区拥有的加密协议】取代【公司拥有的平台】。
3. Web3.0 应用落地，要有媲美 Web2.0 应用的用户体验和服务能力。

重点介绍了通过 Substrate 框架开发应用链。

## Web3.0 隐私计算系统设计

**演讲者**：周顺帆

Phala Network 是尝试给区块链提供隐私计算的网络，通过什么样的技术手段和系统设计保证合约隐私、安全和整体性能。

## Akash 开源云计算市场

**演讲者**：红军大叔

Akash 一个去中心化的亚马逊云或阿里云。当前云计算现状是由四大巨头（亚马逊云、谷歌云、阿里云、微软云）垄断，垄断最终伤害用户权益。Akash 借助区块链的特性和虚拟化容器技术提供去中心化版本的云计算。

## Web3.0 存储和可信计算

**演讲者**：熊炜

开发过程是遇到很多问题，包括，矿工费贵、用户门槛高等，这些问题一直在困扰着开发着，去年参加星火组织的以太坊 MeetUp 产生新的想法，使用一条区块链构建一个新的应用。

## Programming On Solana

**演讲者**：葛鑫

Francium 是一个杠杆收益平台。在 Solana 上开发合约，需要理解账户模型，Transaction 和 Instruction 之间的关系，最后是对 Program 的理解。

## Applications of zkSNARK in Decentralized Finance

**演讲者**：Jamie

在 Web3.0 中，在 DeFi 领域如何使用零知识证明技术达到隐私保护目的。
