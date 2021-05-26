---
pub_date: Sat, 27 Feb 2021 16:00:00 GMT
description: Deciphering the stable version of Rust 1.50

---

# 解读 Rust 1.50 稳定版

作者：张汉东 / 后期编辑： 张汉东

---

2021 年 2 月 11 号，[Rust 1.50 稳定版发布](https://blog.rust-lang.org/2021/02/11/Rust-1.50.0.html)。1.50 版更新包括：

- 语言级特性
- 编译器
- 标准库
- 稳定的 API 
- Cargo 相关
- 其他
- 兼容性提示


以下挑一些重点讲解。

# 语言级特性

### [常量泛型 `[CONST; N]`](https://rust-lang.github.io/rfcs/2000-const-generics.html) 进一步得到完善：

- 常量泛型数组实现了 `ops::Index` 和 `ops::IndexMut`。
- 值重复的常量数组`[x; N]`现在支持 常量值作为 x ，无论 x 是否实现 `Copy`。

 Rust 有一种内置数组类型`[T; LEN]`，但是这个 `LEN`一直无法支持泛型，所以这类数组就沦为了二等公民。比如 `[0,0,0]`和`[0,0,0,0]`不是同一个类型。所谓一等公民应该是不管数组长度如何，至少可以用同一个类型表示。为了提升这个数组类型，就引入了常量泛型的支持。`[CONST; N]` 是从 1.38 版本开始筹划，在 Rust 1.38~1.46 版本内，引入了一个`std::array::LengthAtMost32`来限制默认`[T; LEN]`的长度不能超过 32 。到 Rust 1.47 版本，首次在内部引入了 `[CONST; N]` 的支持。

直到 `Rust 1.50 `版本，进一步对`[CONST; N]` 功能进行了完善。

对常量泛型数组实现了 `ops::Index` 和 `ops::IndexMut`：

```rust
fn second<C>(container: &C) -> &C::Output
where
    C: std::ops::Index<usize> + ?Sized,
{
    &container[1]
}

fn main() {
    let array: [i32; 3] = [1, 2, 3];
    assert_eq!(second(&array[..]), &2); // 之前必须转成切片才可以
    assert_eq!(second(&array), &2); // 现在直接传引用就可以了
}
```

值重复的常量数组`[x; N]`现在支持 常量值作为 x ：

```rust
fn main() {
    // 这行代码是不允许的，因为`Option<Vec<i32>>` 没有实现 `Copy`。
    let array: [Option<Vec<i32>>; 10] = [None; 10];

  	// 但是，现在改成 `const` 定义就可以了
    const NONE: Option<Vec<i32>> = None;
    const EMPTY: Option<Vec<i32>> = Some(Vec::new());

    // 虽然没有实现`Copy`，但是现在可以重复`const`的值了。
    let nones = [NONE; 10];
    let empties = [EMPTY; 10];
}
```

这样写起来可能比较麻烦，但是在随后 [RFC 2920: inline const ](https://github.com/rust-lang/rfcs/blob/master/text/2920-inline-const.md)功能稳定后，就可以写成下面这种形式了：

```rust
fn main() {
    // 这行代码是不允许的，因为`Option<Vec<i32>>` 没有实现 `Copy`。
    let array: [Option<Vec<i32>>; 10] = [None; 10];

    // 虽然没有实现`Copy`，但是现在可以重复`const`的值了。
    let nones : [Option<Vec<i32>>; 10] = [const {None}; 10];
    let empties : [Option<Vec<i32>>; 10]  = [const {Some(Vec::new())}; 10];
}
```

其实可以 Rust 本可以做到下面这种形式：

```rust
fn main() {
    // 这行代码是不允许的，因为`Option<Vec<i32>>` 没有实现 `Copy`。
    let array: [Option<Vec<i32>>; 10] = [None; 10];

    // 虽然没有实现`Copy`，但是现在可以重复`const`的值了。
    let nones : [Option<Vec<i32>>; 10] = [None; 10];
    let empties : [Option<Vec<i32>>; 10]  = [Some(Vec::new()); 10];
}
```

上面`None`和`Some(Vec::new())`可以自动被编译器提升为常量，但这样可能为用户带来困扰，对于一些不能被自动提升为常量的类型，还需要用户去学习一大堆[常量提升规则](https://github.com/rust-lang/const-eval/blob/master/promotion.md#promotability)，并且使用 `const fn`等功能来定义常量。倒不如显示地加一个 const 块表达式来直接标注更好。

另外，关于`#![feature(min_const_generics)]`[将在 Rust 1.51 中稳定](https://github.com/rust-lang/rust/pull/79135)，预计 `2021-03-25`。



### 将共用体(`union`)中[`ManualDrop`](https://doc.rust-lang.org/stable/std/mem/struct.ManuallyDrop.html?search=)类型字段的分配视为安全

```rust
// Rust 1.49 新增特性，允许 union 中使用 ManuallyDrop
use core::mem::ManuallyDrop;

union MyUnion {
    f1: u32,
    f2: ManuallyDrop<String>,
}

fn main() {
    let mut u = MyUnion { f1: 1 };

    // These do not require `unsafe`.
    u.f1 = 2;
    u.f2 = ManuallyDrop::new(String::from("example"));
}

```

在`Union` 类型 中 `Copy`或`ManuallyDrop`的字段不会调用析构函数，所以不必加 `unsafe`块。

进一步，当 Drop 一个 Union 类型的时候，需要手工去实现 Drop。因为 共用体 本身的特性，它不会知道该 drop 哪个字段才是安全的，所以才需要字段都是 `Copy` 或 `ManuallyDrop`的。

```rust
#![feature(untagged_unions)]
use std::mem::ManuallyDrop;
use std::cell::RefCell;

union U1 {
    a: u8
}

union U2 {
    a: ManuallyDrop<String>
}

union U3<T> {
    a: ManuallyDrop<T>
}

union U4<T: Copy> {
    a: T
}

// 对于 ManuallyDrop 之外的 非 Copy 类型，目前还是 unstable，需要 `#![feature(untagged_unions)]` 特性门支持。
union URef {
    p: &'static mut i32,
}

// RefCell 没有实现 Drop ，但是它是非 Copy 的
union URefCell { // field that does not drop but is not `Copy`, either
    a: (RefCell<i32>, i32),
}

fn generic_noncopy<T: Default>() {
    let mut u3 = U3 { a: ManuallyDrop::new(T::default()) };
    u3.a = ManuallyDrop::new(T::default()); // OK (assignment does not drop)
    
}

fn generic_copy<T: Copy + Default>() {
    let mut u3 = U3 { a: ManuallyDrop::new(T::default()) };
    u3.a = ManuallyDrop::new(T::default()); // OK
    
    let mut u4 = U4 { a: T::default() };
    u4.a = T::default(); // OK
}

fn main() {
    let mut u1 = U1 { a: 10 }; // OK
    
    u1.a = 11; // OK

    let mut u2 = U2 { a: ManuallyDrop::new(String::from("old")) }; // OK
    u2.a = ManuallyDrop::new(String::from("new")); // OK (assignment does not drop)

    let mut u3 = U3 { a: ManuallyDrop::new(0) }; // OK
    u3.a = ManuallyDrop::new(1); // OK

    let mut u3 = U3 { a: ManuallyDrop::new(String::from("old")) }; // OK
    u3.a = ManuallyDrop::new(String::from("new")); // OK (assignment does not drop)
    
}
```



# 编译器

- [添加对`armv5te-unknown-linux-uclibcgnueabi`目标的内置支持](https://github.com/rust-lang/rust/pull/78142)。 基于ARMv5TE指令集的，你可以认为是ARM处理器，但实际上已经有原来intel的很多技术在里面进行了修改。
- [在ARM Mac上添加对Arm64 Catalyst的支持](https://github.com/rust-lang/rust/pull/77484)。苹果很快将发布基于ARM64的Mac，macOS应用将使用在ARM上运行的Darwin ABI。 该PR增加了对ARM Macs上Catalyst应用程序的支持：为darwin ABI编译的iOS应用程序。
- [修复 FreeBSD 上的链接问题](https://github.com/rust-lang/rust/pull/79484)。在FreeBSD上，有时会出现一个问题，即使基本系统中包含`lld`，由于 Rust 未找到链接程序，链接 Rust 程序也会失败。 这似乎主要影响裸机/交叉编译，例如`wasm`构建和`arm / riscv`裸机工作（例如，尝试编译时）。 在`Linux`和其他操作系统上，启用了用于构建 Rust 的完整工具，因此没有链接问题。 如果使用这些选项正确构建了 Rust，则此PR应该可以在FreeBSD上启用完整的功能。

除了这三个，还有其他 target 支持，查看[Platform Support 页面](https://forge.rust-lang.org/release/platform-support.html)。



# 标准库

### [为` proc_macro::Punct `增加 `PartialEq<char> `](https://github.com/rust-lang/rust/pull/78636)

用于在宏中判断特殊标点符号更加方便。比如：

```rust
// ...
else if let TokenTree::Punct(ref tt) = tree {
  if tt.as_char() == '$' {
    after_dollar = true;
    return None;
  }
  // ...
 if p.as_char() == '>' { 
  // ...
if tt.as_char() == '=' { 
```



### Unix 平台优化：`Option<File>` 大小等价于 `File`

在Unix平台上，Rust 的文件仅由系统的整数文件描述符组成，并且它永远不会为`-1`！ 返回文件描述符的系统调用使用`-1`表示发生了错误（检查errno），因此`-1`不可能是真实的文件描述符。 从`Rust 1.50`开始，此niche（特定生态场景）被添加到类型的定义中，因此它也可以用于布局优化。 因此，`Option <File>`现在将具有与`File`本身相同的大小！



# 兼容性变更



### [过期 compare_and_swap 方法](https://github.com/rust-lang/rust/pull/79261)

推荐使用 `compare_exchange` 和 `compare_exchange_weak`。过期这个cas方法一方面是为了和` cpp` 的  `compare_exchange_strong` 和  `compare_exchange_weak` 对应，另一方面也是为了避免使用这个cas在 arm 架构下产生不必要的指令，因为有 cas 的时候，很多人可能会直接使用 cas，从而在 ARM 下产生不必要的指令。

> ARM 架构实现LL/SC对(load-linked/store-conditional) ，可以基于它们实现 cas。Load-linked（LL） 运算仅仅返回指针地址的当前变量值，如果指针地址中的内存数据在读取之后没有变化，那么 Store-conditional（SC）操作将会成功，它将LL读取 指针地址的存储新的值，否则，SC将执行失败。

>  通过LL/SC对实现的CAS并不是一个原子性操作，但是它确实执行了原子性的CAS，目标内存单元内容要么不变，要么发生原子性变化。由于通过LL/SC对实现的CAS并不是一个原子性操作，于是，该CAS在执行过程中，可能会被中断。因此`C++11`标准中添入两个`compare_exchange`原语: `compare_exchange_weak`和`compare_exchange_strong`。即使当前的变量值等于预期值，这个弱的版本也可能失败，比如返回false。可见任何weak CAS都能破坏CAS语义，并返回false，而它本应返回true。而Strong CAS会严格遵循CAS语义。

> 何种情形下使用Weak CAS，何种情形下使用Strong CAS呢？通常执行以下原则：

> 倘若CAS在循环中（这是一种基本的CAS应用模式），循环中**不存在**成千上万的运算（循环体是轻量级和简单的），使用`compare_exchange_weak`。否则，采用强类型的`compare_exchange_strong`。

因此，Rust 标准库过期 cas 方法，就是为了让开发者可以根据场景来判断使用 强还是弱的 cas 语义。而 标准库里的cas方法则只是对 `compare_exchange` 的包装，而 Rust 中 `compare_exchange` 对应 强CAS 语义，所以容易被滥用。

### [放弃对所有 cloudabi target 的支持](https://github.com/rust-lang/rust/pull/78439)

包括：

- aarch64-unknown-cloudabi
- armv7-unknown-cloudabi
- i686-unknown-cloudabi
- x86_64-unknown-cloudabi

因为 [CloudABI 不再被维护了](https://github.com/NuxiNL/cloudabi#note-this-project-is-unmaintained)，可以考虑 WASI 了，WASI 的一些概念就是受到 CloudABI 的启发，现在算是 CloudABI 的接班人了。

