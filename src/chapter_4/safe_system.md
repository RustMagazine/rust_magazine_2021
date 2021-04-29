# 论文鉴赏 | 使用 Rust 进行安全系统编程

译者： [MATRIXKOO](https://github.com/MATRIXKOO)
原文： https://cacm.acm.org/magazines/2021/4/251364-safe-systems-programming-in-rust/fulltext#FNA

> 关键观点

- Rust 是第一个受行业支持的在高级语言带来的安全性和由较低级别的`系统编程`语言提供的资源管控之间做出长远平衡的语言。 

- 它以强大的类型系统解决了这一挑战，这种基于所有权和原则的类型系统静态地防止了共享状态的改变。这种方法使许多常见的系统编程的漏洞在编译时被检测到。 
- 有许多数据类型的实现从根本上取决于处于共享的可变状态，因此不能依靠 Rust 的严格要求进行类型检查所有权。为了支持这样的数据类型，Rust  明智的拥抱 `Unsafe`,  使用 `safe` 的 API 封装在其中的`Unsafe` 代码。 
- 作为`RustBelt project` 的一部分， 健壮的 `semantic type` 证明技术，以及先进的 `separation logic` 和机器证明技术使得我们能够为 Rust 建立严格可靠的基础。

> 译者注: `RustBelt project`: https://plv.mpi-sws.org/rustbelt/#project
>
> 一个致力于用形式化证明来保证 Rust 安全性的项目

-----



在编程语言设计中一直存在着两个看似不可调和的令人绝望的问题。

 - 安全。 我们想要强类型系统来排除静态地消除大量错误。 我们要自动内存管理。我们想要数据封装， 这样我们就可以对私有变量执行不变的对象的表示形式，并确保它们将不会被不受信任的代码破坏。 
 - 控制。 至少对于 Web浏览器，操作系统，或游戏引擎这样的 `系统编程` 程序，约束它们性能或资源是一个重要的问题，我们想了解数据的字节级表示。 我们想要用`低级语言` 的编程技术优化我们程序的时间和空间的使用。 我们希望在需要时使用 `裸机` 。


然而，按照传统的看法，鱼和熊掌不能兼得。 Java 之类的语言使我们极大的安全保证，但代价是牺牲对底层的控制。结果，对于许多系统编程应用程序，唯一现实的选择是使用一种像 C 或 C++ 提供细粒度的语言控制资源管理。 但是，获得这种控制需要很高的成本。例如，微软最近报告说，他们修复的 70％ 安全漏洞都归因于内存安全违规行为 [33](#33)，并且都是能被强类型系统排除的问题。同样，Mozilla 报告指出，绝大多数关键 他们在Firefox中发现的错误是内存有关的[16 ](#16)。如果可以以某种方式两全其美: 安全系统编程的同时对底层有控制权...

键入 **Rust** 。由Mozilla赞助，并在过去十年中由众多不同的贡献者社区积极开发，Rust 支持许多从现代 C ++ 派生的常见的底层编程习惯用法和 API 。 但是，与C ++不同，Rust 通过强大的静态类型系统来强制安全地使用这些 API 。 

特别的，像 Java 一样，Rust 可以保护程序员免受内存安全性问题的侵害（例如，``use-after-free``的错误）。 但是 Rust 更进一步地捍卫程序员免受其他主流语言无法阻止的其他更隐蔽的异常现象的影响。 例如，考虑*数据竞争*：对共享内存的不同步访问（至少其中之一是写操作）。 即使数据争用有效地构成了并发代码的未定义（或弱定义）行为，大多数`安全`语言（例如 Java 和 Go ）都允许它们的存在，并且它们是并发错误的可靠来源[35](#35)。与之不同的是，Rust 的类型系统在编译时就能探测到数据竞争。

 Rust 一直在稳步普及，以至于现在许多主要的工业软件供应商（例如 Dropbox ，Facebook ，Amazon和 `Cloudflare` ）都在内部使用 Rust，并且 Rust 在 过去五年的 Stack Overflow 的``最受欢迎的``编程语言列表中排名第一。 微软的安全响应中心团队最近宣布，它正在积极探索对 Microsoft 使用 Rust 可能性，以减少系统软件中的安全漏洞。 [8](#8),[25](#25)

Rust 的设计深深地吸取了关于安全系统编程的学术研究的精髓。特别是，与其他主流语言相比，Rust 设计的最大特色在于采用了*所有权类型系统*（在学术文献中通常称为*仿射*或*子结构*类型系统[36](#36)）。所有权类型系统通过限制对象的应用， 可用于在程序执行的任何给定点进行变异来帮助程序员实施较低级编程的安全模式。 但是，Rust 至少用两种新颖而令人兴奋的方式超越了先前工作的所有权类型系统：

1. Rust采用了`借用`和`生存期`的机制，这使得表达常见的C ++风格的习惯用法变得容易得多，并确保了它们的安全使用。 
2. Rust还提供了丰富的 API 集（例如，用于并发抽象，高效的数据结构和内存管理），它们通过支持比 Rust 的核心类型系统更灵活的别名和突变组合，从根本上扩展了语言的功能。相应地，这些 API 不能在 Rust 的安全片段内实现：相反，它们在内部使用了该语言潜在的 `Unsafe` C 风格的特性，是以一种声称不会干扰 Rust 的语言级别安全性保证的方式进行安全封装。 

Rust 的设计的这些方面不仅对其成功至关重要，它们还提出了有关其语义和安全性保证的基本研究问题，从而使编程语言社区才刚刚开始探索。 在本文中，我们首先为读者提供了Rust 编程语言的鸟瞰图，重点介绍了 Rust 的一些基本功能，这些功能使它与同时代产品脱颖而出。其次，我们描述了 `RustBelt`初步进展，该项目是由欧洲研究委员会（ERC）资助的正在进行中的项目，其目的是为 Rust 的安全性主张提供第一个正式的（并经过机器检查的）基础。通过这个项目，我们希望激发计算机科学研究领域的其他成员开始更加关注 Rust ，并为这种突破性语言的发展做出贡献。 



## 动机: C++ 中的无效指针

为了演示在系统编程语言中常见的那种内存安全问题，让我们考虑一下


 ![](https://dl.acm.org/cms/attachment/1f582f38-4371-4a65-af48-ac767ff53686/f1.jpg)

 **Figure 1. Use-after-free bug in C++ and how the bug is prevented in Rust.**



在第一行中，该程序创建一个整数的 `std::vector`（可增长数组）。 `v` 的初始内容（两个元素`10`和`11`）存储在内存中的缓冲区中。在第二行中，我们创建一个指向该缓冲区的指针 `vptr`。具体来说，它指向第二个元素（当前值为 `11` ）的存储位置。现在，`v` 和  `vptr` 都指向同一个缓冲区（重叠的部分）。我们说这两个指针是`混叠` 的。在第三行中，我们将新元素 `push` 到 `v` 的末尾。在缓冲区`v`中，将元素`12` 添加到`11`之后。如果没有更多空间可容纳其他元素，将分配一个新的缓冲区，并将所有现有元素移到上方。让我们假设这就是这里发生的情况。为什么这种情况很有趣？因为`vptr`仍然指向旧缓冲区。换句话说，在`v`中添加一个新元素已经将`vptr`变成了悬空的指针。这是可能的，因为两个指针都是引用：通过指针（`v`）进行的操作通常也会影响其所有引用（`vptr`）。 [图1]可视化了整个情况。

 `vptr` 现在是一个悬空指针，在第四行中存在一个问题。在这里，我们从`vptr` 解引用，并且由于它是一个悬空指针，所以这是一个`use-after-free`错误。 

实际上，这种问题很普遍，以至于它的一个实例拥有自己的名称：*迭代器无效*，这是指迭代器（通常在内部用指针实现）无效的情况，因为迭代所遍历的数据结构是可变的在迭代过程中。最常见的情况是在一个循环中遍历某个容器数据结构，然后间接但偶然地调用一个使数据结构发生突变的操作。注意，实际上，对改变数据结构的操作的调用（在我们的示例的第3行中为` push_back`）可能深深地嵌套在几层抽象的后面。特别是当重构代码或添加新功能时，通常几乎无法确定是否推送到某个 `vector`是否会使程序中其他位置的指针失效，不存在 `use-after-free`。 

**与垃圾回收语言进行比较。**诸如Java ，Go 和 OCaml 之类的语言可避免使用垃圾回收的使用后释放的错误：仅当程序无法使用内存时才释放内存。因此，不能有悬空的指针，也不没有`use-after-free`。 垃圾回收存在的问题是，为了提高效率，此类语言通常不允许*内部* 指针（即，指针放入数据结构）。例如，Java中的数组` int []` 的表示方式类似于 C++ 中的 `std::vector <int>`（Java中的数组不能增长）。但是，与C ++不同，一个Java数组只能`获取`和 `设置`元素，而不能对其进行`引用`。为了使元素本身可寻址，它们必须是单独的对象，然后可以将对它们的引用存储在数组中--也就是说，需要将元素`装箱`。为了安全，牺牲了性能和对存储器布局的控制。 最重要的是，垃圾回收甚至不能正确解决迭代器无效的问题。在Java中遍历集合时对其进行改变不会导致指针悬空，但可能会导致在运行时抛出` ConcurrentModificationException` 。类似地，尽管 Java 确实防止了由于空指针滥用引起的安全漏洞，但它通过引发 `NullPointerException` 的运行时检查来做到这一点。在这两种情况下，其结果显然都比 C++程序的相应未定义行为更好。这仍然有很多不足之处：我们希望从一开始(编译期)就防止错误发生，而不是交付不正确的代码然后在运行时才检测到问题。 

**Rust解决指针无效的方法**。在 Rust 中，编译器会静态检测到迭代器无效和空指针滥用等问题，它们会导致编译时错误，而不是运行时异常。为了解释它是如何工作的，请在[图1]底部参考我们C ++示例的 Rust转换。 像在 C++ 版本中一样，内存中有一个缓冲区，而 `vptr` 指向该缓冲区的中间（导致引用）； `push`可能会重新分配缓冲区，这导致 `vptr` 变成悬空的指针，并导致第4行中的 use-after-free。 

但是这一切都没有发生。相反，编译器显示错误：`一次不能多次借用可变的变量v` 我们将很快回到`借用`，但是关键思想（Rust 通过这种机制在存在指向数据结构的指针的情况下实现内存安全的机制）已经在这里变得显而易见：类型系统强制执行该规则（我们将在以后介绍）不会有多个可变引用。在并发的上下文中，这个原理听起来应该很熟悉，并且 Rust 确实使用这个方法来确保不存在数据竞争。但是，正如我们被 Rust 编译器拒绝的示例所示，引用和可变性的不受限制的组合即使对于顺序程序也是造成灾难的原因：在第3行中，`vptr`和`v`引用（`v`被认为是指向它的所有内容，与`vptr` 重叠），我们正在使用一个可变引用，这将导致第4行出现的内存访问错误。 



## 所有权和借用

Rust防止不可控制的引用的核心机制是*所有权*。 Rust 中的内存始终具有唯一的所有者，如示例2中所示。 

![](https://dl.acm.org/cms/attachment/4d8b8636-fbe9-4742-a47b-e29c462cf6d0/f2.jpg)

**Figure 2. Rust example: Moving ownership.**

在这里，我们构造与第一个示例类似的` v`，然后将其传递给` consume`。在操作上，就像在C ++中一样，参数是按值传递的，但是副本是浅复制的—指针被复制，但它们的指针不会重复。这意味着` v`和` w`指向内存中的相同缓冲区。 

如果程序同时使用 `v` 和 `w` 两者就会出现这个问题，但是在第6行尝试这样做的时候会导致编译时错误。这是因为 Rust 认为  `v` 的所有权作为调用的一部分已经移动到 `consume` 上，这意味着 `consume` 可以用 `w` 来做任何想要的事情，并且调用者可能不再访问这个 `Vec` 的内存。 

**资源管理。** Rust的所有权不仅可以防止内存错误，而且还构成了 Rust 的内存管理方法（更广泛地说，是资源管理的核心。当拥有内存的变量（例如，Vector 的内存中的缓冲区的 `Vec <T>`类型的变量）超出作用域的时候，我们可以确定不再需要该内存了，因此编译器可以在那时自动释放内存。为此，就像在C++中一样，编译器透明地插入*destructor* 调用。例如，在 `consume `函数中，实际上没有必要显式调用析构函数方法（`drop`）。我们可以将该函数的主体保留为空，并且它将自身自动释放` w`。 

因此，Rust程序员几乎不必担心内存管理：尽管缺少垃圾收集器，但它基本上是自动的。此外，内存管理也是 `静态的`（在编译时确定）这一事实产生了极大的好处：它不仅有助于降低最大的内存消耗，而且还可以在反应式系统中(例如 web 服务器 )提供良好的 `最坏情况` 的 `latency` 。最重要的是，Rust 的方法超出了内存管理的范围：文件描述符，套接字，锁，句柄等其他资源都使用相同的机制处理，因此Rust程序员不必担心关闭文件或释放锁。C++以 RAII（资源获取即初始化）的形式为使用析构函数进行自动资源管理的方式开了先河； [31](#31) Rust 中的主要区别在于类型系统可以静态地确保资源在销毁后不再使用。 

---



**可变的引用。**严格的所有权准则既令人愉悦又简单，但是使用起来并不方便。通常，人们想 *暂时* 地向某个函数提供数据，在函数允许之后后将其返回。例如，我们希望`v.push（12）` 赋予 `push` 来使 `v`改变的特权，但是我们不希望它`consume` vector `v`。 

在Rust 中，这是通过 *借用* 实现的，它从 *region types* 的先前工作中获得了很多启发。[13](#13),[34](#34)

 函数`add_ something`接受类型为 `＆mut Vec<i32>`的参数，该参数表示对`Vec<i32>`的 `可变引用` 。在操作上，这就像C ++中的引用一样，即 `Vec`通过引用传递。 在类型系统中，这被解释为向自调用者的`Vec`的`add_something` *借用* 所有权。 

[![f3.jpg](https://dl.acm.org/cms/attachment/8f380ed9-5373-457c-ad22-ca28804ed776/f3.jpg)](https://dl.acm.org/cms/attachment/8f380ed9-5373-457c-ad22-ca28804ed776/f3.jpg)

**Figure 3. Rust example: Mutable references.**

函数 ` add_something` 展示了格式化过的借用看起来是什么样子。 为了搞清楚为什么编译器在拒绝前面的指针无效示例，而这段代码却可以通过，我们必须引入另一个概念：*lifetimes*。 就像在现实生活中一样，借用某物时，可以通过事先就可以借用多久达成共识，来防止误解。 因此，当创建引用时，会为其分配一个生存期，并以完整的引用类型形式记录下来：`＆'a mut T`表示生存期` 'a`。编译器会确保引用（` v`， 在我们的示例中）仅在该生命周期内被使用，并且直到生命周期结束，引用对象才被再次使用。

在我们的例子中，生存期（全部由编译器推断）分别持续到 `add_something` 和 `Vec::push` 结束。 在之前借用的生命周期尚未结束之前，`v`不会被使用。

相比之下，[图4] 显示了从[图1]推断出的上一个示例的生命周期。`vptr` 借用生命周期的`'a` 从第2行开始，一直持续到第4行。`vptr` 因为在第4行中被使用了，因此不能变得更短了。这意味着在第3行中，`v`  使用了借出的 ，这是错误的。 

![f4.jpg](https://dl.acm.org/cms/attachment/e1a6ee8d-bc5e-458f-8b3d-7ade88032266/f4.jpg)

**Figure 4. Use-after-free example with inferred reference lifetime.**

总结一下：每当通过值传递某些东西时（如在 `consume` 中），Rust 会将其解释为所有权转移。 当某些变量引用传递时（如在 `add_something` 中），Rust 将此解释为在特定生命周期内借用。

----

**共享引用。**遵循我们可以可变引用不可共享的原则，可变引用是 `unique pointers`：它们不允许引用。 为了完善这种规则，Rust 提供了第二种引用，称为 `共享引用` ，写为`&Vec<i32>`或 `＆'a Vec <i32>` ，允许多个引用但不能改变。 共享引用的一种主要用例是在多个线程之间共享只读数据，如图5所示。

![f5.jpg](https://dl.acm.org/cms/attachment/7d2a2da7-ce35-4321-a112-3d4eb716939e/f5.jpg)

 **Figure 5. Rust example: Shared references.**

在这里，我们创建一个共享引用 `vptr` ，指向（并借用）` v [1]`。此处的竖线表示不带任何参数的 *closure*（有时也称为匿名函数或 ` lambda` ）。这些闭包被传递给 `join` ，这是 Rust 版本的 `并行组合`( `parallel composition` )：它需要两个闭包，并行地运行两个闭包，等待两个闭包完成，然后返回两个结果。当`join`返回时，借用结束，因此我们可以再次对`v` 进行修改。 

就像可变引用一样，共享引用也存在着生命周期。深入代码背后，Rust 的编译器使用生命周期来跟踪两个线程之间临时共享 ` v` 的时间。在生存期结束后（第5行），`v`的原始所有者重新获得了完全控制权。此处的主要区别在于，允许多个共享引用在同一生存期内共存，只要它们仅用于 `读取`  而不是用于 `写入` 即可。将示例中的两个线程之一更改为`||v.push(12)`，就可以看到这条规则的实现: 编译器会给出一条错误，告诉我们不能同时具有可变引用和对 `Vec` 的共享引用。的确，该程序在读取线程和` v.push(12)` 的线程之间存在致命的数据争用，因此，编译器能静态的检测到此类情况是很重要的。 

共享引用在顺序执行的代码中也很有用；例如，在对 `vec` 用 `shared iterator` 进行遍历时，我们仍然可以传一个 `vec` 共享引用到其他函数里。但是在本文中，我们将重点讨论为实现并发而使用共享引用。 

-----



**总结。**为了获得安全性，Rust 类型系统强制执行以下原则：共享不可变，可变不共享。拥有 `T` 类型的值意味着变量完全 `拥有` 它。可以用可变引用（`&mut T`）或共享引用（`&T`) 去对 `T` 进行引用。 



## 用 `safe` API 放宽 Rust 的严格所有权规则

Rust的核心所有权原则具有足够的灵活性，可以解决许多底层编程的习惯用法。但是对于实现某些数据结构而言，它可能过于严格。例如，引用态无法被更改，就不可能实现双向链接列表，因为每个节点都被其下一个和上一个节点引用 。

Rust 对这个问题采取了一种不寻常的方法。比起允许其类型系统复杂化以解决不遵守该数据类型的数据结构实现，或者引入动态检查来在运行时强制执行安全检查，Rust 允许通过开发 *`safe` 的API* 来放宽其所有权准则-API通过安全地控制引用的可变状态的使用来扩展语言的表达能力。尽管这些 API 的实现不遵循 Rust 严格的所有权原则（我们将在后面再讲），但 API 本身却严格利用了 Rust 的所有权和借用机制以确保它们整体上保持 Rust 一致的的安全性。现在让我们看几个例子。 

----

**共享可变状态**  Rust 的共享引用允许多个线程同时读取共享数据。 但是仅 *读取* 数据的线程只是事情的一半，接下来我们将看到 `Mutex` API 如何是实现跨线程安全地共享可变`mutable`状态。 乍一看，这似乎与我们到目前为止对 Rust 的安全性所说的一切相矛盾：Rust 的所有权准则要点不是要 `防止` 共享状态的改变吗？ 的确，但是我们将看到如何使用`Mutex` 充分限制这种改变，以免破坏内存或线程安全。 现在来看图6。

![f6.jpg](https://dl.acm.org/cms/attachment/a2dd9c19-b0b2-400b-bf41-0f73aa941100/f6.jpg)

 **Figure 6. Rust example: Shared mutable state.**

我们再次使用结构化并发和共享引用，但是现在将 `vec` 包装在 `Mutex` 中：变量 `mutex_v` 的类型为` Mutex<Vec<i32>>`。` Mutex` 的的关键操作是` lock`。它将一直阻塞直到获得独占锁为止。当变量超出范围时，锁将由 `v` 的析构函数隐式释放。最终，如果第一个线程设法先获取锁，则该程序将打印` [10,11,12]`，如果第二个线程则获取到了， 就会打印` [10，11]`。 

为了弄清楚示例程序的类型检查方式，让我们仔细研究一下`lock`。它（差不多（实际类型是 被`LockResult<…>` 包装起来的类型，这也是为什么使用了  `unwrap` ）的类型为 `fn（&'a Mutex <T>)-> MutexGuard <'a，T>` 可以使用对互斥锁的共享引用来调用，这就是 Rust 允许我们在两个线程上调用锁定的原因：两个闭包都捕获`&Mutex<Vec<i32>>`，并与`&i32`类型的 `vptr` 一样，在我们的第一个并发示例中，两个线程可以同时使用该引用。实际上，`lock` 获取一个共享而不是可变的引用是至关重要的；否则，两个线程将无法尝试同时获取该锁，并且一开始就不需要锁。 

`lock` 的返回类型，即 `MutexGuard <'a，T>`，基本上与`＆'a mut T'` 相同：它给予对存储在锁中的`T` 的独占访问权限。此外，当超出范围时，它会自动释放锁（在 C++世界中被称为RAII [33](#33)）。 

在我们的示例中，这意味着两个线程 *临时* 都具有对该 `vec`  的独占访问权，并且它们都有可变引用，这说明了一个事实-由于锁正确地实现了互斥，因此它们永远不会同时具有可变引用，因此保持了可变引用的唯一性。换句话说，`Mutex` 可以安全地提供引用状态的改变，因为它实现了运行时检查，确保不会在改变时产生可变引用。

---

**可变计数** 我们已经看到，共享引用提供了一种在程序中不同部分方之间共享数据的方法。但是，共享引用具有 `静态确定` 的生存期，并且当该生存期结束时，数据将再次被唯一拥有。这与结构化并行机制（如上一示例中的 ` join` ）配合得很好，但不适用于 `非结构化` 的并行机制，在这种情况下，会产生独立运行于父进程的进程。 

在 Rust 中，在这种情况下共享数据的典型方法是使用 `原子引用计数` 指针：` Arc<T>` 是指向 `T` 的指针，它会计算存在多少指向`T` 的指针，并当最后一个指针销毁时 (引用计数归零) 释放 `T`（并释放其相关资源）。（这可以看作是轻量级实现垃圾回收的一种形式）由于数据是共享的，因此我们无法从`Arc<T>`中获取`&mut T`，但是我们可以获得`&T`。 （在这种情况下，编译器确保在引用生存期内不会破坏 `Arc<T>`）

![f7.jpg](https://dl.acm.org/cms/attachment/be6b6fee-b9f0-40b3-af33-55264c275aae/f7.jpg)

**Figure 7. Rust example: Reference counting.**

我们首先创建一个指向我们通常 `vec` 的 ` Arc` 。 ` arc_v2 `是通过`clone arc_v1`获得的，这意味着引用计数增加了一个，但是数据本身不会增加。 然后我们起一个使用 `arc_v2` 的线程； 即使我们在此编写的函数返回后，这个线程仍在后台运行。 因为这是非结构化的并行，所以我们必须显式地将` arc_v2` 移动（即转移其所有权）到另一个线程中运行的闭包中。 ` Arc` 是一个 `智能指针`（类似于C ++中的` shared_ptr`），因此我们可以像使用`&Vec<i32>` 一样使用它。尤其是在第3行和第4行中，我们可以打印出索引为1 的元素的值。当 `arc_v1` 和`arc_v2` 超出作用域时，它们的析构函数将被隐式调用，最后 `Arc` 会销毁 `vec` 。 

----

**线程安全** 

在这个关于 Rust 的简短介绍中 ，`Rc <T>` 是我们最后一个要谈论的类型。 `Rc <T>` 是一个引用计数类型，它与 `Arc <T>` 非常相似，但是关键区别在于`Arc < T>` 使用原子 （获取和添加指令）来更新引用计数，而 `Rc <T>` 使用非原子操作。结果，`Rc <T>`可能更快，但不是线程安全的。 ` Rc<T>` 类型在复杂的顺序执行的代码中很有用，在这种情况下，共享引用的强制执行的静态作用域不够灵活，或者无法静态预测对对象的最后一个引用被销毁的时间，对象无法在应该被回收时被销毁。 

由于`Rc<T>`不是线程安全的，因此我们需要确保程序员在应该使用 `Arc<T>` 时不要混用使用`Rc<T>`。这很重要：如果我们采用我们前面的 `Arc` 示例，并用 `Rc` 替换了所有`Arc`，这样程序就会产生数据竞争，可能会过早分配内存或根本不分配内存。但是，非常值得注意的是，Rust 编译器依然能够捕获这种错误。 Rust 使用了一种叫做`Send` 的 trait ：这是一种类型的属性，只有当类型 `T` 的元素可以安全地发送到另一个线程时，类型 `T` 才能使用它。类型`Arc <T>`是 `Send` 的，但是`Rc<T>` 不是。 `join` 和`spawn`都要求它们运行的闭包捕获的所有内容都具有 `Send` ，因此，如果我们在闭包中捕获非`Send` 类型的 `Rc <T>` 的变量，将导致编译失败。 

Rust对 `Send` 的使用证明了强静态类型的限制有时会有更强大的表现力。与 C++ 的智能引用计数指针 `std::shared_ptr` 更是如此，因为 `std::shared_ptr` 使用了原子指令。（更准确地说，在Linux上，如果程序使用 `pthreads`，它使用的任何代码可能产生线程，就会使用原子指令。 ），因为具有更有效的非线程安全变体（如`Rc` ）被认为过于冒险。相比之下，Rust的`send` 允许人们 `无畏编码` ： [26](#26)），Rust 是一种将线程安全的数据结构（例如 `Arc`）和非线程安全的数据结构（例如`Rc`）同时包含在中的语言，同时以模块化的方式确保不会错误地使用两者。 

##  安全封装 `Unsafe`

我们已经看到了诸如 `Arc` 和 `Mutex` 这样的类型如何使 Rust 程序 安全地使用诸如引用计数和共享引用之类。但是，这有一个问题：这些类型实际上不能在 Rust 中实现。或更确切地说，它们不能在 *`safe Rust `* 中实现：编译器会因为可能违反引用规则而拒绝执行 `Arc` 。实际上，它甚至会拒绝使用 `Vec` 来访问可能未初始化的内存。出于效率原因，`Vec` 手动管理底层缓冲区并跟踪其初始化部分。当然，` Arc` 的实现实际上并没有违反引用规则，而 `Vec` 实际上并没有访问未初始化的内存，但是这些推断对于编译器来说太过于细微了。

为了解决这个问题，Rust 提供了一个 `后门` ：Rust 不仅包含我们到目前为止讨论的安 ` Safe Rust`，而且还提供了一些 `Unsafe` 功能，例如 C 风格的非受限指针。编译器不能保证这些功能的安全性（内存安全性和/或线程安全性），因此它们仅在标记有 `unsafe` 关键字的语法块内可用。这样，可以确保不会 *偶然* 离开 `Safe Rust`。

>*我们希望激发计算机科学研究领域的其他成员开始更加关注Rust，并为这种突破性语言的发展做出贡献。* 

----

例如，`Arc` 的实现使用 `Unsafe` 来实现在 `Safe Rust` 中无法表达的模式：没有明确所有者的共享引用，由线程安全引用计数进行管理。 对`弱(weak)引用`的支持使情况更加复杂：`弱引用` 不能使引用对象保持存活，但可以通过原子方式检查其有效性，并升级为完整的`Arc` 。 Rust编译器根本无法静态验证当引用计数达到零时释放内存实际上是安全的。 

----

**`unsafe` 块的替代品** 可以将 `Arc` 或`Vec` 之类的东西转换到语言层面。例如，Python 和 Swift 具有内置的引用计数，Python 具有与 `Vec` 等效的内置的 `list`。但是，这些语言功能是在 C 或 C++中实现的，因此它们实际上没有比 `unsafe Rust` 实现更安全。除此之外，将不安全的操作限制为语言内置的实现还严重限制了灵活性。例如，Firefox 使用 Rust 库实现了 `Arc` 的变体，但不支持弱引用，从而提高了代码的空间使用率和性能。语言是否应该为任何内置类型的设计空间中的每个可能的位置提供原语？ 

避免 `unsafe` 另一种选择是使类型系统具有足够的表达力，以实际能够验证诸如 ` Arc` 之类的类型的安全性。但是，由于此类数据结构的安全性的可能性很小（实际上是 ` Arc` 和简化的变体，其中一些已被用作最近几份正式验证论文中的主要案例研究[9](#9),[12],(#12)，[18](#18)，但这基本上只能被有形式的通用定理证明经历和具有足够背景知识的研究人员使用。开发者和种定理证明社区还有很大的距离。

----

**安全抽象** 相反，Rust选择允许程序员在必要时灵活地编写不安全的代码，尽管期望它应该被  `safe` 的 API 封装。安全封装意味着，无论使用诸如 `Arc` 或 `Vec`  之类的 Rust API 都是通过 `unsafe` 代码实现的，这些 API 的用户都不会受到影响：只要用户在 `safe` 片段中编写类型正确的代码，由于 Rust 在 API 的实现中使用了不安全的代码，他们永远都观察不到异常行为。这与 C++ 形成鲜明对比，C++的弱类型系统缺乏甚至无法强制 `安全使用` API 的能力。结果，像`shared_ptr` 或 `vector` 之类的 C++ API 容易被滥用，导致引用计数错误和迭代器无效，这些错误在Rust中都不会出现。 

编写不安全代码的能力就像 Rust 程序员用来使类型系统更有用而又不会将其变成定理证明器的杠杆一样，我们确信这是 Rust 成功的关键。 Rust 社区正在开发一个有着安全可用的高性能库的生态系统，让程序员可以在它们之上构建安全而高效的应用程序。 

但是，当然天下没有免费的午餐：Rust 库的作者必须以某种方式确保，他们会非常谨慎，在使用 `unsafe` 的时候不会破坏 Rust 的安全保证。一方面，这比C/C++ 要好得多，因为绝大多数 Rust代码 是用该语言的 `safe` 编写的，因此 Rust 的 `攻击面` 要小得多。另一方面，当 `Unsafe`  变得不可或缺的时候，程序员知道自己是否足够 `谨慎` 远非显而易见。

 因此，为了保持对 Rust 生态系统安全性的信心，我们真心希望有一种形式化的方法，可以正式的验证使用安全地封装在安全API 的 `unsafe` 的背后的行为。 这正是` RustBelt` 项目的目标。

 

### RustBelt: Rust 基础设施的保卫者

验证 Rust 的安全性的主要挑战是考虑 `safe` 与 `unsafe` 之间的相互作用。 要了解为什么这具有挑战性，让我们简要地看一下验证编程语言安全性的标准技术，即所谓的*syntactic approach* 。[14](#14)，[37](#37) 使用该技术，安全性是由基于大量数学推导给出正式结构的类型检查器的  *syntactic typing judgment* 来表示的。 

---

**Theorem 1***( *Syntactic type soundness* ) 如果程序 e 在 *syntactic typing judgment* 后是 `well-typed` 的，则 e 是安全的。 

> well-typed 参见 [Type safety](https://en.wikipedia.org/wiki/Type_safety)

不幸的是，这个定理对于我们的目的来说太弱了，因为它仅在语法上谈论安全程序，从而排除了使用 `不安全`代码的程序。 例如，`if true {e} else {crash（）}` 在语法上不是 `well-typed`，但由于从未执行`crash（）`，所以它仍然是安全的。 

---

**解决方案: *Semantic type soundness*.**

> [Semantic type soundness](https://blog.sigplan.org/tag/semantic-type-soundness/) 

为了说明 `safe` 与 `unsafe` 之间的相互作用，我们改用称为 *Semantic type soundness* 的技术，该技术根据程序的`行为`来表示安全性，而不是使用固定的推理规则集。 *Semantic type soundness* 的关键要素是 *logical relation(逻辑关系)*，该逻辑关系为每个 API 分配了 *safety contract*   。 它表示如果API 中每个方法的输入均符合其指定的类型，则输出也是如此。使用形式验证中的技术，可以证明 API 的实现满足指定的 *safety contract* ，如[图8]。

![f8.jpg](https://dl.acm.org/cms/attachment/2c46d078-5266-431c-96b0-80fd8eee1c18/f8.jpg)

 **Figure 8. Semantic type soundness.**

对于 `safe` 与 `unsafe` 组合的程序进行推理， *Semantic type soundness* 是理想的选择。 对于任何使用 `unsafe` 代码块的库（例如` Arc Mutex Rc 和 Vec`），都必须手动证明该实现满足 *safety contract* 。 例如： 

**Theorem 2.** *Arc satisfies its safety contract*.

对于程序的 `safe` 代码块，验证是自动的。 以下定理表达了这一点，该定理说，如果将组件写入 Rust 的 `safe` 代码块中，则它通过构造满足其 *safety contract*。 

**Theorem 3** (Fundamental theorem). *如果 组件 e 在语法上是 `well-typed`的 ，则 e 满足 safety contract*。

综上所述，如果“`unsafe` 的只出现是在经过手动验证可以满足 *safety contract* 的库中，则这表示 Rust 程序是安全的。 

**使用 [Iris logic](https://iris-project.org/) 来解码 *safety contract*  ** *Semantic type soundness* 是一种古老的技术，至少可以追溯到米尔纳（1978年）关于类型健全性的开创性论文，[28](#28)，但将其扩展到像 Rust 这样的工业现代语言被证明是一个艰巨的挑战。实际上，在开发“`"step-indexed Kripke logical relations`”（SKLR）模型[3](#3),[5](#5) 之前，将其扩展到具有可变状态和高阶函数的语言仍然是一个悬而未决的问题。[2](#2),[4](#4)，作为基础证明代码项目的一部分。即便如此，使用 SKLR 模型直接编码的*safety contract*  的验证仍然是非常繁琐，低级且难以维护的。 

在 `RustBelt` 项目中，我们以 Iris 的最新工作为基础[19](#19),[20](#20),[21](#21),[23](#23)，(一个证明框架，用于更高阶，并发，命令式程序，使用 Coq proof assistant实现)。[1](#1) Iris提供了一种更高级的语言来编码和使用 SKLR 模型，从而使我们能够扩展此类模型，以处理Rust等复杂的语言。特别是基于*separation logic*，[29](#29), [30](#30), Hoare逻辑的扩展[15](#15) 专门针对指针操作程序的模块化推理，并以所有权概念为中心。这为我们提供了一种理想的语言，可用于在Rust中建模所有权类型的语义。 

Iris扩展了传统的 *separation logic*，并增加了一些对 Rust 至关重要的附加功能： 

- Iris 支持*用户定义的 ghost state*：定义自定义逻辑资源的能力，这些逻辑资源对于证明程序的正确性十分有用，但并不直接对应于其物理状态下(译者注: 指内存)的任何内容。 Iris 用户定义的 *ghost state* 使我们能够验证诸如 `Arc` 之类的库的健全性，这些库的所有权并不对应于物理所有权（例如，两个单独拥有的 `Arc<T>` 可能在同一款内存下）-- 一种称为 ` fictional separation` 的现象。[10](#10)，[11](#11) 

  通过（在Iris内）派生了一个新的，特定于领域的`lifetime logic`，它还使我们能够以更高的抽象水平来思考 Rust 的借用和生命周期。 

-  Iris支持 *impredicative invariants* ：程序状态上的不变式，可能会循环引用其他不变式的存在。[32 ](#32)*impredicative invariants* 构建核心类型系统（例如递归类型和闭包）中起着至关重要的作用。 

Rust 的复杂性要求我们对语义的健全性证明进行*machine-checked*, 因为这太繁琐且容易出错，无法手动进行证明。 幸运的是，Iris带有丰富的 *separation-logic tactics* ，这些策略是根据标准 Coq 策略制定的，因此可以让 Coq用户 经过 ` time-tested` 的方式熟悉的开发 *machine-checked* 的语义完整性证明。 [22](#22),[24](#24)

### 结论和展望

在本文中，我们给出了 Rust 的鸟瞰图，展示了 Rust 的核心概念，例如借用，生存期以及封装在 `safe` API 中的 `unsafe` 代码。这些功能帮助 Rust 第一个受行业支持的在高级语言带来的安全性和由较低级别的`系统编程`语言提供的资源管控之间做出长远平衡的语言。 

为了研究 `Rust ` 的安全声明，我们描述了语义类型健全性的证明技术，这使我们能够开始在 `RustBelt` 项目中为 Rust 构建严格的基础。有关Rust 和 `RustBelt` 的更多详细信息，请向感兴趣的读者可以去阅读我们的POPL'18论文[18](#18) 和第一作者的博士学位论文。[17](#17) 

我们还有很多工作要做。尽管 `RustBelt` 最近为Rust 考虑 从C++继承来的宽松内存并发模型( relaxed-memory concurrency model )[9](#9)，当还没有涵盖许多其他 Rust 功能和 API，例如其 `trait` 系统，该系统非常复杂以至于会出现很多微妙的 bug 。 [7](#7)，尽管验证`unsafe rust` 库的健全性目前需要形式语义的深厚背景，但我们希望最终开发出可以直接交给程序员的形式化方法。 

最后，尽管 `RustBelt` 专注于为 Rust 本身打好安全基础，但我们很高兴看到其他研究项目（特别是 `Prusti` [6](#6) 和 `RustHorn` [27](#27)）正开始探索一个令人激动的正交方向：即Rust 的强类型系统有可能用作简化系统代码形式验证的有力工具。 

##  References

 <span id="1"></span> 1.The Coq proof assistant, 2019; https://coq.inria.fr/.

 <span id="2"></span> 2. Ahmed, A., Appel, A.W., Richards, C.D., Swadi,  K.N., Tan, G. and Wang, D.C. Semantic foundations for typed assembly  languages. *TOPLAS 32*, 3 (2010).

 <span id="3"></span>3. Ahmed, A.J. Semantics of types for mutable state. Ph.D. thesis, Princeton University, 2004.

 <span id="4"></span>4. Appel, A.W. Foundational proof-carrying code. *LICS*, 2001.

 <span id="5"></span>5. Appel, A.W. and McAllester, D. An indexed model of recursive types for foundational proof-carrying code. *TOPLAS 23*, 5 (2001).

 <span id="6"></span>6. Astrauskas, V., Müller, P., Poli, F. and Summers, A.J. Leveraging Rust types for modular specification and verification. *PACMPL 3 (OOPSLA)*, 2019.

 <span id="7"></span>7. Ben-Yehuda, A. Coherence can be bypassed by an indirect impl for a trait object, 2019; https://github.com/rust-lang/rustissues/57893.

 <span id="8"></span>8. Burch, A. Using Rust in Windows. Blog post, 2019; https://msrc-blog.microsoft.com/2019/11/07/using-rust-in-windows/.

 <span id="9"></span>9. Dang, H.-H., Jourdan, J.-H., Kaiser, J.-O. and Dreyer, D. RustBelt meets relaxed memory. *PACMPL 4 (POPL)*, 2020.

 <span id="10"></span>10. Dinsdale-Young, T., Dodds, M., Gardner, P., Parkinson, M.J. and Vafeiadis, V. Concurrent abstract predicates. *ECOOP*, 2010.

 <span id="11"></span>11. Dinsdale-Young, T., Gardner, P. and Wheelhouse, M.J. Abstraction and refinement for local reasoning. *VSTTE*, 2010.

 <span id="12"></span>12. Doko, M. and Vafeiadis, V. Tackling real-life relaxed concurrency with FSL++. *ESOP 10201, LNCS*, 2017.

 <span id="13"></span>13. Grossman, D., Morrisett, G., Jim, T., Hicks, M., Wang, Y. and Cheney, J. Region-based memory management in Cyclone. *PLDI*, 2002.

 <span id="14"></span>14. Harper, R. *Practical Foundations for Programming Languages* (2nd Ed.). Cambridge University Press, 2016.

 <span id="15"></span>15. Hoare, C.A.R. An axiomatic basis for computer programming. *Commun. ACM 12*, 10 (1969).

 <span id="16"></span>16. Hosfelt, D. Implications of rewriting a browser component in Rust. Blog post, 2019; https://hacks.mozilla.org/2019/02/rewriting-a-browser-component-in-rust/.

 <span id="17"></span>17. Jung, R. Understanding and Evolving the Rust Programming Language. Ph.D. thesis, Universität des Saarlandes, 2020; https://people.mpi-sws,org/~jung/thesis.html.

 <span id="18"></span>18. Jung, R., Jourdan, J.-H., Krebbers, R. and Dreyer, D. RustBelt: Securing the foundations of the Rust programming language. *PACMPL 2 (POPL)*, 2018.

 <span id="19"></span>19. Jung, R., Krebbers, R., Birkedal, L. and Dreyer, D. Higher-order ghost state. *ICFP*, 2016.

 <span id="20"></span>20. Jung, R., Krebbers, R., Jourdan, J.-H., Bizjak, A., Birkedal, L. and Dreyer, D. Iris from the ground up: A modular  foundation for higher- order concurrent separation logic. *JFP 28* (2018).

 <span id="21"></span>21. Jung, R., Swasey, D., Sieczkowski, F.,  Svendsen, K., Turon, A., Birkedal, L. and Dreyer, D. Iris: Monoids and  invariants as an orthogonal basis for concurrent reasoning. *POPL*, 2015.

 <span id="22"></span>22. Krebbers, R., Jourdan, J.-H., Jung, R.,  Tassarotti, J., Kaiser, J.-O, Timany, A., Charguéraud, A. and Dreyer, D. MoSeL: A general, extensible modal framework for interactive proofs in  separation logic. *PACMPL 2 (ICFP)*, 2018.

 <span id="23"></span>23. Krebbers, R., Jung, R., Bizjak, A., Jourdan,  J., Dreyer, D. and Birkedal, L. The essence of higher-order concurrent  separation logic. *ESOP*, 2017.

 <span id="24"></span>24. Krebbers, R., Timany, A. and Birkedal, L. Interactive proofs in higher-order concurrent separation logic. *POPL*, 2017.

 <span id="25"></span>25. Levick, R. Why Rust for safe systems programming. Blog post, 2019; https://msrc-blog.microsoft.com/2019/07/22/why-rust-for-safe-systems-programming/.

 <span id="26"></span>26. Matsakis, N. and Turon, A. Rust in 2016, 2015. Blog post; https://blog.rust-lang.org/2015/08/14/Next-year.html.

 <span id="27"></span>27. Matsushita, Y., Tsukada, T. and Kobayashi, N. RustHorn: CHC-based verification for Rust programs. *ESOP*, 2020.

 <span id="28"></span>28. Milner, R. A theory of type polymorphism in programming. *J. Computer and System Sciences 17*, 3 (1978).

 <span id="29"></span>29. O'Hearn, P.W., Reynolds, J.C. and Yang, H. Local reasoning about programs that alter data structures. *CSL*, 2001.

 <span id="30"></span>30. O'Hearn, P.W. Resources, concurrency, and local reasoning. *Theoretical Computer Science 375*, 1–3 (2007).

 <span id="31"></span>31. Stroustrup, B. *The C++ Programming Language*. Addison-Wesley, 2013.

 <span id="32"></span>32. Svendsen, K. and Birkedal, L. Impredicative concurrent abstract predicates. *ESOP*, 2014.

 <span id="33"></span>33. Thomas, G. A proactive approach to more secure code. Blog post, 2019; https://msrc-blog.microsoft.com/2019/07/16/a-proactive-approach-to-more-secure-code/.

 <span id="34"></span>34. Tofte, M. and Talpin, J. Region-based memory management. Information and Computation 132, 2 (1997).

 <span id="35"></span>35. Tu, T., Liu, X., Song, L. and Zhang, Y. Understanding real-world concurrency bugs in Go. *ASPLOS*, 2019.

 <span id="36"></span>36. Walker, D. Substructural type systems. *Advanced Topics in Types and Programming Languages*. B.C. Pierce, Ed. MIT Press, Cambridge, MA, 2005.

 <span id="37"></span>37. Wright, A.K. and Felleisen, M. A syntactic approach to type soundness. *Information and Computation 115*, 1 (1994).