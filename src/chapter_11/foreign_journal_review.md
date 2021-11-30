# 外刊评论 ｜ 知名项目进展报告

编辑：张汉东 

> 编者按：国外的 Rust 资源非常丰富，将其都翻译过来也不现实，所以想到这样一个栏目。外刊评论，主要是收集优秀的 Rust 相关文章，取其精华，浓缩为一篇简单评论。
>
> 欢迎大家贡献：[https://github.com/RustMagazine/rust_magazine_2021/discussions/129](

---

## 从语言架构的角度来看，Rust 最不适合哪些场景？

来自于 Reddit 的一篇帖子： [What is Rust NOT suitable for, architecturally?](https://www.reddit.com/r/rust/comments/r4dx1t/what_is_rust_not_suitable_for_architecturally/) 

这里引用来自用户 [`Prokopyl`](https://www.reddit.com/user/Prokopyl/) 的观点，大多数人也比较赞同。

与所有编程语言（不仅仅是像 C/C++ 这样的系统编程语言）相比，Rust 最大的设计方向是：

- 对系统细节的底层控制能力。引用、内存分配和布局、线程/异步等等（在嵌入式领域更是如此）。
- 非常高的性能（或低延迟）。大约 <1 毫秒，每秒数千或数百万次处理（无论您的情况是什么“处理”）。还严格控制内存使用。
- 非常注重正确性，尤其是在错误管理方面。换句话说，就是 “失败不是一种选择”。

当你的需求离这个设计方向越远，Rust 越会妨碍你的生产力，它就越不适合你的任务：

- 如果你不关心你的数据在内存中的布局，你的数据结构（例如 OOP 中的对象/类）只是为了代码组织和架构，那么 Rust 语言和标准库的很大一部分只是充其量看起来很奇怪或愚蠢，最糟糕的是令人讨厌。
- Rust 的错误管理绝对是一流的，但前提是你需要它，否则就很痛苦。当你的程序在发生不好的情况下如果只需要对用户说一句“抱歉！”就行时，也许异常处理更适合你。
- 性能很大程度上应该看情况：
  - 如果你有一个“需要每秒生成 TB 数据”的需求，那么性能对你来说几乎就成了唯一需要考虑的问题。
  - 如果你的程序变得更快可以让你的用户更加 Happy， 那你应该考虑性能。比如 检索更快、音视频处理更快、图形渲染更快等。
  - 如果你的用户或者你认为你的用户只是抱着“又不是不能用”的想法，那性能就无所谓了。
  - Rust 的性能设计几乎是 “要想性能更佳，则需要在问题点上投入更多的工程（engineering）”。所以，这取决于你对于 工程投入的成本 ，如果你投入更多机器的成本低于工程投入的成本，那就没必要用 Rust。
  - 如果你是 “真的不需要*那么多*原始性能（或者负担不起工程）” ，那你选择带 GC 的语言就可以了。

另外一个用户[`TheWaterOnFire`](https://www.reddit.com/user/TheWaterOnFire/)的观点也发人深省：

> 生产力是一个相对的尺度。例如，在$job，我刚刚修复了一个已经生产了两年的Go服务中的并发错误。Go中甚至都没有警告过这个问题，但是在Rust中写这个bug会让人更讨厌。它可以促使开发者采用一种可以防止这种事情发生的设计。
>
> 使用Go会更有成效吗？毕竟它已经 "完成 "了，而且多年来没有人触发这个bug！但是现在，我在我的工作中失去了生产力，因为我不得不重新审视那些 "已经完成 "的代码。
>
> **所以我想说，Rust 在让没有经验的开发者快速提供 "足够好 "的软件方面 并不出色。**





## 你觉得Rust目前还缺乏哪些成熟的开源库？

来自于 Reddit 的一篇帖子 ： [What sort of mature, open-source libraries do you feel Rust should have but currently lacks?](https://www.reddit.com/r/rust/comments/qkt2j7/what_sort_of_mature_opensource_libraries_do_you/)

总结一下帖子下提到的呼声比较高的库：

1. 类似 Cuda 的GPU/GPGPU库；
2. 图片处理相关的库，支持WebP、XBM、XPM等格式；支持类似python中Pillow库功能的图片处理库；
3. 图像处理库；
4. 纯Rust实现的解压缩算法库，支持LZMA、BZip2等格式及其衍生格式；
5. 文档处理库，支持PDF、ODF、OOXML、EPUB、MOBI等格式的解析和提取操作；
6. 检验和相关的库；
7. Qt相关的生成和绑定库；
8. 完全支持XML格式文档的读写操作的库；
9. LLVM高级绑定的库；
10. 机器学习相关的库；

更多想法和细节欢迎围观该[讨论帖](https://www.reddit.com/r/rust/comments/qkt2j7/what_sort_of_mature_opensource_libraries_do_you/)。

## Rust适合做科学计算吗？

来自于 Reddit 的一篇帖子 ： [Is rust good for mathematical computing?](https://www.reddit.com/r/rust/comments/qv5i0n/is_rust_good_for_mathematical_computing/)

科学计算相关的任务：

- 矩阵计算
- 数值分析

对编程语言的要求：

- 对lambda的良好支持；
- 简单易用的函数组合；

Rust语言本身对lambda(在Rust中称为闭包)、通用组合和函数式编程有很好的支持，但像柯里化(curry)这样的东西看起来并不漂亮。

目前Rust语言实现的跟科学计算相关的crate列表，可以在此查看：[Scientific Computing](https://www.arewelearningyet.com/scientific-computing/)
