---
pub_date: Tue, 31 May 2021 21:00:00 GMT
description: Proc Macro Workshop Guide

---

# Rust过程宏系列教程 | Proc Macro Workshop 之 Builder 实现

作者：米明恒 / 后期编辑：张汉东

> 文字版首发：[https://blog.ideawand.com/2021/03/24/rust_procedural_macro/rust_proc_marco_workshop_guide-02/](https://blog.ideawand.com/2021/03/24/rust_procedural_macro/rust_proc_marco_workshop_guide-02/)
>
> 视频版本首发：[https://space.bilibili.com/500416539](https://space.bilibili.com/500416539)
>
> 了解过程宏开发环境并熟悉基本原理请阅读：[https://blog.ideawand.com/2021/02/27/rust_procedural_macro/rust_proc_marco_workshop_guide-01/](https://blog.ideawand.com/2021/02/27/rust_procedural_macro/rust_proc_marco_workshop_guide-01/)

---

本文以戴维·托尔奈（David Tolnay，也就是`syn`和`quote`这两个库的作者）的教学项目`proc-macro-workshop`出发，带领大家实战几个Rust过程宏的编写。

`proc-macro-workshop`是一个包含5个过程宏的“解题游戏”，每一个过程宏都是有实际应用价值的案例，通过一系列由简到繁的测试用例，指导你去完成每一个过程宏的开发，而我们要做的，就是编写代码，像闯关游戏一样依次通过每一个测试用例，在这个过程中，我们会学到不同类型的过程宏的开发方法。

好了，不废话了，准备好一台电脑，我们从第一个过程宏挑战任务`builder`开始，每一关对应的视频教程我也放在下面了。

序言视频版：
<iframe src="//player.bilibili.com/player.html?aid=332284630&bvid=BV16A411N7m2&cid=316964888&page=1" scrolling="no" border="0" frameborder="no" framespacing="0" allowfullscreen="true"> </iframe>

首先克隆https://github.com/dtolnay/proc-macro-workshop这个项目到本地，我们将在这个项目中进行开发

```shell
cd ~/blog.ideawand.com  # 进入我的工作目录
git clone https://github.com/dtolnay/proc-macro-workshop.git
```

克隆项目后，我们会得到下面这样的目录结构。回忆我们上一篇文章中提到的，声明过程宏的crate需要和使用过程宏的crate独立开，这个项目也不例外，其中，5个题目分别独立存放在5个文件夹中。而目录的最顶层定义了一个bin类型的crate，可以使用我们定义的过程宏。我们后续的一些调试工作会借助外层crate的`main.rs`来进行。每一个题目所在的crate目录中都有一个tests文件夹，里面有通过数字编号命名的测试用例，以及一个`progress.rs`文件，`progress.rs`控制了哪些测试用例是生效的，在解题闯关的过程中，每当你通过一个测试用例，你就要到`progress.rs`中开启下一个测试用例，直到通过所有的测试用例。当然啦，解题过程中每一步的提示信息，都在以数字编号开头的测试用例中。

```
└── blog.ideawand.com           -- 我们的工作目录
    └── proc-macro-workshop
        ├── bitfield            -- 虽然排在第一个，却是最难的一个，我们在最后一篇文章再去讨论它
        │   ├── impl
        │   ├── src
        │   ├── tests
        │   │   ├── <省略这里的内容。。。>
        │   └── Cargo.toml
        ├── builder             -- builder项目，我们今天的主角
        │   ├── src
        │   ├── tests
        │   │   ├── 01-parse.rs
        │   │   ├── 02-create-builder.rs
        │   │   ├── 03-call-setters.rs
        │   │   ├── 04-call-build.rs
        │   │   ├── 05-method-chaining.rs
        │   │   ├── 06-optional-field.rs
        │   │   ├── 07-repeated-field.rs
        │   │   ├── 08-unrecognized-attribute.rs
        │   │   ├── 08-unrecognized-attribute.stderr
        │   │   ├── 09-redefined-prelude-types.rs
        │   │   └── progress.rs  -- 闯关进度控制文件，在里面决定开启哪些测试用例
        │   └── Cargo.toml
        ├── debug               
        │   ├── src
        │   ├── tests
        │   │   ├── <省略这里的内容。。。>
        │   └── Cargo.toml
        ├── seq
        │   ├── src
        │   ├── tests
        │   │   ├── <省略这里的内容。。。>
        │   └── Cargo.toml
        ├── sorted
        │   ├── src
        │   ├── tests
        │   │   ├── <省略这里的内容。。。>
        │   └── Cargo.toml
        ├── Cargo.toml
        ├── LICENSE-APACHE
        ├── LICENSE-MIT
        ├── README.md
        └── main.rs            -- 外层Crate，可以在这里使用上面定义的各个过程宏，我们将通过这个文件来查看过程宏展开后生成的代码
```

以上信息主要来自于`proc-macro-workshop`项目的readme文档，如果大家的英文水平还可以的话，建议阅读一下原始的文档。

了解完整体项目的结构后，我们开始正式挑战`builder`项目！ 首先我们需要看一下`builder`题目要实现什么功能，每一个挑战题目到实现的目标都写在了`proc-macro-workshop`的readme文档中，下面我来把和`builder`项目相关的部分大致转述一下：
* 我们要实现一个派生宏`derive(Builder)`
* 这个宏提供了一个初始化结构体的方式，其使用效果大致如下,将`derive(Builder)`宏作用于`Command`结构体以后，会根据结构体的字段，自动生成一系列方法，来为我们提供初始化结构体中每个字段的方法：
```rust
use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    #[builder(each = "arg")]
    args: Vec<String>,
    current_dir: Option<String>,
}

fn main() {
    let command = Command::builder()
        .executable("cargo".to_owned())
        .arg("build".to_owned())
        .arg("--release".to_owned())
        .build()
        .unwrap();

    assert_eq!(command.executable, "cargo");
}
```

了解完任务目标以后，我们可以开始解题了。首先进入`builder`目录下的`tests`目录，并编辑`progress.rs`文件，将其中`t.pass("tests/01-parse.rs");`这一行的注释去掉，表示我们要检验第一个测试用例的结果是否正确。然后，我们打开`tests/01-parse.rs`这个文件，查看第一题的解题提示。

### 第一关
第一关视频版：
<iframe src="//player.bilibili.com/player.html?aid=587300512&bvid=BV1TB4y1P7ak&cid=317585480&page=1" scrolling="no" border="0" frameborder="no" framespacing="0" allowfullscreen="true"> </iframe>


> * 这个测试检测是否定义了一个名为`Builder`的派生宏，也就是说，只要我们定义了这个宏，什么都不用做，就可以通过这一关
> * 什么都不做，意味着我们只需要返回空的TokenStream即可，但是，考官建议我们额外多做一些尝试：
>   * 尝试把输入的TokenStream解析为`syn::DeriveInput`这个语法树节点类型
>   * 阅读官方文档中关于`syn::DeriveInput`这个结构体的说明，看看其中的字段分别包含了哪些信息，对我们后续解题有什么用处。
>     * 对于这个结构体的描述，我们会在第二关的闯关过程中再介绍，当然你也可以提前阅读文档

通过这一关的技能我们在上一篇文章中已经遇到过了，因此这一关不难，直接上代码。我们编写代码的位置是`builder/src/lib.rs`这个文件，代码如下：
```rust
use proc_macro::TokenStream;
use syn;

#[proc_macro_derive(Builder)] // 注意，这里和第一篇文章里的 #[proc_macro_attribute]不同
pub fn derive(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as syn::DeriveInput);
    TokenStream::new()
}
```
上述代码中，我们使用了`syn`包的`parse_macro_input!`宏将类型为`proc_macro::TokenStream`的input解析成了`syn::DeriveInput`类型的语法树节点`st`，然后产生一个空的`TokenStream`对象，返回给编译器。

这里需要额外注意的一点是，上一篇入门文章中我们给出的示例是一个属性样式的过程宏，属性样式的过程宏使用`#[proc_macro_attribute]`来定义，而本篇我们编写的是一个派生样式的过程宏，派生样式的过程宏用`#[proc_macro_derive(XXX)]`的形式来定义，除此之外，还有：
* 派生样式的过程宏，其名字不是由函数名来定义的，而是由`#[proc_macro_derive(XXX)]`中的`XXX`来定义的
* 派生样式的过程宏，其定义的函数签名只包含一个`TokenStream`输入，相当于上篇文章介绍的属性式过程宏的`item`入参，也就是说派生式的过程宏相比于属性式的过程宏，少了`attr`这个入参
* 开发派生式过程宏时，通常将输入的`TokenStream`解析为`syn::DeriveInput`，这是派生宏定义对应的语法树节点

> 其实，大家可以把派生过程宏认为是一种特殊的属性式过程宏，把属性式过程宏看成是派生式的扩展版本，能用派生式过程宏实现的，用属性式过程宏也可以实现。另外从发展时间线上来说，属性式过程宏是在2018版本中新加入的，所以它和派生式过程宏有功能重叠也是有历史原因的。

> 上述代码直接返回了空的`TokenStream`，而上一篇文章我们在定义属性式过程宏时曾经提到，最简单的过程宏就是对输入不做修改，原封不动返回给编译器，可是这里为什么你返回了一个空的`TokenStream`呢？这样做不是就把输入给修改了吗？ 这其实是派生式过程宏和属性式过程宏的另外一个区别，先存疑，我们会在稍后的第二关详细介绍这个问题。

由于我们引用了`syn`和`quote`两个包，我们还需要修改一下`builder/cargo.toml`文件，将`dependencies`小节下的`# TODO`替换为如下的依赖声明,`proc-macro2`包我们后面会用到，这里也先引用过来：
```toml
syn = {version="1.0", features=["extra-traits"]}
proc-macro2 = {version="1.0"}
quote = {version="1.0"}
```
上述`syn`包额外加入的`extra-traits`特性是为了后续调试的方便。接下来，我们在`builder`目录下运行`cargo test`，不出意外，我们应该顺利通过了第一关测试。现在可以修改`builder/tests/progress.rs`文件,开启第二关的测试。

### 第二关
第二关视频版：
<iframe src="//player.bilibili.com/player.html?aid=802590935&bvid=BV1uy4y147zG&cid=322449371&page=1" scrolling="no" border="0" frameborder="no" framespacing="0" allowfullscreen="true"> </iframe>

> 我们要生成一个辅助的结构体，他的命名格式为`<结构体名称>+Builder`，同时，我们要为原始结构体生成生成一个`builder`方法，让他返回一个辅助结构体的实例。最终要生成的代码样式如下。：
> ```rust
> pub struct CommandBuilder {
>     executable: Option<String>,
>     args: Option<Vec<String>>,
>     env: Option<Vec<String>>,
>     current_dir: Option<String>,
> }
>    
> impl Command {
>     pub fn builder() -> CommandBuilder {
>         CommandBuilder {
>             executable: None,
>             args: None,
>             env: None,
>             current_dir: None,
>         }
>     }
> }
> ```

为了实现上面的功能，我们要用到几个个知识点，这些个知识点的其他介绍可以参考官方文档：
* 标识符使用`syn::Ident`类型来表示
* 从`syn::DeriveInput`类型中获取原始输入结构体的标识符
* 构建出`CommandBuilder`这个新的标识符
* 从`syn::DeriveInput`类型中获取原始输入结构体各个字段的信息，比如字段名、字段的类型等信息
* 使用`quote`包中提供的`quote!`宏来按照模板生成`TokenStream`的方法

首先是关于标识符的获取与构建:
* `syn::DeriveInput`类型提供了`ident`属性，可以获取到被过程宏修饰的原始结构体的标识符对象
* `syn::Ident`类型提供了`to_string()`方法，可以将标识符转换为字符串
* `syn::Ident::new()`方法可以创建一个新的`syn::Ident`类型的变量
因此，我们通过下面几行代码可以构建出新的辅助结构体的名称标识符：
```rust
    let struct_name_literal = st.ident.to_string();
    let builder_name_literal = format!("{}Builder", struct_name_literal);
    let builder_name_ident = syn::Ident::new(&builder_name_literal, st.span());
```
在上面的代码中，有几点说明：
* 注意区分`_literal`和`_ident`这两类变量的命名，这是我个人的习惯，大家也可以用其他的命名方式，但要注意区分，字符串和标识符不是一种类型
* 标识符除了名字的字符串之外，还携带了它在源代码中的位置信息，也就是`span`
* 上述的第三行中，在创建新的`Ident`时，将`st`的span作为新创建标识符的span，是出于这样的原因：
  * `span`信息主要用于发生编译错误时，编译器给用户指示出错误的位置
  * 编译器报错时，不会展示过程宏生成的代码，而只会展示用户编写的原始文件代码
  * 因此，对于我们通过过程宏产生出来的代码，应该指向用户原始代码文件中的某个位置，而不是凭空指向一个不存在的位置，否则后续一旦产生编译器报错，将会产生令人难以理解的错误提示
  * 由于我们后续要生成的代码都是由用户输入的原始结构体产生的，所以将原始输入结构体的位置信息当做虚构出的标识符的位置信息，后续一旦报错，编译器显示的错误提示将指向用户原始的结构体，从而引导用户有效排查问题


构建出新的标识符后，我们就可以生成一些简单的代码了，整体的代码如下所示，在下面会详细解释：
```rust
use proc_macro::TokenStream;
use syn::{self, spanned::Spanned};
use quote::{ToTokens, quote};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as syn::DeriveInput);
    match do_expand(&st) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn do_expand(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    
    let struct_name_literal = st.ident.to_string();
    let builder_name_literal = format!("{}Builder", struct_name_literal);
    let builder_name_ident = syn::Ident::new(&builder_name_literal, st.span());

    let struct_ident = &st.ident;  // 模板代码中不可以使用`.`来访问结构体成员，所以要在模板代码外面将标识符放到一个独立的变量中

    let ret = quote! {     // ----------------------------------+
        pub struct #builder_name_ident {                   //   |
            // TODO                                             |
        }                                                  //   |
        impl #struct_ident {                               //   |
            pub fn builder() -> #builder_name_ident {      //  被quote!宏包裹的是模板代码
                #builder_name_ident{                       //   |
                    // TODO                                     | 
                }                                          //   |
            }                                              //   |
        }                                                  //   |
    };                     // ----------------------------------+

    return Ok(ret);
}
```
> 划重点:
> 首先观察上面代码的骨架，分为了两个函数，宏定义本身将TokenStream转换为语法树对象后，交给另一个函数去完成主要功能，并处理其返回结果；`do_expand`函数返回一个`syn::Result`类型的结果。这种框架模式可以使得我们在后续书写代码时方便的进行错误处理，后续的几个实验项目我们也会使用类似的骨架。

对上面代码几个重点的说明：
* `to_compile_error`方法是`syn`包提供的一个返回错误的方式，它会产生一个包含错误信息的`proc_macro2::TokenStream`类型的结果，将其返回给编译器后，我们就可以在编译器的输出日志中看到用波浪线标注出的错误位置以及上下文代码了。我们在后面会介绍如何产生错误信息。
* `quote!`宏可以通过模板的形式生成TokenStream,在上一篇文章中我们已经了解到，所谓的过程宏，就是返回一个加工修改后的TokenStream，而TokenStream的结构很复杂，手工来生成一层层的嵌套结构太痛苦了，所以我们就有了`quote!`宏这个工具来帮助我们把和rust语言很相近的模板语言转换为TokenStream，在模板中我们可以做变量替换。
  * `quote!`宏内部形如`#xxxx`的部分会被替换为`quote!`宏外面定义的变量所表达的语法树元素
  * `quote!`宏的用法与rust内置的`macro_rules!`宏很类似，也支持重复替换等功能，区别是`quote!`里面用`#`而`macro_rules!`里面用`$`
* 代码中两个地方使用到了`.into()`，这些都是为了在`proc_macro::TokenStream`和`proc_macro2::TokenStream`类型之间转换，基本上`syn`和`quote`包产生的结果都是`proc_macro2::TokenStream`，我们的大原则就是在中间环节都用`proc_macro2`包的，只有在最终返回编译器的时候再转换成`proc_macro`包的。

上述代码写完以后，我们就可以看看我们的过程宏生成的结果是什么样的了，这里我们要借助`cargo expand`这个命令来查看展开的结果，如果没有安装过这个工具，可以通过`cargo install cargo-expand`命令来安装。

目前我们关心的是我们编写的过程宏在第二关的测试用例中使用时，会产生什么样的效果，因此我们就以第二关的测试用例来查看展开效果，将`blog.ideawand.com/proc-macro-workshop/builder/tests/02-create-builder.rs`文件中的内容全部拷贝到`blog.ideawand.com/proc-macro-workshop/main.rs`中，覆盖原来`main.rs`中的全部内容即可，然后在`proc-macro-workshop`目录下执行`cargo expand`命令，应该可以看到下面的输出,我将过程宏生成的代码用注释标记了出来：
```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use derive_builder::Builder;
pub struct Command {                 // --------------------+
    executable: String,                                  // |
    args: Vec<String>,                                   // 用户原始的代码
    env: Vec<String>,                                    // |
    current_dir: String,                                 // |
}                                    // --------------------+
pub struct CommandBuilder {}         // -----------------+
impl Command {                                        // |
    pub fn builder() -> CommandBuilder {              // 由过程宏生成的代码片段
        CommandBuilder {}                             // | 
    }                                                 // |
}                                    // -----------------+
fn main() {
    let builder = Command::builder();
    let _ = builder;
}
```

> 可以看出，派生式过程宏会保留用户的原始输入的代码，然后把我们返回的TokenStream追加在用户原始代码的下方。它的行为和属性式过程宏是不一样的，属性式过程宏允许你修改用户的原始代码。所以，这里可以稍微纠正一下我们之前对于过程宏的描述，之前我们一直说，过程宏是把用户输入的代码做一顿处理之后，把加工处理后的代码给到编译器继续编译，但实际上，更准确的说法应该是，依照用户输入的代码信息，修改用户原始代码，`或生成全新的代码`，把修改后或`新生成`的代码返回给编译器继续编译。不同类型的过程宏赋予你操作用户代码的范围是不一样的。


好了，开始下一步操作。我们已经生成出了新的结构体定义，但是结构体中的字段还没有定义出来，下面我们要开始遍历原始输入结构体的各个字段，获取这些字段的信息。

我们现在手头的输入信息只有一个类型为`syn::DeriveInput`类型的语法树节点，看起来所有的信息系都要从它里面挖掘了，为了获取输入结构体的信息，我们先看看`syn::DeriveInput`这个语法树节点是怎么定义的，它的定义如下：
```rust
    pub struct DeriveInput {
        /// Attributes tagged on the whole struct or enum.
        pub attrs: Vec<Attribute>,

        /// Visibility of the struct or enum.
        pub vis: Visibility,

        /// Name of the struct or enum.
        pub ident: Ident,

        /// Generics required to complete the definition.
        pub generics: Generics,

        /// Data within the struct or enum.
        pub data: Data,
    }
```
首先尝试自己看一下`syn::DeriveInput`的各个字段，然后尝试把他和结构体的定义联系起来。假设我们有下面这样一个结构定义,那么这里的各个语法元素和`syn::DeriveInput`中各个字段的映射关系可以用下图来表示
```rust
#[derive(Builder)]
#[blog.ideawand.com]
#[geek_kindergarten]
pub struct Foo <T> {
    foo: T,
    bar: i64,
}
```

![deriveInput类型和结构体的对应关系](./image/derive_input_mapping.png)


> 划重点： 通过上面的例子，要建立起一种认知，rust中各个语法元素，都可以被不同的数据结构(也就是`syn`包提供的语法树节点类型)来表示。我们上面的例子是带领大家分析了`syn::DeriveInput`结构和struct定义之间的对应关系，后续我们还会遇到各种各样的语法元素，我们不能一一给大家作图展示对应关系，这就要求大家能够自己阅读`syn`包的文档，并逐步掌握rust语言中常见语法元素与`syn`包中对应的各种数据结构。方法都是一样的，并不困难，学会多看文档。

> 从上面`DeriveInput`结构源代码的注释中，我们可以看到很多地方都写了`struct or enum`，也就是说，派生样式的过程宏不仅可以用在`struct`上，也可以用在`enum`上


知道了语法树节点的结构，接下来我们就可以开始从中抽取数据了。我们现在要处理的输入结构比较简单，`attrs`和`generics`暂时都用不到，看来只要搞清楚`data`字段里的结构就行了，但是`data`节点里有什么呢？大家需要继续去继续阅读`syn::Data`数据类型的源码，一层一层去查看语法树中各种节点之间是怎样嵌套的，但语法树的嵌套层级非常灵活，你极有可能不知道写出的一段代码究竟会被解析成什么样的语法树，因此，我们可以回忆一下上一篇文章中的重点知识：

> * 遇到不熟悉的语法树节点，就打印出来看看，回忆一下我们在上一篇文章中提到的重点内容：`print大法是开发调试rust过程宏的利器`
> * print大法和阅读文档，要交替使用！

为了看看我们的原始输入结构体被解析成了什么样的语法树，我们在上述代码的`do_expand`函数开头加入一行打印语句如下所示：
```rust
fn do_expand(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    eprintln!("{:#?}", st.data);
    // 其他代码保持不变，省略。。。
}
```
然后依然在`proc-macro-workshop`目录下,这次执行`cargo check`命令（这个命令在上一篇文章中介绍过），因为我们不需要打印过程宏展开的代码，我们只是想通过`check`命令触发宏展开的过程，执行我们的调试打印代码，这次应该会在终端中看到很长的输出，我把关键结构放在这里,删掉了一些暂时不用的字段：
```rust
Struct(
    DataStruct {
        fields: Named(
            FieldsNamed {
                named: [
                    Field {
                        ident: Some(
                            Ident {
                                ident: "executable",
                                span: #0 bytes(1388..1398),
                            },
                        ),
                        ty: Path(
                           // ...
                        ),
                    },
                    Comma,
                    // 下面开始第二个字段的定义了，我们省略不写了
                ],
            },
        ),
        semi_token: None,
    },
)
```
阅读上面的代码，我们可以看到`DeriveInput`的`data`字段是一个枚举类型，从`data`字段开始，到我们想要得到的字段列表，也就是上面结构中`Field`结构体构成的列表，中间还间隔了很多层不同的语法树节点，因此我们的下一步就是通过一顿模式匹配操作，拿到结构体字段的列表。

为了代码结构清晰，我们将其定义为一个独立的函数吧，像下面这样：

```rust
type StructFields = syn::punctuated::Punctuated<syn::Field,syn::Token!(,)>;

fn get_fields_from_derive_input(d: &syn::DeriveInput) -> syn::Result<&StructFields> {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = d.data{
        return Ok(named)
    }
    Err(syn::Error::new_spanned(d, "Must define on a Struct, not Enum".to_string()))
}
```
在上面这段代码中，语法树各个层级的嵌套关系、枚举类到底是哪一个成员等信息，是通过print大法得到的，而解析后数据类型的泛型参数，则是通过翻`syn`包的文档或者代码得到的，这里大家要Get的重点不是代码现在写成的这个样子，而是怎样去综合使用各种手段来找到解析语法树各式各样节点的方法，注意这里是`授之以渔`，不是`授之以鱼`。说实话，我在完成这些挑战的时候，有相当多的时间是在翻文档和打印调试信息，从而捋清楚如何操作这些语法树节点。如果大家观看视频版本，就可以看到我是如何一层层阅读文档源码、找出类型定义的过程了。

关于错误信息，我们使用了`syn::Error::new_spanned()`方法，其第一个参数用于指定错误对应的span信息，也就是编译器在打印错误信息时，要显示用户源代码中的哪一段上下文，我们在这里直接将输入的原始信息放在了这里，这样报错的时候，就会显示出有问题的结构体定义了。我们这里出于演示目的，如果上面的模式匹配和解构流程失败，就返回一个"该过程宏只能作用在结构体上`的错误，如果做得完善一些，上面的代码中其实有两个枚举类型的匹配，应该分别返回不同的错误信息，有兴趣的同学可以完善一下

拿到输入结构体中的字段以后，就可以开始产生`Builder`结构体的字段定义了，其实产生字段定义的方法有很多种，rust过程宏的机制就是，你最后能拼出来有效的TokenStream就行，至于你是模块化的拼接，还是流水账式的拼接，都可以。我们只能给大家演示一种写法，大家如果自己写，可以随意发挥。我们这里要给大家顺带展示一下`quote!`宏里面重复展开的用法，代码如下：
```rust
fn generate_builder_struct_fields_def(fields: &StructFields) -> syn::Result<proc_macro2::TokenStream>{
    let idents:Vec<_> = fields.iter().map(|f| {&f.ident}).collect();
    let types:Vec<_> = fields.iter().map(|f| {&f.ty}).collect();

    let token_stream = quote!{
        #(#idents: std::option::Option<#types>),*
    };
    Ok(token_stream)
}
```

上述代码中：
* 首先通过两次迭代输入的字段列表，分别得到`idents`和`types`这两个列表变量，由于迭代过程是保序的，所以这两个列表的长度是相等的，并且列表中相同下标位置的元素恰好是`(标识符，类型)`的一对儿数据。
  * `types`里面的每一个元素都是一个`&syn::Type`类型的枚举，里面还有很深的嵌套层级，但我们现在可以把它当做一个整体（也就是一个语法树节点）来用，没必要去解析里面。
* 在`quote!`宏中，可以使用`#(#var1 #var2 ... #varN ),*`的形式，来把一个列表中的内容展开成一组类似的代码，模板语言的详细说明请大家参阅官方文档
* `quote!`宏返回的是一个`proc_macro2::TokenStream`类型数据，`quote!`宏里面可以再次加入其它`quote!`宏产生的结果，这个用法我们马上就会看到。

> 划重点： 
> * 上述代码中用到了非常常见的`Option`枚举类型，但这里使用了`std::option::Option`这种完整路径的写法。这是由于你无法预料到过程宏在什么场景下被使用，有可能过程宏展开的上下文中，用户定义了其他的`Option`类型，因此，为了保证过程宏的稳定性，请大家尽量使用绝对路径。
> * 如果你在看B站视频，请一键三连，或者，至少点个赞吧~，如果你在看文字版，请关注我的微信公众号：极客幼稚园

上述代码在第二关的测试用例中执行后，预期会生成如下的代码片段,注意这四行代码周围并没有括号等其他语法元素的包围，它们仅仅是一些片段，后面我们要把这些片段插入到其他片段中，来组成更加复杂的代码：
```rust
    executable: std::option::Option<String>,
    args: std::option::Option<Vec<String>>,
    env: std::option::Option<Vec<String>>,
    current_dir: std::option::Option<String>,
```


准备好上述两个函数以后，我们可以修改一下之前的`do_expand`函数了，修改后的函数如下所示，新增代码在注释中标出：
```rust
fn do_expand(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_literal = st.ident.to_string();
    let builder_name_literal = format!("{}Builder", struct_name_literal);
    let builder_name_ident = syn::Ident::new(&builder_name_literal, st.span());

    let struct_ident = &st.ident;

    // 以下两行代码是新增的，调用上述新定义的两个函数
    let fields = get_fields_from_derive_input(st)?;
    let builder_struct_fields_def = generate_builder_struct_fields_def(fields)?;

    let ret = quote! {
        pub struct #builder_name_ident {
            // 下面这行代码是增增的，注意这里的用法：
            // 在当前这个`quote!`宏中，引用了其他`quote!`宏返回的结果
            // 在这里把不同的代码碎片拼接起来，就像搭积木一样
            #builder_struct_fields_def
        }
        impl #struct_ident {
            pub fn builder() -> #builder_name_ident {
                #builder_name_ident{
                    // TODO 后面会再写一个函数，生成这里需要的代码片段
                }
            }
        }
    };

    return Ok(ret);
}
```

修改完代码后，我们再在`proc-macro-workshop`目录下执行以下`cargo expand`，查看一下现在过程宏展开得到的代码，应该是下面这个样子,重点关注一下我们拼接出的`CommandBuilder`结构体的成员定义部分：
```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use derive_builder::Builder;
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: String,
}
struct CommandBuilder {
    executable: std::option::Option<String>,
    args: std::option::Option<Vec<String>>,
    env: std::option::Option<Vec<String>>,
    current_dir: std::option::Option<String>,
}
impl Command {
    pub fn builder() -> CommandBuilder {
        CommandBuilder {}
    }
}
fn main() {
    let builder = Command::builder();
    let _ = builder;
}
```

让我们来继续产生其他代码片段，我们现在需要生成`builder()`方法中，结构体初始化的相关代码，最简单的做法就是把上面的代码简单修改一下即可，但我们前面提到过，rust过程宏的实现方法非常灵活，只要你能拼出正确的TokenStream就行，所以，这里我们采用了另外一种写法，下面的函数返回的是一个由TokenStream组成的列表，重复展开的工作留到`do_expand`函数：
```rust
fn generate_builder_struct_factory_init_clauses(fields: &StructFields) -> syn::Result<Vec<proc_macro2::TokenStream>>{
    let init_clauses: Vec<_> = fields.iter().map(|f| {
        let ident = &f.ident;
        quote!{
            #ident: std::option::Option::None
        }
    }).collect();

    Ok(init_clauses)
}
```
相应的，我们修改一下`do_expand`函数，改动部分用注释标出：
```rust
fn do_expand(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_literal = st.ident.to_string();
    let builder_name_literal = format!("{}Builder", struct_name_literal);
    let builder_name_ident = syn::Ident::new(&builder_name_literal, st.span());

    let struct_ident = &st.ident;

    let fields = get_fields_from_derive_input(st)?;
    let builder_struct_fields_def = generate_builder_struct_fields_def(fields)?;
    // 下面这一行是新加的 
    let builder_struct_factory_init_clauses = generate_builder_struct_factory_init_clauses(fields)?;

    let ret = quote! {
        pub struct #builder_name_ident {
            #builder_struct_fields_def
        }
        impl #struct_ident {
            pub fn builder() -> #builder_name_ident {
                #builder_name_ident{
                    // 下面这一行是新加的，注意我们在这里重复展开了每一个字段
                    #(#builder_struct_factory_init_clauses),*
                }
            }
        }
    };

    return Ok(ret);
}
```

接下来大家可以再运行一下`cargo expand`来观察一下过程宏展开的结果，我们就不再列出来了。现在，我们看看第二关能不能通过了，确保`proc-macro-workshop/builder/tests/progress.rs`文件中对第二关的注释已经去掉,然后进入到`proc-macro-workshop/builder`目录下，执行`cargo test`，不出意外的话，第二关应该顺利通过！

### 第三关
第三关视频版本：
<iframe src="//player.bilibili.com/player.html?aid=290008560&bvid=BV1Sf4y1W7He&cid=322455034&page=1" scrolling="no" border="0" frameborder="no" framespacing="0" allowfullscreen="true"> </iframe>

请阅读`proc-macro-workshop/builder/tests/03-call-setters.rs`文件中的说明,主要就是要产生一组类似下面这种形式的代码，我们要把其中的`executable`和`String`全部动态替换掉：
```rust
fn executable(&mut self, executable: String) -> &mut Self {
    self.executable = Some(executable);
    self
}
```

这一关要实现的功能与第二关几乎完全一样，因此我就不再做其他解释了。大家可以先尝试自己编码之后，再看看我下面给出的一个参考实现。

在这个参考实现中，我再给大家展示一种代码片段拼接的方式，即直接通过`proc_macro2::TokenStream`类型提供的`extend`方法，把多个TokenStream串接在一起，就像字符串拼接一样：

```rust
fn generate_setter_functions(fields: &StructFields) -> syn::Result<proc_macro2::TokenStream>{
    let idents:Vec<_> = fields.iter().map(|f| {&f.ident}).collect();
    let types:Vec<_> = fields.iter().map(|f| {&f.ty}).collect();

    // 创建一个空的TokenStream
    let mut final_tokenstream = proc_macro2::TokenStream::new();

    for (ident, type_) in idents.iter().zip(types.iter()) {
        let tokenstream_piece = quote!{
            fn #ident(&mut self, #ident: #type_) -> &mut Self {
                self.#ident = std::option::Option::Some(#ident);
                self
            }
        };
        // 不断追加新的TokenStream片段到一个公共的TokenStream上
        final_tokenstream.extend(tokenstream_piece);
    }

    Ok(final_tokenstream)
}
```

```rust
fn do_expand(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_literal = st.ident.to_string();
    let builder_name_literal = format!("{}Builder", struct_name_literal);
    let builder_name_ident = syn::Ident::new(&builder_name_literal, st.span());

    let struct_ident = &st.ident;

    let fields = get_fields_from_derive_input(st)?;
    let builder_struct_fields_def = generate_builder_struct_fields_def(fields)?;
    let builder_struct_factory_init_clauses = generate_builder_struct_factory_init_clauses(fields)?;

    // 下面这一行是第三关新加的
    let setter_functions = generate_setter_functions(fields)?;

    let ret = quote! {
        pub struct #builder_name_ident {
            #builder_struct_fields_def
        }
        impl #struct_ident {
            pub fn builder() -> #builder_name_ident {
                #builder_name_ident{
                    #(#builder_struct_factory_init_clauses),*
                }
            }
        }

        // 下面这三行是第三关新加的
        impl #builder_name_ident {
            #setter_functions
        }
    };

    return Ok(ret);
}
```

### 第四关
第四、五关视频版本：
<iframe src="//player.bilibili.com/player.html?aid=205229090&bvid=BV1bh411S7zV&cid=325369691&page=1" scrolling="no" border="0" frameborder="no" framespacing="0" allowfullscreen="true"> </iframe>

这一关要生成的代码比之前的复杂了不少，需要由宏来产生出一些判断逻辑，错误信息等，但是换汤不换药，本质还是生成代码片段后进行拼接。大家可以在本关中继续尝试各种写法。

接下来我要给出的写法中，采用了先将各个片段放入一个Vec中，然后再展开的做法，这是因为这样写以后，对于后面关卡的实现会比较容易，大家完全可以尝试其他的方式来写。

产生`build()`方法代码片段的示例代码如下：
```rust
fn generate_build_function(fields: &StructFields, origin_struct_ident: &syn::Ident) -> syn::Result<proc_macro2::TokenStream>{
    let idents:Vec<_> = fields.iter().map(|f| {&f.ident}).collect();

    let mut checker_code_pieces = Vec::new();
    for idx in 0..idents.len() {
        let ident = idents[idx];
        checker_code_pieces.push(quote!{
            if self.#ident.is_none() {
                let err = format!("{} field missing", stringify!(#ident));
                return std::result::Result::Err(err.into())
            }
        });
    }

    let mut fill_result_clauses = Vec::new();
    for idx in 0..idents.len() {
        let ident = idents[idx];
        fill_result_clauses.push(quote!{
            #ident: self.#ident.clone().unwrap()
        });
    }


    let token_stream = quote!{
        pub fn build(&mut self) -> std::result::Result<#origin_struct_ident, std::boxed::Box<dyn std::error::Error>> {
            #(#checker_code_pieces)*
                               //  ^--注意，由于我们要重复的是一组if判断代码块，它们之间不需要用逗号分隔，所以这里的重复模式是`*`，而不是之前重复结构体字段时用到的`,*`
            let ret = #origin_struct_ident{
                #(#fill_result_clauses),*
            };
            std::result::Result::Ok(ret)
        }
    };
    Ok(token_stream)
}
```

对do_expand()的修改如下：
```rust
fn do_expand(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_literal = st.ident.to_string();
    let builder_name_literal = format!("{}Builder", struct_name_literal);
    let builder_name_ident = syn::Ident::new(&builder_name_literal, st.span());

    let struct_ident = &st.ident;

    let fields = get_fields_from_derive_input(st)?;
    let builder_struct_fields_def = generate_builder_struct_fields_def(fields)?;
    let builder_struct_factory_init_clauses = generate_builder_struct_factory_init_clauses(fields)?;
    let setter_functions = generate_setter_functions(fields)?;
    // 下面这一行是第四关新加的
    let generated_builder_functions = generate_build_function(fields,struct_ident)?;

    let ret = quote! {
        pub struct #builder_name_ident {
            #builder_struct_fields_def
        }
        impl #struct_ident {
            pub fn builder() -> #builder_name_ident {
                #builder_name_ident{
                    #(#builder_struct_factory_init_clauses),*
                }
            }
        }
        impl #builder_name_ident {
            #setter_functions
            // 下面这1行是第四关新加的
            #generated_builder_functions
        }
    };

    return Ok(ret);
}
```

### 第五关
看一下题目要求，好开心，我们的代码已经实现了第五关的所有功能，什么都不用做了~

### 第六关
第六关视频版本：
<iframe src="//player.bilibili.com/player.html?aid=545152453&bvid=BV1hi4y1A79p&cid=328346252&page=1" scrolling="no" border="0" frameborder="no" framespacing="0" allowfullscreen="true"> </iframe>

这一关的题目要求比较长，在其中介绍了一个rust过程宏机制的缺陷：rust的引用消解，或者说是符号解析，是在宏展开之后进行的，这也就是说，例如有如下代码：
```rust
use std::any::TypeId;

pub mod blog {
    pub mod ideawand {
        pub mod com {
            pub struct GeekKindergarten{}
        }
    }
}

use blog::ideawand::com::GeekKindergarten;
use blog::ideawand::com::GeekKindergarten as MyPersonalBlog;

fn main() {
    assert!(TypeId::of::<GeekKindergarten>() == TypeId::of::<blog::ideawand::com::GeekKindergarten>());
    assert!(TypeId::of::<MyPersonalBlog>() == TypeId::of::<blog::ideawand::com::GeekKindergarten>());
}
```
正常来说，上面代码中的`GeekKindergarten`类型，可以用完整的路径名来表示，也可以通过`use`关键字将其导入到当前上下文中，使用短名字来表示,还可以通过`as`关键字为其重命名，但它们都是同一个类型；然而在过程宏中，我们看到的TokenStream也好，还是TokenStream解析出的`syn`包中定义的语法树节点类型也好，它们都还没有经过引用消解，所以在rust的过程宏中，`GeekKindergarten`和`blog::ideawand::com::GeekKindergarten`以及`MyPersonalBlog`是不同的，或者说，我们无法判断他们是不是相同的类型。

第六关的题目是说，我们要识别出用户给出的结构体中，类型为`Option`类型的字段，对于这些字段，我们允许他们为None，也就是说要放宽第四关中`builder`方法的校验规则。为了简化这个题目，题目要求我们只要识别`Option`类型即可，不考虑通过`as`关键词重命名导致的不可识别问题，也不考虑其他同名的`Option`的情况。同时，第六关的提示中给出了我们要匹配`Option`枚举和泛型时可能用到的模式，如下所示,我们要查看结构体的某个字段是不是满足这个这个模式：
```rust
Type::Path(             // ------------------------------------------- 这个节点表示形如 `std::option::Option<Vec<String>>`这个整体
    TypePath {  
        qself: None,
        path: Path {        // --------------------------------------- 到这个节点其实也还是表示 `std::option::Option<Vec<String>>`这个整体
            segments: [           // --------------------------------- 这个列表表示 [std, option, Option<Vec<String>>]  这三个独立的段
                PathSegment {           // --------------------------- 这个结构体表示每一段的具体内容，我们以最后一段`Option<Vec<String>>`为例
                    ident: "Option",           // -------------------- 这里表示Option这个标识符
                    arguments: PathArguments::AngleBracketed(       // 这个节点表示<Vec<String>>
                        AngleBracketedGenericArguments {
                            args: [                                 // 由于泛型参数可能是<T,U,V>等多个，所以这里是一个列表
                                GenericArgument::Type(              // 在上面的例子中，这个节点表示`Vec<String>`
                                    ...                             // 这里可以将`Vec<String>`按照上面的套路继续展开。。。
                                ),
                            ],
                        },
                    ),
                },
            ],
        },
    },
)
```

有了上面的模式参考，我们来写一个新的函数，用来识别上面的模式，如果一个type能匹配上面的模式，就把Option里面的泛型参数返回出来，否则返回一个None
```rust
fn get_optional_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(syn::TypePath { ref path, .. }) = ty {
        // 这里我们取segments的最后一节来判断是不是`Option<T>`，这样如果用户写的是`std:option:Option<T>`我们也能识别出最后的`Option<T>`
        if let Some(seg) = path.segments.last() {
            if seg.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    ref args,
                    ..
                }) = seg.arguments
                {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.first() {
                        return Some(inner_ty);
                    }
                }
            }
        }
    }
    None
}
```
在能够判断一个field是否为`Option`以后，我们还需要对之前的几个函数都做出相应的调整，因此想通过这一关，改动还是比较大的，主要的改动包括：
* 在创建builder对应的结构体时，`Option`类型字段要特殊处理，如果还按照原来的模板来生成，就会产生出`Option<Option<T>>`这种类型
* `Option<T>`类型字段的setter接受的类型应该是`T`，如果使用原来的模板，会拼接出来`Option<T>`
* `builder`方法对于`Option<T>`字段的校验规则和其他字段不同

下面我们来一个个修改，首先是修改`generate_builder_struct_fields_def()`这个函数，修改后如下：
```rust
fn generate_builder_struct_fields_def(
    fields: &StructFields,
) -> syn::Result<proc_macro2::TokenStream> {
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    // 第六关，对types 变量的构建逻辑进行了调整
    let types: Vec<_> = fields
        .iter()
        .map(|f| {
            // 针对是否为`Option`类型字段，产生不同的结果
            if let Some(inner_ty) = get_optional_inner_type(&f.ty) {
                quote!(std::option::Option<#inner_ty>)
            } else {
                let origin_ty = &f.ty;
                quote!(std::option::Option<#origin_ty>)
            }
        })
        .collect();

    let token_stream = quote! {
        // 下面这一行，也做了修改
        #(#idents: #types),*
    };
    Ok(token_stream)
}
```

然后修改`generate_setter_functions()`函数
```rust
fn generate_setter_functions(fields: &StructFields) -> syn::Result<proc_macro2::TokenStream> {
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    let mut final_tokenstream = proc_macro2::TokenStream::new();

    for (ident, type_) in idents.iter().zip(types.iter()) {
        let tokenstream_piece;
        // 第六关，对tokenstream_piece 变量的构建逻辑进行了调整
        if let Some(inner_ty) = get_optional_inner_type(type_) {
            tokenstream_piece = quote! {
                fn #ident(&mut self, #ident: #inner_ty) -> &mut Self {
                    self.#ident = std::option::Option::Some(#ident);
                    self
                }
            };
        } else {
            tokenstream_piece = quote! {
                fn #ident(&mut self, #ident: #type_) -> &mut Self {
                    self.#ident = std::option::Option::Some(#ident);
                    self
                }
            };
        }
        final_tokenstream.extend(tokenstream_piece);
    }

    Ok(final_tokenstream)
}
```
最后，我们对`generate_build_function()`函数进行修改：
```rust
fn generate_build_function(
    fields: &StructFields,
    origin_struct_ident: &syn::Ident,
) -> syn::Result<proc_macro2::TokenStream> {
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    // 下面这一行是第六关新加的，之前没用到type相关信息，就没写下面这一行
    let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    let mut checker_code_pieces = Vec::new();
    for idx in 0..idents.len() {
        let ident = idents[idx];
        // 第六关修改，只对不是`Option`类型的字段生成校验逻辑
        if get_optional_inner_type(&types[idx]).is_none() {
            checker_code_pieces.push(quote! {
                if self.#ident.is_none() {
                    let err = format!("{} field missing", stringify!(#ident));
                    return std::result::Result::Err(err.into())
                }
            });
        }
    }

    let mut fill_result_clauses = Vec::new();
    for idx in 0..idents.len() {
        let ident = idents[idx];
        // 这里需要区分`Option`类型字段和非`Option`类型字段
        if get_optional_inner_type(&types[idx]).is_none() {
            fill_result_clauses.push(quote! {
                #ident: self.#ident.clone().unwrap()
            });
        }else {
            fill_result_clauses.push(quote! {
                #ident: self.#ident.clone()
            });
        }
    }

    let token_stream = quote! {
        pub fn build(&mut self) -> std::result::Result<#origin_struct_ident, std::boxed::Box<dyn std::error::Error>> {
            #(#checker_code_pieces)*
            let ret = #origin_struct_ident{
                #(#fill_result_clauses),*
            };
            std::result::Result::Ok(ret)
        }
    };
    Ok(token_stream)
}
     
```

### 第七关
第七关视频版本：
<iframe src="//player.bilibili.com/player.html?aid=715287871&bvid=BV1eQ4y1Z7h9&cid=331167283&page=1" scrolling="no" border="0" frameborder="no" framespacing="0" allowfullscreen="true"> </iframe>

这一关会涉及到如何解析结构体中字段附加的属性信息(field attribute)，例如下面这个结构体
```rust
#[derive(Builder)]
struct ArticleAuthor {
    Blog String,
    #[geek_kindergarten(foo=bar)]
    IdeaWand String,
    Com String,
}
```
在这个结构体中，`#[geek_kindergarten(foo=bar)]`这一行就是附加在`IdeaWand`这个字段上的属性，属性的名字是`geek_kindergarten`，这里需要注意的一个重点是：
> * 在派生宏中，上面这样的属性称为`惰性属性`（inert attributes），惰性属性必须指定关联一个派生过程宏，限定这个惰性属性只能在这个过程宏内使用。
那么，如何向编译器注册一个惰性属性的名字呢？在第七关的指导文档里有说明，简单来说，就是要在`#[proc_macro_derive(Builder)]`这个标签中添加上属性的名字，加入我们要加入一个名为`geek_kindergarten`的属性，那么就要这样写：`#[proc_macro_derive(Builder, attributes(geek_kindergarten))]`

接下来的操作，和前几关大致一样，我们先来把要做的事情列一下：
* 解析每个field的`attrs`字段，它是一个`Vec<Attribute>`类型，`Attribute`是一个我们前面没遇到过的语法树节点，我们的处理思路还是一样的，需要通过文档 + Print大法来搞清楚里面的结构
* 第六关中我们识别了`Option<T>`这种模式的代码，本关我们要识别`Vec<T>`，稍微调整一下，复用之前代码即可
* 针对是不是`Vec<T>`类型,以及是不是有属性标签，我们要用不同的模板来产生代码

下面，我们先来看看怎么解析`Attribute`类型的语法树节点，由于这个属性标签是一个挺通用的解析需求，而且还比较灵活复杂，所以我们就多花点时间来介绍一下。为了探索他的行为，我们创建另一个极简版本的派生宏，专门用来打印`Attribute`结构的信息。同时，`Attribute`不一定是要写在结构体字段上的，我们也可以用一个属性来装饰整个结构体，所以`DeriveInput`语法树节点中就有`attrs`属性，因此我在接下来的小实验中，就不再去层层解析寻找字段上的属性了，直接用结构体的属性标签来做实验，效果都是一样的。下面，我们来搭建这个小的实验环境:
* 在`proc-macro-workshop/builder/src/lib.rs`中，添加如下代码：
    ```rust
    #[proc_macro_derive(ExploreAttribute)]
    pub fn attribute_explore(input: TokenStream) -> TokenStream {
        let st = syn::parse_macro_input!(input as syn::DeriveInput);
        let attr = st.attrs.first().unwrap();
        eprintln!("{:#?}", attr);
        proc_macro2::TokenStream::new().into()
    }
    ```
* 将`proc-macro-workshop/main.rs`中的内容清空，替换为下面的代码：
    ```rust
    use derive_builder::ExploreAttribute;

    #[derive(ExploreAttribute)]
    #[blog::ideawand::com(Bar)]
    pub struct Foo{}
    ```
在`proc-macro-workshop`目录下执行`cargo check`,观察输出结果，对于上面的例子，我们得到的输出为：
```rust
Attribute {
    pound_token: Pound,
    style: Outer,
    bracket_token: Bracket,
    path: Path {
        leading_colon: None,
        segments: [
            PathSegment {
                ident: Ident {
                    ident: "blog",
                    span: #0 bytes(1383..1387),
                },
                arguments: None,
            },
            Colon2,
            PathSegment {
                ident: Ident {
                    ident: "ideawand",
                    span: #0 bytes(1389..1397),
                },
                arguments: None,
            },
            Colon2,
            PathSegment {
                ident: Ident {
                    ident: "com",
                    span: #0 bytes(1399..1402),
                },
                arguments: None,
            },
        ],
    },
    tokens: TokenStream [
        Group {
            delimiter: Parenthesis,
            stream: TokenStream [
                Ident {
                    ident: "Bar",
                    span: #0 bytes(1403..1406),
                },
            ],
            span: #0 bytes(1402..1407),
        },
    ],
}
```

简单解释一下，就是这样的：
```
#[blog::ideawand::com(Bar)]
^ ^^^^^^^^^^^^^^^^^^^ ^^^
|         |            |
|         |            +--这里对应的是`syn::Attribute`节点的`tokens`属性,它的类型是`proc_macro2::TokenStream`
|         +--这里对应的是`syn::Attribute`节点的`path`属性,它的类型是`syn::Path`,这个语法树节点我们在前面已经见到过了
+--这里对应`syn::Attribute`节点的`style`属性，`#`现在对应到的是`outer`,如果是`#!`对应处的就是`inner`
```

上面的`style`和`path`两个字段都没什么要说的，好玩并且复杂的事情是`tokens`这个属性，我们可以看到，这个属性的类型并没有在`syn`包中定义对应的语法树节点，这是为什么呢？查看官方文档，我们可以了解到其背后的原因是：
* Rust 属性的书写语法非常灵活，甚至可以不是有效的Rust标准语法，例如包含用户自定义的语法规则，因此，`syn`包没法通过有限的语法树节点类型来表示它的内容
* 之所以要为`syn::Attribute`节点设置一个`path`部分，是可以把`path`作为一个类型提示，过程宏或者编译器可以根据`path`来决定如何解析后面的`token`部分
* 如果在你的自定义属性中，要求属性是符合标准Rust语法的，那么`syn::Attribute`节点提供了一个`parse_meta()`方法，可以将整个属性（也就是`path`和`token`两部分）中的内容解析为`syn::Meta`枚举类型，而这个枚举类型的内容又十分丰富。

`syn::Meta`枚举各个成员的含义：
* `syn::Meta::Path`表示一个路径，下面这些形式的都可以认为是路径：
  * `#[A]`中的`A`,它只有一个小节，没有`::`分割的多个小节，但它也是一个Path
  * `#[A::B::C]`中的`A::B::C`
* `syn::Meta::List`表示一个列表，这个列表必须由一个前置路径和一个括号标记组成，括号里面通过逗号分隔为多个条目，组成一个列表，列表中的每一个条目又是一个`syn::Meta`枚举
  * `#[Foo(AAA,BBB,CCC)]`中的`Foo`是前置路径，后面的`AAA`、`BBB`、`CCC`就是三个列表项，它们三个都是`syn::Meta`类型的枚举
    * 从形式上可以看出，这三个枚举类型实际上存放的又是三个`syn::Path`类型的数据
  * `#[Foo(AAA,BBB(CCC,DDD))]` 这种形式，演示了列表中每个项目是可以嵌套的，其中的`BBB`是嵌套内层`syn::Meta::List`的前置路径，而`CCC`和`DDD`是内层的列表项
* `syn::Meta::NameValue`表示一个kv对，其中key部分是一个`syn::Path`,而value部分是一个字符串字面量
  * `#[xxx = "yyy"]`就是一个典型的例子

通过上面的嵌套规则可以看出，Rust允许你书写的属性规则是非常灵活的，例如你可以书写这样的一个属性，你可以尝试解释一下下面这个属性的嵌套层级：
```
#[Foo::Bar(A1::A2(B1::B2="C",D="E",F1::F2,G,H(I,J)))]
```


大家可以通过修改`proc-macro-workshop/main.rs`中的代码来尝试各种不同形式的Attribute是如何被解析为`syn::Meta`类型的枚举的，当然，需要修改一下`proc-macro-workshop/builder/src/lib.rs`的测试派生宏的定义代码如下：
```rust
#[proc_macro_derive(ExploreAttribute)]
pub fn attribute_explore(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as syn::DeriveInput);
    let attr = st.attrs.first().unwrap();
    let meta = attr.parse_meta(); // 解析为`sny::Meta`对象
    eprintln!("{:#?}", meta);
    proc_macro2::TokenStream::new().into()
}
```

现在我们回归正题，看一下第七关的题目，我们要解析形如`#[builder(each = "arg")]`的惰性属性，分解一下，首先这是一个`syn::Meta::List`枚举成员，它的前置路径是`builder`，后面的列表里有一个`syn::Meta::NameValue`枚举成员，在这个kv对中，`each`是固定的，而后面的值，我们要把它当做一个ident来使用，拼接到生成的代码中。开始撸代码：
```rust
fn get_user_specified_ident_for_vec(field: &syn::Field) -> Option<syn::Ident> {
    for attr in &field.attrs {
        if let Ok(syn::Meta::List(syn::MetaList {
            ref path,
            ref nested,
            ..
        })) = attr.parse_meta()
        {
            if let Some(p) = path.segments.first() {
                if p.ident == "builder" {
                    if let Some(syn::NestedMeta::Meta(syn::Meta::NameValue(kv))) = nested.first() {
                        if kv.path.is_ident("each") {
                            if let syn::Lit::Str(ref ident_str) = kv.lit {
                                return Some(syn::Ident::new(
                                    ident_str.value().as_str(),
                                    attr.span(),
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
    None
}
```

解析Attribute的工作完成了，但为了使用上面的函数我们还得费一番脑筋，做出不少修改，让我们来仔细看看第七关题目给出的要求：
* 我们需要兼容指定了each属性的字段和没有指定each属性的字段：
  * 如果指定了each属性，又分为两种情况：
    * 如果each指定的名字和原始字段名一致，则只产生出每次追加一个值的setter函数
    * 如果each指定的名字和原始字段名不一致，则需要产生两个版本的函数：
      * 与原始字段名同名的函数用于接收一个完整列表
      * 用户指定名称的setter，每次只接收一个条目
    * Vec字段和Option字段一样，不需要做为空的检查，对于Vec字段，我们要将其自动初始化
  * 如果没指定each属性，则可以把这个字段看做一个普通类型对待，前六关的测试用例还得通过才行
  
* 为了实现上面的需求，我们之前每一个函数几乎都要修改。在前四关中，我给出了三种拼接`TokenStream`的方法，在这次大改造中你会发现：
  * 对于一些简单的拼接，使用迭代器和`quote!`宏的重复展开功能，几行简短的代码就可以优雅地实现拼接（第一关和第二关的写法），但对于复杂的需求，写起来就会很别扭
  * 对于有比较复杂逻辑的生成业务，自己维护一个`Vec<TokenStream>`数组，或者自己去extend一个`TokenStream`会更加灵活顺手（也就是第三关和第四关的写法）


有了上面的大方向，我们开始继续撸代码。

第一个修改点是，因为我们这次不仅要获取`Option<T>`的内部类型，还有处理`Vec<T>`,所以我们来修改一下之前的`get_optional_inner_type()`函数，使其能够被复用，我们将其重命名为`get_generic_inner_type()`,新的定义如下，修改完成后，需要调整一下之前调用它的代码：
```rust
fn get_generic_inner_type<'a>(ty: &'a syn::Type, outer_ident_name: &str) -> Option<&'a syn::Type> {
    if let syn::Type::Path(syn::TypePath { ref path, .. }) = ty {
        // 这里我们取segments的最后一节来判断是不是`T<U>`，这样如果用户写的是`foo:bar::T<U>`我们也能识别出最后的`T<U>`
        if let Some(seg) = path.segments.last() {
            if seg.ident == outer_ident_name {
                if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    ref args,
                    ..
                }) = seg.arguments
                {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.first() {
                        return Some(inner_ty);
                    }
                }
            }
        }
    }
    None
}
```

接下来一个个调整之前编写的函数，首先是`generate_builder_struct_fields_def()`函数，修改点在注释中：
```rust
fn generate_builder_struct_fields_def(
    fields: &StructFields,
) -> syn::Result<proc_macro2::TokenStream> {
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let types: Vec<_> = fields
        .iter()
        .map(|f| {
            if let Some(inner_ty) = get_generic_inner_type(&f.ty,"Option") {
                quote!(std::option::Option<#inner_ty>)
            // 以下三行是第七关新加入的
            } else if get_user_specified_ident_for_vec(f).is_some() {
                let origin_ty = &f.ty;
                quote!(#origin_ty)  // 题目中设定，如果用户指定了each属性，我们就可以认为它一定是作用在一个Vec字段上

            } else {
                let origin_ty = &f.ty;
                quote!(std::option::Option<#origin_ty>)
            }
        })
        .collect();

    let token_stream = quote! {
        #(#idents: #types),*
    };
    Ok(token_stream)
}
```

然后是初始化函数：
```rust
fn generate_builder_struct_factory_init_clauses(fields: &StructFields) -> syn::Result<Vec<proc_macro2::TokenStream>>{
    let init_clauses: Vec<_> = fields.iter().map(|f| {
        let ident = &f.ident;
        // 下面这个if分支是第七关加入的，在第六关的时候只有else分支里的代码
        if get_user_specified_ident_for_vec(f).is_some() {
            quote!{
                #ident: std::vec::Vec::new()  //指定了each属性的Vec需要初始化
            }
        } else {
            quote!{
                #ident: std::option::Option::None
            }
        }
    }).collect();

    Ok(init_clauses)
}
```

然后是setter的生成：
```rust
fn generate_setter_functions(fields: &StructFields) -> syn::Result<proc_macro2::TokenStream> {
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    let mut final_tokenstream = proc_macro2::TokenStream::new();

    // 第七关修改，这里之前用了zip串联了两个迭代器，现在需要拿到对应的原始field，所以又加了一层`enumerate()`迭代器
    // 这里写成 for idx in 0..fields.len() {let ident = &fields[idx].ident; let type_ = &fields[idx].ty;...} 这种写法或许更优雅一些
    for (idx,(ident, type_)) in idents.iter().zip(types.iter()).enumerate() {
        let mut tokenstream_piece;
        if let Some(inner_ty) = get_generic_inner_type(type_,"Option") {
            tokenstream_piece = quote! {
                fn #ident(&mut self, #ident: #inner_ty) -> &mut Self {
                    self.#ident = std::option::Option::Some(#ident);
                    self
                }
            };
        
        // 下面这个分支是第七关加入的
        } else if let Some(ref user_specified_ident) = get_user_specified_ident_for_vec(&fields[idx]) {
            let inner_ty = get_generic_inner_type(type_,"Vec").ok_or(syn::Error::new(fields[idx].span(),"each field must be specified with Vec field"))?;
            tokenstream_piece = quote! {
                fn #user_specified_ident(&mut self, #user_specified_ident: #inner_ty) -> &mut Self {
                    self.#ident.push(#user_specified_ident);
                    self
                }
            };
            // 如果用户指定的setter名字和原始字段的名字不一样，那么产生另一个setter，这个setter是一次性传入一个列表的
            if user_specified_ident != ident.as_ref().unwrap() {
                tokenstream_piece.extend(
                    quote! {
                        fn #ident(&mut self, #ident: #type_) -> &mut Self {
                            self.#ident = #ident.clone();
                            self
                        }
                    }
                );
            }
        } else {
            tokenstream_piece = quote! {
                fn #ident(&mut self, #ident: #type_) -> &mut Self {
                    self.#ident = std::option::Option::Some(#ident);
                    self
                }
            };
        }
        final_tokenstream.extend(tokenstream_piece);
    }

    Ok(final_tokenstream)
}
```

最后是我们的`build()`方法：
```rust
fn generate_build_function(
    fields: &StructFields,
    origin_struct_ident: &syn::Ident,
) -> syn::Result<proc_macro2::TokenStream> {
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    let mut checker_code_pieces = Vec::new();
    for idx in 0..idents.len() {
        let ident = idents[idx];
        // 第七关修改，只对不是`Option`类型且没有指定each属性的字段生成校验逻辑
        if get_generic_inner_type(&types[idx],"Option").is_none() && get_user_specified_ident_for_vec(&fields[idx]).is_none() {
            checker_code_pieces.push(quote! {
                if self.#ident.is_none() {
                    let err = format!("{} field missing", stringify!(#ident));
                    return std::result::Result::Err(err.into())
                }
            });
        }
    }

    let mut fill_result_clauses = Vec::new();
    for idx in 0..idents.len() {
        let ident = idents[idx];
        // 第七关，这里需要判断是否有each属性。第一个分支是本关加入的。注意这里几个分支的先后判断顺序
        // 看我写在这里的代码可能没什么感觉，但如果是自己写的话，这几个分支的判断先后顺序是很重要的，否则可能生成出有问题的代码
        // 这里主要的问题是梳理清楚【是否有each属性】和【是否为Option类型】这两个条件的覆盖范围
        if get_user_specified_ident_for_vec(&fields[idx]).is_some() {
            fill_result_clauses.push(quote! {
                #ident: self.#ident.clone()
            });
        } else if get_generic_inner_type(&types[idx],"Option").is_none() {
            fill_result_clauses.push(quote! {
                #ident: self.#ident.clone().unwrap()
            });
        } else {
            fill_result_clauses.push(quote! {
                #ident: self.#ident.clone()
            });
        }
    }

    let token_stream = quote! {
        pub fn build(&mut self) -> std::result::Result<#origin_struct_ident, std::boxed::Box<dyn std::error::Error>> {
            #(#checker_code_pieces)*
            let ret = #origin_struct_ident{
                #(#fill_result_clauses),*
            };
            std::result::Result::Ok(ret)
        }
    };
    Ok(token_stream)
}
```
大功告成


### 第八关
第八、九关视频版本：
<iframe src="//player.bilibili.com/player.html?aid=247776475&bvid=BV1vv411L7XQ&cid=331683140&page=1" scrolling="no" border="0" frameborder="no" framespacing="0" allowfullscreen="true"> </iframe>

这一关是一个如何打印友好错误信息的挑战，完成这个挑战时，因为会拿编译器输出的结果做字符串匹配比较，所以我们要保证代码的干净，清除掉所有的未引入引用等Warning

首先把错误产生出来，我们需要修改`get_user_specified_ident_for_vec()`函数，让他返回一个`syn::Result`。
```rust
fn get_user_specified_ident_for_vec(field: &syn::Field) -> syn::Result<Option<syn::Ident>> {
    for attr in &field.attrs {
        if let Ok(syn::Meta::List(syn::MetaList {
            ref path,
            ref nested,
            ..
        })) = attr.parse_meta()
        {
            if let Some(p) = path.segments.first() {
                if p.ident == "builder" {
                    if let Some(syn::NestedMeta::Meta(syn::Meta::NameValue(kv))) = nested.first() {
                        if kv.path.is_ident("each") {
                            if let syn::Lit::Str(ref ident_str) = kv.lit {
                                return Ok(Some(syn::Ident::new(
                                    ident_str.value().as_str(),
                                    attr.span(),
                                )));
                            }
                        } else {
                            // 第八关加入，注意这里new_spanned函数的参数，我们需要在语法树中找到一个合适的节点来获取它的span，如果这个语法树节点找的不对，产生出的错误信息就会不一样
                            if let Ok(syn::Meta::List(ref list)) = attr.parse_meta() {
                                return Err(syn::Error::new_spanned(list, r#"expected `builder(each = "...")`"#))
                            }
                        }
                    }
                } 
            }
        }
    }
    Ok(None)
}
```
注意上面的代码，我们需要找到一个合适的语法树节点来产生错误的位置，例如，上面代码我们使用了语法树中的`syn::MetaList`节点，产生的报错信息是这样的：
```
error: expected `builder(each = "...")`
  --> $DIR/08-unrecognized-attribute.rs:22:7
   |
22 |     #[builder(eac = "arg")]
   |       ^^^^^^^^^^^^^^^^^^^^
```
而如果将上面`new_spanned`函数的第一个传入参数从`list`改为`attr`，即我们使用了`syn::Attribute`这个语法树节点的时候，给出的错误信息就会是这样的，注意其中`^`符号数量和位置的变化：
```
error: expected `builder(each = "...")`
  --> $DIR/08-unrecognized-attribute.rs:22:5
   |
22 |     #[builder(eac = "arg")]
   |     ^^^^^^^^^^^^^^^^^^^^^^^
```

可以返回Result以后，对这个函数的调用部分也需要改一下，对于普通的调用，我们直接在后面加一个`?`就好了，但是有两个函数是在迭代器map的闭包里调用了，这样改起来就有些麻烦了，这也再一次印证了上面的观点：对于复杂的过程宏，少用迭代器的map方法，自己构建循环展开可控性更好！

首先是`generate_builder_struct_fields_def()`函数，可以参考注释：
```rust
fn generate_builder_struct_fields_def(
    fields: &StructFields,
) -> syn::Result<proc_macro2::TokenStream> {
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    // 第八关修改，从这里又可以看出，对于复杂的过程宏，采用迭代器比较麻烦，返回一个错误要费一些周折
    // 这里修改了map中闭包的返回值，使其返回一个syn::Result<T>
    let types: syn::Result<Vec<proc_macro2::TokenStream>> = fields
        .iter()
        .map(|f| {
            if let Some(inner_ty) = get_generic_inner_type(&f.ty,"Option") {
                Ok(quote!(std::option::Option<#inner_ty>))
            } else if get_user_specified_ident_for_vec(f)?.is_some() {
                let origin_ty = &f.ty;
                Ok(quote!(#origin_ty)) 

            } else {
                let origin_ty = &f.ty;
               Ok(quote!(std::option::Option<#origin_ty>))
            }
        })
        .collect();
    
    let types = types?;
    let token_stream = quote! {
        #(#idents: #types),*
    };
    Ok(token_stream)
}
```

然后是`generate_builder_struct_factory_init_clauses()`函数:
```rust
fn generate_builder_struct_factory_init_clauses(fields: &StructFields) -> syn::Result<Vec<proc_macro2::TokenStream>>{
    // 第八关修改，从闭包中返回错误信息
    let init_clauses: syn::Result<Vec<proc_macro2::TokenStream>> = fields.iter().map(|f| {
        let ident = &f.ident;
        if get_user_specified_ident_for_vec(f)?.is_some() {
            Ok(quote!{
                #ident: std::vec::Vec::new()  
            })
        } else {
            Ok(quote!{
                #ident: std::option::Option::None
            })
        }
    }).collect();

    Ok(init_clauses?)
}
```

终于，大功告成！

### 第九关
哈哈哈，由于我们之前已经严格遵守了规定，第九关自然就通过了。不过，关于第九关的知识点，我还是要给大家再明确一下：
* 我们说的要使用绝对路径，是在生成的代码中要使用绝对路径
* 过程宏本身的代码，也就是我们用来生成代码的代码，里面还是可以直接使用短名字的
* 过程宏代码，和过程宏生成的代码，编译和运行在两个完全独立的时间以及空间上，请大家一定要想清楚~

---
作者简介：

米明恒，高级Golang、Python开发工程师，初级架构师，业余无线电爱好者（呼号BG2CCF），Rust业余爱好者。业余时间各种不务正业，研究奇怪的东西。

* 个人博客： http://blog.ideawand.com
* 微信公众号： 【极客幼稚园】