# 外刊评论

精选 Reddit 讨论热帖

---

## 大纲

- 2021 总结系列
    - Rust嵌入式工作组2021总结
    - Cranelift 进度报告：2021 年回顾
    - Rust-Analyzer 2021 总结 
    - SixtyFPS 2021 总结
    - Dimforge 的 2021 和 2022 目标
    - Mozilla Glean 的 2021 总结
    - hyper 生态相关的 2021 总结
    - Wgpu 2021 总结
- Reddit 讨论热帖
    - 你有哪些项目没有使用 Rust 语言？为什么？
    - 有哪些重要特性是 Rust 缺失 而 C/Cpp 中存在的？
    - 用 Rust 编写科学计算有哪些资源？
    - Rust 是在炒作吗？
    - Rust 只是复制而不是 Move ？ 
    - 你梦想中的 Rust 大学课程是什么样的？


## 2021 总结系列

> [https://blog.rust-embedded.org/this-year-in-embedded-rust-2021/](https://blog.rust-embedded.org/this-year-in-embedded-rust-2021/)

### 对Rust嵌入式工作组2021总结的总结，看完以后感觉Rust在嵌入式大有可为：

1. 树莓派2021发布首款RP2040微控制器中有两个Cortex M0内核。这让工作组的成员开始思考，在多核微控制器下该如何提供安全性，由此有了 rp-rs 组织。
2. Espressif （乐鑫）正式雇佣mabez 针对eso芯片开发Rust支持：esp-rs
3. 其他平台也逐渐开始支持Rust，包括：Atmel ARM SAM-D和SAM-E、Atmel AVR、NXP ARM iMX. RT微控制器、ARM nRF51、52和9160蓝牙/LTE设备、RISC-V、树莓派、STM32等。
4. 嵌入式Rust生态得到长足发展：嵌入式并发框架已经1.0 、嵌入式异步框架正在大力开发且支持STM32，nRF和RP2040平台，并且还深深影响着Rust异步的改进、嵌入式开发和调试工具又发布了新的探针工具、嵌入式smoltcpTCP/IP栈发布了新版本、嵌入式图形库Matrix发布了新版本、新的嵌入式实时OS Hubirs 开源。
5. 嵌入式工作组自身维护的项目在这一年也是大力开发和维护中。

### Cranelift 进度报告：2021 年回顾

> [https://bytecodealliance.org/articles/cranelift-progress-2021](https://bytecodealliance.org/articles/cranelift-progress-2021)

Cranelift 是字节码联盟的原生代码编译器，作为Wasmtime和Lucet WebAssembly 虚拟机的基础，也用于其他环境，例如作为Rust 编译器的替代后端。

2021 年完成的目标：

1. 从旧后端框架到新后端框架的迁移（全部完成）。这项工作持续了两年时间，新的后端框架可以达成“在更换发动机的同时保持飞机飞行”。
2. 安全性：CVE、VeriWasm（完成）。收到了[第一个 CVE](https://github.com/bytecodealliance/wasmtime/security/advisories/GHSA-hpqh-2wqx-7qp5)，并且开发 VeriWasm 工具来检查从 WebAssembly 编译的机器代码，以确保其沙箱属性完好无损。现在也将其集成到了 lucet 中。
3. 模糊测试和正确性（完成）。除了 VeriWasm 之外，还在 [oss-fuzz](https://github.com/google/oss-fuzz) 上运行很多模糊测试目标。现在还有针对V8、官方 Wasm 规范解释器和CLIF（Cranelift IR）解释器的差异化模糊测试。还有一个极具创新性的项目来构建“自定义修改器”，[wasm-mutate](https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wasm-mutate)，它允许我们所有基于 Wasm 的模糊测试通过进行语义保留更改来更有效地生成和测试有趣的程序。
4. 增加基准测试套件 [sightglass](https://github.com/bytecodealliance/sightglass) ，在改进 Cranelift 时可以提供稳定的基准测试基础。该基准测试确保引入新的指令 DSL 没有性能损失。
5. SIMD 支持。目前 Cranelift 已优先完成对 aarch64 和 x86-64 的 SIMD 支持。并且现在已经有了一个完整的 Wasm-SIMD 实现。
6. 引入寄存器分配器 [regalloc2](https://github.com/bytecodealliance/regalloc2)。为了优化编译器的性能，引入了 regalloc2，目前这部分工作还在持续改进中。
7. [指令选择器 DSL (ISLE, Instruction Selection Lowering Expressions)](https://github.com/bytecodealliance/wasmtime/blob/main/cranelift/isle/docs/language-reference.md)。指令选择(instruction selection)是将中间语言转换成汇编或机器代码的过程。ISLE 是一种领域特定语言，用于编写指令选择和重写规则。ISLE 源文本被编译成 Rust 代码。ISLE 的目标是表示指令降低模式。指令降低模式是一种规范，即 IR (CLIF) 中的某种运算符组合，当在特定条件下组合时，可以编译成特定的机器指令序列。
8. 收到了一项重大贡献：一个支持[IBM z/Architecture](https://en.wikipedia.org/wiki/Z/Architecture)（也就是64 位 ISA，具有直接向后兼容的 1960 年代大型机谱系）的[全功能 s390x 后端](https://github.com/bytecodealliance/wasmtime/pull/2874)。

### Rust-Analyzer 2021 总结

> [https://rust-analyzer.github.io/blog/2021/12/30/2021-recap.html](https://rust-analyzer.github.io/blog/2021/12/30/2021-recap.html)

2022年 ra 可能会被官方收录。

1. 最大改进之一可能是属性过程宏的支持。过程宏目前ABI不稳定，虽然它的不断发展，但每次更改都会破坏 proc-macro 服务器。为了解决这个问题，ra团队现在尝试至少支持 ~3 个 ABI 版本，这些版本名义上对应于最新的稳定版、测试版和夜间工具链。实际上，ra团队目前支持更旧的版本：1.47 及更高版本。
2. 本地语法项（item）解析。不如函数内部 `use` 自定义枚举。
3. 整合 [chalk](https://github.com/rust-lang/chalk) 中的类型替代之前的类型表示，避免每次都进行一次多余的转换。 
4. 开始考虑常量泛型参数的支持，这非常困难，目前[进展缓慢](https://github.com/rust-analyzer/rust-analyzer/issues/8655)。因为 Rust 编译器通过 miri 来进行常量表达式求值，但是 ra 并没有共享 miri 使用的数据结构，在未来也不打算这么做，ra团队自己实现了常量表达式求值的功能。
5. ra使用的语法树库 [rowan](https://github.com/rust-analyzer/rowan) 已被调整为允许创建不可变（Immutable）语法树的可变副本。
6. 今年的最终版本为语言服务器二进制文件的获取方式带来了两个重要的变化。到目前为止，该扩展程序调用 GitHub API 来查找匹配的版本，并从那里下载服务器。此外，如果你选择加入Nightly channel，则扩展程序每天都会搜索更新的 VSIX。
7. 支持独立的 Rust 文件，可以使用大部分不依赖于 cargo 的功能。
8. 另外还改进了很多小功能，提升 IDE 体验。

### SixtyFPS 2021 总结

> [https://sixtyfps.io/blog/2021-in-review.html](https://sixtyfps.io/blog/2021-in-review.html)

从2021年开始，就看到一些基于Rust和开源来创业的公司慢慢增加，这个专注于GUI的SixtyFPS也是其中一个，这个开源GUI工具库面向全平台，目标之一是取代Qt。

看他们的 2021 总结报告，发现他们在2021才注册了公司，和tQCS这样的咨询公司建立了合作关系，找到了第一个客户，招募了新成员。（感觉国外技术流创业跟国内还是有点区别）。tQCS提供世界No.1的Qt咨询和UI/UX设计服务，选择和SixtyFPS合作，这意味着什么呢？之前知乎上还有人黑Rust说连qt都支持不好，没有前途？

2022年他们可能要长足支持嵌入式，要有新品牌名字了。

### Dimforge 的 2021 和 2022 目标

2021 年，[Dimforge 开源组织](https://github.com/dimforge)为 Rust 社区带来了用于线性代数和物理模拟的开源 crate 中最重要的新增内容。

- Rapier, 用 Rust 编写的用于游戏、机器人和动画的 2D 和 3D 物理引擎。2021年发布了新的版本，新增了很多改进。此版本的重点是对多体关节的支持。
- nalgebra，是 Rust 的通用线性代数库。2021 年新增功能包括：
    - 发布新的nalgebra-sparse crate
    - 集成常量泛型来改进了人体工程学、可调试性和泛型编程，用于涉及静态大小矩阵的代码
- 修复一些 Bug
- 增加了与最近宣布的rust-cuda项目的兼容性。
- 发布了新的碰撞检测库parry
- 致力于通过 SIMD 优化来提高流体模拟 Salva 的性能

对于 2022 年，有两个主要目标：

- 专注于实现 Rapier 的高级特性。
- 继续对用于流体和变形体模拟的 MPM的探索。

目前 Dimforge 接受的赞助商有 ： [croquet: 做 web 实时服务](https://www.croquet.io/) / [embark： 游戏工作室](https://embark-studios.com/) / [fragcolor: 正在为分布式游戏生态系统构建引擎](https://fragcolor.com/)


### Mozilla Glean 的 2021 总结

> [https://fnordig.de/2021/12/17/glean-in-2021/](https://fnordig.de/2021/12/17/glean-in-2021/)

[glean](https://github.com/mozilla/glean) 是 Mozilla 在 2021 年发布的现代化跨平台遥测（telemetry）库。

Glean SDK 是一个完全可自助服务的遥测 SDK，可跨不同平台使用。它使产品所有者和工程师能够检测他们的产品并依赖他们的数据收集，同时遵循 Mozilla 政策和隐私标准。

2021 年总共发布了 9 个主要版本。现在 Mozilla 越来越多的产品和项目正在使用 Glean，包括Rally 研究、Mozilla VPN 客户端和新版Focus for Android。[Glean Dictionary ](https://dictionary.telemetry.mozilla.org/)中提供了完整的产品列表。

在 [Mozilla 数据文档](https://docs.telemetry.mozilla.org/concepts/glean/glean.html#the-glean-design-principles)中可以看到关于 Clean 详细介绍。


### hyper 生态相关的 2021 总结

> [https://seanmonstar.com/post/672473147126300672/hyper-ish-2021-in-review](https://seanmonstar.com/post/672473147126300672/hyper-ish-2021-in-review)

1. hyper 1.0 即将到来。
2. 为 hyper 提供了 C-API，允许 curl 使用 hyper 作为 HTTP 后端。相关文章：[https://aws.amazon.com/blogs/opensource/how-using-hyper-in-curl-can-help-make-the-internet-safer/](https://aws.amazon.com/blogs/opensource/how-using-hyper-in-curl-can-help-make-the-internet-safer/)
3. 开发 http3 库 [h3](https://github.com/hyperium/h3)，目前已经更新到 Tokio 1.0 和 quinn 0.8 中。
4. 开发 Axum，它是一个 Rust 服务器框架，旨在从 Tower 和 Tower-HTTP 扩展。
5. 推出了 [Tokio Console](https://tokio.rs/blog/2021-12-announcing-tokio-console)的第一个版本。

### Wgpu 2021 总结

gfx-rs 是一个为 Rust 带来高效跨平台图形的项目，这篇文章包含该项目的主要里程碑、概念和回顾。

从 gfx-hal 转移到新创建的 wgpu-hal 并重组了代码仓库以将所有内容保持在一起。 与此同时放弃了 SPIRV-Cross 以支持 naga，实现纯 Rust 技术堆栈。在 0.10 发布帖子中了解更多信息，归功于@kvark。

同时，@cwfitzgerald 使用 Rust 集成测试和示例快照改进了我们的测试基础设施。 最重要的是，wgpu 已经与 Deno 紧密集成（感谢 Deno 团队的努力！），开辟了在真正的 CTS 上进行测试的道路，现在可以在 CI 中使用。

WebGL 适配变得可用，现在 wgpu-rs 示例可以使用 WebGL 在线运行。

[https://gfx-rs.github.io/2021/12/25/this-year.html](https://gfx-rs.github.io/2021/12/25/this-year.html)


## 你有哪些项目没有使用 Rust 语言？为什么？

> [https://www.reddit.com/r/rust/comments/r8xsag/what_projects_didnt_you_make_in_rust/](https://www.reddit.com/r/rust/comments/r8xsag/what_projects_didnt_you_make_in_rust/)

Rust 社区大部分都是宣传自己使用 Rust 做了什么，而这个帖子则反问：Rust 不能做什么？

大家的回答：

1. 想用 Python 快速做出艺术作品推荐引擎的原型，因为是机器学习相关。但是想稍后用 Rust 重写它们。
2. 写bash脚本恐怕不太方便。
3. 写 GUI 依然比较困难。但可能没有使用过 egui/ iced/ druid / tauri 这些 GUI 框架。
4. 高性能科学计算领域目前还比较困难。AMD 和 Nvidia 都花费大量资金确保他们的编译器链与 C++ 兼容，因此使用 C++ 时很容易在 GPU 上启动和运行。但是像  rust-cuda  这样的库，依旧很有前景。
5. 用 Rust 实现游戏，目前还没有一个成熟的游戏引擎。
6. web 前端应用不会考虑 Rust 。
7. 与 DevOps 相关的脚本。

还有很多评论，就不一一列出。这些评论里也存在一些对 Rust 语言的误解，总之，见仁见智吧。

有一句评论，我觉得说的很好：Rust 语言你平常可以不用，但是当你真正需要的时候，你会发现 Rust 语言很好。


## 有哪些重要特性是 Rust 缺失 而 C/Cpp 中存在的？

>  [https://www.reddit.com/r/rust/comments/rj8gfg/which_important_features_from_cc_are_missing_in/](https://www.reddit.com/r/rust/comments/rj8gfg/which_important_features_from_cc_are_missing_in/)

这篇帖子同样也是值得看的，这里摘录了部分评论：

1. C 语言中的 位域，是 Rust 缺失的。至于Rust为什么目前没有支持，可能是考虑到 可移植性。如果Rust放弃可移植性，那么位域的支持不是什么困难的问题。
2. Rust 标准库中还缺乏一些 侵入式的集合。
3. Rust 目前无法为集合单独指定内存分配器。
4. 没有稳定的 ABI 。 （好像 C/Cpp 都不存在稳定的 ABI，C-ABI只是一种事实标准）
5. 泛型特化 / 泛型关联类型 / 泛型常量 等
6. 面向零基础的 Rust 教学材料。为什么 C/Cpp 语言学习可以面向零基础？

Rust 虽然还在完善中，但其实并不一定要把 C/Cpp 支持的特性都再次支持一遍。不过这篇帖子里评论大部分还是比较中肯的，建议阅读。

## 用 Rust 编写科学计算有哪些资源？

> [https://www.reddit.com/r/rust/comments/rk12bg/writing_rust_libraries_for_the_python_scientific/](https://www.reddit.com/r/rust/comments/rk12bg/writing_rust_libraries_for_the_python_scientific/)

这里摘录该贴评论中提及的第三方库：

- [polars](https://github.com/pola-rs/polars)，Rust 实现的多线程 DataFrame 库。
- [PyO3](https://github.com/PyO3) 和 [maturin](https://github.com/PyO3/maturin) ，用于创建 Rust 和 Python 的混合库。这有一个示例可以参考：[https://github.com/entity-neural-network/ragged-buffer](https://github.com/entity-neural-network/ragged-buffer)

这篇帖子里评论数不是很多，在科学计算领域还有一些其他库，这里就不一一列出来了。

## Rust 是在炒作吗？

> [https://www.reddit.com/r/rust/comments/rkx12s/stop_whining_about_rust_hype_a_prorust_rant/](https://www.reddit.com/r/rust/comments/rkx12s/stop_whining_about_rust_hype_a_prorust_rant/)

有一篇文章[《停止抱怨Rust炒作》](https://thenewwazoo.github.io/whining.html)在 Reddit 上引起了讨论。

该篇文章主要是劝诫一些人，停止抱怨 Rust 炒作，而应该去好好了解一下 Rust ，不要动不动就用“炒作”这个词。

国内技术社区我好像并没有看到“炒作”这个词用到 Rust 身上，但知乎里确实见了几个 Rust 黑粉。在国外技术社区论坛上也很少看见这类言论。毕竟 Rust 还是最受欢迎的语言。

我们来看看 Reddit 里评论怎么说的，这里摘录几条：

1. “我选择在我的公司采用 Rust，并帮助所有其他 3 名工程师学习这门新语言。对于 Rust 炒作的各种抱怨，我基本上只是一个旁观者”。
2. “我可以像编写 Python 一样快速地编写 Rust，其他人也可以。”
3. “大约两年前，我想知道它是否会发生在我身上，或者我是否会在使用 Rust 构建之前使用 Python/F# 进行原型设计。现在，我可以确认，我用 Rust 的开发速度和用 Python 一样快”
4. “大声笑，我记得几年前在 Discord 上看到有人这么说，并认为他满嘴胡话，现在……我可以像编写 Python 一样快速地编写 Rust。”

太多的评论就不贴了。你是否也认为 Rust 是过度炒作呢？至少我不这么认为，甚至我还觉得 Rust 从来都没有炒作过。

## Rust 只是复制而不是 Move ？ 

来自： [https://www.reddit.com/r/rust/comments/rlzhy1/rust_is_creating_copy_instead_of_moving/](https://www.reddit.com/r/rust/comments/rlzhy1/rust_is_creating_copy_instead_of_moving/)

Reddit 一位网友贴出如下代码：

```rust
struct Massive {
    a: [i128; 10000],
}

impl Massive {
    #[inline(always)]
    fn stupid(mut self) -> Self {
        println!("{:?}", &mut self.a[1] as *mut i128); // 0x7ffe178babc0

        //do some stuff to alter it
        self.a[1] += 23;
        self.a[4] += 24;
        
        self
    }

}

fn main() {
    let mut f = Massive { a: [10i128; 10000] }; // 0x7ffe17845870

    println!("{:?}", &mut f.a[1] as *mut i128);

    let mut f2 = f.stupid();

    println!("{:?}", &mut f2.a[1] as *mut i128); // 0x7ffe17893ac0
}

```

结构体 Massive 没有实现 Copy trait，所以默认是 Move 语义，但是 move 以后它们的地址都变了。这意味着每次 Rust 都是在复制这个大的结构体对象，性能很差。所以他好奇，为什么移动语义下还创建这三个副本？然后他在release编译时启用了lto优化，也无济于事。

他怀疑这是 Rust 的 Bug ，或者 Rust 没有实现类似 Cpp 里的 复制消除（Copy Elision）优化。

真的如此吗？

其实，在 Rust 里，Move 语义是语言的语义，像上面的结构体 Massive，它是存储在栈上。栈上的数据，如果要实现 Move 语义，那么必须经过 memcpy （按位复制）来进行 Move。

也就是说，Move 是语义，而 memcpy 是底层实现。语义和实现要分开理解。

复制消除（Copy Elision）是属于性能优化层面，不是语言的语义，不能混为一谈。

Reddit 中 一个网友指出，通过查看去除 `println!` 语句之后再编译后的汇编：[https://godbolt.org/z/G5Ghr5jxd](https://godbolt.org/z/G5Ghr5jxd)，发现并没有通过 memcpy指令进行复制。所以看上去是打印语句抑制了编译器的复制消除优化。

所以，Rust 编译器是有 复制消除 优化的，只不过 `println!` 打印语句中使用了借用，而抑制了编译器优化。

至于这个问题本身就是错误的，不应该是“为什么只是复制而不是Move”，而应该换成 “为什么 Move 没有优化” 才对。 移动语义本身就是复制数据，至于只复制指针，还是整个结构，这取决于这个数据结构自身如何存储。

移动语义是 Rust 中唯一的语义。Rust 确实提供了两个工具来提供 C++ 的面向复制的语义：Copy和Clone。但是 Clone 永远不会被隐式调用，它只能被显式调用。Copy 是 Clone 的一个特例，它只实现了按位复制。并且为一些基本数据类型实现了 Copy ，在赋值的时候隐式调用，但这个只是一种手段，避免Move以后的类型变为未初始化而已。Move 语义的本质就是让移动的变量变为未初始化状态，编译器就不允许它继续被使用了。

还是那句话，语义和实现不要混为一谈。


## 你梦想中的 Rust 大学课程是什么样的？

> [https://www.reddit.com/r/rust/comments/rwp8mo/teaching_rust_at_the_university_of_warsaw/](https://www.reddit.com/r/rust/comments/rwp8mo/teaching_rust_at_the_university_of_warsaw/)

该贴作者在[波兰华沙（Warsaw）大学](https://mimuw.edu.pl/en) 担任助教，一直在教授 C 语言中的内存管理和编程（第一年）和 Java 和 C 中的并发和并行编程（第二年）。目前该学校决定为二年级学生开设 Rust 课程，他被委托负责准备课程和授课。

他想向 Rust 开发者寻求建议，你梦想中的 Rust 大学课程是什么样的呢？

> 背景：该课程将包括约 13 次 1.5 小时的每周会议（结合讲座和实验室）。将有 2 个小组，大约 15 名学生参加（这不是必修课，可以从列表中选择）。学生将具备 Java、C 和 C++（包括其最新特性，如概念）、并发编程和算法以及数据结构的丰富知识。

他目前的教学计划大概如下：

1. 将官方的 the book 作为教材。作为教师，他会将 the book 中的知识浓缩为 PPT 进行教授。在学习语法的过程中，也掺杂一些 Rust 生态的最佳实践内容。
2. 在学习完 the book 之后，将以异步为主要侧重内容。
3. 设置五个不同等级的小任务，让学生完成，每个任务有十天时间来完成。

详细内容可以参考reddit贴。在评论中有人给出一些建议：

1. 耶鲁大学的钟林教授开设 [《CPSC 429a, 计算机系统设计原理》 和 《CPSC 425b，移动和嵌入式系统 》](http://catalog.yale.edu/ycps/courses/cpsc/) 支持 Rust 语言。但是该课程不是公开课。
2. 一位本科生使用 Rust 完成强化学习库：[https://github.com/ZuseZ4/Rust_RL](https://github.com/ZuseZ4/Rust_RL)。但是他反对将 Rust 用于机器学习的教学中，因为使用 Python 学习机器学习更加简单。（这样说也很有道理，学习的时候要讲究学习目标，学生的学习目标是学习机器学习领域知识，而非 Rust）。
3. 一位朋友说，在大学里应该教授学生如何解决问题。比如通过学习多线程生命游戏的实现，来学习解决多线程并发的问题。