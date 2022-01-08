# Rust 裸函数相关新RFC 和 序列化

> 本篇将会简要介绍什么是《This Week in Rust》，第418、419篇推文中有关Rust新RFC，第417篇推文中有关Rust Serde实现序列化相关的内容。

## RFC 2972：Constrained Naked Functions

裸函数（Naked Functions）可以像普通函数一样定义，但是实际上与普通的函数不同。裸函数不会入栈，也就是说，并不像普通函数在被调用或结束调用时，会存储或恢复寄存器，裸函数没有函数序言（Prologue）和尾声（Epilogue），而是更类似于C中的`GOTO`标签。如果在裸函数中写`return`，此种行为将是未定义的，因为并不像普通函数返回时读取寄存器中存储的地址，没有函数序言和尾声的裸函数会读取调用者存储的`return`地址，`return`并不会像想象中的逻辑一样被执行。

在此前，Rust复制了其他编译器支持裸函数的方式，以在Rust中实现这个功能，但是由于裸函数的未定义行为以及复杂的管理模式，往往不被推荐使用。RFC 2972尝试更好地在Rust中限定与定义裸函数，使其便利性能够被利用。

在Rust中，裸函数可以通过`#[naked]`来表示，函数体应只包含汇编代码。详细的规则可见[RFC 2972](https://github.com/rust-lang/rfcs/blob/master/text/2972-constrained-naked.md)，概述如下：

- 除了`extern 'Rust'`以外，必须声明调用规则
- 应当只规定FFI安全参数以及返回值
- 不可以使用`#[inline]`或`#[inline(*)]`属性
- 函数体只包含单一`asm!()`表达式，且该表达式：
  - 除了`const`或`sym`不能包含任何其他操作符，避免使用或引用栈上变量导致未定义行为
  - 必须包含`noreturn`选项
  - 除了`att_syntax`以外不能包含其他选项
  - 必须保证函数是`unsafe`的或满足调用规则

编译器将不允许其中出现未使用的变量，并且隐式使用属性`#[inline(never)]`。使用例子如下：

```rust
const THREE: usize = 3;

#[naked]
pub extern "sysv64" fn add_n(number: usize) -> usize {
    unsafe {
        asm!(
            "add rdi, {}"
            "mov rax, rdi"
            "ret",
            const THREE,
            options(noreturn)
        );
    }
}
```

例中调用规范为`sysv64`，因此函数输入位于`	rdi`寄存器，函数返回位于`rax`寄存器。

不过，这个方案依然存在缺陷。这样的实现方法无法兼容目前nightly版本的`#[naked]`属性使用，也会改变`asm!()`的参数定义。不作为`asm!()`操作符的寄存器原本定义为包含未定义值，然而在修订后裸函数的语境下，这个定义被更新为初始寄存器状态不受函数调用修改。其次这些定义相较于原始的裸函数定义，或许过为严格。在裸函数中使用Rust而不是汇编理论可行，但是实际上也非常困难。最后，不同架构对于裸函数的支持或许不同。



## RFC 3180：Cargo --crate-type CLI Argument

[`crate-type`](https://doc.rust-lang.org/cargo/reference/cargo-targets.html)属性定义了`cargo build`会生成的目标crate类型，只能对于库（Library）与例子（Example）声明。二进制（Binaries）、测试（Tests）以及Benchmarks永远都是"bin"类型。

默认类型如下：

| Target             | Crate Type     |
| ------------------ | -------------- |
| Normal library     | `"lib"`        |
| Proc-macro library | `"proc-macro"` |
| Example            | `"bin"`        |

可声明选项具体解释可见"[Linkage](https://doc.rust-lang.org/reference/linkage.html#linkage)"。概括如下：

- `bin`：可执行文件。
- `lib`：Rust库，具体类型以及表达方式由编译器决定，输出库可以被rustc使用。
- `rlib`：Rust库文件，类似于静态库，但是会在未来的编译中被编译器链接，这一点形似动态库。
- `dylib`：动态库，可以作为其他库或是可执行文件依赖，文件类型在Linux中为`*.so`，macOS中为`*.dylib`，Windows中为`*.dll`。
- `cdylib`：动态系统库，当编译需要提供给其他语言调用的库时使用，文件类型在Linux中为`*.so`，macOS中为`*.dylib`，Windows中为`*.dll`。
- `staticlib`：静态库，编译器永远不会尝试链接静态库，库会包括所有的本地crate代码以及所有上游依赖项，文件类型在Linux、macOS、Windows（minGW）中为`*.a`，Windows（MSVC）中为`*.lib`。
- `proc-macro`：输出形式将不在此说明，但是要求提供`-L`路径，编译器将认为输出文件是一个可以用于某个程序的宏，并且依赖于当前架构编译。由这个选项编译的crate应当只输出过程宏。

[RFC 3180](https://github.com/rust-lang/rfcs/pull/3180)新增了Cargo的命令行使用功能，`--crate-type <crate-type>`可以作为命令行参数输入了，功能与在`Cargo.toml`中定义`crate-type`一致，不过享有更高的优先级。也就是说，如果命令行参数定义值与`Cargo.toml`中定义值冲突，Cargo会认为该值与命令行输入相同。



## 序列化与Serde

序列化（Serialization）是将对象的状态信息转换为可以存储或传输的形式的过程，反序列化（Deserialization）则是其逆过程。[`Serde`](https://serde.rs/)是Rust的一个序列化与反序列化框架，可以解析Json、Yaml、Toml、Bson以及其他一些序列化文件。`serde_json`则基于此框架，实现Json文件序列化与反序列化。



### Serde

Serde通过trait实现序列化与反序列化。如果一个数据结构实现了Serde的`Serialize`与`Deserialize` trait，那么它就可以被序列化或反序列化。



#### `Serialize`

Serde的`Serialize` trait[定义](https://docs.serde.rs/serde/trait.Serialize.html)如下：

```rust
pub trait Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}
```

实现`Serialize` trait的实例可以调用方法`serialize`来序列化，并使用对应的实现`Serializer` trait的实例作为参数。`Serializer` trait定义如下：

```rust
pub trait Serializer: Sized {
    type Ok;
    type Error: Error;
    type SerializeSeq: SerializeSeq<Ok = Self::Ok, Error = Self::Error>;
    type SerializeTuple: SerializeTuple<Ok = Self::Ok, Error = Self::Error>;
    type SerializeTupleStruct: SerializeTupleStruct<Ok = Self::Ok, Error = Self::Error>;
    type SerializeTupleVariant: SerializeTupleVariant<Ok = Self::Ok, Error = Self::Error>;
    type SerializeMap: SerializeMap<Ok = Self::Ok, Error = Self::Error>;
    type SerializeStruct: SerializeStruct<Ok = Self::Ok, Error = Self::Error>;
    type SerializeStructVariant: SerializeStructVariant<Ok = Self::Ok, Error = Self::Error>;
    
    // 需要实现的方法
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error>;
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error>;
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error>;
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error>;
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error>;
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error>;
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error>;
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error>;
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error>;
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error>;
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error>;
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error>;
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error>;
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error>;
    fn serialize_none(self) -> Result<Self::Ok, Self::Error>;
    fn serialize_some<T: ?Sized>(
        self, 
        value: &T
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize;
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error>;
    fn serialize_unit_struct(
        self, 
        name: &'static str
    ) -> Result<Self::Ok, Self::Error>;
    fn serialize_unit_variant(
        self, 
        name: &'static str, 
        variant_index: u32, 
        variant: &'static str
    ) -> Result<Self::Ok, Self::Error>;
    fn serialize_newtype_struct<T: ?Sized>(
        self, 
        name: &'static str, 
        value: &T
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize;
    fn serialize_newtype_variant<T: ?Sized>(
        self, 
        name: &'static str, 
        variant_index: u32, 
        variant: &'static str, 
        value: &T
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize;
    fn serialize_seq(
        self, 
        len: Option<usize>
    ) -> Result<Self::SerializeSeq, Self::Error>;
    fn serialize_tuple(
        self, 
        len: usize
    ) -> Result<Self::SerializeTuple, Self::Error>;
    fn serialize_tuple_struct(
        self, 
        name: &'static str, 
        len: usize
    ) -> Result<Self::SerializeTupleStruct, Self::Error>;
    fn serialize_tuple_variant(
        self, 
        name: &'static str, 
        variant_index: u32, 
        variant: &'static str, 
        len: usize
    ) -> Result<Self::SerializeTupleVariant, Self::Error>;
    fn serialize_map(
        self, 
        len: Option<usize>
    ) -> Result<Self::SerializeMap, Self::Error>;
    fn serialize_struct(
        self, 
        name: &'static str, 
        len: usize
    ) -> Result<Self::SerializeStruct, Self::Error>;
    fn serialize_struct_variant(
        self, 
        name: &'static str, 
        variant_index: u32, 
        variant: &'static str, 
        len: usize
    ) -> Result<Self::SerializeStructVariant, Self::Error>;
    
    // 已实现方法
    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> { ... }
    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> { ... }
    fn collect_seq<I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Serialize,
    { ... }
    fn collect_map<K, V, I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        K: Serialize,
        V: Serialize,
        I: IntoIterator<Item = (K, V)>,
    { ... }
    fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Display,
    { ... }
    fn is_human_readable(&self) -> bool { ... }
}
```

Serde试图通过此trait中的方法，将所有的Rust数据结构分类至某一种可能的类型中。当为某个数据类型实现`Serialize`时，只要调用对应的方法即可。例如：

```rust
impl<T> Serialize for Option<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Some(ref value) => serializer.serialize_some(value),
            None => serializer.serialize_none(),
        }
    }
}
```

`Serialize` trait已对大部分原始类型实现，具体列表可见["Module serde::ser"](https://docs.serde.rs/serde/ser/index.html)。`Serializer` trait的实现则由第三方库或用户自定义，如`serde_json`。



#### `serde_json`与序列化

`serde_json`序列化使用如下例：

```rust
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Address {
	street: String,
	city: String,
}

let address = Address {
	street: "10 Downing Street".to_owned(),
	city: "London".to_owned(),
};

// Serialize it to a JSON string.
let j = serde_json::to_string(&address)?;

// Print, write to a file, or send to an HTTP server.
println!("{}", j);
```

或是对于一个Json对象，`Value`进行序列化操作：

```rust
let John = json!({
	"name": "John Doe",
	"age": 43,
	"phones": [
		"+44 1234567",
		"+44 2345678"
	]
});

println!("first phone number: {}", john["phones"][0]);

// Convert to a string of JSON and print it out
println!("{}", john.to_string());
```

`serde_json`提供枚举`Value`，成员类型与Json对应，包括`Null`，`Bool`，`Number`，`String`，`Array`，`Object`，实现`Display` trait与`to_string`方法，以此将Json对象（在`serde_json`中，即`Value`实例）转换为字符串。`to_string`通过一个实现`ser::Serializer` trait并包含`Writer`与`Formatter`的同名结构体`Serializer`对象来实现，`Display`的成员方法`fmt`会对于`Value`对象调用`serialize`方法，而`serialize`方法会调用该同名结构体对象。该对象实现`ser::Serializer`，在各个序列化方法中调用`Formatter`的方法函数写出字符串至`Writer`。`Formatter`则负责书写的格式，例如match布尔值然后输出对应的字符串"false"或"true"，以及在每个序列化字符串之前和之后添加相应字符。

对于实现`Serialize` trait的对象，可以通过`to_value`方法将其转换为`Value`对象，`to_value`将会调用`value.serialize`，函数参数为`serde_json::value::Serializer`。`serde_json/value/ser.rs`定义了同名结构体`Serializer`，根据Json对象规则，将输入转换为对应的`Value`对象。

对于自定义的struct，可以通过`#[derive(Serialize)]`来使其获得该属性。`#[derive(Serialize)]`过程宏的定义`fn derive_serialize(input: TokenStream)`在`serde/serde_derive` crate中，并依赖于rust库，`proc-macro2`与`syn`。`proc-macro2`是编译器crate `proc_macro`的包装，提供了新的接口以及功能，例如实现`syn`和`quote`。`syn`是Rust源码解析库，可以将Rust代码解析成抽象语法树，`quote`则提供了将抽象语法树转换为Rust源码的功能。`derive_serialize`宏将输入的`TokenStream`使用`as`显示转换为`syn::DeriveInput`，也就是将目标结构体或枚举递归解析为语法树，并记录它的属性、可见性、名字、使用到的泛型、以及数据。`ser::expand_derive_serialize`则接受这个语法树作为参数，对其进行一系列过滤与再处理后，使用`quote`库实现`impl serde::Serialize`的过程宏。在`ser.rs`中，`serde_derive`提供了例如`serialize_enum`、`serialize_struct`、`serialize_tuple_struct`、`serialize_variant`等函数，对于不同的对象类型实现对应的`impl`源码。

例如，对于自定义结构体`Person`，`#[derive(Serialize)]`展开结果如下：

```rust
use serde::ser::{Serialize, SerializeStruct, Serializer};

struct Person {
	name: String,
    age: u8,
    phones: Vec<String>,
}

// This is what #[derive(Serialize)] would generate.
impl Serialize for Person {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
	{
		let mut s = serializer.serialize_struct("Person", 3)?;
		s.serialize_field("name", &self.name)?;
		s.serialize_field("age", &self.age)?;
		s.serialize_field("phones", &self.phones)?;
		s.end()
	}
}
```

依赖于在`serde_json`中已经定义好的`Serializer`，可以将`Person`实例转换为对应的Json对象或是Json格式字符串。

此外，`serde_json`提供了宏`json!()`，可以将Json格式的token串转换为`Value`实例，例如，

```rust
use serde_json::json;

let value = json!({
	"code": 200,
	"success": true,
	"payload": {
		"features": [
			"serde",
			"json"
		]
	}
});
```

该宏将输入解析为抽象语法树，并定义内部宏`json_internal`，对于不同的语法树匹配做不同的处理，并且生成对应的`Value`实例。



#### `Deserialize`

Serde的`Deserialize` trait[定义](https://docs.serde.rs/serde/trait.Serialize.html)如下：

```rust
pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}
```

同样，实现`Deserialize` trait的实例可以调用方法`deserialize`来反序列化，即从某个格式的数据输入中恢复实例。方法`deserialize`将使用实现了`Deserializer` trait的实例作为参数。`Deserializer` trait定义如下：

```rust
pub trait Deserializer<'de>: Sized {
    type Error: Error;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_byte_buf<V>(
        self, 
        visitor: V
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_unit_struct<V>(
        self, 
        name: &'static str, 
        visitor: V
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_newtype_struct<V>(
        self, 
        name: &'static str, 
        visitor: V
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_tuple<V>(
        self, 
        len: usize, 
        visitor: V
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_tuple_struct<V>(
        self, 
        name: &'static str, 
        len: usize, 
        visitor: V
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_struct<V>(
        self, 
        name: &'static str, 
        fields: &'static [&'static str], 
        visitor: V
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_enum<V>(
        self, 
        name: &'static str, 
        variants: &'static [&'static str], 
        visitor: V
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_identifier<V>(
        self, 
        visitor: V
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_ignored_any<V>(
        self, 
        visitor: V
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    { ... }
    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    { ... }
    fn is_human_readable(&self) -> bool { ... }
}
```

对于某种格式的数据输入，`Deserializer` trait提供了转换数据为Rust实例的方法。与`Serialize` trait相同，当为某个数据类型实现`Deserialize`时并希望以后通过这个trait恢复数据结构时，只要调用对应的方法即可。例如：

```rust
#[cfg(any(feature = "std", feature = "alloc"))]
impl<'de> Deserialize<'de> for String {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(StringVisitor)
    }

    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(StringInPlaceVisitor(place))
    }
}
```

同样，虽然与实现`Serialize` trait的原始类型列表不同，`Deserialize` trait也对很多原始类型实现，具体列表可见["Module serde::de"](https://docs.serde.rs/serde/de/index.html)。`Deserializer` trait的实现同样由第三方库或用户自定义。



##### `serde_json`与反序列化

`serde_json`反序列化使用如下例：

```rust
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
	name: String,
	age: u8,
	phones: Vec<String>,
}

// Some JSON input data as a &str. Maybe this comes from the user.
let data = r#"
	{
		"name": "John Doe",
		"age": 43,
		"phones": [
			"+44 1234567",
			"+44 2345678"
		]
	}"#;

// Parse the string of data into serde_json::Value.
let v: Value = serde_json::from_str(data)?;

// Parse the string of data into a Person object. This is exactly the
// same function as the one that produced serde_json::Value above, but
// now we are asking it for a Person as output.
let p: Person = serde_json::from_str(data)?;

// Do things just like with any other Rust data structure.
println!("Please call {} at the number {}", p.name, p.phones[0]);
```

`from_str`定义如下：

```rust
fn from_trait<'de, R, T>(read: R) -> Result<T>
where
    R: Read<'de>,
    T: de::Deserialize<'de>,
{
    let mut de = Deserializer::new(read);
    let value = tri!(de::Deserialize::deserialize(&mut de));

    // Make sure the whole stream has been consumed.
    tri!(de.end());
    Ok(value)
}
```

`from_str`会间接调用指定实例的`de::Deserialize::deserialize(&mut de)`方法，实例类型由赋值时指定变量的类型决定。同名结构体`Deserializer`负责将Json数据转换为`Value`对象，实现`de::Deserializer` trait，并实现解析方法。例如，对于给定输入，`fn deserialize_any<V>(self, visitor: V)`会`peek`第一个字符，并进入选择分支。对于布尔值匹配如下：

```rust
b't' => {
    self.eat_char();
    tri!(self.parse_ident(b"rue"));
    visitor.visit_bool(true)
}
b'f' => {
    self.eat_char();
    tri!(self.parse_ident(b"alse"));
    visitor.visit_bool(false)
}
```

`tri!`实现相当于`Result::map`的功能，`parse_ident`会逐字检查输入是否符合预期值，例如检查布尔值是否是"true"或"false"。`visitor`实例实现`serde::de::Visitor` trait，表明给定的输入是否满足预期格式。在布尔值匹配的例子中，根据给定的`visitor`实例规则，`visit_bool`方法将返回对应的值或错误。

对于`Value`实例，`de::Deserialize::deserialize(deserializer: serde::Deserializer<'de>)`方法中定义了`ValueVisitor`，实现`serde::de::Visitor` trait的各个方法，并规定当给定输入满足Json格式时，返回相应的值，反之则报错，随后调用`deserializer.deserialize_any(ValueVisitor)`，从而将Json格式字符串转换为对应的`Value`实例。

过程宏`#[derive(Deserialize)]`的实现与`#[derive(Serialize)]`类似，同样先将目标结构体源码解析为语法树，随后调用`de::expand_derive_deserialize`。这个方法的代码逻辑和`ser::expand_derive_serialize`一致，并且同样在`de.rs`中提供`deserialize_enum`、`deserialize_struct`、`deserialize_tuple`等函数实现`impl`源码。这些函数同样利用`quote`库，实现`Visitor`实例，并将其传递给`deserializer.deserialize_any`，从而实现`Deserialize` trait。
