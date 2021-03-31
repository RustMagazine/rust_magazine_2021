# 「Rust入门系列」Rust 中使用 Mysql


作者：张军军 / 后期编辑：张汉东

> 这个系列的文章，我计划给大家讲解如何在Rust中使用Mysql作为存储，先从简单的开始，然后在后面展示如何在开发`Web api`中使用。


### 数据表


本次我会使用一张订单表`order`。订单表的具体`schema`如下。


```sql
CREATE TABLE `student` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(128) NOT NULL,
  `age` int(11) NOT NULL,
  `id_card` varchar(128) NOT NULL,
  `last_update` date NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- 插入测试数据
insert into student (name, age, id_card, last_update) values ('张三', 23, '123456789X', CURRENT_DATE());
insert into student (name, age, id_card, last_update) values ('李四', 24, '8382353902', CURRENT_DATE())
```


### 创建应用程序
```rust
cargo new mysql-test-01
```


由于要使用`Mysql`的驱动，所以添加依赖到`Cargo.toml`


```rust
[dependencies]
mysql = "*" // 通配符*表示可以使用任何版本，通常会拉取最新版本
chrono = "0.4"
```
在这里，我使用`chrono`来处理日期和时间列。具体 可以参考[ `https://docs.rs/chrono/0.4.19/chrono/`](https://docs.rs/chrono/0.4.19/chrono/)


### 开始


在main.rs中导入命名空间
```rust
use mysql::*;
use mysql::prelude::*;
use chrono::prelude::*; // 用来处理日期
```


获取`Mysql`连接


```rust
fn main() {
    let url = "mysql://root:password@localhost:3306/MYDB";
    let pool = Pool::new(url).unwrap(); // 获取连接池
    let mut conn = pool.get_conn().unwrap();// 获取链接
}
```


先跑一下，确保可以打开一个连接


```rust
cargo run
```


第一次下载和编译所有依赖，可能需要一点点时间，看到命令行编译过去了，表示和数据库已经打通了。


### 流式查询


流式查询，其实结果数据是逐行读取的。 好处就是，整个数据永远不会存储在内存中，如果要读取大量数据，使用`query_iter`很好。

```rust
 conn.query_iter("select * from student")
        .unwrap()
        .for_each(|row| {
            let r: (i32, String, i32, String, NaiveDate) = from_row(row.unwrap());
            println!("{}, {},{},{}, {:?}", r.0, r.1, r.2, r.3, r.4);
        });
```


上面代码中的`row`的类型是`mysql_common::row::Row`，这种类型把数据以字节的形式存储。所以这里需要把低级的字节转换成我们想要的类型比如`i32,String`等，这里我使用了`from_row`。注意，转换后的数据以元组的形式返回，其中每一项和选择列的顺序相同。


### 聚合查询结果


其实， 可以将查询结果收集到Vec中。 Vec中的每个元素都是一个元组。


```rust
// 输出到Vec
let res: Vec<(i32, String, i32, String, NaiveDate)> =
	conn.query("select * from student").unwrap();
for r in res {
    println!("{}, {},{},{}, {:?}", r.0, r.1, r.2, r.3, r.4);
}
```
`query`函数已经将字节转换为选择的数据类型，因此不需要再转换了。 需要注意的就是，这里必须明确元组的数据类型。 否则，编译器没办法做转换。

### 结果到结构体


使用元组也可以。 但是我们实际写代码时，数据表列数多，最普遍的做法就是定义一个结构体。比如这里叫`Student`, 然后，可以使用`query_map`将查询结果映射到`Student`对象。这里
不需要置顶元组的数据类型，编译器会自动推导字段类型根据Student类型
```rust
struct Student {
    id: u64,
    name: String,
    age: u16,
    id_card: String,
    last_changed_on: NaiveDate,
}

let res = conn.query_map(
    "select * from student",
    |(id, name, age, id_card, update)| Student {
        id: id,
        name: name,
        age: age,
        id_card: id_card,
        last_changed_on: update,
    },
).expect("Query failed.");

for i in res {
    println!(
        "{}, {},{},{}, {:?}",
        i.id, i.name, i.age, i.id_card, i.last_changed_on
    )
}
```


### 单条数据查询


查询特定数据行，可能会出现下面几种情况

- 找到，返回实际数据
- 没有找到行
- 发生错误



所以，使用query_first函数返回的是Option的结果。 需要将其解包两次才可以获取实际的行数据。

```rust
 // 条件查询，查询单个数据
let res = conn.query_first("select * from student where name = '张三'")
.map(
    // Unpack Result
    |row| {
        row.map(|(id, name, age, id_card, update)| Student {
            id: id,
            name: name,
            age: age,
            id_card: id_card,
            last_changed_on: update,
        })
    },
);

match res.unwrap() {
    Some(student) => println!(
        "{}, {},{},{}, {:?}",
        student.id, student.name, student.age, student.id_card, student.last_changed_on
    ),
    None => println!("Sorry no student found."),
}
```


### 命名参数的使用


```rust
 let res = conn
        .exec_first(
            "select * from student where name = :name",
            params! {
                "name" => "李四"
            },
        )
        .map(
            // Unpack Result
            |row| {
                row.map(|(id, name, age, id_card, update)| Student {
                    id: id,
                    name: name,
                    age: age,
                    id_card: id_card,
                    last_changed_on: update,
                })
            },
        );
```


总结

- 经常使用的时间处理库：`chrono`
- 流式查询使用： `query_iter`
- 输出到Vec使用：`query`
- 映射到结构体使用： `query_map`
- 获取单条数据使用：`query_first`
- 命名参数查询使用：`exec_first`









