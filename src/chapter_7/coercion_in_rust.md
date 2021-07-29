# Rust 中的隐式类型转换

[原文](https://www.possiblerust.com/guide/what-can-coerce-and-where-in-rust) / 译者：[iamazy](https://github.com/iamazy)

Rust 支持多种[类型转换](https://doc.rust-lang.org/reference/type-coercions.html)，它可以**隐式**的将一种类型转换成另一种类型。与其他支持类型转换的语言一样，Rust 在易读性与易写性之间做了权衡。虽然对于 Rust 支持类型转换的清单是否最好存在分歧，但是学习类型转换是有意义的，因为有些是惯用的 (idiomatic) Rust 代码的核心。在本文中，我将描述 Rust 支持什么样的类型转换，以及在何处应用。

> 说明：本文介绍的所有类型转换都是隐式强制转换，将简称为强转。

## 什么是(类型)强转

在讨论类型强转之前，最好先弄清楚它的含义。Rust 支持多种类型转换的方式。`From`和`Into`特性用于库级别的可靠 (infallible) 转换。`TryFrom`和`TryInto`用于处理易出错的类型转换。`AsRef`，`AsMut`，`Borrow`和`ToOwned`提供了更多不同类型之间库级转换。但是，这些都是显式的。要执行转换，用户必须调用相关的函数。相比之下，强转是隐式的，这些转换的隐式属性意味着它们仅在其裨益依赖于易用性时才是可用的，并且隐式类型更改造成的潜在危害最小。使用`as`关键字完成的转换是显式的，并且允许的显式强转 (cast) 的种类比隐式强转 (coercion) 要多。

> **INFO 1** ，`transmute` - unsafe 转换  
> 标准库中有一个函数`std::mem::transmute`，它可以将任意类型转换成其他类型。该函数是`unsafe`的，因为它不能保证输入类型的有效位可以表示为输出类型的有效位。确保这两种类型兼容由用户决定。  
>
> 有一个致力于在 Rust 中开发“safe transmute”选项的工作，可以称之为“Project Safe Transmute”。他们的工作正在进行中，目的是当讨论的转化合法时，不需要使用`unsafe`版本的`transmute`(意味着源类型的有效位始终是目标类型中的有效位)。

## 有哪些类型强转 (coercion) 呢？

Rust 支持多种隐式的类型强转，尽管它们的定义都是非正式的，但是仍然需要进行一定程度的标准化。事实上，这些转换的长期规范预计将成为最终标准化过程的一部分，因为它们对于理解 Rust 的类型系统至关重要。

> **INFO 2**，标准化编程语言  
> 由于缺乏规范，Rust 不如 C/C++ 值得信赖的批评定期出现，在这里我要解释一下：首先，Rust 确实没有像 C/C++ 那样的规范(由国际标准组织发布和管理)，但这并不意味着 Rust 完全没有标准。
> Rust 有一个 [reference](https://doc.rust-lang.org/reference/introduction.html)，它编纂 (codify) 了该语言的大部分预期语义。它还具有管理语言变化的 [RFC 流程](https://github.com/rust-lang/rfcs)，以及监督 (oversee) 语言发展的团队。这些团队包括不安全代码指南工作组 (Unsafe Code Guidelines Working Group)，旨在更好的指定影响 unsafe Rust 代码的语义，要求和保证。该小组开发了`miri`，这是 Rust 中的 MIR (Mid-Level Internal Representation) 语言的解释器，它可以自动验证 MIR 代码是否与 Rust 语义中的“stacked borrows”模型(由 UCG WG 提出)一致。主要的 Rust 编译器也经过彻底测试，包括实验特性变更和新编译器版本的自动回归测试。  
> rustc 有一个可用的替代实现 - [mrustc](https://github.com/thepowersgang/mrustc)，尽管它通常不供最终用户使用。在实现支持 Rust 的 GNU 编译器集合方面还有更新的工作，称为“[rust-gcc](https://rust-gcc.github.io/)”。  
> [Ferrocene](https://ferrous-systems.com/blog/sealed-rust-the-pitch/) 一直在致力于获得 Rust 认证以用于关键的安全领域，包括航空电子(avionic) 和自动化行业。它由 Ferrous Systems (一家 Rust 咨询公司) 维护，其团队中包括主要的语言和社区贡献者。 
> 最终，正式指定的挑战以及证明 Rust 的保证已经在学术中得到解决，多个项目构建了模型，包括 Patina，Oxide，RustBelt，KRust 和 K-Rust。这些工作在 Alexa White 的硕士学位论文 [Towards a Complete Formal Semantics of Rust](https://digitalcommons.calpoly.edu/cgi/viewcontent.cgi?article=3804&context=theses) 中得到了研究和扩展，该论文是理解这些不同科研工作的一个很好的切入点。  
> 所有这些虽然不是标准，但是提高了 Rust 的水平，使其可以达到它所保证的能力。主 Rust 编译器中存在[可靠性漏洞](https://github.com/rust-lang/rust/labels/I-unsound)，这些漏洞会随着时间的推移被跟踪解决。如 [RFC 1122](https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md#soundness-changes) 所述，Rust 稳定性策略为修复可靠性漏洞的破坏性更改留下了一个例外。  
> 还值得注意的是，C 语言在 1972 年引入，而 C 语言标准的第一个正式非草案版本在 1989 年问世 (ANSI X3.159-1989 “编程语言 - C,” 现已撤回)。C++ 于 1985 年推出，其标准的第一个非草案版本于 1998 年发布 (ISO/IEC 14882:1998 “编程语言 — C++”)。  
> Rust 第一个公开版本是在 2010 年发布的，它在 2015 年 5 月 15 日对早期版本的语言进行了重大更改后，发布了 1.0 版本。从 1.0 发布之日算起，已经过去了 6 年。标准化需要时间，耐心是一种美德。

### 引用降级强转

引用降级强转是一种非常常见的强转操作，它可以将`&mut T`强转为`&T`。显然，这种强转总是安全的，因为不可变引用会受到更多的限制。它还允许借用检查器接受一些你可能认为不会编译或正常工作的代码。

一个引用降级强转的例子如下所示：

```rust
struct RefHolder<'a> {
    x: &'a i64,
}

impl<'a> RefHolder<'a> {
    fn new(x: &'a i64) -> RefHolder<'a> {
        RefHolder { x }
    }
}

fn print_num(y: &i64) {
    println!("y: {}", y);
}

fn main() {
    // Create `x`
    let mut x = 10;

    // Make sure `y` is `&mut i64`.
    let y = &mut x;

    // Package the downgraded reference into a struct.
    let z = RefHolder::new(y);
    
    // Print `y` downgrading it to an `&i64`.
    print_num(y);
    
    // Use the `z` reference again.
    println!("z.x: {}", z.x);
}
```

在该例中，我们可以看到`print_num`函数只需要`&i64`，但它传入了一个`&mut i64`。它可以正常运行是因为引用降级强转成了一个不可变引用。这也解决了给可变借用起别名的问题。`RefHolder`类型的构造函数也会发生同样的情况。

请注意该强转发生的次数。这里有一个类似的无法编译的例子。

```rust
struct RefHolder<'a> {
    x: &'a i64,
}

impl<'a> RefHolder<'a> {
    fn new(x: &'a i64) -> RefHolder<'a> {
        RefHolder { x }
    }
}

fn print_num(y: &i64) {
    println!("y: {}", y);
}

fn main() {
    // Create `x`
    let mut x = 10;

    // Make sure `y` is `&mut i64`.
    let y = &mut x;

    // Package the downgraded reference into a struct.
    //
    //---------------------------------------------------
    // NOTE: this is a _fresh_ reference now, instead of
    //       being `y`.
    //---------------------------------------------------
    let z = RefHolder::new(&mut x);
    
    // Print `y` and update it, downgrading it
    // to `&i64`.
    print_num(y);
    
    // Use the `z` reference again.
    println!("z.x: {}", z.x);
}
```

在该例中，即使引用在函数签名中降级，借用检查器仍然观察到在同一作用域内(针对同一内存)创建了两个可变引用，这是不被允许的。

### 解引用强转

下一种强转是 Rust 人体工程学 (ergonomics) 的基石 (cornerstone)。“解引用强转”是由两个特征的实现产生的强转：`Deref`和`DerefMut`。这些(特征)明确存在的目的是选择加入这种强转，让容器可以透明使用它们包含的类型(这些容器通常称为“智能指针”)。

这类特征定义如下所示：

```rust
pub trait Deref {
    type Target: ?Sized;

    pub fn deref(&self) -> &Self::Target;
}

pub trait DerefMut: Deref {
    pub fn deref_mut(&mut self) -> &mut Self::Target;
}
```

第一个特征`Deref`定义了一个类型，可以提供对其他“目标”类型的引用。这个目标是一个关联类型，而不是一个类型参数，因为每个“智能指针”应该只能被解引用为一种类型。如果它被定义为`Deref<Target>`，则任何类型都可以提供尽可能多的实现，因为它们可以提供内部类型，然后编译器根据某种机制来选择正确的内部类型。解引用强转的关键在于它们是隐式的，因此通常明确的类型注释会与解引用强转功能的好处相冲突。

`DerefMut`特征需要`Deref`作为超类型，这既可以让其访问 Target 关联类型，也可以确保`Deref`和`DerefMut`的目标类型始终一致。否则，你可能会在可变上下文中启用对一种类型的强转，而在不可变上下文中启用对另一种类型的强转。这种级别的灵活性为解引用强转增加了更多的复杂性，但没有明显的好处，因此它不可用。

这两个特征所需的方法`deref`和`deref_mut`，在实现这些特征的类型上调用方法时会被隐式调用。比如，在`Box<T>`上实现了`Deref<Target = T>`，因此可以透明地调用其包含类型的方法，这使得`Box<T>`比用户必须为每个操作显式访问其内容更符合人体工程学。

### 裸指针强转

Rust 的裸指针可能会从`*mut T`强转为`*const T`。尽管通过解引用来使用这些指针是 unsafe 的，并且受制于 Rust 对指针的[安全性要求](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)(即访问永远不会悬垂或未对齐)，但是这些转换是 safe Rust 的一部分(即不是为在 unsafe 上下文中使用而保留的功能)。

裸指针的强转示例如下所示：

```rust
#[derive(Debug)]
struct PtrHandle {
    ptr: *const i32,
}

fn main() {
    let mut x = 5;
    let ptr = &mut x as *mut i32;

    // The coercion happens on this line, where
    // a `*mut i32` is set as the value for a field
    // with type `*const i32`, coercing to that type.
    let handle = PtrHandle { ptr };

    println!("{:?}", handle);
}
```

> **INFO 3** 指针转换的安全性  
> Rust 还允许将`*const T`通过`as`转换成`*mut T`。  
> 虽然允许将`*const T`转换为`*mut T`似乎让人感到惊讶，但有时这种转换是必要的。例如，FFI 代码可能会从`Box::into_raw`中创建一个`*mut T`，但只想为 API 的 C 使用者提供一个`*const T`。因此 FFI 接口提供的等效删除函数需要将`*const T`作为参数，将其转换回`*mut T`以将其传递给`Box::from_raw`，从而使 Rust 在函数结束时对`Box`进行释放。  
> 虽然指针出处 (provenance) 的细节意味着这种转换并不总是未定义的行为，但如果指针的原始出处不是可变的，则它可能是未定义的行为。换句话说，如果一个值最初是`*mut T`类型，它可以在将来用作`*mut T`，即使类型在此期间 (interim) 被转换为`*const T`。

### 引用与裸指针强转

你可以将`&T`转换为`*const T`，将`&mut T`转换为`*mut T`。尽管产生的裸指针只能在 unsafe 的代码块中解引用，但是这些强转是 safe 的。

和上一个例子类似，但是这一次是将引用转换成指针而不是改变指针类型的可变性。

```rust
// Notice that these coercions work when
// generic types are present too.
#[derive(Debug)]
struct ConstHandle<T> {
    ptr: *const T,
}

#[derive(Debug)]
struct MutHandle<T> {
    ptr: *mut T,
}

fn main() {
    let mut x = 5;

    let c_handle = ConstHandle {
        // Coercing `&i32` into `*const i32`
        ptr: &x,
    };

    let m_handle = MutHandle {
        // Coercing `&mut x` into `*mut i32`
        ptr: &mut x,
    };

    println!("{:?}", c_handle);
    println!("{:?}", m_handle);
}
```

### 函数指针强转

闭包是函数加上其执行的上下文。这使得它们在许多情况下非常有用，但有时它们携带的这种额外状态会阻碍 (impede) 它们的使用，特别是没有实际的状态捕获时。在 Rust 中，除了编译时生成的无名闭包类型之外，还有函数指针类型表示没有上下文环境的函数。为了使闭包尽可能灵活，，当且仅当它们不从上下文中捕获任何变量时，闭包才会强制使用指针。

一个函数指针的示例如下：

```rust
// This function takes in a function pointer, _not_ a generic type
// which implements one of the function traits (`Fn`, `FnMut`, or
// `FnOnce`).
fn takes_func_ptr(f: fn(i32) -> i32) -> i32 {
    f(5)
}

fn main() {
    let my_func = |n| n + 2;

    // The coercion happens here, and is possible because `my_func`
    // doesn't capture any variables from its environment.
    println!("{}", takes_func_ptr(my_func));
}
```

请注意，在 Rust 中使用泛型实现`Fn`，`FnMut`，`FnOnce`特征的方式比使用函数指针要常见的多。如果你想要传递或存储从上下文中捕获的闭包，则需要使用这三种特征的其中一个。

### 子类型强转

令某些人惊讶的是，Rust 支持子类型强转。虽然 Rust 的类型系统通常被认为仅支持参数多态性，但实际上它也支持子类型多态性，适用于生存期。当一个生存期比另一个生存期更长时，Rust 中的生存期会彼此形成子类型关系。在这种情况下，生存期较长的是子类型，生存期较短的是超类型。因为在子类型多态中，任何子类型都可以代替超类型，这对于生存期就意味着当预期的生存期较短时，可以安全地使用较长的生存期。

这种强转意味着允许在**强转点**“缩短”生存期，因此可以使用更长的生存期来代替函数所需的较短边界。对于 Rustacean 来说，这样做的最终结果是编译器可以接受更多的程序。

像 Rust 一样支持参数和子类型多态的语言中出现的一个共同问题是范型类型的子类型关系如何与其范型参数的子类型关系相关联。该属性称为型变 (variance)。

范型类型有三个有用的变型。它们每一个都与特定的通用类型相关；如果一个类型有多个范型参数，它将对每个参数进行单独的型变确定。

- **协变 (Covariance)**：对于类型`A<T>`，如果`T`是`U`的子类型，则`A<T>`是`A<U>`的子类型。容器的子类型与其范型参数子类型相匹配。

- **逆变 (Contravariance)**：对于类型`A<T>`，如果`T`是`U`的子类型，则`A<U>`是`A<T>`的子类型。容器的子类型与其范型参数的子类型互逆。

- **不变 (Invariance)**：对于类型`A<T>`，在`A<T>`与`A<U>`之间不存在子类型关系。容器没有子类型。

在 Rust 中，由于子类型只存在于生存期中，并且生存期表示数据存活的时间，这就意味着：

- 协变类型的生存期允许比预期的更长(这些生存期允许“缩短”，这样不会有问题是因为引用的使用时间总是少于它们的有效时间)。

- 逆变类型的生存期允许延长(就像通过要求使用`'static`而不是`'a`的生存期，来避免函数指针携带引用类型)。

- 不变类型没有子类型关系，需要一个既不会缩短也不会延长的生存期。

也许一个带有子类型强转的逆变生存期示例可以帮助理解：

```rust
struct FnHolder {
    f: fn(&'static str) -> i32,
}

fn number_for_name<'a>(name: &'a str) -> i32 {
    match name {
        "Jim" => 32,
        _ => 5,
    }
}

fn main() {
    // Voila! A subtype coercion! In this case coercing a
    // lifetime in a contravariant context (the lifetime in
    // the function pointer type parameter) from `'a` to `'static`.
    //
    // `'static` is longer than `'a`, which in this case is safe
    // because it's always fine to make the function _less_ accepting.
    //
    // Once it's been assigned into the `FnHolder` type, it'll only
    // accept string literals (which have a `'static` lifetime).
    let holder = FnHolder { f: number_for_name };
    
    // The extra parentheses are part of the syntax for calling
    // functions as fields, to disambiguate between this and
    // calling a method on the `FnHolder` type.
    println!("{}", (holder.f)("Jim"));
}
```

### never 强转

Rust 类型系统中有一个特殊的类型 - never 类型(写作`!`)。此类型可以强转为其他所有类型，通常表示非终止 (non-termination)。例如，`unimplemented!`，`unreachable!`和`todo!`宏都返回`!`类型。`!`类型强转可以利用这些宏类型检查，如果它们在运行时中执行，则`!`被实现为当前线程有保证的 panic。退出当前进程的`std::process::exit`函数返回`!`也是出于相同的原因。

never 类型强转让程序可以使用 panic 或 exit 通过类型检查。

```rust
// Turn off some warnings about unreachable code.
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(dead_code)]

struct Value {
    x: bool,
    y: String,
}

fn never() -> ! {
    // `loop`s without some way to exit
    // like this have the `!` type, because
    // the expression (and, in this case,
    // the containing function) will never
    // terminate / return.
    loop {}
}

fn main() {
    let x = never();
    
    let v = Value {
        x: todo!("uhhh I haven't gotten to this"),
        y: unimplemented!("oh, not this either"),
    };
    
    // This program compiles because `never`,
    // `todo!`, and `unimplemented!` all return
    // the `!` type, which coerces into any type.
}
```

### 切片强转

切片强转是指从数组到切片的转换。它们是“未知大小强转”集合(以及特征对象强转和 trailing unsized 强转)中的一部分。之所以这么称呼它们，是因为它们涉及从有大小的类型(在编译时已知大小并实现了`Sized`特征的类型)到未知大小的类型(在编译时不知道其类型的大小，并且没有实现 `Sized`特征)。在切片强转的过程中，已知大小的类型为`[T; n]`(具有固定大小 n 的 T 数组)，未知大小的类型为`[T]`(T 数组的切片)。

切片强转发生的次数可能比你想到的还要多：

```rust
#[derive(Debug)]
struct SliceHolder<'a> {
    slice: &'a [i32],
}

fn main() {
    let nums = [1, 2, 3, 4, 5];
    
    // It may not look like, but there's a coercion here!
    //
    // The type of `&nums` is `&[i32; 5]`, which is coerced
    // into `&[i32]` to match the `slice` field on `SliceHolder`.
    let holder = SliceHolder { slice: &nums };
    
    println!("{:#?}", holder);
}
```

请注意，虽然也可以将`Vec<T>`强转为`&[T]`，但它不是切片强转，而是解引用强转。由于语言的历史原因，数组无法与 const 泛型一起使用(因为 const 泛型没有实现`Deref`)，因此需要特殊的强转将其静默转换成切片。

### 特征对象强转

特征对象是 Rust 的动态调度机制，并且特征对象强转的存在是为了可以轻松构建特征对象。这种强转从某种类型`T`转换成`dyn U`，其中`U`是被`T`实现的特征，并且`U`满足 Rust 的对象安全规则。[我们之前已经讨论过对象安全规则](https://www.possiblerust.com/pattern/3-things-to-try-when-you-can-t-make-a-trait-object#what-makes-a-trait-object-safe)，但要点是对象特征类型必须是可构造的(这意味着它在任何地方都不依赖在编译时不确定的泛型类型(泛型不包括关联的函数，没有引用`Self`的方式 - 在编译时期无法确定，并且在不包含`Self: Sized`边界的情况下，不包括按值获取`Self`的函数)。

一个函数通过特征对象强转进行调用的示例：

```rust
trait HasInt {
    fn get(&self) -> i32;
}

struct IntHolder {
    x: i32,
}

impl HasInt for IntHolder {
    fn get(&self) -> i32 {
        self.x
    }
}

fn print_int(x: &dyn HasInt) {
    println!("{}", x.get());
}

fn main() {
    let holder = IntHolder { x: 5 };
    // The coercion happens here, from `&IntHolder`
    // into `&dyn HasInt`.
    print_int(&holder);
}
```

### trailing unsized 强转

trailing unsized 强转意味着，如果类型`T`的最后一个字段是已知大小的且可以转换为未知大小的类型，并存在一个`U`类型，它是`T`类型但执行了最后一个字段的强转，那么`T`可以被强转为`U`。因为这个定义非常特殊，我们可以具体说明：

- `T`必须是一个结构体
- `T`的字段`A`必须可以强转为未知大小的`B`类型
- `T`的最后一个字段必须包含`A`
- `T`的其他字段不能包含`A`
- 如果`T`最后一个字段本身就是包含`A`的结构体，则该结构体必须可以强转为另一种类型，该类型包含未知大小的，用来替换`A`的`B`类型。

这比最初的解释更准确。本质上，当相关字段是最后一个字段时，允许在结构体内进行有限的 unsized 强转。

### 最小上限强转

有时 Rust 需要同时在多个强转点进行强转，这样它们都可以变成相同的类型。例如，这可能发生在`if/else`表达式中，其中条件的每个分支都返回一个需要强转的类型。在这种情况下，Rust 试图找到最通用的类型，这被称为“最小上限强转”。

该强转可以被以下情况触发：

1. 一系列`if/else`分支
2. 一系列`match`分支
3. 一系列数组元素
4. 在闭包中的一系列`返回值`
5. 在函数中的一系列`返回值`

执行此强转的过程是遍历每个系列中的每种类型，检查它们是否可以转换为先前确定的相同类型。如果可以，则继续。如果不能，则尝试找出一种类型`C`，可以将先前看到的类型`A`与最新的类型`B`都强转为类型`C`。最终的类型`C`被确定为该系列中所有表达式的类型。

### 传递性强转

Rust 支持传递性强转，如果类型`A`可以强转为类型`B`，并且类型`B`可以强转为类型`C`，则类型`A`可以强转为类型`C`。这个特性目前正在开发，可能不是总是有效。

## 哪里会发生强转

代码中发生类型强转的位置被称为“强转点 (coercion sites)”，Rust 中有多种类型的强转点。

### 强转点

首先是变量的声明，不管是通过`let`，`const`还是`static`进行声明。在这些情况下，如果在左侧显式声明变量的类型，则右侧将被强转为该类型。如果无法进行这种强转，则会发出编译器错误。

接下来是函数参数，其中实参被强转为型参的类型。在方法调用中，接收者类型(`Self`的类型)只能使用 unsized 强转。

然后你就可以拥有任何结构或枚举的字面量实例。这些数据类型中的字段被实例化的位置是强转点，实际类型被强转为整体数据类型声明中定义的正式类型。

### 强转传播的表达式

有些表达式被认为是“强转传播 (coercion propagating)”，这意味着它们会将强转检查传递给它们的子表达式。

数组字面量是强转传播的，并传播到数组字面量声明的每个元素定义中。如果与重复语法一起使用，该语法将重复给定次数的元素的初始定义。

元组在它们内部的每个单独表达式上也类似的进行强转传播。

如果表达式带有括号，则强转将传播到括号内的表达式。如果它被括号括起来，使它成为一个块，那么强转将传播到该块的最后一行。

## unsized 强转和强转点

与其他强转相比，unsized 强转(上述对切片，特征对象或 trailing unsized 类型的强转)可以在一个额外的上下文中发生。具体来说，如果有一个指向类型 T 的引用，裸指针或(有所有权的)指针，其中 T 具有对类型 U 的 usized 强转，则可以通过引用或指针类型进行强转。

这意味着以下强转点仅对 unsized 强转有效：

- `&T`到`&U`
- `&mut T`到`&mut U`
- `*const T`到`*const U`
- `*mut T`到`*mut U`
- `Box<T>`到`Box<U>`

这就是为什么上述切片强转的示例可以正常运行的原因！这种情况下的强转发生在引用之后，将[i32; 5]强转为[i32]。

## 结论

强转功能十分强大，因为它们是隐式的，有时会引起争议 (controversial)。

无论你对正确使用强转有何看法，重要的是了解什么是可能的强转，以及它们可能发生的位置。在本文中，我们命名并描述了 Rust 中所有可能的强转，并描述了哪些类型的表达式可能包含强转，以及哪些表达式可以传播强转。希望这有助于使 Rust 中这个经常被隐藏的部分变得更加清晰。