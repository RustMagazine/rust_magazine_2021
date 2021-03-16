# 蚂蚁集团 CeresDB 团队 | 关于 Rust 错误处理的思考

作者：evenyag / 后期编辑：张汉东

---

错误处理并非一件容易的事情，尽管在使用 Rust 时，有编译器不厌其烦地督促我们，基本不存在漏掉错误不处理的情况了，但这并不意味着错误处理这件事情变简单了。这里也记录一下我使用 Rust 一段时间后，对于错误处理的一些思考，包含大量主观看法，欢迎读者拍砖。

## 不可恢复错误和可恢复错误

使用 Rust 的人都知道， Rust 错误处理的手段主要分为两种，对于不可恢复的错误（unrecoverable error），可以通过 panic 来直接中断程序的执行，而对于可恢复的错误（recoverable error），一般会返回 Result 。至于什么时候使用 panic ，什么时候使用 Result ，官方提供了一些指导意见，很多文章对这块都有讨论，相信不少人在这上面是能达成共识的，因此本文在这块也不做过多展开。

错误处理中最麻烦的，还是处理可恢复的错误。

## Error 类型

在进行错误处理，首先，你得把自己 Error 类型给定义了。我认为，对于一个新项目来说，定义好自己的 Error 类型甚至是属于最先要做的几件事情之一。即便一开始不做，等到你写到了第一个 Result 时，你也不得不考虑了。定义 Error 类型是一个可简单，可复杂的事情，毕竟在 `Result<T, E>` 里，`E` 其实可以塞任何东西。如果你胆子够大，甚至可以直接把 String 作为 Error 来使用，还能带上一定的错误信息。

```rust
fn make_string_err() -> Result<(), String> {
    Err(format!("Oh, string is not {}", 1))
}

fn string_err_example() -> Result<(), String> {
    make_string_err()?;
    Ok(())
}
```

String 甚至可以转为来使用 `Box<dyn Error>`

```rust
fn string_box_err() -> Result<(), Box<dyn std::error::Error>> {
    Err(format!("Oops, {}", 1))?;
    Ok(())
}
```

不过这种错误处理方式过于简单粗暴，而错误一旦转为了 String ，就丧失了大部分可编程性，上层想要针对某些类型的错误做针对性的处理就会变得非常困难 —— 唯一的手段估计就只剩下字符串匹配了。

更多的时候，我们可能会想要把错误定义为一个 Enum 或者 Struct ，并实现 Error 等相关的 trait 。这是个体力活，如果你还需要处理 std 或者第三方库抛出来的 Error ，还需要手工实现一大堆 `From` 来为自己的 Error 实现相应的转换规则。这样下去，还没等 Error 类型定义完，写代码的热情就已经冷却了。

这些工作太枯燥了，就应该交给工具库去做！而当你去找 Rust 相关的错误处理库（严格来说，可能称为错误管理或者错误定义库更合适）时，就会发现， Rust 的错误处理库也太多了，而且以后可能会更多，这对于有选择困难症的来说简直是灾难。后面我也会从早期到近期挑选出一些比较有代表性的错误处理库，谈下我对他们的理解和在错误处理上的一些看法。当然，由于不是每个库我都使用过，所以也难免理解存在偏颇，欢迎大家指正

## quick-error

在我刚接触 Rust 时，市面上的错误处理库还没有现在多，或者说我对 Rust 错误处理还不如现在了解，挑选库的过程反而比较简单。由于当时 tikv 已经挺有名气了，于是我直接打开 tikv 的项目，发现它在使用 quick-error ，就决定跟着它用了。当时我的需求也很简单，就是希望有个工具库帮我把定义错误的这些 boilerplate code 给包掉，而 quick-error 也正如其名，能够比较麻利地帮我把 Error 类型定义出来。而 Rust 最早的错误处理库基本上也就只帮你干这样的事情，因此其实更像是错误定义库（如今 quick-error 也不仅仅只能帮你定义错误了，不过也是后话了）。

例如下面就是个使用 quick-error 的例子，定义了一个 Error 类型，并且自动实现了 `From<io::Error>`

```rust
quick_error! {
    #[derive(Debug)]
    pub enum MyError {
        Io(err: io::Error) {
            from()
            display("I/O error: {}", err)
            source(err)
        }
        Other(descr: &'static str) {
            display("Error {}", descr)
        }
    }
}
```

## 丢失上下文

然而，仅仅只是把 Error 定义出来只不过是刚刚踏入了错误处理的门，甚至可以说定义 Error 也只是错误处理那一系列 boilerplate code 的一小部分而已。单纯见到错误就往上抛并不难，而且 Rust 还提供了 `?` 运算符来让你可以更爽地抛出错误，但与之相对的，直接上抛错误，就意味着丢弃了大部分错误的上下文，也会给时候定位问题带来不便。

例如有类似下面的代码，使用了刚刚在上面定义的 Error 类型，而 eat()/drink()/work()/sleep() 中任意一个都有可能抛出 `io::Error` 的函数。那么当 daily() 出错时，你拿到的最终信息可能只是个 "I/O error: failed to fill whole buffer" ，而到底是哪里出的错，为什么出错了呢？不知道，因为错误来源丢失了。
```rust
fn daily() -> Result<(), MyError> {
    eat()?;
    drink()?;
    work()?;
    sleep()?;
    Ok(())
}
```

丢失错误源头这种问题在 Rust 里还是很容易发生的，也是 Rust 错误处理里较恼人的一件事。当然，很大的原因还是在于错误提供没有 backtrace （现在也尚未 stable）。为了避免出现类似的问题，遇到错误时就需要注意保存一些调用信息以及错误的现场，概况下来，就是两样东西
- 调用栈，或者说 backtrace
- 错误的上下文，如关键入参

严格来说， backtrace 也属于上下文的一部分，这里分开提更多是考虑到两者在实现层面是有所区分的。有 backtrace 自然方便，但 backtrace 也并不能解决所有问题：

- 光靠 backtrace 其实只能回答哪里出了错的问题，而回答不了为什么出错的
- 一些预期内时常会抛错误的代码路径也不宜获取 backtrace

反过来，通过在日志里打印或者在 Error 类型中追加上下文信息，其实是能反过来推断出调用链路的，使得排查问题不强依赖 backtrace。我在 Rust 里进行的错误处理时做得最多的事情就是，考虑这个地方适不适合打印错误日志：

- 如果适合，打下错误日志和相关信息，继续抛错误
- 不适合，考虑错误直接抛上去了后续是否方便定位问题
    - 如果不方便，还会把 error 和上下文信息 format 下得到新的 error message ，然后产生个新的错误抛出去

这种方式虽说能解决问题，不过并不认为是一种最佳实践，更称不上优雅，光是打印日志和补充错误信息，就得写不少代码，更不提日志和错误信息里有不少内容可能还是相互重复的。

## error-chain 和 failure

有没有办法更方便地将错误的上下文信息放到 Error 里面呢？早期的 error-chain 库在这方面做了不少尝试，其中 `chaining errors` 模式有点类似 golang 中的 `errors.Wrap()` ，允许用户通过 `chain_err()` 将错误或者可转换为错误的类型（如 String）不断地串联起来。

```rust
let res: Result<()> = do_something().chain_err(|| "something went wrong");
```
除此之外，这个库还提供了 `ensure!` ， `bail!` 等工具宏以及 backtrace 功能，这些我认为对后来错误处理库的发展都是由一定启发作用的。不过 error-chain 文档里那一大坨宏定义，各种概念以及说明，对于刚接触 Rust 的人还是比较劝退的。

到了 failure 库， `chain_err()` 的模式改为了通过 `context()` 来携带错误的上下文信息。
```rust
use failure::{Error, ResultExt};

fn root() -> Result<(), Error> {
    a().context("a failed")?;
    b().context("b failed")?;
    Ok(())
}
```

如今错误处理库也基本沿用了 `context()` 这一 api 命名，甚至 `context()` 已经成为了 Rust 风格错误处理的一部分。

尽管我也考虑过使用这两个库替换掉自己项目里在用的 quick-error ，不过，一旦项目变庞大后，这种替换错误处理库以及错误处理风格的工作就多少有点工作量抵不上收益了。另一方面， error-chain 和 failure 作为出现得比较早的错误处理库，更多起到探索和过渡的作用，他们当初需要解决的问题在 std 的 Error trait 的演进下，很多也都不复存在了（起码在 nightly 上是这样），因此他们的演进也基本走到尽头了。包括 failure 的开发后来也逐渐停滞，现在已经是处于 deprecated 的状态了，项目维护者也都推荐用一些更新的错误处理库。

## thiserror + anyhow

对于一些新的错误处理库，目前社区里较为主流的建议可能是组合使用 thiserror 和 anyhow 这两个库。其中 thiserror 可以看作是定义 Error 的一个工具，它只帮你生成一些定义 Error 的代码，别的什么都不做，相当纯粹。

而 anyhow 则为你定义好了一个 Error 类型，基本可以看作是一个 `Box<dyn Error>` ，同时还提供了一些如 `context` 等扩展功能，用起来更加无脑。
```rust
use anyhow::{Context, Result};

fn main() -> Result<()> {
    ...
    it.detach().context("Failed to detach the important thing")?;

    let content = std::fs::read(path)
        .with_context(|| format!("Failed to read instrs from {}", path))?;
    ...
}
```

除此之外， anyhow 的 Error 只占用一个指针大小的栈空间，相应的 Result 的栈空间占用也会变小，在一些[场景](https://zhuanlan.zhihu.com/p/191655266)下也比较有用。

这两个库的作者 dtolnay 建议，如果你是在开发库，则用 thiserror ，而如果是开发应用则使用 anyhow 。这在实践时遇到的一个问题就是所谓库和应用的边界有时候并没有那么清晰：对一个多模块的应用来说，本质上也可以看作是由若干个库构成的，而这些模块或者"库"之间，也可能是有层级关系的。对于这些模块，使用 anyhow 就存在以下问题
- 需要使用 anyhow 专门提供的 Error 类型，可能直接将 `anyhow::Error` 暴露到库的 api 上
- 调用方拿到的不是明确的错误类型
- 无法对 `anyhow::Error` 做 pattern match
- 更近一步，应用也不保证不会有处理具体错误的需求

本质上， `anyhow::Error` 库提供的 Error 类型，更类似一种 Report 类型，适合汇报错误，而不适合处理具体的错误。如果使用 thiserror ，就失去了便利的 `context` 功能，用起来相对没那么方便，而作者看上去也不打算支持这一点。总的看下来， thiserror + anyhow 的组合方案还是存在一定局限性，似乎用起来并没有那么顺手。

## snafu

而 snafu 的方案，则让我看到 context 也是可以和具体的 Error 类型比较优雅地结合起来。不妨看下 snafu 官方的例子

```rust
use snafu::{ResultExt, Snafu};
use std::{fs, io, path::PathBuf};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Unable to read configuration from {}: {}", path.display(), source))]
    ReadConfiguration { source: io::Error, path: PathBuf },

    #[snafu(display("Unable to write result to {}: {}", path.display(), source))]
    WriteResult { source: io::Error, path: PathBuf },
}

type Result<T, E = Error> = std::result::Result<T, E>;

fn process_data() -> Result<()> {
    let path = "config.toml";
    let configuration = fs::read_to_string(path).context(ReadConfiguration { path })?;
    let path = unpack_config(&configuration);
    fs::write(&path, b"My complex calculation").context(WriteResult { path })?;
    Ok(())
}

fn unpack_config(data: &str) -> &str {
    "/some/path/that/does/not/exist"
}
```

上面的例子就体现出 snafu 的一些特点：

- 基于 context selector 的 context 方案
    - 同样是 `io::Error` ， snafu 可以通过不同的 context 返回不同的 enum variant ，同时还能带上一些错误相关信息
    - 比起为 Error 直接实现 `From<io::Error>` 要更有意义，毕竟我们更希望拿到的错误告诉我是 read configuration 出错了，还是 write result 出错了，以及出错的文件 path 是哪个
    - 本质上是把 context 的类型也提前定义了
- 产生的 Error 就是我们自己定义的 Error，无需依赖 snafu 提供的 Error 类型
- 这里其实还有一个隐含的好处，就是这个 Error 是可以做 pattern match 的

关于 snafu 和错误处理， influxdb_iox 其实总结了一份他们错误处理的 style guide ，我觉得很有参考价值，里面也提到了 snafu 的一些[设计哲学](https://docs.rs/snafu/0.6.10/snafu/guide/philosophy/index.html)
- 同样的底层错误可以根据上下文不同而转换为不同的领域特定错误，例如同样是 io 错误，根据上层业务语义的不同能够转换为不同的业务错误
- 在库和应用的场景下都同样好用
- 模块级别的 Error 类型，每个模块都应该定义一个，甚至多个自己专用的错误类型

而这些设计哲学，我认为也是错误处理里比较好的实践。其中，关于 Error 类型应该做到模块级别还是做到 crate 级别（全局），可能会有较多争议，也值得发散开来聊聊。

## 模块级 Error 类型与全局 Error 类型

先摆观点，我认为 Error 类型尽量做到模块级别是更好的，甚至部分函数有专门的 Error 类型也不过分，但是也要摆一个事实，那就是我自己的代码里这一点做得也还不够好。

所以，这里还是要提一下全局 Error 类型的一些好处，起码包括
- 方便做一套全局的错误码，而且类型参数不合法就是比较常见的错误
- 不需要花太多精力定义 Error 类型，很多 enum variant 可以共用，`Result<T, Error>` 也只需要定义一份，，这也是全局 Error 类型最大的优势

但是，全局 Error 类型也存在相应的缺陷
- 所有用到了 Error 类型的模块，其实通过 Error 类型间接和其他模块耦合了，除非你的 Error 类型只想用 `anyhow::Error` 这样的类型
- 即使来源 Error 相同，上下文也不同，定义到一个 enum variant 里面不见得合适
- 更容易出现 Error 抛着抛着不知道哪来的情况

而模块级的 Error 类型则看上去也更符合一个模块化的 crate 应有的设计
- 不存在共用 Error 类型导致的间接耦合
- 更加内聚，每个模块可以专心处理自己的错误， match 错误的范围也大大减少
- 即使不依赖 backtrace ，错误本身也能明确反映出了层次关系和链路

当然，模块级的 Error 类型也并非没有缺点，例如
- 定义 Error 的工作会变多，做全局的错误码会麻烦些，可能需要在上层做一次转换
- 模块层次过深的话，或者一些模块的 Error 字段较多，由于 Rust enum 的特点，越上层的 Error 类型就会越大（std::mem::size_of::<Error>()），像 snafu 同样也会有这样的问题

## 总结

错误处理可能不存在最佳方案一说，更多还是要结合实际场景。即便是谈到错误处理库，我要是大喊一声 snafu 是 Rust 最好的错误处理库，相信社区里肯定也会有一堆人跳出来反对我。而实际上 snafu 也存在自身的缺点，例如 Error 定义的工作量相对大（需要定义各种 context）， Error 类型体积可能会比较大等。

总的来说，错误处理一直是一件麻烦的事。我觉得能做到错误的现场可追溯，就已经算错误处理做得不错了的。经过几年的发展， Rust 的错误处理库初步发展出了 context 和 backtrace 两种记录错误上下文的手段，同时也更加强大和易用了，但我认为目前他们尚未发展到终态，也尚未出现一个库独大的局面。如果说现在我新起个项目或者模块，需要选择一个错误处理库的话，我可能会先尝试下 snafu 。

## 关于我们

我们是蚂蚁智能监控技术中台的时序存储团队，我们正在使用 Rust 构建高性能、低成本并具备实时分析能力的新一代时序数据库，欢迎加入或者推荐，联系人 jiachun.fjc@antgroup.com

## 参考
- https://blog.yoshuawuyts.com/error-handling-survey/
- https://www.ncameron.org/blog/migrating-a-crate-from-futures-0-1-to-0-3/
- https://zhuanlan.zhihu.com/p/225808164
- https://nick.groenen.me/posts/rust-error-handling/
- https://doc.rust-lang.org/book/ch09-00-error-handling.html
- https://github.com/tikv/rfcs/pull/38#discussion_r370581410
- https://github.com/shepmaster/snafu/issues/209
- https://github.com/rust-lang/project-error-handling/issues/24
- https://github.com/rust-lang/rust/issues/53487
- https://github.com/rust-lang/rfcs/blob/master/text/2504-fix-error.md
- https://zhuanlan.zhihu.com/p/191655266
- https://docs.rs/snafu/0.6.10/snafu/guide/philosophy/index.html
- https://doc.rust-lang.org/src/std/error.rs.html#48-153
- https://github.com/facebook/rocksdb/blob/00519187a6e495f0be0bbc666cacd9da467a6c1e/include/rocksdb/status.h#L34
- https://github.com/tailhook/quick-error/issues/22
- https://github.com/dtolnay/anyhow
- https://github.com/dtolnay/thiserror
- https://github.com/tailhook/quick-error
- https://github.com/rust-lang-nursery/failure
- https://github.com/rust-lang-nursery/error-chain
