# 【我读】Rust 语言应该使用什么内存模型？

> 原文：[https://paulmck.livejournal.com/66175.html](https://paulmck.livejournal.com/66175.html) 

## 引子

[《深入理解并行编程》](https://mirrors.edge.kernel.org/pub/linux/kernel/people/paulmck/perfbook/perfbook.html) 作者 Paul E. McKenney 最近几天写了这篇文章。可以通过知乎这篇文章[《深入理解并行编程》笔记](https://zhuanlan.zhihu.com/p/56873613)来了解下这本书。

该书 [PDF 版本下载](https://mirrors.edge.kernel.org/pub/linux/kernel/people/paulmck/perfbook/perfbook-e2.pdf)，放心，这本书是公开免费，下载链接是作者公开的。

Paul 日常应该工作于 Facebook，拥有 30 年的 Cpp 并发工作经验，是 [Linux 内核内存模型（LKMM）](https://github.com/torvalds/linux/tree/master/tools/memory-model)的主要维护者。他写这篇文章的初衷是想为 Rust 社区的并发相关日常工作提供一个良好的起点。Paul 非常关注 Rust 进入 Linux 内核这件事，他在之前的博客文章中提出一个见解：在 Linux 内核中减少 Unsafe Rust 代码的一种方法就是将 Unsafe 操作下推到原子操作、内存屏障和锁原语中。

他的博客里也写了很多 Rust 和 Linux 的文章： [https://paulmck.livejournal.com/tag/rust](https://paulmck.livejournal.com/tag/rust)

本篇是我学习这篇文章过程中的阅读记录，内容不仅仅是这篇文章，同时也参考了文章中所提及的一些资料。

##  Rust 当前的内存模型

在 Rust 标准库  `std::sync::atomic`模块写道：“These orderings are the same as the [C++20 atomic orderings](https://en.cppreference.com/w/cpp/atomic/memory_order).” 。意思就是说， Rust 现在的内存顺序采用的是 C++ 20 的原子内存顺序。

在 Rust nomicon 一书中的[ `Atomics` 章节](https://doc.rust-lang.org/stable/nomicon/atomics.html)， 谈到，实际上这个模型相当复杂，并且已知有[几个缺陷](http://plv.mpi-sws.org/c11comp/popl15.pdf)。

> Rust 很明显地只是从 C++20 继承了原子的内存模型。这并不是因为该模型特别出色或易于理解。事实上，这个模型非常复杂，并且已知有几个缺陷。相反，这是对每个人都不擅长原子建模这一事实的务实让步。至少，我们可以从围绕 C/C++ 内存模型的现有工具和研究中受益。（你会经常看到这个模型被称为“C/C++11”或“C11”。C 只是复制了 C++ 内存模型；C++11 是该模型的第一个版本，但它已经收到了一些从那以后的错误修正。）
>
> 试图完全解释本书中的模型是相当无望的。它是根据引发疯狂的因果关系图来定义的，需要一整本书才能以实用的方式正确理解。如果你想要所有的细节，你应该查看[C++ 规范](https://en.cppreference.com/w/cpp/atomic/memory_order)。 
>
> C++ 内存模型从根本上是试图弥合我们想要的语义、编译器想要的优化以及我们的硬件想要的不一致的混乱之间的差距。我们只想编写程序，让它们完全按照我们说的去做。

## Linux 内核内存模型（LKMM）

 LKMM 中最容易令人生畏的地方包括：

1. [控制依赖](https://paulmck.livejournal.com/63151.html)
2. [地址和数据相关性](https://paulmck.livejournal.com/63316.html)

### 控制依赖

在许多弱序架构的汇编语言层面上，条件性分支充当了一个非常弱、非常便宜但非常有用的内存屏障指令。它命令任何返回值进入条件代码的`load`，在分支指令完成后执行的所有`store`之前，无论分支是否被采纳。`ARMv8`也有一个条件移动指令（`CSEL`），提供类似的排序。

因为条件分支的排序属性涉及从加载（`Load`）到分支以及从分支到存储（`Store`）的依赖关系，并且因为分支是控制流指令，所以这种排序被称为控制依赖。

因为编译器不理解它们，所以控制依赖非常脆弱。但是它们的成本非常低，因此它们被用于 Linux 内核中一些非常重要的快速路径。

Rust 可以通过多种方式处理控制依赖：

1. 简单的解决方案是将控制依赖项的`load`提升到`smp_load_acquire()`。这有效，但在某些架构上增加了指令开销，并不必要地限制了所有架构上的编译器优化（但公平地说，`ARMv8 `在使用链接时优化构建时正是这样做的）。另一个困难是确定（无论是手动还是自动）确切地需要提升哪些`READ_ONCE()`调用。
2. 一个更简单的解决方案是将包含控制依赖项的代码分类为 Rust 范围之外的核心 Linux 内核代码。由于在 Linux 内核中很少使用控制依赖项，因此 Rust 采取这种方法不会损失太多。此外，还有可能创建更高级别的 C 语言原语，其中包含所需的控制依赖项，然后将其包装起来以供 Rust 语言使用。
3. 从`Linux-kernel-in-Rust` 开发人员的角度来看，最好的方法是让 Rust 强制执行`memory-barriers.txt 中`记录的代码样式限制。然而，这种方法有可能被证明是不平凡(non-trivial)的。
4. 等待编译器后端了解控制依赖项。这可能需要等待一段时间，尤其是考虑到在` C/C++` 标准的当前命名法中甚至难以定义控制依赖项。

###  地址和数据相关性

地址依赖涉及一个加载，它的返回值直接或间接决定了后面加载或存储的地址，这导致较早的加载在后面的加载或存储之前被排序。数据依赖涉及一个加载，它的返回值直接或间接决定了后面的存储存储的值，这导致加载在存储之前被排序。这些被` RCU` (`Read Copy Update`，读复制更新)大量使用。尽管它们不像控制依赖那样脆弱，但编译器仍然不知道它们。因此，仍然需要小心，如[`rcu_dereference.rst`](https://www.kernel.org/doc/Documentation/RCU/rcu_dereference.rst) Linux 内核编码指南中所见。与控制依赖一样，地址和数据依赖的开销非常低，但与控制依赖不同，它们在 Linux 内核中被大量使用。

Rust 内存模型应该将其对 Linux 内核原子操作的支持限制为提供排序的那些。这些将是返回值的非宽松（non-relaxed）读-修改-写 (RMW) 原子操作以及非返回值的 RMW 原子操作的`_acquire()`和`_release()`变体。允许无序 RMW 操作与组合内存屏障的组合也可能有意义，例如，`atomic_inc()`后跟`smp_mb__after_atomic()`，但将它们组合包装为单个 Rust 可访问原语会更有意义。这个组合的 Rust 原语将不再是无序的，因此可以作为一个有序的单元包含在 Rust 内存模型中。或者，无序原子操作（relax）可能会降级为 Rust 的`unsafe`模式。

因此，从 LKMM 开始，我们得到了一个支持有序原子操作和锁定的模型，可能包括`unsafe`模式下的无序原子操作。

## C++ 内存模型

Cpp 的 `memory_order_relaxed` 会导致 `out-of-thin-air (OOTA) ` 的值出现。所以，Rust 中 `memory_order_relaxed` 建议只允许在 `unsafe` 代码中使用。

而安全的 Rust 代码应该允许使用这四个顺序：`memory_order_acquire`、``memory_order_release` 和`memory_order_acq_rel` 。

## 建议

从 Linux 内核内存模型 和 Cpp 内存模型的一些问题，作者对 Rust 中的内存顺序改进提出以下建议：

1. 可以在 `Safe` 和 `UnSafe` 下使用 锁原语。
2. `memory_order_seq_cst`、`memory_order_acquire`、``memory_order_release` 和`memory_order_acq_rel` 可以在 `Safe` 和 `UnSafe` 下使用。
3. `memory_order_relaxed`和`memory_order_consume` 只可在 `UnSafe`  下使用。

在安全模式下采用`C/C++`内存模型中没有问题的部分，其余部分采用`UnSafe`模式。这种方法允许人们用 Rust 编写每天的并发算法，并确信所产生的代码在未来仍然可以工作。

就看 Rust 社区如何选择了。

## 来自 Rust 社区的声音

有另一位读者对他给 Rust 社区的建议做了如下回复（以下摘要）:

>  `Relax `读取的 OOTA 行为本身不会违反 Rust 的任何内存安全保证 —— 一般而言，`Relax` 内存操作仍不会引起未定义的行为。因此，没有真正的理由从 Rust 安全代码中排除更多“异乎寻常”的操作。
>
>  `Relax` 的操作也有实际的编译器支持，并且在许多对性能非常重要的情况下很有用，尤其是在弱排序的硬件上，因此出于纯粹的教学原因将它们全部升级为更昂贵的操作似乎很愚蠢。
>
> 所以这就是 Rust 现在的位置：它基于 `C++` 内存模型不是因为人们认为没有更好的东西（尤其是在 `Relaxed` 周围），也不是因为人们不接受显式内存或控制依赖，而是因为那些是编译器目前提供的实际“工作”并且可以证明事情的原语。
>
> 至于“Rust 现在真的需要它的内存模型吗”：我认为是的，它确实需要。但它不需要成为最终的内存模型。

仁者见仁，智者见智吧。

## 参考

- [ LINUX 内核内存屏障](https://www.kernel.org/doc/Documentation/memory-barriers.txt)
- [ LINUX 内核内存屏障 【中文版】](https://maple-leaf-0219.github.io/2020/linux%E5%86%85%E6%A0%B8%E4%B8%AD%E7%9A%84%E5%86%85%E5%AD%98%E5%B1%8F%E9%9A%9C-%E8%AF%91/)

