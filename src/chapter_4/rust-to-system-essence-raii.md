# Rust 探索系统本原 | RAII 

作者：陈天 / 后期编辑：张汉东

> 原文链接：[https://mp.weixin.qq.com/s/jaKjzc_1rkDe67rfpnFTgg](https://mp.weixin.qq.com/s/jaKjzc_1rkDe67rfpnFTgg)

---

# 透过 Rust 探索系统的本原：RAII

RAII - Resource Acquisition Is Initialization[1] 是个听起来高深莫测不知所云，但实际理解起来并不困难的概念。我们在理解一个新的解决方案的时候，先深入了解它面临的问题，再看之前的解决方案（prior work），然后再看它是怎么解决同样的问题，最后比较优劣。这大概是做研究的通识，也是我们从本原去真正理解事物的最佳途径。

如果要找计算机历史上最离谱的缩略词，RAII 一定会榜上有名，从嘴里吐出这个词却不做额外解释，除了用于居高临下地吓唬初学者，似乎别无他用。Resource Acquisition Is Initialization，中文翻译为「资源获取即初始化」，无论中英文，都很难让人明白它的真实含义。但这里有个词值得我们警觉：资源。

这里的资源，是指文件，锁，套接字等由操作系统提供给应用程序的，数量有限的东西。虽然内存往往也是操作系统提供的一种资源，但它的处理和上述其它资源还是有许多不同。

在我的上一篇文章《透过 Rust 探索系统的本原：编程语言》里，我提到了 Rust 是如何通过把问题回归到「一个值（value）在上下文中被安全访问的基本规则」，从而解决了内存安全的问题。

虽然很多语言也可以通过其他手段解决内存安全的问题，比如使用 GC 来在运行时对内存的使用做引用计数，如果一个 socket 对象的引用计数为零，GC 会把 socket 对象对应的内存清除，但如果 socket 本身没有被显式 close，这个 socket 资源就会被泄露。很多 Java 工程师会遇到系统运行一段时间便累计了大量 `CLOSE_WAIT` 的 socket，最终会吃光进程的文件句柄，抛出 too many files 异常的问题[2]。所以，如果你没有在各种可能的路径中妥善地关闭 socket，那么，即便使用一个内存安全的系统，依然有资源泄露的可能。

所以，虽然同为资源，内存和其它操作系统提供的资源，对于程序员来说，其处理方式是非常不同的。你即便不用担心内存资源的释放，但却要妥善地释放锁，关闭 socket，关闭文件等等。

有没有可能让资源的处理方式和内存的处理方式统一起来？这就是 RAII 要解决的问题。统一的方式其实很直观，就是让资源和资源对应的对象的生命周期保持一致，具体来说：

- 对象的初始化会导致资源的初始化
- 对象的释放会导致资源的释放

这里有两重保障：对象创建成功一定意味着资源获取成功；而对象释放成功则资源一定得到释放。所以使用 RAII 的话，我们只需要保证内存安全（没有内存泄漏），就能够保证资源安全（没有资源泄露）。

从这里我们可以看到 RAII 是一个多么奇怪的名字 —— 它字面上只包含了上面的第一层功能（对象的初始化会导致资源的初始化），直接忽略了第二层功能，而被忽略的第二层功能其实是 RAII 真正需要保障的。

RAII 是一种资源释放的思路，这种思路被应用在各种语言之中（尽管具体采用的方法不同），比如：

- 在 Python 中，我们可以使用 `with` 来确保资源在退出某个 scope 时一定会被释放
- 在 Golang 中，我们可以使用 `defer` 让资源获取的代码和资源释放的代码尽可能靠近，这样让程序容易理解。
- 在 Rust 中，RAII 的思维被深深地嵌入到语言之中：所有权模型保证了当前 scope 所拥有的对象在退出 scope 时必然会被释放，而 `Drop` trait 保证了释放时，其相关的操作系统资源也得到释放。

我们以 Mutex Lock 为例，看看 Non-RAII 代码和 RAII 代码的区别：

```rust
fn bad() {
  m.lock();
  f(protected_data);
  if failed() return;
  m.unlock();
  ...
}
```

这段代码是典型的 Non-RAII 代码（也是很多语言处理资源的通用方式），调用者需要显式地释放锁。然而在整个过程中，可能出现错误，可能抛出异常，所以，释放锁的代码很可能没有执行，导致锁资源泄露。这是很多死锁问题出现的一大原因。

而支持 RAII 的代码：

```rust
fn good(m: Arc<Mutex<Protected>>) {
  let guard = m.lock();
  f(*guard);
  if failed() return;
  ...
}
```

在这里例子里，`lock()` 使用 RAII 实现，在获取这个锁的时候，它初始化了一个 `MutexGuard` 结构，这个结构里包含了对 `Mutex` 的引用，在做 `DerefMut` 时，可以访问到 `Mutex` 内部数据的可变引用，因而可以对数据进行修改。同时，`MutexGuard` 实现了 `Drop` ，里面处理了锁的释放，这样，当前的函数（scope）执行完退出时，锁就自动被释放了。

## 为什么 RAII 没有被普遍实现？

通过上文的介绍，我们可以看到 RAII 极大地解放了开发者，使其不必关心资源的释放，所写的代码反而比进行资源释放的代码更加安全可靠。为什么这种实践没有成为一种更加广泛的实践呢？似乎目前只有 C++ 在有限的场合使用，而实现地比较透彻的只有 Rust？

这还是因为 Rust 的所有权模型从本质上规范了一个值可以被安全使用和安全释放的场景，而这种规范对资源来说也是完美契合的。这就好比麦克斯韦把电磁光统一在一个框架之下，Rust 也把内存和其它资源统一成一种行为。

我们还是拿 Java 这种使用 GC 的语言来比较（不好意思 Java 我不是针对你）。如果你在堆上创建了一个文件资源，如果使用 RAII，意味着堆上的文件对象被释放的时候，这个文件资源也该关闭（`file.close()`），这么做是最安全也是最合理的。然而，因为堆上的对象什么时候被释放是不可知的，就算所有引用都不存在，GC 已经将其 mark 成可回收，回收线程什么时候被调度，依旧是无法保证的，就像薛定谔的猫。所以 Java 建议你资源用完了就要立刻关闭，于是你不得不殚精竭虑地照顾好每一个 try catch，每一次提前返回，以便 `finally` 你总是可以把资源关闭；你还得小心地处理资源的传递，传来传去，到处引用的资源，何时关闭可不是一件容易说清楚的事情。那么，既然你接受了「资源用完就立即关闭」的设定，你便不能在 `finalize` 里做关闭文件的事情，因为这将会导致资源的重复释放。这就是为什么 Java 无法做 RAII，或者 Python，golang 这些语言无法做 RAII 的原因。这也是为什么 Pythong 发展出了 `with`，golang 发展处 `defer` 来确保同一个 scope 下创建的资源，可以自动（正常）释放。不过，这些都是于小处的补丁，他们并不能完全解决 RAII 要解决的问题。

在 Rust 里，如果你在堆上创建一个文件资源，根据所有权模型，堆上的内存必然会有一个栈上的 owner（所有者），不管这个 owner 是单一 owner（如 Box），还是引用计数的 owner（如 Arc），也不管他们如何移动（move），被各种引用，Rust 只需关心 owner 离开 scope（对于 Arc 来说，最后一个 Arc 的 owner 离开 scope），此时 `Drop` 会被调用（仿佛此处有只麦克斯韦妖），从而释放堆上的内存和资源，简单直观。而所有这一切的决策，都在编译时完成，运行时只不过是按部就班执行编译出来的代码而已。

## 贤者时刻

和所有权模型一样，RAII 也是一种大道至简的处理问题的思路。相对于「使用完资源后，在所有可能的地方都妥善释放之以避免资源泄露」这种「头疼医头脚疼医脚」的方法，当我们重新考虑资源的生命周期，为其添加约束，限制住资源和对应的内存对象相同的生命期之后，一切变得简单而可控。

## 参考资料

[1] RAII: [https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization](https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization)

[2] Socket leak in Java app: [http://www.javamonamour.org/2019/09/sockets-leak-case-study.html](http://www.javamonamour.org/2019/09/sockets-leak-case-study.html)

[3] Socket leak in golang: [https://www.reddit.com/r/golang/comments/b3adpq/nethttp_transport_leaking_established_connections/](https://www.reddit.com/r/golang/comments/b3adpq/nethttp_transport_leaking_established_connections/)
