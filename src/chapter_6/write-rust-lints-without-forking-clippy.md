# 无需 fork Clippy 就可以编写 Rust lints

![dylint](../image/dylint.jpeg)

[原文](https://www.trailofbits.com/post/write-rust-lints-without-forking-clippy)

本文主要介绍 [Dylint](https://github.com/trailofbits/dylint)，它是一个可以从动态库中加载 Rust lints 规则的工具。Dylint 可以让开发人员轻松维护自己的个人 lint 集合。

在此之前，编写一个新的 Rust lint 的最简单的方式就是 fork [Clippy](https://github.com/rust-lang/rust-clippy)，它是 Rust 事实上的 (de facto) lint 工具。但是这种方式在运行或维护新 lint 时存在缺陷 (drawback)。Dylint 最大程度的减少了这方面的干扰 (distraction)，让开发者可以专注于编写 lint。

首先，我们将回顾 (go over) Rust linting 的当前状态以及 Clippy 的工作原理。然后，我们将解释 Dylint 是如何改善现状 (quo) 并提供一些关于如何开始使用它的提示。如果你想直接编写 lint，请跳到最后一节。

### Rust linting 和 Clippy

Clippy 等工具利用了 Rust 编译器对 linting 的专用支持。Rust linter 的核心组件(即“驱动程序”)可以链接到对应的库 (rustc_driver) 中。通过这种方式，驱动程序本质上是对 Rust 编译器做了封装。

为了运行 linter，环境变量 [RUSTC_WORKSPACE_WRAPPER](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-reads) 需要指向驱动程序并运行`cargo check`。Cargo 会注意到该环境变量已被赋值并调用该驱动程序，而不是调用 **rustc**。当驱动程序被调用时，它在 Rust 编译器中的 [Config](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_interface/interface/struct.Config.html) 结构体中设置了一个 [callback](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_interface/interface/struct.Config.html#structfield.register_lints)。该 [callback](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_interface/interface/struct.Config.html#structfield.register_lints) 注册了一些 lint，它们将会与 Rust [内置的 lint](https://doc.rust-lang.org/rustc/lints/listing/index.html) 一起运行。

Clippy 执行一些[检查](https://github.com/rust-lang/rust-clippy/blob/12fce557669a0de230399cf8e6eee4f5307bf87b/src/driver.rs#L329-L338)以确保它已被启用，否则将以上述方式进行工作。(关于 Clippy 架构，请参阅图 1)。尽管它在安装之后人们对它的认识依旧不是那么的清晰，但是 Clippy 实际上有两个二进制文件：一个 Cargo 命令以及一个 **rustc** 驱动。你可以输入以下命令进行验证：

```console
which cargo-clippy
which clippy-driver
```

<center><img alt="dylint1" src="../image/dylint1.png"/></center>
<center><span style="font-size: 1em">图 1：Clippy 架构</span></center>

现在假设你想编写自己的 lint。你该怎么办？你需要一个驱动程序来运行它们，而 Clippy 有一个驱动程序，因此 fork Clippy 看起来是一个合理的步骤。但是这个解决方案有一些缺陷，即 (namely) 运行和维护你将开发的 lint。

首先，你的 fork 将拥有两个二进制文件的副本，确保它们可以被找到是一件很麻烦 (hassle) 的事情。你必须确保至少 cargo 命令在你的`PATH`中，并且你可能必须将二进制文件重命名，以保证它们不会干扰 Clippy。虽然这些问题不是难以克服 (insurmountable)，但你可能会选择尽量避免它们。

其次，所有的 lint (包括 Clippy 的 lint) 都是在 [unstable](https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/index.html#note) 编译器 API 之上构建的。一起编译的 lint 必须使用相同版本的 API。为了理解为什么会出现这个问题，我们将参考 [clippy_utils](https://github.com/rust-lang/rust-clippy/tree/master/clippy_utils) - Clippy 作者慷慨地公开的一组实用程序。请注意，**clippy_utils** 使用与 lint 相同的编译器 API，并且同样不提供稳定性保证(参见下文)。

假设你已经 fork Clippy，然后你希望添加一个新的 lint。很显然，你希望新的 lint 使用最新版本 **clippy_utils**。但是假设 **clippy_utils** 使用的编译器版本是 B，而你 fork 的 Clippy 使用的编译器版本是 A。然后你将面临一个困境 (dilemma)：你应该使用一个旧版本的 **clippy_utils** (使用的 A 版本的编译器)还是将 fork 中所有 lint 更新到 B 版本的编译器？两者都不是理想的选择。

Dylint 同时解决了这两个问题。首先，它提供了一个 Cargo 命令，使你不必管理多个这样的命令。其次，对于 Dylint，lint 是在一起编译的以生成动态库。因此在上述情况下，你可以简单地将新的 lint 存储在使用B 版本的编译器的新动态库中。你可以根据需要将这个新库与现有库一起使用，如果你愿意的话，可以将现有库升级到更新的库的编译器版本。

Dylint 提供了与重用中间编译结果相关的额外好处。要理解它，我们需要了解 Dylint 的工作原理。

### Dylint 的工作原理

和 Clippy 一样，Dylint 提供了一个 cargo 命令。可以指定用户想要加载 lint 的动态库。Dylint 以确保在将控制权移交给 Rust 编译器之前注册 lint 的方式运行`cargo check`。

然而，Dylint 的 lint 注册过程比 Clippy 更复杂。Clippy 的所有 lint 都使用相同的编译器版本，因此只需要一个驱动程序。但是 Dylint 用户可以选择从不同编译器版本的库中加载 lint。

Dylint 按需即时 (on-the-fly) 构建驱动程序来处理此类情况。换句话说，如果用户想要 A 版本的编译器库中加载 lint，并且找不到 A 版本的驱动程序，Dylint 将构建一个新的 A 版本的驱动程序。驱动程序缓存在用户的主目录中，因此仅在必要时重建它们。

<center><img alt="dylint1" src="../image/dylint2.png"/></center>
<center><span style="font-size: 1em">图 2：Dylint 架构</span></center>

这给我们带来了上面暗指 (alluded to) 的额外好处。Dylint 根据它们使用的编译器版本对库进行分组，使用相同编译器版本的库一起加载，并在它们的 lint 一起运行。这允许在 lint 之间共享中间编译结果(如：符号解析，类型检查，特征求解等)。

举个例子，在图 2 中，如果库 U 和 V 都使用了 A 版本的编译器，这两个库将被放到同一个分组中。A 版本编译器的驱动程序将只被调用一次。驱动程序在将控制权移交给 Rust 编译器之前会在库 U 和库 V 中注册 lint。

为了理解为什么这种方式更好，可以做如下思考。假设 lint 由编译器驱动程序(而不是动态库)直接存储，并回顾一下驱动程序本质上是 Rust 编译器的封装。因此，如果在使用相同编译器版本的两个编译器的驱动程序中有两个 lint，则在同一代码上运行这两个驱动程序将等同于该代码进行了两次编译。通过将 lint 存储在动态库中并按照编译器版本对它们进行分组，Dylint 避免了这些低效的操作。

### 应用：特定项目的 lint

你是否知道 Clippy 包含 lint，其唯一目的是对 Clippy 的代码进行 lint？[这是真的](https://github.com/rust-lang/rust-clippy/blob/master/clippy_lints/src/utils/internal_lints.rs)。Clippy 包含用于检查的 lint，例如：每个 lint 都有一个关联的 **LintPass**，它使用某些 Clippy 封装函数而不是它们自己封装的函数，并且每个 lint 都有一个非默认的描述。将这些 lint 应用于 Clippy 以外的代码是没有意义的。但是没有规定所有 lint 都必须是通用的，Clippy 就利用了这一点。

Dylint 包含 lint 的主要目的是对 Dylint 的代码进行 lint。例如：在开发 Dylint 时，我们发现自己编写了如下代码：

```rust
let rustup_toolchain = std::env::var("RUSTUP_TOOLCHAIN")?;
...
std::env::remove_var("RUSTUP_TOOLCHAIN");
```

这么做不是很好。为什么？因为我们对字符串字面量进行 fat-fingered 只是时间问题。

```rust
std::env::remove_var("RUSTUP_TOOLCHIAN"); // Oops
```

更好的方法是使用常量而不是字符串字面量，就如下代码所示：

```rust
const RUSTUP_TOOLCHAIN: &str = "RUSTUP_TOOLCHAIN";
...
std::env::remove_var(RUSTUP_TOOLCHAIN);
```

因此当使用 Dylint 时，我们编写了一个 lint 来检查这种不适当的做法并提出适当的建议。我们将该 lint 应用到 Dylint 源码。lint 称其为 [env_literal](https://github.com/trailofbits/dylint/tree/master/examples/env_literal)，其当前的核心实现如下：

```rust
impl<'tcx> LateLintPass<'tcx> for EnvLiteral {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &Expr<'_>) {
        if_chain! {
            if let ExprKind::Call(callee, args) = expr.kind;
            if is_expr_path_def_path(cx, callee, &REMOVE_VAR)
            || is_expr_path_def_path(cx, callee, &SET_VAR)
            || is_expr_path_def_path(cx, callee, &VAR);
            if !args.is_empty();
            if let ExprKind::Lit(lit) = &args[0].kind;
            if let LitKind::Str(symbol, _) = lit.node;
            let ident = symbol.to_ident_string();
            if is_upper_snake_case(&ident);
            then {
            span_lint_and_help(
                cx,
                ENV_LITERAL,
                args[0].span,
                "referring to an environment variable with a string literal is error prone",
                None,
                &format!("define a constant `{}` and use that instead", ident),
            );
            }
        }
    }
}
```

以下是它可以产生的警告示例：

```console
warning: referring to an environment variable with a string literal is error prone
--> src/main.rs:2:27
|
2 |     let _ = std::env::var("RUSTFLAGS");
|                           ^^^^^^^^^^^
|
= note: `#[warn(env_literal)]` on by default
= help: define a constant `RUSTFLAGS` and use that instead
```

回顾之前所说的，编译器以及 **clippy_utils** 都没有为它的 API 提供稳定性保证，因此 **env_literal** 的未来版本可能看起来有点不同。(实际上，当本文还在撰写的过程中，**clippy_utils**  某个 API 的变更就已经导致 **env_literal** 某个实现发生改变!)。当前版本的 **env_literal** 总是可以在 Dylint 仓库中的 [examples](https://github.com/trailofbits/dylint/tree/master/examples) 目录下找到。

但是 Clippy “自我 lint” 的方式与 Dylint 略有不同。Clippy 的内部 lint 被编译成启用了特定功能的 Clippy 版本。但是对于 Dylint，**env_literal** lint 被编译成了一个动态库。因此，**env_literal** 不是 Dylint 的一部分。它本质上是输入。

为什么这很重要？因为你可以为你的项目编写自定义 lint 并使用 Dylint 来运行它们，就像 Dylint 运行自己的 lint 一样。在 Dylint 仓库中 Dylint 运行的 lint 来源没有任何重要意义。Dylint 可以很轻易的在你的仓库中运行该仓库的 lint。

最重要的是 (The bottom line is this)：如果你发现不喜欢自己编写的代码，并且可以使用 lint 检测该代码，Dylint 可以帮助你清除该代码并防止重新引入。

### 开始 linting

使用以下命令安装 Dylint：

```console
cargo install cargo-dylint
```

我们还推荐安装 [dylint-link](https://github.com/trailofbits/dylint/tree/master/dylint-link) 来处理超链接：

```console
cargo install dylint-link
```

编写 Dylint 库的最简单的方式是 fork [dylint-template](https://github.com/trailofbits/dylint-template) 仓库。该仓库直接生成了一个可加载的库。你可以按如下方法进行验证：

```console
git clone https://github.com/trailofbits/dylint-template
cd dylint-template
cargo build
cargo dylint fill_me_in --list
```

你只需实现 [LateLintPass](https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html) 特征并容纳要求填写的符号即可。

以下资源对你编写 lint 将很有帮助：

- [添加一个新的 lint](https://github.com/rust-lang/rust-clippy/blob/master/doc/adding_lints.md) (针对 Clippy 但依然很有用)
- [编写 lint 的常用工具](https://github.com/rust-lang/rust-clippy/blob/master/doc/common_tools_writing_lints.md)
- [rustc_hir 文档](https://doc.rust-lang.org/stable/nightly-rustc/rustc_hir/index.html)

也可以考虑使用上面提到的 [clippy_utils](https://github.com/rust-lang/rust-clippy/tree/master/clippy_utils)。它包含许多底层任务的功能，如查找符号和打印诊断信息，可以让编写 lint 变得更加容易。

我们十分感谢 Clippy 作者将 **clippy_utils** 开放在 Rust
 社区。我们也十分感谢 [Philipp Krones](https://github.com/flip1995) 在本文的早期版本中提供了有用的建议。