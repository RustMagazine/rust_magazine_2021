
# 【完整】Rust 标准库 Trait 指南

译者： Praying

> - 原文标题：Tour of Rust's Standard Library Traits
> - 原文链接：[https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md ](https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md )
> - 说明： 可能网络中也存在其他翻译，但本篇是由 Praying 独立翻译完成的。

## 内容目录

- 引言 
- Trait 基础 
- 自动 Trait
- 泛型 Trait
- 格式化 Trait
- 操作符 Trait
- 转换 Trait
- 错误处理 
- 迭代器 Trait
- I/O Trait
- 总结 

## 引言

你是否曾想过下面这些 trait 有什么不同？

- `Deref<Traget=T>`，`AsRef<T>`，以及`Borrow<T>`？
- `Clone`，`Copy`，和`ToOwned`？
- `From<T>` 和`Into<T>`?
- `TryFrom<&str>` 和`FromStr`？
- `FnOnce`，`FnMut`，`Fn`和`fn`?

或者你曾问过自己下面这些问题：

- “我在 trait 中，什么时候使用关联类型（associated type），什么时候使用泛型（generic types）？”

- “什么是泛型覆盖实现（generic [blanket impls](https://doc.rust-lang.org/book/ch10-02-traits.html?highlight=blanket#using-trait-bounds-to-conditionally-implement-methods "blanket impls")）”?

- “subtrait 和 supertrait 是如何工作的？”

- “为什么这个 trait 没有任何方法？”

那么这篇文章就是为你而写的！它回答了包括但不限于上述所有的问题。我们将一起对 Rust 标准库中所有最流行和最常用的 trait 进行快速的浏览。<br/>

你可以按章节顺序阅读本文，也可以跳到你最感兴趣的 trait，因为每个 trait 章节的开头都有一个指向前置章节的链接列表，你应该阅读这些链接，以便有足够的背景知识来理解当前章节的解释（译注：很抱歉，译文中暂时无法提供链接跳转）。

## Trait 基础

我们将会覆盖足够多的基础知识，这样文章的其余部分就可以精简，而不必因为它们在不同的 trait 中反复出现而重复解释相同的概念。

### Trait 项（Item）

Trait 项是指包含于 trait 声明中的任意项。

#### Self

`Self`总是指代实现类型。

```rust
trait Trait {
    // always returns i32
    fn returns_num() -> i32;

    // returns implementing type
    fn returns_self() -> Self;
}

struct SomeType;
struct OtherType;

impl Trait for SomeType {
    fn returns_num() -> i32 {
        5
    }

    // Self == SomeType
    fn returns_self() -> Self {
        SomeType
    }
}

impl Trait for OtherType {
    fn returns_num() -> i32 {
        6
    }

    // Self == OtherType
    fn returns_self() -> Self {
        OtherType
    }
}
```

#### 函数（Function）

Trait 函数是指第一个参数不是`self`关键字的任意函数。

```rust
trait Default {
    // function
    fn default() -> Self;
}
```

Trait 函数可以通过 trait 或者实现类型的命名空间来调用。

```rust
fn main() {
    let zero: i32 = Default::default();
    let zero = i32::default();
}
```

#### 方法（Method）

Trait 方法是指，第一个参数使用了`self`关键字并且`self`的类型是`Self`,`&Self`，`&mut Self`之一。`self`的类型也可以被`Box`，`Rc`，`Arc`或`Pin`来包装。

```rust
trait Trait {
    // methods
    fn takes_self(self);
    fn takes_immut_self(&self);
    fn takes_mut_self(&mut self);

    // above methods desugared
    fn takes_self(self: Self);
    fn takes_immut_self(self: &Self);
    fn takes_mut_self(self: &mut Self);
}

// example from standard library
trait ToString {
    fn to_string(&self) -> String;
}
```

Trait 方法可以通过在实现类型上使用点（.）操作符来调用。

```rust
fn main() {
    let five = 5.to_string();
}
```

此外，trait 方法还可以像函数那样由 trait 或者实现类型通过命名空间来调用。

```rust
fn main() {
    let five = ToString::to_string(&5);
    let five = i32::to_string(&5);
}
```

#### 关联类型（Associated Types）

Trait 可以有关联类型。当我们需要在函数签名中使用`Self`以外的某个类型，但是希望这个类型可以由实现者来选择而不是硬编码到 trait 声明中，这时关联类型就可以发挥作用了。

```rust
trait Trait {
    type AssociatedType;
    fn func(arg: Self::AssociatedType);
}

struct SomeType;
struct OtherType;

// any type implementing Trait can
// choose the type of AssociatedType

impl Trait for SomeType {
    type AssociatedType = i8; // chooses i8
    fn func(arg: Self::AssociatedType) {}
}

impl Trait for OtherType {
    type AssociatedType = u8; // chooses u8
    fn func(arg: Self::AssociatedType) {}
}

fn main() {
    SomeType::func(-1_i8); // can only call func with i8 on SomeType
    OtherType::func(1_u8); // can only call func with u8 on OtherType
}
```

#### 泛型参数（Generic Parameters）

“泛型参数”泛指泛型类型参数（generic type parameters）、泛型生命周期参数（generic lifetime parameters）、以及泛型常量参数（generic const parameters）。因为这些说起来比较拗口，所以人们通常把它们简称为 “泛型类型（generic type）”、“生命周期（lifetime）”和 “泛型常量（generic const）”。由于我们将要讨论的所有标准库 trait 中都没有使用泛型常量，所以它们不在本文的讨论范围之内。

我们可以使用参数来对一个 trait 声明进行泛化（generalize ）。

```rust
// trait declaration generalized with lifetime & type parameters
trait Trait<'a, T> {
    // signature uses generic type
    fn func1(arg: T);

    // signature uses lifetime
    fn func2(arg: &'a i32);

    // signature uses generic type & lifetime
    fn func3(arg: &'a T);
}

struct SomeType;

impl<'a> Trait<'a, i8> for SomeType {
    fn func1(arg: i8) {}
    fn func2(arg: &'a i32) {}
    fn func3(arg: &'a i8) {}
}

impl<'b> Trait<'b, u8> for SomeType {
    fn func1(arg: u8) {}
    fn func2(arg: &'b i32) {}
    fn func3(arg: &'b u8) {}
}
```

泛型可以具有默认值，最常用的默认值是`Self`，但是任何类型都可以作为默认值。

```rust
// make T = Self by default
trait Trait<T = Self> {
    fn func(t: T) {}
}

// any type can be used as the default
trait Trait2<T = i32> {
    fn func2(t: T) {}
}

struct SomeType;

// omitting the generic type will
// cause the impl to use the default
// value, which is Self here
impl Trait for SomeType {
    fn func(t: SomeType) {}
}

// default value here is i32
impl Trait2 for SomeType {
    fn func2(t: i32) {}
}

// the default is overridable as we'd expect
impl Trait<String> for SomeType {
    fn func(t: String) {}
}

// overridable here too
impl Trait2<String> for SomeType {
    fn func2(t: String) {}
}
```

除了可以对 trait 进行参数化之外，我们还可以对单个函数和方法进行参数化。

```rust
trait Trait {
    fn func<'a, T>(t: &'a T);
}
```

#### 泛型类型 vs 关联类型

泛型类型和关联类型都把在 trait 的函数和方法中使用哪种具体类型的决定权交给了实现者，因此这部分内容要去解释什么时候使用泛型类型，什么时候使用关联类型。

通常的经验法则是：

- 当每个类型只应该有 trait 的一个实现时，使用关联类型。

- 当每个类型可能会有 trait 的多个实现时，使用泛型类型。

比如说我们想要定义一个名为`Add`的 trait，该 trait 允许我们对值进行相加。下面是一个最初的设计和实现，里面只使用了关联类型。

```rust
trait Add {
    type Rhs;
    type Output;
    fn add(self, rhs: Self::Rhs) -> Self::Output;
}

struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Rhs = Point;
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };
    let p3 = p1.add(p2);
    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);
}
```

假设现在我们想要添加这样一种功能：把`i32`加到`Point`上，其中`Point`里面的成员`x`和`y`都会加上`i32`。

```rust
trait Add {
    type Rhs;
    type Output;
    fn add(self, rhs: Self::Rhs) -> Self::Output;
}

struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Rhs = Point;
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add for Point { // ❌
    type Rhs = i32;
    type Output = Point;
    fn add(self, rhs: i32) -> Point {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };
    let p3 = p1.add(p2);
    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);

    let p1 = Point { x: 1, y: 1 };
    let int2 = 2;
    let p3 = p1.add(int2); // ❌
    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);
}
```

上面的代码会抛出错误：

```
error[E0119]: conflicting implementations of trait `Add` for type `Point`:
  --> src/main.rs:23:1
   |
12 | impl Add for Point {
   | ------------------ first implementation here
...
23 | impl Add for Point {
   | ^^^^^^^^^^^^^^^^^^ conflicting implementation for `Point`
```

因为`Add` trait 没有被任何的泛型类型参数化，我们只能在每个类型上实现这个 trait 一次，这意味着，我们只能一次把`Rhs`和`Output`类型都选取好！为了能够使`Point`和`i32`类型都能和`Point`相加，我们必须把`Rhs`从一个关联类型重构为泛型类型，这样就能够让我们根据`Rhs`不同的类型参数来为`Point`实现 trait 多次。

```rust
trait Add<Rhs> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

struct Point {
    x: i32,
    y: i32,
}

impl Add<Point> for Point {
    type Output = Self;
    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<i32> for Point { // ✅
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };
    let p3 = p1.add(p2);
    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);

    let p1 = Point { x: 1, y: 1 };
    let int2 = 2;
    let p3 = p1.add(int2); // ✅
    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);
}
```

假如说我们增加了一个名为`Line`的新类型，它包含两个`Point`，现在，在我们的程序中存在这样一种上下文环境，即将两个`Point`相加之后应该产生一个`Line`而不是另一个`Point`。这在当我们当前的`Add` trait 设计中是不可行的，因为`Output`是一个关联类型，但是我们通过把`Output`从关联类型重构为泛型类型来实现这个新需求。

```rust
trait Add<Rhs, Output> {
    fn add(self, rhs: Rhs) -> Output;
}

struct Point {
    x: i32,
    y: i32,
}

impl Add<Point, Point> for Point {
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<i32, Point> for Point {
    fn add(self, rhs: i32) -> Point {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

struct Line {
    start: Point,
    end: Point,
}

impl Add<Point, Line> for Point { // ✅
    fn add(self, rhs: Point) -> Line {
        Line {
            start: self,
            end: rhs,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };
    let p3: Point = p1.add(p2);
    assert!(p3.x == 3 && p3.y == 3);

    let p1 = Point { x: 1, y: 1 };
    let int2 = 2;
    let p3 = p1.add(int2);
    assert!(p3.x == 3 && p3.y == 3);

    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };
    let l: Line = p1.add(p2); // ✅
    assert!(l.start.x == 1 && l.start.y == 1 && l.end.x == 2 && l.end.y == 2)
}
```

所以，哪个`Add` trait 是最好的呢？这取决于你程序中的需求！放在合适的场景中，它们都很好。

### 作用域（Scope）

只有当 trait 在作用域之中时，trait 项才能被使用。大多数 Rustaceans 在第一次尝试写一个 I/O 相关的程序时，都会在吃过一番苦头之后了解到这一点，因为`Read`和`Write`的 trait 并不在标准库的预置（prelude）中。

```rust
use std::fs::File;
use std::io;

fn main() -> Result<(), io::Error> {
    let mut file = File::open("Cargo.toml")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?; // ❌ read_to_string not found in File
    Ok(())
}
```

`read_to_string(buf: &mut String)`声明于`std::io::Read`中并且被`std::fs::File`结构体实现，但是要想调用它，`std::io::Read`必须在当前作用域中。

```rust
use std::fs::File;
use std::io;
use std::io::Read; // ✅

fn main() -> Result<(), io::Error> {
    let mut file = File::open("Cargo.toml")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?; // ✅
    Ok(())
}
```

标准库预置（The standard library prelude）是标准库中的一个模块，也就是说，`std::prelude::v1`，它在每个其他模块的顶部被自动导入，即`use std::prelude::v1::*`。这样的话，下面这些 trait 就总会在作用域中，我们不需要自己显式地导入它们，因为它们是预置的一部分。

- AsMut
- AsRef
- Clone
- Copy
- Default
- Drop
- Eq
- Fn
- FnMut
- FnOnce
- From
- Into
- ToOwned
- IntoIterator
- Iterator
- PartialEq
- PartialOrd
- Send
- Sized
- Sync
- ToString
- Ord

### 派生宏（Derive Macros）

标准库导出了一小部分派生宏，这么派生宏可以让我们可以便捷地在一个类型上实现 trait，前提是该类型的所有成员都实现了这个 trait。派生宏以它们所实现的 trait 来命名。

- Clone
- Copy
- Debug
- Default
- Eq
- Hash
- Ord
- PartialEq
- PartialOrd

使用示例：

```rust
// macro derives Copy & Clone impl for SomeType
#[derive(Copy, Clone)]
struct SomeType;
```

注意：派生宏也是过程宏（procedural macros），它们可以被用来做任何事情，没有强制规定它们必须要实现一个 trait，或者它们只能在所有成员都实现 trait 的情况下才能工作，这些只是标准库中派生宏所遵循的惯例。

### 默认实现（Default Impls）

Trait 可以为它们的函数和方法提供默认实现。

```rust
trait Trait {
    fn method(&self) {
        println!("default impl");
    }
}

struct SomeType;
struct OtherType;

// use default impl for Trait::method
impl Trait for SomeType {}

impl Trait for OtherType {
    // use our own impl for Trait::method
    fn method(&self) {
        println!("OtherType impl");
    }
}

fn main() {
    SomeType.method(); // prints "default impl"
    OtherType.method(); // prints "OtherType impl"
}
```

如果 trait 中的某些方法是完全通过 trait 的另一些方法来实现的，这就非常方便了。

```rust
trait Greet {
    fn greet(&self, name: &str) -> String;
    fn greet_loudly(&self, name: &str) -> String {
        self.greet(name) + "!"
    }
}

struct Hello;
struct Hola;

impl Greet for Hello {
    fn greet(&self, name: &str) -> String {
        format!("Hello {}", name)
    }
    // use default impl for greet_loudly
}

impl Greet for Hola {
    fn greet(&self, name: &str) -> String {
        format!("Hola {}", name)
    }
    // override default impl
    fn greet_loudly(&self, name: &str) -> String {
        let mut greeting = self.greet(name);
        greeting.insert_str(0, "¡");
        greeting + "!"
    }
}

fn main() {
    println!("{}", Hello.greet("John")); // prints "Hello John"
    println!("{}", Hello.greet_loudly("John")); // prints "Hello John!"
    println!("{}", Hola.greet("John")); // prints "Hola John"
    println!("{}", Hola.greet_loudly("John")); // prints "¡Hola John!"
}
```

标准库中的很多 trait 为很多它们的方法提供了默认实现。

### 泛型覆盖实现（Generic Blanket Impls）

泛型覆盖实现是一种在泛型类型而不是具体类型上的实现，为了解释为什么以及如何使用它，让我们从为整数类型实现一个`is_even`方法开始。

```rust
trait Even {
    fn is_even(self) -> bool;
}

impl Even for i8 {
    fn is_even(self) -> bool {
        self % 2_i8 == 0_i8
    }
}

impl Even for u8 {
    fn is_even(self) -> bool {
        self % 2_u8 == 0_u8
    }
}

impl Even for i16 {
    fn is_even(self) -> bool {
        self % 2_i16 == 0_i16
    }
}

// etc

#[test] // ✅
fn test_is_even() {
    assert!(2_i8.is_even());
    assert!(4_u8.is_even());
    assert!(6_i16.is_even());
    // etc
}
```

很明显，上面的实现十分啰嗦。而且，所有我们的实现几乎都是一样的。此外，如果 Rust 决定在未来增加更多的整数类型，我们必须回到这段代码中，用新的整数类型来更新它。我们可以通过使用泛型覆盖实现来解决所有的问题。

```rust
use std::fmt::Debug;
use std::convert::TryInto;
use std::ops::Rem;

trait Even {
    fn is_even(self) -> bool;
}

// generic blanket impl
impl<T> Even for T
where
    T: Rem<Output = T> + PartialEq<T> + Sized,
    u8: TryInto<T>,
    <u8 as TryInto<T>>::Error: Debug,
{
    fn is_even(self) -> bool {
        // these unwraps will never panic
        self % 2.try_into().unwrap() == 0.try_into().unwrap()
    }
}

#[test] // ✅
fn test_is_even() {
    assert!(2_i8.is_even());
    assert!(4_u8.is_even());
    assert!(6_i16.is_even());
    // etc
}
```

不同于默认实现，泛型覆盖实现提供了方法的实现，所以它们不能被重写。

```rust
use std::fmt::Debug;
use std::convert::TryInto;
use std::ops::Rem;

trait Even {
    fn is_even(self) -> bool;
}

impl<T> Even for T
where
    T: Rem<Output = T> + PartialEq<T> + Sized,
    u8: TryInto<T>,
    <u8 as TryInto<T>>::Error: Debug,
{
    fn is_even(self) -> bool {
        self % 2.try_into().unwrap() == 0.try_into().unwrap()
    }
}

impl Even for u8 { // ❌
    fn is_even(self) -> bool {
        self % 2_u8 == 0_u8
    }
}
```

上面的代码会抛出下面的错误：

```
error[E0119]: conflicting implementations of trait `Even` for type `u8`:
  --> src/lib.rs:22:1
   |
10 | / impl<T> Even for T
11 | | where
12 | |     T: Rem<Output = T> + PartialEq<T> + Sized,
13 | |     u8: TryInto<T>,
...  |
19 | |     }
20 | | }
   | |_- first implementation here
21 |
22 |   impl Even for u8 {
   |   ^^^^^^^^^^^^^^^^ conflicting implementation for `u8`
```

这些实现有重叠，因此它们是冲突的，所以 Rust 拒绝编译这段代码以确保 trait 的一致性。trait 一致性是指，对于任意给定的类型，最多存在某一 trait 的一个实现。Rust 用来强制执行特质一致性的规则，这些规则的含义，以及针对这些含义的变通方案都不在本文的讨论范围之内。

### Subtraits & Supertraits

`subtrait`中的`sub`指的是子集（subset)，`supertrait`中的`super`指的是超集（superset）。如果我们有下面这个 trait 声明：

```rust
trait Subtrait: Supertrait {}
```

所有实现了`Subtrait`的类型是所有实现了`Supertrait`的类型的子集，或者反过来讲：所有实现了`Supertrait`的类型是所有实现了`Subtrait`类型的子集。而且，上面的代码是一种语法糖，展开来应该是：

```rust
trait Subtrait where Self: Supertrait {}
```

这是一个微妙而重要的区别，要明白约束在`Self`上，也就是实现`Subtrait`的类型而非`Subtrait`自身。后者也没有意义，因为 trait 约束只能作用于能够实现 trait 的具体类型，trait 本身不能实现其他的 trait：

```rust
trait Supertrait {
    fn method(&self) {
        println!("in supertrait");
    }
}

trait Subtrait: Supertrait {
    // this looks like it might impl or
    // override Supertrait::method but it
    // does not
    fn method(&self) {
        println!("in subtrait")
    }
}

struct SomeType;

// adds Supertrait::method to SomeType
impl Supertrait for SomeType {}

// adds Subtrait::method to SomeType
impl Subtrait for SomeType {}

// both methods exist on SomeType simultaneously
// neither overriding or shadowing the other

fn main() {
    SomeType.method(); // ❌ ambiguous method call
    // must disambiguate using fully-qualified syntax
    <SomeType as Supertrait>::method(&st); // ✅ prints "in supertrait"
    <SomeType as Subtrait>::method(&st); // ✅ prints "in subtrait"
}
```

此外，对于一个类型如何同时实现一个 subtrait 和一个 supertrait，也没有明确的规则。它可以在另一个类型的实现中实现其他的方法。

```rust
trait Supertrait {
    fn super_method(&mut self);
}

trait Subtrait: Supertrait {
    fn sub_method(&mut self);
}

struct CallSuperFromSub;

impl Supertrait for CallSuperFromSub {
    fn super_method(&mut self) {
        println!("in super");
    }
}

impl Subtrait for CallSuperFromSub {
    fn sub_method(&mut self) {
        println!("in sub");
        self.super_method();
    }
}

struct CallSubFromSuper;

impl Supertrait for CallSubFromSuper {
    fn super_method(&mut self) {
        println!("in super");
        self.sub_method();
    }
}

impl Subtrait for CallSubFromSuper {
    fn sub_method(&mut self) {
        println!("in sub");
    }
}

struct CallEachOther(bool);

impl Supertrait for CallEachOther {
    fn super_method(&mut self) {
        println!("in super");
        if self.0 {
            self.0 = false;
            self.sub_method();
        }
    }
}

impl Subtrait for CallEachOther {
    fn sub_method(&mut self) {
        println!("in sub");
        if self.0 {
            self.0 = false;
            self.super_method();
        }
    }
}

fn main() {
    CallSuperFromSub.super_method(); // prints "in super"
    CallSuperFromSub.sub_method(); // prints "in sub", "in super"

    CallSubFromSuper.super_method(); // prints "in super", "in sub"
    CallSubFromSuper.sub_method(); // prints "in sub"

    CallEachOther(true).super_method(); // prints "in super", "in sub"
    CallEachOther(true).sub_method(); // prints "in sub", "in super"
}
```

希望上面的例子能够表达出，subtrait 和 supertrait 之间可以是很复杂的关系。在介绍能够将这些复杂性进行整洁封装的心智模型之前，让我们快速回顾并建立我们用来理解泛型类型上的 trait 约束的心智模型。

```rust
fn function<T: Clone>(t: T) {
    // impl
}
```

在不知道这个函数的实现的情况下，我们可以合理地猜测，`t.clone()`会在某个时候被调用，因为当一个泛型类型被一个 trait 所约束时，意味着它对 trait 有依赖性。泛型与 trait 约束之间关系的心智模型是一个简单而直观的模型：泛型依赖于 trait 约束。

现在让我们看看`Copy`的 trait 声明：

```rust
trait Copy: Clone {}
```

上面的语法看起来与在一个泛型类型上应用 trait 约束很相似，但是`Copy`完全不依赖于`Clone`。之前的模型在这里没有帮助。个人认为，理解 subtrait 和 supertrait 最为简洁优雅的心智模型是：subtrait 细化（refine）了它们的 supertrait。

“细化（Refinement）”刻意保持一定的模糊性，因为它们在不同的上下文环境中会有不同的含义：

- subtrait 可能会使得 supertrait 的方法实现更为具体，快速，占用更少的内存，例如，`Copy:Clone`；

- subtrait 可能会对 supertrait 的方法实现增加额外的保证，例如：`Eq: PartialEq`,`Ord: PartialOrd`,`ExactSizeIterator: Iterator`;

- subtrait 可能会使得 supertrait 的方法更为灵活和易于调用，例如：`FnMut: FnOnce`,`Fn: FnMut`;

- subtrait 可能会扩展 supertrait 并添加新的方法，例如：`DoubleEndedIterator: Iterator`,`ExactSizeIterator: Iterator`。

### Trait 对象

泛型给我们提供了编译期多态，而 trait 对象给我们提供了运行时多态。我们可以使用 trait 对象来让函数在运行时动态地返回不同的类型。

```rust
fn example(condition: bool, vec: Vec<i32>) -> Box<dyn Iterator<Item = i32>> {
    let iter = vec.into_iter();
    if condition {
        // Has type:
        // Box<Map<IntoIter<i32>, Fn(i32) -> i32>>
        // But is cast to:
        // Box<dyn Iterator<Item = i32>>
        Box::new(iter.map(|n| n * 2))
    } else {
        // Has type:
        // Box<Filter<IntoIter<i32>, Fn(&i32) -> bool>>
        // But is cast to:
        // Box<dyn Iterator<Item = i32>>
        Box::new(iter.filter(|&n| n >= 2))
    }
}
```

Trait 对象还允许我们在集合中存储多种类型：

```rust
use std::f64::consts::PI;

struct Circle {
    radius: f64,
}

struct Square {
    side: f64
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn get_total_area(shapes: Vec<Box<dyn Shape>>) -> f64 {
    shapes.into_iter().map(|s| s.area()).sum()
}

fn example() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }), // Box<Circle> cast to Box<dyn Shape>
        Box::new(Square { side: 1.0 }), // Box<Square> cast to Box<dyn Shape>
    ];
    assert_eq!(PI + 1.0, get_total_area(shapes)); // ✅
}
```

Trait 对象是没有大小的，所以它们必须总是在一个指针后面。我们可以根据类型中`dyn`关键字的存在来区分具体类型和 trait 对象在类型级别上的区别。

```rust
struct Struct;
trait Trait {}

// regular struct
&Struct
Box<Struct>
Rc<Struct>
Arc<Struct>

// trait objects
&dyn Trait
Box<dyn Trait>
Rc<dyn Trait>
Arc<dyn Trait>
```

不是所有的 trait 都可以被转成 trait 对象。当且仅当一个 trait 满足下面这些要求时，它才是对象安全的（object-safe）：

- trait 不要求`Self:Sized`
- trait 的所有方法都是对象安全的

当一个 trait 方法满足下面的要求时，该方法是对象安全的：

- 方法要求`Self:Sized` 或者
- 方法在其接收者位置仅使用一个`Self`类型

理解为什么要求是这样的，与本文的其余部分无关，但如果你仍然好奇，可以阅读[Sizeness in Rust](https://github.com/pretzelhammer/rust-blog/blob/master/posts/sizedness-in-rust.md "Sizeness in Rust")（译注：Sizedness in Rust 这篇文章已翻译，可在公众号翻阅往期文章）。

### 标记 Trait（Marker Traits）

标记 trait 是不含 trait 项的 trait。它们的工作把实现类型“标记（mark）”为具有某种属性，否则就没有办法在类型系统中去表示。

```rust
// Impling PartialEq for a type promises
// that equality for the type has these properties:
// - symmetry: a == b implies b == a, and
// - transitivity: a == b && b == c implies a == c
// But DOES NOT promise this property:
// - reflexivity: a == a
trait PartialEq {
    fn eq(&self, other: &Self) -> bool;
}

// Eq has no trait items! The eq method is already
// declared by PartialEq, but "impling" Eq
// for a type promises this additional equality property:
// - reflexivity: a == a
trait Eq: PartialEq {}

// f64 impls PartialEq but not Eq because NaN != NaN
// i32 impls PartialEq & Eq because there's no NaNs :)
```

### 自动 Trait（Auto Trait）

自动 Trait 是指如果一个类型的所有成员都实现了该 trait，该类型就会自动实现该 trait。“成员（member）”的含义取决于类型，例如：结构体的字段、枚举的变量、数组的元素、元组的项，等等。

所有的自动 trait 都是标记 trait，但不是所有的标记 trait 都是自动 trait。自动 trait 必须是标记 trait，所以编译器可以为它们提供一个自动的默认实现，如果它们有任何 trait 项，这就不可能实现了。

自动 trait 的例子。

```rust
// implemented for types which are safe to send between threads
unsafe auto trait Send {}

// implemented for types whose references are safe to send between threads
unsafe auto trait Sync {}
```

### 不安全 Trait（Unsafe Trait）

Trait 可以被标记为 unsafe，以表明实现该 trait 可能需要 unsafe 代码。`Send`和`Sync`都被标记为 unsafe，因为如果它们不是自动实现的类型，就意味着它必须包含一些非`Send`或非`Sync`的成员，如果我们想手动标记类型为`Send`和`Sync`，作为实现者我们必须格外小心，确保没有数据竞争。

## 自动 Trait

### Send & Sync

所需预备知识


```rust
unsafe auto trait Send {}
unsafe auto trait Sync {}
```

如果一个类型是`Send`，这就意味着它可以在线程之间被安全地发送（send）。如果一个类型是`Sync`，这就意味着它可以在线程间安全地共享引用。说得更准确点就是，当且仅当`&T`是`Send`时，类型`T`是`Sync`。

几乎所有的类型都是`Send`和`Sync`。唯一值得注意的`Send`例外是`Rc`，`Sync`例外中需要注意的是`Rc`，`Cell`，`RefCell`。如果我们需要一个满足`Send`的`Rc`，我们可以使用`Arc`。如果我们需要一个`Cell`或`RefCell`的`Sync`版本，我们可以使用`Mutex`或`RwLock`。尽管我们使用`Mutex`和`RwLock`来包装一个原始类型，但通常来讲，使用标准库提供的原子类型会更好一些，比如`AtomicBool`，`AtomicI32`，`AtomicUsize`等等。

几乎所有的类型都是`Sync`这件事，可能会让一些人感到惊讶，但它是真的，即使是对于没有任何内部同步的类型来讲，也是如此。这能够得以实现要归功于 Rust 严格的借用规则。

我们可以传递同一份数据的若干个不可变引用到多个线程中，由于只要有不可变引用存在，Rust 就会静态地保证底层数据不被修改，所以我们可以保证不会发生数据竞争。

```rust
use crossbeam::thread;

fn main() {
    let mut greeting = String::from("Hello");
    let greeting_ref = &greeting;

    thread::scope(|scoped_thread| {
        // spawn 3 threads
        for n in 1..=3 {
            // greeting_ref copied into every thread
            scoped_thread.spawn(move |_| {
                println!("{} {}", greeting_ref, n); // prints "Hello {n}"
            });
        }

        // line below could cause UB or data races but compiler rejects it
        greeting += " world"; // ❌ cannot mutate greeting while immutable refs exist
    });

    // can mutate greeting after every thread has joined
    greeting += " world"; // ✅
    println!("{}", greeting); // prints "Hello world"
}
```

同样地，我们可以把数据的一个可变引用传递给一个单独的线程，由于 Rust 静态地保证不存在可变引用的别名，所以底层数据不会通过另一个可变引用被修改，因此我们也可以保证不会发生数据竞争。

```rust
use crossbeam::thread;

fn main() {
    let mut greeting = String::from("Hello");
    let greeting_ref = &mut greeting;

    thread::scope(|scoped_thread| {
        // greeting_ref moved into thread
        scoped_thread.spawn(move |_| {
            *greeting_ref += " world";
            println!("{}", greeting_ref); // prints "Hello world"
        });

        // line below could cause UB or data races but compiler rejects it
        greeting += "!!!"; // ❌ cannot mutate greeting while mutable refs exist
    });

    // can mutate greeting after the thread has joined
    greeting += "!!!"; // ✅
    println!("{}", greeting); // prints "Hello world!!!"
}
```

这就是为什么大多数类型在不需要任何显式同步的情况下，都满足`Sync`的原因。当我们需要在多线程中同时修改某个数据`T`时，除非我们用`Arc<Mutex<T>>`或者`Arc<RwLock<T>>`来包装这个数据，否则编译器是不会允许我们进行这种操作，所以编译器会在需要时强制要求进行显式地同步。

### Sized


如果一个类型是`Sized`，这意味着它的类型大小在编译期是可知的，并且可以在栈上创建一个该类型的实例。

类型的大小及其含义是一个微妙而巨大的话题，影响到编程语言的许多方面。因为它十分重要，所以我单独写了一篇文章[Sizedness in Rust](https://github.com/pretzelhammer/rust-blog/blob/master/posts/sizedness-in-rust.md "Sizedness in Rust")，如果有人想要更深入地了解 sizedness，我强烈推荐阅读这篇文章。我会把这篇文章的关键内容总结在下面。

1. 所有的泛型类型都有一个隐含的`Sized`约束。

```rust
fn func<T>(t: &T) {}

// example above desugared
fn func<T: Sized>(t: &T) {}
```

2. 因为所有的泛型类型上都有一个隐含的`Sized`约束，如果我们想要选择退出这个约束，我们需要使用特定的“宽松约束（relaxed bound）”语法——`?Sized`，该语法目前只为`Sized` trait 存在。

```rust
// now T can be unsized
fn func<T: ?Sized>(t: &T) {}
```

3. 所有的 trait 都有一个隐含的`?Sized`约束。

```rust
trait Trait {}

// example above desugared
trait Trait: ?Sized {}
```

这是为了让 trait 对象能够实现 trait，重申一下，所有的细枝末节都在[Sizedness in Rust](https://github.com/pretzelhammer/rust-blog/blob/master/posts/sizedness-in-rust.md )中。

## 泛型 traits

### Default

```rust
trait Default {
    fn default() -> Self;
}
```

可以为实现了`Default`的类型构造默认值。

```rust
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Default for Color {
    // default color is black
    fn default() -> Self {
        Color {
            r: 0,
            g: 0,
            b: 0,
        }
    }
}
```

这在快速构建原型的时候十分有用，尤其是在我们没有过多要求而只需要一个类型实例的情况下：

```rust
fn main() {
    // just give me some color!
    let color = Color::default();
}
```

当我们想要显式地把函数暴露给用户时，也可以选择这样做：

```rust
struct Canvas;
enum Shape {
    Circle,
    Rectangle,
}

impl Canvas {
    // let user optionally pass a color
    fn paint(&mut self, shape: Shape, color: Option<Color>) {
        // if no color is passed use the default color
        let color = color.unwrap_or_default();
        // etc
    }
}
```

当我们需要构造泛型类型时，`Default`在泛型上下文中也是有用的：

```rust
fn guarantee_length<T: Default>(mut vec: Vec<T>, min_len: usize) -> Vec<T> {
    for _ in 0..min_len.saturating_sub(vec.len()) {
        vec.push(T::default());
    }
    vec
}
```

我们还可以利用`Default`类型结合 Rust 的结构体更新语法（struct update syntax）来对结构体部分初始化。现在，我们有一个`Color`结构体构造函数`new`，该函数接收结构体的所有成员作为参数：

```rust
impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Color {
            r,
            g,
            b,
        }
    }
}
```

但是，我们可以有更为便利的构造函数，这些构造函数分别只接收结构体的一部分成员，结构体剩下的其他成员使用默认值：

```rust
impl Color {
    fn red(r: u8) -> Self {
        Color {
            r,
            ..Color::default()
        }
    }
    fn green(g: u8) -> Self {
        Color {
            g,
            ..Color::default()
        }
    }
    fn blue(b: u8) -> Self {
        Color {
            b,
            ..Color::default()
        }
    }
}
```

还有一个`Default`派生宏，通过使用它我们可以像下面这样来写`Color`：

```rust
// default color is still black
// because u8::default() == 0
#[derive(Default)]
struct Color {
    r: u8,
    g: u8,
    b: u8
}
```

### Clone

```rust
trait Clone {
    fn clone(&self) -> Self;

    // provided default impls
    fn clone_from(&mut self, source: &Self);
}
```

我们能够把`Clone`类型的不可变引用转换为所拥有的值，即`&T`->`T`。`Clone`不保证这种转换的效率，所以它会很慢并且成本较高。我们可以使用派生宏在一个类型上快速实现`Clone`：

```rust
#[derive(Clone)]
struct SomeType {
    cloneable_member1: CloneableType1,
    cloneable_member2: CloneableType2,
    // etc
}

// macro generates impl below
impl Clone for SomeType {
    fn clone(&self) -> Self {
        SomeType {
            cloneable_member1: self.cloneable_member1.clone(),
            cloneable_member2: self.cloneable_member2.clone(),
            // etc
        }
    }
}
```

`Clone`可以用于在泛型上下文中构造一个类型实例。下面是从前面章节拿过来的一个例子，其中的`Default`被替换为了`Clone`：

```rust
fn guarantee_length<T: Clone>(mut vec: Vec<T>, min_len: usize, fill_with: &T) -> Vec<T> {
    for _ in 0..min_len.saturating_sub(vec.len()) {
        vec.push(fill_with.clone());
    }
    vec
}
```

人们通常把克隆（clone）作为一种避免和借用检查器打交道的逃生出口（escape hatch）。管理带有引用的结构体很具有挑战性，但是我们可以通过克隆把引用变为所拥有的值。

```rust
// oof, we gotta worry about lifetimes 😟
struct SomeStruct<'a> {
    data: &'a Vec<u8>,
}

// now we're on easy street 😎
struct SomeStruct {
    data: Vec<u8>,
}
```

如果我们正在编写的程序对性能不敏感，那么我们就不需要担心克隆数据的问题。Rust 是一门暴露了很多底层细节的语言，所以开发者很容易陷入过早的优化而非真正解决眼前的问题。对于很多程序来讲，最好的优先级顺序通常是，首先构建正确性，其次是优雅性，第三是性能，仅当在对性能进行剖析并确定性能瓶颈之后再去关注性能。通常而言，这是一个值得采纳的好建议，但是你需要清楚，它未必适用于你的程序。

### Copy

```rust
trait Copy:Clone{}
```

我们拷贝`Copy`类型，例如：`T`->`T`.`Copy`承诺拷贝操作是简单的按位拷贝，所以它是快速高效的。我们不能自己实现`Copy`，只有编译器可以提供实现，但是我们可以通过使用`Copy`派生宏让编译器这么做，就像使用`Clone`派生宏一样，因为`Copy`是`Clone`的一个 subtrait:

```rust
#[derive(Copy, Clone)]
struct SomeType;
```

`Copy`对`Clone`进行了细化。一个克隆（clone）操作可能很慢并且开销很大，但是拷贝（copy）操作保证是快速且开销较小的，所以拷贝是一种更快的克隆操作。如果一个类型实现了`Copy`，`Clone`实现就无关紧要了：

```rust
// this is what the derive macro generates
impl<T: Copy> Clone for T {
    // the clone method becomes just a copy
    fn clone(&self) -> Self {
        *self
    }
}
```

当一个类型实现了`Copy`之后，它在被移动（move）时的行为就发生了改变。默认情况下，所有的类型都有*移动（move）语义* ，但是一旦某个类型实现了`Copy`，它就有了*拷贝（copy）语义* 。为了解释二者的不同，让我们看一下这些简单的场景：

```rust
// a "move", src: !Copy
let dest = src;

// a "copy", src: Copy
let dest = src;
```

在上面两种情况下，`dest = src`对`src`的内容进行按位拷贝并把结果移动到`dest`，唯一的不同是，在第一种情况（"a move"）中，借用检查器使得`src`变量失效并确保它后面不会在任何其他地方被使用;在第二种情况下（"a copy"）中，`src`仍然是有效且可用的。

简而言之：拷贝就是移动，移动就是拷贝。它们之间唯一的区别就是其对待借用检查器的方式。

来看一个关于移动（move）的更具体的例子，假定`sec`是一个`Vec<i32>`类型，并且它的内容看起来像下面这样：

```rust
{ data: *mut [i32], length: usize, capacity: usize }
```

当我们执行了`dest = src`，我们会得到：

```rust
src = { data: *mut [i32], length: usize, capacity: usize }
dest = { data: *mut [i32], length: usize, capacity: usize }
```

在这个未知，`src`和`dest`对同一份数据各有一个可变引用别名，这是一个大忌，因此，借用检查器让`src`变量失效，在编译器不报错的情况下。使得它不能再被使用。

再来看一个关于拷贝（copy）的更具体的例子，假定`src`是一个`Option<i32>`，且它的内容看起来如下：

```rust
{ is_valid: bool, data: i32 }
```

现在，当我们执行`dest = src`时，我们会得到：

```rust
src = { is_valid: bool, data: i32 }
dest = { is_valid: bool, data: i32 }
```

它们俩同时都是可用的！因此，`Option<i32>`是`Copy`。

尽管`Copy`是一个自动 trait，但是 Rust 语言设计者决定，让类型显式地选择拷贝语义，而不是在类型符合条件时默默地继承拷贝语义，因为后者可能会引起经常导致 bug 的混乱行为。

### Any



```rust
trait Any: 'static {
    fn type_id(&self) -> TypeId;
}
```

Rust 的多态风格是参数化的，但是如果我们正在尝试使用一种类似于动态类型语言的更为特别（ad-hoc）的多态风格，那么我们可以通过使用`Any` trait 来进行模拟。我们不必手动为我们的类型实现`Any` trait，因为这已经被 generic blanket impl 所涵盖：

```rust
impl<T: 'static + ?Sized> Any for T {
    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

```

我们通过使用`downcast_ref::<T>()`和`downcast_mut::<T>()`方法从一个`dyn Any`中拿出一个`T`:

```rust
use std::any::Any;

#[derive(Default)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn inc(&mut self) {
        self.x += 1;
        self.y += 1;
    }
}

fn map_any(mut any: Box<dyn Any>) -> Box<dyn Any> {
    if let Some(num) = any.downcast_mut::<i32>() {
        *num += 1;
    } else if let Some(string) = any.downcast_mut::<String>() {
        *string += "!";
    } else if let Some(point) = any.downcast_mut::<Point>() {
        point.inc();
    }
    any
}

fn main() {
    let mut vec: Vec<Box<dyn Any>> = vec![
        Box::new(0),
        Box::new(String::from("a")),
        Box::new(Point::default()),
    ];
    // vec = [0, "a", Point { x: 0, y: 0 }]
    vec = vec.into_iter().map(map_any).collect();
    // vec = [1, "a!", Point { x: 1, y: 1 }]
}
```

这个 trait 很少需要用到，因为在大多数情况下，参数化多态要优于临时多态性，后者也可以用枚举（enum）来模拟，枚举具有更好的类型安全，需要的间接（抽象）也更少。例如，我们可以用下面的方式实现上面的例子：

```rust
#[derive(Default)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn inc(&mut self) {
        self.x += 1;
        self.y += 1;
    }
}

enum Stuff {
    Integer(i32),
    String(String),
    Point(Point),
}

fn map_stuff(mut stuff: Stuff) -> Stuff {
    match &mut stuff {
        Stuff::Integer(num) => *num += 1,
        Stuff::String(string) => *string += "!",
        Stuff::Point(point) => point.inc(),
    }
    stuff
}

fn main() {
    let mut vec = vec![
        Stuff::Integer(0),
        Stuff::String(String::from("a")),
        Stuff::Point(Point::default()),
    ];
    // vec = [0, "a", Point { x: 0, y: 0 }]
    vec = vec.into_iter().map(map_stuff).collect();
    // vec = [1, "a!", Point { x: 1, y: 1 }]
}

```

尽管`Any`很少被需要用到，但是在某些时候它也会十分地便利，正如我们在后面错误处理（Error Handling）部分所看到的那样。

## 格式化 Traits (Formatting Traits)

我们可以使用`std::fmt`中的格式化宏来把类型序列化(serialize)为字符串，其中最为我们熟知的就是`println!`。我们可以把格式化参数传递给`{}`占位符，这些占位符用于选择使用哪个 trait 来序列化占位符参数。

| Trait      | Placeholder | Description      |
| ---------- | ----------- | ---------------- |
| `Display`  | `{}`        | 显示表示         |
| `Debug`    | `{:?}`      | 调试表示         |
| `Octal`    | `{:o}`      | 八进制表示       |
| `LowerHex` | `{:x}`      | 小写十六进制表示 |
| `UpperHex` | `{:X}`      | 大写十六进制表示 |
| `Pointer`  | `{:p}`      | 内存地址         |
| `Binary`   | `{:b}`      | 二进制表示       |
| `LowerExp` | `{:e}`      | 小写指数表示     |
| `UpperExp` | `{:E}`      | 大写指数表示     |

### Display & ToString


```rust
trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}
```

`Display`类型可以被序列化为对用户更为友好的`String`类型。以`Point`类型为列：

```rust
use std::fmt;

#[derive(Default)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    println!("origin: {}", Point::default());
    // prints "origin: (0, 0)"

    // get Point's Display representation as a String
    let stringified_point = format!("{}", Point::default());
    assert_eq!("(0, 0)", stringified_point); // ✅
}
```

除了使用`format!`宏让一个类型以`String`类型显示，我们还可以使用`ToString` trait:

```rust
trait ToString {
    fn to_string(&self) -> String;
}
```

这个 trait 不需要我们实现，事实上，由于 generic blanket impl，我们也不能去实现它，因为所有实现了`Display`的类型都会自动实现`ToString`：

```rust
impl<T: Display + ?Sized> ToString for T;
```

在`Point`上使用`ToString`：

```rust
#[test] // ✅
fn display_point() {
    let origin = Point::default();
    assert_eq!(format!("{}", origin), "(0, 0)");
}

#[test] // ✅
fn point_to_string() {
    let origin = Point::default();
    assert_eq!(origin.to_string(), "(0, 0)");
}

#[test] // ✅
fn display_equals_to_string() {
    let origin = Point::default();
    assert_eq!(format!("{}", origin), origin.to_string());
}
```

### Debug

```rust
trait Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}
```

`Debug`和`Display`有着相同的签名。唯一的不同在于，只有当我门指定了`{:?}`才会调用`Debug`实现。`Debug`可以被派生：

```rust
use std::fmt;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// derive macro generates impl below
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
```

为一个类型实现`Debug`能够使得这个类型在`dbg!`中使用，`dbg!`宏在快速打印日志方面比`println!`更合适，它的一些优势如下：

1. `dbg!`打印到 stderr 而不是 stdout，因此在我们的程序中，能够很容易地和标准输出的输出结果区分。
2. `dbg!`会连同传入的表达式和表达式的计算结果一起打印出来。
3. `dbg!`会获取传入参数的所有权并将其返回，因此你可以在表达式中使用它：

```rust
fn some_condition() -> bool {
    true
}

// no logging
fn example() {
    if some_condition() {
        // some code
    }
}

// println! logging
fn example_println() {
    // 🤦
    let result = some_condition();
    println!("{}", result); // just prints "true"
    if result {
        // some code
    }
}

// dbg! logging
fn example_dbg() {
    // 😍
    if dbg!(some_condition()) { // prints "[src/main.rs:22] some_condition() = true"
        // some code
    }
}

```

`dbg!`的唯一缺点就是它不会在 release 构建中自动裁剪，所以如果我们不想在最后生成的二进制包含这些内容，就必须手动移除它。

## 操作符 Trait（Operator Traits）

Rust 中所有的操作符都和 trait 关联，如果我们想要为我们的类型实现一些操作符，我们就必须实现与之关联的 trait。

| Trait(s) | 分类（Category） | 操作符（Operator(s)） | 描述（Description） |
|----------|----------|-------------|-------------|
| `Eq`, `PartialEq` | 比较 | `==` | 相等 |
| `Ord`, `PartialOrd` | 比较 | `<`, `>`, `<=`, `>=` | 比较 |
| `Add` | 算术 | `+` | 相加 |
| `AddAssign` | 算术 | `+=` | 相加并赋值 |
| `BitAnd` | 算术 | `&` | 按位与 |
| `BitAndAssign` | 算术 | `&=` | 按位与并赋值 |
| `BitXor` | 算术 | `^` | 按位异或 |
| `BitXorAssign` | 算术 | `^=` | 按位异或并赋值 |
| `Div` | 算术 | `/` | 除 |
| `DivAssign` | 算术 | `/=` | 除并赋值 |
| `Mul` | 算术 | `*` | 乘 |
| `MulAssign` | 算术 | `*=` | 乘并赋值 |
| `Neg` | 算术 | `-` | 一元求反 |
| `Not` | 算术 | `!` | 一元逻辑求反 |
| `Rem` | 算术 | `%` | 求余 |
| `RemAssign` | 算术 | `%=` | 求余并赋值|
| `Shl` | 算术 | `<<` | 左移 |
| `ShlAssign` | 算术 | `<<=` | 左移并赋值 |
| `Shr` | 算术 | `>>` | 右移 |
| `ShrAssign` | 算术 | `>>=` | 右移并赋值 |
| `Sub` | 算术 | `-` | 减 |
| `SubAssign` | 算术 | `-=` | 减并赋值 |
| `Fn` | 闭包 | `(...args)` | 不可变闭包调用 |
| `FnMut` | 闭包 | `(...args)` | 可变闭包调用 |
| `FnOnce` | 闭包 | `(...args)` | 一次性闭包调用 |
| `Deref` | 其他 | `*` | 不可变解引用 |
| `DerefMut` | 其他 | `*` | 可变解引用 |
| `Drop` | 其他 | - | 类型析构 |
| `Index` | 其他 | `[]` | 不可变索引 |
| `IndexMut` | 其他 | `[]` |可变索引|
| `RangeBounds` | 其他 | `..` | 区间 |

### 比较 Trait （Comparison Traits）

| Trait(s)            | 分类（Category） | 操作符（Operator(s)） | 描述（Description） |
| ------------------- | ---------------- | --------------------- | ------------------- |
| `Eq`, `PartialEq`   | 比较             | `==`                  | 相等                |
| `Ord`, `PartialOrd` | 比较             | `<`, `>`, `<=`, `>=`  | 比较                |

#### PartialEq & Eq

```rust
trait PartialEq<Rhs = Self>
where
    Rhs: ?Sized,
{
    fn eq(&self, other: &Rhs) -> bool;

    // provided default impls
    fn ne(&self, other: &Rhs) -> bool;
}

```

`PartialEq<Rhs>`类型可以通过`==`操作符检查是否和`Rhs`类型相等。

所有的`PartialEq<Rhs>`实现必须确保相等性是对称的和可传递的。这意味着，对于任意的`a`、`b`、`c`:

- `a == b`也意味着`b == a`（对称性）
- `a == b && b == c` 意味着 `a == c` （传递性）

默认情况下，`Rhs = Self`，因为我们几乎总是想要比较同一类型的不同实例，而不是不同类型的不同实例。这也保证了我们的实现是对称的和可传递的。

```rust
struct Point {
    x: i32,
    y: i32
}

// Rhs == Self == Point
impl PartialEq for Point {
    // impl automatically symmetric & transitive
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}
```

如果一个类型的所有成员都实现了`PartialEq`，则它会派生实现`PartialEq`：

```rust
#[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32
}

#[derive(PartialEq)]
enum Suit {
    Spade,
    Heart,
    Club,
    Diamond,
}

```

一旦我们为自己的类型实现了`PartialEq`，我们就能够轻松地在类型的引用之间进行相等性比较，这要归功于 generic blanket impls：

```rust
// this impl only gives us: Point == Point
#[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32
}

// all of the generic blanket impls below
// are provided by the standard library

// this impl gives us: &Point == &Point
impl<A, B> PartialEq<&'_ B> for &'_ A
where A: PartialEq<B> + ?Sized, B: ?Sized;

// this impl gives us: &mut Point == &Point
impl<A, B> PartialEq<&'_ B> for &'_ mut A
where A: PartialEq<B> + ?Sized, B: ?Sized;

// this impl gives us: &Point == &mut Point
impl<A, B> PartialEq<&'_ mut B> for &'_ A
where A: PartialEq<B> + ?Sized, B: ?Sized;

// this impl gives us: &mut Point == &mut Point
impl<A, B> PartialEq<&'_ mut B> for &'_ mut A
where A: PartialEq<B> + ?Sized, B: ?Sized;

```

因为这个 trait 是泛型的，所以我们可以在不同的类型之间定义相等性（比较）。标准库利用这一点实现了类字符串类型之间的相互比较，比如`String`、`&str`、`PathBuf`、`&Path`、`OsString`、`&OsStr`等等。

通常，我们应该仅为特定的不同类型之间实现相等性，这些不同类型包含了相同类型的数据，并且它们之间唯一的区别是表现数据的方式和与数据交互的方式。

下面是一个反面实例，关于某人试图在没有满足上述规则的不同类型之间实现`PartialEq`用以检查完整性的例子：

```rust
#[derive(PartialEq)]
enum Suit {
    Spade,
    Club,
    Heart,
    Diamond,
}

#[derive(PartialEq)]
enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(PartialEq)]
struct Card {
    suit: Suit,
    rank: Rank,
}

// check equality of Card's suit
impl PartialEq<Suit> for Card {
    fn eq(&self, other: &Suit) -> bool {
        self.suit == *other
    }
}

// check equality of Card's rank
impl PartialEq<Rank> for Card {
    fn eq(&self, other: &Rank) -> bool {
        self.rank == *other
    }
}

fn main() {
    let AceOfSpades = Card {
        suit: Suit::Spade,
        rank: Rank::Ace,
    };
    assert!(AceOfSpades == Suit::Spade); // ✅
    assert!(AceOfSpades == Rank::Ace); // ✅
}

```

`Eq`是一个标记 trait，并且是`PartialEq<Self>`的一个 subtrait。

```rust
trait Eq: PartialEq<Self> {}
```

如果我们为一个类型实现了`Eq`，在`PartialEq`所要求的对称性和可传递性之上，我们还保证了反射性（reflexivity），也就是对于任意的`a`，都有`a == a`。从这种意义上来说，`Eq`对`PartialEq`进行了细化，因为它表示了一个更为严格的相等性。如果一个类型的所有成员都实现了`Eq`，那么`Eq`的实现可以派生到这个类型。

浮点型实现了`PartialEq`但是没有实现`Eq`，因为`NaN != NaN`。几乎所有其他的实现了`PartialEq`的类型都实现了`Eq`，除非它们包含浮点类型。

一旦一个类型实现了`PartialEq`和`Debug`，我们可以就可以在`assert_eq!`宏中使用它。我们还可以比较实现了`PartialEq`类型的集合。

```rust
#[derive(PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn example_assert(p1: Point, p2: Point) {
    assert_eq!(p1, p2);
}

fn example_compare_collections<T: PartialEq>(vec1: Vec<T>, vec2: Vec<T>) {
    // if T: PartialEq this now works!
    if vec1 == vec2 {
        // some code
    } else {
        // other code
    }
}

```

### Hash

```rust
trait Hash {
    fn hash<H: Hasher>(&self, state: &mut H);

    // provided default impls
    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H);
}
```

这个 trait 没有与任何操作符关联，但是讨论它的最好时机就是在`PartialEq`和`Eq`之后，所以把它写在这里。`Hash`类型可以通过一个`Hasher`被（计算）哈希。

```rust
use std::hash::Hasher;
use std::hash::Hash;

struct Point {
    x: i32,
    y: i32,
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_i32(self.x);
        hasher.write_i32(self.y);
    }
}

```

使用派生宏可以生成和上面一样的实现：

```rust
#[derive(Hash)]
struct Point {
    x: i32,
    y: i32,
}

```

如果一个类型同时实现了`Hash`和`Eq`，那么这些实现必须达成一致，从而保证对于所有的`a`和`b`，如果`a == b`那么`a.hash() == b.hash()`。因此，当为一个类型同时实现这两个 trait 时，要么都用派生宏，要么都手动实现，但是不要混合，否则我们就有可能破坏上面的不变性。

为一个类型实现`Eq`和`Hash`的最大好处是，它让我们能够把类型作为 key 存储在`HashMap`和`HashSet`中。

```rust
use std::collections::HashSet;

// now our type can be stored
// in HashSets and HashMaps!
#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn example_hashset() {
    let mut points = HashSet::new();
    points.insert(Point { x: 0, y: 0 }); // ✅
}

```

### PartialOrd & Ord

```rust
enum Ordering {
    Less,
    Equal,
    Greater,
}

trait PartialOrd<Rhs = Self>: PartialEq<Rhs>
where
    Rhs: ?Sized,
{
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

    // provided default impls
    fn lt(&self, other: &Rhs) -> bool;
    fn le(&self, other: &Rhs) -> bool;
    fn gt(&self, other: &Rhs) -> bool;
    fn ge(&self, other: &Rhs) -> bool;
}

```

`PartialOrd<Rhs>`类型可以通过`<`、`<=`、`>=`操作符和`Rhs`类型比较。所有的`PartialOrd<Rhs>`实现必须保证比较时非对称和可传递的。这意味着，对于任意的`a`、`b`和`c`：

- `a < b`意味着`!(a>b)`（非对称性）
- `a < b && b < c` 意味着`a < c`(传递性)

`PartialOrd`是`PartialEq`的一个 subtrait，并且它们的实现必须相互一致。

```rust
fn must_always_agree<T: PartialOrd + PartialEq>(t1: T, t2: T) {
    assert_eq!(t1.partial_cmp(&t2) == Some(Ordering::Equal), t1 == t2);
}

```

当比较`PartialEq`类型时，我们可以检查是否它们相等或者不相等，但是当比较`PartialOrd`类型时，我们除了可以检查是否它们相等或不相等之外，如果它们不相等，我们还可以检查它们不相等是因为第一项小于第二项或者是第一项大于第二项。

默认情况下，`Rhs == Self`，因为我们总是想要比较同一类型的实例，而不是对不同类型的实例。这也自动保证了我们的实现是对称的和可传递的。

```rust
use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32
}

// Rhs == Self == Point
impl PartialOrd for Point {
    // impl automatically symmetric & transitive
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(match self.x.cmp(&other.x) {
            Ordering::Equal => self.y.cmp(&other.y),
            ordering => ordering,
        })
    }
}

```

如果一个类型的所有成员都实现了`PartialOrd`，那么它就可以被派生：

```rust
#[derive(PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(PartialEq, PartialOrd)]
enum Stoplight {
    Red,
    Yellow,
    Green,
}
```

派生宏`PartialOrd`根据字典序（lexicographical）对它们的成员进行排序：

```rust
// generates PartialOrd impl which orders
// Points based on x member first and
// y member second because that's the order
// they appear in the source code
#[derive(PartialOrd, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// generates DIFFERENT PartialOrd impl
// which orders Points based on y member
// first and x member second
#[derive(PartialOrd, PartialEq)]
struct Point {
    y: i32,
    x: i32,
}

```

`Ord`是`Eq`和`PartialOrd<Self>`的一个 subtrait:

```rust
trait Ord: Eq + PartialOrd<Self> {
    fn cmp(&self, other: &Self) -> Ordering;

    // provided default impls
    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
    fn clamp(self, min: Self, max: Self) -> Self;
}

```

如果我们为一个类型实现了`Ord`，在`PartialOrd`保证了非对称性和传递性之上，我们还能保证整体的非对称性，即对于任意给定的`a`、`b`，`a < b`、`a == b`或`a > b`中必有一个为真。从这个角度来讲，`Ord`细化了`Eq`和`PartialOrd`，因为它表示一个更严格的比较。如果一个类型实现了`Ord`，我们就可以利用这个实现来实现`PartialOrd`、`PartialEq`和`Eq`：

```rust
use std::cmp::Ordering;

// of course we can use the derive macros here
#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// note: as with PartialOrd, the Ord derive macro
// orders a type based on the lexicographical order
// of its members

// but here's the impls if we wrote them out by hand
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.x.cmp(&other.x) {
            Ordering::Equal => self.y.cmp(&other.y),
            ordering => ordering,
        }
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for Point {}

```

浮点型实现了`PartialOrd`但是没有实现`Ord`，因为`NaN < 0 == false`和`NaN >= 0 == false`都为真。几乎所有的其他的`PartialOrd`类型都实现了`Ord`，除非它们中包含有浮点型。

一旦一个类型实现了`Ord`，我们就可以把它存储在`BTreeMap`和`BTreeSet`，还可以在 slice 上使用 sort()方法对其进行排序，这同样适用于其他可以解引用为 slice 的类型，比如数组、`Vec`和`VecDeque`。

```rust
use std::collections::BTreeSet;

// now our type can be stored
// in BTreeSets and BTreeMaps!
#[derive(Ord, PartialOrd, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn example_btreeset() {
    let mut points = BTreeSet::new();
    points.insert(Point { x: 0, y: 0 }); // ✅
}

// we can also .sort() Ord types in collections!
fn example_sort<T: Ord>(mut sortable: Vec<T>) -> Vec<T> {
    sortable.sort();
    sortable
}

```
### 算术 Trait（Arithmetic Traits）

| Trait(s)       | 分类（Category） | 操作符（Operator(s)） | 描述（Description） |
| -------------- | ---------------- | --------------------- | ------------------- |
| `Add`          | 算术             | `+`                   | 相加                |
| `AddAssign`    | 算术             | `+=`                  | 相加并赋值          |
| `BitAnd`       | 算术             | `&`                   | 按位与              |
| `BitAndAssign` | 算术             | `&=`                  | 按位与并赋值        |
| `BitXor`       | 算术             | `^`                   | 按位异或            |
| `BitXorAssign` | 算术             | `^=`                  | 按位异或并赋值      |
| `Div`          | 算术             | `/`                   | 除                  |
| `DivAssign`    | 算术             | `/=`                  | 除并赋值            |
| `Mul`          | 算术             | `*`                   | 乘                  |
| `MulAssign`    | 算术             | `*=`                  | 乘并赋值            |
| `Neg`          | 算术             | `-`                   | 一元求反            |
| `Not`          | 算术             | `!`                   | 一元逻辑求反        |
| `Rem`          | 算术             | `%`                   | 求余                |
| `RemAssign`    | 算术             | `%=`                  | 求余并赋值          |
| `Shl`          | 算术             | `<<`                  | 左移                |
| `ShlAssign`    | 算术             | `<<=`                 | 左移并赋值          |
| `Shr`          | 算术             | `>>`                  | 右移                |
| `ShrAssign`    | 算术             | `>>=`                 | 右移并赋值          |
| `Sub`          | 算术             | `-`                   | 减                  |
| `SubAssign`    | 算术             | `-=`                  | 减并赋值            |

我们没有必要把所有的算术操作符都仔细看一遍，毕竟它们中大多数都只作用于数值类型。我们将会讨论`Add`和`AddAssign`，因为`+`操作符经常被重载用来完成其他事情，比如往集合里添加一项，或者进行拼接操作，这样我们就可以从最有趣的地方入手而不会重复。

#### Add & AddAssign

```rust
trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}
```

`Add<Rhs, Output = T>`类型可以被加到`Rhs`类型上并产生一个`T`作为输出。

例如，在`Point`上实现`Add<Point, Output = Point>`:

```rust
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;
    assert_eq!(p3.x, p1.x + p2.x); // ✅
    assert_eq!(p3.y, p1.y + p2.y); // ✅
}

```

但是，如果我们只有`Point`的引用，那该怎么办呢？我们还能把它们相加么？让我们试试：

```rust
fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = &p1 + &p2; // ❌
}
```

显然不可以，编译器抛出下面的提示：

```rust
error[E0369]: cannot add `&Point` to `&Point`
  --> src/main.rs:50:25
   |
50 |     let p3: Point = &p1 + &p2;
   |                     --- ^ --- &Point
   |                     |
   |                     &Point
   |
   = note: an implementation of `std::ops::Add` might be missing for `&Point`

```

在 Rust 的类型系统中，对于某个类型`T`，`T`、`&T`、`&mut T`都会被视作是完全不同的类型，这意味着我们必须分别为它们提供 trait 的实现。让我们为`&Point`实现`Add`：

```rust
impl Add for &Point {
    type Output = Point;
    fn add(self, rhs: &Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = &p1 + &p2; // ✅
    assert_eq!(p3.x, p1.x + p2.x); // ✅
    assert_eq!(p3.y, p1.y + p2.y); // ✅
}

```

尽管如此，但是仍然感觉有些地方不太对。我们针对`Point`和`&Point`实现了两份`Add`，它们恰好目前还做了相同的事情，但是我们不能保证将来也是如此。例如，假设我们决定，当我们把两个`Point`相加时，我们想要创建一个包含这两个`Point`的`Line`类型而不是创建一个新的`Point`，那么我们会把`Add`的实现更新：

```rust
use std::ops::Add;

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
struct Line {
    start: Point,
    end: Point,
}

// we updated this impl
impl Add for Point {
    type Output = Line;
    fn add(self, rhs: Point) -> Line {
        Line {
            start: self,
            end: rhs,
        }
    }
}

// but forgot to update this impl, uh oh!
impl Add for &Point {
    type Output = Point;
    fn add(self, rhs: &Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let line: Line = p1 + p2; // ✅

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let line: Line = &p1 + &p2; // ❌ expected Line, found Point
}

```

我们当前针对`&Point`的`Add`实现就产生了一个不必要的维护负担，我们希望这个实现能够自动匹配`Point`的实现而无需我们每次在修改`Point`的实现时都手动维护更新。我们想要保持我们的代码尽可能地 DRY（Don't Repeat Yourself，不要重复自己）。幸运的是这是可以实现的：

```rust
// updated, DRY impl
impl Add for &Point {
    type Output = <Point as Add>::Output;
    fn add(self, rhs: &Point) -> Self::Output {
        Point::add(*self, *rhs)
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let line: Line = p1 + p2; // ✅

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let line: Line = &p1 + &p2; // ✅
}

```

`AddAssign<Rhs>`类型能够让我们和`Rhs`类型相加并赋值。该 trait 声明如下：

```rust
trait AddAssign<Rhs = Self> {
    fn add_assign(&mut self, rhs: Rhs);
}
```

以`Point`和`&Point`为例：

```rust
use std::ops::AddAssign;

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<&Point> for Point {
    fn add_assign(&mut self, rhs: &Point) {
        Point::add_assign(self, *rhs);
    }
}

fn main() {
    let mut p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    p1 += &p2;
    p1 += p2;
    assert!(p1.x == 7 && p1.y == 10);
}

```

### 闭包 Trait（Closure Traits）

| Trait(s) | 分类（Category） | 操作符（Operator(s)） | 描述（Description） |
| -------- | ---------------- | --------------------- | ------------------- |
| `Fn`     | 闭包             | `(...args)`           | 不可变闭包调用      |
| `FnMut`  | 闭包             | `(...args)`           | 可变闭包调用        |
| `FnOnce` | 闭包             | `(...args)`           | 一次性闭包调用      |

#### FnOnce, FnMut, & Fn

```rust
trait FnOnce<Args> {
    type Output;
    fn call_once(self, args: Args) -> Self::Output;
}

trait FnMut<Args>: FnOnce<Args> {
    fn call_mut(&mut self, args: Args) -> Self::Output;
}

trait Fn<Args>: FnMut<Args> {
    fn call(&self, args: Args) -> Self::Output;
}

```

虽然存在这些 trait，但是在 stable 的 Rust 中，我们无法为自己的类型实现这些 trait。我们能够创建的唯一能够实现这些 trait 的类型就是闭包。闭包根据其从环境中所捕获的内容来决定它到底是实现`FnOnce`、`FnMut`还是`Fn`。

`FnOnce`闭包只能被调用一次，因为它会在执行过程中消耗掉某些值:

```rust
fn main() {
    let range = 0..10;
    let get_range_count = || range.count();
    assert_eq!(get_range_count(), 10); // ✅
    get_range_count(); // ❌
}

```

迭代器上的`.count()`方法会消耗迭代器，因此它只能被调用一次。因此，我们的闭包也只能调用一次。这也是为什么我们在尝试调用第二次的时候会得到下面的错误：

```shell
error[E0382]: use of moved value: `get_range_count`
 --> src/main.rs:5:5
  |
4 |     assert_eq!(get_range_count(), 10);
  |                ----------------- `get_range_count` moved due to this call
5 |     get_range_count();
  |     ^^^^^^^^^^^^^^^ value used here after move
  |
note: closure cannot be invoked more than once because it moves the variable `range` out of its environment
 --> src/main.rs:3:30
  |
3 |     let get_range_count = || range.count();
  |                              ^^^^^
note: this value implements `FnOnce`, which causes it to be moved when called
 --> src/main.rs:4:16
  |
4 |     assert_eq!(get_range_count(), 10);
  |                ^^^^^^^^^^^^^^^

```

`FnMut`闭包可以被多次调用，并且可以修改它从环境中捕获到的变量。我们可以说`FnMut`有副作用或者是有状态的（stateful）。下面是一个闭包的示例，通过从迭代器中追踪它见到的最小值来过滤所有非升序的值。

```rust
fn main() {
    let nums = vec![0, 4, 2, 8, 10, 7, 15, 18, 13];
    let mut min = i32::MIN;
    let ascending = nums.into_iter().filter(|&n| {
        if n <= min {
            false
        } else {
            min = n;
            true
        }
    }).collect::<Vec<_>>();
    assert_eq!(vec![0, 4, 8, 10, 15, 18], ascending); // ✅
}

```

`FnOnce`会获取它的参数的所有权并且只能被调用一次，但是`FnMut`仅要求获取参数的可变引用并且可以被多次调用，从这一点上来讲，`FnMut`细化了`FnOnce`。`FnMut`可以被用于任何可以使用`FnOnce`的地方。

`Fn`闭包也可以被调用多次，但是它不能修改从环境中捕获的变量。我们可以说，`Fn`闭包没有副作用或者无状态的（stateless）。下面是一个示例，从一个迭代器中过滤出所有小于某个栈上变量的数字，该变量是它是环境中捕获到的：

```rust
fn main() {
    let nums = vec![0, 4, 2, 8, 10, 7, 15, 18, 13];
    let min = 9;
    let greater_than_9 = nums.into_iter().filter(|&n| n > min).collect::<Vec<_>>();
    assert_eq!(vec![10, 15, 18, 13], greater_than_9); // ✅
}

```

`FnMut`要求可变引用并且可以被多次调用，`Fn`只要求不可变引用并可以被多次调用，从这一点来讲，`Fn`细化了`FnMut`。`Fn`可以被用于任何可以使用`FnMut`的地方，当然也包括可以使用`FnOnce`的地方。

如果一个闭包不从环境中捕获任何变量，从技术角度来讲它算不上是闭包，而只是一个被匿名声明的内联函数，并且可以作为一个普通函数指针（即`Fn`）被使用和传递，这包括可以使用`FnMut`和`FnOnce`的地方。

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let mut fn_ptr: fn(i32) -> i32 = add_one;
    assert_eq!(fn_ptr(1), 2); // ✅

    // capture-less closure cast to fn pointer
    fn_ptr = |x| x + 1; // same as add_one
    assert_eq!(fn_ptr(1), 2); // ✅
}

```

下面是一个传递普通函数指针而不是闭包的示例：

```rust
fn main() {
    let nums = vec![-1, 1, -2, 2, -3, 3];
    let absolutes: Vec<i32> = nums.into_iter().map(i32::abs).collect();
    assert_eq!(vec![1, 1, 2, 2, 3, 3], absolutes); // ✅
}

```

### 其他 Trait （Other Traits）

| Trait(s)      | 分类（Category） | 操作符（Operator(s)） | 描述（Description） |
| ------------- | ---------------- | --------------------- | ------------------- |
| `Deref`       | 其他             | `*`                   | 不可变解引用        |
| `DerefMut`    | 其他             | `*`                   | 可变解引用          |
| `Drop`        | 其他             | -                     | 类型析构            |
| `Index`       | 其他             | `[]`                  | 不可变索引          |
| `IndexMut`    | 其他             | `[]`                  | 可变索引            |
| `RangeBounds` | 其他             | `..`                  | 区间                |

```rust
trait Deref {
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
}

trait DerefMut: Deref {
    fn deref_mut(&mut self) -> &mut Self::Target;
}
```

`Deref<Target = T>`类型可以使用`*`操作符解引用为`T`类型。这在像`Box`和`Rc`这样的智能指针类型中有很明显的用例。尽管如此，但是我们在 Rust 代码中很少见到这种显式的解引用操作，这是因为 Rust 有一个被称为`解引用强制转换（deref coercion）`的特性。

当类型被作为函数参数传递、从函数返回或者作为方法调用的一部分时，Rust 会自动对这些类型进行解引用。这也解释了为什么我们可以在一个期望`&str`和`&[T]`的函数中可以传入`&String`和`&Vec<T>`，因为`String`实现了`Deref<Target = str>`并且`Vec<T>`实现了`Deref<Target = [T]>`。

`Deref`和`DerefMut`应该仅被实现于智能指针类型。人们误用和滥用这些 trait 的最常见的方式是，试图把 OOP（面向对象程序设计）风格的数据继承塞进 Rust 中。这样是行不通的。Rust 不是 OOP。让我们进行一些测试，来看看它是在哪里、怎么样以及为什么行不通。让我们从下面的例子开始：

```rust
use std::ops::Deref;

struct Human {
    health_points: u32,
}

enum Weapon {
    Spear,
    Axe,
    Sword,
}

// a Soldier is just a Human with a Weapon
struct Soldier {
    human: Human,
    weapon: Weapon,
}

impl Deref for Soldier {
    type Target = Human;
    fn deref(&self) -> &Human {
        &self.human
    }
}

enum Mount {
    Horse,
    Donkey,
    Cow,
}

// a Knight is just a Soldier with a Mount
struct Knight {
    soldier: Soldier,
    mount: Mount,
}

impl Deref for Knight {
    type Target = Soldier;
    fn deref(&self) -> &Soldier {
        &self.soldier
    }
}

enum Spell {
    MagicMissile,
    FireBolt,
    ThornWhip,
}

// a Mage is just a Human who can cast Spells
struct Mage {
    human: Human,
    spells: Vec<Spell>,
}

impl Deref for Mage {
    type Target = Human;
    fn deref(&self) -> &Human {
        &self.human
    }
}

enum Staff {
    Wooden,
    Metallic,
    Plastic,
}

// a Wizard is just a Mage with a Staff
struct Wizard {
    mage: Mage,
    staff: Staff,
}

impl Deref for Wizard {
    type Target = Mage;
    fn deref(&self) -> &Mage {
        &self.mage
    }
}

fn borrows_human(human: &Human) {}
fn borrows_soldier(soldier: &Soldier) {}
fn borrows_knight(knight: &Knight) {}
fn borrows_mage(mage: &Mage) {}
fn borrows_wizard(wizard: &Wizard) {}

fn example(human: Human, soldier: Soldier, knight: Knight, mage: Mage, wizard: Wizard) {
    // all types can be used as Humans
    borrows_human(&human);
    borrows_human(&soldier);
    borrows_human(&knight);
    borrows_human(&mage);
    borrows_human(&wizard);
    // Knights can be used as Soldiers
    borrows_soldier(&soldier);
    borrows_soldier(&knight);
    // Wizards can be used as Mages
    borrows_mage(&mage);
    borrows_mage(&wizard);
    // Knights & Wizards passed as themselves
    borrows_knight(&knight);
    borrows_wizard(&wizard);
}

```

乍看之下，上面的代码似乎还不错！但是，仔细观察之后它就没这么好了。首先，解引用强制转换仅作用于引用，因此，当我们想要传递所有权的时候它是行不通的：

```rust
fn takes_human(human: Human) {}

fn example(human: Human, soldier: Soldier, knight: Knight, mage: Mage, wizard: Wizard) {
    // all types CANNOT be used as Humans
    takes_human(human);
    takes_human(soldier); // ❌
    takes_human(knight); // ❌
    takes_human(mage); // ❌
    takes_human(wizard); // ❌
}

```

此外，解引用强制转换在泛型上下文中是无法工作的。假定我们仅在 humans 上实现某个 trait：

```rust
trait Rest {
    fn rest(&self);
}

impl Rest for Human {
    fn rest(&self) {}
}

fn take_rest<T: Rest>(rester: &T) {
    rester.rest()
}

fn example(human: Human, soldier: Soldier, knight: Knight, mage: Mage, wizard: Wizard) {
    // all types CANNOT be used as Rest types, only Human
    take_rest(&human);
    take_rest(&soldier); // ❌
    take_rest(&knight); // ❌
    take_rest(&mage); // ❌
    take_rest(&wizard); // ❌
}

```

而且，尽管解引用强制转换在很多场景都可以使用，但它不是万能的。它无法作用于操作数，尽管操作符只是方法调用的语法糖。假定，我们想要`Mage（魔术师）`通过`+=`操作符学会`Spell（拼写）`：

```rust
impl DerefMut for Wizard {
    fn deref_mut(&mut self) -> &mut Mage {
        &mut self.mage
    }
}

impl AddAssign<Spell> for Mage {
    fn add_assign(&mut self, spell: Spell) {
        self.spells.push(spell);
    }
}

fn example(mut mage: Mage, mut wizard: Wizard, spell: Spell) {
    mage += spell;
    wizard += spell; // ❌ wizard not coerced to mage here
    wizard.add_assign(spell); // oof, we have to call it like this 🤦
}

```

在具有 OOP 风格的数据继承的编程语言中，一个方法中的`self`的值总是等于调用这个方法的类型，但是在 Rust 中，`self`的值永远等于实现这个方法的类型：

```rust
struct Human {
    profession: &'static str,
    health_points: u32,
}

impl Human {
    // self will always be a Human here, even if we call it on a Soldier
    fn state_profession(&self) {
        println!("I'm a {}!", self.profession);
    }
}

struct Soldier {
    profession: &'static str,
    human: Human,
    weapon: Weapon,
}

fn example(soldier: &Soldier) {
    assert_eq!("servant", soldier.human.profession);
    assert_eq!("spearman", soldier.profession);
    soldier.human.state_profession(); // prints "I'm a servant!"
    soldier.state_profession(); // still prints "I'm a servant!" 🤦
}

```

当在一个新类型上实现`Deref`或`DerefMut`时，上面的陷阱令人震惊。假定我们想要创建一个`SortedVec`类型，它就是一个`Vec`只不过是有序的。下面是我们可能的实现方式：

```rust
struct SortedVec<T: Ord>(Vec<T>);

impl<T: Ord> SortedVec<T> {
    fn new(mut vec: Vec<T>) -> Self {
        vec.sort();
        SortedVec(vec)
    }
    fn push(&mut self, t: T) {
        self.0.push(t);
        self.0.sort();
    }
}

```

显然，这里我们不能实现`DerefMut<Target = Vec<T>>`，否则任何使用`SortedVec`的人都能轻易打破已排好的顺序。但是，实现`Deref<Target = Vec<T>>`就一定安全么？试试找出下面程序中的 bug:

```rust
use std::ops::Deref;

struct SortedVec<T: Ord>(Vec<T>);

impl<T: Ord> SortedVec<T> {
    fn new(mut vec: Vec<T>) -> Self {
        vec.sort();
        SortedVec(vec)
    }
    fn push(&mut self, t: T) {
        self.0.push(t);
        self.0.sort();
    }
}

impl<T: Ord> Deref for SortedVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}

fn main() {
    let sorted = SortedVec::new(vec![2, 8, 6, 3]);
    sorted.push(1);
    let sortedClone = sorted.clone();
    sortedClone.push(4);
}

```

我们未曾给`SortedVec`实现`Clone`，所以当我们调用`.clone()`方法时，编译器使用解引用强制转换把它解析为`Vec`上的方法调用，所以它会返回一个`Vec`而不是一个`SortedVec`！

```rust
fn main() {
    let sorted: SortedVec<i32> = SortedVec::new(vec![2, 8, 6, 3]);
    sorted.push(1); // still sorted

    // calling clone on SortedVec actually returns a Vec 🤦
    let sortedClone: Vec<i32> = sorted.clone();
    sortedClone.push(4); // sortedClone no longer sorted 💀
}

```

不管怎样，上面的限制、约束或者陷阱都不是 Rust 的错，因为 Rust 从来都没有被设计成一门 OO（面向对象）的语言或者把支持 OOP（面向对象程序设计）模式放在首位。

本节的要点在于不要试图在`Deref`和`DerefMut`的实现耍小聪明。它们仅仅适用于智能指针类型，目前只能在标准库中实现，因为智能指针类型目前需要 unstable 的特性和编译器的魔法才能工作。如果我们想要类似于`Deref`和`DerefMut`的功能和行为，我们可以去了解一下后面会提到的`AsRef`和`AsMut`。

### Index & IndexMut

```rust
trait Index<Idx: ?Sized> {
    type Output: ?Sized;
    fn index(&self, index: Idx) -> &Self::Output;
}

trait IndexMut<Idx>: Index<Idx> where Idx: ?Sized {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
}

```

我们可以将`[]`索引到带有 T 值的`Index<T, Output = U>`类型，索引操作将返回`&U`值。为了语法方便，编译器会自动在索引操作返回值的前面插入一个解引用操作符`*`：

```rust
fn main() {
    // Vec<i32> impls Index<usize, Output = i32> so
    // indexing Vec<i32> should produce &i32s and yet...
    let vec = vec![1, 2, 3, 4, 5];
    let num_ref: &i32 = vec[0]; // ❌ expected &i32 found i32

    // above line actually desugars to
    let num_ref: &i32 = *vec[0]; // ❌ expected &i32 found i32

    // both of these alternatives work
    let num: i32 = vec[0]; // ✅
    let num_ref = &vec[0]; // ✅
}

```

为了展示我们自己如何实现`Index`，下面是一个有趣的示例，这个例子展示了我们如何使用一个新类型和`Index`trait 在`Vec`上实现环绕索引和非负索引：

```rust
use std::ops::Index;

struct WrappingIndex<T>(Vec<T>);

impl<T> Index<usize> for WrappingIndex<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        &self.0[index % self.0.len()]
    }
}

impl<T> Index<i128> for WrappingIndex<T> {
    type Output = T;
    fn index(&self, index: i128) -> &T {
        let self_len = self.0.len() as i128;
        let idx = (((index % self_len) + self_len) % self_len) as usize;
        &self.0[idx]
    }
}

#[test] // ✅
fn indexes() {
    let wrapping_vec = WrappingIndex(vec![1, 2, 3]);
    assert_eq!(1, wrapping_vec[0_usize]);
    assert_eq!(2, wrapping_vec[1_usize]);
    assert_eq!(3, wrapping_vec[2_usize]);
}

#[test] // ✅
fn wrapping_indexes() {
    let wrapping_vec = WrappingIndex(vec![1, 2, 3]);
    assert_eq!(1, wrapping_vec[3_usize]);
    assert_eq!(2, wrapping_vec[4_usize]);
    assert_eq!(3, wrapping_vec[5_usize]);
}

#[test] // ✅
fn neg_indexes() {
    let wrapping_vec = WrappingIndex(vec![1, 2, 3]);
    assert_eq!(1, wrapping_vec[-3_i128]);
    assert_eq!(2, wrapping_vec[-2_i128]);
    assert_eq!(3, wrapping_vec[-1_i128]);
}

#[test] // ✅
fn wrapping_neg_indexes() {
    let wrapping_vec = WrappingIndex(vec![1, 2, 3]);
    assert_eq!(1, wrapping_vec[-6_i128]);
    assert_eq!(2, wrapping_vec[-5_i128]);
    assert_eq!(3, wrapping_vec[-4_i128]);
}

```

这里没有要求`Idx`类型是数值类型或者是一个`Range`，它也可以是一个枚举！下面是一个使用篮球位置在一支球队里检索球员的例子：

```rust
use std::ops::Index;

enum BasketballPosition {
    PointGuard,
    ShootingGuard,
    Center,
    PowerForward,
    SmallForward,
}

struct BasketballPlayer {
    name: &'static str,
    position: BasketballPosition,
}

struct BasketballTeam {
    point_guard: BasketballPlayer,
    shooting_guard: BasketballPlayer,
    center: BasketballPlayer,
    power_forward: BasketballPlayer,
    small_forward: BasketballPlayer,
}

impl Index<BasketballPosition> for BasketballTeam {
    type Output = BasketballPlayer;
    fn index(&self, position: BasketballPosition) -> &BasketballPlayer {
        match position {
            BasketballPosition::PointGuard => &self.point_guard,
            BasketballPosition::ShootingGuard => &self.shooting_guard,
            BasketballPosition::Center => &self.center,
            BasketballPosition::PowerForward => &self.power_forward,
            BasketballPosition::SmallForward => &self.small_forward,
        }
    }
}

```

### Drop

```rust
trait Drop {
    fn drop(&mut self);
}
```

如果一个类型实现了`Drop`，那么`drop`将会在该类型离开作用域但是销毁之前被调用。我们很少需要去为我们的类型实现它，但是如果一个类型中持有某些外部资源，这些资源需要在类型销毁时被清理，这种情况下就会用到了。

标准库中有一个`BufWriter`类型让我们能够把写入的数据缓冲到`Write`类型中。但是，如果`BufWriter`在它里面的内容被刷入到底层的`Write`类型之前就被销毁了，该怎么办呢？幸运的是那是不可能的！`BufWriter`实现了`Drop`trait，因此，无论什么它什么时候离开作用域，`flush`总会被调用！

```rust
impl<W: Write> Drop for BufWriter<W> {
    fn drop(&mut self) {
        self.flush_buf();
    }
}
```

此外，Rust 中的`Mutexs`没有`unlock()`方法，因为它们不需要！在`Mutex`上调用`lock()`会返回一个`MutexGuard`，当`MutexGuard`离开作用域时，它会自动解锁（unlock）`Mutex`，这要归功于它的`Drop`实现:

```rust
impl<T: ?Sized> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        unsafe {
            self.lock.inner.raw_unlock();
        }
    }
}
```

一般而言，如果你正在实现对某类资源的抽象，这类资源需要在使用后被清理，那就是时候充分利用`Drop` trait 了。

## 转换 Traits（Conversion Traits）

### From & Into

```rust
trait From<T> {
    fn from(T) -> Self;
}
```

`From<T>`类型允许我们把`T`转换为`Self`。

```rust
trait Into<T> {
    fn into(self) -> T;
}
```

`Into<T>`类型允许我们把`Self`转换为`T`。
它们就像是一个硬币的两面。我们只能为自己的类型实现`From<T>`，因为`Into<T>`的实现会通过 generic blanket impl 自动提供：

```rust
impl<T, U> Into<U> for T
where
    U: From<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}

```

这两个 trait 之所以存在，是因为它能够让我们以稍微不同的方式来进行 trait 约束（bound）：

```rust
fn function<T>(t: T)
where
    // these bounds are equivalent
    T: From<i32>,
    i32: Into<T>
{
    // these examples are equivalent
    let example: T = T::from(0);
    let example: T = 0.into();
}
```

没有规则强制要求什么时候使用前者或后者，所以在每种情景下采用最合理的方式就可以了。现在让我们来看一个例子：

```rust
struct Point {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Point { x, y }
    }
}

impl From<[i32; 2]> for Point {
    fn from([x, y]: [i32; 2]) -> Self {
        Point { x, y }
    }
}

fn example() {
    // 使用 From
    let origin = Point::from((0, 0));
    let origin = Point::from([0, 0]);

    // 使用 Into
    let origin: Point = (0, 0).into();
    let origin: Point = [0, 0].into();
}

```

这个实现不是对称的，因此，如果我们想要把`Point`转为 tuple 和 array，我们必须显式地添加下面的内容：

```rust
struct Point {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Point { x, y }
    }
}

impl From<Point> for (i32, i32) {
    fn from(Point { x, y }: Point) -> Self {
        (x, y)
    }
}

impl From<[i32; 2]> for Point {
    fn from([x, y]: [i32; 2]) -> Self {
        Point { x, y }
    }
}

impl From<Point> for [i32; 2] {
    fn from(Point { x, y }: Point) -> Self {
        [x, y]
    }
}

fn example() {
    // 从 (i32, i32) 到 Point
    let point = Point::from((0, 0));
    let point: Point = (0, 0).into();

    // 从 Point 到 (i32, i32)
    let tuple = <(i32, i32)>::from(point);
    let tuple: (i32, i32) = point.into();

    // 从 [i32; 2] 到 Point
    let point = Point::from([0, 0]);
    let point: Point = [0, 0].into();

    // 从 Point 到 [i32; 2]
    let array = <[i32; 2]>::from(point);
    let array: [i32; 2] = point.into();
}

```

`From<T>`的一个常见用法是精简模板代码。假定我们想要在程序中添加一个`Triangle`类型，它里面包含三个`Point`，下面是我们可以构造它的方式：

```rust
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Point {
        Point { x, y }
    }
}

struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

impl Triangle {
    fn new(p1: Point, p2: Point, p3: Point) -> Triangle {
        Triangle { p1, p2, p3 }
    }
}

impl<P> From<[P; 3]> for Triangle
where
    P: Into<Point>
{
    fn from([p1, p2, p3]: [P; 3]) -> Triangle {
        Triangle {
            p1: p1.into(),
            p2: p2.into(),
            p3: p3.into(),
        }
    }
}

fn example() {
    // 手动构造
    let triangle = Triangle {
        p1: Point {
            x: 0,
            y: 0,
        },
        p2: Point {
            x: 1,
            y: 1,
        },
        p3: Point {
            x: 2,
            y: 2,
        },
    };

    // 使用 Point::new
    let triangle = Triangle {
        p1: Point::new(0, 0),
        p2: Point::new(1, 1),
        p3: Point::new(2, 2),
    };

    // 使用 From<(i32, i32)> for Point
    let triangle = Triangle {
        p1: (0, 0).into(),
        p2: (1, 1).into(),
        p3: (2, 2).into(),
    };

    // 使用 Triangle::new + From<(i32, i32)> for Point
    let triangle = Triangle::new(
        (0, 0).into(),
        (1, 1).into(),
        (2, 2).into(),
    );

    // 使用 From<[Into<Point>; 3]> for Triangle
    let triangle: Triangle = [
        (0, 0),
        (1, 1),
        (2, 2),
    ].into();
}

```

关于你应该什么时候，以什么方式、什么理由来为我们的类型实现`From<T>`，并没有强制规定，这取决于你对具体情况的判断。

`Into<T>`一个常见的用途是，使得需要拥有值的函数具有通用性，而不必关心它们是拥有值还是借用值。

```rust
struct Person {
    name: String,
}

impl Person {
    // 接受:
    // - String
    fn new1(name: String) -> Person {
        Person { name }
    }

    // 接受:
    // - String
    // - &String
    // - &str
    // - Box<str>
    // - Cow<'_, str>
    // - char
    // 因为上面所有的类型都可以转换为 String
    fn new2<N: Into<String>>(name: N) -> Person {
        Person { name: name.into() }
    }
}

```

## 错误处理（Error Handling）

讨论错误处理和`Error` trait 的最好时机应该是紧跟在`Display`、`Debug`、`Any`、`From`之后，但是在`TryFrom`之前，这也是为什么把错误处理部分尴尬地嵌入在转换 trait 之间。

### Error

```rust
trait Error: Debug + Display {
    // 提供默认实现
    fn source(&self) -> Option<&(dyn Error + 'static)>;
    fn backtrace(&self) -> Option<&Backtrace>;
    fn description(&self) -> &str;
    fn cause(&self) -> Option<&dyn Error>;
}
```

在 Rust 中，错误（error）是被返回（return）的，而不是被抛出（throw）的，让我们看个例子。

因为整数除以 0 会 panic，如果我们想要让我们的程序更为安全，我们可以实现一个`safe_div`函数，它会返回一个`Result`，就像下面这样：

```rust
use std::fmt;
use std::error;

#[derive(Debug, PartialEq)]
struct DivByZero;

impl fmt::Display for DivByZero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "division by zero error")
    }
}

impl error::Error for DivByZero {}

fn safe_div(numerator: i32, denominator: i32) -> Result<i32, DivByZero> {
    if denominator == 0 {
        return Err(DivByZero);
    }
    Ok(numerator / denominator)
}

#[test] // ✅
fn test_safe_div() {
    assert_eq!(safe_div(8, 2), Ok(4));
    assert_eq!(safe_div(5, 0), Err(DivByZero));
}

```

因为错误是被返回而不是被抛出，所以这些错误必须被显式地处理，如果当前函数无法处理错误，该函数应该把错误传递给自己的调用者。传递错误的最常用方式是使用`?`操作符，它是现在已经弃用的`try!`宏的语法糖：

```rust
macro_rules! try {
    ($expr:expr) => {
        match $expr {
            // if Ok just unwrap the value
            Ok(val) => val,
            // if Err map the err value using From and return
            Err(err) => {
                return Err(From::from(err));
            }
        }
    };
}
```

如果我们想要写一个函数，该函数读取文件内容到`String`里，我们可以像这样写：

```rust
use std::io::Read;
use std::path::Path;
use std::io;
use std::fs::File;

fn read_file_to_string(path: &Path) -> Result<String, io::Error> {
    let mut file = File::open(path)?; // ⬆️ io::Error
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // ⬆️ io::Error
    Ok(contents)
}
```

假定我们当前正在读取的文件内容是一串数字，并且我们想要把这些数字求和，我们可能会把函数更新成这样：

```rust
use std::io::Read;
use std::path::Path;
use std::io;
use std::fs::File;

fn sum_file(path: &Path) -> Result<i32, /*这里放置什么? */> {
    let mut file = File::open(path)?; // ⬆️ io::Error
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // ⬆️ io::Error
    let mut sum = 0;
    for line in contents.lines() {
        sum += line.parse::<i32>()?; // ⬆️ ParseIntError
    }
    Ok(sum)
}
```

但是，现在我们的`Result`里的错误类型应该是什么？它要么返回一个`io::Error`，要么返回一个`ParseIntError`。我们尝试寻找第三种方式来解决这个问题，以最快最乱的方式开始，以最健壮的方式结束。

第一种方式就是，识别出所有实现了`Error`和`Display`的类型，这样我们把所有的错误映射（map）到`String`类型并把`String`作为我们的错误类型:

```rust
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

fn sum_file(path: &Path) -> Result<i32, String> {
    let mut file = File::open(path)
        .map_err(|e| e.to_string())?; // ⬆️ io::Error -> String
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| e.to_string())?; // ⬆️ io::Error -> String
    let mut sum = 0;
    for line in contents.lines() {
        sum += line.parse::<i32>()
            .map_err(|e| e.to_string())?; // ⬆️ ParseIntError -> String
    }
    Ok(sum)
}
```

但是，这种方式的缺点在于，我们会丢弃所有的错误类型信息，从而导致调用者在处理错误时十分困难。

另外一个不太明显的优点则是，我们可以定制字符串来提供更多的特定上下文信息。例如，`ParseIntError`通常会变成字符串`“invalid digit found in string”`，这个信息就非常模糊并且没有提及无效的字符串是什么或者它正在尝试解析到哪一类整数类型。如果我们正在调试这个问题，这个错误信息几乎没什么用。尽管如此，我们还可以自己动手提供所有的上下文信息来改善这个问题：

```rust
sum += line.parse::<i32>()
    .map_err(|_| format!("failed to parse {} into i32", line))?;

```

第二种方式则是充分利用标准库中的 generic blanket impl：

```rust
impl<E: error::Error> From<E> for Box<dyn error::Error>;
```

这意味着，任意的`Error`类型都可以通过`?`被隐式地转换为`Box<dyn error::Error>`，因此我们可以把任何可能产生错误的函数返回的`Result`中的错误类型设置为`Box<dyn error::Error>`，这样`?`操作符就可以帮我们完成剩下的工作：

```rust
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::error;

fn sum_file(path: &Path) -> Result<i32, Box<dyn error::Error>> {
    let mut file = File::open(path)?; // ⬆️ io::Error -> Box<dyn error::Error>
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // ⬆️ io::Error -> Box<dyn error::Error>
    let mut sum = 0;
    for line in contents.lines() {
        sum += line.parse::<i32>()?; // ⬆️ ParseIntError -> Box<dyn error::Error>
    }
    Ok(sum)
}

```

虽然更为简洁，但是它似乎也存在着前面一种方式的缺点，即丢掉了类型信息。大多数情况下的确如此，但是如果调用者知道函数的实现细节，它们仍然可以通过使用`error::Error`上的`downcast_ref()`方法来处理不同的错误类型，这与它在`dyn Any`类型上的作用相同。

```rust
fn handle_sum_file_errors(path: &Path) {
    match sum_file(path) {
        Ok(sum) => println!("the sum is {}", sum),
        Err(err) => {
            if let Some(e) = err.downcast_ref::<io::Error>() {
                // 处理 io::Error
            } else if let Some(e) = err.downcast_ref::<ParseIntError>() {
                // 处理 ParseIntError
            } else {
                // 我们知道 sum_file 只会返回上面错误中的其中一个
                // 所以不会到达这个分支
                unreachable!();
            }
        }
    }
}

```

第三种方法是最稳健和类型安全的方法，它可以汇总这些不同的错误，使用一个枚举类型构建我们自己的自定义错误类型：

```rust
use std::num::ParseIntError;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::error;
use std::fmt;

#[derive(Debug)]
enum SumFileError {
    Io(io::Error),
    Parse(ParseIntError),
}

impl From<io::Error> for SumFileError {
    fn from(err: io::Error) -> Self {
        SumFileError::Io(err)
    }
}

impl From<ParseIntError> for SumFileError {
    fn from(err: ParseIntError) -> Self {
        SumFileError::Parse(err)
    }
}

impl fmt::Display for SumFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SumFileError::Io(err) => write!(f, "sum file error: {}", err),
            SumFileError::Parse(err) => write!(f, "sum file error: {}", err),
        }
    }
}

impl error::Error for SumFileError {
    // 这个方法的默认实现总是返回 None
    //但是我们现在重写它，让它更有用    
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            SumFileError::Io(err) => err,
            SumFileError::Parse(err) => err,
        })
    }
}

fn sum_file(path: &Path) -> Result<i32, SumFileError> {
    let mut file = File::open(path)?; // ⬆️ io::Error -> SumFileError
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // ⬆️ io::Error -> SumFileError
    let mut sum = 0;
    for line in contents.lines() {
        sum += line.parse::<i32>()?; // ⬆️ ParseIntError -> SumFileError
    }
    Ok(sum)
}

fn handle_sum_file_errors(path: &Path) {
    match sum_file(path) {
        Ok(sum) => println!("the sum is {}", sum),
        Err(SumFileError::Io(err)) => {
            // 处理 io::Error
        },
        Err(SumFileError::Parse(err)) => {
            // 处理 ParseIntError
        },
    }
}

```

## 继续转换类型（Conversion Traits Continued）

### TryFrom & TryInto

`TryFrom`和`TryInto`是`From`和`Into`的可能会失败的版本。

```rust
trait TryFrom<T> {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

trait TryInto<T> {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}

```

类似于`Into`，我们无法实现`TryInto`，因为它的实现是由 generic blanket impl提供：

```rust
impl<T, U> TryInto<U> for T
where
    U: TryFrom<T>,
{
    type Error = U::Error;

    fn try_into(self) -> Result<U, U::Error> {
        U::try_from(self)
    }
}

```

假定在我们的程序上下文环境中，`Point`中的`x`和`y`如果值小于`-1000`或者大于`1000`没有意义。下面是我们使用`TryFrom`重写之前的`From`实现来告诉用户，现在这种转换可以失败。

```rust
use std::convert::TryFrom;
use std::error;
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct OutOfBounds;

impl fmt::Display for OutOfBounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "out of bounds")
    }
}

impl error::Error for OutOfBounds {}

// 现在是可以出错的
impl TryFrom<(i32, i32)> for Point {
    type Error = OutOfBounds;
    fn try_from((x, y): (i32, i32)) -> Result<Point, OutOfBounds> {
        if x.abs() > 1000 || y.abs() > 1000 {
            return Err(OutOfBounds);
        }
        Ok(Point { x, y })
    }
}

// 仍然是不会出错的
impl From<Point> for (i32, i32) {
    fn from(Point { x, y }: Point) -> Self {
        (x, y)
    }
}

```

下面是对`Triangle`的`TryFrom<[TryInto<Point>; 3]>`实现：

```rust
use std::convert::{TryFrom, TryInto};
use std::error;
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct OutOfBounds;

impl fmt::Display for OutOfBounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "out of bounds")
    }
}

impl error::Error for OutOfBounds {}

impl TryFrom<(i32, i32)> for Point {
    type Error = OutOfBounds;
    fn try_from((x, y): (i32, i32)) -> Result<Self, Self::Error> {
        if x.abs() > 1000 || y.abs() > 1000 {
            return Err(OutOfBounds);
        }
        Ok(Point { x, y })
    }
}

struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

impl<P> TryFrom<[P; 3]> for Triangle
where
    P: TryInto<Point>,
{
    type Error = P::Error;
    fn try_from([p1, p2, p3]: [P; 3]) -> Result<Self, Self::Error> {
        Ok(Triangle {
            p1: p1.try_into()?,
            p2: p2.try_into()?,
            p3: p3.try_into()?,
        })
    }
}

fn example() -> Result<Triangle, OutOfBounds> {
    let t: Triangle = [(0, 0), (1, 1), (2, 2)].try_into()?;
    Ok(t)
}

```

### FromStr

```rust
trait FromStr {
    type Err;
    fn from_str(s: &str) -> Result<Self, Self::Err>;
}
```

`FromStr` 类型允许执行一个从`&str`到`Self`的可失败的转换。最常见的使用是在`&str`上调用`.parse()`方法：

```rust
use std::str::FromStr;

fn example<T: FromStr>(s: &'static str) {
    // 这些都是相等的
    let t: Result<T, _> = FromStr::from_str(s);
    let t = T::from_str(s);
    let t: Result<T, _> = s.parse();
    let t = s.parse::<T>(); // 最常见的
}
```

例如，在`Point`上的实现：

```rust
use std::error;
use std::fmt;
use std::iter::Enumerate;
use std::num::ParseIntError;
use std::str::{Chars, FromStr};

#[derive(Debug, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, PartialEq)]
struct ParsePointError;

impl fmt::Display for ParsePointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse point")
    }
}

impl From<ParseIntError> for ParsePointError {
    fn from(_e: ParseIntError) -> Self {
        ParsePointError
    }
}

impl error::Error for ParsePointError {}

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let is_num = |(_, c): &(usize, char)| matches!(c, '0'..='9' | '-');
        let isnt_num = |t: &(_, _)| !is_num(t);

        let get_num =
            |char_idxs: &mut Enumerate<Chars<'_>>| -> Result<(usize, usize), ParsePointError> {
                let (start, _) = char_idxs
                    .skip_while(isnt_num)
                    .next()
                    .ok_or(ParsePointError)?;
                let (end, _) = char_idxs
                    .skip_while(is_num)
                    .next()
                    .ok_or(ParsePointError)?;
                Ok((start, end))
            };

        let mut char_idxs = s.chars().enumerate();
        let (x_start, x_end) = get_num(&mut char_idxs)?;
        let (y_start, y_end) = get_num(&mut char_idxs)?;

        let x = s[x_start..x_end].parse::<i32>()?;
        let y = s[y_start..y_end].parse::<i32>()?;

        Ok(Point { x, y })
    }
}

#[test] // ✅
fn pos_x_y() {
    let p = "(4, 5)".parse::<Point>();
    assert_eq!(p, Ok(Point::new(4, 5)));
}

#[test] // ✅
fn neg_x_y() {
    let p = "(-6, -2)".parse::<Point>();
    assert_eq!(p, Ok(Point::new(-6, -2)));
}

#[test] // ✅
fn not_a_point() {
    let p = "not a point".parse::<Point>();
    assert_eq!(p, Err(ParsePointError));
}
```

`FromStr`和`TryFrom<&str>`有着相同的签名。只要我们通过其中一个实现另一个，先实现哪个并不重要。下面是对`Point`实现`TryFrom<&str>`，假定它已经实现了`FromStr`:

```rust
impl TryFrom<&str> for Point {
    type Error = <Point as FromStr>::Err;
    fn try_from(s: &str) -> Result<Point, Self::Error> {
        <Point as FromStr>::from_str(s)
    }
}

```

### AsRef & AsMut

```rust
trait AsRef<T: ?Sized> {
    fn as_ref(&self) -> &T;
}

trait AsMut<T: ?Sized> {
    fn as_mut(&mut self) -> &mut T;
}

```

`AsRef`被用于轻量级的引用到引用之间的转换。然而，它最常见的一个用途是使函数在是否获取所有权上具有通用性：

```rust
// 接受:
//  - &str
//  - &String
fn takes_str(s: &str) {
    // use &str
}

// 接受:
//  - &str
//  - &String
//  - String
fn takes_asref_str<S: AsRef<str>>(s: S) {
    let s: &str = s.as_ref();
    // 使用 &str
}

fn example(slice: &str, borrow: &String, owned: String) {
    takes_str(slice);
    takes_str(borrow);
    takes_str(owned); // ❌
    takes_asref_str(slice);
    takes_asref_str(borrow);
    takes_asref_str(owned); // ✅
}

```

另一个常见用途是返回一个内部私有数据的引用，该数据由一个保护不变性的类型所包裹。标准库中一个比较好的示例是`String`，它包裹了`Vec<u8>`：

```rust
struct String {
    vec: Vec<u8>,
}
```

内部的`Vec<u8>`不能被公开，因为如果这样的话，人们就会修改里面的字节并破坏`String`中有效的 UTF-8 编码。但是，暴露内部字节数组的一个不可变的只读引用是安全的，即下面的实现：

```rust
impl AsRef<[u8]> for String;
```

一般而言，只有当一个类型包裹了其他类型用来为该内部类型提供了额外功能或者保护内部类型的不变性时，为这样的类型实现`AsRef`才有意义。
让我们来看一个`AsRef`的不合适使用：

```rust
struct User {
    name: String,
    age: u32,
}

impl AsRef<String> for User {
    fn as_ref(&self) -> &String {
        &self.name
    }
}

impl AsRef<u32> for User {
    fn as_ref(&self) -> &u32 {
        &self.age
    }
}

```

一开始是可行的，而且看上去还有点道理，但是当我们为`User`添加更多成员时，问题就出现了：

```rust
struct User {
    name: String,
    email: String,
    age: u32,
    height: u32,
}

impl AsRef<String> for User {
    fn as_ref(&self) -> &String {、
        //我们返回 name 还是 email?        
    }
}

impl AsRef<u32> for User {
    fn as_ref(&self) -> &u32 {
        //我们返回 age 还是 height？
    }
}

```

`User`是由`String`和`u32`组成，但是它并不等同于一个`String`和一个`u32`，甚至我们还会有更多的类型：

```rust
struct User {
    name: Name,
    email: Email,
    age: Age,
    height: Height,
}
```

对于这样的类型实现`AsRef`没有什么意义，因为`AsRef`用于语义相等的事物之间引用到引用的转换，而且`Name`、`Email`、`Age`以及`Height`并不等同于一个`User`。

下面是一个好的示例，其中，我们会引入一个新类型`Moderator`，它只包裹了一个`User`并添加了特定的审核权限：

```rust
struct User {
    name: String,
    age: u32,
}

//不幸地是，标准库并没有提供一个generic blanket impl来避免这种重复的实现
impl AsRef<User> for User {
    fn as_ref(&self) -> &User {
        self
    }
}

enum Privilege {
    BanUsers,
    EditPosts,
    DeletePosts,
}

//尽管 Moderators 有一些特殊权限，它们仍然是普通的 User 
//并且应该做相同的事情
struct Moderator {
    user: User,
    privileges: Vec<Privilege>
}

impl AsRef<Moderator> for Moderator {
    fn as_ref(&self) -> &Moderator {
        self
    }
}

impl AsRef<User> for Moderator {
    fn as_ref(&self) -> &User {
        &self.user
    }
}

//使用 User 和 Moderators （也是一种User）应该都是可以调用的
fn create_post<U: AsRef<User>>(u: U) {
    let user = u.as_ref();
    // etc
}

fn example(user: User, moderator: Moderator) {
    create_post(&user);
    create_post(&moderator); // ✅
}

```

这是有效的，因为`Moderator`就是`User`。下面是`Deref`章节中的例子，我们用了`AsRef`来实现：

```rust
use std::convert::AsRef;

struct Human {
    health_points: u32,
}

impl AsRef<Human> for Human {
    fn as_ref(&self) -> &Human {
        self
    }
}

enum Weapon {
    Spear,
    Axe,
    Sword,
}

// a Soldier is just a Human with a Weapon
struct Soldier {
    human: Human,
    weapon: Weapon,
}

impl AsRef<Soldier> for Soldier {
    fn as_ref(&self) -> &Soldier {
        self
    }
}

impl AsRef<Human> for Soldier {
    fn as_ref(&self) -> &Human {
        &self.human
    }
}

enum Mount {
    Horse,
    Donkey,
    Cow,
}

// a Knight is just a Soldier with a Mount
struct Knight {
    soldier: Soldier,
    mount: Mount,
}

impl AsRef<Knight> for Knight {
    fn as_ref(&self) -> &Knight {
        self
    }
}

impl AsRef<Soldier> for Knight {
    fn as_ref(&self) -> &Soldier {
        &self.soldier
    }
}

impl AsRef<Human> for Knight {
    fn as_ref(&self) -> &Human {
        &self.soldier.human
    }
}

enum Spell {
    MagicMissile,
    FireBolt,
    ThornWhip,
}

// a Mage is just a Human who can cast Spells
struct Mage {
    human: Human,
    spells: Vec<Spell>,
}

impl AsRef<Mage> for Mage {
    fn as_ref(&self) -> &Mage {
        self
    }
}

impl AsRef<Human> for Mage {
    fn as_ref(&self) -> &Human {
        &self.human
    }
}

enum Staff {
    Wooden,
    Metallic,
    Plastic,
}

// a Wizard is just a Mage with a Staff
struct Wizard {
    mage: Mage,
    staff: Staff,
}

impl AsRef<Wizard> for Wizard {
    fn as_ref(&self) -> &Wizard {
        self
    }
}

impl AsRef<Mage> for Wizard {
    fn as_ref(&self) -> &Mage {
        &self.mage
    }
}

impl AsRef<Human> for Wizard {
    fn as_ref(&self) -> &Human {
        &self.mage.human
    }
}

fn borrows_human<H: AsRef<Human>>(human: H) {}
fn borrows_soldier<S: AsRef<Soldier>>(soldier: S) {}
fn borrows_knight<K: AsRef<Knight>>(knight: K) {}
fn borrows_mage<M: AsRef<Mage>>(mage: M) {}
fn borrows_wizard<W: AsRef<Wizard>>(wizard: W) {}

fn example(human: Human, soldier: Soldier, knight: Knight, mage: Mage, wizard: Wizard) {
    // all types can be used as Humans
    borrows_human(&human);
    borrows_human(&soldier);
    borrows_human(&knight);
    borrows_human(&mage);
    borrows_human(&wizard);
    // Knights can be used as Soldiers
    borrows_soldier(&soldier);
    borrows_soldier(&knight);
    // Wizards can be used as Mages
    borrows_mage(&mage);
    borrows_mage(&wizard);
    // Knights & Wizards passed as themselves
    borrows_knight(&knight);
    borrows_wizard(&wizard);
}

```

`Deref`在之前的例子中没有起作用，是因为解引用强制转换是类型间的隐式转换，这就为人们制定错误的想法并对其行为方式的期望留下了空间。`AsRef`能够工作是因为它让类型之间的转换变为显式的，并且没有给开发者错误的想法和期望留有余地。

### Borrow & BorrowMut

```rust
trait Borrow<Borrowed>
where
    Borrowed: ?Sized,
{
    fn borrow(&self) -> &Borrowed;
}

trait BorrowMut<Borrowed>: Borrow<Borrowed>
where
    Borrowed: ?Sized,
{
    fn borrow_mut(&mut self) -> &mut Borrowed;
}

```

这些 trait 被发明用于解决非常具体的问题，即使用`&str`类型的值在`HashSet`、`HashMap`、`BTreeSet`和`BTreeMap`中查找`String`类型的 key。

我们可以把`Borrow<T>`和`BorrowMut<T>`看作更严格的`AsRef<T>`和`AsMut<T>`，它们返回的引用`&T`与`Self`有等价性的`Eq`、`Hash`和`Ord`实现。通过下面的例子会更易于理解：

```rust
use std::borrow::Borrow;
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;

fn get_hash<T: Hash>(t: T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

fn asref_example<Owned, Ref>(owned1: Owned, owned2: Owned)
where
    Owned: Eq + Ord + Hash + AsRef<Ref>,
    Ref: Eq + Ord + Hash
{
    let ref1: &Ref = owned1.as_ref();
    let ref2: &Ref = owned2.as_ref();

    // refs aren't required to be equal if owned types are equal
    assert_eq!(owned1 == owned2, ref1 == ref2); // ❌

    let owned1_hash = get_hash(&owned1);
    let owned2_hash = get_hash(&owned2);
    let ref1_hash = get_hash(&ref1);
    let ref2_hash = get_hash(&ref2);

    // ref hashes aren't required to be equal if owned type hashes are equal
    assert_eq!(owned1_hash == owned2_hash, ref1_hash == ref2_hash); // ❌

    // ref comparisons aren't required to match owned type comparisons
    assert_eq!(owned1.cmp(&owned2), ref1.cmp(&ref2)); // ❌
}

fn borrow_example<Owned, Borrowed>(owned1: Owned, owned2: Owned)
where
    Owned: Eq + Ord + Hash + Borrow<Borrowed>,
    Borrowed: Eq + Ord + Hash
{
    let borrow1: &Borrowed = owned1.borrow();
    let borrow2: &Borrowed = owned2.borrow();

    // borrows are required to be equal if owned types are equal
    assert_eq!(owned1 == owned2, borrow1 == borrow2); // ✅

    let owned1_hash = get_hash(&owned1);
    let owned2_hash = get_hash(&owned2);
    let borrow1_hash = get_hash(&borrow1);
    let borrow2_hash = get_hash(&borrow2);

    // borrow hashes are required to be equal if owned type hashes are equal
    assert_eq!(owned1_hash == owned2_hash, borrow1_hash == borrow2_hash); // ✅

    // borrow comparisons are required to match owned type comparisons
    assert_eq!(owned1.cmp(&owned2), borrow1.cmp(&borrow2)); // ✅
}

```

意识到这些 trait 以及它们为什么存在是有益的，因为它有助于搞清楚`HashSet`、`HashMap`、`BTreeSet`以及`BTreeMap`的某些方法，但是我们很少需要为我们的类型实现这些 trait，因为我们很少需要创建一对儿类型，其中一个是另一个的借用版本。如果我们有某个类型`T`，`&T`在 99.99%的情况下可以完成工作，并且因为 generic blanket impl，`T:Borrorw<T>`已经为所有的类型`T`实现了，所以我们不需要手动地实现它并且我们不需要创建一个`U`以用来`T:Borrow<U>`。

### ToOwned

```rust
trait ToOwned {
    type Owned: Borrow<Self>;
    fn to_owned(&self) -> Self::Owned;

    // 提供默认实现
    fn clone_into(&self, target: &mut Self::Owned);
}
```

`ToOwned`是`Clone`的一个更为通用的版本。`Clone`允许我们获取一个`&T`并把它转为一个`T`，但是`ToOwned`允许我们拿到一个`&Borrowed`并把它转为一个`Owned`，其中`Owned: Borrow<Borrowed>`。

换句话说，我们不能从一个`&str`克隆一个`String`，或者从一个`&Path`克隆一个`PathBuf`，或者从一个`&OsStr`克隆一个`OsString`，因为`clone`方法签名不支持这种跨类型的克隆，这就是`ToOwned`产生的原因。

类似于`Borrow`和`BorrowMut`，知道这个 trait 并理解它什么存在同样是有益的，只是我们几乎不需要为我们的类型实现它。

## Iteration Traits
```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;

    // provided default impls
    fn size_hint(&self) -> (usize, Option<usize>);
    fn count(self) -> usize;
    fn last(self) -> Option<Self::Item>;
    fn advance_by(&mut self, n: usize) -> Result<(), usize>;
    fn nth(&mut self, n: usize) -> Option<Self::Item>;
    fn step_by(self, step: usize) -> StepBy<Self>;
    fn chain<U>(
        self, 
        other: U
    ) -> Chain<Self, <U as IntoIterator>::IntoIter>
    where
        U: IntoIterator<Item = Self::Item>;
    fn zip<U>(self, other: U) -> Zip<Self, <U as IntoIterator>::IntoIter>
    where
        U: IntoIterator;
    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        F: FnMut(Self::Item) -> B;
    fn for_each<F>(self, f: F)
    where
        F: FnMut(Self::Item);
    fn filter<P>(self, predicate: P) -> Filter<Self, P>
    where
        P: FnMut(&Self::Item) -> bool;
    fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F>
    where
        F: FnMut(Self::Item) -> Option<B>;
    fn enumerate(self) -> Enumerate<Self>;
    fn peekable(self) -> Peekable<Self>;
    fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P>
    where
        P: FnMut(&Self::Item) -> bool;
    fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P>
    where
        P: FnMut(&Self::Item) -> bool;
    fn map_while<B, P>(self, predicate: P) -> MapWhile<Self, P>
    where
        P: FnMut(Self::Item) -> Option<B>;
    fn skip(self, n: usize) -> Skip<Self>;
    fn take(self, n: usize) -> Take<Self>;
    fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F>
    where
        F: FnMut(&mut St, Self::Item) -> Option<B>;
    fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F>
    where
        F: FnMut(Self::Item) -> U,
        U: IntoIterator;
    fn flatten(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator;
    fn fuse(self) -> Fuse<Self>;
    fn inspect<F>(self, f: F) -> Inspect<Self, F>
    where
        F: FnMut(&Self::Item);
    fn by_ref(&mut self) -> &mut Self;
    fn collect<B>(self) -> B
    where
        B: FromIterator<Self::Item>;
    fn partition<B, F>(self, f: F) -> (B, B)
    where
        F: FnMut(&Self::Item) -> bool,
        B: Default + Extend<Self::Item>;
    fn partition_in_place<'a, T, P>(self, predicate: P) -> usize
    where
        Self: DoubleEndedIterator<Item = &'a mut T>,
        T: 'a,
        P: FnMut(&T) -> bool;
    fn is_partitioned<P>(self, predicate: P) -> bool
    where
        P: FnMut(Self::Item) -> bool;
    fn try_fold<B, F, R>(&mut self, init: B, f: F) -> R
    where
        F: FnMut(B, Self::Item) -> R,
        R: Try<Ok = B>;
    fn try_for_each<F, R>(&mut self, f: F) -> R
    where
        F: FnMut(Self::Item) -> R,
        R: Try<Ok = ()>;
    fn fold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B;
    fn fold_first<F>(self, f: F) -> Option<Self::Item>
    where
        F: FnMut(Self::Item, Self::Item) -> Self::Item;
    fn all<F>(&mut self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> bool;
    fn any<F>(&mut self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> bool;
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool;
    fn find_map<B, F>(&mut self, f: F) -> Option<B>
    where
        F: FnMut(Self::Item) -> Option<B>;
    fn try_find<F, R>(
        &mut self, 
        f: F
    ) -> Result<Option<Self::Item>, <R as Try>::Error>
    where
        F: FnMut(&Self::Item) -> R,
        R: Try<Ok = bool>;
    fn position<P>(&mut self, predicate: P) -> Option<usize>
    where
        P: FnMut(Self::Item) -> bool;
    fn rposition<P>(&mut self, predicate: P) -> Option<usize>
    where
        Self: ExactSizeIterator + DoubleEndedIterator,
        P: FnMut(Self::Item) -> bool;
    fn max(self) -> Option<Self::Item>
    where
        Self::Item: Ord;
    fn min(self) -> Option<Self::Item>
    where
        Self::Item: Ord;
    fn max_by_key<B, F>(self, f: F) -> Option<Self::Item>
    where
        F: FnMut(&Self::Item) -> B,
        B: Ord;
    fn max_by<F>(self, compare: F) -> Option<Self::Item>
    where
        F: FnMut(&Self::Item, &Self::Item) -> Ordering;
    fn min_by_key<B, F>(self, f: F) -> Option<Self::Item>
    where
        F: FnMut(&Self::Item) -> B,
        B: Ord;
    fn min_by<F>(self, compare: F) -> Option<Self::Item>
    where
        F: FnMut(&Self::Item, &Self::Item) -> Ordering;
    fn rev(self) -> Rev<Self>
    where
        Self: DoubleEndedIterator;
    fn unzip<A, B, FromA, FromB>(self) -> (FromA, FromB)
    where
        Self: Iterator<Item = (A, B)>,
        FromA: Default + Extend<A>,
        FromB: Default + Extend<B>;
    fn copied<'a, T>(self) -> Copied<Self>
    where
        Self: Iterator<Item = &'a T>,
        T: 'a + Copy;
    fn cloned<'a, T>(self) -> Cloned<Self>
    where
        Self: Iterator<Item = &'a T>,
        T: 'a + Clone;
    fn cycle(self) -> Cycle<Self>
    where
        Self: Clone;
    fn sum<S>(self) -> S
    where
        S: Sum<Self::Item>;
    fn product<P>(self) -> P
    where
        P: Product<Self::Item>;
    fn cmp<I>(self, other: I) -> Ordering
    where
        I: IntoIterator<Item = Self::Item>,
        Self::Item: Ord;
    fn cmp_by<I, F>(self, other: I, cmp: F) -> Ordering
    where
        F: FnMut(Self::Item, <I as IntoIterator>::Item) -> Ordering,
        I: IntoIterator;
    fn partial_cmp<I>(self, other: I) -> Option<Ordering>
    where
        I: IntoIterator,
        Self::Item: PartialOrd<<I as IntoIterator>::Item>;
    fn partial_cmp_by<I, F>(
        self, 
        other: I, 
        partial_cmp: F
    ) -> Option<Ordering>
    where
        F: FnMut(Self::Item, <I as IntoIterator>::Item) -> Option<Ordering>,
        I: IntoIterator;
    fn eq<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialEq<<I as IntoIterator>::Item>;
    fn eq_by<I, F>(self, other: I, eq: F) -> bool
    where
        F: FnMut(Self::Item, <I as IntoIterator>::Item) -> bool,
        I: IntoIterator;
    fn ne<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialEq<<I as IntoIterator>::Item>;
    fn lt<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialOrd<<I as IntoIterator>::Item>;
    fn le<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialOrd<<I as IntoIterator>::Item>;
    fn gt<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialOrd<<I as IntoIterator>::Item>;
    fn ge<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialOrd<<I as IntoIterator>::Item>;
    fn is_sorted(self) -> bool
    where
        Self::Item: PartialOrd<Self::Item>;
    fn is_sorted_by<F>(self, compare: F) -> bool
    where
        F: FnMut(&Self::Item, &Self::Item) -> Option<Ordering>;
    fn is_sorted_by_key<F, K>(self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> K,
        K: PartialOrd<K>;
}

```
`Iterator<Item = T>`类型可以被迭代并产生`T`类型。没有`IteratorMut` trait。每个`Iterator`实现可以指定它返回的是不可变引用、可变引用还是拥有通过`Item`关联类型的值。

| `Vec<T>` 方法 | 返回 |
|-----------------|-------------------|
| `.iter()` | `Iterator<Item = &T>` |
| `.iter_mut()` | `Iterator<Item = &mut T>` |
| `.into_iter()` | `Iterator<Item = T>` |

大多数类型没有它们自己的迭代器，这对于初级Rustaceans来说，并不明显，但中级Rustaceans认为这是理所当然的。如果一个类型是可迭代的，我们几乎总是实现自定义的迭代器类型来迭代它，而不是让它自己迭代。

```rust

struct MyType {
    items: Vec<String>
}

impl MyType {
    fn iter(&self) -> impl Iterator<Item = &String> {
        MyTypeIterator {
            index: 0,
            items: &self.items
        }
    }
}

struct MyTypeIterator<'a> {
    index: usize,
    items: &'a Vec<String>
}

impl<'a> Iterator for MyTypeIterator<'a> {
    type Item = &'a String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.items.len() {
            None
        } else {
            let item = &self.items[self.index];
            self.index += 1;
            Some(item)
        }
    }
}

```
为了便于教学，上面的例子展示了如何从头开始实现一个迭代器，但在这种情况下，常用的解决方案是直接延用`Vec`的`iter`方法。
```rust
struct MyType {
    items: Vec<String>
}

impl MyType {
    fn iter(&self) -> impl Iterator<Item = &String> {
        self.items.iter()
    }
}

```
而且，这也是一个需要注意到的generic blanket impl：
```rust
impl<I: Iterator + ?Sized> Iterator for &mut I;
```
一个迭代器的可变引用也是一个迭代器。知道这一点是有用的，因为它让我们能够使用`self`作为接收器（receiver）的迭代器方法，就像`&mut self`接收器一样。

举个例子，假定我们有一个函数，它处理一个数据超过三项的迭代器，但是函数的第一步是取出迭代器的前三项并在迭代完剩余项之前单独处理它们，下面是一个初学者可能会写出的函数实现：
```rust
fn example<I: Iterator<Item = i32>>(mut iter: I) {
    let first3: Vec<i32> = iter.take(3).collect();
    for item in iter { // ❌ iter consumed in line above
        // process remaining items
    }
}

```
这看起来有点让人头疼。`take`方法有一个`self`接收器，所以我们似乎不能在没有消耗整个迭代器的情况下调用它！下面是对上面代码的重构：
```rust
fn example<I: Iterator<Item = i32>>(mut iter: I) {
    let first3: Vec<i32> = vec![
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ];
    for item in iter { // ✅
        // process remaining items
    }
}
```
这样是没问题的，但是实际中通常会这样重构：
```rust
fn example<I: Iterator<Item = i32>>(mut iter: I) {
    let first3: Vec<i32> = iter.by_ref().take(3).collect();
    for item in iter { // ✅
        // process remaining items
    }
}
```
这种写法不太常见，但不管怎样，现在我们知道了。

此外，对于什么类型可以或者不可以是迭代器，并没有规则或者约定。如果一个类型实现了`Iterator`，那么它就是一个迭代器。下面是标准库中一个新颖的例子：
```rust
use std::sync::mpsc::channel;
use std::thread;

fn paths_can_be_iterated(path: &Path) {
    for part in path {
        // iterate over parts of a path
    }
}

fn receivers_can_be_iterated() {
    let (send, recv) = channel();

    thread::spawn(move || {
        send.send(1).unwrap();
        send.send(2).unwrap();
        send.send(3).unwrap();
    });

    for received in recv {
        // iterate over received values
    }
}

```
### IntoIterator
```rust
trait IntoIterator 
where
    <Self::IntoIter as Iterator>::Item == Self::Item, 
{
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}
```
正如其名，`IntoIterator`类型可以转化为迭代器。当一个类型在一个`for-in`循环里被使用的时候，该类型的`into_iter`方法会被调用：
```rust
// vec = Vec<T>
for v in vec {} // v = T

// above line desugared
for v in vec.into_iter() {}
```
不仅`Vec`实现了`IntoIterator`，如果我们想在不可变引用或可变引用上迭代，`&Vec`和`&mut Vec`同样也是如此。
```rust
// vec = Vec<T>
for v in &vec {} // v = &T

// above example desugared
for v in (&vec).into_iter() {}

// vec = Vec<T>
for v in &mut vec {} // v = &mut T

// above example desugared
for v in (&mut vec).into_iter() {}

```

### FromIterator
```rust
trait FromIterator<A> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = A>;
}

```
正如其名，`FromIterator`类型可以从一个迭代器创建而来。`FromIterator`最常用于`Iterator`上的`collect`方法调用：
```rust
fn collect<B>(self) -> B
where
    B: FromIterator<Self::Item>;

```
下面是一个例子，搜集（collect）一个`Iterator<Item = char>` 到 `String`:
```rust
fn filter_letters(string: &str) -> String {
    string.chars().filter(|c| c.is_alphabetic()).collect()
}

```
标准库中所有的集合都实现了`IntoIterator`和`FromIterator`，从而使它们之间的转换更为简单：
```rust
use std::collections::{BTreeSet, HashMap, HashSet, LinkedList};

// String -> HashSet<char>
fn unique_chars(string: &str) -> HashSet<char> {
    string.chars().collect()
}

// Vec<T> -> BTreeSet<T>
fn ordered_unique_items<T: Ord>(vec: Vec<T>) -> BTreeSet<T> {
    vec.into_iter().collect()
}

// HashMap<K, V> -> LinkedList<(K, V)>
fn entry_list<K, V>(map: HashMap<K, V>) -> LinkedList<(K, V)> {
    map.into_iter().collect()
}

// and countless more possible examples

```

## I/O Traits

```rust
trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    // provided default impls
    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize>;
    fn is_read_vectored(&self) -> bool;
    unsafe fn initializer(&self) -> Initializer;
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize>;
    fn read_to_string(&mut self, buf: &mut String) -> Result<usize>;
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()>;
    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized;
    fn bytes(self) -> Bytes<Self>
    where
        Self: Sized;
    fn chain<R: Read>(self, next: R) -> Chain<Self, R>
    where
        Self: Sized;
    fn take(self, limit: u64) -> Take<Self>
    where
        Self: Sized;
}

trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    // provided default impls
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> Result<usize>;
    fn is_write_vectored(&self) -> bool;
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_all_vectored(&mut self, bufs: &mut [IoSlice<'_>]) -> Result<()>;
    fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result<()>;
    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized;
}

```
值得关注的generic blanket impls:
```rust
impl<R: Read + ?Sized> Read for &mut R;
impl<W: Write + ?Sized> Write for &mut W;

```
也就是说，`Read`类型的任何可变引用也都是`Read`，`Write`同理。知道这些是有用的，因为它允许我们使用任何带有`self`接收器的方法，就像它有一个`&mut self`接收器一样。我们已经在迭代器trait部分讲过了它是如何起作用的以及为什么很有用，所以这里不再赘述。

这里我想指出的是，`&[u8]` 实现了`Read`，`Vec<u8>`实现了`Write`。因此我们可以对我们的文件处理函数进行简单的单元测试，通过使用`String`转换为`&[u8]`以及从`Vec<u8>` 转换为`String`：
```rust
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io;

// function we want to test
fn uppercase<R: Read, W: Write>(mut read: R, mut write: W) -> Result<(), io::Error> {
    let mut buffer = String::new();
    read.read_to_string(&mut buffer)?;
    let uppercase = buffer.to_uppercase();
    write.write_all(uppercase.as_bytes())?;
    write.flush()?;
    Ok(())
}

// in actual program we'd pass Files
fn example(in_path: &Path, out_path: &Path) -> Result<(), io::Error> {
    let in_file = File::open(in_path)?;
    let out_file = File::open(out_path)?;
    uppercase(in_file, out_file)
}


// however in unit tests we can use Strings!
#[test] // ✅
fn example_test() {
    let in_file: String = "i am screaming".into();
    let mut out_file: Vec<u8> = Vec::new();
    uppercase(in_file.as_bytes(), &mut out_file).unwrap();
    let out_result = String::from_utf8(out_file).unwrap();
    assert_eq!(out_result, "I AM SCREAMING");
}
```

## 总结
我们一起学到了很多! 事实上是太多了。这是我们现在的样子：

![](https://gitee.com/praying/picbed/raw/master/2021-7-25/1627191055521-image.png)
