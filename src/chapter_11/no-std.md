## 华为 | no std的可执行文件编写和运行

作者： 楼智豪

---

### 背景

众所周知，Rust支持no std的编译方式，可以支持在没有标准库的情况下运行程序。

对于一个Rust lib库而言，只要在库lib.rs开头加上一句`#![no_std]`就可以很方便的将其定义为no std环境下的crate。

但是，这并不意味着该crate可以在no std环境下运行。目前，no std的crate依然可以依赖于std的crate并且可以编译成功。no std仅仅检查该crate自身，而不检查依赖项是否是no std（[详见 issue #38509]( https://github.com/rust-lang/rust/issues/38509)）。

所以，想要验证一个库在no std环境下的功能是否正常，就需要编译一个no std的可执行文件来进行实际的运行验证，而不能仅仅依赖std环境下的测试。

### 实际操作

no std的程序和一般的程序差别比较大，除了std库里的println、String等无法使用以外，还需要用户自行配置很多内容，程序才可以正常运行。主要分为以下几点

1. 主要流程代码如下所示，其中，`#![no_std]`表示该crate为不依赖std，`#![no_main]`表示该crate没有main函数。

   一般情况下，程序从main函数开始，但实际上，程序执行的流程为`_start->libc_start_main->main->exit`，其中执行了一些环境初始化流程，在no std中则没有这些，所以我们需要自行实现start和exit函数，如下所示。`exit`退出函数也是必要的，否则也会导致段错误。

2. 注意在start函数的开头有一句汇编指令`push rbp`，这条语句的作用是将函数的栈帧按16字节对齐，在x86中某些汇编指令比如`movdqa`有栈帧16字节对齐的要求，如果没对齐则会导致段错误。一般如果程序比较简单的话不会生成`movdqa`指令，那么不用对齐也能正常运行。

```rust
// main.rs
#![feature(lang_items, asm, start)]
#![no_std]
#![no_main]

/// no std下无法自动运行main函数，所以需要命名为start
#[no_mangle]
pub extern "C" fn _start(_argc: i32, _argv: *const *const u8) -> i32 {
    // 用于在x86下进行函数栈帧16字节对齐
    #[cfg(target_arch = "x86_64")]
    unsafe {
        asm!("push rbp");
    }
    exit(0)
}

/// 使用系统调用进行进程退出。否则会导致coredump
pub fn exit(code: isize) -> ! {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        asm!(
        "syscall",
        in("rax") 60, // exit
        in("rdi") code,
        options(noreturn)
        );
    }
}
```

3. 如下所示，`eh_personality`和`panic_handler`用来处理Rust异常panic。在no std中，我们需要自行接管和处理panic，这两个语言项是必要的。程序发生panic之后就会跳转到panic handler。

   注意：此处调用了println，正常情况下no std中是不能使用println的，这样会导致错误信息无法打印，调试起来就会相当困难。所以我们可以使用libc的printf替代原本的println，使得在no std中也可以打印出异常信息，方便调试，当然前提是依赖于libc，如果环境中没有libc那也无法使用。

```
#[lang = "eh_personality"]
extern "C" fn eh_personality() {
    println!("eh_personality");
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    println!("{}", _info);
    exit(11)
}
```

* println的实现如下所示，使用write方法写入相关的信息，现在panic就可以打印出错误信息了。也可以使用println打印其他想要打印的信息。

```
panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `2`', src/main.rs:25:5
```

```
/// 由于在no std下无法使用println，所以使用libc::printf来接管标准库的println
#[macro_export]
macro_rules! println {
    () => ($crate::libc_printf(core::format_args!("")));
    ($($arg:tt)*) => ({
        $crate::libc_printf(core::format_args!($($arg)*));
    })
}

const BUFFER_LEN: usize = 512;

struct LibcWriter {
    buffer: [u8; BUFFER_LEN],
    used: usize,
}

impl LibcWriter {
    pub fn new() -> Self {
        Self {
            buffer: [0; BUFFER_LEN],
            used: 0,
        }
    }
}

impl Write for LibcWriter {
    fn write_str(&mut self, s: &str) -> Result {
        if self.used > self.buffer.len() {
            return Err(Error);
        }
        let remaining_buf = &mut self.buffer[self.used..];
        let raw_str = s.as_bytes();
        let len = min(raw_str.len(), remaining_buf.len());
        remaining_buf[..len].copy_from_slice(&raw_str[..len]);
        self.used += raw_str.len();

        Ok(())
    }
}

pub fn libc_printf(args: Arguments<'_>) {
    let mut writer = LibcWriter::new();
    let _res = write!(&mut writer, "{}", args);
    writer.buffer[BUFFER_LEN - 1] = 0;
    unsafe {
        libc::printf("%s\n\0".as_ptr() as *const _, writer.buffer.as_ptr());
    }
}
```

4. 该crate的编译指令为cargo +nightly rustc --release -- -C link-arg=-nostartfiles -lc。其中很多功能都只能在nightly才能用。`-C link-arg=-nostartfiles`和上文提到的`#![no_main]`是一个概念，没有main函数取而代之的是start函数。-lc则表示需要链接libc库。另外注意，`--release`现在是必须的，no std程序目前只能以release方式运行，debug版本编译会报错，详见[issue #47493](https://github.com/rust-lang/rust/issues/47493)

5. 上面编写了这么多，最终其实只是想为no std的库提供一个测试环境。因为原生的cargo test很难用于no std的测试。固然，我们也可以在测试文件的开头加上一句`#![no_std]`来表明该测试文件是no std的，但其实用处不大。

```
//test.rs
#![no_std]

#[test]
fn test() {
	//println!();
	test_func();
}
```

* 如上所示，这样的测试代码，如果我们在其中使用println，那么编译器会正常报错，告诉我们no std中没有println，这看起来很正常。但是如果在test_func中调用了println，那么编译不会报错且完全正常运行。所以实际上它依然依赖于std，这样的测试很难保证功能的正确性。为了这一点，所以我们单独编写了一个no std的测试环境，来完成这一切。