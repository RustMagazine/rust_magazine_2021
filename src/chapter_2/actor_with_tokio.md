# 「译」使用 Tokio 实现 Actor 系统

译者：Matrixtang

原文：[https://ryhl.io/blog/actors-with-tokio/](https://ryhl.io/blog/actors-with-tokio/)

---

  本文将不使用任何 Actors 库(例如 Actix ) 而直接使用Tokio实现 Actors 系统。事实上这甚至是更容易的，但是还是有一些细节需要注意：

1. `tokio::spawn` 的调用位置。
2. 使用带有`run`方法的结构体还是裸函数。
3. Actor 的 Handle 函数。
4. 背压( Backpressure ) 和 有界信道。
5. 优雅的关闭。

本文概述的技术适用于任何执行器，但为简单起见，我们仅讨论Tokio。与Tokio教程中的 [spawning](https://tokio.rs/tokio/tutorial/spawning) 和[channel chapters](https://tokio.rs/tokio/tutorial/channels)章节有一些重叠， 当然啦，我建议也阅读这些章节。

​	在讨论如何编写 Actor 之前，我们需要知道 Actor 是什么。Actor 背后的基本思想是产生一个独立的任务，该任务独立于程序的其他部分执行某些工作。 通常，这些参与者通过使用消息传递信道与程序的其余部分进行通信。 由于每个 Actor 独立运行，因此使用它们设计的程序自然是并行的。 Actor 的一个常见用法是为 Actor 分配你要共享的某些资源的专有所有权，然后让其他任务通过与 Actor 通信来间接访问彼此的资源。 例如，如果要实现聊天服务器，则可以为每个连接生成一个任务，并在其他任务之间路由一个聊天消息的主任务。 十分有用，因为主任务可以避免必须处理网络IO，而连接任务可以专门处理网络IO。 



## 实现

​	Actor 分为两部分：任务和handle。 该任务是独立生成的Tokio任务，实际上执行 Actor 的职责，而 handle 是一种允许你与该任务进行通信的结构。 	

​	让我们考虑一个简单的 Actor 。 Actor 在内部存储一个计数器，该计数器用于获取某种唯一ID。 Actor 的基本结构如下所示：

```rust
use tokio::sync::{oneshot, mpsc};

struct MyActor {
    receiver: mpsc::Receiver<ActorMessage>,
    next_id: u32,
}
enum ActorMessage {
    GetUniqueId {
        respond_to: oneshot::Sender<u32>,
    },
}

impl MyActor {
    fn new(receiver: mpsc::Receiver<ActorMessage>) -> Self {
        MyActor {
            receiver,
            next_id: 0,
        }
    }
    fn handle_message(&mut self, msg: ActorMessage) {
        match msg {
            ActorMessage::GetUniqueId { respond_to } => {
                self.next_id += 1;

                // The `let _ =` ignores any errors when sending.
                // `let _ =` 忽略了发送的任何 error
                // This can happen if the `select!` macro is used
                // to cancel waiting for the response.
                // 当 `select!` 宏被用到时将会停止接受响应
                let _ = respond_to.send(self.next_id);
            },
        }
    }
}

async fn run_my_actor(mut actor: MyActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }
}
```

现在我们有了 Actor 本身，我们还需要一个与 actor 配套的handle 。 handle  是其他代码段可以用来与 actor 对话的对象，也是让 Actor 存活的原因。 

以下是 handle 的实现： 

```rust
#[derive(Clone)]
pub struct MyActorHandle {
    sender: mpsc::Sender<ActorMessage>,
}

impl MyActorHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = MyActor::new(receiver);
        tokio::spawn(run_my_actor(actor));
        // 译者提醒： 注意 tokio::spawn 的位置
        Self { sender }
    }

    pub async fn get_unique_id(&self) -> u32 {
        let (send, recv) = oneshot::channel();
        let msg = ActorMessage::GetUniqueId {
            respond_to: send,
        };

        // Ignore send errors. If this send fails, so does the
        // recv.await below. There's no reason to check for the
        // same failure twice.
        // 忽略发送 error 。如果它发送失败， 将会执行下方的 recv.await
        // 检测同样的错误两次是没有道理的。
        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor task has been killed")
    }
}
```

[full example](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=1e60fb476843fb130db9034e8ead210c)

让我们仔细看一下本示例中的不同部分。

**`ActorMessage.`** `ActorMessage` 枚举定义了我们可以发送给 Actor 的消息类型。 通过使用这个枚举，我们可以拥有许多不同的消息类型，并且每种消息类型都可以具有自己的参数集。我们通过[`oneshot`](https://docs.rs/tokio/1/tokio/sync/oneshot/index.html)信道向 sender 返回值 , 而这种信道只允许发送一条消息。

在上面的示例中，我们在 actor 结构的 `handle_message` 方法中的枚举上进行了匹配，但这不是构造此方法的唯一办法。 也可以在 `run_my_actor` 函数的枚举中进行匹配。 然后，此匹配项中的每个分支都可以在 actor 对象上调用各种方法，例如  `get_unique_id` 。 

**发送消息时出错** 在处理信道时，并非所有错误都是致命( fatal )的。 因此，该示例有时使用 `let _ =` 来忽略错误。 通常，如果 receiver 被丢弃，那在信道上的 `send` 操作将失败。 在我们的示例中，此操作的第一个实例是 actor 中我们响应已发送的消息的那行 。

```` rust 
let _ = respond_to.send(self.next_id);)
````

 这将发生在接收方不再需要操作的结果的情形下，例如 发送消息的任务可能已被杀死。


**关闭Actor**  我们可以通过查看接收消息是否失败来决定何时关闭 Actor 。 在我们的示例中，这发生在以下 while 循环中： 

```rust
while let Some(msg) = actor.receiver.recv().await {
    actor.handle_message(msg);
}
```

当所有发送到`receiver` 的 `sender` 都被丢弃时，我们就知道将不会再收到其他信息了，因此可以关闭 Actor 。 当这种情况发生时，调用`.recv（）`将返回 `None` ，并且由于它与模式`Some（msg）`不匹配，while 循环将退出并且函数会返回。 



## 结构体的 run 方法

​	我上面给出的示例使用的顶层函数并未在任何结构上定义，因为我们将其作为 Tokio 任务产生 ，但是许多人发现直接在 MyActor 结构体中定义 `run`方法并且启动更加自然。 也不是不行，但是我举这个使用顶层函数的示例的原因是，使用这种方法就可以避免很多由生命周期而产生的问题了。 为了说清楚这种问题，我准备了一个例子，说明不熟悉该模式的人经常会想到什么。 

```rust
impl MyActor {
    fn run(&mut self) {
        tokio::spawn(async move {
            while let Some(msg) = self.receiver.recv().await {
                self.handle_message(msg);
            }
        });
    }

    pub async fn get_unique_id(&self) -> u32 {
        let (send, recv) = oneshot::channel();
        let msg = ActorMessage::GetUniqueId {
            respond_to: send,
        };

        // Ignore send errors. If this send fails, so does the
        // recv.await below. There's no reason to check for the
        // same failure twice.
        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor task has been killed")
    }
}

... and no separate MyActorHandle
```

这个示例存在两个问题：

1. `tokio::spawn`在  `run` 方法中被调用。
2. Actor 和 handle 其实是一个结构体。

导致问题的第一个原因是，因为`tokio :: spawn`函数要求参数为 `'static'`。那就意味着新任务必须拥有完整的所有权，这就导致了该方法借用了`self`，所以它无法将 `self` 的所有权交给新任务。

第二个问题是，因为Rust强制实施了单一所有权原则。 如果将 actor 和 handle 都合并为同一个结构体，则（至少从编译器的角度来看）将使每个handle 都可以访问 actor 的任务所拥有的全部字段。 例如， `next_id` 应仅由 actor 任务拥有，而且不应该让任何 handle 直接访问。 

也就是说，有一个通过解决以上两个问题，变得可行的版本。代码如下： 

```rust
impl MyActor {
    async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg);
        }
    }
}

impl MyActorHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = MyActor::new(receiver);
        tokio::spawn(async move { actor.run().await });

        Self { sender }
    }
}
```

该函数与顶层函数相同。 请**注意**，严格来讲，可以编写`tokio :: spawn`在`run`内的那种 ， 但是我并不推荐。



## actor 的 其他变体

​	我在本文中的示例使用了参与者使用消息的请求-响应模型(request-response)，但是这不是必须的。 在本节中，我将给你一些使用其他方式的例子，给你一些启发。



### 不对消息回应

​	在之前的示例中我们介绍了一种使用`oneshot`信道发送对消息响应的方式，但是并不总是需要响应。在这些情况下，仅在消息枚举中不包含 `oneshot` 信道是没有问题的。当信道中有空间时，这甚至可以让你在处理完消息之前就返回。 但是仍应确保使用有界信道，以保证在该信道中等待的消息数不会无限增长。在某些情况下，这意味着仍然需要由一个异步函数来处理`发送`操作，用于处理等待信道需要更多空间的情况。 但是，还有一种替代方法可以使`send`操作成为异步的。即使用 `try_send` 方法，并通过简单地杀死 Actor 来处理发送失败的情况。这在 Aoctor 管理 `TcpStream` 时，用于转发发送到连接中的任何消息的情况下是很有用的。这种情况下，如果无法继续向 `TcpStream` 写入 ，则可直接关闭连接。 



### 多个handle共享一个 Actor

​	如果需要从不同的地方向 actor 发送消息，则可以使用多个 handle 来强制某些消息只能从某些地方发送。 当使用这种方法时，你仍然可以在内部重复使用相同的 `mpsc` 通道，并使用其中包含所有可能的消息类型的枚举。 如果你**不得不**想要为此使用单独的信道，则  actor 可以使用 [`tokio::select!`](https://docs.rs/tokio/1/tokio/macro.select.html) 来一次性冲多个信道中接受信息。

```rust
loop {
    tokio::select! {
        Some(msg) = chan1.recv() => {
            // handle msg
        },
        Some(msg) = chan2.recv() => {
            // handle msg
        },
        else => break,
    }
}
```

需要注意的是在信道关闭时的处理方式，因为在这种情况下，它们的 `recv` 方法会立即返回 `None` 。 幸运的是，`tokio :: select！` 宏允许您通过提供 `Some（msg）` 来处理这种情况。 如果仅关闭一个信道，则该分支将被禁用，另外一个信道依旧是可用的。 当两者都关闭时，else分支运行并使用`break`退出循环。 



### Actors 间发送信息

​	让 Actor 将消息发送给其他 Actor 也是可行的。 为此，只需为一个 Actor 提供其他 Actor 的 handle 即可。 当Actor 形成了循环时，需要上点心，因为为了保持彼此的 handle 存活，防止 Actor 被关闭最后一个 `sender` 不会被丢弃。 为了处理这种情况，您可以让一个 actor 具有两个带有独立的`mpsc`通道的 handle ，`tokio :: select！`会被用在下面这个示例里 ： 

````rust
loop {
    tokio::select! {
        opt_msg = chan1.recv() => {
            let msg = match opt_msg {
                Some(msg) => msg,
                None => break,
            };
            // handle msg
        },
        Some(msg) = chan2.recv() => {
            // handle msg
        },
    }
}

````

如果 `chan1` 关闭，即使`chan2`仍然打开，上述循环也将退出。 如果` chan2`  是 Actor 循环的一部分，则这会中断该循环并让 Actor 关闭。 

只需要简单的在循环里调用  [`abort`](https://docs.rs/tokio/1/tokio/task/struct.JoinHandle.html#method.abort) 就可以了。



### 多个 Actors 共享一个 handle

​	就像每个 Actor 可以共享多个 handle 一样，每个 handle 也可以共享多个  Actors 。 最常见的示例是在处理诸如 `TcpStream`之类的连接时，通常会产生两个任务：一个用于读，一个用于写。 使用此模式时，需要将读和写入任务变得尽可能简单——它们的唯一工作就是执行IO。 读任务会将接收到的所有消息发送给其他任务，通常是另一个 Actor ，而写任务会将接收到的所有消息转发给连接。 这种模式非常有用，因为它把与执行IO相关的复杂性隔离开来，这意味着其他程序部分可以假装将某些内容立即写入连接，尽管实际的写入其实是在 Actor 处理消息后进行的。



## 当心循环



​	我已经在`Actors 间发送信息` 标题下讨论了一些关于循环的问题，在此我讨论了如何关闭循环的Actors。但是，如何关闭并不是循环可能导致的唯一问题，因为这种循环还会产生死锁，循环中的每个 Actor 都在等待下一个 Actor 接收消息，但是下一个 Actor 直到它的下一个Actor接收到消息才会接收到该消息，依此类推。 为避免这种死锁，必须确保循环的信道容量都不受限。这样做的原因是有界信道上的 `send` 方法不会立即返回，而具有立即返回`send` 方法的信道是不记入这种循环，因为这种`send`方法是不会产生死锁的。 当心，这意味着`oneshot` 信道也不会产生死锁，因为它们也有 立即返回的 `send`  方法。还要当心，如果使用的是 `try_send` 而不是`send`来发送消息，那么这也不是死锁循环的一部分。 

感谢 [matklad](https://matklad.github.io/)指出循环和死锁的问题。 

---

译者简介：

Matrixtang，Rust/cpp 程序员，对编译相关领域感兴趣，不会 pwn 的安全爱好者。