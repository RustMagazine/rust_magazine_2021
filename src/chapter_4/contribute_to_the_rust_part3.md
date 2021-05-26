---
pub_date: Thu, 30 Apr 2021 18:00:00 GMT
description: How to impl Rust lint

---

# 如何为 Rust 语言做贡献 Part 3 | 年轻人的第一个代码质量检测(Lint)

作者：CrLF0710（野喵）/ 后期编辑：张汉东

---

大家好，这次以为 rustc 增加一个代码质量检测为例，介绍一下编译器的一些内部细节。

先来看下本次要讲的内容的主角。这个代码检测检测是 **Rust RFC《允许使用非ASCII标识符》**中的一个小的功能点，名叫 uncommon_codepoints 。这次要检查的内容是代码中的标识符中是否含有非常规的Unicode代码点，如果有，默认给用户报一个编译警告。如果大家想在Github上阅读原PR的代码的话，[链接在这里](https://github.com/rust-lang/rust/pull/67810)。

# 理解 Rustc 总体过程

大家都知道，当前 rustc 的编译过程主要分成四个大的阶段：解析(parsing)，展开(expansion), 分析(analysis), 代码生成(codegen)。

* 第一步的解析是读取你提供的源码的过程，把它变成编译器的内部数据结构。

* 第二步的展开是对内部结构进行一些初步的变换，去除各种语法糖，同时处理掉各种路径解析之类的工作，使其统一化。

* 第三步的分析是对内部结构描述的程序进行进一步的变换，在这个阶段里也会完成各种类型检查、借用检查等等工作，确保你的代码是符合Rust规格的。

* 第四部的代码生成则是将内部结构描述的程序单态化，然后送给后端（现在有两个官方支持的后端：LLVM和Cranelift）让它们生成优化过的目标指令，最后归档或链接。

听起来有点繁琐对不对，但是实际上大多数部分都只是代码数量比较大，做的工作并不复杂。毕竟编译器只是一个数据变换程序，从你提供给它的文本输入数据变换成一些目标指令塞进某种作为容器的文件格式里而已。

# 代码质量检测模块 (rustc_lint)

在 rust 代码仓库 `compiler/rustc_lint`目录下是一个专门用来实现各种代码质量检测分析过程(Lint pass)的模块。Lint 有好几个时机执行，从而分成了若干种类，在这里我们这词要写的这个lint是要对编译器实际处理的字面代码进行检查，也不涉及到各种类型信息相关的分析，所以我们可以选择一个比较靠前的执行时机。

按照rustc_lint规定的分类，我们可以把它作为是early分类下的实现。

在 `src/librustc_lint/non_ascii_idents.rs`文件里，已经有一个关于非ASCII标识符的代码质量分析过程了，它现在含有一项lint，我们在里面加一项，代码如下：

```rust
declare_lint! {
    pub UNCOMMON_CODEPOINTS,
    Warn,
    "detects uncommon Unicode codepoints in identifiers"
}
```

这个宏是 rustc_lint 这个模块定义的，第一项是lint的名称的全大写形式`(UNCOMMON_CODEPOINTS)`；第二项是默认等级，我们在这里默认是报警告，所以是Warn；第三项是这个Lint的文本说明，这个说明是会显示在`rustc -W help`命令的输出中的。

以上就是这个lint本身的声明了，然后我们修改这个lint pass的声明，将

```rust
declare_lint_pass!(NonAsciiIdents => [NON_ASCII_IDENTS]);
```

改成

```rust
declare_lint_pass!(NonAsciiIdents => [NON_ASCII_IDENTS, UNCOMMON_CODEPOINTS]);
```

这样编译器的代码就知道这个分析过程会负责这两项Lint的分析。

# 实现代码质量检测逻辑

接着我们开始真正实现必要的逻辑。修改下面的实际lint pass的实现：

```rust
impl EarlyLintPass for NonAsciiIdents {
    fn check_ident(&mut self, cx: &EarlyContext<'_>, ident: ast::Ident) {
        // ...
    }
}
```

这里我们要对`ident`的文本内容做个检查。我们调用`ident.name.as_str()`获取字符串，然后调用我们事先实现的包`unicode-security`中的检查方法，
就可以知道这个标识符里有没有非常规Unicode代码点了。

当我们发现了的确有这样的代码点存在，我们就调用cx上面的lint方法：

```rust
cx.struct_span_lint(
    UNCOMMON_CODEPOINTS,
    ident.span,
    "identifier contains uncommon Unicode codepoints",
)
.emit();
```

这个函数会根据你第一个参数指定的lint，去检查lint机制本身的当前提示级别`(allow, warn, deny, forbid)`，有可能产生一个警告或报错。

第二个参数`span`在编译器里用来标识一段原始代码。编译器警告错误提示在有`span`的情况下会“引述”你的源代码，展示给用户说是这里产生了警告或错误。

第三个参数则是给用户展示的提示文字，告诉用户这里产生警告或错误的原因。

我们没有其他需要给用户显示的提示啦，所以这里最后调用`emit()`方法，就完成了。

# 准备单元测试用例

Rust 要求每个面向用户的功能都有测试用例。在这里我们新加了一个Lint，所以也需要写一个。
于是我在`src/test/ui/lint/rfc-2457-non-ascii-idents/`这个目录下增加一个`lint-uncommon-codepoints.rs`文件，
里面首先用`#![deny(uncommon_codepoints)]`来将警告变成了错误（这样方便写测试用例），然后试着写了几个具有不推荐使用的Unicode代码点的标识符。
其中一行是这样的：
```rust
const µ: f64 = 0.000001; //~ ERROR identifier contains uncommon Unicode codepoints
```
注意右边的注释，`//~ ERROR` 表示期望这一行产生一个编译错误，错误提示以右边的文本开头。我们编译并跑一下这个测试用例(`x.py test src/test/ui/ --bless`)，
还会记录一个参照用的错误提示文件`lint-uncommon-codepoints.stderr`。

# 一点额外的收尾工作 & 上传提交
我们这次修改有一点额外的收尾工作，因为引入了`unicode-security`这个新的包和它的依赖项`unicode-script`，因此我们更新了`Cargo.toml`, `Cargo.lock` 还有
`src/tools/tidy/src/deps.rs`这些文件，新增了新的依赖项相关的信息。

都准备好之后，我们就提交PR，并且喊上官方的大佬来review 啦。在处理了官方大佬的几个review意见后，我们的代码就通过了review，并且合并进了仓库。

怎么样，是不是很容易呢？欢迎大家也来多多参与，做出自己的贡献。
