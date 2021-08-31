# Rust 技巧篇 ｜ 用 `#[doc]` 属性宏改善你的文档注释

编辑： 张汉东

> 说明： 本文是在原文基础上的梳理，也引入了其他内容。

---

## 属性宏的新特性介绍

从 [Rust 1.54](https://blog.rust-lang.org/2021/07/29/Rust-1.54.0.html) 开始，属性宏增加了类函数宏的支持。

类函数的宏可以是基于`macro_rules！`的声明宏，也可以是像`macro！(...)`那样被调用的过程宏。这对于文档注释相当有好处。如果你的项目的 README 是一个很好的文档注释，你可以使用`include_str!`来直接纳入其内容。以前，各种变通方法允许类似的功能，但从`1.54`开始，这就更符合人体工程学了。

```rust
#![doc = include_str!("README.md")]
```

如果你看过一些 Rust 开源项目，你应该在 `lib.rs` 中看到过一大堆文档注释吧？这些注释太长，导致真正的代码被挤到到最下面。有了这个功能，就可以解决这类问题了。

```rust
macro_rules! make_function {
    ($name:ident, $value:expr) => {
        // 这里使用 concat! 和 stringify! 构建文档注释
        #[doc = concat!("The `", stringify!($name), "` example.")]
        ///
        /// # Example
        ///
        /// ```
        #[doc = concat!(
            "assert_eq!(", module_path!(), "::", stringify!($name), "(), ",
            stringify!($value), ");")
        ]
        /// ```
        pub fn $name() -> i32 {
            $value
        }
    };
}

make_function! {func_name, 123}
```

也可以像这样，在属性中嵌入宏调用来构建文档注释。可以对比下展开后的代码：

```rust
///The `func_name` example.
///
/// # Example
///
/// ```
///assert_eq!(doc_attr::func_name(), 123);
/// ```
pub fn func_name() -> i32 {
    123
}

```

这样的话，文档也可以复用了。当然你也可以扩展出其他用法。 

## 其他用法

在 [国外社区朋友的这篇文章](https://blog.guillaume-gomez.fr/articles/2021-08-03+Improvements+for+%23%5Bdoc%5D+attributes+in+Rust)中，他列举了一些应用场合。


### 用文档测试扩展测试能力

Rust 的文档测试相当灵活，假如你写了一些函数或者宏，你想确保它在输入某个值的时候不能编译。使用单元测试比较麻烦，但是用文档测试就很方便了。 

```rust
/// ```compile_fail
#[doc = include_str!("compile_fail.rs")]
/// ```
mod doc_test {}
```

你可以把相关测试放到 `complile_fail.rs` 中，然后使用 文档注释将其包括进来，这样在 cargo 执行测试的时候就可以进行测试了。而且对于 Rust 代码整体增加了可读性和可维护性。同样，你也可以检查 panic 等。

我们也不希望这种注释出现在最终用户的文档中，或者是编译文件中，所以需要使用 `cfg(doctest)` 来将其隐藏：

```rust 
#[cfg(doctest)]
/// ```compile_fail
#[doc = include_str!("compile_fail.rs")]
/// ```
mod doc_test {}
```


