---
pub_date: Thu, 30 Apr 2021 18:00:00 GMT
description: use nom to parse url

---

# 使用 nom 解析 url

编辑：张汉东

---

> [原文](https://blog.logrocket.com/parsing-in-rust-with-nom/) 

</br>

![nom](../image/parsing-rust-nom.png)

</br>

在本教程中，我们将演示如何使用 nom 解析器组合器库在 Rust 中编写一个非常基础的 URL 解析器。我们将包含以下内容

- [什么是解析器组合器?](#什么是解析器组合器?)
- [nom是如何工作的](#nom是如何工作的)
- [设置nom](#设置nom)
- [数据类型](#数据类型)
- [nom中的错误处理](#nom中的错误处理)
- [使用Rust写一个解析器](#使用Rust写一个解析器)
- [解析待授权的URL](#解析待授权的URL)
- [Rust解析：主机，IP和端口](#Rust解析：主机，IP和端口)
- [使用Rust解析路径](#使用Rust解析路径)
- [查询和片段](#查询和片段)
- [在Rust中使用nom解析：最终的测试](#在Rust中使用nom解析：最终的测试)

## 什么是解析器组合器?

解析器组合器是高阶函数，可以接受多个解析器作为输入，并返回一个新的解析器作为输出。

这种方式让你可以为简单的任务(如：解析某个字符串或数字)构建解析器，并使用组合器函数将它们组合成一个递归下降(recursive descent)的解析器。

组合解析的好处包括可测试性，可维护性和可读性。每个部件都非常小且具有自我隔离性，从而使整个解析器由模块化组件构成。

如果你对这个概念不熟悉，我强烈推荐你阅读 Bodil Stokke 的[用 Rust 学习解析器组合器](./01-用Rust学习解析器组合器.md)。

## nom是如何工作的

[nom](https://github.com/Geal/nom) 是使用 Rust 编写的解析器组合器库，它可以让你创建安全的解析器，而不会占用内存或影响性能。它依靠 Rust 强大的类型系统和内存安全来生成既正确又高效的解析器，并使用函数，宏和特征来抽象出容易出错的管道。

为了演示 `nom` 是如何工作的，我们将创建一个基础的 URL 解析器。我们不会完整的实现 [URL 规范](https://url.spec.whatwg.org/)；这将远远超出此代码示例的范围。相反，我们将采用一些捷径。

最终的目标是能够将合法的 URL (如：[https://www.zupzup.org/about/?someVal=5&anotherVal=hello#anchor](https://www.zupzup.org/about/?someVal=5&anotherVal=hello#anchor) 和 [http://user:pw@127.0.0.1:8080](http://user:pw@127.0.0.1:8080)) 解析成相关的结构，并在解析过程中为非法的 URL 返回一个有用的错误。

而且，由于可测试性被认为是解析器组合器的一大优势，我们将对大多数组件进行测试，以了解其具体的优势。

让我们开始吧！

## 设置nom

为了进行下面的一系列操作，你需要安装最新的 Rust 版本 (1.44+)。

首先，创建一个新的 Rust 项目:

```console
cargo new --lib rust-nom-example
cd rust-nom-example
```

然后，编辑`Cargo.toml`文件并添加你需要的依赖：

```toml
[dependencies]
nom = "6.0"
```

是的，我们需要的是最新版本的`nom`库(在撰写本文时是 6.0)。

## 数据类型

编写解析器时，通常先定义输出结构以了解你需要哪些部分是很有意义的。

在这里，我们正在解析一个 URL，因此，让我们给它定义一个结构：

```rust
#[derive(Debug, PartialEq, Eq)]
pub struct URI<'a> {
    scheme: Scheme,
    authority: Option<Authority<'a>>,
    host: Host,
    port: Option<u16>,
    path: Option<Vec<&'a str>>,
    query: Option<QueryParams<'a>>,
    fragment: Option<&'a str>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Scheme {
    HTTP,
    HTTPS,
}

pub type Authority<'a> = (&'a str, Option<&'a str>);

#[derive(Debug, PartialEq, Eq)]
pub enum Host {
    HOST(String),
    IP([u8; 4]),
}

pub type QueryParam<'a> = (&'a str, &'a str);
pub type QueryParams<'a> = Vec<QueryParam<'a>>;
```

让我们逐行进行说明。

这些字段是根据它们在常规 URI 中出现的顺序进行排列的。首先，我们有 scheme。在这里，我们将 URI 的前缀限制为`http://`和`https://`，但是请注意，这里还有很多其它可选的 scheme。

接下来是`authority`部分，它由用户名和可选密码组成，通常是完全可选的。

host 可以是 IP，(在我们的示例中仅为 IPv4)，也可以是主机字符串，如：`example.org`，后面跟一个可选的port，port 仅是个数字：如：`localhost:8080`。

在端口之后是 path。它是由`/`分割的字符串序列，如：`/some/important/path`。query 和 fragment 部分是可选的，它们表示 URL 的`?query=some-value&another=5`和`#anchor`部分。query 是字符串元组的可选列表，而 fragment 只是可选字符串(完整的 URL 示例是`https://some/important/?query=some-value&another=5#anchor`)。

如果你对这些类型中的生命周期(`'a`)感到困惑，请不用感到沮丧；它不会真的影响到我们写代码的方式。本质上，我们可以使用指向输入字符串各部分的指针，而不是为 URL 的每一部分分配新的字符串，只要输入的生命周期和我们 URI 结构一样长就可以了。

在开始解析之前，让我们实现`From`特征将合法的 scheme 转换成`Scheme`枚举：

```rust
impl From<&str> for Scheme {
    fn from(i: &str) -> Self {
        match i.to_lowercase().as_str() {
            "http://" => Scheme::HTTP,
            "https://" => Scheme::HTTPS,
            _ => unimplemented!("no other schemes supported"),
        }
    }
}
```

顺便说一句，让我们从顶部开始，开始解析 scheme。

## nom中的错误处理

在我们开始之前，先讨论一下 `nom` 中的错误处理。虽然我们不会面面俱到，但是至少会让调用者大致了解在解析的哪一步出了什么问题。

为了达到我们的目的，我们将使用`nom`中的`context`组合器。在`nom`中，一个解析器通常会返回如下类型：

```rust
type IResult<I, O, E = (I, ErrorKind)> = Result<(I, O), Err<E>>;
```

在本例中，我们将返回一个输入值(`&str` - 输入字符串)的元组类型。它包含仍需要解析的字符串，以及输出的值。当解析失败时，它也会返回一个错误。

标准的`IResult`只允许我们使用 nom 内置的错误类型，如果我们想要创建自定义的错误类型以及在这些错误中添加一些上下文呢？

`ParserError` 特征和 `VerboseError` 类型让我们可以构建自己的错误类型，并可以在已有的错误中添加上下文。在这个简单的例子中，我们将会在我们的解析错误类型中添加上下文。为了方便起见，让我们定义一个自己的结果类型。

```rust
type Res<T, U> = IResult<T, U, VerboseError<T>>;
```

除了它带有`VerboseError`之外，本质上是相同的。这意味着我们可以使用 nom 的上下文组合器，它允许我们在任何解析器中隐式地添加错误上下文。

nom 的官方文档包含这些选项，但是错误处理并不是最直观的方法。

为了看到它的实际效果，让我们为该 scheme 创建第一个解析器。

## 使用Rust写一个解析器

为了解析 URL 的scheme，我们想要匹配`http://`和`https://`，除此之外没有别的了。由于我们使用的是功能强大的解析器组合器库，因此我们不需要手动编写底层的解析器。`nom` 已经帮我们覆盖了。

[解析器组合器宏清单](https://github.com/fucking-translation/tutorial/Rust/nom/选择nom组合器.md)讲述了在某些用例中如何使用 nom 中的解析器组合器。

我们将会使用`tag_no_case`解析器和`alt`组合器来做基础的说明：“每个小写(输入)应该是`http://`或`https://`” 。在本教程中，我们将只使用常规函数，但请注意，nom 中的许多解析器和组合器也可以作为宏使用。

在 Rust 中使用 nom 如下所示：

```rust
fn scheme(input: &str) -> Res<&str, Scheme> {
    context(
        "scheme",
        alt((tag_no_case("HTTP://"), tag_no_case("HTTPS://"))),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}
```

如你所示：我们使用`context`组合器封装了实际的解析器并在其中添加了`scheme`上下文，因此，此处触发的任何错误都将在结果中标记为`scheme`。

一旦将解析器和组合器组装成了整个解析器，便使用输入字符串来调用它，这是我们唯一的输入参数。然后我们对结果进行`map` - 如上所述，它由剩余的输入和解析的输出组成，并通过实现前面提到的`.into()`特征将我们解析后的 scheme 转换成`Scheme`枚举。

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err as NomErr,
    };

    #[test]
    fn test_scheme() {
        assert_eq!(scheme("https://yay"), Ok(("yay", Scheme::HTTPS)));
        assert_eq!(scheme("http://yay"), Ok(("yay", Scheme::HTTP)));
        assert_eq!(
            scheme("bla://yay"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    ("bla://yay", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("bla://yay", VerboseErrorKind::Nom(ErrorKind::Alt)),
                    ("bla://yay", VerboseErrorKind::Context("scheme")),
                ]
            }))
        );
    }
}
```

如你所见，在成功的情况下，我们取回已解析`Scheme`枚举和剩余待解析的字符串(`yay`)。另外，如果有错误，我们将列举出已触发的错误以及定义的上下文列表(`scheme`)。

在本例中，两次`tag`调用都失败了，因此，`alt`组合器也失败了，因为它无法产生单个值。

那不是很难。在上面我们基本上只是解析了一个常量的字符串，让我们通过解析`authority`部分来尝试更高级的内容。

## 解析待授权的URL

如果我们还记得我们在之前的 URI 的结构，尤其是 authority 部分，我们会看到我们正在寻找一个完全可选的结构。如果它存在，则需要一个用户名和一个可选的密码。

这是我们使用的类型别名：

```rust
pub type Authority<'a> = (&'a str, Option<&'a str>);
```

我们该怎么办呢？在 URL 中，它看起来像：

[https://username:password@example.org](https://username:password@example.org)

`:password`是可选的，但是在任何情况下，它都会以`@`作为结尾，所以我们可以通过使用`terminated`解析器开始。这给了我们一个字符串，该字符串是通过终止另一个字符串得到的。

在`authority`部分中，我们看到`:`作为一个分隔符。根据文档，我们可以使用`separated_pair`组合器，它通过分割一个字符串给我们提供了两个值。但是我们如何处理实际的文本呢？这里有几种选项，一种是使用`alphanumeric1`解析器。它生成了一个至少包含一个字符的字母数字字符串。

为了简单起见，我们不必担心可以在 URL 的不同部分使用哪些字符。这与编写和构造解析器无关，只会使所有的内容变得更长且更不方便。出于我们的目的，我们假设 URL 的大部分都可以由字母数字组成，有时候还包含连字符和点 - 根据 [URL 标准](https://url.spec.whatwg.org/#url-code-points)，这当然是错误的。

让我们来看看组合后的`authority`解析器：

```rust
fn authority(input: &str) -> Res<&str, (&str, Option<&str>)> {
    context(
        "authority",
        terminated(
            separated_pair(alphanumeric1, opt(tag(":")), opt(alphanumeric1)),
            tag("@"),
        ),
    )(input)
}
```

我们通过运行一些测试用例来检验它是否工作：

```rust
#[test]
fn test_authority() {
    assert_eq!(
        authority("username:password@zupzup.org"),
        Ok(("zupzup.org", ("username", Some("password"))))
    );
    assert_eq!(
        authority("username@zupzup.org"),
        Ok(("zupzup.org", ("username", None)))
    );
    assert_eq!(
        authority("zupzup.org"),
        Err(NomErr::Error(VerboseError {
            errors: vec![
                (".org", VerboseErrorKind::Nom(ErrorKind::Tag)),
                ("zupzup.org", VerboseErrorKind::Context("authority")),
            ]
        }))
    );
    assert_eq!(
        authority(":zupzup.org"),
        Err(NomErr::Error(VerboseError {
            errors: vec![
                (
                    ":zupzup.org",
                    VerboseErrorKind::Nom(ErrorKind::AlphaNumeric)
                ),
                (":zupzup.org", VerboseErrorKind::Context("authority")),
            ]
        }))
    );
    assert_eq!(
        authority("username:passwordzupzup.org"),
        Err(NomErr::Error(VerboseError {
            errors: vec![
                (".org", VerboseErrorKind::Nom(ErrorKind::Tag)),
                (
                    "username:passwordzupzup.org",
                    VerboseErrorKind::Context("authority")
                ),
            ]
        }))
    );
    assert_eq!(
        authority("@zupzup.org"),
        Err(NomErr::Error(VerboseError {
            errors: vec![
                (
                    "@zupzup.org",
                    VerboseErrorKind::Nom(ErrorKind::AlphaNumeric)
                ),
                ("@zupzup.org", VerboseErrorKind::Context("authority")),
            ]
        }))
    )
}
```

看起来很不错！对于各种情况，我们都有与之对应的测试用例，缺少密码，缺少`@`以及其他几种错误的情况。

让我们继续来到 `host` 部分。

## Rust解析：主机，IP和端口

因为 host 部分可以包含 主机字符串或者 IP，这一步将会有点复杂。更糟的是，在结尾还有一个可选的`:port`。

为了尽可能保持简单，我们只支持 IPv4 的 IP。我们将从 host 开始。让我们看一下它的实现并逐行进行说明。

```rust
fn host(input: &str) -> Res<&str, Host> {
    context(
        "host",
        alt((
            tuple((many1(terminated(alphanumerichyphen1, tag("."))), alpha1)),
            tuple((many_m_n(1, 1, alphanumerichyphen1), take(0 as usize))),
        )),
    )(input)
    .map(|(next_input, mut res)| {
        if !res.1.is_empty() {
            res.0.push(res.1);
        }
        (next_input, Host::HOST(res.0.join(".")))
    })
}
```

首先你注意到这里有两个选项(`alt`)。在这两种情况下，都有一个元组，并包含了一个解析器链。

在第一种情况下，我们想要一个或多个(`many1`)字母数字字符串，包含一个连字符，被一个`.`终结并以顶级域名 (alpha1) 结尾。

`alphanumerichyphen1`解析器如下所示：

```rust
fn alphanumerichyphen1<T>(i: T) -> Res<T, T>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    i.split_at_position1_complete(
        |item| {
            let char_item = item.as_char();
            !(char_item == '-') && !char_item.is_alphanum()
        },
        ErrorKind::AlphaNumeric,
    )
}
```

这有点复杂，但基本上是 nom 中`alphanumeric1`解析器带有`-`的复制版本。我不知道它是否是最好的方式，但是它确实有用。

在任何情况下，主机部分都有第二个选项，它是一个字符串，如：`localhost`。

为什么我们要用将1和1传给`many_m_n`解析器这种看起来很无用的方式来表示呢？这里的问题是，在`alt`组合器中，所有的选项都必须返回相同的类型 - 在这里，它是一个字符串向量和另一个字符串的元组。

我们也在`map`函数中看到，如果元组的第二部分不为空(顶级域名)，则将其添加到元组的第一部分。最后，我们构建了一个 HOST 枚举，将字符串部分用一个`.`相连，并创建了一个原始的主机字符串。

让我们来看一些测试用例：

```rust
#[test]
fn test_host() {
    assert_eq!(
        host("localhost:8080"),
        Ok((":8080", Host::HOST("localhost".to_string())))
    );
    assert_eq!(
        host("example.org:8080"),
        Ok((":8080", Host::HOST("example.org".to_string())))
    );
    assert_eq!(
        host("some-subsite.example.org:8080"),
        Ok((":8080", Host::HOST("some-subsite.example.org".to_string())))
    );
    assert_eq!(
        host("example.123"),
        Ok((".123", Host::HOST("example".to_string())))
    );
    assert_eq!(
        host("$$$.com"),
        Err(NomErr::Error(VerboseError {
            errors: vec![
                ("$$$.com", VerboseErrorKind::Nom(ErrorKind::AlphaNumeric)),
                ("$$$.com", VerboseErrorKind::Nom(ErrorKind::ManyMN)),
                ("$$$.com", VerboseErrorKind::Nom(ErrorKind::Alt)),
                ("$$$.com", VerboseErrorKind::Context("host")),
            ]
        }))
    );
    assert_eq!(
        host(".com"),
        Err(NomErr::Error(VerboseError {
            errors: vec![
                (".com", VerboseErrorKind::Nom(ErrorKind::AlphaNumeric)),
                (".com", VerboseErrorKind::Nom(ErrorKind::ManyMN)),
                (".com", VerboseErrorKind::Nom(ErrorKind::Alt)),
                (".com", VerboseErrorKind::Context("host")),
            ]
        }))
    );
}
```

让我们来到 主机是 IP 的情况。首先，我们需要能够解析 IPv4 的 IP 中每一个的部分(如：127.0.0.1)：

```rust
fn ip_num(input: &str) -> Res<&str, u8> {
    context("ip number", n_to_m_digits(1, 3))(input).and_then(|(next_input, result)| {
        match result.parse::<u8>() {
            Ok(n) => Ok((next_input, n)),
            Err(_) => Err(NomErr::Error(VerboseError { errors: vec![] })),
        }
    })
}

fn n_to_m_digits<'a>(n: usize, m: usize) -> impl FnMut(&'a str) -> Res<&str, String> {
    move |input| {
        many_m_n(n, m, one_of("0123456789"))(input)
            .map(|(next_input, result)| (next_input, result.into_iter().collect()))
    }
}
```

为了获取每一个数字，我们尝试使用`n_to_m_digits`解析器来寻找一到三个连续的数字并将他们转换成 `u8`。

通过这种方式，我们可以查看如何将完整的 IP 解析成`u8`数组：

```rust
fn ip(input: &str) -> Res<&str, Host> {
    context(
        "ip",
        tuple((count(terminated(ip_num, tag(".")), 3), ip_num)),
    )(input)
    .map(|(next_input, res)| {
        let mut result: [u8; 4] = [0, 0, 0, 0];
        res.0
            .into_iter()
            .enumerate()
            .for_each(|(i, v)| result[i] = v);
        result[3] = res.1;
        (next_input, Host::IP(result))
    })
}
```

在这里，我们要查找的查好是3个后面跟`.`的`ip_num`，然后是另一个`ip_num`。在映射函数中，我们将这些独立的结果拼接，从而将`u8`数组转换成`Host::IP`枚举。

再一次，我们将写一些测试用例来确保它是正常工作的：

```rust
#[test]
fn test_ipv4() {
    assert_eq!(
        ip("192.168.0.1:8080"),
        Ok((":8080", Host::IP([192, 168, 0, 1])))
    );
    assert_eq!(ip("0.0.0.0:8080"), Ok((":8080", Host::IP([0, 0, 0, 0]))));
    assert_eq!(
        ip("1924.168.0.1:8080"),
        Err(NomErr::Error(VerboseError {
            errors: vec![
                ("4.168.0.1:8080", VerboseErrorKind::Nom(ErrorKind::Tag)),
                ("1924.168.0.1:8080", VerboseErrorKind::Nom(ErrorKind::Count)),
                ("1924.168.0.1:8080", VerboseErrorKind::Context("ip")),
            ]
        }))
    );
    assert_eq!(
        ip("192.168.0000.144:8080"),
        Err(NomErr::Error(VerboseError {
            errors: vec![
                ("0.144:8080", VerboseErrorKind::Nom(ErrorKind::Tag)),
                (
                    "192.168.0000.144:8080",
                    VerboseErrorKind::Nom(ErrorKind::Count)
                ),
                ("192.168.0000.144:8080", VerboseErrorKind::Context("ip")),
            ]
        }))
    );
    assert_eq!(
        ip("192.168.0.1444:8080"),
        Ok(("4:8080", Host::IP([192, 168, 0, 144])))
    );
    assert_eq!(
        ip("192.168.0:8080"),
        Err(NomErr::Error(VerboseError {
            errors: vec![
                (":8080", VerboseErrorKind::Nom(ErrorKind::Tag)),
                ("192.168.0:8080", VerboseErrorKind::Nom(ErrorKind::Count)),
                ("192.168.0:8080", VerboseErrorKind::Context("ip")),
            ]
        }))
    );
    assert_eq!(
        ip("999.168.0.0:8080"),
        Err(NomErr::Error(VerboseError {
            errors: vec![
                ("999.168.0.0:8080", VerboseErrorKind::Nom(ErrorKind::Count)),
                ("999.168.0.0:8080", VerboseErrorKind::Context("ip")),
            ]
        }))
    );
}
```

将它们放置在一起，我们需要另一个可以同时解析 IP 和 host 的解析器，并返回一个`Host`：

```rust
fn ip_or_host(input: &str) -> Res<&str, Host> {
    context("ip or host", alt((ip, host)))(input)
}
```

最后，让我们来解析端口(原文遗漏)：

```rust
fn port(input: &str) -> Res<&str, u16> {
    context(
        "port",
        tuple((
            tag(":"),
            n_to_m_digits(1, 5)
        )),
    )(input)
        .and_then(|(next_input, result)| {
            let port = result.1.parse::<u16>();
            match port {
                Ok(port) => Ok((next_input, port)),
                Err(e) => Err(NomErr::Error(VerboseError { errors: vec![ (input, VerboseErrorKind::Nom(ErrorKind::Digit))] }))
            }
        })
}
```

并使用一些测试用例保证它是可以正常工作的：

```rust
#[test]
fn test_port() {
    assert_eq!(port(":0"), Ok(("", 0u16)));
    assert_eq!(port(":65535"), Ok(("", 65535u16)));
    assert_eq!(
        port(":65536"),
        Err(NomErr::Error(VerboseError {
            errors: vec![
                (":65536", VerboseErrorKind::Nom(ErrorKind::Digit))
            ]
        })));
    assert_eq!(
        port(":a"),
        Err(NomErr::Error(VerboseError {
            errors: vec![
                ("a", VerboseErrorKind::Nom(ErrorKind::OneOf)),
                ("a", VerboseErrorKind::Nom(ErrorKind::ManyMN)),
                (":a", VerboseErrorKind::Context("port"))
            ]
        })));
}
```

还不错，一切正常！

## 使用Rust解析路径

下一步是解决路径问题。在此，我们再次假设该路径中的字符串只能包含带有连字符和点的字母数字字符串，并使用以下帮助程序进行解析：

```rust
fn url_code_points<T>(i: T) -> Res<T, T>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    i.split_at_position1_complete(
        |item| {
            let char_item = item.as_char();
            !(char_item == '-') && !char_item.is_alphanum() && !(char_item == '.')
            // ... actual ascii code points and url encoding...: https://infra.spec.whatwg.org/#ascii-code-point
        },
        ErrorKind::AlphaNumeric,
    )
}
```

为了解析`path`，我们希望可以将由`/`分隔的字符串解析成字符串向量：

```rust
fn path(input: &str) -> Res<&str, Vec<&str>> {
    context(
        "path",
        tuple((
            tag("/"),
            many0(terminated(url_code_points, tag("/"))),
            opt(url_code_points),
        )),
    )(input)
    .map(|(next_input, res)| {
        let mut path: Vec<&str> = res.1.iter().map(|p| p.to_owned()).collect();
        if let Some(last) = res.2 {
            path.push(last);
        }
        (next_input, path)
    })
}
```

我们总是由`/`开始。这已经是一个合法的路径了，但是我们仍然可以有`0`个或更多个(`many0`)由`/`分隔的字符串，后面跟一个最终的可选的字符串(如：`index.php`)。

在映射中，我们检查元组的第三部分(最后一部分)是否存在，如果存在，则将其添加到路径向量中。

让我们为路径也写一点测试用例：

```rust
#[test]
fn test_path() {
    assert_eq!(path("/a/b/c?d"), Ok(("?d", vec!["a", "b", "c"])));
    assert_eq!(path("/a/b/c/?d"), Ok(("?d", vec!["a", "b", "c"])));
    assert_eq!(path("/a/b-c-d/c/?d"), Ok(("?d", vec!["a", "b-c-d", "c"])));
    assert_eq!(path("/a/1234/c/?d"), Ok(("?d", vec!["a", "1234", "c"])));
    assert_eq!(
        path("/a/1234/c.txt?d"),
        Ok(("?d", vec!["a", "1234", "c.txt"]))
    );
}
```

看起来不错！我们获取到了路径中的不同部分以及剩余的字符串，并且它们都被添加到了字符串向量中了。

让我们通过解析 query 和 URL 部分的 fragment 来增强功能。

## 查询和片段

查询主要是由键值对组成：第一个键前面跟一个`?`，其余的查询由`&`进行分隔。再一次，我们将自己限制为有限的`url_code_points`。

```rust
fn query_params(input: &str) -> Res<&str, QueryParams> {
    context(
        "query params",
        tuple((
            tag("?"),
            url_code_points,
            tag("="),
            url_code_points,
            many0(tuple((
                tag("&"),
                url_code_points,
                tag("="),
                url_code_points,
            ))),
        )),
    )(input)
    .map(|(next_input, res)| {
        let mut qps = Vec::new();
        qps.push((res.1, res.3));
        for qp in res.4 {
            qps.push((qp.1, qp.3));
        }
        (next_input, qps)
    })
}
```

实际上这相当不错，因为解析器是非常直观(intuitive)且可读性的。我们解析`?`后面的第一个键值对的元组，使用`=`分隔，然后同样的操作执行`0`或多次，它们是以`&`而不是`?`开头。

然后，在映射中，我们简单的将所有的键值对放在向量中，然后就有了我们在文章的开头定义的结构。

```rust
pub type QueryParam<'a> = (&'a str, &'a str);
pub type QueryParams<'a> = Vec<QueryParam<'a>>;
```

这里有一组基础的测试用例：

```rust
#[test]
fn test_query_params() {
    assert_eq!(
        query_params("?bla=5&blub=val#yay"),
        Ok(("#yay", vec![("bla", "5"), ("blub", "val")]))
    );

    assert_eq!(
        query_params("?bla-blub=arr-arr#yay"),
        Ok(("#yay", vec![("bla-blub", "arr-arr"),]))
    );
}
```

最后一部分是 fragment，它其实就是一个`#`后面跟一个字符串：

```rust
fn fragment(input: &str) -> Res<&str, &str> {
    context("fragment", tuple((tag("#"), url_code_points)))(input)
        .map(|(next_input, res)| (next_input, res.1))
}
```

在介绍了所有这些复杂的解析器之后，为了达到良好的效果，让我们编写一些完整性检查测试：

```rust
#[test]
fn test_fragment() {
    assert_eq!(fragment("#bla"), Ok(("", "bla")));
    assert_eq!(fragment("#bla-blub"), Ok(("", "bla-blub")));
}
```

## 在Rust中使用nom解析:最终的测试

让我们将它们都放在最顶层的 URI 解析器函数中：

```rust
pub fn uri(input: &str) -> Res<&str, URI> {
    context(
        "uri",
        tuple((
            scheme,
            opt(authority),
            ip_or_host,
            opt(port),
            opt(path),
            opt(query_params),
            opt(fragment),
        )),
    )(input)
    .map(|(next_input, res)| {
        let (scheme, authority, host, port, path, query, fragment) = res;
        (
            next_input,
            URI {
                scheme,
                authority,
                host,
                port,
                path,
                query,
                fragment,
            },
        )
    })
}
```

我们有一个强制的(mandatory)`scheme`，后面跟一个可选的`authority`，然后再跟一个强制的`ip 或 host`。最后后面跟可选的`port`，`path`，`query 参数`，和一个`fragment`。

在映射中，剩下的唯一一件事就是将解析后的元素构成成我们的`URI`结构。

在这一点上，你可以看到整个结构的美观性和模块化。如果 uri 函数是你的起点，那么你可以从头到尾查看每个单独的解析器，以了解整个过程在做什么。

当然，我们也需要对`uri`解析器进行一些测试：

```rust
#[test]
fn test_uri() {
    assert_eq!(
        uri("https://www.zupzup.org/about/"),
        Ok((
            "",
            URI {
                scheme: Scheme::HTTPS,
                authority: None,
                host: Host::HOST("www.zupzup.org".to_string()),
                port: None,
                path: Some(vec!["about"]),
                query: None,
                fragment: None
            }
        ))
    );

    assert_eq!(
        uri("http://localhost"),
        Ok((
            "",
            URI {
                scheme: Scheme::HTTP,
                authority: None,
                host: Host::HOST("localhost".to_string()),
                port: None,
                path: None,
                query: None,
                fragment: None
            }
        ))
    );

    assert_eq!(
        uri("https://www.zupzup.org:443/about/?someVal=5#anchor"),
        Ok((
            "",
            URI {
                scheme: Scheme::HTTPS,
                authority: None,
                host: Host::HOST("www.zupzup.org".to_string()),
                port: Some(443),
                path: Some(vec!["about"]),
                query: Some(vec![("someVal", "5")]),
                fragment: Some("anchor")
            }
        ))
    );

    assert_eq!(
        uri("http://user:pw@127.0.0.1:8080"),
        Ok((
            "",
            URI {
                scheme: Scheme::HTTP,
                authority: Some(("user", Some("pw"))),
                host: Host::IP([127, 0, 0, 1]),
                port: Some(8080),
                path: None,
                query: None,
                fragment: None
            }
        ))
    );
}
```

它没问题！你可以在 [Github](https://github.com/zupzup/rust-nom-parsing)找到完整的代码。

## 结论

真是太好了！我希望本文能够使你对 Rust 中的解析器特别是解析器组合器感到兴奋。

`nom`库解析速度特别快，是很多生产级别的库和系统的基础。除此之外，它还提供了出色的 API 和文档。

Rust 生态系统还提供了更多的解析选项，如：[combine](https://github.com/Marwes/combine) 和 [pest](https://github.com/pest-parser/pest)。