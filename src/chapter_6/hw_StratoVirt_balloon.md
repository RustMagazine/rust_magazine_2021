# 华为 | StratoVirt - 基于Rust的 balloon 功能实践

作者： 杨铭

---

StratoVirt是计算产业中面向云数据中心的企业级虚拟化平台，实现了一套架构统一支持虚拟机、容器、Serverless三种场景。StratoVirt在轻量低噪、软硬协同、Rust语言级安全等方面具备关键技术竞争优势。

### 背景介绍：

通常，在同一台服务器上存在着不同的用户，而多数用户对内存的使用情况是一种间断性的使用。也就是说用户对内存的使用率并不是很高。在服务器这种多用户的场景中，如果很多个用户对于内存的使用率都不高的话，那么会存在服务器实际占用的内存并不饱满这样一种情况。实际上各个用户使用内存的分布图可能如下图所示（黄色部分表示used部分，绿色部分表示free的部分）。

![1](./image/hw/memory.png)

### 解决方案：

为了解决上述服务器上内存使用率低的问题，可以将虚拟机中暂时不用的内存回收回来给其他虚拟机使用。而当被回收内存的虚拟机需要内存时，由host再将内存归还回去。有了这样的内存伸缩能力，服务器便可以有效提高内存的使用率。在StratoVirt中，我们使用balloon设备来对虚拟机中的空闲内存进行回收和释放。下面详细了解一下StratoVirt中的balloon设备。

### balloon设备简介：

由于StratoVirt只是负责为虚拟机分配内存，只能感知到每个虚拟机总的内存大小。但是在每个虚拟机中如何使用内存，内存剩余多少。StratoVirt是无法感知的，也就无法得知该从虚拟机中回收多少内存了。为此，需要在虚拟机中放置一个“气球（balloon）”设备。该设备通过virtio半虚拟化框架来实现前后端通信。当Host端需要回收虚拟机内部的空闲内存时，balloon设备“充气”膨胀，占用虚拟机内部内存。而将占用的内存交给Host使用。如果虚拟机的空闲内存被回收后，虚拟机内部由于业务要求突然需要内存时。位于虚拟机内部的balloon设备可以选择“放气”缩小。释放出更多的内存空间给虚拟机使用。

### balloon实现：

balloon的具体代码实现位于StratoVirt项目的/virtio/src/balloon.rs文件中，相关细节可阅读代码理解。代码架构如下：

```
virtio
├── Cargo.toml
└── src
    ├── balloon.rs
    ├── block.rs
    ├── console.rs
    ├── lib.rs
    ├── net.rs
    ├── queue.rs
    ├── rng.rs
    ├── vhost
    │   ├── kernel
    │   │   ├── mod.rs
    │   │   ├── net.rs
    │   │   └── vsock.rs
    │   └── mod.rs
    └── virtio_mmio.rs
```

由于balloon是一个virtio设备，所以在前后端通信时也使用了virtio框架提供的virtio queue。当前StratoVirt支持两个队列：inflate virtio queue（ivq）和deflate virtio queue（dvq）。这两个队列分别负责balloon设备的“充气”和“放气”。

气球的充放气时，前后端的信息是通过一个结构体来传递。

```rust
struct VirtioBalloonConfig {
    /// Number of pages host wants Guest to give up.
    pub num_pages: u32,
    /// Number of pages we've actually got in balloon.
    pub actual: u32,
}
```

因此后端向前端要内存的时候，只需要修改这个结构体中的num_pages的数值，然后通知前端。前端读取配置结构体中的num_pages成员。并与本身结构体中的actual对比，判断是进行inflate还是deflate。

- inflate

如果是inflate，那么虚拟机以4k页为单位去申请虚拟机内存，并将申请到的内存地址保存在队列中。然后通过ivq将保存了分配好的页面地址的数组分批发往后端处理（virtio queue队列长度最大256，也就是一次最多只能传输1M内存信息，对于大于1M的内存只能分批传输）。后端通过得到信息后，找到相应的MemoryRegion，将对应的page标记为”WILLNEED“。然后通知前端，完成配置。

- deflate

如果是deflate则从保存申请到的内存地址队列中弹出一部分内存的地址。通过dvq分批次传输给后端处理。后端将page标记为“DONTNEED"。

下面结合代码进行说明：

定义BalloonIoHandler结构体作为处理balloon事件的主体。

```rust
struct BalloonIoHandler {
    /// The features of driver.
    driver_features: u64,
    /// Address space.
    mem_space: Arc<AddressSpace>,
    /// Inflate queue.
    inf_queue: Arc<Mutex<Queue>>,
    /// Inflate EventFd.
    inf_evt: EventFd,
    /// Deflate queue.
    def_queue: Arc<Mutex<Queue>>,
    /// Deflate EventFd.
    def_evt: EventFd,
    /* 省略 */
}
```

其中包含上述的两个virtio队列`inf_queue`和`def_queue`，以及对应的触发事件描述符（EventFd）`inf_evt`和`def_evt`。两个队列均使用了`Mutex`锁，保证了队列在同一时刻只有一个使用者对该队列进行操作。保证了多线程共享的数据安全。

```rust
fn process_balloon_queue(&mut self, req_type: bool) -> Result<()> {
    let queue = if req_type {
        &mut self.inf_queue
    } else {
        &mut self.def_queue
    }; //获得对应的队列
    let mut unlocked_queue = queue.lock().unwrap();
    while let Ok(elem) = unlocked_queue
        .vring
        .pop_avail(&self.mem_space, self.driver_features)
    {
        match Request::parse(&elem) {
            Ok(req) => {
                if !self.mem_info.has_huge_page() {
                    // 进行内存标记
                    req.mark_balloon_page(req_type, &self.mem_space, &self.mem_info);
                }
                /* 省略 */
            }
            Err(e) => {
                /* 省略错误处理 */
            }
        }
    }
    /* 省略 */
}
```

当相应的`EventFd`被触发后`process_balloon_queue`函数将会被调用。通过判断请求类型确定是“充气”还是”放气“，然后再从相应的队列中取数据进行内存标记。其中`while let`是Rust语言提供的一种循环模式匹配机制。借助该语法可以将队列中pop出来的所有数据遍历取出到`elem`中。



### 内存标记及优化：

标记内存在`mark_balloon_page`函数中进行实现，起初的实现思路为：将虚拟机传送过来的地址逐个进行标记。即，从队列中取出一个元素，转化为地址后立即进行标记。后来经过测试发现：balloon设备在对页地址进行一页一页标记内存时花费时间巨大。而同时也发现通过虚拟机传回来的地址中有大段的连续内存段。于是通过改变标记方法：由原来的一页一页标记改为将这些连续的内存统一标记。大大节省了标记时间。下面代码为具体实现：

```rust
fn mark_balloon_page(
        &self,
        req_type: bool,
        address_space: &Arc<AddressSpace>,
        mem: &BlnMemInfo,
    ) {
        let advice = if req_type {
            libc::MADV_DONTNEED
        } else {
            libc::MADV_WILLNEED
        };
        /* 略 */
        for iov in self.iovec.iter() {
            let mut offset = 0;
            let mut hvaset = Vec::new();
            while let Some(pfn) = iov_to_buf::<u32>(address_space, iov, offset) {
                offset += std::mem::size_of::<u32>() as u64;
                let gpa: GuestAddress = GuestAddress((pfn as u64) << VIRTIO_BALLOON_PFN_SHIFT);
                let hva = match mem.get_host_address(gpa) {
                    Some(addr) => addr,
                    None => {
                        /* 略 */
                    }
                };
                //将hva地址保存在hvaset的vec中
                hvaset.push(hva);
            }
            //对hvaset进行从小到大排序。
            hvaset.sort_by_key(|&b| Reverse(b));
            /* 略 */
                //将hvaset中连续的内存段进行标记
                while let Some(hva) = hvaset.pop() {
                    if last_addr == 0 {
                        free_len += 1;
                        start_addr = hva;
                    } else if hva == last_addr + BALLOON_PAGE_SIZE {
                        free_len += 1;
                    } else {
                        memory_advise(
                            start_addr as *const libc::c_void as *mut _,
                            (free_len * BALLOON_PAGE_SIZE) as usize,
                            advice,
                        );
                        free_len = 1;
                        start_addr = hva;
                    }

                    if count_iov == iov.iov_len {
                        memory_advise(
                            start_addr as *const libc::c_void as *mut _,
                            (free_len * BALLOON_PAGE_SIZE) as usize,
                            advice,
                        );
                    }
                    count_iov += std::mem::size_of::<u32>() as u64;
                    last_addr = hva;
                }
            /* 略 */
        }
    }
}
```

首先将virtio队列中的地址全部取出，并保存在vec中，然后将该vec进行从小到大的排序。有利于快速找出连续的内存段并进行标记。由于hvaset中的地址是按照从小到大排列的，因此可以从头开始遍历hvaset，遇到不连续的地址后将前面的连续段进行标记。这样就完成了由原来逐页标记到连续内存段统一标记的优化。

经过测试，StratoVirt的balloon速度也有了极大的提高。

## 关注我们

StratoVirt当前已经在openEuler社区（openEuler是一个开源、免费的Linux发行版平台，将通过开放的社区形式与全球的开发者共同构建一个开放、多元和架构包容的软件生态体系）开源。在未来的一段时间我们将开展一系列主题的分享，让大家更加详细的了解StratoVirt实现，非常期待您的围观和加入！

项目地址：https://gitee.com/openeuler/stratovirt

项目wiki：https://gitee.com/openeuler/stratovirt/wikis

项目交流：[virt邮件列表](https://mailweb.openeuler.org/postorius/lists/virt.openeuler.org/)或是提交一个[issue](https://gitee.com/openeuler/stratovirt/issues)。



