---
pub_date: Sat, 27 Feb 2021 16:00:00 GMT
description: Rust chat room February summary

---

# 本月简报 | Rust 唠嗑室本月汇总

- 来源：[Rust 唠嗑室](https://space.bilibili.com/25566598)
- 主持人：MikeTang
- 后期编辑：高宪凤

---

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

<center><font color=#f56c6c>----------> Rust 牛年春晚 <----------</font></center>

### P1【4 点场】 Rust1.50 最新改动讲解

**嘉宾**：张汉东

张汉东老师以一段 `Rust` 宏代码开启欢乐的 Rust 牛年春晚。随后汉东老师着重讲解了这次 [Rust1.50 版本][rust1.50]更新的主要内容。这次更新主要包括： `语言级特性`、`编译器`， `标准库`、 `稳定的 API`、`Cargo 相关`、`其他`、`兼容性提示`几个方面。

<!-- - 语言级特性方面
  - 常量泛型`[CONST; N]`进一步得到完善
  - 将共用体(union)中`ManualDrop`类型字段的分配视为安全
- 编译器方面
  - 添加对`armv5te-unknown-linux-uclibcgnueabi`目标的内置支持
  - 在ARM Mac上添加对Arm64 Catalyst的支持
  - 修复 FreeBSD 上的链接问题
- 标准库方面
  - 为` proc_macro::Punct `增加 `PartialEq`
  - Unix 平台优化：Option<File> 大小等价于 File
- 兼容性变更
  - 过期 compare_and_swap 方法
  - 放弃对所有 cloudabi target 的支持 -->

[查看回放](https://www.bilibili.com/video/BV1pi4y1T78V?p=1)

**扩展资料**
1. 暖场代码
```Rust
macro_rules! m {
    ($($s:stmt)*) => {
        $(
            { stringify!($s); 1 }
        )<<*
    };
}

fn main() {
    print!(
        "{}{}{}",
        m! { return || true },
        m! { (return) || true },
        m! { {return} || true },
    );
}
```
2. [Rust 1.50 稳定版发布解读](https://mp.weixin.qq.com/s/s7MCqwJWk_Kh77tvLRCKwA)

<center><font color=#f56c6c>----------> Rust 牛年春晚 <----------</font></center>

### P2【5 点场】 Delay-Timer 分享

**嘉宾**：炮炮

[Delay-Timer](https://github.com/BinChengZhao/delay-timer) 是一个类似于管理周期性任务的库，目前支持同步、异步任务进行周期化交付，支持一些任务在调度过程中动态添加和动态提交任务的操作。炮炮老师分享了开发过程中的心路历程。

[查看回放](https://www.bilibili.com/video/BV1pi4y1T78V?p=2)

**扩展资料**：
1. 暖场代码
```Rust
fn main() {
    let a = 4;
    println!("{},{}", --a, --a);
}
```
2. [Delay-Timer](https://github.com/BinChengZhao/delay-timer)

<center><font color=#f56c6c>----------> Rust 牛年春晚 <----------</font></center>

### P3【5 点场】Libra 代码分析讲解

**嘉宾**：Shara

[Libra](https://developers.diem.com/docs/welcome-to-diem/) Facebook 开发的一个 Rust 区块链项目，它的使命是为全球数十亿人建立一个简单的全球货币和金融基础设施。Share 老师分享了分析 Libra 代码的思路。

[查看回放](https://www.bilibili.com/video/BV1pi4y1T78V?p=3)

**扩展资料**：
[Libra](https://developers.diem.com/docs/welcome-to-diem/)

<center><font color=#f56c6c>----------> Rust 牛年春晚 <----------</font></center>

### P4【6 点场】Rust 开发嵌入式烂苹果

**嘉宾**：王 Mono

王老师现场撸代码，使用 Rust 一步一步完成开发嵌入式烂苹果。

[查看回放](https://www.bilibili.com/video/BV1pi4y1T78V?p=4)

**扩展资料**
1. 暖场代码
```Rust
trait Trait {
    fn f(self);
}

impl<T> Trait for fn(T) {
    fn f(self) {
        print!("1");
    }
}

impl<T> Trait for fn(&T) {
    fn f(self) {
        print!("2");
    }
}

fn main() {
    let a: fn(_) = |_: u8| {};
    let b: fn(_) = |_: &u8| {};
    let c: fn(&_) = |_: &u8| {};
    a.f();
    b.f();
    c.f();
}
```
2. [Longan 文档](https://longan.sipeed.com/zh/)

<center><font color=#f56c6c>----------> Rust 牛年春晚 <----------</font></center>

### P5【8 点场】来自 go 社区大佬的视角

**嘉宾**：云喝酒

Go 和 Rust 作为两门新生语言，Go 的开发者人数大约是 Rust 的64倍。几位来自 Go 社区大佬以不同的视角一起聊聊。

[查看回放](https://www.bilibili.com/video/BV1pi4y1T78V?p=5)

**扩展资料**
1. Cloubhouse

<center><font color=#f56c6c>----------> Rust 牛年春晚 <----------</font></center>

### P6【9 点场】程序员的吉他课

**嘉宾**：MiskoLee

MiskoLee 老师现场教授弹吉他，妥妥的程序员吉他速成班。

[查看回放](https://www.bilibili.com/video/BV1pi4y1T78V?p=6)

<center><font color=#f56c6c>----------> Rust 牛年春晚 <----------</font></center>

### P7【9 点场】SNＭP 项目介绍

**嘉宾**：Robin

SNMP 是专门设计用于在 IP 网络管理网络节点（服务器、工作站、路由器、交换机及HUBS等）的一种标准协议，它是一种应用层协议。 SNMP 使网络管理员能够管理网络效能，发现并解决网络问题以及规划网络增长。通过 SNMP 接收随机消息（及事件报告）网络管理系统获知网络出现问题。Robin 老师分享 SNMP 在自己工作中实际应用。

[查看回放](https://www.bilibili.com/video/BV1pi4y1T78V?p=7)

<center><font color=#f56c6c>----------> Rust 牛年春晚 <----------</font></center>

### P8【10 点场】Maya-rs 分享

**嘉宾**：JungWoo

在 Maya 中运用 Rust 实现噪声效果的案例。原理：使用 Rust 调用 Python API，然后再将结果给到 Python API。

[查看回放](https://www.bilibili.com/video/BV1pi4y1T78V?p=8)

**扩展资料**
1. [Maya PolyNoise](https://github.com/Choi-Jungwoo/maya_poly_noise_rs)
2. [Maya帮助](http://help.autodesk.com/view/MAYAUL/2019/ENU/)

<center><font color=#f56c6c>----------> Rust 牛年春晚 <----------</font></center>

### P9【10 点场】关于数据库研究和开发的一些话

**嘉宾**：金明剑

金明剑老师结合自己实际经验聊了聊对 Rust 的理解，既有深度又有广度。

[查看回放](https://www.bilibili.com/video/BV1pi4y1T78V?p=9)

<center><font color=#f56c6c>----------> Rust 牛年春晚 <----------</font></center>

### P10【11 点场】wasm 与 rust 及 vitejs-rs 分享

**嘉宾**：夏歌&lencx

夏歌老师根据自己整理的 WebAssembly 生态图，对其整体状况进行简单介绍。

Lencx 老师现场演示，通过一个标准的 [Vite](https://vitejs.dev/) 脚手架开始项目，集成进 Rust，最后打包生成 Wasm 项目。

[查看回放](https://www.bilibili.com/video/BV1pi4y1T78V?p=10)

**扩展资料**
1. https://github.com/second-state/tencent-tensorflow-scf
2. https://mtc.nofwl.com/tech/post/wasm-start.html#rust
3. https://vitejs.dev/

[vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[into_boxed_slice]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.into_boxed_slice
[ref_cell]: https://doc.rust-lang.org/std/cell/struct.RefCell.html
[cell]: https://doc.rust-lang.org/core/cell/struct.Cell.html
[unsafe_cell]: https://doc.rust-lang.org/core/cell/struct.UnsafeCell.html
[rust1.50]: https://blog.rust-lang.org/2021/02/11/Rust-1.50.0.html