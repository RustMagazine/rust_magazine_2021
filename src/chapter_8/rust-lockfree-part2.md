# Datenlord | Rust 无锁编程之Crossbeam Epoch算法解析

作者：施继成

-----

上次的[文章](https://rustmagazine.github.io/rust_magazine_2021/chapter_6/rust-lockfree.html) 介绍了无锁数据结构的内存管理机制 EBR，该机制相较于其他的内存管理机制具有更高的执行效率。然而由于理念的复杂性，EBR 的实现并不容易，为每一个无锁数据结构从头实现 EBR 也务必要，因此很自然得大家会考虑将 EBR 的核心理念 epoch 抽取出来变成库，让大家能够复用。[Crossbeam-epoch](https://github.com/crossbeam-rs/crossbeam) 是一套成熟的被大家广泛使用的 EBR 库，本文将从实现原理部分进行较为详细的解析，并且在此过程中进行。

## Guard 只是最外层的壳

如[前文](https://rustmagazine.github.io/rust_magazine_2021/chapter_6/rust-lockfree.html)所述，大家一般在和Crossbeam-epoch 进行交互时仅仅使用 guard，如下所示：

```rust
/// delete data
{
    let guard = epoch::pin();
    guard.defer(move || mem.release());
}

/// get data for reading
{
    let guard = epoch::pin();
    let value = map.get(&key, &guard);
    /// ... Use the value
}
```

在读取数据的时候，guard扮演的角色仅仅是生命周期守护者，其确保获取出来的data引用（上述代码中的 value）生命周期一定不长于 guard，当 guard 被销毁时，value 也一定被销毁。删除数据过程中，guard扮演的角色要更复杂一些，其负责将销毁函数注册到 defer 延迟执行的队列中。至于该销毁函数何时被调用，则需要进一步深入了解其内部实现细节。

## Pin 到底做了什么

epoch::pin() 到底做了什么，官方的代码注释中给出了解释，即将当前的 thread pin 住，以便将堆上数据的指针放到栈上。该解释其实只解释了上述读取数据本分的内容，其底层执行的操作如下：

1. 将当前线程注册到 Global 收集器，该注册过程每个线程仅仅做一次。
2. 获取当前全局Epoch并设置到当前线程，表示在pin有效的这段时间，当前线程属于哪个Epoch。
3. 记录当前线程已经被pin的次数，当次数达到一定数量，尝试让 Global 收集器推进 Epoch 增长，回收垃圾。
4. 增加 guard_count 计数，记录有多少guard被创建出来且还没有被销毁。

由于 pin() 可以反复调用，所以连续两次调用 epoch::pin() 也被允许。只是除了第一次调用，其他的调用都不会有任何作用，仅仅增加了guard_count 计数。具体实现参见 `internal.rs` 文件中 `Local` struct 的 `pin` 方法。

这里提及的 Global 收集器负责所有资源的回收工作，其从各个线程收集需要回收的垃圾，并在适当的时机进行回收。

## Epoch 推进

Epoch Number 需要不停向前迭代，在迭代的过程中，垃圾回收器将隶属与老的 Epoch Number的可回收垃圾回收掉。每次 Global 收集器想要回收垃圾时都会尝试推进 Epoch Number，满足下列条件则 Epoch Number 向前推进成功：

1. 所有注册的线程都处于当前的Epoch，即不存在线程处于上一个Epoch。

如果条件不满足则 Epoch 推进失败。具体实现请参见 `internal.rs` 文件中 `Global` struct 的 `try_advance` 方法。

## 垃圾回收机制

如果所有的线程都将待回收的垃圾注册到 Global 收集器，那么会出现非常巨大的竞争关系，线程越多操作越频繁则性能影响越大。为了解决共享数据结构造成的竞争，每个线程都会维护自己的垃圾回收队列，队列长度为62（神奇的magic number，猜测和CPU cache line 相关）。当队列被装满时，线程会将本地的队列中的数据统一移动到到 Global 收集器，放到收集器的垃圾链表当中。这里值得注意的是，放入链表的除了垃圾回收函数，还有该垃圾产生的 Epoch Number，该数字被用于决定是否可以回收对应的垃圾。

垃圾回收的触发点有两个，一个主动一个被动。主动的触发点为 Guard 的 flush 方法，调用该方法则会使得 Global 收集器尝试收集垃圾链表中的垃圾。被动的触发点为 Guard 的 pin 方法，即 pin 每被调用 128 次会触发一次垃圾回收。

满足下列条件的垃圾可以被回收：

1. (Global Epoch Number) > ((Garbage Epoch Number) + 1)，即垃圾对应的 Epoch 至少比当前 Epoch 早两个世代。

具体实现请参见 `internal.rs` 文件中 `Global` struct 的 `collect` 方法。

## 内部数据结构

其内部数据结构值得一提有两个，一个List，一个Queue。List 被用于管理注册的线程，Queue 被用于管理等待被回收的垃圾。这两个数据结构的共同点是被多个线程同时操作，为了高效的实现，crossbeam没有使用Lock来保护数据结构，而是实现了内部的无锁数据结构。

### List
List 有新元素插入方法，插入的方法就是将新元素插入到 List 的 head 位置，实现中使用了 CAS 原子操作。在多线程同时进行插入时，同一时间只有一个能够成功，失败的操作会获得新的 header 值进行再次尝试。

List 删除操作并不真正移除元素，而是在该元素上进行标记，最后在某次 Iteration 过程中被真正删除，该删除操作也使用了 CAS 原子操作。如果有多个线程在尝试删除同一个元素，只有一个能够成功。如果在删除某个元素时发现其前一个元素也被标记为可被删除，则通知 Iteration 调用方需要重头再遍历一次。当然调用方可以根据情况重头遍历，还是留给其他人来处理。

具体实现请参见 `list.rs` 文件。

### Queue
Queue 的插入和 List 类似，区别在于插入点为 tail。Queue 的删除操作叫做 pop，从 Queue 的 head 开始弹出数据，如果弹出数据出错则说明有其他线程也在进行弹出操作，那么需要重新使用获取 head 的位置再次尝试。

那些从 List 和 Queue 中删除的元素如何处理呢？ crossbeam 采用了自举的方法，即也放入垃圾回收队列中，等待之后的某次操作触发垃圾回收。

## 总结

Crossbeam-epoch 给大家提供了一个极其方便的工具，将epoch的实现细节隐藏在库中，暴露给用户极其简单的接口，使得大家在实现无锁数据结构时更多关注数据结构的细节，将内存回收工作交给 Crossbeam-epoch 来处理即可。
