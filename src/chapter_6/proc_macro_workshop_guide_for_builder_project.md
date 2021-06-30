# Rust过程宏系列教程 | Proc Macro Workshop 之 Debug 

作者：米明恒 / 后期编辑：张汉东

> [本文来自于 blog.ideawand.com 投稿 ](https://blog.ideawand.com/2021/06/26/rust_procedural_macro/rust_proc_marco_workshop_guide-03/)

---

本系列的上一篇文章中，我们实战了`proc_macro_workshop`项目的`builder`题目。并且介绍了`proc_macro_workshop`这个项目的概况，如果您是第一次阅读本系列文章，对`proc_macro_workshop`项目的结构还是不很熟悉的话，可以先阅读一下上一篇文章。

好了，不废话了，准备好一台电脑，开始我们的第二个挑战任务`debug`

<!--more-->

首先打开`proc_macro_workshop`项目的`readme.md`文件，看一下`debug`这个项目要实现什么样的功能。根据其中的描述，这个题目的最终目标是实现一个可以输出指定格式调试信息的派生宏，他要实现的功能和rust自带的`Debug`派生宏是一样的，只不过我们要实现的这个宏比`Debug`更强大一些，可以指定每一个字段的输出格式。

我们之前提到过，Rust中的过程宏分为三种样式：派生样式的、属性样式的，还有函数样式的，上一篇和本篇要讨论的过程宏都是派生样式的，另外两种样式的过程宏会在后续文章中对另外三道题目讲解时介绍。如果你对派生样式的过程宏还不了解，请一定先阅读本系列的前一篇文章。本篇文章介绍的`debug`挑战题目，除了在大量使用上一篇`builder`项目使用的知识点之外，主要增加了对泛型的处理。

### 第一关
第一关视频版：[https://www.bilibili.com/video/BV1vU4y187TR?zw](https://www.bilibili.com/video/BV1vU4y187TR?zw)

第一关的工作和上一篇文章中介绍的`builder`题目的第一关一样，搭建一个框架，配置好`cargo.toml`，然后把输入的`TokenStream`转换为`syn::DeriveInput`类型即可。项目代码结构可以参考第一篇文章[Rust过程宏系列教程(2)--实现proc-macro-workshop项目之builder题目](./rust_proc_marco_workshop_guide-02.md)，回忆一下上篇文章提到的知识点，我们要实现一个类似下面这种结构的框架，便于我们做错处处理：

```rust
use proc_macro::TokenStream;
use syn::{self, spanned::Spanned};
use quote::{ToTokens, quote};

#[proc_macro_derive(CustomDebug)]
pub fn derive(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as syn::DeriveInput);
    match do_expand(&st) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn do_expand(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let ret = proc_macro2::TokenStream::new();
    return Ok(ret);
}
```


### 第二关

第二关视频版：[https://www.bilibili.com/video/BV1Kf4y1W7ss?zw](https://www.bilibili.com/video/BV1Kf4y1W7ss?zw)

第二关要实现基本的`Debug` Trait，其原理是使用rust标准库提供的`std::fmt::DebugStruct`结构来实现，例如对于下面这个结构体
```rust
struct GeekKindergarten {
    blog: String,
    ideawand: i32,
    com: bool,
}
```
我们要生成如下模式的代码,实现`Debug` Trait：
```rust
impl fmt::Debug for GeekKindergarten {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("GeekKindergarten")
           .field("blog", &self.blog)
           .field("ideawand", &self.ideawand)
           .field("com", &self.com)
           .finish()
    }
}
```
这样，我们就可以在`println!`中使用`{:?}`来打印结构体中的各个字段。
```rust
fn main() {
    let g = GeekKindergarten{blog:"foo".into(), ideawand:123, com:true};
    println!("{:?}", g);
}
```
所以，我们目标也很明确了，和上一篇的`builder`类似，我们要首先读取出被过程宏处理的结构体的每一个字段的名字，然后按照模板生成上面的代码即可，没有什么新的知识，所以我们直接给出代码即可：
```rust
fn do_expand(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let ret = generate_debug_trait(st)?;
    return Ok(ret);
}

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

fn generate_debug_trait(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let fields = get_fields_from_derive_input(st)?;
    let struct_name_ident = &st.ident;
    let struct_name_literal = struct_name_ident.to_string();

    let mut fmt_body_stream = proc_macro2::TokenStream::new();

    fmt_body_stream.extend(quote!(
        fmt.debug_struct(#struct_name_literal) // 注意这里引用的是一个字符串，不是一个syn::Ident，生成的代码会以字面量形式表示出来
    ));
    for field in fields.iter(){
        let field_name_idnet = field.ident.as_ref().unwrap();
        let field_name_literal = field_name_idnet.to_string();
        
        fmt_body_stream.extend(quote!(
            .field(#field_name_literal, &self.#field_name_idnet)  // 这行同样注意literal和ident的区别
        ));
    }

    fmt_body_stream.extend(quote!(
        .finish()
    ));

    let ret_stream = quote!(
        impl std::fmt::Debug for #struct_name_ident {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                #fmt_body_stream
            }
        }
    );

    return Ok(ret_stream)
}
```

### 第三关

第三关视频版：[https://www.bilibili.com/video/BV1df4y1a73x?zw](https://www.bilibili.com/video/BV1df4y1a73x?zw)

这一关要求我们可以为结构体中的每一个字段指定自己独立的格式化样式，也就是可以在结构体内部写下面这样的惰性属性标注：
```rust
#[derive(CustomDebug)]
struct GeekKindergarten {
    blog: String,
    #[debug = "0b{:32b}"]    // 在派生式过程宏中，这是个`惰性属性`，在上一篇文章中有介绍
    ideawand: i32,
    com: bool,
}
```

在上面代码中，`#[debug=xxx]`是我们编写的`CustomDebug`派生宏的惰性属性，派生宏惰性属性的概念在上一篇文章中介绍过了，因此我们可以参照一下前面文章中编写的提取惰性属性的函数，进行编写即可。一个提示是，在编写的过程中，可以继续使用print大法来分析属性的结构。
* 在上一篇文章中，我们处理的惰性属性是`#[builder(each=xxxx)]`的形式，这个写法在解析成语法树时是两层的嵌套结构，外面的`builder(xxxx)`转换成语法树节点的`MetaList`类型结构，而内部的`each=xxxx`转换成语法树节点的`NameValue`类型结构
* 本篇文章处理的，直接就是`#[debug = xxxx]`的形式，所以处理起来，其实比上一题的简单一些

在我们从惰性属性中拿到格式模板以后，接下来要做的就是使用`fmt`包中提供的相关方法，来格式化我们的结构，这部分只是与过程宏的开发无关，主要是`fmt`包中相关格式化工具的用法，参考资料在第三关的测试文件中也已经给出，我将其复制在下面，大家可以自行参阅：
* https://doc.rust-lang.org/std/macro.format_args.html

通过阅读上面的参考资料，我们可以了解到，想在`debug_struct`工具方法里指定输出的格式，我们就要借助`format_args!`宏，生成例如下面这样的代码：
```rust
// 原来的样子是：
// .field("ideawand", &self.ideawand)
// 现在的样子是：
.field("ideawand", &format_args!("0b{:32b}", self.ideawand))
```

有了上述的分析，我们可以直接给出第三关的核心代码，首先是提取字段惰性属性的函数：
```rust
fn get_custom_format_of_field(field: &syn::Field) -> syn::Result<Option<String>> {
    for attr in &field.attrs {
        if let Ok(syn::Meta::NameValue(syn::MetaNameValue {
            ref path,
            ref lit,
            ..
        })) = attr.parse_meta()
        {
            if path.is_ident("debug") {
                if let syn::Lit::Str(ref ident_str) =lit {
                    return Ok(Some(
                        ident_str.value()
                    ));
                }
            }
        }
    }
    Ok(None)
}
```

然后修改一下`generate_debug_trait()`函数，这里只给出了进行调整的核心循环体的代码片段
```rust
fn generate_debug_trait(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    
    // <此处省略未修改代码> ................

    for field in fields.iter(){
        let field_name_idnet = field.ident.as_ref().unwrap();
        let field_name_literal = field_name_idnet.to_string();
        
        // 以下若干行代码直到循环体结束 是第三关进行修改的部分
        let mut format_str = "{:?}".to_string();
        if let Some(format) = get_custom_format_of_field(field)? {
            format_str = format;
        } 
        // 这里是没有指定用户自定义的格式
        fmt_body_stream.extend(quote!(
            .field(#field_name_literal, &format_args!(#format_str, self.#field_name_idnet))
        ));

    }

    // <此处省略未修改代码> ................

}
```


### 第四关

第四关视频版：[https://www.bilibili.com/video/BV1S64y1d7fc?zw](https://www.bilibili.com/video/BV1S64y1d7fc?zw)

从这一关开始，我们接触一下泛型参数的处理，有点小激动~~

在前几关中我们生成的`Debug` Trait的代码是没有泛型参数的，因此，对于形如下面的带有泛型参数的结构体：
```rust
struct GeekKindergarten<T> {
    blog: T,
    ideawand: i32,
    com: bool,
}
```
我们生成的代码应该是形如
```rust
impl<T> Debug for GeekKindergarten<T> {
    // .....
}
```
但由于我们在代码模板中只是用了结构体的标识符，也就是`GeekKindergarten`这一部分，而没有使用泛型参数信息，也就是丢掉了`<T>`这一部分，因此我们生成的代码会是下面这个样子：
```rust
impl Debug for GeekKindergarten {
//  ^---- 这里丢掉了泛型参数 -----^
}
```

第四关的提示文档里，给出了泛型参数语法树节点的链接：
* https://docs.rs/syn/1.0/syn/struct.Generics.html
  
这个语法树节点提供了一个工具函数，可以帮助我们把泛型参数切分成三个用于生成`impl`块的片段，这个函数是：
* https://docs.rs/syn/1.0/syn/struct.Generics.html#method.split_for_impl

此外，他还给出了另一个示例程序的代码库地址，里面演示了如何处理泛型参数，推荐大家去看一下，不过毕竟这个链接只是纯代码，没什么讲解，所以大家还是要先读完我的文章，关注一下我的微信公众号【极客幼稚园】~ 示例项目地址：
* https://github.com/dtolnay/syn/tree/master/examples/heapsize

我们重点来看一下`split_for_impl()`这个工具函数的用法，比如说我们有这样一个泛型结构体，泛型参数`T`，`U`分别受到`Blog`、`IdeaWand`、`Com`这三个Trait Bound的限制：
```rust
struct GeekKindergarten<T, U> where T: Blog + IdeaWand, U: Com {}
```
那么，我们生成的`Debug` Trait的形式应该是下面这样的：
```rust
impl<T,U> Debug for GeekKindergarten<T, U> where T: Blog + IdeaWand + Debug, U: Com + Debug {
 // ^^^^^                           ^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^          ^^^^^^
 //  |                                 |            |                           |   
 //  |                                 |            +---------第三部分 ----------+
 //  |                                 +--第二部分
 //  +------第一部分
}
```
而`split_for_impl()`这个工具函数就是用来帮助我们生成上面这三个代码片段的，上面的限定在还出现了`Debug`，这个是我们要后面再手动加上去的，并不是`split_for_impl()`能帮我们生成的，所以我没有把他们标出来。

好了，我们总结一下第四关需要做的事情，然后给出示例代码：
* 从`DeriveInput`语法树节点获取泛型参数信息
* 为每一个泛型参数都添加一个`Debug` Trait限定
* 使用`split_for_impl()`工具函数切分出用于模板生成代码的三个片段
* 修改`impl`块的模板代码，使用上述三个片段，加入泛型参数信息
* 此外由于目前`generate_debug_trait()`函数已经较为冗长，我们也对代码的结构进行微调，将其拆分为两个函数。

接下来上代码,首先是拆分出的新函数：
```rust
fn generate_debug_trait_core(st :&syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let fields = get_fields_from_derive_input(st)?;
    let struct_name_ident = &st.ident;
    let struct_name_literal = struct_name_ident.to_string();
    let mut fmt_body_stream = proc_macro2::TokenStream::new();

    fmt_body_stream.extend(quote!(
        fmt.debug_struct(#struct_name_literal) 
    ));
    for field in fields.iter(){
        let field_name_idnet = field.ident.as_ref().unwrap();
        let field_name_literal = field_name_idnet.to_string();
        
        let mut format_str = "{:?}".to_string();
        if let Some(format) = get_custom_format_of_field(field)? {
            format_str = format;
        } 
        // 这里是没有指定用户自定义的格式
        fmt_body_stream.extend(quote!(
            .field(#field_name_literal, &format_args!(#format_str, self.#field_name_idnet))
        ));
    }

    fmt_body_stream.extend(quote!(
        .finish()
    ));
    return Ok(fmt_body_stream)
}
```

然后是我们生成impl块的代码,详细阅读一下其中的注释，添加`Debug` Trait限定的代码一开始我也不知道怎么写，是参考了第四关提示中的`heapsize`项目的范例：
```rust
fn generate_debug_trait(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {

    let fmt_body_stream = generate_debug_trait_core(st)?;

    let struct_name_ident = &st.ident;

    // 从输入的派生宏语法树节点获取被修饰的输入结构体的泛型信息
    let mut generics_param_to_modify = st.generics.clone();
    // 我们需要对每一个泛型参数都添加一个`Debug` Trait限定
    for mut g in generics_param_to_modify.params.iter_mut() {
        if let syn::GenericParam::Type(t) = g {
            t.bounds.push(parse_quote!(std::fmt::Debug));
        }
    }

    // 使用工具函数把泛型抽取成3个片段
    let (impl_generics, type_generics, where_clause) = generics_param_to_modify.split_for_impl();

    let ret_stream = quote!(
        // 注意下面这一行是如何使用三个与泛型参数有关的代码片段的
        impl #impl_generics std::fmt::Debug for #struct_name_ident #type_generics #where_clause {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                #fmt_body_stream
            }
        }
    );

    return Ok(ret_stream)
}
```

### 第五关

第五关视频版：[https://www.bilibili.com/video/BV1v44y1z7eb?zw](https://www.bilibili.com/video/BV1v44y1z7eb?zw)


这一关的关卡说明非常长，信息量不少，我们来仔细看看：

首先抛出了一个问题，例如对于下面这个结构体：
```rust
pub struct GeekKindergarten<T> {
    ideawand: PhantomData<T>,
}
```
这个结构体中使用了`PhantomData`这个类型，而`PhantomData`类型本身在标准库中实现了`Debug` Trait，如下所示：
```rust
impl<T: ?Sized> Debug for PhantomData<T> {...}
```
在这种情况下，我们没有必要限定`T`是`Debug`的。面对这类本身已经实现了`Debug`的泛型结构体，我们有一种应对办法，即我们在生成Trait限定时，不是针对每一个泛型参数去限制，而是对结构体中每一个字段的类型来添加限制，还是以下面这个结构体为例来说：
```rust
pub struct GeekKindergarten<T, U> {
    blog: Foo<U>,
    ideawand: PhantomData<T>,
}
```
原来我们代码生成的限定条件是:
* `T: Debug, U: Debug`
而我们现在应该生成的是:
* `Foo<U>: Debug, PhantomData<T>: Debug`

但是，这样的限定会有很大的副作用，这些副作用在后续的关卡中大家会遇到，所以，解题提示中给我们指出了另一个方法：
* 因为`PhantomData`类型的使用太常见了，所以我们就把`PhantomData`这个类型作为一个特例，我们检测是不是有`PhantomData`类型的字段，如果有，我们看看它所使用的泛型参数是不是只在`PhantomData`中出现过，如果是，我们就不为它添加`Debug`限定了。
* 在后续关卡中，我们会为`PhantomData`之外的情况，提供一个“逃生出口”(escape hatch)，用来标记某一个字段的类型限定。

在给出上述提示之后，出题人还给我们介绍了一下Rust过程宏的一些设计上的取舍：
* 在Rust过程宏中，你不可能获得到一个完全正确的Trait限定，这是因为假设要实现这个功能，就要在展开过程宏时做命名解析。而这样做会导致rust编译器复杂度急剧上升
* Rust核心团队认为这样的取舍带来的收益非常大，因此没有任何计划打算在后续支持宏展开时的命名解析
* 使用escape hatch来解决问题是一种常用手段
* 另一种更加常见的手段是，通过Rust的Trait机制，将命名解析的执行时间推后到真正的编译阶段去处理
  * 特别注意一下本关的测试用例代码，看看过程宏的调用是如何能够在不知道`S`指代的是`String`类型的情况下，产生出可以调用`String`类型的`Debug`Trait实现的。

第五关的解题提示到这里就分析完了，在我们开始写代码之前，我们先看看下面新增代码的主要逻辑，例如有这样一个泛型结构体,则我们的过程宏的行为应该是:
```rust
struct GeekKindergarten<T, U, V, W> {
    blog: T,
    ideawand: PhantomData<U>,
    com: U,
    foo: PhantomData<V>,
    bar: Baz<W>,
}
```
* 对T，由于没有出现在`PhantomData`中，则需要对T增加`Debug`限定
* 对U，虽然出现在`PhantomData`中,但因为其同时直接作为`com`字段的类型，所以仍然需要加`Debug`限定
* 对于V，满足这一关设定的特殊条件，不添加`Debug`限定
* 对于W，因为其不在`PhantomData`的泛型参数中，所以需要加`Debug`限定

可以看到，想实现上面的逻辑，我们需要获取`<>`之前的类型名称，以及`<>`内部的类型名称，剩下的就是各种判断这些类型名字对应的字符串是不是满足各种组合条件了。

下面开始撸代码，先定义一个获取`PhantomData`泛型参数名字的函数，这个函数的作用是把`PhantomData<X>`里面的`X`作为字符串提取出来：
```rust
fn get_phantomdata_generic_type_name(field: &syn::Field) -> syn::Result<Option<String>> {
    if let syn::Type::Path(syn::TypePath{path: syn::Path{ref segments, ..}, ..}) = field.ty {
        if let Some(syn::PathSegment{ref ident, ref arguments}) = segments.last() {
            if ident == "PhantomData" {
                if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments{args, ..}) = arguments {
                    if let Some(syn::GenericArgument::Type(syn::Type::Path( ref gp))) = args.first() {
                        if let Some(generic_ident) = gp.path.segments.first() {
                            return Ok(Some(generic_ident.ident.to_string()))
                        }
                    }
                }
            }
        }
    }
    return Ok(None)
}
```

然后再定义一个函数，用于把结构体定义中`foo: XXX`或`foo:XXX<YYY>`这种形式中，`XXX`所在位置的类型名字（即不包括泛型参数）作为字符串返回：
```rust
fn get_field_type_name(field: &syn::Field) -> syn::Result<Option<String>> {
    if let syn::Type::Path(syn::TypePath{path: syn::Path{ref segments, ..}, ..}) = field.ty {
        if let Some(syn::PathSegment{ref ident,..}) = segments.last() {
            return Ok(Some(ident.to_string()))
        }
    }
    return Ok(None)
}
```

然后我们来修改`generate_debug_trait()`函数的代码，请仔细阅读其中的注释：
```rust
fn generate_debug_trait(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {

    let fmt_body_stream = generate_debug_trait_core(st)?;

    let struct_name_ident = &st.ident;

    
    let mut generics_param_to_modify = st.generics.clone();

    // 以下代码构建两个列表，一个是`PhantomData`中使用到的泛型参数，另一个是输入结构体中所有字段的类型名称
    let fields = get_fields_from_derive_input(st)?;
    let mut field_type_names = Vec::new();
    let mut phantomdata_type_param_names = Vec::new();
    for field in fields{
        if let Some(s) = get_field_type_name(field)? {
            field_type_names.push(s);
        }
        if let Some(s) = get_phantomdata_generic_type_name(field)? {
            phantomdata_type_param_names.push(s);
        }
    }

    for mut g in generics_param_to_modify.params.iter_mut() {
        if let syn::GenericParam::Type(t) = g {
            let type_param_name = t.ident.to_string();
            // 注意这个判断条件的逻辑，精华都在这个条件里了，自己试着看看能不能把上面的4种情况整理成这个条件
            // 如果是PhantomData，就不要对泛型参数`T`本身再添加约束了,除非`T`本身也被直接使用了
            if phantomdata_type_param_names.contains(&type_param_name) && !field_type_names.contains(&type_param_name) {
                continue;
            }
            t.bounds.push(parse_quote!(std::fmt::Debug));
        }
    }

    // <省略未修改代码> ............
    
}
```


在结束第五关之前，我们再来回想一下出题人给我们留下的思考题：
> 在第五关的测试用例中，过程宏的调用是如何能够在不知道`S`指代的是`String`类型的情况下，产生出可以调用`String`类型的`Debug`Trait实现的？

其实，这个问题很简单，大家只要记住一件事，过程宏，其实就是在玩“字符串替换拼接”的游戏，在过程宏执行的时候，虽然我们把它解析成了语法树，但语法也只是一种字符串排列形式上的约束，并没有类型的概念。你只要能生成出符合rust语法的字符串排列组合即可。真正的符号消解、类型检验等等，是在后面的编译阶段完成的。

### 第六关
这一关展示了第五关被舍弃掉的一个解决方案所存在的问题，比较绕，有兴趣的同学可以自己看看。我们的代码不需要修改就可以通过第六关。

### 第七关

第七关视频版：[https://www.bilibili.com/video/BV1Gq4y1774f?zw](https://www.bilibili.com/video/BV1Gq4y1774f?zw)

第七关我们要处理关联类型的问题。从第七关的提示里，我们了解到需要做的一个主要工作是寻找出同时满足如下要求的类型为`syn::TypePath`的语法树节点：
* 其Path长度大于等于2
* 其Path的第一项为泛型参数列表中某一个

根据Rust的语法，我们面临的关联类型可以有如下的形式：
```rust
pub trait Trait {
    type Value;    // 定义一个关联类型
}

pub struct GeekKindergarten<T:Trait> {
    blog: T::Value,
    ideawand: PhantomData<T::Value>,
    com: Foo<Bar<Baz<T::Value>>>,
}
```
也就是说，我们要寻找的形如`T::Value`的代码片段，可能嵌套在很深的地方，根据前面的经验，我们可能要写一个嵌套了几层if条件的递归函数来在整个语法树中遍历，有没有更优雅的写法呢，幸好`syn`库为我们提供了visit模式来访问语法树中你感兴趣的节点。

默认情况下，Visit访问模式在`syn`库中是没有开启的，根据`syn`官方文档首页中的描述，我们需要在cargo.toml里添加`visit`这个特性后才可以使用。所以我们首先需要更新一下cargo.toml。

Visit模式的使用说明可以参阅官方文档：https://docs.rs/syn/1.0.64/syn/visit/index.html

Visit模式的核心原理是，其定义了一个名为`Visit`的Trait，这个Trait中包含了上百个类型的语法树节点各自对应的回调函数，当其遍历语法树时，每遍历到一个类型的语法树节点，就会调用相应的回调函数。在第七关中，由于我们只希望筛选出所有`syn::TypePath`类型的节点，所以我们只需要实现这个节点对应的回调函数，然后在其中判断当前节点是否满足上述要求即可。大家可以看一下官方文档给出的实例，这里我就直接给出相关代码实现：

首先是Visitor的定义：
```rust
use syn::visit::{self, Visit};

// 定义一个用于实现`Visit` Trait的结构体，结构体中定义了一些字段，用于存储筛选条件以及筛选结果
struct TypePathVisitor {
    generic_type_names: Vec<String>,  // 这个是筛选条件，里面记录了所有的泛型参数的名字，例如`T`,`U`等
    associated_types: HashMap<String, Vec<syn::TypePath>>,  // 这里记录了所有满足条件的语法树节点
}

impl<'ast> Visit<'ast> for TypePathVisitor {
    // visit_type_path 这个回调函数就是我们所关心的
    fn visit_type_path(&mut self, node: &'ast syn::TypePath) {
        
        if node.path.segments.len() >= 2 {
            let generic_type_name = node.path.segments[0].ident.to_string();
            if self.generic_type_names.contains(&generic_type_name) {
                // 如果满足上面的两个筛选条件，那么就把结果存起来
                self.associated_types.entry(generic_type_name).or_insert(Vec::new()).push(node.clone());
            }
        }
        // Visit 模式要求在当前节点访问完成后，继续调用默认实现的visit方法，从而遍历到所有的
        // 必须调用这个函数，否则遍历到这个节点就不再往更深层走了
        visit::visit_type_path(self, node);
    }
}
```

然后是我们初始化Visitor然后执行遍历访问，最终返回筛选结果的函数:
```rust
fn get_generic_associated_types(st: &syn::DeriveInput) -> HashMap<String, Vec<syn::TypePath>> {
    // 首先构建筛选条件
    let origin_generic_param_names: Vec<String> = st.generics.params.iter().filter_map(|f| {
        if let syn::GenericParam::Type(ty) = f {
            return Some(ty.ident.to_string())
        }
        return None
    }).collect();

    
    let mut visitor = TypePathVisitor {
        generic_type_names: origin_generic_param_names,  // 用筛选条件初始化Visitor
        associated_types: HashMap::new(),
    };

    // 以st语法树节点为起点，开始Visit整个st节点的子节点
    visitor.visit_derive_input(st);
    return visitor.associated_types;
}
```
例如对于下面这样的关联类型和结构体：
```rust

pub trait TraitA {
    type Value1;
    type Value2;
}

pub trait TraitB {
    type Value3;
    type Value4;
}

pub struct GeekKindergarten<T: TraitA, U: TraitB> {
    blog: T::Value1,
    ideawand: PhantomData<U::Value3>,
    com: Foo<Bar<Baz<T::Value2>>>,
}
```

则我们上面函数将会返回这样一个结构，之所以用了一个字典，是为了后续检索方便，而字典的值又是一个列表的原因是，一个Trait里面可能有多个关联类型：
```rust
{
    "T": [T::Value1, T::Value2],
    "U": [U::Value3],
}
```

筛选出所有的关联类型后，我们再更新一下`impl`块的生成代码，与之前不同的是，对于关联类型的限定，只能放在where子句中，代码如下：
```rust
fn generate_debug_trait(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {

    // <此处省略没有修改的代码> ..........

    // 下面这一行是第七关新加的，调用函数找到关联类型信息
    let associated_types_map = get_generic_associated_types(st);
    for mut g in generics_param_to_modify.params.iter_mut() {
        if let syn::GenericParam::Type(t) = g {
            let type_param_name = t.ident.to_string();        
            
            if phantomdata_type_param_names.contains(&type_param_name) && !field_type_names.contains(&type_param_name){
                continue;
            }

            // 下面这3行是本次新加的，如果是关联类型，就不要对泛型参数`T`本身再添加约束了,除非`T`本身也被直接使用了
            if associated_types_map.contains_key(&type_param_name) && !field_type_names.contains(&type_param_name){
                continue
            }

            t.bounds.push(parse_quote!(std::fmt::Debug));
        }
    }

    // 以下6行是第七关新加的，关联类型的约束要放到where子句里
    generics_param_to_modify.make_where_clause();
    for (_, associated_types) in associated_types_map {
        for associated_type in associated_types {
            generics_param_to_modify.where_clause.as_mut().unwrap().predicates.push(parse_quote!(#associated_type:std::fmt::Debug));
        }
    }

    // <此处省略没有修改的代码> ..........
}
```

### 第八关

第八关视频版：[https://www.bilibili.com/video/BV1vV41147ES?zw](https://www.bilibili.com/video/BV1vV41147ES?zw)

这一关要实现的是之前提到的“逃生出口“(escape hatch)，由于前面介绍过的Rust过程宏展开机制的缺陷，在一些边界情况下我们无法正确推断出泛型的Trait限定，这时候，我们就需要提供一个人为干预的后门。本关分为两部分，一部分是必答题，提供一个全局的干预方式，还有一个是选做题，精确到对每个字段进行控制。因为这篇文章已经很长了，所以我们就只做必答题，选做题留给大家自己去实现了。

首先是要解析一个全局的属性标签，属性标签我们已经解析过很多次了，这次就直接给大家代码了：
```rust
fn get_struct_escape_hatch(st: &syn::DeriveInput) -> Option<String> {
    if let Some(inert_attr) = st.attrs.last() {
        if let Ok(syn::Meta::List(syn::MetaList { nested, .. })) = inert_attr.parse_meta() {
            if let Some(syn::NestedMeta::Meta(syn::Meta::NameValue(path_value))) = nested.last() {
                if path_value.path.is_ident("bound") {
                    if let syn::Lit::Str(ref lit) = path_value.lit {
                        return Some(lit.value());
                    }
                }
            }
        }
    }
    None
}
```

然后，我们拿到了用户输入的干预指令，其实就是一小段Rust的代码，我们要把这一小段Rust代码解析为语法树的节点后插入到where子句对应的节点中。解析用户的输入可以使用`syn::parse_str()`这个函数来实现。好了，直接上代码：
```rust
fn generate_debug_trait(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
   
    // <此处省略没有修改的代码> ..........


    // 判定是否设置了限定条件干预，如果设定了，则不进行推断，直接使用用户给出的限定条件放到where子句中
    if let Some(hatch) = get_struct_escape_hatch(st) {
        generics_param_to_modify.make_where_clause();
        generics_param_to_modify
                    .where_clause
                    .as_mut()
                    .unwrap()
                    .predicates
                    .push(syn::parse_str(hatch.as_str()).unwrap());
    } else {
        // 原来位于此处的代码，全部移动到else分支里面，其他不变，省略 ..........
    }

    let (impl_generics, type_generics, where_clause) = generics_param_to_modify.split_for_impl();

    // <此处省略没有修改的代码> ..........


}
```

最后，需要承认的一点是，上面写的这些代码肯定是不严谨的，或者说是漏洞百出的。一方面是这只是为了通过测试用例，并没有充分考虑测试用例没有覆盖到的场景；另一方面，大家也应该充分认识到，Rust的过程宏就是一个复杂的“字符串拼接”过程，他没有类型校验，我们通过字符串匹配来关联一些“类型”，因此你完全可以通过构造一些冲突的命名来迷惑我们的代码。这就是Rust的过程宏，充满了Trick。