# 为 reqwest 增加中间件支持

译者：华为-王江桐，华为-周紫鹏

---

<table><tr><td bgcolor=#D2D2D2><a href="https://truelayer.com/blog/adding-middleware-support-to-rust-reqwest">原文</a></td></tr></table>



<b>继续我们的开源系列，我们为无处不在的reqwest Rust crate提供了一个中间件适配器。</b>

这是我们<b>开源系列</b>的第二篇文章，我们将在其中讨论TrueLayer的工程挑战并开源我们的解决方案。如果你错过了我们的第一篇推文，我们的第一篇推文是Rust中的gRPC负载平衡。（[原文](https://truelayer.com/blog/grpc-load-balancing-in-rust)/[中文月刊](https://rustmagazine.github.io/rust_magazine_2021/chapter_5/rust_grpc_load_balancing.html)）

本文主题是[reqwest-middleware](https://github.com/truelayer/reqwest-middleware)，一个构建在reqwest HTTP客户端之上的crate，用于提供中间件功能。

 

## 问题

当通过网络与内部和外部服务进行通信时，由于服务会失败，大规模运行应用程序需要内置的韧性。 

重试是提高可靠性的常用策略。机制相当简单：将每个请求包装在一个循环中并重试，直到获得成功响应或尝试次数用完为止。

我们的代码库中有数十个客户端：我们不想以特别的方式为每个客户端重新实现重试。

同时，我们更愿意让我们的域代码不受这种网络级别的限制——最完美的方式是，在 HTTP 客户端本身中透明地实现重试。

我们可以编写一个<span style="background-color:#D2D2D2">RetryHttpClient</span>来包装标准客户端，以增加重试功能——但重试并不是全部。我们希望 HTTP 客户端处理其他功能：分布式跟踪header的传播、缓存、日志记录。 

但是我们不想编写<span style="background-color:#D2D2D2">TracingRetryableClient</span>、<span style="background-color:#D2D2D2">TracingRetryableCachingHttpClient</span>、<span style="background-color:#D2D2D2">RetryableTracingCachingHttpClient</span>（顺序很重要！）以及所有其他可能的组合。

我们想要一种**可组合**的抽象模式。

所有这些功能都遵循相同的模式：

\-     我们想在执行请求之前和之后运行一些任意逻辑

\-     该逻辑完全独立于问题域，它只关注底层传输和整个组织统一的要求（例如日志记录标准）。

好消息是，这是软件系统中常见的一个问题，因此也有一个通用的解决方案：中间件。

（P.S.：请注意，我们在本文中指的是一种非常特殊的中间件，该术语本身更为笼统。有关中间件在其他上下文中的用法，请参阅中间件的[维基百科页面](https://en.wikipedia.org/wiki/Middleware)）

 

## Rust HTTP客户端中间件

在TrueLayer，我们使用[reqwest](https://crates.io/crates/reqwest)作为我们所有Rust服务的 HTTP 客户端。

我们选择它是因为它提供了async-first API，与tokio兼容，并且它已广泛的在生产中使用。

遗憾的是，<span style="background-color:#D2D2D2">reqwest</span>不支持现有即用的中间件。

#### 我们的选择是什么？

\-     使用现成的crate替换<span style="background-color:#D2D2D2">reqwest</span>，或者在<span style="background-color:#D2D2D2">reqwest</span>之上做拓展。在撰写本文时，对我们来说，没有其他完善的、支持中间件的 Rust HTTP 客户端能够提供与reqwest一样的功能。[surf](https://crates.io/crates/surf)非常流行并且内置中间件，但[它需要引入async-std](https://github.com/http-rs/surf/issues/295)。

\-     尝试去获取上游实现的中间件支持。reqwest的维护者从 2017 年开始讨论这个问题（请参阅[ISSUE](https://github.com/seanmonstar/reqwest/issues/155)），但似乎仍然没有达成共识，甚至没有就此类功能是否属于该crate 达成共识。因此，我们不太可能在短期内完成某些事情。

\-     最后一个选择是，包装<span style="background-color:#D2D2D2">reqwest</span>并在其上实现中间件，所以这就是我们采用的方法。<span style="background-color:#D2D2D2">reqwest-middleware</span>诞生了。

使用<span style="background-color:#D2D2D2">reqwest-middleware</span>我们能够将中间件附加到<span style="background-color:#D2D2D2">Client</span>上，然后就像我们直接使用<span style="background-color:#D2D2D2">reqwest</span>一样发出请求：

 

```rust
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use reqwest_tracing::TracingMiddleware;

#[tokio::main]
async fn main() {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(TracingMiddleware)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    run(client).await;
}

async fn run(client: ClientWithMiddleware) {
    // free retries!
    client
        .get("https://some-external-service.com")
        .header("foo", "bar")
        .send()
        .await
        .unwrap();
}
```



## 现有技术

在讨论我们的实现之前，让我们先看看现有的一些常用的中间件API：

#### Surf

Surf 是一个Rust HTTP客户端。这是他们[文档](https://docs.rs/surf/2.2.0/surf/middleware/index.html)中的中间件示例：

```rust
/// Log each request's duration
#[derive(Debug)]
pub struct Logger;

#[surf::utils::async_trait]
impl Middleware for Logger {
    async fn handle(
        &self,
        req: Request,
        client: Client,
        next: Next<'_>,
    ) -> Result<Response> {
        println!("sending request to {}", req.url());
        let now = time::Instant::now();
        let res = next.run(req, client).await?;
        println!("request completed ({:?})", now.elapsed());
        Ok(res)
    }
}
```

我们能看到，它接受一个请求对象和一个<span style="background-color:#D2D2D2">next</span>值，该值可用于将该请求转发到剩余的管道中，并返回一个<span style="background-color:#D2D2D2">Response</span>。这让我们在向下转发之前，可以通过改变请求方式来处理请求，我们还可以在返回之前更改从<span style="background-color:#D2D2D2">next.run</span>返回的<span style="background-color:#D2D2D2">res</span>值。

我们甚至可以在<span style="background-color:#D2D2D2">next</span>周围使用控制流，它允许重试和短路：

```rust
#[derive(Debug)]
pub struct ConditionalCall;

#[surf::utils::async_trait]
impl Middleware for ConditionalCall {
    async fn handle(
        &self,
        req: Request,
        client: Client,
        next: Next<'_>,
    ) -> Result<Response> {
        // Silly example: return a dummy response 50% of the time
        if rand::random()::<bool>() {
          let res = next.run(req, client).await?;
          Ok(res)
        } else {
          let response = http_types::Response::new(StatusCode::Ok);
          Ok(response)
        }
    }
}
```



#### Express

Express是一个完善的Node.js Web框架。它的中间件被编写为普通函数，这是他们[文档](https://expressjs.com/en/guide/using-middleware.html#middleware.application)中的一个例子：

```js
app.use(function (req, res, next) {
  console.log('Time:', Date.now())
  next()
})
```

这与surf的方法非常相似，除了我们使用response对象并可以直接改变它：中间件函数不返回任何内容。

#### Tower

<span style="background-color:#D2D2D2">tower</span>是用于网络应用程序的通用Rust组件库。 

它被用于许多著名crate中，例如<span style="background-color:#D2D2D2">hyper</span>和<span style="background-color:#D2D2D2">tonic</span>。<span style="background-color:#D2D2D2">tower</span>的中间件有点复杂，很可能是因为，他们不想强制用户使用动态调度（例如<span style="background-color:#D2D2D2">async_trait</span>）。 

至于其他库，这是tower[文档](https://docs.rs/tower/0.4.8/tower/trait.Layer.html)中给出的示例：

```rust
pub struct LogLayer {
    target: &'static str,
}

impl<S> Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(&self, service: S) -> Self::Service {
        LogService {
            target: self.target,
            service
        }
    }
}

// This service implements the Log behavior
pub struct LogService<S> {
    target: &'static str,
    service: S,
}

impl<S, Request> Service<Request> for LogService<S>
where
    S: Service<Request>,
    Request: fmt::Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        // Insert log statement here or other functionality
        println!("request = {:?}, target = {:?}", request, self.target);
        self.service.call(request)
    }
}
```

忽略用于反压的<span style="background-color:#D2D2D2">poll_ready</span>方法，<span style="background-color:#D2D2D2">tower</span>的<span style="background-color:#D2D2D2">Service</span>被定义为从请求到响应的函数：<span style="background-color:#D2D2D2">call</span>返回一个<span style="background-color:#D2D2D2">Future</span>，其中<span style="background-color:#D2D2D2">Future::Item</span>是<span style="background-color:#D2D2D2">Service::Response</span>的关联类型。

<span style="background-color:#D2D2D2">surf</span>中的异步中间件的trait更为简单，因为它依赖于过程宏(<span style="background-color:#D2D2D2">async_trait</span>)，在trait中使用<span style="background-color:#D2D2D2">async fn</span>语法——在底层它转换为boxing futures 。这是必要的，因为trait方法尚不支持异步。请参阅Nicholas D. Matsakis的[这篇文章](http://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/)以深入了解原因。

<span style="background-color:#D2D2D2">tower</span>中的中间件是通过<span style="background-color:#D2D2D2">Layer</span> trait定义的，该trait将一个服务映射到另一个服务。实现这个特性通常涉及让一个通用结构包装一些<span style="background-color:#D2D2D2">Service</span>并委托对它的调用。

被包装的<span style="background-color:#D2D2D2">Service</span>与<span style="background-color:#D2D2D2">surf</span>和<span style="background-color:#D2D2D2">express</span>中的<span style="background-color:#D2D2D2">next</span>参数起到相同的作用。它提供了一种调用中间件链其余部分的方法。这种方法仍然允许我们使用<span style="background-color:#D2D2D2">next</span>的API相同的方式处理请求和响应。

 

#### Finagle

Finagle是一个用Scala编写的JVM RPC系统。让我们也从finagle[文档](https://twitter.github.io/finagle/guide/ServicesAndFilters.html#filters)中举一个中间件示例：

```scala
class TimeoutFilter[Req, Rep](timeout: Duration, timer: Timer)
  extends SimpleFilter[Req, Rep] {
  def apply(request: Req, service: Service[Req, Rep]): Future[Rep] = {
    val res = service(request)
    res.within(timer, timeout)
  }
}
```



这里的<span style="background-color:#D2D2D2">Service</span>与<span style="background-color:#D2D2D2">tower</span>非常相似：一个从请求到响应的函数。

<span style="background-color:#D2D2D2">Finagle</span>中的中间件称为<span style="background-color:#D2D2D2">Filter</span>。<span style="background-color:#D2D2D2">Filter</span>类型比<span style="background-color:#D2D2D2">tower</span>的<span style="background-color:#D2D2D2">Layer</span>更复杂，因为它不要求<span style="background-color:#D2D2D2">apply</span>中的<span style="background-color:#D2D2D2">Req</span>和<span style="background-color:#D2D2D2">Rep</span>类型与服务参数中请求和回复的类型保持一致。

<span style="background-color:#D2D2D2">SimpleFilter</span>，顾名思义，是具有固定请求/响应类型的简化版本。<span style="background-color:#D2D2D2">SimpleFilter</span>将一个请求和包装服务作为参数，并返回一个响应，因此它的功能类似<span style="background-color:#D2D2D2">tower</span> API，但是将<span style="background-color:#D2D2D2">Layer::layer</span>和<span style="background-color:#D2D2D2">Service::call</span>压缩到了单个<span style="background-color:#D2D2D2">SimpleFilter::apply</span>方法中。

 

#### 中间件类型

一般来说，你会发现，中间件API分为两类：要么是一个参数为请求和next的函数，就像<span style="background-color:#D2D2D2">surf</span>和<span style="background-color:#D2D2D2">express</span>；或者从一个映射服务到另一个，就像<span style="background-color:#D2D2D2">tower</span>和<span style="background-color:#D2D2D2">Finagle</span>.

总的来说，这两种方法都提供了同样多的灵活性。两者都需要每个中间件至少有一个额外的动态分发，因为 Rust不支持在 trait 方法的返回类型中包含<span style="background-color:#D2D2D2">impl Trait</span>（目前），所以我们采用<span style="background-color:#D2D2D2">Next</span>方法，因为这使得更容易实现中间件。<span style="background-color:#D2D2D2">surf</span>和<span style="background-color:#D2D2D2">tower</span>之间的差异证明了这一点。

 

### reqwest-中间件

我们最终得到了一个非常标准的中间件API（有关API的更详细描述，请参阅[文档](https://docs.rs/reqwest-middleware/0.1.0/reqwest_middleware/)）：

 ```rust
 #[async_trait]
 pub trait Middleware {
   async fn handle(&self, req: Request, extensions: &mut Extensions, next: Next<'_>) 
    -> Result<Response>;
 }
 ```



[Extensions](https://docs.rs/truelayer-extensions/0.1.0/truelayer_extensions/)用于以类型安全的方式跨中间件获取任意信息，不论是从外部中间件到更深的中间件，还是从内部中间件到以前的中间件。

出于演示目的，举例一个简单的日志中间件实现：

 ```rust
 use reqwest::{Request, Response};
 use reqwest_middleware::{Middleware, Next};
 use truelayer_extensions::Extensions;
 
 struct LoggingMiddleware;
 
 #[async_trait::async_trait]
 impl Middleware for LoggingMiddleware {
     async fn handle(
         &self,
         req: Request,
         extensions: &mut Extensions,
         next: Next<'_>,
     ) -> reqwest_middleware::Result<Response> {
         tracing::info!("Sending request {} {}", req.method(), req.url());
         let resp = next.run(req, extensions).await?;
         tracing::info!("Got response {}", resp.status());
         Ok(resp)
     }
 }
 ```



```rust
use reqwest_middlewar::ClientBuilder;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let client = ClientBuilder::new(reqwest::Client::new())
        .with(LoggingMiddleware)
        .build();
    client
        .get("https://truelayer.com/")
        .send()
        .await
        .unwrap();
}
```



```dos
$ RUST_LOG=info cargo run
Jul 20 19:59:35.585  INFO post_reqwest_middleware: Sending request GET https://truelayer.com/
Jul 20 19:59:35.705  INFO post_reqwest_middleware: Got response 200 OK
```



## 结论

我们使用启用中间件的客户端包装<span style="background-color:#D2D2D2">reqwest</span>，该客户端使用相同的简单API。这使得能够为我们的韧性和可观察性需求构建可重用的组件。 

最重要的是，我们还发布了<span style="background-color:#D2D2D2">reqwest-retry</span>和<span style="background-color:#D2D2D2">reqwest-opentracing</span>，它们应该能涵盖reqwest crate很多的使用场景。

开发人员现在可以通过导入几个crate并将<span style="background-color:#D2D2D2">with_middleware</span>调用添加到客户端设置代码来强化与远程HTTP的集成——而不会中断任何其他应用程序代码。

