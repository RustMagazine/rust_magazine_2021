---
pub_date: Sat, 27 Mar 2021 16:00:00 GMT
description: Exploring the origin of the system through Rust Concurrency

---

## 透过 Rust 探索系统的本原：并发篇

作者：陈天 / 后期编辑：[NiZerin](https://github.com/NiZerin)

> 原文链接：[https://mp.weixin.qq.com/s/9g0wVT-5PpmXRoKJZo-skA](https://mp.weixin.qq.com/s/9g0wVT-5PpmXRoKJZo-skA)

---

rust 是一门非常优秀的语言，我虽然没有特别正式介绍过 rust 本身，但其实已经写了好多篇跟 rust 相关的文章：

- [沅有芷兮：类型系统的数学之美](http://mp.weixin.qq.com/s?__biz=MzA3NDM0ODQwMw==&mid=2649828208&idx=1&sn=1599b7cbc3bcc2c050c2689b9e46acbd&chksm=8704a96cb073207a890c0056332ede6ac01eda5f5bcc02ec016b466b0fb6b6c91c3637fee1c1&scene=21#wechat_redirect)
- [Noise 协议的应用](http://mp.weixin.qq.com/s?__biz=MzA3NDM0ODQwMw==&mid=2649828386&idx=1&sn=555e16a024e027f6cde350a0a09d3af4&chksm=8704ae3eb0732728040c309dbd4200c93483f6f40b1ac86dc7d8810f53290d870657b89ea5ce&scene=21#wechat_redirect)
- [用 noise 协议的思路来点对点加密文件？](http://mp.weixin.qq.com/s?__biz=MzA3NDM0ODQwMw==&mid=2649828413&idx=1&sn=2eadc24e7fe82581c70f78c186d0e678&chksm=8704ae21b0732737f456bc372e8317edd532edeb63ee60a3a92efd748e2c5a9b5a9ec6e3caf8&scene=21#wechat_redirect)
- [如何安全地保存密码？](http://mp.weixin.qq.com/s?__biz=MzA3NDM0ODQwMw==&mid=2649828397&idx=1&sn=37978455a88361c65663bc7e73fa63f3&chksm=8704ae31b0732727ea88c86b95b551307b2a2d55834d3ee2a9d731165bce96289fe4a12cec0e&scene=21#wechat_redirect)
- [从微秒到纳秒：关于性能的奇妙旅程](http://mp.weixin.qq.com/s?__biz=MzA3NDM0ODQwMw==&mid=2649828863&idx=1&sn=5ff0ccb8b286e9ba86e2c944f244ce6d&chksm=8704afe3b07326f50e903b975d655248b0136dff262a776291cf662d7c7a0f30889648570b9f&scene=21#wechat_redirect)

我打算写一个系列，讲讲如果透过 rust 来更好地探索系统的本原。我不知道我能写多少，也许就这一篇，也许很多篇，不管怎样，每篇都会介绍独立的概念。这个系列并不会介绍大量的 rust 代码，因此其内容对非 rust 程序员也有好处。

这一篇我们讲并发。几年前我曾经写过一篇介绍并发概念的文章：[concurrency](http://mp.weixin.qq.com/s?__biz=MzA3NDM0ODQwMw==&mid=401691172&idx=1&sn=b4b3a8fe51eb4250ff82e9153580f1dd&chksm=0d04d0383a73592ea5f61767d2c6ee7f1ee364153fc6a3ff4d4d8253a6a17afe71c693e941e8&scene=21#wechat_redirect)，大家感兴趣可以看看。这篇我们从更加务实的角度，以一个简单的字典服务器程序的迭代为引子，把并发中涉及的概念和解决方法串起来。

## v1：循环处理

我们的字典服务器监听 8888 端口，在服务器端维护一个 KV db（使用 hash map）。客户端可以插入（更新）一个 key 和相关的 value，也可以查询一个 key，获得对应的 value。嗯，就像 redis 服务器一样，只不过比 redis 简单十万八千倍。

这个需求很简单，我们马上可以想到：

1. 监听 8888 端口
2. 写一个死循环，不断 `accept` socket，然后对 socket 里收到的数据进行处理。

![](https://oss.iacblog.com/rust/rust-to-system-essence-concurrent/1.webp)

但这样是串行服务，我们只有处理完上一个 socket 的数据，才有机会处理下一个 socket，吞吐量非常有限。显然，我们需要改进。

## v2：多线程处理

接下来我们需要解决串行服务的瓶颈。一个方法是 `accept` 之后，将新的 socket 放入一个线程里执行，于是主线程不会被阻塞住，可以继续 `accept` 后续的 socket。这样，每个 client 过来的请求都可以独立地处理。

![](https://oss.iacblog.com/rust/rust-to-system-essence-concurrent/2.png)

可是，这带来了一个显而易见的问题：我们的 KV db 成为了一个共享状态，它在多个线程之间共享数据。这是并发处理的第一种范式：共享状态的并发（Shared-State Concurrency）。

既然引入了共享状态，那么我们需要在访问它的时候做妥善的保护 —— 这个访问和操作共享状态的代码区域叫临界区（Critical Section）。如果你还记得操作系统课程的内容，你会知道，最基本的操作是使用互斥量（Mutex）来保护临界区。

互斥量本质是一种二元锁。当线程获得锁之后，便拥有了对共享状态的独占访问；反之，如果无法获得锁，那么将会在访问锁的位置阻塞，直到能够获得锁。在完成对共享状态的访问后（临界区的出口），我们需要释放锁，这样，其它访问者才有机会退出阻塞状态。一旦忘记释放锁，或者使用多把锁的过程中造成了死锁，那么程序就无法响应或者崩溃。rust 的内存安全模型能够避免忘记释放锁，这让开发变得非常轻松，并且最大程度上解决了（不同函数间）死锁问题。

但任何语言的任何保护都无法避免逻辑上的死锁，比如下面这个显而易见的例子：

 ```rust
use std::sync::Mutex;

fn main() {
    let data = Mutex::new(0);
    let _d1 = data.lock();
    let _d2 = data.lock(); // deadlock now
}
 ```

互斥锁往往锁的粒度太大，在很多场景下效率太低。于是我们在此基础上分离了读写的操作，产生了读写锁（RwLock），它同一时刻允许任意数量的共享读者或者一个写者。读写锁的一个优化是顺序锁（SeqLock），它提高了读锁和写锁的独立性 —— 写锁不会被读锁阻塞，读锁也不会被写锁阻塞。，但写锁会被写锁阻塞。

读写锁适用于读者数量远大于写者，或者读多写少的场景。在我们这个场景下，读写的比例差别可能并不是特别明显，从 Mutex 换到 RwLock 的收益需要在生产环境中具体测试一下才能有结论。

## v3：锁的优化

但即使我们无法通过使用不同实现的锁来优化对共享状态访问的效率，我们还是有很多方法来优化锁。无论何种方法，其核心思想是：**尽可能减少锁的粒度**。比如，对数据库而言，我们可以对整个数据库管理系统加锁，也可以对单个数据库的访问加锁，还可以对数据表的访问加锁，甚至对数据表中的一行或者一列加锁。对于我们的 KV db 而言，我们可以创建 N 个 hashmap（模拟多个数据库），然后把 Key 分散到这 N 个 hashmap 中，这样，不管使用什么锁，其粒度都变成之前的 1/N 了。

![](https://oss.iacblog.com/rust/rust-to-system-essence-concurrent/3.webp)

新的 KV db 的定义，以及添加 / 访问数据的代码：

```rust
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};

struct KvDb(Arc<Vec<RwLock<HashMap<String, Vec<u8>>>>>);

impl KvDb {
    pub fn new(len: usize) -> Self {
        let mut dbs: Vec<RwLock<HashMap<String, Vec<u8>>>> = Vec::with_capacity(len);
        for _i in 0..len {
            dbs.push(RwLock::new(HashMap::new()))
        }
        Self(Arc::new(dbs))
    }

    pub fn insert(&self, k: &str, v: Vec<u8>) {
        let dbs = self.0.clone();
        let mut writer = dbs[(self.hash(k) % dbs.len()) as usize].write().unwrap();
        writer.insert(k.into(), v);
    }

    pub fn get(&self, k: &str) -> Vec<u8> {
        let dbs = self.0.clone();
        let reader = dbs[(self.hash(k) % dbs.len()) as usize].read().unwrap();
        reader.get(k).unwrap().to_owned()
    }

    fn hash(&self, k: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        k.to_owned().hash(&mut hasher);
        hasher.finish() as usize
    }
}
```

rust 里面的 dashmap 提供了一个类似思路的高并发访问的 hashmap。

## v4：share memory by communicating

前面的迭代不管怎么优化都跳脱不出同一种思路：Shared-state concurrency，或者说：communicate by share memory。这种方法限制很少，非常灵活，适用于任何并发场景，因而它是所有并发方案的基石。然而，灵活度带来的问题就是容易出错，需要额外的约定和限制来避免一些问题的产生。

那么，有没有办法把并发的需求抽象出来，设计一些更高级的数据结构和使用方法，把锁的使用隐藏起来？

当然有。

其中最有效最优雅的方法是消息传递（message passing）。我们把问题的两端分别定义成生产者和消费者。KvDb 的客户端是生产者，它们提交请求（update / get），而 KvDb 的服务器是消费者，它接受请求，返回处理的结果。连接两端的是一个消息通道（channel）。我们可以根据消息通道的两端的使用情况，将其进一步细分成几种访问模型：

- spsc：单生产者单消费者（Single producer single consumer）。这是最简单的访问模型，它可以用锁（RwLock）来实现并发，也可以通过一个 ring buffer 实现无锁（lock-free）并发。rust 的标准库没有 spsc 的实现，但第三方库，如 tokio，提供了 `oneshot` channel。当然我们也可以封装 `VecDeque` 来模拟 spsc。

- mpsc：多生产者单消费者（ Multiple producer single consumer）。这是最典型的并发使用模型，大部分的客户端/服务器实现都能用 mpsc 模型来处理。rust 标准库里有 `std::mpsc::channel` 来处理 mpsc 模型。

- spmc：单生产者多消费者（Single producer multiple consumer）。这可能是使用最少的消息模型。rust 标准库里没有对应的实现，也鲜有第三方库单独实现它。Jonhoo 做了一个 crate `bus` ，是 spmc broadcast channel，它是单个生产者对所有消费者的广播。

- mpmc：多生产者多消费者（ Multiple producer Multiple consumer）。mpmc 是最复杂的情况，可以用来实现之前的几种模式。但因为 spsc / mpsc 有很多使用场景，所以一般我们不会用 mpmc 来模拟。rust 标准库里没有 mpmc 的实现，但 `crossbeam` 实现了高效的 mpmc channel。

使用消息通道的思路，我们可以进一步迭代我们的 KvDb —— 在处理 socket 的线程和处理 state 的线程之间建立一个 mpsc channel：

![](https://oss.iacblog.com/rust/rust-to-system-essence-concurrent/4.png)

这种方式是否更高效？不见得。但从并发处理的角度来看，它结构上更清晰，不容易出错。

使用消息传递来处理并发的思路是如此重要，以至于两门非常有影响力的语言将其内置在语言的运行时里，成为语言的一部分：

golang 内建了 channel，使用 goroutine 和 channel 来处理并发。其语言的核心思想是：

> Do not communicate by sharing memory; instead, share memory by communicating.

而 erlang 内建了 actor model，让 `send`，`receive` 成为其最基本的六个函数之一。两个 actor（process）之间唯一的交流方式就是找到对方的 pid，然后发送消息。

## v5：协程（async/await or 异步处理）

我们在使用多线程做并发处理时，使用的是操作系统的调度能力。这样的好处是，我们无需自己再做一个调度器，进行复杂的调度处理；坏处是，操作系统处理线程的调度需要复杂的上下文切换，其中包括用户态和内核态的切换，所以它的效率不够高，尤其是如果我们需要大量的随用随抛的「线程」时。

然而，「现代」的应用程序因为复杂程度越来越高，所以其并发程度也越来越高，大量的操作都涉及随用随抛的「线程」。如果我们用操作系统线程来实现这些「线程」，会大大拖累系统的整体效率，甚至会触及操作系统的限制（`/proc/sys/kernel/threads-max`）。

因而，「现代」的编程语言都有协程的支持 —— 在 golang 里是 goroutine，在 erlang 里是 process，在 python 里是 coroutine，在 rust 里是 future。它们可以以一个更小的粒度在用户态进行并发处理，代价是用户态需要一个调度器。golang / erlang 在语言层面的运行时提供了这个调度器，而 rust 需要引入相关的库。这些语言的用户态调度器的实现都大同小异：

- 使用 N 个操作系统线程（一般来说 N= 硬件线程的数量）

- 每个线程上维护若干个队列，保存不同状态下的异步任务。当 ready 队列有任务时，执行该任务，直到其再度挂起或者执行完毕。所以每个异步任务本身要避免没有 IO 或系统调用的大量纯计算工作（computation intensive），如果有这样的工作，那么要主动 yield。

- 如果某个线程上没有待执行的任务，它可以去其它线程上「偷」任务（work stealing scheduler）。

- 如果某个线程上正在运行的任务被阻塞（比如执行 syscall），那么一般而言调度器会把队列里的其它任务交给没有阻塞的线程（golang），或者把阻塞操作交给其它专门的线程处理。

无论从 v3 还是 v4 版本，我们都很容易把一个多线程的实现变成多协程的实现。对于 rust 而言，就是引入 async / await：

- 把相应的函数变成 async 函数，这样函数的返回值会变成一个 `Future`。

- 在调用 async 函数的地方，添加 `.await` 来处理 async 的状态机。

- 在使用 `spawn` 的地方，使用 `tokio` 或者 `async_std` 对应的 `spawn`，来创建一个协程。

- 在入口函数，引入 `executor`，比如使用宏 `#[tokio::main]`。

对于我们的 kv server，因为协程处理的流程图和线程处理类似（内部机制大不一样），所以这里我就不附图了。

## One more thing：线程和协程间的同步

在一个复杂的系统里，线程和协程可能会同时出现。我们用线程做计算密集的事情，而用协程做 IO 密集的事情，这样系统可以达到最好的吞吐能力。遗憾的是，很多以协程为卖点的语言，如 erlang 和 golang，你所面临的环境是受控的（某种意义上说，这也是优势 - don't make me think），只能创建协程，而不能创建线程。所以无法做这样的优化。而另一些语言，如 Python，Scala，虽然同时支持线程和协程，两者混合使用要么效率不高，要么没有很好的库，用起来很别扭（我并没有 scala 经验，关于 akka 和 thread 混用的别扭只是道听途说）。

而 Rust 处理得很优雅 — `tokio::sync` 提供了在同步和异步线程之间使用 channel 同步的工具。你甚至感觉不到你的数据在不同的 runtime 间穿梭。其实站在操作系统的角度想想也能释然：管它是线程和协程，在操作系统层面都是线程，只不过协程是运行在某些线程上的受那些线程独立调度的数据结构而已。所以，线程和协程间的同步，归根结底，还是线程之间的同步问题。而线程间同步的手段，我们都可以使用，只不过在这种场景下，channel 是最好（最舒服）的选择。

所以，我们可以在系统启动时（或者服务器启动时），在普通的线程和 tokio 管理的线程（Runtime）间创建好一个 channel，然后在各自的上下文中处理流入流出 channel 的数据，如下图所示：

![](https://oss.iacblog.com/rust/rust-to-system-essence-concurrent/5.webp)

本文中我们提到的这个 KV store 的例子太简单，并不涉及同步线程和异步线程之间的同步，我举个其它例子。上篇文章《[从微秒到纳秒](http://mp.weixin.qq.com/s?__biz=MzA3NDM0ODQwMw==&mid=2649828863&idx=1&sn=5ff0ccb8b286e9ba86e2c944f244ce6d&chksm=8704afe3b07326f50e903b975d655248b0136dff262a776291cf662d7c7a0f30889648570b9f&scene=21#wechat_redirect)》讲了如何使用多线程来处理不同 repo 下的事件的写入。下图是之前文章里的主流程：

![](https://oss.iacblog.com/rust/rust-to-system-essence-concurrent/6.webp)

在这个流程的基础上，我们需要添加一个新的功能：当日志文件 rotate 时，我们发一个消息出去，由一组 uploader 线程负责把刚刚关闭封存的日志文件传输到 S3。

Rust 下和 S3 打交道的库是 Rusoto，Rusoto 是全异步的处理，因而我们需要一个 Tokio runtime 来处理异步的任务。我们可以在 `Server.start` 接口来处理 Runtime 的创建，然后创建 channel，把 rx 交给 Tokio runtime 下运行的一个死循环的异步任务，这个任务从 rx 里取数据，然后 spawn 新的异步任务将 file 上传到 S3 对应 bucket 的 key 下。而 channel 的 tx 端则传给每个 repo 的 `LoggerWriter`，这样，`LoggerWriter` 在做 rotation 的时候，就可以通过 tx 发送要上传给 S3 的本地文件名 file，以及上传到 S3 的对象的 key。如下图所示：

![](https://oss.iacblog.com/rust/rust-to-system-essence-concurrent/7.webp)

整个流程同样看上去不容易实现，但最终添加的也就是二十行代码而已（不计入 S3 具体上传的代码）。
