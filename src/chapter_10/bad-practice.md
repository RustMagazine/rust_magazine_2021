# Rust 中常见的新手错误和不良实践：不良习惯

译者：柴杰

> [原文](https://adventures.michaelfbryan.com/posts/rust-best-practices/bad-habits/#not-leveraging-pattern-matching)

---

[TOC]

如果你有其它语言的编程经验，在切换到Rust时，你会带着以前的经验。通常情况下，这是很好的，因为这意味着你不用从头开始学习编程。然而，这也可能带来一些坏习惯，让你写出糟糕的代码。



## **使用哨兵值**

在大多数类C的编程语言（C、C#、Java等）中，表明某操作失败或无法找到的方式是返回一个 "特殊 "值。例如，C#的String.IndexOf()方法扫描一个数组中的某个元素，并返回其索引。如果没有找到，则返回-1。

这就导致了下面这样的代码：

```rust
string sentence = "The fox jumps over the dog";

int index = sentence.IndexOf("fox");

if (index != -1)
{
  string wordsAfterFox = sentence.SubString(index);
  Console.WriteLine(wordsAfterFox);
}
```

你经常可以看到这种 "用一个哨兵值来表示一些特殊的东西 “的做法，类似的哨兵值还有""或null（有人曾把它们称为 "十亿美元的错误"）。

这是一个坏主意，因为绝对没有什么可以阻止你忘记这个检查。这意味着你可能会因为一个错误的假设，或者生成哨兵的代码与使用它的代码距离较远，而意外地使程序崩溃。

不过在Rust中我们可以做得更好。只要使用Option!

在Rust中，如果不处理Option可能是None的情况，就无法获取有效值。这是编译器在编译时强制执行的，这意味着忘记检查的代码甚至不会被编译。

```rust
let sentence = "The fox jumps over the dog";
let index = sentence.find("fox");

// let words_after_fox = &sentence[index..]; // Error: Can't index str with Option<usize>

if let Some(fox) = index {
  let words_after_fox = &sentence[fox..];
  println!("{}", words_after_fox);
}
```



## **匈牙利记号法** 

在20世纪70年代，一种被称为 "匈牙利记号法 "的命名惯例开始在无类型或动态类型语言中使用。它的原理是在名字的开头添加一个助记符，以表示它所代表的内容，例如，布尔型访问变量可能被称为bVisited，字符串名称可能被称为strName。

你仍然可以在Delphi语言中看到这种命名惯例，类（类型）以T开头，字段以F开头，参数以A开头等。

```
type
 TKeyValue = class
  private
    FKey: integer;
    FValue: TObject;
  public
    property Key: integer read FKey write FKey;
    property Value: TObject read FValue write FValue;
    function Frobnicate(ASomeArg: string): string;
  end;
```

C#也有一个惯例，即所有的接口都应该以I开头，这意味着从C#切换到Rust的程序员有时也会在traits之前加上I。

```rust
trait IClone {
  fn clone(&self) -> Self;
}

let account_bytes: Vec<u8> = read_some_input();
let account_str = String::from_utf8(account_bytes)?;
let account: Account = account_str.parse()?;
```

在这种情况下，只要去掉前面的I就可以了。Rust的语法保证了不可能将trait和普通类型混淆，所以这种标记是多余的。这与C#形成了鲜明的对比，在C#中接口和类在很大程度上是可以互换的。

这在函数中也可以看到，人们在将某物从一种形式转换为另一种形式时，会为它重新命名。这些名字往往是愚蠢的或臆造的，给读者提供的额外信息可以忽略不计。

```rust
let account_bytes: Vec<u8> = read_some_input();
let account_str = String::from_utf8(account_bytes)?;
let account: Account = account_str.parse()?;
```

我的意思是，如果我们调用String::from_utf8()，我们已经知道account_str将是一个字符串，那么为什么还要加上_str后缀呢？

与其他很多语言不同的是，Rust鼓励你将变量从一种形式转化为另一种形式时对其进行隐式处理，特别是当之前的变量不再访问时（例如它被移动了）。

```rust
let account: Vec<u8> = read_some_input();
let account = String::from_utf8(account)?;
let account: Account = account.parse()?;
```

这可以说是一种优势，因为我们可以用同样的名字来表示同样的概念。

其他语言不喜欢隐式处理，因为它很容易失去对一个变量所包含的类型的跟踪（例如，在像JavaScript这样的动态类型语言中），或者容易引入错误（例如，程序员认为一个变量是某个类型，但它实际上包含其它东西）。

这两种情况，对于像Rust这样具有移动语义的强类型语言来说，都不太重要，所以你可以自由地使用隐式处理，不用担心会给自己带来麻烦。



## **大量的Rc<RefCell<T>>**

在面向对象的语言中，一个常见的模式是接受某个对象的引用，这样你就可以调用它的方法。

这本身并没有错，依赖注入是一件非常好的事情，但与大多数OO语言不同的是，Rust没有垃圾收集器，并且Rust对共享可变性有强烈的感受。

也许用一个例子会更容易理解。假设我们正在实现一个游戏，玩家需要殴打一群怪物，直到对它们造成一定量的伤害。

我们创建了一个怪物类，它有一个健康属性和一个takeDamage()方法，为了能够跟踪已经造成了多少伤害，我们将让人们提供回调，每当怪物受到伤害时就会调用。

```typescript
type OnReceivedDamage = (damageReceived: number) => void;

class Monster {
    health: number = 50;
    receivedDamage: OnReceivedDamage[] = [];

    takeDamage(amount: number) {
        amount = Math.min(this.health, amount);
        this.health -= amount;
        this.receivedDamage.forEach(cb => cb(amount));
    }

    on(event: "damaged", callback: OnReceivedDamage): void {
        this.receivedDamage.push(callback);
    }
}
```

让我们也创建一个DamageCounter类，追踪我们造成了多少伤害，并让我们知道什么时候达到了这个目标。

```typescript
class DamageCounter {
    damageInflicted: number = 0;

    reachedTargetDamage(): boolean {
        return this.damageInflicted > 100;
    }

    onDamageInflicted(amount: number) {
        this.damageInflicted += amount;
    }
}
```

现在我们将创建一些怪物，并不断造成随机数量的伤害，直到DamageCounter满意为止。

```typescript
const counter = new DamageCounter();

const monsters = [new Monster(), new Monster(), new Monster(), new Monster(), new Monster()];
monsters.forEach(m => m.on("damaged", amount => counter.onDamageInflicted(amount)));

while (!counter.reachedTargetDamage()) {
    // pick a random monster
    const index = Math.floor(Math.random()*monsters.length);
    const target = monsters[index];
    // then damage it a bit
    const damage = Math.round(Math.random() * 50);
    target.takeDamage(damage);

    console.log(`Monster ${index} received ${damage} damage`);
}
```

现在把这段代码用Rust编写，使用Box<dyn Fn(u32)>来表示接受单个u32参数的闭包（JavaScript中所有闭包默认都分配在堆上）。

```rust
type OnReceivedDamage = Box<dyn Fn(u32)>;

struct Monster {
    health: u32,
    received_damage: Vec<OnReceivedDamage>,
}

impl Monster {
    fn take_damage(&mut self, amount: u32) {
        let damage_received = cmp::min(self.health, amount);
        self.health -= damage_received;
        for callback in &mut self.received_damage {
            callback(damage_received);
        }
    }

    fn add_listener(&mut self, listener: OnReceivedDamage) {
        self.received_damage.push(listener);
    }
}

impl Default for Monster {
    fn default() -> Self {
        Monster { health: 100, received_damage: Vec::new() }
    }
}
```

接下来是DamageCounter。

```rust
#[derive(Default)]
struct DamageCounter {
    damage_inflicted: u32,
}

impl DamageCounter {
    fn reached_target_damage(&self) -> bool {
        self.damage_inflicted > 100
    }

    fn on_damage_received(&mut self, damage: u32) {
        self.damage_inflicted += damage;
    }
}
```

最后是关于造成伤害的代码。

```rust
fn main() {
    let mut rng = rand::thread_rng();
    let mut counter = DamageCounter::default();
    let mut monsters: Vec<_> = (0..5).map(|_| Monster::default()).collect();

    for monster in &mut monsters {
        monster.add_listener(Box::new(|damage| counter.on_damage_received(damage)));
    }

    while !counter.reached_target_damage() {
        let index = rng.gen_range(0..monsters.len());
        let target = &mut monsters[index];

        let damage = rng.gen_range(0..50);
        target.take_damage(damage);

        println!("Monster {} received {} damage", index, damage);
    }
}
```

当编译代码时，Rustc给出了4个关于monster.add_listener()的编译错误。

```rust
error[E0596]: cannot borrow `counter` as mutable, as it is a captured variable in a `Fn` closure
  --> src/main.rs:47:48
   |
47 |         monster.add_listener(Box::new(|damage| counter.on_damage_received(damage)));
   |                                                ^^^^^^^ cannot borrow as mutable

error[E0499]: cannot borrow `counter` as mutable more than once at a time
  --> src/main.rs:47:39
   |
47 |         monster.add_listener(Box::new(|damage| counter.on_damage_received(damage)));
   |                              ---------^^^^^^^^------------------------------------
   |                              |        |        |
   |                              |        |        borrows occur due to use of `counter` in closure
   |                              |        `counter` was mutably borrowed here in the previous iteration of the loop
   |                              cast requires that `counter` is borrowed for `'static`

error[E0597]: `counter` does not live long enough
  --> src/main.rs:47:48
   |
47 |         monster.add_listener(Box::new(|damage| counter.on_damage_received(damage)));
   |                              ------------------^^^^^^^----------------------------
   |                              |        |        |
   |                              |        |        borrowed value does not live long enough
   |                              |        value captured here
   |                              cast requires that `counter` is borrowed for `'static`
...
60 | }
   | - `counter` dropped here while still borrowed

error[E0502]: cannot borrow `counter` as immutable because it is also borrowed as mutable
  --> src/main.rs:50:12
   |
47 |         monster.add_listener(Box::new(|damage| counter.on_damage_received(damage)));
   |                              -----------------------------------------------------
   |                              |        |        |
   |                              |        |        first borrow occurs due to use of `counter` in closure
   |                              |        mutable borrow occurs here
   |                              cast requires that `counter` is borrowed for `'static`
...
50 |     while !counter.reached_target_damage() {
   |            ^^^^^^^ immutable borrow occurs here
```

这些编译问题可以归结为：

- 闭包捕获了一个对counter的引用

- counter.on_damage_received()方法需要&mut self，所以我们的闭包需要一个&mut引用。我们在一个循环中添加闭包，所以我们最终在同一时间对同一个对象获取多个&mut引用。

- listener 是一个没有生命周期参数的的盒式闭包，这意味着它拥有该变量的所有权。因为我们在循环中把counter移到闭包中，所以会出现 “use of moved value "的错误。

- 在把counter传递给add_listener()后，又试图在循环条件中使用它。

上述问题的典型解决办法是把DamageCounter包裹在一个引用计数的指针中，它允许同时存在多个句柄。然后因为我们还需要调用一个&mut self方法，所以需要一个RefCell来将借用检查从编译时"移动"到运行时。

```rust
fn main() {
     let mut rng = rand::thread_rng();
-    let mut counter = DamageCounter::default();
+    let mut counter = Rc::new(RefCell::new(DamageCounter::default()));
     let mut monsters: Vec<_> = (0..5).map(|_| Monster::default()).collect();

     for monster in &mut monsters {
-        monster.add_listener(Box::new(|damage| counter.on_damage_received(damage)));
+        let counter = Rc::clone(&counter);
+        monster.add_listener(Box::new(move |damage| {
+            counter.borrow_mut().on_damage_received(damage)
+        }));
     }

-    while !counter.reached_target_damage() {
+    while !counter.borrow().reached_target_damage() {
         let index = rng.gen_range(0..monsters.len());
         let target = &mut monsters[index];
         ...
     }
 }
```

嗯......这很有效。但这种方法往往会产生混乱，特别是当你在结构中存储一些非平凡的东西，如Rc<RefCell<Vec<Foo>>>>（或关于多线程的Arc<Mutex<Vec<Foo>>>>）。

这也为你提供了这样的情况：RefCell可能被多次借用，因为你的代码很复杂，而调用栈中更高的东西已经在使用RefCell。对于Mutex来说，这将导致一个死锁，而RefCell会引发panic，这两种情况都对程序的可靠性有负面影响。

一个更好的方法是改变你的API，不持有对其他对象的长期引用。根据不同的情况，在Monster::take_damage()方法中接受一个回调参数可能是明智的做法。

```rust
struct Monster {
    health: u32,
}

impl Monster {
    fn take_damage(&mut self, amount: u32, on_damage_received: impl FnOnce(u32)) {
        let damage_received = cmp::min(self.health, amount);
        self.health -= damage_received;
        on_damage_received(damage_received);
    }
}

impl Default for Monster {
  fn default() -> Self { Monster { health: 100 } }
}

...

fn main() {
    let mut rng = rand::thread_rng();
    let mut counter = DamageCounter::default();
    let mut monsters: Vec<_> = (0..5).map(|_| Monster::default()).collect();

    while !counter.reached_target_damage() {
        let index = rng.gen_range(0..monsters.len());
        let target = &mut monsters[index];

        let damage = rng.gen_range(0..50);
        target.take_damage(damage, |dmg| counter.on_damage_received(dmg));

        println!("Monster {} received {} damage", index, damage);
    }
}
```

这样做的一个好处是，我们摆脱了所有的回调管理模板代码，这意味着这个版本只有47行，而不是Rc<RefCell<_>>版本的62行。

在某些情况下，给take_damage()一个回调参数可能是不可接受的，此时你可以返回一个发生了什么的"摘要"，这样调用者就可以决定下一步该怎么做。

```rust
impl Monster {
    fn take_damage(&mut self, amount: u32) -> AttackSummary {
        let damage_received = cmp::min(self.health, amount);
        self.health -= damage_received;
        AttackSummary { damage_received }
    }
}

struct AttackSummary {
    damage_received: u32,
}

...

fn main() {
    let mut rng = rand::thread_rng();
    let mut counter = DamageCounter::default();
    let mut monsters: Vec<_> = (0..5).map(|_| Monster::default()).collect();

    while !counter.reached_target_damage() {
        let index = rng.gen_range(0..monsters.len());
        let target = &mut monsters[index];

        let damage = rng.gen_range(0..50);
        let AttackSummary { damage_received } = target.take_damage(damage);
        counter.on_damage_received(damage_received);

        println!("Monster {} received {} damage", index, damage);
    }
}
```

这是我的首选解决方案；根据经验，对于较大规模的代码或代码比较复杂的情况下，它往往效果很好。



## **使用错误的整数类型 **

编写大量C语言的导致的另一个坏习惯是使用错误的整数类型，并且对usize的频繁转换感到沮丧。

特别是在索引的时候，C语言的程序员都被教导要使用int来做索引和for-loop，所以当他们使用Rust，需要存储一个索引列表时，往往会使用Vec<i32>。然后他们就会感到沮丧，因为Rust在索引方面相当严格，像数组、切片和Vec这样的标准类型只能使用usize（相当于size_t）进行索引，这意味着他们的代码被从i32到usize的转换弄得一团糟。

为什么Rust只允许用usize进行索引，有许多合理的理由：

- 有一个负的索引是没有意义的（在一个片断开始之前访问项目是不合法的），所以我们可以通过用无符号整数做索引来避免bug。

- usize被定义为一个整数，其大小与普通指针相同，这意味着指针运算不会有任何隐藏的转换。

- std::mem::size_of() 和 std::mem::align_of() 函数返回usize类型。

由此观之，解决方案也显而易见。为你的应用选择正确的整数类型，当用于索引时，这个"正确的整数类型”很可能是usize。



## **不安全 - 我知道我在做什么 **

每当我看到一个灰头土脸的C程序员因为借用检查器一直拒绝他们的代码而去找原始指针或std::mem::transmute()时，我就会想到这句话。

你经常会看到有人想黑掉隐私，创建自我引用的结构，或者用不安全的方式创建全局可变的变量。经常会有这样的评论："但我知道这个程序只用一个线程，所以访问静态的Mut就可以了 "或者 "但这在C语言中完全可以工作"。

现实情况是，不安全代码是有细微差别的，你需要对Rust的借用检查规则和内存模型有良好的直觉。我不想当守门员，说 "你必须有这么高的身高才能写出多线程的不安全代码"，但是如果你是这个语言的新手，你很有可能没有这种直觉，而且会让你自己和你的同事承受很大的痛苦。

如果你想更多的了解Rust，或者你知道自己在做什么并合法地使用它，那么玩玩不安全的东西是可以的，但不安全的东西并不是一个神奇的避难所，它可以让编译器停止抱怨，让你用Rust语法写C语言。



## **不使用命名空间 **

在C语言中，一个常见的做法是在函数前加上库或模块的名字，以帮助读者了解它来自哪里，并避免重复符号错误（例如rune_wasmer_runtime_load()）。

然而，Rust有真正的命名空间，并允许你将方法附加到类型上（例如，rune::wasmer::Runtime::load()）。



## **过度使用切片索引 **

for-loop和索引在类C语言中的使用频繁很高。

```rust
let points: Vec<Coordinate> = ...;
let differences = Vec::new();

for i in 1..points.len() [
  let current = points[i];
  let previous = points[i-1];
  differences.push(current - previous);
]
```

然而，在使用索引时，即使是经验丰富的程序员也不能避免引入错误（例如，我需要记住从1开始循环并减去1以获得前一个点）。

在这种情况下，Rust鼓励你用迭代器。切片类型甚至带有高级工具，如windows()和array_windows()方法，让你在相邻的元素对上进行迭代。

```rust
let points: Vec<Coordinate> = ...;
let mut differences = Vec::new();

for [previous, current] in points.array_windows().copied() {
  differences.push(current - previous);
}
```

你甚至可以删除for-loop 和 differences变量。

```rust
let differences: Vec<_> = points
  .array_windows()
  .copied()
  .map(|[previous, current]| current - previous)
  .collect();
```

有些人认为带有map()和collect()的版本更干净或更 "实用"，但我会让你自己来判断。

而且迭代器通常有更好的性能，因为检查可以作为循环条件的一部分来完成，而不是单独进行。



## **过度使用迭代器** 

过度使用迭代器也会产生问题：当你拥有的只是一把锤子时，一切看起来都像钉子。

长长的map()、filter()和and_then()调用链会让人很难读懂并追踪到实际发生的事情，尤其是当类型推理让你省略闭包参数的类型时。

其他时候，你基于迭代器的解决方案只是不必要的复杂。作为一个例子，看一下这段代码，看看你是否能弄清楚它要做什么。

```rust
pub fn functional_blur(input: &Matrix) -> Matrix {
    assert!(input.width >= 3);
    assert!(input.height >= 3);

    // Stash away the top and bottom rows so they can be
    // directly copied across later
    let mut rows = input.rows();
    let first_row = rows.next().unwrap();
    let last_row = rows.next_back().unwrap();

    let top_row = input.rows();
    let middle_row = input.rows().skip(1);
    let bottom_row = input.rows().skip(2);

    let blurred_elements = top_row
        .zip(middle_row)
        .zip(bottom_row)
        .flat_map(|((top, middle), bottom)| blur_rows(top, middle, bottom));

    let elements: Vec<f32> = first_row
        .iter()
        .copied()
        .chain(blurred_elements)
        .chain(last_row.iter().copied())
        .collect();

    Matrix::new_row_major(elements, input.width, input.height)
}

fn blur_rows<'a>(
    top_row: &'a [f32],
    middle_row: &'a [f32],
    bottom_row: &'a [f32],
) -> impl Iterator<Item = f32> + 'a {
    // stash away the left-most and right-most elements so they can be copied across directly.
    let &first = middle_row.first().unwrap();
    let &last = middle_row.last().unwrap();

    // Get the top, middle, and bottom row of our 3x3 sub-matrix so they can be
    // averaged.
    let top_window = top_row.windows(3);
    let middle_window = middle_row.windows(3);
    let bottom_window = bottom_row.windows(3);

    // slide the 3x3 window across our middle row so we can get the average
    // of everything except the left-most and right-most elements.
    let averages = top_window
        .zip(middle_window)
        .zip(bottom_window)
        .map(|((top, middle), bottom)| top.iter().chain(middle).chain(bottom).sum::<f32>() / 9.0);

    std::iter::once(first)
        .chain(averages)
        .chain(std::iter::once(last))
```

信不信由你，但这是我见过的可读性较强的版本之一。现在我们来看看命令式的实现。

```rust
pub fn imperative_blur(input: &Matrix) -> Matrix {
    assert!(input.width >= 3);
    assert!(input.height >= 3);

    // allocate our output matrix, copying from the input so
    // we don't need to worry about the edge cases.
    let mut output = input.clone();

    for y in 1..(input.height - 1) {
        for x in 1..(input.width - 1) {
            let mut pixel_value = 0.0;

            pixel_value += input[[x - 1, y - 1]];
            pixel_value += input[[x, y - 1]];
            pixel_value += input[[x + 1, y - 1]];

            pixel_value += input[[x - 1, y]];
            pixel_value += input[[x, y]];
            pixel_value += input[[x + 1, y]];

            pixel_value += input[[x - 1, y + 1]];
            pixel_value += input[[x, y + 1]];
            pixel_value += input[[x + 1, y + 1]];

            output[[x, y]] = pixel_value / 9.0;
        }
    }

    output
}
```

我喜欢这个可读性好的版本。



## **不使用模式匹配** 

在大多数其他主流语言中，程序员在做一个可能抛出异常的操作之前写一个检查是很常见的。我们前面的C# IndexOf()片段就是一个很好的例子。

```rust
int index = sentence.IndexOf("fox");

if (index != -1)
{
  string wordsAfterFox = sentence.SubString(index);
  Console.WriteLine(wordsAfterFox);
}
```

你可能会看到这样的代码：

```rust
let opt: Option<_> = ...;

if opt.is_some() {
  let value = opt.unwrap();
  ...
}
```

或者

```rust
let list: &[f32] = ...;

if !list.is_empty() {
  let first = list[0];
  ...
}
```

现在这两个片段都是完全有效的代码，而且永远不会失败，但与哨兵值类似，在未来重构代码时容易引入错误。

使用模式匹配和Option可以帮助你避免这种情况，因为它可以确保你访问一个值的唯一方法是它是有效的。

```rust
if let Some(value) = opt {
  ...
}

if let [first, ..] = list {
  ...
}
```



## **构造后初始化** 

在许多语言中，调用对象的构造函数并在之后初始化其字段是很正常的（手动或通过调用一些init() 方法）。然而，这违背了Rust "使无效状态无法表示” 的一般惯例。

假设你正在编写一个NLP应用程序，并且有一个包含所有可能的单词的字典。

下面是创建字典的一种方式：

```rust
let mut dict = Dictionary::new();
// read the file and populate some internal HashMap or Vec
dict.load_from_file("./words.txt")?;
```

然而，这样写Dictionary意味着它现在有两个（隐藏的）状态--空和已填充。

所有使用 Dictionary 的下游代码都会认为它已经被填充了，并据此编写代码。如果把一个空的 dictionary 传递给期望有一个填充的 dictionary 的代码，可能会引发恐慌。比如用 dict["word"]索引到字典中，如果 "word "不在那里，就会引发panic。

但这是完全没有必要的。只要确保字典在构建后立即可用，而不是在事后填充它。

```rust
let dict = Dictionary::from_file("./words.txt")?;

impl Dictionary {
  fn from_file(filename: impl AsRef<Path>) -> Result<Self, Error> {
    let text = std::fs::read_to_string(filename)?;
    let mut words = Vec::new();
    for line in text.lines() {
      words.push(line);
    }
    Ok(Dictionary { words })
  }
}
```

Dictionary::from_file()可能会创建一个空的Vec，并逐步填充它，但它还不会被存储在Dictionary的字段中，所以不存在它被填充和有用的假设。

你陷入这种反模式的频率在很大程度上取决于你的背景和编码风格。

函数式语言通常是不可变的，所以你会很自然地使用惯用模式。毕竟，当你不允许改变任何东西的时候，要创建一个半初始化的东西并在以后填充它是有些困难的。

另一方面，面向对象语言允许在对象构建完成后对其进行初始化，特别是因为对象引用默认为空，而且他们对可变性没有任何顾虑。这会导致面向对象语言容易出现 NullPointerException。

## **防御性拷贝** 

不可变对象的一个非常好的属性是不可变。然而，在像Python和Java这样的语言中，不可变性不是传递性的--也就是说，如果x是一个不可变的对象，x.y也不能保证是不可变的，除非它被明确定义为不可变的。

这意味着有可能写出这样的代码。

```rust
class ImmutablePerson:
  def __init__(self, name: str, age: int, addresses: List[str]):
    self._name = name
    self._age = age
    self._addresses = addresses

  # read-only properties
  @property
  def name(self): return self._name
  @property
  def age(self): return self._age
  @property
  def addresses(self): return self._addresses
```

然后有人来了，不小心把地址列表弄乱了。

```rust
def send_letters(message: str, addresses: List[str]):
  # Note: the post office's API only works with with uppercase letters so we
  # need to pre-process the address list
  for i, address in enumerate(addresses):
    addresses[i] = addresses.upper()

  client = PostOfficeClient()
  client.send_bulk_mail(message, addresses)


person = ImmutablePerson("Joe Bloggs", 42, ["123 Fake Street"])

send_letters(
  f"Dear {person.name}, I Nigerian prince. Please help me moving my monies.",
  person.addresses
)

print(person.addresses) # ["123 FAKE STREET"]
```

虽然我承认这个例子有点矫揉造作，但函数修改它们所给的参数是很常见的。通常这是好的，但是当你的ImmutablePerson假设它的地址字段永远不会改变时，项目另一边的一些随机代码在你不知道的情况下修改它是令人讨厌的。

典型的解决方案是预先复制列表，这样即使调用者试图改变它的内容，他们也会改变一个副本而不是原来的地址域。

```rust
class ImmutablePerson:
  ...

  @property
  def addresses(self): return self._addresses.copy()
```

一般来说，你会看到防御性副本被用在任何想要确保另一段代码不会在不恰当的时候修改某个共享对象的地方。

考虑到这是一篇关于Rust的文章，你可能已经猜到了这是什么根源--别名和可变性的组合。

你可能也猜到了为什么在编写Rust代码时，防御性拷贝并不是真正必要的 —— 生命期和引用的 "shared immutable XOR single mutable" 规则意味着，如果不首先向其原始所有者请求可变访问或通过使用std::sync::Mutex<T>这样的类型明确选择共享变异，代码就不可能修改某物。



## **结论** 

还有一堆其他的坏习惯，我还没来得及触及，或者因为我想不出一个简洁的例子，所以没有包括在内。最后感谢所有回复我的帖子并提出建议的人。