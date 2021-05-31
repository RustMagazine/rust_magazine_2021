# Rust生态安全漏洞总结系列 | Part 2

作者：张汉东

---

本系列主要是分析[`RustSecurity` 安全数据库库](https://rustsec.org/advisories/)中记录的`Rust`生态社区中发现的安全问题，从中总结一些教训，学习`Rust`安全编程的经验。

本期分析了下面六个安全问题：

- RUSTSEC-2021-0067 : Cranelift 模块中代码生成缺陷导致可能的 WASM 沙箱逃逸
- RUSTSEC-2021-0054：rkyv crate 可能包含未初始化的内存
- RUSTSEC-2021-0041：parse_duration 通过用太大的指数解析 Payload 来拒绝服务(DOS)
- RUSTSEC-2021-0053： 算法库中 `merge_sort::merge()` 导致实现 Drop 的类型 双重释放（ double-free）
- RUSTSEC-2021-0068: iced x86 版本中 不合理（Soundness） 的问题
- RUSTSEC-2021-0037： Diesel 库的 Sqlite 后端 UAF(use-after-free) bug

看是否能给我们一些启示。

## RUSTSEC-2021-0067 : Cranelift 模块中代码生成缺陷导致可能的 WASM 沙箱逃逸

在 Cranelift 中发现了一个漏洞。具有未知输入的操作导致特权升级漏洞。 CWe正在将问题分类为CWE-264。 这将对机密性，完整性和可用性产生影响。

### 漏洞描述：

- 漏洞类型：Vulnerability
- 漏洞分类：code-execution/ memory-corruption/ memory-exposure
- CVE 编号：CVE-2021-32629
- 详细：[https://github.com/bytecodealliance/wasmtime/security/advisories/GHSA-hpqh-2wqx-7qp5](https://github.com/bytecodealliance/wasmtime/security/advisories/GHSA-hpqh-2wqx-7qp5)
- 影响架构：x86
- 补丁：`>=0.73.1` 和 `>=0.74.0`

Cranelift X64后端的`0.73.0`中有一个错误，可以创建一个可能导致 Webassembly 模块中的潜在沙箱逃逸(sandbox escape )的场景。 版本`0.73.0`的Cranelift的用户应升级到`0.73.1`或`0.74`，以修复此漏洞。 

如果未使用旧的默认后端，则在`0.73.0`之前的 Cranelift 用户应该更新为`0.73.1`或`0.74`。

### 漏洞分析


此问题是在 Cranelift 新后端中引入的（Cranelift 经历过大的重构）。

> 一些背景： 寄存器分配
> 
> 如果物理寄存器的数量不足以满足虚拟寄存器的需求，有些虚拟寄存器显然就只能映射到内存。这些虚拟寄存器称为溢出（spill）虚拟寄存器。寄存器分配算法的好坏直接决定了程序中寄存器的利用率。
>
> Cranelift 寄存器分配相关文章：[https://cfallin.org/blog/2021/03/15/cranelift-isel-3/](https://cfallin.org/blog/2021/03/15/cranelift-isel-3/)
> 
> 该文章还详细介绍了该团队如何保证 Cranelift 生成正确的代码。即便如此，还是产生了逻辑 Bug。

这个 Bug 是一个逻辑 Bug:

原因是，寄存器分配器重新加载比 64位 窄的溢出（spill）整数值时，从栈上加载的值执行了符号扩展而不是零扩展。 

这对另一个优化产生了糟糕的影响：当我们知道产生32位值的指令实际上将其目标寄存器的高32位置零时，指令选择器将选择一个32到64位的零扩展运算符。因此，我们依赖于这些归零位，但值的类型仍然是I32，并且溢出/重新加载将这些比特位重构为I32的MSB的符号扩展。

所以，在某些特定情况下，如果i32值为指针，则可能会出现沙箱逃逸的情况。为堆访问发出的常规代码对 WebAssembly 堆地址进行零扩展，将其添加到64位堆基，然后访问结果地址。如果零扩展成为符号扩展，则模块可以在堆开始之前向后访问并访问最大2GiB的内存。

> 符号扩充 (sign-extend): 指在保留数字的符号（正负性）及数值的情况下，增加二进制数字位数的操作。
>
> 零扩充（zero-extend）：用于将无符号数字移动至较大的字段中，同时保留其数值。

该 Bug 的影响力依赖于堆的实现。具体而言：

如果堆有边界检查。并且，不完全依赖于保护页面。并且堆绑定为2GiB或更小。则该 Bug 无法用于从另一个 WebAssembly 模块堆访问内存。

如果使用此 Bug 可访问的范围中没有映射内存，例如，如果 WebAssembly 模块堆之前有 2 GiB 保护区域，则可以减轻此漏洞的影响。

- 修复 PR: [https://github.com/bytecodealliance/wasmtime/pull/2919/files](https://github.com/bytecodealliance/wasmtime/pull/2919/files)
- 点击查看详细内容，了解对 lucet 和 wastmtime的影响：[https://github.com/bytecodealliance/wasmtime/security/advisories/GHSA-hpqh-2wqx-7qp5](https://github.com/bytecodealliance/wasmtime/security/advisories/GHSA-hpqh-2wqx-7qp5)

## RUSTSEC-2021-0054：rkyv crate 可能包含未初始化的内存


### 漏洞描述：

- 漏洞类型：Vulnerability
- 漏洞分类： memory-exposure
- CVE 编号：无
- 详细：[https://github.com/djkoloski/rkyv/issues/113](https://github.com/djkoloski/rkyv/issues/113)
- 补丁：`>=0.6.0`

rkyv是一个序列化框架 在序列化期间，可能无法初始化结构填充字节和未使用的枚举字节。 这些字节可以写入磁盘或发送不安全的通道。

### 漏洞分析

补丁代码：[https://github.com/djkoloski/rkyv/commit/9c65ae9c2c67dd949b5c3aba9b8eba6da802ab7e](https://github.com/djkoloski/rkyv/commit/9c65ae9c2c67dd949b5c3aba9b8eba6da802ab7e)

有问题的代码：

```rust
unsafe fn resolve_aligned<T: Archive + ?Sized>(
        &mut self,
        value: &T,
        resolver: T::Resolver,
    ) -> Result<usize, Self::Error> {
    // ...
    let mut resolved = mem::MaybeUninit::zeroed();
    // ...
}
```

`mem::MaybeUninit::zeroed()`函数会创建一个新的`MaybeUninit<T>`实例，并且该内存位会被填充`0`。但是这依赖于 `T`是否能被正确初始化。比如：`MaybeUninit<usize>::zeroed()`是初始化，但是`MaybeUninit<&'static i32>::zeroed()`就没有被正确初始化。这是因为 Rust 里引用不能为空。

所以，现在这个 resolver 是个泛型 `T`，不一定能正确初始化，所以有未初始化的风险。

修复之后的代码：

```rust
    let mut resolved = mem::MaybeUninit::<T::Archived>::uninit();
    resolved.as_mut_ptr().write_bytes(0, 1);
```

直接假设其没有正确初始化，然后使用`write_bytes`手工将其初始化，确保正确。

## RUSTSEC-2021-0041：parse_duration 通过用太大的指数解析 Payload 来拒绝服务(DOS)

### 漏洞描述：

- 漏洞类型：Vulnerability
- 漏洞分类： denial-of-service
- CVE 编号：CAN-2021-1000007 / CVE-2021-29932
- 详细：[https://github.com/zeta12ti/parse_duration/issues/21](https://github.com/zeta12ti/parse_duration/issues/21)
- 补丁：无，作者放弃维护

### 漏洞解析

parse_duration 库用来将字符串解析为持续时间（duration）。

问题代码：

```rust
if exp < 0 {
    boosted_int /= pow(BigInt::from(10), exp.wrapping_abs() as usize);
} else {
    boosted_int *= pow(BigInt::from(10), exp.wrapping_abs() as usize);
}
duration.nanoseconds += boosted_int;
```

此为 parse 函数内的代码片段，允许使用指数级的持续时间字符串解析，其中BigInt 类型与 pow 功能一起用于这类 Payload。该功能会导致长时间占用CPU和内存。

这允许攻击者使用 parse 功能来制造 DOS 攻击。虽然该库已经不维护了，而且star数也不多，但是不清楚依赖它的库有多少，可以使用 cargo-audit 来检查你项目里的依赖。

## RUSTSEC-2021-0053： 算法库中 `merge_sort::merge()` 导致实现 Drop 的类型 双重释放（ double-free）

- 漏洞类型：Vulnerability
- 漏洞分类： memory-corruption
- CVE 编号：无
- 详细：[https://github.com/AbrarNitk/algorithmica/issues/1](https://github.com/AbrarNitk/algorithmica/issues/1)
- 补丁：暂无

### 漏洞分析

[algorithmica](https://github.com/AbrarNitk/algorithmica)是 Rust 实现算法的教学库，网站为：[https://www.fifthtry.com/abrar/rust-algorithms/](https://www.fifthtry.com/abrar/rust-algorithms/)。

该库中的归并排序的实现中，merge 函数导致 对列表元素持有双份所有权，所以会双重释放（double free）。

注意下面源码中，为 unsafe rust 实现。

```rust
 fn merge<T: Debug, F>(list: &mut [T], start: usize, mid: usize, end: usize, compare: &F) 
 where 
     F: Fn(&T, &T) -> bool, 
 { 
     let mut left = Vec::with_capacity(mid - start + 1); 
     let mut right = Vec::with_capacity(end - mid); 
     unsafe { 
         let mut start = start; 
         while start <= mid { 
             left.push(get_by_index(list, start as isize).read()); 
             start += 1; 
         } 
         while start <= end { 
             right.push(get_by_index(list, start as isize).read()); 
             start += 1; 
         } 
     } 
  
     let mut left_index = 0; 
     let mut right_index = 0; 
     let mut k = start; 
  
     unsafe { 
         while left_index < left.len() && right_index < right.len() { 
             if compare(&left[left_index], &right[right_index]) { 
                 
                 // 通过 `list[k] = ` 这种方式重复持有元素所有权
                 list[k] = get_by_index(&left, left_index as isize).read(); 
                 
                 left_index += 1; 
             } else { 
                 list[k] = get_by_index(&right, right_index as isize).read(); 
                 right_index += 1; 
             } 
             k += 1; 
         } 
  
         while left_index < left.len() { 
             list[k] = get_by_index(&left, left_index as isize).read(); 
             left_index += 1; 
             k += 1; 
         } 
  
         while right_index < right.len() { 
             list[k] = get_by_index(&right, right_index as isize).read(); 
             right_index += 1; 
             k += 1; 
         } 
     } 
 } 

unsafe fn get_by_index<T>(list: &[T], index: isize) -> *const T {
    let list_offset = list.as_ptr();
    list_offset.offset(index)
}
```

Bug 复现：

```rust
#![forbid(unsafe_code)]
use algorithmica::sort::merge_sort::sort;

fn main() {
    let mut arr = vec![
        String::from("Hello"),
        String::from("World"),
        String::from("Rust"),
    ];

    // Calling `merge_sort::sort` on an array of `T: Drop` triggers double drop
    algorithmica::sort::merge_sort::sort(&mut arr);
    dbg!(arr);
}
```

输出： 

```rust
free(): double free detected in tcache 2

Terminated with signal 6 (SIGABRT)
```

该 Bug 还未得到修复。

此问题给我们的启示：不要为了刷题而忽略安全。

## RUSTSEC-2021-0068: iced x86 版本中 不合理（Soundness） 的问题

### 漏洞描述：

- 漏洞类型：Vulnerability
- 漏洞分类： soundness
- CVE 编号：无
- 详细：[https://github.com/icedland/iced/issues/168](https://github.com/icedland/iced/issues/168)
- 补丁：`>1.10.3`

### 漏洞分析

iced 用户在使用 miri 编译其项目时，发现 UB:

```rust
error: Undefined Behavior: memory access failed: pointer must be in-bounds at offset 4, but is outside bounds of alloc90797 which has size 3
    --> C:\Users\lander\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\slice\mod.rs:365:18
     |
365  |         unsafe { &*index.get_unchecked(self) }
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ memory access failed: pointer must be in-bounds at offset 4, but is outside bounds of alloc90797 which has size 3
     |
     = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
     = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
             
     = note: inside `core::slice::<impl [u8]>::get_unchecked::<usize>` at C:\Users\lander\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\slice\mod.rs:365:18
     = note: inside `iced_x86::Decoder::new` at C:\Users\lander\.cargo\registry\src\github.com-1ecc6299db9ec823\iced-x86-1.9.1\src\decoder\mod.rs:457:42
note: inside `Emulator::run` at src\lib.rs:563:27
    --> src\lib.rs:563:27
     |
563  |         let mut decoder = Decoder::new(self.bitness, bytes, self.decoder_options);
```

该用户在使用 `Decoder::new` 的时候出现了 UB。在 iced相关源码中，即 `iced/src/rust/iced-x86/src/decoder.rs` 中，存在

```rust
let data_ptr_end: *const u8 = unsafe { 
    data.get_unchecked(data.len()) 
}; 
```

根据[标准库文档](https://doc.rust-lang.org/std/primitive.slice.html#method.get_unchecked)描述：

> Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used.
>
> 使用 界外索引调用该方法就是 未定义行为（UB），即便这个结果的引用没有被使用。

示例：

```rust
let x = &[1, 2, 4];

unsafe {
    assert_eq!(x.get_unchecked(1), &2);
    assert_eq!(x.get_unchecked(3), &2); // UB
}
```

该代码已经被修复为，不再使用 get_unchecked ：

```rust
let data_ptr_end = data.as_ptr() as usize + data.len();
```


### RUSTSEC-2021-0037： Diesel 库的 Sqlite 后端 UAF(use-after-free) bug

### 漏洞描述：

- 漏洞类型：Vulnerability
- 漏洞分类： memory-corruption
- CVE 编号：CVE-2021-28305
- 详细：[https://github.com/diesel-rs/diesel/pull/2663](https://github.com/diesel-rs/diesel/pull/2663)
- 补丁：`>=1.4.6`

### 漏洞分析

Diesel 的 sqlite 后端使用了 libsqlite3_sys 这个库来调用 sqlite 提供的sql函数。比如`sqlite3_finalize` 和 `sqlite3_step` 之类。

> sqlite 函数执行调用过程：
> - sqlite3_open()
> - sqlite3_prepare()
> - sqlite3_step() // 用于执行有前面sqlite3_prepare创建的 预编译语句
> - sqlite3_column() // 从执行sqlite3_step()执行一个预编译语句得到的结果集的当前行中返回一个列
> - sqlite3_finalize() // 销毁前面被sqlite3_prepare创建的预编译语句
> - sqlite3_close()


Diesel 的 by_name 查询通用做法是将预编译语句的所有字段名称保存为字符串切片以备以后使用。

但是sqlite的行为是：

- 返回的字符串指针一直有效，直到准备好的语句被 `sqlite3_finalize()` 销毁，
- 或者直到第一次调用 `sqlite3_step()` 为特定运行自动重新预编译该语句，
- 或者直到下一次调用 `sqlite3_column_name()` 或 `sqlite3_column_name16()` 在同一列。

在之前版本的 Diesel 中，没有注意到这种情况，在调用 `sqlite3_step()` 之后，因为重新预编译语句，导致之前字符串切片指针就无效了。就造成 UAF 的情况。

这个案例告诉我们，在使用 FFi 的时候，要注意绑定sys库 的相关行为。这个在 Rust 编译器这边是无法检查发现的，案例应该属于逻辑 Bug。