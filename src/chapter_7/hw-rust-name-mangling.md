# 华为 | Rust 调用约定和名称修饰规则介绍

作者：周紫鹏 / 后期编辑：张汉东

---

## 背景介绍

 一般来说调用的约定和名称修饰（[Name mangling](https://en.wikipedia.org/wiki/Name_mangling)）对于程序员来说是不需要感知到的，它们一般都是通过编译器来完成，但是如果涉及到汇编代码的函数调用，就需要了解所编写代码最终编译生成的符号信息。调用约定和名称修饰都是属于ABI（[Application Binary Interface](https://en.wikipedia.org/wiki/Application_binary_interface)）范畴内容，当前Rust还没有一个[稳定的ABI](https://people.gnome.org/~federico/blog/rust-stable-abi.html)，所以编译的时候需要将所有依赖的crates一起编译。名称修饰最主要解决的一个问题是，保证代码链接时名称的唯一性。因为在一些编程语言中，支持命名空间、泛型、重载等特性，支持在同一个命名空间中存在相同的函数名称或者标识符，为了做区分就需要在编译时做相应的修饰，比如加入crate信息、命名空间信息等。

在进行[Rust二进制大小分析](https://rustmagazine.github.io/rust_magazine_2021/chapter_4/hw_bin_opt.html)和Rust热补丁分析时，对Rust符号组名规则的分析同样也是必要的，这有助于我们了解Rust代码生成的符号信息，所以本文将对Rust名称修饰规则做一个简单的介绍。

## legacy 规则

Rust从 1.9版本开始使用legacy规则，Rust的legacy规则是基于[Itanium IA-64 C++ ABI](https://itanium-cxx-abi.github.io/cxx-abi/abi.html#mangling)进行了部分的修改，最主要的是在符号最后加了哈希值用来解决部分场景下的符号唯一性的问题。

### 普通函数

```rust
// crate 名称:legacy_mangling
fn foo() {
    println!("foo");
}

fn foo_arg(x: i32) {
    println!("x = {}", x);
}
```

 `foo`组名后：`_ZN15legacy_mangling3foo17h7bf46936ec8fddf1E`

其中`_ZN`为legacy规则的组名符号开头，和Itanium IA-64 C++ ABI规则一致，后面紧跟的15是crate的`legacy_mangling`的字符个数，包括了中间的下划线。3表示函数`foo`，最后面紧跟着17是hash值`h7bf46936ec8fddf1`，并以`E`表示结束。

`foo_arg`组名后：`_ZN15legacy_mangling7foo_arg17h9d3deebd56cd9668E`

可以发现参数并不会体现在前面的组名中，而是通过hash值来做区分。



### 带泛型参数的函数

```rust
// crate 名称:legacy_mangling
fn main() {
    foo_generic(1);
    foo_generic(1.0);
    foo_generic("Hello");
}
fn foo_generic<T: std::fmt::Display>(x: T) {
    println!("x = {:#}", x);
}
```

`foo_generic`组名后：

```rust
_ZN15legacy_mangling11foo_generic17hf6d667a670f9aa59E
_ZN15legacy_mangling11foo_generic17ha0a4115d4cba4650E
_ZN15legacy_mangling11foo_generic17he59e5604e24e62a6E
```

泛型参数实例化后符号差异也是在hash值上，不会将泛型参数信息体现在组名中。由于参数不会体现在，这也使得在函数热补丁场景下，很难确定具体的补丁函数是哪个。

### 结构体方法

```rust
fn main() {
    let point = Point{
        x: 1,
        y: 2,
    };
    println!("{}", point.add());
    println!("{}", point.sub())
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn add(&self) -> i32 {
        self.x + self.y
    }

    fn sub(&self) -> i32 {
        self.x - self.y
    }
}
```

Point结构体方法`add`和`sub`组名后:

```
_ZN15legacy_mangling5Point3add17h9b332fc1bb45a67eE
_ZN15legacy_mangling5Point3sub17hf189faef70b4895dE
```

组名后是crate + Struct + func + hash的组成方式，同样是不会携带参数信息。由于Struct名称也会当成组名的一部分，所以此时不允许定义mod和Struct同名的子模块。编译器在编译的时候，也会判断函数是否被使用，如果没有被使用的函数，不会进行编译，比如main函数中只调用了`point.add()`，那不会将sub函数编译到二进制中，这也是编译器对可执行二进制大小的优化。

### trait方法

```rust
fn main() {
    let point = Point{
        x: 1,
        y: 2,
    };
    println!("{}", point.add());
}

pub trait Compute {
    fn add(&self) -> i32;
}

struct Point {
    x: i32,
    y: i32,
}

impl Compute for Point{
    fn add(&self) -> i32 {
        self.x + self.y
    }
}
```

组名后：`_ZN67_$LT$legacy_mangling..Point$u20$as$u20$legacy_mangling..Compute$GT$3add17h9b332fc1bb45a67eE`

可以看到组名之后变得相对比较复杂，因为我们在不违反孤儿原则的情况下，可以实现另外crate中的trait，或者Struct在另外的crate中，所以在组名中的trait和Struct包含了crate名称。`$`和`.`两个符号是保留的特殊符号，这两个符号在编码中用来命名会报错。



### 闭包

```rust
fn main() {
    let add_one = |x: u32| -> u32 { x + 1 };
    println!("{}", add_one(1));

}
```

组名后：`_ZN15legacy_mangling4main28_$u7b$$u7b$closure$u7d$$u7d$17h5e4f3fa236bcd1c3E`

闭包组名会包含crate名称和闭包所在函数名称，并且会携带`closure`关键字，但是不会体现具体的捕获参数等信息。

关于legacy规则的其他语法特性的组名方式此处不再进行更多的介绍，并且官方对于legacy的组名规则说明也比较少，后面介绍正在标准化过程中的V0规则。

## V0规则

  V0规则的[RFC](https://rust-lang.github.io/rfcs/2603-rust-symbol-name-mangling-v0.html)当前正在实现过程中，对应的[ISSUE](https://github.com/rust-lang/rust/issues/60705)和[PR](https://github.com/rust-lang/rfcs/pull/2603)，从Rust1.39版本之后的Nightly版本中可以通过rustflags设置使用V0规则。使用方式：可以通过设置`RUSTFLAGS=-Zsymbol-mangling-version=v0`或者在.cargo/config中添加rustflags

  ```rust
  [build]
  rustflags = ["-Z", "symbol-mangling-version=v0"]
  ```

  当前V0规则标准化过程涉及的GDB工具、GCC、Perf、valgrind等适配已经完成，按照[进展ISSUE](https://github.com/rust-lang/rust/issues/60705)描述的情况看，只剩下Doc相关的适配，相信在不久将来会合入到stable版本中。

  RFC中总结了当前legacy规则存在如下一些问题：

  - 通用的参数和一些其他的信息在重组名过程中会丢失，并且不能从符号中获取到单态函数的参数类型。如同我们前面做的测试，泛型函数和带参数的函数并不能通过重组名的符号反推出来，因为都是以Hash值作为结尾。
  - 方案存在不一致情况，大部分使用 [Itanium ABI](http://refspecs.linuxbase.org/cxxabi-1.86.html#mangling)样式进行编码，但有些却没有使用
  - 生成的符号中包含`.`，但是该符号在部分平台上不支持
  - 它取决于编译器内部结构，其结果无法被其他编译器实现或外部工具复制

  V0规则主要解决如下问题：

  - 它以可逆的方式编码有关泛型参数的信息。也就是可以通过符号反推出泛型的参数信息。
  - 它有一个一致的定义，不依赖于漂亮地打印某些语言结构。
  - 字符由`A-Z`，`a-z`， `0-9`，和`_`组成

  V0规则很重要的一个点是取消了Hash值，可以通过重组后的符号信息解码出源码的信息，这也是后续如果Rust的ABI稳定的一部分，可以预测给定的代码生成的符号信息。

  下面以几个简单的例子介绍V0规则函数名称重组之后的情况



### 普通函数

```rust
// crate 名称:v0_mangling
fn foo() {
    println!("foo");
}

fn foo_arg(x: i32) {
    println!("x = {}", x);
}
```

 `foo`组名后：`_RNvCs1L72TZisdJI_11v0_mangling3foo`

`foo_arg`组名后：`_RNvCs1L72TZisdJI_11v0_mangling7foo_arg`

V0 组名规则以_R作为开头，去掉了后面的Hash值，普通函数中也去掉了E函数作为结尾。



### mod中函数

```rust
fn foo() {
    fn bar() {}
}

mod foo {
    fn bar() {}
}
```

```
_RNvNtCs1234_7mycrate3foo3bar
<>^^^^^<----><------><--><-->
 ||||||   |      |     |   |
 ||||||   |      |     |   +--- "bar" identifier
 ||||||   |      |     +------- "foo" identifier
 ||||||   |      +------------- "mycrate" identifier
 ||||||   +-------------------- disambiguator for "mycrate"
 |||||+------------------------ start-tag for "mycrate"
 ||||+------------------------- namespace tag for "foo"
 |||+-------------------------- start-tag for "foo"
 ||+--------------------------- namespace tag for "bar"
 |+---------------------------- start-tag for "bar"
 +----------------------------- common Rust symbol prefix
```

上图从RFC中复制的内容，对每个字段进行了详细的说明，当然具体的编码方式可以参见RFC。



### 带泛型参数的函数

```rust
// crate 名称:legacy_mangling
fn main() {
    foo_generic(1);
    foo_generic(1.0);
    foo_generic("Hello");
}
fn foo_generic<T: std::fmt::Display>(x: T) {
    println!("x = {:#}", x);
}
```

`foo_generic`组名后：

```rust
_RINvCs1L72TZisdJI_11v0_mangling11foo_genericReEB2_        -->  foo_generic("Hello");
_RINvCs1L72TZisdJI_11v0_mangling11foo_genericdEB2_         -->  foo_generic(1.0);
_RINvCs1L72TZisdJI_11v0_mangling11foo_genericlEB2_         -->  foo_generic(1);
```

泛型参数实例化之后会再最后的参数中体现，其中参数对照如下，比如`foo_generic("Hello")`传入的是`&str`所以符号名称为`ReE`，`R`代表`&`，`e`表示的是str类型，E表示参数结束。同理传入为float的实例化是`dE`，传入i32的实例化是`lE`。

```
<type> = <basic-type>
       | <path>                      // named type
       | "A" <type> <const>          // [T; N]
       | "S" <type>                  // [T]
       | "T" {<type>} "E"            // (T1, T2, T3, ...)
       | "R" [<lifetime>] <type>     // &T
       | "Q" [<lifetime>] <type>     // &mut T
       | "P" <type>                  // *const T
       | "O" <type>                  // *mut T
       | "F" <fn-sig>                // fn(...) -> ...
       | "D" <dyn-bounds> <lifetime> // dyn Trait<Assoc = X> + Send + 'a
       | <backref>

<basic-type> = "a"      // i8
             | "b"      // bool
             | "c"      // char
             | "d"      // f64
             | "e"      // str
             | "f"      // f32
             | "h"      // u8
             | "i"      // isize
             | "j"      // usize
             | "l"      // i32
             | "m"      // u32
             | "n"      // i128
             | "o"      // u128
             | "s"      // i16
             | "t"      // u16
             | "u"      // ()
             | "v"      // ...
             | "x"      // i64
             | "y"      // u64
             | "z"      // !
             | "p"      // placeholder (e.g. for generic params), shown as _
```

其他语法特性的组名规则不再一一进行列举，V0相对legacy规则有一些的改进，可以方便的从组名后的符号方便的推测出组名之后的代码。组名规则的变更会涉及到大量的工具需要适配，V0的RFC从18年就开始提出，当前大部分的适配已经完成，V0规则对需要精确知道组名之后的源代码是很有帮助的，比如在我们实际业务中会涉及到Rust函数级别的热补丁实现，就需要知道我们所要打补丁的代码最终会生成的符号组成，legacy规则就很难解决这个问题。


## 参考：

https://doc.rust-lang.org/reference/abi.html

https://people.gnome.org/~federico/blog/rust-stable-abi.html

https://rust-lang.github.io/rfcs/2603-rust-symbol-name-mangling-v0.html#unresolved-questions