# 本月简报 | Rust 唠嗑室本月汇总

- 来源：[Rust 唠嗑室](https://space.bilibili.com/25566598)
- 主持人：MikeTang

## 《Rust 唠嗑室》第 18 期 - 剖析 Rust 的引用

**时间**: 2021/02/02 20:30-21:30

**主讲人**：舒乐之（Andy）

一网网络工程师，2018 年开始写 Rust，参与 ImmuxDB 不可变数据库和 ImmuxCompute 计算引擎的设计开发；曾用 C 开发比特币节点 tinybtc；曾任 Matters Lab 首席工程师，Web 前后端都写过。

**内容**：

这次的主要内容，是从零开始，解释 Rust 中「引用」的概念，以及一批与引用相关的概念：地址、指针、借用、切片、智能指针、胖指针、裸指针、所有权、生命周期、作用域等。

还会谈到一些关于 Rust 引用的问题，比如：

- 生命周期与作用域的关系是什么？
- 为什么 str 不会单独出现，总是以要靠引用（比如&str）使用？
- [Vec][vec] 有一个 [into_boxed_slice()][into_boxed_slice]方法 —— boxed slice 是什么，与 Vec 有什么区别？
- [RefCell][ref_cell]、[Cell][cell]、[UnsafeCell][unsafe_cell] 的区别是什么？什么时候用什么？

[查看回放](https://www.bilibili.com/video/BV15N411o7e4)

**扩展资料**：

- 官方文档

  - https://doc.rust-lang.org/stable/reference/types/pointer.html
  - https://doc.rust-lang.org/stable/reference/types/function-pointer.html
  - https://doc.rust-lang.org/nomicon/ownership.html
  - https://github.com/rust-lang/rfcs/blob/master/text/2094-nll.md
  - http://rust-lang.github.io/rfcs/1558-closure-to-fn-coercion.html
  - https://prev.rust-lang.org/en-US/faq.html#ownership
  - https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html

- 博客
  - http://smallcultfollowing.com/babysteps/blog/2014/05/13/focusing-on-ownership/
  - https://ricardomartins.cc/2016/06/25/interior-mutability-thread-safety
  - https://limpet.net/mbrubeck/2019/02/07/rust-a-unique-perspective.html
  - https://internals.rust-lang.org/t/function-pointers-are-inconsistent-with-other-language-features/12439

---

## Rust 牛年春晚

**时间**：2021/02/14 16:00 - 24:00

### 【4 点场】 Rust1.50 最新改动讲解

**嘉宾**：张汉东

[查看回放](https://www.bilibili.com/video/BV1pi4y1T78V?p=1)

### 【5 点场】 Delay-Timer 分享

**嘉宾**：炮炮

[查看回放](https://www.bilibili.com/video/BV1pi4y1T78V?p=2)

**扩展资料**：
[Delay-Timer](https://github.com/BinChengZhao/delay-timer)

### 【5 点场】Libra 代码分析讲解

**嘉宾**：Shara

[查看回放](https://www.bilibili.com/video/BV1pi4y1T78V?p=3)

### 【6 点场】Rust 开发嵌入式烂苹果

**嘉宾**：王 Mono

[查看回放](https://www.bilibili.com/video/BV1pi4y1T78V?p=4)

### 【8 点场】来自 go 社区大佬的视角

**嘉宾**：云喝酒

[查看回放](https://www.bilibili.com/video/BV1pi4y1T78V?p=5)

### 【9 点场】程序员的吉他课

**嘉宾**：MiskoLee

[查看回放](https://www.bilibili.com/video/BV1pi4y1T78V?p=6)

### 【9 点场】SNＭP 项目介绍

**嘉宾**：Robin

[查看回放](https://www.bilibili.com/video/BV1pi4y1T78V?p=7)

### 【10 点场】Maya-rs 分享

**嘉宾**：JungWoo

[查看回放](https://www.bilibili.com/video/BV1pi4y1T78V?p=8)

### 【10 点场】关于数据库研究和开发的一些话

**嘉宾**：金明剑

[查看回放](https://www.bilibili.com/video/BV1pi4y1T78V?p=9)

### 【11 点场】wasm 与 rust 及 vitejs-rs 分享

**嘉宾**：夏歌&lencx

[查看回放](https://www.bilibili.com/video/BV1pi4y1T78V?p=10)

[vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[into_boxed_slice]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.into_boxed_slice
[ref_cell]: https://doc.rust-lang.org/std/cell/struct.RefCell.html
[cell]: https://doc.rust-lang.org/core/cell/struct.Cell.html
[unsafe_cell]: https://doc.rust-lang.org/core/cell/struct.UnsafeCell.html
