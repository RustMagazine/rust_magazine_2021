# 常见问题汇总

> 说明：
>
> 常见问题汇总来自于：
> - [StackOverflow - Rust](https://stackoverflow.com/questions/tagged/rust?tab=Votes)
> - [知乎 - Rust](https://www.zhihu.com/topic/19674381/hot)
> - [https://rust-zh.github.io/faq/](https://rust-zh.github.io/faq/)


---

## 为什么go仅仅160M的安装包就可以编译程序？而rust却还需要几个G的msvc才能编译？

> 来源：[知乎](https://www.zhihu.com/question/458838401)

> 在windows下，我下载了一个go1.16.zip版，仅仅150M，解压后，都不需要安装，就可以用命令行编译程序，非常小巧。但是，为什么rust，在安装了自身安装包后，却还得要vstudio build，安装得好几个G！为什么都同样是编译程序，差距如此之大？我的疑问是差距在哪里，在编译方面？难道rust不能设计成象go这样，不依赖msvc？

### 回答：

**什么是 MSVC ？**

the Microsoft Visual C++ compiler and libraries toolset 。微软 Visual C++ 编译器和库工具集。

**为什么要有 MSVC ？**

MSVC的目标是成为Windows上针对Windows的最佳编译器选择，无论您选择使用哪种编辑器或IDE。 我们旨在通过在以下关键领域上进行持续投资来实现我们的目标：完全符合C ++，更好的错误和警告，运行时性能，可靠性，构建吞吐量和最佳安全性。

**Rust 为什么要支持 MSVC 以及如何支持 ？**

[Windows - The rustup book](https://link.zhihu.com/?target=https%3A//rust-lang.github.io/rustup/installation/windows.html)

因为 Rust 要支持 Windows 开发，而 Windows 上面有两类 ABI ：

- Visual Studio使用的本机（MSVC）ABI
- GCC工具链使用的GNU ABI。

你需要哪种版本的Rust很大程度上取决于您要与哪些C / C ++库进行互操作：

- 要与Visual Studio生产的软件互操作，请使用Rust的MSVC版本；
- 要与使用MinGW / MSYS2工具链构建的GNU软件互操作，请使用GNU构建。

以MSVC ABI为目标时，Rust还需要安装Visual Studio 2013（或更高版本）或Visual C ++ Build Tools 2019，以便rustc可以使用其链接器。 对于Visual Studio，请确保选中“ C ++工具”和“ Windows 10 SDK”选项。 基本使用GNU构建不需要额外的软件安装。

如果你计划仅使用 Rust 库和可以与 MinGW 一起构建和使用的各种开源库，则可以完全避免使用Microsoft 的工具。 只需将 Rust 切换到x86_64-pc-windows-gnu目标即可。

但是，如果你打算使用本地Windows库进行Windows开发，则需要Microsoft的链接器，并且应该使用 Rust 的 x86_64-pc-windows-msvc 目标与之兼容。 由于Windows上的大多数人都对Windows开发感兴趣，因此这是默认设置。

**而 Rust却还需要几个G的 msvc 才能编译？**

因为找不到人实现必要的功能，不得不依赖Windows sdk 。

**go 为什么不需要 MSVC ？**

因为go根本没做 msvc 的支持（不是默认支持，需要你自己手工再做处理），你用 cgo 只支持用 GCC。 （不晓得现在支持 msvc 没有）。

**为什么 Rust 不能像 go 那样，不依赖 msvc ？**

所以，你觉得呢？

这正是 Rust 和 Go 设计目标的差别了。

Rust 语言是一个通用的系统级语言，Go 语言则不是这个目标。所以 Go 可以不依赖 MSVC ，Rust 则不可以。

未来，Rust 将越来越倾向于使用 LLVM LLD 链接器，而不是平台本机链接器（Microsoft或GNU binutils）。 LLD 是通用的，旨在支持所有平台和ABI。 因此，未来预计将不需要任何其他工具。

---

## 为什么Rust中的String不能用整数下标进行切片？

> 来源：[知乎](https://www.zhihu.com/question/458788810)

    ```rust
    fn main() {
        let name: String = "ABCD".to_string();
        println!("{}", &name[2..3]);
    }
    // 为什么不能直接用 name[2] 或者 &name[2] ？
    ```

> 另外，为什么切片还要加&？

### 回答：

[std::string::String - Rust](https://doc.rust-lang.org/std/string/struct.String.html#utf-8)

文档里说的很清楚了。

Rust 里 String 总是按 UTF-8 编码的。而索引旨在进行恒定时间操作，但是UTF-8编码不允许我们执行此操作。 另外，索引应返回哪种类型呢？字节，码点 还是 字素簇(grapheme cluster)。

实际上String 文档里帮你定义基于 字节和字符 处理的两类基本方法。

切片为啥加 & ，因为 Rust 里 切片 是一个 DST，必须放 & 后面。


---





---

## 其他摘录

> 来源：[https://rust-zh.github.io/faq/](https://rust-zh.github.io/faq/)
>
> 欢迎贡献：
> 
> 更多阅读： [https://rust-zh.github.io/faq/](https://rust-zh.github.io/faq/)

---

