---
pub_date: Tue, 31 May 2021 21:00:00 GMT
description: Rust Lang

---

# Rust 官方动态

编辑： 张汉东

---

## Rust基金会成员开放申请了：

[https://foundation.rust-lang.org/info/become-a-member/](https://foundation.rust-lang.org/info/become-a-member/)

## Rust 1.52 稳定版发布

要点：

1. unsafe_op_in_unsafe_fn lint 稳定。当前 Rust 把 unsafe 函数本身当作一个 unsafe block，该lint的出现就是为来改变这种行为： 现在要求，unsafe函数中调用unsafe函数，必须再加一个 unsafe block。 该lint现在是 allow-by-default ，就是说，默认不会更改 当前行为，只有你用 #[deny(unsafe_op_in_unsafe_fn)] 或 #[warn(unsafe_op_in_unsafe_fn)] 才会允许更改当前行为。 
2. 允许可变数组引用直接转换为可变/不可变原生指针。
3. 编译器后端升级到 LLVM12
4.  现在所有的整数除法和余数操作都是 const fn 了
5. 稳定了 str/slice/char 类型的多个 API ，并且部分 API 都是 const fn 都了
6. Rustdoc markdown 现在支持 task list 了
7. cargo test 支持传入多个文件

兼容性变更：

1.  RUSTC_BOOTSTRAP 已经在 build.rs 中被禁止设置
2.  代码中要使用了 proc-macro-hack  可能会导致 panic，需要执行 cargo unpdate -p proc-macro-hack 解决此问题

[https://blog.rust-lang.org/2021/05/06/Rust-1.52.0.html](https://blog.rust-lang.org/2021/05/06/Rust-1.52.0.html)

## Rust 1.52.1 发布 

此版本主要解决增量编译中的一个错误，该错误在1.52.0中变成了编译器错误。

建议所有Rust用户，包括当前使用1.52.0之前的稳定版本的用户，升级到1.52.1或禁用增量编译。

目前只是一个临时的修复计划，目前此错误还未被完全修复，毕竟增量编译功能是刚需。主要是修复 verify-ich 相关 issues，目前还有很多 ：[https://github.com/rust-lang/rust/issues?q=is%3Aissue+is%3Aopen+unstable+fingerprints](https://github.com/rust-lang/rust/issues?q=is%3Aissue+is%3Aopen+unstable+fingerprints)

目前官方正在积极修复此错误。 Rust团队还将制定计划，以确保在将来有更好的bug跟踪系统，既可以防止此类情况再次发生，也可以通过更准确地跟踪bug来进一步提高我们版本的稳定性。 

[https://blog.rust-lang.org/2021/05/10/Rust-1.52.1.html](https://blog.rust-lang.org/2021/05/10/Rust-1.52.1.html)

## 官方宣布  Rust 2021 edition plan 

[https://blog.rust-lang.org/2021/05/11/edition-2021.html](https://blog.rust-lang.org/2021/05/11/edition-2021.html)

之前翻译版本 ：[https://mp.weixin.qq.com/s/C36k7_ZEcgpesAYmqDdV-w]( https://mp.weixin.qq.com/s/C36k7_ZEcgpesAYmqDdV-w)

P.S 大家要不要听听 Rust Edition 之歌:

[https://smallcultfollowing.com/babysteps/blog/2021/05/26/edition-the-song/](https://smallcultfollowing.com/babysteps/blog/2021/05/26/edition-the-song/)


## 新的 RFC 被合并： const-ub 

为了在 CTFE 的时候进行  UB-checking

[https://github.com/rust-lang/rfcs/blob/master/text/3016-const-ub.md](https://github.com/rust-lang/rfcs/blob/master/text/3016-const-ub.md)

## rustc 从 1.46 到  1.51  性能提升不少

有人在reddit上发帖，说他在 OpenBSD 平台上将 Rust 从1.46更新到1.51，发现他的项目编译时间减少了30％。

[https://www.reddit.com/r/rust/comments/n2lh7z/rustc_performance_improvement_from_rust_146_to_151/](https://www.reddit.com/r/rust/comments/n2lh7z/rustc_performance_improvement_from_rust_146_to_151/)

