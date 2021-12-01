# 区块链快讯

来自于区块链领域的信息，来自于 [https://rustinblockchain.org/newsletters/rib-newsletter-29/](https://rustinblockchain.org/newsletters/rib-newsletter-29/)

---

随着区块链世界的[总价值锁定（TVL）](https://coinmarketcap.com/alexandria/glossary/total-value-locked-tvl)测量不断上升，以太坊极度拥挤，流动性提供者逃往高收益和低费用的区块链，许多链上都付出了巨大的努力来快速推出 dapps 并吸引市场份额。在 Rust 世界中，有一些值得注意的发展：

- NEAR[推出了 8 亿美元的生态系统基金，](https://near.org/blog/near-announces-800-million-in-funding-initiatives-to-support-ecosystem-growth/)并一直在悄悄攀升 DeFi TVL 排行榜。

- Secret Network 现在可以连接到多个链，其[SecretSwap](https://www.secretswap.io/)具有足够的流动性，很快就会成为在新兴的多链生态系统之间谨慎转移资产的可行途径。

- Internet Computer（以前称为 DFINITY）一直在[发布](https://medium.com/dfinity-network-blog)有关技术问题和合作伙伴关系的[博客文章](https://medium.com/dfinity-network-blog)。他们正在运行一个开发者资助计划。

- Nervos 已经[启动了他们的 Force Bridge](https://www.nervos.org/blog/force-bridge-mainnet/)来连接到以太坊，并且正在努力增加[EVM 兼容性](https://docs.nervos.org/docs/essays/polyjuice)。

- Polkadot 一直在对其 Canary 网络 Kusama 上的[平行链插槽](https://polkadot.network/blog/kusama-batch-2-auctions-report/)进行高调[拍卖](https://polkadot.network/blog/kusama-batch-2-auctions-report/)，并将很快开始对主要 Polkadot 网络的拍卖。一些 Kusama 平行链，特别是[Moonriver](https://moonbeam.network/networks/moonriver/)，已经启动并运行并吸引了 Sushi 等与 EVM 兼容的 dapp。

- Solana 已经变成了一个拥有巨大资金支持的巨头，而 TVL 仅次于以太坊和币安智能链。它一直吸引着以前仅支持 EVM 的 dapp 和协议，例如[Lido](https://lido.fi/)和[RenVM](https://renproject.io/)；但它也有自己的 Solana 原生 dapp 的强大稳定。

  

 

## 项目聚焦

每个月我们都喜欢关注一个著名的 Rust 区块链项目。本月该项目是……

[mina-rs](https://github.com/ChainSafe/mina-rs) .

[Mina](https://minaprotocol.com/) 是一个新的区块链网络，它使用零知识证明来验证链的状态，而无需访问完整的区块链，而只是一个很小的 (~22k) 证明。这应该使甚至移动设备也能够作为完整的验证者参与网络，与当今大多数客户端连接到托管在云中的其他人的完整节点的情况相比，需要更少的信任。他们将这种风格的链称为“简洁区块链”，并引起了其他一些项目的注意，结成合作伙伴关系，将这种想法带到其他链上。

[mina-rs](https://github.com/ChainSafe/mina-rs)是 Mina 在 Rust 中的一个实现，由[ChainSafe](https://chainsafe.io/)开发。它的开发不仅考虑了移动环境，还考虑了 WASM，这表明我们将能够直接在浏览器中嵌入完整节点。

关于 Mina / mina-rs 的一些最新信息：

- [潮起潮落：Mina 协议如何使 Web 3.0 受益](https://medium.com/chainsafe-systems/mina-wasm-benefits-for-web-3-0-3d25991c3b75)
- [Mina：大规模去中心化加密货币（白皮书）](https://docs.minaprotocol.com/static/pdf/technicalWhitepaper.pdf)
- [22kB 大小的区块链——技术参考](https://minaprotocol.com/blog/22kb-sized-blockchain-a-technical-reference)
- [Mina 产品优先级和 Mina 基金会使命](https://minaprotocol.com/blog/mina-protocol-product-priorities-mina-foundation-mission)

 

## 有趣的东西

#### 博客文章

- [Rudra：在生态系统范围内寻找 Rust 中的内存安全漏洞](https://www.micahlerner.com/2021/10/31/rudra-finding-memory-safety-bugs-in-rust-at-the-ecosystem-scale.html)
- [STARK 的解剖](https://aszepieniec.github.io/stark-anatomy/)
- [FROST：灵活的回合优化 Schnorr 阈值签名](https://blog.coinbase.com/frost-flexible-round-optimized-schnorr-threshold-signatures-b2e950164ee1)
- [分片](https://adlrocha.substack.com/p/adlrocha-sharding)
- [反恐精英：阈值攻击](https://medium.com/velasblockchain/counter-strike-threshold-attack-87f3b456b1e0)
- [AppliedZKP zkEVM Circuit 代码指南](https://medium.com/@sin7y/sin7y-tech-review-12-a-guide-to-appliedzkp-zkevm-circuit-code-3e0691056b48)

#### 文件

- [Plomo：超轻区块链客户端](https://eprint.iacr.org/2021/1361)
- [未来的加密：向未来（匿名）委员会发送秘密信息的范式](https://eprint.iacr.org/2021/1423)
- [具有附加密钥派生和预签名的 ECDSA 的安全性](https://eprint.iacr.org/2021/1330)
- [如何证明 Schnorr 假设 Schnorr：多重和阈值签名的安全性](https://eprint.iacr.org/2021/1375)

#### 项目

- [Shade Protocol](https://github.com/securesecrets/shade) 是一系列建立在 Secret Network 上的连接隐私保护 dApp。

 