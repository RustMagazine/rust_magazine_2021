# 华为 | Rust 编译后二进制大小和常用优化方式

作者： 周紫鹏 / 后期编辑：张汉东

---

#### 背景介绍

Rust编译后的可执行文件大小一直是大家谈论[比较多的问题](https://stackoverflow.com/questions/29008127/why-are-rust-executables-so-huge)，对于嵌入式单板空间有限的场景下，太大的可执行文件往往是不可接受的。当前的项目也经常会因为几K的可执行文件增大而进行优化。

本篇文章对比Rust和C语言可执行文件大小和组成，并尝试提供一些有效的优化方式。Rust选择的是[Tokio v1.5.0](https://github.com/tokio-rs/tokio/releases/tag/tokio-1.5.0)作为测试对象，C语言则选择公司内部某项目组模块作为测试对象。


#### Rust生成二进制类型介绍

Rust支持生成多种格式的动态库和静态库，在Cargo.toml文件中，新增[lib]段指定[crate-type](https://doc.rust-lang.org/reference/linkage.html)就可以进行配置。

```rust
[lib]
crate-type = ["dylib"]
```

- [crate_type = "bin"]

  生成可执行文件，crate中必须要有main函数作为入口，如果crate中已经有main函数，其实不需要在toml文件中显示指定。生成的可执行文件中，会包含所有Rust相关的库和依赖。也就是生成的可执行文件可以在没有安装Rust环境的机器上运行。

- [crate_type = "lib"]

  生成一个Rust库，但是具体的形态会根据不同的编译器来生成对应的lib库，生成的库是给rustc使用的，所以这个库的形式也会跟着rustc的变化而变化。

- [crate_type = "dylib"]

  生成一个动态的Rust库（Linux 上为 .so，MacOS 上为 .dylib， Windows 上为 .dll），生成的动态库可以作为其他库或者可执行文件的依赖库。该动态库会包含Rust的一些特定段，如.rustc等。

- [crate_type = "staticlib"]

  生成一个静态库（Linux\MacOS 上为 .a，Windows 上为 .lib）,Rust编译器不会链接staticlib生成的静态库，因为该静态库会包含Rust库和依赖的第三方库，一般适合作为独立的Rust库实现提供给第三方，和bin的区别是，没有携带main函数。

- [crate_type = "cdylib"]

  C类型的动态库，与 dylib 类似，也会生成 .so, .dylib 或 .dll 文件，但是生成的为C-ABI格式的二进制，可以提供给C语言作为FFI调用。

- [crate_type = "rlib"]

  Rust lib文件，由于当前Rust的二进制格式是不稳定的，所以当前Rust还是使用源码集成一起编译的方式来进行构建，当前没有办法通过Cargo.toml的方式依赖编译好的SO、*.rlb或者.a。rlib作为Rust编译生成的中间二进制文件，会携带很多Rust语言相关的信息，最终是作为rustc的输入。在编译的过程中，可以在`target\release\deps`下看到依赖的三方库被编译成rlib。

- [crate_type = "proc-macro"]

   不会产生特定类型的库文件，Rust过程宏使用需要独立的crate，其他库通过依赖指定的`proc-macro`库进行使用。

本次分析主要以dylib库方式进行，避免引入第三方库依赖的影响。

#### 可执行文件组成

[Tokio v1.5.0](https://github.com/tokio-rs/tokio/releases/tag/tokio-1.5.0)中tokio模块的代码(NBNC)有36,473行，使用[tokei](https://crates.io/crates/tokei)工具进行统计的结果。

在`tokio\tokio\Cargo.toml`文件中添加`crate-type = ["dylib"]`，指定编译结果为动态库形式。

- 使用`cargo build --release`编译

  生成的libtokio.so大小为`5,385,736`字节，每个段的分布如下。第二列为段名称，第三列为段大小，最后一列为每千行代码包含的二进制大小。段的大小单位都为字节。

| [Nr]  | Section Name      | Section Size | Section Size / KLOC |
| ----- | ----------------- | ------------ | ------------------- |
| [ 1]  | .hash             | 12,496       | 347                 |
| [  2] | .gnu.hash         | 12,928       | 359                 |
| [  3] | .dynsym           | 50,376       | 1,399               |
| [  4] | .dynstr           | 194,040      | 5,390               |
| [  5] | .gnu.version      | 4,198        | 117                 |
| [  6] | .gnu.version_r    | 256          | 7                   |
| [  7] | .rela.dyn         | 59,616       | 1,656               |
| [  8] | .rela.plt         | 48           | 1                   |
| [  9] | .init             | 26           | 1                   |
| [10]  | .plt              | 48           | 1                   |
| [11]  | .plt.got          | 16           | 0                   |
| [12]  | .text             | 689,517      | 19,153              |
| [13]  | .fini             | 9            | 0                   |
| [14]  | .rodata           | 31,222       | 867                 |
| [15]  | .eh_frame_hdr     | 38,660       | 1,074               |
| [16]  | .eh_frame         | 179,868      | 4,996               |
| [17]  | .gcc_except_table | 28,468       | 791                 |
| [18]  | .tdata            | 56           | 2                   |
| [19]  | .tbss             | 211          | 6                   |
| [20]  | .init_array       | 8            | 0                   |
| [21]  | .fini_array       | 8            | 0                   |
| [22]  | .data.rel.ro      | 31,304       | 870                 |
| [23]  | .dynamic          | 576          | 16                  |
| [24]  | .got              | 5,008        | 139                 |
| [25]  | .data             | 168          | 5                   |
| [26]  | .bss              | 160          | 4                   |
| [27]  | .comment          | 17           | 0                   |
| [28]  | .rustc            | 3,318,060    | 92,168              |
| [29]  | .debug_aranges    | 128          | 4                   |
| [30]  | .debug_info       | 68           | 2                   |
| [31]  | .debug_abbrev     | 36           | 1                   |
| [32]  | .debug_line       | 197          | 5                   |
| [33]  | .debug_str        | 107          | 3                   |
| [34]  | .debug_ranges     | 128          | 4                   |
| [35]  | .symtab           | 185,592      | 5,155               |
| [36]  | .strtab           | 539,047      | 14,974              |
| [37]  | .shstrtab         | 342          | 10                  |

从表格中可以看到，release中仍然存在调试相关信息，包括符号表信息。针对调测信息，我们对SO进一步进行strip。

- **strip**

  strip命令可以将29到37的调测信息段删除，删除之后的libtokio.so大小为`4,659,816`，仍有4.5M左右的大小。

- **.rustc段**

  .rustc段大概占了整体大小的60%，关于.rustc段的作用是这样的，由于动态库dylib采用Rust ABI，目前这个ABI尚不稳定，需要.rustc这一节来附加额外的版本控制信息，在最终的可执行文件中不会存在rustc段。可以通过`strip libtokio.so -R .rustc`将.rustc段删除，删除之后的大小为`1,341,680`大小为1.3M 左右。

- **各段占比以及和C的对比**

| tokio数据 |                       |             |            |            | C语言数据  |            |               |
| --------- | --------------------- | ----------- | ---------- | ---------- | ---------- | ---------- | ------------- |
| 序号      | 段                    | 段大小      | 每千行大小 | 百分比     | 百分比     | 每千行大小 | 段            |
| [  1]     | .hash                 | 12,496      | 347        | 0.93%      | 1.94%      | 270        | .hash         |
| [  2]     | .gnu.hash             | 12,928      | 359        | 0.96%      | 2.25%      | 313        | .gnu.hash     |
| [  3]     | .dynsym               | 50,376      | 1,399      | 3.75%      | 7.26%      | 1,010      | .dynsym       |
| **[  4]** | **.dynstr**           | **194,040** | **5,390**  | **14.46%** | **6.01%**  | **836**    | **.dynstr**   |
| [  7]     | .rela.dyn             | 59,616      | 1,656      | 4.44%      | 5.53%      | 769        | .rela.dyn     |
| **[12]**  | **.text**             | **689,517** | **19,153** | **51.39%** | **50.09%** | **6,965**  | **.text**     |
| [14]      | .rodata               | 31,222      | 867        | 2.33%      | 8.34%      | 1,159      | .rodata       |
| [15]      | .eh_frame_hdr         | 38,660      | 1,074      | 2.88%      | 1.87%      | 261        | .eh_frame_hdr |
| **[16]**  | **.eh_frame**         | **179,868** | **4,996**  | **13.41%** | **10.05%** | **1,397**  | **.eh_frame** |
| [17]      | **.gcc_except_table** | **28,468**  | **791**    | **2.12%**  |            |            |               |
| [22]      | .data.rel.ro          | 31,304      | 870        | 2.33%      | 0.01%      | 2          | .data.rel.ro  |
| [24]      | .got                  | 5,008       | 139        | 0.37%      | 1.56%      | 217        | .got          |
|           |                       |             | **37042**  |            |            | **13,198** |               |

表格中C采用`-O2`优化等级，并通过strip之后的数据。按照经验值来看，每千行C代码编译出的二进制大小大概在13K左右。从表格对比来看，Rust编译出来的可执行文件大概是C语言的3倍。最主要增大点在.text段和.dynstr段。其中tokio比C多了.gcc_except_table段，该段和try-catch-finally 控制流块的异常相关，部分信息用于处理异常，其他信息用于清除代码（即：在展开堆栈时调用对象析构函数）。



- **.dynsym**  

  这一节存储的是关于动态链接的符号表，每一个表项占24字节，tokio总共有2099个动态符号，相比较于C，Rust会存在更多的库函数、数据结构和异常处理等。

  ```
  //Rust dynsym符号表
  Symbol table '.dynsym' contains 2099 entries:
     Num:    Value          Size Type    Bind   Vis      Ndx Name
       0: 0000000000000000     0 NOTYPE  LOCAL  DEFAULT  UND 
       1: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND _ZN3std3net3tcp9TcpStream
       2: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND _ZN3std2fs8DirEntry9file_
       3: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND _ZN4core3fmt3num53_$LT$im
       4: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND _ZN4core3fmt3num53_$LT$im
       5: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND _ZN51_$LT$$RF$std..fs..Fi
       6: 0000000000000000     0 OBJECT  GLOBAL DEFAULT  UND _ZN3std10std_detect6detec
       7: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND _ZN3std3sys4unix6thread6T
       8: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND _ZN4core3fmt3num52_$LT$im
       9: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND pipe2@GLIBC_2.9 (2)
      10: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND _ZN91_$LT$std..io..cursor
      11: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND _ZN74_$LT$std..fs..DirEnt
      12: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND _ZN4core6option13expect_f
      13: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND _ZN3std3net4addr12SocketA
      14: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND _ZN3std4path4Path5_join17
      15: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND _ZN4core3fmt3num53_$LT$im
      ....
  ```

  ```
  //C dynsym符号表
     Num:    Value          Size Type    Bind   Vis      Ndx Name
       0: 0000000000000000     0 NOTYPE  LOCAL  DEFAULT  UND 
       1: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND free@GLIBC_2.2.5 (2)
       2: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND __isoc99_fscanf@GLIBC_2.7 (3)
       3: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND puts@GLIBC_2.2.5 (2)
       4: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND clock_gettime@GLIBC_2.17 (4)
       5: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND fclose@GLIBC_2.2.5 (2)
       6: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND printf@GLIBC_2.2.5 (2)
       7: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND __assert_fail@GLIBC_2.2.5 (2)
       8: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND __libc_start_main@GLIBC_2.2.5 (2)
       9: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND feof@GLIBC_2.2.5 (2)
      10: 0000000000000000     0 NOTYPE  WEAK   DEFAULT  UND __gmon_start__
      11: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND memcpy@GLIBC_2.14 (5)
      12: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND malloc@GLIBC_2.2.5 (2)
      13: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND fopen@GLIBC_2.2.5 (2)
  ```

  

- **.dynstr**

  dynstr段用来存储dysym符号表中的符号，本次测试使用的是rustc 1.48.0，组名规则为[legacy](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_symbol_mangling/index.html#the-rust-linkage-model-and-symbol-names)，类似于C++的组名规则，符号名中间会加上crate、mod、struct等信息，想比于C语言的组名要大很多。

  当前nightly版本支持了新的组名规则，[V0规则](https://github.com/rust-lang/rfcs/blob/master/text/2603-rust-symbol-name-mangling-v0.md)，新的规则会删除符号最后的哈希值，但是组名之后的符号仍然是很长的。

  ```
  //Rust 字符串表
  String dump of section '.dynstr':
    [     1]  libstd-f14aca24435a5414.so
    [    1c]  _ITM_deregisterTMCloneTable
    [    38]  __gmon_start__
    [    47]  _Jv_RegisterClasses
    [    5b]  _ITM_registerTMCloneTable
    [    75]  _ZN58_$LT$std..io..error..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17heb882e9e5723aaeaE
    [    cd]  _ZN244_$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$GT$..from..StringError$u20$as$u20$core..fmt..Display$GT$3fmt17h0381a183d16c0bdbE
    [   1e0]  _ZN3std2rt19lang_start_internal17h73711f37ecfcb277E
    [   214]  _ZN56_$LT$std..io..Guard$u20$as$u20$core..ops..drop..Drop$GT$4drop17h17ecb6f4aa594fe8E
    [   26b]  _ZN4core6result13unwrap_failed17he7cdc7a46f93cfbeE
    [   29e]  _ZN3std2fs11OpenOptions4read17hb9e61755aa4c5dd0E
  
  ```

  ```
  //C 字符串表
  String dump of section '.dynstr':
    [     1]  libc.so.6
    [     b]  fopen
    [    11]  puts
    [    16]  __assert_fail
    [    24]  printf
    [    2b]  feof
    [    30]  __isoc99_fscanf
    [    40]  memcpy
    [    47]  fclose
    [    4e]  malloc
    [    55]  clock_gettime
  ```

- .text段

  最后再打开看看最大头的代码段。.text段大概也是C的三倍左右大小，通过汇编指令打开查看，Rust比C多出点在异常处理、调用栈、析构函数、泛型实例化、Vec，Result，Box，String，Map等结构的处理、运行时边界校验等。



#### 优化方式

上述我们只是用`cargo build --release`的方式进行了代码的优化，当然Rust编译器还提供了不同的优化手段。本节还是基于tokio，介绍常用的二进制优化手段。

| 优化手段                    | 二进制大小（字节） |
| --------------------------- | ------------------ |
| debug模式编译               | 22,287,016         |
| release模式编译             | 5,385,736          |
| strip之后大小               | 4,659,816          |
| strip libtokio.so -R .rustc | 1,341,680          |
| codegen-units = 1           | 1,046,768          |
| panic = 'abort'             | 未测试             |
| Optimize libstd with Xargo  | 未测试             |

cargo支持的性能和二进制大小优化选项可以参见[这里](https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles)。

- [codegen-units](https://doc.rust-lang.org/rustc/codegen-options/index.html#codegen-units) 

  其中codegen-units = 1优化效果比较明显。该选项用来将crate分割成多个代码生成单元，当生成多个代码单元时，LLVM会并行的来处理，减少编译的时间。如果将codegen-units设置为1的时候，可以提升代码的运行速度，和减少生成的可执行文件，但是会大大增加编译的时间开销。在仅使用release时tokio编译时间为25s，在设置codegen-units = 1的时候，编译时间为39s，大概增加了**60%**的时间。默认情况下全量编译设置的值为16，增量编译下设置的值为256。

- [min-sized-rust](https://github.com/johnthagen/min-sized-rust)

  该仓中介绍了几种常用的优化方式，但是尝试使用`opt-level = 'z' `和`lto = true`两个选型对tokio最终生成的二进制并没有影响，当然这两个选项对性能有一定的提升。

  Jemalloc在1.32版本已经被删除。

  panic = 'abort'添加之后编译失败，正常Rust在panic的时候，会记录调用栈，如果改为panic='abort'之后，将会直接退出，而不会打印异常信息。

  其他优化手段，如重新编译libstd、#![no_std]不使用标准库，也没有在本次测试范围内。

#### 结论

Rust由于其组名规则和语言特性等原因，在使用了各种优化之后，编译出来的二进制大小大概是C语言的三倍左右，主要增大在代码段和动态符号表上。但是Rust语言比C的表达能力更强，同样的功能下，可以使用更少于C的代码量来实现，所以其二进制的增大还是可以接受。