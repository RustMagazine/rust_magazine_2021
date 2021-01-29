# Cita: 用Rust写Protobuf扩展

作者： 宁志伟

---

作者简介：

宁志伟

溪塔科技首席架构师
首个微服务架构区块链`CITA` 首席架构师，区块链+云原生框架 `CITA-Cloud` 设计者。前阿里巴巴、华为技术专家，超过 `10` 年分布式系统架构设计，编程语言和虚拟机方面工作经验。

- Blog   : [https://rink1969.github.io](https://rink1969.github.io/)
- GitHub : [https://github.com/rink1969](https://github.com/rink1969)


- 为国产自主云原生区块链`CITA-Cloud`点赞[https://github.com/cita-cloud/cita_cloud_proto
](https://github.com/cita-cloud/cita_cloud_proto)


## Protobuf

`Protocol Buffers` (简称 `Protobuf` ) ，是 `Google` 出品的序列化框架，与开发语言无关，和平台无关。具有体积小，速度快，扩展性好，与 `gRPC` 搭配好，支持的语言多等特点，是目前应用最广泛的序列化框架。

使用场景一般是在微服务架构中，用来定义微服务之间的 `gRPC` 接口，以及相关的参数/返回值等数据结构的定义。

通过官方的编译器 `protoc` 以及相应的插件可以方便的生成不同语言的实现代码。这样不同的微服务可以使用不同的开发语言，同时还能顺利进行交互。

## `CITA-Cloud`中的`Protobuf`

`CITA-Cloud` 采用了[微服务架构](https://cita-cloud-docs.readthedocs.io/zh_CN/latest/blockchain.html)，因此也采用了 `Protobuf` 和 `gRPC` 的组合。

但是因为 `Protobuf` 语言无关的特性和广泛的应用，使得其具有抽象和通用的特点。因此也可以把 `Protobuf` 当作一种建模语言来使用，[参见文章](https://zhuanlan.zhihu.com/p/162839054)。

`CITA-Cloud` 目前是在[协议](https://github.com/cita-cloud/cita_cloud_proto)中直接把交易和区块等数据结构固定下来的。但是最近的思考发现，其中的很多字段都是为了实现某种应用层面的协议而存在的。比如交易中的 `nonce` 字段就是为了实现应用层面的去重协议。

因此，后续计划提供一个框架，方便用户自定义交易和区块等核心数据结构，以及相关的处理函数。但是 `Protobuf` 通常只能生成数据结构，以及相关的 `get/set` 等模式比较固定的代码，如果要生成复杂的成员函数，就需要一些扩展能力。

## `Protobuf`扩展

`Protobuf` 的扩展能力可以分为两种： `Protobuf` 本身的扩展和 `Protobuf` 插件。

`Protobuf` 其实是个标准的编译器架构。我们可以把 `.proto `文件视作源码，官方的 `protoc` 编译器可以对应到编译器前端。

`protoc` 接收一个或者一批 `.proto` 文件作为输入，解析之后输出一种中间描述格式，对应编译器中的 `IR` 。

但是有意思的是，这种中间描述格式是二进制的，其结构依旧由 `Protobuf` 本身描述。详细可以参见[descriptor.proto](https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/descriptor.proto)。

`Protobuf` 插件可以对应到编译器后端，接收中间描述格式，解析其中的信息，据此生成具体语言的代码。

这里其实有个非常有意思的问题。插件在解析中间描述格式的数据时，因为这种格式是由 `descriptor.proto` 描述的，所以得先有个插件能把 `descriptor.proto` 生成开发插件所使用的开发语言的代码。

上面的话有点绕，举个具体的例子。比如我想用 `Rust` 实现一个插件，假如目前还没有 `Protobuf` 相关的 `Rust` 库，那就没办法用 `Rust` 代码来解析 `descriptor.proto` 对应的中间描述格式的数据，也就没法实现插件了。

这个问题其实就对应编译器里的自举问题。比如，想用 `Rust` 来写 `Rust` 编译器，那么一开始就是个死结了。解决办法也很简单，最开始的 `Rust` 编译器是用 `Ocaml` 实现的，然后就可以用 `Rust` 来写 `Rust` 编译器，实现编译器的 `Rust` 代码用前面 `Ocaml` 实现的版本去编译就可以解决自举问题了。

`Protobuf` 这里也是同样的，官方提供了 `Java/Go/C++/Python` 等版本的实现，可以先用这些语言来过渡。

另外一种扩展方式是 `Protobuf` 本身提供了语法上的[扩展机制](https://developers.google.com/protocol-buffers/docs/proto#extensions)。这个功能可以对应到编程语言提供的宏等元编程功能。

`Protobuf` 这个扩展能力有点类似[`AOP`](https://www.liaoxuefeng.com/wiki/1252599548343744/1266265125480448)，可以方便的在已经定义的 `Message` 中增加一些成员。

更有意思的是，前面提到过，所有的 `.proto` 文件，经过 `protoc `之后，会被转换成由 `descriptor.proto` 对应的中间描述格式。而 `descriptor.proto` 中的 `Message` 也同样支持上述扩展功能，因此可以实现一种类似全局 `AOP` 的功能。

通过扩展 `descriptor.proto` 中的 `Message` ，可以实现给所有的 `Message` 都加一个 `option` 这样的操作。

## `Rust`中相关的库

`dropbox` 实现了一个 `Protobuf `库[`pb-jelly`](https://github.com/dropbox/pb-jelly)，它就是用 `Python` 来实现生成 `Rust` 代码部分的功能。具体实现其实比较简单，就是在拼 `Rust` 代码字符串。

[`rust-protobuf`](https://github.com/stepancheg/rust-protobuf)是一个实现比较完整的 `Protobuf` 库，支持 `gRPC` 和相关的扩展能力。其中实现分为两部分，生成数据结构 `Rust` 代码的插件和生成 `gRPC` 相关代码的插件。具体实现封装的稍微好了一点，但是基本上还是在拼 `Rust` 代码字符串。

[`prost`](https://github.com/danburkert/prost)是一个比较新的 `Protobuf` 库实现。功能上有点欠缺，不支持扩展。库本身只支持生成数据结构的`Rust` 代码。生成 `gRPC` 相关代码的功能在[`tonic-build`](https://github.com/hyperium/tonic)里，这个有点奇怪。

但是 `prost` 采用了很多新的技术。前面提到，插件只会生成数据结构相关的 `get/set` 等模式比较固定的代码， `prost` 实现了一个 `derive` 来自动给数据结构增加这些成员函数，这样生成的 `Rust` 代码就大大简化了，[参见例子](https://github.com/cita-cloud/cita_cloud_proto/blob/master/src/common.rs)。

这也跟编译器架构能对应上：一个选择是把编译器后端做的很复杂，直接生成所有的代码，运行时比较薄；另外一个选择是编译器后端做的很简单，生成的代码也简单，但是运行时比较厚重。

另外 `gRPC` 相关的代码比较复杂， `tonic-build` 在生成的时候用了[`quote`](https://github.com/dtolnay/quote)库，提供类似` Rust` 代码语法树上的 `sprintf` 方法的功能，不管是便利性还是代码的可读性都比之前两个库好很多。

## 后续计划

后续计划使用 `Protobuf` 及其扩展能力，实现一个框架，不但用来描述交易和区块等核心数据结构，也以一种可配置的方式生成一些比较复杂的相关代码。

最重要的第一步就是要能解析出 `Protobuf` 扩展相关的信息，因为正常的 `.proto` 文件只能用于描述数据结构，扩展的 `option` 是唯一可以赋值的地方。

目前实现了一个[`proto_desc_printer`](https://github.com/rink1969/proto_desc_printer)，可以解析中间描述格式，特别是其中的扩展信息。

后续可以在这个基础上去做代码生成部分的工作，这里可以从 `prost` 吸取很多好的经验。
