# 华为 | 硬件加速指令实践心得

作者：胡凯

---

最近在尝试使用Rust标准库core_arch下的硬件加速指令来实现一些通用加密算法。在这些指令使用的过程中，遇到了一些实现上的问题。在此我将把解决方法总结如下，希望能对后续Rust core_arch库的使用者提供一些帮助。我将从一个最简单问题开始来介绍这些解决方法。

## 最初的需求
假设我们需要实现一个函数`func`提供给外部使用，并且在不同架构上的实现不同，应该怎么实现呢？这个问题比较简单，Rust有提供`#[cfg]`宏来区分不同的架构、操作系统等。

区分不同架构，我们可以使用`target_arch`：
```rust
// lib.rs 文件下
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use x86::func;

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use aarch64::func;
```
```rust
// x86.rs 文件下
fn func() {
    // ... 具体实现
}
```
```rust
// aarch64.rs 文件下
fn func() {
    // ... 具体实现
}
```
OK，按照以上方法即可完成对应的功能，该模块的使用者只需在对应环境上进行编译即可使用。`#[cfg]`宏还具备很多其他的选项供选择，例如`target_os`，`target_endian`等，使用者可以根据自身需求选择。

## 更进一步
假设我们要在func函数中使用不同架构下的硬件加速指令，应该怎么做呢？还是和之前一样使用`#[cfg]`宏，可以使用`target_feature`。以AES硬件加速指令为例：
```rust
// lib.rs 文件下
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "aes",
))]
mod x86;
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "aes",
))]
pub use x86::func;

#[cfg(all(
    target_arch = "aarch64",
    target_feature = "aes",
))]
mod aarch64;
#[cfg(all(
    target_arch = "aarch64",
    target_feature = "aes",
))]
pub use aarch64::func;
```
```rust
// x86.rs 文件下
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

fn func() {
    // ... 具体实现，调用core_arch\x86下的指令
}
```
```rust
// aarch64.rs 文件下
#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

fn func() {
    // ... 具体实现，调用core_arch\aarch64下的指令
}
```
这样就能使用对应架构下的指定硬件加速指令了。这里有几点需要注意：
1. 除了x86\x86_64下的硬件加速指令以外，其他的所有core_arch下的硬件加速指令现在都只能在Rust nightly版本下编译和执行。
2. 在一些架构上`target_feature`有时并不能检测出机器是否提供指定功能，编译时需要使用以下方式之一：
   ```sh
    $ RUSTFLAGS='-C target-cpu=native' cargo build
   ```
   ```sh
    $ RUSTFLAGS='-C target-feature=+aes' cargo build
   ```

## 遇到的第一个问题

我尝试将以上代码放到不同机器上编译、执行，大部分机器都能成功了。但出现少部分机器编译失败的情况（找不到func）。出现这种情况主要是因为有一些机器不支持对应的硬件加速指令。上面的代码默认在遇到指定架构时使用了硬件加速，没有注意可能有些机器不支持。于是需要做一些修改：
```rust
// lib.rs 文件下
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "aes",
))]
mod x86;
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "aes",
))]
pub use x86::func;

#[cfg(all(
    target_arch = "aarch64",
    target_feature = "aes",
))]
mod aarch64;
#[cfg(all(
    target_arch = "aarch64",
    target_feature = "aes",
))]
pub use aarch64::func;

#[cfg(not(any(
    all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "aes",
    ),
    all(
        target_arch = "aarch64",
        target_feature = "aes",
    )
)))]
mod soft;
#[cfg(not(any(
    all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "aes",
    ),
    all(
        target_arch = "aarch64",
        target_feature = "aes",
    )
)))]
pub use soft::func;
```
```rust
// x86.rs 文件下
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

fn func() {
    // ... 具体实现，调用core_arch\x86下的指令
}
```
```rust
// aarch64.rs 文件下
#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

fn func() {
    // ... 具体实现，调用core_arch\aarch64下的指令
}
```
```rust
// soft.rs 文件下
pub fn func() {
    // ... 具体实现，一般的实现方式，不使用硬件加速
}
```
这里添加了一个默认的软件实现方式，用于在指定架构上不支持硬件加速或者不是以上架构的情况下使用的func。

## 遇到的第二个问题

以上方法能够解决编译失败的问题，但是当你在某一个架构机器上进行编译后，编译出来的二进制文件在另一个不同架构机器或是不支持硬件加速指令的机器上运行时会指令异常。这里就涉及交叉编译的问题。因为`#[cfg]`是静态编译，编译出来的结果只和当前环境有关。core_arch库中提供了一种动态检测的方法，这种动态检测方法能够解决同一架构下是否支持硬件加速的情况，不需要交叉编译，但是在不同架构下仍需交叉编译解决此问题。

例如在x86\x86_64架构下提供了`is_x86_feature_detected!`宏，用于动态检测指定cpu功能，我们可以使用该宏来对代码进行改造：
```rust
// lib.rs 文件下
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use x86::func;

// aarch64 下暂无动态检测机制。

pub(crate) mod soft;
```
```rust
// x86.rs 文件下
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

pub(crate) use soft::func;

fn func() {
   if is_x86_feature_detected!("aes") {
      // ... 具体实现，调用core_arch\x86下的指令
   } else {
      func()
   }
}
```
```rust
// soft.rs 文件下
pub(crate) fn func() {
    // ... 具体实现，一般的实现方式，不使用硬件加速
}
```
这种方式可以将x86\x86_64架构下编译好的二进制文件放到另一台x86\x86_64的机器上去运行，无需交叉编译。

## 总结
以上便是我对Rust core_arch库的一些使用方法总结。更具体的使用方式可以查看core_arch下的core_arch_docs.md，里面有对整个core_arch库更为详细的介绍。