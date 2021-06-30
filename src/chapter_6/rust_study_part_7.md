# Rust 学习笔记系列｜ Part 7

作者：李大狗

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

在之前的两篇文章中，我们分别介绍了：

- 如何本地存储 WeId

  [用 Sqlite 存储 WeId | Rust 学习笔记（五）](https://mp.weixin.qq.com/s/872b8pmCB-WJ_QcS_uUGAg)

- 调用接口在链上生成 WeId

  [链上注册WeId与错误处理 | Rust 学习笔记（六）](https://mp.weixin.qq.com/s/heiZgabqzvBue4SOx0LjBg)

将二者整合，我们可以得到 WeId 的链上生成 — 本地存储的完整闭环。这个项目我已推送到 Github 上并补充了 README，欢迎 STAR~

> https://github.com/leeduckgo/weid-rust-sample

## 项目结构

```bash
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── bin # bin 中放置的是 diesel 二进制软件体
├── diesel # diesel 开头均为数据库相关
├── diesel_cli
├── diesel_derives
├── diesel_migrations
├── ethereum # 以太坊包，为之后本地生成-链上注册做准备
├── model # 同上
├── examples.db # sqlite 数据库
├── migrations # 数据库记录文件
├── src # 主文件
|     ├── main.rs
|     └── models.rs
├── weid-light-client # 解耦轻客户端，对接 weid-rest-service 服务
└── target # 编译后的文件
```

## 当前流程

1. 调用 WeIdGenerator，在链上注册`WeIdentity`。

借助了`reqwest`库，详细使用方法见：

> [链上注册WeId与错误处理 | Rust 学习笔记（六）](https://mp.weixin.qq.com/s/heiZgabqzvBue4SOx0LjBg)

`weid-light-client/src/weid_generator.rs`源码：

```rust
use serde_json::{Value};
use thiserror::Error;

/// Provide an implementation for the default() method:
/// https://doc.rust-lang.org/stable/core/default/trait.Default.html
#[derive(Default)]
pub struct WeIdGenerator{
    endpoint_url: String,
    weid: String, 
}

impl WeIdGenerator{
    pub fn new(endpoint_url: String) -> WeIdGenerator {
        WeIdGenerator {endpoint_url, ..Default::default()}
    }
    /// String or &str?
    /// Ref: https://zhuanlan.zhihu.com/p/123278299
    /// 显然，这取决于很多因素，但是一般地，保守来讲，如果我们正在构建的API不需要拥有或者修改使用的文本，
    /// 那么应该使用&str而不是String。
    /// 等一下，但是如果这个API的调用者真的有一个String并且出于某些未知原因无法将其转换成&str呢？完全没有问题。
    /// Rust有一个超级强大的特性叫做deref coercing，这个特性能够允许把传进来的带有借用操作符的String引用，
    /// 也就是&String，在API执行之前转成&str。我们会在另一篇文章里介绍更多地相关细节。
    pub fn generate_local(&mut self, chain_id: i32, addr: &str) -> String {
        self.weid = "did:weid:".to_string() + &chain_id.to_string() + ":" + addr;
        // Ref: https://stackoverflow.com/questions/38304666/how-to-define-a-copyable-struct-containing-a-string
        // String is copyable, use .clone()
        // String is not implicitly copyable, because that would cause non-obvious memory allocations to occur
        self.weid.clone()
    }

    /// create weid online.
    pub fn create_weid_online(&self) -> Result<Value, GenerateWeIdError>{
        let response = self.call_create_weid()?;
        let resp = self.str_to_json(&response)?;
        Ok(resp)
    }

    fn str_to_json(&self, payload: &str) -> Result<Value, serde_json::Error> {
        serde_json::from_str(payload)
    }
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
}
/// multi error handle:
/// https://my.oschina.net/jmjoy/blog/3190024
#[derive(Error, Debug)]
pub enum GenerateWeIdError {
    #[error("req error")]
    RequestError(#[from] reqwest::Error),
    #[error("parse error")]
    ParseError(#[from] serde_json::Error),
}
```

2. `create_weid_online()`函数结果处理，见`src/main.rs`：

```rust
fn gen_weid_online_and_save(weid_generator: WeIdGenerator) -> Result<Value, GenerateWeIdError>{
  let result = weid_generator.create_weid_online();

  match result {
    Ok(payload) => {
      // TODO
      //weid = payload |> to_weid
      //vec_weid = weid |> vec
      // save to local sqlite
      // info
      Ok(payload)
    },
    Err(e) => {
      info!("{}", e);
      Err(e)
    }
  }
}
```

3. 定义数据结构与数据 CRUD 操作

详见：

> [用 Sqlite 存储 WeId | Rust 学习笔记（五）](https://mp.weixin.qq.com/s/872b8pmCB-WJ_QcS_uUGAg)

`src/models.rs`源码：

```rust
use chrono::NaiveDateTime;
#[cfg(test)]
use diesel::debug_query;
use diesel::insert_into;
use diesel::prelude::*;

use std::env;
use dotenv::dotenv;

#[cfg(test)]
use diesel::sqlite::Sqlite;

pub mod schema {
    diesel::table! {
        weids {
            id -> Integer,
            chain_id -> Integer,
            addr -> Text,

            created_at -> Timestamp,
            updated_at -> Timestamp,
        }
    }
}

use schema::weids;

#[derive(Insertable)]
#[table_name = "weids"]
pub struct NewWeId {
    pub chain_id: i32,
    pub addr: String,
}

#[derive(Queryable, PartialEq, Debug)]
pub struct WeId {
    pub id: i32,
    pub chain_id: i32,
    pub addr: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime, 
}

pub fn insert_default_values(conn: &SqliteConnection) -> QueryResult<usize> {
    use schema::weids::dsl::*;

    insert_into(weids).default_values().execute(conn)
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn save_weid(conn: &SqliteConnection, c_id: i32, address: &str) -> usize {
    
    let new_weid = NewWeId {chain_id: c_id, addr: address.to_string()};

    diesel::insert_into(weids::table)
        .values(&new_weid)
        .execute(conn)
        .expect("Error saving new weid")
}
```

4. 补充 2 中的 TODO 部分

```rust
fn gen_weid_online_and_save(weid_generator: WeIdGenerator) -> Result<Value, GenerateWeIdError>{
    let result = weid_generator.create_weid_online();

    match result {
        Ok(payload) => {
          	// str handle
            let weid_str: String = 
                payload["respBody"]
                .to_string()
                .replace("\"", "");
          	// str to vec
            let vec: Vec<&str> = 
                weid_str
                .split(":")
                .collect();
          	

            let chain_id: i32 = vec[2].parse().unwrap();
            let addr: &str = vec[3];
            // create data
            let sqlite_conn = models::establish_connection();
            
            models::save_weid(&sqlite_conn, chain_id, addr);
            info!("gen and save weid to local {}.", weid_str);
            Ok(payload)
        },
        Err(e) => {
            info!("{}", e);
            Err(e)
        }
    }
}
```

5. 在 main 函数中调用

```rust
fn main(){
    pretty_env_logger::init();
  	// 从环境变量中拿取 "WEID_URL"
    let url = env::var("WEID_URL").expect("DATABASE_URL must be set");
    let weid_generator = WeIdGenerator::new(url.to_string());
    gen_weid_online_and_save(weid_generator);
}
```

## 运行项目

1. 初始化数据库

```bash
./bin/diesel database reset
```

2. 设置环境变量

```bash
# 推荐使用direnv
export DATABASE_URL="examples.db"
export BACKEND="sqlite"
export WEID_URL=<weid-rest-service url>
```

3. 运行项目

```
RUST_LOG=info cargo run
```

目前会在链上创建托管型`WeId`并存储在本地的`Sqlite`数据库中。

![image-20210617172840467](https://tva1.sinaimg.cn/large/008i3skNgy1grmd7pdyzaj30ll0a2wfx.jpg)

## 升级方向

1. 添加「私钥不出域」创建WeId的方式；
2. 实现 `WeIdentity Document`的同步操作；
3. 目前项目开发需要先启动`WeIdentity-Rest-Service`，考虑设计 Mock 接口，方便开发。