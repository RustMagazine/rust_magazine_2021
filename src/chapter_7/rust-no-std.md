# Rust no-std 工程实践

作者：苏胤榕

> 改写std的库为支持no_std的库及写出一个支持std和no_std库的经验谈
> github repo: https://github.com/DaviRain-Su/rust-no-std-source

## 简介

首先介绍std和no_std的区别，然后介绍使用no_std库的方式，由于支持no_std的特性有两种不同的方式，因此使用no_std库也有两种方式。其次，验证一个库是否支持no_std特性的验证方式，如何改写一个std的库为同时支持std和no_std的特性方法。具体的如何写一个支持std和no_std的库。一些在std和no_std下都可以使用的primitive的仓库和相关的资源和文章。

## 目录

- std和no_std的区别
- Rust中使用no_std的两种方式
- 验证一个库是否支持no_std特性的验证方式
- 具体的写一个支持std和no_std的库
- 一些no_std和std可以使用primitive类型的仓库和相关资源的文章

## std和no_std的区别

> ## 核心库
> Rust语言的语法由核心库和标准库共同提供。 
> 其中Rust核心库是标准库的基础。核心库中定义的是Rust语言的核心，不依赖于操作系统和网络等相关的库，甚至不知道堆分配，也不提供并发和I/O
>
> 可以通过在模块顶部引入#![no_std]来使用核心库。核心库和标准库的功能有一些重复，包括如下部分：
>
> - 基础的trait，如Copy、Debug、Display、Option等。
> - 基本原始类型，如bool、char、i8/u8、i16/u16、i32/u32、i64/u64、isize/usize、f32/f64、str、array、slice、tuple、pointer等。
> - 常用功能型数据类型，满足常见的功能性需求，如String、Vec、HashMap、Rc、Arc、Box等。
> - 常用的宏定义，如println！、assert！、panic！、vec！等。 做嵌入式应用开发的时候，核心库是必需的。
> ## 标准库
> Rust标准库提供应用程序开发所需要的基础和跨平台支持。标准库包含的内容大概如下：
> - 与核心库一样的基本trait、原始数据类型、功能型数据类型和常用宏等，以及与核心库几乎完全一致的API。
> - 并发、I/O和运行时。例如线程模块、用于消息传递的通道类型、Sync trait等并发模块，文件、TCP、UDP、管道、套接字等常见I/O。
>- 平台抽象。os模块提供了许多与操作环境交互的基本功能，包括程序参数、环境变量和目录导航；路径模块封装了处理文件路径的平台特定规则。
>- 底层操作接口，比如 std：：mem、std：：ptr、std：：intrinsics 等，操作内存、指针、调用编译器固有函数。
>- 可选和错误处理类型Option和Result，以及各种迭代器等。

还有一些解释，#![no_std] 是一个crate level 级别的属性，表示core crate将链接到core crate而不是std crate。

下面是std crate和core crate的解释，其实这里也就解释了标准库与和核心库之间的区别。当让也内在的包括了std与no_std之间的区别。

首先是，std crate 是Rust 的标准库。它包含的功能假定程序将在操作系统上运行，而不是直接在裸系统上运行。std还假定操作系统是一个通用的操作系统，就像人们在服务器和台式机上看到的那样。出于这个原因，std为通常在这类操作系统中发现的功能提供了一个标准的API: 线程、文件、套接字、文件系统、进程等等。

然后是，core crate是std crate的一个子集，对程序运行的系统不做任何假设。因此它提供了基于语言的API，如浮点，字符串和切片，以及暴露处理器特性的API，如原子操作和SIMD指令。然而，它缺乏涉及堆内存分配和I/O的任何API。

对于一个应用程序来说，std所做的不仅仅是提供一种访问操作系统抽象的方式，std还负责涉及堆栈溢出保护，处理命令行参数，以及在程序的主函数被调用之前生成主线程。一个#![no_std]应用程序缺乏所有这些标准的运行时，所以它必须初始化自己的运行时，如果需要的话。

由于这些特性，#![no_std]应用程序可以是第一个或者唯一在系统上运行的代码。

## Rust中no_std的一些使用方法

主要具体介绍第二种方式的使用no_std

具体如何使用，参见写一个no_std的库的第二种使用方式。

也可参考，实例:[serde no-std的使用规范](https://serde.rs/no-std.html)

## 验证一个库是否支持no_std的验证方式

`cargo check --target wasm32-unknown-unknown`

但是wasm环境不一定就是no_std，或者别的编译目标也可以，也就是裸露的编译目标环境不带有任何系统的环境。

参考文档: [使用Rust编写操作系统（一）：独立式可执行程序](https://zhuanlan.zhihu.com/p/53064186)

## 具体的写一个no_std的库

### 创建一个no_std库的第一种方式（使用#![no_std])

使用#![no_std]的话，默认的就是这个库是在no_std环境下的，然而又因为no_std下的库 一般来说都是核心库，而核心库又是标准库的子集，所以声明#![no_std]写出来的库，也可以在 std（标准库环境）下使用。

1. [创建一个仓库](https://github.com/DaviRain-Su/rust-no-std-source/commit/cd90f28855cfe794c235976bb58c1c5ecb8c7fa9)
2. [使用#![no_std]将这个仓库中的函数能支持在no_std和std下使用](https://github.com/DaviRain-Su/rust-no-std-source/commit/d3c05920865a44ab7cbaf82a72f21c7b6b8beeb0)
3. [开始添加一个函数编译报错commit 1](https://github.com/DaviRain-Su/rust-no-std-source/commit/8bcd0b909ee116d3dc9c6464c2548e1c008d672e)
4. [修复错误commit 2](https://github.com/DaviRain-Su/rust-no-std-source/commit/ae94f9cf147b7ce37632cb4e9c36e20c5135b3ad)

### 创建no_std库的第二种方式（使用#![cfg_attr(not(features = "std"), no_std)] )

1. [创建一个仓库](https://github.com/DaviRain-Su/rust-no-std-source/commit/8cfd063be536028d9f8cfe1c7c04f16765825f8c)
2. [使用#![cfg_attr(not(feature = "std"), no_std)]](https://github.com/DaviRain-Su/rust-no-std-source/commit/aa09b0d2e2807d788564aea5fa4fc8cbfc760043)
3. [添加的一些函数和测试](https://github.com/DaviRain-Su/rust-no-std-source/commit/aa09b0d2e2807d788564aea5fa4fc8cbfc760043)

## 使一些不能在no_std环境下运行的仓库也能在no_std下支持

首先，要验证这个库能不能支持no_std的环境（见，验证一个库是否支持no_std的验证方式）。

找出这个库依赖的库支持no_std的方式，如果使用的是#![no_std] 那么这个库本身就是可以在std和no_std下同时的运行。

如果使用的是#![cfg_attr(not(features = "std"), no_std)],就需要打开default-features = false,进行配置。

最后可能需要做一些标准库的替换，使其能在no_std和std同时编译成功，一些可以使用的类型库有sp-std(这个库仅仅封装了一部分的类型，例如有些类型是没有的，string,File, IO)当然,IO,File，这些标准库在核心库当中是没有的。还有rust本身的alloc, core这些都是属于核心库的。也是在no_std环境下支持的。

具体的使用案例:

[相关的Pr,使ics23支持no_std](https://github.com/confio/ics23/pull/41)

[有些代码也在no_std写测试很难。因为这里做了编译选择处理](https://github.com/confio/ics23/pull/41/commits/dac5c3d0fc368e0b92c4a4804b6787bd1c3fb168)

## 一些no_std和std可以使用的primitive 类型的仓库

- [sp-std](https://github.com/paritytech/substrate/tree/master/primitives/std)
- [rust Alloc crate](https://doc.rust-lang.org/alloc/index.html)
- [rust Core crate](https://doc.rust-lang.org/core/index.html)

## 引用及资源

- [Rust 编程之道核心库和标准库的介绍](https://weread.qq.com/web/reader/0303203071848774030b9d6k9bf32f301f9bf31c7ff0a60)
- [Rust embedded book](https://docs.rust-embedded.org/book/intro/no-std.html)
- [扩展no_std crate 的最佳实践](https://users.rust-lang.org/t/best-practice-of-extending-a-no-std-crate/12281/5)
- [Rust API guidelines](https://github.com/rust-lang/api-guidelines)
- [Rust API guidelines Nameing](https://rust-lang.github.io/api-guidelines/naming.html#c-feature)
- [serde no_std的使用规范](https://serde.rs/no-std.html)
- [awesome-embedded-rust#no-std-crates](https://github.com/rust-embedded/awesome-embedded-rust#no-std-crates)
- [no standard library](https://crates.io/categories/no-std)
- [serde使用的第二种方式](https://github.com/serde-rs/serde/blob/master/serde/src/lib.rs#L113-L193) 
- [Rust RFC Book no_std](https://rust-lang.github.io/rfcs/1184-stabilize-no_std.html)
- [Rust no_std DAQ](https://justjjy.com/Rust-no-std)
- [testing-for-no-std-compatibility](https://blog.dbrgn.ch/2019/12/24/testing-for-no-std-compatibility/)
- [substrate中关于cfg_attr的介绍](https://substrate.dev/docs/zh-CN/tutorials/add-a-pallet/import-a-pallet)

## 结论

参照serder的使用以及一些论坛的讨论，推荐使用#![cfg_attr(not(feature = "std"), no_std ))]来同时支持std和no_std.