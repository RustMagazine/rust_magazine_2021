# Rust 生态观察 |  事件处理系统

本文是我学习 Tremor 项目的记录（膨胀了，竟然看起流处理系统了）。

---

## 文前

我是从  [Rust 插件系统系列](https://nullderef.com/series/) 文章中了解到 Tremor 项目的。

简而言之，Tremor 是一个事件处理系统。 它最初是为了替代 Logstash 或 Telegraf 等软件而设计的。 然而，通过支持更复杂的工作流（例如聚合、汇总、ETL 语言和查询语言），tremor 已经超出了这个单一用例的范围。

Tremor 每年 365 天 24x7 运行，并使用 Rust 编程语言实现。

> 深挖了一下 tremor-runtime 项目背后的公司，原来是 Wayfair 。Wayfair 是美国最大的家具电商，2017 年市值就达58亿美元，前身是早在2002年就成立的CNSStores。亚马逊都吃不下它。
>
> Tremor 应该是 Wayfair 公司旗下的开源项目，已经进入 CNCF 。今年九月份还召开了一次小型的线上的 [Tremor Conf](https://community.cncf.io/events/details/cncf-tremor-community-presents-tremor-con-2021)
>
> 去年（2020）3月份的一次分享：Rust 如何为 Wayfair 省掉数千个核心和TB级的内存的成本 ：[2020-03-31-RustAndTellBerlin-functions](https://www.tremor.rs/slides/2020-03-31-RustAndTellBerlin-functions.pdf)
>
> 从2018年开始， tremor 就是跑在了 wayfair生产环境中，每天处理10兆字节的数据，或每分钟100亿条消息，每秒1000万个指标。tremor 降低了成本，减少了复杂性，巩固和简化了操作环境，以激发SRE的乐趣，减少NOC的工作量，并降低运营成本。

## 背景：什么是事件处理系统

事件处理也有很多其他别名：实时分析、流分析、复杂事件处理、实时流分析。

事件处理系统响应系统环境或用户界面中的事件。事件处理系统的关键特征是事件发生的时间是不可预测的，系统必须能够在这些事件发生时进行处理。

事件，是定义域内足以被重视而进行标记的任何离散事件。在大多数情况下，事件以流的形式组织，这些流可以从各种来源到达并具有异构性。事件处理概念和技术在需要高数据吞吐量和快速决策制定的行业中得到验证。

事件处理应用程序和普通事务（或请求-响应）系统之间的主要区别在于它们的查询范式。传统应用程序根据请求进行查询并提供先前存储的数据，而事件处理应用程序则通过新数据流运行连续查询，并在它们发生时提供发现。

算法交易系统、社交网络分析系统、多种监控应用程序、智能数据传输和集成架构，它们都成功地在分布式系统中对实时大数据进行了流分析。即使是使用复杂或不确定数据的概率和预测场景，也可以通过事件处理技术来解决。

事件处理的一些用例可以作如下分类：

- 算法交易、股市监控、
- 智能病人护理
- 监控生产线
- 供应链优化
- 入侵、监视和欺诈检测（例如[优步](https://eng.uber.com/fraud-prevention-team-profile/)）
- 大多数智能设备应用：智能汽车、智能家居..
- 智能电网——（例如负载预测和异常插头检测见[智能电网，40 亿个事件，整个范围为 100Ks](http://srinathsview.blogspot.com/2014/05/debs-grand-challenge-2014-smart-grids-4.html)）
- 交通监控、地理围栏、车辆和野生动物跟踪——例如[TFL 伦敦](https://wso2.com/blogs/thesource/2017/10/a-smarter-transport-management-system-for-london-with-the-help-of-wso2/)交通管理系统
- 体育分析 — 通过实时分析增强体育运动（例如，这是我们对真实足球比赛所做的一项工作（例如，[在足球广播上叠加实时分析](http://srinathsview.blogspot.com/2014/06/glimpse-of-future-overlaying-realtime.html)）
- 情境感知促销和广告
- 计算机系统和网络监控
- 预测性维护，（例如[用于预测性维护的机器学习技术）](https://www.infoq.com/articles/machine-learning-techniques-predictive-maintenance)
- 地理空间数据处理十种事件处理设计模式

- 过滤（**Filtering**）：简单的限制，如在 `SQL WHERE` 子句中，处理单个事件（对象、行）。过滤的常见用途是减少初始数据负载，过滤后的数据更准确地反映感兴趣的区域。它可以以更复杂的形式存在，作为对外部函数的评估。
- 缓存（**Caching**）: 事件缓存在内存中以进行联合处理时的场景。
- 聚合（**Aggregation**）： 对缓存数据进行分组统计（最小值/最大值、计数、平均值、总和、标准偏差等）。
- 数据库查找（**Database lookups**）： 访问历史或参考数据，这些数据与入口流事件数据相结合。
- 数据库写入（**Database writes**）： 撰写事件或定期报告。
- 相关性（**Correlation**）：加入和比较多个流/源，如在 SQL JOIN 操作中。
- 模式匹配（**Patterns**）： 模式将多个简单事件塑造成一个复杂事件。模式表述通常通过事件存在或不存在的定义给出，在一定的时间相互依赖。
- 有限自动机（**Finite automata**）
- 分层事件（**Hierarchical events**）： 事件具有嵌套结构时分层处理。
- 动态查询（**Dynamic queries**），用户可以进行 即席查询（Ad Hoc）事件。



### 事件流处理（ESP） vs  复杂事件处理（CEP）

#### 事件流处理（ESP）

事件流处理（ESP）是一组旨在促进事件驱动的信息系统生成的技术。 ESP由基本元素组成，例如事件可视化，事件数据库，事件驱动的中间件和事件处理语言（也称为复杂事件处理（CEP））。

尽管ESP和CEP略有不同，但它们通常可以互换使用。

ESP处理来自各种来源的数据，以尝试识别复杂事件模式中的含义。 为此，ESP使用以下技术：

- 在一组事件中检测复杂的模式
- 事件关联和抽象
- 确定事件的层次结构
- 确定事件之间的关系，其中可能包括因果关系，成员关系和时间安排。

#### 复杂事件处理（CEP）

**复杂事件处理** ( **CEP** ) 是一种技术，允许在事件存储之前对事件进行连续处理，以便根据一组预定义的规则识别有意义的事件或事件组合。事件被定义为发生的任何事情，例如状态变化。复杂事件处理是处理多个事件流以及关联看似无关的事件以识别机会或威胁的方法。传统的 [事件处理 ](https://www.tibco.com/reference-center/what-is-event-processing)通常不涉及建立关联、查找或比较历史数据。与传统的事件处理技术不同，CEP 将所有事件视为潜在的重大事件，并异步记录它们。

典型的 CEP 应用领域可以被认定为具有在实际业务情况中重叠的 “情况感知”、“感知和响应” 或 “跟踪和追踪” 方面的某个方面。所有这些都可以归类为活动监测类型，适合对传入事件进行持续评估。CEP 从多个来源获取多个数据点，并根据该数据作出复杂的推断。

CEP 系统必须能够接收和记录事件，并识别这些事件模式和任何相关数据。CEP 系统还必须处理时间限制或基于时间的限制，尤其是在处理不发生事件时。

CEP 使企业能够识别机会和威胁并迅速应对，这在当今快节奏的世界中是必不可少的。

### 一些较知名的事件处理工具列表：

- Amazon **Kinesis**
- Apache **Apex**
- Apache **Flink** （风头正盛）
- Apache **Heron**
- Apache **Kafka**
- **Apache **Samza
- **Apache **Spark
- **Apache **Storm
- **EsperTech **Esper, NEsper
- **Evam **Evam Streaming Analytics
- **DataTorrent **RTS (Real-time Streaming)
- FeedZai
- **Fujitsu **Interstage Big Data Complex Event Processing Server
- **Hitachi **Streaming Data Platform (HSDP)
- **IBM **Streaming Analytics
- **IBM **Operational Decision Manager (ODM)
- **Informatica **RulePoint CEP
- **Google **DataFlow （被称为现代流式计算基石）
- **LG CNS **EventPro
- **Microsoft **Azure Stream Analytics
- **Microsoft **StreamInsight
- **Microsoft **Trill
- **OneMarketData **OneTick CEP
- **Oracle **Event Processor
- **RedHat **Drools Fusion
- **SAP **Event Stream Processor
- **SAS **DataFlux
- ScaleOut Software
- **Software AG **Apama Event Processing Platform
- **SQLStream **s-Server
- **Striim **Striim
- **Tibco **BusinessEvents
- **Tibco **StreamBase
- **Vitria **Operational Intelligence Platform
- **WS02 **Complex Event Processor
- **WS02 **Siddhi**

事件处理工具的一些关键特性包括：

- 流模型（**Streaming model**）
- API或事件处理语言（**API**, or event processing language）
- 状态管理（**State management**）
- 容错+系统恢复（**Fault tolerance** + system recovery）
- 性能： 延迟、吞吐量、可伸缩性（**Performance**）
- 成熟度（**Maturity** ）

Rust 生态中的一些 事件流处理相关项目：

- [tornado](https://github.com/WuerthPhoenix/tornado) ，用于事件处理的一个库
- [timely-dataflow](https://github.com/TimelyDataflow/timely-dataflow) : 数据流引擎 （https://materialize.com/ 公司开源产品），相关论文 [Online Analysis of Distributed Dataflows with Timely Dataflow](https://arxiv.org/pdf/1912.09747.pdf)
- [materialize](https://github.com/MaterializeInc/materialize) :  专注于实时应用的流数据库，基于 timely-dataflow 实现
- [fluvio](https://github.com/infinyon/fluvio) :  可编程高性能分布式流平台



## Tremor 

### Tremor 特色

Tremor 是为高容量信息传递环境而设计的，适合于下列场景：

- 中间人桥接。 我们生活在一个分布式的世界中，分布式系统通信的本质是异步且不可靠的。Tremor 可以桥接从异步的上游源（Source） 与同步的下游接收器（Sink），更一般来说， Tremor 擅长从同步/异步 到 异步/同步 的智能桥接。
- 生产中重新部署。Tremor 可以进行重新配置，而无需重新部署。
- 事件处理 - Tremor 采用来自 DEBS（基于分布式事件的系统）、ESP（事件流处理器）和 CEP（复杂事件处理）社区的许多原则。







## 参考

- [https://www.quora.com/How-is-stream-processing-and-complex-event-processing-CEP-different](https://www.quora.com/How-is-stream-processing-and-complex-event-processing-CEP-different)
- [https://medium.com/stream-processing/what-is-stream-processing-1eadfca11b97](https://medium.com/stream-processing/what-is-stream-processing-1eadfca11b97)
- [用于构建流和实时应用程序的 13 种流处理模式](https://iwringer.wordpress.com/2015/08/03/patterns-for-streaming-realtime-analytics/)