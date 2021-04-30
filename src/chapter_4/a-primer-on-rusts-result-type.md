g# 【译】Rust 的 Result 类型基础

>* A Primer on Rust’s Result Type 译文
>* 原文链接：https://medium.com/@JoeKreydt/a-primer-on-rusts-result-type-66363cf18e6a
>* 原文作者：[Joe Kreydt](https://medium.com/@JoeKreydt?)
>* 译文来自：[RustMagazine 2021 期刊](https://github.com/RustMagazine/rust_magazine_2021)
>* 译者：[suhanyujie](https://www.github.com/suhanyujie)
>* 标签：Rust, Result
>* tips：水平有限，翻译不当之处，还请指正，谢谢！

* ![](https://miro.medium.com/max/2099/1*AoZOz1AJS15yyB3TLUn93A.jpeg)

`Result` 类型是 Rust 中处理错误的常用方法类型，它比较灵活；应该是非常灵活！

对于那些正在学 Rust 的人来讲，Result 可能不太直观，你可以通过阅读它的标准库[文档](https://doc.rust-lang.org/std/result/)来了解如何使用是个不错的方法。如果你想迫切的学会它，也是可以的，但如果你只是用它处理错误或者使用某个返回 Result 类型的函数（很多人都这样做），你可能体会不到它的妙处。

为了节省大家的时间，我打算使用英语来解释 Rust 的 Result 类型。

# Result 是什么？
## 参考[Rust 权威指南](https://doc.rust-lang.org/1.30.0/book/first-edition/error-handling.html)

“Result 表达的是错误的可能性。通常错误是用来解释某种任务执行失败的原因。”

![](https://miro.medium.com/max/400/1*g1A-DkLZ6dPjOKTo4kzrGg.gif)

## 用朴素的英语解释

Result 是一个函数返回的类型，它可以是 Ok，也可以是 Err。如果是 Ok，则表示函数按照预期执行完成。如果是 Err，则该函数出现了错误。

# Result 用来做什么？

## 根据 Rust 权威指南

Result 类型是对计算过程中可能出现的结果的表示方式。按照惯例，如果一个结果是预期的 `Ok`，那么另一个结果则是意料之外的，即 `Err`。

## 请再直观点
函数返回了值。这些值具有特定的数据类型。函数可以返回 Result 类型的结果。Result 类型根据函数是否按预期执行而变化。然后，程序员可以编写一些代码，如果函数按预期执行则返回 A，如果遇到异常，则返回 B。

# 不处理 Result，则产生异常

```
error[E0308]: mismatched types
  --> main.rs:20:26
   |
20 |     let my_number: f64 = my_string.trim().parse(); //.unwrap();
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^ expected f64, found enum `std::result::Result`
   |
   = note: expected type `f64`
              found type `std::result::Result<_, _>`
error: aborting due to previous error
For more information about this error, try `rustc --explain E0308`.
compiler exit status 1
```

报错信息中关键的部分是，“expected f64, found enum.”，类似的场景中，可能还会有：

    - “expected u32, found enum”
    - “expected String, found enum”
    - “expected [insert type here], found enum”

**如果你得到一个类似上面的错误，那是因为需要你处理函数返回的 Result 类型数据**

# 类型为 Error 的 Result 的程序

```rust
use std::io::{stdin, self, Write};
fn main(){
    let mut my_string = String::new();
    print!(“Enter a number: “);
    io::stdout().flush().unwrap();
    stdin().read_line(&mut my_string)
        .expect(“Did not enter a correct string”);
    let my_number: f64 = my_string.trim().parse();
    println!(“Yay! You entered a number. It was {:?}”, my_num);
}
```

在这个程序中，它提示用户输入一个数字。然后将输入作为字符串读入并存储下来。我们想要的是一个数值类型，不是 String，所以我们需要使用 _parse()_ 函数将其转换为一个 64 位浮点数（f64）。

如果用户输入的是一个数字，那么 _parse()_ 函数将其转换为 f64 没什么大问题。但我们仍然会得到一个错误。

发生错误是因为 _parse()_ 函数不只是将 String 转换为数字并返回。相反，它接受字符串，将其转换为数字，然后返回 Result 类型。Result 类型需要被解包才能得到我们需要的数值。

## 用 Unwrap() 或 Expect() 修复错误

转换后的数字可以可以通过在 _parse()_ 后面附加调用 _unwrap()_ 函数将数字从 Result 中“解包”出来，类似于这样： 

```rust
let my_number: f64 = my_string.trim().parse().unwrap();
```

_unwrap()_ 函数可以看出 Result 中类型，可能是 _Ok_，也可能是 _Err_。如果 Result 中包裹的类型是 _Ok_，那么 _unwrap()_ 则返回它的值。如果 Result 中的类型是 _Err_，_unwrap()_ 则会让程序崩溃。

![](https://miro.medium.com/max/470/1*bPYM5NAZ8OYenRAejcI7uA.gif)

你也可以用 _expect()_ 函数像下方这样来处理 Result：

```rust
let my_number: f64 = my_string.trim().parse().expect(“Parse failed”);
```

_expect()_ 的工作方式类似于 _unwrap()_，假如 Result 是 _Err_，_expect()_ 将会使程序崩溃**并且**将其中的字符串内容 —— “Parse failed.”展示在标准输出中。

## 使用 unwrap() 和 expect() 的缺点

当我们使用 _unwrap()_ 和 _expect()_ 函数时，如果遇到错误，程序会发生崩溃。如果错误发生的几率非常小，这也许可以容忍，但在某些情况下，错误发生的概率会比较大。

在上面的示例中，用户可能输入错误，输入的不是数值（可能是字母或者特殊符号）。我们并不想每次用户输入错误的内容程序就发生崩溃。相反，我们应该提示用户应该输入数字。这种场景下，Result 就非常有用，尤其是当它与一个模式匹配的表达式相结合的时候。

# 用匹配表达式修复错误

```rust
use std::io::{stdin, self, Write};
fn main(){
    let mut my_string = String::new();
    print!(“Enter a number: “);
    io::stdout().flush().unwrap();
    let my_num = loop {
        my_string.clear();
        stdin().read_line(&mut my_string)
            .expect(“Did not enter a correct string”);
        match my_string.trim().parse::<f64>() {
            Ok(_s) => break _s,
            Err(_err) => println!(“Try again. Enter a number.”)
        }
    };
    println!(“You entered {:?}”, my_num);
}
```

如果你问我怎么实现，上面就是示例代码！

前面提到的不优雅的实现和优雅的实现方式的不同点是在循环体内部。我们可以分解一下。

# 代码分析

在 loop 之前，我们提示用户输入一个数字。接着我们声明 my_num。

我们将循环体中返回的值（用户的输入，它将从字符串转换为数字）赋给 my_num：

```rust
let my_num = loop {
```

在循环体中，我们阻塞等待用户输入。然后接收用户的输入，在这个过程中我们有三个问题要解决。

- 1.我们需要确定用户输入的是数字而非其他的字符，一个词或者一个字母。
- 2.Rust 中的 _read_line()_ 函数能够以字符串的类型拿到用户的输入。我们需要将其转换为浮点数。
- 3.如果用户没有输入数字，我们需要清理变量，并提示和等待用户再次输入。

在第三部分问题（清理 my_string 变量）在循环体内的第一行就已经实现了：

```rust
my_string.clear();
```

下一步，我们接收用户的输入：

```rust
stdin().read_line(&mut my_string)
    .expect(“Did not enter a correct string”);
```

_read_line()_ 函数返回一个 Result 类型。我们使用 _expect()_ 函数处理它。在这种情形下是完全没问题的，因为 _read_line()_ 出错的几率非常小。用户通常只能在终端输入一个字符串，而这正是 _read_line()_ 所需要处理的。

通过 _read_line()_ 把用户输入的字符串返回并存在 _my_string_ 变量中。

## 重要部分

现在我们已经将输入的字符串存在 _my_string_ 中，我们需要将其转换为浮点数。使用 _parse()_ 函数可以实现，然后将浮点数结果返回。所以我们有不止 Result 的类型需要处理，但这一次，我们很可能会出现一个错误。如果用户输入的是非数字， _parse()_ 将会返回一个错误类型的 Result（_Err_）。如果发生这种情况，我们不希望程序崩溃。而是希望提示用户没有输入正确的数字，请再试一次。为此，我们需要写好调用 _parse()_ 成功时的逻辑，还要写好调用失败时的逻辑。类似于逐个处理匹配表达式可能的结果。

## 分析匹配表达式

```rust
match my_string.trim().parse::<f64>() {
    Ok(_s) => break _s,
    Err(_err) => println!(“Try again. Enter a number.”)
}
```

首先，我们使用 match 关键字来声明匹配表达式。然后，我们提供与表达式匹配的可能的值。这个值就是下面所示：

```rust
my_string.trim().parse::<f64>()
```

这段代码接收 my_string 参数，它将用户输入的内容保存下来，并提供给 _trim()_ 函数。_trim()_ 函数会删除掉字符串两侧可能存在的额外空行或空格。我们之所以需要 _trim()_ 是因为 _read_line()_ 函数在输入中附加了一个额外的空行，这会导致转换会出现异常。然后将清理了空格字符的 my_string 传递到 _parse()_ 函数中，该函数会尝试将其转换为浮点数。

如果 _parse()_ 成功地将 my_string 转换为数字，则返回 Ok。在这个情况下，我们可以得到浮点数。如果用户输入的不是数字，那么 _parse()_ 将无法正常完成转换，它会返回 Err。

在匹配表达式的花括号（主体）中，我们根据 _parse()_ 返回的类型告诉计算机怎么做：

```rust
Ok(_s) => break _s,
Err(_err) => println!(“Try again. Enter a number.”)
```

**如果结果是 Ok**，则表示 _parse()_ 能够转换该类型。这时，我们调用一个 break，停止循环，并返回存储在 Ok 中的值，这个值会被放在 _s 变量中。

**如果结果是 Err**，_parse()_ 无法完成转换。这时，我们会告诉用户“重试一次。输入一个数字”。由于我们不调用 break，所以循环重新开始。

>如果必须用一句话解释 Result，那就是：如果一个函数返回 Result，一个匹配表达式可以根据结果是 Ok 还是 Err 来执行不同的代码。

## 在你的函数中使用 Result

既然你已经了解了处理 Result 的方法，那么你可能会希望在你自己创建的函数中使用它。

我们先看一个例子。

```rust
fn main(){
    let my_num = 50;
    
    fn is_it_fifty(num: u32) -> Result<u32, &’static str> {
        let error = “It didn’t work”;
        if num == 50 {
            Ok(num)
        } else {
            Err(error)
        }
    }
    match is_it_fifty(my_num) {
        Ok(_v) => println!(“Good! my_num is 50”),
        Err(_e) => println!(“Error. my_num is {:?}”, my_num)
    }
}
```

这个程序检查 _my_num_ 的值。如果值为 50，则表示成功；如果不是，则表示错误。

这段代码的主体是 _is_it_fifty()_ 函数。它是有返回结果的声明式函数。我们逐行看其中的代码。

首先，我们声明 _my_num_ 并给它赋值。然后，我们声明 _is_it_fifty()_ 函数：

```rust
fn is_it_fifty(num: u32) -> Result<u32, &’static str> {
```

在我们的声明中，我们指定该函数接收一个名为 num 的参数，其类型是 32 位无符号整数类型（u32）。接下来，我们指定函数的返回值类型。表示函数会返回一个结果，类型是 u32 或字串（&'static str）

然后，我们编写 _is_it_fifty()_ 的函数体。

```rust
let error = “It didn’t work”;
if num == 50 {
    Ok(num)
} else {
    Err(error)
}
```

函数体中的代码是一个 if else 表达式。它用于判断传入的参数。

如果值是 50，那么函数将返回 Ok 的 Result。Ok 中将会包含传递给函数的值（_num_）。

如果参数不是 50，函数将返回 Err 的 Result。_Err_ 会包含错误变量的值，也即 “It didn’t work.”

无论何时使用该函数，都必须处理它返回的 Result。在我们的程序中，与大多数 Rust 程序一样，是通过一个匹配表达式完成的。我在之前已经描述过部分匹配表达式。

Result 类型可以使用 _unwrap()_ 或 _expect()_ 来处理 —— 前面也已经解释过。

![](https://miro.medium.com/max/480/1*ZLronSWbmj4IwGoWepecHQ.gif)

## 总结

Result 是一个函数的返回类型，它表示函数执行是否成功。

Rust 的许多内置函数都是返回 Result 类型，如果是这样的话，就没有办法避开它。如果一个函数返回 Result，它必须要被妥善处理。

处理 Result 常用的方法是使用 _unwrap()_ 和 _expect() 函数以及匹配表达式。

可以从自己定义的函数中返回 Result。这是处理错误的好办法。

关于 Rust 的 Result 类型，你需要知道的就这些了，但是如果想了解更多信息，或者想知道我从哪儿收集的这些信息，可以参考下方的资源列表。

## 资源

* https://doc.rust-lang.org/std/result/
* https://doc.rust-lang.org/1.2.0/book/match.html
    * 查看 `matching on enums` 部分
* https://doc.rust-lang.org/1.30.0/book/first-edition/error-handling.html
* https://doc.rust-lang.org/rust-by-example/flow_control/match.html
* https://blog.jonstodle.com/things-i-enjoy-in-rust-error-handling/
* https://stevedonovan.github.io/rust-gentle-intro/6-error-handling.html
* https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
* https://doc.rust-lang.org/std/result/enum.Result.html#method.expect
