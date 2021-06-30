# 推荐项目 ｜ 基础工具库

编辑：张汉东

---

## helix ： Rust  实现的新一代文本编辑器

Helix 编辑器具备以下主要特性：

- 多项选择（Multiple selections）
- 通过 tree-sitter 实现语法高亮和代码编辑
- 解析过程是渐进式的，并且速度足够快，足以运行每一次击键。并且它足够稳健，甚至能够输出带有语法错误的结果。
- 内置语言服务器支持
- 语境感知补全： 文档/诊断/Code actions
- 在终端中运行：其基础实现利用终端前端，且与 tmux 连接良好或使用 ssh。
- 使用 Rust 构建，未使用 Electron、VimScript 和 JavaScript，更少能耗。

[https://github.com/helix-editor/helix](https://github.com/helix-editor/helix)

## Findomain ： 发布了新版本

功能介绍摘录：

Findomain提供了一个Subdomains监控服务，提供：目录模糊/端口扫描/漏洞发现 - 以及更多 

这允许您使用多个顶级工具（OWASP AMASS，SUBLIST3R，ASSETFINDER和SUBFINDER）监视您的目标域，并在新子域名时发送警报，以便发送到Discord，Slack，Telegram，电子邮件或推送通知（Android / IOS / Smart Watch /桌面） 。

 唯一需要做的是使用电子邮件地址（如果适用）或/和Webhooks / Telegram聊天信息配置文件，并将您的域放在另一个文件中，一旦您完成了保持您的完整自动子域监控服务 最新使用已发现的新子域，htth网站，打开端口，子域CNAME等的HOST IP，HTTP状态，屏幕截图。 您的所有数据都会安全地保存在关系数据库中，您可以随时申请数据转储。

[https://github.com/Findomain/Findomain](https://github.com/Findomain/Findomain)

## 脑机接口 协议/API Rust 实现

(满脑子都是 Rust.jpg )

针对 NeuroSky MindWave EEG headset 设备 

应该是这家公司 [http://www.neurosky.com.cn/products-markets/mindwave-mobile-2/ ](http://www.neurosky.com.cn/products-markets/mindwave-mobile-2/ )的设备

[https://github.com/junjunjd/rustymind](https://github.com/junjunjd/rustymind)

##  Rust 高性能计时库 

> by lemonhx & zhongzc @ PingCAP 

minstant 在 Linux x86 x86_64 下使用 TSC 和在其他平台上使用 MONOTONIC_COARSE 吊打 std , 精度突破 10ns !

大家快来试用吧!

[https://github.com/LemonHX/minstant](https://github.com/LemonHX/minstant)

## tabled : 用于表格化打印结构体或枚举类型

```rust
use tabled::{Tabled, table};

#[derive(Tabled)]
struct Language {
    name: &'static str,
    designed_by: &'static str,
    invented_year: usize,
}

let languages = vec![
    Language{
        name: "C",
        designed_by: "Dennis Ritchie",
        invented_year: 1972
    },
    Language{
        name: "Rust",
        designed_by: "Graydon Hoare",
        invented_year: 2010
    },
    Language{
        name: "Go",
        designed_by: "Rob Pike",
        invented_year: 2009
    },
];

let table = table!(&languages);
let expected = "+------+----------------+---------------+\n\
                | name |  designed_by   | invented_year |\n\
                +------+----------------+---------------+\n\
                |  C   | Dennis Ritchie |     1972      |\n\
                +------+----------------+---------------+\n\
                | Rust | Graydon Hoare  |     2010      |\n\
                +------+----------------+---------------+\n\
                |  Go  |    Rob Pike    |     2009      |\n\
                +------+----------------+---------------+\n";

assert_eq!(expected, table);
```

[https://github.com/zhiburt/tabled](https://github.com/zhiburt/tabled)

## Crusty

Crusty，一个快速，可扩展，礼貌性的通用网络爬虫（Broad Web Crawler）。Crusty 旨在：

提供一种研究 www 和通用网络爬虫领域的方法；

- 为可扩展性、可配置性和自定义数据收集提供程序接口；
- 快速，在稳定、可预测的单节点性能、不错的硬件饱和度方面；
- 易于扩展；
- 礼貌性，可能是通用网络爬虫中最重要的部分；
- 可监控，日志、自定义指标、实时 Grafana 仪表板；
- 易于交互，仅使用一个命令构建和运行，可重现的 docker 构建；

[https://github.com/let4be/crusty](https://github.com/let4be/crusty)

## trillium.rs: Rust 异步 Web 组件

trillium 是一个 toolkit,可以让你快速的创建 Rust 的异步 Web 应用. 可以运行在 tokio, async-std, 或者smol上. 用起来就像下面一样简单.

```rust

fn main() {
    trillium_smol::run(|conn: trillium::Conn| async move {
        conn.ok("hello from trillium!")
    });
}
```

- [book](https://trillium.rs/)
- [https://github.com/trillium-rs/trillium](https://github.com/trillium-rs/trillium)

## 玄铁处理核 Rust 语言支持库现已发布

这个支持库允许Rust语言访问玄铁处理器的专有功能，包括特殊的页表、寄存器和专有指令，调试模块的支持也在编写之中。支持库适用于玄铁9系列RISC-V处理核，它能很好地支持裸机引导程序、调试器软件和机器监视环境的开发工作，也可用于研发新型操作系统内核。

同时，Rust芯片支持库对裸机环境的开发有一定的帮助。例如，它可以用于开发适配RustSBI的引导程序环境，以制作基于玄铁C906处理核的全志D1芯片引导程序。

支持库已经发布到crates平台，它详细的文档托管在docs.rs网站上；使用木兰宽松许可协议第2版开源，允许商业使用。

[https://github.com/luojia65/xuantie](https://github.com/luojia65/xuantie)

## ripgrep 发布 v13.0.0

ripgrep 13 是一个新的主要版本，本次更新包含bug修复，性能优化，和几个小的突破性的改进，并且修复了windows平台下的一个安全漏洞。 详细的版本说明可以参见：[https://github.com/BurntSushi/ripgrep/releases/tag/13.0.0](https://github.com/BurntSushi/ripgrep/releases/tag/13.0.0)

ripgrep 是一个面向文本行的搜索工具，可以根据正则表达式递归搜索当前的目录。ripgrep在功能上类似其他流行的搜索工具，如 The Silver Searcher，ack 和 grep 等。ripgrep支持Windows，macOS，和Linux，并且对于每次发行版都提供构建好的二进制程序以供下载。

## cuda-oxide

cuda-oxide 是 CUDA 的安全包装器，使用它可以执行和协调 CUDA 内核。

CUDA是Compute Unified Device Architecture的缩写，是Nvidia开发的一项技术，可加速GPU计算流程。

[https://github.com/Protryon/cuda-oxide](https://github.com/Protryon/cuda-oxide)

## xh：友好快速的 HTTP 请求工具
尽可能多地重新实现了 HTTPie 的优秀设计。

```rust
$ xh get baidu.com

HTTP/1.1 200 OK
accept-ranges: bytes
cache-control: max-age=86400
connection: Keep-Alive
content-length: 81
content-type: text/html
date: Sun, 20 Jun 2021 05:13:12 GMT
etag: "51-47cf7e6ee8400"
expires: Mon, 21 Jun 2021 05:13:12 GMT
last-modified: Tue, 12 Jan 2010 13:48:00 GMT
server: Apache

<html>
<meta http-equiv="refresh" content="0;url=http://www.baidu.com/">
</html>
```

[https://github.com/ducaale/xh](https://github.com/ducaale/xh)

## elfcat: ELF visualizer

elfcat 可以从 ELF binary 中生成 HTML, 从而可以让你以一种非常生动的形式查看 ELF. 可以点击示例地址进行查看.

- [演示](http://ruslashev.github.io/elfcat/hello_world.html)
- [https://github.com/ruslashev/elfcat](https://github.com/ruslashev/elfcat)

## gtk4-rs 发布: GTK4 的 rust 绑定
GTK4 的 rust 绑定正式发布，新功能，新官网，新的教程。

- 链接：[https://gtk-rs.org/blog/2021/06/22/new-release.html](https://gtk-rs.org/blog/2021/06/22/new-release.html)
- 教程：[https://gtk-rs.org/gtk4-rs/stable/latest/book/introduction.html](https://gtk-rs.org/gtk4-rs/stable/latest/book/introduction.html)

## FeoBlog v0.4.0 发布

FeoBlog 是对分布式社交网络（协议 + 实现）的探索。

[https://github.com/NfNitLoop/feoblog](https://github.com/NfNitLoop/feoblog)

## smallnum: 编译期间的 number 优化

smallnum 可以对 number 进行 编译时大小优化。 宏返回能够适合静态边界的最小数值类型。对于无符号整数，宏输入是一个最大值。对于有符号整数，宏输入可以是最大值或最小值。

[https://github.com/tnballo/smallnum](https://github.com/tnballo/smallnum)


## fang: 后台任务执行库

fang是一个后台任务执行库. 支持:

- 任务存储数据库
- 并发任务数
- 重启任务等

- [https://www.badykov.com/rust/2021/06/27/fang/](https://www.badykov.com/rust/2021/06/27/fang/)
- [https://github.com/ayrat555/fang](https://github.com/ayrat555/fang)

