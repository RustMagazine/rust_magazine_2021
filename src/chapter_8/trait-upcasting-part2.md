# Trait Upcasting 系列 ｜ Part II

作者：张汉东 / 审校：CrLF0710

记录 Trait Upcasting系列 系列 PR 过程。

PR 系列：

1. [Refactor vtable codegen #86291](https://github.com/rust-lang/rust/pull/86291)
2. [Change vtable memory representation to use tcx allocated allocations.#86475 ](https://github.com/rust-lang/rust/pull/86475)
3. [Refactor vtable format for upcoming trait_upcasting feature. #86461](https://github.com/rust-lang/rust/pull/86461)
4. [Trait upcasting (part1) #86264](https://github.com/rust-lang/rust/pull/86264)
5. [Trait upcasting (part2) ]()

本文为 第二个 PR 的描述。

---

在第一个 PR 发出之后，收到了官方成员（Member）的一些 review 意见。其中之一就是促进第二个 PR 的原因，被记录于 [issues #86324](https://github.com/rust-lang/rust/issues/86324) 。

第二个 PR 的目标是在[#86291 (comment)](https://github.com/rust-lang/rust/pull/86291#issuecomment-861306586) 中描述：

>  第一步是重构 miri 中的 vtable 生成，以在`Machine`上下文之外创建一个`Allocation`。 
>
> 在 `cg_{clif,ssa} `中的 vtable 代码生成器的地方可以调用此函数，然后再调用任何用于降级到后端常量分配的方法。
>
>  将 `trait + type -> allocation` 的映射添加到 `tcx.alloc_map` 或类似的东西来替换后端内部实现也不错。

一句话描述：修改`miri`和两套`codegen` 以便让它使用`tcx`中构建的用`allocation`表示的 vtable。

## 编译器内部概念说明

`tcx` 是指类型上下文，是由编译器内部 `rustc_middle::ty`模块定义的，它是编译器内部核心数据结构。

Rust 的类型在编译器内部，由 `Ty `表示。当我们说`Ty`的时候，是指`rustc_middle::ty::Ty`，而不是指`rustc_hir::Ty`，了解它们之间的区别是比较重要的。

### `rustc_hir::Ty` vs `ty::Ty`

`rustc_hir::Ty`表示脱糖以后的类型，而`ty::Ty` 代表了类型的语义。

例如，` fn foo(x: u32) → u32 { x } ` 这个函数中，`u32` 出现两次。从 HIR 的角度看，这是两个不同的类型实例，因为它们出现在程序中不同的地方，也就是说，它们有两个不同的 Span （位置）。但是对于 `ty::Ty`来说，`u32` 在整个程序中都是同一个类型，它代表的不是具体的类型实例。

除此之外，HIR 还会有更多的信息丢失。例如， `fn foo(x: &u32) -> &u32`，在 HIR 看来，它不需要 lifetime 信息，所以 `&u32` 是不完整的。但是对于 `ty::Ty` 来说，它是完整的包含了 lifetime 信息。

一个简单总结：

| `rustc_hir::Ty`                                              | `ty::Ty`                                                     |
| ------------------------------------------------------------ | ------------------------------------------------------------ |
| 描述类型的「语法」                                           | 描述类型的「语义」                                           |
| 每一个 `rustc_hir::Ty`都有自己的 `Span`                      | 整个程序而言都是同一个类型，并不特指某个类型实例             |
| `rustc_hir::Ty `有泛型和生命周期； 但是，其中一些生命周期是特殊标记，例如 `LifetimeName::Implicit`。 | `ty::Ty` 具有完整的类型，包括泛型和生命周期，即使用户将它们排除在外 |



HIR 是从 AST 中构建的，它产生在 `ty::Ty` 之前。在 HIR 构建之后，一些基本的类型推导和类型检查就完成了。`ty::Ty`就是被用于类型检查，并且确保所有的东西都有预期的类型。 `rustc_typeck::astconv` 模块负责将 `rustc_hir::Ty`转换为`ty::TY`。

### `ty::Ty` 实现

`rustc_middle::ty::Ty`实际上是`&TyS`的一个类型别名。`&TyS ` 是 `Type Structure`的简称。一般情况下，总是会通过 `ty::Ty` 这个类型别名来使用 `&TyS `  。

要分配一个新的类型，你可以使用`tcx`上定义的各种`mk_`方法。这些方法的名称主要与各种类型相对应。例如:

```rust
let array_ty = tcx.mk_array(elem_ty, len * 2); // 返回 Ty<'tcx> 
```

你也可以通过访问`tcx`的字段来找到`tcx`本身的各种常见类型：`tcx.types.bool`，`tcx.types.char`，等等。



## 修改文件概述

本次修改涉及 21 个文件。

1. compiler/rustc_codegen_cranelift/src/common.rs 
2. compiler/rustc_codegen_cranelift/src/constant.rs 
3. compiler/rustc_codegen_cranelift/src/lib.rs
4. compiler/rustc_codegen_cranelift/src/unsize.rs 
5. compiler/rustc_codegen_cranelift/src/vtable.rs 
6. compiler/rustc_codegen_llvm/src/common.rs
7. compiler/rustc_codegen_ssa/src/meth.rs
8. compiler/rustc_codegen_ssa/src/traits/consts.rs
9. compiler/rustc_middle/src/ty/context.rs 
10. compiler/rustc_middle/src/ty/mod.rs
11. compiler/rustc_middle/src/ty/vtable.rs
12. compiler/rustc_mir/src/interpret/eval_context.rs 
13. compiler/rustc_mir/src/interpret/intern.rs
14. compiler/rustc_mir/src/interpret/memory.rs 
15. compiler/rustc_mir/src/interpret/terminator.rs
16. compiler/rustc_mir/src/interpret/traits.rs 
17. src/test/ui/consts/const-eval/ub-upvars.32bit.stderr 
18. src/test/ui/consts/const-eval/ub-upvars.64bit.stderr
19.  src/test/ui/consts/issue-79690.64bit.stderr
20. src/test/ui/consts/miri_unleashed/mutable_references_err.32bit.stderr
21. src/test/ui/consts/miri_unleashed/mutable_references_err.64bit.stderr

修改主要涉及 五个组件：

1. `rustc_middle`，属于 rust 编译器的 main crate ，包含rustc“家族”中的其他crate使用的通用类型定义，包括 HIR/MIR/Types。
2. `rustc_codegen_ssa`，截至2021年1月，RustC_Codegen_SSA 为所有后端提供了一个抽象的接口，以允许其他Codegen后端（例如Cranelift）。
3. `rustc_mir`，用于操作 MIR 的库。
4. `rustc_codegen_cranelift`，是 基于 cranelift 的编译器后端，专门用于 debug 模式
5. `rustc_codegen_llvm`，是 基于 llvm 的编译器后端，专门用于 release 模式



###  rustc_middle 库中的修改



1. 首先新增 `src/ty/vtable.rs` 模块，将 `vtable ` 的内存分配移动到 `rustc_middle`，以达到通用的目的。
2. 在 `src/ty/mod.rs` 中将 `vtable` 模块导入
3. 在 `src/ty/context.rs` 中增加 `vtables_cache`。



 **`src/ty/vtable.rs` 模块** 

```rust
use std::convert::TryFrom;

use crate::mir::interpret::{alloc_range, AllocId, Allocation, Pointer, Scalar};
use crate::ty::fold::TypeFoldable;
use crate::ty::{self, DefId, SubstsRef, Ty, TyCtxt}; // 导入 `ty`模块中相关类型 
use rustc_ast::Mutability;

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

impl<'tcx> TyCtxt<'tcx> {
  	// 给 vtable 分配内存，`TyCtxt` 中包含一个缓存，所以必须删除其重复数据
    /// Retrieves an allocation that represents the contents of a vtable.
    /// There's a cache within `TyCtxt` so it will be deduplicated.
    pub fn vtable_allocation(
        self,
        ty: Ty<'tcx>,
        poly_trait_ref: Option<ty::PolyExistentialTraitRef<'tcx>>,
    ) -> AllocId {
        let tcx = self;
        let vtables_cache = tcx.vtables_cache.lock();
        if let Some(alloc_id) = vtables_cache.get(&(ty, poly_trait_ref)).cloned() {
            return alloc_id;
        }
        drop(vtables_cache);

        // See https://github.com/rust-lang/rust/pull/86475#discussion_r655162674
        assert!(
            !ty.needs_subst() && !poly_trait_ref.map_or(false, |trait_ref| trait_ref.needs_subst())
        );
        let param_env = ty::ParamEnv::reveal_all();
        let vtable_entries = if let Some(poly_trait_ref) = poly_trait_ref {
            let trait_ref = poly_trait_ref.with_self_ty(tcx, ty);
            let trait_ref = tcx.erase_regions(trait_ref);

            tcx.vtable_entries(trait_ref)
        } else {
            COMMON_VTABLE_ENTRIES
        };

        let layout =
            tcx.layout_of(param_env.and(ty)).expect("failed to build vtable representation");
        assert!(!layout.is_unsized(), "can't create a vtable for an unsized type");
        let size = layout.size.bytes();
        let align = layout.align.abi.bytes();

        let ptr_size = tcx.data_layout.pointer_size;
        let ptr_align = tcx.data_layout.pointer_align.abi;

        let vtable_size = ptr_size * u64::try_from(vtable_entries.len()).unwrap();
        let mut vtable = Allocation::uninit(vtable_size, ptr_align);

      
        // 无需对下面的内存访问进行任何对齐检查，因为我们知道
				// 分配正确对齐，因为我们在上面创建了它。 我们也只是抵消了
				// `ptr_align` 的倍数，这意味着它将与 `ptr_align` 保持对齐
        // No need to do any alignment checks on the memory accesses below, because we know the
        // allocation is correctly aligned as we created it above. Also we're only offsetting by
        // multiples of `ptr_align`, which means that it will stay aligned to `ptr_align`.

        for (idx, entry) in vtable_entries.iter().enumerate() {
            let idx: u64 = u64::try_from(idx).unwrap();
            let scalar = match entry {
                VtblEntry::MetadataDropInPlace => {
                    let instance = ty::Instance::resolve_drop_in_place(tcx, ty);
                    let fn_alloc_id = tcx.create_fn_alloc(instance);
                    let fn_ptr = Pointer::from(fn_alloc_id);
                    fn_ptr.into()
                }
                VtblEntry::MetadataSize => Scalar::from_uint(size, ptr_size).into(),
                VtblEntry::MetadataAlign => Scalar::from_uint(align, ptr_size).into(),
                VtblEntry::Vacant => continue,
                VtblEntry::Method(def_id, substs) => {
                    // See https://github.com/rust-lang/rust/pull/86475#discussion_r655162674
                    assert!(!substs.needs_subst());

                    // Prepare the fn ptr we write into the vtable.
                    let instance =
                        ty::Instance::resolve_for_vtable(tcx, param_env, *def_id, substs)
                            .expect("resolution failed during building vtable representation")
                            .polymorphize(tcx);
                    let fn_alloc_id = tcx.create_fn_alloc(instance);
                    let fn_ptr = Pointer::from(fn_alloc_id);
                    fn_ptr.into()
                }
            };
            vtable
                .write_scalar(&tcx, alloc_range(ptr_size * idx, ptr_size), scalar)
                .expect("failed to build vtable representation");
        }

        vtable.mutability = Mutability::Not;
        let alloc_id = tcx.create_memory_alloc(tcx.intern_const_alloc(vtable));
        let mut vtables_cache = self.vtables_cache.lock();
        vtables_cache.insert((ty, poly_trait_ref), alloc_id);
        alloc_id
    }
}
```



**`src/ty/context.rs`**

```rust
pub struct GlobalCtxt<'tcx> {
    // ...
  
    // 不过在合并以后，eddyb 对此代码提出了异议： https://github.com/rust-lang/rust/pull/86475/files#r680788892
    // FxHashMap 是 rustc 内部使用的一个 hashmap 结构，使用了比 fnv 还快的 hasher，因为这里没有必要防止 DoS 攻击
    pub(super) vtables_cache:
        Lock<FxHashMap<(Ty<'tcx>, Option<ty::PolyExistentialTraitRef<'tcx>>), AllocId>>,
}

impl<'tcx> TyCtxt<'tcx> {
    pub fn create_global_ctxt( /* ... */ ) {
        // ...
        GlobalCtxt {
            // ...
            vtables_cache: Default::default(),
        }
    }
}
```





###  rustc_codegen_ssa 中的修改



修改 `src/traits/consts.rs` 中的 `ConstMethods ` trait，该 trait 定义了一些方法用于调用不同 后端的相关实现。比如在 `rustc_codegen_llvm`中：

```rust
impl ConstMethods<'tcx> for CodegenCx<'ll, 'tcx> {
    // ... 
}
```

在 `src/traits/consts.rs` 中 ：

```rust


pub trait ConstMethods<'tcx>: BackendTypes {
 
    // ...
  
    fn const_data_from_alloc(&self, alloc: &Allocation) -> Self::Value;
    // ...
  
}

```

然后在` src/meth.rs` 中引入 `ty::Ty`，并移除 vtable 内存分配相关代码

```rust


use rustc_middle::ty::{self, Ty};


pub fn get_vtable<'tcx, Cx: CodegenMethods<'tcx>>(
    cx: &Cx,
    ty: Ty<'tcx>,
    trait_ref: Option<ty::PolyExistentialTraitRef<'tcx>>,
) -> Cx::Value {
    let tcx = cx.tcx();

    debug!("get_vtable(ty={:?}, trait_ref={:?})", ty, trait_ref);

    // Check the cache.
    if let Some(&val) = cx.vtables().borrow().get(&(ty, trait_ref)) {
        return val;
    }
   
    // 新增
    let vtable_alloc_id = tcx.vtable_allocation(ty, trait_ref);
    let vtable_allocation = tcx.global_alloc(vtable_alloc_id).unwrap_memory();
    let vtable_const = cx.const_data_from_alloc(vtable_allocation);
  
    let align = cx.data_layout().pointer_align.abi;
    let vtable = cx.static_addr_of(vtable_const, align, Some("vtable"));
    cx.create_vtable_metadata(ty, vtable);
    cx.vtables().borrow_mut().insert((ty, trait_ref), vtable);
    vtable
}
```



###  rustc_mir 中的修改

viable 内存分配已经被定义在了 `rustc_middle::ty::Ty` 中，所以要移除 `rustc_mir` 中 vtable 内存分配相关代码。

`rustc_mir` 中修改的是 miri 相关代码，miri 用于编译器常量计算。

在 `compiler/rustc_mir/src/interpret/intern.rs` 内删除 Vtable 相关内存类型。 该模块用于 常量计算的全局内存分配。

```rust
// compiler/rustc_mir/src/interpret/intern.rs
fn intern_shallow<'rt, 'mir, 'tcx, M: CompileTimeMachine<'mir, 'tcx, const_eval::MemoryKind>>(
    ecx: &'rt mut InterpCx<'mir, 'tcx, M>,
    leftover_allocations: &'rt mut FxHashSet<AllocId>,
    alloc_id: AllocId,
    mode: InternMode,
    ty: Option<Ty<'tcx>>,
) -> Option<IsStaticOrFn> {
    // ...
    match kind {
        MemoryKind::Stack
        | MemoryKind::Machine(const_eval::MemoryKind::Heap)
//      | MemoryKind::Vtable  // 移除
        | MemoryKind::CallerLocation => {}
    }
    // ...
  
}

```



在 `compiler/rustc_mir/src/interpret/eval_context.rs` 中删除 vtable cache相关：

```rust
// compiler/rustc_mir/src/interpret/eval_context.rs

pub struct InterpCx<'mir, 'tcx, M: Machine<'mir, 'tcx>> {
    // ...
  
   //  移除下面三行
   //  /// A cache for deduplicating vtables
   //  pub(super) vtables:
   //      FxHashMap<(Ty<'tcx>, Option<ty::PolyExistentialTraitRef<'tcx>>), Pointer<M::PointerTag>>,
 
    // ...
  
}

impl<'mir, 'tcx: 'mir, M: Machine<'mir, 'tcx>> InterpCx<'mir, 'tcx, M> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        root_span: Span,
        param_env: ty::ParamEnv<'tcx>,
        machine: M,
        memory_extra: M::MemoryExtra,
    ) -> Self {
        InterpCx {
            machine,
            tcx: tcx.at(root_span),
            param_env,
            memory: Memory::new(tcx, memory_extra),
//          vtables: FxHashMap::default(), // 移除此行
        }
    }
    // ...
}
```



在 `compiler/rustc_mir/src/interpret/memory.rs` 中：

```rust
impl<T: MayLeak> MayLeak for MemoryKind<T> {
    #[inline]
    fn may_leak(self) -> bool {
        match self {
            MemoryKind::Stack => false,
//          MemoryKind::Vtable => true, // 移除此行
            MemoryKind::CallerLocation => true,
            MemoryKind::Machine(k) => k.may_leak(),
        }
    }
}

impl<T: fmt::Display> fmt::Display for MemoryKind<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryKind::Stack => write!(f, "stack variable"),
//          MemoryKind::Vtable => write!(f, "vtable"), // 移除此行
            MemoryKind::CallerLocation => write!(f, "caller location"),
            MemoryKind::Machine(m) => write!(f, "{}", m),
        }
    }
}
```

在 `compiler/rustc_mir/src/interpret/terminator.rs ` 中：

```rust
impl<'mir, 'tcx: 'mir, M: Machine<'mir, 'tcx>> InterpCx<'mir, 'tcx, M> {
    // ...
  
    /// Call this function -- pushing the stack frame and initializing the arguments.
    fn eval_fn_call(
        &mut self,
        fn_val: FnVal<'tcx, M::ExtraFnVal>,
        caller_abi: Abi,
        args: &[OpTy<'tcx, M::PointerTag>],
        ret: Option<(&PlaceTy<'tcx, M::PointerTag>, mir::BasicBlock)>,
        mut unwind: StackPopUnwind,
    ) -> InterpResult<'tcx> {
         // ...
      	 
         // 这里处理trait对象
         ty::InstanceDef::Virtual(_, idx) => {
             // ...
             // Find and consult vtable
             let vtable = receiver_place.vtable();
             let fn_val = self.get_vtable_slot(vtable, u64::try_from(idx).unwrap())?; // 修改 `drop_val` 为 `fn_val`

             // ...
             // recurse with concrete function
             self.eval_fn_call(fn_val, caller_abi, &args, ret, unwind)
         }
    }
    // ...
  
}
```

在 `compiler/rustc_mir/src/interpret/traits.rs` 中：

```rust
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
        trace!("get_vtable(trait_ref={:?})", poly_trait_ref);

        let (ty, poly_trait_ref) = self.tcx.erase_regions((ty, poly_trait_ref));

        // All vtables must be monomorphic, bail out otherwise.
        ensure_monomorphic_enough(*self.tcx, ty)?;
        ensure_monomorphic_enough(*self.tcx, poly_trait_ref)?;

        // 移除了之前的大部分代码，浓缩为这两行
        // 为 vtable 分配内存，并拿到相关指针
        let vtable_allocation = self.tcx.vtable_allocation(ty, poly_trait_ref);
        let vtable_ptr = self.memory.global_base_pointer(Pointer::from(vtable_allocation))?;

        Ok(vtable_ptr)
    }
}
```



### rustc_codegen_cranelift 中的修改

在 `rustc_codegen_cranelift` 中也是移除 vtable 内存分配相关代码。

上一个 PR 分析文章中说到， `rustc_codgen_cranelift` 因为没有依赖 `rust_codgen_ssa`的一些关键trait，所以vtable 内存分配这里还存在冗余代码。在重构 vtable 内存分配之后，就可以将这些冗余代码消除了。



在 `compiler/rustc_codegen_cranelift/src/vtable.rs` 中：

```rust
pub(crate) fn get_vtable<'tcx>(
    fx: &mut FunctionCx<'_, '_, 'tcx>,
    ty: Ty<'tcx>, // 这里使用了 `ty::Ty`
    trait_ref: Option<ty::PolyExistentialTraitRef<'tcx>>,
) -> Value {
    // 删除了之前的内存分配相关代码（主要是 build_vtable 函数），精简很多
    let vtable_ptr = if let Some(vtable_ptr) = fx.vtables.get(&(ty, trait_ref)) {
        *vtable_ptr
    } else {
        let vtable_alloc_id = fx.tcx.vtable_allocation(ty, trait_ref);
        let vtable_allocation = fx.tcx.global_alloc(vtable_alloc_id).unwrap_memory();
        let vtable_ptr = pointer_for_allocation(fx, vtable_allocation);

        fx.vtables.insert((ty, trait_ref), vtable_ptr);
        vtable_ptr
    };

    vtable_ptr.get_addr(fx)
}


```

主要是这个方法的修改，其他修改都是围绕该方法的琐碎修改。

### rustc_codegen_llvm 中的修改

在 `compiler/rustc_codegen_llvm/src/common.rs ` 中：

```rust
impl ConstMethods<'tcx> for CodegenCx<'ll, 'tcx> {
    // ...
  
  	fn const_data_from_alloc(&self, alloc: &Allocation) -> Self::Value {
        const_alloc_to_llvm(self, alloc)
    }
  
    // ...
  
}
```

## 小结

这次 PR  主要是将 vtable 的内存分配重构到 `rustc_middle::ty::Ty` ，以便其他组件可以公用。这里只是一个大概梳理，还有很多细节可以深究。


