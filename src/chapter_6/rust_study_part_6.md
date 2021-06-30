# Rust 学习笔记系列｜ Part 6

作者： 李大狗（李骜华）

---

> **系列简介：**狗哥 Rust 学习笔记系列是大狗为对抗 Rust 陡峭的学习曲线而推出的 Rust 学习系列，具备如下原则：
>
> 1. **循序渐进原则**
>
> 按照阶梯法则（下一篇的难度是上一篇难度+1）原则进行设计，让学习 Rust 跟打游戏一样简单。
>
> 2. **单一知识点原则**
>
> 一篇文章只讲一个一个知识点，保证简单性与专注性。
>
> 3. **实用原则**
>
> 所有案例均是真实实践案例，实用性超强。

在上一篇文章里，我们开启了`weid-rust-example`项目，学习了如何通过`diesel`项目玩转`SQLite`数据库。今天我们依然在这个项目的基础上往前推进。

## 实现功能

我们将围绕结构体`Struct WeId`，实现`create_weid_online`函数：

```rust
#[derive(Default)]
pub struct WeId{
    endpoint_url: String,
    weid: String, 
}

impl WeId{
	pub fn create_weid_online(&self) -> ... {
    
  }
}
```

 和以往我们实现过的函数不同的是，在这个函数中，我们可以遇到多种可能的错误（Error），因此，在返回值里我们就不能向过去一样，填写`Result<Value, reqwest::Error>`，我们需要通过一个枚举（Enum）把可能的错误打包在一起。

Let's Go!

## create_weid_online 函数拆解

在函数式编程中，我们会遵循「单一职责原则」，简单来说，就是一个函数只做一件事。因此，即使`create_weid_online`是简单的函数，我们依然可以将其拆分：

```
create_weid_online —— 子函数的组合
    |------ call_create_weid —— 通过weid-rest-service的接口注册托管型 weid 并获得返回值
    |------ str_to_json —— 将 &str 值转换为 json 
```

## 子函数的实现

`str_to_json`函数：

```rust
fn str_to_json(&self, payload: &str) -> Result<Value, serde_json::Error> {
	serde_json::from_str(payload)
}
```

这个函数中使用了`serde_json`库，在`Cargo.toml`为：

```rust
[dependencies]
...
serde_json = { version = "1.0" }
...
```

`call_create_weid`函数：

```rust
pub fn call_create_weid(&self) -> Result<String, reqwest::Error> {
    let mut url =self.endpoint_url.to_string();
    url += &"/weid/api/invoke".to_string();
    // ::blocking:: to block
    let response = reqwest::blocking::Client::new()
    .post(&url)
    .json(&serde_json::json!({
        "functionArg": {},
        "transactionArg": {},
        "v": "1.0.0",
        "functionName": "createWeId"
    }))
    .send()?
    .text();
    
    response
}
```

> **注：**在前面的文章中我们介绍了用 reqwest 调用 get 接口，这次我们调用 post 接口。

这个函数中使用了`reqwest`库，在`Cargo.toml`为：

```rust
[dependencies]
...
reqwest = { version = "0.10", features = ["blocking", "json"] }
tokio = { version = "0.2", features = ["full"] }
...
```

WeId-Rest-Service 的接口说明请见：

> https://weidentity.readthedocs.io/zh_CN/latest/docs/weidentity-rest-api.html

## 主函数的实现

以下是`create_weid_online`函数的源码：

```rust
pub fn create_weid_online(&self) -> Result<Value, GenerateWeIdError>{
    let response = self.call_create_weid()?; // line1
    let resp = self.str_to_json(&response)?; // line2
    Ok(resp)
}
```

我们可以看到，在返回值里的 Error 处，我们填的是自定义的错误类型`GenerateWeidError` 。

所以，在`line1`处可能发生的`reqwest::Error`错误，和`line2`处可能发生的`serde_json::Error`，会被汇集在`GenerateWeIdError`中。

## 聚合错误处理的实现

在这里我们使用`thiserror`这个库，这是目前的最佳处理方案。

在`Cargo.toml`中引用`thiserror`：

```rust
[dependencies]
...
thiserror = "1.0"
...
```

官方的例子是这样的：

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
}
```

在这里我们简单使用：

```rust
// 记得 enum 要写在 Struct 外面。
#[derive(Error, Debug)]
pub enum GenerateWeIdError {
    #[error("req error")]
    RequestError(#[from] reqwest::Error),
    #[error("parse error")]
    ParseError(#[from] serde_json::Error),
}
```

这样，就能让`GenerateWeIdError`囊括这个函数中的所有可能 error 了。

> 【补充资料】关于 Enum 枚举：
>
> https://kaisery.github.io/trpl-zh-cn/ch06-01-defining-an-enum.html

## main函数

在`main`函数中，我们对结构体与函数进行调用：

```rust
fn main(){
    let weid = WeId::new("http://127.0.0.1:6001".to_string());
    let result = weid.create_weid_online();
    match result {
        Ok(payload) => println!("{:}", payload),
        Err(e) => println!("{}", e)
    }
```

执行后，打印出了我们期待的结果：

![image-20210527142348963](https://tva1.sinaimg.cn/large/008i3skNly1gr1vdgbq6jj30o501ut8y.jpg)



> 本项目代码见：
>
> https://github.com/leeduckgo/weid-rust-sample
>
> 本系列代码见：
>
> https://github.com/leeduckgo/Rust-Study

