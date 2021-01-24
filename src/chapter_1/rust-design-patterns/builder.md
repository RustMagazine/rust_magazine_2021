# 建造者模式（Builder)

## 描述

通过使用构建者助手创建一个对象。

## 例子

```rust
fn main() {
    let foo = Foo {
        bar: String::from("Y"),
    };
    let foo_from_builder = FooBuilder::new().name(String::from("Y")).build();
    println!("foo = {:?}", foo);
    println!("foo from builfer = {:?}", foo_from_builder);
}

#[derive(Debug, PartialEq)]
pub struct Foo {
    // lots of complicated fields
    bar : String,
}

pub struct FooBuilder {
    // Probably lots of optional fields.
    bar: String,
}

impl FooBuilder {
    pub fn new() -> Self {
        // set the minimally required fields of Foo.
        Self {
            bar: String::from("x"),
        }
    }

    pub fn name(mut self, bar: String) -> FooBuilder {
        // set the name on the builder iteself,
        // and return the builder by value.
        self.bar = bar;
        self 
    }
    // if we can get away with not consuming the builder here, that is an 
    // advantage. It means we can use the FooBuilder as a template for constructing many Foo.
    pub fn build(self) -> Foo {
        // Create a Foo from Foo the FooBuilder, applying all settings in FooBuilder to Foo. 
        Foo { bar: self.bar }
    }
}
```

```rust
// Rust 编程之道. P234
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
    fn new() -> CircleBuilder {
        CircleBuilder {
            x: 0.0, y: 0.0, radius: 1.0,
        }
    }
}

impl CircleBuilder {
    fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.x = coordinate;
        self
    }
    fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.y = coordinate;
        self
    }
    fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
        self.radius = radius;
        self
    }

    fn build(&self) -> Circle {
        Circle {
            x: self.x, y: self.y, radius: self.radius,
        }
    }
}

fn main() {
  let c = Circle::new().x(1.0).y(2.0).radius(2.0).build();
  println!("area = {:?}", c.area());
  println!("c.x = {:?}", c.x);
  println!("c.y = {:?}", c.y);
}
```

## 动机

当你需要许多不同的构造函数或者当构造有副作用时，这种方法有用。

## 优点

将构造方法与其他方法分离。

防止构造函数的扩散

可用于单次初始化以及更加复杂的构造。

## 缺点

比直接创建结构对象或简单的的构造函数更复杂。

## 讨论

这种模式在Rust（以及简单对象）中比在其他许多语言中更常见，这是因为Rust缺乏重载。由于你只能使用给定名称的单个方法，因此在Rust中使用多个构造函数要比C++、Java或其他语言好。

这种模式通常用于构建器对象本身就很有用的地方，而不仅仅是一个构建器。例如：std::process::Command 是Child的构建器。在这种情况下，不使用T和TBuilder的命名模式。

该示例通过值获取并返回生成器。接受并返回构建器作为可变引用通常更符合人体工程学（并且更有效）。

```rust
let mut fb = FooBuilder::new();
fb.a();
fb.b();
let f = fb.builder();
```

以及FooBuilder::new().a().b().builder()样式。

## 参见

- [Description in the style guide](https://web.archive.org/web/20210104103100/https://doc.rust-lang.org/1.12.0/style/ownership/builders.html)
- [derive_builder](https://crates.io/crates/derive_builder), a crate for automatically implementing this pattern while avoiding the boilerplate.
- [Constructor pattern](https://rust-unofficial.github.io/patterns/idioms/ctor.html) for when construction is simpler.
- [Builder pattern (wikipedia)](https://en.wikipedia.org/wiki/Builder_pattern)
- [Construction of complex values](https://web.archive.org/web/20210104103000/https://rust-lang.github.io/api-guidelines/type-safety.html#c-builder)
- Rust编程之道 ch7,p234