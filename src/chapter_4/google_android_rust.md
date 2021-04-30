---
pub_date: Thu, 30 Apr 2021 18:00:00 GMT
description: Rust in Android Platform

---

# Google | 在 Android 平台使用 Rust

翻译：budshome / 编辑：张汉东

> 原文：[https://security.googleblog.com/2021/04/rust-in-android-platform.html](https://security.googleblog.com/2021/04/rust-in-android-platform.html)
>
> 译文转载自：[https://blog.budshome.com/budshome/android-tuan-dui-xuan-bu-android-kai-yuan-xiang-mu-(aosp),yi-zhi-chi-rust-yu-yan-lai-kai-fa-android-xi-tong-ben-shen](https://blog.budshome.com/budshome/android-tuan-dui-xuan-bu-android-kai-yuan-xiang-mu-(aosp),yi-zhi-chi-rust-yu-yan-lai-kai-fa-android-xi-tong-ben-shen)

---


> 对于 Android 来说，如果代码是用 C/C++ 编写的，并且在解析不可信的输入，那么它应该包含在一个严格受约束和特殊的沙箱中。但沙盒的开销昂贵：需要引入新进程，消耗额外的开销，并且由于 IPC 和额外的内存使用，而引入了延迟机制。沙箱技术，并不能消除代码中的漏洞，它的效率，会随着高 bug 密度而降低，从而允许攻击者将多个漏洞链接在一起。像 Rust 这样的内存安全语言，通过两种方式帮助 Android 克服这些限制：降低了代码中 bug 的密度，从而提高了当前沙盒的有效性；减少了 Android 的沙箱技术需求，允许引入更安全、资源更轻的新功能。

Android 平台中，代码的正确性，是每个版本 Android 系统的安全性、稳定性，及其质量的重中之重。C/C++ 语言中的内存安全漏洞，仍然是最难解决的错误来源。我们投入了大量的精力和资源来检测、修复和缓解这类 bug，这些努力有效地防止了大量 bug 进入 Android 系统。然而，尽管做出了这些努力，内存安全漏洞仍然是稳定性问题的主要原因。并且，在 Android 系统高严重性的安全漏洞中，其始终占据[大约 70% 的比例](https://security.googleblog.com/2021/01/data-driven-security-hardening-in.html)。

除了[正在进行的](https://android-developers.googleblog.com/2020/02/detecting-memory-corruption-bugs-with-hwasan.html)和[即将进行的](https://security.googleblog.com/2019/08/adopting-arm-memory-tagging-extension.html)改进内存错误检测的工作之外。首当其冲地，我们正在加大力度防止它们。内存安全类编程语言，是防止内存错误的最经济有效方法。除了像 Kotlin 和 Java 这样的内存安全语言之外，我们很高兴地宣布：Android 开源项目（AOSP），现在支持 Rust 编程语言来开发 Android 操作系统本身。

## 系统级编程

Java 和 Kotlin 等受监管/托管类语言，是 Android 应用开发的最佳选择。这些语言是为易于使用、可移植性，以及安全性而设计的。[Android 运行时（ART）](https://source.android.com/devices/tech/dalvik)，代表开发者管理内存。Android 操作系统广泛使用 Java，有效地保护了大部分 Android 平台不受内存缺陷的影响。不幸的是，对于操作系统的底层，Java 和 Kotlin 不是一个选项。

较低级别的操作系统，需要系统级编程语言，如 C、C++，以及 Rust。这些语言的设计目标是控制性和可预测性。它们提供对底层系统资源和硬件资源的访问。它们占用资源较少，并且具有更可预测的性能特征。

对于 C/C++，开发人员需要负责管理内存生命周期。不幸的是，这样做很容易出错，特别是在复杂的多线程代码中。

## 沙箱技术（sandboxing）的极限

C/C++ 语言，不提供相同的安全保证，需要强大的手动隔离。所有 Android 进程，都是基于沙箱技术（sandboxing）的，我们遵循 [规则 2](https://chromium.googlesource.com/chromium/src/+/master/docs/security/rule-of-2.md)（译注：是指 Android 开发中关于 App 沙箱的规则限制，下同），以决定功能是否需要额外的隔离和剥离。规则 2 很简单：给定三个选项，开发人员只能选择三个选项中的两个。

对于 Android 来说，这意味着：如果代码是用 C/C++ 编写的，并且在解析不可信的输入，那么它应该包含在一个严格受约束和特殊的沙箱中。虽然[遵守规则 2](https://android-developers.googleblog.com/2019/05/queue-hardening-enhancements.html)，在降低安全漏洞的严重性和可访问性方面是有效的，但它确实有局限性。沙盒的开销昂贵：[需要引入新进程，消耗额外的开销，并且由于 IPC 和额外的内存使用，而引入了延迟机制](https://www.usenix.org/conference/enigma2021/presentation/palmer)。沙箱技术，并不能消除代码中的漏洞，它的效率，会随着高 bug 密度而降低，从而允许攻击者将多个漏洞链接在一起。

像 Rust 这样的内存安全语言，通过两种方式帮助我们克服这些限制：

- 降低了代码中 bug 的密度，从而提高了当前沙盒的有效性。
- 减少了我们的沙箱技术需求，允许引入更安全、资源更轻的新功能。

## 那么，现有的 C++ 呢？

当然，引入一种新的编程语言，并不能解决现有 C/C++ 代码中的问题。即使我们重新调整了 Android 团队中每个软件工程师的工作方向，重写数千万行代码，也是很难解决的。

上文中，对 Android 平台中内存安全漏洞的历史分析（从它们第一次引入时，就已经测量过），表明了为什么我们的内存安全语言工作，最关注的是新开发，而不是重写成熟的 C/C++ 代码。我们的大多数内存错误都发生在新的，或最近修改的代码中，大约 50% 的错误发生在不到一年的时间里。

比较稀疏的老旧内存错误，可能会让一些人感到惊讶，但我们发现旧代码并不是我们最迫切需要改进的地方。随着时间的推移，软件缺陷会被发现并修复，因此我们预计正在维护的，但未积极开发的代码中，其缺陷数量会随着时间的推移而减少。正如减少 bug 的数量和密度，可以提高沙盒的有效性一样，它也可以提高 bug 检测的有效性。

## 检测的局限性

通过健壮的测试、[清理（sanitization）](https://github.com/rust-lang/rust/pull/81506)，以及[模糊测试（fuzzing ）](https://android-review.googlesource.com/c/platform/build/soong/+/1403607/)，进行 bug 检测，对于提高所有软件（包括用 Rust 编写的软件）的质量和正确性至关重要。最有效的内存安全检测技术，其一个关键限制是：为了检测到错误状态，必须在代码中实际触发错误状态。即使在具有出色的 test/fuzz 覆盖的代码库中，这也会导致许多错误未被发现。

另一个限制是，[bug 检测比 bug 修复扩展得更快](https://lore.kernel.org/dri-devel/20200710103910.GD1203263@kroah.com/)。在一些项目中，检测到的 bug 并不总是得到修复。错误修复是一个漫长而昂贵的过程。

这些步骤都很昂贵，缺少其中任何一个，都可能导致某些或所有用户无法对 bug 进行调度。对于复杂的 C/C++ 代码库，通常只有少数人能够开发和检查修复，即使花费大量的精力来修复错误，[有时修复后也不完全正确](https://googleprojectzero.blogspot.com/2015/09/stagefrightened.html)。

当错误相对较少时，bug 检测最有效，并且可以给予它们紧急性和优先级。我们从改进 bug 检测中，获益的能力要求我们优先考虑：防止引入新的 bug。

## 优先性任务
Rust 对一系列语言特性，进行了现代化的设计和开发，从而提高了代码的正确性：

- 内存安全——通过编译器和运行时检查的组合，以强制执行内存安全。
- 数据并行——防止数据争用。这使得开发者能够轻松地编写高效、线程安全的代码，这也催生了 “[Rust 无畏并行（Fearless Concurrency）](https://doc.rust-lang.org/book/ch16-00-concurrency.html)”的口号。
- 更具表现力的类型系统——有助于防止逻辑编程错误（例如：newtype 包装、包含内容的枚举变量等）。
- 默认情况下，引用和变量在是不可变的——帮助开发人员遵循最小特权的安全原则，仅当他们真正希望引用或变量可变时，才将其标记为可变。尽管 C++ 有一定的特点，但它往往不经常使用，也不一致。相比之下，Rust 编译器通过为从不突变的可变值提供警告，来帮助避免不必要的可变注释。
- 在标准库中，有更好的错误处理方式——在结果中，包装可能失败的调用，这会导致编译器要求用户检查失败原因，甚至是没有返回所需值的函数。这可以防止诸如 [Rage Against the Cage](https://android.googlesource.com/platform/system/core/+/44db990d3a4ce0edbdd16fa7ac20693ef601b723%5E%21/) 漏洞之类的 bug，该漏洞即是由未处理的错误导致的。
- 初始化赋值——要求在使用前，初始化所有变量。未初始化的内存漏洞一直是 Android 平台上 3-5% 比例的安全漏洞的根本原因。在 Android 11 中，我们开始在 [C/C++ 中自动初始化内存](https://security.googleblog.com/2020/06/system-hardening-in-android-11.html)，以减少这个问题。但是，初始化为零并不总是安全的，特别是对于返回值这样的情况，这可能成为错误处理的新来源。Rust 要求每个变量在使用前，都初始化为其类型的合法成员，避免了无意中初始化为不安全值的问题。类似于 C/C++ 的编译器 Clang，Rust 编译器知道初始化要求，并且避免了多次初始化的任何潜在性能开销。
- 更安全的整数处理——默认情况下，对 Rust 调试和构建，启用溢位清理（overflow sanitization），鼓励程序员指定一个 `wrapping_add`（如果他们真的希望溢位计算），或 `saturating_add`（如果他们不希望溢位计算）。我们打算为 Android 平台中的所有构建，都启用溢位清理。此外，所有整数类型转换，都是显式强制转换：当分配给变量或尝试对其他类型执行算术运算时，开发人员不能在函数调用期间，意外地强制转换。

## 未来计划

为 Android 平台添加一种新的编程语言，是一项艰巨的任务。有需要维护的工具链，以及依赖项。也必须有更新的测试基础设施和工具，以及需要培训的开发人员。在过去的 18 个月里，我们一直在为 Android 开源项目添加 Rust 支持。我们有几个早期采用者项目，我们将在未来几个月内分享。将其扩展到更多的操作系统，是一个多年的项目。请继续关注，我们将在这个博客上发布更多更新。
