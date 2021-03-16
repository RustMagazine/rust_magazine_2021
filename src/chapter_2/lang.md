# 本月简报 | Rust官方动态

- 来源：[Rust日报](https://rustcc.cn/section?id=f4703117-7e6b-4caf-aa22-a3ad3db6898f)
- 作者：`Rust`日报小组
- 后期编辑：张汉东


---

## 官宣，Rust基金会正式成立！

基金会初创白金成员包括：

AWS，Google, HUAWEI（华为）, Microsoft, Mozilla

官网地址：https://foundation.rust-lang.org/

相关阅读：

- [华为 | 可信编程 -- 华为引领Rust语言开发的实践和愿景](./huawei_rust.md)
- [Rust 语言圆桌年话 | 关于 Rust 语言基金会成立，你有什么想说的呢？](https://www.zhihu.com/question/443595816)



## Rust 1.50 稳定版发布

关于 Rust 1.50 详细解读，请跳转自 [解读 Rust 1.50 稳定版](./rust_1.50.md) 一文阅读。

## Rust语言团队二月份第一次会议

Rust 语言团队2月3号第一次召开了规划会议，并总结了会议纪要。从今以后，语言团队计划每个月的第一个星期三举行这样的会议。

举行规划会议的目的：检查我们正在进行的项目的状态，计划本月剩余时间的design meeting。

本次会议的主要内容：

1. async foundations: 异步基础

continued progress on polish, new traits (继续改进优化新的trait)

making plans to stabilize async functions in traits (制定稳定Trait中async函数的规划)

working on a vision document that lays out a multi-year vision for how async I/O should look/feel in Rust (编写一份愿景文档规划未来几年Rust 异步IO的愿景)

2. const generics 常量泛型

3. rfc 2229 ("minimal closure capture")
continued progress on the implementation, things are going well

we will likely add a capture! macro to use for migration; it would force the capture of a particular local variable (and not some subpath of it)

链接：[https://blog.rust-lang.org/inside-rust/2021/02/03/lang-team-feb-update.html](https://blog.rust-lang.org/inside-rust/2021/02/03/lang-team-feb-update.html)

会议纪要：[https://github.com/rust-lang/lang-team/blob/master/design-meeting-minutes/2021-02-03-Planning-Meeting.md#project-updates-and-discussion](https://github.com/rust-lang/lang-team/blob/master/design-meeting-minutes/2021-02-03-Planning-Meeting.md#project-updates-and-discussion)


## 关于 Const Generics MVP 你需要知道的


自从最初的 const 泛型 RFC 被接受以来已有3年多的时间了，Rust beta 现已提供 const 泛型的第一个版本！ 它将在`1.51` 版本中提供，该版本预计将于2021年3月25日发布。Const泛型是Rust最受期待的功能之一。

**什么是常量泛型**

常量泛型功能在 [解读 Rust 1.50 稳定版](./rust_1.50.md) 一文中也有介绍。

一个典型的示例：

```rust
struct ArrayPair<T, const N: usize> {
    left: [T; N],
    right: [T; N],
}

impl<T: Debug, const N: usize> Debug for ArrayPair<T, N> {
    // ...
}
```

其中，`[T; N]`就是常量泛型的应用。

即将在 1.51 稳定版发布的 const 泛型是一个受限制的版本，换句话说，此版本是 const 泛型的 MVP（最小可行产品）版本。因为做一个通用版本的 const 泛型十分复杂，目前还在完善中。

**MVP 版本限制如下：**

1. 目前唯一可以用作 const 泛型参数类型的类型是整数（即有符号和无符号整数，包括`isize`和`usize`）以及`char`和`bool`的类型。 这已经可以涵盖 const 泛型的主要用例，即对数组进行抽象。 将来会取消此限制，以允许使用更复杂的类型，例如`＆str`和 用户定义的类型。

2. const 参数中不能有复杂的泛型表达式。当前，只能通过以下形式的 const 参数实例化 const 参数：
    
    - 一个独立的常量参数。
    - 一个字面量。
    - 一个没有泛型参数的具体常量表达式（用{}括起来）。

    示例：
    ```rust
    fn foo<const N: usize>() {}

    fn bar<T, const M: usize>() {
        foo::<M>(); // ok: `M` 是常量参数
        foo::<2021>(); // ok: `2021` 是字面量
        foo::<{20 * 100 + 20 * 10 + 1}>(); // ok: 常量表达式不包括泛型
        
        foo::<{ M + 1 }>(); // error: 常量表达式包括泛型参数 `M`
        foo::<{ std::mem::size_of::<T>() }>(); // error: 常量表达式包括泛型参数 `T`
        
        let _: [u8; M]; // ok: `M` 是常量参数
        let _: [u8; std::mem::size_of::<T>()]; // error: 常量表达式包括泛型参数 `T`
    }
    ```

**标准库内部利用常量泛型的改进**

 伴随常量泛型在 1.51 稳定的还有 [`array::IntoIter`](https://doc.rust-lang.org/nightly/std/array/struct.IntoIter.html) ，它允许通过值而不是通过引用来迭代数组，从而解决了一个重大缺陷。 尽管仍然存在必须解决的向后兼容性问题，但仍在继续讨论是否可以直接为数组实现`IntoIterator`的可能性。 `IntoIter::new`是一种临时解决方案，可大大简化数组的处理。

 还有很多 API 在基于常量泛型改进，但还不会在 1.51 中稳定。


 ```rust
use std::array;
fn needs_vec(v: Vec<i32>) {
    // ...
}

let arr = [vec![0, 1], vec![1, 2, 3], vec![3]];
for elem in array::IntoIter::new(arr) {
    needs_vec(elem);
}
 ```

**未来计划**

1. 解决默认参数和常量泛型位置冲突的问题。

Rust 目前的泛型参数必须按特定顺序排列：生命周期（lifetime），类型（type），常量（const）。 但是，这会在尝试将默认参数与const参数一起使用时造成困难。为了使编译器知道哪个泛型参数，任何默认参数都必须放在最后。 接下来将解决这个问题。

2. 为自定义类型支持常量泛型

从理论上讲，要使一个类型有效作为const参数的类型，我们必须能够在编译时比较该类型的值。所以在 const泛型 RFC 中引入了结构相等的概念：本质上，它包括任何带有`＃[derive（PartialEq，Eq）]`且其成员也满足结构相等的类型。

3. 为复杂类型支持常量泛型

 Nightly Rust 提供了一个`feature(const_evaluatable_checked)`，该特性门启用了对 const 泛型的复杂表达式支持。

 目前的困难：

 ```rust
// 下面代码中两个表达式中的`N+1`是不同的，如果需要将它们看作相同，则需要检查的方法。这是面对复杂表达式中的一个难点。
fn foo<const N: usize>() -> [u8; N + 1] {
    [0; N + 1]
}

// 还需要处理常量泛型操作中存在的潜在错误的方法
// 如果没有办法在此处限制M的可能值，则在计算`0-1`时（在声明时未捕获），调用`generic_function::<0>()`会导致错误，因此对于下游用户可能会意外失败。
fn split_first<T, const N: usize>(arr: [T; N]) -> (T, [T; N - 1]) {
    // ...
}

fn generic_function<const M: usize>(arr: [i32; M]) {
    // ...
    let (head, tail) = split_first(arr);
    // ...
}

 ```

原文： [https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta](https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta)


## Rust 错误处理工作组计划将Error trait迁移至 core 模块

如果迁移之后，在no_std模式下也可以使用Error trait了。

链接：[https://github.com/rust-lang/rust/pull/77384#issuecomment-772835929](https://github.com/rust-lang/rust/pull/77384#issuecomment-772835929)