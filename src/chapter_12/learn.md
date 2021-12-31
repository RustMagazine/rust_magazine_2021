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

