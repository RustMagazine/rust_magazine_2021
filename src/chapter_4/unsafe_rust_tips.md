---
pub_date: Thu, 30 Apr 2021 18:00:00 GMT
description: Unsafe Rust Tips 01

---

# Unsafe Rust 编码技巧 | 01

编辑：张汉东

> 汇总一些 Unsafe Rust 编码技巧，欢迎补充。

---

## 编写 Unsafe Rust 的标准规范

来源：[wasmtime](https://github.com/bytecodealliance/wasmtime/blob/main/crates/wasmtime/src/func/typed.rs)


```rust
    /// Invokes this WebAssembly function with the specified parameters.
    ///
    /// Returns either the results of the call, or a [`Trap`] if one happened.
    ///
    /// For more information, see the [`Func::typed`] and [`Func::call`]
    /// documentation.
    ///
    /// # Panics
    ///
    /// This function will panic if it is called when the underlying [`Func`] is
    /// connected to an asynchronous store.
    pub fn call(&self, params: Params) -> Result<Results, Trap> {
        assert!(
            !cfg!(feature = "async") || !self.func.store().async_support(),
            "must use `call_async` with async stores"
        );
        unsafe { self._call(params) }
    }

```

当函数中调用了 Unsafe 函数，必须对其进行安全抽象。

上面代码示例中，使用 `assert!` 宏，将 `_call`调用控制在了安全边界内，所以函数 `call` 目前是一个安全的函数，所以不需要在 `fn` 前面增加 `unsafe` 标签。

```rust
    unsafe fn _call(&self, params: Params) -> Result<Results, Trap> {
        // Validate that all runtime values flowing into this store indeed
        // belong within this store, otherwise it would be unsafe for store
        // values to cross each other.
        if !params.compatible_with_store(&self.func.instance.store) {
            return Err(Trap::new(
                "attempt to pass cross-`Store` value to Wasm as function argument",
            ));
        }

        // ...
        // ignore others codes
        // ...

        // This can happen if we early-trap due to interrupts or other
        // pre-flight checks, so we need to be sure the parameters are at least
        // dropped at some point.
        if !called {
            drop(params.assume_init());
        }
        debug_assert_eq!(result.is_ok(), returned);
        result?;

        Ok(ret.assume_init())
    }
```

对于 `_call` 函数来说，因为无法在函数内验证所有传入的运行时值是否在合法的安全边界，所以需要将其标记为 Unsafe 函数，即在 `fn` 前面加上 `unsafe` 标签。除此之外，还必须在函数内脆弱的地方，加上必须的注释来说明什么情况下会突破安全边界。

```rust
/// A trait implemented for types which can be arguments and results for
/// closures passed to [`Func::wrap`] as well as parameters to [`Func::typed`].
///
/// This trait should not be implemented by user types. This trait may change at
/// any time internally. The types which implement this trait, however, are
/// stable over time.
///
/// For more information see [`Func::wrap`] and [`Func::typed`]
pub unsafe trait WasmTy {
    #[doc(hidden)]
    type Abi: Copy;
    #[doc(hidden)]
    #[inline]
    fn typecheck(ty: crate::ValType) -> Result<()> {
        if ty == Self::valtype() {
            Ok(())
        } else {
            bail!("expected {} found {}", Self::valtype(), ty)
        }
    }
    #[doc(hidden)]
    fn valtype() -> ValType;
    #[doc(hidden)]
    fn compatible_with_store(&self, store: &Store) -> bool;
    #[doc(hidden)]
    fn into_abi(self, store: &Store) -> Self::Abi;
    #[doc(hidden)]
    unsafe fn from_abi(abi: Self::Abi, store: &Store) -> Self;
}
```

对于上面的 trait ，因为是内部使用，随时可能发生改变。所以标记为 Unsafe ，并加上注释提示该 trait 不该又库用户自己实现，而是由维护者在内部为指定类型实现，这些类型应该是稳定的。如果用户想自己实现，那么要明白它是 Unsafe 的。

所以，不一定是出于内存安全才指定 Unsafe ，也可以作为一种和库用户的约定。


## 在 FFi 时方便调用 Rust 闭包

```rust
use std::os::raw::c_void;

pub type Callback = unsafe extern "C" fn(user_data: *mut c_void, arg: i32) -> i32;

pub unsafe extern "C" fn execute_a_closure(arg: i32, cb: Callback, user_data: *mut c_void) -> i32 {
    cb(user_data, arg)
}

/// 获取一个可以用作[`Callback`]的函数指针，该函数指针将指向闭包的指针作为其`user_data`。
pub fn raw_callback<F>(_closure: &F) -> Callback
where
    F: FnMut(i32) -> i32,
{
    unsafe extern "C" fn wrapper<P>(user_data: *mut c_void, arg: i32) -> i32
    where
        P: FnMut(i32) -> i32,
    {
        let cb = &mut *(user_data as *mut P);

        cb(arg)
    }

    wrapper::<F>
}

fn main() {
    let mut calls = 0;
    let mut closure = |arg: i32| {
        calls += 1;
        arg
    };

    unsafe {
        let func = raw_callback(&closure);

        let got = execute_a_closure(42, func, &mut closure as *mut _ as *mut c_void);

        assert_eq!(got, 42);
        assert_eq!(calls, 1);
    }
}
```

经过 Miri 检测没有 UB。