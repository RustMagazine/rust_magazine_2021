# 蚂蚁集团 | Trait Object 还是 Virtual Method Table


> Trait object 是 Rust 动态分发的实现方式。在 2021 年 4 月发刊的 Rust Magazine 中，Jiacai Liu 同学在《Trait 使用及实现分析》文章中介绍了 Rust 中 Ad-hoc 多态的使用方式，包括静态分发与动态分发，并且对 trait object 中的对象安全问题以及原因做出了详细解释。
> 那么，使用 trait object 就是 Rust 中动态分发的终点吗？事实上我们发现，在很多 Rust 代码中使用了原始的虚表而不是 trait object，这其中的原因又是什么呢？
> 在本文中，会先简单介绍一下 trait object 与虚表，然后结合笔者挑选出的几个具有代表性的代码片段，讨论手动构造虚表而不使用 trait object 的优缺点。

## 简介

在 Rust 中使用 trait 实现多态有两种方式，静态分发或者动态分发。静态分发使用 trait bound 或者 impl trait 方式实现编译期单态化，根据类型参数生成对应的结构或者函数。动态分发使用 trait object 的方式实现，而由于 trait object 是动态大小类型，无法在编译期确定类型大小，所以一般会使用指向 trait object 的引用或者指针来操作 trait object。而指向 trait object 的引用或者指针本质上是一个胖指针，其中包含了指向擦除了具体类型的对象指针与虚函数表。所以每次调用 trait object 的方法时，需要解引用该胖指针，所以部分观点认为动态分发比静态分发开销更大，而相反的观点认为使用静态分发会导致编译时间变长，编译后二进制文件膨胀以及增加缓存失效概率等问题，所以具体使用哪种方式就见仁见智了。

![trait object](./image/ant/1.jpg)

然而，标准的 `Trait` 结构也有着一些缺陷，比如由于对象安全的要求，一些 trait 无法通过 trait object 的方式使用。所以我们在使用或者阅读一些 Rust crate 的时候会发现，这些库实现了自己的 trait object 结构，比如标准库的 `RawWaker` 结构，`tokio` 的 `RawTask` 结构，`bytes` 的 `Bytes` 结构，`anyhow` 的 `ErrorImpl` 结构，以及关于类型擦除的内存分配器 [^1] 的讨论。在接下来的内容里，我对其中几个实现进行了一些粗浅的分析，并结合一些已有的讨论 [^2]，尝试总结它们的共同点。笔者水平有限，如有错漏，烦请指出。

## Examples

<!-- 在这里，我挑选了几个在 Rust 生态系统中广泛使用的 crate，甚至是 Rust 标准库的一些内容。它们的共同点在于，使用了手动实现的虚表来实现动态分发。 -->

### `std` 中的 `RawWaker` {#RawWaker}

Rust 异步编程的核心是 Executor 与 Reactor，其中 Reactor 部分主要是 `Waker` 结构。查看源代码发现 `Waker` 结构仅仅包装了 `RawWaker` 结构。而 `RawWaker` 的结构与指向 trait object 的胖指针十分相似，包含了一个指向任意类型的数据指针 `data` 与自定义此 `Waker` 行为的虚函数指针表 `vtable`。当调用 `Waker` 的相关方法时，实际上会调用虚表中对应的函数，并将 `data` 作为函数的第一个参数传入。

```Rust
/// A `RawWaker` allows the implementor of a task executor to create a [`Waker`]
/// which provides customized wakeup behavior.
///
/// [vtable]: https://en.wikipedia.org/wiki/Virtual_method_table
///
/// It consists of a data pointer and a [virtual function pointer table (vtable)][vtable]
/// that customizes the behavior of the `RawWaker`.
#[derive(PartialEq, Debug)]
#[stable(feature = "futures_api", since = "1.36.0")]
pub struct RawWaker {
    /// A data pointer, which can be used to store arbitrary data as required
    /// by the executor. This could be e.g. a type-erased pointer to an `Arc`
    /// that is associated with the task.
    /// The value of this field gets passed to all functions that are part of
    /// the vtable as the first parameter.
    data: *const (),
    /// Virtual function pointer table that customizes the behavior of this waker.
    vtable: &'static RawWakerVTable,
}

#[stable(feature = "futures_api", since = "1.36.0")]
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct RawWakerVTable {
    clone: unsafe fn(*const ()) -> RawWaker,
    wake: unsafe fn(*const ()),
    wake_by_ref: unsafe fn(*const ()),
    drop: unsafe fn(*const ()),
}
```

在学习这部分代码的时候，我产生了一个疑问，为什么不使用 Rust 提供的 `Trait` 作为 `Waker` 的抽象，而是要手动实现一个类似 trait object 胖指针的复杂，危险且容易出错的 `RawWaker`。为了解开这个疑问，我尝试使用 `Trait` 来模拟 `RawWaker` 的功能。

```Rust
pub trait RawWaker: Send + Sync {
    fn clone(&self) -> Box<dyn RawWaker>;

    fn wake(&self);
    fn wake_by_ref(&self);
}

impl Clone for Box<dyn RawWaker> {
    fn clone(&self) -> Self {
        RawWaker::clone(&**self)
    }
}

pub struct Waker {
    waker: Box<dyn RawWaker>,
}
```

根据虚表 `RawWakerVTable` 中要求的方法，我们可以写出一个简单的 `RawWaker` trait。这里遇到了几个问题，首先，`RawWaker` 要求实现 `Clone`，这样做的原因在 Saoirse Shipwreckt [^3] 的博客文章中有过简单的总结：

> 当事件源注册某个 `future` 将等待一个事件时，它必须存储 `Waker`，以便稍后调用 `wake` 方法。为了引入并发，能够同时等待多个事件是非常重要的，因此 `Waker` 不可能由单个事件源唯一拥有，所以 `Waker` 类型需要是可克隆的。

然而，`Clone` trait 本身不是对象安全的，因为它有着 `Sized` supertrait 限定。也就是说，如果我们使用 `pub trait RawWaker: Clone` 的写法，则该 `trait` 将无法作为 trait object 使用。所以在使用 `trait` 模拟的 `RawWaker` 中，我退而求其次的为 `Box<dyn RawWaker>` 实现了 `Clone`，并将具体的细节转移到了 `RawWaker::clone` 内，这样一来，每次调用 `clone` 方法都会构造一个新的 trait object，并且这些 trait object 会共享同一些数据。

其次，为了能够在多线程环境中使用，我要求 `RawWaker` 的 supertrait 为 `Send + Sync`，这样我们可以将其在多个线程间共享或者发送到某个线程中。

最后，为了通过指针使用 trait object，我们需要通过将该对象装箱在堆上。那么我们应该选用哪种智能指针呢？在上面的代码中，我使用了 `Box` 作为具体的指针类型，不使用 `Arc` 的原因是唤醒器中公共数据的共享方式应该由具体的实现决定。比如 `RawWaker` 的实现可以使用引用计数的方式跟踪另一个堆上的对象，或者全部指向静态全局的某个对象，比如：

```Rust
use std::sync::Arc;

#[derive(Debug, Default)]
struct RcWakerInner {}

#[derive(Debug, Default)]
pub struct RcWaker {
    inner: Arc<RcWakerInner>,
}

impl RawWaker for RcWaker {
    fn clone(&self) -> Box<dyn RawWaker> {
        Box::new(RcWaker {
            inner: self.inner.clone(),
        })
    }

    fn wake(&self) {
        todo!()
    }

    fn wake_by_ref(&self) {
        todo!()
    }
}

static GLOBAL_RAW_WAKER: StaticWakerInner = StaticWakerInner {};

#[derive(Debug, Default)]
struct StaticWakerInner {}

#[derive(Debug)]
pub struct StaticWaker {
    global_raw_waker: &'static StaticWakerInner,
}

impl Default for StaticWaker {
    fn default() -> Self {
        Self {
            global_raw_waker: &GLOBAL_RAW_WAKER,
        }
    }
}

impl RawWaker for StaticWaker {
    fn clone(&self) -> Box<dyn RawWaker> {
        Box::new(StaticWaker {
            global_raw_waker: self.global_raw_waker,
        })
    }

    fn wake(&self) {
        todo!()
    }

    fn wake_by_ref(&self) {
        todo!()
    }
}
```

接下来我们将标准库的 `RawWaker` 与上述实现方式进行一些对比可以发现：

- 由于我们发现 `Box<dyn RawWaker>` 首先经过了一层指针的包装，用来实现 trait object，而具体的 `RawWaker` 实现也很可能会使用指针来共享同一个对象。这样的不仅会在堆内存中占用额外的存储空间，产生许多小对象，也会由于解引用多级指针而带来额外的时间开销。
- 标准库的 `Waker` 位于 `core::task` 模块下，而 `Box` 与 `Arc` 等结构都位于 `alloc` 模块下，它们都是 `std` 的子集。在通常的 `std` 应用程序中，我们确实可以使用 `Arc` 等智能指针。但 Rust 不想在 `no_std` 的 futures 上妥协，所以我们必须使用特别的技巧来实现这种能力。

考虑到上面的这些原因，Rust 选择了使用数据指针与虚表的方式来实现高性能的动态分发。`std::task::RawWaker` 中的 `data` 指针既提供了类型擦除的能力，也实现了对象的共享，非常的灵活。

### `tokio` 与 `async-task` 中的 `RawTask` {#RawTask}

在 `tokio` 与 `async-task` 的代码中，都将 `RawTask` 作为 `Task` 结构的具体实现。与刚才提到的 `RawWaker` 相似，`RawTask` 也通过虚表提供了类似于 trait object 的功能，然而，它们在内存布局上却有着不同的选择。下面以 `tokio::runtime::raw::RawTask` 为例。

```Rust
#[repr(C)]
pub(crate) struct Header {
    pub(super) state: State,
    pub(super) owned: UnsafeCell<linked_list::Pointers<Header>>,
    pub(super) queue_next: UnsafeCell<Option<NonNull<Header>>>,

    /// Table of function pointers for executing actions on the task.
    pub(super) vtable: &'static Vtable,

    pub(super) owner_id: UnsafeCell<u64>,
    #[cfg(all(tokio_unstable, feature = "tracing"))]
    pub(super) id: Option<tracing::Id>,
}

/// Raw task handle
pub(super) struct RawTask {
    ptr: NonNull<Header>,
}
```

通过与 `RawWaker` 的结构相对比，我们发现 `RawTask` 将虚表部分移动到了数据指针 `ptr` 内。这么做的好处显而易见，`RawTask` 的内存结构更为紧凑，只需要占用一个指针的大小，缺点是多了一层解引用的开销。
所以，通过自定义类似 trait object 胖指针的结构，我们可以控制内存布局，使指针更瘦，或者更胖（比如 `async_task::raw::RawTask`）。

### `bytes` 中的 `Bytes` {#Bytes}

`bytes` crate 提供用于处理字节的抽象，它包含了一种高效的字节缓冲结构与相关的 traits。其中 `bytes::bytes::Bytes` 是用于存储和操作连续内存切片的高效容器，这是通过允许多个 `Bytes` 对象指向相同的底层内存来实现的。`Bytes` 结构充当了接口的功能，本身占用四个 `usize` 的大小，主要包含了内联的 trait object。

```Rust
pub struct Bytes {
    ptr: *const u8,
    len: usize,
    // inlined "trait object"
    data: AtomicPtr<()>,
    vtable: &'static Vtable,
}
```

它的虚表主要是 `clone` 方法，这允许 `Bytes` 的具体实现来定义具体的克隆或者共享策略。`Bytes` 的文档中举了两个例子

- 对于 `Bytes` 引用常量内存（例如通过 `Bytes::from_static()` 创建）的实现，`clone` 实现将是空操作。
- 对于 `Bytes` 指向引用计数共享存储（例如 `Arc<[u8]>`）的实现，将通过增加引用计数来实现共享。

可以看到，与 `std::task::RawWaker` 相似，`Bytes` 需要使用 `clone` 方法，并且具体的实现完全交给了实现方。如果选择 `Trait` 接口的方式，由于公共的数据部分已经是指针的形式，会引入额外的内存分配与解引用开销，感兴趣的同学可以尝试使用 `Trait` 的方式实现一下这两个例子，最终效果和上文中的 `RawWaker` 类似。而在内联了 trait object 之后，整个设计非常优雅，`data` 部分指向共享的内存，`vtable` 定义了如何进行 `clone`，其余字段作为独占的数据。

## 总结

Rust 提供了安全的抽象以避免产生安全问题或者错误。比如我们使用 `RC` 而不直接管理引用计数，使用 `Box` 而不是 `malloc/free` 直接管理内存分配。同样，`dyn Trait` 隐藏了复杂而又为危险的虚表实现，为我们提供了简单而又安全的动态分发。我们看到，上述手动实现虚表的代码中充斥着大量的 `unsafe`，稍有不慎，就会引入 bug。如果你的设计不能使用标准的 `dyn Trait` 结构来表达，那么你首先应该尝试重构你的程序，并参考以下理由来决定是否使用自定义的虚表。

- 你想要为一类指针对象实现多态，并且无法忍受多级指针解引用造成的性能开销，参考 [RawWaker](#RawWaker) 与 [Bytes](#Bytes)。
- 你想要自定义内存布局，比如像 C++ 中虚表一样紧凑的内存结构（虚表指针位于对象内），参考 [RawTask](#RawTask)。
- 你的 crate 需要在 `no_std` 环境中使用动态分发，参考 [RawWaker](#RawWaker)。
- 或者，标准的 trait object 确实无法实现你的需求。

## 相关链接

[^1]: https://github.com/rust-lang/wg-allocators/issues/33
[^2]: https://users.rust-lang.org/t/dyn-trait-vs-data-vtable/36127/3
[^3]: https://boats.gitlab.io/blog/post/wakers-i/