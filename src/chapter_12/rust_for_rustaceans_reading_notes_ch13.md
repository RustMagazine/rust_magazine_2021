# Rust For Rustaceans 读书笔记之 the rust ecosystem

作者: 吴翱翔

## cargo-deny

在字节的 monoio 库源码和众多开源库中都能看到该工具的配置文件 deny.toml

公司中有同事大力推荐引进项目，我的理解是集成了一堆工具例如 cargo-audit

其中 license 功能很有意思，分析项目所有依赖有没有开源许可冲突

打个比方 MIT2 和 MIT3 开源许可在法律上有冲突，
那么 cargo-deny 能帮你找出项目有没有依赖 A 是 MIT2 许可的同时依赖 B 是 MIT3 许可这样项目的开源许可会有法律风险

别小看项目开源许可的法律风险问题，例如 rust-lang/rust_codegen_gcc 库曾因为 libgccjit.so 的开源许可的法律问题让 Rust 基金会的律师进行处理，所以我估计 monoio 在开源前也是让字节的法务部帮忙检查下开源后有没有法律风险

## pin_project

pin_project 的作用在*酷熊(https://fasterthanli.me/)博客*上面好几篇文章介绍过，

我的印象是 pin_project 的作者是 tokio 成员而且异步和过程宏造诣很深，

社区的油条哥的 async_graphql::Object 和 poem_openapi::OpenApi 这两个过程宏
应该是参考了 pin_project 宏的一种设计模式/编程范式，可以让属性宏也能像 derive 宏一样在 "语法树父子节点" 间传递宏的上下文和元信息

## cargo tree --invert

例如通过 cargo-deny/cargo-audit 发现 chrono 有安全漏洞，想看看项目中哪些库引用了 chrono

如果用 cargo tree | grep chrono 去看依赖树很长，很难找

```
[w@ww polars]$ cargo tree | grep -B2 -A2 chrono
│   ├── anyhow v1.0.51
│   ├── arrow2 v0.8.1 (https://github.com/jorgecarleitao/arrow2?rev=89921d33bdb4becf8e9197fdc8392818862357cd#89921d33)
│   │   ├── chrono v0.4.19
│   │   │   ├── libc v0.2.112
│   │   │   ├── num-integer v0.1.44
```

如果加上 --invert 参数能将依赖树「颠倒过来」

```
[w@ww polars]$ cargo tree --invert --package chrono
chrono v0.4.19
├── arrow2 v0.8.1 (https://github.com/jorgecarleitao/arrow2?rev=89921d33bdb4becf8e9197fdc8392818862357cd#89921d33)
│   ├── polars-arrow v0.18.0 (/home/w/repos/clone_repos/polars/polars/polars-arrow)
│   │   ├── polars-core v0.18.0 (/home/w/repos/clone_repos/polars/polars/polars-core)
│   │   │   ├── polars v0.18.0 (/home/w/repos/clone_repos/polars/polars)
│   │   │   ├── polars-io v0.18.0 (/home/w/repos/clone_repos/polars/polars/polars-io)
│   │   │   │   ├── polars v0.18.0 (/home/w/repos/clone_repos/polars/polars)
│   │   │   │   └── polars-lazy v0.18.0 (/home/w/repos/clone_repos/polars/polars/polars-lazy)
│   │   │   │       └── polars v0.18.0 (/home/w/repos/clone_repos/polars/polars)
│   │   │   └── polars-lazy v0.18.0 (/home/w/repos/clone_repos/polars/polars/polars-lazy) (*)
│   │   ├── polars-io v0.18.0 (/home/w/repos/clone_repos/polars/polars/polars-io) (*)
│   │   ├── polars-lazy v0.18.0 (/home/w/repos/clone_repos/polars/polars/polars-lazy) (*)
│   │   └── polars-time v0.18.0 (/home/w/repos/clone_repos/polars/polars/polars-time)
│   │       └── polars-core v0.18.0 (/home/w/repos/clone_repos/polars/polars/polars-core) (*)
│   ├── polars-core v0.18.0 (/home/w/repos/clone_repos/polars/polars/polars-core) (*)
│   └── polars-io v0.18.0 (/home/w/repos/clone_repos/polars/polars/polars-io) (*)
└── polars-time v0.18.0 (/home/w/repos/clone_repos/polars/polars/polars-time) (*)
```

很容易发现项目中 polars-time 是直接依赖了 chrono
