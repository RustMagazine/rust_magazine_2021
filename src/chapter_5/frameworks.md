# 推荐项目 |  框架引擎

## Cube.js: Rust 实现的 BI 框架

Cube.js 是一个开源的分析 API 平台。主要用于构建内部商业智能工具或将客户面向客户的分析添加到现有应用程序。

你可以使用 Cube.js 构建现代化的数据分析平台。

架构： Data(DB) -> Backend (Cube.js server) -> Frontend(React.js / Ant Design)

在最简单的方案中，Cube.js使用内存高速缓存和查询队列来提供比数据存储能够传递的更好的性能。 但是，它通常是最不可扩展且经济高效的解决方案，因此不建议生产。

因此引入 Cube Store，以提供有保证的高并发性和亚秒级延迟，用于分析查询的性能优化以及通过跨数据库连接的数据联合等附加功能。 Cube Store 由 Rust 实现。


- [Cube.js](https://github.com/cube-js/cube.js)
- [Docs](https://cube.dev/docs/introduction)
- [Cube Store 更多介绍](https://dev.to/cubejs/introducing-cube-store-high-concurrency-and-sub-second-latency-for-any-database-3n6n)


## Warp: Web server 框架

Warp 构建于 hyper 之上。 特色是：Filter 系统。

**tower vs wrap vs tower-web **

- tower，类似于 Finagle （Twitter 研发的RPC系统）
- warp，类似于 finch（用于构建Finagle HTTP服务的Scala组合器库）
- tower-web，类似于 finatra （一个scala 异步 web 框架） 

warp 的 Filter 抽象 和 Tower-web 的 Middleware 抽象 有异曲同工之效。[阅读更多](https://github.com/seanmonstar/warp/issues/58)。

- [warp](https://github.com/seanmonstar/warp)
- [tower-web](https://github.com/carllerche/tower-web)

目前 Warp 还在积极维护中。

## Rocket 最近发布了 0.4.10

Rocket 的更新节奏比较缓慢，但还在维护中，五月份连续三天连发三个小版本。

最新的版本更新是移除了一处 unsafe 代码，修复了一个 soundless 的问题。

[https://github.com/SergioBenitez/Rocket](https://github.com/SergioBenitez/Rocket)

## sqlx: 发布了新版本 0.5.5

sqlx 是一个纯 Rust 的异步 SQL 库。它的特点是编译期查询检查，并且没有提供任何 DSL，所以它不是一个 ORM。

sqlx 跨数据库/跨运行时/跨TLS后端，不绑定于任何特定数据库和运行时(async-std / tokio / actix)和TLS后端（native-tls, rustls）。

[sqlx](https://github.com/launchbadge/sqlx)


## MoonZoon: 全栈 Rust 开发框架进展

MoonZoon 号称是 Rust 全栈框架。

最近 MoonZoon 好像取得了一些进展。增加了一个 光线追踪的教程：[https://github.com/MartinKavik/ray_tracer/blob/main/README.md](https://github.com/MartinKavik/ray_tracer/blob/main/README.md)。MoonZoon 支持 HTML canvas 和 后端自动加载，支持快速开发。

[https://github.com/MoonZoon/MoonZoon](https://github.com/MoonZoon/MoonZoon)