# 推荐项目 |  框架引擎

编辑：张汉东

---

## Hitbox：异步分布式缓存框架

Hitbox是一个异步缓存框架，支持多个后端，适用于分布式和单机应用程序。 目前 Actix 已经集成了该框架。

后端支持 Redis ，该框架还在逐步完善中。

[https://github.com/hit-box/hitbox](https://github.com/hit-box/hitbox)


## Neuronika: Rust 新的机器学习框架 

随着 Rust 语言热度的强劲增长，也有许多研究者探索 Rust 在机器学习等方面的研究。近日，有研究者用 Rust 编写了一个机器学习框架 Neuronika。

Neuronika 是由 Francesco Iannelli 等人开发，他们现在是计算机科学的硕士生。该框架提供了自动微分和动态神经网络，非常像 Pytorch。

- [https://github.com/neuronika/neuronika](https://github.com/neuronika/neuronika)
- [https://zhuanlan.zhihu.com/p/381854038](https://zhuanlan.zhihu.com/p/381854038)


## Thruster发布v1.1.2

Thruster，一个旨在让开发者在项目和团队之间保持高效一致的快速、直观的 Rust Web 框架。

v1.1.2版本的一些亮点：

- 完全重建的路由树和解析器；
- 完全重建的内部“中间件”系统，获取中间件功能列表并使用闭包将它们组合起来；
- 更容易使用的中间件宏；
- 升级依赖库；

在生产中使用 Thruster 的一些有趣指标：

- 在过去的四个月里，它处理了 240 万个请求；
- 实例中没有（0%） 与框架相关的致命错误；
- 实例中95%的平均响应时间远低于 25 毫秒；
- 每个实例的平均 RAM 为 14.75MB；

版本详情参见Release，[https://github.com/thruster-rs/Thruster/blob/master/RELEASE.md](https://github.com/thruster-rs/Thruster/blob/master/RELEASE.md)

## tantivy: v0.15 发布了


tantivy 是 Rust 写的全文搜索引擎库. 现在tantivy 由 quickwit-inc 开发托管.

[https://github.com/quickwit-inc/tantivy](https://github.com/quickwit-inc/tantivy)

## messages: 异步 actor 框架

messages 是一个异步的 actor 框架, 受 actix 启发.

[https://github.com/popzxc/messages-rs](https://github.com/popzxc/messages-rs)

## 发布rg3d 0.20，一个功能丰富且易于使用的游戏引擎

rg3d 0.20 已经发布。此版本包含许多新功能和改进。最大功能的是 WebAssembly 支持、初始 2D 支持和多层地形。

- 文章链接 [https://rg3d.rs/general/2021/06/11/rg3d-0.20-progress.html ](https://rg3d.rs/general/2021/06/11/rg3d-0.20-progress.html )
- [https://github.com/rg3dengine/rg3d](https://github.com/rg3dengine/rg3d)

## Wasmer 2.0 发布！

WebAssembly运行时Wasmer 2.0发布了：运行时速度快了50+%，热启动速度快了70+%，还有诸如引用类型、SIMD等更多新功能发布！

wasmer-2.0 发布：[https://wasmer.io/posts/wasmer-2.0](https://wasmer.io/posts/wasmer-2.0)

## Walleye：国际象棋引擎

Walleye 是一种使用经典 alpha-beta 风格 AI 编写的与 UCI 兼容的引擎。 支持从任意 FEN 字符串加载棋盘位置、Unicode 漂亮地打印到控制台和帮助调试的 UCI 通信日志。注意，这是引擎，如果要玩儿的话要加 GUI，比如已经做过测试的 [Cute Chess(https://cutechess.com/)]。

[https://github.com/MitchelPaulin/Walleye](https://github.com/MitchelPaulin/Walleye)

注：FEN 是 Forsyth-Edwards Notation 的简称，用来描述棋盘位置，详见[维基百科](https://www.chessprogramming.org/Forsyth-Edwards_Notation)。