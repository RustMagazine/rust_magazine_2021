# Rust官方动态

- 来源：[Rust日报](https://rustcc.cn/section?id=f4703117-7e6b-4caf-aa22-a3ad3db6898f)
- 作者：`Rust`日报小组
- 后期编辑： 张汉东

---

## 建立 Async Rust 的共同愿景

2021年3月18日·Niko Matsakis 代表 [Async Foundations Working Group](https://rust-lang.github.io/wg-async-foundations/)


在 [异步基础工作组](https://rust-lang.github.io/wg-async-foundations/) 认为 Rust 能够成为最热门的选择之一为构建分布式系统，从嵌入式设备到基础云服务。无论他们将其用于什么，我们都希望所有开发人员都喜欢使用 Async Rust。为了实现这一点，我们需要将 Async Rust 移至目前的“MVP”状态之外，并使所有人都可以使用它。

我们正在开展合作，为 Async Rust 构建共享的 [愿景文档](https://rust-lang.github.io/wg-async-foundations/vision.html#-the-vision) 。`我们的目标是让整个社区参与到集体的想象中`：我们如何才能使使用异步 I/O 的端到端体验不仅是一种务实的选择，而且是一种快乐的选择？

[点此阅读该文中文翻译](./async-vision-doc.md)


## Rust 1.51 稳定版发布

$ rustup update stable

该版本主要是带来了 ：

1. Const Generics MVP  ： https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html 
2. 顺便  std::array::IntoIter 也稳定了

```rust
pub struct IntoIter<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    alive: Range<usize>,
}

impl<T, const N: usize> IntoIter<T, N> {

}
```

3. 新的 cargo crate 依赖管理机制。 具体查看 RFC 2957。 简单来说，通过设置 resolver="2" 来告诉 cargo 启用新的解析 features 方法，从而解决当前因为cargo 默认合并features带来的问题。概述：

- 对于 dev dependencies： 当包（package）作为常规依赖项和开发依赖项共享时，仅当当前构建包含开发依赖项时，才启用开发依赖项features
- Host Dependencies ：当包作为 常规依赖 和 构建依赖或proc-macro共享时，用于常规依赖的features 将独立于构建依赖或proc-macro。
- Target Dependencies: 当包在构建图中多次出现，并且其中一个实例是特定于目标的依赖项时，仅当当前正在构建目标时，才启用特定于目标的依赖项的features。

不过这样可能会导致编译时间加长（因为可能多次编译同一个crate），更详细内容可以看 Cargo Guide 的 "Feature Resolver" 小节。

```rust
[package]
resolver = "2"
# Or if you're using a workspace
[workspace]
resolver = "2"
```

4.  针对 MacOS 平台对 Debug 模式构建时间做了优化。去掉了之前通过 dsymutil 工具将debug信息收集到.dSYM目录下的方式，而使用新的方式，从而减少debuginfo的构建时间，并显着减少所使用的磁盘空间量。但还期待macOS 用户的更多构建报告。

```rust
[profile.dev]
split-debuginfo = "unpacked"
```

这样设置就可以启用新的行为

5. 稳定了很多 API ，就不细说了。值得一提的是 `task::Wake`现在稳定了。

[https://blog.rust-lang.org/2021/03/25/Rust-1.51.0.html](https://blog.rust-lang.org/2021/03/25/Rust-1.51.0.html)


## Rust 2021 Edition 计划10月21号发布

Rust 采用每六周一个小版本和每三年一个 Edition 版本的方式来迭代更新。相比于 2018 Edition，2021 Edition 会是一个相对小的版本，官方计划于 2021年10月21号（1.56）正式发布。目前并没有完全确定下来哪些功能将纳入 2021 Edition，但有部分特性是已经确定好的了，这些特性包括：

Prelude 加入新的 `trait`：`TryFrom / TryInto`, `FromIterator`

更 ergonomic 的闭包变量捕获规则。

现在的闭包变量捕获非常严格，就算你只引用了单个 struct 的字段，它也会把整个 struct 捕获进来。新的规则会做到尽量小范围的捕获变量，比如下面两个例子在 2018 Edition 编译不通过，但是 2021 Edition 是可以的：

```rust
let _a = &mut foo.a;
|| &mut foo.b; // (Edition 2018) Error! cannot borrow `foo`

let _a = &mut foo.a;
move || foo.b; // (Edition 2018) Error! cannot move `foo`
改善 or 模式匹配
// 以前需要这么写的或规则匹配：
Some(Enum::A) | Some(Enum::B) | Some(Enum::C) | Some(Enum::D) => ..

// 2021 Edition 之后可以写成这样了！
Some(Enum::A | Enum::B | Enum::C | Enum::D) => ..
```

统一 macro_rules 定义的宏的默认可见性，移除`#[macro_export]` 和 `#[macro_use]` 宏：
Rust 所有类型可见性默认都是私有，只有加 pub 或 pub($PATH) 才能修改为公开可见，而 macro_rules 定义的宏却不是这样的，你需要使用 `#[macro_export]` 才能让这个宏公开。从 2021 Edition 开始，macro_rules 定义的宏默认为私有，同样需要加 pub 或 pub($PATH) 才能修改可见性。`#[macro_export] `和 `#[macro_use]` 这两个宏就没什么用了，直接移除。

[链接](https://blog.rust-lang.org/inside-rust/2021/03/04/planning-rust-2021.html)


## Rust 编译器后端升级为 LLVM 12

[链接](https://github.com/rust-lang/rust/pull/81451)

## gloo: 一个官方的 rustwasm 项目寻找 maintainer

gloo 是 rustwasm 下的一个官方项目 (801星) , 由于作者不能再维护, 所以在寻找一个maintainer. 感兴趣的小伙伴可以尝试联系一下.

[原始issue地址](https://github.com/rustwasm/gloo/issues/119)

## Miri运行在wasm上！

现在已经有方法可以将miri编译到wasm了。

[issue ](https://github.com/rust-lang/miri/issues/722#issuecomment-795763551)