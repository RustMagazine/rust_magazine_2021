# 「译」Arenas in Rust

原文: [Arenas in Rust](https://manishearth.github.io/blog/2021/03/15/arenas-in-rust/)

译者：[MATRIXKOO](https://github.com/MATRIXKOO)



# Rust 中的 Arenas 内存池

最近有一些关于 Rust 中 `Arenas` 的讨论，我想我应该写一篇关于这个文章。

在 Rust 中，`Arenas` 并不是那种 "典型的" 问题，所以很少有人了解它们。只会在各种用例的应用程序中有看到它们。一般来说，你只需要调包就行了，没必要为它使用 `unsafe` 。因此无需专门去学会写它，但这些知识也不是无用的，尤其是对有使用 `Arenas` 的人来说更是如此 。

此外，我在实现自引用的 `Arenas` 时会涉及到一系列非常*酷*的 `lifetime`操作 ，我以前完全没写过。

我主要是为了写一些*很酷*的生命周期效应而写的，但是我认为有必要写一篇针对所有 `Rustaceans` 的介绍。如果你已经知道什么是 `Arenas`，并且想看到一些很炫酷的 `lifetime` 技巧，则可以直接跳到[这里](https://manishearth.github.io/blog/2021/03/15/arenas-in-rust/#implementing-a-self-referential-arena)阅读。

## 什么是 arena ?

 `Arenas` 本质上是一种将预期生命周期相同的内存进行分组的模式。比如说有时需要在某个生命周期内分配一堆对象，之后这些对象将被全部销毁。每次都调用系统分配器效率低下，更可取的是为对象预分配一堆内存，并在完成处理后立即将其全部清理干净。
 >译者注: 没错就是 cache

从广义上讲，使用 `Arenas`  有以下两个原因：

首先，如上所述，使用它主要目标可能是减轻内存消耗。例如，在游戏或应用程序中，可能存在大量需要逐帧分配，使用完立即销毁的情况。特别是在游戏开发中，这非常普遍，而内存压力是游戏开发人员比较关心的事情。使用  `Arenas`，可以轻松地分配一个 `Arena`，在每个帧中将其填满，并在帧结束后将其清空。缓存局部性还能其他好处：可以确保大多数每帧对象在帧期间（可能比其他对象使用得更多）位于缓存中，因为它们是相邻分配的。

另一个原因可能是用于编写自引用的数据，例如带有环的复杂图，使用此方法数据可以一次全部清除。例如，在编写编译器时，`type`信息可能将需要引用其他类型或者其他数据，从而导致复杂的，可能是 `type` 的有环图。一旦推导出类型，就不需要专门销毁它了，因此可以使用一个 `Arenas` 来存储所有计算出的类型信息，当类型信息无关紧要时，就可以直接将他们一次性清楚。使用这种模式可以让代码不必担心自引用位是否会 “提前” 释放，它可以保证，如果存在一个 `Ty` ，则它的寿命与其他所有 `Ty` 一样长，并且可以直接引用它们。
> 译者注: 不会导致空引用

这两个目标不一定是相互联系的：可以使用一个 `Arenas`来同时实现两个目标。但是，也可以拥有一个禁止使用自我引用类型的 `Arenas`（有得有失，你能得到一些好处）。在本文的稍后部分，我将实现一个允许自我引用类型但对减轻内存分配压力影响不大的  `Arenas`，主要是为了易于实现。*通常*，如果要为编写用于自引用类型 的 `Arenas`，则可以使其同时减少分配器压力，但可能会有所取舍。 

## 在 Rust 中如何使用 arena ?
一般来说，要使用 arena ，只需要调包。 我简单的搜索一下现有的 `Arenas` 实现，在[这里](https://crates.io/search?q=arena)。下面我会介绍两个我已经了解的库，但我只是挂 "二" 漏万。

要指出的是，如果只需要环结构，而不必使用 `arenas` ，那么出色的 [petgraph](https://docs.rs/petgraph/) 通常就足够了。 [slotmap](https://docs.rs/slotmap/) 也很不错； 它是类似地图的数据结构，可用于基于分代索引的自引用数据。 



### Bumpalo

`Bumpalo` 是一种快速的 `bump allocator[1]`，它允许异构的内容，并且仅在不关心析构函数运行的情况下才允许循环。
> 参见: [1]https://blog.codingnow.com/2013/11/bump_pointer_allocator.html

```rust
use bumpalo::Bump;

// (example slightly modified from `bumpalo` docs)

// Create a new arena to bump allocate into.
let bump = Bump::new();

// Allocate values into the arena.
let scooter = bump.alloc(Doggo {
    cuteness: u64::max_value(),
    age: 8,
    scritches_required: true,
});

// Happy birthday, Scooter!
scooter.age += 1;
```

每次对 `Bump::alloc()` 的调用都会返回对已分配对象的可变引用。 这可以分配不同的对象，它们甚至可以相互引用 (不用环的话,借用检查就会强制这样做) 。 默认情况下，它不会在其内容上调用析构函数。 但是，可以使用 `bumpalo::boxed`（或Nightly上的自定义分配器）来实现这个效果。 可以类似地使用 `bumpalo::collections`来获取 `bumpalo` 支持的向量和字符串。` bumpalo::boxed` 将不允许自引用。 
x
### typed-arena

［typed-arena](https://docs.rs/typed-arena/)是一个 `areana` 分配器，它只能存储单一类型的对象，但是就可以循环引用： 
```rust
// Example from typed-arena docs

use std::cell::Cell;
use typed_arena::Arena;

struct CycleParticipant<'a> {
    other: Cell<Option<&'a CycleParticipant<'a>>>,
}

let arena = Arena::new();

let a = arena.alloc(CycleParticipant { other: Cell::new(None) });
let b = arena.alloc(CycleParticipant { other: Cell::new(None) });

// mutate them after the fact to set up a cycle
a.other.set(Some(b));
b.other.set(Some(a));
```
和 `bumpalo` 不同的是，`typed-arena` 当 `arena` 本身超出范围时，就会使用析构函数
>你或许想知道，析构函数在引用数据上的安全性–毕竟，无论哪一个变量被第二次销毁，析构函数都会读到悬挂的引用。 我们将在文章的后面对此进行介绍，但这与 `drop` 检查有关，特别是如果尝试自引用时，则 arena 元素 本身允许的唯一显式析构函数将是带有适当标记类型的析构函数。 

## 实现一个支持自引用的 arena

写自引用代码是很有趣的，因为 Rust 非常警惕自我参照数据。 但是 `areana` 可以让你清楚地将“我不在乎此对象”和“可以删除此对象”阶段分开，以允许自引用和循环类型出现。

>人们很少需要实现自己的 arena，Bumpalo和Typedarena涵盖了大多数使用场景，实在没办法的话不妨先在 crates.io上 找一下。 但是，如果你的确需要直接实现的话，或者对具体的生命周期细节感兴趣，那么本节适合你。 

在以下规则中实现输入条目为Entry的竞技场Arena的关键：

- `Arena` 和 `Entry` 都应具有生命周期参数：`Arena <'arena>` 和 `Entry <'arena>`
- `Arena` 方法都应将 `Arena <'arena>` 接收为 `＆'arena` 自身，即其自身类型为`＆'arena Arena <'arena>`
- `Entry`几乎应该始终以 `＆'arena Entry <'arena>` 的形式传递（为此创建别名非常有用）
- 使用内部可变性； `Arena`上的 `＆mut self` 将使所有代码停止编译。 如果使用 `unsafe`的可变性，请确保 `RefCell <Entry <'arena >>`  具有 [PhantomData](https://doc.rust-lang.org/std/marker/struct.PhantomData.html) 。

从生命周期的角度来看基本上就是这样，剩下的全部就是确定所需的 API 。 掌握了以上规则，只要确保定义区域与所需的保证一起使用，就不必了解底层生命周期的状况。

让我们看一个 实现，然后剖析其工作原理。 

###　实现
我的库 [elsa](https://docs.rs/elsa) 在其中一个示例中使用 100％ `safe` 的代码实现了一个 `arena` 。由于 `elsa :: FrozenVec` 要求其内容位于间接引用之后，因此该 `arena` 无法节省分配，并且它不是通用的，但这是一种合理的方式来说明生命周期的工作方式，而无需陷入 使用 `unsafe` 带来的麻烦之中。

该示例实现了 `Person <'arena>` 类型的 `Arena` ，`Arena <'arena>` 。目标是实现某种可能有环的有向社交图。

```rust
use elsa::FrozenVec;

struct Arena<'arena> {
    people: FrozenVec<Box<Person<'arena>>>,
}
```
[elsa::FrozenVec](https://docs.rs/elsa/1.4.0/elsa/vec/struct.FrozenVec.html)是类似于 `Vec` 的仅支持追加内容的抽象，可让你调用`push` 而不需要传入可变的引用，这是只使用 `safe` 的一个实现。

每个 `Person <'arena>` 都有一个他们关注的人的列表，但也跟踪他们关注的人： 

```rust
struct Person<'arena> {
    pub follows: FrozenVec<PersonRef<'arena>>,
    pub reverse_follows: FrozenVec<PersonRef<'arena>>,
    pub name: &'static str,
}

// following the rule above about references to entry types
type PersonRef<'arena> = &'arena Person<'arena>;
```

这个生命周期 `arena`  其实是 “arena本身的生命周期”。 从这开始事情就变得奇怪起来了：通常，如果一个有生命周期参数，则调用者可以选择其中的内容。 不必只是说“这是对象本身的生命周期”，调用者通常可以根据需要实例化 `arena <'static>` 或为某个 `'a` 实例化 `Arena <'a>` 。 但是在这里，我们声明 “` 'arena` 是 `arena` 自身的生命周期" ； 很明显，一定有东西不太对。

这是我们实际实现的地方： 

```rust

impl<'arena> Arena<'arena> {
    fn new() -> Arena<'arena> {
        Arena {
            people: FrozenVec::new(),
        }
    }
    
    fn add_person(&'arena self, name: &'static str,
                  follows: Vec<PersonRef<'arena>>) -> PersonRef<'arena> {
        let idx = self.people.len();
        self.people.push(Box::new(Person {
            name,
            follows: follows.into(),
            reverse_follows: Default::default(),
        }));
        let me = &self.people[idx];
        for friend in &me.follows {
            // We're mutating existing arena entries to add references,
            // potentially creating cycles!
            // 把每一个元素都加上了 引用 , 很有可能导致循环引用
            friend.reverse_follows.push(me)
        }
        me
    }

    fn dump(&'arena self) {
        // code to print out every Person, their followers, and the people who follow them
        // 打印出 `Person` ,他们的关注者 ,和关注的人
    }
}

```

注意 `add_person中的&'arena self`。

此处的很好的实现了,“如果A 关注了 B，然后B又关注A” 这种通常需要分开处理的情况，但这仅是示例。

最后，我们可以像这样使用 `arena` ： 
```rust
fn main() {
    let arena = Arena::new();
    let lonely = arena.add_person("lonely", vec![]);
    let best_friend = arena.add_person("best friend", vec![lonely]);
    let threes_a_crowd = arena.add_person("threes a crowd", vec![lonely, best_friend]);
    let rando = arena.add_person("rando", vec![]);
    let _everyone = arena.add_person("follows everyone", vec![rando, threes_a_crowd, lonely, best_friend]);
    arena.dump();
}
```
在这种情况下，所有 “可变性” 都发生在 `arena` 本身的实现中，但是此代码可能会将元素直接添加到 `follows/reverse_follows` 列表中，或者 `Person` 可能具有用于其他类型链接的 `RefCells` 。

### 生命周期是如何工作的

那么这是如何工作的呢？ 如前所述，在 Rust 中使用这样的抽象，调用者通常可以自由地根据其处理方式来设置生存期。 例如，如果 `HashMap<K，&'a str>`，则 `'a` 将根据你尝试插入的内容的生存期进行调整。

当构造 `Arena` 时，其生命周期确实仍然不受限制，我们可以通过检查以下强制约束生命周期的代码来进行测试。 (依然是能过编译的)

```rust
let arena: Arena<'static> = Arena::new();
```
当你想做点什么的时候,就会停止工作:
```rust
let arena: Arena<'static> = Arena::new();
let lonely = arena.add_person("lonely", vec![]);
```
```shell
error[E0597]: `arena` does not live long enough
  --> examples/mutable_arena.rs:5:18
   |
4  |     let arena: Arena<'static> = Arena::new();
   |                -------------- type annotation requires that `arena` is borrowed for `'static`
5  |     let lonely = arena.add_person("lonely", vec![]);
   |                  ^^^^^ borrowed value does not live long enough
...
11 | }
   | - `arena` dropped here while still borrowed
```

`add_person` 方法以某种方式强制将 `Arena` 的 `arena` 参数设置为自己的生命周期，从而对其进行约束（并且无法用类型注释将其强制约束为其他任何值）。 这是与 `add_person` 的`＆'arena` 自签名（即 self 是 `＆'arena Arena <'self>` ）的巧妙互动，以及`'Arena in Arena <'arena>` 是[不变的生命周期](https://doc.rust-lang.org/nomicon/subtyping.html#variance)。 

通常，在 Rust 程序中，生命周期具有"伸缩性"。 以下代码可以通过编译：

```rust
// ask for two strings *with the same lifetime*
// 要求具有相同 生命周期的 string 
fn take_strings<'a>(x: &'a str, y: &'a str) {}

// string literal with lifetime 'static
// 要求具有 `'static' 生命周期的 ` string literal`  
let lives_forever = "foo";
// owned string with shorter, local lifetime
// 要求具有 `local` 生命周期
let short_lived = String::from("bar");

// still works!
// 能跑通
take_strings(lives_forever, &*short_lived);
```

 

在这段代码中，Rust 很高兴地注意到，虽然 `live_forever` 和 `&* short_lived` 具有不同的生命周期，但假装 `life_forever` 在 `take_strings` 函数的有效期内具有较短的生命周期是完全可以接受的。这只是引用，使用生命周期长的引用也适用于生命周期短的情况。

事实是，这种伸缩性并非对所用的生命周期都一样！[nomicon chapter on subtyping and variance](https://doc.rust-lang.org/nomicon/subtyping.html) 一章详细说明了为什么会这样，但一般的经验法则是，大多数生命周期都是“紧缩的” (更专业的说法是 `协变的` )，就像上面的`&a str` 中的一样，但是如果涉及某种形式的可变性，它们是不可变的，也称为“不变式”。如果使用的是函数类型，则具有 `弹性的` 生命周期 (即抗变的)，但是这种情况很少见。 

我们的 `Arena <'arena>` 使用内部可变性（通过 `FrozenVec`）使' `arena`不变。 让我们再次看一下两行代码。当编译器看到下面代码的第一行时，便会构建 `arena`，我们将其生命周期称为“ a”。此时 Arena 类型是 `Arena <'？>` ，其中的'？由表示形式表示，但生命周期不受限制。 

```rust
let arena = Arena::new(); 
let lonely = arena.add_person("lonely", vec![]);
```

让我们把生命周期写清楚一点

```rust
let arena = Arena::new(); // type Arena<'?>, lives for 'a

// explicitly write the `self` that gets constructed when you call add_person
// 显式写出 在调用 add_person 时的构建函数
let ref_to_arena = &arena; // type &'a Arena<'?>
let lonely = Arena::add_person(ref_to_arena, "lonely", vec![]);
```

还记得我之前列出的第二条规则吗？     

- Arena方法都应将Arena <'arena>接收为＆'arena自身，即其自身类型为＆'arena Arena <'arena> 我们遵循这条规则； 

`add_person` 的签名是 `fn add_person(&'arena self)`。这意味着 `ref_to_arena` 的生存期必须与 `&'arena Arena <'arena>` 模式匹配。目前，它的生命周期是`&'a Arena <'?>`，表示` '？`强制与'a相同，即 `arena` 变量本身的生存期。如果生命周期是可变的，则编译器可以压缩其他生存期来适配它，但它是不变的，并且不受限制的生存期被强制转变成一个确切的生命周期。

 通过这个巧妙的技巧，我们可以强制编译器将 `Arena <'arena>` 的生存期参数设置为其实例的生存期。 

在此之后，其余的工作就非常简单了。 `Arena <'arena>` 拥有类型为 `Person <'arena>`的元素，也就是说：“`Person` 被允许引用具有 `'arena` 生命周期的元素, 例如 `Arena` "。

`type PersonRef <'arena> =&'arena Person <'arena>`是“引用在 `Arena` 中并允许从其中引用对象 `Person` 的引用的便捷缩写。



### 析构器如何工作

到目前为止，我还没有讨论存在析构函数的情况下如何保证安全。 如果 `Arena` 具 有循环引用，并且编写了一个析构函数读取去这些循环引用，那么在销毁的过程中就会导致悬垂引用。 

这是 rust 十分模糊的地方。 除了“明确析构器巧妙地改变借用检查行为”，你没有什么需要必须去了解的。 但是了解这里的机制对建立一个更好的心智模型更有帮助。 

如果将以下代码添加到 `arena` 示例中： 

```rust
impl<'arena> Drop for Person<'arena> {
    fn drop(&mut self) {
        println!("goodbye {:?}", self.name);
        for friend in &self.reverse_follows {
            // potentially dangling!
            println!("\t\t{}", friend.name);
        }
    }
}
```

报错:

```shell
error[E0597]: `arena` does not live long enough
  --> examples/mutable_arena.rs:5:18
   |
5  |     let lonely = arena.add_person("lonely", vec![]);
   |                  ^^^^^ borrowed value does not live long enough
...
11 | }
   | -
   | |
   | `arena` dropped here while still borrowed
   | borrow might be used here, when `arena` is dropped and runs the destructor for type `Arena<'_>`
```

析构函数的存在在自引用数据的生命周期内巧妙地更改了借用检查器的行为。准确的规则是十分 `tricky`的，并在 [nomicon](https://doc.rust-lang.org/nomicon/dropck.html)中进行了解释，但实际上发生的是，在Person <'arena>上存在自定义析构函数后，`'person arena`（因为是Arena）的 '`'arena`变成了一个“在销毁时观测到的的生命周期"。然后在借用检查期间将其考虑在内---知道作用域末尾隐式调用`drop()`能够读取`'arena`的数据，Rust 做出了适当的结论，由于销毁本身是可变的操作，在销毁之后，调用`drop()` 读取内容是可行的 。 

当然，需要问的一个合理问题是，如果析构函数不允许用 `'arena` '“包装”数据，应该如何在 `arena` 中存储例如 `Box` 或者 `FrozenVec`之类的东西呢? 

原因是 Rust 知道 `Box::Drop `由于不清楚 `Person` 是什么,也不会去试图知道,而无法检查 `Person.follows`的内容。

当然凡事都有例外,由于析构器可以调用指定的 `trait` 方法(或者特化的方法)来告诉如何读取`Person`的内容,如果有一个随机的泛型类型提供了这种方法,就可以再次巧妙地更改借用检查器的行为。` stdlib` 类型和其他自定义数据结构通过转义填充`＃[may_dangle]`（也称为“ eyepatch” 毕竟析构器 "看不到" 生命周期）来实现这种目的，声明不会从生命周期或通用参数中读取自定义析构函数。 

这也适用于诸如  [typed-arena ](https://docs.rs/typed-arena/)之类的库；如果需要创建循环引用，则将无法在为放置在 `arena`  上的类型上编写自定义析构函数。但是只要避免创建循环引用就可以使用 `typed-arena` 编写自定义析构函数了；因此将无法使用内部可变性来使一个 `arena` 指向另一个 `arena`。 

感谢 [Mark Cohen](https://mpc.sh/) 和 [Nika Layzell](https://twitter.com/kneecaw/) 审阅了这篇文章的草稿。 