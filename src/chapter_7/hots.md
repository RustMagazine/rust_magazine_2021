# 社区热点

编辑：张汉东 

---

## Facebook 从 Java 到 Rust | Buck 的未来

Buck 是 Facebook 在 2013 年的Facebook Mobile DevCon上亮相的一个快速的 Android 构建系统。从那时起，它已经扩展到支持用15种以上的语言编写的应用程序，目标平台包括手机、服务器、智能设备和VR头盔等等。

不过，随着时间的推移，Buck并没有跟上这种功能和灵活性的增长，没有进行必要的架构改变和改进来管理这种复杂性。随着Facebook内部和外部开发新的功能，发现这导致了巨大的实施复杂性，对核心和语言规则进行修改的挑战越来越大。虽然该团队在近4年前就开始了一项举措，逐步对 Buck 进行一些这样的跨领域的架构改进，但这些改变是非常困难的。

所以，从2020年开始，该团队开始构思，如果从零开始构建 Buck 会怎么样？怎样才能写出一个可以在未来 10 年或 20 年中继续扩展的构建系统？

在考虑这次重写的同时，我们利用这个机会实验并验证了Rust作为构建系统的技术栈。Facebook对Rust编程语言的投资越来越大，许多项目都在使用它，并取得了巨大的成功。我们发现这种语言非常合适，原因有很多：

1. Rust的`async/await`语法使得编写异步代码非常顺畅，而且Rust有助于正确处理复杂的并发性细节。Buck的Java计算向并发计算的每一次迁移都是一个长达数月的艰难过渡，而且那里仍然存在着明显的单线程瓶颈。
2. Rust有很多高级语言特性，使开发更容易，更愉快。这些都是像枚举、模式匹配、特质（trait）、过程宏和所有其他的功能，这些功能都受到 Rust开发者的广泛喜欢。
3. Rust对内存分配提供了更大的控制。GC语言（即便是分代收集）在处理像Buck这样的增量计算时也有挑战。
4. Rust是高性能的。我们已经看到了将一些程序用Rust重写后的显著加速。

在接下来的几个月里，你可能会看到Buck的进展较少，但请放心，我们将继续努力为社区提供最好的构建系统。我们认识到，分享过程的一个重要部分是为Buck的用户定义一个平稳的过渡，并确保社区可以和我们一起前进。我们计划在2022年1月前公开这个方案，届时会有更多关于当前Buck用户过渡的细节。

[https://developers.facebook.com/blog/post/2021/07/01/future-of-buck](https://developers.facebook.com/blog/post/2021/07/01/future-of-buck)

## 知乎近期 Rust 相关问题摘录

-  [2021年了，Rust在偏底层的某些领域是替代C++的一个好的选择吗？](https://www.zhihu.com/question/451687128)
- [相比Rust，现代C++有什么难度吗?](https://www.zhihu.com/question/447731745)
- [如何看待 Rust 的应用前景？](https://www.zhihu.com/question/30407715)
- [在2021 年，Rust 将会比 C++ 强在哪里？](https://www.zhihu.com/question/437987252)
- [Rust 的优点是什么？](https://www.zhihu.com/question/463506409)
- [就高频量化交易系统而言，据说rust作为主要面向安全的高性能计算编程语言，比c++要强，这个是真的吗？](https://www.zhihu.com/question/390738348)

## Rust + Copilot 什么效果？

近日 Discord 工程师尝试用 copilot 来辅助开发 Rust 项目。效果不是很好。

视频观看：[https://t.me/rust_daily_news/4914](https://t.me/rust_daily_news/4914)

## rustc_codegen_gcc 的 MCP 已经被接受

在不久的将来，Rust 就会多一个 GCC 的后端。

同类项目还有 [GCC-rs ](https://github.com/Rust-GCC/gccrs),GCC-rs  是 用 Cpp 重新实现 Rustc 的一个 GCC 前端。

为什么有 GCC-rs 这个项目？

1. 想要支持更多的 CPU 架构
2. 跨语言 LTO。GCC-RS FAQ将Linux列为激励示例。 具有讽刺意味的是，Linux支持ltvm但不是gcc！
3. Rust 自举（Bootstrap）链很长，因为需要从C到OCAML，然后编译预发布 Rust 以编译 Rust 1.0编译 Rust 1.1 、1.2等，直到捕获最多1.53（或者最新版本）。 因此，如果您可以用C++中编写的 Rust 编译器直接编译1.53，则可以节省一些时间。
4. 复用 GCC 插件

但 [rustc_codegen_gcc](https://github.com/antoyo/rustc_codegen_gcc)  作者认为 GCC-rs 其实没有很好的解决这些问题。

rustc_codegen_gcc 项目只需将GCC插入现有的Rust编译器作为代码生成后端，就可以简单的达成这些目标。

该项目的主要目标是能够在LLVM不支持的平台上编译 Rust 代码。 次要目标是检查使用GCC后端是否提供任何编译速度改进。

现在 rustc_codegen_gcc 已经被接受，gcc-rs 该何去何从？

相关阅读：[Rust 与 开源 | GPL 许可证引发的问题](https://zhuanlan.zhihu.com/p/387946955)

## Rust GameDev #23 

这一期游戏开发报告中包含了一些很有创意的游戏。写 Rust 累了，可以玩一玩，都是开源的。

1.  吃尾蛇。支持wasm，可以网页玩耍。关卡设计的很有心。基于 bevy 0.5 实现。

- [https://github.com/szunami/taileater/](https://github.com/szunami/taileater/)
- [https://szunami.itch.io/taileater](https://szunami.itch.io/taileater)

2. Egregoria，模拟城市建设者，试图复制现代社会以及尽可能复制。基于  Legion ecs 实现。

[https://github.com/Uriopass/Egregoria](https://github.com/Uriopass/Egregoria)


3. Blightmud ，是一款 命令行终端的 mud 客户端，可以支持很多 mud server，比如 bat.org 等。

[https://github.com/Blightmud/Blightmud](https://github.com/Blightmud/Blightmud)

4.  Dango， 多人物理沙盒游戏。基于 bevy , Nphysics 物理引擎， CrystalObs 网络库等。Dango 目前在浏览器中基于wasm 运行 server，其他玩家通过 webrtc 加入。

[https://github.com/ErnWong/dango-tribute](https://github.com/ErnWong/dango-tribute)

5. hyper-farmer ： 基于bevy实现，游戏虽然简单但是挺有创意，锻炼双手协调

- [https://wilsk.itch.io/hyper-farmer](https://wilsk.itch.io/hyper-farmer) 在线玩
-  [https://github.com/will-hart/cloud-surfer ](https://github.com/will-hart/cloud-surfer )

6. fish-game，基于 macroquad 游戏引擎实现，支持wasm

- [https://fedorgames.itch.io/fish-game](https://fedorgames.itch.io/fish-game) 在线玩
- [https://github.com/heroiclabs/fishgame-macroquad](https://github.com/heroiclabs/fishgame-macroquad) 源码
-  [https://github.com/not-fl3/macroquad](https://github.com/not-fl3/macroquad) 游戏引擎

[https://gamedev.rs/news/023/](https://gamedev.rs/news/023/)