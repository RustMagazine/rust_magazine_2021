# 华为 | Rust语言中SIMD计算加速指令的使用

作者：李原

---

## 一. SIMD简介

SIMD全称Single Instruction Multiple Data，即单指令多数据流，是一种基于特定CPU指令集的计算性能优化技术。顾名思义，指的是在一条CPU指令的执行期间，可以同时进行多条数据的计算。使得在科学计算、多媒体应用等数据密集型运算场景下达到数倍乃至数十倍的性能提升。

## 二. Rust语言官方SIMD加速库介绍

Rust语言是一种可以选择不同编译后端的编程语言，当然，目前业界绝大部分Rust项目都是以编译器默认选择的llvm作为编译后端。值得注意的是，llvm本身已经集成了绝大部分主流CPU架构的包含SIMD在内的各类指令集。这为Rust语言使用SIMD提供了天然的方便，因为Rust可以在编译器甚至用户代码中以静态链接的形式直接使用llvm提供的SIMD函数接口，而不用像Go、C等语言一样由开发者自己编写汇编代码。

Rust语言在官方github项目群中提供了两个simd加速库：[stdarch](https://github.com/rust-lang/stdarch)及[stdsimd](https://github.com/rust-lang/stdsimd)。这里给出了它们的github仓库地址。stdarch以模块化的方式为每种不同的CPU架构提供了各自的专用simd加速指令集，比如x86架构的AVX、AVX512、SSE、SSE2等指令集；ARM/Aarch64平台的NEON、SVE指令集；以及RISCV、WASM等架构的simd指令集等，使用时用户必须对自己所使用的CPU架构及该架构所提供的simd指令集功能有所了解。而stdsimd则是提供了大量各平台通用的抽象simd函数接口，如向量加减乘除、位移、类型转换等。读者不必对自己所使用的硬件架构以及指令集有所了解，使用起来相对更方便，但在使用功能上会有所限制。这两个项目在功能设计的出发点上有所不同，而且各自项目的维护者也有所不同。下面会具体介绍他们的设计以及使用。

## 三. 多架构通用加速库stdsimd的使用

stdsimd提供了各平台通用的simd加速接口，其实现依赖于Rust编译器所提供的platform-intrinsic接口集合。该接口集合又是对llvm所提供的各平台指令集的封装，因此它们之间的关系应该是：

**stdsimd**   —封装→   **Rust编译器**   —封装→   **llvm**

stdsimd项目因功能尚未集成完全，目前尚未集成到Rust标准库中，读者可以通过将源码克隆到自己的项目中进行使用，或者使用stdsimd的社区版本[packed_simd](https://crates.io/crates/packed_simd_2)（在Cargo.toml文件中加入`packed_simd = { version = "0.3.4", package = "packed_simd_2" }`）。下面的使用也基于社区版本进行介绍。

packed_simd项目提供了一系列向量数据类型Simd<[T; N]>，即由N个T元素组成的向量，并为他们提供了简单易懂的的类型别名，比如f32x4类型，就代表着Simd<[f32; 4]>。packed_simd所提供的SIMD加速功能，也都是基于这种向量数据类型所实现的。

packed_simd一共提供了以下几种SIMD数据类型（element_width代表数据的大小和数量，比如32x4、64x8）：

- `i{element_width}`: 有符号整数类型
- `u{element_width}`: 无符号整数类型
- `f{element_width}`: 浮点数类型
- `m{element_width}`: bool类型
- `*{const,mut} T`: 可变或不可变SIMD类型指针

默认情况下，对向量结构的操作是“垂直”的，即它们独立于其他向量应用于每个向量通道，比如下面这个例子：

```rust
let a = i32x4::new(1, 2, 3, 4);
let b = i32x4::new(5, 6, 7, 8);
assert_eq!(a + b, i32x4::new(6, 8, 10, 12));
```

该例子声明了两个i32x4向量，并通过加法运算符重载计算出了它们的和。另一方面，“水平”的操作当然也是有提供的，比如下面的例子：

```rust
assert_eq!(a.wrapping_sum(), 10);
```

总体上，"垂直"的操作总是最快的，而"水平"的操作相对会较慢。也就是说，在计算一个数组的和时，最快的方法是使用多次"垂直"运算加一次"水平"运算，如下所示：

```rust
fn reduce(x: &[i32]) -> i32 {
    assert!(x.len() % 4 == 0);
    let mut sum = i32x4::splat(0); // [0, 0, 0, 0]
    for i in (0..x.len()).step_by(4) {
        sum += i32x4::from_slice_unaligned(&x[i..]);
    }
    sum.wrapping_sum()
}

let x = [0, 1, 2, 3, 4, 5, 6, 7];
assert_eq!(reduce(&x), 28);
```

下面再给出一些常见的用例：

```rust
// 生成元素全为0的i32x4向量:
let a = i32x4::splat(0);

// 由数组中的前4个元素生成i32x4向量:
let mut arr = [0, 0, 0, 1, 2, 3, 4, 5];
let b = i32x4::from_slice_unaligned(&arr);

// 读取向量中的元素:
assert_eq!(b.extract(3), 1);

// 替换向量中对应位置的元素:
let a = a.replace(3, 1);
assert_eq!(a, b);

// 将向量写入数组中:
let a = a.replace(2, 1);
a.write_to_slice_unaligned(&mut arr[4..]);
assert_eq!(arr, [0, 0, 0, 1, 0, 0, 1, 1]);
```

除此之外，packed_simd还提供了向量的条件运算，比如下面的代码表示根据m中元素是否为真进行向量中对应元素的+1操作：

```rust
let a = i32x4::new(1, 1, 2, 2);

// 将a中的前两个元素进行+1操作.
let m = m16x4::new(true, true, false, false);
let a = m.select(a + 1, a);
assert_eq!(a, i32x4::splat(2));
```

由此可以衍生出更灵活的使用方法，比如由两个向量中各个位置的较大值组成新的向量

```rust
let a = i32x4::new(1, 1, 3, 3);
let b = i32x4::new(2, 2, 0, 0);

// ge: 大于等于计算，生成bool元素类型的向量
let m = a.ge(i32x4::splat(2));

if m.any() {
    // 根据m中的结果选择a或b中的元素
    let d = m.select(a, b);
    assert_eq!(d, i32x4::new(2, 2, 3, 3));
}
```

以上就是stdsimd(packed_simd)的基础使用方法，该项目可以让开发者方便地通过SIMD类型数据结构享受到SIMD加速的效果。但该项目也存在一定的缺陷，比如用户必须手动选择向量的长度。因为大多数CPU架构都至少提供了128位SIMD指令集，因此选择128位的向量长度总是合理的。但当CPU提供了更高级的SIMD指令集（比如AVX512）时，选择更长的指令集会获得更好的效果。因此当开发者拥有一定的CPU架构及SIMD相关的知识储备时，使用起来会有事半功倍的效果。

## 四. 专用指令加速库stdarch的使用

stdarch已经集成到了Rust语言的标准库中，可以在代码中通过`use std::arch`语句进行使用。这里要注意的是，目前只有x86_64以及x86两种架构已经发布了stable版本，因此其他结构比如arm、aarch64等必须将Rust编译器切换到nightly版本（在命令行输入rustup default nightly命令）方可编译及使用。因此下面主要使用stable版本可用的x86_64(x86)为例进行介绍。

stdarch以静态链接的方式封装了诸多llvm所提供的SIMD指令集，并以模块的方式提供了各种主流架构下的SIMD指令集，如下所示。每种架构下可用的SIMD函数接口可以点进相应的链接进行查阅。

- [x86](https://docs.rs/core_arch/0.1.5/core_arch/x86/index.html)
- [x86_64](https://docs.rs/core_arch/0.1.5/core_arch/x86_64/index.html)
- [arm](https://docs.rs/core_arch/0.1.5/core_arch/arm/index.html)
- [aarch64](https://docs.rs/core_arch/0.1.5/core_arch/aarch64/index.html)
- [mips](https://docs.rs/core_arch/0.1.5/core_arch/mips/index.html)
- [mips64](https://docs.rs/core_arch/0.1.5/core_arch/mips64/index.html)
- [powerpc](https://docs.rs/core_arch/0.1.5/core_arch/powerpc/index.html)
- [powerpc64](https://docs.rs/core_arch/0.1.5/core_arch/powerpc64/index.html)
- [nvptx](https://docs.rs/core_arch/0.1.5/core_arch/nvptx/index.html)
- [wasm32](https://docs.rs/core_arch/0.1.5/core_arch/wasm32/index.html)

相比于stdsimd，stdarch对开发者的CPU架构知识储备有着更高的要求。因为stdarch对每个主流的CPU架构都提供了上千个不同功能的SIMD指令，开发者需要手动识别哪一条指令是自己最需要的。

比如下面这个例子：

```rust
#[cfg(
    all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "avx2"
    )
)]
fn foo() {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::_mm256_add_epi64;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::_mm256_add_epi64;

    unsafe {
        _mm256_add_epi64(...);
    }
}
```

这段代码首先使用Rust语言原生提供的CPU特征检测功能，即target_arch属性宏，来检测开发环境是否为x86_64或者x86，再使用target_feature属性宏检测avx2指令集是否可用。在以上条件都满足时，才会编译下面的foo函数。而在foo函数内部，则会根据CPU为x86_64还是x86架构选择相应的simd指令。

或者开发者可以使用动态的特征检测语句`is_x86_feature_detected!`，如下所示：

```rust
fn foo() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { foo_avx2() };
        }
    }

    // return without using AVX2
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn foo_avx2() {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::_mm256_add_epi64;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::_mm256_add_epi64;

    _mm256_add_epi64(...);
}
```

stdarch本身存在着大量类似的条件编译代码。因此相应的指令集模块只有在满足环境的需求时才可用。比如x86_64架构下可以使用`use std::arch::x86_64`语句，却不能使用`use std::arch::x86_64`或者`use std::arch::arm`语句。

下面通过一个具体的例子，即16进制编码函数的simd实现来介绍stdarch的具体使用。这个例子中主要使用了x86及x86_64下的SSE4.1指令集。

具体的代码实现如下，其中使用到的各类SIMD指令及用途都可以在注释或上文中对应模块（x86或x86_64）的链接文档中进行查阅。

```rust
fn main() {
    let mut dst = [0; 32];
    hex_encode(b"\x01\x02\x03", &mut dst);
    assert_eq!(&dst[..6], b"010203");

    let mut src = [0; 16];
    for i in 0..16 {
        src[i] = (i + 1) as u8;
    }
    hex_encode(&src, &mut dst);
    assert_eq!(&dst, b"0102030405060708090a0b0c0d0e0f10");
}

pub fn hex_encode(src: &[u8], dst: &mut [u8]) {
    let len = src.len().checked_mul(2).unwrap();
    assert!(dst.len() >= len);

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("sse4.1") {
            return unsafe { hex_encode_sse41(src, dst) };
        }
    }

    hex_encode_fallback(src, dst)
}

#[target_feature(enable = "sse4.1")]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn hex_encode_sse41(mut src: &[u8], dst: &mut [u8]) {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    // 生成包含16个int8类型的向量，并将全部值设为字符'0'的ascii编号
    let ascii_zero = _mm_set1_epi8(b'0' as i8);
    // 生成包含16个int8类型的向量，并将全部值设为整数9
    let nines = _mm_set1_epi8(9);
    // 生成包含16个int8类型的向量，并将全部值设为字符'a'的ascii编号减去10
    let ascii_a = _mm_set1_epi8((b'a' - 9 - 1) as i8);
    // 生成包含16个int8类型的向量，并将全部值设为二进制数00001111
    let and4bits = _mm_set1_epi8(0xf);

    let mut i = 0_isize;
    while src.len() >= 16 {
        // 从指针中读取128位整数，组成一个128位的向量（可以转化为int8x16、int32x4等形式的向量）
        let invec = _mm_loadu_si128(src.as_ptr() as *const _);
		
        // 将该128位向量类型转化为int8x16类型的向量，并将其中每个元素和二进制数00001111进行与操作
        let masked1 = _mm_and_si128(invec, and4bits);
        // 将该128位向量类型转化为int8x16类型的向量，再将每个元素逻辑右移4位，随后将其中每个元素和二进制数00001111进行与操作
        let masked2 = _mm_and_si128(_mm_srli_epi64(invec, 4), and4bits);

        // 向量对应元素比较大小，获取向量中所有大于9的元素的位置
        let cmpmask1 = _mm_cmpgt_epi8(masked1, nines);
        let cmpmask2 = _mm_cmpgt_epi8(masked2, nines);
		
        // _mm_blendv_epi8表示生成一个新的向量，该向量中的元素是根据cmpmask1中对应位置是否为true选择ascii_zero或者ascii_a中的元素
        // _mm_add_epi8则表示向量对应位置元素相加，结果表示最终生成的十六进制编码的ascii编号
        let masked1 = _mm_add_epi8(
            masked1,
            _mm_blendv_epi8(ascii_zero, ascii_a, cmpmask1),
        );
        let masked2 = _mm_add_epi8(
            masked2,
            _mm_blendv_epi8(ascii_zero, ascii_a, cmpmask2),
        );

        // 生成一个新的向量，其中偶数位置元素（从0开始）来自于masked2，奇数位置元素来自于masked1
        // 该向量共有256位，所以将前128位放入res1中，后128位放入res2中
        let res1 = _mm_unpacklo_epi8(masked2, masked1);
        let res2 = _mm_unpackhi_epi8(masked2, masked1);

        // 将结果向量写入目标指针中
        _mm_storeu_si128(dst.as_mut_ptr().offset(i * 2) as *mut _, res1);
        _mm_storeu_si128(
            dst.as_mut_ptr().offset(i * 2 + 16) as *mut _,
            res2,
        );
        src = &src[16..];
        i += 16;
    }

    let i = i as usize;
    hex_encode_fallback(src, &mut dst[i * 2..]);
}

fn hex_encode_fallback(src: &[u8], dst: &mut [u8]) {
    fn hex(byte: u8) -> u8 {
        static TABLE: &[u8] = b"0123456789abcdef";
        TABLE[byte as usize]
    }

    for (byte, slots) in src.iter().zip(dst.chunks_mut(2)) {
        slots[0] = hex((*byte >> 4) & 0xf);
        slots[1] = hex(*byte & 0xf);
    }
}
```

此处通过这个具体的例子简单呈现了stdarch中SIMD加速指令的用法。可以看出，专用指令的使用相比于stdsimd来说对开发者的SIMD经验要求高上许多，但提供的功能和适用的场景也会更加完备。

以上就是Rust语言中官方SIMD加速库的简单使用介绍，希望能对各位读者的学习开发有所启示和帮助。
