# tokio 异步传播的缺陷

作者: 吴翱翔

> 原文: [tokio 异步传播的缺陷](https://pymongo.github.io/#/2021/12/async_cancel_propagation.md)

最近项目中遇到一些 Bug:` tokio channel `的接收方不知道为何被 drop 掉了导致 `send Error`

经过 Debug 后发现其实是 hyper 的 cancel propagation 导致的，
以下这个对 hyper 源码中 examples/web_api.rs 的改动就能复现

```diff
diff --git a/Cargo.toml b/Cargo.toml
index 862c20f9..694b8855 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -73,7 +73,7 @@ pnet_datalink = "0.27.2"
 
 [features]
 # Nothing by default
-default = []
+default = ["full"]
 
 # Easily turn it all on
 full = [
diff --git a/examples/web_api.rs b/examples/web_api.rs
index 5226249b..6de7f682 100644
--- a/examples/web_api.rs
+++ b/examples/web_api.rs
@@ -56,6 +56,12 @@ async fn api_post_response(req: Request<Body>) -> Result<Response<Body>> {
 
 async fn api_get_response() -> Result<Response<Body>> {
     let data = vec!["foo", "bar"];
+    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
+    tokio::spawn(async move {
+        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
+        tx.send(()).unwrap();
+    });
+    rx.await.unwrap();
     let res = match serde_json::to_string(&data) {
         Ok(json) => Response::builder()
             .header(header::CONTENT_TYPE, "application/json")
```

`async fn api_get_response` 是一个 hyper 处理 http 请求的异步函数，在里面 spawn 了一个 Task 去做一些费时的操作，
我们用 sleep 模拟需要 5 秒才能做完任务，最后通过 chanel 将处理完的数据/结果发送给 `async fn api_get_response`，
如果客户端还没等 server response 就提前主动关闭连接，hyper 会将 `async fn api_get_response` 给 cancel 掉，
所以 rx 就被 drop 掉导致后续的发送失败

## cancel 没有传播带来的问题

### 大量主动断开连接的恶意请求

这样导致的问题是，客户端没等 response 已经掐断连接， server 端还在不停的查 spawn 协程查数据库，
**极容易被人利用攻击服务器**，例如有人恶意发 10 万个处理时间要很长的请求，请求发出去之后客户端立即 cancel 掉，
此时 server 如果还在处理「已经被 cancel 掉的请求」会造成资源浪费

如果客户端断开连接后，理应让处理该请求的所有关联的异步 Task/Future 从根节点开始往后全部往后传播并取消掉，

否则客户端早就断开连接服务端还在**傻乎乎的继续去查**一堆数据库，消耗大量资源查出来的数据准备返回给客户端的时候才发现 Send Error

### systemctl stop 超时

例如 web server 进程一般通过 [libc::signal](https://man7.org/linux/man-pages/man2/signal.2.html) 回调函数，让进程收到关闭的信号后，graceful shutdown web server

一般收到 cancel 后要把信号传播到每个协程中，但有些**顽固**协程活的时间很长(例如 loop sleep 之类的轮询任务)

最终让 systemd stop 超时无奈发 kill 信号停止，然而发 kill 信号停止进程完成部署更新并不好，因为 `libc::signal` 的回调函数不能也无法处理 SIGKILL 信号，无法进行定制的一些资源回收操作

```
Dec 18 10:39:21 ww systemd[715]: Stopping graph...
Dec 18 10:39:21 ww atlasd[1518986]: 2021-12-18 10:39:21.588323  INFO atlasd: Signal SIGTERM received, stopping this daemon server
Dec 18 10:39:21 ww atlasd[1518986]: 2021-12-18 10:39:21.588408  INFO server::graph: Prepare to stop graph server
Dec 18 10:39:21 ww atlasd[1518986]: 2021-12-18 10:39:21.588744  INFO start_prometheus_exporter{ip=0.0.0.0 port=19100 instance_kind=Graph}:prometheus_exporter(accept): common::metrics::prome>
Dec 18 10:39:21 ww atlasd[1518986]: 2021-12-18 10:39:21.588830  INFO web::server: graceful shutdown web server
Dec 18 10:40:51 ww systemd[715]: graph.service: State 'stop-sigterm' timed out. Killing.
Dec 18 10:40:51 ww systemd[715]: graph.service: Killing process 1518986 (atlasd) with signal SIGKILL.
Dec 18 10:40:51 ww systemd[715]: graph.service: Killing process 1518988 (tokio-runtime-w) with signal SIGKILL.
Dec 18 10:40:51 ww systemd[715]: graph.service: Killing process 1518989 (tokio-runtime-w) with signal SIGKILL.
Dec 18 10:40:51 ww systemd[715]: graph.service: Killing process 1518993 (tokio-runtime-w) with signal SIGKILL.
Dec 18 10:40:51 ww systemd[715]: graph.service: Killing process 1519000 (tokio-runtime-w) with signal SIGKILL.
Dec 18 10:40:51 ww systemd[715]: graph.service: Killing process 1519002 (tokio-runtime-w) with signal SIGKILL.
Dec 18 10:40:51 ww systemd[715]: graph.service: Killing process 1519007 (tokio-runtime-w) with signal SIGKILL.
Dec 18 10:40:51 ww systemd[715]: graph.service: Main process exited, code=killed, status=9/KILL
Dec 18 10:40:51 ww systemd[715]: graph.service: Failed with result 'timeout'.
Dec 18 10:40:51 ww systemd[715]: Stopped graph.
```

## Future 剪枝?

`async fn api_get_response` 可以抽象成 Future 的根节点，里面 spawn 的 Future 可以抽象成儿子节点

我以为一个 rpc 请求处理函数的根节点 Future 由于客户端断开连接会被 hyper 掐断掉，连同该 Future 的所有叶子节点 Future 也全部中止掉，但实际上 **tokio** 并没有这样的 API 或者设计

我希望的是类似 scoped_thread(子线程活的更短) 这样的约束可以让 `async fn api_get_response` spawn 出来的 Future 一定活的更短，这样能减少很多心智负担不必担心当父节点被 drop 之后子 Future 还持有父节点的资源

听上去就有点像 scoped future

## Monoio 群的讨论

```
吴翱翔:
假设 hyper http 处理一个 http(rpc) 请求要 15 秒，handler 函数内 tokio::spawn，此时如果请求没处理完 客户端主动断开链接，hyper 会 cancel propagation 将 HTTP server 的当前请求 的 async fn handler 处理函数给 cancel 掉
但是 Rust 的异步 spawn 很大的问题是，我在 async fn handler 里面的 tokio::spawn 不会被 cancel 掉依然继续执行
spawn 里面的代码还想等 15 秒后给外面的 async fn api_get_response() 的 channel 发 send, 结果外面的 Future 早就被 cancel 掉了

Shuai Xu:
所以大家都希望能有 结构化并发的能力

吴翱翔:
我在想让 `async fn api_get_response()` 持有一个 tokio::spawn 返回的 join handle, 这样能不能在 async fn api_get_response() 被 Drop 的时候把里面的 spawn 的
儿子 Future 全部也 cancel 掉， 实现了 cancel 的传播，好像现在 Rust 很难实现啊

Shuai Xu:
是的 最主要的是缺 AsyncDrop
只能自己注意和处理
(翱翔批注: 社区有人提议借助 linear-types 实现 AsyncDrop 抽象, https://aidancully.blogspot.com/2021/12/linear-types-can-help.html)

吴翱翔:
[MoonFace]写代码都得小心翼翼，父 Future 被 cancel 掉一些资源例如 chancel 被释放，结果 儿子 Future 还在往 父 Future 持有的 receiver 发数据，@张汉东  这种情况编译器也没法检查吧

JZ:
smol里task必须显式detach()才会在后台运行，而tokio 和async-std都是drop自动detach，这个取消的问题不好解
如果用的smol，你这里可以join在task和rx上，应该就可以了吧
(翱翔批注: glommio runtime 也是需要显式 detach)

张汉东:
@吴翱翔 这没法检查
这看上去像是代码设计的问题

JZ:
@吴翱翔 你可以搜一下Rust async cancellation，作者应该是是Joshua，里面有写临时解决方案
```

## 怎么让 hyper 的 cancel 传播下去

```rust
let (tx, rx) = tokio::sync::oneshot::channel::<()>();
let task_handle = tokio::spawn(async move {
    dbg!("task start");
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    dbg!("task finished, prepare send back to parent future Node");
    tx.send(()).unwrap();
});
```

以上写法当客户端提前断开连接的时候 `tx.send(()).unwrap();` 这行就会 panic

为了能让 tokio::spawn 的任务能在请求被 cancel 时提前中止，需要引入更多的 boilerplate code

```rust
struct ReqHandlerContext {
    task_handle: tokio::task::JoinHandle<()>,
    // rx: tokio::sync::oneshot::Receiver<()>
}
impl Drop for ReqHandlerContext {
    fn drop(&mut self) {
        dbg!("fn api_get_response() ReqHandlerContext in drop");
        self.task_handle.abort();
    }
}
let ctx = ReqHandlerContext {
    task_handle,
};
```

当客户端提前断开连接的时候，hyper 的日志如下

```
Listening on http://127.0.0.1:1337
[examples/web_api.rs:60] "task start" = "task start"
[examples/web_api.rs:72] "fn api_get_response() ReqHandlerContext in drop" = "fn api_get_response() ReqHandlerContext in drop"
```

可见 `dbg!("task finished` 这行没被执行之前，spawn 已经被 cancel 掉，符合预期

## 写 spawn 之前必须向上级申请

为了避免 tokio::spawn 处理不当引起各种 Bug，必须严格对 spawn 的使用进行 code review 很限制。

我们公司要求引入新的 tokio::spawn 之前必须**向上级领导申请**，
并将为什么这处要使用 spawn 的原因和该 spawn 的生存期、引入的外部资源 **详细的写进注释中**

并且要求显式的存储 spawn 的 JoinHandle 一般会在 Drop 中主动调用 handle.abort()

spawn 不是银弹，如果只是为了不阻塞代码执行其实还有别的写法可以代替，使用 spawn 之前一定要慎重考虑清楚

## tokio 若干问题的思考

### tokio::task::JoinHandle

文档中说 A JoinHandle detaches the associated task when it is dropped

个人感觉无法像 glommio 的协程 detach 或 libpthead.so 的线程 pthread_detach 让调用者自行决定是否需要 detach 确实很不方便

### pin Future to CPU core

tokio 中缺少像 glommio pin to core 的概念，类似于 libpthread.so 的 pthread_setaffinity_np? 似乎 tokio 也没有

这也是 tokio 为了跨平台带来的限制，有些 API 在 Linux 系统上本来就可以实现，但是可能因为 windows/mac 没有所以 tokio 就干脆不提供了

tokio 能不能让某个 Future 固定在某个 CPU 核心上执行，避免在多个核心间执行带来的上下文切换开销，
但这似乎跟 tokio 想让所有核心负载均衡的工作窃取法冲突，tokio 不希望一核有难多核围观

### 单线程的 runtime 有些场合反而更快

在我们项目中 benchmark 发现某些模块用 tokio 的单线程 runtime 反而性能会更好，所以不能迷信多线程就一定比单线程性能好，由于 CPU 多个核心之前同步数据的开销很大，需要具体情况具体分析到底用单线程还是多线程 runtime

(但单线程和多线程的 tokio::spawn 的 Future 签名是一样的... 单线程并没有少一个 Send 约束...)

---

最后大胆的设想下能不能从 Linux only 的角度出发去设计 runtime, 完全不用考虑兼容 mac/windows 这样能用很多 Linux only 的 API 充分榨干 Linux 性能，可能最后发现 Rust 自身的 Future 也不太够用或者不好用，甚至会造一个自己运行时专用的 Future (就像 C++ 很多库各自造各自的 Future 不通用)
