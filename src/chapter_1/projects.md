# 本月简报 | 推荐项目

## 「微软」Rust for Windows

- [仓库链接](https://github.com/microsoft/windows-rs)
- [文档链接](https://microsoft.github.io/windows-docs-rs/doc/bindings/windows)
- [crate 链接](https://crates.io/crates/windows)

这个仓库是 1 月 20 日微软发布的官方 Win32 API crate。

过去用 rust 为 Windows 开发应用程序时，若要调用 Win32 API，必须使用 [winapi-rs](https://github.com/retep998/winapi-rs) 这样的 wrapper 库，此类库需要社区去人工维护和 Win32 API 的绑定。
为了改善这点，微软通过 [win32metadata](https://github.com/microsoft/win32metadata) 项目来加强对 C/C++ 以外的编程语言的支持（[相关链接](https://blogs.windows.com/windowsdeveloper/2021/01/21/making-win32-apis-more-accessible-to-more-languages/)），
其中就包括对 rust 的支持。

现在已经有使用该库实现的[扫雷](https://github.com/robmikh/minesweeper-rs)程序, 除此之外，也有微软工程师发布了一些[示例项目](https://github.com/kennykerr/samples-rs)。

## Czkawka

- [仓库链接](https://github.com/qarmin/czkawka)
- [reddit 讨论](https://www.reddit.com/r/linux/comments/kjcbva/czkawka_200_multithread_support_similar_images/)

*Czkawka* 是一个多平台的空间清理应用，可用于找出系统中的重复的文件、空文件夹、临时文件等。

项目采用 gtk3/gtk-rs 开发 GUI 部分, 同时也提供 CLI 程序。

![czkawka](https://user-images.githubusercontent.com/41945903/103371136-fb9cae80-4ace-11eb-8d72-7b4c8ac44260.png)


## Artichoke

- [项目主页](https://www.artichokeruby.org/)
- [推特主页](https://twitter.com/artichokeruby)
- [仓库链接](https://github.com/artichoke/artichoke)
- [rubyconf 2019 上的相关演讲](https://www.youtube.com/watch?v=QMni48MBqFw&list=PLE7tQUdRKcyZDE8nFrKaqkpd-XK4huygU&index=37)

*Artichoke* 是一个由 rust 开发的 ruby 实现，可以将 ruby 代码编译至 WebAssembly。

当前 Artichoke 依然依赖于 mruby backend，在与 mruby 进行 FFI 交互的同时，改进某些 Kernel 和库函数的实现。例如 [regex](https://github.com/artichoke/artichoke/tree/trunk/artichoke-backend/src/extn/core/regexp) 部分就是由 rust 实现的。

作者表示在未来会开发出一个纯 rust 的实现。

## linfa

- [仓库链接](https://github.com/rust-ml/linfa)
- [文档链接](https://docs.rs/linfa/0.3.0/linfa/)
- [reddit 讨论](https://www.reddit.com/r/rust/comments/e4wh8c/linfa_taking_ml_to_production_with_rust_a_25x/)

*linfa* 是一个机器学习的框架和工具集，其设计参照了 python 的 `scikit-learn` 库。

关于 rust 在机器学习方面的生态系统，可以参考 [arewelearningyet](http://www.arewelearningyet.com/)。

## async-trait-static

- [仓库链接](https://github.com/tiannian/async-trait-static)
- [文档链接](https://docs.rs/async-trait-static/0.1.4/async_trait_static/)

*async-trait-static* 是一个用于在 trait 中声明 async 方法的库，可以在 `no_std` 下使用。

由于 rustc 的限制，[要在 trait 中写出 async 方法是很困难的](https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/)。
针对这个问题，dtolnay 实现了 [async-trait](https://github.com/dtolnay/async-trait)，将 `async fn` 的返回类型转化为 `Pin<Box<dyn Future>>`。

async-trait-static 则采用了 GAT 来实现这个功能，无需用到 trait object。

当前 rust 的 GAT 依然不够完善，因此该库还是有些功能是缺失的。

## regexm

- [仓库链接](https://github.com/TaKO8Ki/regexm)
- [文档链接](https://docs.rs/regexm/0.1.0-beta.1/regexm/)
- [示例](https://github.com/TaKO8Ki/regexm/tree/main/examples)

*regexm* 是一个用于对正则表达式进行模式匹配的库：

```rust
fn main() {
    let text1 = "2020-01-01";
    regexm::regexm!(match text1 {
        r"^\d{4}$" => println!("y"),
        r"^\d{4}-\d{2}$" => println!("y-m"),
        // block
        r"^\d{4}-\d{2}-\d{2}$" => {
            let y_m_d = "y-m-d";
            println!("{}", y_m_d);
        }
        _ => println!("default"),
    });
}
```

## swc

- [项目主页](https://swc.rs/)
- [仓库链接](https://github.com/swc-project/swc)

*swc* 是一个 typescript/javascript 的 transpiler，在运行速度上，单核比 babel 快 4 倍，4 核比 babel 快 70 倍，同时也具有 treeshaking 的功能。

*swc* 被用于 deno 项目中，用于类型擦除。 swc 的作者是一名 97 年的大二学生，如今已经获得了 Deno 官方的顾问合同。

## rlink-rs

*国产项目*

- [仓库链接](https://github.com/rlink-rs/rlink-rs)

rlink-rs是基于rust实现的流式计算引擎，用来作为Apache Flink的替代方案。

相对于在线业务，rlink-rs更关注海量数据的离线流式处理场景，提升吞吐能力、降低资源消耗。其特点是针对exactly once提供计算和输出两种语义；基于特殊的exactly once输出语义，结合rust内存管理模型，实现大部分场景的全内存计算，解决state和checkpoint引起的重量级IO操作。

rlink-rs的目标是成为一个计算驱动引擎，允许基于DAG定制你自己的计算流程、实现自己的计算语义。

目前状态：主要针对flink流计算这块做对比。已经实现基本窗口计算流程。

希望能从社区得到关于流引擎设计方面的帮助：

1.因为rust语言不如Java动态语言可以反射，在用户api上不那么优雅。
2.只是想在语义上实现类似flink的api，实现上还是想走一条新的路线，毕竟flink有历史包袱，它的实现我们不需要100%参考。