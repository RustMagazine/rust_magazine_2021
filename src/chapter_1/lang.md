# 本月简报：官方动态

- 来源：[Rust日报](https://rustcc.cn/section?id=f4703117-7e6b-4caf-aa22-a3ad3db6898f)
- 作者：`Rust`日报小组
- 专题编辑：张汉东

---

## Rust 1.49 稳定版发布

2020年最后一天，Rust 1.49 稳定版发布了。稳定版 Rust 发布周期为六周一次。

值得关注的更新：

- `aarch64-unknown-linux-gnu` 升级为`Tier 1`。
- `aarch64-apple-darwin` 和`aarch64-pc-windows-msvc` 得到`Tier 2` 级别的支持。
- 单元测试中线程中的print输出将会被捕获，默认不会在控制台打印出来了。如果不需要捕获，需要添加--nocapture参数。
- `union`支持 `impl Drop trait` 了
支持使用`ref`关键字让解构的字段不再被move 而是被借用。

```rust,editable
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main(){
    let person = Person {
    name: String::from("Alice"),
    age: 20,
    };

    // `name` is moved out of person, but `age` is referenced.
    let Person { name, ref age } = person;
    println!("{} {}", name, age);
}

```

[https://blog.rust-lang.org/2020/12/31/Rust-1.49.0.html](https://blog.rust-lang.org/2020/12/31/Rust-1.49.0.html)

## Rust 将不再支持 Windows XP

目标i686-pc-windows-msvc和x86_64-pc-windows-msvc是个怪胎。它们对Windows 7+有Tier 1支持，但对Windows XP也有Tier 3支持。这是Firefox需要在XP上运行时的遗留问题。然而在他们放弃XP支持后的几年里，尽管偶尔会有修复，但大多都是任由它过期了。

因此有人建议，正式放弃这个Tier 3支持状态，可以更好地反映出目前对XP的实际支持程度，不再让一个Tier 1目标背负着实际上不支持Tier 3目标的担忧。

只要LLVM和他们的链接器仍然支持XP目标，移除官方的XP支持不会阻止任何人编译到XP（减去std）。

对Windows 7以上的目标的影响将是移除工作区和一个支持XP的运行时兼容性层。此外，还有可能使用更现代的API，而这些API可能曾经因为兼容性问题而被避免。

如果在未来，有人积极支持XP，那么最好的办法是为此创建一个新的目标。这可以从其自身的优点出发，而且它的开发不会影响到一级平台的开发。

官方团队接受了该建议。

[https://github.com/rust-lang/compiler-team/issues/378](https://github.com/rust-lang/compiler-team/issues/378)

## Rustup 宣布发布 1.23.0 版本

官方发布 1.23.0 版本，其中最激动人心的改变就是支持 Apple M1 设备。大家可以安心的买 M1 了！

[原文链接](https://blog.rust-lang.org/2020/11/27/Rustup-1.23.0.html)

## Rust 官方知名开发者陆续入职巨头科技公司

**Niko Matsakis，Esteband K 入职 Amazon**

Niko Matsakis 入职 Amazon 担任 Rust 团队的技术主管。

    Niko的博客链接: https://smallcultfollowing.com/babysteps/blog/2020/12/30/the-more-things-change/

Esteband K 入职 Amazon 研究 Rust 编译器和相关工具。

    Twitter 链接：https://mobile.twitter.com/ekuber/status/1345218814087053312

**Patrick Walton 入职 Facebook**

Patrick Walton 将领导 Facebook 的新 Rust 团队，致力于为 Rust 社区改善其编译器和生态。

    Twitter 链接：https://twitter.com/pcwalton/status/1345094455712333824

## futures-rs 0.3.9 发布

- 把`pin-project`这个`crate`替换成了`pin-project-lite`, 在`--no-default-features`的情况下大幅提高了编译速度.
- 增加了几个新的API方法
    - stream::repeat_with
    - StreamExt::unzip
    - sink::unfold
    - SinkExt::feed

链接：[https://github.com/rust-lang/futures-rs/releases/tag/0.3.9](https://github.com/rust-lang/futures-rs/releases/tag/0.3.9)

## Rust 异常处理小组的工作范围是什么？

该小组的主要重点是继续进行小组成立前的错误处理相关工作。为此而努力系统地解决与错误处理相关的问题，以及消除阻碍RFC停滞不前的障碍。

在小组成立的最初几次会议上，制定了一些短期和长期目标，这些目标主要围绕下面三个主题：

- 使 `Error` trait 在生态中应用更加普及。
- 提升错误处理的开发体验。
- 编写更多的关于错误处理的学习资源。

下面具体来说。

**建立统一的标准`Error` trait。**

`Error` trait 从 `1.0`开始就存在了，并暴露了两个方法。`Error::description`和`Error::cause`。由于它最初的构造，由于一些原因，它太过拘谨。`Failure` crate通过导出Fail trait解决了Error trait的许多缺点。

在这一点上，加强`std::error::Error` trait，使其可以作为`Error` trait被整个Rust社区采用，自2018年8月RFC 2504被合并以来，一直是一个持续的过程。

这个过程还涉及稳定许多`Error` trait API和`crates`，截至本文撰写时，这些API和crates只在Nightly使用。这些包括backtrace和chain方法，这两种方法对于处理错误类型非常有用。如果你有兴趣关注或贡献这项工作，请看一下这个问题。

另一个相关的举措是将`Error` trait迁移到核心，这样它就可以更广泛地用于不同的场景（比如在FFI或嵌入式上下文中）。

**增加通过回溯（backtrace）类型进行迭代的能力**

到目前为止，`backtrace` 类型只实现了`Display`和`Debug`特征。这意味着使用回溯类型的唯一方法是打印出来，这不是很理想。一个能够提供迭代堆栈框架的迭代器API将使用户能够控制他们的反向跟踪如何被格式化，这是一个必要的步骤，将`std::backtrace::Backtrace`支持添加到像`color-backtrace`这样的箱子中。

在研究了如何解决这个问题的策略后，我们发现回溯箱已经有了一个框架方法，可以很好地实现`Iterator` API。在std中公开一个相同的方法应该是一个相对简单的考验。

我们已经为此开了一个`[PR](https://github.com/rust-lang/rust/pull/78299)`，如果有人想看的话，可以去看看。

**通用成员访问**

目前，当我们想要获取一些与错误相关的额外上下文时，需要调用一些特定的方法来获取该上下文。例如，如果要查看一个错误的回溯，我们会调用回溯方法： `let backtrace = some_error.backtrace();`。这种方法的问题是，它不可能支持在`std`之外定义的类型。即使是存在于`std`内的类型，也需要定义一个方法来访问每个各自的类型，这使得事情变得很麻烦，而且更难维护。

顾名思义，通用成员访问，当它得到实现时，是一种类型无关的方法，可以从`Error` trait对象中访问不同的上下文。这有个类比示例，当你要把一个字符串解析成一个数字的时候，用这样的方法。
```rust
let ten = "10".parse::<i32>();
```
或者通过迭代器来collect生成的内容时：
```rust
use std::collections::HashSet;

let a_to_z_set = ('a'..='z').collect::<HashSet<_>>();
```
跟上面用法类似，您可以通过指定错误的类型ID来访问某个上下文片段。
```rust
let span_trace = some_error.context::<&SpanTrace>();
```
这可以用来获取与错误相关的其他上下文，如错误的回溯、错误的来源、状态码、替代的格式化表示（如&dyn Serialize）。

这个功能将使我们计划在以后添加的其他功能成为可能，比如提供一种方法来报告程序中错误来源的所有位置，以及提供一种除了显示和调试之外的更一致的错误报告格式。

Jane在推动这些想法上做了很多工作。你可以查看相关的[RFC](https://github.com/rust-lang/rfcs/pull/2895)。

**编写一本`Rust`错误处理最佳实践的书**

最后但并非最不重要的一点是，围绕创作[The Rust Error Book](https://github.com/rust-lang/project-error-handling)的团队引起了很多兴趣。 本书的目的是根据各自的用例来整理和交流不同的错误处理最佳实践。 这可能包括FFI用例，或有关从程序返回错误代码的最佳实践。

这是一项持续不断的工作，在接下来的几周和几个月中将会看到许多进步！

**脚注**

- `Error::description`方法只支持字符串片段，这意味着创建包含附加上下文的动态错误信息是不直接的。这个方法被弃用，改用`Display`。
- `Error::cause`方法，现在被称为`Error::source`，并没有强制要求错误具有 "静态生命周期"，这意味着 downcasting 错误源是不可能的，这使得使用动态错误处理程序来处理错误变得更加困难。

## Rustdoc 性能提升

有两个PR明确地旨在提高rustdoc的性能：

- Rustdoc：缓存已解析的链接[＃77700](https://github.com/rust-lang/rust/pull/77700)。该`PR`将文档生成的链接的时间缩短了 `90%`。
- 不要在文档内链接中寻找覆盖实现（blanket-impls）[＃79682](https://github.com/rust-lang/rust/pull/79682)。因为它从来没有起过作用，并且已经引起了严重的性能问题。

Rustdoc 团队还清理了一些技术债务。比如 `jyn514` 不久前注意到，Rustdoc中的大部分工作都是重复的: 实际上有三种不同的抽象语法树(ast)！一个用于doctree，一个用于clean，还有一个是编译器使用的原始HIR。Rustdoc花费了大量的时间在它们之间进行转换。大部分的速度改进来自于完全去掉部分AST。

文章里也介绍了Rustdoc的工作原理：

- 运行编译器的某些部分以获得需要的信息。
- 删除编译器提供的不需要的信息（例如，如果一个项目是doc(hidden)，就不需要它）。这一部分有很多话要说，也许会再写一篇博文来详细介绍。
- `doctree pass`，它在编译器的某些项目上添加了一些`rustdoc`需要的额外信息。
- `clean pass`将编译器类型转换为`rustdoc`类型：基本上，它将所有内容都转换为 "可打印 "内容。
- 渲染（render）通证，然后生成所需的输出（HTML 或，在Nightly，JSON）


更多内容： [https://blog.rust-lang.org/inside-rust/2021/01/15/rustdoc-performance-improvements.html](https://blog.rust-lang.org/inside-rust/2021/01/15/rustdoc-performance-improvements.html)

## Nightly的Reference已上线Const Generics的文档

Const Generics 计划在1.50版进入stable，官方今天在nightly的Reference上已更新好相关文档。

链接：[https://doc.rust-lang.org/nightly/reference/items/generics.html#const-generics](https://doc.rust-lang.org/nightly/reference/items/generics.html#const-generics)


## Nightly Edition Guide 文档增加了 Rust 2021 Edition 小节

内容还在逐步更新，可以先关注。

链接： [https://doc.rust-lang.org/nightly/edition-guide/rust-next/index.html](https://doc.rust-lang.org/nightly/edition-guide/rust-next/index.html)


## RFC 2945 : "C unwind" ABI 支持相关情况

官方 FFI-Unwind 项目工作组已经将 RFC 2945 合并了。该 RFC 描述了对 "C unwind" ABI 的支持。

RFC 概要：

> 引入了一个新的`ABI`字符串“`C-unwind`”，以支持从其他语言(如c++)到Rust框架的`unwind`，以及从`Rust`到其他语言的`unwind`。此外，当`unwind`操作以“non `Rust`”、“non `C-unwind`”ABI到达`Rust`函数边界时，我们为之前未定义的有限几种情况定义了行为。作为该规范的一部分，我们引入了术语“Plain Old Frame”(POF)。POF帧不会挂起析构函数，可以轻松地释放析构函数。这个RFC没有定义被外部异常展开的Rust框架中的catch unwind行为。

引入动机：

> 有些Rust项目需要跨语言展开以提供所需的功能。 一个主要的例子是`Wasm`解释器，包括`Lucet`和`Wasmer`项目。
>
> 还有一些现有的`Rust` crate（尤其是围绕libpng和libjpeg C库的包装器）会在`C`帧之间出现混乱。 这种展开的安全性取决于Rust的展开机制与`GCC`，`LLVM`和`MSVC`中的本机异常机制之间的兼容性。 尽管使用了兼容的展开机制，但是当前的`rustc`实现假定“extern `C`”函数无法展开，这允许LLVM在这种展开构成未定义行为的前提下进行优化。
>
> 之前已经在其他RFC（包括＃2699和＃2753）上讨论了对此功能的需求。

RFC 2945: [https://github.com/rust-lang/rfcs/blob/master/text/2945-c-unwind-abi.md](https://github.com/rust-lang/rfcs/blob/master/text/2945-c-unwind-abi.md)

现在 FFI-unwind 工作组正在为`C-unwind` ABI 指定新的行为（覆盖之前的未定义的行为），[RFC 2945 实现PR](https://github.com/rust-lang/rust/pull/76570)。

然而，在起草 "`C unwind` " RFC 时，工作组发现围绕`longjmp`和类似函数的现有保证可以改进。虽然这与`unwind`并没有严格的关系，但它们有着密切的联系：它们都是 `non-local` 的控制流机制，防止函数正常返回。由于`Rust`项目的目标之一是让`Rust`与现有的`C`系语言互操作，而这些控制流机制在实践中被广泛使用，工作组认为`Rust`必须对它们有一定程度的支持。

这篇博文将解释该问题。如果你有兴趣帮助指定这种行为，欢迎参与!

官方博文地址：[https://blog.rust-lang.org/inside-rust/2021/01/26/ffi-unwind-longjmp.html](https://blog.rust-lang.org/inside-rust/2021/01/26/ffi-unwind-longjmp.html)


## Rust Playground 支持 vim 模式

Rust Playground `vim` 模式，可以通过输入 `:w` 回车运行编译，非常棒的使用体验。

