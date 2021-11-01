# Rust Async trait 更新与多线程

作者： 王江桐

> 本篇将会简要介绍什么是《This Week in Rust》，第412篇推文中有关于Rust语言组[十月会议总结](https://blog.rust-lang.org/inside-rust/2021/10/08/Lang-team-Oct-update.html)，其中有关于Async fn的更新以及[多线程的相关内容](https://nickymeuleman.netlify.app/blog/multithreading-rust)。

---

## 语言组十月月会

Rust语言组会在每个月的第一个周三开一次例会，并总结本月更新方向。具体的会议描述可见链接：https://blog.rust-lang.org/inside-rust/2021/10/08/Lang-team-Oct-update.html

十月会议内容大致如下：

- 实现[traits中的异步函数](https://rust-lang.github.io/async-fundamentals-initiative/updates/2021-oct.html)，异步`drop`，异步闭包

- 在模块和impl层面上实现如下[类型定义](https://rust-lang.github.io/impl-trait-initiative/updates/2021-oct.html)：`type Foo = impl Trait`

  - ```Rust
    type Foo = impl Trait;
    
    impl SomeTrait for SomeType {
        type AssocType = impl Debug;
    
        ...
    }
    ```

- 允许dyn trait类型[向上覆盖](https://rust-lang.github.io/dyn-upcasting-coercion-initiative/updates/2021-oct.html)，例如如果`trait Foo: Bar`，可以将类型`dyn Foo `映射为`dyn Bar`

- 允许[关联类型包含泛型参数](https://rust-lang.github.io/generic-associated-types-initiative/updates/2021-oct.html)，例如`type Foo<'me>`

- [let else语法更新](https://github.com/rust-lang/rust/issues/87335#issuecomment-933672440)：在nightly版本中现在允许如下写法：`let Ok(x) = something else panic!()`，预备在stable版本中实现

- 对于实现了`Deref`或`DerefMut`的类型，[允许使用`match`进行模式匹配](https://github.com/rust-lang/lang-team/issues/88#issuecomment-935056996)，并且在进行匹配时使用`Deref`

- `!`（Never）类型更新；目前部分此类类型由`()`表示，在保证现有代码功能性的情况下，之后可能会使用`!`表示



### Async fn更新

在十月例会中，语言组确定了目前大体上对于异步函数功能的拓展，目标如下：

- 在traits中实现Async fn，不论是在static或是在dyn上下文中
- 实现Async drop
- 实现Async closure

在traits中实现异步函数则意味着，循环、读、写等功能在未来都可以异步实现。



#### dyn trait

在讨论为什么异步函数很难在trait之中实现之前，我们需要先了解什么是dyn safe。

dyn trait是一种特殊的类型，通常情况下来说，dyn trait类型将被视作实现了该trait。例如：

```Rust
trait DoTheThing {
	fn do_the_thing(&self);
}

impl DoTheThing for String {
    fn do_the_thing(&self) {
        println!(“{}”, self);
    }
}

fn some_generic_fn<T: ?Sized + DoTheThing>(t: &T) {
	t.do_the_thing();
}

fn some_nongeneric_fn(x: &dyn DoTheThing) {
    some_generic_fn(x)
}
```

对于`some_generic_fn`，我们可以传入一个String，因为它实现了`DoTheThing`这个trait。对于`some_nongeneric_fn`，由于x的类型是`dyn DoTheThing`，x将被视作实现了`DoTheThing`，因此可以将x作为参数传给some_generic_fn。

这样做的便利在于，当函数的参数类型是dyn trait时，用户可以自由地传入任何实现了该trait的类型，但是这个模式同样也带来了问题：编译器不知道这些类型的大小是什么。dyn trait作为一个类型无法和实际的类型，例如String，相互比较，因为dyn trait并没有大小，它的大小会随着具体实现该trait的类型大小而改变。因此，对于dyn trait，参数往往以某种指针的形式传入，例如`Box<dyn DoTheTHing>`或是如例子中使用引用，`&dyn DoTheThing`。泛型函数并不直接接受dyn类型，Rust因此提供了另一种可以在泛型函数中使用的写法，也就是`T: ?Sized + DoTheThing`。

在编译器编译并决定什么时候使用什么函数时，通常有两种模式：静态派遣（static dispatch）与动态派遣（virtual / dyn dispatch）。在静态派遣模式下，编译器知道具体调用哪个函数；而在动态派遣模式下，编译器不知道具体是哪个函数，但是知道函数的地址，以及需要调用在这个地址的函数。当使用dyn trait时，编译器的模式是动态派遣，并且使用vtable来保留函数指针，每个指针指向trait中的方法。vtable形似如下struct：

```Rust
struct DoTheThingVtable {
    do_the_thing: fn(*mut ())
}
```

第一个参数并不是常见的`Self`——对于dyn trait，我们并不知道实际上的类型是什么，因此只能保留某种指针。实际创建vtable可以看做创建例中struct的一个实例，例如对于实现了`DoTheThing`这个trait的`String`，其对应的vtable大概如图：

```Rust
static Vtable_DoTheThing_String: &DoTheThingVtable = &DoTheThingVtable {
    do_the_thing: <String as DoTheThing>::do_the_thing as fn(*mut ())
    //            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //            Fully qualified reference to `do_the_thing` for strings
};
```

在Rust中，形似于`&dyn DoTheTHing`的dyn trait指针是一个宽指针，在运行时包含两个指针，一个指向数据本身，另一个指向trait。如例中该宽指针值为：`(*mut (), &’static DoTheThingVtable)`。当`&String`被映射为`&dyn DoTheThing`，实际上它会先被映射为`*mut ()`，然后再与对应的vtable相关联。因此，当使用泛型时，在编译的时候泛型会被解析为固定的类型，就像其他非泛型的函数参数一样，类型具有具体的限定。



##### dyn safe

一个trait只有在它不包括泛型时可以安全作为dyn trait使用。参考如下例子：

```Rust
PrintPrefixed:

trait PrintPrefixed {
    fn prefix(&self) -> String;
    fn apply<T: Display>(&self, t: T);
}

impl PrintPrefixed for String {
    fn prefix(&self) -> String {
        self.clone()
    }
    fn apply<T: Display>(&self, t: T) {
        println!(“{}: {}”, self, t);
    }
}
```

对于`prefix`，使用dyn trait时，vtable可以建立`String`与`PrintPrefixed`的关联，但是使用`apply`时，关联将无法被建立，因为实现了`Display`这一trait的具体类型是什么仍是未知的。

相反，在静态派遣的模式下，除非被调用，我们不需要知道`T`是什么类型，在调用时也只需生成副本保证运行。

虽然trait可以同时包含dyn safe和非dyn safe的方法，在目前版本的Rust中，只要有一个方法不是dyn safe，那么整个trait就不是dyn safe，因为trait无法拆分方法来实现。虽然`dyn PrintPrefixed`在静态检查时可以通过，但是除非`PrintPrefixed`中所有方法都是dyn safe的，不然`dyn PrintPrefixed`并不会被认作实现了`PrintPrefixed` trait。不过，对于`&dyn PrintPrefixed`类型，`prefix`函数依然可以使用。

对于关联类型，dyn trait必须声明所有的关联类型具体是什么。例如：

```Rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

impl<T> Iterator for dyn Iterator<Item = T> {
    type Item = T;
    
    fn next(&mut self) -> Option<T> {
        let next_fn = /* get next function from vtable */;
        return next_fn(self);
    }
}

```

这个trait不涉及其他的泛型，因此它是dyn safe的。但是在实际使用时，必须声明Item的类型：`dyn Iterator<Item = u32>`。



#### 问题来源与解决方案

目前的问题在于，如果在traits中编写async fn，例如`async fn foo(&self)`，trait以及impl块中语法糖会被解糖为：

```Rust
trait Trait {
    async fn foo(&self);
}

impl Trait for TypeA {
    async fn foo(&self);
}

impl Trait for TypeB { ... }

// 解糖后
trait Trait {
    // 匿名关联类型
    type Foo<'s>: Future<Output = ()> + 's;

    fn foo(&self) -> Self::Foo<'_>;
}

impl Trait for TypeA {
    // 匿名关联类型
    type Foo<'s> = impl Future<Output = ()> + 's;

    fn foo(&self) -> Self::Foo<'_> {
        async move { ... } // has some unique future type F_A
    }
}

impl Trait for TypeB { ... }
```

因此，如果trait使用了异步，那么impl块也需要使用异步函数，反之亦然。

问题在于，使用异步函数的trait在`dyn`情况下并不安全，因为我们并不知道Future具体是什么类型；使用`dyn`时必须列出所有的关联类型的值，也就是说，如果要使用`dyn`，必须这样写：

```Rust
// XXX是impl块定义的future类型
dyn for<'s> Trait<Foo<'s> = XXX>
```

一个更具体的例子：

```Rust
trait AsyncIter {
    type Item;
    async fn next(&mut self) -> Option<Self::Item>;
}

// 需要写成如下形式
for<'me> dyn AsyncIter<
    Item = u32, 
    Next<'me> = SleepyRangeNext<'me>,
>
```

不论繁琐以及是否可能，这使`dyn trait`限制于某一个特定的impl块，而这与`dyn trait`的设计意图冲突：在使用`dyn`时，用户并不知道实际上的类型是什么，只知道类型实现了目标trait。出于这个原因，一个使用`#[async_trait]`的改进方式如下：

```Rust
#[async_trait]
// to state whether Box<...> is send or not if desired: #[async_future(?Send)]
trait Trait {
    async fn foo(&self);
}

// desugars to

trait Trait {
    fn foo(&self) -> Box<dyn Future<Output = ()> + Send + '_>;
}
```

这样子做可以通过编译，缺点在于，哪怕不使用`dyn trait`，也会在堆上为`Box`分配空间，以及用户必须提早声明`Box<...>`是否实现了`Send`trait。这会带来不必要的麻烦，并且与Rust的设计意图冲突。

根据`dyn trait`、Rust本身的设计意图、以及其他的一些限制来看，要解决async fn in traits的问题，必须满足以下条件：

- 在trait中使用async fn应该不强制要求堆分配
- 当使用`dyn trait`，对于所有impl块，future的类型应该相同，使得`dyn trait`泛用，而非局限于某个特定类型。也就是说，future应与实际的impl块独立，并且它应当有固定的大小，保证编译器可以编译

目前的进展是，语言组已实现初步的static async fn in triats，但是只能与impl trait或泛型一起使用，具体可见这篇[MVP](https://rust-lang.github.io/async-fundamentals-initiative/roadmap/mvp.html)（Minimum viable product最小可行产品）。目前解决方案的缺陷在于：

- 不支持dyn
- 无法命名Future types并且获取它们的引用
- 无法限定这些Future，例如限定它们必须实现`Send`



#### Async Drop

异步drop的实现将允许在drop时进行await调用。异步drop的实现问题目前有：

- 对于泛型函数foo：

  - ```Rust
    async fn foo<T>(t: T) {
        runtime::sleep(22).await;
    }
    ```

  - 函数`foo(t)`的类型将会是一个Future，其中包括了所有函数内调用的Future的类型。然而对于泛型T，在编译时我们无法得知T是否也是异步的，由此无法推断函数的类型，也无法推断该函数是否需要实现`async drop`

- 无法确定`async drop`一定会被调用，当调用drop时，编译器或许会调用同步的drop而非异步的

- 如果实现了`async drop`，并且代码有可能因为报错而逻辑中断时，例如：

  - ```Rust
    async fn foo(input: &QueryInput) -> anyhow::Result<()> {
        let db = DatabaseHandle::connect().await;
        let query = assemble_query(&input)?;
        let results = db.perform_query(query).await;
        while let Some(result) = results.next().await? {
            ...
        }
    }
    ```

    - 假设`db`实现了`async drop`，对于使用`?`的函数，它们有概率报错并且使函数在逻辑上应当结束，并调用drop清理函数中每一个变量。因此，如果`?`出错，`AsyncDrop::async_drop(db).await`应被调用，而Rust认为不应由用户手动处理这些事情。同时，这也削弱了Rust对`.await`的要求：每一个阻塞点应该都明确写出，而非隐式调用

此次会议以及”[async fn fundamentals initiative](https://rust-lang.github.io/async-fundamentals-initiative/index.html)“中暂时没有提到关于Async drop问题的解决方案。



## Rust多线程

### 多线程（Multithreading），多进程（Multiprocessing），与多协程（Coroutine）

#### 多线程（Multithreading）

多线程是一种并发执行机制。一个线程可以同时运行一个任务，多个线程则可以运行多个任务。用最简单的话来说，多线程就是计算机同时运行多个线程，从而在同一段时间内并列同时执行多个任务。实际上，虽然CPU可以创建并协调多个线程，目前一个CPU在同一时间只能运行一个线程。在这种情况下，对于单一的CPU而言，线程在实际运行中也不是并行（parallel）的，而是并发（concurrent）的，CPU的调度器会决定何时切换线程，线程的运行顺序无法人为控制。

一个进程可以创建多个线程，这些线程的运行彼此独立，但是它们共享同一个进程资源。

多线程的优势在于：

- 加速程序运行，提高CPU利用率，在有多个CPU的情况下，线程与线程之间甚至可以真正地并行运行；
- 与线程之间彼此独立，某个线程的错误或阻塞不会影响其它线程。

多线程的缺陷在于：

- 切换线程时有上下文切换的开销，在一些场景下，这些切换或许会造成不必要的性能损耗；
- 线程与线程之间进程资源共享，会导致资源竞争问题，例如阅读与修改同一资源，线程饿死，std输出竞争。

多线程常用于：

- 处理程序在一部分情况会阻塞，但是在另一部分需要持续运行的场合，例如网络服务器同时接收并处理多个请求；
- 加速程序运行效率，拆分任务并提高CPU利用率，例如图像分析算法使用多线程计算图像的不同部分，并在最后汇总结果；
- IO密集型计算，同时运行的任务数目要求不多。



#### 多进程（Multiprocessing）

简单来说，进程可以这样定义：

> 进程可以简单的理解为一个可以独立运行的程序单位，它是线程的集合，进程就是有一个或多个线程构成的。而线程是进程中的实际运行单位，是操作系统进行运算调度的最小单位。可理解为线程是进程中的一个最小运行单元。

与多线程相对的概念之一是多进程。在日常生活中，运行多个软件的情况也是计算机正在运行多进程。多进程和多线程有一定的相似度，例如进程和进程之间彼此独立，并且可以提高多喝CPU利用率，但是进程和进程之间并不共享内存，同时进程的上下文切换开销一般大于线程的上下文切换。上下文切换更具体的介绍可以参考博文《[结合中断上下文切换和进程上下文切换分析Linux内核的一般执行过程](https://www.cnblogs.com/fxding/p/13097941.html)》。此外，由于内存不共享，比起共享进程资源的多线程，在一些场景下，多进程需要额外解决通信问题。

创建多进程与多线程的时间开销受操作系统影响。Windows环境下，进程创建开销很大，而在Linux环境下则很小。《[多进程和多线程的概念](https://www.cnblogs.com/linuxAndMcu/p/11064916.html)》博文中记录了作者的一个实验：

> 可以做个实验：创建一个进程，在进程中往内存写若干数据，然后读出该数据，然后退出。此过程重复 1000 次，相当于创建/销毁进程 1000 次。在我机器上的测试结果是：
> UbuntuLinux：耗时 0.8 秒 Windows7：耗时 79.8 秒 两者开销大约相差一百倍。
> 这意味着，在 Windows 中，进程创建的开销不容忽视。换句话说就是，Windows 编程中不建议你创建进程，如果你的程序架构需要大量创建进程，那么最好是切换到 Linux 系统。

在Linux中，可以使用`fork()`命令来根据当前父进程复制一个子进程。

多进程常用于在程序中的重复操作多、且重复操作多为计算操作，即CPU密集型的场景。这些重复计算任务如果使用多线程，由于需要频繁切换上下文，效率不如多进程。



#### 多协程（Coroutine）

协程的概念可见《[This Week in Rust #406：Rust异步](http://openx.huawei.com/Ylong_Rust/dynamicDetail/3283)》。协程优点在于内存开销最少，适用于IO密集计算以及多任务运行。



### Rust实现多线程

Rust中，线程可以用`std::thread::spawn`创建，线程创建时要求一个闭包，作为线程运行的函数使用。线程创建后就与当前线程独立，彼此运行时相互不干扰，当目前线程不是主线程时，创建的线程运行时长可以长于目前线程；对于主线程而言，主线程终止，则程序终止，程序所创建的所有线程都会停止。因此，不论线程在哪里创建，线程接收的闭包内的变量应与主线程生命周期一致。最便捷的实现方式是使用`move`关键字，让线程拥有变量所有权，并管理变量何时drop。

如果主线程不等待子线程，当主线程运行结束时，子线程可能仍未运行完毕。在这种情况下，子线程会随着主线程的结束被提早关闭，无法完成任务。创建线程时，`std::thread::spawn`方法会返回`JoinHandle`，调用它的`join`方法可以阻塞当前线程，并等待目标线程返回。同时，`JoinHandle`可以携带子线程的返回值，使用`join`也可以获取它的返回值。



#### std::thread模块概述

[std::thread模块](https://doc.rust-lang.org/std/thread/index.html)提供了std库中与多线程相关的功能。

主要包含函数如下：

- spawn：生成新线程并运行，返回对应的JoinHandle
- yield_now：当前线程强行放弃运行，让其他线程运行。在调度上不是最优解，在放弃运行时可能没有其他线程准备好运行，可能造成资源浪费
- sleep：使当前线程暂停运行一段时间
- park：阻塞当前线程，直到其他线程调用当前线程的unpark函数
- park_timeout：阻塞当前线程，直到unpark被调用或者超时
- current：获取当前线程对应的句柄



主要包含struct如下：

- Thread：处理线程的句柄。通常推荐使用Builder或者spawn函数生成线程，而非手动创建Thread实例。
  - unpark：原子操作，使对应线程从park阻塞状态恢复
  - id：返回线程ID
  - name：返回线程名字，返回值为Option；默认没有名字，返回None
- ThreadId
- JoinHandle：join线程句柄，当JoinHandle退出作用域时，JoinHandle与对应的线程分离，将导致对应的线程无法被join
  - thread：返回线程句柄引用
  - join：等待对应线程运行结束，阻塞当前线程，返回Result；如果对应线程panic，Result值为Err
- LocalKey：线程私有存储，可以用来存放线程共用变量，并使每一个线程启动时该变量都是初始化时的值。使用`thread_local!`宏建立
  - with：返回key中值的引用
  - try_with：同with，但是返回Result
- Builder：线程仓库，可以修改线程运行设置，目前支持设置线程名字以及线程可用栈大小
  - new
  - name：设置线程名字，返回Builder
  - stack_size：设置线程可用栈大小，返回Builder
  - spawn：以当前设置生成线程并运行，返回Result



#### 共享资源

当线程运行还未结束，但是线程之间需要共享资源时，可以使用`std::sync::mpsc::channel`。Rust标准库中的mpsc意思是多个生产者，单一消费者（Multiple producer, single consumer），因此这类channel可以有多个发送者，但是只能有一个接受者。发送的信息必须获得所有权而不能是引用，以便于在线程之间转移所有权。

`std::sync::mpsc::channel<T>`返回`(Sender<T>, Receiver<T>)`。在很多习惯中，会把返回值命名为`(tx, rx)`，取transmit/TX mode、receive/RX mode的简称。当所有的发送者都退出作用域，或唯一的接受者退出作用域，channel关闭，不再起效。channel关闭以后，receiver的iterator会返回None，从而结束对接收值的遍历。

一个简单的例子如下：

```Rust
use std::mem;
use std::thread;
use std::sync::mpsc;

fn main() {
    let (sender, receiver) = mpsc::channel();
    for i in 0..10 {
        let sender = sender.clone();
        thread::spawn(move|| {
            sender.send(i).unwrap();
        });
    }

    // drop the original sender
    mem::drop(sender);

    // this will wait until all senders are dropped
    // if the original sender isn't dropped manually, it will never be dropped, so this waits forever
    for received in rx {
        println!("Got: {}", received);
    }
}
```

由于Sender的迭代器返回None的条件是所有发送者都被drop，如果在同一个线程中，函数创建了最初的Sender，同时又调用Receiver接收信息，那么需要在接收之前使其退出作用域，例如手动drop第一个Sender，或是使用大括号，不然遍历将永远不会结束。例如在例子中，为了使程序正确结束，`main()`需要手动drop sender。



##### std::sync::mpsc模块概述

[std::sync::mpsc模块](https://doc.rust-lang.org/std/sync/mpsc/index.html)提供了std库中与多线程通信相关的功能。

主要包含函数如下：

- channel：生成异步通信通道，返回对应的Sender和Receiver。Receiver收到信息的顺序与Sender发送的顺序相同，当没有Sender发送信息时，Receiver会被阻塞

- sync_channel：生成同步通信通道，返回对应的SyncSender和Receiver。Receiver收到信息的顺序与SyncSender发送的顺序相同，当没有Sender发送信息时，Receiver会被阻塞。与channel的不同点在于，sync_channel内部缓冲区有大小上限，当缓冲区已满，SyncSender.send会被阻塞，直到缓冲区重开。缓冲区大小可以为0，在这种情况下，除非有Receiver与SyncSender一对一配对，不然SyncSender将被阻塞

  

主要包含struct如下：

- Sender\<T>

  - clone：复制sender，使得多个线程可以使用sender向同一个receiver发送信息
  - send：发送信息，返回值为Result，当Receiver退出作用域时返回SendError

- SyncSender\<T>

  - send：同Sender.send
  - try_send：同send，但是不阻塞

- Receiver\<T>

  - recv：阻塞当前线程，依次接收缓冲区内所有信息，当所有的Sender退出作用域时返回RecvError

  - try_recv：同recv，但是不阻塞

  - recv_timeout：同recv，超时后不阻塞

    - recv_timeout有错误仍未修复，例如以下例子中，recv_timeout会意外panic。具体可见[issue #39364](https://github.com/rust-lang/rust/issues/39364)。

      ```Rust
      use std::sync::mpsc::channel;
      use std::thread;
      use std::time::Duration;
      
      let (tx, rx) = channel::<String>();
      
      thread::spawn(move || {
          let d = Duration::from_millis(10);
          loop {
              println!("recv");
              let _r = rx.recv_timeout(d);
          }
      });
      
      thread::sleep(Duration::from_millis(100));
      let _c1 = tx.clone();
      
      thread::sleep(Duration::from_secs(1));
      ```

  - iter：返回可以阻塞线程的迭代器，当通道关闭，迭代器返回None

  - try_iter：同iter，但是除了通道关闭以外，如果当前没有信息被发送，try_iter同样返回None



### 资源竞争与锁

由于多线程共享进程内存，当线程需要获取同一个数据并对其进行非只读操作，资源竞争就有可能出现。为了防止资源竞争，可以使用`Mutex`保护数据。由于不同的线程需要获取同一个锁，通常使用`Arc`包裹`Mutex`，在保证操作不会被线程切换打断的同时，使锁可以存在多个引用。除了`Mutex`，`RWLock`读写锁也可以防止资源竞争。读写锁允许多个读用户同时存在，而`Mutex`不区分获取锁的种类，同时只能有一个用户获取锁。`Arc`，`Mutex`，`RWLock`更详细的介绍可见《[This Week in Rust #407 & 408：Rust 生命周期和智能指针](http://openx.huawei.com/Ylong_Rust/dynamicDetail/3312)》。

`Mutex`的一个简易例子如下：

```Rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    // threads will all write frequency to this HashMap
    let result = Arc::new(Mutex::new(HashMap::new()));
    let chunks = input.chunks((input.len() / worker_count).max(1));
    let mut handles: Vec<_> = Vec::new();

    for chunk in chunks {
        let string = chunk.join("");
        let result = Arc::clone(&result);
        let handle = thread::spawn(move || {
            let mut map: HashMap<char, usize> = HashMap::new();
            // create a HashMap for this chunk
            for c in string.chars().filter(|c| c.is_alphabetic()) {
                *map.entry(c.to_ascii_lowercase()).or_default() += 1;
            }
            // add the HashMap of this chunk to the HashMap that is wrapped by the Mutex
            let mut result = result.lock().unwrap();
            for (key, value) in map {
                *result.entry(key).or_default() += value;
            }
        });
        handles.push(handle);
    }

    // wait for each thread to finish
    for handle in handles {
        handle.join().unwrap()
    }

    // get the HashMap from the Arc<Mutex<HashMap>>
    Arc::try_unwrap(result).unwrap().into_inner().unwrap()
}
```


## 引用

- [Lang team October update](https://blog.rust-lang.org/inside-rust/2021/10/08/Lang-team-Oct-update.html)
- [Dyn async traits, part 1](https://smallcultfollowing.com/babysteps//blog/2021/09/30/dyn-async-traits-part-1/)
- [Dyn async traits, part 2](https://smallcultfollowing.com/babysteps//blog/2021/10/01/dyn-async-traits-part-2/)
- [Multithreading in Rust](https://nickymeuleman.netlify.app/blog/multithreading-rust)
- [多进程和多线程的概念](https://www.cnblogs.com/linuxAndMcu/p/11064916.html)
- [结合中断上下文切换和进程上下文切换分析Linux内核的一般执行过程](https://www.cnblogs.com/fxding/p/13097941.html)



