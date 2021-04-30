# 推荐项目 ｜ 基础工具库

编辑：张汉东

---

## Rust Search Extension v1.2 发布

- 支持使用 `//` 前缀搜索 rustc 编译器的源码。
- 支持直接搜索 Rust 版本号，跳转到该版本的 release 界面。
- 新的源码跳转模式：在关键字前添加 `src:` 或 `s:`，搜索结果会跳转到源码页面。
- 新的 `:blog` 命令。

[https://rust.extension.sh/changelog/](https://rust.extension.sh/changelog/)

## Ockam：用于端到端加密、相互身份验证通信的Rust库

用于边缘设备和云服务之间的端到端加密、相互认证通信的Rust和Elixir库。

物联网中的机器通过与云服务和其他联网机器交换信息来进行操作。安全的、私有的和可信的连接应用程序必须保护这些消息不被窃听、篡改和伪造。

[https://github.com/ockam-network/ockam/](https://github.com/ockam-network/ockam/)

## Himalaya: 极简命令行邮件客户端

[https://github.com/soywod/himalaya](https://github.com/soywod/himalaya)

## Amadeus - Rust 大数据处理

Amadeus 是一组开箱即用、低层可重用构建件，来构造 Rust 分布式计算和大数据生态。 它宣称符合以下原则：

- 无畏：无数据竞争，无 unsafe 代码，无损失数据克隆
- 无感分布式：分布式运行时和在本地运行时一样简便和高性能
- 数据递进类型化： 当调度计划已知时可以最大化性能，其他情况则可以保持灵活性
- 简洁：尽可能地保持接口和实现简单可靠
- 可靠：尽可能减少不可处理的错误，并且只会抛出内部不可处理的错误

[https://github.com/constellation-rs/amadeus](https://github.com/constellation-rs/amadeus)

## 使用 activex 和 feed-rs 开发的 RSS 浏览器

作者使用actix和feed-rs开发了自己的RSS浏览器。它很简单，作家将尽量保持这种方式，但仍然会添加一些功能。

- Live: [https://mevlyshkin.xyz/rss](https://mevlyshkin.xyz/rss)
- Repo: [https://git.sr.ht/~leinnan/rust_blog](https://git.sr.ht/~leinnan/rust_blog)

## robusta ： 一个方便生成 JNI 交互代码的库

有了它就可以避免写那些繁琐冗长的 Java JNI 函数名了！

例子：
```rust

use robusta_jni::bridge;
use robusta_jni::convert::Signature;

#[bridge]
mod jni {
    #[derive(Signature)]
    #[package(com.example.robusta)]
    struct HelloWorld;

    impl HelloWorld {
        pub extern "jni" fn special(mut input1: Vec<i32>, input2: i32) -> Vec<String> {
            input1.push(input2);
            input1.iter().map(ToString::to_string).collect()
        }
    }
}
```

```java
package com.example.robusta;

import java.util.*;

class HelloWorld {
    private static native ArrayList<String> special(ArrayList<Integer> input1, int input2);

    static {
        System.loadLibrary("robusta_example");
    }

    public static void main(String[] args) {
        ArrayList<String> output = HelloWorld.special(new ArrayList<Integer>(List.of(1, 2, 3)), 4);
        System.out.println(output)
    }
}
```

[https://github.com/giovanniberti/robusta](https://github.com/giovanniberti/robusta)

## SWC  1.2.52 版发布

swc 是一个 Rust 编写的 typescript/javascript 编译器, 可以生成兼容老旧浏览器的 javascript 代码.

单核比 babel 快 20倍, 4 核比 babel 快 70 倍.

[https://swc.rs/blog/2021/04/11/swc-1.2.52/](https://swc.rs/blog/2021/04/11/swc-1.2.52/)

## dipa - 轻松为 Rust 数据结构添加增量编码支持

dipa 可以轻松高效地对大型 Rust 数据结构进行增量编码。增量编码技术可以用于确定在数据结构的两个实例之间发生了什么变化，利用此项技术，可以有效减少传输相似数据所耗费的流量和带宽。

- Github: [https://github.com/chinedufn/dipa](https://github.com/chinedufn/dipa)
- The dipa Book: [https://chinedufn.github.io/dipa](https://github.com/chinedufn/dipa)

## static_init v1.0 发布 - 更快的静态变量

static_init 支持安全的可变静态量和非常量静态量初始化，与 lazy_static 或 parking_lot::RwLock 相比，具有 200 倍的速度提升。

[https://crates.io/crates/static_init](https://crates.io/crates/static_init)

## Macchina: 快速，精简和可定制的系统信息提取器

Macchina允许您查看基本的系统信息，例如主机名，内核，正常运行时间，内存使用情况等等。它为您提供方便的功能和广泛的自定义选项，但不会忽略其两个主要优先级，即简约和性能。

[https://github.com/Macchina-CLI/macchina](https://github.com/Macchina-CLI/macchina)

## Evcxr : Rust  Jupyter  notebook 

本文主要是对 Rust Jupyter 内核的创建者David Lattimore的采访。看完感觉 Rust 用于数据科学领域指日可待。

摘录一些：

1. evcxr的发音是 “e-vix-er” 。
2.  David 之所以做这个，也是因为受他妻子影响。他妻子是数据科学家，常用 Jupyter notebook ，受她影响 David 尝试做了 Jupyter rust 核心，然后把 他之前做的 Rust REPL 工具 集成。
3. 目前 Rust REPL 之类的工具还在维护的就是 evcxr ，以及一个最近几天刚开始的新库 IRust 。像之前的 rusti 之类的库已经停止维护了。
4.  从  syn 改为 使用 rust-analyzer 来获取类型 ：https://github.com/google/evcxr/commit/b82b7eabb9fe5f4fb7de42c686bd52148ad42a24 (将 ra直接当库使用，缺点是编译太慢，另一种作者想到的可以集成的替代方法是实际引入rust-analyzer二进制文件并使用语言服务器协议与其进行对话，但作者还没这么做)
5. 一本新书：《DATA ANALYSIS WITH RUST NOTEBOOKS 》 https://datacrayon.com/shop/product/data-analysis-with-rust-notebooks/
对应 video 介绍：https://www.youtube.com/watch?v=0UEMn3yUoLo

- [https://blog.abor.dev/p/evcxr](https://blog.abor.dev/p/evcxr)
- [https://github.com/sigmaSd/IRust](https://github.com/sigmaSd/IRust)

## rustcommon : Twitter 开源的通用 Rust 库

看来 Twitter 内部也用 Rust 。

这是 Twitter 内部 Rust 项目通用库，包含数据结构/ log/ metrics/ timers/ ratelimiting。

[https://github.com/twitter/rustcommon](https://github.com/twitter/rustcommon)

Twitter 的 另外两个 Rust 开源项目：

- [https://github.com/twitter/rpc-perf](https://github.com/twitter/rpc-perf)
- [https://github.com/twitter/rezolus](https://github.com/twitter/rezolus) （支持  eBPF）

## vicis：允许你用 Rust 操作 LLVM-IR 

目前完成度不高

[https://github.com/maekawatoshiki/vicis](https://github.com/maekawatoshiki/vicis)

## delay-timer  0.4 发布

delay-timer是一个基于时间轮算法构建的lib，它可以很方便地管理定时任务，或者周期性地执行任意任务。 

轻松打造一个延迟/周期任务的运行容器。可以想象成crontab，但可以处理同步/异步任务，但支持动态添加/取消/删除/更新， 单个任务支持配置任务的最大并行数量，运行时间等。 

底层运行时基于的smol和tokio（可选的feature），你可以用其中一个来构建你的应用程序。
  
v0.4.0 新功能:

1. 支持动态的修改运行中的任务。 
2. 支持了insert任务后获取句柄`TaskInstancesChain`，可以动态获取运行中的任务实例`TaskInstance`。 
    - 运行中任务的任务实例可以动态取消。
    - 取消分为三种方式：同步阻塞取消、超时限制取消、异步取消。    
    - 支持读取运行中任务的运行状态。   
3. 支持获取内部异步子任务进程的输出。
4. 更新依赖:  
    - 替换 waitmap -> dashmap . 
    - 升级 cron_clock . 
5. 更新examples: 
    - 增加，async-std & tokio 使用案例。    
    - 增加，动态取消运行中任务实例案例。 
6. 丰富了文档。 

- repo: [delay-timer](https://github.com/BinChengZhao/delay-timer)
- doc: [delay-timer-doc](https://docs.rs/delay_timer)
- crates: [delay-timer-crates](https://crates.io/crates/delay_timer)

## perf-monitor-rs： 飞书 Rust 团队最近开源的一个跨平台的性能数据采集库

目前可以对cpu、内存、io和fd的相关数据进行采集。

[https://github.com/larksuite/perf-monitor-rs](https://github.com/larksuite/perf-monitor-rs)

## Zellij: Rust编写的新终端复用工具

![img](https://raw.githubusercontent.com/zellij-org/zellij/main/assets/demo.gif)

- [https://zellij.dev/](https://zellij.dev/)
- [https://github.com/zellij-org/zellij](https://github.com/zellij-org/zellij)

