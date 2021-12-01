# 社区热点

聚焦 Rust 生态热点新闻

---



##  JetBrains 下一代 IDE Fleet 发布早期预览版

首批支持语言中包括了 Rust。并且 Fleet 使用了 [skiko](https://github.com/JetBrains/skiko) 写界面，用了Kotlin和一点Rust 。

[https://www.jetbrains.com/fleet/](https://www.jetbrains.com/fleet/)

## Rust for linux: 编写安全的抽象和驱动

Rust有一个关键属性，使其成为内核中的第二语言变得非常有趣:它保证不会发生未定义的行为(只要不安全的代码是合理的)。这包括没有 内存释放后内存使用， double free， 数据竞争等等。

[https://linuxfoundation.org/webinars/rust-for-linux-writing-abstractions-and-drivers/](https://linuxfoundation.org/webinars/rust-for-linux-writing-abstractions-and-drivers/)

## lib.rs: 一个 crates.io的替代品

lib.rs 索引了 Rust的 72,419 个应用。快速，轻量级，可以作为 crates.io 的一个替代品.

[lib.rs](https://lib.rs/)

## Poem-openapi 1.0 发布

Poem OpenAPI是一个基于 Poem 的 OpenAPI 服务器端框架。如果你用过FastAPI，它们在使用上是非常相似的。

1.0版简化了一些宏属性，使用起来更加方便。

```rust
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService};

struct Api;

#[OpenApi]
impl Api {
  #[oai(path = "/hello", method = "get")]
  async fn index(
    &self,
    #[oai(validator(max_length = 32))] name: Query<Option<String>>,
  ) -> PlainText<String> {
    match name.0 {
      Some(name) => PlainText(format!("hello, {}!", name)),
      None => PlainText("hello!".to_string()),
    }
  }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  let api_service = OpenApiService::new(Api, "Hello World", "1.0")
      .server("http://localhost:3000/api");
  let ui = api_service.swagger_ui();

  Server::new(TcpListener::bind("127.0.0.1:3000"))
    .run(Route::new().nest("/api", api_service).nest("/", ui))
    .await
}
```

[https://crates.io/crates/poem-openapi](https://crates.io/crates/poem-openapi)

##  w806-pac项目正在招募贡献者

w806是北京联盛德推出的一款IoT嵌入式MCU。嵌入式rust对它的支持项目正在开发中，其中pac库的svd文件编写工作需要更多的贡献者参与。主要工作是，阅读官方文档，把寄存器信息整理到svd文件中，以供后续嵌入式rust使用。 欢迎大家关注项目！如果您愿意为w806-pac项目做出贡献，请打开项目主页的issues，来查看详细的信息。感谢各位对项目的支持！

- [项目主页](https://github.com/luojia65/w806-pac)

## 【投票】为Rust标准库添加控制台输入API

[Simple Console Input API for Standard Library](https://strawpoll.com/zxds5jye6)

**StrawPoll.com**：

> 我们正试图将一个简单的控制台输入API推送到标准库中，以使编写简单的命令行输入变得更容易，我们需要社区决定实现的高级程度。因为这是一个相当有争议的话题（双方的数量非常均匀），所以这次投票就是为了解决这个问题。

注意：下面的例子不是最终的，它只是一个用来阐述这个概念的例子。

简单输入的例子：

```rust
let age: i32 = std::inputln!()?.parse().expect("Invalid age!");
```

高级输入的例子：

```rust
let person = scan!("{} {}", String, u8).expect("Invalid input!");
```

或者

```rust
let name: String;
let age: u8;

scan!("{} {}", name, age).expect("Invalid input!");
```

**「投票」**：

- A. 简单输入系统：一个函数，读取一行、执行错误检查并返回一个字符串；
- B. 高级输入系统：一个或多个宏，提供格式化输入，用于读取多个值和各种类型；
- C. 二者都选择：在需要时提供简单的输入宏和高级宏；
- D. 这些都不是：别的东西。

如果想参与投票，请访问[原文](https://strawpoll.com/zxds5jye6)投票。

##  RustSBI-Nezha项目已经可以在oreboot引导链中使用

oreboot是类似于coreboot的引导程序项目，而RustSBI是适用于RISC-V的引导程序环境，这两者都是纯粹由rust编写的嵌入式应用程序。

经过社区成员@OrangeCMS和贡献者们的进一步适配，RustSBI-Nezha现在能在oreboot引导链中能正常运行。图片的控制台输出显示，引导链在初始化DDR内存后，能够做到M态的陷入处理，随后进入下一步的系统启动过程。这项适配工作意味着，oreboot和RustSBI-Nezha提供了一种启动Linux系统可用的新引导途径。

RustSBI-Nezha项目是湖南农业大学的杨云枫同学、天津科技大学的王涛同学和队友们在暑假的“2021年开源操作系统夏令营”活动中发起的开源项目。

项目分支地址： [https://github.com/orangecms/rustsbi-nezha/tree/rustsbi-nezha](https://github.com/orangecms/rustsbi-nezha/tree/rustsbi-nezha)

推文链接：[https://twitter.com/OrangeCMS/status/1462197961606246403?t=_n8beWS2OFhygZ9CWdrwiA&s=19](https://twitter.com/OrangeCMS/status/1462197961606246403?t=_n8beWS2OFhygZ9CWdrwiA&s=19)

## 当标准输出写入失败时，Clap 的默认设置会导致 Rust CLI 在 `--help` 上出现恐慌

```rust
nu -h | false
fd -h | false
hyperfine -h | false
```

以上这些工具会出现 panic。

 Artichoke （Rust 实现的 Ruby） 是如何避免这个错误的： [https://github.com/artichoke/artichoke/blob/d527412f9438aeba4cadb1f4303237f6f9e0cd4d/src/bin/artichoke.rs#L138-L173](https://github.com/artichoke/artichoke/blob/d527412f9438aeba4cadb1f4303237f6f9e0cd4d/src/bin/artichoke.rs#L138-L173)

[https://www.reddit.com/r/rust/comments/r48hem/claps_defaults_cause_rust_clis_to_panic_on_help/](https://www.reddit.com/r/rust/comments/r48hem/claps_defaults_cause_rust_clis_to_panic_on_help/)

## Hubris ： 新的嵌入式操作系统

Hubris 是由[Oxide Computer Company](https://oxide.computer/)开发的用于微控制器的操作系统。我们还在Oxide 博客上[发布](https://oxide.computer/blog/hubris-and-humility)了[一篇宣布Hubris](https://oxide.computer/blog/hubris-and-humility)的博客文章。

Hubris 提供抢占式多任务处理、单独编译的组件之间的内存隔离、隔离崩溃的驱动程序并在不影响系统其余部分的情况下重新启动它们的能力，以及灵活的组件间消息传递，消除了对大多数系统调用的需要——大约 2000 行 Rust . Hubris 调试器 [Humility](https://github.com/oxidecomputer/humility) 允许我们走近正在运行的系统并检查所有任务的交互，或捕获转储以进行离线调试。

然而，Hubris 可能更有趣，因为它没有运行时创建或销毁任务的操作，没有动态资源分配，没有以特权模式运行的驱动程序代码，系统中也没有C代码。这通过构造消除了许多通常存在于类似系统中的攻击面。

- [https://github.com/oxidecomputer/hubris](https://github.com/oxidecomputer/hubris)
- [文档](https://hubris.oxide.computer/reference/)
