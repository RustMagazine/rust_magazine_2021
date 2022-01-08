# Rust for Linux 源码导读 | `Ref` 引用计数容器

作者：张汉东

--- 

## 引子

2022 年，我们很可能会看到 Linux 内核中的实验性 Rust 编程语言支持成为主流。2021.12.6 早上发出了更新的补丁，介绍了在内核中处理 Rust 的初始支持和基础设施。

这次更新的内容包括：

1. 升级到了最新 Stable 编译器和 Rust 2021 edition 。因此可以摆脱了 `const_fn_transmute`，`const_panic`、`const_unreachable_unchecked`、`core_panic` 和`try_reserve` 这几个之前未稳定的特性。[未稳定特性心愿单]( https://github.com/Rust-for-Linux/linux/issues/2)。
2. 自定义 `core` 和 `alloc`。为 `alloc` 添加了更加模块化的选项，以便禁用一些他们不需要的功能：`no_rc` 和 `no_sync`，主要是为上游 Rust 项目添加。
3.  更严格的代码、文档和新的  `lint`。
4. 抽象和驱动程序更新。添加了序列锁、电源管理回调的抽象，io 内存（`readX`/`writeX`）、irq 芯片和高级流处理程序，gpio 芯片（包括 irq 芯片）、设备、amba 设备和驱动程序以及证书。此外，也改进并简化了 `Ref`（`refcount_t` 支持）对象并用它替换了 Rust 的 `Arc` 的所有实例。完全地从 `alloc` crate 中删除了 `Arc` 和 `Rc`。

从现在开始，Rust for linux 团队将开始定期提交补丁，每两周左右。

除了来自 Arm、Google 和 Microsoft 的支持外，这次该团队又收到一封来自红帽的信：红帽对 Rust 用于内核的工作也非常感兴趣（There is interest in using Rust for kernel work that Red Hat  is considering）。

- [v2 补丁：https://lore.kernel.org/lkml/20211206140313.5653-1-ojeda@kernel.org/](https://lore.kernel.org/lkml/20211206140313.5653-1-ojeda@kernel.org/)
- [https://www.phoronix.com/scan.php?page=news_item&px=Rust-For-Linux-v2](https://www.phoronix.com/scan.php?page=news_item&px=Rust-For-Linux-v2)
- [kernel  crate 文档](https://rust-for-linux.github.io/docs/kernel/)

## 为什么需要引入 `Ref` 来代替 `Arc`

Rust for Linux 中这个 `kernel` crate 中之前使用的是 `Arc` ，但是现在换成了 `Ref`。 通过查看相关PR [rust: update Ref to use the kernel's refcount_t](https://github.com/Rust-for-Linux/linux/pull/377) ，可以了解其中主要有两点原因：

1. 最大化利用现有的 `C` 代码 和 消除恐慌（Panic)。内核中已经有了引用计数的实现 `refcount_t`，而且它超过引用计数的阈值时，不是 Panic（abort） 而是返回最大值（饱和加法）。因为这个原因，也使用 [ `RBTree`（红黑树）](https://github.com/Rust-for-Linux/linux/blob/rust/rust/kernel/rbtree.rs) [替代了 `BTreeMap`](https://github.com/Rust-for-Linux/linux/pull/403)。
2. 不需要弱引用。

> Arc 有一个 MAX_REFCOUNT 的限制，是 `isize::MAX as usize` 大小，引用计数加法超过该大小就会溢出然后发生Panic(abort)。

所以最终实现的 `Ref` 与`Arc`的区别在于：

1. `Ref` 是基于内核的 `refcount_t` 来支持的
2. 它不支持 弱引用，所以大小减少了一半
3. 当它超过阈值时，它使得引用计数饱和(saturating)而非中止(abort)
4. 它不提供 `get_mut` 方法，所以引用计数对象是 Pin 的。

## `Ref`源码分析

接下来分析一下[`Ref`](https://github.com/Rust-for-Linux/linux/blob/rust/rust/kernel/sync/arc.rs#L42)的实现。

### `Ref` 结构体

`Ref` 结构体定义如下：

```rust
/// A reference-counted pointer to an instance of `T`.
///
/// The reference count is incremented when new instances of [`Ref`] are created, and decremented
/// when they are dropped. When the count reaches zero, the underlying `T` is also dropped.
///
/// # Invariants
///
/// The reference count on an instance of [`Ref`] is always non-zero.
/// The object pointed to by [`Ref`] is always pinned.
pub struct Ref<T: ?Sized> {
    ptr: NonNull<RefInner<T>>,
    _p: PhantomData<RefInner<T>>,
}
```

它维护一个不变量（Invariants）：引用计数 `Ref` 总是一个非零的一个实例，并且被 `Ref`引用的对象总是 Pin 的（不可移动）。

该结构体中使用 `NonNull<T>`，而非 `*mut T`，这里需要协变（covariant），而非不变（invariant）。可以参考下面示例：

```rust
use std::ptr::NonNull;

struct Ref<T: ?Sized> {
    x: NonNull<T>,
    // x: *mut T, // 如果换成 *mut T，编译将不会通过
}

fn take<'a>(r: Ref<&'a u32>, y: &'a u32) {}

fn give() -> Ref<&'static u32> { todo!() }

fn test() {
    let y = 1;
    // 协变，能传入 Ref<&'a u32> 的函数take，也能接收 Ref<&'static u32> 类型的参数，因为 'static: 'a ，能接受子类型，也能接受父类型
    take(give(), &y); 
}
```

`NonNull` 是 `*mut T`的协变版本，并且也代表了非空指针，代表了引用计数对象总是非空的，因为当计数为零就会释放。

而这里使用 `PhatomData` 则是为了 Drop 检查，此处表示 Ref 类型拥有 `RefInner<T>`，当 `Ref` 被 Drop 的时候，`RefInner<T>`也能跟着被 Drop 。


### `RefInner` 结构体

再来看 `RefInner` 结构体：

```rust
#[repr(C)]
struct RefInner<T: ?Sized> {
    refcount: Opaque<bindings::refcount_t>,
    data: T,
}
```

`RefInner` 内部包含了内核中 C 语言实现的引用计数结构体 `refcount_t`，这里就是为了复用 C 代码。

其中 [`Opaque` 类型](https://github.com/Rust-for-Linux/linux/blob/rust/rust/kernel/types.rs#L277) 是 `kernel` crate 内置的专门为了和 C 打交道提供的一个包装类型，定义如下：

```rust
pub struct Opaque<T>(MaybeUninit<UnsafeCell<T>>);

impl<T> Opaque<T> {
    /// Creates a new opaque value.
    pub fn new(value: T) -> Self {
        Self(MaybeUninit::new(UnsafeCell::new(value)))
    }

    /// Creates an uninitialised value.
    pub fn uninit() -> Self {
        Self(MaybeUninit::uninit())
    }

    /// Returns a raw pointer to the opaque data.
    pub fn get(&self) -> *mut T {
        UnsafeCell::raw_get(self.0.as_ptr())
    }
}
```

Opaque 类型意味着永远都不需要 Rust 代码来解释的 FFi 对象。所以，为了使用内核中已经存在的引用计数结构体，这里用 `Opaque<bindings::refcount_t>`类型。

### 关于 `refcount_t`

Linux 内核中定义的 `refcount_t` 结构体定义如下：

```C

// from: https://github.com/torvalds/linux/blob/master/tools/include/linux/refcount.h
typedef struct refcount_struct {
	atomic_t refs;
} refcount_t;
```

`refcount_t` API的目标是为实现对象的引用计数器提供一个最小的API。虽然内部使用了原子操作，但一些 `refcount_*()` 和 `atomic_*()` 函数在内存顺序保证方面有很多不同。

`refcount_t` 在2018年曾经发生过 引用计数溢出的安全漏洞，即，当引用计数达到最大值时，如果再加一，则引用计数就会归零。所以，此时引用的对象就会被错误释放。这样就变成了一个 UAF(use-after-free) 漏洞，容易被人利用。

所以现在 `refcount_t` 被增加了引用计数检测：

```C
// from: https://github.com/torvalds/linux/blob/master/tools/include/linux/refcount.h#L69

static inline __refcount_check
bool refcount_inc_not_zero(refcount_t *r)
{
	unsigned int old, new, val = atomic_read(&r->refs);

	for (;;) {
		new = val + 1;

		if (!val)
			return false;

		if (unlikely(!new))
			return true;

		old = atomic_cmpxchg_relaxed(&r->refs, val, new);
		if (old == val)
			break;

		val = old;
	}

	REFCOUNT_WARN(new == UINT_MAX, "refcount_t: saturated; leaking memory.\n");

	return true;
}
```

在达到引用计数最大时采用饱和（saturated）加法，即，返回最大的值，而非零。注意，这里使用了`compare-and-swap`原子操作且并未提供内存顺序（使用relaxed）。


### 为 `Ref` 实现的一些 trait

为了让 `Ref<T>` 拥有一些类似于 `Arc<T>` 的行为，所以为其实现一些内置 trait。

```rust
// This is to allow [`Ref`] (and variants) to be used as the type of `self`.
impl<T: ?Sized> core::ops::Receiver for Ref<T> {}

// This is to allow [`RefBorrow`] (and variants) to be used as the type of `self`.
impl<T: ?Sized> core::ops::Receiver for RefBorrow<'_, T> {}

// This is to allow coercion from `Ref<T>` to `Ref<U>` if `T` can be converted to the
// dynamically-sized type (DST) `U`.
impl<T: ?Sized + Unsize<U>, U: ?Sized> core::ops::CoerceUnsized<Ref<U>> for Ref<T> {}

// This is to allow `Ref<U>` to be dispatched on when `Ref<T>` can be coerced into `Ref<U>`.
impl<T: ?Sized + Unsize<U>, U: ?Sized> core::ops::DispatchFromDyn<Ref<U>> for Ref<T> {}

// SAFETY: It is safe to send `Ref<T>` to another thread when the underlying `T` is `Sync` because
// it effectively means sharing `&T` (which is safe because `T` is `Sync`); additionally, it needs
// `T` to be `Send` because any thread that has a `Ref<T>` may ultimately access `T` directly, for
// example, when the reference count reaches zero and `T` is dropped.
unsafe impl<T: ?Sized + Sync + Send> Send for Ref<T> {}

// SAFETY: It is safe to send `&Ref<T>` to another thread when the underlying `T` is `Sync` for
// the same reason as above. `T` needs to be `Send` as well because a thread can clone a `&Ref<T>`
// into a `Ref<T>`, which may lead to `T` being accessed by the same reasoning as above.
unsafe impl<T: ?Sized + Sync + Send> Sync for Ref<T> {}
```

从上面代码里看得出来，用到的 trait 有：

-  [`core::ops::Receiver`](https://github.com/rust-lang/rust/blob/master/library/core/src/ops/deref.rs#L191) ： 是一个未稳定特性（`receiver_trait` features），它表示一个结构体可以作为方法接收者，不需要`arbitrary_self_types` 特性。标准库中一些智能指针实现了该trait，比如 `Box<T>`/ `Arc<T>` / `Rc<T>` / `&T` / `Pin<P>` 等。
-  [`core::ops::CoerceUnsized`](https://github.com/rust-lang/rust/blob/master/library/core/src/ops/unsize.rs#L36) ：也是一个未稳定特性（`coerce_unsized` features），它表示将 Size 类型转换为 DST 类型。
-  [`core::ops::DispatchFromDyn`](https://github.com/rust-lang/rust/blob/master/library/core/src/ops/unsize.rs#L117)： 同样是一个未稳定的特性（`dispatch_from_dyn` features），它用于对象安全（动态安全 dyn safe）的检查。实现 `DispatchFromDyn` 的类型可以安全地用作对象安全方法中的 self 类型。
- `Send/Sync`，是Rust 中稳定的特性，用于标记线程间可安全传递和共享的类型。

现在为 `Ref<T>` 实现了这些 trait，那么 `Ref<T>` 也就拥有了相应的行为。基本上 `Ref<T>` 的行为和 `Arc<T>` 类似了，除了上面所说的那些区别。


### 引用计数管理

因为 `Ref<T>` 是复用内核 C 代码，所以对于引用计数的管理，只需要实现相应的 trait 即可。

比如，`Clone` 时应该自增引用计数，而 `Drop` 时应该自减引用计数。所以，分别来看一下这两个实现。

```rust

// 实现 Clone trait
impl<T: ?Sized> Clone for Ref<T> {
    fn clone(&self) -> Self {
        // INVARIANT: C `refcount_inc` saturates the refcount, so it cannot overflow to zero.
        // SAFETY: By the type invariant, there is necessarily a reference to the object, so it is
        // safe to increment the refcount.
        unsafe { bindings::refcount_inc(self.ptr.as_ref().refcount.get()) };

        // SAFETY: We just incremented the refcount. This increment is now owned by the new `Ref`.
        unsafe { Self::from_inner(self.ptr) }
    }
}
```

实现 `Clone` trait 很简单，直接通过 `bindings::refcount_inc` 来调用内核中 `refcount_t` 的自增方法 `refcount_inc`即可。

因为 `refcount_inc` 已经是有了引用计数溢出检测，使用饱和加法，所以不用担心归零。

```rust
// 实现 Drop trait
impl<T: ?Sized> Drop for Ref<T> {
    fn drop(&mut self) {
        // SAFETY: By the type invariant, there is necessarily a reference to the object. We cannot
        // touch `refcount` after it's decremented to a non-zero value because another thread/CPU
        // may concurrently decrement it to zero and free it. It is ok to have a raw pointer to
        // freed/invalid memory as long as it is never dereferenced.
        let refcount = unsafe { self.ptr.as_ref() }.refcount.get();

        // INVARIANT: If the refcount reaches zero, there are no other instances of `Ref`, and
        // this instance is being dropped, so the broken invariant is not observable.
        // SAFETY: Also by the type invariant, we are allowed to decrement the refcount.
        let is_zero = unsafe { bindings::refcount_dec_and_test(refcount) };
        if is_zero {
            // The count reached zero, we must free the memory.

            // SAFETY: This thread holds the only remaining reference to `self`, so it is safe to
            // get a mutable reference to it.
            let inner = unsafe { self.ptr.as_mut() };
            let layout = Layout::for_value(inner);
            // SAFETY: The value stored in inner is valid.
            unsafe { core::ptr::drop_in_place(inner) };
            // SAFETY: The pointer was initialised from the result of a call to `alloc`.
            unsafe { dealloc(self.ptr.cast().as_ptr(), layout) };
        }
    }
}
```

实现 `Drop` trait，同样直接通过 `bindings::refcount_dec_and_test` 调用内核 `refcount_dec_and_test` 函数即可，该函数也包含了引用计数溢出检查。但是在引用计数归零的时候，需要释放内存。

注意上面 `Clone` 和 `Drop` 这两个 trait 的实现，是 Unsafe Rust 抽象为 Safe Rust 的一个经典范例，主要是其中的`Safety`注释，考虑了安全边界，并且加以说明。

### 创建新的引用计数对象

接下来需要关注 `Ref<T>` 如何创建新的引用计数对象。

```rust
impl<T> Ref<T> {
    /// Constructs a new reference counted instance of `T`.
    pub fn try_new(contents: T) -> Result<Self> {
        let layout = Layout::new::<RefInner<T>>();
        // SAFETY: The layout size is guaranteed to be non-zero because `RefInner` contains the
        // reference count.
        let inner = NonNull::new(unsafe { alloc(layout) })
            .ok_or(Error::ENOMEM)?
            .cast::<RefInner<T>>();

        // INVARIANT: The refcount is initialised to a non-zero value.
        let value = RefInner {
            // SAFETY: Just an FFI call that returns a `refcount_t` initialised to 1.
            refcount: Opaque::new(unsafe { bindings::REFCOUNT_INIT(1) }),
            data: contents,
        };
        // SAFETY: `inner` is writable and properly aligned.
        unsafe { inner.as_ptr().write(value) };

        // SAFETY: We just created `inner` with a reference count of 1, which is owned by the new
        // `Ref` object.
        Ok(unsafe { Self::from_inner(inner) })
    }
}

```

该 `try_new` 方法中使用 `core::alloc::Layout` 结构体来定义内存布局。

通过 `NonNull::new` 和 自定义的 `core::alloc::alloc` 函数 来分配新的内存，并转换为 `RefInner<T>>` 类型，并通过` bindings::REFCOUNT_INIT`调用内核 C 函数对其初始化为 `1` 。其中 自定义的 `core::alloc` 模块将来都会同步到 `rust` core 中。

其中 `Error::ENOMEM`代表 `OOM` 错误。在 [`kernel/error.rs`](https://github.com/Rust-for-Linux/linux/blob/rust/rust/kernel/error.rs#L64) 中定义了很多内核错误码对应的错误。

Linux 内核中使用整数定义了很多错误码，在 kernel crate 中，使用了 NewType 模式对其进行封装，而非直接使用整数错误码：

```rust
macro_rules! declare_err {
    ($err:tt) => {
        pub const $err: Self = Error(-(bindings::$err as i32));
    };
    ($err:tt, $($doc:expr),+) => {
        $(
        #[doc = $doc]
        )*
        pub const $err: Self = Error(-(bindings::$err as i32));
    };
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Error(c_types::c_int);

impl Error {
    declare_err!(EPERM, "Operation not permitted.");

    declare_err!(ENOENT, "No such file or directory.");

    declare_err!(ESRCH, "No such process.");

    declare_err!(ENOMEM, "Out of memory.");

    // ...

}
```

### 从已经存在的 `RefInner<T>` 构造 `Ref<T>`

在上面的 `try_new` 方法中看到，最后一步使用 `from_inner` 方法将一个裸指针构造为最终的 `Ref<T>`。并且它是一个内部方法，不是公开的 API。

注意，它是一个 unsafe 的方法，因为需要调用者来确保 inner 的指针是有效且非空的，对于这一点其文档注释也写的比较清楚。

```rust
impl<T: ?Sized> Ref<T> {
    /// Constructs a new [`Ref`] from an existing [`RefInner`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that `inner` points to a valid location and has a non-zero reference
    /// count, one of which will be owned by the new [`Ref`] instance.
    unsafe fn from_inner(inner: NonNull<RefInner<T>>) -> Self {
        // INVARIANT: By the safety requirements, the invariants hold.
        Ref {
            ptr: inner,
            _p: PhantomData,
        }
    }

}
```

### `RefBorrow<T>`

不存在对底层引用计数结构体的可变借用，但是存在一个不可变的借用，并且需要手动维护生命周期。

```rust
/// A borrowed [`Ref`] with manually-managed lifetime.
///
/// # Invariants
///
/// There are no mutable references to the underlying [`Ref`], and it remains valid for the lifetime
/// of the [`RefBorrow`] instance.
pub struct RefBorrow<'a, T: ?Sized + 'a> {
    inner: NonNull<RefInner<T>>,
    _p: PhantomData<&'a ()>,
}

impl<T: ?Sized> Clone for RefBorrow<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Copy for RefBorrow<'_, T> {}
```

`RefBorrow` 结构体使用 `PhantomData<&'a ()>` 来持有生命周期参数，并为其实现 Copy trait，其行为和普通的不可变引用类似。

然后为 `Ref<T>` 实现一个 `as_ref_borrow` 方法即可从 `Ref<T>` 得到 `RefBorrow<T>`。 

```rust
impl<T> Ref<T> {

    /// Returns a [`RefBorrow`] from the given [`Ref`].
    ///
    /// This is useful when the argument of a function call is a [`RefBorrow`] (e.g., in a method
    /// receiver), but we have a [`Ref`] instead. Getting a [`RefBorrow`] is free when optimised.
    #[inline]
    pub fn as_ref_borrow(&self) -> RefBorrow<'_, T> {
        // SAFETY: The constraint that lifetime of the shared reference must outlive that of
        // the returned `RefBorrow` ensures that the object remains alive.
        unsafe { RefBorrow::new(self.ptr) }
    }

}

```

其实按 Rust 命名规范，此处 `as_ref_borrow` 改为 `as_ref` 更好。但是这里其实 `as_ref` 另有用处：

```rust
impl<T: ?Sized> AsRef<T> for Ref<T> {
    fn as_ref(&self) -> &T {
        // SAFETY: By the type invariant, there is necessarily a reference to the object, so it is
        // safe to dereference it.
        unsafe { &self.ptr.as_ref().data }
    }
}
```

要通过 `as_ref` 方法从 `Ref<T>` 得到 `&T`。

然后为 `RefBorrow<T>` 实现 `Deref` trait，也可以从 `RefBorrow<T>` 拿到 `&T`。

```rust
impl<T: ?Sized> Deref for RefBorrow<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: By the type invariant, the underlying object is still alive with no mutable
        // references to it, so it is safe to create a shared reference.
        unsafe { &self.inner.as_ref().data }
    }
}
```


###  唯一引用类型 `UniqueRef<T>` 

除了 `Ref<T>` 之外，还实现了一个 `UniqueRef<T>` 类型。顾名思义，该类型表示只有唯一一个引用计数的情况。

```rust
pub struct UniqueRef<T: ?Sized> {
    inner: Ref<T>,
}

impl<T> UniqueRef<T> {
    /// Tries to allocate a new [`UniqueRef`] instance.
    pub fn try_new(value: T) -> Result<Self> {
        Ok(Self {
            // INVARIANT: The newly-created object has a ref-count of 1.
            inner: Ref::try_new(value)?,
        })
    }

    /// Tries to allocate a new [`UniqueRef`] instance whose contents are not initialised yet.
    pub fn try_new_uninit() -> Result<UniqueRef<MaybeUninit<T>>> {
        Ok(UniqueRef::<MaybeUninit<T>> {
            // INVARIANT: The newly-created object has a ref-count of 1.
            inner: Ref::try_new(MaybeUninit::uninit())?,
        })
    }
}
```

没有为其实现 `Clone` 和 `Drop` 这两个 trait，所以它只能持有唯一一个引用。引入该类型也许可以为内核开发提供更多便利。

### 其他

`Ref<T>` 还实现了其他 trait，比如 `From/TryFrom` ，可以从裸指针和 `Ref<T>`之间相互转换。

一个值得注意的地方是：

```rust
impl<T> Ref<T> {
    /// Deconstructs a [`Ref`] object into a raw pointer.
    ///
    /// It can be reconstructed once via [`Ref::from_raw`].
    pub fn into_raw(obj: Self) -> *const T {
        let ret = &*obj as *const T;
        core::mem::forget(obj);
        ret
    }
}
```

将 `Ref<T>` 转换为裸指针时，注意使用 `core::mem::forget(obj)` 避免调用 `obj` 的 Drop ，否则会让引用计数减少而引起问题。

### 小结

从 Rust for Linux 源码中可以学习很多 Unsafe Rust 的相关技巧，尤其是和 C 语言打交道的一些比较好的实践。如果你感兴趣，还能学习 Linux 内核相关的一些内容，为将来是要 Rust 编写 Linux 内核驱动做一些准备。

