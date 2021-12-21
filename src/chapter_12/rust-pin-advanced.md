# Rust Pin 进阶

> 原博客地址：https://folyd.com/blog/rust-pin-advanced/

去年我写了一篇关于 Pin 的文章，算是由浅入深介绍了 Pin 到底是怎么一回事，为什么需要 Pin。但只掌握那部分知识依然不够，所以这篇文章希望能系统性的梳理跟 Pin 有关的知识点，所以标题我命名为《Rust Pin 进阶》。

## Pin API 剖析

要想深入理解 Pin，熟悉 Pin 所有的方法是必不可少的。除去 nightly 的 API 后，Pin 总共有 13 个方法：

```rust
// Pin<P> where P: Deref
impl Pin<P> where P: Deref {
  unsafe fn new_unchecked(pointer: P) -> Pin<P>
  
  fn as_ref(&self) -> Pin<&P::Target>
  
  unsafe fn into_inner_unchecked(pin: Pin<P>) -> P
}

impl<P: Deref<Target: Unpin>> Pin<P> {
  fn new(pointer: P) -> Pin<P>
  
  fn into_inner(pin: Pin<P>) -> P
}

impl<'a, T: ?Sized> Pin<&'a T> {
  unsafe fn map_unchecked<U, F>(self, func: F) -> Pin<&'a U>
    where
        U: ?Sized,
        F: FnOnce(&T) -> &U
  
  fn get_ref(self) -> &'a T
}


// Pin<P> where P: DerefMut
impl<P: DerefMut> Pin<P> {
  fn as_mut(&mut self) -> Pin<&mut P::Target>
  
  fn set(&mut self, value: P::Target) where P::Target: Sized
}

impl<'a, T: ?Sized> Pin<&'a mut T> {
  fn into_ref(self) -> Pin<&'a T>
  
  fn get_mut(self) -> &'a mut T where T: Unpin
  
  unsafe fn get_unchecked_mut(self) -> &'a mut T
  
  unsafe fn map_unchecked_mut<U, F>(self, func: F) -> Pin<&'a mut U>
    where
        U: ?Sized,
        F: FnOnce(&mut T) -> &mut U
}
```

这些方法可以分为两个大类：

- `Pin<P> where P: Deref`
- `Pin<P> where P: DerefMut`

之前的文章有说过，Pin 一般以 `Pin<P<T>>` 的形式来表示（P 是 Pointer 的缩写，T 是 Type 的缩写） ，所以 Pin 包裹的内容只能是智能指针（实现了 `Deref` trait 的类型都可以称为智能指针），对其他普通类型是没有意义的。因为 `&T` 和 `&mut T`分别实现了 `Deref` 和 `DerefMut`，所以 `Pin<&'a T>` 和 `Pin<&'a mut T>` 分别算这两大类下的特化实现。

初看起来这 13 个方法有点杂乱，但其实他们的设计非常讲究，甚至可以说存在对称性。按功能来划分的话，这些方法可以分为 5 大类，各个类别按可变性或是符合 `T: Unpin` 限定来细分为 2~3 种。其中可变的版本都以 `mut` 结尾，因为不符合`T: Unpin` 限定的 `unsafe` 版本都包含 `unchecked`。

| 功能                                  | 方法                                              | 备注                                                         |
| ------------------------------------- | :------------------------------------------------ | ------------------------------------------------------------ |
| **构造 `Pin`**                        | `new()` / `new_unchecked()`                       | 按是否满足 `T: Unpin` 限定来区分 safe 和 unsafe 两个版本。   |
| **转换 Pin 类型**                     | `as_ref()` / `as_mut()`                           | 将 `&/&mut Pin<P<T>>` 转换成 `Pin<&/&mut T>`。               |
| **获取 `Pin<P<T>>` 里面 `T` 的借用**  | `get_ref()` / `get_mut()` / `get_unchecked_mut()` | 消耗所有权，拿到里面的 `T` 的借用。按可变性分为两个版本。因为 `&mut T` 是"万恶之源"，所以 `get_mut` 又按是否满足 `T: Unpin` 限定来区分 safe 和 unsafe 两个版本。 |
| **消耗 Pin 所有权，拿到里面的指针 P** | `into_inner()` / `into_inner_unchecked()`         | 按是否满足 `T: Unpin` 限定来区分 safe 和 unsafe 两个版本。另外，为了避免和 `P` 自己的 into 类方法冲突，这几个 API 都设计成静态方法，必须通过 `Pin::into_inner()` 这种形式调用，不能用 `pin.into_inner()`。 |
| **Pin projection**                    | `map_unchecked()` / `map_unchecked_mut()`         | 通常用来做 Pin projection。                                  |

> 只剩下两个方法没有归类到上面的表格中，它们也比较简单，分别是：
>
> - `Pin::set()` - 设置 `Pin<P<T>>` 中新的 `T` 的值。
> - `Pin<&mut Self>::into_ref()` - 将 `Pin<&mut T>` 转换成 `Pin<&T>`。

值得注意的是其实 `new()` 和 `new_unchecked()`， `get_mut()`与 `get_unchecked_mut()` ， `into_inner()` 和 `into_inner_unchecked()` 的实现是完全一样的，唯一的区别是 safe 版有 `Unpin` 限定。

```rust
pub const fn new(pointer: P) -> Pin<P> 
  where P: Deref,
	     <P as Deref>::Target: Unpin {
    unsafe { Pin::new_unchecked(pointer) }
}

pub const unsafe fn new_unchecked(pointer: P) -> Pin<P> {
		Pin { pointer }
}

pub const fn get_mut(self) -> &'a mut T where T: Unpin {
		self.pointer
}

pub const unsafe fn get_unchecked_mut(self) -> &'a mut T {
		self.pointer
}

pub const fn into_inner(pin: Pin<P>) -> P
	where P: Deref,
		    <P as Deref>::Target: Unpin {
    pin.pointer
}

pub const unsafe fn into_inner_unchecked(pin: Pin<P>) -> P {
		pin.pointer
}
```

为什么相同的代码要区分 safe 和 unsafe 两个版本呢？要解答这个问题，还是要回到 `Pin` 的本质。**`Pin` 本质上解决的问题是在 safe Rust 下保证 `Pin<P<T>` 中的 `T` 的内存地址不会被改变（也就是不被 move），除非 `T` 满足 `T: Unpin`。**保证 `T` 的内存地址不会被改变的本质是避免暴露`T` 或 `&mut T`（"万恶之源"）。暴露 `T` 的话，随随便便就能 move 掉它；暴露 `&mut T` 的话，开发者可以调用 `std::mem::swap()` 或 `std::mem::replace()` 这类方法来 move 掉 `T`。还有一条，Rust 里面 safe 和 unsafe 的边界一定要区分非常明确，不能有任何含糊。所以只要你不满足 `T: Unpin` ，然后需要构造 `Pin<P<T>>`、暴露 `T` 或者 `&mut T` 的方法都应该是 unsafe 的。

|           | 满足 `T: Unpin` | 不满足 `T: Unpin` |
| --------- | --------------- | ----------------- |
| 构造 `Pin` | safe            | **unsafe**        |
| 暴露 `T`   | safe            | **unsafe**        |
| 暴露 `&T`  | safe            | safe              |
| 暴露 `&mut T`| safe          | **unsafe**        |

比如，`into_inner_unchecked()` 虽然返回的是 `P`，但是它是间接暴露了 `T` 和 `&mut T`。因为你可以通过`*P` 或 `&mut *P`  轻而易举的拿到 `T` 或 `&mut T`。而你构造 `Pin<P<T>>` 的时候相当于是承若要遵守 **Pin 的契约** 的，但这步操作明显就违约了。

为什么 `Pin::get_ref()` 是 safe 的呢？因为它返回的是 `&T`，你没有办法 move 掉它：`std::mem::swap()` 类方法只支持 `&mut T`，解引用 `&T` 的话编译器会报错阻止你。（再一次感谢 rustc）另外需要强调的是内部可变性的类型。比如 `RefCell<T>`， `Pin<&mut RefCell<T>>.into_ref().get_ref()` 返回的是 `&RefCell<T>`，而 `RefCell<T>::into_inner()` 之类的方法可以拿到 `T` 然后 move 掉它。但是这个没关系，因为 `Pin<P<T>>` 的契约是保证 `P` 里面的 `T` 不被 move，而这里的 `P` 是 `&`, `T` 是 `RefCell`，并不是 `RefCell<T>` 里面的 `T`。只要没有额外 `Pin<&T>` 指向 `RefCell<T>` 里面的 `T` 就行，但是你构造 `RefCell<T>` 的时候其实已经自动杜绝这种可能了。因为 `RefCell::new()`  的参数是 `value: T`，早就把 `T` move 进来了。

> 类似的，`Pin<&mut Box<T>>` 保证的是 `Box<T>` 本身不被 move，而不是 `Box` 里面的 `T`。如果要保证 `Box<T>` 里面的 `T` 不被 move，直接使用 `Pin<Box<T>>` 即可。

## Pin 额外的属性

### #[fundamental]

使用`#[fundamental]`属性标记的 trait 是不受孤儿规则约束。所以你可以给 `Pin<P<T>>` impl 你本地的 trait。

```rust
use std::pin::Pin;

trait LocalTrait {}

impl<P> LocalTrait for Pin<P> {
}
```

### #[repr(transparent)]

`#[repr(transparent)]` 这个属性可以让 `Pin` 拥有和里面 `pointer` 字段同样的 ABI 布局，在 FFI 的场景下会很有用。

> `#[repr(transparent)]` 是 1.28 稳定的，详细可以看 [release note](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1280-2018-08-02)：
>
> [The `#[repr(transparent)]` attribute is now stable.](https://github.com/rust-lang/rust/pull/51562/) This attribute allows a Rust newtype wrapper (`struct NewType<T>(T);`) to be represented as the inner type across Foreign Function Interface (FFI) boundaries.

## Pin 实现的 trait

再来看看 `Pin` 实现了哪些值得关注的 trait。

### Unpin

```rust
impl<P> Unpin for Pin<P> where P: Unpin {}
```

因为 `Unpin` 是 auto trait，所以 `Pin<P<T>` 如果满足 `P: Unpin` ，则 `Pin<P<T>>` 也会实现 `Unpin`。而几乎所有 `P` 都会是 `Unpin` ，所以 `Pin<P<T>>` 几乎总会是 `Unpin` 的。这个实现很重要，特别是如果这里的 `T` 是 `Future` 的时候。不管你的 `Future` 是不是满足 `Unpin`，把你用 `Pin<&mut ...>` 包裹之后，它就是一个满足 `Unpin` 的 `Future` 了（因为 `Pin<P>` 实现了 `Future`，后面会讲到）。很多异步的方法可能需要你的 `Future` 满足 `Unpin` 才能调用，而 `async fn` 方法返回的 `Future` 显然不满足 `Unpin`，这个时候往往需要你把这个 `Future` pin 住才行。比如使用 [tokio::pin!()](https://docs.rs/tokio/latest/tokio/macro.pin.html) 这个宏。

```rust
use tokio::pin;

async fn my_async_fn() {
    // async logic here
}

#[tokio::main]
async fn main() {
    let future = my_async_fn();
    pin!(future);

    (&mut future).await;
}
```

另外，需要再次强调的是：

- **`Pin` 本身是不是 `Unpin` 跟 `T` 是不是 `Unpin` 没有任何关系，只跟 `P` 有关系。**
- **`Pin` 能不能把 `T`  pin 住跟 `P` 是不是 `Unpin` 没有任何关系，只跟 `T` 有关系。**

上面两句话虽然有点绕，但是搞清楚了后，很多 Pin 的场景就不会懵逼了。

### Deref 和 DerefMut

```rust
impl<P: Deref> Deref for Pin<P> {
    type Target = P::Target;
    fn deref(&self) -> &P::Target {
        Pin::get_ref(Pin::as_ref(self))
    }
}

impl<P: DerefMut<Target: Unpin>> DerefMut for Pin<P> {
    fn deref_mut(&mut self) -> &mut P::Target {
        Pin::get_mut(Pin::as_mut(self))
    }
}
```

这两个 trait 对 `Pin` 非常关键。只有实现了 `Deref` 后 `Pin<P>` 才是智能指针，开发者才可以无缝的调用 `P` 的方法。值得注意的是，只有满足 `T: Unpin` ，才会给 `Pin<P<T>>` 实现 `DerefMut`。因为 Safe Rust 下，`Pin<P<T>>` 的职责之一就是不能在不满足 `T: Unpin` 的前提条件下暴露 `&mut T`。

另外，实现了这两个 trait 之后，分别可以解引用拿到 `&T` 和 `&mut T`，但这种解引用的方式和 `get_ref()`、`get_mut()` 是有区别的。以 `&T` 为例，假设有 `let p = Pin::new(&T);`, 解引用 `p` 拿到 `&T`： `let t = &*p`;，这里拿到的 `&T` 的生命周期其实等于 `&Pin::new(&T)` 的生命周期。而 `Pin::new(&T).get_ref()` 拿到的 `&T` 的生命周期和 `Pin` 自身的生命周期是相等的。

为什么会这样呢？我们把解引用智能指针的语法糖展开之后看看。

```rust
let p = Pin::new(&T);
// let t = &*p; 展开语法糖之后如下：
let t = &*Deref::deref(&p);
```

`Pin` 的 `Deref` 实现代码是：`Pin::get_ref(Pin::as_ref(self))`，而 `Pin::as_ref()` 的代码如下。通过对比，就能发现解引用拿到的 `&T` 的生命周期确实和 `get_ref()` 拿到的是不一样的。

```rust
impl Pin<P> where P: Deref {
    pub fn as_ref(&self) -> Pin<&P::Target> {
        unsafe { Pin::new_unchecked(&*self.pointer) }
    }
}

// 对比上面 Deref 的实现
impl<'a, T: ?Sized> Pin<&'a T> {
    pub const fn get_ref(self) -> &'a T {
        self.pointer
    }
}
```

另外一个值得注意的地方是 `Pin::as_ref()` 和 `Pin::as_mut()` 里面会解引用 `self.pointer`，其实就是会调用它的 `deref()` 或 `deref_mut()` 方法。这两个方法是由 `P` 自己实现的，所以这里会存在“恶意实现”把 `T` move 掉的可能性。但是这种“恶意实现”会被 **Pin 的契约** 排除掉：这是你自己“恶意实现”导致的，并不是由于使用 `Pin` 导致。

> Pin::new_unchecked() 的文档特意强调到了这一点：
>
> By using this method, you are making a promise about the `P::Deref` and `P::DerefMut` implementations, if they exist. Most importantly, they must not move out of their `self` arguments: `Pin::as_mut` and `Pin::as_ref` will call `DerefMut::deref_mut` and `Deref::deref` *on the pinned pointer* and expect these methods to uphold the pinning invariants.

```rust
use std::pin::Pin;
use std::marker::PhantomPinned;
use std::ptr::NonNull;
use std::ops::{Deref, DerefMut};

struct Unmovable {
    data: String,
    slice: NonNull<String>,
    _pin: PhantomPinned,
}

impl Unmovable {
    fn new(data: String) -> Pin<Boz<Self>> {
        let res = Unmovable {
            data,
            slice: NonNull::dangling(),
            _pin: PhantomPinned,
        };
        let mut bozed = unsafe { Pin::new_unchecked(Boz(res)) };

        let slice = NonNull::from(&bozed.data);
        // we know this is safe because modifying a field doesn't move the whole struct
        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut bozed);
            Pin::get_unchecked_mut(mut_ref).slice = slice;
        }
        bozed
    }
}

impl Default for Unmovable {
    fn default() -> Self {
        Unmovable {
            data: String::new(),
            slice: NonNull::dangling(),
            _pin: PhantomPinned,
        }
    }
}

struct Boz<T>(T);

impl<T> Deref for Boz<T>  {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// “Malicious” implementations of DerefMut
impl<T: Default> DerefMut for Boz<T>  {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let _s = std::mem::take(&mut self.0);
        &mut self.0
    }
}

fn main() {
   let mut unmovable = Unmovable::new(String::from("Malicious!!!"));
   unmovable.as_mut();
}
```

上面的例子，我们构造了一个 `Pin<Boz<Unmovable>>`，然后调用 `as_mut()` 方法的时候会解引用这个 `Boz`，`Boz` 有一个“恶意”的 `DerefMut` 实现，里面会 move 掉这个 `Unmovable`。但明明我是是把它 `Pin` 住了的。

### Future

`Pin` 还实现了 `Future`， 这个和 `Unpin` 关系密切，我们在接下来的部分统一讲解。

## Unpin 和 Future

Rust 的 pinning API 让初学者困惑的一大难点就是 `Unpin` 的引入，往往很容易把人绕晕，所以我们必须要彻底搞清楚 `Unpin`，特别是它和 `Future` 的关系。 

前面说过，`Unpin` 是一个 auto trait，几乎所有类型都实现了 `Unpin`，包括你没意识到的一些类型。比如：

- **&T**: `impl<'a, T: ?Sized + 'a> Unpin for &'a T {}`
- **&mut T**: `impl<'a, T: ?Sized + 'a> Unpin for &'a mut T {}`
- __*const T__: `impl<T: ?Sized> Unpin for *const T {}`
- __*mut T__: `impl<T: ?Sized> Unpin for *mut T {}`
- 其他，包括 `Box`, `Arc`, `Rc` 等等

注意这里是不管 `T` 满不满足 `T: Unpin` ，它们都是 `Unpin`。原因我们前面已经说了：**`Pin` 能不能把 `T`  pin 住跟 `P` 是不是 `Unpin` 没有任何关系，只跟 `T` 有关系。**

> 第一篇文章有讲过，只有 **std::marker::PhatomPinned** ，包含 **PhatomPinned** 的类型，还有 `.await` 解语法糖之后的那个结构体是 `!Unpin`，这里不再赘述。

### Unpin 是 safe trait

另外一个很重要的特点：**`Unpin` 是一个 safe trait。**也就是说你可以在 safe Rust 下给任意类型实现 `Unpin`，包括你的 `Future` 类型。

> 我们提前准备好两个 assert 函数，后面会用到：
>
> ```rust
> fn assert_future<F: Future>(_f: F) {}
> fn assert_unpin<T: Unpin>(_t: T) {}
> ```

```rust
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

#[derive(Clone)]
struct Dummy(String);

impl Future for Dummy {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("{}", self.0);
        Poll::Ready(())
    }
}

// 不加这一行，编译器也会帮你自动实现 Unpin
impl Unpin for Dummy {}

fn main() {
    let dummy = Dummy(String::from("hello world!"));
    assert_future(dummy.clone());
    assert_unpin(dummy);
}
```

如果你要在另外一个 `Future` 中去 poll 这个 `Dummy` future 也是完全没有问题的。`futures` crate 甚至还提供了一系列 [unpin 版的方法](https://docs.rs/futures/latest/futures/index.html?search=unpin)来帮助你做这件事，比如 [FutureExt::poll_unpin()](https://docs.rs/futures/latest/futures/future/trait.FutureExt.html#method.poll_unpin) 。

```rust
pub trait FutureExt: Future {
    /// A convenience for calling `Future::poll` on `Unpin` future types.
    fn poll_unpin(&mut self, cx: &mut Context<'_>) -> Poll<Self::Output>
    where
        Self: Unpin,
    {
        Pin::new(self).poll(cx)
    }
}
```

可以看到这里是 `&mut self`，而不是 `self: Pin<&mut Self>`。

但是，pin projection 场景下需要特别注意，如果你的类型中有字段是 `!Unpin` 的，你就不能给这个类型实现 `Unpin`。具体可以看官网 [Pinning *is* structural for field](https://doc.rust-lang.org/std/pin/index.html#pinning-is-structural-for-field) 。

### 为什么 Future 可以是 Unpin 的

可能有人就要问了，**Pin** 设计之初不是为了解决实现 `Future` 的那个自引用结构体不被 move 的问题吗？为什么还可以给 `Future` 类型实现 `Unpin`？原因是这样的：如果你实现 `Future` 的类型是自引用结构体，那当然不能是 `Unpin` 的，除此之外实现 `Unpin` 完全没问题。上面那个例子，也包括很多第三方库的 `Future` 类型，都不会存在自引用结构体，可以放心大胆 move，所以完全可以是 `Unpin` 的。另外一个好处是，完全可以用 safe 版的 `Pin::new()` 方法构造 `Pin` 去poll future，不需要跟 unsafe 打交道。

### Pin 的 Future 实现

之所以移到这里来讲 `Pin` 的 `Future` 实现，是因为 **1.56** 有一个 PR [#81363](https://github.com/rust-lang/rust/pull/81363)  把 `P: Unpin` 的限定去掉了。我们先来看一下为什么要给 `Pin` 实现 `Future`，然后再来分析为什么这里的 `Unpin`  限定可以放开。

```diff
impl<P> Future for Pin<P>
where
-   P: Unpin + ops::DerefMut<Target: Future>,
+   P: ops::DerefMut<Target: Future>,
{
    type Output = <<P as ops::Deref>::Target as Future>::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
-       Pin::get_mut(self).as_mut().poll(cx)
+       <P::Target as Future>::poll(self.as_deref_mut(), cx)
        // self.as_deref_mut() 其实就是 unsafe { self.get_unchecked_mut() }.as_mut()
    }
}
```

给 `Pin` 实现 `Future` 的理由很简单，就是为了方便调用 `poll()`，特别是在 pin projection 的场景。因为 `poll()` 的 `self` 是 `Pin<&mut Self>` 类型，你没办法直接用 `future` 来调用 `poll()`。

```
error[E0599]: no method named `poll` found for struct `Dummy` in the current scope
  --> src/main.rs:35:20
   |
35 |         Dummy(String::from("hello world!")).poll(cx)
   |                    						 ^^^^ method not found in `Dummy`
```

你必须要先构造一个 `Pin<&mut Dummy>` 才能调用 `poll()`。给 `Pin` 实现 `Future` 之后，你可以直接这样写：`Pin::new(&mut dummy).poll(ctx)`，要不然你还需要写 `Future::poll(Pin::new(&mut dummy), ctx)`。

再来看看为什么这里不需要 `P: Unpin`。首先，这个方法的目的是要去 poll `P::Target` 这个 `Future`，而 `poll()` 方法的 `Self` 是 `Pin<P<T>>`，`self` 是 `Pin<&mut Pin<P<T>>>`（注意这里有两层 `Pin`）。我们要把 `Pin<&mut Pin<P<T>>>` 安全地转换成 `Pin<&mut T>` 才能调用 `P::Target` 的 `poll()`。那查一下 `Pin` 的 API 来倒推一下，拿到 `Pin<&mut T>` 很简单，有 `Pin::as_mut()`，前后两个版本的最后都是调用 `as_mut()` 的，这里没问题。但 `as_mut()` 的签名是 `&mut self`，也就是我们要先拿到 `&mut Pin<P<T>>`。如果把 `Pin<&mut Pin<P<T>>>`还原成基本形式 `Pin<P<T>>` 的话，那 `&mut` 是那个 `P`，`Pin<P<T>>` 是那个 `T`。从 `Pin<&mut Pin<P<T>>>` 拿到 `&mut Pin<P<T>>`  其实就是需要从 `Pin<P<T>>` 拿到 `&mut T`。`get_mut()`  和 `get_unchecked_mut()` 两个方法都能满足，唯一的区别是 `Unpin` 限定，这也是那个 PR 改动的地方。没有 `Unpin` 限定的话，我们只能使用 unsafe 版的 `get_unchecked_mut()` 。但是这里是完全安全的，因为我们拿到 `&mut Pin<P<T>>` 后马上就调用 `as_mut()` 了，并不会 move 它。所以之前的 `P: Unpin` 是多余的。更多细节可以查看 [Pin::as_deref_mut()](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.as_deref_mut) 的文档和源码注释。

### 为什么需要 Unpin 限定

上面也提到过，有些异步相关的 API 需要你的类型满足 `Unpin` 才能调用。目前我所了解到的，这些 API 大致可以分为三类：
1. **需要 `&mut future` 的场景。**比如 [tokio::select!()](https://docs.rs/tokio/latest/tokio/macro.select.html)， 这个宏需要你的 `Future` 满足 `Unpin`；
2. **`AsyncRead` / `AsyncWrite` 的场景。**比如 [tokio::io::AsyncWriteExt](https://docs.rs/tokio/latest/tokio/io/trait.AsyncWriteExt.html) 的方法需要你的 `Self` 满足  `Unpin`。
3. **`Future` 本身就是符合 `Unpin`，并且不想直接和 `Pin` 打交道。**上面提到的 `FutureExt::poll_unpin()`方法就属于这一类。

第 (2) 类主要跟 `AsyncRead` / `AsyncWrite` 的 `self` 需要 `Pin<&mut Self>` 有关，社区也有不少这方面的讨论，不是这篇文章的重点，感兴趣可以查看下面的资料。

> - futures-rs: [Should AsyncRead and AsyncWrite take self by Pin?](https://github.com/rust-lang/futures-rs/issues/1454)
> - tokio: [Should AsyncRead/AsyncWrite required pinned self?](https://github.com/tokio-rs/tokio/issues/1272)
> - [Tokio’s AsyncReadExt and AsyncWriteExt require Self: Unpin. Why and what to do about it?](https://users.rust-lang.org/t/tokios-asyncreadext-and-asyncwriteext-require-self-unpin-why-and-what-to-do-about-it/64134/4)
>
> 其次，tower 也在考虑要不要加 `Pin<&mut Self>`：[Pinning and Service](https://github.com/tower-rs/tower/issues/319 ).

关于第 (1) 类，主要原因是给 `&mut Future` 实现 `Future` 的时候指定需要了 `F: Unpin`。

```rust
impl<F: ?Sized + Future + Unpin> Future for &mut F {
    type Output = F::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        F::poll(Pin::new(&mut **self), cx)
    }
}
```

所以归根结底需要搞清楚为什么这里需要 `Unpin`。先从场景谈起，假如我们有一个 `future` 需要在 `loop` 中不断地 poll 它，但是 `Future::poll()` 每调用一次就会消耗掉 `self` 的所有权。因此我们需要可变借用这个 `future`，避免消耗掉 `future` 的所有权。但是 `&mut future` 之后会存在 move 掉这个 `future` 的风险（“万恶之源”），所以要么你这个 `future` 是 `Unpin` 的，要么你就要把它 pin 住再可变借用它（也就是 `&mut Pin<&mut future>`）。而刚好  `Pin<P>
where P: DerefMut` 实现了 `Future`！（前面部分刚讲过的）而且 `Pin<P>` 也是满足 `Unpin` 的！真的太完美了，我们干脆给 `&mut F` 实现 `Future` 吧，只要 `F` 满足 `Future + Unpin` 就行。这样的好处是如果你的 `future` 满足 `Unpin`，那你直接 `loop` 中多次 poll 就行，不用担心 move 的问题；如果你的 `future` 不满足 `Unpin`，那也没关系，把它 pin 住就好了。比如下面的例子，因为 [tokio::time::Sleep](https://docs.rs/tokio/latest/tokio/time/struct.Sleep.html) 不满足 `Unpin`，你需要先使用 `tokio::pin!()` 把它 pin 住才可以编译通过。

```rust
use tokio::time::{self, Duration, Instant};

#[tokio::main]
async fn main() {
    let sleep = time::sleep(Duration::from_millis(10));
    tokio::pin!(sleep);

    loop {
        tokio::select! {
            () = &mut sleep => {
                println!("timer elapsed");
                sleep.as_mut().reset(Instant::now() + Duration::from_millis(50));
            },
        }
    }
}
```

另外，同样的道理，给 `Box<F>` 实现 `Future` 的时候也需要 `Unpin`。

```rust
impl<F: ?Sized + Future + Unpin, A: Allocator> Future for Box<F, A>
where
    A: 'static,
{
    type Output = F::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        F::poll(Pin::new(&mut *self), cx)
    }
}
```

## 其他需要 Pin 的场景

经常碰到有人问类似"我这个场景是不是需要用 `Pin` 来解决？"的问题，我看了一下，根本跟 `Pin` 没有任何关系，于是会用这句经典名言回复他：

> Rust 社区经典名言：**Whenever you wonder if Pin could be the solution, it isn't.**

**Pinning API** 是朝着通用性设计的，并不是只为了解决异步里面自引用结构体 move 的一个问题，其他一些场景也需要用到 `Pin`。

### Intrusive collections

**Intrusive collections**（侵入式集合） 是 `Pin` 的另外一个应用场景。 `Pin` 的文档上有提到 **intrusive doubly-linked list** （侵入式双链表）这个例子，但是对于其他侵入式数据结构也类似（比如侵入式单链表）。不过文档只有言简意赅的几句话，并不是很好理解，我在这里简单的总结一下。

首先需要了解 **instrusive collections** 是什么。我们平时用到的集合数据结构几乎都是 **non-intrusive** 的，比如标准库的 `Vec`，`LinkedList` 等。**non-intrusive** 类型集合的特点是集合中的元素和集合自身是完全解耦的，集合不需要关心每一个元素的类型是什么，集合也可以用来存放任意类型的元素。但 **intrusive** 类型的集合是一种完全侵入式的集合，它的 `prev` 或 `next` 指针是定义在元素上面的。

以 C++ 为例子 ，**non-intrusive** 的双链表可以这样定义：

```c++
struct Point {
    float x, y;
};

struct ListNode {
    Point val;
    ListNode *next, *prev;
};
```

而 **intrusive** 的版本需要这样写：

```c++
struct Point {
    float x, y;
    Point *next, *prev;
};
```

Rust 版 **intrusive** 的伪代码大概也是这样：

```rust
struct Point {
    x: f64,
    y: f64,
    prev: Option<Rc<Point>>,
    next: Option<Rc<Point>>,
}
```

可以看到两者最大的区别在于指针是放在集合上面还是放在元素上面。两种类型的集合各自有优缺点，而 **intrusive** 类型最大的优点在于性能更好，缺点也很明显，不通用，不同的元素需要重复定义集合。相关的知识不是本文的重点，更多详细信息可以看看下面的资料：

> - [Google Fuchsia 提供的侵入式容器](https://fuchsia.dev/fuchsia-src/development/languages/c-cpp/fbl_containers_guide/introduction)
> - [Intrusive linked lists](https://www.data-structures-in-practice.com/intrusive-linked-lists/)
> - [Safe Intrusive Collections with Pinning](https://www.ralfj.de/blog/2018/04/10/safe-intrusive-collections-with-pinning.html)

那为什么侵入式集合需要用到 `Pin` 呢？其原因在于元素之间互相有 `prev` 或 `next` 指针指向自己，如果中间某个元素发生 move 了，那其他元素指向它的指针地址就失效了，导致不安全行为。所以必须要用 `Pin` 来把元素给 pin 住！Rust 有一个叫 [intrusive-collections](https://docs.rs/intrusive-collections) 的库提供了众多侵入式的集合类型，另外 Tokio  里面也定义了[侵入式集合](https://docs.rs/tokio/latest/src/tokio/util/linked_list.rs.html)，毫无疑问他们都用到了 `Pin`。

### 其他

其实，只要是需要处理防止被 move 的场景，理论上都需要用到 `Pin` 来解决。其他的 case 我暂时想不出来了，以后有发现新的场景再补充，或者如果大家知道其他场景欢迎告诉我。

## 总结

这篇文章稍微有点长，我们来总结一下：

- `Pin` 的 API 设计很讲究，甚至充满对称性，它的方法大致可以分为 5 类。其中涉及到 `Unpin` 和 `&mut T` 又可以细分为 safe 和 unsafe 两种；
- `Pin` 的 `#[fundamental]` 和 `#[repr(transparent)]` 很重要，但你一般不需要关心它；
- `Pin` 实现的 trait 需要重点关注 `Unpin` ，`Deref` / `DerefMut`  和 `Future`，搞懂他们你才能完全掌握 `Pin`；
- `Unpin` 和 `Future` 关系十分密切。`Unpin` 是 safe trait，理论上可以任意实现，`Future` 也可以是 `Unpin` 的。一些异步 API 中可能需要 `Unpin` 限定，需要理解它的原因，而不是一味的只顾使用。
- `Pin` 是一种通用 API，除了 `async / await` 下需要 `Pin` 之外，也会有其他场景需要用 `Pin` 来解决，比如 **intrusive collections**。

文章中多次提到的 **Pin projection** 没有展开讲，下一篇文章再来详细探讨它。再会！

