---
pub_date: Tue, 31 May 2021 21:00:00 GMT
description: Things you can not do in Rust

---

# Rust 中无法办到的事情(以及如何替代)

译者：Matrixtang

> [原文](https://blog.logrocket.com/what-you-cant-do-in-rust-and-what-to-do-instead/) 

---

作为 [Rust subreddit](https://www.reddit.com/r/rust/)的主持人，我经常发布有关开发人员试图将其各自的语言范例转换为 Rust 的帖子，结果不一而足，取得了不同程度的成功。 在本指南中，我将描述开发人员在将其他语言范例转换为 Rust 时遇到的一些问题，并提出一些替代解决方案来帮助你客服 Rust 的局限性。

## Rust 中的继承

可以说，继承是在面向对象语言中被问到最多的缺失特性。为什么 Rust 不让一个结构 (`struct`) 继承另一个结构呢？

你可以肯定地说，即使在 OO 世界中，继承的名声也没好到哪里去，而且实践者通常尽可能地喜欢组合(`composition` )。但是你也可以认为，允许类型以不同的方式执行方法可能会提高性能，因此对于那些特定的实例来说是可取的。

这是一个来自 Java 的经典示例：

```java
interface Animal {
    void tell();
    void pet();
    void feed(Food food);
}class Cat implements Animal {
    public void tell() { System.out.println("Meow"); }
    public void pet() { System.out.println("purr"); }
    public void feed(Food food) { System.out.println("lick"); }
}// this implementation is probably too optimistic...
class Lion extends Cat {
    public void tell() { System.out.println("Roar"); }
}
```

对于 Rust，第一部分可以用 traits 实现：

```rust
trait Animal {
    fn tell(&self);
    fn pet(&mut self);
    fn feed(&mut self, food: Food);
}struct Cat;impl Animal for Cat {
    fn tell(&self) { println!("Meow"); }
    fn pet(&mut self) { println!("purr");
    fn feed(&mut self, food: Food) { println!("lick"); }
}
```

但第二部分并没用这么容易：

```rust
struct Lion;impl Animal for Lion {
    fn tell(&self) { println!("Roar"); }
    // Error: Missing methods pet and feed
    // 错误: 缺少 `pet` 和 `feed` 方法
}
```

显然，最简单的方法是复制这些方法。是的，重复是不好的。这样也会使得代码更加复杂。如果你需要代码复用的话, 不妨把这些方法抽出来, 在 `Cat` 和 `Lion` 中调用它们。

但是，你也许会察觉到,如何实现 OO 中的多态性部分呢？这就是复杂的地方。面向对象语言通常给你提供动态转发，而 Rust 让你在静态和动态分发中做出选择，不管选择哪一种都有失有得。

```rust
// static dispatch
// 静态分发
let cat = Cat;
cat.tell();let lion = Lion;
lion.tell();// dynamic dispatch via enum // 通过enum 进行动态分发
enum AnyAnimal {
   Cat(Cat),
   Lion(Lion),
}// `impl Animal for AnyAnimal` left as an exercise for the readerlet animals = [AnyAnimal::Cat(cat), AnyAnimal::Lion(lion)];
for animal in animals.iter() {
   animal.tell();
}// dynamic dispatch via "fat" pointer including vtable
// 动态分发通过`胖` 指针来实现
let animals = [&cat as &dyn Animal, &lion as &dyn Animal];
for animal in animals.iter() {
   animal.tell();
}
```

> 译者注: 动态分发参见 https://juejin.cn/post/6872898487244029960 以及 https://alschwalm.com/blog/static/2017/03/07/exploring-dynamic-dispatch-in-rust/

注意，与垃圾收集语言不同的是，在 ( Rust 中) 每个变量在编译时必须有一个具体的类型。此外，对于 `enum` 的情况，使用进行委托 `trait` 的实现是冗长乏味的，但是像 `ambassador`[1] 这样的 crates 可以提供帮助。

将函数委托给成员的一种相当 `hacky` 的方法是使用 [Deref trait for polymorphism](https://rust-unofficial.github.io/patterns/anti_patterns/deref.html)`，这样在 `derefee` 上可以直接调用`Deref` 目标定义的函数。但是请注意，这通常被认为是一种反模式。

最后，可以为所有实现许多其他特性之一的类实现一个 `trait`，但它需要专门化，这是目前的一个 `nightly` 特性（尽管有一个可用的解决方案 `workaround`[2]，如果你不想写出所需的所有样板代码，可以把他们打包在一个`macro crate` 中）。`trait` 很可能是相互继承的，尽管它们只规定行为，而不是数据。

## 链表或者其他基于指针的数据结构

许多从 C++ 来到 Rust 的人一开始会想实现一个 “简单的” 双向链表，但很快就会发现它远非 简单。这是因为 Rust 想要明确所有权，因此双向列表需要对指针和引用进行相当复杂的处理。

一个新手可能会尝试写下面的 struct：

```rust
struct MyLinkedList<T> {
    value: T
    previous_node: Option<Box<MyLinkedList<T>>>,
    next_node: Option<Box<MyLinkedList<T>>>,
}
```

当他们注意到这个方法失败时，他们会添加 `Option` 和 `Box`。但是一旦他们尝试实现插入，他们就会感到很惊讶：

```rust
impl<T> MyLinkedList<T> {
    fn insert(&mut self, value: T) {
        let next_node = self.next_node.take();
        self.next_node = Some(Box::new(MyLinkedList {
            value,
            previous_node: Some(Box::new(*self)), // Ouch
            next_node,
        }));
    }
}
```

当然，borrow checker[3] 不会允许这样做。值的所有权完全是混乱的。`Box` 拥有它所包含的数据，因此列表中每个节点都将由列表中的上一个和下一个节点拥有。Rust 中的每个数据只允许有一个所有者，所以这将至少需要一个 `Rc` 或 `Arc` 才能工作。但是即使这样做也会很快变得麻烦，更不用说引用计数带来的开销了。

幸运的是，你不需要自己编写双向链表，因为标准库已经包含了一个（`std::collections::LinkedList`）。而且，与简单的 Vecs 相比，这种方法可能并不能给你带来好的性能，因此你可能需要相应地进行测试。

如果你真的想写一个双向链表列表，你可以参考[Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)][4] ，它会教会你写链表，并在这个过程中学到很多关于 `Unsafe Rust` 的知识。

（此外：单列表完全可以用一连串的 `box` 来构建。实际上，Rust 编译器包含一个[实现](https://github.com/rust-lang/rust/blob/f811f14006fa46030f1af714f7d640580d3ad822/compiler/rustc_data_structures/src/tiny_list.rs)。）

同样的情况也适用于图结构，尽管你可能需要一个依赖项来处理图数据结构。Petgraph[5] 是目前最流行的，它提供了数据结构和一些图算法。

## 自引用类型

当面对自引用类型的概念时，很容易会问出: “谁拥有它？”同样，这也是 `borrow checker` 不乐意听到的关于 `ownership`的事情。

当你具有所有权关系并希望在一个结构中同时存储所有权对象和被所有的对象时，就会遇到这个问题。`天真地`尝试一下这个方法，你会有一段艰难的时期去尝试生命周期 (`lifetime`)。

我们只能猜测，许多 `rustacean` 已经转向 `Unsafe Rust`，这很微妙的，并且很容易出错。当然，使用普通指针而不是引用会消除生命周期烦恼，因为指针不会有生命周期(`lifetime`)的烦恼。但是，这需要手动承担管理生命周期的责任。

幸运的是，有一些 crate 可以采用这种解决方案并提供一个安全的接口，比如 `ouroboros`[6], `self_cell`[7] 和 `one_self_cell`[8] 等 crates。

## 全局可变状态

来自 C 或 C++ (或是来自动态语言) 的开发者，有时习惯于在他们的代码中创建和修改全局状态( `global state` )。例如，一位 reddit 用户说：“这是完全安全的，但 Rust 不让你这么做。”

下面是一个稍微简化的例子：

```C++
#include <iostream>
int i = 1;int main() {
    std::cout << i;
    i = 2;
    std::cout << i;
}
```

在 Rust 中，这大致可以理解为：

```rust
static I: u32 = 1;fn main() {
    print!("{}", I);
    I = 2; // <- Error: Cannot mutate global state
    print!("{}", I);
}
```

许多 `Rustacean` 会告诉你，你并不需要这种全局的状态。当然，在这样一个简单的例子中，这是正确的。但是对于大量的用例，确实需要全局可变状态的时候，例如，在一些嵌入式应用程序中。

当然，有一种方法可以做到这一点，使用 `Unsafe Rust`。但是在这之前，根据场景的不同，你可能只想使用互斥对象（`Mutex`）即可。或者，如果可变只需要在初始化时使用一次，那么 `OnceCell` 或 `lazy_static` 就可以巧妙地解决这个问题。或者，如果你真的只需要整数，那么 `std::sync::Atomic*` 类型也可以使用。

话虽如此，尤其是在每个字节数和资源通常映射到内存的嵌入式世界中, 拥有一个可变的静态变量通常是首选的解决方案。因此，如果你真的必须这么做，写起来像这样：

```rust
static mut DATA_RACE_COUNTER: u32 = 1;fn main() {
    print!("{}", DATA_RACE_COUNTER);
    // I solemny swear that I'm up to no good, and also single threaded.
    // 我发誓即使是单线程,依然跑不了
    unsafe {
        DATA_RACE_COUNTER = 2;
    }
    print!("{}", DATA_RACE_COUNTER);
}
```

再次强调，除非真的需要，否则你不应该这样做。如果你想问这是不是一个好主意，答案是否定的。

## 直接初始化一个数组

新手可能会倾向于声明如下数组：

```rust
let array: [usize; 512];for i in 0..512 {
    array[i] = i;
}
```

这会报错，因为数组从未初始化。然后我们尝试给它赋值，但是没有告诉编译器，它甚至不会为我们在堆栈上保留一个写入的位置。Rust 是这样挑剔，它根据数组的内容来区分数组。此外，在我们读取它们之前，需要对它们进行初始化。

通过初始化 `let array = [0usize; 512] ;` ，我们以双重初始化为代价来解决这个问题，双重初始化可能会也可能不会得到优化——或者，根据类型的不同，甚至可能是无法实现的。参见 Unsafe Rust: How and when not to use it[9] 的解决方案。

## 总结

### 参考资料

[1]ambassador: https://docs.rs/ambassador/0.2.1

[2]workaround: https://github.com/dtolnay/case-studies/blob/master/autoref-specialization/README.md

[3]borrow checker: https://blog.logrocket.com/introducing-the-rust-borrow-checker/

[4] Learn Rust With Entirely Too Many Linked Lists : https://rust-unofficial.github.io/too-many-lists/

[5]Petgraph: https://crates.io/crates/petgraph

[6]`oeuroboros`: https://docs.rs/ouroboros/0.9.2/ouroboros/

[7]`self_cell`: https://docs.rs/self_cell/0.8.0/self_cell/

[8]`one_self_cell`: https://docs.rs/once_self_cell/0.6.3/once_self_cell/

[9]Unsafe Rust: How and when not to use it: https://blog.logrocket.com/unsafe-rust-how-and-when-not-to-use-it/
