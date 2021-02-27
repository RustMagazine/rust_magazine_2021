# 本月简报 | 推荐项目

- 来源：[Rust日报](https://rustcc.cn/section?id=f4703117-7e6b-4caf-aa22-a3ad3db6898f)
- 作者：`Rust`日报小组

## Rust-SQLite

- [仓库链接](https://github.com/joaoh82/rust_sqlite)

Rust-SQLite (SQLRite) 是一个 SQLite clone。SQLRite 有很完善的文档，代码质量非常高，而且有非常丰富的单元测试用例。

## Tauri

- [仓库链接](https://github.com/tauri-apps/tauri)
- [项目主页](https://tauri.studio/en/)

Tauri 是一个桌面应用开发框架，包含了 JavaScript API，可以结合各种主流前端框架进行开发。

[有 Twitter 网友分享](https://twitter.com/victorhqc/status/1356990383792791555)，
他把自己的 Electron 写的应用迁移至 Rust 的 Tauri，
内存使用从 300M 降低至 6M，二进制大小从 195M 降至 7M。

## RustPython

- [仓库链接](https://github.com/RustPython/RustPython) 
- [FOSDEM 2019 上的相关演讲](https://www.youtube.com/watch?v=nJDY9ASuiLc)

RustPython 是用 Rust 实现的 Python 3（CPython >= 3.8.0） 解释器。 RustPython 可以将 Python 嵌入到 Rust 程序中；也可以将 RustPython 编译为 WebAssembly，这样开发者可以在浏览器中运行其 Python 代码。此外，RustPython 也包含一个实验性的 JIT 编译器。

## Thirtyfour

- [仓库链接](https://github.com/stevepryde/thirtyfour)

Thirtyfour 是一个 Selenium WebDriver 客户端，可以用于自动化 UI 测试。Thirtyfour 完全支持 W2C WebDriver spec，可以搭配 tokio 或者 async-std 使用。

## Lunatic

- [仓库链接](https://github.com/lunatic-solutions/lunatic)
- [项目主页](https://lunatic.solutions/)

Lunatic 是一个服务端的 WebAssembly 运行时，有以下特点：

- 受到 Erlang 的启发，有一个抢占式调度的运行时, 生成占用资源极少的用户态线程。
- 借助 wasm 虚拟机，保证隔离和安全性。
- 会在未来完全兼容 WASI 

## Postage

- [仓库链接](https://github.com/austinjones/postage-rs) 
- [文章链接](https://implaustin.hashnode.dev/announcing-postage)

Postage 是一个异步通道库，提供了丰富的通道集，并在 Sink/Stream 上有很多实用的组合子，方便了异步程序的开发。

作者同时也是 [tab](https://github.com/austinjones/tab-rs/) 的作者。

## RustSBI

- [仓库链接](https://github.com/luojia65/rustsbi)

RustSBI 是洛佳老师开发的一个 RISC-V SBI 实现，支持常见的硬件核心和模拟器，能够引导启动符合 RISC-V SBI 标准的操作系统，包括 Linux、rCore 等。

## Similar

- [仓库链接](https://github.com/mitsuhiko/similar)

similar 是一个现代化的 diff 库，借鉴了 [pijul](https://pijul.org/) 实现的耐心排序算法，并结合了 Myer 的 diff 算法。

## tantivy

- [仓库链接](https://github.com/tantivy-search/tantivy)

tantivy 是一个全文搜索引擎库, 类似于 Apache Lucene。

## xh

- [仓库链接](https://github.com/ducaale/xh)

xh 是一个 Httpie clone。

![ht](https://github.com/ducaale/xh/raw/master/assets/xh-demo.gif)

## meio

- [仓库链接](https://github.com/rillrate/meio)

meio 是一个异步 actor 框架，其设计受 Erlang/OTP 启发，并可以很好地结合 rust 中的异步生态系统使用。作者正在尝试使其能 WebAssembly 兼容。

## message-io

- [仓库链接](https://github.com/lemunozm/message-io)

message-io 是一个是事件驱动的消息库，可轻松快速地构建网络应用程序。message-io 可以管理和处理套接字数据流，以便向用户提供简单的事件消息 API。作为通用网络管理器，它允许你遵循一些规则来实现自己的协议，而繁琐的异步和线程管理则由 message-io 帮你管理。 

## Cranelift

- [仓库链接](https://github.com/bytecodealliance/wasmtime/tree/main/cranelift)
- [文章链接](https://blog.benj.me/2021/02/17/cranelift-codegen-primer/)

Cranelift 是用 Rust 编程语言实现的代码生成器，旨在成为快速的代码生成器，其输出以合理速度运行的机器代码。
如今，它被用于包括 Wasmtime 和 Wasmer 在内的几种不同的 WebAssembly 运行时中，并且还可以作为 Rust 调试编译的替代后端。

## Voyager

- [仓库链接](https://github.com/mattsse/voyager)

voyager 是一个用 Rust 实现的爬虫库。

## Starlight

- [仓库链接](https://github.com/Starlight-JS/Starlight)
- [reddit 链接](https://github.com/Starlight-JS/Starlight)

Starlight 是一个 JavaScript 的运行时，其设计重点放在运行速度上，已经通过了 2k+test262 测试。Starlight 比 Boa（另一个Rust写的JS引擎）更快，其目标是和V8一样快。

## Moonzone

- [仓库链接](https://github.com/MoonZoon/MoonZoon)

MoonZoon 一个 Rust 全栈框架。维护这个框架的作者是seed的主要维护者，所以，理所当然地在前端部分选择了seed。

## Lettre

- [仓库链接](https://github.com/lettre/lettre)

Lettre 是一个可以用于发送 email 的库。
