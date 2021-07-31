# Trait Upcasting 系列 ｜ Part I

作者： 张汉东 / 审校：crlf0710

---

## 引子

记录 @crlf0710 的 Trait Upcasting系列 系列 PR 过程。因为后续我也参与其中一个 PR，所以先要理清楚 @crlf0710 的 PR 思路，以便后续我的参与。也借此机会分享出来，希望更多的人能参与到 Rust 语言贡献中。

PR 系列：

1. [Refactor vtable codegen #86291](https://github.com/rust-lang/rust/pull/86291)
2. [Change vtable memory representation to use tcx allocated allocations.#86475 ](https://github.com/rust-lang/rust/pull/86475)
3. [Refactor vtable format for upcoming trait_upcasting feature. #86461](https://github.com/rust-lang/rust/pull/86461)
4. [Trait upcasting (part1) #86264](https://github.com/rust-lang/rust/pull/86264)
5. [Trait upcasting (part2) ]()

本文为 第一个 PR 的描述。

---

## 前情提要

故事要从 [Trait upcasting #60900](https://github.com/rust-lang/rust/pull/60900) 这个 PR 讲起 。

Trait upcasting ，是 trait 向上转型的意思。这个 PR 提出，当 `Foo: Bar` ，即 Foo trait 继承自 Bar trait 时，允许从 `dyn Foo`转到 `dyn Bar`。

目前 Rust 版本中，不支持此功能。因为目前trait 继承情况下， trait 对象的方法都是存储在同一个虚表中，无法区分哪个函数是属于哪个trait 对象。

社区内有一个通用的解决办法：

```rust
trait Base {
    fn base(&self) {
        println!("base...");
    }
}

trait AsBase {
    fn as_base(&self) -> &dyn Base; //返回 Base trait对象
}

// blanket implementation
// 为所有实现 Base 的 T 来实现 AsBase
impl<T: Base> AsBase for T {
  	// 返回 Base trait对象
    fn as_base(&self) -> &dyn Base {
        self
    }
}

trait Foo: AsBase {
    fn foo(&self) {
        println!("foo..");
    }
}

#[derive(Debug)]
struct MyStruct;

impl Foo for MyStruct {}
impl Base for MyStruct {}

fn main() {
    let s = MyStruct;
    let foo: &dyn Foo = &s;
    foo.foo();
    let base: &dyn Base = foo.as_base(); // 通过 as_base 来返回 Base trait对象达到 upcasting 的效果
    base.base();
}

```



在 PR  #60900  中，作者给出了一些实现，但是因为太大了，需要对这份工作进行重构，然后这个 PR 就被关闭了。关于这个 PR 的相关讨论被记录于 [rust-lang.zulipchat.](https://rust-lang.zulipchat.com/#narrow/stream/144729-wg-traits/topic/object.20upcasting)

这份重构的工作，就由 **[crlf0710](https://github.com/crlf0710)** 承接起来了，这就是这个系列 PR 的由来。相关提案：[Trait Upcasting #98](https://github.com/rust-lang/lang-team/issues/98) ， 跟踪 issues :[Tracking issue for trait upcasting #65991 ](https://github.com/rust-lang/rust/issues/65991)



##  第一步工作： 重构 vtable 代码生成

状态：这部分工作已经被合并。

相关PR: [Refactor vtable codegen #86291](https://github.com/rust-lang/rust/pull/86291) 。

### 修改文件概述

本次修改涉及 十个文件。

1. `compiler/rustc_codegen_cranelift/src/vtable.rs `
2. `compiler/rustc_codegen_ssa/src/glue.rs`
3. `compiler/rustc_codegen_ssa/src/meth.rs `
4. `compiler/rustc_codegen_ssa/src/mir/block.rs`
5. `compiler/rustc_middle/src/query/mod.rs `
6. `compiler/rustc_middle/src/ty/mod.rs`
7. `compiler/rustc_mir/src/interpret/traits.rs `
8. `compiler/rustc_mir/src/monomorphize/collector.rs `
9. `compiler/rustc_trait_selection/src/traits/mod.rs `
10. `compiler/rustc_trait_selection/src/traits/select/confirmation.rs`



这十个文件涉及五个 crate：

1. `rustc_codegen_cranelift`，是 基于 cranelift 的编译器后端，专门用于 debug 模式。
2. `rustc_codegen_ssa`，截至2021年1月，RustC_Codegen_SSA 为所有后端提供了一个抽象的接口，以允许其他Codegen后端（例如Cranelift）。
3. `rustc_middle`，属于 rust 编译器的 main crate ，包含rustc“家族”中的其他crate使用的通用类型定义，包括 HIR/MIR/Types。
4. `rustc_mir`，用于操作 MIR 的库。
5. `rustc_trait_selection`，该库定义了 trait resolution 相关方法。详细内容：[Trait resolution (old-style)](https://rustc-dev-guide.rust-lang.org/traits/resolution.html) 。

### rustc_middle 库中的修改

在 `compiler/rustc_middle/src/ty/mod.rs` 中新增了枚举类型：VtblEntry。

```rust
#[derive(Clone, Copy, Debug, PartialEq, HashStable)]
pub enum VtblEntry<'tcx> {
    MetadataDropInPlace,
    MetadataSize,
    MetadataAlign,
    Vacant,
    Method(DefId, SubstsRef<'tcx>),
}

pub const COMMON_VTABLE_ENTRIES: &[VtblEntry<'_>] =
    &[VtblEntry::MetadataDropInPlace, VtblEntry::MetadataSize, VtblEntry::MetadataAlign];

pub const COMMON_VTABLE_ENTRIES_DROPINPLACE: usize = 0;
pub const COMMON_VTABLE_ENTRIES_SIZE: usize = 1;
pub const COMMON_VTABLE_ENTRIES_ALIGN: usize = 2;
```

这是为了识别 vtable 中的不同 entry，这样才有可能识别 存储在vtable中的不同 trait 对象。



接下来，在 `compiler/rustc_middle/src/query/mod.rs ` 中把  `query vtable_methods` 修改为 `query vtable_entries`。

```rust
// 使用的是一个宏
rustc_queries! {
   // ...
   query vtable_entries(key: ty::PolyTraitRef<'tcx>)
                        -> &'tcx [ty::VtblEntry<'tcx>] {
        desc { |tcx| "finding all vtable entries for trait {}", tcx.def_path_str(key.def_id()) }
    }
  
    // ...
}
```

在 rust_middle 中定义了 rustc 的 [query 系统](https://rustc-dev-guide.rust-lang.org/overview.html?highlight=query#queries) 。Rust 使用查询系统，是为了支持增量编译。参考 [编译器概览](https://rustcrustc.github.io/rustc-dev-guide-zh/overview.html#编译器概览) 。

举个例子。假如有一条查询负责询问某个东西的类型， 而另一条查询负责询问某个函数的优化后的 MIR。这些查询可以相互调用并且由查询系统所跟踪。 查询的结果被缓存于硬盘上，这样我们就可以分辨相较于上次编译，哪些查询的结果改变了，并且仅重做这些查询。 这就是增量编译是如何工作的。

> 类型上下文（TyCtxt），它是一个相当巨大的结构体， 是所有东西的中心。所有查询都被定义为在[`TyCtxt`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/struct.TyCtxt.html)类型上 的方法，并且内存中的查询缓存也同样被存储在此。在代码中，通常会有一个名为`tcx`变量，它是 类型上下文上的一个句柄。有同样会见到名为`'tcx`的生命周期，这意味着有东西被和`TyCtxt`的 生命周期绑定在了一起（通常它会被存储或者被驻留化）。

 **[`ty::Ty`](https://rustcrustc.github.io/rustc-dev-guide-zh/overview.html#tyty) 介绍**

类型在 Rust 中相当重要，并且形成了许多编译器分析的核心。用于表示类型（在用户程序中）的 主要类型（在编译器中）是 [`rustc_middle::ty::Ty`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/type.Ty.html)。它是如此的重要以至于我们为其 设置了一整章[`ty::Ty`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/type.Ty.html)，但是对于现在而言，我们只想提到它存在并且是`rustc`用来表示类型的方法！

同样注意到`rustc_middle::ty`模块定义了我们之前提到的`TyCtxt`结构体。



### rustc_codegen_ssa 中的修改

因为 rustc_codegen_ssa 是 后端 codegen 的接口，所以先看这里。

rustc_codegen_ssa 主要入口点： [`rustc_codegen_ssa::base::codegen_crate`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_codegen_ssa/base/fn.codegen_crate.html)

- 它单态化并且产出 LLVM IR给一个代码生成单元。 它之后启动一个后台线程来运行一个之后必须被结合的LLVM。
- 单态化通过[`FunctionCx::monomorphize`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_codegen_ssa/mir/struct.FunctionCx.html#method.monomorphize) 懒启动以及[`rustc_codegen_ssa::base::codegen_instance`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_codegen_ssa/base/fn.codegen_instance.html)

在 rust_codgen_ssa 出现之前，生成代码都是由 rust_codgen_llvm 处理。

LLVM codegen的两个最重要的结构是`CodegenCx`和`Builder`。它们由多个生命期参数和Value的类型组成。

```rust

struct CodegenCx<'ll, 'tcx> {
  /* ... */
}

struct Builder<'a, 'll, 'tcx> {
  cx: &'a CodegenCx<'ll, 'tcx>,
  /* ... */
}
```

`CodegenCx`是用来编译一个可以包含多个函数的 codegen-unit 的，而`Builder` 是为了编译一个基本块而创建的。`CodegenCx`和`Builder`将是实现所有定义后端接口的traits的结构。

这些 trait 被定义在`rustc_codegen_ssa/traits`文件夹中，所有与后端无关的代码都以它们为参数。

在  rustc_codegen_ssa 有个关键的 trait ：` BuilderMethods `，它表示后端实现的构建方法。那么实际上， rustc_codegen_cranelift 目前并没有依赖  rustc_codegen_ssa 的这个 BuilderMethods trait， 而 rustc_codegen_llvm 依赖了。看来目前 rustc_codegen_ssa 并未重构完成。

重构 vtable 的相关工作，主要涉及三个文件： 

1. `rustc_codegen_ssa/src/meth.rs `
2. `rustc_codegen_ssa/src/glue.rs`
3. `rustc_codegen_ssa/src/mir/block.rs`

 **在 `meth.rs` 中：**

```rust
use rustc_middle::ty::{self, Instance, Ty, VtblEntry, COMMON_VTABLE_ENTRIES}; // 引入 rustc_middle 新加的枚举 VtblEntry 相关

impl<'a, 'tcx> VirtualIndex {
    pub fn from_index(index: usize) -> Self {
        VirtualIndex(index as u64) // 修改虚表index ，之前是偏移 3 ，因为之前是没有 vtable Entry 的，所以 DESTRUCTOR（index 0）,SIZE（index 1）,ALIGN（index 2） 都展开放了，现在则不需要。
    }
  
    // ...
  
    // 修改
    pub fn get_vtable<'tcx, Cx: CodegenMethods<'tcx>>(
    cx: &Cx,
    ty: Ty<'tcx>,
    trait_ref: Option<ty::PolyExistentialTraitRef<'tcx>>,
) -> Cx::Value {
  			// ...
        // 新增
        // 当有  T: Trait 或 SubTrait: ParentTrait 这种形式出现时，就会有 trait_ref 
        // 所以，相当于是 如果是有 trait 继承的情况下，就利用 query vtable_entries 来查询该trait
        // 并返回 vtable_entries ，否则返回 COMMON_VTABLE_ENTRIES，代表是单一的trait 对象
  	    let vtable_entries = if let Some(trait_ref) = trait_ref {
            tcx.vtable_entries(trait_ref.with_self_ty(tcx, ty))
        } else {
            COMMON_VTABLE_ENTRIES
        };
  
  			let layout = cx.layout_of(ty); // 新增
    // /////////////////////////////////////////////////////////////////////////////////////////////
    // If you touch this code, be sure to also make the corresponding changes to
    // `get_vtable` in `rust_mir/interpret/traits.rs`.
      // /////////////////////////////////////////////////////////////////////////////////////////////
    // 新增
    // 迭代处理每个 vtable entry 的元信息：drop/大小/对齐/方法等
    let components: Vec<_> = vtable_entries
        .iter()
        .map(|entry| match entry {
            VtblEntry::MetadataDropInPlace => {
                cx.get_fn_addr(Instance::resolve_drop_in_place(cx.tcx(), ty))
            }
            VtblEntry::MetadataSize => cx.const_usize(layout.size.bytes()),
            VtblEntry::MetadataAlign => cx.const_usize(layout.align.abi.bytes()),
            VtblEntry::Vacant => nullptr,
            VtblEntry::Method(def_id, substs) => cx.get_fn_addr(
                ty::Instance::resolve_for_vtable(
                    cx.tcx(),
                    ty::ParamEnv::reveal_all(),
                    *def_id,
                    substs,
                )
                .unwrap()
                .polymorphize(cx.tcx()),
            ),
        })
        .collect();
  
        // ...
  
     }
}
```

文档：[rustc_middle::ty::PolyExistentialTraitRef](https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/ty/type.PolyExistentialTraitRef.html) ，该类型表示 对一个已经擦除了 Self 的 trait 的存在性引用，所以使用 `with_self_ty` 来提供一个self占位。

**在 `src/glue.rs` 中：**

```rust
// BuilderMethods 是通用后端接口，但目前只有llvm用这个
pub fn size_and_align_of_dst<'a, 'tcx, Bx: BuilderMethods<'a, 'tcx>>(
    bx: &mut Bx,
    t: Ty<'tcx>,
    info: Option<Bx::Value>,
) -> (Bx::Value, Bx::Value) {
  
 		// ...
  
  	match t.kind() {
        ty::Dynamic(..) => {
            // load size/align from vtable
            // 新增
            let vtable = info.unwrap();
            (
                meth::VirtualIndex::from_index(ty::COMMON_VTABLE_ENTRIES_SIZE)
                    .get_usize(bx, vtable),
                meth::VirtualIndex::from_index(ty::COMMON_VTABLE_ENTRIES_ALIGN)
                    .get_usize(bx, vtable),
            )
        }
    		// ...  
    }
  
    // ...
}
```

问题： 既然目前 BuilderMethods 只有 llvm使用而 cranelift没有使用， 为什么 [rustc_codegen_cranelift/src/unsize.rs#L131](https://github.com/rust-lang/rust/blob/master/compiler/rustc_codegen_cranelift/src/unsize.rs#L131) 中对应的 `size_and_align_of_dst` 函数不做对应修改？

答：因为 rustc_codegen_cranelift 中 vtable 要做相应修改，具体在后面描述。

 `src/glue.rs` 就是一个胶水模块，在生成底层指令的相关模块中，会调用该方法。

**在 `src/mir/block.rs` 中：**

```rust
struct TerminatorCodegenHelper<'tcx> {
    bb: mir::BasicBlock,
    terminator: &'tcx mir::Terminator<'tcx>,
    funclet_bb: Option<mir::BasicBlock>,
}

impl<'a, 'tcx> TerminatorCodegenHelper<'tcx> {
    // ...
  
  	fn codegen_drop_terminator(
        &mut self,
        helper: TerminatorCodegenHelper<'tcx>,
        mut bx: Bx,
        location: mir::Place<'tcx>,
        target: mir::BasicBlock,
        unwind: Option<mir::BasicBlock>,
    ) {
        // ...
         let (drop_fn, fn_abi) = match ty.kind() {
            // FIXME(eddyb) perhaps move some of this logic into
            // `Instance::resolve_drop_in_place`?
            ty::Dynamic(..) => {
                 // ...
                 // 新增
                 (
                   meth::VirtualIndex::from_index(ty::COMMON_VTABLE_ENTRIES_DROPINPLACE)
                        .get_fn(&mut bx, vtable, &fn_abi),
                    fn_abi,
                )
            }
            // ... 
          }
        // ...
    }
  
    // ...
  
}
```

`src/mir/block.rs` 顾名思义，这个是和 MIR 生成 basicblock 有关。代码中，要生成 drop 相关的终止符，所以需要得到虚表中 `COMMON_VTABLE_ENTRIES_DROPINPLACE`相关 index信息。



### rustc_codegen_cranelift 库中的修改

在 `compiler/rustc_codegen_cranelift/src/vtable.rs` 中，定义了一些自由函数，用于定义 trait 对象中 vtable相关。

因为在 rustc_codegen_ssa 做了一些相关修改，而目前 rustc_codegen_cranelift 并没有使用 rustc_codegen_ssa 的统一接口，所以需要修改 rustc_codegen_cranelift vtable相关代码。

```rust

use ty::VtblEntry; // 新增

pub(crate) fn drop_fn_of_obj(fx: &mut FunctionCx<'_, '_, '_>, vtable: Value) -> Value {
    let usize_size = fx.layout_of(fx.tcx.types.usize).size.bytes() as usize;
    fx.bcx.ins().load(
        pointer_ty(fx.tcx),
        vtable_memflags(),
        vtable,
        (ty::COMMON_VTABLE_ENTRIES_DROPINPLACE * usize_size) as i32, // 新增
    )
}

pub(crate) fn size_of_obj(fx: &mut FunctionCx<'_, '_, '_>, vtable: Value) -> Value {
    let usize_size = fx.layout_of(fx.tcx.types.usize).size.bytes() as usize;
    fx.bcx.ins().load(
        pointer_ty(fx.tcx),
        vtable_memflags(),
        vtable,
        (ty::COMMON_VTABLE_ENTRIES_SIZE * usize_size) as i32, // 新增
    )
}

pub(crate) fn min_align_of_obj(fx: &mut FunctionCx<'_, '_, '_>, vtable: Value) -> Value {
    let usize_size = fx.layout_of(fx.tcx.types.usize).size.bytes() as usize;
    fx.bcx.ins().load(
        pointer_ty(fx.tcx),
        vtable_memflags(),
        vtable,
        (ty::COMMON_VTABLE_ENTRIES_SIZE * usize_size) as i32, // 新增
    )
}

pub(crate) fn get_ptr_and_method_ref<'tcx>(
    fx: &mut FunctionCx<'_, '_, 'tcx>,
    arg: CValue<'tcx>,
    idx: usize,
) -> (Value, Value) {
    let (ptr, vtable) = if let Abi::ScalarPair(_, _) = arg.layout().abi {
        arg.load_scalar_pair(fx)
    } else {
        let (ptr, vtable) = arg.try_to_ptr().unwrap();
        (ptr.get_addr(fx), vtable.unwrap())
    };

    let usize_size = fx.layout_of(fx.tcx.types.usize).size.bytes();
    let func_ref = fx.bcx.ins().load(
        pointer_ty(fx.tcx),
        vtable_memflags(),
        vtable,
        (idx * usize_size as usize) as i32, // 修改，因为 idx 变了，之前是 idx+3
    );
    (ptr, func_ref)
}

fn build_vtable<'tcx>(
    fx: &mut FunctionCx<'_, '_, 'tcx>,
    layout: TyAndLayout<'tcx>,
    trait_ref: Option<ty::PolyExistentialTraitRef<'tcx>>,
) -> DataId {
    let tcx = fx.tcx;
    let usize_size = fx.layout_of(fx.tcx.types.usize).size.bytes() as usize;

    let drop_in_place_fn = import_function(
        tcx,
        fx.module,
        Instance::resolve_drop_in_place(tcx, layout.ty).polymorphize(fx.tcx),
    );

    // 新增
    let vtable_entries = if let Some(trait_ref) = trait_ref {
        tcx.vtable_entries(trait_ref.with_self_ty(tcx, layout.ty))
    } else {
        ty::COMMON_VTABLE_ENTRIES
    };
  
     let mut data_ctx = DataContext::new();
     let mut data = ::std::iter::repeat(0u8)
         .take(vtable_entries.len() * usize_size)
         .collect::<Vec<u8>>()
         .into_boxed_slice();
      // 新增
      // 迭代处理 vtable entry
      for (idx, entry) in vtable_entries.iter().enumerate() {
          match entry {
              VtblEntry::MetadataSize => {
                  write_usize(fx.tcx, &mut data, idx, layout.size.bytes());
              }
              VtblEntry::MetadataAlign => {
                  write_usize(fx.tcx, &mut data, idx, layout.align.abi.bytes());
              }
              VtblEntry::MetadataDropInPlace | VtblEntry::Vacant | VtblEntry::Method(_, _) => {}
          }
      }
      data_ctx.define(data);

  
      // 迭代处理 vtable entry
      for (idx, entry) in vtable_entries.iter().enumerate() {
          match entry {
              VtblEntry::MetadataDropInPlace => {
                  let func_ref = fx.module.declare_func_in_data(drop_in_place_fn, &mut data_ctx);
                  data_ctx.write_function_addr((idx * usize_size) as u32, func_ref);
              }
              VtblEntry::Method(def_id, substs) => {
                  let func_id = import_function(
                      tcx,
                      fx.module,
                      Instance::resolve_for_vtable(tcx, ParamEnv::reveal_all(), *def_id, substs)
                          .unwrap()
                          .polymorphize(fx.tcx),
                  );
                  let func_ref = fx.module.declare_func_in_data(func_id, &mut data_ctx);
                  data_ctx.write_function_addr((idx * usize_size) as u32, func_ref);
              }
              VtblEntry::MetadataSize | VtblEntry::MetadataAlign | VtblEntry::Vacant => {}
          }
      }

    // ... 
}
```



对 vtable 的修改，类似于 rustc_codegen_ssa  相关代码修改，只不过 rustc_codegen_cranelift 没有完全使用 rustc_codegen_ssa 的接口，所以需要另行单独处理。



### rustc_trait_selection 中的修改 

该库定义了 trait resolution 相关方法。Rust 编译器类型检查，mir层都依赖于该库。

Trait Resolution 主要用于判断该如何选择合理的 trait。 比如：

```rust
trait Convert<Target> {
    fn convert(&self) -> Target;
}
```

这个trait只有一个方法。它是最简单的。它从（隐含的）Self类型转换到Target类型。如果我们想允许isize和usize之间的转换，我们可以这样实现Convert。

```rust

impl Convert<usize> for isize { ... } // isize -> usize
impl Convert<isize> for usize { ... } // usize -> isize
```

现在想象一下，有一些像下面这样的代码。

```rust
let x: isize = .....;
let y = x.convert();
```

对convert的调用将为isize生成一个`trait reference Convert<$Y>`，其中`$Y`是代表y类型的类型变量。在我们可以看到的两个impls中，唯一匹配的是`Convert<usize> for isize`。因此，我们可以选择这个函数，这将导致`$Y`的类型被统一为`usize`。(注意，在组装候选程序时，我们在一个事务中进行初始统一，这样它们就不会相互影响。)

还有其他情况，可以参考 [Trait resolution (old-style)](https://rustc-dev-guide.rust-lang.org/traits/resolution.html) 。

既然 vtable 已经修改，那也必须得修改该库中相关代码。

一共修改两个文件：`rustc_trait_selection/src/traits/mod.rs` 和 `rustc_trait_selection/src/traits/select/confirmation.rs`。

**在 `src/traits/mod.rs`中：**

```rust
use rustc_middle::ty::{
    self, GenericParamDefKind, ParamEnv, ToPredicate, Ty, TyCtxt, VtblEntry, WithConstness,
    COMMON_VTABLE_ENTRIES,
}; // 引入 新增的 VtblEntry类型

pub use self::util::{
    supertrait_def_ids, supertraits, transitive_bounds, transitive_bounds_that_define_assoc_type,
    SupertraitDefIds, Supertraits,
};

/// Given a trait `trait_ref`, iterates the vtable entries
/// that come from `trait_ref`, including its supertraits.
// 修改 原方法
fn vtable_entries<'tcx>(
    tcx: TyCtxt<'tcx>,
    trait_ref: ty::PolyTraitRef<'tcx>,
) -> &'tcx [VtblEntry<'tcx>] {
    debug!("vtable_entries({:?})", trait_ref);

    let entries = COMMON_VTABLE_ENTRIES.iter().cloned().chain(
        supertraits(tcx, trait_ref).flat_map(move |trait_ref| {
            let trait_methods = tcx
                .associated_items(trait_ref.def_id())
                .in_definition_order()
                .filter(|item| item.kind == ty::AssocKind::Fn);

            // Now list each method's DefId and InternalSubsts (for within its trait).
            // If the method can never be called from this object, produce `Vacant`.
            trait_methods.map(move |trait_method| {
                debug!("vtable_entries: trait_method={:?}", trait_method);
                let def_id = trait_method.def_id;

                // Some methods cannot be called on an object; skip those.
                if !is_vtable_safe_method(tcx, trait_ref.def_id(), &trait_method) {
                    debug!("vtable_entries: not vtable safe");
                    return VtblEntry::Vacant;
                }

                // The method may have some early-bound lifetimes; add regions for those.
                let substs = trait_ref.map_bound(|trait_ref| {
                    InternalSubsts::for_item(tcx, def_id, |param, _| match param.kind {
                        GenericParamDefKind::Lifetime => tcx.lifetimes.re_erased.into(),
                        GenericParamDefKind::Type { .. } | GenericParamDefKind::Const { .. } => {
                            trait_ref.substs[param.index as usize]
                        }
                    })
                });

                // The trait type may have higher-ranked lifetimes in it;
                // erase them if they appear, so that we get the type
                // at some particular call site.
                let substs =
                    tcx.normalize_erasing_late_bound_regions(ty::ParamEnv::reveal_all(), substs);

                // It's possible that the method relies on where-clauses that
                // do not hold for this particular set of type parameters.
                // Note that this method could then never be called, so we
                // do not want to try and codegen it, in that case (see #23435).
                let predicates = tcx.predicates_of(def_id).instantiate_own(tcx, substs);
                if impossible_predicates(tcx, predicates.predicates) {
                    debug!("vtable_entries: predicates do not hold");
                    return VtblEntry::Vacant;
                }

                VtblEntry::Method(def_id, substs)
            })
        }),
    );

    tcx.arena.alloc_from_iter(entries)
}

/// Find slot base for trait methods within vtable entries of another trait
// 新增 ： 查找其他 trait 的VTable条目中的 trait 方法的位置
fn vtable_trait_first_method_offset<'tcx>(
    tcx: TyCtxt<'tcx>,
    key: (
        ty::PolyTraitRef<'tcx>, // trait_to_be_found
        ty::PolyTraitRef<'tcx>, // trait_owning_vtable
    ),
) -> usize {
    let (trait_to_be_found, trait_owning_vtable) = key;

    let mut supertraits = util::supertraits(tcx, trait_owning_vtable);

    // For each of the non-matching predicates that
    // we pass over, we sum up the set of number of vtable
    // entries, so that we can compute the offset for the selected
    // trait.
    let vtable_base = ty::COMMON_VTABLE_ENTRIES.len()
        + supertraits
            .by_ref()
            .take_while(|t| *t != trait_to_be_found)
            .map(|t| util::count_own_vtable_entries(tcx, t))
            .sum::<usize>();

    vtable_base
}

// 修改
pub fn provide(providers: &mut ty::query::Providers) {
    object_safety::provide(providers);
    structural_match::provide(providers);
    *providers = ty::query::Providers {
        // ...
      
        vtable_entries,
  
        // ...
    };
}
```



**在 `src/traits/select/confirmation.rs ` 中：**

该模块用于确认选中的 trait 。

```rust
 fn confirm_object_candidate(
        &mut self,
        obligation: &TraitObligation<'tcx>,
        index: usize,
    ) -> Result<ImplSourceObjectData<'tcx, PredicateObligation<'tcx>>, SelectionError<'tcx>> {
    // ...
    let unnormalized_upcast_trait_ref =
            supertraits.nth(index).expect("supertraits iterator no longer has as many elements"); // 修改
      
    // ...
    let vtable_base = super::super::vtable_trait_first_method_offset(
            tcx,
            (unnormalized_upcast_trait_ref, ty::Binder::dummy(object_trait_ref)),
        );
    // ...
}
```







###  rustc_mir 中的修改

在 rust_mir 中涉及两个文件修改: `rustc_mir/src/interpret/traits.rs ` 和 `rustc_mir/src/monomorphize/collector.rs ` 。

**在 `src/interpret/traits.rs` 中：**

interpret 是和 mir 转译为 llvm ir 相关。

```rust
use rustc_middle::ty::{
    self, Instance, Ty, VtblEntry, COMMON_VTABLE_ENTRIES, COMMON_VTABLE_ENTRIES_ALIGN,
    COMMON_VTABLE_ENTRIES_DROPINPLACE, COMMON_VTABLE_ENTRIES_SIZE,
}; // 修改，引入 VtblEntry 相关新类型


// 修改原方法
// InterpCx 是 interpret 上下文
impl<'mir, 'tcx: 'mir, M: Machine<'mir, 'tcx>> InterpCx<'mir, 'tcx, M> {
    /// Creates a dynamic vtable for the given type and vtable origin. This is used only for
    /// objects.
    ///
    /// The `trait_ref` encodes the erased self type. Hence, if we are
    /// making an object `Foo<Trait>` from a value of type `Foo<T>`, then
    /// `trait_ref` would map `T: Trait`.
    pub fn get_vtable(
        &mut self,
        ty: Ty<'tcx>,
        poly_trait_ref: Option<ty::PolyExistentialTraitRef<'tcx>>,
    ) -> InterpResult<'tcx, Pointer<M::PointerTag>> {
        // ...
      
        // 获取 vtable entries
      	let vtable_entries = if let Some(poly_trait_ref) = poly_trait_ref {
            let trait_ref = poly_trait_ref.with_self_ty(*self.tcx, ty);
            let trait_ref = self.tcx.erase_regions(trait_ref);

            self.tcx.vtable_entries(trait_ref)
        } else {
            COMMON_VTABLE_ENTRIES
        };

        // ...
        // 新增
        ////////////////////////////////////////////////////////////////////////
        // If you touch this code, be sure to also make the corresponding changes to
        // `get_vtable` in `rust_codegen_llvm/meth.rs`.
        // /////////////////////////////////////////////////////////////////////
        let vtable_size = ptr_size * u64::try_from(vtable_entries.len()).unwrap();
      
        // ...
        // 新增
        // No need to do any alignment checks on the memory accesses below, because we know the
        // allocation is correctly aligned as we created it above. Also we're only offsetting by
        // multiples of `ptr_align`, which means that it will stay aligned to `ptr_align`.
        // 迭代处理 vtable entries 中每个虚表的布局
        let scalars = vtable_entries
            .iter()
            .map(|entry| -> InterpResult<'tcx, _> {
                match entry {
                    VtblEntry::MetadataDropInPlace => Ok(Some(drop.into())),
                    VtblEntry::MetadataSize => Ok(Some(Scalar::from_uint(size, ptr_size).into())),
                    VtblEntry::MetadataAlign => Ok(Some(Scalar::from_uint(align, ptr_size).into())),
                    VtblEntry::Vacant => Ok(None),
                    VtblEntry::Method(def_id, substs) => {
                        // Prepare the fn ptr we write into the vtable.
                        let instance =
                            ty::Instance::resolve_for_vtable(tcx, self.param_env, *def_id, substs)
                                .ok_or_else(|| err_inval!(TooGeneric))?;
                        let fn_ptr = self.memory.create_fn_alloc(FnVal::Instance(instance));
                        Ok(Some(fn_ptr.into()))
                    }
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut vtable_alloc =
            self.memory.get_mut(vtable.into(), vtable_size, ptr_align)?.expect("not a ZST");
        for (idx, scalar) in scalars.into_iter().enumerate() {
            if let Some(scalar) = scalar {
                let idx: u64 = u64::try_from(idx).unwrap();
                vtable_alloc.write_ptr_sized(ptr_size * idx, scalar)?;
            }
        }
        // ...  
      
        // 修改原方法
        /// Resolves the function at the specified slot in the provided
    /// vtable. Currently an index of '3' (`COMMON_VTABLE_ENTRIES.len()`)
    /// corresponds to the first method declared in the trait of the provided vtable.
    pub fn get_vtable_slot(
        &self,
        vtable: Scalar<M::PointerTag>,
        idx: u64,
    ) -> InterpResult<'tcx, FnVal<'tcx, M::ExtraFnVal>> {
        let ptr_size = self.pointer_size();
        let vtable_slot = vtable.ptr_offset(ptr_size * idx, self)?; // 新增
        let vtable_slot = self
            .memory
            .get(vtable_slot, ptr_size, self.tcx.data_layout.pointer_align.abi)?
            .expect("cannot be a ZST");
        let fn_ptr = vtable_slot.read_ptr_sized(Size::ZERO)?.check_init()?;
        self.memory.get_fn(fn_ptr)
    }

    /// Returns the drop fn instance as well as the actual dynamic type.
    pub fn read_drop_type_from_vtable(
        &self,
        vtable: Scalar<M::PointerTag>,
    ) -> InterpResult<'tcx, (ty::Instance<'tcx>, Ty<'tcx>)> {
        let pointer_size = self.pointer_size();
        // We don't care about the pointee type; we just want a pointer.
        let vtable = self
            .memory
            .get(
                vtable,
                pointer_size * u64::try_from(COMMON_VTABLE_ENTRIES.len()).unwrap(),
                self.tcx.data_layout.pointer_align.abi,
            )?
            .expect("cannot be a ZST");
        let drop_fn = vtable
            .read_ptr_sized(
                pointer_size * u64::try_from(COMMON_VTABLE_ENTRIES_DROPINPLACE).unwrap(),
            )?
            .check_init()?;
          // ....
        }
      
        // ...
        // 修改原方法
        pub fn read_size_and_align_from_vtable(
        &self,
        vtable: Scalar<M::PointerTag>,
        ) -> InterpResult<'tcx, (Size, Align)> {
            let pointer_size = self.pointer_size();
            // We check for `size = 3 * ptr_size`, which covers the drop fn (unused here),
            // the size, and the align (which we read below).
            let vtable = self
                .memory
                .get(
                    vtable,
                    pointer_size * u64::try_from(COMMON_VTABLE_ENTRIES.len()).unwrap(),
                    self.tcx.data_layout.pointer_align.abi,
                )?
                .expect("cannot be a ZST");
            let size = vtable
                .read_ptr_sized(pointer_size * u64::try_from(COMMON_VTABLE_ENTRIES_SIZE).unwrap())?
                .check_init()?;
            let size = u64::try_from(self.force_bits(size, pointer_size)?).unwrap();
            let align = vtable
                .read_ptr_sized(pointer_size * u64::try_from(COMMON_VTABLE_ENTRIES_ALIGN).unwrap())?
                .check_init()?;
            let align = u64::try_from(self.force_bits(align, pointer_size)?).unwrap();
            let align = Align::from_bytes(align).map_err(|e| err_ub!(InvalidVtableAlignment(e)))?;

            if size >= self.tcx.data_layout.obj_size_bound() {
                throw_ub!(InvalidVtableSize);
            }
            Ok((Size::from_bytes(size), align))
        }
        // ...
    }
}
```



**在 `src/monomorphize/collector.rs `中：**

monomorphize 意思是 单态化，意味着 这个模块用于 泛型单态化。

```rust
use rustc_middle::ty::{self, GenericParamDefKind, Instance, Ty, TyCtxt, TypeFoldable, VtblEntry}; // 引入新的 VtablEntry 类型

/// Creates a `MonoItem` for each method that is referenced by the vtable for
/// the given trait/impl pair.
fn create_mono_items_for_vtable_methods<'tcx>(
    tcx: TyCtxt<'tcx>,
    trait_ty: Ty<'tcx>,
    impl_ty: Ty<'tcx>,
    source: Span,
    output: &mut Vec<Spanned<MonoItem<'tcx>>>,
) {
   // ...
   if let ty::Dynamic(ref trait_ty, ..) = trait_ty.kind() {
       if let Some(principal) = trait_ty.principal() {
           // ...
           // Walk all methods of the trait, including those of its supertraits
           // 走查所有的 trait 方法，包括 supertrait 的
           let entries = tcx.vtable_entries(poly_trait_ref);
           let methods = entries
                .iter()
                .filter_map(|entry| match entry {
                    VtblEntry::MetadataDropInPlace
                    | VtblEntry::MetadataSize
                    | VtblEntry::MetadataAlign
                    | VtblEntry::Vacant => None,
                    VtblEntry::Method(def_id, substs) => ty::Instance::resolve_for_vtable(
                        tcx,
                        ty::ParamEnv::reveal_all(),
                        *def_id,
                        substs,
                    )
                    .filter(|instance| should_codegen_locally(tcx, instance)),
                })
                .map(|item| create_fn_mono_item(tcx, item, source));
            output.extend(methods);
       }
   }
   // ...
}
```



## 小结

第一步工作，主要是为了改进生成的 vtable结构，能够识别 多个trait 对象。

从 `rustc_middle -> rustc_codgen_ssa -> rustc_codegen_cranelift -> rustc_trait_selection -> rustc_mir` 这个过程，是从上到下，从抽象类型 到 能转译为 llvm IR 的 MIR。

如果 `rustc_codgen_cranelift` 能够完全使用 `rustc_codgen_ssa` ，那么代码修改起来应该更方便了。

后续：看到 [rustc_codegen_gcc](https://github.com/antoyo/rustc_codegen_gcc) 就是基于 `rustc_codgen_ssa`来实现的。



