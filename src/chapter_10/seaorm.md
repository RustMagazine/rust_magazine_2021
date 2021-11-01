# Rust 生态观察｜ SeaORM: 要做 Rust 版本的 ActiveRecord 

作者：张汉东

---

## 引子

有些人说用 Rust 进行 Web 开发 是杀鸡用牛刀，这种观点其实是对「系统级语言」的刻板印象造成的。无论从性能、工程架构还是开发效率，Rust 其实都很出色，目前就是需要一套比较成熟的框架。无论如何，Rust 在 Web 开发领域的生态在逐步成型。

注意：这里的 Web 是指更加广义的 Web ，不仅仅是 CRUD，还包括 网络服务、云原生服务端、 WebAssembly 、嵌入式物联网、区块链等等。

这也促使我想写《Rust Web 生态观察》系列文章，时间精力有限，不定时更新。希望能给大家提供一个视角，来客观地 了解 Rust 在 Web 开发领域的发展。

## Rust ORM 生态

Rust ORM 生态中，最早的 ORM 是  [Diesel](https://diesel.rs/)。Diesel 的作者 `sgrif` 曾经也是 ActiveRecord （知名 Web 框架 Ruby on Rails 内置的 ORM ）的核心贡献者。Diesel ORM 的设计也是 sgrif 总结了 AR 中的经验教训。Diesel 是一个优秀的 ORM 框架，但是它并不支持异步。并且，Diesel 并不是 ActiveRecord 的 Rust 复刻版。

> Active Record ，是一种领域模型模式，特点是一个模型类对应关系型数据库中的一个表，而模型类的一个实例对应表中的一行记录。它并不是 Ruby on Rails 首创，而是由 Martin Fowler 的《企业应用架构模式》一书中提出的。
>
> Rails 的 Active Record ORM 框架，和 Rails 框架一样，遵循的是「约定大于配置」的惯例。比如 ：
>
> - User 模型，对应的是 users 表。遵循单复数的约定。
> - 默认会以 `id`字段为主键。而以 `_id`后缀的字段作为外键。
> - 自动生成 `find_by_id` 之类的查询方法。
> - 以 `created_at` 和 `updated_at` 在创建和更新记录的时候，自动设置时间戳。
> - `(table_name)_count ` ，保存关联对象的数量。
> - 其他。
>
> ORM 有两种模式： **Active Record 与 Data Mapper**  
>
> ActiveRecord : 一个对象既包含数据又包含行为。这些数据大部分是持久性的，需要存储在数据库中。Active Record使用最明显的方法，将数据访问逻辑放在域对象中。这样，所有人都知道如何在数据库中读取和写入数据。
>
> DataMapper: 与`Active Record`不一样的地方在于它增加了一个映射器，把持久化对象的数据跟行为分开了。它的关键地方在于数据模型遵循了单一职责原则。DataMapper 适合更加复杂的层次结构。

随着 Rust 异步生态的发展，ORM 异步支持的需求也逐渐增多。

之后，[`sqlx`](https://github.com/launchbadge/sqlx) 出现了。Go 语言生态中也有同名的数据库包，不确定 Rust 这个 `sqlx` 的命名是否参考它。

 `sqlx`并不是一个 ORM 框架，它没有像`Diesel`这类支持orm框架的 DSL ，用户可以自己编写`sql`语句，将查询结果按列取出或映射到`struct`上。它的一些特点：

- 支持 `async-std` 和` tokio`
- 编译时查询检查（可选）
- 内置连接池
- 支持 `postgresql` 、`mysql/maridb`、`sqlite`
- 纯 `Rust`实现`mysql`和`postgresql` 访问驱动程序（`sqlite`使用了` libsqlite3 C` 库）
- 支持 TLS
- 嵌套事务

sqlx 使用起来相对比较“原始”，直接操作 SQL 语句，没有 ORM 不太方便。

国内 Rust 社区小伙伴 [@zhuxiujia](https://github.com/zhuxiujia) 也实现了一个异步 ORM 框架 [rbatis](https://github.com/rbatis/rbatis)。Rbatis 并不是基于 sqlx 实现的，它的灵感来自于 Java 的 ORM 框架 Mybatis。Rbatis 提供了一些内置插件，可以针对一些常用场景增加开发效率。

而我们今天要看的主角是[`sea-orm`](https://github.com/SeaQL/sea-orm) ，它是基于 `sqlx` 实现的 ORM 框架，号称要实现 Rust 版本 的 [ActiveRecord](https://guides.rubyonrails.org/active_record_basics.html) 。

## SeaORM : 要做 Rust 版本的 Active Record

既然 `sea-orm` 喊出这样的口号，那它的架构设计肯定和 Active Record 是有点关系吧？让我们先从它的 API 开始探索。

### SeaORM 示例

从它的 example 项目中可以看到如下使用示例：

```rust
// https://github.com/SeaQL/sea-orm/blob/master/examples/rocket_example/src/main.rs
// 只摘录关键代码

mod post;
pub use post::Entity as Post;

const DEFAULT_POSTS_PER_PAGE: usize = 5;

// 使用 Rocket web 框架的一个 endpoint api
#[post("/", data = "<post_form>")]
async fn create(conn: Connection<Db>, post_form: Form<post::Model>) -> Flash<Redirect> {
    let form = post_form.into_inner();

    // 注意 ActiveModel ，这个在 Rails 的 ActiveRecord 中也有同名组件
    post::ActiveModel {
        title: Set(form.title.to_owned()),
        text: Set(form.text.to_owned()),
        ..Default::default()
    }
    .save(&conn)
    .await
    .expect("could not insert post");

    Flash::success(Redirect::to("/"), "Post successfully added.")
}

#[post("/<id>", data = "<post_form>")]
async fn update(conn: Connection<Db>, id: i32, post_form: Form<post::Model>) -> Flash<Redirect> {
    // 注意： find_by_id 关联函数
    let post: post::ActiveModel = Post::find_by_id(id)
        .one(&conn)
        .await
        .unwrap()
        .unwrap()
        .into();

    let form = post_form.into_inner();

    post::ActiveModel {
        id: post.id,
        title: Set(form.title.to_owned()),
        text: Set(form.text.to_owned()),
    }
    .save(&conn)
    .await
    .expect("could not edit post");

    Flash::success(Redirect::to("/"), "Post successfully edited.")
}

#[get("/?<page>&<posts_per_page>")]
async fn list(
    conn: Connection<Db>,
    posts_per_page: Option<usize>,
    page: Option<usize>,
    flash: Option<FlashMessage<'_>>,
) -> Template {
    // Set page number and items per page
    let page = page.unwrap_or(1);
    let posts_per_page = posts_per_page.unwrap_or(DEFAULT_POSTS_PER_PAGE);
    if page == 0 {
        panic!("Page number cannot be zero");
    }

    // Setup paginator
    // 注意： find() 函数
    let paginator = Post::find()
        // 注意 order_by_asc 函数
        .order_by_asc(post::Column::Id)
        .paginate(&conn, posts_per_page);
    let num_pages = paginator.num_pages().await.ok().unwrap();

    // Fetch paginated posts
    let posts = paginator
        .fetch_page(page - 1)
        .await
        .expect("could not retrieve posts");

    Template::render(
        "index",
        context! {
            page: page,
            posts_per_page: posts_per_page,
            posts: posts,
            flash: flash.map(FlashMessage::into_inner),
            num_pages: num_pages,
        },
    )
}

#[get("/<id>")]
async fn edit(conn: Connection<Db>, id: i32) -> Template {
    // 注意： post::Model 
    let post: Option<post::Model> = Post::find_by_id(id)
        .one(&conn)
        .await
        .expect("could not find post");

    Template::render(
        "edit",
        context! {
            post: post,
        },
    )
}

```

上面示例中，我们发现有很多来自于 ActiveRecord 的影子（标注注释的地方）。

如果你没有使用 Rails 和 ActiveRecord 的经验，也没有关系。至少你现在已经对 ActiveRecord 有了一个初步的印象：

1. 数据模型 和 数据表 存在一一映射的关系，命名上甚至可能还有默认约定存在。
2. ORM 会自动生成一些查询方法，比如 `find_by_id` / `find` 等等。

然后，我们在看看 `post.rs` 示例：

```rust
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, FromForm)]
#[serde(crate = "rocket::serde")]
// 关于表名，和 Diesel 处理类似，你可以自己设置
// 这个 Model 是示例中定义的和数据表 `posts` 对应的数据模型，你也可以命名为 `Post`
#[sea_orm(table_name = "posts")] 
pub struct Model {
    // 可以通过宏指定主键
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub text: String,
}

// 暂时不清楚这是起什么作用
// 几乎每个示例都会有这个类型，但没有使用它的地方
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

// 为 `ActiveModel` 实现一个 `ActiveModelBehavior ` trait 
// 这里有点莫名其妙 ，`ActiveModel` 和 `ActiveModelBehavior ` 应该都是 sea-orm 内部的
// 暂时猜测这行代码是为 Model 实现了一些默认行为，比如`find_by_id` 之类
impl ActiveModelBehavior for ActiveModel {}
```

至少，我们通过示例代码，找到了 SeaORM 框架架构的关键信息： `ActiveModel`/ `ActiveModelBehavior` / `Entity` 等。

我们继续找一个更加复杂的例子: [examples/async-std](https://github.com/SeaQL/sea-orm/tree/master/examples/async-std) 

在这个例子里描述了如图这样的表关系：

![1](./image/seaorm/1.png)

按照 ActiveRecord 的思想，每个表要映射一个数据模型：

```rust
// https://github.com/SeaQL/sea-orm/blob/master/examples/async-std/src/example_cake.rs
// example_cake.rs 对应 cake 表

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "cake")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

// 现在我们已经明确，Relation 类型是为了定义表关系
// 这里 Fruit 和 Cake 之间存在关系
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Fruit,
}

// 这里就是使用 `RelationTrait` 来定义它们之间的关系
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            // 通过 `Entity::has_many` 函数来指定 Cake 和 Fruit 的一对多关系
            // Cake has_many Fruit
            // 返回的是 RelationDef 类型
            Self::Fruit => Entity::has_many(super::fruit::Entity).into(),
        }
    }
}

// 另外一个 trait ： Related
impl Related<super::fruit::Entity> for Entity {
    // 此次应该是返回 Cake 模型有关系的 model 信息
    fn to() -> RelationDef {
        Relation::Fruit.def()
    }
}

// Cake 和 filling 之间是 多对多关系
impl Related<super::filling::Entity> for Entity {
    fn to() -> RelationDef {
        // 多对多关系通过中间表 cake_filling 来指定
        super::cake_filling::Relation::Filling.def()
    }

    fn via() -> Option<RelationDef> {
        // 多对多关系通过中间表 cake_filling 来指定
        // 这里是指 via Cake to filling
        Some(super::cake_filling::Relation::Cake.def().rev())
    }
}

// 熟悉的行为
// 为什么不直接由框架实现？
impl ActiveModelBehavior for ActiveModel {}

```

再看看 `Fruit` :

```rust
// https://github.com/SeaQL/sea-orm/blob/master/examples/async-std/src/example_fruit.rs
use sea_orm::entity::prelude::*;

// 注意这个结构体 Entity 
#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

// 提供 EntityName trait 来指定 table name
// 根据之前的示例，这里也可以使用宏指定
impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "fruit"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub id: i32,
    pub name: String,
    pub cake_id: Option<i32>,
}

// 这里有一个 DeriveColumn
#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Name,
    CakeId,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

// 实现 PrimaryKeyTrait ，指定 auto_increment 
// 猜测应该也可以通过宏指定
impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;

    fn auto_increment() -> bool {
        true
    }
}

// 设置 Fruit 和 Cake 有关系
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Cake,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
		// ColumnType 指定了对应数据库表的类型
    // 猜测框架应该有默认类型映射，这里是出于文档作用来显式指定
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Name => ColumnType::String(None).def(),
            Self::CakeId => ColumnType::Integer.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            // 指定 和 Cake 的关系，是一对多
            // Fruit belongs_to Cake
            Self::Cake => Entity::belongs_to(super::cake::Entity)
                .from(Column::CakeId) // 指定外键
                .to(super::cake::Column::Id)
                .into(),
        }
    }
}

impl Related<super::cake::Entity> for Entity {
    // 设置关系
    fn to() -> RelationDef {
        Relation::Cake.def()
    }
}

// 熟悉的操作
impl ActiveModelBehavior for ActiveModel {}
```

再看 `CakeFilling` :

```rust
// https://github.com/SeaQL/sea-orm/blob/master/examples/async-std/src/example_cake_filling.rs

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "cake_filling"
    }
}

// Cake 和 Filling 是多对多的关系，所以这个 cake_filling 表是中间表
// 这里需要两个表的外键
#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub cake_id: i32,
    pub filling_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    CakeId,
    FillingId,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    CakeId,
    FillingId,
}

// 中间表的外键不能自增
impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = (i32, i32);

    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Cake,
    Filling,
}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::CakeId => ColumnType::Integer.def(),
            Self::FillingId => ColumnType::Integer.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            // 设置 多对多关系
            // CakeFilling belongs_to Cake
            Self::Cake => Entity::belongs_to(super::cake::Entity)
                .from(Column::CakeId)
                .to(super::cake::Column::Id)
                .into(),
						// CakeFilling belongs_to Filling
            Self::Filling => Entity::belongs_to(super::filling::Entity)
                .from(Column::FillingId)
                .to(super::filling::Column::Id)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}

```

接下来，我们可以看看示例代码中关于表操作的代码：

```rust
// https://github.com/SeaQL/sea-orm/blob/master/examples/async-std/src/select.rs

// 查询一对多关系的方法
async fn find_together(db: &DbConn) -> Result<(), DbErr> {
    print!("find cakes and fruits: ");

    // 通过 find_also_related 方法进行一对多关联查询
    let both: Vec<(cake::Model, Option<fruit::Model>)> =
        Cake::find().find_also_related(Fruit).all(db).await?;

    println!();
    for bb in both.iter() {
        println!("{:?}\n", bb);
    }

    Ok(())
}


// 查询多对多关系的方法
async fn find_many_to_many(db: &DbConn) -> Result<(), DbErr> {
    print!("find cakes and fillings: ");

    // 看得出来，通过提供的 `find_with_related` 可以进行关联查询
    let both: Vec<(cake::Model, Vec<filling::Model>)> =
        Cake::find().find_with_related(Filling).all(db).await?;

    println!();
    for bb in both.iter() {
        println!("{:?}\n", bb);
    }

    print!("find fillings for cheese cake: ");

    let cheese = Cake::find_by_id(1).one(db).await?;

    if let Some(cheese) = cheese {
        // find_related
        let fillings: Vec<filling::Model> = cheese.find_related(Filling).all(db).await?;

        println!();
        for ff in fillings.iter() {
            println!("{:?}\n", ff);
        }
    }

    print!("find cakes for lemon: ");

    let lemon = Filling::find_by_id(2).one(db).await?;

    if let Some(lemon) = lemon {
        let cakes: Vec<cake::Model> = lemon.find_related(Cake).all(db).await?;

        println!();
        for cc in cakes.iter() {
            println!("{:?}\n", cc);
        }
    }

    Ok(())
}

// from : https://github.com/SeaQL/sea-orm/blob/master/examples/async-std/src/operation.rs

pub async fn insert_and_update(db: &DbConn) -> Result<(), DbErr> {
    let pear = fruit::ActiveModel {
        // 注意 ： Set 是函数
        name: Set("pear".to_owned()),
        ..Default::default()
    };
    // insert 函数
    let res = Fruit::insert(pear).exec(db).await?;

    println!();
    println!("Inserted: last_insert_id = {}\n", res.last_insert_id);

    let pear: Option<fruit::Model> = Fruit::find_by_id(res.last_insert_id).one(db).await?;

    println!();
    println!("Pear: {:?}\n", pear);

    let mut pear: fruit::ActiveModel = pear.unwrap().into();
    pear.name = Set("Sweet pear".to_owned());
    // update 函数
    let pear: fruit::ActiveModel = pear.update(db).await?;

    println!();
    println!("Updated: {:?}\n", pear);

    Ok(())
}
```

通过上面一系列从模型定义到数据操作，我们看得出来，SeaORM 的设计确实和 ActiveRecord 类型。如果开发者对 ActiveRecord 熟悉，那么会感觉很容易上手。比如，设置表关系的 DSL 方法： `has_many`  和 `belongs_to`  。

当然，SeaORM 也提供一些方便的编写数据迁移功能的方法和函数：

```rust
// https://github.com/SeaQL/sea-orm/blob/master/examples/rocket_example/src/setup.rs

use sea_orm::sea_query::{ColumnDef, TableCreateStatement};
use sea_orm::{error::*, sea_query, DbConn, ExecResult};

async fn create_table(db: &DbConn, stmt: &TableCreateStatement) -> Result<ExecResult, DbErr> {
    let builder = db.get_database_backend();
    db.execute(builder.build(stmt)).await
}

pub async fn create_post_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = sea_query::Table::create()
        .table(super::post::Entity)
        .if_not_exists()
        .col(
            ColumnDef::new(super::post::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(super::post::Column::Title)
                .string()
                .not_null(),
        )
        .col(
            ColumnDef::new(super::post::Column::Text)
                .string()
                .not_null(),
        )
        .to_owned();

    create_table(db, &stmt).await
}

```

是通过 [`sql_query`](https://github.com/SeaQL/sea-query) 组件提供的功能，我们接下来就会介绍它。

到目前为止，我们已经基本了解 SeaORM 的 架构设计 和 关键 概念和 API，让我们继续探索 SeaORM 的源码实现。

###  SeaORM 源码架构

Rails 的 ActiveRecord ORM 是一个功能相当丰富和成熟的框架，并且还细分了很多组件:

- ActiveModel: 是从 ActiveRecord 抽象出来的组件，它是数据模型的抽象接口。
- ActiveRecord:  专注于 数据库相关功能
- ActiveStorage:  是 ActiveRecord 抽象的延伸，专门负责抽象和处理文件上传相关。

反观 SeaORM ，目前还很单薄，但是反过来看，未来也是充满想象的。

SeaORM 中也提供了 ActiveModel 抽象。

####  Entity 与 ActiveModel 抽象

##### Entity 抽象

主要代码在 `https://github.com/SeaQL/sea-orm/tree/master/src/entity` 目录下。

```rust
// Entity 必须是有 Entity Name 的，并且要实现 

// 这种写法避免了泛型限定过长
// `Iden` 是在 SeaQuery 中定义的，它表示任意查询语句中的标识符，可以转换为字符串
pub trait IdenStatic: Iden + Copy + Debug + 'static {
    fn as_str(&self) -> &str;
}

// 作为一个 Entity ，应该有特定的行为
pub trait EntityName: IdenStatic + Default {
    fn schema_name(&self) -> Option<&str> {
        None
    }

    fn table_name(&self) -> &str;

    fn module_name(&self) -> &str {
        self.table_name()
    }

    fn table_ref(&self) -> TableRef {
        match self.schema_name() {
            Some(schema) => (Alias::new(schema).into_iden(), self.into_iden()).into_table_ref(),
            None => self.into_table_ref(),
        }
    }
}

/// An Entity implementing `EntityTrait` represents a table in a database.
///
/// This trait provides an API for you to inspect it's properties
/// - Column (implemented [`ColumnTrait`])
/// - Relation (implemented [`RelationTrait`])
/// - Primary Key (implemented [`PrimaryKeyTrait`] and [`PrimaryKeyToColumn`])
///
/// This trait also provides an API for CRUD actions
/// - Select: `find`, `find_*`
/// - Insert: `insert`, `insert_*`
/// - Update: `update`, `update_*`
/// - Delete: `delete`, `delete_*`
pub trait EntityTrait: EntityName {
    type Model: ModelTrait<Entity = Self> + FromQueryResult;

    type Column: ColumnTrait;

    type Relation: RelationTrait;

    type PrimaryKey: PrimaryKeyTrait + PrimaryKeyToColumn<Column = Self::Column>;

    fn belongs_to<R>(related: R) -> RelationBuilder<Self, R>
    where
        R: EntityTrait,
    {
        RelationBuilder::new(RelationType::HasOne, Self::default(), related, false)
    }

    fn has_one<R>(_: R) -> RelationBuilder<Self, R>
    where
        R: EntityTrait + Related<Self>,
    {
        RelationBuilder::from_rel(RelationType::HasOne, R::to().rev(), true)
    }

    fn has_many<R>(_: R) -> RelationBuilder<Self, R>
    where
        R: EntityTrait + Related<Self>,
    {
        RelationBuilder::from_rel(RelationType::HasMany, R::to().rev(), true)
    }
  
  	fn find() -> Select<Self> {
        Select::new()
    }
  
  	fn find_by_id(values: <Self::PrimaryKey as PrimaryKeyTrait>::ValueType) -> Select<Self> {
        let mut select = Self::find();
        let mut keys = Self::PrimaryKey::iter();
        for v in values.into_value_tuple() {
            if let Some(key) = keys.next() {
                let col = key.into_column();
                select = select.filter(col.eq(v));
            } else {
                panic!("primary key arity mismatch");
            }
        }
        if keys.next().is_some() {
            panic!("primary key arity mismatch");
        }
        select
    }
  	
  	fn insert<A>(model: A) -> Insert<A>
    where
        A: ActiveModelTrait<Entity = Self>,
    {
        Insert::one(model)
    }
  
  	fn insert_many<A, I>(models: I) -> Insert<A>
    where
        A: ActiveModelTrait<Entity = Self>,
        I: IntoIterator<Item = A>,
    {
        Insert::many(models)
    }
  
  	fn update<A>(model: A) -> UpdateOne<A>
    where
        A: ActiveModelTrait<Entity = Self>,
    {
        Update::one(model)
    }

  	
  	fn update_many() -> UpdateMany<Self> {
        Update::many(Self::default())
    }
  
  	fn delete<A>(model: A) -> DeleteOne<A>
    where
        A: ActiveModelTrait<Entity = Self>,
    {
        Delete::one(model)
    }
  
  	fn delete_many() -> DeleteMany<Self> {
        Delete::many(Self::default())
    }
}
```

通过上面的关键代码，看得出来，一个 Entity 满足下面条件：

1. 必须实现 `EntityTrait`
2. 有 `Model`/ `Clomen`/ `Relation`/ `PrimaryKey` 四个关联类型
3. 提供一些默认行为，包括：`belongs_to`/ `has_many`/  `CRUD`相关方法

再看 `ModelTrait` :

```rust

// https://github.com/SeaQL/sea-orm/blob/master/src/entity/model.rs
pub trait ModelTrait: Clone + Send + Debug {
    type Entity: EntityTrait;

    fn get(&self, c: <Self::Entity as EntityTrait>::Column) -> Value;

    fn set(&mut self, c: <Self::Entity as EntityTrait>::Column, v: Value);

    // 内连接（inner join）
    // Select 结构体对应查询对象
    fn find_related<R>(&self, _: R) -> Select<R>
    where
        R: EntityTrait,
        Self::Entity: Related<R>,
    {
        <Self::Entity as Related<R>>::find_related().belongs_to(self)
    }

		// 内连接（inner join），方向与 find_related 相反
    fn find_linked<L>(&self, l: L) -> Select<L::ToEntity>
    where
        L: Linked<FromEntity = Self::Entity>,
    {
        let tbl_alias = &format!("r{}", l.link().len() - 1);
        l.find_linked().belongs_to_tbl_alias(self, tbl_alias)
    }
}
```

如果说 Entity 是对 数据库中表的映射，那么 Model 是对 Entity 行为的抽象。

`ModelTrait ` 中定义了 一个 Model 应该可以 Get/Set 一个字段的值（Value），并且可以通过 `find_related` 方法可以查询 `belongs_to` 关系。

##### ActiveModel 抽象

```rust
// https://github.com/SeaQL/sea-orm/blob/master/src/entity/active_model.rs

// ActiveRecord 模式中，Entity 对应每张表，那么表中每一行数据就代表一个 Active 对象
// ActiveValue 代表「当前活动行」Value
#[derive(Clone, Debug, Default)]
pub struct ActiveValue<V>
where
    V: Into<Value>,
{
    value: Option<V>,
    state: ActiveValueState,
}

// 这里刻意使用 驼峰式 来定义这个方法，个人理解是为了突出表达语义
#[allow(non_snake_case)]
pub fn Set<V>(v: V) -> ActiveValue<V>
where
    V: Into<Value>,
{
    ActiveValue::set(v)
}

// ActiveValue 状态
#[derive(Clone, Debug)]
enum ActiveValueState {
    Set,
    Unchanged,
    Unset,
}

// ActiveModelTrait trait 定义了 ActiveModel 的行为
#[async_trait]
pub trait ActiveModelTrait: Clone + Debug {
    type Entity: EntityTrait;

    fn take(&mut self, c: <Self::Entity as EntityTrait>::Column) -> ActiveValue<Value>;

    fn get(&self, c: <Self::Entity as EntityTrait>::Column) -> ActiveValue<Value>;

    fn set(&mut self, c: <Self::Entity as EntityTrait>::Column, v: Value);

    fn unset(&mut self, c: <Self::Entity as EntityTrait>::Column);

    fn is_unset(&self, c: <Self::Entity as EntityTrait>::Column) -> bool;

    fn default() -> Self;

    async fn insert(self, db: &DatabaseConnection) -> Result<Self, DbErr>
    where
        <Self::Entity as EntityTrait>::Model: IntoActiveModel<Self>,
    {
        let am = self;
        let exec = <Self::Entity as EntityTrait>::insert(am).exec(db);
        let res = exec.await?;
        // Assume valid last_insert_id is not equals to Default::default()
        if res.last_insert_id
            != <<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType::default()
        {
            let found = <Self::Entity as EntityTrait>::find_by_id(res.last_insert_id)
                .one(db)
                .await?;
            match found {
                Some(model) => Ok(model.into_active_model()),
                None => Err(DbErr::Exec("Failed to find inserted item".to_owned())),
            }
        } else {
            Ok(Self::default())
        }
    }

    async fn update(self, db: &DatabaseConnection) -> Result<Self, DbErr> {
        let exec = Self::Entity::update(self).exec(db);
        exec.await
    }

    /// Insert the model if primary key is unset, update otherwise.
    /// Only works if the entity has auto increment primary key.
    async fn save(self, db: &DatabaseConnection) -> Result<Self, DbErr>
    where
        Self: ActiveModelBehavior,
        <Self::Entity as EntityTrait>::Model: IntoActiveModel<Self>,
    {
        let mut am = self;
        am = ActiveModelBehavior::before_save(am);
        let mut is_update = true;
        for key in <Self::Entity as EntityTrait>::PrimaryKey::iter() {
            let col = key.into_column();
            if am.is_unset(col) {
                is_update = false;
                break;
            }
        }
        if !is_update {
            am = am.insert(db).await?;
        } else {
            am = am.update(db).await?;
        }
        am = ActiveModelBehavior::after_save(am);
        Ok(am)
    }

    /// Delete an active model by its primary key
    async fn delete(self, db: &DatabaseConnection) -> Result<DeleteResult, DbErr>
    where
        Self: ActiveModelBehavior,
    {
        let mut am = self;
        am = ActiveModelBehavior::before_delete(am);
        let exec = Self::Entity::delete(am).exec(db);
        exec.await
    }
}

// ActiveModelBehavior 中定义用户可以自定义的行为
/// Behaviors for users to override
pub trait ActiveModelBehavior: ActiveModelTrait {
    /// Create a new ActiveModel with default values. Also used by `Default::default()`.
    fn new() -> Self {
        <Self as ActiveModelTrait>::default()
    }

    /// Will be called before saving
    fn before_save(self) -> Self {
        self
    }

    /// Will be called after saving
    fn after_save(self) -> Self {
        self
    }

    /// Will be called before deleting
    fn before_delete(self) -> Self {
        self
    }
}
```

ActiveModel 代表的就是 活动中的 数据模型，对应当前被操作的表数据。

Rails 中的 ActiveModel 还提供一些模型验证等丰富的功能，目前 SeaORM 中的 ActiveModel 抽象也正在完善相关功能，参见 [PR: Update  `ActiveModelBehavior`  API #210](https://github.com/SeaQL/sea-orm/pull/210) 。

**Entity 和 ActiveModel 抽象是 SeaORM 抽象架构的基石。**

#### DSL: 宏与代码生成

我们通过前面的示例看到 SeaORM 提供了一些 DSL 方法。除此之外，SeaORM 还提供了一些代码生成和宏，来方便开发。

**[sea-orm-cli](https://github.com/SeaQL/sea-orm/tree/master/sea-orm-cli)** 

为 `cargo run` 提供了命令参数 `-- generate entity` 根据数据库表自动生成 `Entity` 文件。

 ```rust
 # MySQL (`--database-schema` option is ignored) 
 cargo run -- generate entity -u mysql://sea:sea@localhost/bakery -o out
 
 # PostgreSQL
 cargo run -- generate entity -u postgres://sea:sea@localhost/bakery -s public -o out
 ```

内部是通过 [`sea-orm-codegen` 组件提供的 `transform` 来生成的 Entity 文件](https://github.com/SeaQL/sea-orm/blob/master/sea-orm-codegen/src/entity/transformer.rs) 。

**[`sea-orm-macros`](https://github.com/SeaQL/sea-orm/blob/master/sea-orm-macros/src/lib.rs)**

在 `sea-orm-macros` 组件中，实现了 `DeriveEntity`/ `DeriveColumn`/ `DerivePrimaryKey` /`DeriveModel`/`DeriveActiveModel`/ `DeriveActiveModelBehavior`等过程宏。

你可以通过 `cargo run -- generate entity` 来自动生成 Entity 文件，也可以通过这些过程宏自定义 Entity 文件。

#### 多数据库支持

SeaORM 的 `src` 目录下还有关于 `database`/ `driver`/ `query`/ `executor` 等模块，主要负责底层数据库交互了，这些功能主要基于 `sqlx` 和 `SeaQuery` 构建。

#### SeaQuery

 SeaQuery是一个查询生成器，是 SeaORM的基础，用来在Rust中构建动态SQL查询，使用一个符合人体工程学的 API 将表达式、查询和模式构建为抽象语法树（AST）。在同一个 接口 后面，统一支持MySQL、Postgres和SQLite。它类似于 Rails 的 ActiveRecord ORM 框架的 [Arel](https://github.com/rails/rails/tree/main/activerecord/lib/arel) 组件。

示例代码：

```rust
// 参数绑定
assert_eq!(
    Query::select()
        .column(Glyph::Image)
        .from(Glyph::Table)
        .and_where(Expr::col(Glyph::Image).like("A"))
        .and_where(Expr::col(Glyph::Id).is_in(vec![1, 2, 3]))
        .build(PostgresQueryBuilder),
    (
        r#"SELECT "image" FROM "glyph" WHERE "image" LIKE $1 AND "id" IN ($2, $3, $4)"#
            .to_owned(),
        Values(vec![
            Value::String(Some(Box::new("A".to_owned()))),
            Value::Int(Some(1)),
            Value::Int(Some(2)),
            Value::Int(Some(3))
        ])
    )
);

// 动态查询
Query::select()
    .column(Char::Character)
    .from(Char::Table)
    .conditions(
        // some runtime condition
        true,
        // if condition is true then add the following condition
        |q| {
            q.and_where(Expr::col(Char::Id).eq(1));
        },
        // otherwise leave it as is
        |q| {},
    );

// 生成查询 SQL 
let query = Query::select()
    .column(Char::Character)
    .column((Font::Table, Font::Name))
    .from(Char::Table)
    .left_join(Font::Table, Expr::tbl(Char::Table, Char::FontId).equals(Font::Table, Font::Id))
    .and_where(Expr::col(Char::SizeW).is_in(vec![3, 4]))
    .and_where(Expr::col(Char::Character).like("A%"))
    .to_owned();

assert_eq!(
    query.to_string(MysqlQueryBuilder),
    r#"SELECT `character`, `font`.`name` FROM `character` LEFT JOIN `font` ON `character`.`font_id` = `font`.`id` WHERE `size_w` IN (3, 4) AND `character` LIKE 'A%'"#
);
assert_eq!(
    query.to_string(PostgresQueryBuilder),
    r#"SELECT "character", "font"."name" FROM "character" LEFT JOIN "font" ON "character"."font_id" = "font"."id" WHERE "size_w" IN (3, 4) AND "character" LIKE 'A%'"#
);
assert_eq!(
    query.to_string(SqliteQueryBuilder),
    r#"SELECT `character`, `font`.`name` FROM `character` LEFT JOIN `font` ON `character`.`font_id` = `font`.`id` WHERE `size_w` IN (3, 4) AND `character` LIKE 'A%'"#
);
```



## 小结

SeaORM 目前只是 0.2 版本，对比 Rails 的 ActiveRecord 来看，SeaORM 还有很长的路要走。通过这篇文章，我们大概对 SeaORM 有了高屋建瓴的理解，为使用 SeaORM 或 给 SeaORM 做贡献打一个基础。












