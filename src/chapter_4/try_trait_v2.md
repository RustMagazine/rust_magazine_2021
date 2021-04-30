---
pub_date: Thu, 30 Apr 2021 18:00:00 GMT
description: Try trait V2

---

# RFC 介绍 | try-trait v2

编辑： 张汉东

> 编者按：
>
> [RFC 3058 try_trait_v2](https://rust-lang.github.io/rfcs/3058-try-trait-v2.html) 被合并了，这意味着，`?` 操作符的行为在设计上已经趋于稳定，只等待它的实现。
> 
> 在 RustFriday 飞书群线上沙龙 第四期 也讲过这个 RFC ，可以观看录播：[https://www.bilibili.com/video/BV1xy4y147Ve/](https://www.bilibili.com/video/BV1xy4y147Ve/)
> 
> Rust 中文社群 飞书群 邀请你加入：[https://applink.feishu.cn/TeLAcbDR ](https://applink.feishu.cn/TeLAcbDR)


---

## 背景介绍

目前 Rust 允许通过 `?` 操作符可以自动返回的 `Result<T, E>` 的 `Err(e)` ，但是对于 `Ok(o)` 还需要手动包装。

比如：

```rust
fn foo() -> Result<PathBuf, io::Error> {
    let base = env::current_dir()?;
    Ok(base.join("foo"))
}
```

那么这就引出了一个 术语： Ok-Wrapping 。很明显，这个写法不够优雅，还有很大的改进空间。

因此 Rust 官方成员 withoutboats 开发了一个库 [fehler](https://github.com/withoutboats/fehler)，引入了一个 throw 语法。

用法如下：

```rust
#[throws(i32)]
fn foo(x: bool) -> i32 {
    if x {
        0
    } else {
        throw!(1);
    }
}

// 上面foo函数错误处理等价于下面bar函数

fn bar(x: bool) -> Result<i32, i32> {
    if x {
        Ok(0)
    } else {
        Err(1)
    }
}
```

通过 throw 宏语法来帮助开发者省略 Ok-wrapping 和 Err-wrapping 的手动操作。这个库一时在社区引起了一些讨论。它也在促进着 Rust 错误处理体验提升。

于是错误处理就围绕着 Ok-wrapping 和 Err-wrapping 这两条路径，该如何设计语法才更加优雅为出发点。

## `try`块 和 `try trait` 的区别

当前 Nightly Rust 中也提供了一个 `try` 块语法，要使用 `#![feature(try_blocks)]`。

用法如下：

```rust

#![feature(try_blocks)]
use std::path::PathBuf;

fn foo() -> Result<PathBuf, std::io::Error> {
    try {
        let base = std::env::current_dir()?;
        base.join("foo")
    }
}
```

`try` 块在 `Ok` 情况下自动 Ok-wrapping 返回 `Ok(PathBuf)`，而问号操作符返回 `Err(io::Error)`。所以，这个 `try` 块语法 和 `try trait` 是相互配合的。

所以：

- `try` 块 （try-block）是控制 Ok-wrapping
- `try trait` 是控制问号操作符的行为 Err-wrapping

## try-trait RFC 导读

经过很久很久的讨论，try-trait-v2 RFC 被合并了，意味着一个确定的方案出现了。

**在这个方案中，引入了一个新类型：`ControlFlow`。**


```rust
enum ControlFlow<B, C = ()> {
    /// Exit the operation without running subsequent phases.
    Break(B),
    /// Move on to the next phase of the operation as normal.
    Continue(C),
}

impl<B, C> ControlFlow<B, C> {
    fn is_break(&self) -> bool;
    fn is_continue(&self) -> bool;
    fn break_value(self) -> Option<B>;
    fn continue_value(self) -> Option<C>;
}
```


`ControlFlow` 中包含了两个值：

- `ControlFlow::Break`，表示提前退出。但不一定是`Error`  的情况，也可能是 `Ok`。 
- `ControlFlow::Continue`，表示继续。


**还引入了一个新的trait：`FromResidual`**

```rust
trait FromResidual<Residual = <Self as Try>::Residual> {
    fn from_residual(r: Residual) -> Self;
}
```

Residual 单词有 「剩余」之意，因为 要把 Result / Option/ ControlFlow 之类的类型，拆分成两部分（两条路径），用这个词就好理解了。

而  `Try` trait 继承自 `FromResidual` trait ：

```rust
pub trait Try: FromResidual {
    /// The type of the value consumed or produced when not short-circuiting.
    type Output;

    /// A type that "colours" the short-circuit value so it can stay associated
    /// with the type constructor from which it came.
    type Residual;

    /// Used in `try{}` blocks to wrap the result of the block.
    fn from_output(x: Self::Output) -> Self;

    /// Determine whether to short-circuit (by returning `ControlFlow::Break`)
    /// or continue executing (by returning `ControlFlow::Continue`).
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output>;
}

pub trait FromResidual<Residual = <Self as Try>::Residual> {
    /// Recreate the type implementing `Try` from a related residual
    fn from_residual(x: Residual) -> Self;
}
```

所以，在 `Try` trait 中有两个关联类型：

- `Output`，如果是 Result 的话，就对应 Ok-wrapping 。
- `Residual`，如果是 Result 的话，就对应 Err-wrapping 。

所以，现在 `?` 操作符的行为就变成了：

```rust

match Try::branch(x) {
    ControlFlow::Continue(v) => v,
    ControlFlow::Break(r) => return FromResidual::from_residual(r),
}

```

然后内部给 Rusult 实现 `Try` ：

```rust
impl<T, E> ops::Try for Result<T, E> {
    type Output = T;
    type Residual = Result<!, E>;

    #[inline]
    fn from_output(c: T) -> Self {
        Ok(c)
    }

    #[inline]
    fn branch(self) -> ControlFlow<Self::Residual, T> {
        match self {
            Ok(c) => ControlFlow::Continue(c),
            Err(e) => ControlFlow::Break(Err(e)),
        }
    }
}

impl<T, E, F: From<E>> ops::FromResidual<Result<!, E>> for Result<T, F> {
    fn from_residual(x: Result<!, E>) -> Self {
        match x {
            Err(e) => Err(From::from(e)),
        }
    }
}
```

再给 Option 实现 `Try` ：

```rust
impl<T> ops::Try for Option<T> {
    type Output = T;
    type Residual = Option<!>;

    #[inline]
    fn from_output(c: T) -> Self {
        Some(c)
    }

    #[inline]
    fn branch(self) -> ControlFlow<Self::Residual, T> {
        match self {
            Some(c) => ControlFlow::Continue(c),
            None => ControlFlow::Break(None),
        }
    }
}

impl<T> ops::FromResidual for Option<T> {
    fn from_residual(x: <Self as ops::Try>::Residual) -> Self {
        match x {
            None => None,
        }
    }
}
```

再给 Poll 实现 `Try` : 

```rust
impl<T, E> ops::Try for Poll<Result<T, E>> {
    type Output = Poll<T>;
    type Residual = <Result<T, E> as ops::Try>::Residual;

    fn from_output(c: Self::Output) -> Self {
        c.map(Ok)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Poll::Ready(Ok(x)) => ControlFlow::Continue(Poll::Ready(x)),
            Poll::Ready(Err(e)) => ControlFlow::Break(Err(e)),
            Poll::Pending => ControlFlow::Continue(Poll::Pending),
        }
    }
}

impl<T, E, F: From<E>> ops::FromResidual<Result<!, E>> for Poll<Result<T, F>> {
    fn from_residual(x: Result<!, E>) -> Self {
        match x {
            Err(e) => Poll::Ready(Err(From::from(e))),
        }
    }
}

impl<T, E> ops::Try for Poll<Option<Result<T, E>>> {
    type Output = Poll<Option<T>>;
    type Residual = <Result<T, E> as ops::Try>::Residual;

    fn from_output(c: Self::Output) -> Self {
        c.map(|x| x.map(Ok))
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Poll::Ready(Some(Ok(x))) => ControlFlow::Continue(Poll::Ready(Some(x))),
            Poll::Ready(Some(Err(e))) => ControlFlow::Break(Err(e)),
            Poll::Ready(None) => ControlFlow::Continue(Poll::Ready(None)),
            Poll::Pending => ControlFlow::Continue(Poll::Pending),
        }
    }
}

impl<T, E, F: From<E>> ops::FromResidual<Result<!, E>> for Poll<Option<Result<T, F>>> {
    fn from_residual(x: Result<!, E>) -> Self {
        match x {
            Err(e) => Poll::Ready(Some(Err(From::from(e)))),
        }
    }
}
```

再给 ControlFlow 实现 `Try` : 

```rust
impl<B, C> ops::Try for ControlFlow<B, C> {
    type Output = C;
    type Residual = ControlFlow<B, !>;

    fn from_output(c: C) -> Self {
        ControlFlow::Continue(c)
    }

    fn branch(self) -> ControlFlow<Self::Residual, C> {
        match self {
            ControlFlow::Continue(c) => ControlFlow::Continue(c),
            ControlFlow::Break(b) => ControlFlow::Break(ControlFlow::Break(b)),
        }
    }
}

impl<B, C> ops::FromResidual for ControlFlow<B, C> {
    fn from_residual(x: <Self as ops::Try>::Residual) -> Self {
        match x {
            ControlFlow::Break(r) => ControlFlow::Break(r),
        }
    }
}
```

**这就实现了 错误类型转换 大统一。** 

我在 2017 年给官方提过一个 Issue: [why havn't implemented Error trait for std::option::NoneError ?](https://github.com/rust-lang/rust/issues/46871)，是因为当时引入了 `NoneError，但没有个` `NoneError` 实现 `Error` trait，所以无法在 Result 和 Option 之间无缝转换。

现在如果这个 RFC 实现，Result/Option 之间可以无缝转换，而完全不需要 `NoneError` 了，也许 `NoneError`就可以移除了。甚至在写异步 poll 方法的时候，也会变得非常简单了。


## 最后再看一个示例：

```rust
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ResultCode(pub i32);
impl ResultCode {
    const SUCCESS: Self = ResultCode(0);
}

use std::num::NonZeroI32;
pub struct ResultCodeResidual(NonZeroI32);

impl Try for ResultCode {
    type Output = ();
    type Residual = ResultCodeResidual;
    fn branch(self) -> ControlFlow<Self::Residual> {
        match NonZeroI32::new(self.0) {
            Some(r) => ControlFlow::Break(ResultCodeResidual(r)),
            None => ControlFlow::Continue(()),
        }
    }
    fn from_output((): ()) -> Self {
        ResultCode::SUCCESS
    }
}

impl FromResidual for ResultCode {
    fn from_residual(r: ResultCodeResidual) -> Self {
        ResultCode(r.0.into())
    }
}

#[derive(Debug, Clone)]
pub struct FancyError(String);

impl<T, E: From<FancyError>> FromResidual<ResultCodeResidual> for Result<T, E> {
    fn from_residual(r: ResultCodeResidual) -> Self {
        Err(FancyError(format!("Something fancy about {} at {:?}", r.0, std::time::SystemTime::now())).into())
    }
}

```
