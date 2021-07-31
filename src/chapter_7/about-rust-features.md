## 为什么你不应该沉迷于 Rust 的“特性”

译者：[NiZerin](https://github.com/NiZerin)

> 原文链接：[https://nullderef.com/blog/rust-features/#cargo-release](https://nullderef.com/blog/rust-features/#cargo-release)

---

Rust 使得表达条件编译变得非常容易，特别是由于它的“特性”。它们很好地集成到语言中并且非常易于使用。但是我通过维护Rspotify（Spotify API 的库）学到的一件事 是，人们不应该对它们着迷。当条件编译是解决问题的唯一方法时，应该使用条件编译，原因有很多，我将在本文中解释。

这对某些人来说可能很明显，但对我来说，当我开始使用 Rust 时并没有那么清楚。即使您已经知道，这也可能是一个有趣的提醒；也许您在最新的项目中忘记了它并添加了一个不必要的功能。

条件编译也不是什么新鲜事。C 和 C++ 已经这样做了很长时间，一方面。所以同样的事情可以应用于这些情况。然而，根据我的经验，在 Rust 中使用条件编译要容易得多，这意味着它也更有可能被滥用。

### 问题
在决定如何在 Rspotify 中配置缓存令牌时，我遇到了这个困境 。所述库使您可以通过 JSON 文件持久管理身份验证令牌。这样，当程序再次启动时，可以重复使用前一个会话中的令牌，而无需再次执行完整的身份验证过程——也就是说，直到令牌过期。

最初，这将是一个名为cached_token. 我并没有想太多。如果您不需要它，为什么需要代码来保存和读取令牌文件？最简单的方法是使用一个功能，你可以在你的 Cargo.toml.

但是，我后来需要另一个非常相似的功能，refreshing_token. 当可选地启用时，客户端将自动刷新过期的令牌。随着这种模式在库中越来越多地出现，我想确保它的设计是最佳的。深入研究后，我开始发现功能的许多不便之处：

它们是不灵活的：你不能在同一个程序中拥有一个带有缓存令牌的客户端和另一个没有它们的客户端。这是一个图书馆范围的事情，所以你要么启用它们，要么不启用。显然，它们也不能在运行时进行配置；用户可能想选择什么样的行为遵循，而 在程序运行。

它们很丑：写作 ``#[cfg(feature = "cached_token")]``比普通的更奇怪和冗长 ``if cached_token``。

它们很乱：代码库中的功能很难管理。你可以很容易地发现自己处于 Rust 中，相当于 ``#ifdef`` 地狱。

它们很难记录和测试：Rust 没有提供公开库功能的方法。您所能做的就是在文档的主页中手动列出它们。测试也更难，因为您必须弄清楚要使用哪些功能组合来覆盖整个代码库，并在您想要运行测试时应用它们。

仅仅保证二进制文件不会包含您不需要的代码，所有这些都被认为是重要的。但这有多真实，真的吗？它有多重要？

### 替代
事实证明，编译器可以实现的最简单的优化之一是常量的传播。这与去除死代码相结合，可以产生与特征完全相同的效果，但以更自然的方式。除了添加功能来配置程序的行为之外，您还可以对 Config 结构进行相同的操作。如果它只是一个要配置的选项，您甚至可能不需要结构体，但这样它就可以面向未来。例如：

```rust
#[derive(Default)]
struct Config {
    cached_token: bool,
    refreshing_token: bool,
}
```
然后，您可以修改您的客户端，以便有选择地采用Config结构：

```rust
struct Client {
    config: Config
}

impl Client {
    /// Uses the default configuration for the initialization
    fn new() -> Client {
        Client {
            config: Config::default(),
        }
    }

    /// Uses a custom configuration for the initialization
    fn with_config(config: Config) -> Client {
        Client {
            config,
        }
    }

    fn do_request(&self) {
        if self.config.cached_token {
            println!("Saving cache token to the file!");
        }
        // The previous block used to be equivalent to:
        //
        // #[cfg(feature = "cached_token")]
        // {
        //     println!("Saving cache token to the file!");
        // }

        if self.config.refreshing_token {
            println!("Refreshing token!");
        }

        println!("Performing request!");
    }
}
```

最后，用户可以以一种非常自然的方式在代码中自定义他们想要的客户端：

```rust
fn main() {
    // Option A
    let client = Client::new();

    // Option B
    let config = Config {
        cached_token: true,
        ..Default::default()
    };
    let client = Client::with_config(config);
}
```

### 证明你最终得到了相同的代码
感谢出色的Compiler Explorer，我们可以使用以下代码段确保编译符合我们的预期：

![image](https://user-images.githubusercontent.com/18081398/125720306-c8b247fe-3f07-454f-9343-f1da72ef1325.png)

似乎从 Rust 1.53 开始，对于 ``opt-level`` 大于或等于 2 的值，已停用功能的代码甚至不会出现在程序集中（通过查看末尾的字符串很容易看到）。``cargo build --release`` 配置 ``opt-level`` 为 3，因此对于生产二进制文件应该不是问题。

我们甚至没有使用 ``const`` ！我想知道在这种情况下会发生什么。使用 这个稍微修改的片段：

![image](https://user-images.githubusercontent.com/18081398/125720399-9729bbcb-ecae-4470-93be-6166b46022a4.png)

唔。我们实际上得到了相同的结果。生成的程序集完全相同，可选代码仅从 ``opt-level=2``.

问题是这 ``const`` 仅仅意味着它的值可以（而不是必须）被内联。没有其他的。所以我们仍然没有任何保证，内联不足以简化函数内部的代码。

因此，对于我所调查的内容，最好不要担心它并使用变量而不是 ``const``. 它看起来更好，并得到相同的结果。

### 无论如何

即使之前的优化没有实现，可选代码真的会对最终的二进制文件造成任何伤害吗？我们是否一如既往地过度设计了解决方案？事实是缓存/刷新令牌的可选代码甚至没有那么膨胀。

当然，这取决于，但在我看来，二进制膨胀对于更高级别的二进制文件来说并不是什么大问题。Rust 已经在每个二进制文件中静态嵌入了它的标准库、运行时和大量调试信息，总大小约为 3MB。您在运行时可能获得的唯一开销是分支。

### 结论

有时你只需要使用条件编译；没有办法解决它。您可能正在处理特定于平台的代码或想要减少 crate 的依赖项数量，在这种情况下，功能非常有用。

但这不是 Rspotify 的情况。条件编译绝对不是要走的路。当你准备向你的 crate 引入一个新特性时，想想自己，“我真的需要条件编译吗？”。

既不遵循通常的推理，cached_token也不refreshing_token遵循为什么可能会添加功能的原因。他们不允许访问新的功能/模块。它们无助于摆脱可选的依赖项。而且它们当然不是特定于平台的功能。他们只是配置库的行为。

为了避免这种情况，也许功能的命名可能会有所不同？启用对缓存令牌的支持听起来确实是一项“功能”，而特定于操作系统的代码似乎并不是真正的功能。有时我也觉得很困惑，谷歌在这一点上同意我的观点。寻找与 Rust 特性相关的信息可能会返回完全不相关的东西，因为结果有“特性”这个词，但意思是“程序的一个属性或方面”。有点像你必须谷歌“golang X”而不是“go X”，否则它没有意义。但无论如何，我的意见已经太迟了。
