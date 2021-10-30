# Rust 学习笔记系列｜ Part 5

作者： 李大狗

---

> 本项目代码见：
>
> https://github.com/leeduckgo/weid-rust-sample
>
> 本系列代码见：
>
> https://github.com/leeduckgo/Rust-Study

在上一篇文章中，我们使用 Rust 和 FISCO BCOS 开源框架中的 WeBase 进行了交互，使用了 reqwest 这个 Rust 中的 http 库，同时介绍了 Rust 项目中模块分离的设计。

今天，我们将结合 FISCO BCOS 生态中的数字身份组件 WeIdentity，来讲 Rust 中的数据库操作。

## 1 什么是 数字身份标识与数字身份体系？

首先，让我们来看下什么是分布式数字身份标识（DID）：

> 分布式数字标识符（DID）是一种新型标识符，用以标识可验证的分布式的数字身份。 DID的控制者决定标识的主体（例如，人，组织，事物，数据模型，抽象实体等）。
>
> — W3C DID规范

围绕分布式数字身份标识，我们可以构建如下组件：

- **DID 文档**——用以对该 DID 相关的地址、服务以及其它特性进行进一步的阐述。
- **可验证声明**——通过可验证声明，DID 控制者可以发放、持有、验证电子证书与电子凭证。
- **选择型披露**——结合隐私保护技术，DID 控制者可以在保障自己隐私的情况下，向需要的第三方选择性披露自己的数据，如证明自己的年龄大于 18 岁。
- **数据存证**——将可信数据和 DID 进行挂钩，便完整的形成了「数字身份—数字凭证—数据存证」的体系，如「学生身份—毕业证书—课堂表现」。

这便是**数字身份体系**。

## 2 什么是 WeIdentity？

> WeIdentity是一套分布式多中心的技术解决方案，可承载实体对象（人或者物）的现实身份与链上身份的可信映射、以及实现实体对象之间安全的访问授权与数据交换。WeIdentity由微众银行自主研发并完全开源，秉承公众联盟链整合资源、交换价值、服务公众的理念，致力于成为链接多个垂直行业领域的分布式商业基础设施，促进泛行业、跨机构、跨地域间的身份认证和数据合作。
>
> WeIdentity 目前主要包含两大模块：WeIdentity DID 以及 WeIdentity Credential。
>
> WeIdentity 参考场景：
>
> ![roles-relation.png](https://tva1.sinaimg.cn/large/008i3skNly1gqonvch3gyj30pv0ea0tt.jpg)

## 3 WeIdentity DID规范

WeIdentity 对基本的 DID 规范进行了扩展处理。

基本的 DID 规范：

DID是一个简单的文本字符串，由三部分组成：
1）DID过的URI方案标识符（Scheme，固定就是 did）
2）DID方法的标识符（DID Method）
3）DID方法生成的标识符（DID Method-Specific Identifier）
![](/Users/liaohua/Documents/parts-of-a-did.png)

扩展后的 WeIdentity：

![weidentity-did-format1.png](https://weidentity.readthedocs.io/zh_CN/latest/_images/weidentity-did-format1.png)

| 字段               | 说明                                                         |
| ------------------ | ------------------------------------------------------------ |
| did                | 遵循DID规范，使用固定字符“did”                               |
| weid               | WeIdentity DID规范的method name字段，固定为“weid”            |
| chain-id           | 链 ID，用于路由到不同的链网络（如果需要跟其他链打通，需要找 WeIdentity 开源项目的 owner 微众银行注册路由信息），例如同时使用 WeIdentity 的可能有多条区块链，可以使用这个字段作为标识信息，路由到特定区块链 |
| bs-specific-string | 基于底层区块链平台生成，代表Entity在链上的地址，保证全网唯一 |

备注：bsSpecificString根据区块链底层平台和业务具体情况来确定生成规则，例如可以是随机字符串，或者区块链上的地址。

示例(这个例子中，chain-id是``101``: `"did:weid:101:0x0086eb1f712ebc6f1c276e12ec21"`。

## 4 数据结构设计

在今天的实践中，我们希望可以把weid保存到本地数据库中。

我们这次选择的是 Sqlite 数据库，在Rust - ORM 选择上，我们选择的是 Diesel，这个项目有 6.8k Stars。

> https://github.com/diesel-rs/diesel
>
> **Tips：** 接触新库时，我们可以通过学习 Repo 中的 Examples，来掌握 Repo 的用法。

因为weid中，前半部分`did:weid`是不变的，所以我们只需保存`chain_id`和``bs-specific-string`即可。

在Rust中数据结构如下：

```rust
pub struct Weid {
    id: i32,
    chain_id: i32, //当然也可以同样设置为 String
    addr: String,

    created_at: NaiveDateTime, // 创建时间
    updated_at: NaiveDateTime, // 更新时间
}
```

数据库的创建语句如下：

```sql
CREATE TABLE weids (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  addr TEXT NOT NULL,
  chain_id INTEGER, 
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

我们用标准规范创建数据库迁移文件夹`migration`：

```
migrations
└── 2020-05-13-105400_create_weids
    ├── down.sql
    └── up.sql
```

其中，`up.sql`的内容即是上面的内容：

```sql
CREATE TABLE weids (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  addr TEXT NOT NULL,
  chain_id INTEGER,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

`down.sql`的内容就是移除`weids`表：

```
DROP TABLE weids;
```

## 5 数据库的创建与建表（Create&Migrate）

此处我们使用`diesel`使用的命令行工具。

**（1）复制必要文件到项目目录下**

从 Diesel 的代码仓库中 Clone 代码到本地。

> https://github.com/diesel-rs/diesel

将 Repo 中的 `diesel`、`diesel_cli`、`diesel_derives`、`diesel_migrations`复制到项目根目录下。

在项目根目录下新建`bin`文件夹。

编译 Diesel Repo，将`target/debug/diesel`文件复制到`bin`目录下。

**（2）配置环境变量**

我们需要设置两个环境变量：

- DATABASE_URL——数据库路径
- BACKEND———数据库类型

直接执行如下命令即可：

```bash
export DATABASE_URL="examples.db"
export BACKEND="sqlite"
```

**（3）创建数据库与建表**

执行如下命令：

```bash
./bin/diesel database reset
```

顺利的话，会出现如下返回：

![image-20210513171857371](https://tva1.sinaimg.cn/large/008i3skNly1gr1ve5v2z4j312m04odh4.jpg)

同时根目录下出现`examples.db`文件。

## 6 models.rs

我们在`src`目录下创建`models.rs`文件，在其中定义结构体`Weid`与`NewWeid`，定义 schema（模式）`weids`。

> Scheme，可以简单的理解为我们告诉程序数据库中有哪些字段，这样程序才能顺利对接数据库。

`models.rs`：

```rust
use chrono::NaiveDateTime;
#[cfg(test)]
use diesel::debug_query;
use diesel::insert_into;
use diesel::prelude::*;
#[cfg(test)]
use diesel::sqlite::Sqlite;
use serde_derive::Deserialize;
use std::error::Error;

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
pub struct NewWeid {
    pub chain_id: i32,
    pub addr: String,
}

#[derive(Queryable, PartialEq, Debug)]
pub struct Weid {
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
```

**代码解析：**

> **注：** 以下内容对《Rust 程序设计语言（第一版）》有所参考。
>
> https://kaisery.gitbooks.io/rust-book-chinese/content/content/Traits.html

trait 是一个告诉 Rust 编译器一个类型必须提供哪些功能语言特性。

例如，我们可以为结构体`Circle`实现`HasArea`这个`trait`：

```rust
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}
```

如你所见，`trait`块与`impl`看起来很像，不过我们没有定义一个函数体，只是函数标记。当我们`impl`一个trait时，我们使用`impl Trait for Item`，而不是仅仅`impl Item`。

重复的实现像`Debug`和`Default`这样的 trait 会变得很无趣。为此，Rust 提供了一个[属性](https://kaisery.gitbooks.io/rust-book-chinese/content/content/Attributes 属性.md)来允许我们让 Rust 为我们自动实现 trait：

```rust
#[derive(Debug)]
struct Foo;

fn main() {
    println!("{:?}", Foo);
}
```

`Rust 1.15`中引入了自定义`derive`特性，从而让`derive`有了更多的想象空间。

我们通过`#[derive(Insertable)]`与`#[derive(Queryable, PartialEq, Debug)]`，让该结构体具备可插入数据库，或从数据库查询的特性。

## 7 main.rs

main.rs 的内容如下所示：

```rust
extern crate pretty_env_logger;

pub mod models;

use diesel::prelude::*;
use std::env;
use dotenv::dotenv;

use weid_light_client::WeIdRestService;
use models::*;

use models::schema::weids;
use models::schema::weids::dsl::*;

#[macro_use] extern crate log;

fn main(){
    pretty_env_logger::init(); 

    // create data
    let sqlite_conn = establish_connection();
    create_weid(&sqlite_conn, 1, "34be11396f3a91c5Ab5A1220e756C6300FB2b20a");
    
    // query data
    let results = weids.load::<Weid>(&sqlite_conn)
        .expect("Error loading weids");
    // log weids
    info!("Displaying {} weids", results.len());
    for weid in results{
        info!("did:weid:{}:{}", weid.chain_id, weid.addr);
    }
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_weid(conn: &SqliteConnection, c_id: i32, address: &str) -> usize {
    

    let new_weid = NewWeid {chain_id: c_id, addr: address.to_string()};

    diesel::insert_into(weids::table)
        .values(&new_weid)
        .execute(conn)
        .expect("Error saving new weid")
}
```

**代码解析：**

`establish_connection`直接拷贝自 Diesel 的Examples，作用是根据环境变量中的`DATABASE_URL`连接`sqlite`数据库。

`create_weid`函数中，我们先创建一个 NewWeid 结构体对象，然后通过`diesel::insert_into`函数将新建的结构体对象插入数据库。

通过`loads`函数，我们从数据库中加载`Weid`结构体。

## 8 运行

执行`RUST_LOG=trace cargo run`。

我们成功向数据库插入一条 weid 数据，并读取 出weid 数据。

![image-20210513180708811](https://tva1.sinaimg.cn/large/008i3skNly1gqonwobwtgj31cw076ac3.jpg)

关于`diesel`的更多更详细的用法，见：

> https://diesel.rs/guides/getting-started

