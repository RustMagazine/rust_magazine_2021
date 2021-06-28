# 用 Rust 学习解析器组合器  

#### [原文](https://bodil.lol/parser-combinators/) / 译者: [iamazy](https://github.com/iamazy), [Matrixtang](https://github.com/MATRIXKOO)

</br>

本文向已是 Rust 程序员的人们传授解析器组合器的基础知识。它假定不涉及其他知识，并将解释与 Rust 没有直接关系的所有内容，以及为达到此目的使用 Rust 的一些令人意外的特性。如果你不了解 Rust，他不会教你 Rust 的内容，但是这样的话，同样也无法更好的教你解析器和组合器的知识。如果你想要学习 Rust 语言，我推荐  [Rust编程语言](https://doc.rust-lang.org/book/) 这本书。  

## Beginner's Mind

当每个程序员发现自己需要解析器时，它们的生活将变得有意义。  

初级程序员将会问：“什么是解析器？”。  
中级程序员将会说：“这很简单，我将写一串正则表达式”。  
高级程序员将会说：“退一步考虑问题，我知道`Lex`(词法分析器)和`Yacc`(语法解析器)”  

初级程序员的想法是正确的。

不是说正则表达式不好(但是请不要用正则表达式写复杂的解析器)。使用经过数千年 (millennia) 磨练至完美的解析器和词法分析生成器之类的强大工具并不是没有乐趣可言。但是从头开始一点一点学习解析器将更有趣。这两者只是对当前实际问题的抽象，如果你直接选择正则表达式或者解析器生成工具，你将失去这一份乐趣。在初学者眼中，正如人们说的：本来(解决这个问题)有很多种方法，但是在专家眼里，已经形成思维定视，只会选择一种他们最习惯的方式。
在本文中我们将从头开始学习如何构建解析器，基于被称为解析器组合器的函数式编程语言的通用技术。一旦你掌握了它们的基本概念，它们的优势将非常巨大，同时又非常接近第一原理。因为这里唯一的抽象是你将在基础的组合器之上构建你自己的抽象。所有这些，你必须先构建它们，才能使用它们。

## How To Work Through This Article

强烈建议你初始化一个 Rust 项目，并且在`src/lib.rs`中书写你阅读到的代码片段(你可以直接从页面上直接复制，但是最好还是自己手敲，因为这样会自动确保你完整阅读代码)。本文将按序介绍你需要的每段代码。请注意，有时会引入你先前编写功能的最新版本，这时，你需要将旧版本替换为新版本。

该代码是使用 2018 年语言版本的`rustc`的 1.34.0 版本编写的。你应该尽可能使用最新版本的编译器，只要你确保你使用的是 2018 的版本即可(检查`Cargo.toml`文件中是否包含`edition = "2018"`)。代码不需要任何的外部依赖。

如果要运行本文中的测试用例，请执行`cargo test`。

## The Xcruciating Markup Language

我们将用简化的XML格式写一个解析器，如下所示：

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

#### Exercises

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

现在，我们已经可以解析`<`，以及之后的标志符了，但是我们需要同时将它们进行解析，以便在这里可以取得进展。因此接下来将编写另一个解析器的构造函数，它将两个解析器作为输入并返回一个新的解析器，并按顺序解析它们。换言之，它是一个解析器组合器，因为它将两个解析器组合成一个新的解析器。让我们看看我们是否能够做到这一点。

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

在我们深入讨论之前，先介绍另一个组合器：`map`，它将使编写这两个解析器更加简单。

这个组合器有一个目的：改变结果的类型。例如，假设你有一个解析器返回`((), String)`，但是你希望能够将其返回值类型修改为`String`。

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

`Result`的`and_then`方法和`map`类似，不同之处在于`map`函数不会返回新值到`Result`内部，而是返回一个新的`Result`。上面的代码与先前使用`match`块的版本效果相同。稍后我们将回到`and_then`，但是现在，既然我们有一个干净简洁的`map`，我们可以真正实现`left`，`right`组合器。

## Left And Right

有了`pair`和`map`，我们可以非常简洁的编写`left`和`right`组合器：

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

我们使用`pair`组合器将两个解析器组合成一个解析器，然后使用`map`组合器选择其结果元组中我们想要保留的部分。

现在我们需要为元素标签的前两部分重写测试，使它更简洁一些，在此过程中，我们将获得了一些重要的新解析器组合器功能。

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

甚至更糟的是，我们需要处理一个或多个空格，因为`<element      attribute="value"/>`也是一个合法的语法，即使它的空格很多。所以这似乎是我们考虑是否可以编写一个组合器来表示一个或多个解析器想法的好时机。

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

我们拥有这个解析器，所以我们不能将其作为参数传递两次，编译器会试图对你大喊，因为你试图移动一个已经移动的值。那么我们可以让组合器代替引用吗？不，事实证明，我们还遇到另一整套借用检查器的问题 - 我们目前不会试图解决这些问题。并且因为这些解析器是函数，所以它们不会实现`Clone`，该 trait 本来可以帮我们节省一整天的时间，所以我们被困在一个约束中，我们不能在组合器中轻易地复用我们的解析器。

不过，这不是一个大问题。它只是说明我们不能使用组合器来表达`one_or_more`，但事实证明，这两个通常是你需要的唯一组合器，它们往往会重用解析器，而且，如果你想变得非常花哨，除了解析器，你还可以编写一个带有`RangeBound`的组合器，并在一个范围内对其进行重复：`zero_or_more`使用`range(0..)`，`one_or_more`使用`range(1..)`，`five_or_six`使用`range(5..=6)`，依此类推。

不过，让我们把它留给读者作为练习。现在我们只需使用`zero_or_more`和`one_or_more`即可。

另一个练习可能是找到解决这些所有权问题的方法 - 也许可以通过将解析器包装在`Rc`中使其支持克隆？