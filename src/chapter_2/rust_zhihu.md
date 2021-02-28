# 知乎 Rust 圆桌年话专题问答精选

编辑：张汉东

在牛年春节期间，我在知乎发起 [Rust 语言圆桌年话 | 关于 Rust 语言基金会成立，你有什么想说的呢？
](https://www.zhihu.com/question/443595816)

---

**[关于 Rust 语言基金会成立，你有什么想说的呢？](https://www.zhihu.com/question/443595816)**

**@韩朴宇：**
    
    链接：https://www.zhihu.com/question/443595816/answer/1734191236
    来源：知乎
    著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

华为在创始成员中既惊讶又感到正常，因为并没有听说华为在rust项目上的投资（其他4个成员公司存在大量的Rust项目组成员），但是华为也有Rust写的产品，比如StratoVirt。StratoVirt 是华为的企业级Linux操作系统EulerOS的开源版openEuler旗下的一个项目，是一个基于Linux KVM的虚拟机StratoVirt兼容QEMU的QMP API,同时支持x86和鲲鹏arm处理器，并且使用virtio半虚拟化设备接口。除了华为的StratoVirt, 还有一些Rust编写的虚拟机。最早的应该是Google的crosvm （cros是ChromeOS的缩写），这个虚拟机管理器是为了在ChromeOS上运行一个单独的Linux虚拟机而设计的（即Crostini 计划）。

ChromeOS是一个类似于Android的系统，其系统分区是只读的，使用A/B分区的方式无缝升级，并且使用单独的用户数据分区。但是不同于Android高度定制化的用户空间，ChromeOS的用户空间就是用Gentoo Linux的包管理器Portage编译出来的，因此ChromeOS是一个标准的GNU/Linux系统。但是Google认为直接在该系统上运行任意的Linux程序会损害ChromeOS的安全性，因此在ChromeOS上运行一个轻量级虚拟机来运行一个命令行版的ChromeOS, 该系统可以运行LXC容器，默认的容器是Debian。Google认为这样套娃下来，既可以运行普通的Linux程序，又不会产生安全性问题。crosvm的特色是实现了一个基于virtio的Wayland总线，可以将虚拟机的Wayland/Xwayland程序的窗口直接穿过虚拟机的界限绘制到主系统的Wayland合成器上。使用最广的应该是AWS的 firecracker-microvm/firecracker ，AWS已经将其用于生成环境。此外还有Intel的 cloud-hypervisor/cloud-hypervisor，不仅支持x64, 而且像前3者一样也支持ARM64,而且还支持Windows 10。Rust在KVM上的生态离不开rust-vmm项目，该项目提供了对KVM api的绑定，该项目带起了整个Rust虚拟机的生态。

    
**@iyacontrol：**

    链接：https://www.zhihu.com/question/443595816/answer/1723079060
    来源：知乎
    著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。


首先恭喜Rust有了好爸爸，而且不止一个。而且可以预见不久的未来，IBM、阿里云、腾讯云等大厂也会加入进来。有了这么多的好爸爸的加持，小伙伴们可以放心大胆地用Rust了，不用再担心Rust被砍掉了。通过基金会的成员来看，除了亲爸爸Mozilla，其他member大多都和云有关系。可以得出两点：Rust 的安全性和不差的性能，适合用来写一些偏底层的软件，比如各种运行时。而且也得到了大家一致的认可。Rust 将在云原生领域大放异彩了。目前来看，很有可能和Golang相互配合，Rust负责底层部分，Go负责中间部分，共同服务上层各种语言的应用。另外，感谢Mozilla的不为五斗米折腰，没有让Rust走了Java的路。如果Rust卖给类似于甲骨文的公司，那么Rust的前景就不好说了。


**@最帅的物理课代表：**

    链接：https://www.zhihu.com/question/443595816/answer/1734618924
    来源：知乎
    著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

虽然我是老华为黑粉了，但是其实很开心能看到华为在创始人名单之列。rust语言是很有前途的语言，这几乎是业界共识。华为有自研的容器项目，采用rust语言编写，这是一个很有意义的作品，比hm系列高到不知道哪里去。我们能通过这些看到华为的决心和勇气。同时这也很能带动国内的其他互联网企业，一起为rust投入更多精力，也给全球的rust社区添砖加瓦。我国的互联网发展和欧美一些国家一直都有较大的差距。但是众所周知，我们的传统艺能就是弯道超车。


还有很多回答，可以去知乎查看。

---

**[您对 2021 年的 Rust 语言有哪些期待？](https://www.zhihu.com/question/438833112)**


**@韩朴宇:**

    链接：https://www.zhihu.com/question/438833112/answer/1673155747
    来源：知乎
    著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

我在`rustbuild（即src/bootstrap）`上提过几个pr，因此说几个和`rustc`相关的（或者说和语言无关的工程问题）。

1. `cranelift`以及`rustc_codegen_cranelift`可以大大加速debug build，test，proc_macro和build.rs的速度，结合`jit`模式，可以实现以接近cargo check的速度同时检查语法错误，借用检查错误和逻辑错误。目前cg_clif已经进入rust仓库，在SYSV abi，Windows ABI，原子操作，内联汇编，SIMD上还有一些问题。cg_clif是由一位开发者bjorn3单枪匹马写出来的，很厉害。另外新的asm！内联汇编宏不再使用llvm_asm的语法，就是因为有朝一日rustc会集成上全功能的rust编写的后端。由Inline Assembly project group开发

2. `std aware cargo`也就是 `cargo -Z build-std`，这个功能在优化二进制大小上很有用，在操作系统开发上是必需品。由std Aware Cargo working group负责。

3. `core::error::Error`, `core::io::Error`和`backtrace`支持这是`Error handling project group`的工作重点，目前已有demo可用。有了这个wasm，嵌入式和操作系统开发也可以用常用的错误处理库了。

4. `chalk` 。trait 系统的改进全靠这个，包括`GAT`由`traits working group`负责为什么我的期待都有working group，因为这就是rust项目的治理方式，没有working group的东西就肯定是没戏的，至少一年内是如此。比如取一个稳定的abi，作为rust abi和c++ abi的子集和C abi的超集，已经吵了好几年了，估计今年也是没戏。


**@Nugine：**

    链接：https://www.zhihu.com/question/438833112/answer/1672070201
    来源：知乎
    著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

`min const generics` 将于 1.51 稳定，大约3月底，可以解锁一些较为常规的设计方法。

GAT 仍然是我最期待的有生之年的特性，它与 async trait, monad 之类的东西相关，能派生出很多魔法设计。

`async-std 1.8`，`tokio 1.0`，希望更多常用的库不再犹豫，赶紧1.0。

希望 tracing 加快速度到 0.2，异步上下文追踪就指望它了。

生态中很多常见领域都已经有了至少一两个占主导地位的库，但还需要打磨。希望做到商业级、工业级可用。

希望 2021 Rust 多出一些杀手级产品，最好是国产的。


**@dontpanic:**

    链接：https://www.zhihu.com/question/438833112/answer/1673710125
    来源：知乎
    著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

我比较没出息，我只想要糖…… 最想要的几个：

1. `arbitrary_self_types`(p.s 这个例子并不是必须使用arbitrary self types，使用 associate function可以有同样的效果，参见评论区）真的好用，已经离不开了。

目前能用做 Self 类型的，只有 `self/&self/&mut self/Box<Self>/Rc<Self> `等几个类型。 Arbitrary self types 允许使用任意 Deref 到 Self 的类型用作 self。有什么用呢？比如，我想扩展下面的 

```rust
Base:trait Derived {
    fn foobar(&self);
}

struct Base<T: Derived> {
    ext: T,
}

impl<T: Derived> Base<T> {
    fn foo(&self) {
        self.ext.foobar();
    }

    fn bar(&self) {
        println!("bar!");
    }
}

struct DerivedImpl {
    base: Weak<RefCell<Base<DerivedImpl>>>,
}

impl Derived for DerivedImpl {
    fn foobar(&self) {
        self.base.upgrade().unwrap().borrow().bar();
        println!("foobar!");
    }
}
```

这样的实现就会强制 base 必须以使用 Rc 的方式使用，并且要小心多次 BorrowMut（很容易发生，要么就需要 Derived 提供 interior mutability）。或者也可以在 trait Derived 的函数参数里面把 base 传进去，但是有点 verbose。当然也可以说这种设计不够 rust idiomatic...不过有了 Arbitrary self types 之后，世界就清爽了。

首先实现一下`deref/deref_mut`：

```rust
impl<T: Derived + 'static> Deref for Base<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        &self.ext
    }
}

impl<T: Derived + 'static> DerefMut for Base<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.ext
    }
}
然后 Derived 可以直接：trait Derived : 'static + Sized {
    fn foobar(self: &mut Base<Self>);
}

struct DerivedImpl {
}

impl Derived for DerivedImpl {
    fn foobar(self: &mut Base<Self>) {
        self.bar();  // !!!!!
        println!("foobar!");
    }
}
```


多了 'static + Sized，但也可以接受。

2. [let_chains_2](https://github.com/rust-lang/rust/issues/53667)，啥也不说了，羡慕 Swift。

3. 标准库里面有很多 unstable 的函数，经常会一用上来发现还是 unstable 需要开 feature。自己的项目随便开开倒是无所谓，但生产环境必定要谨慎的多。希望能够尽快 stable，比如 drain_filter。


longfangsong:

    链接：https://www.zhihu.com/question/438833112/answer/1674659637
    来源：知乎
    著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。    

其他答主说的已经很好了，尤其是语言设计上的（GAT什么的大家都久等了），我再补充几点：

语言设计：

1. 看Rust Internals的时候看到的一个感觉有点意思的idea：更细粒度的unsafe。
2. 看到前面有人说的enumerate variant as type，我想要的一个和这个比较像的一个糖是 Typescript 那样的 (untagged) union type，目前我习惯是用enum_dispatch crate来部分模拟这个特性。

工具方面的：

1. IDE支持，CLion 现在index不了编译时生成的代码（即使开了RA也一样）。vsc可以但是RA有时会莫名其妙地hang住。
2. 能不能修修`cargo` 的 `[patch]` 只认repo的url而不管rev的问题，即`cargo#7497`
3. 求编译能再快一点……编译产物能再多复用一点……

社区建设方面：

1. 现在感觉很多还没有入门rust的人都被它“传言中”的难度吓到了，实际上rust也就是一门普通的语言，可能所有权检查、强制性的线程安全这些特性确实是别的语言没有的，但掌握这些特性其实也不比掌握指针之类的概念困难，还有其实很多看着很长很可怕的写法（`Option<Rc<RefCell>>>`）虽然第一眼看上去劝退实际上却更合理更可读（分离了是否可空、是否多个所有者、是否可变三个关注点，相比其他某些语言一个指针走天下其实更容易分析）。其实宣传的时候可以更多的去给新人一种rust并不难的印象，可以更好地壮大社区。
2. 有没有入口可以给rust foundation捐钱啊（x

还有很多回答，可以去知乎查看。

---

还有很多精彩的问题等待你的探索和回答：

- [Rust相较于Haskell除了效率还有何优势？](https://www.zhihu.com/question/31644802)
- [如何看待 Rust 的应用前景？](https://www.zhihu.com/question/30407715)
- [写 wasm 项目选 C++ 还是 Rust？](https://www.zhihu.com/question/442315024)
- [学Rust之前，是不是应该先学C++？](https://www.zhihu.com/question/400001723)
- [学习Rust, 可以绕开C语言吗?](https://www.zhihu.com/question/424290703)
- [在中国有多少开发者使用Rust编程语言？](https://www.zhihu.com/question/344733952)
- [只学过 C 语言适合学 Rust 吗？](https://www.zhihu.com/question/308540043)
- [GitHub 上有哪些值得关注的 Rust 项目？](https://www.zhihu.com/question/30511494)
- [如何开始学习 Rust 语言?](https://www.zhihu.com/question/31038569)
- [学习Rust适合写什么练手项目？](https://www.zhihu.com/question/34665842)
- [Rust程序员都做什么项目？](https://www.zhihu.com/question/352420716)
- [本科毕业论文想写点 Rust 语言相关的内容，什么样的题目比较好？](https://www.zhihu.com/question/441960256)
- [我应该放弃 C++，学习 Rust 吗？](https://www.zhihu.com/question/30408031)
