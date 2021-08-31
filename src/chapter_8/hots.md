# 社区热点

编辑：张汉东

---

## Rust 毫无意外地，连续六年成为了 Stackoverflow 榜单最受喜爱语言。

[https://insights.stackoverflow.com/survey/2021#technology-most-loved-dreaded-and-wanted](https://insights.stackoverflow.com/survey/2021#technology-most-loved-dreaded-and-wanted)


## RustChinaConf 2021 正式启动

好消息，2021 Rust China Conf 要来了！ 

本次大会初步定于2021年10月16、17日在上海市徐汇区云锦路701号西岸智塔AI Tower 45层 举行。

详情：[RustChinaConf 2021 正式启动](./rust_china_conf_2021.md)

RustChinaConf 2021 议题开放申请[https://shimo.im/forms/xqpwpdXw6YxrJTj9/fill](https://shimo.im/forms/xqpwpdXw6YxrJTj9/fill)

## UltraOS获第一届全国大学生操作系统比赛一等奖

2021第一届全国大学生操作系统比赛的比赛结果公布了，哈工大（深圳）的李程浩，宫浩辰，任翔宇获得了内核实现赛道的一等奖，指导教师为夏文老师和江仲鸣老师。 他们用Rust语言设计实现了基于RISC-V64的多核操作系统UltraOS，支持qemu和k210平台运行，在比赛过程的多个评比阶段排名第一。 项目网址 [https://gitlab.eduxiji.net/ultrateam/ultraos https://github.com/xiyurain/UltraOS](https://gitlab.eduxiji.net/ultrateam/ultraos https://github.com/xiyurain/UltraOS) 项目使用GPL3.0协议，欢迎开发者使用该项目进行学习。项目使用了洛佳等开发者的RustSBI 2021.03.26版本，以及吴一凡等开发者的rCoreTutorial-v3 2021.03.26版本（清华大学计算机系2021 OS课实验指导教程）。 这也说明了，基于开源社区的模式，采用Rust开发操作系统等系统软件是Rust语言的一种发展趋势。

## Rust Search Extension 1.3 发布

小编的 Rust 插件发布新版本半个多月了，一直忘了发帖了，今天补上。欢迎大家体验！

更新说明：https://github.com/huhu/rust-search-extension/wiki/V1.3.0-Release-Candidate-(zh_CN)

[https://rust.extension.sh/changelog](https://rust.extension.sh/changelog)


## RIIR (Rewrite it in Rust ) Rome !

其官网写道：

> Rome 是一家工具开发公司，为 JavaScript 和 Web 开发构建了第一个一体化工具。 我们希望让产品开发人员专注于构建他们的产品，而不是为其提供工具和配置。 我们已收到 450 万美元的资金，并致力于开源社区。我们正在壮大我们的团队，以从头开始用 Rust 重写 Roma，并为我们未来的所有工作奠定基础。

- [https://rome-tools-inc.breezy.hr/](https://rome-tools-inc.breezy.hr/)
- [https://rome.tools/](https://rome.tools/)

## Bevy 发布一周年 

其中作者如图写道。

在微软上班一个月目测 至少 1w6 美刀/月， 但是这哥们现在准备考虑以开源为职业目标了，4000刀/月甚至可以让他存一点钱了，这就是真爱。

https://bevyengine.org/news/bevys-first-birthday/

## Next.js 11.1 最新版发布

亮点：使用 基于 Rust 的工具 swc 替代了 babel和terser。

将 swc 作者也招募到团队里了。

[https://nextjs.org/blog/next-11-1](https://nextjs.org/blog/next-11-1)

## 第一个非官方的 Rust 编程语言游戏Jam

这个Jam更侧重于使用 Rust 而不是其他任何东西。 这意味着您不受设计、音乐或图形的限制，只要您使用 Rust 来制作它！

[https://itch.io/jam/rusty-jam](https://itch.io/jam/rusty-jam)

## Rust 云原生组织（github 组织）成立

[Rust 云原生组织(https://rust-cloud-native.github.io/)]成立，用于推动 Rust 在云原生领域的生态发展。

其实 Rust 生态中已经有一些有关云原生的项目了：

- Bottlerocket OS
- Cloud Native Rust Day
- Firecracker
- kube-rs
- Kubewarden
- Krustlet
- Rust Foundation
- TiKV

这个组织的存在是为了使Rust在 "云 "中的使用成为可能，其重点是基础设施软件和相关组件。你可能对 "云 "技术很熟悉，比如Docker和Kubernetes。该组织的存在是为了促进存在于同一技术领域的项目。

这份博客通告，只是一种呼吁。由于目前还没有一个关于云原生Rust的中心位置，组织者主要是想启动一些东西。

详情请参考：[https://nickgerace.dev/post/launching-rust-cloud-native](https://nickgerace.dev/post/launching-rust-cloud-native)

## wgpu v0.10 发布，采用纯 Rust 写成

wgpu 是安全和可移植的 GPU 抽象，使用 Rust 写成，并实现了 WebGPU API 。

新版本值得期待的特性包括更为扁平化的 GPU 抽象和小巧的代码库，以及完全用 Rust 技术栈带来的便利，包括更方便的构建以及减少编译时的怪异行为。

Bevy 团队决定完全移除旧的 “抽象渲染层” ，转为使用 wgpu 作为核心的渲染抽象，以使得代码更精练、更简单，并且与广泛的 wgpu 生态更兼容。

[https://gfx-rs.github.io/2021/08/18/release-0.10.html](https://gfx-rs.github.io/2021/08/18/release-0.10.html)

Bevy 核心开发者的评论: 

> 作为Bevy的首席开发者，这个版本让我感到兴奋，原因有很多。
> - 更小的（和更少的抽象）代码库意味着我们可以更容易地用我们现在和将来需要的功能来扩展wgpu。(例如：XR、光线追踪、暴露原始后端apis）。) 进入的门槛低了很多。
> - 这表明wgpu团队是可以接受我们的反馈的。在我们的 "新渲染器实验 "中，有一段时间我们正在考虑为我们的新渲染器采用其他 "更扁平 "的gpu抽象。他们立即考虑到了这一点，并启动了这个重新架构。还有其他一些人也有类似的反馈，所以我不能完全归功于此，但这个时机是完美的。
> - 纯粹的Rust栈意味着我们的构建更加简单。结合Naga的着色器反射和编译，我们可以在我们的管道中移除许多来自非rust依赖的 "构建怪癖"。Windows尤其受到这种构建怪异的影响，我很高兴不再需要处理这些问题。
> - 由于过去的几点，将wgpu作为我们的 "主要gpu抽象层 "的 "风险 "已经大大降低了。因此，我们决定完全删除我们以前的 "抽象渲染层"，转而使用wgpu。这意味着wgpu不再是一个 "bevy_render后端"。它现在是bevy_render的核心gpu抽象。这使得我们的代码更小，更简单，并且与更广泛的wgpu生态系统更兼容。
> - 新的wgpu有一个正在进行的WebGL2后端。这将有望最终消除对第三方bevy_webgl2后端的需求（它为我们提供了良好的服务，但它有自己的怪癖和复杂性）。

## Apache Arrow DataFusion 5.0.0 版本

Apache Arrow 团队很高兴地宣布 DataFusion 5.0.0 版本。这涵盖了 4 个月的开发工作，包括来自以下 31 位不同贡献者的 211 次提交。

[https://arrow.apache.org/blog/2021/08/18/datafusion-5.0.0/](https://arrow.apache.org/blog/2021/08/18/datafusion-5.0.0/)

## 如何组织大型 Rust workspace

在本文中，作者分享了自己组织大型Rust项目的经验。这并不权威, 只是作者通过反复试验发现的一些技巧。

[https://matklad.github.io/2021/08/22/large-rust-workspaces.html](https://matklad.github.io/2021/08/22/large-rust-workspaces.html)
