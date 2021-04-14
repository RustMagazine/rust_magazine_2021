---
pub_date: Sat, 27 Mar 2021 16:00:00 GMT
description: Executable file under `no_std` environment

---

# `no_std` 环境下的可执行文件

作者: 吴翱翔@pymongo / 后期编辑： 张汉东

> 原文: [no_std binary(static link)](https://pymongo.github.io/#/2021/03/statically_linked_executable.md)

由于作者身边只有 linux 操作系统的设备，所以本文内容仅探讨 Rust/C/C++ 在 linux 操作系统下 no_std 的可执行文件

本文更多探讨的是编译生成纯静态链接没有动态链接的 no_std 可执行文件，不仅连 Rust 的标准库也不用，连操作系统自带的 C 标准库也不用的环境

推荐这个 [Making our own executable packer](https://fasterthanli.me/series/making-our-own-executable-packer)(linux) 系列文章: 

在介绍Rust如何编译运行 no_std 的可执行文件之前，先看看汇编和 C/C++ 是如何编译 no_std 的可执行文件

## 汇编语言编译可执行文件

x86 汇编主要有两种语法，一是 Unix 的 AT&T syntax，另一个则是 windows 的 Intel syntax

由于 AT&T 有贝尔实验室，而 Unix 操作系统和 C 语言都是贝尔实验室发明的，所以 linux 的 gcc 和 as 都用 AT&T 汇编语法

如果想用 Intel 汇编语法可以用 llvm 或 nasm 工具

rustc 生成的汇编默认是 Intel 语法，可以传入 llvm 参数让 rustc 生成 AT&T 语法的汇编代码

> rustc --emit asm -C llvm-args=-x86-asm-syntax=att main.rs

以这个网站[GNU Assembler Examples](https://cs.lmu.edu/~ray/notes/gasexamples/)
介绍的第一段汇编代码为准

编译运行这段代码有两个方法:

> gcc -c s.s && ld s.o && ./a.out

或者用as工具(GNU assembler (GNU Binutils))

> as s.s && ld s.o && ./a.out

可以用 ldd 工具校验编译生成的可执行文件是不是 statically linked (没有引入任何动态链接库)

汇编的劣势在于代码跟硬件架构绑定，gcc 编译这段汇编代码时加上`-m32`参数指定生成32位的可执行文件时就会报错

## C 编译 no_std 可执行文件

用 gcc 或 clang 的 `-nostdlib`参数很容易生成无动态链接库的可执行文件

```
[w@w-manjaro temp]$ echo "int main(){return 0;}" > main.c && gcc -nostdlib main.c && ldd ./a.out
/usr/bin/ld: warning: cannot find entry symbol _start; defaulting to 0000000000001000
        statically linked
```

C 在 no_std 的环境下程序的入口函数名字不能是 main,要改成 _start

```
[w@w-manjaro temp]$ echo "int _start(){return 0;}" > main.c && gcc -nostdlib main.c && ldd ./a.out
        statically linked
```

当然也可以让 gcc 加上`-m32`参数生成32位的可执行文件

注意在 mac 或 windows 上用gcc 或 clang 的 `-nostdlib`参数可能会报错

```
$ clang -nostdlib c.c
ld: dynamic main executables must link with libSystem.dylib for architecture x86_64
clang: error: linker command failed with exit code 1 (use -v to see invocation)
```

根据苹果开发者文档，[Apple does not support statically linked binaries on Mac OS X](https://developer.apple.com/library/archive/qa/qa1118/_index.html)

可能 macOS 要用特殊的 ld 工具或稍微复杂点的方法才能编译纯静态链接的可执行文件，不过这不在本文的探讨范围内了

## Rust 编译 no_std 可执行文件

```rust
#![no_std]
#![no_main]
#![feature(lang_items,asm)]

/// entry_point/start_address of process, since the linker looks for a function named `_start` by default
#[no_mangle]
extern "C" fn _start() -> ! {
    exit(0); // macOS: illegal hardware instruction
}

fn exit(code: isize) -> ! {
    unsafe {
        asm!(
            "syscall",
            in("rax") 60, // exit
            in("rdi") code,
            options(noreturn)
        );
    }
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
```

源码在我[这个仓库](https://github.com/pymongo/no_std_binary/blob/main/main.rs)，linux 下的编译方法:

> rustc -C link-arg=-nostartfiles main.rs

或者将以下两行写到`.cargo/config.toml`中

```
[target.'cfg(target_os = "linux")']
rustflags = ["-C", "link-arg=-nostartfiles"]
```

如果只是编译 no_std 环境下的 动态链接库(cdylib)，则不需要加上述 rustc 参数
