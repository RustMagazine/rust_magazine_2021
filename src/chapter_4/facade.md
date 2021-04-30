# 真实世界的设计模式 | 外观模式（Facade Pattern）

作者：张汉东 / 编辑：张汉东

> 编者按：
>
> 本文摘录自[开源电子书《Real World Rust Design Pattern》](https://github.com/ZhangHanDong/real-world-rust-design-pattern)，这本书也是我创建的免费开源电子书，目前正在逐步完善中，欢迎贡献。
>
> 这本书旨在挖掘和记录 Rust 开源生态中设计模式的真实实践。欢迎参与贡献！

---

# Facade（外观）模式

Rust 中最常用的设计模式是哪个？答案是，外观模式。

为什么这么说？看完本文就明白了。


## 一句话介绍

Facade，中文术语叫「外观模式」，也叫「门面模式」。在经典设计模式中，归为结构型（Structural）模式分类，因为这种模式用于帮助构建结构。它可以为程序库、框架或其他复杂情况提供一个简单的接口。

## 解决了什么问题

在软件开发中，有时候要处理很多同类型的业务，但具体处理方式却不同的场景。因此，建立一个「门面」来达到统一管理和分发的目的。

Facade 模式，帮忙建立了统一的接口，使得调用复杂的子系统变得更加简单。因为 Facade 模式只包括应用真正关心的核心功能。


## 如何解决

心智图：


```text
                 +------------------+
+-------+        |                  |         +---------------+
|       |        |                  |         |  additional   |
|client +------> |     facade       +-------> |  facade       |
+-------+        |                  |         |               |
                 |                  |         |               |
                 +--+----+------+--++         +---------------+
                    |    |      |  |
           +--------+    |      |  +--------+
           |          +--+      +-+         |
           |          |           |         |
           v          |           v         v
       +---+---+  +---v--+   +----+--+  +---+----+
       |       |  |      |   |       |  |        |
       | system|  |system|   |system |  | system |
       |       |  |      |   |       |  |        |
       +-------+  +------+   +-------+  +--------+

```

## 真实案例

**实现方式：**

Rust 中的 门面模式 实现有三类：

- 模块 re-export： 
    - [Rust libstd reexport libcore](https://github.com/rust-lang/rust/tree/master/library/std/src/sys)
    - [Futures-rs](https://github.com/rust-lang/futures-rs/blob/master/futures/src/lib.rs)
- 条件编译：[tikv/tikv](https://github.com/tikv/tikv/tree/master/components/tikv_alloc)
- 利用 「类型」 和 「Trait」： 
    - [log](https://github.com/rust-lang/log)
    - [mio](https://github.com/tokio-rs/mio)
    - [cranelift]()
        - [MachBackend](https://github.com/bytecodealliance/wasmtime/search?q=MachBackend)
        - [LowerBackend](https://github.com/bytecodealliance/wasmtime/search?q=LowerBackend)


### 模块 Re-Export

模块 Re-Export 是重导出功能。

比如，现在有如下模块层级：

```rust
src/
    - lib.rs
    - module/
        -- mod.rs
        -- submodule/
            --- mod.rs
```

Rust 允许你将 潜入到最深处的那个模块 submodule 里定义的函数，使用重导出功能，变成整个库的「门面」接口。

```rust
// in module/submodule/mod.rs
pub fn goodbye(){}

// in lib.rs
pub use module::submodule::goodbye;
```

那么在使用这个库（假设叫 hello）的时候，只需要使用 `hello::goodby`就可以使用这个函数。

这种方式在 Rust 的世界大量使用。比如 标准库 很多接口是重导出了 核心库 的 API。

在 Furutes-rs 中也有很多重导出。


### 条件编译

条件编译也是一种 门面模式。

比如在 TiKV 中，使用 条件编译 和 features 来支持多种内存分配器。

```rust
#[cfg(all(unix, not(fuzzing), feature = "jemalloc"))]
#[path = "jemalloc.rs"]
mod imp;
#[cfg(all(unix, not(fuzzing), feature = "tcmalloc"))]
#[path = "tcmalloc.rs"]
mod imp;
#[cfg(all(unix, not(fuzzing), feature = "mimalloc"))]
#[path = "mimalloc.rs"]
mod imp;
#[cfg(not(all(
    unix,
    not(fuzzing),
    any(feature = "jemalloc", feature = "tcmalloc", feature = "mimalloc")
)))]
#[path = "system.rs"]
mod imp;
```

实际上并不存在 imp 模块，通过不同的 `cfg` 判断，对应不同的 `path`，从而选择相应的模块：`jemalloc.rs`/`tcmalloc.rs`/`mimalloc.rs`/`system.rs`。而 imp 模块就是一个「门面」。

### 利用 类型 和 Trait

第三种方式，就是常规的 利用 类型 和 trait 来实现门面模型。

**最典型的就是官方出的 log 库。**

```rust
pub trait Log: Sync + Send {
    /// Determines if a log message with the specified metadata would be
    /// logged.
    ///
    /// This is used by the `log_enabled!` macro to allow callers to avoid
    /// expensive computation of log message arguments if the message would be
    /// discarded anyway.
    fn enabled(&self, metadata: &Metadata) -> bool;

    /// Logs the `Record`.
    ///
    /// Note that `enabled` is *not* necessarily called before this method.
    /// Implementations of `log` should perform all necessary filtering
    /// internally.
    fn log(&self, record: &Record);

    /// Flushes any buffered records.
    fn flush(&self);
}
```

官方通过指定这个 trait ，来创建了一个 「门面」。其他 log 库，比如 env_log / sys_log 等其他 log 库，都可以实现 `Log` trait。


```rust
// env_log
impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.filter.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        if self.matches(record) {
            // ignore many codes
        }
    }

    fn flush(&self) {}
}


// syslog

impl Log for BasicLogger {
  fn enabled(&self, metadata: &Metadata) -> bool {
    true
  }

  fn log(&self, record: &Record) {
    //FIXME: temporary patch to compile
    let message = format!("{}", record.args());
    let mut logger = self.logger.lock().unwrap();
    match record.level() {
      Level::Error => logger.err(message),
      Level::Warn  => logger.warning(message),
      Level::Info  => logger.info(message),
      Level::Debug => logger.debug(message),
      Level::Trace => logger.debug(message)
    };
  }

  fn flush(&self) {
      let _ = self.logger.lock().unwrap().backend.flush();
  }
}

```

这样，不管用户使用哪个 log 库，行为是一样的，达到了一致的用户体验。

**第二个例子是 mio 库。**

mio 库中的 poll 方法，就使用了门面模式。

```rust

pub struct Poll {
    registry: Registry,
}

/// Registers I/O resources.
pub struct Registry {
    selector: sys::Selector,
}

impl Poll {
    /// Create a separate `Registry` which can be used to register
    /// `event::Source`s.
    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    pub fn poll(&mut self, events: &mut Events, timeout: Option<Duration>) -> io::Result<()> {
        self.registry.selector.select(events.sys(), timeout)
    }
}
```

mio 是实现了跨平台的非阻塞I/O接口的 Rust 抽象，通过实现 Poll 这样一个门面，屏蔽了底层不同平台的 I/O 系统调用细节，比如 epoll/kqueue/IOCP。

**第三个案例是 Cranelift**

Cranelift 是一个编译器，目前用于 wasmtime 和 rustc debug 模式下。最近 Cranelift 在重构新的 后端，以支持不同的架构平台：Arm/X86等。

在 Cranelift 内部通过一个 `MachBackend` trait 来抽象出一个 后台门面，只关心核心逻辑：编译给定的函数。

```rust
/// Top-level machine backend trait, which wraps all monomorphized code and
/// allows a virtual call from the machine-independent `Function::compile()`.
pub trait MachBackend {
    /// Compile the given function.
    fn compile_function(
        &self,
        func: &Function,
        want_disasm: bool,
    ) -> CodegenResult<MachCompileResult>;

    // ignore others functions
}
```

然后给不同的平台来实现这个 trait：

```rust

impl MachBackend for AArch64Backend {
    fn compile_function(
        //...
    ){/* ... */}
}

impl MachBackend for X64Backend {
    fn compile_function(
        //...
    ){/* ... */}
}

impl MachBackend for Arm32Backend {
    fn compile_function(
        //...
    ){/* ... */}
}

```

然后在上层代码 Context 接口调用 [compile_and_emit](https://github.com/bytecodealliance/wasmtime/blob/main/cranelift/codegen/src/context.rs#L129) 方法时，就可以按当前平台信息生成相应指令：

```rust
pub fn compile_and_emit(/*...*/){

    // ...
    let info = self.compile(isa)?;
    //
}

pub fn compile(&mut self, isa: &dyn TargetIsa) -> CodegenResult<CodeInfo> {
    // ...
    if let Some(backend) = isa.get_mach_backend() {
        let result = backend.compile_function(&self.func, self.want_disasm)?; // 调用 compile_function
        let info = result.code_info();
        self.mach_compile_result = Some(result);
        Ok(info)
    } 
    // ...

}

// cranelift/codegen/src/machinst/adapter.rs 
// 返回 MachBackend 对象
fn get_mach_backend(&self) -> Option<&dyn MachBackend> {
    Some(&*self.backend)
}

```

所以，整个调用流程是：`Context -> compile_and_emit -> compile -> get_mach_backend -> compile_function` ，然后到各个架构平台。


## 结语

综上，门面模式是 Rust 应用最广泛的一个设计模式。感谢阅读，如有错漏，欢迎反馈和补充。

