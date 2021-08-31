# 推荐项目 |  框架引擎

编辑： 张汉东

---

## Poem是一个基于tokio/hyper的WEB服务端框架

以下为作者自述：

为什么要做一个新的web框架：

- actix-web目前仅维护状态，和tokio1兼容的新版本始终出不来（就算出来和其它生态结合也有问题，因为用得单线程runtime）
- tide也和 actix-web 类似
- warp太复杂，就算是一个rust老手也会经常被整的死去活来
- axum目前比较火，但是在我深入研究之后，发现也存在和warp同样的问题。
- 在和社区的朋友聊axum的过程中，发现大家都不太玩得明白，我突然就想做一个用起来简单点的。

Poem简单在哪里：

warp 复杂在于大量的泛型造成代码难以理解，甚至连IDE都无法正确识别类型造成编码的困难。

但简单不代表没有泛型，而是去掉一些不必要的泛型。

Poem在对性能影响不大的地方尽量减少泛型的使用，定义IDE友好，容易理解的API。

Poem的当前状态：

完全覆盖warp的功能，API已经基本稳定。

Poem的后续目标

- 更完善的文档以及使用手册。
- 覆盖更全面的测试用例。
- 提供更多开箱即用的功能。
- 内置openapi(swagger)的支持。

感谢：

感谢张汉东提供的Poem注册名，小丁制作的网站，以及社区各位朋友提供的意见和PR。

[https://github.com/poem-web/poem](https://github.com/poem-web/poem)

## pgx : 方便用 Rust 扩展 PostgreSQL 的框架

pgx 是一个在 Rust 中开发 PostgreSQL 扩展的框架，并力求尽可能地惯用和安全。

特点：

- 通过一系列 cargo 子命令来管理开放环境。
- 支持 Postgres v10、v11、v12 和 v13。
- 使用Rust features 来使用特定版本的API。
- 自动生成 Schema。
- 为常见的SQL对象生成DDL。
- 安全第一：将Rust的恐慌转化为Postgres的ERROR，中止事务，而不是中止进程。

[https://github.com/zombodb/pgx](https://github.com/zombodb/pgx)


## rg3d 游戏引擎发布v0.22 - 同时发布了一个展示版本特色的视屏

rg3d 最近发布了 0.22 版，作者在 Reddit 发布了一段 rg3d 的演示视频，看起来非常棒！可能是目前最成熟的一个 Rust 3D 游戏引擎了。

视频：[https://www.youtube.com/watch?v=N8kmZ9aBtZs](https://www.youtube.com/watch?v=N8kmZ9aBtZs)

[https://github.com/rg3dengine/rg3d](https://github.com/rg3dengine/rg3d)


## Rust Search Extension 1.3 发布

小编的 Rust 插件发布新版本半个多月了，一直忘了发帖了，今天补上。欢迎大家体验！

更新说明：https://github.com/huhu/rust-search-extension/wiki/V1.3.0-Release-Candidate-(zh_CN)

Changelog: https://rust.extension.sh/changelog


