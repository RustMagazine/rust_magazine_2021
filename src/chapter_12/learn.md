# 学习资源

聚焦学习 Rust 的网络资源

---

## Rust 编译为什么慢？

本文作者使用的硬件很高端，`AMD Ryzen9 5950X CPU` 32 核，`128`GB内存 和 `SSD` 硬盘。理论上编译 Rust 项目是非常快的。

他的项目在这个硬件配置下编译时间最多只有两秒零九。（但是作者还觉得 Rust 编译很慢）

所以作者开始探索 Rust 编译时候到底在干什么。

[https://fasterthanli.me/articles/why-is-my-rust-build-so-slow](https://fasterthanli.me/articles/why-is-my-rust-build-so-slow)

## 用 Rust 和 WebAssembly 在浏览器中绘制“甜甜圈”圆环

该demo 作者分享给想进入计算机图形学新手的学习路线：

> 如果您有兴趣深入研究图形世界，请查看[学习 OpenGL](https://learnopengl.com/)。我经历这件事的速度很慢——主要是因为工作，不知道 C++，有编程之外的生活，需要边学习线性代数；但是一旦我完成了“入门”部分，我就有了足够的知识来学习 WebGL，并且知道使用什么方程式来实现基本的变换（旋转、平移和缩放）和相机移动。
>
> 如果您不了解 C++，请不要被它吓倒，因为如果您能阅读 Rust，您就可以阅读作者在书中写的 C++。为了与 OpenGL 交互，我使用了[glium](https://github.com/glium/glium)；不过，将 C++ 代码翻译成 Rust/glium 可能很棘手。与作者使用的线性代数库 (GLM) 等效的 Rust 是[nalgebra_glm](https://docs.rs/nalgebra-glm/latest/nalgebra_glm/)。
>
> 现在，如果您不知道线性代数，那会有点困难，但是，那里有大量资源，它们基本上概述了在图形中产生生产力所需的最少线性代数。

作者对 Rust 和 WebAssembly 工具链的看法：

> 我还没有完全阐明我的观点，但对我来说最大的吸引力当然是能够用 Rust 编写浏览器代码并以 WASM 为目标，这非常安全、小巧且高效。然而，也有缺点，其中大部分在于开发人员的体验。
>
> Rust + WASM 工具链还很年轻，生态系统还很不成熟，这意味着在学习最佳实践、如何启动项目以及如何将 Rust + WASM 引入时，那里的资源稀缺现有项目。
>
> 然而，在一天结束时，我很高兴我可以用 JavaScript 以外的其他东西编写严格的浏览器代码，让该语言编译为 JavaScript 以外的东西（看看你的 TypeScript），并且可以肯定，我得到的二进制文件是小巧、安全、高效。

- 在线 Demo： [https://parametric-surfaces.herokuapp.com/](https://parametric-surfaces.herokuapp.com/)
- 源码：[https://github.com/solidiquis/parametric_surfaces](https://github.com/solidiquis/parametric_surfaces)

## 用 egui & Wasm 学习 Rust，做了一个小的在线进制转换工具作为圣诞节项目

作者并没有分享源码，但是分享了他学习 egui 的经验：

> 对我来说，使用 egui 非常简单，GitHub 上有一个模板项目“eframe_template”（由 egui 作者 emilk 维护），它可以轻松编译本机和 Wasm。因此，针对本机后端进行开发并最终在需要时为 Wasm 构建是一个非常快速的周转。虽然我也简要地研究过 imgui-rs，我还没有尝试过，所以我无法比较这两个，但根据我的理解 imgui-rs / Dear ImGui 功能更丰富，但是 egui 的视觉效果和简单性更吸引我，并且我还想从一个 Rust 原生库开始，尽可能少的依赖，所以我选择了 egui。说到功能，我发现 egui 开箱即用，非常适合各种本机工具，对于 Web 应用程序，它在桌面上运行良好，在移动设备上，体验可能会好一点，例如，我在移动设备上需要的 UI 缩放方面有点挣扎，这可以通过使用样式系统（顺便说一句，功能强大）以某种方式解决，但这需要一些额外的工作，所以我很好奇 egui 在这方面将如何发展。在任何情况下，如果 Wasm 是主要目标，我会考虑使用 iced lib，它使用本机 Web DOM，这对于传统的 Web 体验和集成来说绝对更好。

[https://apps.4fips.com/nubco/](https://apps.4fips.com/nubco/)

## 使用 cargo-udeps 检查 Rust 项目中未使用的依赖项

cargo-udeps 是一个了不起的项目，可帮助您分析 Rust 代码库上未使用的依赖项。您可以简单cargo install cargo-udeps地安装它。 

1、安装

```rust
cargo install cargo-udeps --locked
```

2、使用

```rust
cargo +nightly udeps
```

3、忽略依赖项 要忽略某些依赖项，请添加package.metadata.cargo-udeps.ignore到Cargo.toml.

```rust
[package.metadata.cargo-udeps.ignore]
normal = ["if_chain"]
#development = []
#build = []

[dependencies]
if_chain = "1.0.0" # Used only in doc-tests, which `cargo-udeps` cannot check.
https://erayerdin.com/checking-unused-dependencies-in-a-rust-project-with-github-actions-ckwm3yov901cwlvs1h48z54xi
```

## 使用 rg3d 游戏引擎进行游戏开发

Rg3d, Rust Game engine 3D and 2D, 一个用 Rust 编写的功能丰富、生产环境就绪、通用的 2D/3D 游戏引擎，并带有场景编辑器。

- [rg3d Github](https://github.com/rg3dengine/rg3d)
- [Youtube 回放](https://youtu.be/TQaCyC_tGko)

## 用 Vim 写 Rust 代码

Neovim 是 vim 的一个分支，它专注于可扩展性和可用性。比如，能够使用 Lua 以及各种脚本来编写插件，并为扩展编辑器提供更大的灵活性。

在 Neovim 0.5 版本中，开发者引入了语言服务器协议 (LSP) 客户端框架 (:help lsp)

这意味着，Neovim 可以充当 LSP 服务器（如 rust-analyzer）的客户端，并协助构建增强的 LSP 工具。

下面这篇文章就详细说明了 如何为 Neovim 配置 Rust 插件，可以省去很多自己折腾的时间。

[https://sharksforarms.dev/posts/neovim-rust/](https://sharksforarms.dev/posts/neovim-rust/)


## 为 Rust 借用检查建模

这篇文章通过为 Rust 借用检查建模，探索 Rust 借用检查的工作机制，这个思考过程值得学习。

[https://whileydave.com/2021/12/06/modelling-borrow-checking-in-rust/](https://whileydave.com/2021/12/06/modelling-borrow-checking-in-rust/)

## Rust 错误处理

一篇介绍 Rust 错误处理的新文章

[https://www.unwoundstack.com/blog/rust-error-handling.html](https://www.unwoundstack.com/blog/rust-error-handling.html)

## 用 Nom 在 Rust 中构建一个 CEDICT 解析器

CEDICT 格式是一种简单的、创造性的、通用许可的中/英词典文件格式。虽然有很多只支持普通话的CEDICT解析器，但在英语编程世界中，基本上不支持粤语的jyutping。作为一个希望在节目中使用广东话发音的人，一开始作者被困住了。最终,作者自己动手写了一个解析器.

[https://briankung.dev/2021/12/07/building-a-cedict-parser-in-rust-with-nom/](https://briankung.dev/2021/12/07/building-a-cedict-parser-in-rust-with-nom/)

## Embark Studio : 使用 Rust 进行本地渲染

作为 Rust 的忠实拥护者，Embark Studio 的工程师们正在使用 Rust 开发渲染引擎和游戏，并且贡献了 rust-gpu 这个项目。

在这篇技术博客中，一位来自 Embark 的工程师讲述了如何处理用户创建的世界的 3D 渲染，以及 Rust 如何帮助他们更好地实现这个目标。

并且宣布开源了实验性全局光照渲染引擎：[kajiya](https://github.com/EmbarkStudios/kajiya)

[https://medium.com/embarkstudios/homegrown-rendering-with-rust-1e39068e56a7](https://medium.com/embarkstudios/homegrown-rendering-with-rust-1e39068e56a7)

## 关于Rust中上下文和能力的思考

今天早些时候，我阅读了Tyler Mandry的《Rust中的上下文和能力》，我非常喜欢我看到的内容。但是我和很多人一样，有很多问题。这将如何影响语言？这是否会损害定位性/可读性？需要什么来使之适应？

所以这里是我到目前为止的想法的探索。

[https://jam1.re/blog/thoughts-on-contexts-and-capabilities-in-rust](https://jam1.re/blog/thoughts-on-contexts-and-capabilities-in-rust)

## 无缓冲 I/O 会使您的 Rust 程序变慢

Rust 作为一种开发语言已经确立让开发人员能够编写快速和安全的代码的声誉现在。像 Mozilla、Microsoft、Dropbox 和 Amazon（仅举几例）这样的大型组织都依赖 Rust 为他们的客户提供一流的性能，同时避免许多影响用 C 或 C++ 编写的程序的安全问题。性能是 Rust 成为大多数人选择语言的一个主要原因。然而，仅仅用 Rust 编写代码并不能保证高性能。Rust 是好的，但它不是魔法。它是一种工具，与任何工具一样，我们必须有效地使用它才能获得最佳结果。在这篇文章中，我们将研究 Rust 代码性能不佳的常见原因，即使是资深开发人员也可能会遇到这种情况。也就是说，默认情况下，不缓冲文件的读取和写入。

[https://era.co/blog/unbuffered-io-slows-rust-programs](https://era.co/blog/unbuffered-io-slows-rust-programs)

## 在没有 Docker 的情况下在 macOS 上交叉编译 Rust Lambdas

在 Rust 中开发 Lambda 函数的标准方法是使用 AWS 提供的自定义 Lambda 运行时并在部署之前交叉编译所有内容。这里介绍如何在 macos 上进行开发 Lambda 函数。

```
brew tap messense/macos-cross-toolchains
brew install aarch64-unknown-linux-gnu
```

然后，在您的环境中设置这些变量（例如在bashrc 中）：

```
export CC_aarch64_unknown_linux_gnu=aarch64-unknown-linux-gnu-gcc
export CXX_aarch64_unknown_linux_gnu=aarch64-unknown-linux-gnu-g++
export AR_aarch64_unknown_linux_gnu=aarch64-unknown-linux-gnu-ar
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-unknown-linux-gnu-gcc
```

现在cargo build --target aarch64-unknown-linux-gnu将在 macOS 上按预期工作，并生成准备部署到 AWS Lambda 的二进制文件 - 无需 Docker

[https://noserve.rs/rust-lambdas-macos/](https://noserve.rs/rust-lambdas-macos/)

## Rust 安全可移植的数据结构设计

这是来自 Code and Supply Lighting Talk 的一个演讲, 主题是如何 设计一个数据结构, 他能够在任意的系统和任意的嵌入式设备.

- [https://www.youtube.com/watch?v=1UtklNrB8XA](https://www.youtube.com/watch?v=1UtklNrB8XA)
- [https://tiemoko.com/slides/SafeAndPortableDataStructureDesign_CodeAndSupply_Dec2021.pdf](https://tiemoko.com/slides/SafeAndPortableDataStructureDesign_CodeAndSupply_Dec2021.pdf)

## 使用 Rust 编写prometheus exporter

作者详细描述了自己如何使用 Rust 编写 prometheus的 exporter 到存储和画图的过程。这对于了解 Prometheus 导出器工作原理有帮助。

[https://mateusfreira.github.io/@mateusfreira-writing-a-prometheus-exporter-in-rust-from-idea-to-grafana-chart/](https://mateusfreira.github.io/@mateusfreira-writing-a-prometheus-exporter-in-rust-from-idea-to-grafana-chart/)

## 在Rust中模拟HTTP服务

[Mocking HTTP Services in Rust](https://dev.to/alexliesenfeld/mocking-http-services-in-rust-58ee)

本文展示了如何使用mock库来模拟HTTP服务。如何使我们在自动化测试期间验证应用程序发送的HTTP请求是否符合我们的期望，并且还可以模拟来自依赖服务的HTTP响应，以确保我们的应用程序能够相应地处理它们。此外，作者还展示了模拟工具如何在开发过程中替换不可用的HTTP服务，并使它们能够同时被许多应用程序访问。

多功能模拟工具可以在开发生命周期的多个阶段中实用，而不仅仅是集成测试。然而，它们对于增强基于HTTP的API客户端特别有用，并允许我们测试很难复现的边界case。

作者介绍了一些可以做mock的开源库，并做了对比：

| Library  | Execution | Custom Matchers | Mockable APIs | Sync API | Async API | Stand-alone Mode |
| -------- | --------- | --------------- | ------------- | -------- | --------- | ---------------- |
| mockito  | serial    | no              | 1             | yes      | no        | no               |
| httpmock | parallel  | yes             | ∞             | yes      | yes       | yes              |
| httptest | parallel  | yes             | ∞             | yes      | no        | no               |
| wiremock | parallel  | yes             | ∞             | no       | yes       | no               |



## 《在Rust中制作游戏》系列教程

> 现在我们有了一种语言：Rust，一种游戏引擎：Bevy，还有一种类型：Platformer。 这一系列文章将是我用这些工具构建一个小型平台游戏之旅的日志。

- [在Rust中制作游戏.第1部分.Bevy和ECS](https://dev.to/sbelzile/rust-platformer-part-1-bevy-and-ecs-2pci)
- [在Rust中制作游戏.第2部分.绘图材料和照相机](https://dev.to/sbelzile/making-games-in-rust-part-2-drawing-stuff-and-cameras-1jon)
- [在Rust中制作游戏.第3部分.地板和重力](https://dev.to/sbelzile/making-games-in-rust-part-3-floors-and-gravity-3lag)
- [在Rust中制作游戏.第4部分.跳跃](https://dev.to/sbelzile/making-games-in-rust-part-4-jumps-2jne)
- [在Rust中制作游戏.第5部分.运动](https://dev.to/sbelzile/making-games-in-rust-part-5-movement-4f11)
- [在Rust中制作游戏.第6部分.生成地图](https://dev.to/sbelzile/making-games-in-rust-part-6-generating-a-map-4aic)
- [在Rust中制作游戏.第7部分.修复玩家随机困扰问题](https://dev.to/sbelzile/making-games-in-rust-part-7-fixing-the-player-randomly-stuck-issue-3mj5)
- 更新中...

## infinitree - 嵌入式加密数据库

Infinitree 是一个嵌入式加密数据库。

- 默认线程安全
- 透明地处理热/温/冷存储层；目前支持 S3 兼容的后端
- 可以使用Iterator trait查询,无需完全加载的版本化数据结构
- 加密所有磁盘数据，仅在使用时解密
- 专注于性能和灵活
- 可扩展的自定义数据类型和存储策略

[https://github.com/symmetree-labs/infinitree](https://github.com/symmetree-labs/infinitree)

## 使用 Rust 编写的生成艺术服务

chilipepperhott/generative-art 是一个全新的交互式生成艺术服务，用 Rust 编写，可以编译为 wasm 。访问 [https://elijahpotter.dev/art/](https://elijahpotter.dev/art/) 即可在线体验。

[https://github.com/chilipepperhott/generative-art](https://github.com/chilipepperhott/generative-art)

## 一行代码如何让 24 核服务器比笔记本电脑还慢

[https://pkolaczk.github.io/server-slower-than-a-laptop/](https://pkolaczk.github.io/server-slower-than-a-laptop/)

## Rust写Python扩展的9个原则

- 创建一个同时包含 Rust 和 Python 项目的仓库。
- 使用 maturin & PyO3 在 Rust 中创建 Python 可调用的翻译器函数。
- 让 Rust 翻译器函数调用 “好的” Rust 函数。
- 在 Python 中预分配内存。
- 将好的 Rust 错误处理转换为好的 Python 错误处理。
- 使用 Rayon 和 ndarray::parallel 的多线程，返回任何错误。
- 允许用户控制并行线程数。
- 将好的动态类型 Python 函数转换为好的 Rust 通用函数。
- 同时创建 Rust 和 Python 的测试。

[https://towardsdatascience.com/nine-rules-for-writing-python-extensions-in-rust-d35ea3a4ec29](https://towardsdatascience.com/nine-rules-for-writing-python-extensions-in-rust-d35ea3a4ec29)

## 了解原子和内存排序的工作原理

作者使用一个带有简单单元测试的 repo 增强对这个主题的理解。其中，包含了一个简单例子和一个复杂例子。

[https://github.com/blasrodri/atomic-story](https://github.com/blasrodri/atomic-story)

## 在 Rust 中为 RISC-V OS 实现自旋锁

自旋锁是最基本的同步实现之一，也是实现操作系统时首先要考虑的组件之一。文章将简要回顾自旋锁的基础知识，如何在 Rust 中为自制操作系统实现它，以及它相对于 C 语言的优势。

[https://vmm.dev/en/rust/spinlock.md](https://vmm.dev/en/rust/spinlock.md)

## SNAFU 0.7 发布

SNAFU 是一个库，可以在添加上下文的同时轻松地将底层错误分配到特定于域的错误中。可以类似这样处理错误：

```rust
// We support struct errors ...
#[derive(Debug, Snafu)]
#[snafu(display("An error occurred for {username}"))]
struct OneKindOfError { username: String }

// ... enum errors ...
#[derive(Debug, Snafu)]
enum AnotherKindOfError {
    #[snafu(display("Unable to finish situation one"))]
    SituationOne { source: OneKindOfError },

    #[snafu(display("Unable to finish situation two for {user_id}"))]
    SituationTwo { source: OneKindOfError, user_id: u32 },
}

// ... and opaque errors, great for exposing as part of a public API.
#[derive(Debug, Snafu)]
pub struct Error(AnotherKindOfError);
```

1 月 4 日 SNAFU 0.7 发布，更新了包括：使用结构和枚举的自定义错误类型，上下文选择器可以放在一个模块中等。

[https://users.rust-lang.org/t/snafu-0-7-released/69766](https://users.rust-lang.org/t/snafu-0-7-released/69766)

## 将 Rust 的 std 移植到 rustix

Rustix 是一个具有多个后端的系统调用包装库。它有一个原始的 Linux 系统调用后端，以及一个 libc 后端，其他后端正在开发中。Rustix 专为内存安全、I/O 安全和性能而设计。rustix 简化了与 C 整数类型大小相关的系统调用 API 中的一些小缺陷。举个例子：

```rust
let len = cmp::min(buf.len(), <wrlen_t>::MAX as usize) as wrlen_t;
   let ret = cvt(unsafe {
       c::send(self.inner.as_raw(), buf.as_ptr() as *const c_void, len, MSG_NOSIGNAL)
   })?;
```

将会变成

```rust
   let ret = rustix::net::send(&self.inner, buf, SendFlags::NOSIGNAL)?;
```

这将重点放在send操作上，而没有unsafe、原始指针、wrlen_t类型和cvt错误处理的干扰。Rustix 还能够从 Rust 代码直接进行 Linux 系统调用。比如 origin 是一个 Rust 库，它能够启动和关闭进程和线程（类似于 crt1.o 和 libpthread）。有了这些，我们就有了在 Linux 上运行 Rust 程序所需的所有东西。

[https://blog.sunfishcode.online/port-std-to-rustix/](https://blog.sunfishcode.online/port-std-to-rustix/)

## Rust 比 C 代码更容易移植

pngquant/libimagequant 的作用把这个库使用 Rust 重写了。他发现使用 Rust 写的版本，更容易移植。这个库是用来处理 png 图片压缩的其中一步的。

[https://pngquant.org/rust.html](https://pngquant.org/rust.html)

## Rust 和 Valgrind 配合使用

这篇文章讲了为什么 Valgrind 对 Rust 是有用的。

- Valgrind 不止是内存错误探测工具
- Rust 并不是完全内存安全的（因为有的时候 unsafe 代码不可避免）

[https://nnethercote.github.io/2022/01/05/rust-and-valgrind.html](https://nnethercote.github.io/2022/01/05/rust-and-valgrind.html)


## 让数字解析快 4 倍

一篇有关解析器 parser 的文章，学习。

[https://cantortrading.fi/rust_decimal_str/](https://cantortrading.fi/rust_decimal_str/)

## cargo 的 strip 指令稳定下来了

也就是裁减编译后的二进制文件体积大小。

[https://github.com/rust-lang/cargo/pull/10088](https://github.com/rust-lang/cargo/pull/10088)

## Aero - 一个现代、实验性、unix-like 的操作系统

又一个用 Rust 写的操作系统！

[https://github.com/Andy-Python-Programmer/aero](https://github.com/Andy-Python-Programmer/aero)