# 每月安全公告

来自[RustSec](https://rustsec.org/advisories/)和[GitHub 咨询](https://github.com/advisories?query=ecosystem%3Arust)。

---

## 12 月

每月安全公告，来自[RustSec](https://rustsec.org/advisories/)和[GitHub 公告](https://github.com/advisories?query=ecosystem%3Arust)。这里的粗体条目与区块链项目特别相关。

- **[RUSTSEC-2021-0122：`flatbuffers`](https://rustsec.org/advisories/RUSTSEC-2021-0122.html)。** 生成的代码可以在安全代码中越界读写。这是一个严重的网络暴露漏洞，此 crate 的用户必须采取行动。
- [RUSTSEC-2021-0125：`simple_asn1`](https://rustsec.org/advisories/RUSTSEC-2021-0125.html).
- [RUSTSEC-2021-0123：`fruity`](https://rustsec.org/advisories/RUSTSEC-2021-0123.html)。转换`NSString`为字符串会在空字节处截断。
- **[RUSTSEC-2021-0124：`tokio`](https://rustsec.org/advisories/RUSTSEC-2021-0124.html)。** 关闭 oneshot 通道后发送和接收时的数据竞争。
- [RUSTSEC-2021-0126：rust-embed ](https://rustsec.org/advisories/RUSTSEC-2021-0126.html)。RustEmbed 生成的`get`方法允许在从磁盘读取文件时进行目录遍历。
- [CVE-2021-3917：`coreos-installer`< 0.10.0 ](https://github.com/advisories/GHSA-862g-9h5m-m3qv)。
- [CVE-2021-43174：`routinator`内存耗尽](https://github.com/advisories/GHSA-6mv9-qcx2-3hh3)。
- [CVE-2021-43790：`lucet`](https://github.com/advisories/GHSA-hf79-8hjp-rrvq).