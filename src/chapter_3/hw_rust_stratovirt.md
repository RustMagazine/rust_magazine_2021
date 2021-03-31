# 华为 | 基于Rust的下一代虚拟化平台-StratoVirt

作者： 徐飞 / 后期编辑： 张汉东

---


## StratoVirt是什么

Strato，取自stratosphere，意指地球大气层中的平流层，大气层可以保护地球不受外界环境侵害，而平流层则是大气层中最稳定的一层；类似的，虚拟化技术是操作系统平台之上的隔离层，既能保护操作系统平台不受上层恶意应用的破坏，又能为正常应用提供稳定可靠的运行环境；以Strato入名，寓意为保护openEuler平台上业务平稳运行的轻薄保护层。同时，Strato也承载了项目的愿景与未来： 轻量、灵活、 安全和完整的保护能力。

StratoVirt是计算产业中面向云数据中心的企业级虚拟化平台，实现了一套架构统一支持虚拟机、容器、Serverless三种场景，在轻量低噪、软硬协同、安全等方面具备关键技术竞争优势。StratoVirt在架构设计和接口上预留了组件化拼装的能力和接口，StratoVirt可以按需灵活组装高级特性直至演化到支持标准虚拟化，在特性需求、应用场景和轻快灵巧之间找到最佳的平衡点。

<div align=center> <img src=".\sratovirt-images\StratoVirt.png" width="280" height="280" /></div>

## 为什么选择Rust
在项目成立初期，我们调研了业界成熟基于C语言开发的虚拟化软件-QEMU，统计了在过去十几年中QEMU的CVE问题，发现其中有将近一半是因为内存问题导致的，例如缓冲区溢出、内存非法访问等等。如何有效避免产生内存问题，成为我们在编程语言选型方面的重要考虑。因此，专注于安全的Rust语言进入我们视线。
* Rust语言拥有强大的类型系统、所有权系统、借用和生命周期等机制，不仅保证内存安全，还保证并发安全，极大的提升软件的质量。在支持安全性的同时，具有零成本抽象特点，既提升代码的可读性，又不影响代码的运行时性能。
* Rust语言拥有强大的软件包管理器和项目管理工具-Cargo
	* Cargo能够对项目的依赖包进行方便、统一和灵活的管理。项目所有的依赖包都定义在Cargo.toml文件中，开发者可以按需使用来自Rust官方仓库crates.io的各类功能包。
	* Cargo集成了完整的代码管理工具，例如项目创建（cargo new）、构建（cargo build）、清理（cargo clean）、测试（cargo test）、运行（cargo Run）等等。
	* Cargo在代码静态扫描方面提供相应的工具，能够进一步提升开发者编码风格和代码质量。
		* cargo fmt：使用符合rust-lang定义的Rust代码风格来规范Rust代码。
		* cargo check：可以对本地项目库和所有依赖进行编译检查，它会通过对项目进行编译来执行代码检查。
		* cargo clippy：一个Rust语言的lint工具集合包，包含了超过350种lint规则。

## StratoVirt的优势

StratoVirt是openEuler最稳定、最坚固的保护层。它重构了openEuler虚拟化底座，具有以下六大技术特点。

* 强安全性与隔离性
  * 采用内存安全语言Rust编写， 保证语言级安全性；
  * 基于硬件辅助虚拟化实现安全多租户隔离，并通过seccomp进一步约束非必要的系统调用，减小系统攻击面；
* 轻量低噪
  * 轻量化场景下冷启动时间<50ms，内存底噪<4M；
* 高速稳定的IO能力
  * 具有精简的设备模型，并提供了稳定高速的IO能力；
* 资源伸缩
  * 具有ms级别的设备伸缩时延，为轻量化负载提供灵活的资源伸缩能力；
* 全场景支持
  * 完美支持X86和Arm平台：X86支持VT，鲲鹏支持Kunpeng-V，实现多体系硬件加速； 
  * 可完美集成于容器生态，与Kubernetes生态完美对接，在虚拟机、容器和serverless场景有广阔的应用空间；
* 扩展性
  * 架构设计完备，各个组件可灵活地配置和拆分；
  * 设备模型可扩展，可扩展PCIe等复杂设备规范，实现标准虚拟机演进； 

## StratoVirt的架构
  StratoVirt核心架构自顶向下分为三层：

* OCI兼容接口：兼容qmp协议，具有完备的OCI兼容能力。
* BootLoader：抛弃传统的BIOS + GRUB启动模式，实现了更轻更快的BootLoader，并达到极限启动时延。
* MicroVM：充分利用软硬协同能力；精简化设备模型；低时延资源伸缩能力；

<div align=center> <img src=".\sratovirt-images\StratoVirt-arch.png" width="500" height="500" /></div>

StratoVirt[源码目录](https://gitee.com/openeuler/stratovirt)解析主要分为四部分：

- address_space：地址空间模拟，实现地址堆叠等复杂地址分配模式。
- boot_loader：内核引导程序，实现快速加载和启动功能。
- device_model：仿真各类设备，可扩展，可组合。
- machine_manager：提供虚拟机管理接口，兼容QMP等常用协议，可扩展。

<div align=center> <img src=".\sratovirt-images\code_directory.png" width="400" height="400" /></div>

当前StratoVirt开源代码中实现的是轻量化虚拟机模型，是能实现运行业务负载的最小的设备集合。因此LightMachine是StratoVirt最重要的顶层数据结构，它的逻辑上分为CPU模拟管理，地址空间管理，IO设备模拟管理（包括中断控制器和bus数据结构中管理各类仿真设备，例如virtio设备，serial设备等），如下图右侧所示：

<div align=center> <img src=".\sratovirt-images\light_machine.png" /></div>

首先，我们先看一下address_space地址空间模拟实现功能：

<div align=center> <img src=".\sratovirt-images\address_space.png" /></div>

- 内存地址空间通过Region组成树形层次关系，支持地址堆叠和优化级。
- 通过快速映射算法形成扁平地址空间（Flat View)。
- 通过设置Listener监听地址空间变化，执行相关回调函数。

其次，我们再看一下CPU模拟实现功能：

<div align=center> <img src=".\sratovirt-images\CPU.png" /></div>

- 基于KVM暴露接口实现虚拟CPU的硬件加速。
- 通过ArchCPU结构隐藏体系架构（aarch64和x86_64）差异，具体实现位于体系架构相关目录中。
- Arc反向索引该CPU所属的LightMachine虚拟机对象，使得后续在虚拟机内扩展设备时，CPU可访问该对象。

最后，我们再看一下IO设备模拟功能：

轻量化虚拟机的主要设备均通过VirtioMMIO协议实现，下图右侧是VirtioMmioDevice的通用数据结构。

<div align=center> <img src=".\sratovirt-images\IO.png" /></div>

在IO设备初始化阶段，通过VirtioMMIO协议协商前后端都可以访问的virtio queue、中断事件以及通知事件等等。当前端VM有IO请求时，将请求数据写入virtio queue中，通过通知事件告知后端StratoVirt；后端监听通知事件发生时，读取virtio queue中的请求数据，根据请求数据进行IO处理，IO请求处理完成后，并以中断事件方式通知前端VM。


<div align=center> <img src=".\sratovirt-images\IO_process.png" width="500" height="300" /></div>



## StratoVirt未来

StratoVirt的发展路标为，通过一套架构，支持轻量虚拟机和标准虚拟机两种模式：

* 轻量虚拟机模式下，单虚机内存底噪小于4MB，启动时间小于50ms，且支持ms级时延的设备极速伸缩能力，当前已经开发完毕，2020年9月已经在openEuler社区开源；
* 标准虚拟机模式下，可支持完整的机器模型，启动标准内核镜像，可以达成Qemu的能力，同时在代码规模和安全性上有较大优势。 

## 关注我们

StratoVirt当前已经在openEuler社区（openEuler是一个开源、免费的Linux发行版平台，将通过开放的社区形式与全球的开发者共同构建一个开放、多元和架构包容的软件生态体系）开源。在未来的一段时间我们将开展一系列主题的分享，让大家更加详细的了解StratoVirt实现，非常期待您的围观和加入！

项目地址：https://gitee.com/openeuler/stratovirt

项目wiki：https://gitee.com/openeuler/stratovirt/wikis

项目交流：[virt邮件列表](https://mailweb.openeuler.org/postorius/lists/virt.openeuler.org/)或是提交一个[issue](https://gitee.com/openeuler/stratovirt/issues)。