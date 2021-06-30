# 用 Rust 学习解析器组合子 (combinator)

[原文](https://bodil.lol/parser-combinators/)

> 译者: [iamazy](https://github.com/iamazy), [Matrixtang](https://github.com/MATRIXKOO)

本文向已是 Rust 程序员的人们传授解析器组合子的基础知识。它假定不涉及其他知识，并将解释与 Rust 没有直接关系的所有内容，以及为达到此目的使用 Rust 的一些令人意外的特性。如果你不了解 Rust，他不会教你 Rust 的内容，但是这样的话，同样也无法更好的教你解析器和组合子的知识。如果你想要学习 Rust 语言，我推荐 [Rust 编程语言](https://doc.rust-lang.org/book/) 这本书。

## Beginner's Mind

当每个程序员发现自己需要解析器时，它们的生活将变得有意义。

初级程序员将会问：“什么是解析器？”。  
中级程序员将会说：“这很简单，我将写一串正则表达式”。  
高级程序员将会说：“退一步考虑问题，我知道`Lex`(词法分析器)和`Yacc`(语法解析器)”

初级程序员的想法是正确的。

不是说正则表达式不好(但是请不要用正则表达式写复杂的解析器)。使用经过 `数千年 (millennia)` 磨练至完美的解析器和词法分析生成器之类的强大工具并不是没有乐趣可言。但是从头开始一点一点学习解析器将更有趣。这两者只是对当前实际问题的抽象，如果你直接选择正则表达式或者解析器生成工具，你将失去这一份乐趣。在初学者眼中，正如人们说的：本来(解决这个问题)有很多种方法，但是在专家眼里，已经形成思维定视，只会选择一种他们最习惯的方式。
在本文中我们将从头开始学习如何构建解析器，基于被称为解析器组合子的函数式编程语言的通用技术。一旦你掌握了它们的基本概念，它们的优势将非常巨大，同时又非常接近第一原理。因为这里唯一的抽象是你将在基础的组合子之上构建你自己的抽象。所有这些，你必须先构建它们，才能使用它们。

## How To Work Through This Article

强烈建议你初始化一个 Rust 项目，并且在`src/lib.rs`中书写你阅读到的代码片段(你可以直接从页面上直接复制，但是最好还是自己手敲，因为这样会自动确保你完整阅读代码)。本文将按序介绍你需要的每段代码。请注意，有时会引入你先前编写功能的最新版本，这时，你需要将旧版本替换为新版本。

该代码是使用 2018 年语言版本的`rustc`的 1.34.0 版本编写的。你应该尽可能使用最新版本的编译器，只要你确保你使用的是 2018 的版本即可(检查`Cargo.toml`文件中是否包含`edition = "2018"`)。代码不需要任何的外部依赖。

如果要运行本文中的测试用例，请执行`cargo test`。

## The Xcruciating Markup Language

我们将用简化的 XML 格式写一个解析器，如下所示：

```xml
<parent-element>
  <single-element attribute="value" />
</parent-element>
```

XML 元素使用`<`符号开头和一个由字母组成的标志符开头，后面可以根任意数量的字母，数字和`-`。后面跟一些空格，以及一些属性对的可选列表：前面定义一个标识符，后面跟一个`=`符号和一个双引号字符串，最后以`/>`结尾来表示一个没有子节点的单个元素。或者用`>`表示后面跟了一串子节点，最终是一个以`</`开头的结束标签，后跟一个必须与开始标识匹配的标志符，然后以`>`结束。

这就是我们将要支持的特性。没有命名空间，没有文本节点，其余的都没有，并且没有模式校验。我们甚至都不会自寻烦恼，去实现字符串的转义引号(`\"`) - 它们由第一个双引号开始，到下一个双引号结束，就是这样。如果你想在实际字符串中使用双引号，你可以将这个不合理的要求转移到其他地方。

我们将用下面的结构体来解析这些元素。

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
struct Element {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Element>,
}
```

没有花哨的类型，仅仅用字符串表示名称(那是每个标签开始的标志符)，用字符串元组表示属性(标志符和对应的值)，还有一个看起来和父元素完全一样的子元素列表。

(如果你正在输入，请加上那一系列的`derive`，你将在后面用到它们)。

## Defining The Parser

好了，是时候编写解析器了。

解析是从数据流中派生结构的过程。解析器就是要弄清楚该结构的工具。

在我们即将探索的知识中，解析器最简单的一种形式就是一个函数，该函数接受一些输入并返回解析后的输出以及输入的其余部分，或者是一个错误信息：“我无法解析该输入”。

事实证明，简而言之，更复杂的解析器看起来也是如此。如果你要获取友好的错误提示，你可能会使输入，输出和错误的含义复杂化，但是解析器的特征依然如此：消费输入并产生解析后的数据以及输入的其余部分，或者会让你知道无法将输入解析成输出。

让我们将其(解析器)写成函数的形式。

```rust
Fn(Input) -> Result<(Input, Output), Error>
```

更具体的说，就我们而言，我们希望可以填充这些类型，就像下面代码块所示。因为我们要做的是将一个字符串转化为`Element`结构体，并且在这方面我们不想陷入错综复杂的错误提示中，所以当我们不能够解析输入的字符串时，只需返回一段提示错误的字符串即可。

```rust
Fn(&str) -> Result<(&str, Element), &str>
```

我们使用字符串切片(`&str`)，是因为他是指向字符串的有效指针。我们可以进一步对其进行切片，通过切掉已解析的部分来消耗输入，并将余下的输入和结果一起返回。

可能使用`&[u8]`类型作为输入(一串字节数组，如果将输入的字节限制在`ASCII`范围内，则`u8`和`char`相对应)会看起来更加干净，尤其是字符串切片(`&str`)的行为和大多数切片的行为有所不同。你不能够通过一个数字来检索它们，你必须使用一个分片`input[0..1]`。另一方面，它们对于解析字符串有很多有用的函数，而字节切片没有。

事实上，我们通常都需要依赖这些函数而不是像`input[0]`这样对他进行索引。因为 Rust 的字符串是`UTF-8`格式的，这些索引并不总是对应于单个字符，所以对于我们来说，让标准库为我们处理这些问题更好。

## Our First Parser

让我们来一起写一个解析器，只需要关注字符串中的第一个字符并判断它是否是字母`a`

```rust
fn the_letter_a(input: &str) -> Result<(&str, ()), &str> {
  match input.chars().next() {
      Some('a') => Ok((&input['a'.len_utf8()..], ())),
      _ => Err(input),
  }
}
```

首先，我们来看一下输入和输入和输出的类型：和上面讨论的一样，我们让字符串切片作为输入，并返回携带`(&str, ())`元组或者错误类型`&str`的`Result`。`(&str, ())`元组比较有趣：正如我们所说的，我们想要返回一个包含下一次待解析的输入和输出。`&str`是下一次输入，解析返回的结果仅仅是一个单元类型`()`。因为如果解析成功，只可能有一种结果(我们找到了字母`a`)，但是这里我们并不需要返回字母`a`，我们只需要告诉调用者我们成功发现了字母`a`即可。

然后，让我们看下解析器本身的代码。我们从提取输入的第一个字符开始：`input.chars().next()`。依靠标准库来避免带来的 Unicode 编码问题并不是在开玩笑 - 我们要求它为字符串的字符提供一个`chars()`迭代器，然后从中取出第一项。该项是封装在`Option`中的`char`类型，因此是`Option<char>`，如果它的值为`None`则表示我们尝试在空字符串中提取一个`char`。

更糟的是，`char`可能并不是你想的那样是 Unicode 字符。它很可能就是 Unicode 所说的“[字素簇](http://www.unicode.org/glossary/#grapheme_cluster)”，它可以由几个`char`组成，实际上代表“[标量值](http://www.unicode.org/glossary/#unicode_scalar_value)”，大约比字素簇低两级。但是这种方式太疯狂了，出于我们(讲解)的目的，我们可能根本不会看到除 ASCII 之外的字符集，所以这种情况我们不做讨论。

我们模式匹配上了`Some('a')`，这是我们正在寻找的特定结果，如果匹配上，则返回结果`Ok((&input['a'.len_utf8()..], ()))`。也就是说，我们在字符串切片中删除了刚刚解析的`'a'`并返回剩余部分以及我们的解析结果(一个空的`()`)。考虑到 Unicode 怪物，我们在切片之前向标准库询问了`'a'`在 UTF-8 编码中的长度，但是，请永远不要假设 Unicode 怪物。

如果我们匹配到其他的`Some(char)`或者`None`，我们将返回一个错误。你可能还记得，我们的错误类型目前还是解析失败时的字符串切片，也就是作为`input`传入的字符串切片。它不以`'a'`开头，因此这是我们的错误。但这不是一个大的错误，但是至少比“其他地方出了问题”要好一些。

虽然我们不需要使用此解析器解析 XML，但是我们要做的第一件事就是查找`<`，因此我们可能需要一些相似的东西。我们还需要专门解析`>`，`/`，`=`，因此我们可以创建一个函数为我们想要的字符构建一个解析器？

## A Parser Builder

我们想象一下：编写一个函数，可以为任意长度的静态字符串(而不仅仅是单个字符)生成一个解析器。这种方式甚至更容易，因为一个字符串切片已经是一个合法的 UTF-8 字符串切片，我们不需要再去考虑 Unicode 怪物。

```rust
fn match_literal(expected: &'static str)
    -> impl Fn(&str) -> Result<(&str, ()), &str>
{
    move |input| match input.get(0..expected.len()) {
        Some(next) if next == expected => {
            Ok((&input[expected.len()..], ()))
        }
        _ => Err(input),
    }
}
```

这个函数看起来有点不同。

我们先看下类型。我们的函数看起来不再像是一个解析器，它将我们期望的字符串作为参数，并返回一个看似解析器的东西。它是一个返回函数的函数 - 换言之，是一个高阶函数。基本上，我们在编写一个函数，该函数可以构建一个类似之前的`the_letter_a`函数。

因此，我们不是在函数体中完成工作，而是返回一个闭包用来处理这些事情，该闭包与我们之前解析器的类型签名相匹配。

模式匹配看起来是一样的，除了我们不能直接匹配字符串字面量，因为我们不知道它具体是什么，所以我们使用匹配条件`if next == expected`代替。否则它和以前完全一样，只是在闭包的主体内。

## Testing Our Parser

我们来为它编写测试用例，确保这个函数没有问题。

```rust
#[test]
fn literal_parser() {
    let parse_joe = match_literal("Hello Joe!");
    assert_eq!(
        Ok(("", ())),
        parse_joe("Hello Joe!")
    );
    assert_eq!(
        Ok((" Hello Robert!", ())),
        parse_joe("Hello Joe! Hello Robert!")
    );
    assert_eq!(
        Err("Hello Mike!"),
        parse_joe("Hello Mike!")
    );
}
```

首先，我们构建了一个解析器：`match_literal("Hello Joe!")`。它应该消耗字符串`"Hello Joe!"`并返回字符串的剩余部分，或者失败并返回整个字符串。

在第一种情况下，我们为它提供了它期望的确切字符串，并看到它返回了一个空字符串以及`()`值，这表示“我们解析了期望的字符串，并且不需要将它真的返回”。

在第二种情况下，我们提供了字符串`"Hello Joe! Hello Robert!"`，并且我们看到它确实消耗了字符串`"Hello Joe!"`并且返回了输入字符串的剩余部分：`"Hello Robert!"`(包括空格)。

在第三种情况下，我们提供了一个不正确的输入`"Hello Mike!"`，并注意到该函数拒绝了这个输入并返回了一个错误。不是说 Mike 作为一般规则是不正确的，而是它不是此解析器所需要的。

## Exercises

- 你能在标准库中找到一个关于`str`类型的方法，让你编写`match_literal()`时不必做麻烦的`get`索引吗？

## A Parser For Something Less Specific

我们可以继续解析`<`，`>`，`=`以及`</`和`/>`。我们几乎已经完成了。

`<`之后需要识别的部分是元素名称。我们不能通过简单的字符串比较做到这一点。但是我们可以使用正则表达式。

但是我们需要克制一下自己，这将是一个很容易在简单代码中复用的正则表达式，我们不需要为此引入`regex`库。让我们尝试一下是否可以只使用 Rust 标准库来编写自己的解析器。

回顾元素名称标志符的规则：首位是字母，后跟零个或多个字母，数字或`-`。

```rust
fn identifier(input: &str) -> Result<(&str, String), &str> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
        Some(next) if next.is_alphabetic() => matched.push(next),
        _ => return Err(input),
    }

    while let Some(next) = chars.next() {
        if next.is_alphanumeric() || next == '-' {
            matched.push(next);
        } else {
            break;
        }
    }

    let next_index = matched.len();
    Ok((&input[next_index..], matched))
}
```

与往常一样，我们首先查看类型。这次我们不是编写函数来构建解析器，只需要编写解析器本身，就像第一次一样。这里显著的区别是，我们返回元组中的`String`和剩余的输入，而不是`()`的结果类型。这个`String`将包含我们刚刚解析的标志符。

考虑到这一点，我们首先创建一个空字符串并将称之为`matched`。它是我们将要返回的结果值。我们在`input`中获取一个字符的迭代器，我们将对其进行拆解。

第一步是查看前面是否有字母。我们从迭代器中取出第一个字符并检查它是否是一个字母：`next.is_alphabetic()`。Rust 标准库当然是帮我们来处理 Unicode 的 - 这将匹配任何字母表中的字母，而不仅仅是 ASCII。如果它是一个字母，我们将它放入我们的`matched`字符串中，如果不是，显然我们不是在查看元素标志符，所以我们立即返回一个错误。

第二步，我们不断从迭代器中取出字符，将它们放入我们正在构建的字符串中，直到我们找到一个既不是`is_alphanumeric()`(就像是`is_alphabetic()`，它只是不能匹配字母表中的任何数字)也不是破折号`'-'`的字符。

当我们第一次看到不符合这些条件的字符时，意味着我们已经完成了解析，所以我们跳出循环并返回我们构建的`String`，记住去掉我们在`input`中消耗的字符。如果迭代器用完了字符，意味着我们到达了`input`的末尾。

值得注意的是，当我们看到不是字母数字 (alphanumeric) 或破折号(`-`)的内容时，我们不会返回错误。一旦我们匹配了第一个字母，我们就已经有足够的东西来创建一个有效的标志符。而且在我们解析完标志符以后，输入字符串中有更多待解析的字符是非常正常的，所以我们只是停止解析并返回我们的结果，只有当我们找不到第一个字母时，我们才真正的返回错误，因为在这种情况下，肯定没有标志符。

还记得我们要将 XML 文档解析成的`Element`结构吗？

```rust
struct Element {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Element>,
}
```

我们实际上只是完成了解析器的第一部分，`name`字段。我们解析器返回的字符串就在那里。它也是解析每个`attribute`第一部分需要的解析器。

让我们对其进行测试。

```rust
#[test]
fn identifier_parser() {
    assert_eq!(
        Ok(("", "i-am-an-identifier".to_string())),
        identifier("i-am-an-identifier")
    );
    assert_eq!(
        Ok((" entirely an identifier", "not".to_string())),
        identifier("not entirely an identifier")
    );
    assert_eq!(
        Err("!not at all an identifier"),
        identifier("!not at all an identifier")
    );
}
```

我们可以在第一种情况中看到，字符串`"i-am-an-identifier"`被完整解析，只留下空的字符串。在第二种情况中，解析器返回`"not"`作为标志符，并且字符串的剩余部分作为剩余的输入返回。在第三种情况中，解析彻底 (outright) 失败，因为它找到的第一个字符不是字母。

## Combinators

现在，我们已经可以解析`<`，以及之后的标志符了，但是我们需要同时将它们进行解析，以便在这里可以取得进展。因此接下来将编写另一个解析器的构造函数，它将两个解析器作为输入并返回一个新的解析器，并按顺序解析它们。换言之，它是一个解析器组合子，因为它将两个解析器组合成一个新的解析器。让我们看看我们是否能够做到这一点。

```rust
fn pair<P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Fn(&str) -> Result<(&str, (R1, R2)), &str>
where
    P1: Fn(&str) -> Result<(&str, R1), &str>,
    P2: Fn(&str) -> Result<(&str, R2), &str>,
{
    move |input| match parser1(input) {
        Ok((next_input, result1)) => match parser2(next_input) {
            Ok((final_input, result2)) => Ok((final_input, (result1, result2))),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}
```

这里有点复杂，但是你知道该怎么做：从查看类型开始。

首先，我们有四种类型变量：`P1`，`P2`，`R1`以及`R2`。这是 Parser1，Parser2，Result1，Result2 的缩写。`P1`，`P2`是函数，你会注意到它们遵循解析器函数的既定模式 (established pattern)：就像返回值，它们将`&str`作为输入并返回一个`Result`类型，该`Result`类型可能是一个包含剩余输入以及解析结果的二元组，或者是一个错误。

但是查看每个函数的结果类型：`P1`如果解析成功，将会产生`R1`，`P2`同理会产生`R2`。最终解析器的结果 - 从我们的函数返回的结果 - 是`(R1, R2)`。因此该解析器的工作是先在输入上运行解析器`P1`，然后在`P1`返回的剩余输入上运行`P2`，如果这两个解析器都执行成功，则我们将两个解析器返回的结果组合进元组`(R1, R2)`。

查看代码，我们看到这也正是它所做得。我们首先在输入上运行第一个解析器，然后是第二个解析器，接着将两个结果组合成一个元组并返回。如果这些解析器中的任何一个执行失败，我们会立即返回它给出的错误。

这样，我们应该能够结合之前的两个解析器`match_literal`和`identifier`来实际解析 XML 标签的第一部分(`<my-first-element/>`)。让我们编写测试用例看它是否正确。

```rust
#[test]
fn pair_combinator() {
    let tag_opener = pair(match_literal("<"), identifier);
    assert_eq!(
        Ok(("/>", ((), "my-first-element".to_string()))),
        tag_opener("<my-first-element/>")
    );
    assert_eq!(Err("oops"), tag_opener("oops"));
    assert_eq!(Err("!oops"), tag_opener("<!oops"));
}
```

它看起来成功了！但是看下结果类型：`((), String)`。很明显，我们只关心右边的值 - `String`。这种情况相当普遍 - 我们的一些解析器只匹配输入中的模式但不产生值，因此可以安全的忽略他们的输出。为了适应这种模式，我们将使用`pair`组合子编写另外两个组合子`left`，它丢弃第一个解析器的结果，只返回第二个，以及它的相反数`right`，我们想在上面的测试中使用`right`而不是`pair` - 它可以丢弃二元组左边的`()`只保留右边的`String`。

## Enter The Functor

在我们深入讨论之前，先介绍另一个组合子：`map`，它将使编写这两个解析器更加简单。

这个组合子有一个目的：改变结果的类型。例如，假设你有一个解析器返回`((), String)`，但是你希望能够将其返回值类型修改为`String`。

为了做到这点，我们传入一个函数，该函数知道如何将原始类型转换成新的类型。在我们的示例中，该函数十分简单：`|(_left, right)| right`。它的一般格式就像`Fn(A) -> B`，其中`A`是解析器的原始类型，`B`是期望的新类型。

```rust
fn map<P, F, A, B>(parser: P, map_fn: F) -> impl Fn(&str) -> Result<(&str, B), &str>
where
    P: Fn(&str) -> Result<(&str, A), &str>,
    F: Fn(A) -> B,
{
    move |input| match parser(input) {
        Ok((next_input, result)) => Ok((next_input, map_fn(result))),
        Err(err) => Err(err),
    }
}
```

这些类型说明什么？`P`是我们的解析器。它在成功时将返回`A`。`F`是我们将用于将`P`映射到我们的返回值中的函数，它看起来和`P`很像，但是它的返回值类型是`B`而不是`A`。

在代码中，我们运行`parser(input)`，如果执行成功，我们将拿到`result`并在其上运行函数`map_fn(result)`，然后将`A`转换成`B`。

实际上，我们可以稍微放纵 (indulge) 一下自己并缩短一下这个函数，因为`map`实际上是处理`Result`的一种常见模式：

```rust
fn map<P, F, A, B>(parser: P, map_fn: F) -> impl Fn(&str) -> Result<(&str, B), &str>
where
    P: Fn(&str) -> Result<(&str, A), &str>,
    F: Fn(A) -> B,
{
    move |input|
        parser(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
}
```

这种模式在 Haskell 以及范畴论 (category theory) 中被称为”函子 (functor)“。如果你在其中有一个类型为`A`的东西，并且有一个可用的`map`函数，你可以将某个函数从`A`传递到`B`以将其转换成相同类型的东西，但是在其中使用类型`B`，这就是一个函子。你可以在 Rust 中很多地方见到它，如在 [Option](https://doc.rust-lang.org/std/option/enum.Option.html#method.map) 中，[Result](https://doc.rust-lang.org/std/result/enum.Result.html#method.map) 中，[Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map) 甚至在 [Future](https://docs.rs/futures/0.1.26/futures/future/trait.Future.html#method.map) 中，但它没有被明确命名。因为在 Rust 的类型系统中，你不能真正的将函子表示为泛化的东西，因为它缺乏更高级的类型，但那是另一回事了，所以我们只需关注这些函子，寻找`map`函数即可。

## Time For A Trait

你可能已经注意到，我们不断重复解析器类型签名的格式：`Fn(&str) -> Result<(&str, Output), &str>`。你可能已经厌倦了像我写的那样完整的阅读它，所以我认为是时候引入一个 trait，让这些更具有可读性，还可以为解析器添加一些扩展性。

但是首先，我们先为一直使用的返回值类型创建一个类型别名：

```rust
type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;
```

现在，我们可以不用一直输入那串冗长的类型，可以直接使用`ParseResult<String>`。我们可以在这里添加一个生命周期，因为类型的声明需要它，但是大多数时候 Rust 编译器可以帮你推断。通常，可以尝试不添加生命周期，并观察 rustc 是否报错，如果报错则添加生命周期。

在此例中，`'a`特指`input`的生命周期。

现在，对于 trait，我们也需要在这里声明生命周期，并且当你使用该 trait 时，生命周期通常也是必须的。这是一个额外的输入，但它击败了以前的版本。

```rust
trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;
}
```

它目前只有一个方法：`parse()`方法，它看起来似曾相识：它和我们之前编写的解析器函数基本相同。

为了让它更加简单，我们实际上可以为任何匹配解析器签名的函数实现这个 trait。

```rust
impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(&'a str) -> ParseResult<Output>,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self(input)
    }
}
```

通过这种方式，我们不仅可以传递迄今为止已传递的相同函数作为解析器，并完全实现`Parser` trait，还可以使用其他类型作为解析器。

但是，更重要的是，它把我们从必须一直输入这些函数签名的噩梦中解救了出来。让我们重写`map`函数，并看它是如何工作的。

```rust
fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |input|
        parser.parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
}
```

这里需要特别注意的一件事是：我们现在必须执行`parser.parse(input)`，而不是将解析器作为函数调用，因为我们不知道类型`P`是否是函数，我们只知道它实现了`Parser`，并且我们必须使用`Parser`提供的接口。但是函数体看起来完全一样，类型看起来也更加简洁。只是有一个新的额外噪音：生命周期`'a`，但总的来说是一个很大的改进。

如果我们以相同的方式重新编写`pair`函数，会看起来更加简洁：

```rust
fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| match parser1.parse(input) {
        Ok((next_input, result1)) => match parser2.parse(next_input) {
            Ok((final_input, result2)) => Ok((final_input, (result1, result2))),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}
```

和之前一样，这里唯一的改变就是类型签名更加简洁，并且需要执行`parser.parse(input)`而不是`parse(input)`。

实际上，我们还可以简化`pair`函数的主题，通过使用`map`：

```rust
fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| {
        parser1.parse(input).and_then(|(next_input, result1)| {
            parser2.parse(next_input)
                .map(|(last_input, result2)| (last_input, (result1, result2)))
        })
    }
}
```

`Result`的`and_then`方法和`map`类似，不同之处在于`map`函数不会返回新值到`Result`内部，而是返回一个新的`Result`。上面的代码与先前使用`match`块的版本效果相同。稍后我们将回到`and_then`，但是现在，既然我们有一个干净简洁的`map`，我们可以真正实现`left`，`right`组合子。

## Left And Right

有了`pair`和`map`，我们可以非常简洁的编写`left`和`right`组合子：

```rust
fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R1>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(left, _right)| left)
}

fn right<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(_left, right)| right)
}
```

我们使用`pair`组合子将两个解析器组合成一个解析器，然后使用`map`组合子选择其结果元组中我们想要保留的部分。

现在我们需要为元素标签的前两部分重写测试，使它更简洁一些，在此过程中，我们将获得了一些重要的新解析器组合子功能。

不过，我们必须先更新我们的两个解析器以使用`Parser`和`ParseResult`。`match_literal`是更复杂的一个：

```rust
fn match_literal<'a>(expected: &'static str) -> impl Parser<'a, ()> {
    move |input: &'a str| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], ())),
        _ => Err(input),
    }
}
```

除了改变返回值类型之外，我们还必须确保闭包的输入类型是`&'a str`，否则 rustc 会报错。

对于`identifier`，只需更改返回值类型即可，编译器会自动为你推断生命周期。

```rust
fn identifier(input: &str) -> ParseResult<String> {}
```

接下来进行测试，可以看见在结果中没有那个笨拙 (ungainly) 的`()`。

```rust
#[test]
fn right_combinator() {
    let tag_opener = right(match_literal("<"), identifier);
    assert_eq!(
        Ok(("/>", "my-first-element".to_string())),
        tag_opener.parse("<my-first-element/>")
    );
    assert_eq!(Err("oops"), tag_opener.parse("oops"));
    assert_eq!(Err("!oops"), tag_opener.parse("<!oops"));
}
```

## One Or More

让我们继续解析元素标签。我们已经解析了`<`和标志符。然后呢？应该是我们的第一个属性对。

实际上这些属性对是可选的。我们需要找到一种方式来处理这种可选的标记。

等等，实际上在我们得到第一个可选属性对之前我们必须处理一些事情：空格。

在元素名称的末尾以及第一个属性名称(如果存在的话)的开头之间，存在一个空格。我们需要处理这个空格。

甚至更糟的是，我们需要处理一个或多个空格，因为`<element attribute="value"/>`也是一个合法的语法，即使它的空格很多。所以这似乎是我们考虑是否可以编写一个组合子来表示一个或多个解析器想法的好时机。

我们在`identifier`解析器中已经处理过这个问题，但是一切都是在那里手动完成的。毫不奇怪，总体思路的代码并没有什么不同。

```rust
fn one_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        if let Ok((next_input, first_item)) = parser.parse(input) {
            input = next_input;
            result.push(first_item);
        } else {
            return Err(input);
        }

        while let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}
```

首先，我们构建的解析器返回值类型是`A`，组合解析器的返回值类型是`Vec<A>` - 任意数量的`A`。

这个代码看起来确实和`identifier`很像。首先，我们解析第一个元素，如果不存在，则返回一个错误。然后我们尽可能多的解析元素，直到解析失败，此时我们返回包含收集元素的向量。

将上述代码修改为支持解析零次或多次的解析器也很简单，只需删除前部分`if`判断即可：

```rust
fn zero_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        while let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}
```

让我们编写测试用例以验证这两个解析器的功能符合预期。

```rust
#[test]
fn one_or_more_combinator() {
    let parser = one_or_more(match_literal("ha"));
    assert_eq!(Ok(("", vec![(), (), ()])), parser.parse("hahaha"));
    assert_eq!(Err("ahah"), parser.parse("ahah"));
    assert_eq!(Err(""), parser.parse(""));
}

#[test]
fn zero_or_more_combinator() {
    let parser = zero_or_more(match_literal("ha"));
    assert_eq!(Ok(("", vec![(), (), ()])), parser.parse("hahaha"));
    assert_eq!(Ok(("ahah", vec![])), parser.parse("ahah"));
    assert_eq!(Ok(("", vec![])), parser.parse(""));
}
```

请注意两者的不同之处：对于`one_or_more`，解析空字符串时会返回错误，因为它需要找到至少一个满足其子解析器的输入，但是对于`zero_or_more`，一个空字符串只是意味着没有满足条件的输入，它并不是一个错误。

鉴于此，可以考虑将这两个方法进行泛化，因为一个是另一个的副本，只是移除了一部分功能。用`zero_or_more`来表示`one_or_more`可能会很诱人 (tempt)。

```rust
fn one_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    map(pair(parser, zero_or_more(parser)), |(head, mut tail)| {
        tail.insert(0, head);
        tail
    })
}
```

在这里，我们遇到了 Rust 的一个问题，我甚至不是指`Vec`没有`cons`方法的问题，但我知道每个阅读那段代码的 Lisp 程序员都在思考这个问题：那就是所有权。

我们拥有这个解析器，所以我们不能将其作为参数传递两次，编译器会试图对你大喊，因为你试图移动一个已经移动的值。那么我们可以让组合子代替引用吗？不，事实证明，我们还遇到另一整套借用检查器的问题 - 我们目前不会试图解决这些问题。并且因为这些解析器是函数，所以它们不会实现`Clone`，该 trait 本来可以帮我们节省一整天的时间，所以我们被困在一个约束中，我们不能在组合子中轻易地复用我们的解析器。

不过，这不是一个大问题。它只是说明我们不能使用组合子来表达`one_or_more`，但事实证明，这两个通常是你需要的唯一组合子，它们往往会重用解析器，而且，如果你想变得非常花哨，除了解析器，你还可以编写一个带有`RangeBound`的组合子，并在一个范围内对其进行重复：`zero_or_more`使用`range(0..)`，`one_or_more`使用`range(1..)`，`five_or_six`使用`range(5..=6)`，依此类推。

不过，让我们把它留给读者作为练习。现在我们只需使用`zero_or_more`和`one_or_more`即可。

另一个练习可能是找到解决这些所有权问题的方法 - 也许可以通过将解析器包装在`Rc`中使其支持克隆？

## A Predicate Combinator [#](https://bodil.lol/parser-combinators/#a-predicate-combinator)

我们现在有了需要用 `one_or_more` 解析空白的 `xml` 块，并用 `zero_or_more` 解析属性对。 其实，稍等一下。我们其实不想先解析空格 _然后_ 解析属性 (`attributes`)。如果你考虑一下，如果没有属性，空格是可选的，我们可能会遇到直接的 `>` 或 `/>`。但是如果有一个属性，那么*必须*是空格。幸运的是，每个属性之间也必须有空格，如果有多个，所以我们在这里真正看到的是一系列*零或更多*出现的*一个或多个*空格项，然后是属性。

我们首先需要一个解析单个空白项的解析器。我们可以选择下列三种方式之一。

一，我们可以愚蠢地使用我们的 `match_literal` 解析器和一个只包含一个空格的字符串。为什么这么傻？因为空格也是换行符、制表符和大量呈现为空格的奇怪 Unicode 字符。我们将不得不再次依赖 Rust 的标准库，当然 `char` 有一个 `is_whitespace` 方法，就像它有 `is_alphabetic` 和 `is_alphanumeric` 一样。

二，我们可以使用 `is_whitespace` Predicate 写出另外一个解析器，它消耗任意数量的空白字符，就像我们之前编写的 `identifier` 一样。

三，我们可以写的更加巧妙。编写一个解析器 `any_char`，它返回一个单一的 `char`，只要输入中还剩下一个，以及一个组合子 `pred`，它接受一个解析器和一个谓词 (Predicate）函数，并像这样将两者结合起来：`pred (any_char, |c| c.is_whitespace())`。这有一个额外的好处，通过它，编写我们将需要的最终的解析器变得非常容易：属性值的引用字符串。 `any_char` 解析器写起来很简单，但我们得记得注意那些 UTF-8 问题。

```rust
fn any_char(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some(next) => Ok((&input[next.len_utf8()..], next)),
        _ => Err(input),
    }
}
```

对 `经验丰富` 的我们来说， `pred` 组合子也并没有给我们太多惊喜 。 我们调用解析器，然后我们在解析器成功的情况下调用我们的谓词函数，并且只有当它返回 true 时，我们才真正返回 `success`，否则我们将返回与解析器失败一致的错误。

```rust
fn pred<'a, P, A, F>(parser: P, predicate: F) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
    F: Fn(&A) -> bool,
{
    move |input| {
        if let Ok((next_input, value)) = parser.parse(input) {
            if predicate(&value) {
                return Ok((next_input, value));
            }
        }
        Err(input)
    }
}
```

写一个快速测试以确保一切正常：

```rust
#[test]
fn predicate_combinator() {
    let parser = pred(any_char, |c| *c == 'o');
    assert_eq!(Ok(("mg", 'o')), parser.parse("omg"));
    assert_eq!(Err("lol"), parser.parse("lol"));
}
```

有了这两个组件，我们可以用一个快速的单行代码编写我们的 `whitespace_char` 解析器：

```rust
fn whitespace_char<'a>() -> impl Parser<'a, char> {
    pred(any_char, |c| c.is_whitespace())
}
```

并且，现在我们有了`whitespace_char`，我们也可以用它实现我们之前的想法，_一个或多个空白_，以及它的姊妹想法，_零个或多个空白_。 让我们专注于几个简单的地方，并分别称它们为`space1`和`space0`。

```rust
fn space1<'a>() -> impl Parser<'a, Vec<char>> {
    one_or_more(whitespace_char())
}

fn space0<'a>() -> impl Parser<'a, Vec<char>> {
    zero_or_more(whitespace_char())
}
```

## Quoted Strings

完成所有这些组件后，我们现在终于可以解析这些属性了吗？ 当然啦，我们只需要确保我们拥有所有这些属性组件的单独的解析器。 我们已经为属性名称提供了 `identifier`（尽管使用 `any_char` 和 `pred` 加上我们的 `*_or_more` 组合子来重写它是很诱人的）。 `=` 只是 `match_literal("=")`。 不过，我们只有一个带引号的字符串解析器，所以让我们把它组合起来。 幸运地是，我们已经拥有了完成它所需的所有组合子。

```rust
fn quoted_string<'a>() -> impl Parser<'a, String> {
    map(
        right(
            match_literal("\""),
            left(
                zero_or_more(pred(any_char, |c| *c != '"')),
                match_literal("\""),
            ),
        ),
        |chars| chars.into_iter().collect(),
    )
}
```

组合子的嵌套在这一点上变得有点烦人，但我们暂时不会重构所有代码来修复它，而是专注于这里发生的事情。

最外面的组合子是一个 `map`，因为前面提到的烦人的嵌套，从这开始理解代码是很让人困惑的，所以让我们试着找出它真正开始的地方：第一个引号字符。在`map`里面，有一个`right`，而`right`的第一部分就是我们要找的：`match_literal("\"")`。那是我们的引号的开始 。

`right` 的第二部分是字符串的其余部分。在 `left` 里面，我们很快注意到那个 `left` 的 _right_ 参数，我们一直忽视的那个: 是另一个 `match_literal("\"")` - 引号的结束。所以左手部分是我们用引号包裹的字符串。

我们在这里利用新的 `pred` 和 `any_char` 来获得一个接受*除另一个引号之外的任何内容*的解析器，并将其放入 `zero_or_more` 中，因此我们所说的实现如下：

- 一个引号

- 后跟零个或多个*不是*另一个引号的内容

- 接着是另一个引号


  并且，在 `right` 和 `left` 之间，我们丢弃结果值中的引号并取回引用的字符串。

  但是等等，这不是一个字符串。还记得 `zero_or_more` 返回什么吗？内部解析器的返回类型 A 的 `Vec<A>`。对于`any_char`来说，就是`char`。那么，我们得到的不是字符串而是`Vec<char>`。这就是 `map` 的用武之地：我们使用它把 `Vec<char>` 转换为 `String`，因为你可以从 `Iterator<Item = char>` 构建一个 `String`，所以我们可以调用 `vec_of_chars.into_iter().collect()`，并且由于类型推断的强大功能，我们有了 `String`。

  在我们继续之前，让我们编写一个快速测试以确保一切正常，因为如果我们需要这么多词来解释它，这可能不是我们作为程序员对自己有信心的样子。

```rust
#[test]
fn quoted_string_parser() {
    assert_eq!(
        Ok(("", "Hello Joe!".to_string())),
        quoted_string().parse("\"Hello Joe!\"")
    );
}
```

所以, 终于, 我们可以解析点属性了.

## At Last, Parsing Attributes

我们现在可以解析空格、标识符、`=` 符号和带引号的字符串。 最后，这就是我们解析属性所需的全部内容。

首先，让我们为一个属性对编写一个解析器。 我们将把它们存储为 `Vec<(String, String)>`，你可能还记得，所以我们需要一个解析器来处理 `(String, String)` 元组,来提供给我们值得信赖的 `zero_or_more` 组合子。 让我们看看我们是否可以写出一个。

```rust
fn attribute_pair<'a>() -> impl Parser<'a, (String, String)> {
    pair(identifier, right(match_literal("="), quoted_string()))
}
```

易如反掌！ 总结一下：我们已经有一个方便的组合子来解析一个值的元组，`pair`，所以我们将它与 `identifier` 解析器一起使用，产生一个 `String` 和一个带有 `=` 符号的 `right`， 它包括了我们不想保留的值，以及我们新的 `quoted_string` 解析器，它给了我们另一个 `String`。

现在，让我们将其与 `zero_or_more` 结合起来构建该 `vector` - 但不要忘记它们之间的空白。

```rust
fn attributes<'a>() -> impl Parser<'a, Vec<(String, String)>> {
    zero_or_more(right(space1(), attribute_pair()))
}
```

零次或多次出现以下内容：一个或多个空白字符，然后是一个属性对。 我们使用`right`来丢弃空格并保留属性对。

让我们测试一下。

```rust
#[test]
fn attribute_parser() {
    assert_eq!(
        Ok((
            "",
            vec![
                ("one".to_string(), "1".to_string()),
                ("two".to_string(), "2".to_string())
            ]
        )),
        attributes().parse(" one=\"1\" two=\"2\"")
    );
}
```

测试通过! 起飞!

实际上，并没有，在叙述者点上, 我的 `rustc` 抱怨我的类型变得非常复杂，我需要增加最大允许的类型大小才能继续。你也可能遇到这种情况，如果发生了, 你需要知道如何处理它。 幸好，在这些情况下，`rustc` 通常会给出很好的建议，所以当它告诉你将 `#![type_length_limit = "...some big number..."]` 添加到文件顶部时，就按照它说的去做。 实际上，只需将其设为 `#![type_length_limit = "16777216"]`，这将使我们进一步深入复杂类型的平流层。 全力以赴，我们现在是宇航员！

## So Close Now

这个时候，事情似乎即将开始合为一体，这让人松了一口气，因为我们的类型正在快速接近 NP 完整性。 我们只需要处理两个版本的元素标签：单个元素和带有子元素的父元素，但是我们非常有信心，一旦我们有了这些，解析子元素将只是 `zero_or_more` 的问题， 对吧？

所以让我们先从单一元素开始，把孩子的问题推迟一点。 或者，更好的是，让我们首先为两者的所有共同点编写一个解析器：开头的 `<`、元素名称和属性。 让我们看看我们是否可以从几个组合子中得到 `(String, Vec<(String, String)>)`类型。

```rust
fn element_start<'a>() -> impl Parser<'a, (String, Vec<(String, String)>)> {
    right(match_literal("<"), pair(identifier, attributes()))
}
```

有了它，我们可以快速地给它打上标签，为单个元素创建一个解析器。

```rust
fn single_element<'a>() -> impl Parser<'a, Element> {
    map(
        left(element_start(), match_literal("/>")),
        |(name, attributes)| Element {
            name,
            attributes,
            children: vec![],
        },
    )
}
```

万岁，感觉我们的目标已经触手可及了——我们现在实际上正在构建一个 `Element`！

让我们来测试一下这个现代科技的奇迹吧。

```rust
#[test]
fn single_element_parser() {
    assert_eq!(
        Ok((
            "",
            Element {
                name: "div".to_string(),
                attributes: vec![("class".to_string(), "float".to_string())],
                children: vec![]
            }
        )),
        single_element().parse("<div class=\"float\"/>")
    );
}
```

...我想我们才刚刚突破大气层。

`single_element` 的返回类型非常复杂，编译器会花费很长时间，直到遇到我们之前给它的非常大的类型大小限制 (`#![type_length_limit = ""]`)，要求更大的类型。 很明显我们不能再忽视这个问题，因为它是一个相当简单的解析器和(应该只需要)几分钟的编译时间——对于成品来说甚至可能是几个小时——似乎有点不合理。

在继续之前，你最好在我们修复问题时注释掉这两个函数和测试......

## To Infinity And Beyond

如果你曾经尝试过在 Rust 中编写递归类型，你可能已经知道我们的小问题的解决方案。

递归类型的一个非常简单的例子是单向链表。 原则上，可以将其表示为这样的枚举：

```rust
enum List<A> {
    Cons(A, List<A>),
    Nil,
}
```

`rustc` 会非常明智地反对你的递归类型 `List<A>` ,因为它具有无限大小，因为在每个 `List::<A>::Cons` 内部是另一个 `List<A>`，这意味着它也是一个 `List<A>`......直到无穷大。就 `rustc` 而言，我们要求一个无限列表，我们要求它能够*分配*一个无限列表。

在许多语言中，无限列表原则上对于类型系统来说不是问题，实际上对于 Rust 也不是问题。问题是在 Rust 中，如前所述，我们需要能够*分配*它，或者更确切地说，我们需要能够在我们构造它时预先确定类型的 _大小_，以及当类型是无限的，这意味着大小也必须是无限的。

解决方案是使用一点间接性。我们的`List::Cons` 不是`A` 的一个元素和`A` 的另一个*list*，而是我们使它成为`A` 的一个元素和一个指向`A` 列表的*指针*。我们知道指针的大小，不管它指向什么都是一样的，所以我们的 List::Cons 现在有一个固定的和可预测的大小，无论列表的大小如何。在 Rust 中，将一个拥有的东西变成指向堆上拥有的东西的指针的方法是用 `box` 包裹它。

```rust
enum List<A> {
    Cons(A, Box<List<A>>),
    Nil,
}
```

`Box` 的另一个有趣的特性是它里面的类型可以是抽象的。 这意味着我们可以让类型检查器处理一个非常简洁的 `Box<dyn Parser<'a, A>>`，而不是我们现在非常复杂的解析器函数类型。

听起来不错。 有什么缺点？ 好吧，我们可能会因为必须遵循该指针而失去一两个指令周期，也可能是编译器失去了一些优化解析器的机会。 但回想一下 Knuth 关于过早优化(1)的告诫：它会没事的。 你完全可以负担得起这些周期。 来这里是为了解 解析器组合子，而不是了解手写的超专业化 [SIMD 解析器](https://github.com/lemire/simdjson)（尽管它们本身就很令人兴奋）。 因此，除了迄今为止我们一直在使用的裸函数之外，让我们继续为 _boxed_ 解析器函数实现 `Parser`。

> 译者注: 1: 过早优化是万恶之源

```rust
struct BoxedParser<'a, Output> {
    parser: Box<dyn Parser<'a, Output> + 'a>,
}

impl<'a, Output> BoxedParser<'a, Output> {
    fn new<P>(parser: P) -> Self
    where
        P: Parser<'a, Output> + 'a,
    {
        BoxedParser {
            parser: Box::new(parser),
        }
    }
}

impl<'a, Output> Parser<'a, Output> for BoxedParser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self.parser.parse(input)
    }
}
```

出于礼节，我们创建了一个新类型 `BoxedParser`来保存我们的 `box`。 要从任何其他类型的解析器（包括另一个`BoxedParser`，即使那毫无意义）创建一个新的`BoxedParser`，我们提供了一个函数`BoxedParser::new(parser)`，它只会将该解析器放入 我们的新类型中的一个 `Box`。 最后，我们为它实现了 `Parser`，这样它就可以作为解析器互换使用。

这使我们能够将解析器函数放入 `Box`，并且 `BoxedParser` 将与函数一样用作 `Parser`。 现在，正如前面提到的，这意味着将解析器移动到堆中，并且必须取消引用一个指针才能找到它，这可能会花费我们*几个宝贵的纳秒*，所以我们实际上可能想要推迟使用 `box`。 将一些更常用的组合子放到 `box` 就足够了。

## **An Opportunity Presents Itself**

但是，稍等片刻，这为我们提供了解决另一个变得麻烦的问题的机会。

还记得我们写的最后几个解析器吗？ 因为我们的组合子是独立的函数，当我们嵌套大量的组合子时，我们的代码开始变得有点不可读。 回想一下我们的 `quoted_string` 解析器：

```rust
fn quoted_string<'a>() -> impl Parser<'a, String> {
    map(
        right(
            match_literal("\""),
            left(
                zero_or_more(pred(any_char, |c| *c != '"')),
                match_literal("\""),
            ),
        ),
        |chars| chars.into_iter().collect(),
    )
}
```

如果我们可以在解析器上使用这些组合子方法而不是独立函数，它会有更好的可读性。 如果我们可以将组合子声明为 `Parser` trait 上的方法会怎样？

问题是，如果我们这样做，我们就失去了依赖 `impl Trait` 作为返回类型的能力，因为 `impl Trait` 不允许出现在 trait 声明中。

……但现在我们有了`BoxedParser`。 我们不能声明一个返回 `impl Parser<'a, A>` 的 trait ，但我们肯定*可以*声明一个返回 `BoxedParser<'a, A>` 的 trait 。

最好的部分是我们甚至可以使用默认实现声明这些，这样我们就不必为实现 `Parser` 的每个类型重新实现每个组合子。

让我们用 `map` 来试试，通过扩展我们的 `Parser` trait 如下：

```rust
trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;

    fn map<F, NewOutput>(self, map_fn: F) -> BoxedParser<'a, NewOutput>
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        F: Fn(Output) -> NewOutput + 'a,
    {
        BoxedParser::new(map(self, map_fn))
    }
}
```

啊这，好多`'a'`，唉，它们都是必要的。 幸运的是，我们仍然可以不变地重用旧的组合子函数——而且，我们还得到一个额外的好处，不仅可以获得更好的语法来应用它们，我们还通过自动 `box` 来摆脱爆炸性的 `impl Trait` 类型。 现在我们可以稍微改进我们的 `quoted_string` 解析器：

```rust
fn quoted_string<'a>() -> impl Parser<'a, String> {
    right(
        match_literal("\""),
        left(
            zero_or_more(pred(any_char, |c| *c != '"')),
            match_literal("\""),
        ),
    )
    .map(|chars| chars.into_iter().collect())
}
```

乍一看现在更明显，正在对 `right()` 的结果调用 `.map()`。

我们也可以给 `pair`、`left` 和 `right` 相同的处理，但是对于这三个，我认为当它们是函数时读起来更容易，因为它们反映了 `pair` 的输出结构 类型。 如果你不同意，完全可以像我们对 `map` 所做的那样将它们添加到 trait 中，并且非常欢迎你继续尝试将其作为练习。

不过，另一个主要候选人是 `pred`。 让我们将它的定义添加到 `Parser` trait 中：

```rust
fn pred<F>(self, pred_fn: F) -> BoxedParser<'a, Output>
where
    Self: Sized + 'a,
    Output: 'a,
    F: Fn(&Output) -> bool + 'a,
{
    BoxedParser::new(pred(self, pred_fn))
}
```

这下我们就可以用 `pred` 重写 `quoted_string`：

```rust
zero_or_more(any_char.pred(|c| *c != '"')),
```

我认为这读起来更好一些，我认为我们也会保留 `zero_or_more` 原样 - 它读起来就像应用了以下谓词的零个或多个 `any_char`，这对我来说听起来很半。 当然，如果你愿意这么写，你也可以继续将 `zero_or_more` 和 `one_or_more` 移到 trait 中。

除了重写`quoted_string`，我们还要修正`single_element`中的`map`：

```rust
fn single_element<'a>() -> impl Parser<'a, Element> {
    left(element_start(), match_literal("/>")).map(|(name, attributes)| Element {
        name,
        attributes,
        children: vec![],
    })
}
```

让我们试着取消对 `element_start` 和我们之前注释掉的测试的注释，看看情况是否变得更好。 开始编译并尝试运行测试......

……而且，是的，现在编译时间恢复正常了。 甚至可以继续删除文件顶部的字体大小设置。

这只是通过 `box` 两个 `map` 和一个 `pred` - *而且*我们从中得到了更好的语法！

## Having Children

现在让我们为父元素的开始标记编写解析器。 它几乎与`single_element` 相同，只是它以`>` 结尾而不是`/>`。 后面还有零个或多个子元素和一个结束标记，但首先我们需要解析实际的开始标记，所以让我们完成这个函数。

```rust
fn open_element<'a>() -> impl Parser<'a, Element> {
    left(element_start(), match_literal(">")).map(|(name, attributes)| Element {
        name,
        attributes,
        children: vec![],
    })
}
```

现在，我们如何得到这些子元素？ 它们将是单个元素或父元素本身，并且它们有零个或多个，所以我们有我们可信赖的 `zero_or_more` 组合子，但是我们提供什么？ 我们还没有编写过的一件事是多重解析器：解析*或者*单个元素*或*父元素的东西。

为了完成这个特性，我们需要一个组合子，它按顺序尝试两个解析器：如果第一个解析器成功，我们就完成了，我们返回它的结果，就是这样。 如果失败，我们不会返回错误，而是在*相同的输入*上尝试第二个解析器。 如果成功，很好，如果没有，我们也会返回错误，因为这意味着我们的两个解析器都失败了，这是一个整体性失败。

```rust
fn either<'a, P1, P2, A>(parser1: P1, parser2: P2) -> impl Parser<'a, A>
where
    P1: Parser<'a, A>,
    P2: Parser<'a, A>,
{
    move |input| match parser1.parse(input) {
        ok @ Ok(_) => ok,
        Err(_) => parser2.parse(input),
    }
}
```

这使得我们可以声明一个解析器`element`，它匹配单个元素或父元素（现在，我们只使用`open_element` 来表示它，一旦我们有了`element`，我们将处理 `element`）。

```rust
fn element<'a>() -> impl Parser<'a, Element> {
    either(single_element(), open_element())
}
```

现在让我们为结束标记添加一个解析器。 它具有必须匹配开始标签的特性，这意味着解析器必须知道开始标签的名称是什么。 但这就是函数参数的用途，对吧？

```rust
fn close_element<'a>(expected_name: String) -> impl Parser<'a, String> {
    right(match_literal("</"), left(identifier, match_literal(">")))
        .pred(move |name| name == &expected_name)
}
```

事实证明，那个 `pred` 组合子真的很有用，不是吗？

现在，让我们为完整的父元素解析器、子元素和所有元素组合起来：

```rust
fn parent_element<'a>() -> impl Parser<'a, Element> {
    pair(
        open_element(),
        left(zero_or_more(element()), close_element(…oops)),
    )
}
```

通过使用 `and_then`，我们现在可以通过使用该函数构建正确版本的 `close_element` 来获得正确的结果。

```rust
fn parent_element<'a>() -> impl Parser<'a, Element> {
    open_element().and_then(|el| {
        left(zero_or_more(element()), close_element(el.name.clone())).map(move |children| {
            let mut el = el.clone();
            el.children = children;
            el
        })
    })
}
```

现在看起来有点复杂，因为 `and_then` 必须在 `open_element()` 中进行，在那里我们找到进入 `close_element` 的地方。这意味着 `open_element` 之后的解析器的其余部分都必须在 `and_then` 闭包内构造。此外，因为该闭包现在是来自 `open_element` 的 `Element` 结果的唯一接收者，我们返回的解析器也必须向前传递该信息。

我们在生成的解析器上 `map`的内部闭包具有对外部闭包中的 `Element`（`el`）的引用。我们必须使用 `clone()` ，因为一个 `Fn` 中，只有对它的引用。我们获取内部解析器的结果（我们的 `Vec<Element>` 子元素）并将其添加到我们克隆的 `Element` 中，然后将其作为最终结果返回。

我们现在需要做的就是返回到我们的 `element` 解析器并确保我们将 `open_element` 更改为 `parent_element`，这样它就会解析整个元素结构，而不是只是它的开头，我相信我们已经完成了！

## Word Or Do I Have To?

还记得我们讨论过 `map` 模式如何在 Planet Haskell 上被称为“函子”吗？

`and_then` 模式是你在 Rust 中经常看到的另一种模式，通常与 `map` 位于相同的位置。它在 `Iterator` 上被称为 [`flat_map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map)，但它的模式与其他模式相同。

它的花名是`monad`。如果你有一个东西 `Thing<A>`，并且你有一个可用的 `and_then` 函数，它可以将一个函数从 `A` 传递给 `Thing<B>`，这样现在你就有了一个新的 `Thing<B>` 来替代原来的，这就是一个 monad。

该函数可能会立即被调用，比如当你有一个 `Option<A>` 时，我们已经知道它是一个 `Some(A)` 还是一个 `None`，我们直接应用该函数，如果它是一个 `Some(A)`，那么就会给我们一个`Some(B)`。

它也可能被称为 `惰性 (lazy)`。例如，如果有一个仍在等待解析的 `Future<A>`，它不会立即调用该函数来创建一个 `Future<B>`，而是创建一个新的 `Future<B>`它包含 `Future<A>` 和函数，然后等待 `Future<A>` 完成。当它这样做时，它会使用 `Future<A>` 的结果调用函数，这样就搞定了 [1](https://bodil.lol/parser-combinators/#footnote_1)(He isn't really your uncle.)，得到你的 `Future< B>` 。换句话说，在`Future` 的情况下，可以将传递给`and_then` 的函数视为*回调函数*，因为它会在完成时使用原始 future 的结果进行调用。它还比这更有趣，因为它返回一个 _new_ `Future`，它可能已经或可能没有被解决，所以它是一种将 future 连在一起的方法。

然而，与函子一样，Rust 的类型系统目前不能表达 `monad`，所以让我们只需注意这种模式被称为 `monad`，而且令人失望的是，这和网上的搜到的意思并不一样，它与墨西哥卷饼毫无关系。

## Whitespace, Redux

只有最后一件事。

我们现在应该有一个能够解析一些 XML 的解析器，但是他并不能很好的接受空格。 标签之间应该允许任意空格，这样我们就可以自由地在标签之间插入换行符等（原则上，标识符和文字之间应该允许空格，比如`< div />`，但这里跳过它）。 在这一点上，我们应该能够毫不费力地为此组合一个快速组合子。

```rust
fn whitespace_wrap<'a, P, A>(parser: P) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
{
    right(space0(), left(parser, space0()))
}
```

如果我们将 `element` 包裹在里面，它将忽略 `element` 周围的所有前面和后面空格，这意味着我们可以随意使用尽可能多的换行符和尽可能多的缩进。

```rust
fn element<'a>() -> impl Parser<'a, Element> {
    whitespace_wrap(either(single_element(), parent_element()))
}
```

## We're Finally There! 

我想我们终于做到了！ 让我们写个测试庆祝一下！

```RUST
#[test]
fn xml_parser() {
    let doc = r#"
        <top label="Top">
            <semi-bottom label="Bottom"/>
            <middle>
                <bottom label="Another bottom"/>
            </middle>
        </top>"#;
    let parsed_doc = Element {
        name: "top".to_string(),
        attributes: vec![("label".to_string(), "Top".to_string())],
        children: vec![
            Element {
                name: "semi-bottom".to_string(),
                attributes: vec![("label".to_string(), "Bottom".to_string())],
                children: vec![],
            },
            Element {
                name: "middle".to_string(),
                attributes: vec![],
                children: vec![Element {
                    name: "bottom".to_string(),
                    attributes: vec![("label".to_string(), "Another bottom".to_string())],
                    children: vec![],
                }],
            },
        ],
    };
    assert_eq!(Ok(("", parsed_doc)), element().parse(doc));
}

```

下面这个测试会因为有未闭合 tag 而解析失败：

```rust
#[test]
fn mismatched_closing_tag() {
    let doc = r#"
        <top>
            <bottom/>
        </middle>"#;
    assert_eq!(Err("</middle>"), element().parse(doc));
}
```

好消息是它返回不匹配的结束标记作为错误。 坏消息是它实际上并没有*说*问题是不匹配的结束标签，只是*错误在哪里*。 总比没有好，但是，老实说，随着错误消息的出现，它仍然很糟糕。 但是想让它能成功的找到错至少得再写一篇同样长的文章。

让我们关注好消息：我们使用解析器组合子从头开始编写了一个解析器！ 我们知道解析器既构成函子又构成单子，因此你现在可以在令人生畏的范畴论知识聚会上给人们留下深刻印象了[2](https://bodil.lol/parser-combinators/#footnote_2)。（Please don't be that person at parties. 别真的这么干）

最重要的是，我们现在知道解析器组合子是如何从头开始工作的。 现在没有人能阻止我们！

## Victory Puppies

![img](https://bodil.lol/parser-combinators/many-puppies.gif)

## Further Resources

首先，我对用严格的 `rust`术语 向你解释 monad 感到内疚，而且我知道如果我不向你指出 [他的开创性论文](https://homepages.com)，Phil Wadler 会对我非常不满。 其中详细介绍了更多令人兴奋的细节——包括它们与解析器组合子的关系。

本文中的想法与 [`pom`](https://crates.io/crates/pom) 解析器组合子库背后的想法极为相似，如果这让你想在同一个解析器组合子中使用风格，我强烈推荐它。

Rust 解析器组合子的最新技术仍然是 [`nom`](https://crates.io/crates/nom)，以至于前面提到的 `pom` 显然是派生的名称（而且没有比这更好的称赞了），但它采用了与我们今天在这里构建的方法截然不同的方法。

另一个流行的 Rust 解析器组合库是 [`combine`](https://crates.io/crates/combine)，它可能也值得一看。

Haskell 的开创性解析器组合库是 [Parsec](http://hackage.haskell.org/package/parsec)。

最后，我对解析器组合子的第一次认识归功于 Graham Hutton 所著的 [_Programming in Haskell_](http://www.cs.nott.ac.uk/~pszgmh/pih.html)，这是一本很棒的书，不仅容易读而且还会教给你 Haskell 的积极副作用。

## Licence

This work by [Bodil Stokke](https://bodil.lol/) is licensed under a [Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International License](http://creativecommons.org/licenses/by-nc-sa/4.0/).

本文由 Bodil Stokke 撰写，基于署名-非商业性使用-相同方式共享 4.0 国际 (CC BY-NC-SA 4.0)协议。
