# Rust 学习笔记系列｜ Part 4

作者： 李大狗

本文是"Rust 学习笔记"系列的第四篇（除此之外还有两篇用 Rust 写智能合约的）。

在「Mapping 数据结构 | 用 Rust 写智能合约（二）」一文中，我们尝鲜了 FISCO BCOS 中的 Rust 智能合约，今天我们来看一看如何通过 Rust，和 FISCO BCOS 生态中的重要组件 —— Webase 进行交互。

> **WeBase：**
>
> WeBASE（WeBank Blockchain Application Software Extension） 是在区块链应用和FISCO-BCOS节点之间搭建的一套通用组件。围绕交易、合约、密钥管理，数据，可视化管理来设计各个模块，开发者可以根据业务所需，选择子系统进行部署。WeBASE屏蔽了区块链底层的复杂度，降低开发者的门槛，大幅提高区块链应用的开发效率，包含节点前置、节点管理、交易链路，数据导出，Web管理平台等子系统。

在本例中，为简单起见，我们仅需要安装部署WeBase系列中的WeBaseFront即可：

> **WeBaseFront：**
>
> WeBASE-Front是和FISCO-BCOS节点配合使用的一个子系统。此分支支持FISCO-BCOS 2.0以上版本，集成web3sdk，对接口进行了封装，可通过HTTP请求和节点进行通信。另外，具备可视化控制台，可以在控制台上开发智能合约，部署合约和发送交易，并查看交易和区块详情。还可以管理私钥，对节点健康度进行监控和统计。

安装文档：

> https://webasedoc.readthedocs.io/zh_CN/latest/docs/WeBASE-Front/index.html

本文涉及知识点：

- reqwest 这个 Rust Http 库的使用
- lib 的编写与使用

## 1 确保 WeBaseFront 已启动

![image-20210430152702912](https://tva1.sinaimg.cn/large/008i3skNly1gq38yrd9sxj30vw0343z6.jpg)

## 2 创建新的 Rust 项目

1. 创建项目`webase-interactor-example`

```rust
cargo new webase-interactor-example
```

2. 更新目录结构

在这次学习中，我们引入更复杂的项目结构，我们会在项目中创建一个lib并引用它，做到模块解耦。

我们进入项目目录并在项目下创建一个lib：

```rust
cd webase-interactor-example
cargo new webase-interactor --lib
```

我们再创建一个文件，在webase-interactor/src目录下创建`chain.rs`。

这样，我们得到了这样的目录结构：

```
.
├── Cargo.toml
├── src
│   └── main.rs
└── webase-interactor
    ├── Cargo.toml
    └── src
        ├── chain.rs
        └── lib.rs
```

## 3 编写`webase-interactor`库

### 3.1 编写`webase-interactor`的`Cargo.toml`

`webase-interactor`的`Cargo.toml`如下：

```
[package]
name = "webase-interactor"
version = "0.1.0"
authors = ["leeduckgo <albertschr@163.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
reqwest = { version = "0.10", features = ["blocking", "json"] }
tokio = { version = "0.2", features = ["full"] }
```

我们在此引入了`reqwest`这个`HTTP`库。

> An ergonomic, batteries-included HTTP Client for Rust.
>
> https://github.com/seanmonstar/reqwest

`reqwest`支持同步/异步的`http`调用，在本实例中，我们使用同步方案。

### 3.2 编写`chain.rs`

`chain.rs`的内容如下：

```rust
pub struct Chain{
    ip: String,
}

impl Chain{
    pub fn new(ip: String) -> Chain {
        Chain { ip }
    }
    pub fn get_ip(&self) -> String {
        self.ip.to_string()
    }

    pub fn get_block_number(&self) -> Result<String, reqwest::Error>{
        let mut url =self.ip.to_string();
        url += &"WeBASE-Front/1/web3/blockNumber/".to_string();
        let resp = 
            reqwest::blocking::get(&url)?
            // .await?
            .text();
            // .await?;

        resp
    }
}
```

#### 3.2.1 结构体

我们先定义了一个结构体，关于结构体更详细的介绍请见如下两个链接：

> https://www.runoob.com/rust/rust-struct.html
>
> https://kaisery.github.io/trpl-zh-cn/ch05-01-defining-structs.html

在 Rust 中，Struct 语句仅用来定义，不能声明实例，结尾不需要`;`符号，而且每个字段定义之后用 `,`分隔。

如，定义一个矩形：

```
struct Rectangle {
    width: u32,
    height: u32,
}
```

在本项目中，我们定义了一个结构体`Chain`，这个结构体有一个参数`ip`，我们可以通过这个参数定位到相应的`webase`。

#### 3.2.2 结构体方法

如同在面向对象的编程语言中，函数挂载在类（Class）的里面一样，在 Rust 中，我们可以将函数挂载在结构体里面。

如，实现一个结构体函数`area`，计算出矩形的面积。

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

在本项目中，我们定义了如下方法：

- `new`方法：创建一个新的Chain。
- `get_ip`方法：获取Chain实例的ip。
- `get_block_number`方法：通过和Webase交互，获取当前块高。

在`get_block_number`方法中，我们拼接出url字符串，然后调用`reqwest`的`get`函数。

`get`函数的返回值是`Result<String, reqwest::Error>`，所以我们的函数结构是这样的：

```rust
pub fn get_block_number(&self) -> Result<String, reqwest::Error>{ //注意不要遗漏 &self
  // do sth
  resp // 在 rust 中，我们无需return关键字，最后一行不带;，执行的结构即函数返回值。
}
```

### 3.3 编写`lib.rs`

`lib.rs`内容如下：

```rust
//! # WeBase
//!
//! A library to interact with webase.
#![warn(unused_extern_crates)]

pub mod chain;
pub use self::chain::*;
```

这里我们把`chain`声明为一个命名空间。

mod还支持多级嵌套，如：

```rust
// phrases.rs
pub mod english {
    pub mod greetings {
        pub fn hello() {
            println!("Hello!")
        }
        pub fn hey_guies() {
           println!("Hey, guies!")
        }
    }
    pub mod farewells {
        pub fn goodbye() {
            println!("Goodbye!")
        }
        pub fn see_you() {
            println!("See you!")
        }
    }
}

pub mod chinese {
    pub mod greetings {
        pub fn hello() {
            println!("你好!")
        }
        pub fn have_eaten() {
            println!("吃了么?")
        }
    }
    pub mod farewells {
        pub fn goodbye() {
            println!("再见!")
        }
        pub fn everyone_will_know_you() {
            println("天下谁人不识君!")
        }
    }
}
```

这样来管理我们的模块，我们的代码在各种意义上都会更清晰。

## 4 完成主项目

### 4.1 编写`Cargo.toml`

主项目的`Cargo.toml`如下：

```rust
[package]
name = "webase-interactor-example"
version = "0.1.0"
authors = ["leeduckgo <albertschr@163.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
pretty_env_logger = "0.3"
log = "0.4"

reqwest = { version = "0.10", features = ["json"] }
tokio = { version = "0.2", features = ["full"] }
webase-interacter = { path="./webase-interactor", version = "0.1.0"}
```

在本项目中，我们引入了`pretty_env_logger`，以便更好的进行输出。

除此之外，我们还引入了刚才创建的`webase-interactor`。

### 4.2 编写`main.rs`

`main.rs`的内容如下：

```rust
extern crate pretty_env_logger;

use webase_interactor::Chain;

#[macro_use] extern crate log;

fn main(){
    pretty_env_logger::init(); 
    print_block_number();
    
}

pub fn print_block_number() {
    let ip = "http://127.0.0.1:5002/".to_string();
    let chain = Chain::new(ip);
    let res = chain.get_block_number();
    match res {
        Err(e) => {
            println!("error: {}", e);
        }
        Ok(b_number) => {
            info!("last block height: {}", b_number);
        }
    }
}
```

到此为止，我们的代码已经全部编写完成了。

编译：

```rust
cargo build
```

![image-20210430163532957](https://tva1.sinaimg.cn/large/008i3skNly1gq1uw72r7pj30x50aj0ue.jpg)

执行：

```rust
RUST_LOG=info cargo run
```

![image-20210430165231274](https://tva1.sinaimg.cn/large/008i3skNly1gq1vdmzooaj30x20440tu.jpg)

如期打印出当前块高。

我们在`cargo run`命令前加上`RUST_LOG=info`，所以我们在输出的时候仅会打印`info!`函数中的内容。

本系列所有源码见：

> https://github.com/leeduckgo/RustStudy


