# SecondState | Dapr的一个轻量安全可移植且高性能的Runtime

> 本文作者：Michael Yuan，WasmEdge Maintainer
> 
> 本文源发自 infoQ.com，链接：https://www.infoq.com/articles/webassembly-dapr-wasmedge/

### 本文要点：

* Dapr 是功能强大的构建微服务的框架。
* WebAssembly VM，如 WasmEdge，为微服务应用程序提供高性能且安全的 runtime。
* 基于WebAssembly的微服务可以用多种编程语言编写，包括Rust、C/C++、Swift 和JavaScript。
* WebAssembly 程序嵌入到Dapr sidecar 应用程序中，因此对 Dapr主机环境来说是可移植且跨平台的。
* WasmEdge SDK 为 Tensorflow 推理构建微服务提供了一种简单方法。


自 2019 年发布以来，[Dapr](https://dapr.io/)（Distributed Application runtime ）已迅速成为非常流行的构建微服务的开源框架。它提供了分布式应用程序中常用的构建块和已打包的服务，例如服务调用、状态管理、消息队列、资源绑定和触发器、mTLS 安全连接和服务监控。分布式应用程序开发者可以在 runtime 利用和使用这些基于 Web 的 API，这些 API 是由构建块公开的。这些应用程序通常称为微服务并作为 sidecar 运行。 Dapr 是多 runtime 微服务架构的一个例子，正如 InfoQ 作者 Bilgin Ibryam 所描述的那样。


> Dapr 的 sidecar 模式非常像服务网格（Service Mesh）。然而，不同于传统服务网格意在无需任何代码更改即可管理应用程序，Dapr 应用程序需要集成并积极利用外部 Dapr 构建块服务。


Dapr Sidecar 中的微服务应用程序可以是用 Go 和 Rust 等语言编译的本地客户端 (NaCl) 应用程序，也可以是用 Python 或 JavaScript 编写的托管语言应用程序。换句话说，sidecar 应用程序可以有自己的语言 runtime。 Sidecar 模型允许 Dapr 为其应用程序支持“在任何地方任何语言、任何框架、任何地方”。

## WebAssembly 和 WasmEdge

Dapr 可以直接在操作系统上运行 sidecar 应用程序，也可以通过 Docker 等应用程序容器运行。容器提供了可移植性、易于部署和安全性等优点，但它也伴随着庞大的开销。

在本文中，我们提出了一种运行 Dapr sidecar 应用程序的新方法。我们使用一个用 Rust 或 Go 编写的简单 NaCl 来监听对微服务的 API 请求。它将请求数据传递给 WebAssembly runtime 处理。微服务的业务逻辑是由应用程序开发者创建和部署的 WebAssembly 函数。


> 在本文发布时，你已经可以使用 WasmEdge WASI Socket 来处理网络请求了。具体请见 https://github.com/second-state/dapr-wasm

![](https://i.imgur.com/2Gc8s4B.png)

图 1. 有 WebAssembly 函数的 Dapr 微服务。

WebAssembly runtime 非常适合执行业务逻辑函数。


* WebAssembly 程序可以像编译后的机器原生二进制文件一样快速运行，并且比容器消耗更少的资源。

* WebAssembly 支持高性能语言，如 C/C++、Rust、Swift 和 Kotlin。它还可以支持高级语言，如 JavaScript 和 DSL（Domain Specific Languages）。

* WebAssembly 程序是可移植的，可以轻松部署在不同的操作系统和硬件平台上。

* WebAssembly 提供了一个在 runtime 层级隔离应用程序的安全沙箱。开发者可以通过声明安全策略来限制程序对操作系统或其他资源的访问。

下表总结了 sidecar 应用程序不同方法的优缺点。

![](https://i.imgur.com/tYPBNZe.png)


[WasmEdge](https://github.com/WasmEdge/WasmEdge) 是由 [CNCF（云原生计算基金会）](https://www.secondstate.io/articles/wasmedge-joins-cncf/)/Linux 基金会托管的领先云原生 WebAssembly runtime。它是当今市场上最快的 WebAssembly runtime 之一。 WasmEdge 支持所有标准的 WebAssembly 扩展以及 Tensorflow 推理、KV 存储和图像处理、Socket 等专有扩展。其编译器工具链不仅支持 C/C++、Rust、Swift、Kotlin 和 AssemblyScript 等 WebAssembly 语言，还支持 [常规 JavaScript](https://github.com/WasmEdge/WasmEdge/blob/master/docs/run_javascript.md)。

WasmEdge 应用程序可以嵌入到 [C](https://github.com/WasmEdge/WasmEdge/blob/master/docs/c_api_quick_start.md) 程序、 [Go](https://www.secondstate.io/articles/extend-golang-app-with-webassembly-rust/)  程序、[Rust](https://github.com/WasmEdge/WasmEdge/tree/master/bindings/rust)  程序、[JavaScript](https://www.secondstate.io/articles/getting-started-with-rust-function/) 程序或操作系统的 [CLI](https://github.com/WasmEdge/WasmEdge/blob/master/docs/run.md) 中。Runtime 可以通过 Docker 工具（例如 [CRI-O](https://www.secondstate.io/articles/manage-webassembly-apps-in-wasmedge-using-docker-tools/)）、编排工具（例如 K8s）、serverless 平台（例如 [Vercel](https://www.secondstate.io/articles/vercel-wasmedge-webassembly-rust/)，[Netlify](https://www.secondstate.io/articles/netlify-wasmedge-webassembly-rust-serverless/)，[AWS Lambda](https://www.cncf.io/blog/2021/08/25/webassembly-serverless-functions-in-aws-lambda/)，[腾讯 SCF](https://github.com/second-state/tencent-scf-wasm-runtime)）和数据流框架（例如 [YoMo](https://www.secondstate.io/articles/yomo-wasmedge-real-time-data-streams/) 和 Zenoh）进行管理。

本文中，我将演示如何使用 WasmEdge 作为 Dapr 的 sidecar 应用程序 runtime。

![](https://i.imgur.com/4Y92jyk.jpg)


## 快速开始

首先你需要安装 [Go](https://golang.org/doc/install)、 [Rust](https://www.rust-lang.org/tools/install)、 [Dapr](https://docs.dapr.io/getting-started/install-dapr-cli/)、[WasmEdge](https://github.com/WasmEdge/WasmEdge/blob/master/docs/install.md) 和 [rustwasmc](https://www.secondstate.io/articles/rustwasmc/)  编译器工具。

接下来，从 Github fork 或 clone demo 应用程序。你可以将此 Repo 用作应用程序模板。

```
$ git clone https://github.com/second-state/dapr-wasm
```

该演示有 3 个 Dapr sidecar 应用程序。


*  [web-port](https://github.com/second-state/dapr-wasm/tree/main/web-port) 项目为静态 HTML 页面提供公共 Web 服务。这是应用程序的 UI。
* [image-api-rs](https://github.com/second-state/dapr-wasm/tree/main/image-api-rs) 项目提供了一个 WasmEdge 微服务，使用 [grayscale](https://github.com/second-state/dapr-wasm/tree/main/functions/grayscale)函数将输入图像转换为 grayscale 图像。它演示了将 Rust SDK 用于 Dapr 和 WasmEdge。
* [image-api-go](https://github.com/second-state/dapr-wasm/tree/main/image-api-go) 项目提供了一个 WasmEdge 微服务，使用[分类](https://github.com/second-state/dapr-wasm/tree/main/functions/classify)函数对输入图像上的对象进行识别和分类。它演示了 Go SDK 对 Dapr 和 WasmEdge 的使用。

![](https://i.imgur.com/FjTsIOP.png)

图 2. 演示应用程序中的 Dapr Sidecar 微服务

你可以根据 [README](https://github.com/second-state/dapr-wasm/blob/main/README.md) 中的指示启动 sidecar 服务。 下面是构建 WebAssembly 函数和启动 3 个 sidecar 服务的命令。

```
# Build the classify and grayscale WebAssembly functions, and deploy them to the sidecar projects
$ cd functions/grayscale
$ ./build.sh
$ cd ../../
$ cd functions/classify
$ ./build.sh
$ cd ../../

# Build and start the web service for the application UI
$ cd web-port
$ go build
$ ./run_web.sh
$ cd ../

# Build and start the microservice for image processing (grayscale)
$ cd image-api-rs
$ cargo build
$ ./run_api_rs.sh
$ cd ../

# Build and start the microservice for tensorflow-based image classification
$ cd image-api-go
$ go build --tags "tensorflow image"
$ ./run_api_go.sh
$ cd ../
```

最后，你应该能在浏览器中看到这个 Web UI。

![](https://i.imgur.com/utsx3bZ.png)


图三  生产环境的 demo 应用程序 

## 两个 WebAssembly 函数

我们有两个用 Rust 编写并编译成 WebAssembly 的函数。 它们部署在 sidecar 微服务中，以执行图像处理和分类的实际工作。

虽然我们的示例 WebAssembly 函数是用 Rust 编写的，但你也可以将用 C/C++、Swift、Kotlin 和 AssemblyScript 编写的函数编译为 WebAssembly。 WasmEdge 还为用 JavaScript 和 DSL 编写的函数提供支持。

[grayscale](https://github.com/second-state/dapr-wasm/tree/main/functions/grayscale)函数是一个 Rust 程序，它从 `STDIN` 读取图像数据并将 Grayscale 图像写入 `STDOUT`。

```
use image::{ImageFormat, ImageOutputFormat};
use std::io::{self, Read, Write};

fn main() {
   let mut buf = Vec::new();
   io::stdin().read_to_end(&mut buf).unwrap();

   let image_format_detected: ImageFormat = image::guess_format(&buf).unwrap();
   let img = image::load_from_memory(&buf).unwrap();
   let filtered = img.grayscale();
   let mut buf = vec![];
   match image_format_detected {
       ImageFormat::Gif => {
           filtered.write_to(&mut buf, ImageOutputFormat::Gif).unwrap();
       }
       _ => {
           filtered.write_to(&mut buf, ImageOutputFormat::Png).unwrap();
       }
   };
   io::stdout().write_all(&buf).unwrap();
   io::stdout().flush().unwrap();
}
```

我们使用 [rustwasmc](https://www.secondstate.io/articles/rustwasmc/) 来 build 它并将其复制到 [image-api-rs](https://github.com/second-state/dapr-wasm/tree/main/image-api-rs) sidecar。

```
$ cd functions/grayscale
$ rustup override set 1.50.0
$ rustwasmc  build --enable-ext
$ cp ./pkg/grayscale.wasm ../../image-api-rs/lib
```

[分类](https://github.com/second-state/dapr-wasm/tree/main/functions/classify)函数是一个 Rust 函数，它以图像数据的字节数组作为输入并返回用于分类的字符串。 它使用 [WasmEdge TensorFlow API](https://www.secondstate.io/articles/wasi-tensorflow/)。

```
use wasmedge_tensorflow_interface;

pub fn infer_internal(image_data: &[u8]) -> String {
   let model_data: &[u8] = include_bytes!("models/mobilenet_v1_1.0_224/mobilenet_v1_1.0_224_quant.tflite");
   let labels = include_str!("models/mobilenet_v1_1.0_224/labels_mobilenet_quant_v1_224.txt");

   let flat_img = wasmedge_tensorflow_interface::load_jpg_image_to_rgb8(image_data, 224, 224);

   let mut session = wasmedge_tensorflow_interface::Session::new(
       &model_data,
       wasmedge_tensorflow_interface::ModelType::TensorFlowLite,
   );
   session
       .add_input("input", &flat_img, &[1, 224, 224, 3])
       .run();
   let res_vec: Vec<u8> = session.get_output("MobilenetV1/Predictions/Reshape_1");

   // ... Map the probabilities in res_vec to text labels in the labels file ...
  
   if max_value > 50 {
       format!(
           "It {} a <a href='https://www.google.com/search?q={}'>{}</a> in the picture",
           confidence.to_string(),
           class_name,
           class_name
       )
   } else {
       format!("It does not appears to be any food item in the picture.")
   }
}
```

我们使用 [rustwasmc](https://www.secondstate.io/articles/rustwasmc/) 来 build 它然后将其复制到 [image-api-go](https://github.com/second-state/dapr-wasm/tree/main/image-api-go) sidecar 中。

```
$ cd functions/classify
$ rustup override set 1.50.0
$ rustwasmc  build --enable-ext
$ cp ./pkg/classify_bg.wasm ../../image-api-go/lib/classify_bg.wasm
```

在接下来三个章节中，我们会仔细看看这三个 sidecar 服务。

## 图片处理 sidecar

[image-api-rs](https://github.com/second-state/dapr-wasm/tree/main/image-api-rs) sidecar 应用程序是用 Rust 编写的。 它应该已经安装了上一步中的 WebAssembly 函数 `lib/grayscale.wasm`。 请参考 [functions/bin/install.sh](https://github.com/second-state/dapr-wasm/blob/main/functions/bin/install.sh) 脚本来安装 WasmEdge Runtime二进制 `lib/wasmedge-tensorflow-lite` 及其依赖项。

sidecar 微服务运行一个基于 Tokio 的事件 loop，监听路径 `/api/image` 传入的 HTTP 请求。


```
#[tokio::main]
pub async fn run_server(port: u16) {
   pretty_env_logger::init();

   let home = warp::get().map(warp::reply);

   let image = warp::post()
       .and(warp::path("api"))
       .and(warp::path("image"))
       .and(warp::body::bytes())
       .map(|bytes: bytes::Bytes| {
           let v: Vec<u8> = bytes.iter().map(|&x| x).collect();
           let res = image_process(&v);
           Ok(Box::new(res))
       });

   let routes = home.or(image);
   let routes = routes.with(warp::cors().allow_any_origin());

   let log = warp::log("dapr_wasm");
   let routes = routes.with(log);
   warp::serve(routes).run((Ipv4Addr::UNSPECIFIED, port)).await
}
```

一旦它在 HTTP POST 请求中接收到图像文件，它就会调用 WasmEdge 中的 WebAssembly 函数来执行图像处理任务。 它创建了一个 WasmEdge 实例来与 WebAssembly 程序交互。 

```
pub fn image_process(buf: &Vec<u8>) -> Vec<u8> {
   let mut child = Command::new("./lib/wasmedge-tensorflow-lite")
       .arg("./lib/grayscale.wasm")
       .stdin(Stdio::piped())
       .stdout(Stdio::piped())
       .spawn()
       .expect("failed to execute child");
   {
       // limited borrow of stdin
       let stdin = child.stdin.as_mut().expect("failed to get stdin");
       stdin.write_all(buf).expect("failed to write to stdin");
   }
   let output = child.wait_with_output().expect("failed to wait on child");
   output.stdout
}

```


下面的 Dapr CLI 命令行是在 Dapr runtime 环境启动该微服务。

```
$ cd image-api-rs
$ sudo dapr run --app-id image-api-rs \
        --app-protocol http \
        --app-port 9004 \
        --dapr-http-port 3502 \
        --components-path ../config \
        --log-level debug \
        ./target/debug/image-api-rs
$ cd ../
```

## Tensorflow sidecar

[image-api-go](https://github.com/second-state/dapr-wasm/tree/main/image-api-go) sidecar 应用程序是用 Go 写的。它应该已经有了从前面那一步安装的 WebAssembly 函数 `lib/classify_bg.wasm` 。 请参考 [functions/bin/install.sh](https://github.com/second-state/dapr-wasm/blob/main/functions/bin/install.sh) 脚本安装 WasmEdge Runtime Go SDK。

sidecar 微服务运行一个事件 loop，监听路径 `/api/image` 传入的 HTTP 请求。 

```
func main() {
   s := daprd.NewService(":9003")

   if err := s.AddServiceInvocationHandler("/api/image", imageHandlerWASI); err != nil {
       log.Fatalf("error adding invocation handler: %v", err)
   }

   if err := s.Start(); err != nil && err != http.ErrServerClosed {
       log.Fatalf("error listenning: %v", err)
   }
}
```

一旦它在 HTTP POST 请求中接收到图像文件，它就会调用 WasmEdge 中的 WebAssembly 函数来执行基于 Tensorflow 的图像识别任务。 它利用 WasmEdge 的 Go API 与 WebAssembly 程序交互。 

```
func imageHandlerWASI(_ context.Context, in *common.InvocationEvent) (out *common.Content, err error) {
   image := in.Data

   var conf = wasmedge.NewConfigure(wasmedge.REFERENCE_TYPES)
   conf.AddConfig(wasmedge.WASI)
   var vm = wasmedge.NewVMWithConfig(conf)

   var wasi = vm.GetImportObject(wasmedge.WASI)
   wasi.InitWasi(
       os.Args[1:],     /// The args
       os.Environ(),    /// The envs
       []string{".:."}, /// The mapping directories
       []string{},      /// The preopens will be empty
   )

   /// Register WasmEdge-tensorflow and WasmEdge-image
   var tfobj = wasmedge.NewTensorflowImportObject()
   var tfliteobj = wasmedge.NewTensorflowLiteImportObject()
   vm.RegisterImport(tfobj)
   vm.RegisterImport(tfliteobj)
   var imgobj = wasmedge.NewImageImportObject()
   vm.RegisterImport(imgobj)

   vm.LoadWasmFile("./lib/classify_bg.wasm")
   vm.Validate()
   vm.Instantiate()

   res, err := vm.ExecuteBindgen("infer", wasmedge.Bindgen_return_array, image)
   ans := string(res.([]byte))
  
   vm.Delete()
   conf.Delete()

   out = &common.Content{
       Data:        []byte(ans),
       ContentType: in.ContentType,
       DataTypeURL: in.DataTypeURL,
   }
   return out, nil
}
```

下面的 Dapr CLI 命令行是在Dapr runtime 环境启动微服务。

```
$ cd image-api-go
$ sudo dapr run --app-id image-api-go \
        --app-protocol http \
        --app-port 9003 \
        --dapr-http-port 3501 \
        --log-level debug \
        --components-path ../config \
        ./image-api-go
$ cd ../
```

## The web UI sidecar

Web UI 服务 web-port 是一个用 Go 编写的简单 Web 服务器。 它位于 static 文件时，提供静态 HTML 和 JavaScript 文件，并将上传到 `/api/hello` 的图像发送到 Grayscale 函数或对 Classify sidecar 的 `/api/image` 端点。

```
func main() {
   http.HandleFunc("/static/", staticHandler)
   http.HandleFunc("/api/hello", imageHandler)
   println("listen to 8080 ...")
   log.Fatal(http.ListenAndServe(":8080", nil))
}

func staticHandler(w http.ResponseWriter, r *http.Request) {
   // ... read and return the contents of HTML CSS and JS files ...
}

func imageHandler(w http.ResponseWriter, r *http.Request) {
   // ... ...
   api := r.Header.Get("api")
   if api == "go" {
       daprClientSend(body, w)
   } else {
       httpClientSend(body, w)
   }
}

// Send to the image-api-go sidecar (classify) via the Dapr API
func daprClientSend(image []byte, w http.ResponseWriter) {
   // ... ...
   resp, err := client.InvokeMethodWithContent(ctx, "image-api-go", "/api/image", "post", content)
   // ... ...
}

// Send to the image-api-rs sidecar (grayscale) via the HTTP API
func httpClientSend(image []byte, w http.ResponseWriter) {
   // ... ...
   req, err := http.NewRequest("POST", "http://localhost:3502/v1.0/invoke/image-api-rs/method/api/image", bytes.NewBuffer(image))
   // ... ...
}

```


[page.js](https://github.com/second-state/dapr-wasm/blob/main/web-port/static/page.js) 中的 JavaScript 只是将图像上传到 web-port sidecar 的 `/api/hello` 端点，[web-port](https://github.com/second-state/dapr-wasm/tree/main/web-port) 将根据 request header api 请求分类或 Grayscale 微服务。

```
function runWasm(e) {
   const reader = new FileReader();
   reader.onload = function (e) {
       setLoading(true);
       var req = new XMLHttpRequest();
       req.open("POST", '/api/hello', true);
       req.setRequestHeader('api', getApi());
       req.onload = function () {
           // ...  display results ...
       };
       const blob = new Blob([e.target.result], {
           type: 'application/octet-stream'
       });
       req.send(blob);
   };
   console.log(image.file)
   reader.readAsArrayBuffer(image.file);
}
```

以下 Dapr CLI 命令启动静态 UI 文件的 Web 服务。

```
$ cd web-port
$ sudo dapr run --app-id go-web-port \
        --app-protocol http \
        --app-port 8080 \
        --dapr-http-port 3500 \
        --components-path ../config \
        --log-level debug \
        ./web-port
$ cd ../
```

完成了。你现在得到一个有三个部分的分布式应用，并且是用两种语言编写的。


## 接下来

正如我们所展示的，Dapr 的分布式网络 runtime 和 WasmEdge 的通用语言 runtime 之间有很多协同。 这种方法可以推广并应用于其他服务网格或分布式应用程序框架。与 Dapr 不同，许多服务网格只能在 Kubernetes 作为其控制平面运行，因此依赖于 Kubernetes API。 WasmEdge 是一个与 Kubernetes 兼容的 runtime，可以作为运行微服务的轻量级容器替代方案，起到举足轻重的作用。 敬请关注！



