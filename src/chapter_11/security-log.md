# 每月安全公告

来自[RustSec](https://rustsec.org/advisories/)和[GitHub 咨询](https://github.com/advisories?query=ecosystem%3Arust)。

---



## 11 月 

本月总结来自于  [https://rustinblockchain.org/newsletters/rib-newsletter-29/](https://rustinblockchain.org/newsletters/rib-newsletter-29/)

- [RUSTSEC-2021-0120：不健全](https://rustsec.org/advisories/RUSTSEC-2021-0120.html)。abomonation 在没有足够约束的情况下将` &T `转换为` &[u8]` 和从 `&[u8] `转换。
- **[RUSTSEC-2021-0121：crypto2 中的不健全](https://rustsec.org/advisories/RUSTSEC-2021-0121.html)。** Chacha20 加密解密中未对齐的 u32 读取。
- [CVE-2021-20319：coreos 安装程序在解压缩 gzipped artifact 时不正确地验证 GPG 签名](https://github.com/advisories/GHSA-3r3g-g73x-g593)。解压缩 gzip 压缩的工件时，coreos-installer 无法正确验证 GPG 签名。这允许在 coreos-installer 解压缩下载的 OS 映像的情况下绕过签名验证，允许可以修改 OS 映像的攻击者危及新安装的系统。
- **[CVE-2020-26281：Async-h1 请求可能带有较长的未读正文](https://github.com/advisories/GHSA-4vr9-8cjf-vf9c)。** 此漏洞影响任何在反向代理后面使用 async-h1 的网络服务器，包括所有此类 Tide 应用程序。
- **[CVE-2021-41138：Frontier 中缺少有效性检查](https://github.com/advisories/GHSA-vj62-g63v-f8mf)。** 在pallet-ethereum新引入的signed Frontier-specific extrinsic中，很大一部分交易验证逻辑只在交易池验证中调用，而在块执行中没有调用。恶意验证者可以利用这一点将无效交易放入区块中。
- [CVE-2021-41149：目标名称清理不当](https://github.com/advisories/GHSA-x3r5-q6mj-m485)。在 0.12.0 之前，tough 库在缓存存储库或将特定目标保存到输出目录时无法正确清理目标名称。当目标被缓存或保存时，文件可能会被系统上任何地方的任意内容覆盖。
- [CVE-2021-41150：委派角色名称清理不当](https://github.com/advisories/GHSA-r56q-vv3c-6g9c)。0.12.0 之前的强硬库在缓存存储库或从文件系统加载存储库时无法正确清理委派的角色名称。当存储库被缓存或加载时，以 .json 扩展名结尾的文件可能会被系统上任何地方的角色元数据覆盖。
- **[CVE-2021-41153：JUMPI 中的规范不合规](https://github.com/advisories/GHSA-pvh2-pj76-4m96)。** 在 evm crate < 0.31.0 中，在目标有效性检查之后检查 JUMPI 操作码的条件。然而，根据 Geth 和 OpenEthereum 的说法，条件检查应该发生在目的地有效性检查之前。
- **[GHSA-v935-pqmr-g8v9： num-bigint 中的意外恐慌](https://github.com/advisories/GHSA-v935-pqmr-g8v9)。** 报告了两种情况，其中 BigInt 和 BigUint 乘法可能会意外恐慌。