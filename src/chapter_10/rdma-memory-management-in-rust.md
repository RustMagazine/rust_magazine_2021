# Datenlord |在 Rust 中管理 RDMA 内存

作者： 施继成

-------

[RDMA](https://en.wikipedia.org/wiki/Remote_direct_memory_access) 是近年越来越热门的高速网络传输协议，被广泛应用于超算中心和高端存储领域。RDMA 的全称为 Remote Direct Memory Access，即允许本地内存被远端机器直接访问，该访问不经过被访问机器的操作系统，直接由网卡代为完成。正式因为网卡完成了大部分的数据传输工作，操作系统的负载被降低，使得其在大量数据传输的情况下具有更好的拓展性（scalability）。

为了保证远端能够正确和安全地访问本地内存，RDMA 协议中有一系列规范来约束用户的访问，下面来简单介绍一下。

## RDMA的内存管理

远端想要访问本地内存，首先需要本地的“同意”，即本地仅仅暴露想要暴露的内存，其他内存远端则不可访问。该“同意”操作，我们一般称为 Memory Region （简称 MR） 注册，操作系统在收到该请求时会锁住该段申请内存，防止内存被 swap 到硬盘上，同时将这个注册信息告知 RDMA 专用网卡，RDMA 网卡会保存虚拟地址到物理地址的映射。经过此番操作，由 MR 代表的内存暴露出来了，远端可以对其进行访问。处于安全的考虑，只有被允许的远端才可以访问，这些远端持有远端访问密钥，即Remote Key（简称 RKey），只有带有正确 RKey 的请求才能够访问成功。为了内存管理的细粒度化，RDMA 还提供了 Memory Window（简称 MW），一个 MR 上可以分列出多块 MW，并且每一块 MW 上都可以自定义访问权限。

除了上述中的 MR 和 MW，RDMA 中的内存管理还和 Protect Domain（简称 PD） 和 Queue Pair （简称 QP） 相关，这里不详细阐述这两个概念。下图详细介绍了，这些概念之间的依赖关系：

![rdma dependecy](./image/rdma/rdma-dependency.jpg)

现有的 RDMA 开发接口，即 InfiniBand Verbs 接口（简称 IBV 接口）并没有显式地展现这种依赖关系，但在实际使用中，任何不按规定顺序的资源释放都会造成错误，而用户找到问题的根本原因则非常困难。更进一步，当 MR 或者 MW 中的任何内存段被使用时，对应的 MR 和 MW 都不应该被释放或注销，这些在原有的 IBV 接口中也很难规范化。Rust 作为一门内存安全的语言，在处理类似问题上具有天然优势，接下来我们来分析如何用 Rust 解决上述问题。

## 利用 Rust 特性管理 RDMA 内存

### Allocator API
在 Rust 的 nightly feature 中有一个叫 Allocator API，这个 feature 允许用户创建自己的 Allocator，之后创建堆上数据时可以制定使用用户定制的 Allocator。大家很自然能想到，MR 或者 MW 很适合作为一种 Allocator，后续用户创建的 Vector 或者 Box 都可以使用其中的内存，这些内存可以直接开放给远端访问，既方便又高效。

但 Allocator API 有个核心问题无法保证，即 Allocator 本身应该比所有从其分配的数据活得更久，不然就会产生数据访问不安全的问题，如 use after free 问题。下列例子很好得阐述这一问题：

```rust
fn alloc_vec<'a> () -> Vec<u32, &MemoryRegion> {
    // Allocat missing
    let alloc = CusAllocator::New();
    let mut v = Vec::new_in(alloc);
    v.push(1);
    v
}

fn main() {
    let v = alloc_vec();
    println!("{:?}", v);
}
```

在 alloc_vec 方法中我们创建了一个新的 Allocator 叫 CusAllocator， 在方法结束时，该 Allocator 已经被释放，使用其内存的 vector 仍然存活着，被后续使用。Rust 语言无法判断出潜在的风险，唯一能够解决该问题的办法就是将 CusAllocator 变成 static 变量，这样其生命周期和整个程序一样长，也就不存在 use after free 的问题。然而该解决方法不适用 MR 和 MW 的场景，原因是 MR 和 MW 会随着用户的使用动态注册和注销，无法被注销的 MR 和 MW 会影响使用的便利性。若初始化太大的内存块，系统的内存压力太大，其他程序容易触发 OOM 问题；若初始化内存块太小，用户的使用会受到限制。结合上述考虑，Allocator 不是一个可行的方案。

### Reference 还是 Reference Count
Rust 语言中 Reference 带有生命周期属性，非常适合用来管理依赖关系，即被依赖 Ref 的生命周期不短于依赖者的生命周期，但是其在处理自引用时非常困难，当结构复杂到一定成都仅仅依赖 Reference 很难设计出用户易于使用的接口。因此我们采用了下列的设计方式：
```Rust
struct MemoryRegion {
    pd: Arc<ProtectDomain>,
    ...
}
```

这样核心数据接口都放到了堆上管理，同时保证了被依赖的数据结构一定不会提前释放 —— RC 特性的保证。解决了核心数据结构的管理，内存使用的管理则更加简单，下列方法保证了当有内存被使用时，MR 或者 MW 一定不会被释放。

```Rust
impl MemoryRegion {
    pub fn get_ref(&self, offset: usize, len: usize) -> &[u8] {
        // Just a demo, missing length check
        &self.buf[offset..(offset + len)]
    }
}
```

在此基础上，配合一些序列化方法，Memory Region 则可以处理各种数据结构的传输。

### 远端访问
RDMA 是为了远端数据访问而存在的，仅仅管理好本地内存还不够，如何保证远端访问时本地内存的可靠性也很重要，不过 Rust 语言本身的特性只能够维护本地内存的安全性，远程访问需要更上层的设计来完成。我们在我们的设计中提供了类似的接口来完成相应的任务：

```rust
impl MemoryRegion {
    async fn reserve (&self, timeout: Duration, f: T) where T: Future {
        timeout(timeout, f).await
    }
}
```

`timeout`表示该 MemoryRegion 外部能访问的最长时间段，如果 `f` Future 提前结束，则我们可以提前回收 MemoryRegion，否则至少等待 `timeout` 的时间长度才能回收。其中 `f` 可以在以下场景进行不同的操作：

1. 在一对一传输的场景中，`f` 将传输的必要信息传递给对方，等待对方完成的回复，一旦收到回复则结束 future。
2. 在一对多传输的场景中，`f` 将传输的必要信息放到某个看板上，然后等待 timeout 时间的结束。

这里之所以要在接口中固定一个timeout，是为了防止内存被无限期得占用不能够释放，最终造成内存泄露。例如上述的第一个场景，对方如果由于软硬件的问题程序结束前并没有给出回复，则 timeout 至少可以保证 MemoryRegion 在 timeout 时间段之后还有释放的机会。

值得注意的是，这里的机制并不能完全避免远端访问错误的发生，本次程序无法控制远端程序的弱点仍然存在，因此，RDMA 自带的保护机制也能够避免错误数据访问的发生，相应请求的失败会带来一些性能损失，这是无法避免的 tradeoff。

## 总结

Rust 语言的内存安全性部分地解决 RDMA 内存管理问题，同时上层的使用接口设计也部分解决了 RDMA 远端访问的管理问题。欢迎在我们的 [Rust RDMA 封装项目](https://github.com/datenlord/async-rdma) 交流讨论，促进项目发展，使得 Rust 社区能够更方便地使用 RDMA 网络。
