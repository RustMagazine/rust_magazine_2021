# 基于编译器插件定制 clippy lint

作者: 吴翱翔@pymongo / 后期编辑： 张汉东

> 原文: [基于编译器插件定制 clippy lint](https://pymongo.github.io/#/2021/10/rustc_plugin_custom_lints.md)

本文基于 Rust 月刊先前文章 [华为 | 如何定制 Rust Clippy](https://rustmagazine.github.io/rust_magazine_2021/chapter_6/custom-clippy.html)

上述文章中提供了改 clippy 源码或 dylint 库的两个开发定制 lint 的方案，

但 dylint 不能跨平台且以动态库形式分发难以使用，改 clippy 源码不方便与官方 lint 同步

基于上述困难，我便有了以下 lint 框架的设计需求:
1. 一定要跨平台，同时支持 windows/mac/linux 等主流操作系统
2. 不要有任何依赖，dylint 依赖 clippy 导致 rust-toolchain 被迫绑定跟 clippy 一样的版本
3. 代码足够简单 50 行足以加新的 Lint，不用任何宏导致 IDE 看宏代码时变成"瞎子"
4. 定制的 lint 工具对 **用户/使用者** 而言要易于使用

首先用 rustc_private 编译器模块将自己 lint 框架的库编译为库(以下简称 lints 库)，然后可通过三种渠道运行

1. rustc plugin 编译器动态库补丁
2. ui_test
3. 改 rust 源码引入 lints 库并编译为 toolchains
4. RUSTC=/path/to/my_rustc cargo check

## RUSTC 环境变量

最简单的定制 lint 的方法，将自己写的 lint 注入到 rustc 中假设将自己修改的 rustc 可执行文件叫 my_rustc

然后运行 `"RUSTC=my_rustc" cargo check` 就能运行自己定制的 lint 的检查了

所以我们将问题简化为 如何改 rustc 源码 + 如何编译 rustc 两个部分(当然这么运行会有问题，后文再展开)

## rustc_driver

如果说 rust 源码路径下的 compiler/rustc/src/main.rs 是最终的编译器二进制文件

那么 rustc_driver 模块就是整个编译器的入口和"大脑"，可以通过 rustc_private feature 引入这些编译器的模块

对比 rustc_driver 源码和 rustc 源码在 `compiler/rustc/src/main.rs` 里面也一行调用 `rustc_driver::main()`

发现只需要一行代码就能让 **自己的代码编译出 rustc 编译器**

```rust
fn main() {
    rustc_driver::RunCompiler::new(&std::env::args().collect::<Vec<_>>(), &mut DefaultCallback).run().unwrap();
}
```

`rustc_driver::RunCompiler.run()` 这就是 rustc 编译器的入口函数，我们将自己可执行文件的参数原封不动的转发给 rustc 就能让"编译器"跑起来

**编译源码的子问题被简化为直接调用 RunCompiler，修改源码子问题转换成改 DefaultCallback 即可**

### 如何引入 rustc_driver 库

1. rustup component add rustc-dev 然后就可以用 feature rustc_private
2. crates.io rustc-ap-rustc_driver 这个是 Rust 官方将编译器模块同步上传的版本
3. 自己下载 rustc 源码通过路径引入或者通过 github 链接引入

### trait rustc_driver::Callback

无论是官方的 clippy/miri 工具还是第三方的 flowistry(类似 rust-analyzer 的 Rust vscode 插件)或者 c2rust 等静态分析相关的项目

都是通过重写编译器回调 trait 的各个回调钩子方法，注入自己静态分析代码的逻辑

可以参考 flowistry 静态分析的这行代码:

<https://github.com/willcrichton/flowistry/blob/2f0f843d46995367bf20f76b43315a7199bca70d/src/core/analysis.rs#L50>

## rustc_driver 找不到标准库

`rustc_driver::RunCompiler::new` 这样编译出来的 rustc 运行时报错找不到标准库

> "can't find crate for `std`"

有个不完美的解决方案，在 build.rs 中加入一行，编译时"链接"标准库(以下为 Linux 示例)

> println!("cargo:rustc-link-search=/home/w/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib");

build.rs 的 `cargo:rustc-link-search=` 默认库搜索类型是 all 也包含 Rust ABI 的 rlib 格式库文件

build.rs 方案 **部分解决了**「链接标准库」问题，某些时候也能正常运行，但 ldd 依然提示标准库找不到

```
[w@ww lints]$ ldd ~/.cargo/bin/lints 
        librustc_driver-e1b628cff3afb6ed.so => not found
        libstd-d6566390077dd5f5.so => not found
```

相比 build.rs 中链接标准库，更可靠的做法是运行时给 rustc 加一个 -L 参数链接标准库

例如以下是我 lint 框架的 ui 测试部分代码，我就是通过 rustc 运行时 -L 参数解决标准库找不到的问题

```rust
let stderr = std::process::Command::new("cargo")
    .arg("run")
    .arg("--")
    .arg("--emit=metadata")
    .arg("--crate-type=lib")
    .arg("--allow=dead_code")
    .arg("-L")
    .arg(env!("STD_DYLIB_PATH"))
    .arg(rs_file_path)
    .output()
```

`env!("STD_DYLIB_PATH")` 来自 build.rs，具体可看我 lint 框架源码: <https://github.com/pymongo/lints>

即便运行时加上 -L 参数，但这样的 rustc 依然不容易进行 `cargo build` 或 codegen，建议仅用于分析代码

## 45 行代码写个编译器补丁/插件

例如我想添加一个检测函数名为 foo 的 lint 检查规则，首先 cargo 新建一个库:

> cargo new --lib rustc_plugin_my_lints

然后在 Cargo.toml 将库的类型设置成 dylib

```
[lib]
crate-type = ["dylib"]
```

然后在 src/lib.rs 中写下以下代码:

```rust
#![feature(rustc_private)]
extern crate rustc_ast;
extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_span;

#[no_mangle]
fn __rustc_plugin_registrar(reg: &mut rustc_driver::plugin::Registry) {
    reg.lint_store.register_early_pass(|| Box::new(FnNameIsFoo));
}

struct FnNameIsFoo;
impl FnNameIsFoo {
    const LINT: rustc_lint::Lint = {
        let mut lint = rustc_lint::Lint::default_fields_for_macro();
        lint.name = "fn_name_is_foo";
        lint.default_level = rustc_lint::Level::Warn;
        lint
    };
}

impl rustc_lint::LintPass for FnNameIsFoo {
    fn name(&self) -> &'static str {
        "fn_name_is_foo"
    }
}

impl rustc_lint::EarlyLintPass for FnNameIsFoo {
    fn check_fn(
        &mut self,
        cx: &rustc_lint::EarlyContext<'_>,
        fn_kind: rustc_ast::visit::FnKind<'_>,
        span: rustc_span::Span,
        _: rustc_ast::NodeId,
    ) {
        if let rustc_ast::visit::FnKind::Fn(_, ident, ..) = fn_kind {
            if ident.as_str() == "foo" {
                rustc_lint::LintContext::struct_span_lint(cx, &Self::LINT, span, |diagnostic| {
                    diagnostic.build("foo is a bad name for function").emit();
                });
            }
        }
    }
}
```

然后新建一个 examples/test_plugin.rs 文件测试我们写好的编译器插件/补丁

```rust
#![feature(plugin)]
#![plugin(rustc_plugin_my_lints)]
fn foo() {

}
fn main() {
    foo();
}
```

最后可以运行下编译器补丁的测试:

> cargo run --example test_plugin

然后得到了这样的输出:

```
warning: foo is a bad name for function
 --> examples/test_plugin.rs:3:1
  |
3 | / fn foo() {
4 | |
5 | | }
  | |_^
  |
  = note: `#[warn(fn_name_is_foo)]` on by default
```

## lints 框架的设计思路

由于 plugin feature 已经 deprecated 了，以后可能会被删掉，我的 lint 框架的设计思路是只做 lint 检查规则的功能测试和 ui 测试，定期集成到 rust 源码中编译一套自己的工具链做长远使用，同时提供 my_rustc 可执行文件或者编译器插件库进行使用

在我读了很多 rust-analyzer/rust-clippy/dylint/flowistry 等静态分析相关项目源码后设计了 lints 静态分析框架

这是 lints 项目源码的链接: <https://github.com/pymongo/lints>

以下是 lints 项目的源码结构和解读:

```
.
├── build.rs
├── Cargo.lock
├── Cargo.toml
├── examples
│   └── compiler_plugin.rs # 编译器插件的示例
├── README.md # crate lints 的文档
├── rust-toolchain.toml # 定义所需 rustup components
├── src
│   ├── bin
│   │   └── rustc_.rs # 用于 ui 测试，也能通过 RUSTC=/path/to/rustc_ cargo c 运行在简单项目
│   ├── lib.rs
│   └── lints # early_lint + late_lint 的定义和逻辑
│       ├── check_enum_size.rs
│       ├── fn_name_is_foo.rs
│       └── mod.rs
├── tests # 测试代码
│   └── ui_test.rs # ui 测试
└── ui_test # ui 测试用例: 输入+输出+期待值
    ├── check_enum_size.rs
    ├── check_enum_size.stderr
    ├── fn_name_is_foo.rs
    └── fn_name_is_foo.stderr
```

添加一条 lint 检查规则非常简单只需在 src/lints 下加一个文件，定义一个新的结构体并**注册**在 src/lints/mod.rs 即可

最后，用 rustc plugin 插件不光能定制 lint 还可以做帮组源码中的 enum 排序等等有趣的事情
