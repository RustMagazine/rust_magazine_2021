# 如何为 Rust 语言做贡献 | Part 1

作者：CrLF0710（野喵）/ 后期编辑：张汉东

---

## 引文

> 如果你想成为 Rust 贡献者，那看这系列文章会很有帮助。

本系列文章主要是给大家介绍一下如何为 `Rust Project` （即 Rust 语言本身）做贡献。

随着时间的推移，`Rust Project` 也在不断的演化，本文有效范围仅限于当前发表的时间点（2021.02）。

接下来就随我一起熟悉 Rust Project 吧。

## 熟悉 Rust Project

简单来说 `Rust Project` 的主要目标就是设计、开发、维护Rust这门编程语言。

`Rust Project` 主要由下列三部分构成：

1. 第一部分是现有的技术积累，包括设计文档、代码仓库、文档教程和技术讨论的积淀。
2. 第二部分是 Rust 的项目组织及其延伸，包括整个 Rust 开发者社区。
3. 第三部分是 Rust 的配套资产（如 CI、服务器、域名，乃至于商标）和会议活动等等。

## 熟悉 Rust 代码仓库

Rust 语言的设计文档、代码仓库、文档教程都是存储在Github上的rust-lang这个组织下的。其中[rust-lang/rust](https://github.com/rust-lang/rust)这个仓库是主入口。

感兴趣的话，我们可以用git来直接下载一份下来。注意它是使用了`git submodule`的，相关联的仓库也都是需要的。

代码仓库大概分成六部分:

- 编译器源码：位于`compiler/`目录下，由五十多个crate构成。另外还有它会用到的llvm，位于src/llvm-project目录下。
- 内置库源码：位于`library/`目录下，有十几个crate。我们平时会使用的core, alloc, std, test这些都在其中。
- 其他开发工具：位于`src/librustdoc/`, `src/tools/`目录下，包括我们平时使用的rustdoc, cargo, miri, clippy 等等工具
- 文档书架：位于`src/doc/`目录下，包括官方的the book, reference, nomicon等等的教程和参考文档。
- 测试用例集：位于`src/test/`目录下，大部分是编译器的测试用例，也有少量一些rustdoc和其他工具的测试用例。
- 部署工具和CI脚本：位于`src/bootstrap`, `src/build_helper`, `src/ci`, `.github/` 这几个地方，这些是用来自动化编译一套完整的rust工具链的。

## 编译一套 Rust 工具链

下载好了`rust源码`之后，我们来试着自己编译一份`rust工具链`吧！

首先要在你的机器上准备这些东西：`python3`, `ninja`, `cmake`，还有一套`c++`编译器(`g++`或者`windows`下用`visual studio`)。第一个是用来执行编译脚本的，后两个则是用来编译`llvm`的。

准备好了之后，把`rust`目录里的`config.toml.example`拷贝一份，名叫`config.toml`。其中大部分内容都不用修改，但是我建议可以把增量编译启用，就是找到其中的`#incremental = false`这一行，去掉前面的`#`并且把后面的false改成true。

其他配置选项参考如下，具体作用在配置文件中有注释说明：

```rust
compiler-docs = false
submodules = false
configure-args = []
debug = true
codegen-units = 0
default-linker = "cc"
channel = "nightly"
```

**构建Rust的三个阶段：**

Rust 是⼀个⾃举的编译器，需要通过旧的编译器来构建最新的版本。所以⼀般是分阶段来完成：

1. `Stage0` 阶段。下载最新`beta`版的编译器，这些`x.py`会⾃动完成。你也可以通过修改配置⽂件来使⽤其他版本的Rust。
2. `Stage1` 阶段，使⽤`Stage0`阶段下载的`beta`版编译器来编译从`Git`仓库⾥下载的代码。最终⽣成`Stage1`版编译器。但是为了对其优化，还需要进⾏下⼀阶段。
3. `Stage2`，⽤`Stage1`版编译器继续对源码进⾏编译，以便⽣成Stage2版编译器。

理论上，`Stage1`和`Stage2`编译器在功能上是相同的，但实际上还有些细微的差别。

官⽅推荐的具体构建流程如下：

1. `./x.py check` ，先执⾏此命令，检查编译器是否可以构建。
2. `./x.py build -i --stage 1` ，进⾏`Stage 0`和`Stage 1`阶段的构建，最终构建完成Stage1的编译器。
3. `./x.py build --stage 2 compiler/rustc`，在`Stage1`基础上进⾏增量式构建，最终编译出`Stage2`的编译器。

整个过程是有点慢的，不考虑一开始的下载部分，编译时间随你的硬件配置不等，一般在20到60分钟左右。其中大约有一半的时间是在编译`llvm`。好在`llvm`只要编译一次，后续如果没有版本变化是不需要重新编译的。(`config.toml`里有个选项在版本变化的时候也不重新编译`llvm`)另外记得硬盘剩余空间要保证`30G`以上哦。

然后将其加到Rustup⼯具链中:

```rust
// your-target-tripe 类似：aarch64-apple-darwin/x86_64-apple-darwin 等。
> rustup toolchain link stage2 build/{your-target-tripe}/stage2
```

到此为⽌，准备⼯作就已经做好了。

对这个话题感兴趣的可以继续读读官方准备的书籍[Guide to Rustc Development](https://rustc-dev-guide.rust-lang.org/)，里面有更多的讲解。这本书中文社区也在组织翻译[Guide to Rustc Development 中文版](https://github.com/RustcRustc/rustc-dev-guide-zh)，欢迎大家参与。

## 一起成为 Rust Contributor 吧

接下来，让我们试着为 Rust 项目来做点事情。`Rust Project`是非常欢迎大家参与的，参与的门槛是非常的低。

对于想参与贡献的新手来说，可以从比较轻松的任务做起。由此，我来试着难度从低到高列出一些比较适合新手来做的事情。

### No.1 改进标准库文档

Rust 的每个标准库函数都在旁边有`markdown`语法的文档描述。对这一部分的调整改进是门槛最低的。可以多读读标准库的文档，顺便检查每个条目(item)和关联条目的文档描述是否足够的清晰。（特别是标注着`Experimental`的那些，往往会存在改进空间。）对于没有示例(`Example`)的部分，可以补充示例。对于标注了`unsafe`关键字的部分，可以检查下安全性(`Safety`)一节是否清晰的描述了使用时的约束条件。

### No.2 改进语言参考手册

Rust 有一个相对冷门的资源叫[The Rust Language Reference](https://github.com/rust-lang/reference)，是语言的规格说明的雏形，实际上能做的事情相当多。但是因为人手有限，进度不是很快。对于新手，有很多参加编辑性修改的机会。实质性修改门槛会稍微高一点，需要对语言有比较全面深刻的了解。但是因为是有老手帮助review，对新人来说也是不错的提升自己的机会。缺点是`review`周期可能会相对较长。

### No.3 重构、清理、增加测试用例类任务

Rust里很多地方都有小型的重构、清理任务（而且很多都是故意留给新人练习的），包括`rustc`,`rustdoc`,`cargo`,`chalk`,`polonius`之类的地方都会有。可以多关注一下`E-easy`,`E-mentor`,`E-needs-test`这些标签下的问题条目，也不要忘了多去逛逛`cargo`,`chalk`等等的单独仓库。

### No.4 完善编译器的诊断和代码质量检测

在编译器这一侧，最适合初学者学习的工作有两项，一个是诊断(`diagnostics`)，负责编译报错信息的完善，尽可能推断出用户的原本意图，并给出更好的错误提示。另一个就是代码质量检测(`lint`)。代码质量检测检查的是代码中那些不违反基本规则的那些写法，它们是可配置的，编译器可以配置为允许，警告，拒绝和严禁的形式进行响应。[Guide to Rustc Development中有专门的一节进行讲解](https://rustc-dev-guide.rust-lang.org/diagnostics.html)，可做的事情也是非常多的。对于一些非常具体情况的检测和反馈，也可以放到`clippy`这个专门的检测工具中。可以多关注一下`A-Diagnostics`, `A-suggestion-diagnostics`, `A-lint`这些标签下的问题条目，以及`clippy`仓库中的问题条目。

## Rust PR 流程：从提交到合并

要提交修改只要在`GitHub`上 fork 官方的`rust`仓库，把修改提交到自己的fork仓库里，然后建一个PR(Pull Request)就可以了。

接下来我来试着讲讲提交之后会发生的事情。感兴趣可以了解下，不感兴趣也可以跳过。

**PR CI 阶段**

官方`rust`仓库有好几个自动交互机器人。我们首先会接触到的是一个叫`rust-highfive`的机器人。它负责欢迎新人，并且如果你的 PR 里没写由谁来`review`的话(格式是`r? @XXX`)，它会自动把我们的`PR`随机分配给它觉得合适的人来`review`。分配的方法是它会看你修改了仓库里哪些文件，然后在相应的负责人员列表里随机分配。并且给你的 PR 加上一个`S-waiting-for-review`的标签，表示正在等待`review`的状态。同时 PR CI 会开始运行，如果你的修改有格式问题(没有执行`rustfmt`之类的)、编译或者单元测试不通过，就会被 PR CI 拦下来，告诉你编译失败。你可以继续调整。

**官方 Reviewer 审阅**

接下来几天之内往往就会有官方 Reviewer 来审阅我们的修改了。Reviewer 都是官方某个团队的正式成员。因为 PR 都是公开的，在这期间，其他成员、社区爱好者也有可能会帮忙审阅你的代码，帮我们提出修改意见之类的。Reviewer 看了之后也可能要求我们修改。他们会把 PR 状态改成`S-waiting-for-author`。还有一种情况是这段时间里代码更新导致了合并冲突。机器人会来留言告诉你有合并冲突。这个时候你需要执行一个`git`的`rebase`操作，完成对合并冲突的解决，然后更新你的 PR 分支。

很多 PR 会在这一阶段停留一段时间，官方有一个小的分类处理工作组(`T-release`/`WG-triage`)，会定期来检查各个 PR 的状态。对于等待作者处理的 PR，15 天左右会留言确认状态；如果 30 天左右没有响应，会留言并关闭 PR。对于等待`review`的 PR，会在 15 天左右整理成报告，部分会通知 reviewer 确认审阅进度。

**PR 合并**

Reviewer 觉得你的提交`ok`了之后就会进入下一阶段了。Reviewer 会给另一个名叫`bors`的机器人发指令标识审阅通过(`@bors r+`)。这个命令有俩参数，一个是优先级(`p`)，优先级高的会在排在队列靠前的位置。一个是是否参与批量合并(`rollup`)。如果你的贡献足够微小，Reviewer 会把`rollup`设置为`always`，永不参与单独测试合并。相反如果你的贡献可能会带来编译性能影响之类的，Reviewer 会把`rollup`设置为`never`，永不参与批量测试合并，这样万一以后需要`revert`的话会比较方便。

接下来就是测试合并阶段了。`Bors`机器人管理着一个[PR队列](https://bors.rust-lang.org/queue/rust)。`Bors`机器人会按照队列的顺序一次一个 PR 来**先合并**，再测试，通过后推送远端分支并更新关闭相应的 PR。对于那些`rollup=always`的 PR，`bors`是不会合并的。官方的一些成员会轮流负责`Rollup`工作，每次控制`Bors`机器人来产生一个`8~12`个 PR 构成的一个高优先级的批量合并的 PR 加到队列里，由`bors`来测试合并。

## 小结

这次我们从一个开发者的视角，了解了参与rust项目所需要的一些基本知识和切入点，下一次我们会介绍一下项目组的总体结构以及如何参与一些更大型的工作。到时见！

---

作者介绍：

CrLF0710，C++程序员/ Rust业余爱好者/ Rust Team版本发布团队分类处理工作组（负责参与 Rust Project 的issues 和 PR 分类管理）成员。

业余时间写些`Rust`代码，也对`rustc`, `cargo`, `chalk`, `rustup`, `rustbook`等都做过一些代码贡献。偶尔在知乎Rust主题专栏[《学一点Rust又不会怀孕》](https://www.zhihu.com/column/rust-quickstart)上写一些文章。