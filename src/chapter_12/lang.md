# 官方动态

聚焦 Rust 官方活动与新闻

---

### Rust 1.57 发布

通常来说，Rust每隔6个星期会发布一次新版本。Rust 1.56.0版本发布于2021年10月21日，约6周之后，Rust 1.57已于2021年12月2日发布。

新的稳定版本内容大致如下：

- `panic!`、`assert!`以及其他一些宏可以在`const fn`以及其他编译时上下文中使用了，不过参数仍然只支持`str`类型数据（e.g. `panic!("...")`）或是`&str`类型数据（e.g. `panic!("{}", a)`）

- Cargo profile支持命名后配置了，例如：

  ```
  [profile.production]
  inherits = "release"
  lto = true
  ```

  编译时可以通过`--profile production`来使用此配置。编译后文件会被放在不同的文件夹里（e.g. `target/production`）。

- 其他的一些从nightly版本加入稳定版本的API，例如`try_reverse`，详情可见"[Announcing Rust 1.57.0](https://blog.rust-lang.org/2021/12/02/Rust-1.57.0.html)"


##  Rust 异步改进| 可移植和可互操作的异步 Rust

异步基础工作组的目标是使异步 Rust 具有[可移植性和可互操作性](https://rust-lang.github.io/wg-async-foundations/vision/roadmap/portable.html)：

- 无论开发者使用什么运行时或其他库，都能够从 crates.io 获取库并自信地混合搭配它们
- 能够轻松地编写可与其他库组合且独立于运行时的库
- 能够在运行时之间轻松更改应用程序以探索新的可能性
- 能够轻松编写新的运行时来尝试新的执行策略或一些新的环境，并让它们与大多数现存的库互操作，而无需更改这些库
- 能够找到适合各种场景和使用模式的运行时

但是要达成这个目标，还有很多路要走。Rust 官方团队成员 nrc 现在带头发起这项倡议，并给出了他的愿景：

选择运行时不会让开发者承诺使用到生态系统的特定部分。几乎所有的库都应该与运行时的选择无关。。改变运行时应该是相对容易的，而且你应该能够开始使用异步编程，而不需要为寻找和选择运行时而烦恼。

nrc 并不是希望有一个大一统的终极的运行时执行器，这是不现实的，异步运行时根据不同场景有不同的内容。他所希望的是，能将异步生态系统中的很多抽象和实用程序标准化，这样异步代码就更容易移植。为了达到这个目的，nrc认为很多东西都应该放在标准库中，而不是与执行器捆绑在一起。

[https://www.ncameron.org/blog/portable-and-interoperable-async-rust/](https://www.ncameron.org/blog/portable-and-interoperable-async-rust/)


##  【官方】对 Rust 审核团队问题的后续跟进

对于关心 Rust 的 中文社区的朋友和技术媒体而言，我觉得没必要过度解读。因为我们不了解美国社会以及处于该社会下人们所关心和敏感的问题是什么，真正想去理解也是比较困难的。我们只知道，这是一个超过大多数公司人员规模且都是志愿者组成的开源组织所要面临和解决的问题，问题一旦经过解决，那么这个社区将得到进化，会更加强大。所以没必要担心什么 Rust 会被负面影响。

[详情请移步这里阅读更多](./rust-mod-team-follow-up.md)

## 使用 rust core库就能做risc-v的内核开发和宿主机（虚拟机）开发了

具体来说是core库支持了hypervisor和supervisor的扩展指令，顺便加了个is_riscv_feature_detcted!宏，能在用户态检测riscv扩展是否存在，
已经在真机上跑通，合理利用检测宏，能彻底解决“生态碎片”问题。
如果rollup顺利，能进明天的nightly，然后1.59的stable  

- 来自 @洛佳的 PR : [https://github.com/rust-lang/rust/pull/92574](https://github.com/rust-lang/rust/pull/92574  )
- （模块pr：[https://github.com/rust-lang/stdarch/pull/1271](https://github.com/rust-lang/stdarch/pull/1271)）