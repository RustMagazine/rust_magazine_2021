# Rust生态安全漏洞总结系列 | Part 4

本系列主要是分析[`RustSecurity` 安全数据库库](https://rustsec.org/advisories/)中记录的`Rust`生态社区中发现的安全问题，从中总结一些教训，学习`Rust`安全编程的经验。

本期主要是侧重于 [` Chrono`](https://github.com/chronotope/chrono) 里面的一个情况比较复杂的漏洞分析。

**TL;DR :**  请使用 `time` 代替  `Chrono` 。

---

##  问题

本月在 `Reddit Rust` 频道有人发布了一个帖子，在询问` Chrono` 库是否还有人在维护。之所以问出这个问题，是该贴作者看到 Rust 安全依赖报告中显式 `Chrono ` 有一个安全问题两年了还没有被修复，所以他不知道是怎么回事。

`Chrono` 遇到的安全问题在 [rustsec.org : RUSTSEC-2020-0159](https://rustsec.org/advisories/RUSTSEC-2020-0159.html) 有描述， 大概内容是：

1. `Chrono` 调用 `libc` 的 `localtime_r`，用于将时间转换为本地时间。
2. 而 `libc` 实现库中大多数都是直接调用 `getenv` 
3. 而 `setenv` 和 `getenv` 在 `libc` 中都不是线程安全的
4. 为了确保 `setenv` 的健全性，Rust 的 `libstd` 为其添加了一个锁
5. 这意味着在非 `libstd`下使用  `getenv` 将是不健全的。

这里面也涉及  Rust 中 `std::env` 的相关安全问题。 目前当你在并发环境下使用 `setenv`会存在数据竞争的问题，建议使用 互斥锁来处理，但只能防止  `std::env`。 [建议不要使用 `std::env::set_var`](https://github.com/rust-lang/rust/issues/90308)。对此问题更详细的总结参见 [[同步 FFI 访问 POSIX 环境变量函数](https://internals.rust-lang.org/t/synchronized-ffi-access-to-posix-environment-variable-functions/15475)](https://internals.rust-lang.org/t/synchronized-ffi-access-to-posix-environment-variable-functions/15475)

但是，直接调用 `libc` 的 `setenv` 和 `getenv` 是线程不安全的。

发现这个漏洞的是 [`tokio-rs/io-uring`](https://github.com/tokio-rs/io-uring) 作者 quininer。



## 解决方案：使用 `time`

`Chrono` 的作者现身回复：

1. 出于个人原因，目前已经离开了该项目，但该项目目前有人维护。
2. 这个安全漏洞目前有一些限制，所以无法修复。
   1. `Chrono` 必须支持系统本地时区（`Local::now()`）等。
   2. 该系统本地时区必须与 C 的 `localtime*` 返回的内容相匹配。

避免该漏洞有一个解决办法就是用 `time` 0.3 代替 `chrono`

最近几天  `chrono` 也发布了一个公告： [no time for chrono](https://passcod.name/technical/no-time-for-chrono.html) ，主要内容是：

1. `chrono` 用户可以切换到 `time` 0.3 
2. `localtime_r` 相当复杂，处理时区被认为是所有开发者的噩梦。
3. `time` 0.3 通过移除对  `localtime_r` 的调用来缓解此问题。
4. `Rich Felker`（`musl`的作者）有另一种观点。他认为，问题不在于调用 `localtime_r `函数，而在于修改环境。环境应该是不可改变的。

[Time 的 0.3 版本添加了许多 API](https://github.com/time-rs/time/blob/main/CHANGELOG.md#030-2021-07-30)，它们涵盖了 `Chrono` 的诸多 API：

- 无分配模式
- 该`Month`类型
- 日历/序数/ISO/儒略转换
- 大日期（超过 +/- 9999 年）
- 解析和 serde 支持

还有一些功能仅由较新的` Time` 支持， `Chrono` 则没有：

- `const` 函数
- `datetime!`在编译时构造日期时间的宏
- 序列化`non-ISO8601` 表示
- 随机日期/时间
- [快速检查](https://docs.rs/quickcheck)支持

因此，您现在可以合理地将 `Chrono` 替换为 `Time`！



## 相关链接

https://www.reddit.com/r/rust/comments/qamgyh/is_the_chrono_crate_unmaintained/

https://rustsec.org/advisories/RUSTSEC-2020-0159.html

https://github.com/chronotope/chrono/issues/499

https://passcod.name/technical/no-time-for-chrono.html