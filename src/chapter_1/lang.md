# 本月简报：官方动态

- 来源：[Rust日报](https://rustcc.cn/section?id=f4703117-7e6b-4caf-aa22-a3ad3db6898f)
- 作者：Rust 日报小组

### Rust 1.49 稳定版发布

2020年最后一天，Rust 1.49 稳定版发布了。稳定版 Rust 发布周期为六周一次。

值得关注的更新：

- `aarch64-unknown-linux-gnu` 升级为`Tier 1`。
- `aarch64-apple-darwin` 和`aarch64-pc-windows-msvc` 得到`Tier 2` 级别的支持。
- 单元测试中线程中的print输出将会被捕获，默认不会在控制台打印出来了。如果不- 需要捕获，需要添加--nocapture参数。
- `union`支持 `impl Drop trait` 了
支持使用`ref`关键字让解构的字段不再被move 而是被借用。

```rust,editable
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main(){
    let person = Person {
    name: String::from("Alice"),
    age: 20,
    };

    // `name` is moved out of person, but `age` is referenced.
    let Person { name, ref age } = person;
    println!("{} {}", name, age);
}

```

[https://blog.rust-lang.org/2020/12/31/Rust-1.49.0.html](https://blog.rust-lang.org/2020/12/31/Rust-1.49.0.html)

### Rust 将不再支持 Windows XP



[https://github.com/rust-lang/compiler-team/issues/378](https://github.com/rust-lang/compiler-team/issues/378)