---
pub_date: Sat, 27 Mar 2021 16:00:00 GMT
description: Data manipulation Rust vs Pandas

---

# 「译」数据操作：Rust vs Pandas

译者：[pi-pi-miao](https://github.com/pi-pi-miao) / 后期编辑：张汉东

> 原文：[Data Manipulation: Pandas vs Rust](https://able.bio/haixuanTao/data-manipulation-pandas-vs-rust--1d70e7fc)

---


> Rust requires a lot more work compared to Pandas, but, Rust is way more flexible and performant.

与 pandas 相比，rust 需要做更多的工作，但是 rust 使用起来更灵活，更出色

## 介绍

pandas 是 python 的主要数据分析包，但是由于很多原因，如果没有使用 numpty 等工具的话，原生 python 在数据分析等方面性非常差，pandas 是由 Wes McKinney 开发的，并且将这些操作封装到漂亮的 api 中，方便 python 开发者使用其进行数据分析

rust 因为具有出色的数据性能，这也是为什么 rust 不需要像 pandas 那样进行 api 的包装

我相信在 rust 进行数据操作的方法是构建一堆数据结构，但是我可能理解错了，如果是这样的话，请告诉我

下面是我的经验和推理用来比较 rust 和 pandas

## 数据

性能基准是在这个非常随机的数据集上完成的：[这里](https://www.kaggle.com/START-UMD/gtd)，它提供了大约160,000行/ 130列，总大小为 150Mb 的数据，这个数据集的大小对应于我经常遇到的数据集类型，这就是我选择这个数据集的原因，他并不是世界上最大的数据集，更多的学习应该在更大的数据集上进行

已经合并将使用另一个随机数据集已经完成 [这里](https://datacatalog.worldbank.org/dataset/world-development-indicators), the`WDICountry.csv`



## 1、读取和即时数据

### [pandas]

在 pandas 读取和即时数据非常简单，默认情况会处理很多数据质量问题

```python
import pandas as pd

path = "/home/peter/Documents/TEST/RUST/terrorism/src/globalterrorismdb_0718dist.csv"
df = pd.read_csv(path)
```

[rust] 读取 CSV 文件

对于 rust 来说，管理质量差的数据是非常乏味的，在有些数据集中，有些字段是空的，有些行格式不好，有些没有使用 utf-8 编码

要打开 csv，我使用了 csv crate ，它不但能解决上面所有的问题，所以读取可以使用 csv

```rust
let path = "/home/peter/Documents/TEST/RUST/terrorism/src/foo.csv"
let mut rdr = csv::Reader::from_path(path).unwrap();
```

由于格式化质量差，我的使用如下

```rust
use std::fs::File;    
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;

// ...

    let file = File::open(path)?;
    let transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(file);
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(transcoded);
```

[参考][https://stackoverflow.com/questions/53826986/how-to-read-a-non-utf8-encoded-csv-file*](https://stackoverflow.com/questions/53826986/how-to-read-a-non-utf8-encoded-csv-file)

### [rust]即时数据

为了实现数据的即时化，我使用[Serde](https://serde.rs/) 将我的数据序列化和反序列化

要使用 Serde，需要对数据进行 struct 化，使用 struct 是我的代码遵循基于模型的编程范式，每个字段都有一个定义好的类型，它还能让我能在 struct 之上实现 trait 和方法

然而，我想要的数据有130列...而且它看起来没有办法自动生成 struct的 定义，为了避免手动定义，我必须构建自己的结构生成器

````rust
fn inspect(path: &str) {
    let mut record: Record = HashMap::new();

    let mut rdr = csv::Reader::from_path(path).unwrap();

    for result in rdr.deserialize() {
        match result {
            Ok(rec) => {
                record = rec;
                break;
            }
            Err(e) => (),
        };
    }
    // Print Struct
    println!("#[skip_serializing_none]");
    println!("#[derive(Debug, Deserialize, Serialize)]");
    println!("struct DataFrame {{");
    for (key, value) in &record {
        println!("    #[serialize_always]");

        match value.parse::<i64>() {
            Ok(n) => {
                println!("    {}: Option<i64>,", key);
                continue;
            }
            Err(e) => (),
        }
        match value.parse::<f64>() {
            Ok(n) => {
                println!("    {}: Option<f64>,", key);
                continue;
            }
            Err(e) => (),
        }
        println!("    {}: Option<String>,", key);
    }
    println!("}}");
}
````

生成的 struct 如下

```rust
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
struct DataFrame {
    #[serialize_always]
    individual: Option<f64>,
    #[serialize_always]
    natlty3_txt: Option<String>,
    #[serialize_always]
    ransom: Option<f64>,
    #[serialize_always]
    related: Option<String>,
    #[serialize_always]
    gsubname: Option<String>,
    #[serialize_always]
    claim2: Option<String>,
    #[serialize_always]

    // ...
```

skip_serializing_none : 避免在 csv 中出现空字段的错误

serialize_always : 固定写入 csv 的时候的字段的数量

现在我有了自己的结构体，我使用 serde 序列化来填充结构体的向量

```rust
 let mut records: Vec<DataFrame> = Vec::new();

    for result in rdr.deserialize() {
        match result {
            Ok(rec) => {
                records.push(rec);
            }
            Err(e) => println!("{}", e),
        };
    }
```

这生成了我的向量结构体，赞

一般来说，在使用rust的时候，你不应该期望像使用 python 那样流畅的工作

结论

```
在读取/实例化数据的时候，pandas轻而易举的赢得了rust的csv
```

## 2、过滤

[pandas]

pandas 的过滤方法有很多种，对我来说最常见的方法是

```rust
df = df[df.country_txt == "United States"]
df.to_csv("python_output.csv")
```

[rust]

要在 rust 中使用过滤，可以参考 rust 的[向量文档](https://doc.rust-lang.org/std/vec/struct.Vec.html)

有一大堆向量的过滤方法，有狠多还是 nightly 的特性，这些特性在发布的时候非常适合数据操作，对于这个用例我使用了 retain 方法，因为它完全符合我的需求

```rust
  records.retain(|x| &x.country_txt.unwrap() == "United States");
    let mut wtr =
        csv::Writer::from_path("output_rust_filter.csv")?;

    for record in &records {
        wtr.serialize(record)?;
    }
```

pandas 和 rust 的最大区别是 rust 过滤使用了闭包（比如 python 中的 lambda 函数）而 pandas 过滤式基于列的 pandas API，这意味着 rust 可以制造更复杂的过滤器，在我看来这也增加了可读性

### 性能

|        | 时间        | 内存（Gb）    |
| :----- | :---------- | :------------ |
| pandas | 3.0s        | 2.5 Gb        |
| rust   | 1.6s 🔥 -50% | 1.7 Gb 🔥 -32% |

即使我们使用 pandas 的 api 来过滤，我们也可以使用 rust 获得更好的性能

### 结论

在过滤这方面，rust 更快，并且性能更好

## 3、分组

### [pandas]

分组式 python 中使用 pipline 的重要组成部分，如下：

```python
df = df.groupby(by="country_txt", as_index=False).agg(
    {"nkill": "sum", "individual": "mean", "eventid": "count"}
)
df.to_csv("python_output_groupby.csv")
```

[rust]

对于分组 感谢： [David Sanders](https://able.bio/insideoutclub) 分组恶意使用下面

```rust
use itertools::Itertools;


// ...

#[derive(Debug, Deserialize, Serialize)]
struct GroupBy {
    country: String,
    total_nkill: f64,
    average_individual: f64,
    count: f64,
}

// ... 

    let groups = records
        .into_iter()
        // .sorted_unstable_by(|a, b| Ord::cmp(&a.country_txt, &b.country_txt))
        .group_by(|record| record.country_txt.clone())
        .into_iter()
        .map(|(country, group)| {
            let (total_nkill, count, average_individual) = group.into_iter().fold(
                (0., 0., 0.),
                |(total_nkill, count, average_individual), record| {
                    (
                        total_nkill + record.nkill.unwrap_or(0.),
                        count + 1.,
                        average_individual + record.individual.unwrap_or(0.),
                    )
                },
            );
            lib::GroupBy {
                country: country.unwrap(),
                total_nkill,
                average_individual: average_individual / count,
                count,
            }
        })
        .collect::<Vec<_>>();
    let mut wtr =
        csv::Writer::from_path("output_rust_groupby.csv")
            .unwrap();

    for group in &groups {
        wtr.serialize(group)?;
    }
```

虽然这个解决方案不像 pandas 那样优雅，但是为这种场景提供了更好的灵活性

我认为除了 sum and fold 之外，更多的 reduction 方法将会大大提高 rust 中 map-reduce 式操作的开发体验。

### 性能

|        | 时间       | 内存（Gb）  |
| :----- | :--------- | :---------- |
| pandas | 2.78s      | 2.5 Gb      |
| rust   | 2.0s🔥 -35% | 1.7Gb🔥 -32% |

### 结论：

虽然性能更好的是 rust，我建议在 map-reduce 方法使用 pandas，因为它似乎更合适。

## 4、Mutation

### [pandas]

在 pandas 身上做 mutation 的方法有很多，我通常为了性能和功能风格做下面的方式

```python
df["computed"] = df["nkill"].map(lambda x: (x - 10) / 2 + x ** 2 / 3)
df.to_csv("python_output_map.csv")
```

### [rust]

rust 在 mutation 可以使用 iter

```rust
   records.iter_mut().for_each(|x: &mut DataFrame| {
        let nkill = match &x.nkill {
            Some(nkill) => nkill,
            None => &0.,
        };

        x.computed = Some((nkill - 10.) / 2. + nkill * nkill / 3.);
    });

    let mut wtr = csv::Writer::from_path(
        "output_rust_map.csv",
    )?;
    for record in &records {
        wtr.serialize(record)?;
    }
```

### 性能

|        | 时间        | 内存（Gb）  |
| :----- | :---------- | :---------- |
| pandas | 12.82s      | 4.7Gb       |
| rust   | 1.58s🔥 -87% | 1.7Gb🔥 -64% |

在我看来 mutation 就是 pandas 和 rust 的区别所在，pandas 在这方面表现非常糟糕

### 结论

rust 天生适合 mutation 操作

## 5. Merge

### [python]

一般来说 merge 操作在 python 中式非常高效的

```rust
df_country = pd.read_csv(
    "/home/peter/Documents/TEST/RUST/terrorism/src/WDICountry.csv"
)

df_merge = pd.merge(
    df, df_country, left_on="country_txt", right_on="Short_Name"
)
df_merge.to_csv("python_output_merge.csv")
```

### [rust]

对于 rust 的 struct 来说这是一个棘手的部分，对我来说解决合并的办法式添加一个嵌套字段，这里包含我们要合并的另一个结构体，我首先为新数据创建一个新的结构体和新的堆

```rust
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
struct DataFrameCountry {
    #[serialize_always]
    SNA_price_valuation: Option<String>,
    #[serialize_always]
    IMF_data_dissemination_standard: Option<String>,
    #[serialize_always]
    Latest_industrial_data: Option<String>,
    #[serialize_always]
    System_of_National_Accounts: Option<String>,
    //...

// ...

    let mut records_country: Vec<DataFrameCountry> = Vec::new();
    let file = File::open(path_country)?;
    let transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(file);
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(transcoded); 

    for result in rdr.deserialize() {
        match result {
            Ok(rec) => {
                records_country.push(rec);
            }
            Err(e) => println!("{}", e),
        };
    }
```

然后，我将这个新结构与前面的结构克隆到一个惟一的特定字段上。

```rust

impl DataFrame {
    fn add_country_ext(&mut self, country: Option<DataFrameCountry>) {
        self.country_merge = Some(country)
    }
}

//...

    for country in records_country {
        records
            .iter_mut()
            .filter(|record| record.country_txt == country.Short_Name)
            .for_each(|x| {
                x.add_country_ext(Some(country.clone()));
            });
    }
    let mut wtr =
        csv::Writer::from_path("output_rust_join.csv")
            .unwrap();
    for record in &records {
        wtr.serialize(record)?;
    }
```

为了方便和更好的可比性，我复制了数据，但是如果您能够管理它，可以传递引用。



好了!🚀



除此之外，嵌套结构在 CSV 中还不能序列化  对于 rust [这里](https://github.com/BurntSushi/rust-csv/pull/197)



所以我必须把它改写成:

```rust
impl DataFrame {
    fn add_country_ext(&mut self, country: Option<DataFrameCountry>) {
        self.country_ext = Some(format!("{:?}", country))
    }
}
```

最后我们归并

### 性能

|        | 时间        | 内存（Gb）   |
| :----- | :---------- | :----------- |
| pandas | 22.47s      | 11.8Gb       |
| rust   | 5.48s🔥 -75% | 2.6 Gb🔥 -78% |

### 结论

Rust 可以通过嵌套结构体的方式来实现和 pandans 一样的 merge 功能这并不是真正的一对一比较，在这种情况下，这将取决于您的用例。

## 最后的结论

这次比较之后，我的收获如下

​        使用 pandas 的时候，可以 使用小的 csv（<1M行），进行简单的操作数据清理

​        使用 rust 的时候，你可以进行复杂的操作，内存大或者耗时的 piplines，可以自定义构建函数，扩展软件

rust 和 pandas 相比，rust 提供了非常好的灵活性，以及 rust 比 pandas 可以使用多线程的能力，可以并行操作，我相信 rust 可以解决 pandas 不能解决的问题

此外在任何平台上（ web，安卓或者嵌入式 ）上运行 rust 也是 pandas 无法做到的，并且 rust 也可以为尚未解决的挑战提供了新的解决方案

## 性能

性能表也给了我们更加深入了解 rust 的期望，我相信对于大数据处理方面，rust 会提高2-50倍的性能提升，随着时间的推移，rust 比着 python 内存使用量会大大的减少

## 免责声明

在很多方面，pandas 可以被优化，但是优化式有代价的，无论使硬件(例如集群 Cluster  #Dask, GPU #Cudf)，还是依赖于这些优化包的可靠性和维护。

我非常喜欢使用原生 rust 的原因是，rust 不需要额外的硬件，也不需要额外的软件包，此解决方案不需要额外的抽象层，这使得 rust 在很多方面更加直观

## 代码库

### Git repository

[点击这里](https://github.com/haixuanTao/Data-Manipulation-Rust-Pandas)













