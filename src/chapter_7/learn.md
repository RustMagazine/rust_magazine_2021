# 学习资源

编辑：张汉东

---

## Rand 之书

- [英文：The Rust Rand Book](https://github.com/rust-random/book/)
- [中文：Rand 之书](https://github.com/zjp-CN/Rust-Rand-Book-zh)

## Rust 新书 ：《 Hands-on Rust 》

PragProg 出版社出的一本新书 《Hands-on Rust》，以游戏开发为主题学习 Rust 。 

[https://pragprog.com/titles/hwrust/hands-on-rust/](https://pragprog.com/titles/hwrust/hands-on-rust/)

视频介绍：[https://www.youtube.com/watch?v=DvcWrd5VJ2I](https://www.youtube.com/watch?v=DvcWrd5VJ2I)

## 在 R 语言中调用 Rust 

[https://extendr.github.io/rextendr/](https://extendr.github.io/rextendr/)

## TezEdge: 使用 nom 加速二进制解析

Tezos 是一个开源去中心化区块链网络，为智能合约和数字资产提供平台。 之前 Tezos 节点使用 serde 来序列化/反序列化二进制，但是这样始终维护着一个中间结构，占用了 CPU 和 内存。所以他们使用 nom 直接对二进制流进行解析，就消除来这个中间结构，提升了性能。

- [https://medium.com/tezedge/speeding-up-incoming-message-parsing-by-3-to-10-times-by-switching-from-serde-to-the-nom-library-a74b04391bb4](https://medium.com/tezedge/speeding-up-incoming-message-parsing-by-3-to-10-times-by-switching-from-serde-to-the-nom-library-a74b04391bb4)
- [https://github.com/tezedge/tezedge](https://github.com/tezedge/tezedge)