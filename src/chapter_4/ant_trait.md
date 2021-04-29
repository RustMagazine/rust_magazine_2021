# 蚂蚁集团 | Trait 使用及实现分析

作者：Jiacai Liu / 后期编辑：张汉东

---

- [使用方式](#org2ef86fc)
  - [静态派发](#org8e6944f)
  - [动态派发](#org701d455)
  - [impl trait](#org477c7d4)
- [常见问题](#orgd875cd7)
  - [向上转型（upcast）](#orgd1f35e7)
  - [Object safety](#org9b7752f)
- [总结](#org414df86)
- [参考](#orge4c5893)

在 Rust 设计目标中，零成本抽象是非常重要的一条，它让 Rust 具备高级语言表达能力的同时，又不会带来性能损耗。零成本的基石是范型与 trait，它们可以在编译期把高级语法编译成与高效的底层代码，从而实现运行时的高效。这篇文章就来介绍 trait，包括使用方式与两个常见问题的分析，在问题探究的过程中来阐述其实现原理。


<a id="org2ef86fc"></a>

# 使用方式

Trait 的主要作用是用来抽象行为，类似于其他编程语言中的「接口」，这里举一示例阐述 trait 的主要使用方式：

```rust
trait Greeting {
    fn greeting(&self) -> String;
}

struct Cat;
impl Greeting for Cat {
    fn greeting(&self) ->String {
        "Meow...".to_string()
    }
}

struct Dog;
impl Greeting for Dog {
    fn greeting(&self) ->String {
        "Bark...".to_string()
    }
}
```

在上述代码中，定义了一个 trait Greeting，两个 struct 实现了它，根据函数调用方式，主要两种使用方式：

-   基于范型的静态派发
-   基于 trait object 的动态派发

范型的概念比较常见，这里着重介绍下 trait object 的[定义](https://doc.rust-lang.org/1.51.0/reference/types/trait-object.html)：

> A trait object is an opaque value of another type that implements a set of traits. The set of traits is made up of an object safe base trait plus any number of auto traits.

比较重要的一点是 trait object 属于 [Dynamically Sized Types](https://doc.rust-lang.org/1.51.0/reference/dynamically-sized-types.html)（DST），在编译期无法确定大小，只能通过指针来间接访问，常见的形式有 `Box<dyn trait>` `&dyn trait` 等。

```rust
fn print_greeting_static<G: Greeting>(g: G) {
    println!("{}", g.greeting());
}
fn print_greeting_dynamic(g: Box<dyn Greeting>) {
    println!("{}", g.greeting());
}

print_greeting_static(Cat);
print_greeting_static(Dog);

print_greeting_dynamic(Box::new(Cat));
print_greeting_dynamic(Box::new(Dog));

```


<a id="org8e6944f"></a>

## 静态派发

在 Rust 中，范型的实现采用的是单态化（monomorphization），会针对不同类型的调用者，在编译时生成不同版本的函数，所以范型也被称为[类型参数](https://bluejekyll.github.io/blog/posts/type-parameters/)。好处是没有虚函数调用的开销，缺点是最终的二进制文件膨胀。在上面的例子中， `print_greeting_static` 会编译成下面这两个版本：

```rust
print_greeting_static_cat(Cat);
print_greeting_static_dog(Dog);
```


<a id="org701d455"></a>

## 动态派发

不是所有函数的调用都能在编译期确定调用者类型，一个常见的场景是 GUI 编程中事件响应的 callback，一般来说一个事件可能对应多个 callback 函数，而这些 callback 函数都是在编译期不确定的，因此范型在这里就不适用了，需要采用动态派发的方式：

```rust
trait ClickCallback {
    fn on_click(&self, x: i64, y: i64);
}

struct Button {
    listeners: Vec<Box<dyn ClickCallback>>,
}
```


<a id="org477c7d4"></a>

## impl trait

在 Rust 1.26 版本中，引入了一种新的 trait 使用方式，即：[impl trait](https://doc.rust-lang.org/edition-guide/rust-2018/trait-system/impl-trait-for-returning-complex-types-with-ease.html)，可以用在两个地方：函数参数与返回值。 该方式主要是简化复杂 trait 的使用，算是范型的特例版，因为在使用 impl trait 的地方，也是静态派发，而且作为函数返回值时，数据类型只能有一种，这一点要尤为注意！

```rust
fn print_greeting_impl(g: impl Greeting) {
    println!("{}", g.greeting());
}
print_greeting_impl(Cat);
print_greeting_impl(Dog);

// 下面代码会编译报错
fn return_greeting_impl(i: i32) -> impl Greeting {
    if i > 10 {
        return Cat;
    }
    Dog
}

// | fn return_greeting_impl(i: i32) -> impl Greeting {
// |                                    ------------- expected because this return type...
// |     if i > 10 {
// |         return Cat;
// |                --- ...is found to be `Cat` here
// |     }
// |     Dog
// |     ^^^ expected struct `Cat`, found struct `Dog`
```


<a id="orgd875cd7"></a>

# 常见问题


<a id="orgd1f35e7"></a>

## 向上转型（upcast）

对于 `trait SubTrait: Base` ，在目前的 Rust 版本中，是无法将 `&dyn SubTrait` 转换到 `&dyn Base` 的。这个限制与 trait object 的内存结构有关。

在 [Exploring Rust fat pointers](https://iandouglasscott.com/2018/05/28/exploring-rust-fat-pointers/) 一文中，该作者通过 transmute 将 trait object 的引用转为两个 usize，并且验证它们是指向数据与函数虚表的指针：

```rust
use std::mem::transmute;
use std::fmt::Debug;

fn main() {
    let v = vec![1, 2, 3, 4];
    let a: &Vec<u64> = &v;
    // 转为 trait object
    let b: &dyn Debug = &v;
    println!("a: {}", a as *const _ as usize);
    println!("b: {:?}", unsafe { transmute::<_, (usize, usize)>(b) });
}

// a: 140735227204568
// b: (140735227204568, 94484672107880)
```

从这里可以看出：Rust 使用 fat pointer（即两个指针） 来表示 trait object 的引用，分布指向 data 与 vtable，这和 Go 中的 [interface](https://research.swtch.com/interfaces) 十分类似。

![img](https://img.alicdn.com/imgextra/i2/581166664/O1CN01esAA7q1z6A3inQpnF_!!581166664.jpg "trait object reference")

```rust
pub struct TraitObjectReference {
    pub data: *mut (),
    pub vtable: *mut (),
}

struct Vtable {
    destructor: fn(*mut ()),
    size: usize,
    align: usize,
    method: fn(*const ()) -> String,
}
```

尽管 fat pointer 导致指针体积变大（无法使用 Atomic 之类指令），但是好处是更明显的：

1.  可以为已有类型实现 trait（比如 [blanket implementations](https://users.rust-lang.org/t/what-are-blanket-implementations/49904)）
2.  调用虚表中的函数时，只需要引用一次，而在 C++ 中，vtable 是存在对象内部的，导致每一次函数调用都需要两次引用，如下图所示：
    
    ![img](https://img.alicdn.com/imgextra/i2/581166664/O1CN01u6ms841z6A3cHRdJw_!!581166664.jpg "cpp vtable two-level indirect")

如果 trait 有继承关系时，vtable 是怎么存储不同 trait 的方法的呢？在目前的实现中，是依次存放在一个 vtable 中的，如下图：

![img](https://img.alicdn.com/imgextra/i4/581166664/O1CN01x8adaQ1z6A3bkyKqY_!!581166664.png "多 trait 时 vtable 示意图")

可以看到，所有 trait 的方法是顺序放在一起，并没有区分方法属于哪个 trait，这样也就导致无法进行 upcast，社区内有 [RFC 2765](https://github.com/rust-lang/rfcs/issues/2765) 在追踪这个问题，感兴趣的读者可参考，这里就不讨论解决方案了，介绍一种比较通用的[解决方案](http://stackoverflow.com/a/28664881/403742)，通过引入一个 AsBase 的 trait 来解决：

```rust
trait Base {
    fn base(&self) {
        println!("base...");
    }
}

trait AsBase {
    fn as_base(&self) -> &dyn Base;
}

// blanket implementation
impl<T: Base> AsBase for T {
    fn as_base(&self) -> &dyn Base {
        self
    }
}

trait Foo: AsBase {
    fn foo(&self) {
        println!("foo..");
    }
}

#[derive(Debug)]
struct MyStruct;

impl Foo for MyStruct {}
impl Base for MyStruct {}

fn main() {
    let s = MyStruct;
    let foo: &dyn Foo = &s;
    foo.foo();
    let base: &dyn Base = foo.as_base();
    base.base();
}
```


<a id="org9b7752f"></a>

## Object safety

在 Rust 中，并不是所有的 trait 都可用作 trait object，需要满足一定的条件，称之为 [object safety](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#object-safety-is-required-for-trait-objects) 属性。主要有以下几点：

1.  函数返回类型不能是 Self（即当前类型）。这主要因为把一个对象转为 trait object 后，原始类型信息就丢失了，所以这里的 Self 也就无法确定了。
2.  函数中不允许有范型参数。主要原因在于单态化时会生成大量的函数，很容易导致 trait 内的方法膨胀。比如
    
    ```rust
    trait Trait {
        fn foo<T>(&self, on: T);
        // more methods
    }
    
    // 10 implementations
    fn call_foo(thing: Box<Trait>) {
        thing.foo(true); // this could be any one of the 10 types above
        thing.foo(1);
        thing.foo("hello");
    }
    
    // 总共会有 10 * 3 = 30 个实现
    ```
3.  Trait 不能继承 Sized。这是由于 Rust 会默认为 trait object 实现该 trait，生成类似下面的代码：
    
    ```rust
    trait Foo {
        fn method1(&self);
        fn method2(&mut self, x: i32, y: String) -> usize;
    }
    
    // autogenerated impl
    impl Foo for TraitObject {
        fn method1(&self) {
            // `self` is an `&Foo` trait object.
    
            // load the right function pointer and call it with the opaque data pointer
            (self.vtable.method1)(self.data)
        }
        fn method2(&mut self, x: i32, y: String) -> usize {
            // `self` is an `&mut Foo` trait object
    
            // as above, passing along the other arguments
            (self.vtable.method2)(self.data, x, y)
        }
    }
    ```
    
    如果 Foo 继承了 Sized，那么就要求 trait object 也是 Sized，而 trait object 是 DST 类型，属于 `?Sized` ，所以 trait 不能继承 Sized。
    
    对于非 safe 的 trait，能修改成 safe 是最好的方案，如果不能，可以尝试范型的方式。


<a id="org414df86"></a>

# 总结

本文开篇就介绍了 trait 是实现零成本抽象的基础，通过 trait 可以为已有类型增加新方法，这其实解决了[表达式问题](https://en.wikipedia.org/wiki/Expression_problem)，可以进行运算符重载，可以进行面向接口编程等。希望通过本文的分析，可以让读者更好的驾驭 trait 的使用，在面对编译器错误时，能够做到游刃有余。


<a id="orge4c5893"></a>

# 参考

-   [想要改变世界的 Rust 语言](https://www.infoq.cn/article/Uugi_eIJusEka1aSPmQM)
-   [Abstraction without overhead: traits in Rust](https://blog.rust-lang.org/2015/05/11/traits.html)
-   [Peeking inside Trait Objects](http://huonw.github.io/blog/2015/01/peeking-inside-trait-objects/)
-   [Object Safety](http://huonw.github.io/blog/2015/01/object-safety)
-   [Interface Dispatch](https://lukasatkinson.de/2018/interface-dispatch/)
-   [3 Things to Try When You Can't Make a Trait Object](https://www.possiblerust.com/pattern/3-things-to-try-when-you-can-t-make-a-trait-object)

<a id="org9f48097"></a>

# 关于我们

我们是蚂蚁智能监控技术中台的时序存储团队，我们正在使用 Rust 构建高性能、低成本并具备实时分析能力的新一代时序数据库，欢迎加入或者推荐，目前我们也正在寻找优秀的实习生，也欢迎广大应届同学来我们团队实习，请联系：jiachun.fjc@antgroup.com