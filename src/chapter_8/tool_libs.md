# 推荐项目 ｜ 基础工具库

编辑： 张汉东

---

## Hora 0.1.0 发布

Hora，Rust 实现的近似最邻近搜索（Approximate Nearest Neighbor Search, ANNS）算法库。先发布了 v0.1.0，专注于近似最邻近搜索领域，已经实现了 HNSW（Hierarchical Navigable Small World Graph Index）索引，SSG（Satellite System Graph）索引，PQIVF（Product Quantization Inverted File）索引，BruteForceIndex，其他索引也即将推出。

Hora 可以部署在任何操作系统平台上，已经支持的 PC 操作系统 Linux，Mac OS，Windows，将支持移动设备操作系统 IOS 和Android，以及将来支持嵌入式系统（no_std），并将支持多语言绑定，包括 Python，Javascript，Java，Ruby，Swift 和 R。

相关链接信息：

- Github：[https://github.com/hora-search/hora](https://github.com/hora-search/hora)
- 官网：[https://horasearch.com/](https://horasearch.com/)
- 在线Demo：[https://horasearch.com/#Demos](https://horasearch.com/#Demos)

## voila：另类处理文件的方式

Voila 是一种通过 CLI 工具启动的特定领域语言，用于以快速可靠的方式处理大量文件和目录。

安装需要切换到 nightly 版本：

```rust
$ rustup default nightly
$ cargo install voila
```

一些使用实例：

```rust

# 删除创建日期在 2020年1月1日 之后的所有文件
$ voila ./backup "@creation=date >= 2020-01-01 { print(@name has been deleted) delete(@path) }"
# 删除文件名以 2020 结束的文件
$ voila ./backup "@name ~= #(.*)-2020# { print(@name has been deleted) delete(@path) }"
```

语法如下：

```
$ voila DIRECTORY "<@VARIABLE | STRING | /REGEXP/> OPERATOR <@VARIABLE | STRING | #REGEXP#> [|| | && ANOTHER_CONDITIONAL ...] {OPERATION1-CYCLE-1(ARG1 ARG1, ARG2) OPERATION2-CYCLE-1(ARG1 ARG2) ...; OPERATION1-CYCLE-2(ARG1, ARG2 ARG2, ARG3)...}"
```

[https://github.com/Alonely0/Voila](https://github.com/Alonely0/Voila)

## bustd：用于内存不足场景的进程杀手守护进程

相比 earlyoom 有更少的内存占用（注意是 Linux 系统的，不是跨平台的噢）：

```
$ ps -F -C bustd
UID          PID    PPID  C    SZ   RSS PSR STIME TTY          TIME CMD
vrmiguel  353609  187407  5   151     8   2 01:20 pts/2    00:00:00 target/x86_64-unknown-linux-musl/release/bustd -V -n

$ ps -F -C earlyoom
UID          PID    PPID  C    SZ   RSS PSR STIME TTY          TIME CMD
vrmiguel  350497    9498  0   597   688   6 01:12 pts/1    00:00:00 ./earlyoom/
```

[https://github.com/vrmiguel/bustd](https://github.com/vrmiguel/bustd)

## kas : 一个新的 GUI 库

这个库不知不觉已经来到 v0.9 了。

是否愿意尝试，读者自己决定。

[https://github.com/kas-gui/kas](https://github.com/kas-gui/kas)

## slitter : 可信且经过验证的 slab 分配器

slitter 是由 Backtrace Labs 团队设计实现并用于 C 后端服务器的 slab 分配器，采用 Rust 编写。

在实际生产的两个月中，该团队使用 slitter 来：

- 检测错误的分配类别
- 避免使用任何带内元数据（in-band metadata）
- 保证类型稳定分配
- 允许每个分配类指定如何映射它的备份内存

- 文章： [https://engineering.backtrace.io/2021-08-04-slitter-a-slab-allocator-that-trusts-but-verifies/](https://engineering.backtrace.io/2021-08-04-slitter-a-slab-allocator-that-trusts-but-verifies/)
- GitHub: [https://github.com/backtrace-labs/slitter](https://github.com/backtrace-labs/slitter)

## Connector-x Rust 和 Python 中将数据从 DB 加载到 DataFrame 的最快库

ConnectorX 团队观察到现有解决方案在下载数据时或多或少会多次冗余数据。此外，在 Python 中实现数据密集型应用程序会带来额外的成本。ConnectorX 是用 Rust 编写的，并遵循“零拷贝”原则。这允许它通过变得对缓存和分支预测器友好来充分利用 CPU。此外，ConnectorX 的架构确保数据将直接从源复制到目标一次。

[https://github.com/sfu-db/connector-x](https://github.com/sfu-db/connector-x)

## RillRaate: 带有实时Web界面的系统监控工具

RillRate 是完全使用 Rust 和 Yew 框架制作的机器人、微服务和物联网的快速 UI。 全栈 Rust 是真实存在的！

最新版本增加的新功能：

- 新控件：按钮、开关、选择器和滑块。
- 新数据类型：表格、仪表、直方图（尚未图形化）。

[[Media] System Tools with real-time Web UI 🖥️ 🚀](https://www.reddit.com/r/rust/comments/p1b65e/media_system_tools_with_realtime_web_ui/)

项目使用[RillRate](https://github.com/rillrate/rillrate)(一个为机器人、微服务和IoT设备设计的实时UI工具)，实现了对CPU、内存和系统信息的监控，将数据可视化并实时呈现在web界面上。

[https://github.com/rillrate/rillrate](https://github.com/rillrate/rillrate)

## gzp: v0.3.0 现在支持多线程压缩snappy


关于gzp:

gzp是一个用Rust实现的多线程压缩编码库，目前支持Gzip格式（依赖flate2)和snappy格式(依赖rust-snappy)

[https://github.com/sstadick/gzp](https://github.com/sstadick/gzp)

## httpmock - 一个 http 服务端

- 简单、富有表现力、流畅的 API。
- 许多内置帮助程序可轻松进行请求匹配。
- 并行测试执行。
- 可扩展的请求匹配。
- 具有同步和异步 API 的完全异步核心。
- 高级验证和调试支持。
- 网络延迟模拟。
- 支持正则表达式匹配、JSON、serde、cookies 等。
- 带有Docker 镜像的独立模式。
- 支持基于 YAML 文件的模拟规范。

[https://github.com/alexliesenfeld/httpmock](https://github.com/alexliesenfeld/httpmock)

## helix-editor - 一个受 neovim 启发的编辑器

helix-editor 是一个深受 neovim 启发使用 Rust 开发的编辑器，感兴趣的朋友可以看看。

Github: https://github.com/helix-editor/helix

## cargo-smart-release

cargo-smart-release，无所畏惧地发布工作空间 crate，无需处理依赖关系或版本。

与 cargo release 的比较

cargo-release 是这个工具存在的原因，因为它让我迷上了一个了解git的全自动化发布工作流程。截至2021-08-12，这对简单的工作区或单速率工作区来说是完美的，所以请使用它：cargo install cargo-release。

以下是 cargo smart-release 的不同之处。

- 安全地执行，所以默认情况下，它被解除了武装
- 指定一个或多个 crate，并自动检测哪些板块需要发布
- 处理依赖性循环，以增加整体成功的机会
- 当出现问题时，努力避免让工作区处于不一致的状态
- 成为 gitoxide 的 playground，为应用程序作者提供更多的便利和更多的可行性。

[https://crates.io/crates/cargo-smart-release](https://crates.io/crates/cargo-smart-release)

## jsonschema-rs：Rust Json 校验工具

如果你没有听（用）过 Json Schema，请允许我首先简单介绍一下。JSON Schema 是用于验证 JSON 数据结构的工具，如果你厌恶对 Json 数据各种 if else 的判断和校验，那该工具非常适合。它的官网：JSON Schema | The home of JSON Schema，先看一个简单的例子，假设我们有下面的 Schema：

```rust

{
  "type": "object",
  "properties": {
    "first_name": { "type": "string" },
    "last_name": { "type": "string" },
    "birthday": { "type": "string", "format": "date" },
    "address": {
      "type": "object",
      "properties": {
        "street_address": { "type": "string" },
        "city": { "type": "string" },
        "state": { "type": "string" },
        "country": { "type" : "string" }
      }
    }
  }
}
```

这个 Schema 一共定义了四个字段，每个字段的类型都做了规定，address 本身也是一个 Json Object。此时，有效的数据是：

```json

{
  "first_name": "George",
  "last_name": "Washington",
  "birthday": "1732-02-22",
  "address": {
    "street_address": "3200 Mount Vernon Memorial Highway",
    "city": "Mount Vernon",
    "state": "Virginia",
    "country": "United States"
  }
}
```
而下面这样的无效数据则会被 Json Schema 验证并报错：

```json
{
  "name": "George Washington",
  "birthday": "February 22, 1732",
  "address": "Mount Vernon, Virginia, United States"
}
```

Json Schema 本身是语言无关的，这里已经有很多实现了：Implementations | JSON Schema，Rust 版本的使用与其他语言类似：
```rust
use jsonschema::{Draft, JSONSchema};
use serde_json::json;

fn main() {
    let schema = json!({"maxLength": 5});
    let instance = json!("foo");
    # 编译 Schema
    let compiled = JSONSchema::compile(&schema)
        .expect("A valid schema");
    # 验证实例
    let result = compiled.validate(&instance);
    if let Err(errors) = result {
        for error in errors {
            println!("Validation error: {}", error);
            println!(
                "Instance path: {}", error.instance_path
            );
        }
    }
}
```

这个工具唯一有个麻烦的地方就是编写 Schema 比较费劲，可以理解为设计类。不过好在写好之后就省事了。

[https://github.com/Stranger6667/jsonschema-rs](https://github.com/Stranger6667/jsonschema-rs)

## cargo-auto：自动任务工具
包括：构建、发布、文档等功能。Cargo 功能已经很强大了，为啥还要做这个东西呢？因为有时我们需要做更多的事情，比如复制一些文件、发布到 ftp 或输入长命令。这些重复性任务必须自动化（也称为 “工作流自动化”）。
```rust
$ cargo install cargo-auto
$ cargo auto new
$ cargo auto build
$ cargo auto release
$ cargo auto docs
```

[https://github.com/LucianoBestia/cargo-auto](https://github.com/LucianoBestia/cargo-auto)