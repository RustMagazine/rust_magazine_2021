# 建立Async Rust的共同愿景

2021年3月18日·Niko Matsakis 代表 [Async Foundations Working Group](https://rust-lang.github.io/wg-async-foundations/)


在 [异步基金会工作组](https://rust-lang.github.io/wg-async-foundations/) 认为 Rust 能够成为最热门的选择之一为构建分布式系统，从嵌入式设备到基础云服务。无论他们将其用于什么，我们都希望所有开发人员都喜欢使用 Async Rust。为了实现这一点，我们需要将 Async Rust 移至目前的“MVP”状态之外，并使所有人都可以使用它。

我们正在开展合作，为 Async Rust 构建共享的 [愿景文档](https://rust-lang.github.io/wg-async-foundations/vision.html#-the-vision) 。`我们的目标是让整个社区参与到集体的想象中`：我们如何才能使使用异步 I/O 的端到端体验不仅是一种务实的选择，而且是一种快乐的选择？

### 愿景文件始于现状...

“视觉文档”以一连串字符开头。每个角色都取决于由其背景决定的特定 Rust 值（例如，性能，生产率等）；这种背景也告诉了他们使用 Rust 时所带来的期望。

让我向您介绍一个角色，[格蕾丝（Grace）](https://rust-lang.github.io/wg-async-foundations/vision/characters/grace.html) 。作为一名经验丰富的 C 开发人员，Grace 习惯了高性能和控制能力，但是她喜欢使用 Rust 获得内存安全性的想法。这是她的传记：

> Grace 从事 C 和 C++ 的编写已经有很多年了。她习惯于破解许多底层细节，以哄骗自己的代码获得最大的性能。她还经历了由于 C 中的内存错误而导致的史诗般的调试会话。她对 Rust 感兴趣：她喜欢这样的想法：获得与 C 相同的控制和性能，但又从内存安全性中获得了生产力上的好处。她目前正在尝试将 Rust 引入她正在使用的某些系统中，并且她还在考虑将 Rust 用于一些新项目。

对于每个角色，我们都会编写一系列[“现状”故事](https://rust-lang.github.io/wg-async-foundations/vision/status_quo.html) ，描述他们在尝试实现目标时面临的挑战（通常以戏剧性的方式失败！）。这些故事不是虚构的。它们是对使用 Async Rust 的人们的真实体验的综合，这是通过访谈，博客文章和推文向我们报告的。为了给您一个想法，我们目前有两个示例：一个示例，其中[Grace必须调试她编写的自定义未来](https://rust-lang.github.io/wg-async-foundations/vision/status_quo/grace_deploys_her_service.html) ，而另一个示例中，Alan（来自GC语言的程序员）[遇到堆栈溢出并必须调试原因](https://rust-lang.github.io/wg-async-foundations/vision/status_quo/alan_runs_into_stack_trouble.html) 。

编写“现状”故事有助于我们弥补[知识的诅咒](https://en.wikipedia.org/wiki/Curse_of_knowledge) ：从事 Async Rust 工作的人们往往是 Async Rust 的专家。我们已经习惯了提高生产效率所需的[解决方法](https://github.com/rust-lang/async-book/tree/a927107bfe501a44dde1560a5942b1471c11c71d/src/07_workarounds) ，并且我们知道一些小技巧可以帮助您摆脱困境。这些故事可帮助我们评估所有剪纸对仍在学习中的人所产生的累积影响。这为我们提供了我们需要确定优先级的数据。

### 然后告诉我们我们将如何对其进行更改

当然，愿景文档的最终目标不仅是告诉我们我们现在在哪里，而且还要告诉我们我们要去往何处以及如何到达那里。一旦我们在现状故事方面取得了良好进展，下一步将是开始集思广益地讨论[“光明的未来”](https://rust-lang.github.io/wg-async-foundations/vision/shiny_future.html) 的故事。

闪亮的未来故事讲述了异步世界在未来2或3年后会是什么样。通常，他们将重播与“现状”故事相同的场景，但结局会更好。例如，也许格蕾丝（Grace）可以使用调试工具，该工具能够诊断卡住的任务并告诉她阻止任务的未来类型，因此她不必遍历日志。也许编译器可以警告Alan有关可能的堆栈溢出的信息，或者（更好的是）我们可以调整设计以select首先避免出现此问题。这个想法是雄心勃勃的，并且首先将重点放在我们要创建的用户体验上；我们将找出整个过程中的步骤（如果需要的话，还可以调整目标）。

### 让整个社区参与

异步愿景文档提供了一个论坛，在该论坛上，Async Rust 社区可以为 Async Rust 用户规划出色的整体体验。Async Rust 的设计初衷是不具有“一刀切”的思维方式，我们也不想改变这种状况。我们的目标是为端到端体验建立一个共同的愿景，同时保留我们已建立的松散耦合，面向探索的生态系统。

我们用于编写愿景文档的过程鼓励积极协作和“积极的总和”思考。它从集思广益期开始，在此期间，我们旨在收集尽可能多的“现状”和“光明的未来”故事。这个头脑风暴期持续了六个星期，直到四月底。在前两个星期（直到2021-04-02），我们仅收集“现状”故事。之后，我们将接受“现状”和“光明的未来”这两个故事，直到头脑风暴期结束为止。最后，帽从头脑风暴时期，我们将选择优胜者奖项，如“最幽默的故事”或“必须扶持贡献者”。

头脑风暴期结束后，工作组负责人将开始着手将各种故事和光明的未来汇编成一个连贯的草案。该草案将由社区和 Rust 团队进行审查，并根据反馈进行调整。

### 想帮忙？

如果您想帮助我们编写愿景文档，我们很乐意为您贡献自己的经验和愿景！目前，我们专注于创建现状故事。我们正在寻找人们撰写 PR 或谈论他们在问题或其他方面的经验。如果您想开始使用，请查看有关[现状故事的模板](https://rust-lang.github.io/wg-async-foundations/vision/status_quo/template.html) -它具有打开 PR 所需的所有信息。另外，您可以查看[“如何实现愿景”](https://rust-lang.github.io/wg-async-foundations/vision/how_to_vision.html) 页面，其中详细介绍了整个愿景文档过程。

原文链接：[https://blog.rust-lang.org/2021/03/18/async-vision-doc.html](https://blog.rust-lang.org/2021/03/18/async-vision-doc.html)

译者：
- [NiZerin](https://github.com/NiZerin)
