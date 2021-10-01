# Rust 生命周期和智能指针

作者： 王江同（华为）

本文是对 [第407篇推文](https://this-week-in-rust.org/blog/2021/09/08/this-week-in-rust-407/)，以及[第408篇推文](https://this-week-in-rust.org/blog/2021/09/15/this-week-in-rust-408/)推荐博文中有关于 Rus生命周期与智能指针内容的扩展和梳理。

---

## 生命周期

变量的有效范围被称为变量的生命周期。根据Rust the Book，生命周期的主要作用是避免指针失效/悬垂引用，因此，如果变量的引用超过了变量的生命周期，编译器会发现这个错误，并且拒绝编译文件。而当编译器无法确定引用是否有效时，编译器也会拒绝编译文件，此时编程者可以手动标注生命周期，承诺引用不会失效。例如，当函数返回变量引用时，如果编译器无法确定返回的引用生命周期应该和哪个输入参数保持一致，编译器就会报错，要求注明生命周期。

结构体中，如果成员是对某个类型的引用，则必须标注生命周期，表明结构体实例存在时间会短于类型实例存在时间。换句话说，当结构体的实例有效时，对结构体外的类型的引用一定有效。

长生命周期可以转换为短生命周期（如static项作为参数传递给函数，函数的返回值不再拥有'static生命周期），反之不可以。

生命周期的前缀是撇号(')。示例语法如下：

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

大部分情况下生命周期可以省略。如果省略则遵从以下三条规则：

1. 每一个引用的参数对应的生命周期不同；
2. 如果只有一个输入参数，那么它的生命周期会赋予所有的输出，也就是说，所有输出的值的生命周期都与输入参数相同；
3. 如果输入参数中包括self的引用，那么所有输出值的生命周期都与self——也就是输入的实例——相同。

如果不符合以上规则，那么必须手动注明生命周期。如例子中，如果不注明只有一个生命周期，那么x和y会分别有一个生命周期，一共两个生命周期。由于不能确定函数返回哪个值的引用，编译器无法确定返回值的生命周期，因此会报错。

与生命周期相关的RFC如下：

- [RFC#66：better temporary lifetimes（临时生命周期）](https://rust-lang.github.io/rfcs/0066-better-temporary-lifetimes.html)
- [RFC#141：lifetime elision（生命周期省略）](https://rust-lang.github.io/rfcs/0141-lifetime-elision.html)
- [RFC#556：raw lifetime（裸指针与生命周期）](https://rust-lang.github.io/rfcs/0556-raw-lifetime.html)
- [RFC#1214：projections liftimes and wf（联类型与生命周期）](https://rust-lang.github.io/rfcs/1214-projections-lifetimes-and-wf.html)
- [RFC#1590：macro lifetimes（宏生命周期）](https://rust-lang.github.io/rfcs/1590-macro-lifetimes.html)
- [RFC#2115：argument lifetimes（参数生命周期）](https://rust-lang.github.io/rfcs/2115-argument-lifetimes.html)



### ‘static

'static是一种特殊的生命周期，定义该变量会存活于整个程序运行期间，变量会存储在二进制文件的内存中。所有的str的生命周期都是'static。

使得变量拥有'static生命周期的方式有两种：

1. 声明static或constant项：e.g. static NUM: i32 = 18;
2. 声明str类型：e.g. let static_string = "I'm in read-only memory"。在这个情况下，当static_string这个变量名退出作用域，对其引用将不再有效，但是字符串数据本身仍然存储在二进制文件中。

当'static作为trait bound出现的时候，'static并不要求引用的变量一定拥有static生命周期，只要求这个类型不包含任何非static引用。例如：

```Rust
fn print_it( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}

fn main() {
    // i is owned and contains no references, thus it's 'static:
    let i = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of
    // main(), so it's not 'static:
    print_it(&i);
}
```



#### static项与常量（constant）

static项与常量（constant）不同，尽管两者的生命周期都是'static（除非特殊说明修改生命周期）。static项使用关键字static声明，代表程序中指定的内存位置，所有对于此项的引用都会指向内存中的同一位置。如果将安全相关的信息保存为static项，那么攻击者有可能通过读取内存等方式获取这些内容。如果static项不可改，那么它们可能会被存储在只读内存中；如果static项定义为可改（mut），改动static的代码块必须放在unsafe中，以便定位数据竞争等问题。当程序结束时，static项不会调用drop。

由于以上性质，所有使用static的方法基本都是安全的，但是它们含有以下限制：

- 如果要在多线程中使用，定义为static不可改的类型必须实现Sync trait，static mut则不用；
- 当一个static需要获取另一个static值的时候，只能通过对其引用来获取；
- 常量（constant）不可以引用static。



常量（constant）则不同，它不与某个内存地址关联，当使用时，值会被直接复制进上下文。因此，对同一常量的引用并不保证指向同一内存地址。常量的类型必须拥有'static生命周期。constant可以有析构函数（Destructor），析构函数会在值离开作用域时被调用。

通常情况下更建议使用const，除非以下三种情况：

1. 大量的数据需要存储；
2. 需要static的固定地址或non-inlining性质时；
3. 需要可变性。

如果static或const项值为参数类型为引用的函数或闭包，编译器会首先尝试使用省略规则推断生命周期，如果失败，则认为生命周期为'static。





## 智能指针

有时过多的生命周期标注会使代码可读性下降。从某个角度而言，对于长期存在的变量引用，可以使用智能指针。当然，智能指针不只可以加强代码可读性，可以解决很多其它的问题。

智能指针是一类数据结构，表现类似于指针，但是拥有指针之外的数据和功能。智能指针通常使用结构体实现，并实现Deref和Drop trait。Deref使智能指针表现类似于指针，Drop可以在指针退出作用域时清理堆上或做其它必要清理。

Vector和String都是智能指针。



### Box

Box是Rust的标准库，在std::preclude中。Box将值放在堆上并返回一个指向堆上数据的指针。可以用解引用符号“*”来转移Box内值的所有权，在转移之后，如果值没有实现copy trait，则不可以再使用指针本身或者是Box里的值。使用场景如下：

- 编译时大小未知的类型，如递归类型。由于递归编译器无法确定类型会占有多少空间，但是如果是指针的话，指针的大小是确定的；
- 有大量数据并且希望数据不被拷贝的情况下转移所有权。由于所有权转移，数据在栈上时可能会被拷贝/移动，但是在堆上时只需拷贝/移动指针；
- 只关心值是否实现某一个trait而不是具体类型，在这个情况下使用dyn关键词 e.g. Box\<dyn Trait\>。

Box的常用方法/实现trait如下：

- new / try_new：后者返回一个result，当无法分配内存的时候报错
- pin：返回Pin\<Rc\<T>>，保证Box内值不会移动
- from_raw：根据指定指针获取值，消耗该指针，放入Box并返回Box。这个方法是unsafe，因为这个方法可能在同一个原始指针上重复调用，并在退出作用域时多次调用析构函数（double free）
- into_raw：消耗Box，返回*mut T，非unsafe
- leak：消耗Box，返回对其值引用。如果引用退出作用域，由于Box已经被消耗，drop不会被调用，有可能造成内存泄漏，可以使用from_raw生成一个新的Box。一般在值的生命周期等于程序剩余生命周期时考虑使用。
- borrow / borrow_mut：返回对其值引用，不消耗Box
- clone / clone_from：复制Box中的值，返回新的Box，是deep copy
- 在nightly版本中有一些关于不同初始化模式以及设置Allocator的方法，在此不展开



### Rc

Rc（Reference Counting）是Rust的标准库，但是不在std::preclude中，需要使用use引入作用域。Rc可以启用多所有权，通过引用计数来知晓这个值是否仍在被使用，并且由此决定是否在当前作用域结束的时候，调用drop。由于多所有权，Rc只允许对值的不可变引用。Rc\<T\>将值分配在堆上，以便程序不同部分读取。Rc::clone会增加引用计数，而当引用计数为0时，在当前值退出作用域时清理数据。

有时候这个问题可以用存储引用解决，但是这会引入生命周期标注，降低可读性并增加新的或许不必要的生命周期限制。对于递归结构而言，问题不可以用存储引用解决，因为对于临时空值的引用是无效的。顺带一提，Rust中没有C/C++/Java/Python概念中的null/None。表示空值根据情况不同，或可以使用Option/Result代替。

与RefCell相同，Rc\<T\>只适用于单线程，Rc\<T\>的引用计数加减不是原子操作，在多线程中无法保证它的行为是符合逻辑预期的。原子操作指不会被线程调度机制打断的操作，这个操作一旦开始，会一直运行到结束，中间不会有上下文切换到另一个线程。

Rc的常用方法/实现trait如下：

- new：后者返回一个result，当无法分配内存的时候报错
- pin：返回Pin\<Rc\<T>\>保证值不会移动
- try_unwrap：返回一个Result。如果Rc有正好一个强引用，则返回内部值，否则返回Err以及原来的Rc
- into_raw / as_ptr：返回*const T，前者消耗Rc，后者不消耗。
- from_raw
- downgrade：返回Weak\<T\>
- weak_count / strong_count：返回count次数
- ptr_eq：比较两个Rc是否指向同一个位置
- increment_strong_count / decrement_strong_count：unsafe，修改强引用次数
- get_mut：返回Option，当没有其他Rc或是Weak指针时，返回对值的可变引用，反之则返回None
- make_mut：返回对值的可变引用，当不可行时，如有其他Rc指向同一个位置，复制值保证单一所有权，并返回对新值的可变引用。如果含有Weak指针，Weak指针会指向复制前的位置
- borrow
- clone
- drop：drop会减少强引用计数。drop之后，强引用计数 -= 1。如果强引用次数为0，内部值会被清理。
- 在nightly版本中有一些关于不同初始化模式以及unsafe获取可变引用的方法，在此不展开



#### Weak

Weak指针是Rc的一个小变种，包含对值的引用，但是Weak不像Rc，Weak不拥有该值，同时Weak引用和所有权无关。

Weak可以防止递归结构使用Rc导致的引用死循环。例如，父指向子，子指向父，如果引用是强引用（使用Rc），那么Rc中的强引用计数将永远不会是0；弱引用则可以解决这个问题。通常Weak指针通过调用Rc::downgrade获取。

Weak的常用方法/实现trait如下：

- upgrade：返回Option\<Rc\<T>>。由于Weak特性，Weak不保证值仍然存在，upgrade后如果值不存在，Option值为None。
- strong_count / weak_count：返回Weak指向值的引用计数
- ptr_eq
- clone：复制该弱引用计数
- !Send
- !Sync

其他的一些Weak方法：

- new：返回一个关于指定类型的Weak指针，但是并不分配内存。此时使用upgrade会返回None。
- as_ptr / into_raw：返回*const T。没有强引用保证，指针可能无效。
- from_raw



### Arc

Arc（Atomically Reference Counted）是类似于Rc的指针，但是它可以安全应用于并发环境的类型。Arc的操作是原子性的，意味着它不会被多线程的上下文切换打断，从而保证它的行为符合逻辑预期，但是原子操作的开销高于普通访问内存。Arc和Rc有相同的API，对其引用是只读的。Arc位于std::sync::Arc，同样，在同层下有std::sync::Weak，与Rc的Weak功能相同。

Arc实现了Send和Sync。



### Cell

Cell是一种提供内部可变性的容器，可以在不可变结构中改变Cell中的数据，位于std::cell::Cell。适合实现了Copy的类型，或者体积小的struct。Cell没有运行时开销，运行时安全。Cell在多线程不安全。

Cell的常用方法/实现trait如下：

- new
- set：修改内部值
- swap：交换两个Cell的内部值。不同于std::mem::swap，std::mem::swap需要可变引用
- replace：用新值代替旧值并返回旧值
- into_inner / get / take：返回内部值，into_inner消耗Cell，get复制值并返回，不消耗Cell，take返回值，使Cell内部值为空
- as_ptr：返回裸指针
- get_mut / from_mut：&mut T和&Cell\<T\>的互相转换



### RefCell

当无法确定内部值是否实现了Copy trait时可以使用RefCell。运行时有开销，会执行借用检查，运行时不安全。RefCell在多线程不安全，因为它类似于Rc，会记录作用域内有多少个活动的引用。

RefCell的常用方法/实现trait如下：

- new / take
- borrow / borrow_mut / try_borrow / try_borrow_mut / try_borrow_unguarded：创建不可变和可变引用；try是不panic版本，返回result，unguarded成功返回对内部值的直接不可变引用
- into_inner
- replace / replace_with
- swap：与Cell::swap不同，但是与std::mem::swap类似，如果参数中某一个RefCell已经被可变调用则panic
- as_ptr
- get_mut：通常不使用此方法而使用borrow_mut，因为此方法期望RefCell是可变的。返回对内部值的可变引用，并且不会在运行时检查



#### atomic_refcell

[atomic_refcell](https://docs.rs/atomic_refcell/0.1.7/atomic_refcell/)是一个三方库，提供了RefCell的线程安全版本，其API与RefCell的API基本一致。当用户确定在多线程中不会进行可变借用时，使用RwLock会造成不必要的性能损耗。atomic_refcell复制了RefCell的源码并修改了线程不安全的部分，并不涉及到线程锁，从而提升运行效率。atomic_refcell的核心机制在于，它使用一个高位bit来统计当前值是否已被可变引用，高位bit的比较和翻转的操作都是原子操作，保证其不会被多线程上下文切换打断。当某个线程执行非法借用（例如在已有可变借用时执行不可变借用）时，atomic_refcell根据情况恢复或保留高位bit，并保证当前线程panic不会影响其他线程。



#### qcell

第408篇推文中，本周crate是[qcell](https://docs.rs/qcell/0.4.2/qcell/)。qcell是一个三方库，提供了可以在编译时检查的4种功能形似RefCell的struct。这4个struct没有机制差异，唯一的差异是所有权的表示方式：Qcell使用数字ID，TCell和TLCell使用标记类型，LCell使用Rust生命周期。

qcell的核心机制是，除了类似于RefCell存储值以外，qcell引入了一个存储在栈上的owner，并且只有通过owner才可以借用到qcell中存储的值。由此，当其他引用试图通过owner借用值时，编译器可以像检查出其他引用问题一样查出qcell的引用问题，而不是像RefCell使用计数来统计。

qcell的缺陷在于静态检查的限制有时过于严苛，导致有些可行的代码会无法通过编译。如果不同的cell被被同一个owner所限制，qcell的规则会使这些cell的借用将无法同时发生，然而refcell将会允许这一正确的行为。同时，qcell同时只能允许最多3个对象的借用。

此外，qcell也提供了线程安全版本的refcell。

| Cell      | Owner type  | Cell type   |
| :-------- | :---------- | :---------- |
| `RefCell` | n/a         | Send        |
| `QCell`   | Send + Sync | Send + Sync |
| `TCell`   | Send + Sync | Send + Sync |
| `TLCell`  |             | Send        |
| `LCell`   | Send + Sync | Send + Sync |



### Mutex

RwLock与Mutex都是智能指针，功能上都可用于多线程保护数据。

Mutex（互斥锁）会等待获取锁令牌，同时只有一个线程的Mutex对象可以获取到锁令牌（token），而其他线程会被阻塞，直到锁被释放。在多线程中，经常和Arc一起使用来使多个线程共用一个互斥锁。如果某个已获取但是未释放令牌的线程崩溃，那么其他线程将无法获取令牌，因此造成死锁问题。Rust将这种情况下的Mutex称为poisoned Mutex。如果试图对这类Mutex获取锁，lock方法会阻塞线程，但是try_lock不会阻塞线程，并会返回PoisonError。通过PoisonError的into_inner方法，用户仍然可以获取保存着内部值的MutexGuard。由此，无效的锁并不能完全保证数据的完整性，如果需要通过PoisonError获取锁的内部值，需要注意数据是否已被修改。

Mutex的使用一般只在两个场景：与Arc一起，在线程之间共享数据，提供多线程的变量读写功能；MaybeUninit<Mutex\<T>> 用来做全局变量，Mutex本身并不能保证不变，由MaybeUninit保证其不变性。在单线程中使用Mutex容易造成死锁，并且并不是最优解，一般使用refcell或cell替代。

Mutex的常用方法/实现trait如下：

- new
- lock / try_lock：获取令牌并上锁。前者会阻塞线程，而后者不会。在线程崩溃造成的死锁情况下，如果锁仍然可以由别的方式获取（如PoisonError::into_inner），try_lock将会返回PoisonError，如果由于Mutex已上锁而无法再获取，则返回WouldBlock错误。在Rust中，unlock操作在drop方法中完成，drop方法属于MutexGuard对象，也就是lock返回的LockResult.unwrap()获取的结果。
- is_poisoned：返回当前锁是否被污染，其他获取锁未释放的线程是否崩溃
- into_inner需要：消耗锁并返回LockResult\<T>
- get_mut：对锁的可变引用，并返回LockResult\<&mut T>，由于编译器可以检测可变引用是否合理，实际上不使用上锁机制



### RwLock

RwLock（读写锁）则允许多个读、最多一个写，并且读和写不能同时存在。与Mutex不同，读写锁允许多个读而非仅单一用户获取锁，而互斥锁不区分获取锁的种类。除此以外，RwLock只在内部值\<T>实现Sync trait时，自己也实现Sync trait，以此来保证多个线程可以同时拿到对T的不可变引用，如果T是!Sync，那么RwLock也是!Sync；Mutex对此则不作要求。

类似于Mutex，RwLock也可能被污染（poisoned），不过这个只发生在写模式线程崩溃时。reader崩溃不会影响到锁。

RwLock可以解决多个并发线程试图同时访问同一共享资源的情况，不过锁定策略将会取决于操作系统的实现，如在Windows和Mac中，reader和writer公平排队，但是在Linux中，reader的优先级会高于writer。

RwLock的常用方法/实现trait如下：

- new
- read / try_read：获取读锁，当有writer时线程被阻塞，方法本身不保证reader和writer的优先级
- write / try_write
- is_poisoned：返回当前锁是否被污染，其他获取写锁未释放的线程是否崩溃
- into_inner需要：消耗锁并返回LockResult\<T>
- get_mut：对锁的可变引用，并返回LockResult\<&mut T>，由于编译器可以检测可变引用是否合理，实际上不使用上锁机制





## 引用

Rust the Book，https://kaisery.github.io/trpl-zh-cn/ch10-03-lifetime-syntax.html

Rust by example，https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html

Smart pointers: The secret to write clean Rust code，https://kerkour.com/blog/rust-avoid-lifetimes/

The Rust Reference，http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/reference/introduction.html

Rust中的Mutex，https://zhuanlan.zhihu.com/p/357506863

【Rust每周一知】Rust中的读写锁RwLock，https://blog.csdn.net/u012067469/article/details/105283463

如何理解 rust 中的 Sync、Send？，https://cloud.tencent.com/developer/article/1459819

