# 基于Poem的OpenAPI服务端框架

作者： 孙黎

---

## 目录

- [简介](#简介)
    - [快速开始](#快速开始)
    - [基础类型](#基础类型)
    - [对象类型](#对象类型)
    - [定义API](#定义API)
    - [自定义请求](#自定义请求)
    - [自定义响应](#自定义响应)
    - [文件上传](#文件上传)
    - [参数校验](#参数校验)
    - [认证](#认证)
- [总结](#总结)
- [有用的链接](#有用的链接)


大家好，我是老油条，一个热爱Rust语言的码农。上个月我决定开发一个新的Web框架[Poem](https://github.com/poem-web/poem)，当整个框架基本成型之后，我觉得应该给它添加别的框架所不具备的并且很有用的功能，所以我开发了[Poem-openapi](https://github.com/poem-web/poem/tree/master/poem-openapi)。

# 简介

[OpenAPI]((https://swagger.io/specification/))规范为`RESTful API`定义了一个标准的并且与语言无关的接口，它允许人类和计算机在不访问源代码、文档或通过网络流量检查的情况下发现和理解服务的功能。调用者可以很容易的理解远程服务并与之交互，并提供了一些好用的工具，例如 [Swagger UI](https://swagger.io/tools/swagger-ui/) (在网页中浏览测试测试API)，[Swagger CodeGen](https://swagger.io/tools/swagger-codegen/) (生成多种语言的客户端SDK)。

`Poem-openapi`是基于`Poem`的 [OpenAPI](https://swagger.io/specification/) 服务端框架。

通常，如果你希望让你的API支持该规范，首先需要创建一个 [接口定义文件](https://swagger.io/specification/) ，然后再按照接口定义编写对应的代码。或者创建接口定义文件后，用 `Swagger CodeGen` 来生成服务端代码框架。但`Poem-openapi`区别于这两种方法，它让你只需要编写Rust的业务代码，利用过程宏来自动生成符合OpenAPI规范的接口和接口定义文件（这相当于接口的文档），和我之前开源的另外一个库[`Async-graphql`](https://github.com/async-graphql/async-graphql) 的原理很像，`OpenAPI`和`GraphQL`是互补的关系，它们适用于不同的场景。

有的朋友可能觉得宏很可怕，它会让代码难以理解，但我觉得如果能用正确的方法来实现过程宏，那么它可以帮我们大大提升开发的效率，所以`Poem-openapi`过程宏的实现遵循了以下几个原则：

  1. **你永远都不会直接用到过程宏生成的任何东西。**（因为IDE无法识别过程宏生成的代码，如果直接使用它们，可能会有烦人的红色下划线，并且自动完成也无法使用，相当于让IDE变成了一个文本编辑器）
  2. **如果你的代码无法通过编译，那么你的接口不符合`OpenAPI`规范。**（尽量把所有的问题都暴露在编译阶段）
  3. **不自己发明DSL。**（如果我的代码没法被`Rustfmt`格式化，这会让我相当恼火）
  4. **不带来额外的开销。**（你完全可以纯手工打造符合`OpenAPI`规范的接口，但在执行效率上通常没有任何的提升）

## 快速开始

下面这个例子，我们定义了一个路径为`/hello`的API，它接受一个名为`name`的URL参数，并且返回一个字符串作为响应内容。`name`参数的类型是`Option<String>`，意味着这是一个可选参数。

运行以下代码后，用浏览器打开`http://localhost:3000`就能看到`Swagger UI`，你可以用它来浏览API的定义并且测试它们。

```rust
use poem::{listener::TcpListener, route};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(
        &self,
        #[oai(name = "name", in = "query")] name: Option<String>, // in="query" 说明这个参数来自Url
    ) -> PlainText<String> { // PlainText是响应类型，它表明该API的响应类型是一个字符串，Content-Type是`text/plain`
        match name {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // 创建一个TCP监听器
    let listener = TcpListener::bind("127.0.0.1:3000");
  
    // 创建API服务
    let api_service = OpenApiService::new(Api)
        .title("Hello World")
        .server("http://localhost:3000/api");
  
    // 开启Swagger UI
    let ui = api_service.swagger_ui("http://localhost:3000");

    // 启动服务器，并指定api的根路径为 /api，Swagger UI的路径为 /
    poem::Server::new(listener)
        .await?
        .run(route().nest("/api", api_service).nest("/", ui))
        .await
}
```

这是`poem-openapi`的一个例子，所以你也可以直接执行以下命令来验证：

```shell
git clone https://github.com/poem-web/poem
cargo run --bin example-openapi-hello-world
```

## 基础类型

基础类型可以作为请求的参数，请求内容或者请求响应内容。`Poem`定义了一个`Type trait`，实现了该`trait`的类型都是基础类型，它们能在运行时提供一些关于该类型的信息用于生成接口定义文件。

`Poem`为大部分常用类型实现了`Type`trait，你可以直接使用它们，同样也可以自定义新的类型，但你需要对 [Json Schema](https://json-schema.org/) 有一定了解（这并不难，事实上在写这个库之前我也只会Json Schema的一些简单用法，并没有进行过深入的了解）。

下表是Json Schema中的数据类型对应的Rust数据类型（只是一小部分）：

| Json Schema                            | Rust                              |
|----------------------------------------|-----------------------------------|
| `{type: "integer", format: "int32"}`   | i32                               |
| `{type: "integer", format: "float32"}` | f32                               |
| `{type: "string" }`                    | String, &str                      |
| `{type: "string", format: "binary" }`  | Binary                            |
| `{type: "string", format: "bytes" }`   | Base64                            |
| `{type: "array" }`                     | Vec<T>                            |

## 对象类型

用过程宏`Object`来定义一个对象，对象的成员必须是实现了`Type trait`的类型（除非你用`#[oai(skip)]`来标注它，那么序列化和反序列化时降忽略该字段用默认值代替）。

用以下代码定义了一个对象类型，它包含四个字段，其中有一个字段是枚举类型。

_对象类型也是基础类型的一种，它同样实现了`Type trait`，所以它也可以作为另一个对象的成员。_

```rust
use poem_api::{Object, Enum};

#[derive(Enum)]
enum PetStatus {
    Available,
    Pending,
    Sold,
}

#[derive(Object)]
struct Pet {
    id: u64,
    name: String,
    photo_urls: Vec<String>,
    status: PetStatus,
}
```

## 定义API

下面定义一组API对宠物表进行增删改查的操作。

`add_pet`和`update_pet`用于添加和更新`Pet`对象，**这是我们在之前定义的基本类型，基本类型不能直接作为请求内容，需要使用一个`Payload`类型来包装它**，这样就可以确定内容的`Content-Type`。在下面的例子中，我们使用`payload::Json`来包装它，表示这两个API请求内容的`Content-Type`为`application/json`。

`find_pet_by_id`和`find_pets_by_status`用于查找`Pet`对象，它们的响应也是一个`Pet`对象，同样需要使用`Payload`类型来包装。

我们可以用`#[oai(name = "...", in = "...")]`来修饰一个函数参数用于指定此参数值的来源，`in`的值可以是`query`, `path`, `header`, `cookie`四种类型。`delete_pet`的`id`参数从路径中提取，`find_pet_by_id`和`find_pets_by_status`的参数从Query中获取。如果参数类型不是`Option<T>`，那么表示这个参数不是一个可选参数，提取失败时会返回`400 Bad Request`错误。

你可以定义多个函数参数，但只能有一个`Payload`类型作为请求内容，或者多个基本类型作为请求的参数。

```rust
use poem_api::{
  OpenApi,
  poem_api::payload::Json,
};
use poem::Result;

struct Api;

#[OpenApi]
impl Api {
    /// 添加新Pet
    #[oai(path = "/pet", method = "post")]
    async fn add_pet(&self, pet: Json<Pet>) -> Result<()> {
        todo!()
    }
  
    /// 更新已有的Pet
    #[oai(path = "/pet", method = "put")]
    async fn update_pet(&self, pet: Json<Pet>) -> Result<()> {
        todo!()
    }

    /// 删除一个Pet
    #[oai(path = "/pet/:pet_id", method = "delete")]
    async fn delete_pet(&self, #[oai(name = "pet_id", in = "path")] id: u64) -> Result<()> {
        todo!()
    }
  
    /// 根据ID查询Pet
    #[oai(path = "/pet/:pet_id", method = "delete")]
    async fn find_pet_by_id(&self, #[oai(name = "status", in = "query")] id: u64) -> Result<Json<Pet>> {
        todo!()
    } 
  
    /// 根据状态查询Pet
    #[oai(path = "/pet/findByStatus", method = "delete")]
    async fn find_pets_by_status(&self, #[oai(name = "status", in = "query")] status: Status) -> Result<Json<Vec<Pet>>> {
        todo!()
    }
}

```

## 自定义请求

`OpenAPI`规范允许同一个接口支持处理不同`Content-Type`的请求，例如一个接口可以同时接受`application/json`和`text/plain`类型的Payload，你可以根据不同的`Content-Type`分别做处理。

在`Poem-openapi`中，要支持此类型请求，需要用`ApiRequest`宏自定义一个实现了`Payload trait`的请求对象。

`create_post`函数接受`CreatePostRequest`请求，当创建成功后，返回`id`。

```rust
use poem_open::{
    ApiRequest, Object,
    payload::{PlainText, Json},
};
use poem::Result;

#[derive(Object)]
struct Post {
    title: String,
    content: String,
}

#[derive(ApiRequest)]
enum CreatePostRequest {
    /// 从JSON创建
    Json(Json<Blog>),
    /// 从文本创建
    Text(PlainText<String>),
}

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "post")]
    async fn create_post(
        &self,
        req: CreatePostRequest,
    ) -> Result<Json<u64>> {
        // 根据Content-Type分别处理
        match req {
            CreatePostRequest::Json(Json(blog)) => {
                todo!();
            }
            CreatePostRequest::Text(content) => {
                todo!();
            }
        }
    }
}

```

## 自定义响应

在前面的例子中，我们的所有请求处理函数都返回的`Result`类型，当发生错误时返回一个`poem::Error`，它包含错误的原因以及状态码。但`OpenAPI`规范允许更详细的描述请求的响应，例如该接口可能会返回哪些状态码，以及状态码对应的原因和响应的内容。

下面的我们修改`create_post`函数的返回值为`CreateBlogResponse`。

`Ok`，`Forbidden`和`InternalError`描述了特定状态码的响应类型。

```rust
use poem_openapi::ApiResponse;
use poem::http::StatusCode;

#[derive(ApiResponse)]
enum CreateBlogResponse {
    /// 创建完成
    #[oai(status = 200)]
    Ok(Json<u64>),
    
    /// 没有权限
    #[oai(status = 403)]
    Forbidden,
  
    /// 内部错误
    #[oai(status = 500)]
    InternalError,
}

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn create_post(
        &self,
        req: CreatePostRequest,
    ) -> CreateBlogResponse {
        match req {
            CreatePostRequest::Json(Json(blog)) => {
                todo!();
            }
            CreatePostRequest::Text(content) => {
                todo!();
            }
        }
    }
}
```

当请求解析失败时，默认会返回`400 Bad Request`错误，但有时候我们想返回一个自定义的错误内容，可以使用`bad_request_handler`属性设置一个错误处理函数，这个函数用于转换`ParseRequestError`到指定的响应类型。

```rust
use poem_openapi::{
    ApiResponse, Object, ParseRequestError, payload::Json,
};

#[derive(Object)]
struct ErrorMessage {
    code: i32,
    reason: String,
}

#[derive(ApiResponse)]
#[oai(bad_request_handler = "bad_request_handler")]
enum CreateBlogResponse {
    /// 创建完成
    #[oai(status = 200)]
    Ok(Json<u64>),
    
    /// 没有权限
    #[oai(status = 403)]
    Forbidden,
  
    /// 内部错误
    #[oai(status = 500)]
    InternalError,
    
    /// 请求无效
    #[oai(status = 400)]
    BadRequest(Json<ErrorMessage>),
}

fn bad_request_handler(err: ParseRequestError) ->   CreateBlogResponse {
    // 当解析请求失败时，返回一个自定义的错误内容，它是一个JSON
    CreateBlogResponse::BadRequest(ErrorMessage {
        code: -1,
        reason: err.to_string(),
    })
}
```

## 文件上传

`Multipart`通常用于文件上传，它可以定义一个表单来包含一个或者多个文件以及一些附加字段。下面的例子提供一个创建`Pet`对象的接口，它在创建`Pet`对象的同时上传一些图片文件。


```rust
use poem_openapi::{Multipart, OpenApi}
use poem::Result;

#[derive(Debug, Multipart)]
struct CreatePetPayload {
    name: String,
    status: PetStatus,
    protos: Vec<Upload>, // 多个照片文件
}

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/pet", method = "post")]
    async fn create_pet(&self, payload: CreatePetPayload) -> Result<Json<u64>> {
        todo!()
    }
}
```

完整的代码请参考[例子](https://github.com/poem-web/poem/tree/master/examples/openapi/upload`)。

## 参数校验

`OpenAPI`引用了`Json Schema`的校验规范，`Poem-openapi`同样支持它们。你可以在请求的参数，对象的成员和`Multipart`的字段三个地方应用校验器。校验器是类型安全的，如果待校验的数据类型和校验器所需要的不匹配，那么将无法编译通过。例如`maximum`只能用于数值类型，`max_items`只能用于数组类型。更多的校验器请参考[文档](https://docs.rs/poem-openapi/0.4.0/poem_openapi/attr.OpenApi.html#operation-argument-parameters)。

```rust
use poem_openapi::{Object, OpenApi, Multipart};

#[derive(Object)]
struct Pet {
    id: u64,

    /// 名字长度不能超过32
    #[oai(max_length = "32")]
    name: String,

    /// 数组长度不能超过3
    #[oai(max_items = "3")]
    photo_urls: Vec<String>,

    status: PetStatus,
}
```

## 认证

OpenApi规范定义了`apikey`，`basic`，`bearer`，`oauth2`，`openIdConnect`五种认证模式，它们描述了指定的`API`接口需要的认证参数。

**注意：API的认证信息最主要的用途是让`Swagger UI`在测试该API时能够正确的执行认证流程。**

下面的例子是用`Github`登录，并提供一个获取所有公共仓库信息的接口。

```rust
use poem_openapi::{
    SecurityScheme, SecurityScope, OpenApi,
    auth::Bearer,
};

#[derive(OAuthScopes)]
enum GithubScope {
    /// access to public repositories.
    #[oai(rename = "public_repo")]
    PublicRepo,

    /// access to read a user's profile data.
    #[oai(rename = "read:user")]
    ReadUser,
}

/// Github authorization
#[derive(SecurityScheme)]
#[oai(
    type = "oauth2",
    flows(authorization_code(
        authorization_url = "https://github.com/login/oauth/authorize",
        token_url = "https://github.com/login/oauth/token",
        scopes = "GithubScope",
    ))
)]
struct GithubAuthorization(Bearer);

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/repo", method = "get")]
    async fn repo_list(
        &self,
        #[oai(auth("GithubScope::PublicRepo"))] auth: GithubAuthorization,
    ) -> Result<PlainText<String>> {
        // 使用GithubAuthorization得到的token向Github获取需要的数据
        todo!()
    }
}
```

完整的代码请参考[例子](https://github.com/poem-web/poem/tree/master/examples/openapi/auth`)。

# 总结

当你读到这里时候，恭喜你已经掌握了`Poem-openapi`的大部分用法，使用它开发API接口比直接使用`Poem`这样通用Web框架更加的方便，并且它并不是独立于`Poem`的另外一套框架，你可以很容易复用现有的提取器，中间件等组件。

# 有用的链接

- [Poem仓库](https://github.com/poem-web/poem)
- [Poem文档](https://docs.rs/poem-openapi/0.6.4/poem_openapi/)
- [Poem-openapi文档](https://docs.rs/poem-openapi/0.6.4/poem_openapi/)
- [Swagger官网](https://swagger.io)
- [OpenAPI规范](https://github.com/OAI/OpenAPI-Specification)

