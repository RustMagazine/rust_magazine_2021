# 官方动态

编辑：张汉东

---

## 官方 | 来推动 GAT 稳定吧

GAT RFC 从2016年启动，到今年已经五年了，现在终于接近稳定状态了。GAT 是 Rust github 仓库里期待率最高的一个问题。

现在经过编译器的大量修改，终于让 GAT 达到了一个 「完整」状态，虽然还有一些诊断问题，但现在你在 nightly 下使用 GAT 将不会看到 “generic_associated_types is incomplete”这种错误。 但是现在想 稳定 GAT ，还需要大家来帮助测试此功能，为你发现但任何错误或潜在的诊断改进提出问题。并且官方还希望可以在 GAT 上实现一些有趣的模式。

如果不出意外，未来几个月内稳定应该会比较顺利。

详细内容请看原文吧，原文也介绍了什么是 GAT 。

[https://blog.rust-lang.org/2021/08/03/GATs-stabilization-push.html](https://blog.rust-lang.org/2021/08/03/GATs-stabilization-push.html)

## wasm-bindgen 发布新版本

虽然官方 Rust WebAssembly 工作组的博客已经停更快两年了，但是实际他们的工作还是在继续的。

最近的更新：

- 升级 webpack 示例中 npm 包的依赖版本
- 添加 `no_deref` 属性以选择不为导入的类型生成 `deref` 实现
- 通过非零初始化缓冲区来提高 `TypedArray::to_vec` 性能
- 升级最新的WebGPU WebIDL

[https://github.com/rustwasm/wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)

## Rustdoc 中源码页面中支持跳转到定义功能

比如，在标准库文档中源码（src）页面看到类似下面的代码：

```rust
mod other_module;
struct Foo;
fn bar() {}

fn x<T: other_module::Trait>(f: Foo, g: other_module::Whatever, t: &T) {
    let f: Foo = Foo;
    bar();
    f.some_method();
}
```

其中，`other_module::Trait`, `Foo`, `other_module::Whatever`, `bar` 和 `some_method` 都会出现链接，点击链接可以跳转到其定义页面。

如果有来自另一个crate 的类型，它会链接到它的文档页面而不是它的定义（但你可以点击 `[src]`）。

[https://github.com/rust-lang/rust/pull/84176](https://github.com/rust-lang/rust/pull/84176)

## [CVE-2021-29922] Rust 标准库net 模块漏洞： 前导零改变 IP 地址

本周，在DEF CON上，安全研究人员Cheng Xu、Victor Viale、Sick Codes、Nick Sahler、Kelly Kaoudis、opennota和John Jackson披露了Go和Rust语言的net模块的一个缺陷。CVE-2021-29922（针对Rust） 和 CVE-2021-29923（针对Golang）。

IP地址可以用多种格式表示，包括十六进制和整数，不过最常见的IPv4地址是用十进制格式表示的。

例如，BleepingComputer的IPv4地址以十进制格式表示为`104.20.59.209`，但同样的地址也可以以八进制格式表示为：`0150.0024.0073.0321`。

假设你得到一个十进制格式的IP地址，`127.0.0.1`，这被广泛理解为本地回环地址或`localhost`。

如果你在它前面加上一个0，应用程序是否仍应将0127.0.0.1解析为`127.0.0.1`或其他什么？在Chrome的地址栏中输入`0127.0.0.1`，浏览器会将其视为八进制格式的IP。在按下回车键或返回键时，该IP实际上变成了十进制的`87.0.0.1`，这就是大多数应用程序应该处理这种模糊的IP地址的方式。

根据IETF的原始规范，IPv4地址的部分内容如果前缀为 "0"，可以解释为八进制。

但是Go和Rust中的net模块都忽略了这一点，将部分地址视为十进制。

rust 1.52.1 `std::net` 及以下版本中IP地址输入未按八进制处理而导致不确定的 SSRF 和 RFI 漏洞。

例如，攻击者向依赖`std::net::IpAddr`的网络程序提交IP地址，可以通过输入位组的输入数据引起 SSRF；

如果位组（octet）是3位，攻击者可以提交可利用的IP地址，最小可利用的位组是08（拒绝服务），最大可利用的位组是099（拒绝服务）。

例如，攻击者可以提交`010.8.8.8`，也就是`8.8.8.8`（RFI），然而`std::net::IpAddr`将计算为`10.8.8.8`。同样，攻击者可以输入127.0.026.1，这实际上是127.0.22.1，但Rust将其计算为127.0.26.1。

- SSRF是Server-side Request Forge的缩写，中文翻译为服务端请求伪造。
- RFI 是Remote File Inclusion的缩写，客户端可控制网页包含远程文件。

受影响 Rust 版本： 1.52.1 及以下。

该漏洞已于三月份修复： [https://github.com/rust-lang/rust/pull/83652](https://github.com/rust-lang/rust/pull/83652 )

PoC 代码：

```rust
// ##!/usr/bin/env rustc
// # Authors:      https://twitter.com/sickcodes, https://twitter.com/kaoudis
// # License:      GPLv3+

use std::net::IpAddr;

fn main() {
  let addr = "127.026.0.1".parse::<IpAddr>().unwrap();
  println!("{}", addr.to_string());
  let addr1 = "127.0.026.1".parse::<IpAddr>().unwrap();
  println!("{}", addr1.to_string());
  let addr2 = "127.0.0.093".parse::<IpAddr>().unwrap();
  println!("{}", addr2.to_string());
  let addr3 = "099.0.0.01".parse::<IpAddr>().unwrap();
  println!("{}", addr3.to_string());
}

// $ rustc -o main main.rs
// $ ./main
// 127.26.0.1
// 127.0.26.1
// 127.0.0.93
// 99.0.0.1
```

- [https://github.com/sickcodes/security/blob/master/advisories/SICK-2021-015.md](https://github.com/sickcodes/security/blob/master/advisories/SICK-2021-015.md)
- [https://www.bleepingcomputer.com/news/security/go-rust-net-library-affected-by-critical-ip-address-validation-vulnerability/](https://www.bleepingcomputer.com/news/security/go-rust-net-library-affected-by-critical-ip-address-validation-vulnerability/)

相关：

黑客大会 defconf29 演讲之一 ： 烂代码、老化标准和 IPv4 解析

针对 Rust / Go  前导零改变 IP 地址相关漏洞的演讲

[https://www.youtube.com/watch?v=_o1RPJAe4kU](https://www.youtube.com/watch?v=_o1RPJAe4kU)

## Deprecate llvm_asm! 的 pr 已经被合并了

过期 `llvm_asm!`，而用新的 `asm!`来代替。

[https://github.com/rust-lang/rust/pull/87590#issuecomment-899111280](https://github.com/rust-lang/rust/pull/87590#issuecomment-899111280)

## Rust IO Safety RFC  已经被实现

这是为操作系统资源增加所有权语义的第一步！

[https://github.com/rust-lang/rust/pull/87329](https://github.com/rust-lang/rust/pull/87329)

## Gcc Rust 月报

注意：GCC Rust（gccrs） 是给 gcc 增加 Rust 前端，而现在 Rust 后端正在准备合并的是 `rustc_codegen_gcc`，是给 Rust 增加 gcc 后端。

两者目标不同。

- [https://github.com/Rust-GCC/gccrs](https://github.com/Rust-GCC/gccrs)
- [https://thephilbert.io/2021/08/02/gcc-rust-monthly-report-8-july-2021/](https://thephilbert.io/2021/08/02/gcc-rust-monthly-report-8-july-2021/)