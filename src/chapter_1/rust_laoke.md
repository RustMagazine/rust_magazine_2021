# 本月简报 | Rust 唠嗑室本月汇总


- 来源：[Rust 唠嗑室](https://space.bilibili.com/25566598)
- 主持人：MikeTang

## 《Rust 唠嗑室》第 16 期 - tensorbase 高性能数据仓库

**时间**: 2021/01/05 20:30-21:30

**主讲人**：金明剑

**内容**：金明剑老师在 RustChinaConf2020 上分享了《[基于 Rust 构建高性能新型开源数据仓库](https://www.bilibili.com/video/BV1Yy4y1e7zR?p=25)》，很多人感兴趣 [Tensorbase](https://github.com/tensorbase/tensorbase) 的技术内幕，这次唠嗑室一起来聊 Tensorbase。

[查看回放](https://www.bilibili.com/video/BV1TA411H7ap)

**扩展资料**：

[RustChinaConf2020 大会合集](https://www.bilibili.com/video/BV1Yy4y1e7zR)

---

## 《Rust 唠嗑室》第 17 期 - 用 Rust 写 Protobuf 扩展

**时间**: 2021/01/19 20:30-21:30

**主讲人**：宁志伟

**内容**：

[Protocol Buffers](https://en.wikipedia.org/wiki/Protocol_Buffers) (简称 Protobuf ) ，是 Google 出品的序列化框架，与开发语言无关，和平台无关。具有体积小，速度快，扩展性好，与 [gRPC](https://en.wikipedia.org/wiki/GRPC) 搭配好，支持的语言多等特点，是目前应用最广泛的序列化框架。

[CITA-Cloud](https://github.com/cita-cloud) 是一个以区块链技术为基础，融合云原生技术的柔性集成开放平台。区块链部分提供了非常灵活的微服务架构，可以适应各种各样的企业应用场景。

CITA-Cloud 计划提供一个框架，方便用户自定义交易和区块等核心数据结构。使用 Protobuf 的扩展能力，用户只需用 Protobuf 描述数据结构，框架会自动生成相关代码，得到一个定制的区块链。

这次主要来聊聊 Protobuf 扩展的原理，以及 Rust 已有的相关的库。最后通过一个 [Demo](https://github.com/rink1969/proto_desc_printer) 展示如何使用 Rust 来写 Protobuf 扩展。

[查看回放](https://www.bilibili.com/video/BV1Ff4y1k7Bo)

**扩展资料**：

[CITA-Cloud](https://github.com/cita-cloud)

[CITA-Cloud 文档](https://cita-cloud-docs.readthedocs.io/zh_CN/latest/)

[cita_cloud_proto](https://github.com/cita-cloud/cita_cloud_proto)

[Dropbox-pb-jelly](https://github.com/dropbox/pb-jelly)

[rust-protobuf](https://github.com/stepancheg/rust-protobuf/)

[Prost](https://crates.io/crates/prost)

[Demo 代码](https://github.com/rink1969/proto_desc_printer)
