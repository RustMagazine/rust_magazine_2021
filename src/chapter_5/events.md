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

### 《Rust 唠嗑室》第 24 期 - 大家一起来闲聊一下最近使用 Rust 的心得和遇到的坑

**时间**: 2021/05/11 20:30-21:30

**主讲人**：无

**内容**：大家一起来闲聊一下最近使用 Rust 的心得和遇到的坑

顺便也约了金明剑老师来谈谈 TensorBase 项目的最新的一些进展。

其它几个可能的话题：

- 1.52 编译器的增量编译 bug。紧急修复版本 1.52.1

- bevy 0.5 及其它 Rust 游戏框架等

- Rust 在嵌入式领域的实践

[查看回放](https://www.bilibili.com/video/BV1uo4y1m7vp)

### 《Rust 唠嗑室》第 25 期 - 使用 Rust 快速开发基于 Raft 共识算法的分布式应用

**时间**: 2021/05/25 20:30-21:30

**主讲人**：PsiACE

**内容**：介绍如何基于流行的 raft-rs 开发分布式应用以及 raft-frp/RiteRaft 设计思路

- 回顾 raft 共识算法的一些基本内容

- 如何使用 raft-rs 开发分布式应用

- 降低重复劳动, 160 行代码实现一个简单的分布式键值服务

- 目前项目进展

[查看回放](https://www.bilibili.com/video/BV1ZK4y1G7GR)

参考资料：

1. https://www.cnblogs.com/yanglang/p/10141583.html
2. https://www.jianshu.com/p/81fe3e4f51a5
3. https://zzjw.cc/post/raft-build
4. https://github.com/erikgrinaker/toydb/blob/master/docs/architecture.md
5. https://github.com/MarinPostma/raft-frp
6. https://github.com/canonical/raft
7. https://github.com/blackredscarf/raftfwk
8. https://github.com/tikv/raft-rs

---

<center> 🔥🔥🔥🔥 <strong>RustFriday 飞书群线上沙龙</strong> 🔥🔥🔥🔥 </center>

# 【线上】RustFriday 飞书群线上沙龙

每周五晚八点，限定两个主题：语言特性和开源项目，在线讨论。

Rust 中文社群 飞书群 邀请你加入：

对话群： [https://applink.feishu.cn/TeLAcbDR](https://applink.feishu.cn/TeLAcbDR)

话题群：[https://applink.feishu.cn/TeLD868w](https://applink.feishu.cn/TeLD868w)

视频来源：[https://space.bilibili.com/24917186](https://space.bilibili.com/24917186)

## 第六期讨论主题：

1. 领域项目：eclipse zenoh ，一个零开销的 Pub/sub, Store/Query 和 Compute 平台
2. 语言特性：Rust 中的 高阶类型 GAT (Generic Associated Types)

参考资料：

1. https://rustmagazine.github.io/rust_magazine_2021/chapter_4/zenoh.html
2. https://github.com/eclipse-zenoh/zenoh
3. https://github.com/autocore-ai/SDV

[查看回放](https://www.bilibili.com/video/BV1Hf4y1p7Pi)

## 第七期 讨论主题：

1. 语言特性：实现 RFC : Allow a re-export for main
2. 领域项目： makepad， cloud9 ide 作者新的项目，支持 VR 开发的基于 WebAssembly 的 IDE/ 框架

参考资料：

1. https://github.com/rust-lang/rfcs/blob/master/text/1260-main-reexport.md
2. https://github.com/rust-lang/rust/pull/84217
3. https://github.com/rust-lang/rust/pull/84401
4. https://github.com/rust-lang/rust/pull/84507

[查看回放](https://www.bilibili.com/video/BV1264y1C7cu)

## 第八期 讨论主题：

1. 语言特性： 聊聊 Rust 泛型特化功能现状
2. 领域项目： 介绍一下 WAGI 项目及其进展

参考资料：

1. https://github.com/deislabs/wagi
2. https://deislabs.io/posts/introducing-wagi-easiest-way-to-build-webassembly-microservices/
3. https://github.com/deislabs/hello-wagi-grain
4. https://radu-matei.com/blog/wagi-updates/
5. https://github.com/engineerd/wasm-to-oci
6. https://github.com/WebAssembly/WASI

[查看回放](https://www.bilibili.com/video/BV1m64y1C73P)

## 第九期 讨论主题：WebAssembly 专题课堂（上）

《 WebAssembly Actors: From Cloud to Edge》这套课程简洁精炼，虽然着墨不多，但是把 WebAssembly 及其在 Server Side 的应用关键概念都涉及，并且通过简单的示例展现清楚了。

参考资料：

1. https://www.edx.org/course/webassembly-actors-from-cloud-to-edge

[查看回放](https://www.bilibili.com/video/BV1B44y1r7HA)
