#  Rust生态安全漏洞总结系列 | Part 3

作者： 张汉东

本系列主要是分析[`RustSecurity` 安全数据库库](https://rustsec.org/advisories/)中记录的`Rust`生态社区中发现的安全问题，从中总结一些教训，学习`Rust`安全编程的经验。

本期分析了下面十一个安全问题：

- ### [RUSTSEC-2021-0110: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2021-0110.html) 

- ### [RUSTSEC-2021-0098: Vulnerability in openssl-src](https://rustsec.org/advisories/RUSTSEC-2021-0098.html)

- ### [RUSTSEC-2021-0082: Unsoundness in vec-const](https://rustsec.org/advisories/RUSTSEC-2021-0082.html)

- ### [RUSTSEC-2021-0093: Vulnerability in crossbeam-deque](https://rustsec.org/advisories/RUSTSEC-2021-0093.html)

- ### [RUSTSEC-2021-0077: Vulnerability in better-macro](https://rustsec.org/advisories/RUSTSEC-2021-0077.html)

- ### [RUSTSEC-2021-0106: Vulnerability in bat](https://rustsec.org/advisories/RUSTSEC-2021-0106.html)

- ### [RUSTSEC-2021-0073: Vulnerability in prost-types](https://rustsec.org/advisories/RUSTSEC-2021-0073.html)

- ### [RUSTSEC-2021-0078: Vulnerability in hyper](https://rustsec.org/advisories/RUSTSEC-2021-0078.html)

- ### [RUSTSEC-2021-0072: Vulnerability in tokio](https://rustsec.org/advisories/RUSTSEC-2021-0072.html)

- ### [RUSTSEC-2021-0070: Vulnerability in nalgebra](https://rustsec.org/advisories/RUSTSEC-2021-0070.html)

- ### [CVE-2021-31162: Vulnerability in std](https://rustsec.org/advisories/CVE-2021-31162.html)

看是否能给我们一些启示。

## [RUSTSEC-2021-0110: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2021-0110.html) 

在 Wasmtime 中发现多个代码缺陷。包括 UAF(use-after-free)、越界读写等。

### 漏洞描述：

- 漏洞类型：Vulnerability
- 漏洞分类：memory-corruption/ memory-exposure
- CVE 编号： [CVE-2021-39216](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2021-39216)、[CVE-2021-39219](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2021-39219)、[CVE-2021-39218](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2021-39218)  
- 详细：https://rustsec.org/advisories/RUSTSEC-2021-0110.html
- 补丁：`>=0.30.0`
- 关键字：`use-after-free` / `out-of-bounds read /out-of-bounds write / Wasm`/ `garbage collection`

### 漏洞分析

> 背景： `externref` 是 WebAssembly 引用类型（Reference Types）中引入的概念，用于表示 Host 引用。

####  [Use after free passing `externref`s to Wasm in Wasmtime](https://github.com/bytecodealliance/wasmtime/security/advisories/GHSA-v4cp-h94r-m7xf)

当从 Host 传递给 Guest `externrefs` 时会引发 UAF 。满足下列条件之一可触发此 Bug ：

1. 同时明确地从Host传递多个 `externrefs`给 `wasm` 实例
2. 通过将多个 `externrefs` 作为参数从 Host 代码传递给 `wasm`函数
3. 从Host定义的多值返回函数中返回多个 `externrefs` 给 `wasm`

如果 `Wasmtime` 的 `VMExternRefActivationsTable`在传入第一个`externref`后容量被填满，那么传入第二个`externref`可能会触发垃圾回收。然而，在把控制权传给`Wasm`之前，第一个`externref`是没有根(root)的，因此，如果没有其他东西持有对它的引用或以其他方式保持它的live，就会被GC回收。然后，当控制权在垃圾收集后被传递给`Wasm`时，`Wasm`可以使用第一个`externref`，但这时它已经被释放了。

#### [Out-of-bounds read/write and invalid free with `externref`s and GC safepoints in Wasmtime](https://github.com/bytecodealliance/wasmtime/security/advisories/GHSA-4873-36h9-wv49)

在`Wasmtime`运行使用`externrefs`的`Wasm`时，存在一个无效释放和越界读写的错误。

要触发这个错误，`Wasmtime`需要运行使用`externrefs`的`Wasm`，Host 创建非空的`externrefs`，`Wasmtime`执行一个垃圾收集（GC），并且堆栈上必须有一个`Wasm`帧，它处于GC `Safepoint`（安全点就是指代码运行到这个地方，它的状态是确定的， GC就可以安全的进行一些操作），在这个安全点上没有 Live 的引用，这种情况下 `Wasmtime` 会错误地使用 `GC Stack map` 而非 安全点。这就会导致释放一些不应该释放的内存，以及潜在的越界读写。

#### [Wrong type for `Linker`-define functions when used across two `Engine`s](https://github.com/bytecodealliance/wasmtime/security/advisories/GHSA-q879-9g95-56mx)

> Engine，是在 wasmtime 中被用于跨线程管理wasm模块的全局上下文。
>
> Linker，是用于支持模块链接的结构。

在 `Linker::func_*` 安全函数中发现了一个问题。wasmtime 不支持函数的跨 engine 使用，这可能导致函数指针的类型混乱，导致能够安全地调用一个类型错误的函数。这种情况应该 panic！



## [RUSTSEC-2021-0098: Vulnerability in openssl-src](https://rustsec.org/advisories/RUSTSEC-2021-0098.html)

[openssl-src](https://crates.io/crates/openssl-src) 是用于构建 OpenSSL 给 `openssl-sys` 库使用的。OpenSSL 最近又发现了很多新的安全缺陷，也记录到这里了。

具体这个漏洞是指 处理ASN.1字符串时的读取缓冲区超限问题。

### 漏洞描述：

- 漏洞类型：Vulnerability

- 漏洞分类：denial-of-service / crypto-failure

- CVE 编号：CVE-2021-3712

- 详细：https://www.openssl.org/news/secadv/20210824.txt

- 补丁：`>=111.16`

  

### 漏洞分析

ASN.1字符串在OpenSSL内部被表示为一个`ASN1_STRING`结构，它包含一个容纳字符串数据的缓冲区和一个容纳缓冲区长度的字段。这与普通的C语言字符串不同，后者表示为一个字符串数据的缓冲区，以`NUL（0）`字节结束。

虽然不是严格的要求，但使用OpenSSL自己的 "d2i "函数（和其他类似的解析函数）解析的ASN.1字符串，以及任何用ASN1_STRING_set()函数设置值的字符串，都会在ASN1_STRING结构中以NUL结束字节数。

然而，应用程序有可能直接构建有效的ASN1_STRING结构，通过直接设置ASN1_STRING数组中的 "data "和 "length "字段，不以NUL方式终止字节数组。这也可以通过使用ASN1_STRING_set0()函数来实现。

许多打印ASN.1数据的OpenSSL函数被认为ASN1_STRING字节数组将以NUL结尾，尽管这对直接构建的字符串来说是不保证的。如果应用程序要求打印一个ASN.1结构，而该ASN.1结构包含由应用程序直接构建的ASN1_STRING，而没有以NUL结束 "data "字段，那么就会发生读取缓冲区超限。

如果一个恶意行为者可以使一个应用程序直接构建一个ASN1_STRING，然后通过受影响的OpenSSL函数之一处理它，那么这个问题可能会被击中。这可能导致崩溃（造成拒绝服务攻击,DOS）。它还可能导致私人内存内容（如私人密钥或敏感明文）的泄露。

### 其他 OpenSSL 问题

OpenSSL 缺陷列表： https://rustsec.org/packages/openssl-src.html 

## [RUSTSEC-2021-0082: Unsoundness in vec-const](https://rustsec.org/advisories/RUSTSEC-2021-0082.html)

`vec-const`试图从一个指向常量切片的指针构造一个`Vec`。

### 漏洞描述：

- 漏洞类型：Unsound

- 漏洞分类：memory-corruption

- CVE 编号：CVE-2021-3711

- 详细：https://github.com/Eolu/vec-const/issues/1#issuecomment-898908241

- 补丁：暂无，不建议使用该 crate

- 关键字： memory-safety

  

### 漏洞分析

这个crate 违反了Rust的规则，使用起来会有危害。你不应该使用这个crate。这个crate不应该存在。它创建了不健全的抽象，允许不安全的代码伪装成安全代码。

这个crate声称要构造一个长度和容量都不为零的`const Vec`，但这是做不到的，因为这样的`Vec`需要一个来自分配器(allocator)的指针。参见：https://github.com/rust-lang/const-eval/issues/20。

## [RUSTSEC-2021-0093: Vulnerability in crossbeam-deque](https://rustsec.org/advisories/RUSTSEC-2021-0093.html)

crossbeam-deque中发生了数据竞争。

### 漏洞描述：

- 漏洞类型：Vulnerability

- 漏洞分类：memory-corruption

- CVE 编号： GHSA-pqqp-xmhj-wgcw、 [CVE-2021-32810](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2021-32810)

- 详细：https://github.com/Eolu/vec-const/issues/1#issuecomment-898908241

- 补丁：`>=0.7.4, <0.8.0`  / `>=0.8.1`

  

### 漏洞分析

在受影响的版本中，队列的一个或多个任务会被弹出两次，如果在堆上分配，会导致 dobule free 和 内存泄漏。如果不是堆上分配，则会引起逻辑错误。

修复PR ：https://github.com/crossbeam-rs/crossbeam/pull/726

问题是因为任务窃取相关条件判断错误导致的，是逻辑 Bug。



## [RUSTSEC-2021-0077: Vulnerability in better-macro](https://rustsec.org/advisories/RUSTSEC-2021-0077.html)

better-macro 是一个假的 crate，它在 "证明一个观点"，即`proc-macros`可以运行任意的代码。这是一个特别新颖或有趣的观察。

它目前打开的 https://github.com/raycar5/better-macro/blob/master/doc/hi.md，似乎没有任何恶意的内容，但不能保证会一直如此。

这个 crate 没有任何有用的功能，不应该被使用。

```rust
#[proc_macro]
pub fn println(input: TokenStream) -> TokenStream {
    if let Ok(_) = Command::new("xdg-open").arg(URL).output() {
    } else if let Ok(_) = Command::new("open").arg(URL).output() {
    } else if let Ok(_) = Command::new("explorer.exe").arg(URL).output() {
    }
    let input: proc_macro2::TokenStream = input.into();
    let out = quote! {::std::println!(#input)};
    out.into()
}
```



## [RUSTSEC-2021-0106: Vulnerability in bat](https://rustsec.org/advisories/RUSTSEC-2021-0106.html)

[bat](https://rustsec.org/packages/bat.html) 中存在不受控制的搜索路径元素，可能会导致非预期代码执行。

`0.18.2`之前的windows系统中的bat会从当前工作目录中执行名为`less.exe`的程序。

### 漏洞描述：

- 漏洞类型：Vulnerability

- 漏洞分类：code-execution

- CVE 编号：[CVE-2021-36753](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2021-36753) 、 GHSA-p24j-h477-76q3

- 详细：https://nvd.nist.gov/vuln/detail/CVE-2021-36753

- 补丁：`>=0.18.2` 

- 平台： Windows

  

### 漏洞分析

修复 PR： https://github.com/sharkdp/bat/pull/1724

对传入的 Path 进行了合法验证。使用的库是 `grep_cli`。

## [RUSTSEC-2021-0073: Vulnerability in prost-types](https://rustsec.org/advisories/RUSTSEC-2021-0073.html)

从`prost_types::Timestamp`到`SystemTime`的转换可能导致溢出和恐慌。

### 漏洞描述：

- 漏洞类型：Vulnerability

- 漏洞分类：denial-of-service

- CVE 编号：[CVE-2021-36753](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2021-36753) 、 GHSA-p24j-h477-76q3

- 详细：https://github.com/tokio-rs/prost/issues/438

- 补丁：`>=0.8.0` 

  

### 漏洞分析

在`prost-types 0.7.0`中，从`Timestamp`到`SystemTime`的转换使用`UNIX_EPOCH`上的`+`和`-`运算符。如果输入的`Timestamp`是不被信任的，这可能会溢出和恐慌，造成拒绝服务的漏洞。因为 SystimeTime 内部实现的 `+` 和 `-` 使用 `checked_add/checked_sub`会发生 panic。

```rust
use prost_types::Timestamp;
use std::time::SystemTime;

SystemTime::from(Timestamp {
    seconds: i64::MAX,
    nanos: 0,
}); // panics on i686-unknown-linux-gnu (but not x86_64) with default compiler settings

SystemTime::from(Timestamp {
    seconds: i64::MAX,
    nanos: i32::MAX,
}); // panics on x86_64-unknown-linux-gnu with default compiler settings
```



另外，转换涉及到调用`Timestamp::normalize`，它使用了`+`和`-`运算符。这可能会引起恐慌或环绕（wrap around,取决于编译器设置），如果应用程序被编译为溢出时恐慌，也会产生拒绝服务的漏洞。

解决问题的思路是：

`Timestamp::normalize`可能应该使用 [`saturating_{add,sub}`](https://doc.rust-lang.org/stable/std/time/struct.Duration.html#method.saturating_add) 方法，如果时间戳的`nanos`字段超出了范围，这可能会默默地改变时间戳，最多3秒，但这样的时间戳可以说是无效的，所以这可能是好的。

`SystemTime` 没有`Saturating_{add,sub}`方法，也没有`MIN`和`MAX`常数，应该再次使用 `SystemTime::checked_{add,sub}` 进行转换。

修复 PR： https://github.com/tokio-rs/prost/pull/439

## [RUSTSEC-2021-0078: Vulnerability in hyper](https://rustsec.org/advisories/RUSTSEC-2021-0078.html)

对Content-Length进行宽松的 header 解析，可能会使请求被偷渡（走私，smuggling）。

> 背景： 请求偷渡
>
> 不合法的请求被夹杂在合法请求中被得到处理。需要通过 `content-length` 和 `Transfer-Encoding` 两个header 来构造攻击。

### 漏洞描述：

- 漏洞类型：Vulnerability

- 漏洞分类：http、parsing

- CVE 编号：CVE-2021-32715

- 详细：https://github.com/hyperium/hyper/security/advisories/GHSA-f3pg-qwvg-p99c

- 补丁：`>=0.14.10` 

  

### 漏洞分析

hyper的HTTP/1服务器代码存在一个缺陷，即错误地解析和接受带有前缀加号的Content-Length头的请求，而这一请求本应作为非法请求被拒绝。这与上游HTTP代理不解析这种Content-Length头而转发的情况相结合，可能导致 "请求偷渡（"request smuggling） "或 "去同步攻击（desync attacks）"。

修复代码：https://github.com/hyperium/hyper/commit/06335158ca48724db9bf074398067d2db08613e7 

需要判断 content-lenght 是不是可以正常转换为有效数位。

## [RUSTSEC-2021-0072: Vulnerability in tokio](https://rustsec.org/advisories/RUSTSEC-2021-0072.html)

当用`JoinHandle::abort`中止一个任务时，对于 `LocalSet`上生成的任务不正确， 容易导致竞态条件。

### 漏洞描述：

- 漏洞类型：Vulnerability

- 漏洞分类：memory-corruption

- CVE 编号：CVE-2021-32715

- 详细：https://github.com/tokio-rs/tokio/issues/3929

- 补丁：

  ​	`>=1.5.1, <1.6.0`
  ​	`>=1.6.3, <1.7.0`
  ​	`>=1.7.2, <1.8.0`
  ​	`>=1.8.1`

  

### 漏洞分析

当用`JoinHandle::abort`中止一个任务时，如果该任务当前没有被执行，那么在调用`abort`的线程中，`Future`会被 Drop。这对于在`LocalSet`上生成的任务是不正确的。

这很容易导致竞态条件，因为许多项目在它们的`Tokio`任务中使用`Rc`或`RefCell`以获得更好的性能。

修复 PR： https://github.com/tokio-rs/tokio/pull/3934



## [RUSTSEC-2021-0070: Vulnerability in nalgebra](https://rustsec.org/advisories/RUSTSEC-2021-0070.html)

`nalgebra` 库中` VecStorage` 的`Deserialize`实现没有保持元素数量必须等于`nrows * ncols`的不变性。对特制的输入进行反序列化时，可能会允许超出向量分配的内存访问。

### 漏洞描述：

- 漏洞类型：Vulnerability
- 漏洞分类：memory-corruption/ memory-exposure
- CVE 编号：CVE-2021-32715
- 详细：https://github.com/dimforge/nalgebra/issues/883
- 补丁：`>=0.27.1`
  	

### 漏洞分析

这个缺陷是在`v0.11.0(086e6e)`中引入的，因为为`MatrixVec`增加了一个自动派生（derive）的`Deserialize`实现。`MatrixVec`后来在`v0.16.13(0f66403)`中被改名为`VecStorage`，并继续使用自动派生的`Deserialize`实现。

修复 PR ： https://github.com/dimforge/nalgebra/pull/889

在反序列化的过程中，对 `nrows.value() * ncols.value() == data.len()` 进行校验。

## [CVE-2021-31162: Vulnerability in std](https://rustsec.org/advisories/CVE-2021-31162.html)

在 Rust 1.52.0之前的Rust标准库中，如果释放元素时出现panic ，在`Vec::from_iter`函数中会出现 double free。

### 漏洞描述：

- 漏洞类型：Vulnerability
- 漏洞分类：memory-corruption 
- CVE 编号：CVE-2021-31162
- 详细：https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2021-31162
- 补丁：`>=1.52.0`
  	

### 漏洞分析

漏洞复现代码：

```rust
use std::iter::FromIterator;

#[derive(Debug)]
enum MyEnum {
    DroppedTwice(Box<i32>),
    PanicOnDrop,
}

impl Drop for MyEnum {
    fn drop(&mut self) {
        match self {
            MyEnum::DroppedTwice(_) => println!("Dropping!"),
            MyEnum::PanicOnDrop => {
                if !std::thread::panicking() {
                    panic!();
                }
            }
        }
    }
}

fn main() {
    let v = vec![MyEnum::DroppedTwice(Box::new(123)), MyEnum::PanicOnDrop];
    Vec::from_iter(v.into_iter().take(0));
}

// Output : free(): double free detected in tcache 2
```

因为枚举MyEnum在 析构的时候panic，导致资源泄漏，而引发了双重 free 的问题。

修复 PR： https://github.com/rust-lang/rust/pull/84603

在 `Vec::from_iter` 中执行 `forget_allocation_drop_remaining`，即，忘记已经被drop的`src`的元素分配的内存，即便 drop 发生了 panic，也不会泄漏资源。

