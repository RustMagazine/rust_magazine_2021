# Rust AEAD benchmark与Const generics

作者： 王江桐

> 本篇将会简要介绍什么是《This week in Rust》，[第416篇推文](https://this-week-in-rust.org/blog/2021/11/10/this-week-in-rust-416/)。
中有关Rust密码学相关的两个库，RustCrypto和Ring，中AEAD算法的性能测试，以及Const generics相关的内容。


## AEAD Benchmark

带有关联数据的认证加密（Authenticated Encryption with Associated Data）是一种能够同时保证数据的保密性、 完整性和真实性的一种加密模式。"A Conventional Authenticated-Encryption Mode"中提到，当人们将对称加密与消息认证码手动组合到一起时非常容易出错。很多实际攻击会通过这类错误导致的身份验证（包括SSL与TLS）的不正确实现或缺失，从而攻击安全协议和应用程序。因此，通常来说，比起分组加密，AEAD能提供更高的安全性。

AEAD通常有三种模式：

- Encrypt-then-MAC (EtM)，最后发送消息 = 明文加密 + 密文MAC，当MAC算法强不可伪造时是安全的。是ISO/IEC 19772:2009规定的六种认证加密方法中的一种，并且在RFC 7366中作为TLS与DTLS拓展发布。
- Encrypt-and-MAC (E&M)，最后发送消息 = 明文加密 + 明文MAC，但是E&M方法本身并未被证明是强不可伪造的。
- MAC-then-Encrypt (MtE)，最后发送消息 = 明文与明文MAC拼接之后计算MAC。尽管理论安全，但是SSL/TLS中将其实现为MAC-then-pad-then-encrypt，也就是说，明文会先填充到加密函数的块大小，之后再进行相应计算，而填充错误通常会导致接收方发现可检测到的错误，从而造成Padding oracle attack，例如Lucky Thirteen attack。

在Rust生态中，通常使用的已认证AEAD加密如下：

- RustCrypto
  - [XChaCha20-Poly1305](https://github.com/RustCrypto/stream-ciphers/tree/master/chacha20)
  - [ChaCha20-Poly1305](https://github.com/RustCrypto/stream-ciphers/tree/master/chacha20)
  - [AES-256-GCM](https://github.com/RustCrypto/AEADs/tree/master/aes-gcm)
- Ring
  - [ChaCha20-Poly1305](https://github.com/briansmith/ring)
  - [AES-256-GCM](https://github.com/briansmith/ring)

测试源码可见作者[github repo](https://github.com/skerkour/kerkour.com/tree/main/2021/benchmarking_symmetric_encryption_in_rust)。总体性能表统计如下：

|                                          | 100B                     | 1kB                      | 100kB                    | 1MB                      | 10MB                     | 100MB                    |
| :--------------------------------------- | :----------------------- | :----------------------- | :----------------------- | :----------------------- | :----------------------- | ------------------------ |
| RustCrypto’s `XChaCha20-Poly1305` v0.8.2 | 928.91 ns (102.67 MiB/s) | 1.9851 us (480.41 MiB/s) | 116.50 us (818.58 MiB/s) | 1.1579 ms (823.59 MiB/s) | 11.571 ms (824.17 MiB/s) | 117.74 ms (809.99 MiB/s) |
| RustCrypto’s `ChaCha20-Poly1305` v0.8.2  | 805.40 ns (118.41 MiB/s) | 1.8660 us (511.08 MiB/s) | 116.02 us (821.96 MiB/s) | 1.1522 ms (827.68 MiB/s) | 11.517 ms (828.02 MiB/s) | 117.87 ms (809.11 MiB/s) |
| RustCrypto’s `AES-256-GCM` v0.9.4        | 154.27 ns (618.20 MiB/s) | 910.31 ns (1.0231 GiB/s) | 84.677 us (1.0999 GiB/s) | 844.85 us (1.1023 GiB/s) | 8.4719 ms (1.0993 GiB/s) | 88.666 ms (1.0504 GiB/s) |
| ring’s `ChaCha20-Poly1305` v0.16.20      | 195.90 ns (486.81 MiB/s) | 701.99 ns (1.3267 GiB/s) | 51.594 us (1.8051 GiB/s) | 563.75 us (1.6520 GiB/s) | 5.1991 ms (1.7913 GiB/s) | 54.879 ms (1.6971 GiB/s) |
| ring’s `AES-256-GCM` v0.16.20            | 214.48 ns (444.64 MiB/s) | 455.70 ns (2.0437 GiB/s) | 26.476 us (3.5177 GiB/s) | 264.13 us (3.5260 GiB/s) | 2.6474 ms (3.5179 GiB/s) | 30.450 ms (3.0585 GiB/s) |



## Const generics

在版本1.51之后，Rust支持const generics使用。对于Rust 1.56（Rust 2021）而言，此后并没有对于Const generics进行更新，因此这个特性仍与1.51时相同。

Const generics使得用户定义struct时，可以使用常量变量定义一些类型，例如：

```Rust
struct ArrayPair<T, const N: usize> {
    left: [T; N],
    right: [T; N],
}

impl<T: Debug, const N: usize> Debug for ArrayPair<T, N> {
    // ...
}
```

`T`是一个普通的类型参数，`N`则是常量泛型。不过，当`N`不同时，实例并不属于同一个类型，例如`ArrayPair<_, 16>`并不与`ArrayPair<_, 32>`同属于同一个类型。

这样做的好处之一是，当用户想在列表上实现trait，当不能使用常量泛型时，用户必须对于每一个可能大小的列表都实现这个trait。标准库中的一些trait甚至也受此限制，在Rust 1.47之前很多trait只能在长度小于等于32的列表上实现。

同时这样做带来了另一个便利：限制trait的实现条件。通常情况下，限制可以这样达成：

```Rust
struct Assert<const COND: bool> {}

trait IsTrue {}

impl IsTrue for Assert<true> {}
```

不过同样，如果是对于列表等不定长度的类型实现trait，那么列举这些类型将会非常麻烦。常量变量可以非常快速地解决这个问题：

```Rust
/// The struct we're going to conditionally implement for
#[derive(Clone, Debug)]
struct Foo<const N: usize>{ inner: [usize; N] }

impl<const N: usize> Copy for Foo<N> where Assert::<{N < 128}>: IsTrue {}
```

这个例子限定了只有包含长度为128以下的列表的Foo实例才支持`Copy`。当长度过大时，编译器将会检测到这个错误，并给出相应告警。

另一个好处是，可以避免运行时检查。例如：

```Rust
/// A region of memory containing at least `N` `T`s.
pub struct MinSlice<T, const N: usize> {
    /// The bounded region of memory. Exactly `N` `T`s.
    pub head: [T; N],
    /// Zero or more remaining `T`s after the `N` in the bounded region.
    pub tail: [T],
}

let slice: &[u8] = b"Hello, world";
let reference: Option<&u8> = slice.get(6);
// We know this value is `Some(b' ')`,
// but the compiler can't know that.
assert!(reference.is_some())

let slice: &[u8] = b"Hello, world";
// Length check is performed when we construct a MinSlice,
// and it's known at compile time to be of length 12.
// If the `unwrap()` succeeds, no more checks are needed
// throughout the `MinSlice`'s lifetime.
let minslice = MinSlice::<u8, 12>::from_slice(slice).unwrap();
let value: u8 = minslice.head[6];
assert_eq!(value, b' ')
```

虽然Const generics提供了很多便利，目前的版本只支持数字类型，包括且不限于有符号/无符号数字、char、bool等，并且不支持非常复杂的泛型表达式。常量泛型只能由常量参数初始化，例如单一的常量参数，常量定值（例如有符号/无符号数字、char、bool），或是常量定值的表达计算式，但是不能由涉及到常量泛型的表达计算式。例如：

```Rust
fn foo<const N: usize>() {}

fn bar<T, const M: usize>() {
    foo::<M>(); // ok: `M` is a const parameter
    foo::<2021>(); // ok: `2021` is a literal
    foo::<{20 * 100 + 20 * 10 + 1}>(); // ok: const expression contains no generic parameters
    
    foo::<{ M + 1 }>(); // error: const expression contains the generic parameter `M`
    foo::<{ std::mem::size_of::<T>() }>(); // error: const expression contains the generic parameter `T`
    
    let _: [u8; M]; // ok: `M` is a const parameter
    let _: [u8; std::mem::size_of::<T>()]; // error: const expression contains the generic parameter `T`
}
```

在未来这些限制或许会被取消，Rust版本会支持新的功能，不过由于这些限制涉及到很多的设计问题，或许这个功能的拓展还需要很长的一段时间。不过由于便利性，很多三方库也实现了Rust标准库不支持的一些常量泛型类似的功能，例如[typenum](https://docs.rs/typenum/1.14.0/typenum/)。尽管它的编译性能以及编译器集成并不是特别良好，但是它仅依赖于libcore，并且支持在编译时检查的类型级数字，也就是将数字作为类型而不是值使用。



## 引用

Benchmarking symmetric encryption (AEAD) in Rust，https://kerkour.com/rust-symmetric-encryption-aead-benchmark/

Authenticated encryption，https://en.wikipedia.org/wiki/Authenticated_encryption

A Conventional Authenticated-Encryption Mode，https://csrc.nist.gov/csrc/media/projects/block-cipher-techniques/documents/bcm/proposed-modes/eax/eax-spec.pdf

Const generics MVP hits beta!，https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html

It's Time to Get Hyped About Const Generics in Rust，https://nora.codes/post/its-time-to-get-hyped-about-const-generics-in-rust/

