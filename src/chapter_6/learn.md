# 学习资源

编辑：张汉东

---

## 《Programming Rust》第二版已经官宣发行

摘要：

由O'Reilly出版的《Programming Rust》第二版的电子书和实体书都已经上市了：

- 实体书直达链接： [https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/](https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/)
- 电子书直达链接：[https://www.ebooks.com/en-us/book/210313783/programming-rust/jim-blandy/](https://www.ebooks.com/en-us/book/210313783/programming-rust/jim-blandy/)

关于《Programming Rust》

《Programming Rust》是一本深入考察Rust语言设计思想以及如何将其应用于一系列系统编程问题的技术书籍。本书通过若干入门级的项目示例，深入研究了Rust语言具体的语言特性和各种特性相互作用的逻辑，从类型系统到如何根据类型系统建模现实中的问题，从unsafe rust的本质探讨、如何处理FFI外部函数调用到Rust异步编程等高级特性，无一不做到了深入浅出。

为什么编写第二版？

本书较之第一版，根据Rust编译器最新版本的特性完全进行了跟进，并且新增了一个详尽的讲述异步编程的机制和技巧的章节。 《Programming Rust》是一本适合想要了解Rust语言的C、C++、Python、Java或者其他语言的开发者，或者那些想要提升自己编程技巧的Rust编程人员的书籍。

## 《Rust in Action》 新书发布

- [Manning: Rust in Action](https://www.manning.com/books/rust-in-action?a_aid=rust&a_bid=0367c58f&chan=github)
- [《Rust in Action》随书源码](https://github.com/rust-in-action/code)

## 新书：使用 Rust 进行系统编程

[Hands-On Systems Programming with Rust](https://learning.oreilly.com/library/view/hands-on-systems-programming/9781098109424/)

## Rust 机器学习之书

由 Rust-ml 组织编写，目测正在完善中

该书内容目测是围绕 rust-ml/linfa 库，而/rust-ml/linfa 库 类似于 python  scikit-learn 库。

> scikit-learn，又写作sklearn，是一个开源的基于python语言的机器学习工具包。 它通过NumPy, SciPy和Matplotlib等python数值计算的库实现高效的算法应用，并且涵盖了几乎所有主流机器学习算法。

- [https://rust-ml.github.io/book/](https://rust-ml.github.io/book/)
- [https://github.com/rust-ml/book](https://github.com/rust-ml/book)
- [https://github.com/rust-ml/linfa](https://github.com/rust-ml/linfa)
- [https://blog.logrocket.com/machine-learning-in-rust-using-linfa/](https://blog.logrocket.com/machine-learning-in-rust-using-linfa/)

## Solana Season Hackathon 参赛项目之一

发个 Brian 的 Solana Season Hackathon 参赛项目，现在爆火的 NFT 方向。

Brian 也写了篇文章（First impressions of Rust programming on Solana）来介绍 Solana 的感受。

- [https://github.com/brson/treasuretree](https://github.com/brson/treasuretree)
- [First impressions of Rust programming on Solana](https://brson.github.io/2021/06/08/rust-on-solana)

## Rust 社区朋友关于 Actix-web 和 Rocket 框架的观点

之前用 actix-web 寫了兩個小專案，覺得 actix 還行，但用起來沒有特別有熱情。這兩天藉機第一次試用了下 rocket 0.5.rc1，雖然還沒寫多少代碼，但個人感覺認為 rocket 的設計更 rusty。說真的稍微有點燃起來了，就是當初第一次認識 rust 時的那種愉悅感。

具體說來，rocket 感覺對於安全與正確性的追求非常徹底，幾乎所有東西都強型別且能編譯期進行驗證。

而且，在保證變強的同時，整體寫法也更簡單了；rocket 的設計會自然誘使開發者寫出強型別的代碼。就個人經驗來說，actix_web 沒有這種傾向，或至少傾向較弱。

此外，部份無法於編譯期驗證的項目，凡有注意到的，也都會在服務啟動瞬間驗證完畢。比方說部份 route handler 可能會依賴某些全域 State (等效於 actix_web 中的 web::Data，像是資料庫連接池之類的東西)，這些注入項如果沒有正確初始化，rocket 啟動瞬間就會報錯。而 actix_web 碰到 web::Data 忘記注入的狀況，會要等到呼叫了該 route 時才出錯。除此之外，像是 routing table 是否有衝突等問題，也會在服務啟動瞬間驗證完畢。而在內建服務以外如果有想驗證的東西，也有接口供用戶自定義自己的啟動時驗證方式。

還有很多方便寫碼的小特徵，比方說開發者只有明確需要控制 async 運行狀態時，才需要寫 async fn，否則只要和 fn 打交道即可。除非有東西想要 await 或想平行運行，否則不必然需要處理 Future 接口。

功能方面，我個人特別開心的是 rocket 完整支援了（包含處理檔案上傳在內）Multipart Form 的全部功能。Form 還可以傳輸任意完整的 json-like nested collections 資料格式，這表示不需要透過 javascript 轉成 Json 就能語意精確地發送複雜資料，或是方便地把複雜資料連同檔案合併傳輸。雖然我有點懷疑會有多少人真的這樣設計接口，喂這傢伙不會做過頭了嗎 XDDD

關於 Form 資料的騷操作可看官方說明：
[https://rocket.rs/v0.5-rc/guide/requests/#collections](https://rocket.rs/v0.5-rc/guide/requests/#collections)

Middleware 方面，rocket 的 Fairing 接口也遠比 actix_web 的 Service 容易寫。actix_web 的完整 Service 寫起來實在很整人，有興趣的話可以比較一下兩者 example 的複雜度：
[https://rocket.rs/v0.5-rc/guide/fairings/#example
https://actix.rs/docs/middleware/](https://rocket.rs/v0.5-rc/guide/fairings/#example
https://actix.rs/docs/middleware/)

不過得說一句，fairing 與 middleware 的功能不完全一樣，Rocket 的 Fairing 蓄意被設計得比傳統 middleware 更受限（因為許多問題的解決方案 rocket 不建議由 middleware 處理），因此這種比較可能不完全公平。如此設計的理由可見：

[https://github.com/SergioBenitez/Rocket/issues/55#issuecomment-274655441](https://github.com/SergioBenitez/Rocket/issues/55#issuecomment-274655441)

舉例來說，Rocket 推薦透過一種叫 Custom Guard 的機制，控制每個 route 的用戶身份與存取權。如果 Guard 解析失敗當然無權進入對應的 route，反之如果解析成功，自然會得到已經經過強型別建模後的用戶身份。第一印象看起來非常清晰，之後打算更深入試試看。如果不清楚 Custom Guard 是什麼的話，可看官方範例：

[https://api.rocket.rs/v0.5-rc/rocket/request/trait.FromRequest.html#example-1](https://api.rocket.rs/v0.5-rc/rocket/request/trait.FromRequest.html#example-1)

rocket 用了不少 macro，但是都是非常小且局部的，我認為反而提升了代碼的清晰度。我特別喜歡 `#[derive(Responder)]`，由此建構強型別 Response 非常容易，且單一一個就統整了 actix 的 ResponseError 與 HttpResponse 兩個用例。

在錯誤處理方面，rocket 除 Responder 可回傳 404 等錯誤外，還有額外一層 Catcher 層，能參考 StatusCode 與 Request 內容產生適當響應。這層看上去是通用錯誤如 404, 401 的解決方案，應該是設計來與 Custom Guard 組合使用的，這也表示並非所有錯誤都需要在 route 中手動組裝定義，會對編碼更方便。但關於此處 catcher 方面的想定說明，我還不是很有把握，需要再深入體會一下，此處說法僅供參考。

以上只是個人簡單用過後的印象，兩個框架距離精通都還遠得很，有錯請隨手斧正。

(如果大家想交流，可以在评论区回复)

## 对 王垠《对 Rust 语言的分析》的分析 

时隔五年，回头看 王垠这篇文章 

- 知乎阅读： [https://zhuanlan.zhihu.com/p/382174889](https://zhuanlan.zhihu.com/p/382174889)
- 微信阅读： [https://mp.weixin.qq.com/s/wXKdrl_L65TH9bz_QQfGKw](https://mp.weixin.qq.com/s/wXKdrl_L65TH9bz_QQfGKw)

## Rust vs. C++ for game development

本文在游戏开发的背景下比较Rust与C ++。 并介绍Rust和C++之间的相似性和差异以及使用每个编程语言进行游戏开发的优缺点和工具

[https://blog.logrocket.com/rust-vs-c-for-game-development/](https://blog.logrocket.com/rust-vs-c-for-game-development/)

## 使用 Rust 和 React 构建桌面 App

基于 [Tauri](https://github.com/tauri-apps/tauri) 框架

[https://kent.medium.com/get-started-making-desktop-apps-using-rust-and-react-78a7e07433ce](https://kent.medium.com/get-started-making-desktop-apps-using-rust-and-react-78a7e07433ce)

## 【视频】Rust 无锁编程｜实现险象指针（Hazard Pointers ）

Jon Gjengset 的系列视频之一

[https://www.youtube.com/watch?v=fvcbyCYdR10](https://www.youtube.com/watch?v=fvcbyCYdR10)

## 微软发布 Rust 新的学习视频

[https://channel9.msdn.com/Series/Beginners-Series-to-Rust?WT.mc_id=academic-29077-cxa](https://channel9.msdn.com/Series/Beginners-Series-to-Rust?WT.mc_id=academic-29077-cxa)

## 揭秘Rust中的可变性和引用

1. 对于可变引用，不能出现在不可变引用的声明域和使用域之间；
2. 对于可变/不可变引用，不能出现在在可变引用的声明域和使用域之间。

这就是Rust的借用检查规则：在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用。

[Demystifying Mutability and References in Rust](https://dev.to/arunanshub/demystifying-mutability-and-references-in-rust-caf)

## 给C程序员的Rust入门系列文章

[Learn Rust the Dangerous Way](https://cliffle.com/p/dangerust/)

## 使用Windbg调试非安全Rust代码导致的崩溃

作者在Windows平台写非安全Rust代码的时候遇到一个奔溃：

```rust
error: process didn't exit successfully: `target\debug\rustdesk.exe` (exit code: 0xc0000374, STATUS_HEAP_CORRUPTION)
```

因为这是一个与unsafe的Rust代码相关的分段故障崩溃，很难从控制台消息中得到任何线索，作者便使用了其它工具进行调试，并且记录在了这篇Blog中。

这篇blog的作者是大家最近比较热门的用Rust写的跨平台远程桌面开源软件 RustDesk 的作者。

Read More: [https://dev.to/rustdesk/debugging-a-crash-in-unsafe-rust-with-windbg-2b39](https://dev.to/rustdesk/debugging-a-crash-in-unsafe-rust-with-windbg-2b39)

## 使用 Rust+WASM 的 WebRTC 视频聊天教程

基于Rust+WASM构建的ewebrt信令服务器，结合chrome浏览器构建简单视频聊天应用程序

链接: [https://charles-schleich.medium.com/webrtc-video-chat-tutorial-using-rust-wasm-fa340f7aeef9](https://charles-schleich.medium.com/webrtc-video-chat-tutorial-using-rust-wasm-fa340f7aeef9)

## TheAlgorithms: Rust

大名鼎鼎的 [《The Algorithms》](https://the-algorithms.com/) 的 Rust 版本，使用 Rust 实现所有算法。

[TheAlgorithms/Rust: All Algorithms implemented in Rust](https://github.com/TheAlgorithms/Rust)

## 对比 Java ： Rust 中的 面向对象特性

[https://blog.knoldus.com/object-oriented-programming-concepts-in-rust/](https://blog.knoldus.com/object-oriented-programming-concepts-in-rust/)

## Android 平台中的 Rust/C++ 互操作

[https://security.googleblog.com/2021/06/rustc-interop-in-android-platform.html](https://security.googleblog.com/2021/06/rustc-interop-in-android-platform.html)

## 使用nannou和rust-gpu进行实时光线追踪

Peter Shirley 尝试实时实现“在周末做一个光线追踪”。这是一个个人实验，目的是了解更多关于 rust-gpu、光线追踪以及实时光线追踪的局限性。

[https://github.com/mitchmindtree/nannou-rustgpu-raytracer](https://github.com/mitchmindtree/nannou-rustgpu-raytracer)

## cacao 0.2.0 发布，使用 Rust 构建 native 的 MacOS 应用
该库为 macOS 上的 AppKit 和 iOS/tvOS 上的 UIKit（alpha 质量，请参阅 repo）提供安全的 Rust 绑定。 如果之前使用 Swift 或 Objective-C 为该框架进行过编程，cocao 会尝试提供一种熟悉的开发体验。

由于所有权模型，这在 Rust 中很棘手，但一些创造性的编码和假设可以让我们走得很远。crates.io 上存在 0.2.0 部分是为了使项目能够看到更广泛的使用，这可以为开发提供信息。 也就是说，这个库目前处于早期阶段，可能有错误——你使用它的风险由你自己承担。 但是，只要开发者遵守规则（关于内存/所有权），对于某些应用程序来说就可以了，核心存储库有丰富的示例可以帮助开发者入门。

[https://crates.io/crates/cacao](https://crates.io/crates/cacao)

## Rust中类型的未开发的潜力

Rust 的类型,除了可以用来检查属性外,还可以做一些更有意思的事情. 本文会涉及到很多动态类型.

[https://www.jakobmeier.ch/blogging/Untapped-Rust.html](https://www.jakobmeier.ch/blogging/Untapped-Rust.html)

## 使用Rust+WebAssembly 加速Webcola图可视化工具

摘要：

作者最近从事的项目希望在web页面上展示Spotify上不同音乐家之间的联系。Spotify提供了可以获取数据的官方API，作者获取某个用户最喜欢的若干个音乐家，然后把他们之间的关系在浏览器上展示出来。受HTTP服务器性能调优的启发，借助一系列profile和分析工具，使用Rust+WebAssembly的方式优化了Webcola的调用，使得单机性能从<10 FPS 提升到超过60 FPS。
详细博文可以参见：[https://cprimozic.net/blog/speeding-up-webcola-with-webassembly/](https://cprimozic.net/blog/speeding-up-webcola-with-webassembly/)

## 【教程】用Rust动手实现一个桌面跨平台GUI项目

使用iced构建一个Rust跨平台GUI项目，作者是我，因为有些地方比较仓促，可能有不少错别字，可以的话帮我提一下issue。

其实写到现在，前五章的内容都比较简单，如果目的主要是想要学一下iced怎么用，我的建议是看看官方的例子比较好。

本教程的受众偏向Rust新手，已经看过Rust的大部分概念，急需一个实战项目练手的小伙伴。

目前正在筹备的新章节是Canvas绘制图表，可能干货上比前几章多点。

后续还有多语言支持，多主题支持，以及如何自己构建一个iced的控件，感兴趣的一定不要错过。

0-5章: [https://localnative.app/docs/tutorial0](https://localnative.app/docs/tutorial0)

## Rust中使用 mongoDB 和 redis

本文详细讲解了如果使用 Rust 来操作 mongo 和 redis, 并且使用 actix web 来构建一个简单的 web 应用. 

[https://.com/blog/2021/06/mongodb-redis-rust/](https://.com/blog/2021/06/mongodb-redis-rust/)

## Totally Speedy Transmute： `std::mem::transmute`的安全替代

如果你想要一个标准库中transmute函数的安全替代，可以看看这个。

[https://docs.rs/totally-speedy-transmute/1.69.420/totally_speedy_transmute/](https://docs.rs/totally-speedy-transmute/1.69.420/totally_speedy_transmute/)