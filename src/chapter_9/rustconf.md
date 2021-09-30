# 官方RustConf 2021 盘点 

作者： 张汉东

https://rustconf.com/talks 大会官网。

- [Project Update by Niko Matsakis](https://www.youtube.com/watch?v=ylOpCXI2EMM&list=PL85XCvVPmGQgACNMZlhlRZ4zlKZG_iWH5&index=1)
- [Deadlocked by Mara Bos](https://www.youtube.com/watch?v=DnYQKWs_7EA&list=PL85XCvVPmGQgACNMZlhlRZ4zlKZG_iWH5&index=2)
- [Move Constructors: Is it Possible? by Miguel Young de la Sota](https://www.youtube.com/watch?v=UrDhMWISR3w&list=PL85XCvVPmGQgACNMZlhlRZ4zlKZG_iWH5&index=3)
- [The Importance of Not Over-Optimizing in Rust by Lily Mara](https://www.youtube.com/watch?v=CV5CjUlcqsw&list=PL85XCvVPmGQgACNMZlhlRZ4zlKZG_iWH5&index=4)
- [Identifying Pokémon Cards with Computer Vision by Hugo Peixoto](https://www.youtube.com/watch?v=BLy_YF4nmqQ&list=PL85XCvVPmGQgACNMZlhlRZ4zlKZG_iWH5&index=5)
- [Fuzz Driven Development by Midas Lambrichts](https://www.youtube.com/watch?v=qUu1vJNg8yo&list=PL85XCvVPmGQgACNMZlhlRZ4zlKZG_iWH5&index=6)
- [Writing the Fastest GBDT Library in Rust by Isabella Tromba](https://www.youtube.com/watch?v=D1NAREuicNs&list=PL85XCvVPmGQgACNMZlhlRZ4zlKZG_iWH5&index=7)
- [Whoops! I Rewrote It in Rust by Brian Martin](https://www.youtube.com/watch?v=m-Qg3OoPIdc&list=PL85XCvVPmGQgACNMZlhlRZ4zlKZG_iWH5&index=8)
- [How I Used Rust to Become Extremely Offline by Luke Westby](https://www.youtube.com/watch?v=b0I4vP2CP88&list=PL85XCvVPmGQgACNMZlhlRZ4zlKZG_iWH5&index=9)
- [Supercharging Your Code with Five Little-Known Attributes by Jackson Lewis](https://www.youtube.com/watch?v=8d7DqeYXq7A&list=PL85XCvVPmGQgACNMZlhlRZ4zlKZG_iWH5&index=10)
- [Compile-Time Social Coordination by Zac Burns](https://www.youtube.com/watch?v=4_Jg-rLDy-Y&list=PL85XCvVPmGQgACNMZlhlRZ4zlKZG_iWH5&index=11)
- [Hacking `rustc`: Contributing to the Compiler by Esteban Kuber](https://www.youtube.com/watch?v=9H9SO2u6Q20&list=PL85XCvVPmGQgACNMZlhlRZ4zlKZG_iWH5&index=12)
- [This Week in Rust: 400 Issues and Counting! by Nell Shamrell-Harrington](https://www.youtube.com/watch?v=OZPXhmy-wVw&list=PL85XCvVPmGQgACNMZlhlRZ4zlKZG_iWH5&index=13)

## 1.  Niko：Rust 下一步的目标是广泛使用！

slides：https://nikomatsakis.github.io/rustconf-2021-e44bec44/#1

2021 年对 Rust 来说，是非常令人兴奋的。

2021 年 Rust 基金会成立，以及一些致力于 Rust 开发的团队成立。

Rust 甚至被考虑用于 Linux 内核。

Rust 的学习曲线正在逐步被降低。Rust 生态也越来越丰富。

Rust 2021 edition 蓄势待发！其目标是关键性的采用和可持续发展。

Niko 在该talk 中分享了 Rust 近几年的成长。

下一步 Rust 的目标：广泛使用！

## 2. Mara Bos:  以改进 Rust 标准库锁为例，探讨大型工作团队如何突出困境

slides：https://m-ou.se/rustconf-2021-deadlocked-slides.pdf

**问题：**

在很久之前，为标准库添加同步原语。其中最重要的是 Mutex。因为操作系统已经为我们实现了很多同步原语，那么标准库只需要将它们包起来即可吗？答案是否定的。因为这样做不符合 Rust 的要求，它们是为 C 设计的。

为 Rust 设计实现时，有很多微妙的事情，但其中有三个较大的问题。

 第一个问题是，如果你拥有一个对象，你可以把它移动到内存的不同位置。意味着你不能假设一个对象在其整个生命周期内都会停留在同一个内存位置。但是，操作系统的锁原语是要求该对象在同一个内存位置。比如 SRW 读写锁。你可以将其放到堆上来达到效果，但是很低效。也无法静态实现。

第二个问题是，在 Rust 中 Unsafe 和 Safe 的边界是 非常明确的。即便你的实现是错误的或者是多余的，但内存安全是底线。比如，死锁。死锁是实现错误，但它绝对是安全的。然而，操作系统实现的锁并不总是死锁，还可能是未定义行为。如果只是简单的用 Rust 包装底层原语，那么就会破坏语言的安全保证。

第三个问题是，内存/资源泄漏是安全的。比如我们创建了一个 Mutex锁，但是忘记解锁了，我可以drop它，但它还是锁定的，它是安全的。但是在操作系统的实现中，销毁mutex将引起未定义行为。

**一个解决方案：**

使用 `parking-lot` 全局数据结构来取代这些锁。这个方案在 2018年9月被提出，然后经历了大概一年的讨论，直到大家的精力耗完了。因为这个改变是非常巨大的。最终只留下300多条讨论，PR被关闭。

再之后，当有人对 Mutex 的实现提出一些改进，人们就会回答说，parking lot 会解决这个问题。

人们仍然期望 parking lot在不久后被整合进标准库。但面对之前遗留的讨论，没有人能很好地概述问题是什么，没有任何需要改变的清单，没有明确的地方可以开始这件事。

**让我们从小事做起：**

挑选出一个最微小的障碍，开始尝试解决它。

第一个障碍：稳定性保证。 

Rust 承诺了很多保证，对于整合 parking lot 来说，要保证在某些情况下不会发生恐慌。但正是这个保证，引发了很多讨论。所以，Mara 提出，将这个保证的细节删掉。得到了整个团队的同意。于是，一个干扰的因素被去掉了。

第二个障碍：不可移动。

给微软发了一个 PR，添加了一个说明，澄清如果在windows平台如果锁没有被借用，就可以移动它。仅仅几天就得到了微软的响应，于是 团队得到了一个很大的平台的支持，有了可移动的mutex。

就这样，Mara拆分出了一些很小的问题，逐个去解决它。这样一来，小问题就不会再分散对大问题的注意力了。

**小结**

Mara 组建了 新的 Lib Team，命名为 Lib API Team，致力于 API 的设计，而不会被具体实现分心。本次分享的目的主要是用具体的案例来说明，当事情遭遇困境的时候，应该从微小单一的障碍着手，想办法突破困境。

## 3. Miguel Young： 移动构造函数，Rust 中可能吗？

> 作者说：注意，学习这一切你不需要C++知识!

> 我认为还是需要一点的 ，至少得知道什么是移动构造函数吧？
>
> 值得说明的是，以下这部分知识在 Cpp 中算高阶知识，但是在 Rust 中类似的知识，则是入门级的。
>
> C++ 中为了防止指向堆内存的成员变量被默认的拷贝构造函数浅拷贝而导致双重释放，所以需要开发者自己编写拷贝构造函数。
>
> 虽然 Rust 中没有构造函数，但C++的这个行为和 Rust 的 Copy 语义有些类似。
>
> 但是只有这种拷贝构造函数，在面对处理临时对象的时候，会有性能损失。因为在创建临时对象的时候，拷贝构造函数也会执行，这些临时对象用完就会被释放。
>
> 为了优化拷贝构造函数带来的性能问题，C++ 11 中引入了右值引用和移动构造函数。
>
> 右值的概念，相对于左值。对应于 Rust 中，左值即地址表达式，右值即值表达式。在 Rust 中，右值，即值表达式，如果没有使用let 绑定，它就是一个临时变量。C++ 也是一样，但是在 C++ 中，可以通过右值引用（`&&v`，两个引用前缀） 这个东西，将右值（临时值）的生存期延长了，相当于给右值又赋予一个变量名。
>
> cpp 中 `std::move` 可以将一切值变成右值。
>
> 有了右值引用，就可以实心移动构造函数来复用临时对象了，而不需要重写拷贝构造。移动构造函数，可以将一个对象的指针成员转移给另一个对象。只有创建对象的时候传入的是右值才会执行移动构造函数。

Rust 中处理 自引用类型 是个难题，虽然有 `Pin<T>` ，但是不如 C++ 那么自然。C++ 可以通过移动构造函数安全地处理自引用类型。

一个 "自引用（self-referential） "类型是指持有对自身的引用；异步Futures是目前Rust中最常见的自引用类型。然而，它们不能在不使引用无效的情况下被移动，所以它们被 Pin 在了堆或栈上。你不能返回它们，也不能把它们放入集合中（如果没有Boxing的话）。

该 talk 中，作者使用`Pin<P>`保证，将把所有的C++构造函数，而不仅仅是移动构造函数，移植到Rust中，而不影响Rust的使用后移动(move-after-use)保护（这是C++缺乏的）。在今天的稳定版Rust中完全支持通过移动返回和集合。除了零成本的C++ FFI之外，Rust的 "构造函数 "可以应用于在纯Rust中表达新的数据结构。

该 talk 作者在之前也写过详细的博客文章 ： https://mcyoung.xyz/2021/04/26/move-ctors/ 。

相关的库 ： https://github.com/mcy/moveit 

关于作者：

Miguel Young 是来自 Google OpenTitan 项目的开发者。OpenTitan 项目，想通过开源框架减少芯片被破解的可能。

> OpenTitan  **将由非营利组织lowRISC监督，** 该公司正在开发基于RISC-V架构的免费微处理器。
>
> **OpenTitan项目涵盖了各种逻辑组件的开发** RoT芯片的需求，包括基于RISC-V架构的lowRISC Ibex开放式微处理器，加密协处理器，硬件随机数生成器，恒定和随机存取存储器数据和密钥存储层次结构，机制保护，I / O输入块，安全启动媒体等
>
> 可以在必要时使用OpenTitan，以确保系统的硬件和软件组件的完整性，并确保未更换关键的系统组件，并基于制造商验证和授权的代码。
>
> **基于OpenTitan的芯片可用于** 服务器主板，网卡，消费类设备，路由器，物联网设备，以验证固件（检测恶意软件对固件的修改），提供加密的唯一系统标识符（硬件防伪保护）以及保护加密密钥（如果出现以下情况，则隔离密钥）：攻击者可以获得对计算机的物理访问权限），提供与安全相关的服务，并维护无法编辑或删除的隔离审核跟踪。

本人深挖了一下 [OpenTitan](https://github.com/lowRISC/opentitan) 项目，在 GitHub 的语言成分分析中看不到 Rust 的痕迹。但是在源码中搜索 Rust，则发现很多 Rust 痕迹。

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

   



