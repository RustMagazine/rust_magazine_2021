# RFC 导读 | 构建安全的 I/O

## 动机

最近Rust官方合并了一个[ RFC ](https://github.com/rust-lang/rfcs/blob/master/text/3128-io-safety.md)，通过引入I/O安全的概念和一套新的类型和特征，为`AsRawFd`和相关特质的用户提供关于其原始资源句柄的保证，从而弥补Rust中封装边界的漏洞。

Rust 标准库提供了 I/O 安全性，保证程序持有私有的原始句柄（raw handle），其他部分无法访问它。但是 `FromRawFd::from_raw_fd` 是 Unsafe 的，所以在 Safe Rust中无法做到 `File::from_raw(7)` 这种事。 在这个文件描述符上面进行` I/O` 操作，而这个文件描述符可能被程序的其他部分私自持有。

但是，很多 API 通过接受 原始句柄 来进行 I/O 操作：

```rust
pub fn do_some_io<FD: AsRawFd>(input: &FD) -> io::Result<()> {
    some_syscall(input.as_raw_fd())
}
```

`AsRawFd`并没有限制`as_raw_fd`的返回值，所以`do_some_io`最终可以在任意的`RawFd`值上进行 `I/O `操作。甚至可以写`do_some_io(&7)`，因为`RawFd`本身实现了`AsRawFd`。这可能会导致程序访问错误的资源。甚至通过创建在其他部分私有的句柄别名来打破封装边界，导致一些诡异的 远隔作用（Action at a distance）。

> **远隔作用**（**Action at a distance**）是一种程式设计中的[反模式](https://zh.wikipedia.org/wiki/反模式)，是指程式某一部分的行为会广泛的受到程式其他部分[指令](https://zh.wikipedia.org/wiki/指令)的影响，而且要找到影响其他程式的指令很困难，甚至根本无法进行。

在一些特殊的情况下，违反 I/O 安全甚至会导致内存安全。

## I/O   安全概念引入

标准库中有一些类型和特质： `RawFd(Unix) / RawHandle/RawSocket(Windows)` ，它们代表原始的操作系统资源句柄。这些类型本身并不提供任何行为，而只是代表可以传递给底层操作系统API的标识符。

这些原始句柄可以被认为是原始指针，具有类似的危险性。虽然获得一个原始指针是安全的，但是如果一个原始指针不是一个有效的指针，或者如果它超过了它所指向的内存的生命周期，那么解引用原始指针可能会调用未定义的行为。

同样，通过`AsRawFd::as_raw_fd`和类似的方式获得一个原始句柄是安全的，但是如果它不是一个有效的句柄或者在其资源关闭后使用，使用它来做`I/O`可能会导致输出损坏、输入数据丢失或泄漏，或者违反封装边界。而在这两种情况下，影响可能是非局部的且影响到程序中其他不相关的部分。对原始指针危险的保护被称为内存安全，所以**对原始句柄危险的保护被称为`I/O`安全**。

Rust的标准库也有一些高级类型，如`File`和`TcpStream`，它们是这些原始句柄的封装器，提供了操作系统API的高级接口。

这些高级类型也实现了`Unix-like`平台上的`FromRawFd`和`Windows`上的`FromRawHandle/FromRawSocket`的特性，这些特性提供了包裹底层(low-level )值以产生上层（high-level）值的函数。这些函数是不安全的，因为它们无法保证`I/O`安全，类型系统并不限制传入的句柄。

```rust
use std::fs::File;
use std::os::unix::io::FromRawFd;

// Create a file.
let file = File::open("data.txt")?;

// 从任意的整数值构造 file
// 然而这种类型的检查在运行时可能无法识别一个合法存活的资源
// 或者它可能意外地在程序的其他地方被以别名方式封装处理（此处无法判断）
// 这里添加  unsafe 块 是让调用者来避免上述危险
let forged = unsafe { File::from_raw_fd(7) };

// Obtain a copy of `file`'s inner raw handle.
let raw_fd = file.as_raw_fd();

// Close `file`.
drop(file);

// Open some unrelated file.
let another = File::open("another.txt")?;

// 进一步使用 raw_fd ，也就是 file 的内部原始句柄，将超出操作系统与之相关的生命周期
// 这可能会导致它意外地与其他封装好的 file 实例发生别名，比如 another  
// 因此，这里 unsafe 块是让调用者避免上述危险
let dangling = unsafe { File::from_raw_fd(raw_fd) };
```

调用者必须确保传入`from_raw_fd`的值是明确地从操作系统返回的，而且`from_raw_fd`的返回值不会超过操作系统与句柄相关的生命周期。

`I/O` 安全的概念虽然是新的，但它反映出了一个普遍的做法。 Rust 生态系统将会逐步支持 `I/O` 安全。

##  I/O 安全 Rust 解决方案

### `OwnedFd` 和 `BorrowedFd<'fd>`

这两种类型用于替代 `RawFd` ，对句柄值赋予所有权语义，代表句柄值的 拥有和借用。

`OwnedFd` 拥有一个 `fd` ，会在析构的时候关闭它。`BorrowedFd<'fd>` 中的生命周期参数表示对这个 `fd` 的访问被借用多长时间。

对于Windows来说，也有类似的类型，但都是`Handle`和`Socket`形式。

| 类型             | 类似于     |
| ---------------- | ---------- |
| `OwnedFd`        | `Box<_>`   |
| `BorrowedFd<'a>` | `&'a _`    |
| `RawFd`          | `*const _` |

和其他类型相比，`I/O`  类型并不区分可变和不可变。操作系统资源可以在`Rust`的控制之外以各种方式共享，所以`I/O`可以被认为是使用内部可变性。

### `AsFd`、`Into<OwnedFd>`和`From<OwnedFd>`

这三个概念是`AsRawFd::as_raw_fd`、`IntoRawFd::into_raw_fd`和`FromRawFd::from_raw_fd`的概念性替代，分别适用于大多数使用情况。它们以`OwnedFd`和`BorrowedFd`的方式工作，所以它们自动执行其`I/O`安全不变性。

```rust
pub fn do_some_io<FD: AsFd>(input: &FD) -> io::Result<()> {
    some_syscall(input.as_fd())
}
```

使用这个类型，就会避免之前那个问题。由于`AsFd`只针对那些适当拥有或借用其文件描述符的类型实现，这个版本的`do_some_io`不必担心被传递假的或悬空的文件描述符。

### 逐步采用

`I/O`安全和新的类型和特性不需要立即被采用，可以逐步采用。

- 首先，`std`为所有相关的`std`类型添加新的类型和特质，并提供`impls`。这是一个向后兼容的变化。
- 之后，`crate`可以开始使用新的类型，并为它们自己的类型实现新的特质。这些变化将是很小的，而且是半兼容的，不需要特别的协调。
- 一旦标准库和足够多的流行`crate`实现了新的特质，`crate`就可以按照自己的节奏开始使用新的特质作为接受通用参数时的边界。这些将是与`semver`不兼容的变化，尽管大多数切换到这些新特质的`API`的用户不需要任何改变。



## 原型实现

该 RFC 内容原型已经实现，参见 [io-lifetimes](https://github.com/sunfishcode/io-lifetimes) 。

| `Raw` API  | This experimental API    |
| ---------- | ------------------------ |
| `Raw*`     | `Borrowed*` and `Owned*` |
| `AsRaw*`   | `As*`                    |
| `IntoRaw*` | `Into*`                  |
| `FromRaw*` | `From*`                  |



###  trait 实现

`AsFd` 转换为 原生 `fd` ，是带有生命周期参数的 `BorrowedFd<'_>`

```rust
#[cfg(any(unix, target_os = "wasi"))]
pub trait AsFd {
    /// Borrows the file descriptor.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # #![cfg_attr(io_lifetimes_use_std, feature(io_safety))]
    /// use std::fs::File;
    /// # use std::io;
    /// use io_lifetimes::{AsFd, BorrowedFd};
    ///
    /// let mut f = File::open("foo.txt")?;
    /// let borrowed_fd: BorrowedFd<'_> = f.as_fd();
    /// # Ok::<(), io::Error>(())
    /// ```
    fn as_fd(&self) -> BorrowedFd<'_>;
}

```

`IntoFd`从 原生 `fd` 转为 安全的 `fd`，是 `OwnedFd`

```rust
#[cfg(any(unix, target_os = "wasi"))]
pub trait IntoFd {
    /// Consumes this object, returning the underlying file descriptor.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # #![cfg_attr(io_lifetimes_use_std, feature(io_safety))]
    /// use std::fs::File;
    /// # use std::io;
    /// use io_lifetimes::{IntoFd, OwnedFd};
    ///
    /// let f = File::open("foo.txt")?;
    /// let owned_fd: OwnedFd = f.into_fd();
    /// # Ok::<(), io::Error>(())
    /// ```
    fn into_fd(self) -> OwnedFd;
}
```

`FromFd` 从原生 `fd` 构造 `OwnedFd`

```rust
#[cfg(any(unix, target_os = "wasi"))]
pub trait FromFd {
    /// Constructs a new instance of `Self` from the given file descriptor.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # #![cfg_attr(io_lifetimes_use_std, feature(io_safety))]
    /// use std::fs::File;
    /// # use std::io;
    /// use io_lifetimes::{FromFd, IntoFd, OwnedFd};
    ///
    /// let f = File::open("foo.txt")?;
    /// let owned_fd: OwnedFd = f.into_fd();
    /// let f = File::from_fd(owned_fd);
    /// # Ok::<(), io::Error>(())
    /// ```
    fn from_fd(owned: OwnedFd) -> Self;

    /// Constructs a new instance of `Self` from the given file descriptor
    /// converted from `into_owned`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # #![cfg_attr(io_lifetimes_use_std, feature(io_safety))]
    /// use std::fs::File;
    /// # use std::io;
    /// use io_lifetimes::{FromFd, IntoFd};
    ///
    /// let f = File::open("foo.txt")?;
    /// let f = File::from_into_fd(f);
    /// # Ok::<(), io::Error>(())
    /// ```
    #[inline]
    fn from_into_fd<Owned: IntoFd>(into_owned: Owned) -> Self
    where
        Self: Sized,
    {
        Self::from_fd(into_owned.into_fd())
    }
}
```

上述为针对 Unix 平台的 trait，该库也包含 Windows 平台的相关 trait ： `AsHandle / AsSocket `、`IntoHandle /IntoSocket`  、`FromHandle /FromSocket  ` 。

### 相关类型

`BorrowedFd<'fd>` 

```rust
#[cfg(any(unix, target_os = "wasi"))]
#[derive(Copy, Clone)]
#[repr(transparent)]
#[cfg_attr(rustc_attrs, rustc_layout_scalar_valid_range_start(0))]
// libstd/os/raw/mod.rs assures me that every libstd-supported platform has a
// 32-bit c_int. Below is -2, in two's complement, but that only works out
// because c_int is 32 bits.
#[cfg_attr(rustc_attrs, rustc_layout_scalar_valid_range_end(0xFF_FF_FF_FE))]
pub struct BorrowedFd<'fd> {
    fd: RawFd,
    _phantom: PhantomData<&'fd OwnedFd>,
}

#[cfg(any(unix, target_os = "wasi"))]
#[repr(transparent)]
#[cfg_attr(rustc_attrs, rustc_layout_scalar_valid_range_start(0))]
// libstd/os/raw/mod.rs assures me that every libstd-supported platform has a
// 32-bit c_int. Below is -2, in two's complement, but that only works out
// because c_int is 32 bits.
#[cfg_attr(rustc_attrs, rustc_layout_scalar_valid_range_end(0xFF_FF_FF_FE))]
pub struct OwnedFd {
    fd: RawFd,
}

#[cfg(any(unix, target_os = "wasi"))]
impl BorrowedFd<'_> {
    /// Return a `BorrowedFd` holding the given raw file descriptor.
    ///
    /// # Safety
    ///
    /// The resource pointed to by `raw` must remain open for the duration of
    /// the returned `BorrowedFd`, and it must not have the value `-1`.
    #[inline]
    pub unsafe fn borrow_raw_fd(fd: RawFd) -> Self {
        debug_assert_ne!(fd, -1_i32 as RawFd);
        Self {
            fd,
            _phantom: PhantomData,
        }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl AsRawFd for BorrowedFd<'_> {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl AsRawFd for OwnedFd {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl IntoRawFd for OwnedFd {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        let fd = self.fd;
        forget(self);
        fd
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl Drop for OwnedFd {
    #[inline]
    fn drop(&mut self) {
        #[cfg(feature = "close")]
        unsafe {
            let _ = libc::close(self.fd as std::os::raw::c_int);
        }

        // If the `close` feature is disabled, we expect users to avoid letting
        // `OwnedFd` instances drop, so that we don't have to call `close`.
        #[cfg(not(feature = "close"))]
        {
            unreachable!("drop called without the \"close\" feature in io-lifetimes");
        }
    }
}


```

### 为 std 和其他生态库 支持安全 I/O

再构建一些跨平台抽象类型之后，为  `ffi / async_std/ fs_err/ mio/ os_pipe/ socket2/ tokio / std ` 来支持 安全I/O 抽象。

### 使用案例

```rust
// From: https://github.com/sunfishcode/io-lifetimes/blob/main/examples/hello.rs

#[cfg(all(rustc_attrs, unix, feature = "close"))]
fn main() -> io::Result<()> {
    // write 是 c api，所以用 unsafe
    let fd = unsafe {
        // Open a file, which returns an `Option<OwnedFd>`, which we can
        // maybe convert into an `OwnedFile`.
        // 拥有一个 fd
        let fd: OwnedFd = open("/dev/stdout\0".as_ptr() as *const _, O_WRONLY | O_CLOEXEC)
            .ok_or_else(io::Error::last_os_error)?;

        // Borrow the fd to write to it.
        // 借用这个 fd 
        let result = write(fd.as_fd(), "hello, world\n".as_ptr() as *const _, 13);
        match result {
            -1 => return Err(io::Error::last_os_error()),
            13 => (),
            _ => return Err(io::Error::new(io::ErrorKind::Other, "short write")),
        }

        fd
    };

    // Convert into a `File`. No `unsafe` here!
    // 这里不再需要 Unsafe 了
    let mut file = File::from_fd(fd);
    writeln!(&mut file, "greetings, y'all")?;

    // We can borrow a `BorrowedFd` from a `File`.
    unsafe {
        // 借用 fd
        let result = write(file.as_fd(), "sup?\n".as_ptr() as *const _, 5);
        match result {
            -1 => return Err(io::Error::last_os_error()),
            5 => (),
            _ => return Err(io::Error::new(io::ErrorKind::Other, "short write")),
        }
    }

    // Now back to `OwnedFd`.
    let fd = file.into_fd();

    // 不是必须的，会自动析构 fd 
    unsafe {
        // This isn't needed, since `fd` is owned and would close itself on
        // drop automatically, but it makes a nice demo of passing an `OwnedFd`
        // into an FFI call.
        close(fd);
    }

    Ok(())
}

```



## 理由与替代方案

### 关于  “unsafe 是为了内存安全” 的说法

Rust 在历史上划定了一条界线，指出 unsafe 仅仅是用于 内存安全相关。 比较知名的例子是 `std::mem::forget`， 它增加是 unsafe 的，后来改为了 safe。

声明 unsafe 只用于内存安全的结论表明，unsafe 不应该用于 其他非内存安全类的 API ，比如 标示某个 API 是应该避免使用的之类。

内存安全优先级高于其他缺陷，因为它不仅仅是为了避免非预期行为，而是为了避免无法约束一段代码可能做的事情的情况。

`I/O` 安全也是属于这类情况，理由有二：

1. `I/O`安全错误会导致内存安全错误，在`mmap`周围的安全包装器存在的情况下（在具有操作系统特定API的平台上，允许它们是安全的）。
2. `I/O安全`错误也意味着一段代码可以读取、写入或删除程序中其他部分使用的数据，而不需要命名它们或给它们一个引用。如果不知道链接到程序中的所有其他`crate`的实现细节，就不可能约束一个`crate`可以做的事情的集合。

原始句柄很像进入独立地址空间的原始指针；它们可以悬空或以虚假的方式进行计算。`I/O`安全与内存安全类似；两者都是为了防止诡异的远隔作用，而且在两者中，所有权是健壮抽象的主要基础，所以使用类似的安全概念是很自然的。



## 相关

- [https://github.com/smiller123/bento](https://github.com/smiller123/bento)

- [https://github.com/bytecodealliance/rsix](https://github.com/bytecodealliance/rsix)

- [RFC #3128 IO Safety](https://github.com/rust-lang/rfcs/blob/master/text/3128-io-safety.md)

- [nrc 的 RFC 索引列表](https://www.ncameron.org/rfcs/)