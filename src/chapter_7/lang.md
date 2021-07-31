# 官方动态

编辑： 张汉东

---

## Rust 1.54 稳定版发布

Rust 1.54 更新的特性并不是很多，值得注意的是:

1. 增量编译功能又重新默认开启了。
2. 在 ErrorKind 中 多了一个 OutOfMemory 类型。 

[https://blog.rust-lang.org/2021/07/29/Rust-1.54.0.html](https://blog.rust-lang.org/2021/07/29/Rust-1.54.0.html)

## task::ready! 宏将在 1.56 稳定

futures-core 的 ready!() 宏存在好几年了，这个宏可以在处理 Future 的时候减少很多模板代码。

ready!() 宏稳定之后将会在 std::task 模块。

[https://github.com/rust-lang/rust/pull/81050](https://github.com/rust-lang/rust/pull/81050)

## Rust 2021 进入公开测试期

Rust 2021 版进入 公开测试期 。该版本的所有计划功能现在都可以在 nightly 版本中率先体验。同时还提供了方便的迁移办法，以便将代码从 Rust 2018 快速迁移到 Rust 2021 。

- 安装最近的 nightly 工具链：`rustup update nightly` 。
- 运行 `cargo +nightly fix --edition` 。
- 编辑 `Cargo.toml` ，将 `cargo-features = ["edition2021"]` 置于顶部（位于 `[package]` 上方），并将 `edition` 改为 `edition = "2021" `。
- 运行 `cargo +nightly check` 以确认能否在新版本上正常工作。

Datafuse 团队目前正在进行 Rust 2021 迁移的尝鲜体验，相关工作可以关注：[https://github.com/datafuselabs/datafuse/pull/1159](https://github.com/datafuselabs/datafuse/pull/1159)

Rust 2021 public testing period: [https://blog.rust-lang.org/2021/07/21/Rust-2021-public-testing.html](https://blog.rust-lang.org/2021/07/21/Rust-2021-public-testing.html)

the nightly version of Rust edition guide: [https://doc.rust-lang.org/nightly/edition-guide/rust-2021/index.html](https://doc.rust-lang.org/nightly/edition-guide/rust-2021/index.html)



## rustc_codegen_gcc 的 MCP 已经被接受

在不久的将来，Rust 就会多一个 GCC 的后端。

同类项目还有 [GCC-rs ](https://github.com/Rust-GCC/gccrs),GCC-rs  是 用 Cpp 重新实现 Rustc 的一个 GCC 前端。

为什么有 GCC-rs 这个项目？

1. 想要支持更多的 CPU 架构
2. 跨语言 LTO。GCC-RS FAQ将Linux列为激励示例。 具有讽刺意味的是，Linux支持ltvm但不是gcc！
3. Rust 自举（Bootstrap）链很长，因为需要从C到OCAML，然后编译预发布 Rust 以编译 Rust 1.0编译 Rust 1.1 、1.2等，直到捕获最多1.53（或者最新版本）。 因此，如果您可以用C++中编写的 Rust 编译器直接编译1.53，则可以节省一些时间。
4. 复用 GCC 插件

但 [rustc_codegen_gcc](https://github.com/antoyo/rustc_codegen_gcc)  作者认为 GCC-rs 其实没有很好的解决这些问题。

rustc_codegen_gcc 项目只需将GCC插入现有的Rust编译器作为代码生成后端，就可以简单的达成这些目标。

该项目的主要目标是能够在LLVM不支持的平台上编译 Rust 代码。 次要目标是检查使用GCC后端是否提供任何编译速度改进。

现在 rustc_codegen_gcc 已经被接受，gcc-rs 该何去何从？

相关阅读：[Rust 与 开源 | GPL 许可证引发的问题](https://zhuanlan.zhihu.com/p/387946955)



## GAT （generic associated types ）今年能否稳定？ 我看行。

四天前，在 GAT tracking issues 下有人回复：

摘录

> 我预计我们可能在10月份左右就能实现稳定化；但这确实取决于1-3个月的测试期。如果测试结束后相对来说没有错误，那么应该会很顺利。如果发现一些严重的错误、设计缺陷等，可能会推迟几个月，但这仍然为年底提供一些余地。

鼓励大家来测试相关功能。

详细：[https://github.com/rust-lang/rust/issues/44265#issuecomment-869888398](https://github.com/rust-lang/rust/issues/44265#issuecomment-869888398)

## 错误处理工作组工作进展报告

官方错误处理工作组发布了一篇新文章，描述了当前错误处理需要解决的主要挑战：容易丢失上下文。

以下是一些摘要：

```rust
use std::fmt;

// 需要加载一个配置文件，并且期望配置加载总是成功的。
fn main() {
    let _config = load_config()
        .expect("config is always valid and exists");
}

// 这里实现一个加载配置文件总是错误的样例，因为要展示错误诊断信息
fn load_config() -> Result<(), Error> {
    Err(Error(SourceError))
}

// 我们有一个错误类型，只需打印“invalid config“
// 并且具有仅打印的源错误 "config file does not exist"
#[derive(Debug)]
struct Error(SourceError);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid config")
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

#[derive(Debug)]
struct SourceError;

impl fmt::Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("config file does not exist")
    }
}

impl std::error::Error for SourceError {}
```

我们想要得到的错误信息如下：

```rust
$ cargo run
thread 'main' panicked at 'config is always valid and exists', src/main.rs:4:33

Error:
    0: invalid config
    1: config file does not exist

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```

通过这样的信息，我们可以看出程序退出是因为 panic 发生了。我们可以看出违反了 0 和 1 两种原因而导致了恐慌，并且能定位到恐慌发生的位置。

但是当前实际的输出是：

```rust
$ cargo run
thread 'main' panicked at 'config is always valid and exists: Error(SourceError)', main.rs:4:33
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Rust的panic基础设施没有提供将Error类型转换为panic的方法，它只支持将Debug类型转换为panic，我们觉得这是一个大问题。同样的，语言也没有提供方便的工具来打印一个错误和它的所有源的错误信息。

当我们处理 Result 的时候：

```rust
fn main() {
    let result = load_config();
    let _config = match result {
        Ok(config) => config,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };
}
```

我们希望得到：

```rust
$ cargo run
Error: invalid config: config file does not exist
```

但实际输出：

```rust
$ cargo run
Error: invalid config
```

默认情况下，源的所有错误信息都会丢失。这是因为我们用Display作为单个错误信息的接口。如果我们能回到过去，我们目前会建议在`Error trait`中加入`fn message(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`，但现在这艘船已经起航了。

今天，库中解决这个问题的方法是滥用`Debug trait`。像`eyre`、`anyhow`这样的类型，甚至有时自定义的错误枚举也使用它们的`Debug`输出来打印人类可读报告中的全部错误链。

这样做的好处是使打印完整的错误报告变得容易，并且使`unwrap`、`expect`和从`main`返回都打印完整的错误报告。但这样做使我们无法访问错误的 Derive Debug格式，有可能隐藏了调试可能需要的内部细节，但这些细节并不是供用户阅读的错误信息的一部分。

错误处理的未来

最终，我们希望在Rust中进行错误处理时，你所使用的默认工具都能做正确的事情，并充分利用`Error trait`的设计。Unwrap 一个实现了`Error trait`的类型将保留原始的错误，作为一个动态的`Error`，然后可以在panic hook 中使用。打印一个完整的错误报告将是很容易做到的，而且很明显。有了这些变化，希望在报告错误时很难意外地丢弃信息。

要达成这个目标，需要有两步计划：

1. 集成 `Error Trait` 和 `Panic Runtime`。

    a. 先将`Error trait`移至 `core`
    b. 添加一个接口，用于从Error类型中创建一个panic，比如 `panic_error` 函数。类似于std中已经有的`panic_any`函数。这个函数将让panic处理程序通过`dyn Error`来访问错误。
    c. 更新std提供的默认panic hook，通过Error trait 实际报告恐慌。它应该遍历来源并打印由错误本身捕获的回溯，如果有的话，或者可能自己捕获一个。
    d. 最后，我们需要对`expect`和`unwrap`进行特殊处理，以便对实现了`Error trait`的类型 unwrap 时使用这些新的Error感知恐慌的接口。要做到这一点，我们首先需要解决一个健全性问题，即对基于寿命的条件的trait impls进行专业化处理，不过幸好我们已经有了一个[很好的解决方法](https://smallcultfollowing.com/babysteps/blog/2018/02/09/maximally-minimal-specialization-always-applicable-impls)。

2. 实现一个基础的错误报告工具。

    我们不能完全达到这个目标，因为我们使用`Display`来处理单个错误信息，而且我们不能以向后兼容的方式来改变这一点，但我们希望增加一个方便的方法来打印完整的错误链和一些巧妙的行文，以减轻大部分的压力。

    我们计划通过在标准库中添加一个`Report`类型来解决这个问题，该类型包装了一个`&dyn Error`，并实现了`Display`，这样它就可以按要求打印每个源。我们希望`Report`的 `display `方法的输出能够支持Rust生态系统中最常见的错误串联的风格。

    ```rust
    println!("Error: {}", Report::from(error));

    // Outputs:
    // Error: outermost error: second error: root error

    println!("Error: {:#}", Report::from(error))

    // Outputs:
    // Error: outermost error
    //
    // Caused by:
    //    0: second error
    //    1: root error

    // report 方法大概像这样
    fn report(&self) -> impl Display + '_
    where
        Self: Sized,
    {
        Report::from(self)
    }
    ```

3. 错误处理工作组最近创建了一个关于[如何实现`Display::fmt`和`Error::source`的指导原则](https://github.com/rust-lang/project-error-handling/issues/27#issuecomment-763950178)。
    这个建议只适用于作为库的API的一部分被暴露的错误类型。库或应用程序中的内部错误可以做任何他们想做的事情，但一旦他们需要被第三方用户集成到其他板块，重要的是错误要遵循一致的风格。。如果你对我们的理由感兴趣或有任何意见，请查看我们关于这个主题的github问题：[Rust-lang/project-error-handling#27](https://github.com/rust-lang/project-error-handling/issues/27) 。


这就是目前的计划，它不是我们想做的所有变化的完整计划，但我们认为这是最好的第一步。

> 免责声明：这篇文章是计划和愿望的结合。这里有一些技术上的挑战需要解决，所以最终的结果可能与我们最初的设想有很大的不同，所以请不要认为这些都是最终结果。

[https://blog.rust-lang.org/inside-rust/2021/07/01/What-the-error-handling-project-group-is-working-towards.html](https://blog.rust-lang.org/inside-rust/2021/07/01/What-the-error-handling-project-group-is-working-towards.html)

## cargo-supply-chain: 官方安全代码工作组发布的新工具 

作用：

在依赖包关系图中收集作者、贡献者和发布者的crate数据。

使用场景：

- 寻找值得支持的人和团体。
- 识别依赖关系图中的风险。
- 对所有你通过构建他们的软件而隐含信任的贡献者进行分析。这可能有清醒和谦卑的效果。

cargo-supply-chain 自身也提供了一个输出样本： [publishers](https://gist.github.com/Shnatsel/3b7f7d331d944bb75b2f363d4b5fb43d), [crates](https://gist.github.com/Shnatsel/dc0ec81f6ad392b8967e8d3f2b1f5f80), [json](https://gist.github.com/Shnatsel/511ad1f87528c450157ef9ad09984745)。

用法：

```rust
cargo install cargo-supply-chain
```

该组织内还有另外一个工具：cargo-geiger。 可以检测crate 及其依赖关系中unsafe rust的用法。也可以 Rust 项目的安全检查比率。


相关链接：

- [安全代码工作组官网](https://www.rust-lang.org/governance/wgs/wg-secure-code)
- [安全代码工作组 GitHub 组织](https://github.com/rust-secure-code)
- [cargo-supply-chain](https://github.com/rust-secure-code/cargo-supply-chain)
- [cargo-geiger](https://github.com/rust-secure-code/cargo-geiger)

## 让 Rust 更快的解析 float类型 已被合并到 Rust core 库

作者两年前就提出了如何让 Rust 解析 float 更快更安全, 不过最近, 这些改变才合并到libcore.

这意味着, 当你解析大量的 float 类型时, 性能会得到非常夸张的提升.

例如:

- 0.06,0.13,0.25,0.38,0.44,0.44,0.38,0.44,0.5,0.56 这样的数据大概会提升 2 倍.
- -65.613616999999977,43.420273000000009,-65.619720000000029,43.418052999999986,-65.625,43.421379000000059 这样的数据大约会提升 10倍 性能.
- 8.988465674311580536566680e307 这种数据大概会提升1600~10,000倍的提升.

- [https://github.com/rust-lang/rust/pull/86761](https://github.com/rust-lang/rust/pull/86761)
- [相关算法论文](https://arxiv.org/abs/2101.11408)

## RFC 3128: I/O Safety

Rust 现在已经有一个被采纳的 I/O 安全的 RFC. 该 RFC 用于处理 文件操作符和 sockets 等的生命周期. 这让编写底层安全的系统对象有了可能.

[https://github.com/rust-lang/rfcs/blob/master/text/3128-io-safety.md](https://github.com/rust-lang/rfcs/blob/master/text/3128-io-safety.md)

## rustc_codegen_gcc 进展报告#2

rustc_codegen_gcc 是 rustc 的 GCC 代码，这意味着它可以被现有的 rustc 前端加载，通过支持更多的架构和 GCC 的优化而受益于 GCC。

当前状态：Antoyo 手动实现了 popcount（因为 gcc 会产生一个对某些函数的调用，这在 no-std 中是行不通的）。除此之外，修复了一些东西，使之更容易在 godbolt 中添加这个 gcc 代码，并使 Antoyo 的这个PR合并到 rustc 中。

报告#2的详情参见 Antoyo 的博客链接，[https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-2](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-2)