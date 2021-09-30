# 为 嵌入式 C  程序员编写的 Rust 指南

译者：张汉东 

原文：https://docs.opentitan.org/doc/ug/rust_for_c/ 

作者： Google OpenTitan 团队

---

## 引子

在今年（2021）RustConf 大会上，Miguel Young 分享了《移动构造函数，Rust 中可能吗？》的演讲。

在观看这个演讲视频的时候，本人深挖了一下作者的信息，发现了这篇文章。

> Miguel Young 是来自 Google OpenTitan 项目的开发者。OpenTitan 项目，想通过开源框架减少芯片被破解的可能。
>
> OpenTitan  **将由非营利组织lowRISC监督，** 该公司正在开发基于RISC-V架构的免费微处理器。
>
> **OpenTitan项目涵盖了各种逻辑组件的开发** RoT芯片的需求，包括基于RISC-V架构的lowRISC Ibex开放式微处理器，加密协处理器，硬件随机数生成器，恒定和随机存取存储器数据和密钥存储层次结构，机制保护，I / O输入块，安全启动媒体等
>
> 可以在必要时使用OpenTitan，以确保系统的硬件和软件组件的完整性，并确保未更换关键的系统组件，并基于制造商验证和授权的代码。
>
> **基于OpenTitan的芯片可用于** 服务器主板，网卡，消费类设备，路由器，物联网设备，以验证固件（检测恶意软件对固件的修改），提供加密的唯一系统标识符（硬件防伪保护）以及保护加密密钥（如果出现以下情况，则隔离密钥）：攻击者可以获得对计算机的物理访问权限），提供与安全相关的服务，并维护无法编辑或删除的隔离审核跟踪。

本人又深挖了一下 [OpenTitan](https://github.com/lowRISC/opentitan) 项目，在 GitHub 的语言成分分析中看不到 Rust 的痕迹。但是在源码中搜索 Rust，则发现很多 Rust 痕迹。

一、其中，OpenTitan 的 Software 部分支持 Rust 实现。

- 设备软件的固件镜像，支持 Rust 实现。
- Host 软件必须用 Rust 实现 （也支持 Cpp）。

二、 ROM_EXT 由 Rust 实现

OpenTitan  安全启动过程中，为了增加一定程度的灵活性，特别是为了允许制造商的特定配置和提供安全更新的设施--OpenTitan设计了扩展ROM（ROM_EXT），常驻闪存中。

ROM_EXT由一个 manifest  和 image 本身组成。当 image  生成时，manifest   是 "空白 "的。ROM_EXT签名者的责任是更新manifest  ，签名image，并将签名加入其中。

源码：https://github.com/lowRISC/opentitan/tree/master/sw/host/rom_ext_image_tools/signer ，它是一个 Host 软件。

在 Readme 里介绍了他们为什么选择 Rust : https://github.com/lowRISC/opentitan/blob/master/sw/host/rom_ext_image_tools/signer/README.md 

该项目中其他比较有用的资源：

1. RISC-V Assembly Style Guide ： https://docs.opentitan.org/doc/rm/asm_coding_style/ 
2. FPGA Reference Manual：https://docs.opentitan.org/doc/rm/ref_manual_fpga/ 
3. Rust for Embedded C Programmers https://docs.opentitan.org/doc/ug/rust_for_c/ 

接下来是 Rust for Embedded C Programmers 的翻译正文。

## 正文



### 前言

本文档旨在作为Rust的介绍，针对的是对嵌入式系统C语言有深入接触的工程师，以及几乎没有C++经验和不了解Rust的工程师。本文档将包含以下内容：

- 提供嵌入式 C 语言工具库中和 Rust 相似的内容
- 讨论这些相似内容和 C 语言的区别
- 指出 Rust 内存和执行模型 与 C 语言的实质性差异
- 介绍 Rust 特有的功能，这些功能要么是使用 Rust 的关键，要么是非常有用（引用、生命周期、泛型和特质）。

虽然本文并不是很严谨，但在脚注中也包含了语言律法相关的说明，只不过这些说明不影响理解本文。

学习 Rust 的一个很好的方法是使用编译器，看看能编译什么。 Matt Godbolt 的  [Compiler Explorer](https://rust.godbolt.org/) 对于了解 Rust 产生的汇编很有用。 [Rust Playground](https://play.rust-lang.org/) 也可以用来观察 Rust 代码执行时的情况，不过它的局限性更大。

本文档是针对嵌入式开发而写。这里不会讨论任何非嵌入式的 Rust 特性：见 [https://rust-embedded.github.io/book/intro/no-std.html](https://rust-embedded.github.io/book/intro/no-std.html) 。

Cpp 用户请注意。 Rust 和 Cpp 共享很多术语与概念（所有权、生命周期、析构器、多态性），但 Rust 对它们的实现往往具有明显不同的语义。在 Cpp 中的经验不应该被期望能准确地迁移。

### 什么是 Rust ？

Rust 是一门通用的编程语言，其重点是最大限度地提高程序员的控制能力和零运行时开销。同时消除了传统系统语言中容易“让人中招”的部分。Rust 有时也被称为“系统语言”。

在语法和设计哲学角度， Rust 很像 Cpp 和 ML（一种函数式语言）的结合体，尽管在语义上它与这两种语言有很大的不同。Rust 是第一种流行的，得到良好支持的语言，它提供了绝对的内存安全，而无需使用自动引用计数或垃圾回收器（GC）。Rust 也提供一种在源码中提供注解（生命周期参数）的技术，来避免访问失效的内存，它被称为借用检查器，不是在运行时检查。

> 译注：
>
> 这里说 绝对的内存安全，有点过了。
>
> 作者在脚注里虽然有说明：在Rust中，如果不使用关键字unsafe，使用后释放（use-after-frees）、双重释放（double frees）、取消引用（null dereferences）和数据竞争（data race）都是不可能的；这也适用于其他大多数传统上被认为是C语言中未定义行为的东西。︎
>
> 但是，没有绝对的安全，更没有绝对的内存安全。Rust 只是消除了比较常见的内存安全问题。

Rust 可以编译为本地代码，在内存和计算性方面可以和 C/Cpp 相媲美，并且可以与任何使用 C 调用惯例的东西无缝集成。它还静态地消除了一大类与安全漏洞相关的内存错误。Rust 的工具链是建立在 LLVM 之上的，所以，所有针对 LLVM 性能的工作都会有利于 Rust。

Rust 还包含了一种特殊的方言，叫做 “Unsafe Rust”。 在极少数需要进行底层操作的时候，它无法提供静态检查。本文档将围绕 Unsafe Rust 展开。

### Rust 工具链

一个完整的 Rust 工具链由几个主要部分组成：

- rustc， Rust 编译器。
- rustup，Rust 工具链的安装程序。
- Cargo， Rust 的构建系统（基于 rustc）。
- std 和 core 。

 Rust工具链的发布周期为六周，类似于Chrome的发布周期：每六周，一个发布（release）分支被切割为下一个测试版（beta），六周后成为下一个稳定版（Stable）。Rust的夜间版（Nightly）每天都从主干版（master）上切割下来；正是在夜间版上，非稳定（unstable）的功能才可以被启用。一些非稳定的特性（features）对嵌入式非常有用，所以嵌入式Rust项目使用夜间编译器的情况并不少见。

rustup用于管理Rust的安装。这主要是由于Rust的发布过于频繁，操作系统的包管理器无法跟上，而且项目可以锁定特定版本的Rust。当Rust工具链通过rustup安装时，rustc和cargo等组件会感知到它；`rustc +nightly-2020-03-22`能通过rustup来下载和执行3月22日构建的rustc nightly。项目目录中一个名为rust-toolchain的文件也可以达到同样的效果。

Cargo是一个Rust的构建系统/包管理器。它可以自动构建项目（即有Cargo.toml文件的目录）和它们的依赖项。Rust中的单个编译单元被称为 "crates"，它们要么是静态库（即与`.a`文件相当），要么是完全链接的本地二进制文件。这与C语言不同，在C语言中，每个`.c`文件都会生成一个单独的对象文件。 Rust也没有头文件，尽管它提供了一个模块系统来组织内部的代码，这将在后面讨论。Tock板是一个很好的例子，说明一个更复杂的货物文件是什么样子的：[https://github.com/tock/tock/blob/master/boards/opentitan/Cargo.toml](https://github.com/tock/tock/blob/master/boards/opentitan/Cargo.toml)

一些有用的Cargo子命令包括:

- `cargo check` 运行`rustc`的检查，但在它开始输出代码和优化之前就停止了。这对于开发过程中的错误检查很有用。
- `cargo build` 构建一个库或二进制文件，取决于crate类型。
- `cargo clippy` 运行 Rust linter, Clippy。
- `cargo doc --open` 构建 crate 文档，然后在浏览器中打开它。
- `cargo fmt` 运行 Rust formatter 。

此外，`RUSTFLAGS`环境变量的内容也会传递给`rustc`，作为注入标志的机制。

Rust标准库，像`libc`一样，在嵌入式环境中比较罕见。标准库由三个板块组成：`core`、`alloc`和`std`。`core`，有时被称为`libcore`，是所有的基本定义，不依赖于操作系统的支持。core中的任何东西都不能进行堆分配。`alloc`不需要操作系统的支持，但需要`mallo`c和`free`符号。std是`core+alloc`，以及操作系统API，如文件和线程支持。`#[no_std] ` 禁用了`std`和`alloc`，留下了`core`。在本文档中，我们将只使用`core`类型，尽管我们可以通过std命名空间来引用它们（它们是别名）。也就是说，我们可以引用`std::mem::drop`，尽管在`#[no_std]`代码中它必须被命名为`core::mem::drop`。

rustc有许多 flag。其中最突出的是:

- `--emit asm` 和 `--emit llvm-ir`，对检查编译器输出很有用。
  `--target`，设置交叉编译的目标。它的作用类似于Clang的`-target`、`-march`和`-mabi`标志。它接受一个定义了平台的目标定义（在很多情况下类似于LLVM的目标三要素）。例如，OpenTitan 软件使用`riscv32imc-unknown-none-elf`目标。使用一个不是宿主目标的目标（例如，x86_64-unknown-linux-musl）需要用`rustup component install rust-std-<target>`来安装相应的标准库构建。参见` rustc --print targets`。`--target`也被Cargo直接接受，这与大多数rustc的标志不同。
- `-C link-arg`，等同于Clang的`-T`。
- `-C opt-level`，相当于Clang的`-O`（我们主要使用`-C opt-level=z`来嵌入）。
- `-C lto`，相当于Clang的`-flto`。
- `-C force-frame-pointers`，相当于Clang的`-fno-omit-frame-pointer`。
- `-D warnings`大致等同于`-Werror`。

其他有趣的标志可以在`rustc -C`帮助下找到，在夜间，可以在`rustc -Z`帮助下找到。

### Part I:  用 Rust 重写 C 程序

在 深入研究 Rust 的具体特性前，我们将首先探讨 C 语言的概念如何映射到 Rust 中，以及 Unsafe Rust。 Unsafe Rust 是Rust 的方言，虽然没有 Safe Rust 的太多限制，但也是有一定安全保证。

#### 类型（Type）

Rust 和 C 对类型的处理方法大致相同，尽管 Rust 很少有隐式转换。在这一节中，我们将讨论如何将 C 语言类型转换为 Rust 类型。

##### 整数

Rust缺少C语言中的`int`、`long`、`unsigned`和其他具有实现定义大小的类型。相反，Rust的原生整数类型是精确大小的类型：`i8`、`i16`、`i32`、`i64`和`i128`分别是`8`、`16`、`32`、`64`和`128`位的有符号整数，而`u8`、`u16`、`u32`、`u64`和`u128`是其无符号变体。Rust还提供了`isize`和`usize`，它们对应于`intptr_t`和`uintptr_t11`。对齐要求与C语言完全相同。

Rust支持所有整数类型的常规二进制运算，不过在做算术时不能混合不同的类型，而且与C不同的是，没有整型提升(integral promotion)。Rust中的溢出与C14不同：它是由实现定义的，并且必须使程序崩溃或回绕（ wrap around）。Casting是通过`as`关键字完成的，其行为方式与C语言完全相同。`(uint8_t) x`被写成`u8`。整数类型之间从不进行隐式转换，甚至在有符号和无符号变体之间也是如此。

Rust有常见的整数字元：十进制为`123`，十六进制为`0xdead`，二进制为`0b1010`，八进制为`0o777`。下划线可以任意穿插在一个整数字头中，以分隔数字组。`0xdead_beef`，1_000_000。它们也可以用原生整数类型的名称作为后缀，以强制其类型。`0u8`, `0o777i16`, `12_34_usize`；否则它们将默认为任何类型推导（后面会详细介绍）所选择的类型，如果不受限制，则为`i32`。

Rust也有一个专门的`bool`类型。它不能与整数隐式转换，否则就是一个`u8`，保证具有`0x00`或`0x01`的值，以及各自的字面意义`false`和`true`。`bool`支持所有的位操作，是唯一与短路的`&&`和`||`兼容的类型。它也是唯一可以在`if`和`while`条件下使用的类型。

整数有一套广泛的内置位操作，以方法的形式暴露出来，如`x.count_zeros()`和`x.next_power_of_two()`。例子见[https://doc.rust-lang.org/std/primitive.u32.html](https://doc.rust-lang.org/std/primitive.u32.html)。

##### 结构体和元组

结构体声明和C 相似：

```rust
struct MyStruct {
    pub foo: i32,
    pub bar: u8,
}
```

Rust有每个字段的可见性修改器`pub`；我们将在后面对可见性进行更彻底的处理。

结构值可以使用类似于C语言的指定初始化语法来创建。

```rust
MyStruct { foo: -42, bar: 0xf, }
```

不过，Rust的结构体并不像C结构体那样布局，事实上，Rust并没有指定其结构体的布局。在Rust中可以使用`#[repr(C)]`属性来指定一个C结构。

```rust
#[repr(C)]
struct MyCStruct {
    a: u8,
    b: u32,
    c: u8,
}
```

这保证了按声明顺序排列字段，为对齐添加填充。`#[repr(Rust)] `是隐含的默认值。`#[repr(packed)] `类似于`__attribute__((packed))`，并且不会产生任何`padding`。整个结构的对齐方式可以使用`#[repr(align(N))]`强制为一个较大的值，类似于`_Alignas`。

可以使用与C语言相同的点语法来访问字段：`my_struct.foo, my_struct.bar = 5;`。

Rust还提供了 "类元组结构"，这是有编号而非命名字段的结构体。

```rust
struct MyTuple(pub u32, pub u8);
```

字段的访问采用类似的点状语法：`tuple.0, tuple.1`，并采用类似函数调用的语法构造:`MyTuple(1, 2)`。除了语法之外，它们与普通结构体没有区别。类元组结构上的字段可以省略，以声明一个零字节的结构。

```rust
struct MyEmpty
```

元组的匿名版本也是可用的: `(u32, u8)`。这些本质上是具有未命名字段的匿名结构。空元组类型，`（）`，被称为 "单元"，作为Rust的空类型（与空不同，`（）`只有一个值，也叫`（）`，是零大小）。Rust 还有一个类似于`void`的类型，即`！`，我们将在讨论函数的时候讨论这个类型。

如果一个结构的每个字段都可以用`==`来比较，编译器就可以为你的结构生成一个等价函数。

```rust
#[derive(PartialEq, Eq)]
struct MyStruct {
    a: u32,
    b: u8,
}
```

这使得在 MyStruct 的值上使用`==`成为可能，它会比较字段是否相等。同样也可以对`<`和`>=`这样的排序操作进行操作。`#[derive(PartialOrd, Ord)]`将定义比较函数，按字母顺序比较结构。

##### 枚举体与联合体

和 C 语言一样，Rust 也有枚举，用于描述具有固定数值的类型。

```rust
enum MyEnum {
    Banana, Apple, Pineapple,
}
```


但与C不同的是，MyEnum 是一个实数类型，而不仅仅是一个整数类型的别名。同样与C不同的是，枚举的变体不会被转储到全局命名空间，而是必须通过枚举类型来访问。`MyEnum::Banana`。请注意，与结构不同，枚举的变体是默认 pub 的。

虽然Rust用整数来表示枚举值（这些整数被称为判别值），但是它们的排列方式没有被指定。为了得到一个像C语言那样分配判别符的枚举，我们可以使用一个`repr`属性。

```rust
#[repr(C)]
enum MyCEnum {
    Banana = 0,
    Apple = 5,
    Pineapple = 7,
}
```

但与C不同的是，Rust 只保证明确写下的判别值。这样的枚举可以安全地转换为整数类型（比如`MyCEnum::Apple as u32`），反之则不然：编译器总是假设MyCEnum 的底层值是`0`、`5`或`7`，违反这个约束是未定义行为（UB）。如果我们想要求一个枚举是一个精确的整数宽度，我们可以使用`#[repr(T)]`，其中`T`是一个整数类型，如`u16`或`i8`。

Rust中的`Union`是一个相当新的特性，一般来说，在正常的代码中不会用到很多。它们的声明方式与结构很相似。

```rust
union MyUnion {
    pub foo: i32,
    pub bar: u8,
}
```

并像结构体一样创建。

```rust
MyUnion { bar: 0xa, }  // `bar` is the active variant.
```

对union变体的赋值与结构中的赋值相同，但读取变体需要使用 Unsafe 的Rust，因为编译器无法证明你没有读取未初始化的或无效的数据，所以你需要写上

```rust
unsafe { my_union.bar }  // I assert that bar is the active variant.
```

由于对析构器的关注，Union 对哪些类型可以作为变体也有限制。

由于 Union 在C语言中非常有用，但完全不安全，所以Rust提供了内置的标记(tagged) Union，可以通过枚举语法访问。

```rust
enum MyEnum {
    FooVariant { foo: i32 },
    BarVariant(u8),
}
```

tagged-union 的枚举变体使用与Rust 结构体相同的语法；枚举由一个足以区分所有变体的标签值（判别值）和一个编译器跟踪的变体联合组成。然而，有效使用这种枚举需要模式匹配，我们将在讨论模式时再次看到这些枚举。

就像对待结构体一样，`#[derive]`可以用在枚举上以定义比较运算符，其定义与结构体的情况类似。

##### 数组

Rust数组等同C语言的数组：内联存储编译时已知数量的值。C中的`T[N]`在Rust中被拼成`[T；N]`。数组是用`[a, b, c]`语法创建的，一个有很多相同值的副本的数组可以用`[0x55u8; 1024]`创建。一个多维数组可以被声明为一个数组的数组。`[[T；N]；M]`。

数组元素可以用`x[index]`来访问，就像在C语言中一样。但是请注意，Rust会自动在每个数组访问周围插入边界检查；边界检查失败会引发程序的恐慌（panic）。Unsafe  Rust可以用来欺骗边界检查，当它知道（对程序员来说，而不是Rustc！）没有必要执行边界检查，但当它是性能关键的时候，就可以省略它。

Rust数组是 "真正的 "类型，与C不同，它们可以通过值传递到函数中，并通过值从函数中返回。当传入函数时，它们也不会衰变为指针。

##### 指针

像其他所有的嵌入式语言一样，Rust 也有指针。这些指针通常被称为原始指针，以区别于无数的智能指针类型。Rust将`T` 和`const T` 拼成`mut T`和`const T`。与C不同的是，指针在被解除引用之前不需要与它们的指针类型对齐（与C一样，Rust假设所有指针的读/写都是对齐的）。

请注意，C的基于类型的严格别名在Rust中并不存在。正如我们稍后所了解的，Rust对引用有不同的别名规则，这些规则更加强大，编译器可以自动检查。

空指针可以使用`std::ptr::null()`和`std::ptr::null_mut()`函数创建。Rust指针不支持算术运算符；相反，一个方法填补了这个角色：用 `ptr.offset(4)` 代替 `ptr + 4`。指针之间是否相等是简单的判断地址是否相等。

指针可以用`*ptr`语法进行解引用，尽管这是 Unsafe 的Rust，需要说出`unsafe`。当指针被解引用时，它们必须像C语言一样，良好地对齐并指向有效的内存；不这样做就是不安全。与C语言不同，操作符`&x`产生一个引用，而不是一个指针。

指针的解除引用仍然受移动语义的约束，就像在普通的Rust中一样。指针上的`read()`和`write()`方法可以用来忽略这些规则。`read_unaligned()`和`write_unaligned()` 可以用来执行安全的无对齐访问，而`copy_to()`和`copy_nonoverlapping_to()`分别类似于`memmove()`和`memcpy()`。关于其他有用的指针方法，见[https://doc.rust-lang.org/std/primitive.pointer.html](https://doc.rust-lang.org/std/primitive.pointer.html)。挥发性操作也是使用指针方法进行的，这将在后面单独讨论。

由于所有这些操作都会解除对指针的定义，它们自然被限制在不安全的Rust中。

我们将在后面发现，除了原始指针之外，Rust还有许多其他的指针类型。一般来说，原始指针在 Rust 中只用于指向潜在的未初始化的内存，一般表示地址，而不是实际内存的引用。为此，我们使用引用，这将在后面讨论。

当我们遇到函数时，我们将触及到函数指针。

#### 项（item）

和C语言一样，Rust也有全局变量和函数。这些，连同上面的类型定义，在语法中被称为（语法层面）项（Item），以避免与C的声明/定义的区别相混淆。与C不同的是，Rust没有前向声明或声明顺序语义；所有的东西对整个文件都是可见的。语法项是通过专门的导入语句导入的，而不是通过文本包含；关于这一点，后面会有更多的介绍。

#### 常量和全局

Rust有专门的编译时常量语法，其作用与C语言中的#defined常量相同，其语法为：

```rust
const MY_CONSTANT: u32 = 0x42;
```

这里需要类型，右侧必须是一个常量表达式，大致是字面符号、数字运算符和常量函数的任何组合（稍后会详细介绍）。

常量在运行时不存在。它们可以被认为是固定的表达式，被复制+粘贴到它们被使用的地方，类似于C语言中的`#defines`和`enum`声明器。

Globals看起来像常量，但有一个关键字`static`。

```rust
static MY_GLOBAL: u8 = 0x00;
static mut MY_MUTABLE_GLOBAL: Foo = Foo::new();
```

`Globals`保证住在`.rodata`、`.data`或`.bss`中，这取决于它们的可变性和初始化。与常量不同，它们有唯一的地址，但是与常量一样，它们必须用常量表达式进行初始化。

可变的全局变量特别危险，因为它们可能是多核系统中数据竞争的来源。由于IRQ控制流的存在，可变全局变量也可能成为其他恶意行为的来源。因此，对可变全局的读写，或者创建对其的引用，都需要使用Unsafe的Rust。

#### 函数

在C和Rust中，函数是最重要的句法结构。Rust对函数的声明是这样的。

```rust
fn my_function(x: u32, y: *mut u32) -> bool {
    // Function body.
}
```

在`->`标记后面的返回类型，当它是`()`（"单元"，空元组）时可以省略，它作为Rust的无效类型的等价物。函数的调用采用通常的`foo(a, b, c)`语法。

一个函数的主体由一个语句列表组成，可能以一个表达式结束；该表达式是函数的返回值（不需要返回关键字）。如果缺少表达式，那么`（）`将被假定为返回类型。项目可以与语句混合在一起，这些语句在其当前范围内是局部的，但在所有范围内都是可见的。

Rust 函数可以被标记为`unsafe fn`，这意味着该函数不能被正常调用，而必须使用Unsafe 的Rust调用。`unsafe fn`的主体就像一个`unsafe block`；当我们详细讨论Unsafe 的Rust时，我们会更多地了解这个问题。

Rus t函数有一个未指定的调用约定。为了声明一个具有不同调用约定的函数，该函数被声明为`extern "ABI" fn foo()`，其中ABI是一个支持的ABI。`"C "`是我们真正关心的唯一一个，它将调用约定转换为系统的`C ABI`。默认的、隐式的调用约定是`extern "Rust "`。

将函数标记为extern并不能禁用名字改编（mangling）；这必须通过给函数添加`#[no_mangle]`属性来实现。然后，未改编的函数可以被C语言调用，允许Rust库有一个C语言接口。一个经过处理的`extern "C "`函数的主要用途是变成一个函数指针，传递给C。

函数指针类型看起来就像去掉所有变量名的函数：`fn(u32, *mut u32) -> bool`。函数指针不能为空，而且必须始终指向一个具有正确ABI的有效函数。函数指针可以通过隐含地将一个函数转换为一个函数来创建（没有`&`操作符）。函数指针也可以指定 `unsafe` 和 `extern` ：`unsafe extern "C" fn() -> u32`。

永不返回的函数有一个特殊的返回类型`!`，称为 "永无类型(Never type)"。这类似于C语言中的`noreturn`注解。然而，使用类型为`!`的表达式必然是死代码，因此，`!`将隐含地强制到所有类型（这简化了类型检查，并且完全没有问题，因为这都发生在可证明的死代码中）。

函数也可以被标记为常数。这使得该函数可用于常数计算，但大大限制了可用的操作。不过，在每个版本中，`const`函数的可用语法都在增加。大多数可以成为常数的标准库函数已经是常数了。

#### 宏

Rust和C语言一样，也有宏。Rust的宏比C的宏要强大得多，它在Rust的语法树上操作，而不是通过字符串替换。宏调用与函数调用的区别是在宏名称后面加上一个`!`。例如，`file!()`会扩展为一个带有文件名的字符串字面。要了解更多关于宏的信息，请参见[https://danielkeep.github.io/tlborm/book/index.html](https://danielkeep.github.io/tlborm/book/index.html)。

> 译注： Rust 的宏就目前的实现而言，并不是在 Rust 语法树上操作。但它也可以被认为是在语法树上操作。
>
> 因为 Rust 的语法树 API 本身并未稳定，目前 Rust 的宏都是建立在稳定的 词法分析 API 基础上，然后通过特定的语法解析（声明宏使用宏解析器，过程宏使用第三方syn/quote库）来完成操作。 

####  别名

Rust有type，它的工作原理和C语言中的`typedef`完全一样，其语法为：

```rust
type MyAlias = u32;
```

#### 表达式和语句

与C语言非常不同的是，Rust的语法中几乎没有语句：几乎所有的东西都是某种表达式，并且可以在表达式上下文中使用。粗略地说，语言中唯一的语句是创建一个绑定。

```rust
let x: u32 = foo();
```

`:`后面的类型是可选的，如果缺少，编译器将使用当前范围内的所有信息，包括`let`之前和之后的信息，来推断出一个类型。

表达式后面的分号只是简单地计算表达式的副作用，就像在其它语言中一样。有些表达式，比如`if`、`while`和`for`，不需要在后面加上分号。如果它们没有在表达式中使用，它们将被执行以产生副作用。

`let`绑定在默认情况下是不可变的，但是`let mut x = /* ... */; `将使其成为可变的。

和C语言一样，重新赋值是一个表达式，但和C语言不同的是，它求值为`()`而不是赋值。

和几乎所有其他语言一样，字面意义、运算符、函数调用、变量引用等等都是标准表达式，我们已经看到了Rust的拼写方式。让我们深入了解一下Rust的一些其他表达式。

**块表达式（block）**

Rust中的块就像是C语言中块的更好版本；在Rust中，每个块都是一个表达式。一个块以`{ }`为界，由一组语句和项组成，可能还有一个结束表达式，很像一个函数。然后，该块将求值最后的表达式。比如说：

```rust
let foo = {
    let bar = 5;
    bar ^ 2
};
```

块就像立即执行的局部函数，对于限制变量的范围很有用。

如果一个块没有以表达式结束（也就是说，里面的每个语句都以分号结束），它将隐式返回`()`，就像函数一样。在处理像`if`和匹配表达式这样的结构时，这种自动的`()`很重要，因为它需要将多个执行分支的类型统一为一个。

**条件表达式：if 和 match**

Rust的`if`表达式在语法上与C语言类似。完整的语法是:

```rust
if cond1 {
    // ...
} else if cond2 {
    // ...
} else {
    // ...
}
```

条件必须是求值为 `bool`的表达式。一个条件可以有零个或多个`else if`子句，`else`子句是可选的。正因为如此，Rust不需要（因此也没有）三元操作符。

```rust
let x = if c { a } else { b };
```

Rust中的`if`表达式需要使用大括号。

在Rust中，`if`表达式的值为最终被执行的块的值。因此，所有块都必须有相同的类型。例如，下面这个表达式不会被编译，因为一个错误的分号导致类型检查失败。

```rust
if cond() {
    my_int(4)   // Type is i32.
} else {
    my_int(7);  // Type is (), due to the ;
}
```

`i32`和 `()` 是不同的类型，所以编译器不能将它们统一为整个`if`的整体类型。

一般来说，在`if`子句中用分号结束所有最终表达式是个好主意，除非需要它的值。

和C语言一样，Rust也有一个类似`switch` 的结构，叫做匹配。你可以匹配整数。

```rust
let y = match x {
    0 => 0x00,       // Match 0.
    1..=10 => 0x0f,  // Match all integers from 1 to 10, inclusive.
    _ => 0xf0,       // Match anything, like a `default:` case.
};
```

像`if`表达式一样，匹配表达式产生一个值。语法`case val: stuff; break; `在Rust中大致转化为`val => stuff`。Rust称这些`case`子句为 "匹配分支 "。

与C语言不同的是，`match`语句没有贯穿（fallthrough），特别是只有一个分支被执行。然而，Rust允许一个匹配分支来匹配多个值。

```rust
match x {
    0 | 2 | 4 => /* ... */,
    _ => /* ... */,
}
```

Rust会静态地检查每一种可能的情况是否被覆盖。这在对一个枚举进行匹配时特别有用。

```rust
enum Color { Red, Green, Blue, }
let c: Color = /* ... */;
match c {
    Color::Red =>   /* ... */,
    Color::Green => /* ... */,
    Color::Blue =>  /* ... */,
}
```

不需要`_ => case`，就像C语言中的`default:`一样，因为Rust静态地知道所有的情况都被覆盖了（因为枚举不能接受没有被列为变体的值）。如果在枚举中不需要这种行为（因为将来会有更多的变体加入），`#[non_exhaustive]`属性可以应用于枚举定义，以要求一个默认的分支。

我们将在后面看到，模式匹配使得`match`比C的`switch`要强大得多。

**循环：`loop` 和`while`**

Rust有三种循环：`loop`、`while`和`for`。`for`不是C语言风格的`for`，所以我们将在后面讨论它。`while`是标准的C语言`while`循环，语法略有不同。

```rust
while loop_condition { /* Stuff. */ }
```

它可以作为一个表达式使用，但它的类型总是`()`；当它是一个块中的最后一个表达式时，这一点最值得注意。

`loop`是Rust特有的；它只是一个无限循环。

```rust
loop { /* Stuff. */ }
```

因为无限循环永远不会结束，所以循环表达式的类型（如果其中没有中断！）是`！`因为循环之后的任何代码都是死的。有了无条件的无限循环，Rust可以对循环进行更好的类型和寿命分析。在语言底层实现中，Rust的所有控制流都是以循环、匹配和中断的方式实现的。

#### **控制流**

Rust有`return`、`break`和`continue`，它们具有C语言中的通常含义。它们也是表达式，并且和`loop {}`一样，具有类型`！`因为所有跟在它们后面的代码都不会被执行（因为它们阻碍了控制流）。

`return x` 带着值x提前退出一个函数。` return`只是`return ()`的语法。`break`和`continue` 就是常规循环中的用法。

所有类型的循环都可以用标签进行注释（这是Rust唯一允许标签的地方）。

```rust
'a: loop {
    // ...
}
```

`break`和`continue`可以和这些标签一起使用（例如`break 'a`），这将会破坏或继续带有该标签的循环（而不是最邻近的循环）。虽然C语言缺少这个功能，但大多数没有goto的语言都有这个功能。

也可以从一个无限循环中以值 `break`，这将导致循环表达式计算为该值，而不是`!`。

```rust
let value = loop {
  let attempt = get();
  if successful(attempt) {
    break attempt;
  }
};
```

#### 与 C 语言对话

Rust的一大优势是与现有的C语言库实现了大部分的无缝对接。因为Rust基本上没有运行时间，与C类型相对应的Rust类型可以被简单地共享，Rust可以在几乎没有开销的情况下调用C函数。外部符号的名称可以使用`extern`块进行 "前向声明"，这使得Rust可以命名这些符号，并在之后与之链接。

```rust
extern "C" {
    fn malloc(bytes: usize) -> *mut u8;
    static mut errno: i32;
}
```

当指定的ABI是` "C "`时，它可以不写: `extern {}`是隐含的`extern "C" {}`。

确保这些符号的存在是链接器的责任。此外，还必须注意在边界上发送哪些类型。更多细节见[https://doc.rust-lang.org/reference/items/external-blocks.html](https://doc.rust-lang.org/reference/items/external-blocks.html)。

#### 其他类似的功能

##### Volatile

Rust没有`volatile`限定词。相反，可以使用指针上的`read_volatile()` 和`write_volatile()` 方法进行易失性读取，其行为与C语言中的易失性指针转指完全相同。

注意，这些方法在比架构的易失性加载和存储更宽的类型上工作，这将扩展成一系列的易失性访问，所以要小心。同样的注意事项也适用于C语言：`volatile uint64_t`在32位机器上会发出多个访问。

##### 内联汇编

Rust还不完全支持内联汇编。Clang的内联汇编语法在不稳定的宏`llvm_asm!()`后面可用，它最终会被Rust特有的语法取代，从而更好地与语言结合。 `global_asm!()`也是如此，但可在全局范围内使用，用于定义整个函数。裸函数可以用`#[naked]`来创建。参见[https://doc.rust-lang.org/1.8.0/book/inline-assembly.html](https://doc.rust-lang.org/1.8.0/book/inline-assembly.html)。

请注意，这种语法目前正处于重新设计和稳定的过程中。

##### 按位转换

Rust提供了一个类型系统的陷阱门，可以将任何类型按位转换位任何其他相同大小的类型。

```rust
let x = /* ... */;
let y = std::mem::transmute<A, B>(x);
```

这个陷阱门是非常危险的，只应该在强制转换不够用的情况下使用。

https://doc.rust-lang.org/std/mem/fn.transmute.html 有一个用途列表，其中许多实际上不需要转换。

##### 链接器技巧和其他属性

下面是与嵌入式编程有关的各种属性。其中许多属性会巧妙地影响链接器/优化器的行为，并且在很大程度上属于 "你可能不需要担心它 "的范畴。

- `#[link_section = ".my_section"] `是`__attribute__((section(".my_section"))`的简单拼写，它将在给定的`ELF`(或等价)部分粘贴一个符号。
- `#[used]`可以用来强制链接器保留一个符号  (这在C语言中通常是通过将符号标记为`volatile`来实现的)。通常对`__attribute__((used))`的注意事项，以及其他链接器的提示，在这里也适用。
- `#[inline]`类似于C的`inline`，只是一个提示；`#[inline(always)]`和`#[inline(never)]`将分别总是 或从不被inline。
- `#[cold]`也可以用来对那些不太可能被调用的函数进行最小化的内联。

### Part II : Rust 专属特性

前一部分介绍了可以直接将 C 代码翻译为 Rust 代码所需要的知识。但是，这样直接从 C 翻译的 Rust 代码，其安全性将和 C 代码一样，没有太多保证。这一节，我们将重点介绍 使 Rust （译注：是指 Safe Rust） 更加安全、更容易编写的特性。

#### 所有权（Ownership）

双重释放（double-free），或者一般来说，双重使用，是 C 语言中一大类潜在的 Bug，这些 Bug 一眼看上去并没有明显的错误。

```rust
// `handle` is a managed resource to a peripheral, that should be
// destroyed to signal to the hardware that the resource is not in use.
my_handle_t handle = new_handle(0x40000);
use_for_scheduling(handle);  // Does something with `handle` and destroys it.
// ... 200 lines of scheduler code later ...
use_for_scheduling(handle);  // Oops double free.
```

在 C 语言中，DF（double-free） 和 UAF（use-after-free）是崩溃和安全漏洞常见来源。让我们看看在 Rust 代码中尝试这样做会发生什么?

考虑一下用 Rust 编写等效代码：

```rust
let handle = new_handle(0x40000);
use_for_scheduling(handle);
// ...
use_for_scheduling(handle);
```

如果你试图编译这段代码，你会得到一个错误：

```rust
error[E0382]: use of moved value: `handle`
  --> src/main.rs:10:24
   |
7  |     let handle = new_handle(0x40000);
   |         ------ move occurs because `handle` has type `Handle`,
   |                which does not implement the `Copy` trait
8  |     use_for_scheduling(handle);
   |                        ------ value moved here
9  |     // ...
10 |     use_for_scheduling(handle);
   |                        ^^^^^^ value used here after move
```

UF 和 DF 的错误在 Safe Rust 中是不可能存在的。这类特殊错误（不直接涉及指针）是由移动语义来防止的。正如上面错误示例所示，变量标志着它已经被 "移出"：该变量现在是一个未初始化内存的空槽。编译器会静态地跟踪这一点，如果你试图再次移出，编译会失败。当前存储一个值的变量被称为它的 "所有者 "；所有者有权将所有权移交给另一个变量，但只能这样做一次。

该错误还指出，"Handle没有实现Copy 特质"。特质本身是以后的话题；现在这意味着Handle有移动语义（新类型的默认）。实现了复制的类型具有复制语义；这就是 C 语言中所有按值传递的类型的行为方式：在 C 语言中，按值传递的结构总是复制整个结构，而按引用传递的结构只是复制指向该结构的指针。这就是为什么在处理整数和原始指针时，移动并不相关：它们都是Copy类型。

请注意，您定义的任何结构和枚举都不是默认的复制类型，即使它们的所有字段都是。如果你希望一个字段都是Copy的结构体也是Copy的，你可以使用以下特殊语法。

```rust
#[derive(Clone, Copy)]
struct MyPodType {
  // ...
}
```

当然，复制/移动的区别是一种错误的说法：由于复制和移动语义而导致的重新赋值可以编译成相同的memcpy或寄存器移动代码。这种区分纯粹是为了静态分析。

#### 引用（References）和生命周期（Lifetimes）

另一类 UAF 使用涉及到栈销毁以后的栈变量。考虑下面的 C 代码：

```c
const int* alloc_int(void) {
  int x = 0;
  return &x;
}
```

这个函数显然是错误的，但是这样的错误，即一个指针超过它所指向的数据，在C语言中是很隐蔽的，因为它们很常见。

Rust的主要指针类型: 引用，使得这种情况不可能发生。引用就像原始的指针，只是它们总是对齐良好、非空，并且指向有效的内存；它们也比C语言的指针有更强的别名限制。让我们来探讨一下Rust是如何实现这最后一项保证的。

考虑一下下面这个Rust程序。

```rust
fn alloc_int() -> &i32 {
    let x = 0i32;
    &x
}
```

这个程序将无法编译，并出现一个隐秘的错误：`missing lifetime specifier.`。显然，我们漏掉了什么，但至少编译器没有让这个明显错误的程序通过。

生命周期（lifetime），用`'a`这样的符号表示（撇号通常读作 "tick"），标记源代码的一个区域。Rust中的每个引用都有一个生命周期，代表了一个引用指向有效内存的区域：这是由语法`&'a i32`在`'a`期间对`i32`的引用所指定。生命周期和类型一样，在运行时不存在；它们的存在只是为了让编译器进行借用检查，在这种情况下，编译器会确保引用只存在于各自的生命周期内。一个特殊的生命期，`static`代表整个程序。它是常量和全局变量的生命周期。

考虑一下下面的Rust代码:

```rust
let x: i32 = 42;
let y: &'a i32 = &x;  // Start of 'a.
use_reference(y);
use_value(x);  // End of 'a, because x has been moved.
use_reference(y);  // Error: use of y outside of 'a.
```

引用生命周期从引用被接受时开始，当生命周期超出范围或引用的值被移动时结束。试图在生命周期外使用引用是一个错误，因为它现在是一个悬空的指针。

Rust经常把引用称为借用：引用可以在有限的时间内（生命周期）从它的所有者那里借用一个值，但是必须在所有者把这个值让给其他人之前归还它。引用也有可能是借用的借用，或者是再借用：总是有可能创建一个具有较短生命周期但与另一个引用具有相同价值的引用。重新借用通常是由编译器隐式执行的，通常是在调用点周围，但也可以通过写`&*x`显式执行。

在大多数使用生命周期的地方都可以省略。

```rust
fn get_field(m: &MyStruct) -> &u32 {
  &m.field  // For references, unlike for raw pointers, . acts the same way -> does in C.
}
```

在这里，编译器假定返回类型的生命周期应该与`m`的生命周期相同。然而，我们可以明确地写出这一点。

```rust
fn get_field<'a>(m: &'a MyStruct) -> &'a u32 { /* ... */ }
```

`<'a>`语法在函数签名中引入了一个新的命名生命周期，这样我们就可以明确地告诉编译器 "这两个引用具有相同的生命周期"。当编译器无法做出任何假设时，这对于指定许多生命周期来说特别有用。

```rust
fn get_fields<'a, 'b>(m1: &'a MyStruct, m2: &'b MyStruct) -> (&'a u32, &'b u32) {
    (&m1.field, &m2.field)
}
```

现在我们可以尝试修复我们错误的栈返回函数。我们需要为这个函数引入一个新的生命周期，因为没有函数参数可以得到一个生命周期。

```rust
fn alloc_int<'a>() -> &'a i32 {
    let x = 0i32;
    &x
}
```

现在这给了我们一个直接的错误，表明借用检查可以防止错误的栈返回。

```rust
error[E0515]: cannot return reference to local variable `x`
 --> src/lib.rs:9:3
  |
9 |   &x
  |   ^^ returns a reference to data owned by the current function
```

这种`<'a>`语法也可以应用于结构体等项目。如果你要创建一个包含引用的类型，`<'a>`是必须的。

```rust
struct MyRef<'a> {
    meta: MyMetadata,
    ptr: &'a u32,  // Lifetime elision not allowed here.
}
```

Rust的引用有两种类型：共享和唯一。一个共享引用，`&T`，提供了对`T`类型值的不可改变的访问，并且可以自由复制：`&T`是Copy。唯一的引用，`&mut T`，提供了对`T`类型值的可变访问，但要遵守Rust的别名规则，这比C的严格别名规则要严格得多，而且不能被关闭。

对于一个给定的值，在同一时间只能有一个`&mut T`激活。这意味着在这个唯一引用的有效期内不能创建其他引用。然而，一个`&mut T`可以被重新借用，通常用于传递给一个函数。在再借用的有效期内，不能使用原来的引用。这意味着下面的代码可以正常工作。

```rust
fn do_mut(p: &mut Handle) { /* ... */ }

let handle: &mut Handle = /* ... */;
do_mut(handle);  // Reborrow of handle for the duration of do_mut.
// handle is valid again.
do_mut(handle);  // Reborrow again.
```

换句话说，Rust 没有一个安全的`int*`等价物；它只有`const int*` 和`int* restrict`的等价物......再加上强制去掉 const是即时未定义行为。Rust会假设没有可变的引用别名，以便进行别名分析。这意味着更多的优化机会，而不需要安全代码做任何事情。

最后，不用说，引用只对主（main）内存有用；Rust有权为（可能未使用的）引用产生虚假的load和store，所以MMIO应该完全通过原始指针进行。

[https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html ](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html )进一步阐述了各种生命周期规则。

#### 引用操作

Rust引用的行为更像标量值，而不像指针（除了借用检查）。因为静态地知道每个引用在任何时候都指向一个有效的、初始化了的`T`类型的值，所以在大多数时候显式地取消引用（尽管在必要时，它们可以被取消引用。`*x`是一个可以被赋值的`lvalue`）。)

Rust没有`->`操作符，但是对于`x: &T`来说，点操作符的行为就像`x`是一个`T`一样。例如，如果`field`是`T`的一个字段，`x.field`就是`field`的`lvalue`（在C语言中拼写为`x->field`）。这甚至适用于严重嵌套的引用：`&&&&`的点运算符将触发三次内存查找。这被称为 `auto-deref `行为。

引用是否相等取决于其指向值的是否相等：`x == y`，对于`x: &T`和`y: &T`，变成`*x == *y`。指针相等判断仍然可以用`std::ptr::eq(x, y)`。引用可以被强转为原始指针：`x as *const T`，并直接进行比较。

#### 方法

虽然Rust不是一种面向对象的语言，但它确实提供了一种在类型下对函数进行命名的机制：`impl`（代表实现）块。这也允许你使用Rust的可见性注解，使外部用户无法看到实现细节。

下面是一个带有方法的类型的例子。

```rust
pub struct Counter(u64);  // Non-public field!
impl Counter {
    /// Creates a new `Counter`.
    pub fn new() -> Self {
      Counter(0)
    }

    /// Private helper.
    fn add(&mut self, x: u64) {
        self.0 += x;
    }

    /// Get the current counter value.
    pub fn get(&self) -> u64 {
        self.0
    }

    /// Increment the counter and return the previous value.
    pub fn inc(&mut self) -> u64 {
        let prev = self.get();
        self.add(1);
        prev
    }

    /// Consumes the counter, returning its final value.
    pub fn consume(self) -> u64 {
        self.get()
    }
}
```

外部模块不能访问任何未标记为pub的东西，这使得我们可以对 Counter 执行一个不变性：它是不变的。让我们来解读一下这个语法。

impl 块中的函数被称为 "固有函数 (inherent functions) "或 "方法 (method)"，这取决于它们是否带一个`self`参数。固有函数不需要`self`参数，它的调用方式与`Counter::new() `类似。

`self`参数是一个名为 self 的参数（这是一个关键字），其类型涉及`Self` （另一个关键字，是内含块的类型别名），如`&Self `。`self`、`&self`、`mut self`和`&mut self`的语法 是语法糖，分别对应 `self: Self`、 `self: &Self`、`mut self: Self` 和`self: &mut Self`，分别代表了`self-by-value`、`self-by-reference`、`self-by-mut-value`和`self-by-mut-reference`。

因此，方法可以像这样被调用：`my_counter.inc()`。方法实际上只是普通的函数：你也可以像`Counter::inc(&mut my_counter)`这样调用。请注意，调用一个接收`&self`或`&mut self`的函数会触发接收类型的借用；如果在一个非引用值上调用`&self`函数，该值将被获取其地址，并被传递到方法中。

impl块，像其他项目一样，可以通过生命周期参数化。为了给一个有引用的结构添加方法，可以使用以下语法。

```rust
impl<'a> MyStruct<'a> { /* ... */ }
```

如果'a'在 impl 块内从未实际使用过，可以使用 lifetime 占位符 来写。

```rust
impl MyStruct<'_> { /* ... */ }
```

正如我们已经看到的，许多原始类型也有方法；这些方法被定义在标准库的特殊 impl 块中。

#### 切片（slice） 和 `for`  循环

引用也不允许进行指针运算，所以`&u32`不能被用来指向字的缓冲区。静态缓冲区可以作为数组来传递，比如`&[u32; 1024]`，但我们经常想传递一个运行时已知值的连续内存的指针。切片 是Rust对指针加长度的解决方案。

`T`的切片是`[T]`类型；这种类型最像C语言中的 "灵活数组成员"。

```rust
struct slice {
    size_t len;
    T values[];
}
```

那么，一个`slice*`将指向一个长度，后面是那么多的`T`；除了在指针后面，它不可能合理地存在。同样地，`[T]`就是Rust所说的动态大小的类型，它需要存在于一个引用后面：更常见的是看到`&[T]`和`&mut[T]`。

然而，Rust 仍然与C版本不同：`&[T]`是一个胖指针，有两个字宽。它本质上看起来像这样。

```rust
struct Slice {
    len: usize,
    values: *const T,
}
```

对一个切片的引用与数组引用类似：`&x[n]`提取对切片中第n个元素的引用（有边界检查），`x[n] = y`分配给它。切片的长度也可以用`len`方法提取：`x.len()`。

`str` 是一个类似切片的类型，保证包含`UTF-8`字符串数据。

可以使用 "范围索引操作 "从数组和其他切片创建切片：`&x[a..b]`。这需要数组或切片`x`，并创建一个从索引`a`到索引`b`（包括`a`，不包括`b`）的元素的切片，长度为`b-a`。`&x[a..]`是后缀，从`a`开始，`&x[...b]`是前缀，从`b`结束，`&x[...]`是整个切片，对于将数组转换为切片很有用。包容范围也是可用的，其语法是`a...=b`。

切片可以通过`for`循环进行迭代。

```rust
let slice: &[u32] = /* ... */;
for x in slice {
    // x is a reference to the nth element in slice.
}
```

如果需要一个索引，可以直接在一个范围内迭代。

```rust
for i in 0..slice.len() {
    let x = &slice[i];
    // ...
}
```

这可以与`_`模式相结合，简单地重复一个操作n次。

```rust
for _ in 0...n {
    // ...
}
```

与借用有关的切片的一个重要注意事项是唯一引用。如果我们有一个对切片的唯一引用，就不可能同时对多个元素采取唯一引用。

```rust
let slice: &mut [u32] = /* ... */;
let x = &mut slice[0];
let y = &mut slice[1];  // Error: slice is already borrowed.
```

`split_at_mut() ` 方法可以用来将一个唯一的`slice`引用分割成两个不重叠的唯一`slice`引用。

```rust
let slice: &mut [u32] = /* ... */;
let (slice1, slice2) = slice.split_at_mut(1);
let x = &mut slice1[0];  // slice[0]
let y = &mut slice2[0];  // slice[1]
```

通常情况下，可以通过结构化的方式来避免这种情况的发生，但这种逃逸的方式是为了在必要的时候存在。切片也可以用`as_ptr()`和`len()`函数分解成其指针和长度部分，然后用`std::slice::from_raw_parts()`重新组装起来。这种操作是不安全的，但对于跨越系统调用或IPC边界的C和Rust，或者Rust和Rust之间的桥接非常有用。

更多的切片操作可以在[https://doc.rust-lang.org/std/slice/index.html](https://doc.rust-lang.org/std/slice/index.html) 和 [https://doc.rust-lang.org/std/primitive.slice.html](https://doc.rust-lang.org/std/primitive.slice.html)找到。

#### 字符串字面量

Rust的字符串字头很像C的字符串字面量 : `"abcd..."`。任意的ASCII范围的字节可以用`\xNN`插入，并支持大多数常见的转义序列。然而，所有的Rust字符串都是`UTF-8`编码的字节切片：`&str`是围绕`&[u8]`的一个封装类型，保证里面的字节是有效的`UTF-8`。所有字符串字面的类型是`&'static str`。

Rust字符串字面意义中可以包含任意的换行，这可以被转义。

```rust
// Equivalent to "foo\n  bar".
let s = "foo
  bar";
// Equivalent to "foo  bar".
let s = "foo\
  bar";
```

原始字符串不能使用转义序列，并由任意的、匹配数量的`#`符号分隔。

```rust
let s = r"...";
let s = r#" ..."#;
let s = r#####"..."#####;
```

Rust也有`'z'` 形式的字符字面，不过它们的类型是`char`，一个32位的Unicode代码点。要获得一个`u8`类型的ASCII字节，可以使用`b'z'`。

#### 析构 与 RAII 

析构器是一些特殊的函数，当一个值变得不可触及时（即，最初声明它的`let`不能再被命名，并且对它的最后一次引用已经过期），它将执行清理逻辑。在析构器运行后，如果它是一个结构体或枚举，那么该值的每个字段也被销毁（或 "丢弃"）。

析构器是用一种特殊的 impl 块来声明的（我们将在后面看到更多这样的内容）。

```rust
impl Drop for MyType {
    fn drop(&mut self) {
        // Dtor code.
    }
}
```

如果几个值同时超出了范围，它们会按照声明的相反顺序被丢弃。

Drop方法不能被手动调用；然而，标准库函数`std::mem::drop()`可以用来放弃一个值的所有权并立即销毁它。Union 和具有复制语义的类型不能有析构器。

析构器可以实现资源获取即初始化（RAII）的惯用法。一个持有某种临时资源的类型，像一个外设的句柄，可以有一个析构器来自动释放该资源。一旦句柄超出范围，该资源就会被清理掉。

RAII的典型例子是动态内存管理：你用`malloc`分配内存，把返回的指针藏在一个结构中，然后该结构的析构器对该指针调用`free`。由于在调用`free`时，该结构已经超出了范围，所以UAF是不可能的。由于Rust的移动语义，这个结构不能被复制，所以析构器不能被调用两次。因此，双重释放也是不可能的 。

在某些情况下，调用一个析构器可能是不可取的（例如，在某些不安全的Rust操作中）。标准库提供了特殊的函数`std::mem::forget()`，它消耗一个值而不调用其析构器。`std::mem::ManuallyDrop<T>`类型是一个智能指针 ，它持有一个`T`，同时抑制其析构器。由于这个原因，不存在期望一个析构器实际运行的问题。

`std::mem::needs_drop()`这个函数可以用来发现一个类型是否需要被drop；即使它没有drop方法，它也可能递归地有一个字段可以drop。 `std::ptr::drop_in_place()` 可以用来在一个原始指针后面的值中运行析构器，而在技术上不放弃对它的访问。

#### 模式匹配

引用不能为空，但事实证明，空值有时是有用的。`Option<T>`是一个标准的库类型，代表一个 "可能没有的`T` " 。它被实现为一个枚举。

```rust
enum Option<T> {
    Some(T),
    None,
}
```

`<T>`类似于我们之前看到的lifetime语法；它意味着`Option<T>`是一个通用类型；我们很快就会深入研究这些。

如果我们有一个`Option<T>`类型的值（或者，任何其他的枚举，真的），我们可以使用模式匹配来编写以该值的判别式为条件的代码，这可以通过匹配表达式访问。

```rust
let x: Option<u32> = /* ... */;
let y = match x {
    Some(val) => val,  // If `x` is a `Some`, bind the value inside to `val`.
    None => 42,  // If `x` is a `None`, do this instead.
};
```

模式匹配给我们的关键是能够安全地检查枚举内的联合体：标签(tag)检查是由编译器强制执行的。

模式就像表达式，形成一种小型语言。如果说表达式是通过组合现有的值来建立一个值，那么模式则是相反的：它们通过解构值来建立值。特别是，应用于表达式的模式会执行以下操作。

- 检查表达式的值是否真的与该模式匹配。(注意，类型检查并不包括在内；模式不能表达式的类型）。
- 可以选择将表达式的值绑定到一个名字上。
- 可以选择递归到子模式中。

下面是几个模式的例子。请记住每个模式的匹配、绑定和递归属性。一般来说，模式看起来像它们匹配的表达式的值。

- `_`，一个下划线模式。匹配总是成功的，但是会扔掉匹配的值。这就是相当于默认情况下的_：case。_
- `foo`，一个标识符模式。这个模式与`_`完全一样，但它将匹配的值与它的名字绑定。这就是上面`Some(val)`中的`val`。这也可以作为一个默认的案例，希望对匹配到的值做一些事情。绑定可以通过写`Some(mut val)`而变得可变。
- 任何数字字面量，用于一个字面量模式。这种匹配将匹配的值与字面值进行比较，并且不匹配任何东西。这些也可以是包容性的范围： `5..=1686`.
- `(pat1, pat2, /* etc */）`，一个元组模式。这种匹配对元组类型进行操作，并且总是成功的：它提取元组的各个元素，并将它们应用于模式的子模式。特别是，`()`模式匹配单位值`()`。

```rust
let x: (u32, u32) = /* ... */;
match x {
    (5, u) => /* ... */,  // Check that first element is five,
    // bind the second element to `u`.
    (u, _) => /* ... */,  // Bind the first element to `u`,
    // discard the second element.
}

let y: (u32, (u32, u32)) = /* ... */;
match y {
    // All patterns can nest arbitrarily, like expressions.
    (42, (u, _)) =>  /* ... */,
    // `..` can be used to match either a head or a tail of tuple.
    (.., u) => /* ... */,
    (u, ..) => /* ... */,
    (..) =>    /* ... */,  // Synonymous with _.
}
```

结构模式类似于元组模式。对于类似元组的结构，它们的语法完全相同，但以结构的名称开始。`MyTuple(a, b, _)`。普通结构的语法要有趣得多。

```rust
struct MyStruct { a: i32, b: u32 }
match my_struct {
    MyStruct { a, b } => /* ... */,  // Bind the fields `a` and `b` to
    // names `a` and `b`, respectively.
    MyStruct { a: foo, b: _ } => /* ... */,  // Bind the field `a` to the name
    // `foo`, and discard the field `b`.
    MyStruct { a: -5, .. } => /* ... */  // Check that `a` is -5, and ignore
    // other fields.
}
```

枚举模式可能是最重要的一种模式，也是我们在上面的`Option`的匹配语句中看到的。它们与结构模式非常相似，只是它们不总是成功的，而是检查枚举判别符是否是模式中指定的那个。

```rust
enum MyEnum { A, B{u32), C { a: i32, b: i32 }, }
match my_enum {
    MyEnum::A =>    /* ... */,  // Match for variant `A`.
    MyEnum::B(7) => /* ... */,  // Match for variant `B`, with 7 as the value inside.
    MyEnum::B(x) => /* ... */,  // Match for variant `B`, binding the value inside to
    // `x`.

    MyEnum::C { a: 7, .. } => /* ... */,  // Match for variant `C`, with 7 as the
    // value in `a` and all other fields ignored.

    MyEnum::C { b, .. } => /* ... */,  // Match for variant `C`, binding b to b.
}
```

对模式语法的完整处理可以在[https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html)。

匹配表达式将针对一个值对每个模式进行计算，直到有一个匹配，依次进行；编译器将对无法到达的模式发出警告。编译器还将确保每个值都能与其中一个匹配分支相匹配，这是因为每个情况都被覆盖了（例如，每个枚举变体都存在），或者存在一个不可辩驳（irrefutable ）的模式（即，一个匹配所有值的模式）。`_`, `foo`, `(a, _)`, 和`MyStruct { a, .. }`都是不可反驳的模式的例子。

如果被匹配的值是某种类型的引用，绑定的名字也将是引用。

比如说:

```rust
match &my_struct {
    MyStruct { a, .. } => {
        // Here, `a` is a `&i32`, which is a reference to the `a` field in my_struct.
    },
}
```

这个特性有时被称为匹配的人机工程学，因为在它被添加之前，必须在引用的匹配中添加明确的解除引用和特殊的`ref`模式限定符。

此外，匹配语句在上面讨论的模式语法的基础上支持两个额外的特性。

- 多重匹配分支可以允许一个匹配分支匹配多个模式中的一个：`a | b | c => /* ... */`,. 如果有任何模式匹配，就执行该分支。
- 匹配守卫为你提供了一种快捷方法，用于在某些表达式上限制一个分支：`Some(foo) if foo.has_condition() => /* ... */`,.

另外，标准库提供了 `matches!()` 宏，作为以下常见匹配表达式的简写。

```rust
match expr {
  <some_complex_match_arm> => true,
  _ => false,
}
// ... can be replaced with ...
matches!(expr, some_complex_match_arm)
```

`matches! `也支持多重匹配和匹配守卫。

不可反驳的模式可以与普通变量声明一起使用。语法`let x = /* ... */; `实际上使用了一个模式：`x`是一个模式。当我们写`let mut x = /* ... */;`时，我们使用的是一个`mut x`模式。其他不可辩驳的模式也可以用在这里。

```rust
// Destructure a tuple, rather than using clunky `.0` and `.1` field names.
let (a, b) = /* ... */;

// Destructure a struct, to access its fields directly.
let Foo { foo, bar, baz } = /* ... */;

// Syntactically valid but not allowed: `42` is not an irrefutable pattern.
let 42 = /* ... */;
Special variants of if and while exist to take advantage of patterns, too:

if let Some(x) = my_option {
    // If the pattern succeeds, the body will be executed, and `x` will be bound
    // to the value inside the Option.
    do_thing(x);
} else {
    // Else block is optional; `x` is undefined here.
    // do_thing(x);  // Error.
}

while let Some(x) = some_func() {
    // Loop terminates once the pattern match fails. Again, `x` is bound
    // to the value inside the Option.
}
```

与普通的`let`语句不同，`if let`和`while let`表达式是为了与可反驳模式一起使用。

一般来说，几乎所有绑定值的地方都可以是一个不可反驳的模式，例如函数参数和`for`循环变量。

```rust
fn get_first((x, _): (u32, u32)) -> u32 { x }

for (k, v) in my_key_values {
    // ...
}
```

#### 特质（trait）

特质是Rust的核心代码重复使用抽象。Rust的特质就像其他语言中的接口：一个类型必须实现的方法列表。然而，特质本身并不是类型。

标准库中一个非常简单的`trait`是`Clone`。

```rust
trait Clone {
    fn clone(&self) -> Self;
}
```

一个满足`Clone`接口的类型（用Rust的说法是 "实现Clone"）有一个具有给定签名的`Clone`方法，它返回一个`Self`的副本。为了实现一个特质，你可以使用一个略微有趣的`impl` 语法。

```rust
impl Clone for MyType {
    fn clone(&self) -> Self { /* implementation */ }
}
```

这给了我们一个一致的方式来拼写 "我想要这个值的一个副本"。标准库为一些类似的操作提供了特质，比如`Default`，用于提供默认值；`PartialEq`和`Eq`，用于平等；`PartialOrd`和`Ord`，用于排序；以及`Hash`，用于非加密散列。

上述 特质 的特殊之处在于，它们对一个结构或枚举有微不足道的实现，假设该结构或枚举的所有字段都实现了它。在 "所有权 "一节中描述的`#[derive()]`语法可以与任何这些特质一起使用，为一个类型自动实现它们。普通旧数据（POD）类型看起来像这样的情况并不少见。

```rust
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct MyPod {
    pub a: u8,
    pub b: u8,
    // The following line wouldn't compile, because `derive(Eq)` requires
    // all fields to be `Eq`.
    // c: NonEq,
}
```

特质也可以提供用其他方法实现的内置方法，以提供一个默认的实现（如果对某一特定类型有更有效的实现，则可以重写）。完整的`Clone` trait实际上是这样的。

```rust
pub trait Clone {
    fn clone(&self) -> Self;
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone();
    }
}
```

实现者不需要提供`clone_from`，但如果默认的实现不够好的话，允许他们这样做。

特质和实现它们的类型可以被定义在不同的模块中，只要实现模块定义了特质或类型。这意味着特质方法并不是类型的一部分，而是特质和类型的一部分。因此，为了在一个特定类型上调用特质方法，该特质也必须在范围内。当无歧义时，trait函数可以作为`foo.trait_fn()`、`Foo::trait_fn(foo)`或`Trait::trait_fn(foo)`来调用。然而，由于名字有时可能有歧义，所以有一个完全不模糊的语法 :`<Foo as Trait>::trait_fn(foo)`。最后一种语法在通用语境中也是很有用的，或者可以精确到被引用的确切函数。

特质 也是操作符重载的载体：这些`trait`可以在标准库的`std::ops`模块中找到。

#### Trait 对象

特质可以通过一种叫做特质对象的机制用于动态调度（也被称为虚拟多态性）。给定一个`trait Trait`和一个实现它的类型T，我们可以将一个引用`&T` as 强转成一个动态trait对象：`&dyn Trait`。比如说:

```rust
trait Id {
    fn get_id(&self) -> usize;
}
impl Id for Device {
  // ...
}

let device: Device = /* ... */;
let dyn_id = &device as &dyn Id;  // Create a vtable.
let id = dyn_id.get_id();  // Indirect procedure call.
```

`dyn Trait `是一个动态大小的类型，很像切片，只能存在于一个指针后面。引用`&dyn Trait`看起来像这样。

```rust
struct TraitObject {
    value: *mut (),
    vtable: *mut Vtable,
}

struct Vtable {
    size: usize,
    align: usize,
    dtor: fn(&mut T),
    // Other trait methods.
}
```



因此，对`get_id`的动态函数调用将编译成如下内容。

```rust
let device: Device = /* ... */;
let dyn_id = &device as IdTraitObject;
let id = (dyn_id.vtable.get_id)(dyn_id.value);
```

#### Unsafe Trait

我们可以通过编写`unsafe trait MyTrait { /* ... */ }；`与普通trait的唯一区别是，它需要实现不安全的 impl。Unsafe 的 trait 通常在其方法之外执行某种额外的约束；事实上，Unsafe 的 trait 经常根本就没有方法。例如，标准库的特质`Sync`是由所有同步访问的类型实现的。因为这个特质所断言的不变性超出了编译器可以检查的范围，所以它是一个不安全的特质。

特质方法可以单独标记为` unsafe` 。这样做通常是为了表明，不仅在实现该特质时需要小心，而且调用该函数也需要小心（并发出不安全的声音）。这与将特质标记为` unsafe`是分开的，没有必要将一个特质标记为` unsafe`，因为它有` unsafe`的方法。

#### Auto Trait

Auto Trait 是一种编译器机制，用于自动实现某些特征；在标准库的源代码中，它显示为 auto trait Foo {}。(尽管这种语法在普通库中是不可用的)。如果一个结构或枚举类型的所有字段也实现了该特性，那么自动特性就会自动实现，并用于向特性系统暴露跨行属性。例如，Send和Sync是自动特质；其他一些标记性特质98也是自动特质。

自动特征总是你并不希望选择的标记。它们就像derive()特性的反面，你需要选择加入，因为它们对你的类型的API产生了有意义的影响，能够控制这种影响是很重要的。

#### 泛型编程

泛型编程是指编写可以被编译为许多类型的源代码。泛型是Rust的核心功能之一，它可以实现多态静态调度。

函数可以通过引入类型参数来实现泛型，使用的语法类似于显式生命期。

```rust
fn identity<T>(x: T) -> T {
    x
}
```

这个函数接受一个任何类型的值并立即返回。然后它可以像这样被调用：`identity::<i32>(42)`。使用一个填写了所有类型参数的通用函数会导致它被实例化（或单态化），从而导致为它生成代码。这个过程本质上包括用具体的值替换`T`的每一次出现。

每个不同的实例化在运行时是一个单独的函数，有一个单独的地址，尽管对于产生相同代码的函数，如`identity::<i32>`和`identity::<u32>`，链接器可能会重复它们。过分热衷于使用泛型代码会导致二进制的膨胀。

大多数时候，`::<>`位（被一些参考资料称为 "涡轮鱼"）是不必要的，因为Rust类型演绎可以推断出它：`let x: u64 = identity(42); `会推断出`T = u64`。然而，在没有必要的情况下，包括它也是有用的，可以帮助提高可读性。

类型也可以是泛型的，比如之前的`Option<T>`类型。

```rust
struct MyWrapper<T> {
    foo: usize,
    bar: T,
}
```

具体的类型`MyWrapper<i32>`包括将定义中所有出现的`T`替换成`i32`，否则我们可以将其作为普通类型使用。

```rust
fn get_foo(mw: MyWrapper<i32>) -> usize {
    mw.foo
}
```

注意，`MyWrapper`本身并不是一个类型。

注意不同的泛型实例是不同的类型，有不同的布局和大小，一般来说，它们之间不能相互转换。

不出所料，我们可以将泛型函数与泛型类型结合起来。在这种情况下，我们并不真的需要知道`T = i32`，所以我们把它剔除。

```rust
fn get_foo<T>(mw: MyWrapper<T>) -> usize {
    mw.foo
}
```

我们也可以建立一个泛型函数来提取泛型字段。

```rust
fn get_bar<T>(mw: MyWrapper<T>) -> T {
    mw.bar
}
```

就像对待生命周期一样，impl 块在使用前需要引入类型参数。

```rust
impl<T> MyWrapper<T> {
    // ...
}
```

#### 泛型限定

然而，仅仅是泛型就有一个限制：函数在其泛型形式下只进行一次类型和借用检查，而不是每次实例化；这意味着泛型代码不能仅仅调用`T`的固有方法，并期望查找成功。例如，这段代码不会被编译。

```rust
fn generic_add<T>(x: T, y: T) -> T {
    x + y
}
```

错误看起来像这样。

```rust
error[E0369]: cannot add `T` to `T`
 --> src/lib.rs:2:6
  |
2 |     x+y
  |     -^- T
  |     |
  |     T
  |
  = note: T might need a bound for std::ops::Add
```

编译器很有帮助地建议我们需要某种 "限定"。泛型限定是特质真正发挥作用的地方。

`Add` 是一个标准库特质，看起来像下面这样。

```rust
trait Add<Rhs> {
    type Output;
    fn add(self, other: Rhs) -> Self::Output;
}
```

这个特质不仅是泛型的，而且它还定义了一个相关的类型，允许实现者选择加法运算的返回类型。因此，对于任何类型的`T`和`U`，如果`T`实现了`Add<U>`，我们可以把它们加在一起；操作的返回类型是`<T as Add<U>>::Output`。

因此，我们的`generic_add`函数应该被改写成:

```rust
fn generic_add<T: Add<T>>(x: T, y: T) -> T::Output {
    x + y
}
```

`T: Add<T>`部分是一个泛型约束，断言这个函数只有在所选的`T`实现了`Add<T>`时才能被编译。

如果我们想确保返回一个` T`，我们可以改变约束，要求 `Output `是` T`。

```rust
fn generic_add<T>(x: T, y: T) -> T
  where T: Add<T, Output=T>
{
    // ...
}
```

注意，这个限定被包含在`where`子句中，在返回类型之后。这与把它放在尖括号中是一样的，但对于复杂的限定，建议不要让它们挡住路。括号内的限定和`where`子句适用于所有其他可以有泛型限定的项，比如`traits`、`impls`、`structs`和`enums`。

限定泛型可以被用来模拟各种其他行为。例如，`From` 和` Into` 特质代表无损转换，所以一个想要任何可以转换为` MyType` 的值的函数可能看起来像:

```rust
fn foo<T: Into<MyType>>(x: T) {
    // ...
}
```

然后你可以在 `MyType` 上为所有可以转换为`MyType` 的 `T `实现` From<T>`。当 `U `实现 `From<T>` 时，标准库中的泛型 impl 会使 `T` 实现 `Into<U>`。在调用点，这看起来像一个重载函数。

限定的泛型也可以被用来传递常量。想象一下，我们定义了一个特质，比如

```rust
trait DriverId {
    const VALUE: u8;
}
```

然后，这个特质可以由各种零大小的类型来实现，这些类型的存在只是为了作为类型参数传入。

```rust
struct GpioDriverId;
impl DriverId for GpioDriverId {
    const VALUE: u8 = 0x4a;
}
```

然后，需要接受一个驱动的常数id的函数可以这样编写和调用。

```rust
fn get_device_addr<Id: DriverId>() -> usize {
  // use Id::VALUE somehow ...
}
// ...
get_device_addr::<GpioDriverId>()
```

类型也可以通过生命周期来绑定。绑定`T:'a`表示`T`中的每个引用都比`'a`长；每当一个通用的`&'a T`被传递时，这种绑定将被隐式插入。限定可以被组合: `T: Clone + Default` 和 `T: Clone + 'a `都是有效的限定。最后，生命周期可以被其他生命期所约束：`'a: 'b`意味着生命期`'a`比`'b`长。

#### 幻影类型

以下是 Rust 中的一个错误：

```rust
error[E0392]: parameter `T` is never used
 --> src/lib.rs:2:12
  |
2 | struct Foo<T>;
  |            ^ unused parameter
  |
  = help: consider removing `T`, referring to it in a field,
    or using a marker such as `std::marker::PhantomData`
```

Rust 要求使用所有的生命周期和类型参数，因为生成调用析构器的代码需要知道某个特定类型是否拥有一个`T`。这并不总是理想的，因为有时在你的类型中暴露一个`T`是很有用的，即使你不拥有它；我们可以使用编译器的建议来解决这个问题:`PhantomData`。关于如何使用它的更多信息，请参阅[类型文档](https://doc.rust-lang.org/std/marker/struct.PhantomData.html)或相关的[Rustonomicon条目](https://doc.rust-lang.org/nomicon/phantom-data.html)。

#### 智能指针

在Rust中，"智能指针"是任何实现了`std::ops::Deref`的类型，即解引用操作符 。Deref的定义是这样的。

```rust
trait Deref {
    type Target;
    fn deref(&self) -> &Self::Target;
}
```

实现Deref的类型也可以实现可变体。

```rust
trait DerefMut: Deref {
    fn deref_mut(&mut self) -> &mut Self::Target;
}
```

实现`Deref`特质给了一个类型T两个特征。

它可以被解除引用。`*x成为*(x.deref())`或`*(x.deref_mut())`的语法糖，这取决于产生的左值是否被赋值。
它获得了自动的deref：如果`x.foo`不是`T`的一个字段或方法，那么它将扩展为`x.deref().foo`或`x.deref_mut().foo`，同样取决于用途。
此外，`deref`和`deref_mut`的调用是通过做一个明确的重借：`&*x`和`&mut *x`。

智能指针的一个例子是`ManuallyDrop<T>`。尽管这个类型直接包含了一个`T`（而不是通过引用），它仍然被称为 "智能指针"，因为它可以被解引用以获得里面的`T`，而且`T`的方法可以被调用。我们将在后面看到，`RefCell<T>`类型也会产生智能指针。限制对一个值的访问的泛型包装类型是智能指针，这种情况并不少见。

注意，因为`Target`是一个关联类型，所以一个类型只能转指到一个其他类型。

虽然与智能指针不太相关，但` Index `和` IndexMut `特质类似于 `Deref `和` DerefMut `特质，它可以实现` x[foo] `下标语法。`Index`看起来像这样。

```rust
trait Index<Idx> {
    type Output;
    fn index(&self, index: Idx) -> &Self::Output;
}
```

一个索引操作，很像一个解引用的操作，从`x[idx]`扩展到`*(x.index(idx))`。注意，索引操作可以被重载，这也是通过特质重载的一个有用的例子。例如，`<[u8] as Index<usize>>::Output`是`u8`，而`<[u8] as Index<Range>>::Output`是`[u8]`。用单个索引进行索引产生一个字节，而用一个范围进行索引产生另一个切片。

####  闭包

闭包（有时在其他语言中被称为 "lambda表达式"）是捕获其环境的某些部分的函数字面，它可以被传递给其他函数以定制行为。

Closures不是单纯的函数指针，因为这种捕获的状态。在C语言中，与此最相近的是一个函数，它需要一个函数指针和一些 "上下文"。例如，Linux的`pthread_create()`函数需要一个`void* (*start_routine)(void*)`参数和一个`void* arg`参数，`arg`代表`start_routine`需要执行的状态。以类似的方式，Rust闭包需要额外的状态来执行，只是`arg`成为`start_routine`值的一部分。不仅如此，Rust还会为`arg`合成一个定制的上下文结构，而通常情况下，程序员需要手动完成这一工作。Rust让这个习语更容易使用，因此也更常见。

正如我们将看到的，Rust有许多不同的闭包ABI，其中一些与`pthread_create`的做法非常相似；在某些情况下，函数指针及其上下文甚至可以被内联。

在Rust中，创建闭包的语法是`|arg1, arg2| expr`。它们可以很简单，比如`|(k, _)| k`（使用模式匹配来提取一个元组的第一个元素），也可以很复杂，使用一个块表达式来创建一个较长的函数。`|foo| { /* ... */ } `参数的类型可以选择性地指定为`|foo: Foo| { /* ... */ }`，而返回类型则是`|foo| -> Bar { /* ... */ }`，尽管在几乎所有的情况下，类型推导都可以正确地计算出它们。一个不需要参数的闭包可以写成`|| /* ... */`.

闭包通过引用来捕获它们的环境；该引用的可变性是通过使用来推导的。比如说。

```rust
let x = /* ... */;
let y = /* ... */;
let f = |arg| {
    x.do_thing(arg);  // Takes &self, so this implicitly produces a shared reference.
    y.do_mut_thing(arg);  // Takes &mut self, so it takes a unique reference instead.
};
// Note: f holds a unique borrow of y.
let z = &mut y;  // Error!
```

上面，`f`通过共享引用捕获`x`，通过唯一引用捕获`y`。实际的闭包值`f`是一个包含捕获的合成结构体。

```rust
struct MyClosure<'a> {
    x: &'a X,
    y: &'a mut Y,
}
```

调用一个闭包，比如`f()`，会调用一个合成函数，该函数将`MyClosure`作为其第一个参数。我们可以通过移动到闭包中来代替捕获；这可以通过移动`|arg| { /* ... */ }` 语法来实现。如果将其应用于`f`，`MyClosure`将变成:

```rust
struct MyClosure<'a> {
    x: X,
    y: Y,
}
```

Rust 并不完全支持通过移动捕获和通过引用捕获的混合方式，但通过移动捕获引用的方式可以将它们混合起来。

```rust
let x = /* ... */;
let y = /* ... */;
let x_ref = &x;
let f = move |arg| {
    x_ref.do_thing(arg);  // Capture x_ref by move, aka capture x by shared ref.
    y.do_mut_thing(arg);  // Capture y by move.
};
```

对于Copy类型来说，通过移动捕获和通过引用捕获之间的区别是不重要的。

为了在不同的闭包类型上实现多态性，我们使用了特殊的`Fn`、`FnMut`和`FnOnce`特性。这些代表了可以通过共享引用、唯一引用或移动来调用的函数。只捕获共享引用的闭包实现了所有三种；通过唯一引用捕获的闭包只实现了后两种，而通过移动捕获的闭包只实现了最后一种 。函数指针、函数项 和不捕获的闭包也实现了这三者，并且都可以被转换为函数指针。

这些特质使用类似于函数指针的特殊语法 。例如，`Fn(i32) -> i32`表示接受一个`i32`参数并返回另一个`i32`。如果所有的捕捉器都实现了 `Copy` 和 `Clone`，那么闭包也实现了 `Copy` 和 `Clone`。

#### 作为函数参数的闭包

编写接受闭包参数的函数大致有两种方式：通过动态分发，或通过静态分发，这两种方式分别对性能和大小有影响。

`Fn`和`FnMut`闭包可以使用`trait`对象来接受。

```rust
fn my_do_thing(func: &dyn Fn(i32) -> i32) -> i32 {
    func(MY_CONST)
}
```

这与 C 语言的方法完全相同：合成函数住在 trait 对象的 vtable 中，而捕获则在实际 trait 对象指针本身的后面。换句话说。

```rust
struct DynClosure {
    vtable: *mut Vtable,
    captures: *mut Captures,
}
```

当然，vtable 的调用会带来性能上的损失，但避免了泛型实例化的代码大小开销。

使用泛型允许传递实现`Fn`、`FnMut`或`FnOnce`的闭包，方法是为每个函数类型专门设计调用函数。

```rust
fn my_do_thing<F: Fn(i32) -> i32>(func: F) -> i32 {
    func(MY_CONST)
}
```

这将转化为对合成闭包函数的直接调用，没有任何开销，但会为传入的每个闭包重复该函数，如果在大型函数上使用，会导致很大的尺寸冲击。

我们可以使用简洁的方法来声明这种类型的函数，这样可以避免声明一个类型参数。

```rust
fn my_do_thing(func: impl Fn(i32) -> i32) -> i32 { /* ... */ }
```

impl Trait 可以用在函数参数位置，表示 "这个参数可以是任何实现了 Trait的类型"，这实际上是声明了一个匿名的 泛型参数。请注意，从技术上讲，Trait 可以是任何涉及至少一个trait的泛型限定：` impl Clone + Default `和` impl Clone + 'a `都是有效的。

#### 作为函数返回的闭包

闭包类型通常是不可命名的。返回闭包的典型方式是将 impl Trait 放在返回位置。

```rust
fn new_fn() -> impl Fn(i32) -> i32 {
    |x| x * x
}
```

返回位置 impl Trait 意味着 "此函数返回某个实现 Trait 的未指定类型"。函数的调用者不能使用实际的类型，只能使用通过Trait提供的函数。 impl Trait也可以用来隐藏实现细节，当一个返回值只存在于实现某些trait时。

返回位置 impl Trait 有一个主要的注意事项：它不能返回实现该特征的多个类型。例如，下面的代码是一个类型错误。

```rust
fn new_fn(flag: bool) -> impl Fn(i32) -> i32 {
    if flag {
        |_| 0
    } else {
        |x| x * x
    }
}
```

由此产生的编译器错误看起来像这样。

```rust
  = note: expected type `[closure@src/lib.rs:3:5: 3:10]`
          found closure `[closure@src/lib.rs:5:5: 5:14]`
  = note: no two closures, even if identical, have the same type
  = help: consider boxing your closure and/or using it as a trait object
```

在非嵌入式环境中，解决方案（正如编译器所建议的）是在堆上分配闭包，并使用特质对象。然而，在嵌入式上下文中，分配是有限的，所以这个解决方案是不可用的。

如果没有闭包捕获，返回一个函数指针可能是一个可接受的解决方案。

```rust
fn new_fn(flag: bool) -> fn(i32) -> i32 {
    if flag {
        |_| 0
    } else {
        |x| x * x
    }
}
```

####  作为结构体字段的闭包

如果不能轻松分配，使用闭包作为结构字段是相当有限的。两个选择是：要么让闭包类型的特质泛化（这需要通过使用该结构的所有东西来传播），要么要求闭包不捕获，而是使用函数指针。

```rust
struct MyStruct<F>
where F: Fn(i32) -> i32 {
    val: usize,
    func: F,
}
// Vs.
struct MyStruct {
    val: usize,
    func: fn(i32) -> i32,
}
```

一般来说，函数指针是最简单的，而且没有捕获的要求也不是特别苛刻。泛型变体往往对临时类型更有用，比如组合器。

临时的结构体也可以尝试使用特质对象，但生命周期要求会有相当大的限制。

```rust
struct MyStruct<'a> {
    val: usize,
    func: &'a dyn Fn(i32) -> i32,
}
```

#### Rust 中的错误处理： Option 和 Result

正如我们在上面看到的，`Option` 是一个让我们指定一个 "潜在的未初始化的 "值的类型。虽然使用匹配表达式来处理`Option`是很常见的，但它也有一些方便的函数来缩短常见的代码序列。` is_none()`可以用来检查一个`Option`是否为空；`map`可以用来转换一个`Option`里面的值。

```rust
opt.map(|t| t + 1)  // Increments the value inside, if there is one.
```

`unwrap_or()`可以用来提取里面的值，有一个默认值。

```rust
opt.unwrap_or(42)  // Get the value inside, or the value 42 if the former is unavailable.
```

`Option`的文档描述了许多其他潜在的用途和对`Option`的操作：[https://doc.rust-lang.org/std/option](https://doc.rust-lang.org/std/option)。这个类型的文档本身有一个为 `Option` 定义的所有便利函数的完整列表：[https://doc.rust-lang.org/std/option/enum.Option.html](https://doc.rust-lang.org/std/option/enum.Option.html)。

`Option`的一个关键应用是安全的可归零引用。`Option<T>`。Rust语言保证`Option<&T>`在ABI层与一个可归零的指针相同 ，所以它可以安全地被传递到C代码中。这种优化也适用于至少包含一个引用的结构：`is_none()`位将被压缩到该结构的一个引用字段中。其他一些类型也有资格进行内存布局优化，比如`NonZeroI32` 。

`Result<T, E>`与`Option<T>`类似，但它没有 "空 "的状态，而是有 "错误 "的状态。

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Result<T, E>` 表示对`T`类型的值完成了计算，但可能出了问题。`E`通常是某种枚举类型，因为`Rust`并没有为所有情况提供单一的错误类型。

```rust
enum MyError {
    DeadlineExceeded,
    BufferExhausted(usize),
    OtherError(ErrorCode),
}
```

使用常见的错误枚举为你的代码定义自定义的 `Result `类型是相当普遍的。

```rust
type Result<T> = std::result::Result<T, MyError>;
```

在某种程度上，`Option<T>`只是一个`Result<T, ()>`，其中错误类型只是微不足道的单元元组。Rust 提供了一些函数用于它们之间的转换。

```rust
opt.ok_or(error)  // Converts Option<T> into Result<T, E>, using the provided error if
// the Option is empty.
res.ok()  // Discards the error portion and returns an Option<T>.
res.err()  // Discards the ok portion and returns an Option<E>.
```

为其可能失败的副作用而执行的计算，如写操作，倾向于返回` Result<(), E>`。这与C语言不同，当函数返回`void`时，对可能失败的函数的处理是不一致的，因为`void`不是一个真实的类型。

有时，由于某些特性的接口，有必要为一个不能失败的操作产生一个结果。目前的做法是使用`Result<T, std::convert::Infallible>`类型，它可以被匹配到如下内容。

```rust
let res: Result<T, Infallible> = /* ... */;
match res {
    Ok(t) => { /* ... */ },
    Err(x) => match x {},
}
```

Result 支持一种特殊的提前返回语法。当在一个返回`Result<T, E>`的函数中，你有一个`Result<U, E>`类型的值，表达式`res? `将解开 Result，如果它存在，将得到里面的 "ok "值，如果不存在，则立即返回错误。也就是说，`res?` 被翻译成:

```rust
match res {
    Ok(x) => x,
    Err(e) => return Err(e),
}
```

这个问号操作符可以与方法链在一起，这样就可以写出在第一个错误时提前返回的直接代码，而无需显式控制流。

```rust
let x = my_thing.foo()?.bar()?.baz()?;
```

更多的结果操作见 [https://doc.rust-lang.org/std/result/index.html](https://doc.rust-lang.org/std/result/index.html)。

#### 回顾`for`  ： 迭代器

迭代器是任何实现了迭代器特质的类型，它看起来像这样。

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

一个迭代器产生一个`Option<Item>`值的序列；`next()`方法允许一个迭代器推进一些内部状态，并产生序列中的下一个值。

例如，一个非常简单的迭代器只是依次产生每个非负的整数值。

```rust
struct Counter { state: u64 }
impl Iterator for Counter {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let current = self.state;
        self.state += 1;
        Some(current)
    }
}
```

这个迭代器将永远产生数值：它总是返回`Some`。一个最终产生`None`，然后永远返回`None`的迭代器，被称为 "fused " 。有些迭代器在返回`None`后可能又开始返回`Some`，但大多数Rust结构将所有迭代器都视为fused的。

一个相关的特质是`IntoIter`特质。

```rust
trait IntoIter {
    type Iter: Iterator;
    fn into_iter(self) -> Self::Iter;
}
```

一个`IntoIter`类型可以被转换成一个迭代器。这个类型被用来驱动我们看到的用于迭代切片的`for`循环。

```rust
for pattern in expr {
    // ...
}
// is syntactic sugar for
let iter = expr.into_iter();
while let Some(pattern) = iter.next() {
    // ...
}
```

在切片的例子中，`&'a [T]`实现了`IntoIter`，它产生了一个迭代器，依次产生切片的每个元素；`Range<i32>`类型（也就是语法`0...32`产生的）也实现了`IntoIter`。

所有这些机制允许用户建立自己的迭代器，用于自己类型的`for`循环，或者使用现有的迭代器和迭代器组合器。迭代器特性定义了几十个提供的方法，这些方法可以用来建立更复杂的迭代器。下面是几个特别有用的组合器的例子。

- `iter.chain(iter2)`。将两个具有相同 Item 类型的迭代器链在一起。当 iter 产生 None 时，第二个迭代器开始。
- `iter.peekable()`。将迭代器转换为具有`.peek()`函数的迭代器，该函数返回对序列中下一个值的引用（但不前进）。
- `iter.enumerate()`。将 Item 类型从`T`变为`（usize, T）`，跟踪序列中的当前索引和值。
- `iter.step_by(n)`。改变迭代器以返回每`n`个元素。
- `iter.take(n)`. 缩短迭代器的长度，在 fuse 前返回`n`个元素。
- `iter.map(|x| /* ... */) `。Lazy 地对每个元素应用闭包。
- `iter.filter(|x| /* ... */)`。对每个元素应用一个谓词；如果谓词返回错误，则被`next()`跳过。

一些其他的特质(trait)可以增强迭代器的属性，从而实现进一步的方法。`ExactSizeIterator`迭代器产生一个已知的、固定数量的值；`DoubleEndedIterators`可以从序列的前部和后部提取元素。虽然上面的许多操作在下一个方面有朴素的实现，但当有更有效的算法时，标准库的迭代器会覆盖它们。一般来说，迭代器可以产生非常高效的代码，类似于`while`循环所发出的代码，但在使用特别复杂的组合器链时，应该注意。

参见[https://doc.rust-lang.org/std/iter/trait.Iterator.html](https://doc.rust-lang.org/std/iter/trait.Iterator.html) 和 [https://doc.rust-lang.org/std/iter](https://doc.rust-lang.org/std/iter) 以了解可用操作的全部细节。

#### 模块 和 Crate 布局

每个Rust crate（从编译器的角度来看）都有一个唯一的、单一标识符的名字。这个名字被用来命名一个 crate 的符号 。core 和 std是 crate。

每个crate 都以`lib.rs`或`main.rs`文件为根，这取决于它是一个库还是一个二进制文件。这个文件可以声明模块，这些模块是crate的子命名空间。

```rust
// Declares a public module named `devices`. Its definition is found in
// either `devices.rs` or `devices/mod.rs`, relative to the current file.
pub mod devices;

// Declares a private module named `tests`. Its definition is found
// within the curly braces.
mod tests {
  // ...
}

// Declares a private module named `generated`. Its definition is found
// in the given path.
#[path = "relative/path/to/file.rs"]
mod generated;
```

模块可以任意嵌套：一个模块可以声明更多的模块。

模块中的符号可以通过路径引用：`std::mem::drop`指的是`crate std::mem`模块中的符号`drop`。`crate::devices::gpio:Gpio`指的是当前crate的`devices::gpio`模块中的符号`Gpio`。

`use`项可以用来在当前范围内创建符号别名。

```rust
// Pull in std::mem::drop, aliased to `drop`.
use std::mem::drop;

// Pull in the module std::mem, so we can now write `mem::drop` for `std::mem::drop`.
use std::mem;

// Pull in both size_of and drop:
use std::mem::{size_of, drop};

// Pull in all symbols in `std::mem`, including `drop`. Should typically be best
// avoided, for readability.
use std::mem::*;

// Pull in all symbols from the parent module:
use super::*;

// Pull in a symbol from a submodule (synonymous with using the full
// path starting with `crate`).
use self::devices::Gpio;

// Pull in a name, but rename it.
use std::io::Result as IoResult;

// Pull in a trait to enable its methods, but without pulling its name
// into scope.
use std::io::Write as _;
```

请注意，这是受可见性限制的。默认情况下，所有符号都是 "私有 "的，只在当前模块和它的子模块中可见。这可以明确地拼成`pub(self)`。一个符号可以用`pub(super)`限制在父模块和子模块中，也可以用`pub(crate)`限制在当前的`crate`中。最后，一个符号可以用`pub(in that::path)`限制在一个特定的路径上。`pub`简单地说就是 "完全是公开的"。

几乎所有的项都可以用可见性标记，除了` impl`。用可见性标记一个模块会限制其中所有项目的可见性。一个`pub(crate)`模块中的`pub`符号就是`pub(crate)`。 `use`语句也可以用可见性标记：这将导致导入的符号成为模块的一部分。例如，`std`中充满了`pub use core::Symbol;`的实例，以使`core`符号能够通过`std`被导入。

甚至`use`项也可以被标记为可见性。

```rust
// mod my_mod
pub use std::mem::size_of;
```

这意味着其他模块现在可以通过`my_mod::size_of`访问符号`size_of`，有效地重新导出了该符号。许多基本的核心类型也是这样通过`std crate`访问的。

Rust 没有头文件，也没有声明顺序的限制；一个crate 内的模块可以自由地形成循环的依赖关系，因为它们不是编译的单位，只是命名空间。Rust crate的命名空间是封闭的：在一个crate被完全编译后，没有其他符号可以被添加到其中。

#### 内部可变性

内部可变性是绕过Rust的别名规则的一个借用检查逃生舱。

通常情况下，Rust要求你在改变一个值之前静态地证明你对它有唯一的访问权。`UnsafeCell<T>`是一种特殊的、被编译器所认可的 类型，它包含一个单一的`T`，并且有一个方法`fn get(&self) -> *mut T`。当你在运行时可以保证对`UnsafeCell`的共享引用实际上是唯一的，由`get()`返回的原始指针可以被转换为唯一引用。这使得安全地突变代码成为可能，在运行时，已知代码是唯一的。当然，直接使用`UnsafeCell`是非常不安全的，它的存在是为了形成其他抽象的基础。

有两种安全暴露内部可变性的常见策略：`Cell` 方式 和`RefCell`方式。

Cell 方式根本就没有创建一个唯一的引用：相反，它在任何时候都持有一个有效的`T`，并提供一个交换原语来取出`T`并留下另一个。这样一来，就不需要执行别名规则了，因为没有引用实际指向那个`T`。

RefCell 方式则在运行时进行基本的借用检查。除了持有一个`T`之外，RefCell 还持有一个未完成的共享引用的数量的计数器（或者一个未完成的唯一引用的哨位值）。`try_borrow()`和`try_borrow_mut()`方法动态地检查这种借用是否有效（分别是没有未完成的唯一引用，或者根本没有未完成的引用），并返回一个`Result`来表示成功或失败。在成功的情况下，返回值是一个包裹着引用的智能指针，其析构器将减少原始 RefCell 中的共享/唯一引用计数。换句话说，RefCell 就像一个单线程的读写`mutex`，没有原子性的代价。

其他的抽象可以建立在`UnsafeCell`之上，用其他的策略来维持别名不变性，但它们最终会类似于Cell或RefCell中的一个。

内部可变性也是常量和静态的主要区别之一。

```rust
static S: MyCell<u32> = MyCell::new(0);
const C: MyCell<u32> = MyCell::new(0);

S.set(1);
S.get();  // value = 1, because `set` modified the memory location.
C.set(1);
C.get()  // value = 0, because `set` modified an inlined copy.
```



注意，`S`后面的内存改变了，所以必须在`.data`或`.bss`部分分配。这说明了UnsafeCell的另一个特性：它导致本来被声明为不可变的数据被分配为可变的。

更多细节请参见[https://doc.rust-lang.org/std/cell/index.html](https://doc.rust-lang.org/std/cell/index.html)；就像所有与别名相关的主题一样，它需要仔细关注细节，本节还远远没有穷尽。

#### Unsafe Rust

Unsafe Rust是Rust的一种方言，由关键词`unsafe`来表示：unsafe block，unsafel 函数，unsafe trait。重要的是，所有这些行为都需要说出关键词`unsafe`，这样就可以在代码审查中很容易地发现它们。在unsafe 块中的代码向读者表明，程序员已经检查了微妙的安全保证，而编译器自己是无法做到的。

Unsafe 的Rust从根本上说是通过 "关闭 "编译器通常执行的某些检查来实现的，只要在 `usnafe { /* ... */ }`中，Unsafe Rust可以做的事情是Safe Rust不能做的。

- 调用 Unsafe 的函数。
- 对原始指针的解引用。
- 通过一个可变静态来改变全局状态。
- 读取Union 字段。
- 调用asm!宏。

此外，Unsafe的 impl 可以实现 unsafe trait，但不需要在 unsafe 块内。

典型的参考文献是[Rustonomicon](https://doc.rust-lang.org/stable/nomicon/) ，这是一份非规范性文件，描述了Unafe Rust的常见用途。它是嵌入式编程的必读文件（主要是前半部分）。它包含了正确和不正确使用Unsafe Rust的详细例子，以及关于何时调用Unsafe Rust 的指导。

在本文中，提到了Unsafe Rust，主要是围绕调用unsafe 函数和引用原始指针，这大概是Unsafe Rust能做的所有普通Rust不能做的。有了这些能力就有了责任。Unsafe 的Rust对未定义行为并不安全，它可以让机器处于正常安全的Rust所允许的行为会触发未定义行为的状态。一般来说，有几条经验法则是有用的。

- 每个`unsafe fn`都应该在文档中声明，它假定调用者会坚持哪些不变性，以及它将使机器处于什么状态。例如，`<[T]>::get_unchecked(n)`忽略了索引操作的边界检查，而由调用者来维护它。
- 每当 unsafe 的代码调用到一个unsafe 的函数时，它必须确保在安全的代码中观察不到违反的不变性，这些不变性可能会触发未定义行为。例如，如果我们有一个类型保持着`len>0`的不变性，而我们在 unsafe 块中暂时将其设置为`0`，那么在对该类型调用任何安全方法之前，必须将其恢复为`>0`。
- Unsafe 的代码应该保持在绝对最小的范围内，并且用安全的接口来包装，通过静态类型系统保证或运行时检查来断言不变量。每一行 unsafe 的代码都是浪费了Rust保证的工程成本的地方。

换句话说，Safe Rust能够自由地假设Rust对别名、所有权和值的表示的保证在任何时候都是成立的。这种假设是普遍存在的：不仅编译器使用它来积极优化代码的速度和大小，而且其他库代码，如包装类型的析构器，也这样假设。Unsafe Rust 负责维护这一核心保证。在某种程度上，Unsafe Rust 负责保护 Safe Rust。

