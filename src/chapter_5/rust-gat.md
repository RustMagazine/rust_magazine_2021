---
pub_date: Tue, 31 May 2021 21:00:00 GMT
description: Rust GAT

---

# 了解一点关于泛型关联类型(GAT)的事

作者：CrLF0710 / 后期编辑：张汉东

---

这名字好长！这是啥子哟？

不要紧，我们从头理理。咱们先来温习一下Rust的语法结构。Rust程序是什么构成的？答案是：条目(item)。

每个Rust程序都是一条一条的，每个都是一个条目。比如你在main.rs里写一个结构体定义， 再写一个实现定义为它加两个方法，再写一个main函数。这就是crate对应的模块条目里的三个条目。

说完了条目，咱们再来说关联条目。关联条目不是条目！重点就是在“关联”俩字上，什么是关联呢？ 其实就是“跟某个类型有关”，效果就是可以使用一个专门的关键字叫做Self。 这个关键词就是用来指代刚才说的那个类型的。

关联条目可以定义在两处，一个是特质定义的花括号中，一个是实现定义的花括号中。

关联条目一共有三种：关联常数，关联函数，关联类型(别名)；它们与条目中的三种：常数、函数、类型(别名) 一一对应。

举个栗子吧！
```rust
#![feature(generic_associated_types)]
#![allow(incomplete_features)]
const A: usize = 42;
fn b<T>() {}
type C<T> = Vec<T>;

trait X {
    const D: usize;
    fn e<T>();
    type F<T>; // 新加的就是这个！之前在这里不可以写<T>
}

struct S;
impl X for S {
    const D: usize = 42;
    fn e<T>() {}
    type F<T> = Vec<T>;
}
```

这个有啥用咧？

蛮有用的，但是仅仅是在特定的场景之下。Rust社区里对泛型关联类型有两个用例是非常经典的，我们先试着介绍它们一下。

但是在开始介绍之前，我们还要再来温习一下泛型。泛型这个词英文是generic，其实是通用的意思。 泛型类型是什么呢？简单来说，就是缺点什么参数的类型，让使用的人填充。

顺便说一下，前人把它意译取名叫泛型，因为很多系统里能填的参数是类型。其实在Rust里面，不只是类型可以当泛型参数。 泛型参数有三种：类型、生存期、常数。

好，我们来看一个具体的泛型类型的例子：`Rc<T>`，它是具有一个泛型参数的泛型类型。 泛型类型Rc不是类型哈，只有提供了这个泛型参数的“实参”，才是真正的类型，比如`Rc<bool>`。

如果我写一个数据结构，里面要共享数据，但是我事先不知道使用者到底需要我在这里用Rc还是Arc，怎么办呢？ 最简单的方法就是代码写两遍，这个听起来有点笨拙，确实也是如此，但是也是确实有效的。 随口一提，[http://crates.io](http://crates.io)上有俩库im和im-rc代码主要区别就是里面用的是Arc还是Rc。 实际上泛型关联类型就可以很好的解决这个问题，接下来， 我们就来看泛型关联类型的第一个经典使用场景：类型家族(type family)。

## 任务#1：用泛型关联类型支持类型家族

好，现在我们来做一个“选择器”，让编译器根据这个选择器来知道需要用的到底是`Rc<T>`还是`Arc<T>`。代码长这样：

```rust
trait PointerFamily {
    type PointerType<T>;
}

struct RcPointer;

impl PointerFamily for RcPointer {
    type PointerType<T> = Rc<T>;
}

struct ArcPointer;

impl PointerFamily for ArcPointer {
    type PointerType<T> = Arc<T>;
}
```

挺简单的吧，这样你就定义了两个“选择器”类型，可以用它来代表要用的是Rc还是Arc。实际用用看：

```rust
struct MyDataStructure<T, PointerSel: PointerFamily> {
    data: PointerSel::PointerType<T>
}
```

这样你泛型参数用`RcPointer`或者`ArcPointer` 就可以选择实际的数据表示了。 有了这个功能，刚才说的两个包就可以合成一个包了。好耶~

## 任务#2：用泛型关联类型实现流式处理迭代器

另一个问题其实是Rust比较特有的，其他语言里，要么不存在这个问题（古尔丹：代价是什么呢？）， 要么，放弃治疗这个问题（咳咳）。

这个问题是这样的，希望在API接口上表示输入值与输入值之间、输入值与输出值之间的依赖关系。 依赖关系并不是一个很容易表达出来的东西。Rust的方案是什么呢？ 在Rust里，这个人见人爱的生存期小标记`'_`大家都见过啦。它就负责在API上表示这种依赖关系的对应。

我们来实际用用这个生存期标记，标准库里的迭代器特质大家都见过，它长这样：

```rust
pub trait Iterator {
    type Item;

    pub fn next(&'_ mut self) -> Option<Self::Item>;
    // ...
}
```

挺好的，但是有个小问题。Item类型的值是与Iterator本身的类型(Self)完全不能有依赖关系的。为什么呢？ 因为你从Iterator取一个值这个动作，产生的这个临时范围（也就是上面的'_），是next这个关联函数的泛型参数。 定义的Item是单独的另一个关联类型，怎么可能用到呢？

大多数时候这个不是什么问题，但是对于某些库的API来说，这个就不够用了。 比如假如有一个迭代器，依次递给用户一些临时文件用，用户什么时候关闭都可以。这个时候你用Iterator，没有任何问题。 但是要是每次生成一个临时文件，加载一个什么数据，你用完之后它需要关闭临时文件来删除的那种， 这个迭代器肯定就会希望你能够告诉它你用完了。这样它就可以删掉临时文件了， 或者干脆不删除，而是直接复用它的存储空间来存下一个文件，这些都是ok的。

所以这个时候我们可以用泛型关联类型来设计这个API。

```rust
pub trait StreamingIterator {
    type Item<'a>;

    pub fn next(&'_ mut self) -> Option<Self::Item<'_>>;
    // ...
}
```

实现时你其实就可以让Item的类型是一个带依赖的类型，比如一个借用， 类型系统能够保证你在下次调用next或者移动析构这个迭代器之前，Item已经不再被用户使用了。好耶~

你讲的太接地气了，能不能来点抽象的？
好嘞，从现在起我们开始不说人话了。先说一下，这里要说的依然是简化过的，比如我们会把各种binder和predicate放一边。

首先我们来建立泛型类型的名字和具体类型之间的关系。当然就是个映射关系了。

```rust
/// 伪代码
fn generic_type_mapping(_: GenericTypeCtor, _: Vec<GenericArg>) -> Type;
```

比如`Vec<bool>`中，Vec就是这个泛型类型的名字也是它的构造器, `<bool>` 是这个泛型参数的列表，就一项。经过了这个映射，得到了一个`Vec<bool>`。

好，然后是特质，啥是特质啊，特质其实也是一个映射。

```rust
/// 伪代码
fn trait_mapping(_: Type, _: Trait) -> Option<Vec<AssociateItem>>;
```

这里这个Trait可以起到一个谓词的作用，也就是拿它来对某个类型做判定，结论要么是None，表示“不符合这个特质”， 要么是一个Some(items) ，表示“这个类型符合这个特质”，并映射出一串关联条目。

```rust
/// 伪代码
enum AssociateItem {
    AssociateType(Name, Type),
    GenericAssociateType(Name, GenericTypeCtor), // 这次新加的
    AssociatedFunction(Name, Func),
    GenericFunction(Name, GenericFunc),
    AssociatedConst(Name, Const),
}
```

这里的`AssociateItem::GenericAssociateType`是当前rust里唯一一处间接地执行generic_type_mapping的地方。 通过给trait_mapping的第一个参数传不同的Type，就可以用相同的Trait获取到不同的GenericTypeCtor， 然后执行generic_type_mapping，从而在Rust的语法框架下达到了让不同的GenericTypeCtor跟指定的`Vec<GenericArg>`组合的目的！

顺便提一下，GenericTypeCtor这类东西，就是某些文章里面介绍的HKT。通过以上描述的这套方法，Rust里第一次加入了供用户使用的HKT能力。 虽然只有这一种形式，但是其他使用形式都可以通过这一种形式做出来。总之就是，奇怪的能力增加了！

我和小鸭子学走路
好嘞，作为收尾，我们来试着用 GAT 仿制一些其他语言的一些构造。

```rust
#![feature(generic_associated_types)]
#![allow(incomplete_features)]

trait FunctorFamily {
    type Type<T>;

    fn fmap<T, U, F>(value: Self::Type<T>, f: F) -> Self::Type<U>
    where
        F: FnMut(T) -> U;
}

trait ApplicativeFamily: FunctorFamily {
    fn pure<T>(inner: T) -> Self::Type<T>;

    fn apply<T, U, F>(value: Self::Type<T>, f: Self::Type<F>) -> Self::Type<U>
    where
        F: FnMut(T) -> U;
}

trait MonadFamily: ApplicativeFamily {
    fn bind<T, U, F>(value: Self::Type<T>, f: F) -> Self::Type<U>
    where
        F: FnMut(T) -> Self::Type<U>;
}
```
然后我们来给一个“选择器”实现这些类型：

```rust
struct OptionType;

impl FunctorFamily for OptionType {
    type Type<T> = Option<T>;

    fn fmap<T, U, F>(value: Self::Type<T>, f: F) -> Self::Type<U>
    where
        F: FnMut(T) -> U,
    {
        value.map(f)
    }
}

impl ApplicativeFamily for OptionType {
    fn pure<T>(inner: T) -> Self::Type<T> {
        Some(inner)
    }

    fn apply<T, U, F>(value: Self::Type<T>, f: Self::Type<F>) -> Self::Type<U>
    where
        F: FnMut(T) -> U,
    {
        value.zip(f).map(|(v, mut f)| f(v))
    }
}

impl MonadFamily for OptionType {
    fn bind<T, U, F>(value: Self::Type<T>, f: F) -> Self::Type<U>
    where
        F: FnMut(T) -> Self::Type<U>,
    {
        value.and_then(f)
    }
}
```

好嘞，然后我们就可以通过OptionType这个“选择器”来表达、使用Option作为Functor, Applicative, Monad 的性质了。 怎么样，是不是打开了无数的新的可能性？



