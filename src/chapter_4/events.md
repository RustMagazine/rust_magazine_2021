# 活动回顾

后期编辑：张汉东

> 编者按：
> 
> 总结了本月的活动，包括线上和线下。
> 
> 线上： 《Rust 唠嗑室》和 《RustFriday 飞书群线上沙龙》

---

# 【活动预告】2021.05.15 北京线下 WebAssembly Meetup **

现在征集议题 ：  [议题申请](https://mp.weixin.qq.com/s/5MFN1x8XQBeE3Zs7oqBtsQ)

---

# 【线上】Rust 唠嗑室本月汇总

- 来源：[Rust 唠嗑室](https://space.bilibili.com/25566598/video)
- 主持人：MikeTang
- 后期编辑：高宪凤


### 《Rust 唠嗑室》第 21 期 - Local Native 分布式应用开发

**时间**: 2021/03/30 20:30-21:30

**主讲人**：Yi Wang

**内容**：详解 Rust 实现的跨平台去中心的应用 Local Native

- demo 网络书签信息管理工具 Local Native
- 技术讲解
- 未来的目标
- demo 姐妹项目 Fastxt
- Q&A

[查看回放](https://www.bilibili.com/video/BV1wi4y1N7Ez)

**扩展资料**：

- 官方网站 [https://yilab.com/](https://yilab.com/)

---

### 《Rust 唠嗑室》第 22 期 - 关于 Rust io_uring 异步接口实现的思考

**时间**: 2021/04/13 20:30-21:30

**主讲人**：施继成

**内容**：关于 Rust io_uring 异步接口实现的思考

io_uring 接口在 Linux 中被用于高效的异步 I/O 操作，但是使用 liburing 的简单封装十分不友好，我们探索了一种实现方法，一方面能够简化接口的使用，另外一方面避免不必要的内存拷贝保证效率。借此机会和大家分享一下实现过程中的思考，也聆听大家的建议，不断改进优化实现方法。

[查看回放](https://www.bilibili.com/video/BV1VA411L7tt)

**扩展资料**：

- Liburing 库的简单封装 [https://github.com/ringbahn/uring-sys](https://github.com/ringbahn/uring-sys)
- 队列、注册器等数据结构抽象 [https://github.com/datenlord/ring-io](https://github.com/datenlord/ring-io)
- 基于 Rust 重写 io_uring 接口 [https://github.com/tokio-rs/io-uring](https://github.com/tokio-rs/io-uring)

---

### 《Rust 唠嗑室》第 23 期 - Rust From A Haskeller's View

**时间**: 2021/04/27 20:30-21:30

**主讲人**：火锅 boy 大冬冬

**内容**：Rust From A Haskller's view

从资深 Haskell 程序员的视角，试图了解 Rust 这门年轻的语言，除了从技术的角度和 Haskell 进行对比之外，也分享了关于两个语言社区发展的思考和展望

[查看回放](https://www.bilibili.com/video/BV18h411m7Gf)

---


# 【线上】RustFriday 飞书群线上沙龙

每周五晚八点，限定两个主题：语言特性和开源项目，在线讨论。

Rust 中文社群 飞书群 邀请你加入：

对话群： https://applink.feishu.cn/TeLAcbDR 
话题群：https://applink.feishu.cn/TeLD868w


## 第三期讨论主题： Rust 语言中级中间语言 MIR 的功用

1. 介绍 MIR 与 MIRI
3. 和论文的两位第一作者共同阅读2021年新出炉的论文：《 SafeDrop：通过静态数据流分析检测 Rust 程序中的内存释放错误》

参考资料：

1.  https://rustc-dev-guide.rust-lang.org/mir/dataflow.html
2. https://github.com/rust-lang/miri
3. https://arxiv.org/pdf/2103.15420

[查看回放](https://www.bilibili.com/video/BV1nU4y1h7NN/)

## 第四期 讨论主题： 

1.  Rust 今天新合并的 RFC : try-trait
2.  Lunatic： 一个类似于 Erlang 的 容错 WASM 运行时

[查看回放](https://www.bilibili.com/video/BV1xy4y147Ve/)

## 第五期 讨论主题：

1. 语言特性： 《Rust 标准库 trait 之旅》
2. 领域项目：通过WebSocket实时共享应用程序状态框架 Aper

参考资料：

1. https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md
2. https://github.com/aper-dev/aper
3. https://aper.dev/guide/

[查看回放](https://www.bilibili.com/video/BV1f5411c7qg/)


---

<center> 🔥🔥🔥🔥 <strong>Rust MeetUp</strong> 🔥🔥🔥🔥 </center>

# 【线下】Rust MeetUp 本月汇总

- 来源：[Rust 活动](https://space.bilibili.com/25566598/video)
- 上传者：MikeTang
- 后期编辑：高宪凤

---

### Rust Meetup 北京站

**时间**: 2021/04/10

**地点**：北京中关村创业大街

#### 【P1】 构建安全高性能的网络应用

**嘉宾**：陈天

[查看回放](https://www.bilibili.com/video/BV1R54y1b7qo?p=1)

#### 【P2】 异步化 OS : 使用 async/await 提升 10 倍性能

**嘉宾**：蚂蚁集团 田洪亮

[查看回放](https://www.bilibili.com/video/BV1R54y1b7qo?p=2)

#### 【P3】 Rust和高性能隐私计算

**嘉宾**：数牍科技

[查看回放](https://www.bilibili.com/video/BV1R54y1b7qo?p=3)

#### 【P4】 用Rust实现RDMA编程

**嘉宾**：王璞 @ Datenlord

[查看回放](https://www.bilibili.com/video/BV1R54y1b7qo?p=4)

#### 【P5】 统一相似的异步和同步代码

**嘉宾**：吕国立

[查看回放](https://www.bilibili.com/video/BV1R54y1b7qo?p=5)

#### 【P6】 Unicode标识符简介

**嘉宾**：CRLF0710

[查看回放](https://www.bilibili.com/video/BV1R54y1b7qo?p=6)

#### 【P7】 线性类型映射世界

**嘉宾**：jolestar

[查看回放](https://www.bilibili.com/video/BV1R54y1b7qo?p=7)

---

### Rust Meetup 成都站

**时间**: 2021/04/27

**地点**：成都天府软件园

#### 【P1】 The Future of Embedded System Programming

**嘉宾**：张奕

[查看回放](https://www.bilibili.com/video/BV1Y54y1j7za?p=1)

#### 【P2】 亚马逊教你轻松开发安全隔离的虚拟机

**嘉宾**：孙华

[查看回放](https://www.bilibili.com/video/BV1Y54y1j7za?p=2)

#### 【P3】 从Rust在金融系统的应用到通用事件门溯源模型

**嘉宾**：张宇

[查看回放](https://www.bilibili.com/video/BV1Y54y1j7za?p=3)

#### 【P4】 md5 编码器 live coding

**嘉宾**：吴翱翔

[查看回放](https://www.bilibili.com/video/BV1Y54y1j7za?p=4)