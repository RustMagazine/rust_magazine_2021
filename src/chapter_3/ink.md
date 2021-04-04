---
pub_date: Sat, 27 Mar 2021 16:00:00 GMT
description: Write smart contracts in Rust | Hello, Ink!

---

# 用 Rust 写智能合约 | Hello, Ink! 

作者：李大狗（李骜华）/ 后期编辑： 张汉东

---

## 什么是 WASM 智能合约？

以往，我们谈到智能合约，都是基于 EVM 的 Solidity 智能合约。

目前，随着智能合约技术的发展，出现了一种新的可能性：WASM 智能合约，

WASM 并非一门新的编程语言，而是一种全新的底层二进制语法。

> WASM（WebAssembly）是一种新的字节码格式，是一种全新的底层二进制语法，它所编译的代码指令体积小，可移植，加载快并兼容WEB的全新格式。WASM可以支持C/C++/RUST/GO等多种语言编写合约后编译出节码，且不同语言有附带丰富的底层标准库可供调用。
>
> WASM 的优势：
>
> 作为一种全新的字节码格式，WASM通过自身的创新和优化，使得在使用其对所支持的语言进行编写后的代码指令具有体积小，可以在运存，硬盘存储，带宽占有上得到更多的优化，在节省了区块链网络资源，也明显的提升了网络传输效率。
>
> 在智能合约上使用WASM，也将拥有以上特点，最明显的方面就是占用资源更少，运行合约更快速和稳定，并且网络传输信息更加高效。这可以使得区块链网络上部署更多的智能合约，也可以使得用户在使用智能合约时能获得更好的体验感。
>
> ——WASM智能合约优势分析：https://zhuanlan.zhihu.com/p/344347968

从目前的趋势上来看，Substrate、ETH 2.0等公链与多家联盟链，均表示将支持 WASM 智能合约。

## 可以用什么语言编写 WASM 智能合约？

Wasm 扩展了智能合同开发者可用的语言系列，包括 Rust、C/C++、C#、Typescript、Haxe 和 Kotlin。这意味着你可以用你熟悉的任何语言编写智能合约。

从适配性上来说，Rust 语言目前与 WASM 智能合约的适配性更好，工具链更全，而且写出来的智能合约更加安全。

所以，本系列将以 Subtrate 上的 Ink! 智能合约为例，开始 WASM 智能合约的 101 课程。

本文对 Ink! 官方教程有所参考：

> https://substrate.dev/substrate-contracts-workshop

## Rust 环境配置

### 1.  Rust 环境配置

在 MacOS 或者 Ubuntu 等 Linux 操作系统上，我们可以通过一行命令很容易的安装 Rust：

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

除此之外还要安装`nightly`版本：

```
rustup install nightly
```

Windows 下的安装，请参考：

```
https://forge.rust-lang.org/infra/other-installation-methods.html
```



### 2. 将 Rust 添加到环境中

将如下语句添加到`~/.bashrc`或`~/.zshrc`中：

```
export PATH=~/.cargo/bin:$PATH
```

然后：

```
source ~/.bashrc # source ~/.zshrc
```

###  3. 换源

通过设置如下环境变量，我们把 Rust 源切换到国内：

```bash
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
```

在`~/.cargo/config`文件中写入如下内容：

```
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
```

## Ink! 环境配置

在配置了基本的 Rust 环境后，我们可以配置 Ink! 所需的开发环境了。

```bash
# for substrate
rustup component add rust-src --toolchain nightly
rustup target add wasm32-unknown-unknown --toolchain stable
# for canvas node 
cargo install canvas-node --git https://github.com/paritytech/canvas-node.git --tag v0.1.4 --force --locked
# for ink!CLI
cargo install cargo-contract --vers 0.10.0 --force --locked
```

我们还要安装/升级`binaryen`，Binaryen 是 WebAssembly 的编译器。

Mac 上安装：

```bash
# for mac
brew upgrade binaryen # 如果没安装用 brew install
```

Linux 上安装：

## 创建一个 ink! 项目

执行如下命令：

```
cargo contract new flipper
```

创建完成后进入文件夹：

```
cd flipper/
```

合约项目目录结构：

```
flipper
|
+-- lib.rs                <-- Contract Source Code
|
+-- Cargo.toml            <-- Rust Dependencies and ink! Configuration
|
+-- .gitignore
```

## 合约测试

```
cargo +nightly test
```

一切顺利的话会输出如下结果：

```
$ cargo +nightly test
    running 2 tests
    test flipper::tests::it_works ... ok
    test flipper::tests::default_works ... ok

    test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## 合约编译

```
cargo +nightly contract build
```

![image-20210323213148674](https://tva1.sinaimg.cn/large/008eGmZEgy1goxq16ioatj30mu0n6wj1.jpg)

如果顺利的话，目录下会生成`target/ink`文件夹，里面包含如下文件：

![image-20210323205600627](https://tva1.sinaimg.cn/large/008eGmZEgy1goxq12tx3jj30si04caan.jpg)

其中，`flipper.contract` 是部署时要用的合约文件，可以视为`solidity`合约中的`bin`文件。

`metadata.json`是元数据，可以视为`solidity`合约中的`abi`文件。

## 合约部署

通过`canvas`启动一个本地运行的开发节点！

```
canvas --dev --tmp
```

![An image of the terminal starting a Substrate node](https://tva1.sinaimg.cn/large/008eGmZEgy1goxq15c98vj30si04caan.jpg)

打开如下网址，会这个页面会自动连接本地启动的开发节点：

![image-20210323210306845](https://tva1.sinaimg.cn/large/008eGmZEgy1goxq17aigpj30wq0ln7cz.jpg)

上传`flipper.contract`这个文件：

![Contracts code page for deploying Flipper](https://tva1.sinaimg.cn/large/008eGmZEgy1goxq19w3y0j30z80iqq48.jpg)

一路点击进行部署：

![image-20210323210730452](https://tva1.sinaimg.cn/large/008eGmZEgy1goxq1avddtj31c10u0q6a.jpg)

![image-20210323210747989](https://tva1.sinaimg.cn/large/008eGmZEgy1goxq17hukij31200qmacj.jpg)

![image-20210323210811846](https://tva1.sinaimg.cn/large/008eGmZEgy1goxq18snwhj30u00xb40w.jpg)

## 合约调用

点击`Execute`：

![image-20210323210928445](https://tva1.sinaimg.cn/large/008eGmZEgy1goxq1bnzbuj30xw0i6jss.jpg)

选择`get():bool`函数，点击「调用」：

![image-20210323211004303](https://tva1.sinaimg.cn/large/008eGmZEgy1goxq18ec75j31ck0u0q6r.jpg)

返回调用结果：

![image-20210323211027286](https://tva1.sinaimg.cn/large/008eGmZEgy1goxq1929ldj30wg0codgq.jpg)

## Flipper 源码解读

```rust
// Copyright 2018-2020 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod flipper {
    #[ink(storage)]
    pub struct Flipper {
        value: bool,
    }

    impl Flipper {
        /// Creates a new flipper smart contract initialized with the given value.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Creates a new flipper smart contract initialized to `false`.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// Flips the current value of the Flipper's bool.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Returns the current value of the Flipper's bool.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn default_works() {
            let flipper = Flipper::default();
            assert_eq!(flipper.get(), false);
        }

        #[test]
        fn it_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);
            flipper.flip();
            assert_eq!(flipper.get(), true);
        }
    }
}
```

### 1.  `cfg`和`cfg_attr`的使用

`cfg`是 Rust 中的特殊属性， 它允许我们编译基于标志的代码并传递给编译器。

在本合约中，我们可以看到：

```
#[cfg(test)]
```

这个标识意味着下面的代码是单元测试。

### 2. impl 关键字

> Implement some functionality for a type.
>
> 为一种类型做函数实现。

标准的模板是：

```
struct Example {
    number: i32,
    # 许多变量……
}

impl Example {
    fn boo() {
        println!("boo! Example::boo() was called!");
    }

    fn answer(&mut self) {
        self.number += 42;
    }
		# 许多函数……
}

```

套用到本合约中，首先我们定义本合约的`struct`：

```
pub struct Flipper {
	value: bool, # 其中包含一个变量 value
}
```

然后对`struct`进行补充实现：

```
impl Flipper {
	……
}
```

### 3.  ` #[ink(constructor)]`与`#[ink(message)]`

` #[ink(constructor)]`表示这行语句函数是合约的构造函数，相当于`solidity`合约中的`constructor`。

> https://docs.soliditylang.org/en/v0.7.2/contracts.html#constructor

`#[ink(message)]`表示这行语句下面的函数是合约的普通函数，如例子中的`get`函数：

```
/// Returns the current value of the Flipper's bool.
#[ink(message)]
pub fn get(&self) -> bool {
	self.value
}
```

---

作者简介：

李大狗（李骜华），上海对外经贸大学区块链技术与应用研究中心副主任、柏链教育 CTO、FISCO BCOS（微众银行区块链框架）区块链认证讲师、5 年区块链工程师、北京大学硕士。
研究领域包括：区块链系统、共识机制、智能合约、区块链应用、数字身份等。





















