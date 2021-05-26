---
pub_date: Sat, 27 Mar 2021 16:00:00 GMT
description: DatenLord | Implement RDMA in Rust

---

# DatenLord | 用 Rust实现 RDMA

作者：王璞 / 后期编辑：张汉东

---

RDMA是常用于高性能计算(HPC)领域的高速网络，在存储网络等专用场景也有广泛的用途。RDMA最大的特点是通过软硬件配合，在网络传输数据的时候，完全不需要CPU/内核参与，从而实现高性能的传输网络。最早RDMA要求使用InfiniBand (IB)网络，采用专门的IB网卡和IB交换机。现在RDMA也可以采用以太网交换机，但是还需要专用的IB网卡。虽然也有基于以太网卡用软件实现RDMA的方案，但是这种方案没有性能优势。

RDMA在实际使用的时候，需要采用特定的接口来编程，而且由于RDMA在传输数据的过程中，CPU/内核不参与，因此很多底层的工作需要在RDMA编程的时候自行实现。比如RDMA传输时涉及的各种内存管理工作，都要开发者调用RDMA的接口来完成，甚至自行实现，而不像在socket编程的时候，有内核帮忙做各种缓存等等。也正是由于RDMA编程的复杂度很高，再加上先前RDMA硬件价格高昂，使得RDMA不像TCP/IP得到广泛使用。

本文主要介绍我们用Rust对RDMA的C接口封装时碰到的各种问题，并探讨下如何用Rust对RDMA实现safe封装。下面首先简单介绍RDMA的基本编程方式，然后介绍下采用Rust对RDMA的C接口封装时碰到的各种技术问题，最后介绍下后续工作。我们用Rust实现的RDMA封装已经开源，包括[rdma-sys](https://github.com/datenlord/rdma-sys)和[async-rdma](https://github.com/datenlord/async-rdma)，前者是对RDMA接口的unsafe封装，后者是safe封装（尚未完成）。

## RDMA编程理念

先首先简要介绍下RDMA编程，因为本文重点不是如何用RDMA编程，所以主要介绍下RDMA的编程理念。RDMA的全称是Remote Direct Memory Access，从字面意思可以看出，RDMA要实现直接访问远程内存，RDMA的很多操作就是关于如何在本地节点和远程节点之间实现内存访问。

RDMA的数据操作分为“单边”和“双边”，双边为send/receive，单边是read/write，本质都是在本地和远程节点之间共享内存。对于双边来说，需要双方节点的CPU共同参与，而单边则仅仅需要一方CPU参与即可，对于另一方的CPU是完全透明的，不会触发中断。根据上述解释，大家可以看出“单边”传输才是被用来传输大量数据的主要方法。但是“单边”传输也面临这下列挑战：

1. 由于RDMA在数据传输过程中不需要内核参与，所以内核也无法帮助RDMA缓存数据，因此RDMA要求在写入数据的时候，数据的大小不能超过接收方准备好的共享内存大小，否则出错。所以发送方和接收方在写数据前必须约定好每次写数据的大小。

2. 此外，由于RDMA在数据传输过程中不需要内核参与，因此有可能内核会把本地节点要通过RDMA共享给远程节点的内存给交换出去，所以RDMA必须要跟内核申请把共享的内存空间常驻内存，这样保证远程节点通过RDMA安全访问本地节点的共享内存。

3. 再者，虽然RDMA需要把本地节点跟远程节点共享的内存空间注册到内核，以防内核把共享内存空间交换出去，但是内核并不保证该共享内存的访问安全。即本地节点的程序在更新共享内存数据时，有可能远程节点正在访问该共享内存，导致远程节点读到不一致的数据；反之亦然，远程节点在写入共享内存时，有可能本地节点的程序也正在读写该共享内存，导致数据冲突或不一致。使用RDMA编程的开发者必须自行保证共享内存的数据一致性，这也是RDMA编程最复杂的关键点。

总之，RDMA在数据传输过程中绕开了内核，极大提升性能的同时，也带来很多复杂度，特别是关于内存管理的问题，都需要开发者自行解决。

## RDMA的unsafe封装

RDMA的编程接口主要是C实现的[rdma-core](https://github.com/linux-rdma/rdma-core)，最开始我们觉得用Rust的[bingen](https://github.com/rust-lang/rust-bindgen)可以很容易生成对rdma-core的Rust封装，但实际中却碰到了很多问题。

首先，rdma-core有大量的接口函数是inline方式定义，至少上百个inline函数接口，bindgen在生成Rust封装时直接忽略所有的inline函数，导致我们必须手动实现。Rust社区有另外几个开源项目也实现了对rdma-core的Rust封装，但是都没有很好解决inline函数的问题。此外，我们在自行实现rdma-core的inline函数Rust封装时，保持了原有的函数名和参数名不变。

其次，rdma-core有不少宏定义，bindgen在生成Rust封装时也直接忽略所有的宏定义，于是我们也必须手动实现一些关键的宏定义，特别是要手动实现rdma-core里用宏定义实现的接口函数和一些关键常量。

再有，rdma-core有很多数据结构的定义用到了union，但是bindgen对C的union处理得不好，并不是直接转换成Rust里的union。更严重的是rdma-core的数据结构里还用到匿名union，如下所示：
```C
struct ibv_wc {
    ...
	union {
		__be32		imm_data;
		uint32_t	invalidated_rkey;
	};
    ...
};
```
由于Rust不支持匿名union，针对这些rdma-core的匿名union，bindgen在生成的Rust binding里会自动生成union类型的名字，但是bindgen自动生成的名字对开发者很不友好，诸如`ibv_flow_spec__bindgen_ty_1__bindgen_ty_1`这种名字，所以我们都是手动重新定义匿名union，如下所示：
```Rust
#[repr(C)]
pub union imm_data_invalidated_rkey_union_t {
    pub imm_data: __be32,
    pub invalidated_rkey: u32,
}

#[repr(C)]
pub struct ibv_wc {
    ...
    pub imm_data_invalidated_rkey_union: imm_data_invalidated_rkey_union_t,
    ...
}
```

再次，rdma-core里引用了很多C的数据结构，诸如`pthread_mutex_t`和`sockaddr_in`之类，这些数据结构应该使用[Rust libc](https://github.com/rust-lang/libc)里定义好的，而不是由bindgen再重新定义一遍。所以我们需要配置bindgen不重复生成libc里已经定义好的数据结构的Rust binding。

简单一句话总结下，bindgen对生成rdma-core的unsafe封装只能起到一半作用，剩下很多工作还需要手动完成，非常细碎。不过好处是，RDMA接口已经稳定，此类工作只需要一次操作即可，后续几乎不会需要大量更新。

## RDMA的safe封装

关于RDMA的safe封装，有两个层面的问题需要考虑：
* 如何做到符合Rust的规范和惯例；
* 如何实现RDMA操作的内存安全。

首先，关于RDMA的各种数据结构类型，怎样才能封装成对Rust友好的类型。rdma-core里充斥着大量的指针，绝大多数指针被bindgen定义为`*mut`类型，少部分定义为`*const`类型。在Rust里，这些裸指针类型不是`Sync`也不是`Send`，因此不能多线程访问。如果把这些裸指针转化为引用，又涉及到生命周期问题，而这些指针指向的数据结构都是rdma-core生成的，大都需要显式的释放，比如`struct ibv_wq`这个数据结构由`ibv_create_wq()`函数创建，并由`ibv_destroy_wq()`函数释放：
```C
struct ibv_wq *ibv_create_wq(...);

int ibv_destroy_wq(struct ibv_wq *wq);
```
但是用Rust开发RDMA应用的时候，Rust代码并不直接管理`struct ibv_wq`这个数据结构的生命周期。进一步，在Rust代码中并不会直接修改rdma-core创建的各种数据结构，Rust代码都是通过调用rdma-core的接口函数来操作各种RDMA的数据结构/指针。所以对Rust代码来说，rdma-core生成的各种数据结构的指针，本质是一个句柄/handler，这个handler的类型是不是裸指针类型并不重要。于是，为了在Rust代码中便于多线程访问，我们把rdma-core返回的裸指针类型都转换成`usize`类型，当需要调用rdma-core的接口函数时，再从usize转换成相应的裸指针类型。这么做听上去很hack，但背后的原因还是很显而易见的。进一步，对于在rdma-core中需要手动释放的资源，可以通过实现Rust的`Drop trait`，在`drop()`函数中调用rdma-core相应的接口实现资源自动释放。

其次，关于RDMA的内存安全问题，这部分工作尚未完成。目前RDMA的共享内存访问安全问题在学术界也是个热门研究课题，并没有完美的解决方案。本质上讲，RDMA的共享内存访问安全问题是由于为了实现高性能网络传输、绕过内核做内存共享带来的，内核在内存管理方面做了大量的工作，RDMA的数据传输绕过内核，因此RDMA无法利用内核的内存管理机制保证内存安全。如果要把内核在内存管理方面的工作都搬到用户态来实现RDMA共享内存访问安全，这么做的话一方面复杂度太高，另一方面也不一定有很好的性能。

在实际使用中，人们会对RDMA的使用方式进行规约，比如不允许远程节点写本地节点的共享内存，只允许远程节点读。但即便是只允许远程读取，也有可能有数据不一致的问题。比如远程节点读取了共享内存的前半段数据，本地节点开始更新共享内存。假定本地节点更新的数据很少而远程节点读取的数据很多，因此本地节点更新的速度比远程节点读取的速度快，导致有可能本地节点在远程节点读后半段数据前更新完毕，这样远程节点读取的是不一致的数据，前半段数据不包括更新数据但是后半段包括更新数据。远程节点读到的这个不一致的数据，既不是先前真实存在的某个版本的数据，也不是全新版本的数据，破坏了数据一致性的保证。

针对RDMA内存安全问题，一个常见的解决方案是采用无锁(Lock-free)数据结构。无锁数据结构本质上就是解决并发访问下保证内存安全问题，当多个线程并发修改时，无锁数据结构保证结果的一致性。针对上面提到的远程读、本地写的方式，可以采用[Seqlock](https://en.wikipedia.org/wiki/Seqlock)来实现。即每块RDMA的共享内存空间关联一个序列号(sequence number)，本地节点每次修改共享内存前就把序列号加一，远程节点在读取开始和结束后检查序列号是否有变化，没有变化说明读取过程中共享内存没有被修改，序列号有变化说明读取过程中共享内存被修改，读到了有可能不一致的数据，则远程节点重新读取共享内存。

如果要放宽对RDMA的使用规约，即远程节点和本地节点都可以读写共享内存的场景，那么就需要采用更加复杂的算法或无锁数据结构，诸如[Copy-on-Write](https://en.wikipedia.org/wiki/Copy-on-write)和[Read-Copy-Update](https://en.wikipedia.org/wiki/Read-copy-update)等。内核中大量使用Copy-on-Write和Read-Copy-Update这两种技术来实现高效内存管理。这方面的工作有不少技术难度。

## 后续工作

下一步在完成对RDMA的safe封装之后，我们规划用Rust实现对RDMA接口函数的异步调用。因为RDMA都是IO操作，非常适合异步方式来实现。

对RDMA接口函数的异步处理，最主要的工作是关于RDMA的完成队列的消息处理。RDMA采用了多个工作队列，包括接收队列(RQ)，发送队列(SQ)以及完成队列(CQ)，这些队列一般是RDMA的硬件来实现。其中发送队列和接收队列的功能很好理解，如字面意思，分别是存放待发送和待接收的消息，消息是指向内存中的一块区域，在发送时该内存区域包含要发送的数据，在接收时该内存区域用于存放接收数据。在发送和接收完成后，RDMA会在完成队列里放入完成消息，用于指示相应的发送消息或接收消息是否成功。用户态RDMA程序可以定期不定期查询完成队列里的完成消息，也可以通过中断的方式在CPU收到中断后由内核通知应用程序处理。

异步IO本质上都是利用Linux的epoll机制，由内核来通知用户态程序某个IO已经就绪。对RDMA操作的异步处理，方法也一样。RDMA是通过创建设备文件来实现用户态RDMA程序跟内核里的RDMA模块交互。在安装RDMA设备和驱动后，RDMA会创建一个或多个字符设备文件，`/dev/infiniband/uverbsN`，N从0开始，有几个RDMA设备就有几个`uverbsN`设备文件。如果只有一个那就是`/dev/infiniband/uverbs0`。用户态RDMA程序要实现针对RDMA完成队列的异步消息处理，就是采用Linux提供的epoll机制，对RDMA的`uverbsN`设备文件进行异步查询，在完成队列有新消息时通知用户态RDMA程序来处理消息。

关于RDMA的封装，这块工作我们还没有完成，我们打算把RDMA的safe封装以及对RDMA的共享内存管理都实现，这样才能方便地使用Rust进行RDMA编程，同时我们欢迎有感兴趣的朋友一起参与。
