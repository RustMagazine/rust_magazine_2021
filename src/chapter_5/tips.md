# 语言技巧

编辑：张汉东

---

## 为生命周期参数命名

假如有这样一段代码：

```rust
struct Person {
    name: String
}

impl Person {
    pub fn name<'a>(&'a self) -> &'a str {
        &self.name
    }
}
```

上面示例只是简单的单一生命周期参数`'a`，但我们将其重写命名为`'me`，可能会更加直观：

```rust
struct Person {
    name: String
}

impl Person {
    pub fn name<'me>(&'me self) -> &'me str {
        &self.name
    }
}
```

在下面一些场景中，命名生命周期参数可能更好：

### 场景一

```rust
use once_cell::unsync::OnceCell;

struct Provider {
    data_cache: OnceCell<Data>,
    metadata_cache: OnceCell<Metadata>,
}

// ...

fn process_data(data: &Data) -> Result<&str> {
    // ...
}
```

修正为：

```rust
fn process_data<'prov>(data: &'prov Data) -> Result<&'prov str> {
    // ...
}
```

将生命周期参数命名为 `'prov` 有助于标识 data 数据来自于 `Provider`结构体实例。 在 Rust 编译器源码中也能见到`'tcx `这样的命名，用来标识这是类型上下文（typing context ）。

### 场景二

```rust
struct Article {
    title: String,
    author: Author,
}

#[derive(PartialEq, Eq)]
struct Author {
    name: String,
}

struct ArticleProvider {
    articles: Vec<Article>,
}

struct AuthorProvider {
    authors: Vec<Author>,
}

// 这里具有两种生命周期参数命名
struct AuthorView<'art, 'auth> {
    author: &'auth Author,
    articles: Vec<&'art Article>,
}

// 这里具有两种生命周期参数命名
// 在需要指定两个生命周期参数长短关系的时候可以通过 'auth : 'art 这种方式指定，但是此处不需要
fn authors_with_articles<'art, 'auth>(
    article_provider: &'art ArticleProvider,
    author_provider: &'auth AuthorProvider,
) -> Vec<AuthorView<'art, 'auth>> {
    author_provider
        .authors
        .iter()
        .map(|author| {
            let articles = article_provider
                .articles
                .iter()
                .filter(|article| &article.author == author)
                .collect();

            AuthorView { author, articles }
        })
        .collect()
}
```

## 小结

将生命周期参数重新命名，面对使用引用比较复杂的场景，可以增加可读性，方便开发者分析生命周期参数。这算得上一个最佳实践。

来源： [https://www.possiblerust.com/pattern/naming-your-lifetimes](https://www.possiblerust.com/pattern/naming-your-lifetimes)

## 优化技巧：Rust 中 match 分支语句中避免使用 `?`

来自微信群：David Li

最近碰到rust的一个坑，在match 的分支里面使用`?`可能会导致栈内存特别大。

有一个函数，match 一个 Enum，这个Enum 有十几个定义，因此match 有十几个分支，每个分支里面最后都会调一个函数，返回值是Result，因此使用了`?`。

上周测试报告说 debug 版本跑查询进程会崩掉，分析发现栈溢出了。我们上层代码把线程栈设置为 512 KB，然后调试发现某些函数栈内存竟然用了几百KB。

代码示意：

```rust
match SomeEnum {
    One(_) => get_result()?,
    Two(_) => get_result()?,
    Three(_) => get_result()?,
    Four(_) => get_result()?,
    Five(_) => get_result()?,
    //...
}
```

最后把match分支的Result去掉`?`，把 match表达式赋值 给临时变量之后再用`?`，内存占用降下来了。

代码示意：

```rust
let get_res = match SomeEnum {
    One(_) => get_result(),
    Two(_) => get_result(),
    Three(_) => get_result(),
    Four(_) => get_result(),
    Five(_) => get_result(),
    //...
};

let res = get_res?
```

P.S : 还可以获得一个优化技巧是：先把栈内存调低点，让这种问题尽早暴露。

