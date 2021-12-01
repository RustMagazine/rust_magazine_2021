# 蚂蚁集团 ｜ Play With Your Async Code

作者：夏锐航

> 作为 2018 edition 一个比较重要的特性，rust 的异步编程现在已经得到了广泛的使用。使用的时候难免会有一些好奇它是如何运作的，这篇文章尝试从 generator 以及变量捕获的方面进行探索，而后介绍了在嵌入式时序存储引擎 `ceresdb-helix` 的研发过程中遇到的一个场景。囿于作者水平内容难免存在一些错漏之处，还烦请留言告知。

---


async/.await 语法在 1.39 版本[1]进入 stable channel，它能够很方便地编写异步代码：
```rust
async fn asynchronous() {
    // snipped
}

async fn foo() {
    let x: usize = 233;
    asynchronous().await;
    println!("{}", x);
}
```
在上面的示例中，局部变量 `x` 能够直接在一次异步过程（`fn asynchoronous`）之后使用，和写同步代码一样。而在这之前，异步代码一般是通过类似 futures 0.1[2] 形式的组合子来使用，想要给接下来 (如 `and_then()`) 的异步过程的使用的局部变量需要被显式手动地以闭包出入参的方式链式处理，体验不是特别好。
​

async/.await 所做的实际上就是将代码变换一下，变成 generator/coroutine[3] 的形式去执行。一个 coroutine 过程可以被挂起，去做一些别的事情然后再继续恢复执行，目前用起来就是 `.await` 的样子。以上面的代码为例，在异步过程 `foo()`中调用了另一个异步过程 `asynchronous()` ，在第七行的 `.await` 时当前过程的执行被挂起，等到可以继续执行的时候再被恢复。
​

而恢复执行可能需要之前的一些信息，如在 `foo()`中我们在第八行用到了之前的信息 `x`。也就是说 `async` 过程要有能力保存一些内部局部状态，使得它们能够在 `.await`之后被继续使用。换句话说要在 generator state 里面保存可能在 yield 之后被使用的局部变量。这里需要引入 pin[4] 机制解决可能出现的自引用问题，这部分不再赘述。

## visualize generator via MIR

我们可以透过 MIR[5] 来看一下前面提到的 generator 是什么样子的。MIR 是 rust 的一个中间表示，基于控制流图 CFG[6] 表示。CFG 能够比较直观地展示程序执行起来大概是什么样子，MIR 在有时不清楚你的 rust 代码到底变成了什么样子的时候能够起到一些帮助。
​

想要得到代码的 MIR 表示有几种方法，假如现在手边有一个可用的 rust toolchain，可以像这样传递一个环境变量给 rustc ，再使用 cargo 进行构建来产生 MIR：
```bash
RUSTFLAGS="--emit mir" cargo build
```
构建成功的话会在 `target/debug/deps/` 目录下生成一个 `.mir` 的文件。或者也能通过 [https://play.rust-lang.org/](https://play.rust-lang.org/) 来获取 MIR，在 `Run` 旁边的溢出菜单上选择 MIR 就可以。
​

由 2021-08 nightly 的 toolchain 所产生的 MIR 大概是这个样子的，有许多不认识的东西可以不用管，大概知道一下

- _0, _1 这些是变量
- 有许多语法和 rust 差不多，如类型注解，函数定义及调用和注释等

就行了（我也只知道这么多）。
```
fn future_1() -> impl Future {
    let mut _0: impl std::future::Future; // return place in scope 0 at src/anchored.rs:27:21: 27:21
    let mut _1: [static generator@src/anchored.rs:27:21: 27:23]; // in scope 0 at src/anchored.rs:27:21: 27:23

    bb0: {
        discriminant(_1) = 0;            // scope 0 at src/anchored.rs:27:21: 27:23
        _0 = from_generator::<[static generator@src/anchored.rs:27:21: 27:23]>(move _1) -> bb1; // scope 0 at src/anchored.rs:27:21: 27:23
                                         // mir::Constant
                                         // + span: src/anchored.rs:27:21: 27:23
                                         // + literal: Const { ty: fn([static generator@src/anchored.rs:27:21: 27:23]) -> impl std::future::Future {std::future::from_generator::<[static generator@src/anchored.rs:27:21: 27:23]>}, val: Value(Scalar(<ZST>)) }
    }

    bb1: {
        return;                          // scope 0 at src/anchored.rs:27:23: 27:23
    }
}

fn future_1::{closure#0}(_1: Pin<&mut [static generator@src/anchored.rs:27:21: 27:23]>, _2: ResumeTy) -> GeneratorState<(), ()> {
    debug _task_context => _4;           // in scope 0 at src/anchored.rs:27:21: 27:23
    let mut _0: std::ops::GeneratorState<(), ()>; // return place in scope 0 at src/anchored.rs:27:21: 27:23
    let mut _3: ();                      // in scope 0 at src/anchored.rs:27:21: 27:23
    let mut _4: std::future::ResumeTy;   // in scope 0 at src/anchored.rs:27:21: 27:23
    let mut _5: u32;                     // in scope 0 at src/anchored.rs:27:21: 27:23

    bb0: {
        _5 = discriminant((*(_1.0: &mut [static generator@src/anchored.rs:27:21: 27:23]))); // scope 0 at src/anchored.rs:27:21: 27:23
        switchInt(move _5) -> [0_u32: bb1, 1_u32: bb2, otherwise: bb3]; // scope 0 at src/anchored.rs:27:21: 27:23
    }

    bb1: {
        _4 = move _2;                    // scope 0 at src/anchored.rs:27:21: 27:23
        _3 = const ();                   // scope 0 at src/anchored.rs:27:21: 27:23
        ((_0 as Complete).0: ()) = move _3; // scope 0 at src/anchored.rs:27:23: 27:23
        discriminant(_0) = 1;            // scope 0 at src/anchored.rs:27:23: 27:23
        discriminant((*(_1.0: &mut [static generator@src/anchored.rs:27:21: 27:23]))) = 1; // scope 0 at src/anchored.rs:27:23: 27:23
        return;                          // scope 0 at src/anchored.rs:27:23: 27:23
    }

    bb2: {
        assert(const false, "`async fn` resumed after completion") -> bb2; // scope 0 at src/anchored.rs:27:21: 27:23
    }

    bb3: {
        unreachable;                     // scope 0 at src/anchored.rs:27:21: 27:23
    }
}
```
这个 demo crate 中还有一些别的代码，不过对应上面的 MIR 的源码比较简单：
```rust
async fn future_1() {}
```
只是一个简单的空的异步函数，可以看到生成的 MIR 会膨胀很多，如果内容稍微多一点的话通过文本形式不太好看。我们可以指定一下生成的 MIR 的格式，然后将它可视化。步骤大概如下：
```rust
RUSTFLAGS="--emit mir -Z dump-mir=F -Z dump-mir-dataflow -Z unpretty=mir-cfg" cargo build > mir.dot
dot -T svg -o mir.svg mir.dot
```
能够在当前目录下找到 mir.svg，打开之后可以看到一个像流程图的东西（另一幅差不多的图省略掉了，有兴趣的可以尝试通过上面的方法自己生成一份）。

![image.png](https://intranetproxy.alipay.com/skylark/lark/0/2021/png/308920/1638122524244-d4258004-fc18-4508-973b-f61f1780afce.png#clientId=u7f915272-afe1-4&from=paste&height=492&id=u8859a66a&margin=%5Bobject%20Object%5D&name=image.png&originHeight=984&originWidth=3138&originalType=binary&ratio=1&size=456273&status=done&style=none&taskId=u6290baaa-a3c1-4681-8506-6c601e55534&width=1569)

这里将 MIR 按照基本单位 basic block (bb) 组织，原本的信息都在，并且将各个 basic block 之间的跳转关系画了出来。从上面的图中我们可以看到四个 basic blocks，其中一个是起点，另外三个是终点。首先起点的 `bb0` switch（match in rust）了一个变量 `_5`，按照不同的值分支到不同的 blocks。能大概想象一下这样的代码：
```rust
match _5 {
	0: jump(bb1),
    1: jump(bb2),
    _ => unreachable()
}
```
而 generator 的 state 可以当成就是那个 `_5`，不同的值就是这个 generator 的各个状态。`future_1`的状态写出来大概是这样
```rust
enum Future1State {
    Start,
    Finished,
}
```
如果是 §1 中的 `async fn foo()`，可能还会多一个枚举值来表示那一次 yield。此时再想之前的问题，就能够很自然地想到要跨越 generator 不同阶段的变量需要如何保存了。
```rust
enum FooState {
    Start,
    Yield(usize),
    Finished,
}
```
## generator captured

让我们把保存在 generator state 中，能够跨越 .await/yield 被后续阶段使用的变量称为被捕获的变量。那么能不能知道到底哪些变量实际上被捕获了呢？让我们试一试，首先写一个稍微复杂一点的异步函数：
```rust
async fn complex() {
    let x = 0;
    future_1().await;
    let y = 1;
    future_1().await;
    println!("{}, {}", x, y);
}
```
生成的 MIR 及 svg 比较复杂，截取了一段放在了附录中，可以尝试自己生成一份完整的内容。
​

稍微浏览一下生成的内容，我们可以看到一个很长的类型总是出现，像是这样子的东西：
```rust
[static generator@src/anchored.rs:27:20: 33:2]
// or
(((*(_1.0: &mut [static generator@src/anchored.rs:27:20: 33:2])) as variant#3).0: i32)
```
对照我们代码的位置可以发现这个类型中所带的两个文件位置就是我们异步函数 `complex()`的首尾两个大括号，这个类型是一个跟我们这整个异步函数相关的类型。
​

通过更进一步的探索我们大概能猜一下，上面代码片段中第一行的是一个实现了 Generator trait[7] 的匿名类型（struct），而 "as variant#3" 是 MIR 中的一个操作，Projection 的 Projection::Downcast，大概在这里[8]生成。在这个 downcast 之后所做的 projection 的到的类型是我们认识的 `i32`。综合其他类似的片段我们能够推测这个匿名类型和前面描述的 generator state 是差不多的东西，而各个 variant 是不同的状态元组，投影这个 N 元组能够拿到被捕获的局部变量。
​

## anchored

知道哪些变量会被捕获能够帮助我们理解自己的代码，也能够基于这些信息进行一些应用。
​

先提一下 rust 类型系统中特殊的一种东西 auto trait[9] 。最常见的就是 `Send`和`Sync`，这种 auto trait 会自动为所有的类型实现，除非显式地用 negative impl opt-out，并且 negative impl 会传递，如包含了 `!Send`的 Rc 结构也是 `!Send`的。通过 auto trait 和 negative impl 我们控制一些结构的类型，并让编译器帮忙检查。
​

比如 anchored[10] crate 就是提供了通过 auto trait 和 generator 捕获机制所实现的一个小工具，它能够阻止异步函数中指定的变量穿过 `.await` 点。比较有用的一个场景就是异步过程中关于变量内部可变性的获取。
​

通常来说，我们会通过异步锁如`tokio::sync::Mutex` 来提供变量的内部可变性；如果这个变量不会穿过 `.await` point 即被 generator state 捕获，那么 `std::sync::Mutex`这种同步锁或者 `RefCell`也能使用；如果想要更高的性能，避免这两者运行时的开销，那也能够考虑 `UnsafeCell`或其他 unsafe 手段，但是就有一点危险了。而通过 anchored 我们可以在这种场景下控制不安全因素，实现一个安全的方法来提供内部可变性，只要将变量通过 `anchored::Anchored`这个 ZST 进行标记，再给整个 async fn 带上一个 attribute 就能够让编译器帮我们确认没有东西错误地被捕获并穿越了 `.await`、然后导致灾难性的数据竞争。就想这样：
```rust
#[unanchored]
async fn foo(){
    {
        let bar = Anchored::new(Bar {});
    }
    async_fn().await;
}
```
而这种就会导致编译错误：
```rust
#[unanchored]
async fn foo(){
    let bar = Anchored::new(Bar {});
    async_fn().await;
    drop(bar);
}
```


对于 std 的 `Mutex`, `Ref` 和 `RefMut` 等常见类型，clippy 提供了两个 lints[11] ，它们也是通过分析 generator 的类型来实现的。并且与 anchored 一样都有一个缺点，在除了像上面那样明确使用单独的 block 放置变量外，都会出现 false positive 的情况[12]。因为局部变量在其他的形式下都会被记录下来[13]，导致信息被污染。


anchored 目前还缺少一些 ergonomic 的接口，attribute macro 和 ecosystem 的其他工具交互的时候也存在一点问题，欢迎感兴趣的小伙伴来了解一下 [https://github.com/waynexia/anchored](https://github.com/waynexia/anchored) 👋 （文档 [https://docs.rs/anchored/0.1.0/anchored/](https://docs.rs/anchored/0.1.0/anchored/)）


## Ref

[1]: [https://blog.rust-lang.org/2019/11/07/Async-await-stable.html](https://blog.rust-lang.org/2019/11/07/Async-await-stable.html)
[2]: [https://docs.rs/futures/0.1.21/futures/](https://docs.rs/futures/0.1.21/futures/)
[3]: [https://github.com/rust-lang/rfcs/blob/master/text/2033-experimental-coroutines.md](https://github.com/rust-lang/rfcs/blob/master/text/2033-experimental-coroutines.md)
[4]: [https://doc.rust-lang.org/std/pin/index.html](https://doc.rust-lang.org/std/pin/index.html)
[5]: [https://blog.rust-lang.org/2016/04/19/MIR.html](https://blog.rust-lang.org/2016/04/19/MIR.html)
[6]: [https://en.wikipedia.org/wiki/Control-flow_graph](https://en.wikipedia.org/wiki/Control-flow_graph)
[7]: [https://doc.rust-lang.org/std/ops/trait.Generator.html](https://doc.rust-lang.org/std/ops/trait.Generator.html)
[8]: [https://github.com/rust-lang/rust/blob/b834c4c1bad7521af47f38f44a4048be0a1fe2ee/compiler/rustc_middle/src/mir/mod.rs#L1915](https://github.com/rust-lang/rust/blob/b834c4c1bad7521af47f38f44a4048be0a1fe2ee/compiler/rustc_middle/src/mir/mod.rs#L1915)
[9]: [https://doc.rust-lang.org/beta/unstable-book/language-features/auto-traits.html](https://doc.rust-lang.org/beta/unstable-book/language-features/auto-traits.html)
[10]: [https://crates.io/crates/anchored](https://crates.io/crates/anchored)
[11]: [https://rust-lang.github.io/rust-clippy/master/#await_holding](https://rust-lang.github.io/rust-clippy/master/#await_holding)
[12]: [https://github.com/rust-lang/rust-clippy/issues/6353](https://github.com/rust-lang/rust-clippy/issues/6353)
[13]: [https://doc.rust-lang.org/stable/nightly-rustc/src/rustc_typeck/check/generator_interior.rs.html#325-334](https://doc.rust-lang.org/stable/nightly-rustc/src/rustc_typeck/check/generator_interior.rs.html#325-334)

## Appendix

```rust
fn future_1::{closure#0}(_1: std::pin::Pin<&mut [static generator@src/anchored.rs:35:21: 35:23]>, _2: std::future::ResumeTy) -> std::ops::GeneratorState<(), ()>
let mut _3: ();
let mut _4: std::future::ResumeTy;
let mut _5: u32;
debug _task_context => _4;
fn complex::{closure#0}(_1: std::pin::Pin<&mut [static generator@src/anchored.rs:27:20: 33:2]>, _2: std::future::ResumeTy) -> std::ops::GeneratorState<(), ()>
let mut _3: impl std::future::Future;
let mut _4: std::task::Poll<()>;
let mut _5: std::pin::Pin<&mut impl std::future::Future>;
let mut _6: &mut impl std::future::Future;
let mut _7: &mut impl std::future::Future;
let mut _8: &mut std::task::Context;
let mut _9: &mut std::task::Context;
let mut _10: std::future::ResumeTy;
let mut _11: isize;
let _12: ();
let mut _13: std::future::ResumeTy;
let mut _14: ();
let mut _15: impl std::future::Future;
let mut _16: std::task::Poll<()>;
let mut _17: std::pin::Pin<&mut impl std::future::Future>;
let mut _18: &mut impl std::future::Future;
let mut _19: &mut impl std::future::Future;
let mut _20: &mut std::task::Context;
let mut _21: &mut std::task::Context;
let mut _22: std::future::ResumeTy;
let mut _23: isize;
let _24: ();
let mut _25: std::future::ResumeTy;
let mut _26: ();
let _27: ();
let mut _28: std::fmt::Arguments;
let mut _29: &[&str];
let mut _30: &[&str; 3];
let _31: &[&str; 3];
let mut _32: &[std::fmt::ArgumentV1];
let mut _33: &[std::fmt::ArgumentV1; 2];
let _34: &[std::fmt::ArgumentV1; 2];
let _35: [std::fmt::ArgumentV1; 2];
let mut _36: (&i32, &i32);
let mut _37: &i32;
let mut _38: &i32;
let _39: &i32;
let _40: &i32;
let mut _41: std::fmt::ArgumentV1;
let mut _42: &i32;
let mut _43: for<'r, 's, 't0> fn(&'r i32, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>;
let mut _44: std::fmt::ArgumentV1;
let mut _45: &i32;
let mut _46: for<'r, 's, 't0> fn(&'r i32, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>;
let mut _47: &[&str; 3];
let mut _48: ();
let mut _49: std::future::ResumeTy;
let mut _50: u32;
```


