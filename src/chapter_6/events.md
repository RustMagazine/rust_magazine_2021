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

### 《Rust 唠嗑室》第 26 期 - 利用 Rust 构造 riscv asm 分析工程

**时间**: 2021/06/8 20:30-21:30

**主讲人**：lyzh

**内容**：

1. 如何利用 Pest 库做文法分析，
2. 用 PyO3 与 Python 交互
3. 一些简单的数据流分析方法

[查看回放](https://www.bilibili.com/video/BV1UB4y1T7PB)

---

### 《Rust 唠嗑室》第 27 期-Michael Yuan-WasmEdge 介绍使用 Wasm Reactor 开发飞书机器人

**时间**: 2021/06/23 20:30-21:30

**主讲人**：Michael Yuan@WasmEdge

**内容**：

SaaS 平台通常会为开发者和客户提供 API 以扩展其核心功能，比如 JIRA 的自定义插件或 Slack 上的聊天机器人。但是，传统的 RESTful API 方法通常需要开发者设置服务器来接收和响应来自 SaaS 平台的事件。例如，在 飞书聊天机器人应用程序中，开发者需要设置一个服务器来监听发送给机器人的消息，然后将机器人的响应发送回 Slack。这对开发人员来说既乏味又昂贵。

随着 serverless 计算的进步，尤其是由 WebAssembly 等新运行时启用的轻量级无服务器功能，现在可以将反应式功能直接嵌入到 SaaS 平台中，而无需让创建任何新服务器。

这次将会与大家讨论 SaaS 平台中反应式 serverless 函数背后的想法、用例和应用程序架构。

参考资料：

1. https://github.com/WasmEdge/WasmEdge
2. https://github.com/second-state/runw
3. https://github.com/second-state/crunw
4. http://reactor.secondstate.info
5. http://reactor.secondstate.info/en/docs/user-create-a-bot.html
6. https://github.com/second-state/serverless-reactor-starter

[查看回放](https://www.bilibili.com/video/BV16b4y1C7B5)

---

<center> 🔥🔥🔥🔥 <strong>RustFriday 飞书群线上沙龙</strong> 🔥🔥🔥🔥 </center>

# 【线上】RustFriday 飞书群线上沙龙

每周五晚八点，限定两个主题：语言特性和开源项目，在线讨论。

Rust 中文社群 飞书群 邀请你加入：

对话群： [https://applink.feishu.cn/TeLAcbDR](https://applink.feishu.cn/TeLAcbDR)

话题群：[https://applink.feishu.cn/TeLD868w](https://applink.feishu.cn/TeLD868w)

视频来源：[https://space.bilibili.com/24917186](https://space.bilibili.com/24917186)

## 第十期讨论主题：

1. 语言特性： 聊聊 Rust 类型系统和 特质（trait）系统
2. 领域项目： 继续跟随 Linux 基金会在线 WebAssembly 课程学习 Host 和 guest 高级通信方式：WAPC

参考资料：

1. https://github.com/wasmCloud
2. https://github.com/waPc
3. https://crates.io/crates/wasmtime-provider
4. https://www.infoq.cn/article/bc0fzghd9s6fmm03pmbi

[查看回放](https://www.bilibili.com/video/BV1Lo4y1278N)

---

## Rust 中文社区飞书群线上直播：尝试 websocket 聊天的桌面端开发

1. 考察 tauri 的可行性和易用性
2. gtk glade layout Editor 工具编辑 ListView
3. 学习 TreeView 追加元素的代码
4. 完成类似微信/飞书的聊天 UI 界面
5. 完成文本输入框和点击发送按钮更新聊天消息的功能

[查看回放](https://www.bilibili.com/video/BV1vV411x7fe)

---

## 第十一期 讨论主题：

1. 三个 Rust Friday 线上沙龙，也就是三个周五晚上的时间，学习了 Linux 基金会的免费 Web Assembly 课程，感觉收获颇丰。（沙龙 九/十/十一期）

2. 课程的难度适中，重点在于比较系统。很多细节其实是需要自己去学习和了解的，但是课程为你布置好了体系结构。一些关键的概念都讲到了。可以让对 wasm 在分布式和边缘计算方向有一个比较具象的认识。

3. 最有意思的是作者针对 Web Assembly 现状，提出的 wa PC 协议和 wasmcloud 项目，都很有创新精神。

参考资料：

1. https://wasmcloud.com/
2. https://lunatic.solutions/
3. https://crates.io/crates/nats

[查看回放](https://www.bilibili.com/video/BV17h411a7xq)

---

## 第十二期 讨论主题：

1. 聊聊 eBPF 技术。
2. 聊聊 Rocket web 框架设计。

参考资料：

1. https://jishuin.proginn.com/p/763bfbd4c155
2. https://doc.rust-lang.org/rustc/platform-support.html#tier-3
3. https://security.tencent.com/index.php/blog/msg/124
4. https://www.anquanke.com/post/id/221545
5. https://github.com/alessandrod/aya
6. https://www.infinyon.com/blog/2021/05/ebpf-routing-rust/
7. https://github.com/nacardin/ebpf-proxy

[查看回放](https://www.bilibili.com/video/BV1hq4y1L7GE)
