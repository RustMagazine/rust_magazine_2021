# 推荐项目 ｜ 基础工具库

聚焦 Rust 生态库

---

##  ouch： 压缩和解压缩工具

`ouch`代表**Obvious Unified Compression Helper**，是一个 CLI 工具，可帮助您压缩和解压缩多种格式的文件。

| 格式   | `.tar` | `.zip` | `.bz`, `.bz2` | `.gz` | `.lz4` | `.xz`, `.lzma` | `.zst` |
| ------ | ------ | ------ | ------------- | ----- | ------ | -------------- | ------ |
| 支持的 | ✓      | ✓      | ✓             | ✓     | ✓      | ✓              | ✓      |

[https://github.com/ouch-org/ouch](https://github.com/ouch-org/ouch)

## Meadowlark: 数字音频工作站

Meadowlark 是一个（目前未完成）项目，旨在成为适用于 Linux、Mac 和 Windows 的免费开源 DAW（数字音频工作站）。它的目标是成为世界各地艺术家的强大录音、作曲、编辑、声音设计、混音和母带制作工具，同时还具有直观性和可定制性。

可以关注下此项目，看看它的目标能否实现

[https://github.com/MeadowlarkDAW/Meadowlark](https://github.com/MeadowlarkDAW/Meadowlark)

## gpgpu-rs ： 基于 wgpu 实现的 GPU 计算库

wgpu 是一个安全且可移植的 GPU 抽象库，实现 WebGPU API 。gpgpu-rs 在其之上创建了一个简单的 GPU 计算库。

- [https://github.com/UpsettingBoy/gpgpu-rs](https://github.com/UpsettingBoy/gpgpu-rs)
- [https://github.com/gfx-rs/wgpu](https://github.com/gfx-rs/wgpu)

## libracity:  基于 Bevy 实现的益智游戏

LibraCity - 针尖上的城市规划！LibraCity 是一款益智游戏，您可以用一根针在平衡状态下建造一座城市。要取得成功，请利用建筑物的各种重量，并在确保城市保持稳定的同时放置它们。

[https://github.com/djeedai/libracity](https://github.com/djeedai/libracity)

## Chaos Theory ： 一款小游戏 

用 Rust 和 WASM 实现的一款 H5 小游戏，利用了一个小型的游戏引擎 [ld-game-engine](https://github.com/necauqua/ld-game-engine) 。

- [https://github.com/necauqua/chaos-theory](https://github.com/necauqua/chaos-theory)
- [在线玩：https://ld49.necauqua.dev/](https://ld49.necauqua.dev/)

## soldank: 是 Soldat 游戏的 Rust 实现

[soldat](https://soldat.pl/en/) 是 是一款独特的2D（侧视图）多人动作游戏。 它受到了Liero，Worms，Quake和Counter-Strike等最好的游戏的影响，并提供了大量的鲜血和肉体的快节奏游戏体验。soldank 是这款游戏的 Rust 实现。

[https://github.com/smokku/soldank](https://github.com/smokku/soldank)

## plotters ：快速绘图工具

Plotters 是一个绘图库，设计用于以纯 Rust 渲染图形、绘图和图表。支持各种类型的后端，包括位图、矢量图、活塞窗口、GTK/Cairo 和 WebAssembly。

在有的人看来，这仍然不是"非常"便捷，尤其是在 debug 的时候，仍然需要花费一点精力来绘制。
debug_plotter 这个库帮我们实现了这个需求，通过 plot! 宏，可以快速完成图像绘制，可视化程序运行情况。

```Rust
fn main() {
    for a in 0usize..10usize {
        let b = (a as f32 / 2.0).sin() * 10.0;
        let c = 5 - (a as i32);
        debug_plotter::plot!("My Plot"; a, b, c);
    }
}
```

[https://crates.io/crates/debug_plotter](https://crates.io/crates/debug_plotter)

## rnet - 从 .net 中调用 Rust

这个 crate 原理仍然是将rust编译成 cdylib，然后在 c# 里面调用。不过将这个过程变得更容易了一点。

[https://docs.rs/rnet/0.1.0/rnet/index.html](https://docs.rs/rnet/0.1.0/rnet/index.html)

## Rust-CUDA： 完全用 Rust 编写和执行快速 GPU 代码的库和工具生态系统

Rust CUDA 项目是一个旨在使 Rust 成为使用 CUDA 工具包进行极快 GPU 计算的一级（tier-1）语言的项目。它提供了将 Rust 编译为极快的 PTX 代码的工具，以及使用现有 CUDA 库的库。

过去，通用高性能 GPU 计算是使用 CUDA 工具包完成的。CUDA 工具包主要提供了一种使用 Fortran/C/C++ 代码与单一源的 CPU 代码协同进行 GPU 计算的方法。它还提供了许多库、工具、论坛和文档来补充单源 CPU/GPU 代码。

CUDA 是唯一的 NVIDIA 工具包。已经提出了许多用于跨平台 GPU 计算的工具，例如 OpenCL、Vulkan Computing 和 HIP。然而，到目前为止，CUDA 仍然是此类任务最常用的工具包。这就是为什么必须让 Rust 成为与 CUDA 工具包一起使用的可行选择。

然而，使用 Rust 的 CUDA 在历史上一直是一条非常崎岖的道路。到目前为止，唯一可行的选择是使用 LLVM PTX 后端，但是，LLVM PTX 后端并不总是有效，并且会为许多常见的 Rust 操作生成无效的 PTX，而且近年来已经一次又一次地表明随着 rust-gpu（用于 Rust -> SPIR-V）等项目的出现，GPU 上的 Rust 需要专门的解决方案。

我们希望通过这个项目，我们可以推动 Rust GPU 计算行业向前发展，并使 Rust 成为处理此类任务的优秀语言。Rust 提供了很多好处，例如`__restrict__`每个内核的性能优势、出色的模块/板条箱系统、使用 分隔 CPU/GPU 代码的不安全区域`unsafe`、高级包装器到低级 CUDA 库等。

[https://github.com/Rust-GPU/Rust-CUDA](https://github.com/Rust-GPU/Rust-CUDA)

## minijinja: Rust 实现的一款最小依赖的模板库

作者是 Python 框架 Flask 的作者，minijinja 基于 Python 的 Jinja2 模板引擎语法和行为而实现。

[https://github.com/mitsuhiko/minijinja](https://github.com/mitsuhiko/minijinja)



##  appflowy: 一个开源的基于Rust和Flutter的Notion替代产品

[@annieanqi](https://twitter.com/annieanqi) 开源了一个基于 Rust 和 Flutter 的 Notion 替代产品 appflowy，目前是MVP状态，该项目还处于一个比较早的状态，欢迎各位开发者提出自己的想法。

- [官网](https://www.appflowy.io/)
- [Repo](https://github.com/AppFlowy-IO/appflowy)

## Quinn: 0.8.0发布，正式支持 QUIC v1

Quinn是IETF QUIC传输协议的纯rust、异步兼容的实现。

目前刚发布 0.8.0 版本，正式支持了 QUIC v1标准.

[https://github.com/quinn-rs/quinn/releases/tag/0.8.0](https://github.com/quinn-rs/quinn/releases/tag/0.8.0)

## Docker Activity：获取docker容器的统计数据和能耗

[Get stats and the energy consumption of your docker containers](https://www.reddit.com/r/rust/comments/qv5uxm/get_stats_and_the_energy_consumption_of_your/)

Docker activity是一种用于监视Docker容器统计信息并输出其能耗的工具。

当前它还处于早期阶段，对于demoing来说，它很快就会变得更好。。。

[Docker Activity](https://github.com/jdrouet/docker-activity)

##  pixels - 一个微小的硬件加速像素帧缓冲区。 

- 建立在现代图形 API 的基础上

  - wgpu：Vulkan、Metal、DirectX 12、OpenGL ES3。

  - DirectX 11、WebGL2 和 WebGPU 支持正在进行中。

- 使用您自己的自定义着色器来获得特殊效果。

- 完美像素边界上的硬件加速缩放。

- 支持非方形像素纵横比。

[https://github.com/parasyte/pixels](https://github.com/parasyte/pixels)

## Lemmy v0.14.0 发布

Lemmy 是一款 Reddit 替代品，使用 Rust 编写。在刚刚发布的 v0.14.0 中完成了与 Mastodon 和 Pleroma 联合。这意味着，如果你是 Mastodon 和 Pleroma 的用户，那么你也可以：

- 查看 Lemmy 社区，用户配置文件，帖子和评论
- 关注 Lemmy 社区获取新的帖子和评论
- 答复（提及）双向生效，包括通知

[Lemmy Online](https://lemmy.ml/): https://lemmy.ml

[Lemmy (a federated reddit alternative written in Rust) Release v0.14.0: Federation with Mastodon and Pleroma](https://lemmy.ml/post/89740): https://lemmy.ml/post/89740

##  hRPC：面向用户的 API 的简单 RPC 系统

hRPC 是一个 RPC 系统，在 [Harmony](https://github.com/harmony-development) 仓库中，作者们一直在用 hRPC 开发他们的去中心化聊天协议。hRPC 使用 PB 作为协议中间件，并支持流式传输。hRPC 使用 REST 对普通一元请求建模，使用 WebSockets 对流请求建模。因此，为不支持它的语言编写一个库应该很容易。

- [https://dev.to/harmonydevelopment/introducing-hrpc-a-simple-rpc-system-for-user-facing-apis-16ge](https://dev.to/harmonydevelopment/introducing-hrpc-a-simple-rpc-system-for-user-facing-apis-16ge)

##  elfshaker  一个高性能的针对二进制文件进行了优化的版本控制系统

> 400 GiB -> 100 MiB，访问时间为1s+；当应用于 clang 构建时。

Github[链接](https://github.com/elfshaker/elfshaker)

##  semver-explain: 语义版本解释工具

semver-explain，是一个 CLI 工具，用于解释语义版本控制 （Semantic Versioning）要求，将其转换为仅具有小于、大于或等于比较器的形式，其中主要、次要和补丁版本都是指定的。

SemVer 需求的确切含义解释来自于 Cargo。尤其是它不处理连字符范围或来自 JS node-semver 库的 x-ranges。

```rust
$ semver-explain "^1.4.0"
>=1.4.0, <2.0.0
$ semver-explain "~0.5.3"
>=0.5.3, <0.6.0
$ semver-explain "5.6.*"
>=5.6.0, <5.7.0
```

[https://github.com/alilleybrinker/semver-explain](https://github.com/alilleybrinker/semver-explain)

## pigeon-rs：电子邮件自动化工具

Pigeon 是一种命令行工具，用于以廉价且高效的方式自动化电子邮件工作流程。

比如，查询时事通讯的订阅者并向他们发送电子邮件：

```
pigeon send-bulk \
    sender@your-domain.com \
    --receiver-query "select email from user where newsletter_confirmed = true" \
    --message-file "message.yaml" \
    --display \
    --assume-yes
```

结果如下：

```
> Display query result: shape: (4, 1)
+------------------------------+
| email                        |
| ---                          |
| str                          |
+==============================+
| "marie@curie.com"            |
+------------------------------+
| "alexandre@grothendieck.com" |
+------------------------------+
| "emmy@noether.com"           |
+------------------------------+
| "elie@cartan.com"            |
+------------------------------+
> Sending email to 4 receivers ...
marie@curie.com ... ok
alexandre@grothendieck.com ... ok
emmy@noether.com ... ok
elie@cartan.com ... ok
```

Massage 配置文件格式如下：

```
# You can leave EITHER the text OR the html empty, but not both. Ideally, fill out both.
# You MUST provide a subject. Personalize message by wrapping variables in curly brackets, eg. {firstname}.

message:
    # The subject of your email
    subject: "Test subject"
    # The plaintext version
    text: "This is a test message (plaintext)."
    # The html version
    html: "This is a test message (html)."
```

GitHub：[https://github.com/quambene/pigeon-rs](https://github.com/quambene/pigeon-rs)

## LibertyOS - 使用Rust语言从头开发的操作系统

LibertyOS完全从头开发，具体面向桌面还是IoT目前并不清楚，项目还在早期，感兴趣可参与。

[https://github.com/LibertyOS-Development/kernel](https://github.com/LibertyOS-Development/kernel)

## Persy - 一个单文件数据库存储，类似于 sqlite

Persy完全由Rust开发，目前已发布1.1版本。在性能上有一些改进。

[https://persy.rs/posts/persy-1.1.html](https://persy.rs/posts/persy-1.1.html)

##  yap：一个小型的、基于迭代器的、零依赖的解析库

Yap是一个小型的、零依赖的解释器库，灵感来自于parser-combinator。我试图以简洁性换取简单性，并以迭代器接口的灵活性为基础。它的目标是使解析字符串和切片变得容易，并且易于使用。

在过去的几个星期里，我一直在构建和使用它，我认为它已经准备好向其他可能有兴趣使用它的人发布了!

下面是它的用法:

```rust
use yap::{ 
    // This trait has all of the parsing methods on it:
    Tokens,
    // Allows you to use `.into_tokens()` on strings and slices, 
    // to get an instance of the above:
    IntoTokens
};

// Step 1: convert our input into something implementing `Tokens`
// ================================================================

let mut tokens = "10 + 2 x 12-4,foobar".into_tokens();

// Step 2: Parse some things from our tokens
// =========================================

#[derive(PartialEq,Debug)]
enum Op { Plus, Minus, Multiply }
#[derive(PartialEq,Debug)]
enum OpOrDigit { Op(Op), Digit(u32) }

// The `Tokens` trait builds on `Iterator`, so we get a `next` method.
fn parse_op(t: &mut impl Tokens<Item=char>) -> Option<Op> {
    match t.next()? {
        '-' => Some(Op::Minus),
        '+' => Some(Op::Plus),
        'x' => Some(Op::Multiply),
        _ => None
    }
}

// We also get other useful functions..
fn parse_digits(t: &mut impl Tokens<Item=char>) -> Option<u32> {
    let s: String = t
        .tokens_while(|c| c.is_digit(10))
        .collect();
    s.parse().ok()
}

// As well as combinator functions like `sep_by_all` and `surrounded_by`..
let op_or_digit = tokens.sep_by_all(
    |t| t.surrounded_by(
        |t| parse_digits(t).map(OpOrDigit::Digit),
        |t| { t.skip_tokens_while(|c| c.is_ascii_whitespace()); }
    ), 
    |t| parse_op(t).map(OpOrDigit::Op)
);

// Now we've parsed our input into OpOrDigits, let's calculate the result..
let mut current_op = Op::Plus;
let mut current_digit = 0;
for d in op_or_digit {
    match d {
        OpOrDigit::Op(op) => {
            current_op = op 
        },
        OpOrDigit::Digit(n) => {
            match current_op {
                Op::Plus => { current_digit += n },
                Op::Minus => { current_digit -= n },
                Op::Multiply => { current_digit *= n },
            }
        },
    }
}
assert_eq!(current_digit, 140);

// Step 3: do whatever you like with the rest of the input!
// ========================================================

// This is available on the concrete type that strings
// are converted into (rather than on the `Tokens` trait):
let remaining = tokens.remaining();

assert_eq!(remaining, ",foobar");
```

- [https://github.com/jsdw/yap](https://github.com/jsdw/yap)
- [https://www.reddit.com/r/rust/comments/r3blx1/announcing_yap_a_small_iterator_based_zero/](https://www.reddit.com/r/rust/comments/r3blx1/announcing_yap_a_small_iterator_based_zero/)

##  amdfand v1.0.6发布

新版本的AMD显卡冷却和电压守护程序。

当前版本包括：

- 非常简单的电压管理
- 有关如何启用电压管理的信息 ...

- [https://github.com/Eraden/amdgpud](https://github.com/Eraden/amdgpud)
- [https://www.reddit.com/r/rust/comments/r1wyu7/release_amdfand_v106/](https://www.reddit.com/r/rust/comments/r1wyu7/release_amdfand_v106/)
