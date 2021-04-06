---
pub_date: Sat, 27 Mar 2021 16:00:00 GMT
description: Exploring the origin of the system through Rust Security

---

## 透过 Rust 探索系统的本原：安全篇

作者：陈天 / 后期编辑：[NiZerin](https://github.com/NiZerin)

> 原文链接：[https://mp.weixin.qq.com/s/HCHYr5sWnEG_qOGE3hfNnQ](https://mp.weixin.qq.com/s/HCHYr5sWnEG_qOGE3hfNnQ)

---

安全是我的老本行，隔一段时间不拉出来谈一谈就不舒服。我个人觉得：做应用不谈安全都是在耍流氓。

按照 CISSP[1] 的定义，安全有八大领域：

![](https://oss.iacblog.com/rust/rust-to-system-essence-safety/1.webp)

本文只关注 Communication and Network Security 中 TCP/IP 范畴下的 Session Layer Security，也就是 TCP/UDP 层之上的安全方案：

![](https://oss.iacblog.com/rust/rust-to-system-essence-safety/2.webp)

目前业界首选的方案是 TLS[2]。在所有流行的应用层解决方案中，都离不开 TLS。

在 p2p 领域，TLS 并不那么受待见，大家似乎更喜欢和 TLS 提供了同等安全水平，但更加去中心化（不需要 CA[3]）的 noise protocol[4]。我去年写过一篇关于 Noise protocol 的文章：[Noise 框架：构建安全协议的蓝图](http://mp.weixin.qq.com/s?__biz=MzA3NDM0ODQwMw==&mid=2649828386&idx=1&sn=555e16a024e027f6cde350a0a09d3af4&chksm=8704ae3eb0732728040c309dbd4200c93483f6f40b1ac86dc7d8810f53290d870657b89ea5ce&scene=21#wechat_redirect)。

本文围绕 TLS 和 Noise protocol，以及它们在 Rust 下的使用场景，谈谈我们如何做安全的系统和应用。

## 安全的本质

很多人谈到安全，首先想到的是加密解密。加解密只解决了安全范畴的机密性（confidentialilty）的问题，但它没有触及另外两个重要的范畴：完整性（integrity）和可用性（availability）。我们简单讲一下这三个概念：

- 机密性：信息在使用和传输的过程中以密文保存，只有授权的用户才可以获取到明文。

- 完整性：信息在使用和传输的过程中，不会被非法授权和破坏。

- 可用性：合法用户对信息的访问不会被拒绝。

为了保证可用性，我们会提供服务的高可用性（防止 DoS 以及服务故障），做数据冗余和灾备处理（防止数据丢失），监控，故障转移等等。

为了保证完整性，我们会使用哈希算法，数字签名来保证数据的一致性。

为了保证机密性，我们会使用对称和非对称加密来保证在传输途中，以及在数据载体上的机密性。机密性往往需要完整性作为先决条件；而完整性不一定需要机密性作为先决条件。

下图阐述了安全领域涉及机密性和完整性的主要算法：

![](https://oss.iacblog.com/rust/rust-to-system-essence-safety/3.webp)

注意，这里的一些算法是泛称，而非具体某种算法。比如：sha3 算法族下面除了 keccak 以外，还有 blake2，blake3 等其他算法；而 ECC 算法下面，属于签名算法的有 Ed25519，Ed448 等，属于非对称加密的有 x25519，x448 等。

如果你看了我前几周的文章，大概对我介绍的《[胖客户端，廋服务器](http://mp.weixin.qq.com/s?__biz=MzA3NDM0ODQwMw==&mid=2649828843&idx=1&sn=f60193eba1debfdbab4d9ea48ddf8203&chksm=8704aff7b07326e1c265b21c1617c436e2ab4b4f8911c9b7b64cc8b9f5d34e993aa875090e87&scene=21#wechat_redirect)》有些印象。文章中我提到了服务端把用户端的事件写入到事件日志中，客户端可以 clone / pull 这些事件日志，在本地生成相应的状态。那么问题来了，客户端怎么知道 clone 下来的事件日志是未经第三方篡改的事件日志呢？很简单，我们只需对日志文件做个 hash，然后用服务器的私钥对这个 hash 做一个签名，把签名附带在文件头上。这样客户端收到文件后，用服务器的公钥验证这个签名即可。这样，只要服务器的私钥不泄露，这个签名就可以验证文件的完整性。在比特币的世界里，每个区块的完整性都由打包该区块的矿工的签名来保证，而整个链的完整性则由哈希链和中本聪共识（最长链）保证。

进一步的，如果我们用户的私有 repo 下的所有事件日志都只有用户自己才能访问，其他人（包括服务端应用）都无法访问，那么我们可以用用户的公钥来加密 repo 的所有事件日志。

## DH 算法：互联网安全的基石

当我们需要保证存储在媒介上的信息的安全性时，一切都简单而直观；但当我们需要保证在网络传输中的实时信息的安全性时，我们就面临着巨大的挑战。

这其中第一个挑战就是：每个连接使用什么密钥来加密数据？

我们知道，在网络传输中，非对称加密的效率不高，不适合大量数据的加密，而对称加密则需要双方共享密钥，才能正常通讯。因此，我们需要一种手段，在不安全的网络信道中，只传输部分信息，通过这部分信息 + 本地的私有信息，协商出来双方一致的共享密钥。第三方即便获得明文传输的信息，也无法推导出密钥。如果这样的手段行得通，那么，我们就可以在网络通讯的握手过程，生成每个 session 独立的共享密钥（session key），后续的通讯都通过这个（这对）密钥来加密完成。这个协商的过程就是 DH 算法（Diffie-Hellman Key Exchange Algorithm）[5]（对算法细节感兴趣的可以去看 wikipedia）：

![](https://oss.iacblog.com/rust/rust-to-system-essence-safety/4.webp)

DH 算法是 TLS 以及 Noise protocol 的基石。如果没有它，我们就不会有目前这样一个繁荣且安全的互联网世界。

在 Rust 下，如果你需要直接使用 DH 算法，可以使用 dalek 出品的 `x25519-dalek`[6]。它是使用 curve25519 [7] 的 ECDH（Elliptic Curve Diffie-Hellman） 的实现。代码如下：

```rust
use rand_core::OsRng;
use x25519_dalek::{EphemeralSecret, PublicKey};
let alice_secret = EphemeralSecret::new(OsRng);
let alice_public = PublicKey::from(&alice_secret);
let bob_secret = EphemeralSecret::new(OsRng);
let bob_public = PublicKey::from(&bob_secret);
let alice_shared_secret = alice_secret.diffie_hellman(&bob_public);
let bob_shared_secret = bob_secret.diffie_hellman(&alice_public);
assert_eq!(alice_shared_secret.as_bytes(), bob_shared_secret.as_bytes());
```

你也许会问：我又不去实现 TLS 或者类似的加密协议，而我自己的网络传输都靠 TLS 保护着呢，DH 对我来说有什么用呢？

我能想到的一个场景是文件加密。在本文开头，我说：

> 进一步的，如果我们用户的私有 repo 下的所有事件日志都只有用户自己才能访问，其他人（包括服务端应用）都无法访问，那么我们可以用用户的公钥来加密 repo 的所有事件日志。

这个方案的缺点是效率太低 — 如果需要加密的文件有几个 G，非对称加密显然不好。但我们又没法用对称加密：毕竟我们不能跟每个人都预先共享一组密钥（共享密钥本身又存在安全风险）。这时，我们可以用 DH 算法生成一个只对这个文件有效的密钥，加密文件，然后在文件头提供必要的信息即可：

1. 生成一个临时公钥对

1. 用私钥和用户的公钥算 DH key

1. 用 DH key 作为 AES[8] 或者 ChachaPoly[9] 的密钥，加密文件

1. 在文件头添加临时生成的公钥

这样，在解密端，用户可以用自己的私钥和文件中携带的公钥算出 DH key，然后解密之。

如果大家对这个思路感兴趣，可以参考我用 Noise protocol 做的类似的解决方案：tyrchen/conceal[10]。

除了 x25519-dalek 外，ristretto255-dh[11] 也值得一试，它是 zcash 实现的 Ristretto255[12] DH 算法。

## TLS：不仅仅是 HTTP 的安全防线

作为 HTTPS 的安全协议的唯一选择，相信大家对 TLS（以及它的前身 SSL）有一定的了解 — 起码知道：

- 服务端需要安装经过合法 CA 签署的证书（certificate）。如果你配过 nginx，你还会知道，证书和证书的私钥一般都是 PEM [13] 格式存储在文件系统的。一般来说，除了配置自己的证书外，还需要配置整个服务器证书链以便客户端验证。

- 客户端在连接服务器时，会获取服务器证书，然后通过操作系统受信的根证书来验证服务器的证书以及签署该证书的 CA，以及签署该 CA 的上一级 CA 等形成的整个证书链可以由某个根证书验证。

- 客户端验证了服务器的证书后，会跟服务器交互建立一个安全信道，保证之后的传输的安全。这个过程是 TLS 握手。

- 之后整个 HTTP 协议交互的过程，都被这个安全信道保护起来（说人话就是加密了），第三方无法嗅探到内部的交互，也无法破坏其完整性。

如果你经常调试（或者逆向工程）HTTPS，你大概还知道：

- 通过 Charles Proxy 这样的工具，可以做 Man-In-The-Middle[14]，来截获加密的数据。使用 Charles Proxy 时，需要在操作系统级「信任」其根证书，这是证书验证的流程所决定的。一旦某个根证书被系统信任，那么它可以签署任何域名的证书，这样第三方就可以伪装成目标服务器，terminate 客户端到服务器的任何 TLS 流量，然后再伪装成客户端，向实际服务器发送数据。所以，不要轻易信任来路不明的根证书。

- 如果要避免 Charles Proxy 等工具做 Man-In-The-Middle，你可以使用 certificate pinning。

你大概率不知道：

- TLS 支持 client certificate - 也就是说不光客户端可以验证服务器是否是我要连的服务器；服务器也可以验证客户端是否是我（的 CA）签署的客户端。

- 客户端验证服务器时，除了可以通过系统的根证书验证，也可以预置一个 CA 证书来验证服务器的证书是否由该 CA 签署。

### 证书是个什么鬼？

我们这里所说的证书，是 PKI 体系下的 X.509 证书[16]。X.509 证书用于证明一个公钥的身份。我说我是 domain.com 的合法服务器，何以见得？我生成一对私钥和公钥，通过其签署一个 CSR（Certificate Signing Request [17]），里面通过 CN（Common Name）声索我对 `*.domain.com` 的占有。一般一个 CSR 包含这些信息：

![](https://oss.iacblog.com/rust/rust-to-system-essence-safety/5.webp)

随后我把 CSR 提交给一个由某个根证书签署的 CA，由其签名并发回给我。这样，任何应用通过 HTTPS 连接 domain.com 时就可以正常通讯。

在 letsencrypt[18] 成为主流之前，证书是个几乎相当于特许经营的好生意。像 godaddy 这样的家伙，一个证书可以卖上百美金一年，简直如同抢钱。证书作为一门生意，极大地破坏了互联网的安全性，很多小的玩家不想支付每年的证书费用，干脆就避免使用 HTTPS。letsencrypt 的出现，几乎摧毁了各大吃相难看的 CA 的生意。Letsencrypt 自动化了证书的申请流程，只要你能把某个域名指向你的服务器，让 letsencrypt 验证到你请求的域名就是你拥有的域名，可以立即签署一个有效期是 3 个月的免费证书。至于证书的有效期为啥不能更长，这个，根本不是技术原因，我猜是来自做垂死挣扎的既得利益者们的压力。

### 能不能自己做 CA？

CA 机构是 internet 的不可或缺，却相对脆弱的一环。Letsencrypt 只是解决了证书收费的问题，不过没有解决 CA 机构本身的脆弱性 — 任何一个中心化的，可以签署证书，被数亿设备信任的机构都是有安全风险的，因为黑客随时盯着这些机构的漏洞。一旦一个CA 被攻陷，黑客们可以伪造出成千上万的域名的服务器证书。

有没有可能一个应用的客户端和服务器使用自己的 CA，绕过任何 CA 机构？

当然可以。你可以生成自己的 CA cert（自签名），然后用 CA key 签名 Server cert。你的客户端在启动 TLS 连接时，信任你自己的 CA cert 即可。你甚至还可以通过你的 CA 给每个客户端签名，让服务器也同时验证客户端是你信任的客户端。如下图所示：

![](https://oss.iacblog.com/rust/rust-to-system-essence-safety/6.webp)

一个新的客户端在注册新用户/登录时，服务器会从 CA 获取证书，连同用户登录获得的 token 一同返回给客户端。之后，客户端访问任何服务端的 API（除 auth 之外），都需要提供 client cert 供服务器验证，这样，额外增加安全性，并且，可以杜绝 Charles Proxy 这样的中间人。

当然这样的手段只适合客户端代码由你自己控制（比如 iOS/android/OSX/Linux/Windows app）。你无法让你的服务器证书通过浏览器的安全验证（因为证书不在系统根证书的信任列表中），因而，任何使用浏览器访问你的服务器的用户将无法使用你的服务。

如果你对这样的方案感兴趣，可以看看我的 crate: tyrchen/cellar[19]。它借鉴比特币分层钱包[20]的设计，可以从一个 passphrase 衍生出确定的分层密码/密钥/证书。生成的证书可以被应用在 TLS 应用中，比如：tyrchen/mobc-tonic[21]（我做的一个 grpc client connection pool）。

下面是我通过 celllar 生成的 CA 证书（注意 `CA: TRUE`）：

![](https://oss.iacblog.com/rust/rust-to-system-essence-safety/7.webp)

以及该 CA 签署的服务器证书（注意 `CA: FALSE` 和 `TLS Web Server Authentication`）：

![](https://oss.iacblog.com/rust/rust-to-system-essence-safety/8.webp)

以及该 CA 签署的客户端证书（注意 `CA: FALSE` 以及 `TLS Web Client Authentication`）：

![](https://oss.iacblog.com/rust/rust-to-system-essence-safety/9.webp)

将 TLS 应用在 HTTP 之外

TLS 可以保护我们的 HTTP 应用，其中包括 REST/GraphQL/Websocket API，以及 gRPC API。虽然目前 HTTP 是几乎绝大多数互联网应用使用的协议，但还有大量的其它基于 TCP 层的协议。如果你要保证这些协议的安全性，使用 TLS 是一个简单直接的选择。

然而，理解和使用好 OpenSSL 库不是一件容易的事情。十多年前，我曾经用 C 语言和老旧的 OpenSSL （0.9.x）打过交道，那体验**相当**不好。Python / Erlang 有不错的 OpenSSL 的封装，在应用中使用 TLS 比较舒服自然。如果你熟悉的语言没有很好的库去包装 OpenSSL，那么，在应用中使用 TLS 就不那么容易。

在 Rust 里，除了 OpenSSL 的封装，我们还有 Rustls[22]。它是一个经过了 security auditing[23] 的 TLS 安全裤，性能比 OpenSSL 更好，且理论上更加安全（没有遗留的历史问题，没有 TLS1.1 及以下的不安全代码，没有遗留的不安全的加密算法，比如 RC4，3DES）。

Rustls 虽然比 OpenSSL 容易使用，但成功建立起 TLS 连接，还需要更多对 TLS 细节的理解。为此，我做了一个 crate：tokio-tls-helper[24]，可以让你通过简单的配置，创建 TLS connector (client) 和 acceptor (server)。

比如客户端使用自定义的 CA cert 以及通过自定义 CA 签署的 client cert：

```rust
domain = "localhost"

[cert]
pem = """-----BEGIN CERTIFICATE-----
MIIBeTCCASugAwIBAgIBKjAFBgMrZXAwNzELMAkGA1UEBgwCVVMxFDASBgNVBAoM
C0RvbWFpbiBJbmMuMRIwEAYDVQQDDAlEb21haW4gQ0EwHhcNMjEwMzE0MTg0NTU2
WhcNMzEwMzEyMTg0NTU2WjA3MQswCQYDVQQGDAJVUzEUMBIGA1UECgwLRG9tYWlu
IEluYy4xEjAQBgNVBAMMCURvbWFpbiBDQTAqMAUGAytlcAMhAAzhorM9IPsXjBTx
ZxykGl5xZrsj3X2XqKjaAVutnf7po1wwWjAUBgNVHREEDTALgglsb2NhbGhvc3Qw
HQYDVR0OBBYEFD+NqChBZDOs5FMgefHJSIWiRTHXMBIGA1UdEwEB/wQIMAYBAf8C
ARAwDwYDVR0PAQH/BAUDAwcGADAFBgMrZXADQQA9sligQcYGaBqTxR1+JadSelMK
Wp35+yhVVuu4PTL18kWdU819w3cVlRe/GHt+jjlbk1i22TvfO5AaNmdxySkO
-----END CERTIFICATE-----"""

[identity]
key = """-----BEGIN PRIVATE KEY-----
MFMCAQEwBQYDK2VwBCIEIArjJtHm3xb4aX3fsGHpuB8dD3yzcLxWcPCqy7AGtTG5
oSMDIQD/38MZBnYuyitIGU3ltOGwwDwtB6KYag4rL1zsSGTzYg==
-----END PRIVATE KEY-----"""

[identity.cert]
pem = """-----BEGIN CERTIFICATE-----
MIIBZDCCARagAwIBAgIBKjAFBgMrZXAwNzELMAkGA1UEBgwCVVMxFDASBgNVBAoM
C0RvbWFpbiBJbmMuMRIwEAYDVQQDDAlEb21haW4gQ0EwHhcNMjEwMzE0MTg0NTU2
WhcNMjEwOTEwMTg0NTU2WjAyMQswCQYDVQQGDAJVUzEQMA4GA1UECgwHYW5kcm9p
ZDERMA8GA1UEAwwIYWJjZDEyMzQwKjAFBgMrZXADIQD/38MZBnYuyitIGU3ltOGw
wDwtB6KYag4rL1zsSGTzYqNMMEowFAYDVR0RBA0wC4IJbG9jYWxob3N0MBMGA1Ud
JQQMMAoGCCsGAQUFBwMCMAwGA1UdEwQFMAMBAQAwDwYDVR0PAQH/BAUDAwfgADAF
BgMrZXADQQCKhph1Z3g6E+EULUi5yIROSXmMxWjzi+L1OmqNh9ANJlrQwlcfwq0G
8JbfGVwq1sotEI83mv42mWkSSX98uysO
-----END CERTIFICATE-----"""
```

有了这个配置，客户端可以生成 `ClientTlsConfig`，然后生成 `connector`，在建立好 TCP stream 后，直接调用 `connector.connect(stream)` 就可以将 TCP 连接升级为 TLS 连接，之后可以在其之上进行应用层的协议：

```rust
let msg = b"Hello world\n";
let mut buf = [0; 12];

let config: ClientTlsConfig = toml::from_str(config_file).unwrap();
let connector = config.tls_connector(Uri::from_static("localhost")).unwrap();

let stream = TcpStream::connect(addr).await.unwrap();
let mut stream = connector.connect(stream).await.unwrap();
info!("client: TLS conn established");

stream.write_all(msg).await.unwrap();

info!("client: send data");

let (mut reader, _writer) = split(stream);

reader.read_exact(buf).await.unwrap();

info!("client: read echoed data");
```

服务端的使用也很简单：配置相同的 CA cert，以及服务器的 server/key：

```rust
[identity]
key = """-----BEGIN PRIVATE KEY-----
MFMCAQEwBQYDK2VwBCIEII0kozd0PJsbNfNUS/oqI/Q/enDiLwmdw+JUnTLpR9xs
oSMDIQAtkhJiFdF9SYBIMcLikWPRIgca/Rz9ngIgd6HuG6HI3g==
-----END PRIVATE KEY-----"""

[identity.cert]
pem = """-----BEGIN CERTIFICATE-----
MIIBazCCAR2gAwIBAgIBKjAFBgMrZXAwNzELMAkGA1UEBgwCVVMxFDASBgNVBAoM
C0RvbWFpbiBJbmMuMRIwEAYDVQQDDAlEb21haW4gQ0EwHhcNMjEwMzE0MTg0NTU2
WhcNMjIwMzE0MTg0NTU2WjA5MQswCQYDVQQGDAJVUzEUMBIGA1UECgwLRG9tYWlu
IEluYy4xFDASBgNVBAMMC0dSUEMgU2VydmVyMCowBQYDK2VwAyEALZISYhXRfUmA
SDHC4pFj0SIHGv0c/Z4CIHeh7huhyN6jTDBKMBQGA1UdEQQNMAuCCWxvY2FsaG9z
dDATBgNVHSUEDDAKBggrBgEFBQcDATAMBgNVHRMEBTADAQEAMA8GA1UdDwEB/wQF
AwMH4AAwBQYDK2VwA0EAy7EOIZp73XtcqaSopqDGWU7Umi4DVvIgjmY6qbJZP0sj
ExGdaVq/7MOlZl1I+vY7G0NSZWZIUilX0CoOkrn0DA==
-----END CERTIFICATE-----"""


[client_ca_root]
pem = """-----BEGIN CERTIFICATE-----
MIIBeTCCASugAwIBAgIBKjAFBgMrZXAwNzELMAkGA1UEBgwCVVMxFDASBgNVBAoM
C0RvbWFpbiBJbmMuMRIwEAYDVQQDDAlEb21haW4gQ0EwHhcNMjEwMzE0MTg0NTU2
WhcNMzEwMzEyMTg0NTU2WjA3MQswCQYDVQQGDAJVUzEUMBIGA1UECgwLRG9tYWlu
IEluYy4xEjAQBgNVBAMMCURvbWFpbiBDQTAqMAUGAytlcAMhAAzhorM9IPsXjBTx
ZxykGl5xZrsj3X2XqKjaAVutnf7po1wwWjAUBgNVHREEDTALgglsb2NhbGhvc3Qw
HQYDVR0OBBYEFD+NqChBZDOs5FMgefHJSIWiRTHXMBIGA1UdEwEB/wQIMAYBAf8C
ARAwDwYDVR0PAQH/BAUDAwcGADAFBgMrZXADQQA9sligQcYGaBqTxR1+JadSelMK
Wp35+yhVVuu4PTL18kWdU819w3cVlRe/GHt+jjlbk1i22TvfO5AaNmdxySkO
-----END CERTIFICATE-----"""
```

服务端同样可以通过配置生成 `ServerTlsConfig`，然后生成 `acceptor`，之后正常使用 TCP listener accept 一个 TCP stream 后，就可以通过 `acceptor.accept(stream)` 把 TCP 连接升级为 TLS。这个过程配合客户端的 `connector.connect(stream)`，共同完成前面所说的 DH 过程，协商出来 session key，然后开始加密/解密应用层的数据：

```rust
let config: ServerTlsConfig = toml::from_str(config_file).unwrap();
let acceptor = config.tls_acceptor().unwrap();
let listener = TcpListener::bind(addr).await.unwrap();
tokio::spawn(async move {
    loop {
        let (stream, peer_addr) = listener.accept().await.unwrap();
        let stream = acceptor.accept(stream).await.unwrap();
        info!("server: Accepted client conn with TLS");

        let fut = async move {
            let (mut reader, mut writer) = split(stream);
            let n = copy(&mut reader, &mut writer).await?;
            writer.flush().await?;
            debug!("Echo: {} - {}", peer_addr, n);
        }

        tokio::spawn(async move {
            if let Err(err) = fut.await {
                error!("{:?}", err);
            }
        });
    }
});
```

## Noise Protocol：狂野西部的守护者

如果你没看过我之前的文章，大概率 Noise Protocol 对你来说是个陌生的名字。如果你搭过各种各样的梯子，你也许使用过 Wireguard[25]，那么恭喜你，你已经在不知不觉中使用 Noise Protocol 了 — 因为 Wireguard 在安全层使用了 Noise Protocol。我曾经写过一篇文章：[Wireguard：简约之美](http://mp.weixin.qq.com/s?__biz=MzA3NDM0ODQwMw==&mid=2649828356&idx=1&sn=0cfcf0de0a6a3c1fe9d1fc8d9e7df5f1&chksm=8704ae18b073270e2e7ced09f29846ebc38affed8cb023410f995ee829ceedaddaa7b80cadbe&scene=21#wechat_redirect)，介绍了 Wireguard 这个非常牛逼的 VPN 工具。

因为之前的关于 Wireguard 和 Noise protocol 的文章对 Noise Protocol 本身已经有足够丰富的介绍，这里我就不再赘述 Noise Protocol 的细节。

如果说 TLS 是出身高贵的正规军，那么 Noise Protocol 就是出身草根的土八路。但二者其实互相借鉴，互相学习。TLS 1.3 和 Noise Protocol 的主要区别有两个：

1. 在身份验证方面二者走上了不同的道路（TLS 1.3 使用证书，而 Noise Protocol 完全不使用）

1. 通讯过程中使用的算法一个走协商（TLS）一个走预配置（Noise）

走协商还是走配置这跟协议的使用场景有关。TLS 关注的是大量不同版本的标准客户端（比如 Firefox）和服务器之间的交互，两端支持的算法族可能有不小的出入，协商出双方都能接受的算法是最优的选择，这样可以支持尽可能广的应用场景；而 Noise Protocol 关注的是定制的客户端和服务器之间的交互，因而两端可以通过预配置来确定使用的算法。比如 WireGuard 使用 `Noise_IKpsk2_25519_ChaChaPoly_BLAKE2s`，那么客户端和服务端都需要：

- Curve 25519 做 ECDH

- CharChaPoly 做加密

- Blake2s 做哈希

- 两端使用 pre-shared key 进一步保证安全性

因为避免使用证书这样天然具有中心化的东西，Noise Protocol 在 p2p 领域走出了新的天地。从最简单的 NN（双方都没有固定公钥）KK（双方都知道对方的固定公钥），到最复杂的 XX（双方都有固定公钥，通过网络加密传输给对方），Noise Protocol 定义了 12 种协商模式，再辅以不同的哈希和加密算法，可以组合出上百种安全方案，总有一种适合你：

![](https://oss.iacblog.com/rust/rust-to-system-essence-safety/10.webp)

在 Rust 下，snow[26] 是 Noise Protocol 的非常成熟的实现，而 libp2p 则使用了 snow 来实现 libp2p 协议的安全层。

下面是使用 snow 在 TCP 协议之上建立加密信道的实例。我们可以看到，仅需额外的几行代码就可以将你的网络应用打造得非常安全：

- 创建 snow Builder

- 在建立连接后发送和接收不超过 3 个 Noise protocol 协议报文

- 协议握手完成后，使用 `noise.into_transport_mode()` 将 snow 状态机切换到传输模式

- 之后收到报文后调用 `noise.read_message()` 解密，发送报文前调用 `noise.write_message()` 加密即可。

服务器：

```rust
let params: NoiseParams = "Noise_XX_25519_ChaChaPoly_BLAKE2s".parse().unwrap();
let builder: Builder<'_> = Builder::new(params.clone());
let static_key = builder.generate_keypair().unwrap().private;
let mut noise = builder
    .local_private_key(&static_key)
    .build_responder()
    .unwrap();

// wait on client's arrival
println!("Listening on 0.0.0.0:9999");
let (mut stream, _) = TcpListener::bind("0.0.0.0:9999").unwrap().accept().unwrap();

// <- e
noise
    .read_message(&recv(&mut stream).unwrap(), &mut buf)
    .unwrap();

// -> e, ee, s, es
let len = noise.write_message(&[0u8; 0], &mut buf).unwrap();
send(&mut stream, &buf[..len]);

// <- s, se
noise
    .read_message(&recv(&mut stream).unwrap(), &mut buf)
    .unwrap();

// transition the state machine to transport mode sinc handshake is complete.
let mut noise = noise.into_transport_mode().unwrap();
while let Ok(msg) = recv(&mut stream) {
    let len = noise.read_message(&msg, &mut buf).unwrap();
    println!("client said: {}", String::from_utf8_lossy(&buf[..len]));
}

println!("connection closed");
```

客户端：

```rust
let params: NoiseParams = "Noise_XX_25519_ChaChaPoly_BLAKE2s".parse().unwrap();
let builder: Builder<'_> = Builder::new(params.clone());
let static_key = builder.generate_keypair().unwrap().private;
let mut noise = builder
    .local_private_key(&static_key)
    .build_initiator()
    .unwrap();

// connect to server
let mut stream = TcpStream::connect("127.0.0.1:9999").unwrap();
println!("connected!");

// -> e
let len = noise.write_message(&[], &mut buf).unwrap();
send(&mut stream, &buf[..len]);

// <- e, ee, s, es
noise
    .read_message(&recv(&mut stream).unwrap(), &mut buf)
    .unwrap();

// -> s, se
let len = noise.write_message(&[], &mut buf).unwrap();
send(&mut stream, &buf[..len]);

let mut noise = noise.into_transport_mode().unwrap();
println!("Session established...");

// send secure data
for _ in 0..10 {
    let len = noise.write_message(b"HACK THE PLANET", &mut buf).unwrap();
    send(&mut stream, &buf[..len]);
}
```

因为 snow 的所有操作都直接操作内存的 buffer，所有的 IO 都是由你创建的 TCP stream 完成，所以 snow 可以很好地在同步或者异步模式下工作。

## 贤者时刻

> 连接千万条，安全第一条。网络不加密，亲人两行泪。- 鲁迅：不是我说的
