# 「译」用Rust编写LLVM的玩具编译器

译者：[iamazy](https://github.com/iamazy)

---

原文：[A Toy Front-End for LLVM, written in Rust](https://blog.ulysse.io/post/a-toy-front-end-for-llvm-written-in-rust/)



> 我目前的副业是用 Rust 编写一个可以将代码转换成 LLVM IR 的编译器。LLVM 的 API 对于新手来说有点令人生畏，而且没有很多有关的教程（有限的教程大多数还是基于 C++ 的，如何使用 Rust 做同样的事并不总是那么明确）。我希望当我准备做一件事情时，有人可以手把手的教我，这也是我要写这篇文章的原因。

对于 Rust ，与 LLVM 的接口交互的最佳选择是使用 `llvm-sys`。互联网上的一些热心朋友在[这里](http://rustdoc.taricorp.net/llvm-sys/llvm_sys/)托管了一些关于 `llvm-sys` 的文档。当然，你还应该去查看 LLVM 的[官方指南](http://llvm.org/docs/tutorial/LangImpl01.html)，因为它可以帮助你理解 LLVM 是如何“思考”的。这篇文章基本上是 LLVM 官方指南的 Rust 翻译。

你可以从这里获取最终的[代码](https://github.com/ucarion/llvm-rust-getting-started)。

## 搭建开发环境

对于新手，使用 LLVM 开发有一个可以复用的方式：

```shell
# `curl` is just so we can next install Rust
sudo apt-get -y install clang curl llvm-3.8-dev
curl https://sh.rustup.rs -sSf | sh

# The `llvm-sys` crate expects something called `llvm-config` on your PATH.
sudo ln -s /usr/bin/llvm-config-3.8 /usr/bin/llvm-config
```

如果你是在 Ubuntu 上执行上面的语句（你可能需要执行 `apt-get update` ），则没有任何问题。如果不是，你需要使用下面的 `Vagrantfile` 文件在 Vagrant Box 中运行上述语句。

```shell
Vagrant.configure("2") do |config|
  config.vm.box = "bento/ubuntu-16.04"
end
```

你可以从执行 `cargo init llvm-example --bin` 开始，并将下面（从 llvm-sys 中拷贝）的代码写入 `src/main.rs` 中：

```rust
//! Construct a function that does nothing in LLVM IR.

extern crate llvm_sys as llvm;

use std::ptr;

fn main() {
    unsafe {
        // Set up a context, module and builder in that context.
        let context = llvm::core::LLVMContextCreate();
        let module = llvm::core::LLVMModuleCreateWithName(b"nop\0".as_ptr() as *const _);
        let builder = llvm::core::LLVMCreateBuilderInContext(context);

        // Get the type signature for void nop(void);
        // Then create it in our module.
        let void = llvm::core::LLVMVoidTypeInContext(context);
        let function_type = llvm::core::LLVMFunctionType(void, ptr::null_mut(), 0, 0);
        let function = llvm::core::LLVMAddFunction(module, b"nop\0".as_ptr() as *const _, function_type);

        // Create a basic block in the function and set our builder to generate
        // code in it.
        let bb = llvm::core::LLVMAppendBasicBlockInContext(context, function,b"entry\0".as_ptr() as *const _);
        llvm::core::LLVMPositionBuilderAtEnd(builder, bb);

        // Emit a `ret void` into the function
        llvm::core::LLVMBuildRetVoid(builder);

        // Dump the module as IR to stdout.
        llvm::core::LLVMDumpModule(module);

        // Clean up. Values created in the context mostly get cleaned up there.
        llvm::core::LLVMDisposeBuilder(builder);
        llvm::core::LLVMDisposeModule(module);
        llvm::core::LLVMContextDispose(context);
    }
}
```

并在你的 `Cargo.toml` 文件中：

```toml
[package]
name = "llvm-example"
version = "0.1.0"
authors = ["Ulysse Carion <ulysse@ulysse.io>"]

[[bin]]
name = "main"

[dependencies]
llvm-sys = "0.2"
```

你可以获得：

```shell
vagrant@vagrant:/vagrant$ cargo run
   Compiling llvm-example v0.1.0 (file:///vagrant)
     Running `target/debug/main`
; ModuleID = 'nop'

define void @nop() {
entry:
  ret void
}
```

完美！现在我们可以开始编写自己的东西了。

## 一段不太平凡的程序

首先，让我们编译一个程序，该程序通过从 main 函数中返回一个整数来简单的设置一个返回码。

下面是我使用的方式（我们有时候需要使用一个解析器，所以我先添加了 `peg` 库）：

```rust
#![feature(plugin)]
#![plugin(peg_syntax_ext)]

extern crate llvm_sys as llvm;

use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;

fn main() {
    let mut input = String::new();
    let mut f = File::open("in.ex").unwrap();
    f.read_to_string(&mut input).unwrap();

    let parsed_input = parser::program(&input).unwrap();

    unsafe {
        codegen(parsed_input);
    }
}

peg! parser(r#"
    #[pub]
    program -> String
        = i:int_literal "\n" { i }

    int_literal -> String
        = [0-9]+ { match_str.to_owned() }
"#);

unsafe fn codegen(input: String) {
    let context = llvm::core::LLVMContextCreate();
    let module = llvm::core::LLVMModuleCreateWithName(b"example_module\0".as_ptr() as *const _);
    let builder = llvm::core::LLVMCreateBuilderInContext(context);

    // In LLVM, you get your types from functions.
    let int_type = llvm::core::LLVMInt64TypeInContext(context);
    let function_type = llvm::core::LLVMFunctionType(int_type, ptr::null_mut(), 0, 0);
    let function = llvm::core::LLVMAddFunction(module, b"main\0".as_ptr() as *const _, function_type);

    let entry_name = CString::new("entry").unwrap();
    let bb = llvm::core::LLVMAppendBasicBlockInContext(context, function, entry_name.as_ptr());
    llvm::core::LLVMPositionBuilderAtEnd(builder, bb);

    // The juicy part: construct a `LLVMValue` from a Rust value:
    let int_value: u64 = input.parse().unwrap();
    let int_value = llvm::core::LLVMConstInt(int_type, int_value, 0);

    llvm::core::LLVMBuildRet(builder, int_value);

    // Instead of dumping to stdout, let's write out the IR to `out.ll`
    let out_file = CString::new("out.ll").unwrap();
    llvm::core::LLVMPrintModuleToFile(module, out_file.as_ptr(), ptr::null_mut());

    llvm::core::LLVMDisposeBuilder(builder);
    llvm::core::LLVMDisposeModule(module);
    llvm::core::LLVMContextDispose(context);
}
```

它起作用了！测试一下：

```shell
vagrant@vagrant:/vagrant$ cat in.ex
42
vagrant@vagrant:/vagrant$ cargo run
     Running `target/debug/main`
vagrant@vagrant:/vagrant$ lli-3.8 out.ll ; echo $?
42
```

有点酷哦！顺便提一下，下面是 `out.ll` 文件的内容：

```s
; ModuleID = 'example_module'

define i64 @main() {
entry:
  ret i64 42
}
```

## 算术

让我们添加对数字的加减乘除操作的支持。为了实现这个，我们需要扩展我们的语法。我们引入一个枚举来代表 AST（抽象语法树）。

```rust
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Literal(String),
}
```

并扩展语法：

```rust
// `product` and `sum` are that way to get operator precedence
peg! parser(r#"
    use super::Expr;

    #[pub]
    program -> Expr
        = e:expression "\n" { e }

    expression -> Expr
        = sum

    sum -> Expr
        = a:product _ "+" _ b:sum { Expr::Add(Box::new(a), Box::new(b)) }
        / a:product _ "-" _ b:sum { Expr::Sub(Box::new(a), Box::new(b)) }
        / product

    product -> Expr
        = a:int_literal _ "*" _ b:product { Expr::Mul(Box::new(a), Box::new(b)) }
        / a:int_literal _ "/" _ b:product { Expr::Div(Box::new(a), Box::new(b)) }
        / int_literal

    int_literal -> Expr
        = [0-9]+ { Expr::Literal(match_str.to_owned()) }

    _ = " "*
"#);
```

接下来，可以提交代码。你可以指定诸如 “addtmp” 的字符串，这些字符串将被用作 IR 中对应“寄存器”名称的一部分。

```rust
// When you write out instructions in LLVM, you get back `LLVMValueRef`s. You
// can then use these references in other instructions.
unsafe fn codegen_expr(context: LLVMContextRef, builder: LLVMBuilderRef, expr: Expr) -> LLVMValueRef {
    match expr {
        Expr::Literal(int_literal) => {
            let int_type = llvm::core::LLVMInt64TypeInContext(context);
            llvm::core::LLVMConstInt(int_type, int_literal.parse().unwrap(), 0)
        },

        Expr::Add(lhs, rhs) => {
            let lhs = codegen_expr(context, builder, *lhs);
            let rhs = codegen_expr(context, builder, *rhs);

            let name = CString::new("addtmp").unwrap();
            llvm::core::LLVMBuildAdd(builder, lhs, rhs, name.as_ptr())
        },

        Expr::Sub(lhs, rhs) => {
            let lhs = codegen_expr(context, builder, *lhs);
            let rhs = codegen_expr(context, builder, *rhs);

            let name = CString::new("subtmp").unwrap();
            llvm::core::LLVMBuildSub(builder, lhs, rhs, name.as_ptr())
        },

        Expr::Mul(lhs, rhs) => {
            let lhs = codegen_expr(context, builder, *lhs);
            let rhs = codegen_expr(context, builder, *rhs);

            let name = CString::new("multmp").unwrap();
            llvm::core::LLVMBuildMul(builder, lhs, rhs, name.as_ptr())
        },

        Expr::Div(lhs, rhs) => {
            let lhs = codegen_expr(context, builder, *lhs);
            let rhs = codegen_expr(context, builder, *rhs);

            let name = CString::new("divtmp").unwrap();
            llvm::core::LLVMBuildUDiv(builder, lhs, rhs, name.as_ptr())
        },
    }
}
```

现在，你可以执行 `10 * 4 + 20/2 - 8` 之类的程序！如果你问我，那可真是太酷了。

## 变量

我们将采用简单的方式并假设程序不会执行任何烦人的操作，如引用未定义的变量等。我们只将变量存储在寄存器中，并将它们存在 `HashMap<String, LLVMValueRef>` 中，之所以有用是因为运行该程序只有这一种方式。

我们扩展语言和解析器：

```rust
pub enum Expr {
    Literal(String),
    Ref(String),
    Assign(String, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

peg! parser(r#"
    use super::Expr;

    #[pub]
    program -> Vec<Expr>
        = e:(expression ** "\n") "\n" { e }

    expression -> Expr
        = i:identifier _ "=" _ s:sum { Expr::Assign(i, Box::new(s)) }
        / sum

    sum -> Expr
        = a:product _ "+" _ b:sum { Expr::Add(Box::new(a), Box::new(b)) }
        / a:product _ "-" _ b:sum { Expr::Sub(Box::new(a), Box::new(b)) }
        / product

    product -> Expr
        = a:ref_or_literal _ "*" _ b:product { Expr::Mul(Box::new(a), Box::new(b)) }
        / a:ref_or_literal _ "/" _ b:product { Expr::Div(Box::new(a), Box::new(b)) }
        / ref_or_literal

    ref_or_literal -> Expr
        = i:identifier { Expr::Ref(i) }
        / int_literal

    identifier -> String
        = [a-zA-Z]+ { match_str.to_owned() }

    int_literal -> Expr
        = [0-9]+ { Expr::Literal(match_str.to_owned()) }

    _ = " "*
"#);
```

然后为这两个新的表达式添加支持：

```rust
unsafe fn codegen_expr(context: LLVMContextRef, builder: LLVMBuilderRef, names: &mut HashMap<String, LLVMValueRef>, expr: Expr) -> LLVMValueRef {
    match expr {
        // ...

        Expr::Ref(name) => {
            *names.get(&name).unwrap()
        },

        Expr::Assign(name, expr) => {
            let new_value = codegen_expr(context, builder, names, *expr);
            names.insert(name, new_value);
            new_value
        },
    }
}
```

并迅速的在 `codegen` 函数中更新：

```rust
let zero = llvm::core::LLVMConstInt(int_type, 0, 0);

let mut names = HashMap::new();
let mut return_value = zero; // return value on empty program
for expr in input {
    return_value = codegen_expr(context, builder, &mut names, expr);
}
llvm::core::LLVMBuildRet(builder, return_value);
```

现在让我们来一探究竟：

```rust
vagrant@vagrant:/vagrant$ cat in.ex
a = 3
b = 76
a + b
vagrant@vagrant:/vagrant$ cargo run
     Running `target/debug/main`
vagrant@vagrant:/vagrant$ cat out.ll
; ModuleID = 'example_module'

define i64 @main() {
entry:
  ret i64 79
}
```

## If

在使用 `if` 关键字的时候遇到一些麻烦。让 `if` 起作用的最简单的方式就是将所有的变量存储在堆栈上。并让 LLVM 做一些优化。在 LLVM 中，你可以通过 `alloca` 指令创建一个栈变量，并使用 `load/store` 进行读写。

为了实现这个，我们通过添加新的解析规则再一次扩展了语言和语法。

```rust
expression -> Expr
    = if_expression
    / i:identifier _ "=" _ s:expression { Expr::Assign(i, Box::new(s)) }
    / sum

if_expression -> Expr
    = "if" _ e:expression _ "{\n" _ then_body:statements _ "}" _ "else" _ "{\n" _ else_body:statements _ "}" {
        Expr::If(Box::new(e), then_body, else_body)
    }
```

并在 AST 节点上添加了一个新的类型：

```rust
pub enum Expr {
    Literal(String),
    Ref(String),
    Assign(String, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Vec<Expr>, Vec<Expr>),
}
```

最后，完成关于 `if` 表达式的代码：

```rust
unsafe fn codegen_expr(context: LLVMContextRef, builder: LLVMBuilderRef, func: LLVMValueRef, names: &mut HashMap<String, LLVMValueRef>, expr: Expr) -> LLVMValueRef {
    match expr {
        // ...

        Expr::If(condition, then_body, else_body) => {
            let condition_value = codegen_expr(context, builder, func, names, *condition);
            let int_type = llvm::core::LLVMInt64TypeInContext(context);
            let zero = llvm::core::LLVMConstInt(int_type, 0, 0);

            // `is_nonzero` is a 1-bit integer
            let name = CString::new("is_nonzero").unwrap();
            let is_nonzero = llvm::core::LLVMBuildICmp(builder, llvm::LLVMIntPredicate::LLVMIntNE, condition_value, zero, name.as_ptr());

            // It's fine to create blocks first, and then fill them in later.
            let entry_name = CString::new("entry").unwrap();
            let then_block = llvm::core::LLVMAppendBasicBlockInContext(context, func, entry_name.as_ptr());
            let else_block = llvm::core::LLVMAppendBasicBlockInContext(context, func, entry_name.as_ptr());
            let merge_block = llvm::core::LLVMAppendBasicBlockInContext(context, func, entry_name.as_ptr());

            llvm::core::LLVMBuildCondBr(builder, is_nonzero, then_block, else_block);

            llvm::core::LLVMPositionBuilderAtEnd(builder, then_block);
            let mut then_return = zero;
            for expr in then_body {
                then_return = codegen_expr(context, builder, func, names, expr);
            }
            llvm::core::LLVMBuildBr(builder, merge_block);

            llvm::core::LLVMPositionBuilderAtEnd(builder, else_block);
            let mut else_return = zero;
            for expr in else_body {
                else_return = codegen_expr(context, builder, func, names, expr);
            }
            llvm::core::LLVMBuildBr(builder, merge_block);

            // Position the builder so that it's ready to work on the next
            // expression.
            llvm::core::LLVMPositionBuilderAtEnd(builder, merge_block);
            zero
        }
    }
}
```

代码有点多，但是完成了你所期待的事情。现在，你可以像这样来运行程序：

```groovy
a = 1
if a {
    a = 42
} else {
    a = 13
}
a
```

上述代码对应的 IR 如下所示：

```s
; ModuleID = 'example_module'

define i64 @main() {
entry:
  %a = alloca i64
  store i64 1, i64* %a
  %a1 = load i64, i64* %a
  %is_nonzero = icmp ne i64 %a1, 0
  br i1 %is_nonzero, label %entry2, label %entry3

entry2:                                           ; preds = %entry
  store i64 42, i64* %a
  br label %entry4

entry3:                                           ; preds = %entry
  store i64 13, i64* %a
  br label %entry4

entry4:                                           ; preds = %entry3, %entry2
  %a5 = load i64, i64* %a
  ret i64 %a5
}
```

然而，我们还没有结束。目前，我们的 “if” 表达式的返回结果始终为 zero（见上述 `codegen_expr` 函数中 If 分支的返回值）。而我们想要的正好与其相反，如果我们执行了 “then” 路径，则 if 的求值结果应该为 then_return，否则返回 else_return。

你如何使用 LLVM 跟踪它执行了哪个分支？通过使用 “phi” 节点。你给 phi 指令一个 (block, value) 对，该 phi 节点将会返回与先前执行的块相对应的值。

我们可以这样结束 if。注意，我们必须更新 then_block 和 else_block，因为这是我们在 “then/else” 分支中需要的最后一个块，并且前面的 then_block 是 “then/else” 的第一个块。

```rust
// This is mostly the same code as before, just note the new calls to
// `LLVMGetInsertBlock`.

llvm::core::LLVMPositionBuilderAtEnd(builder, then_block);
let mut then_return = zero;
for expr in then_body {
    then_return = codegen_expr(context, builder, func, names, expr);
}
llvm::core::LLVMBuildBr(builder, merge_block);
let then_block = llvm::core::LLVMGetInsertBlock(builder);

llvm::core::LLVMPositionBuilderAtEnd(builder, else_block);
let mut else_return = zero;
for expr in else_body {
    else_return = codegen_expr(context, builder, func, names, expr);
}
llvm::core::LLVMBuildBr(builder, merge_block);
let else_block = llvm::core::LLVMGetInsertBlock(builder);

// Insert the phi node
llvm::core::LLVMPositionBuilderAtEnd(builder, merge_block);
let phi_name = CString::new("iftmp").unwrap();
let phi = llvm::core::LLVMBuildPhi(builder, int_type, phi_name.as_ptr());

let mut values = vec![then_return, else_return];
let mut blocks = vec![then_block, else_block];

llvm::core::LLVMAddIncoming(phi, values.as_mut_ptr(), blocks.as_mut_ptr(), 2);
phi
```

然后，你就得到了一个令人惊叹的编译器：

```shell
vagrant@vagrant:/vagrant$ cat in.ex
a = 1
b = 0
c = if a {
    if b {
        11
    } else {
        40
    }
} else {
    if b {
        10
    } else {
        20
    }
}
c + 2
vagrant@vagrant:/vagrant$ cargo run
     Running `target/debug/main`
vagrant@vagrant:/vagrant$ lli-3.8 out.ll ; echo $?
42
```

太酷了！下面是我们提供的示例输入程序的 IR：

```s
; ModuleID = 'example_module'

define i64 @main() {
entry:
  %a = alloca i64
  %b = alloca i64
  %c = alloca i64
  store i64 1, i64* %a
  store i64 0, i64* %b
  %a1 = load i64, i64* %a
  %is_nonzero = icmp ne i64 %a1, 0
  br i1 %is_nonzero, label %entry2, label %entry3

entry2:                                           ; preds = %entry
  %b5 = load i64, i64* %b
  %is_nonzero6 = icmp ne i64 %b5, 0
  br i1 %is_nonzero6, label %entry7, label %entry8

entry3:                                           ; preds = %entry
  %b10 = load i64, i64* %b
  %is_nonzero11 = icmp ne i64 %b10, 0
  br i1 %is_nonzero11, label %entry12, label %entry13

entry4:                                           ; preds = %entry14, %entry9
  %iftmp16 = phi i64 [ %iftmp, %entry9 ], [ %iftmp15, %entry14 ]
  store i64 %iftmp16, i64* %c
  %c17 = load i64, i64* %c
  %addtmp = add i64 %c17, 2
  ret i64 %addtmp

entry7:                                           ; preds = %entry2
  br label %entry9

entry8:                                           ; preds = %entry2
  br label %entry9

entry9:                                           ; preds = %entry8, %entry7
  %iftmp = phi i64 [ 11, %entry7 ], [ 40, %entry8 ]
  br label %entry4

entry12:                                          ; preds = %entry3
  br label %entry14

entry13:                                          ; preds = %entry3
  br label %entry14

entry14:                                          ; preds = %entry13, %entry12
  %iftmp15 = phi i64 [ 10, %entry12 ], [ 20, %entry13 ]
  br label %entry4
}

```

请注意：这些块具有以下的模式：不包含第一个条目，它们三个为一组，第一个是 “then” 分支，然后是 “else” 分支，最后是 “merge” 块（带有可识别的 phi 指令）。每一次我们遇到 “if” 表达式时都会在 main 后面附加三个新块。因为要在 AST 中递归查询三元组，所以块的三元组是有序的。

这就是我想要分享的全部内容！希望在这一点上你可以有足够的实力来决定你的命运。