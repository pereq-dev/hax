module
public import CoreModels.Alloc.Funs

@[expose] public section

namespace CoreModels
namespace core

/-!

# Funs Epilogue

This file contains workarounds required to be present **after** `Funs.lean` runs.

See `FunsPrologue.lean` for workarounds that run before `Funs.lean`.

-/

/-! ## core::iter::range — Range iteration

Aeneas extracts `for i in lo..hi { … }` to a loop driven by
`core.iter.range.IteratorRange.next`, which in turn uses a
`core.iter.range.Step` dictionary. We provide both, plus a `StepUsize`
instance, so that downstream extracted code that iterates over `Range<usize>`
type-checks. -/

namespace iter.range

/-- The `Iterator::next` implementation for `core::ops::range::Range<A>`.
    Downstream extractions reference it under this name; the definition itself is
    in `FunsPrologue.lean`, which `Funs.lean` already needs before it. -/
abbrev IteratorRange.next := @_root_.CoreModels.core.IteratorRange.next

end iter.range

abbrev ops.range.Range.Insts.Core_modelsIterTraitsIteratorIterator.next :=
  @iter.range.IteratorRange.next

/-- Downstream `?` references this `Try::branch` impl under the un-suffixed name
    `…CoreOpsTry_traitTry.branch`, but our own extraction suffixes it
    `…TResultInfallibleE.branch`. Alias so `?` on `Result` elaborates. -/
abbrev result.Result.Insts.CoreOpsTry_traitTry.branch :=
  @result.Result.Insts.CoreOpsTry_traitTryTResultInfallibleE.branch

/-- Same aliasing as `Result` above, for `?` on `Option`. -/
abbrev option.Option.Insts.CoreOpsTry_traitTry.branch :=
  @option.Option.Insts.CoreOpsTry_traitTryTOptionInfallible.branch

/-- Downstream `?` on `Result` converts the residual through std's
    `FromResidual<Result<!, E>>` impl: std's `Infallible` is an alias of `!`, so that
    impl is suffixed `…NeverE`, while ours, written against our own `Infallible`, is
    suffixed `…InfallibleE`. Alias so `?` on `Result` elaborates. -/
abbrev result.Result.Insts.CoreOpsTry_traitFromResidualResultNeverE.from_residual :=
  @result.Result.Insts.CoreOpsTry_traitFromResidualResultInfallibleE.from_residual

/-- Same aliasing as `Result` above, for `?` on `Option`. -/
abbrev option.Option.Insts.CoreOpsTry_traitFromResidualOptionNever.from_residual :=
  @option.Option.Insts.CoreOpsTry_traitFromResidualOptionInfallible.from_residual

/-! ## Scalar Debug instances -/

/-! ## Provided methods kept OFF the `Iterator` structure

A field would be `<m>.default SELF`, whose resolution recurses through
`IntoIterator.Blanket SELF` — the cycle `impl_def` reports as `could not resolve
recursive fields`. Aeneas.Std keeps them off the structure too, as standalone
functions where the dictionaries are ordinary parameters. -/
open Aeneas.Std (RustM) in
def iter.traits.iterator.Iterator.collect.default {Self B Clause0_Item : Type _}
    (IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (collectFromIteratorInst : iter.traits.collect.FromIterator B Clause0_Item)
    (self : Self) : RustM B :=
  collectFromIteratorInst.from_iter
    (iter.traits.collect.IntoIterator.Blanket IteratorInst) self

/-! ### `rev`

`Self: DoubleEndedIterator` would make `Iterator` reference a trait that
references it back, which aeneas rejects. `DoubleEndedIterator`,
`ExactSizeIterator`, `Rev` and the `next_back` instances are all generated; only
this dispatch shim is hand-written. -/
-- aeneas emits `Iterator.rev.default (IteratorInst) (DEInst) (self)` at a downstream
-- `.rev()` call (see `iter_rev_range` in `tests/client_test/src/lib.rs`), so THAT
-- is the
-- signature `.default` must have (the dictionaries are ordinary, unused params).
open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::rev"]
def iter.traits.iterator.Iterator.rev.default
    {Self Clause0_Item Clause1_Item : Type _}
    (_IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (_DEInst : iter.traits.double_ended.DoubleEndedIterator Self Clause1_Item)
    (self : Self) : RustM (iter.adapters.rev.Rev Self) :=
  iter.adapters.rev.Rev.new self

/-! ### zip / chain / flat_map / flatten

Their `.default` takes the SELF `Iterator` instance, so a field would
self-reference the instance. -/
open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::zip"]
def iter.traits.iterator.Iterator.zip.default
    {Self U Clause0_Item Clause1_Item IntoIter : Type _}
    (IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (IntoIterInst : iter.traits.collect.IntoIterator U Clause1_Item IntoIter)
    (self : Self) (other : U) :
    RustM (iter.adapters.zip.Zip Self IntoIter) := do
  -- std bounds the argument by `IntoIterator`, so aeneas passes that dictionary
  -- and indexes the result by `U::IntoIter`. The Rust-side `IteratorMethods`
  -- still says `Iterator`: it only feeds F* and the differential tests.
  let b ← IntoIterInst.into_iter other
  iter.adapters.zip.Zip.new IteratorInst IntoIterInst.iteratorIteratorInst self b

open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::chain"]
def iter.traits.iterator.Iterator.chain.default
    {Self U Clause0_Item IntoIter : Type _}
    (IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (IntoIterInst : iter.traits.collect.IntoIterator U Clause0_Item IntoIter)
    (self : Self) (other : U) :
    RustM (iter.adapters.chain.Chain Self IntoIter) := do
  -- As `zip` above: std's bound is `U: IntoIterator<Item = Self::Item>`.
  let b ← IntoIterInst.into_iter other
  iter.adapters.chain.Chain.new IteratorInst IntoIterInst.iteratorIteratorInst self b

open Aeneas.Std (RustM) in
def iter.traits.iterator.Iterator.flat_map.default
    {Self U F Clause0_Item Clause1_Item : Type _}
    (IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (IteratorInst1 : iter.traits.iterator.Iterator U Clause1_Item)
    (FnInst : core.ops.function.FnMut F Clause0_Item U)
    (self : Self) (f : F) :
    RustM (iter.adapters.flat_map.FlatMap Self U F) :=
  iter.adapters.flat_map.FlatMap.new IteratorInst IteratorInst1 FnInst self f

open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::flat_map"]
def iter.traits.iterator.Iterator.flat_map.trait_default
    {Self U F Clause0_Item Clause1_Item : Type _}
    (IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (IteratorInst1 : iter.traits.iterator.Iterator U Clause1_Item)
    (FnInst : core.ops.function.FnMut F Clause0_Item U)
    (self : Self) (f : F) :
    RustM (iter.adapters.flat_map.FlatMap Self U F) :=
  iter.traits.iterator.Iterator.flat_map.default
    IteratorInst IteratorInst1 FnInst self f

open Aeneas.Std (RustM) in
def iter.traits.iterator.Iterator.flatten.default
    {Self Clause0_Item Clause1_Item : Type _}
    (IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (IteratorInst1 : iter.traits.iterator.Iterator Clause0_Item Clause1_Item)
    (self : Self) :
    RustM (iter.adapters.flatten.Flatten Self Clause0_Item Clause1_Item) :=
  iter.adapters.flatten.Flatten.new IteratorInst IteratorInst1 self

open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::flatten"]
def iter.traits.iterator.Iterator.flatten.trait_default
    {Self Clause0_Item Clause1_Item : Type _}
    (IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (IteratorInst1 : iter.traits.iterator.Iterator Clause0_Item Clause1_Item)
    (self : Self) :
    RustM (iter.adapters.flatten.Flatten Self Clause0_Item Clause1_Item) :=
  iter.traits.iterator.Iterator.flatten.default IteratorInst IteratorInst1 self

/-! ### Eager consumers

These consume the iterator, so a field would self-reference the instance. They
delegate to the generated `iter_*` loop helpers. `nth` is absent: its helper is
excluded (a Lean forward reference to `core.Usize.Insts.CoreIterRangeStep`), and
`sum`/`product` need the `Sum`/`Product` accumulator traits. -/
open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::fold"]
def iter.traits.iterator.Iterator.fold.default
    {Self B F Clause0_Item : Type _}
    (IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (FnInst : core.ops.function.FnMut F (B × Clause0_Item) B)
    (self : Self) (init : B) (f : F) : RustM B :=
  iter.traits.iterator.iter_fold IteratorInst FnInst self init f

open Aeneas.Std (RustM) in
-- `all`/`any`/`find`/`find_map`/`position` take `&mut self` in std, so aeneas's
-- `&mut` translation makes the call site destructure a `(result, iterator)` pair.
-- The shims return that pair rather than a bare result, threading the advanced
-- iterator straight out of the `iter_*` helper (which now takes `&mut I`).
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::all"]
def iter.traits.iterator.Iterator.all.default
    {Self F Clause0_Item : Type _}
    (IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (FnInst : core.ops.function.FnMut F Clause0_Item Bool)
    (self : Self) (f : F) : RustM (Bool × Self) :=
  iter.traits.iterator.iter_all IteratorInst FnInst self f

open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::any"]
def iter.traits.iterator.Iterator.any.default
    {Self F Clause0_Item : Type _}
    (IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (FnInst : core.ops.function.FnMut F Clause0_Item Bool)
    (self : Self) (f : F) : RustM (Bool × Self) :=
  iter.traits.iterator.iter_any IteratorInst FnInst self f

open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::find"]
def iter.traits.iterator.Iterator.find.default
    {Self P Clause0_Item : Type _}
    (IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (FnInst : core.ops.function.FnMut P Clause0_Item Bool)
    (self : Self) (predicate : P) :
    RustM ((option.Option Clause0_Item) × Self) :=
  iter.traits.iterator.iter_find IteratorInst FnInst self predicate

open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::find_map"]
def iter.traits.iterator.Iterator.find_map.default
    {Self B F Clause0_Item : Type _}
    (IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (FnInst : core.ops.function.FnMut F Clause0_Item (option.Option B))
    (self : Self) (f : F) : RustM ((option.Option B) × Self) :=
  iter.traits.iterator.iter_find_map IteratorInst FnInst self f

open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::position"]
def iter.traits.iterator.Iterator.position.default
    {Self P Clause0_Item : Type _}
    (IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (FnInst : core.ops.function.FnMut P Clause0_Item Bool)
    (self : Self) (predicate : P) :
    RustM ((option.Option Aeneas.Std.Usize) × Self) :=
  iter.traits.iterator.iter_position IteratorInst FnInst self predicate

open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::count"]
def iter.traits.iterator.Iterator.count.default
    {Self Clause0_Item : Type _}
    (IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (self : Self) : RustM Aeneas.Std.Usize :=
  iter.traits.iterator.iter_count IteratorInst self

open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::last"]
def iter.traits.iterator.Iterator.last.default
    {Self Clause0_Item : Type _}
    (IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (self : Self) : RustM (option.Option Clause0_Item) :=
  iter.traits.iterator.iter_last IteratorInst self

open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::for_each"]
def iter.traits.iterator.Iterator.for_each.default
    {Self F Clause0_Item : Type _}
    (IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (FnInst : core.ops.function.FnMut F Clause0_Item Unit)
    (self : Self) (f : F) : RustM Unit :=
  iter.traits.iterator.iter_for_each IteratorInst FnInst self f

open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::reduce"]
def iter.traits.iterator.Iterator.reduce.default
    {Self F Clause0_Item : Type _}
    (IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (FnInst : core.ops.function.FnMut F (Clause0_Item × Clause0_Item) Clause0_Item)
    (self : Self) (f : F) : RustM (option.Option Clause0_Item) :=
  iter.traits.iterator.iter_reduce IteratorInst FnInst self f

open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::min"]
def iter.traits.iterator.Iterator.min.default
    {Self Clause0_Item : Type _}
    (IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (OrdInst : cmp.Ord Clause0_Item)
    (self : Self) : RustM (option.Option Clause0_Item) :=
  iter.traits.iterator.iter_min IteratorInst OrdInst self

open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::max"]
def iter.traits.iterator.Iterator.max.default
    {Self Clause0_Item : Type _}
    (IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (OrdInst : cmp.Ord Clause0_Item)
    (self : Self) : RustM (option.Option Clause0_Item) :=
  iter.traits.iterator.iter_max IteratorInst OrdInst self

/-! ### Lazy adapter constructors

These could be fields, but that forced a `patch_lean.py` back-fill onto the
cross-crate `alloc` instances, so they are standalone too. Bodies just build the
adapter via its generated `::new`. -/
-- Signatures match aeneas's downstream emission `Iterator.<m>.default (IteratorInst)
-- [otherInsts] (self) [args]`, as pinned by the `iter_*` canary functions in
-- `tests/client_test/src/lib.rs`. `map`'s closure dictionary is `FnMut` (std uses FnMut), matching the
-- `Map` adapter's `FnMut` bound.
open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::map"]
def iter.traits.iterator.Iterator.map.default
    {Self O F Clause0_Item : Type _}
    (_IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (_FnMutInst : core.ops.function.FnMut F Clause0_Item O)
    (self : Self) (f : F) : RustM (iter.adapters.map.Map Self F) :=
  iter.adapters.map.Map.new self f

open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::enumerate"]
def iter.traits.iterator.Iterator.enumerate.default
    {Self Clause0_Item : Type _}
    (_IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (self : Self) : RustM (iter.adapters.enumerate.Enumerate Self) :=
  iter.adapters.enumerate.Enumerate.new self

open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::step_by"]
def iter.traits.iterator.Iterator.step_by.default
    {Self Clause0_Item : Type _}
    (_IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (self : Self) (step : Aeneas.Std.Usize) :
    RustM (iter.adapters.step_by.StepBy Self) :=
  iter.adapters.step_by.StepBy.new self step

open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::take"]
def iter.traits.iterator.Iterator.take.default
    {Self Clause0_Item : Type _}
    (_IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (self : Self) (n : Aeneas.Std.Usize) :
    RustM (iter.adapters.take.Take Self) :=
  iter.adapters.take.Take.new self n

open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::skip"]
def iter.traits.iterator.Iterator.skip.default
    {Self Clause0_Item : Type _}
    (_IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (self : Self) (n : Aeneas.Std.Usize) :
    RustM (iter.adapters.skip.Skip Self) :=
  iter.adapters.skip.Skip.new self n

open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::filter"]
def iter.traits.iterator.Iterator.filter.default
    {Self P Clause0_Item : Type _}
    (_IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (_FnInst : core.ops.function.FnMut P Clause0_Item Bool)
    (self : Self) (predicate : P) : RustM (iter.adapters.filter.Filter Self P) :=
  iter.adapters.filter.Filter.new self predicate

open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::filter_map"]
def iter.traits.iterator.Iterator.filter_map.default
    {Self B F Clause0_Item : Type _}
    (_IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (_FnInst : core.ops.function.FnMut F Clause0_Item (option.Option B))
    (self : Self) (f : F) : RustM (iter.adapters.filter_map.FilterMap Self F) :=
  iter.adapters.filter_map.FilterMap.new self f

open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::take_while"]
def iter.traits.iterator.Iterator.take_while.default
    {Self P Clause0_Item : Type _}
    (_IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (_FnInst : core.ops.function.FnMut P Clause0_Item Bool)
    (self : Self) (predicate : P) :
    RustM (iter.adapters.take_while.TakeWhile Self P) :=
  iter.adapters.take_while.TakeWhile.new self predicate

open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::skip_while"]
def iter.traits.iterator.Iterator.skip_while.default
    {Self P Clause0_Item : Type _}
    (_IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (_FnInst : core.ops.function.FnMut P Clause0_Item Bool)
    (self : Self) (predicate : P) :
    RustM (iter.adapters.skip_while.SkipWhile Self P) :=
  iter.adapters.skip_while.SkipWhile.new self predicate

open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::map_while"]
def iter.traits.iterator.Iterator.map_while.default
    {Self B F Clause0_Item : Type _}
    (_IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (_FnInst : core.ops.function.FnMut F Clause0_Item (option.Option B))
    (self : Self) (f : F) : RustM (iter.adapters.map_while.MapWhile Self F) :=
  iter.adapters.map_while.MapWhile.new self f

open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::inspect"]
def iter.traits.iterator.Iterator.inspect.default
    {Self F Clause0_Item : Type _}
    (_IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (_FnInst : core.ops.function.FnMut F Clause0_Item Unit)
    (self : Self) (f : F) : RustM (iter.adapters.inspect.Inspect Self F) :=
  iter.adapters.inspect.Inspect.new self f

open Aeneas.Std (RustM) in
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::fuse"]
def iter.traits.iterator.Iterator.fuse.default
    {Self Clause0_Item : Type _}
    (_IteratorInst : iter.traits.iterator.Iterator Self Clause0_Item)
    (self : Self) :
    RustM (iter.adapters.fuse.Fuse Self) :=
  iter.adapters.fuse.Fuse.new self

/-! ## `str::as_bytes`

`Str` is definitionally `Slice U8` (Aeneas.Std `StringDef`), so `str::as_bytes` is
the identity on the underlying bytes — faithful, no opacity, no trust expansion.
(The verify path passes a domain-separator `label : Str` to `Transcript::new`.) -/
open Aeneas.Std in
def str.Str.as_bytes (s : Str) : Aeneas.Std.RustM (Slice U8) :=
  .ok s

/-! ## Per-impl specialisations of the eager consumers

Aeneas emits the generic `.default` for the lazy constructors, but a per-impl
`<Receiver>.Insts.<Inst>.<m>` for `count`/`fold`/`last`/`min`/`max` on a concrete
receiver. Each is that default with the receiver's instance applied, so this set
grows with downstream code. The same consumers on a POLYMORPHIC receiver are
record-field projections, which this model cannot serve — see `poly_*` in
`tests/client_test/src/lib.rs`. -/

-- Range<A>
-- (`…IteratorIterator.count` is NOT defined here: `FunsPrologue.lean` already
-- provides it, derived from `Step::steps_between` instead of driving `next`.)
abbrev ops.range.Range.Insts.CoreIterTraitsIteratorIterator.last
    {A : Type _} (StepInst : iter.range.Step A) :=
  iter.traits.iterator.Iterator.last.default
    (ops.range.Range.Insts.CoreIterTraitsIteratorIterator StepInst)

abbrev ops.range.Range.Insts.CoreIterTraitsIteratorIterator.min
    {A : Type _} (StepInst : iter.range.Step A) :=
  iter.traits.iterator.Iterator.min.default
    (ops.range.Range.Insts.CoreIterTraitsIteratorIterator StepInst)

abbrev ops.range.Range.Insts.CoreIterTraitsIteratorIterator.max
    {A : Type _} (StepInst : iter.range.Step A) :=
  iter.traits.iterator.Iterator.max.default
    (ops.range.Range.Insts.CoreIterTraitsIteratorIterator StepInst)

-- slice::Iter<T>
abbrev slice.iter.Iter.Insts.CoreIterTraitsIteratorIteratorSharedAT.count
    {T : Type _} :=
  iter.traits.iterator.Iterator.count.default
    (slice.iter.Iter.Insts.CoreIterTraitsIteratorIteratorSharedAT T)

abbrev slice.iter.Iter.Insts.CoreIterTraitsIteratorIteratorSharedAT.fold
    {T B F : Type _}
    (FoldFnInst : core.ops.function.FnMut F (B × T) B) :=
  iter.traits.iterator.Iterator.fold.default
    (slice.iter.Iter.Insts.CoreIterTraitsIteratorIteratorSharedAT T) FoldFnInst

abbrev slice.iter.Iter.Insts.CoreIterTraitsIteratorIteratorSharedAT.all
    {T F : Type _} (FnInst : core.ops.function.FnMut F T Bool) :=
  iter.traits.iterator.Iterator.all.default
    (slice.iter.Iter.Insts.CoreIterTraitsIteratorIteratorSharedAT T) FnInst

abbrev slice.iter.Iter.Insts.CoreIterTraitsIteratorIteratorSharedAT.any
    {T F : Type _} (FnInst : core.ops.function.FnMut F T Bool) :=
  iter.traits.iterator.Iterator.any.default
    (slice.iter.Iter.Insts.CoreIterTraitsIteratorIteratorSharedAT T) FnInst

-- Map<I, F>
abbrev iter.adapters.map.Map.Insts.CoreIterTraitsIteratorIterator.fold
    {I O F Clause0_Item B G : Type _}
    (IteratorInst : iter.traits.iterator.Iterator I Clause0_Item)
    (FnMutInst : core.ops.function.FnMut F Clause0_Item O)
    (FoldFnInst : core.ops.function.FnMut G (B × O) B) :=
  iter.traits.iterator.Iterator.fold.default
    (iter.adapters.map.Map.Insts.CoreIterTraitsIteratorIterator IteratorInst FnMutInst)
    FoldFnInst

-- Enumerate<I>
abbrev iter.adapters.enumerate.Enumerate.Insts.CoreIterTraitsIteratorIteratorPairUsizeClause0_Item.fold
    {I Clause0_Item B G : Type _}
    (IteratorInst : iter.traits.iterator.Iterator I Clause0_Item)
    (FoldFnInst : core.ops.function.FnMut G (B × (Aeneas.Std.Usize × Clause0_Item)) B) :=
  iter.traits.iterator.Iterator.fold.default
    (iter.adapters.enumerate.Enumerate.Insts.CoreIterTraitsIteratorIteratorPairUsizeClause0_Item
      IteratorInst) FoldFnInst

-- Skip<I>
abbrev iter.adapters.skip.Skip.Insts.CoreIterTraitsIteratorIterator.count
    {I Clause0_Item : Type _}
    (IteratorInst : iter.traits.iterator.Iterator I Clause0_Item) :=
  iter.traits.iterator.Iterator.count.default
    (iter.adapters.skip.Skip.Insts.CoreIterTraitsIteratorIterator IteratorInst)

-- Take<I>
abbrev iter.adapters.take.Take.Insts.CoreIterTraitsIteratorIterator.count
    {I Clause0_Item : Type _}
    (IteratorInst : iter.traits.iterator.Iterator I Clause0_Item) :=
  iter.traits.iterator.Iterator.count.default
    (iter.adapters.take.Take.Insts.CoreIterTraitsIteratorIterator IteratorInst)

-- Fuse<I>
abbrev iter.adapters.fuse.Fuse.Insts.CoreIterTraitsIteratorIterator.count
    {I Clause0_Item : Type _}
    (IteratorInst : iter.traits.iterator.Iterator I Clause0_Item) :=
  iter.traits.iterator.Iterator.count.default
    (iter.adapters.fuse.Fuse.Insts.CoreIterTraitsIteratorIterator IteratorInst)

-- Filter<I, P>
abbrev iter.adapters.filter.Filter.Insts.CoreIterTraitsIteratorIterator.count
    {I P Clause0_Item : Type _}
    (IteratorInst : iter.traits.iterator.Iterator I Clause0_Item)
    (FnMutInst : core.ops.function.FnMut P Clause0_Item Bool) :=
  iter.traits.iterator.Iterator.count.default
    (iter.adapters.filter.Filter.Insts.CoreIterTraitsIteratorIterator IteratorInst FnMutInst)

-- Chain<A, B>
abbrev iter.adapters.chain.Chain.Insts.CoreIterTraitsIteratorIterator.count
    {A B Clause0_Item : Type _}
    (IteratorInst : iter.traits.iterator.Iterator A Clause0_Item)
    (IteratorInst1 : iter.traits.iterator.Iterator B Clause0_Item) :=
  iter.traits.iterator.Iterator.count.default
    (iter.adapters.chain.Chain.Insts.CoreIterTraitsIteratorIterator IteratorInst IteratorInst1)

-- Flatten<I, _, _>
abbrev iter.adapters.flatten.Flatten.Insts.CoreIterTraitsIteratorIterator.count
    {I Clause0_Item Clause1_Item : Type _}
    (IteratorInst : iter.traits.iterator.Iterator I Clause0_Item)
    (IteratorInst1 : iter.traits.iterator.Iterator Clause0_Item Clause1_Item) :=
  iter.traits.iterator.Iterator.count.default
    (iter.adapters.flatten.Flatten.Insts.CoreIterTraitsIteratorIterator
      IteratorInst IteratorInst1)


/-! ## `FromIterator<Result<A, E>> for Result<V, E>`

The Rust model in `result.rs` is the real fold, but extracting it trips aeneas's
`type_var_id` on `IntoIterator::Item`, as for `Vec`. Reproduced here — same fold,
not a stub; `SeqIter` and its instance are generated. -/
open Aeneas.Std (RustM) in
def result.Result.Insts.CoreIterTraitsCollectFromIteratorResult.from_iter_loop
    {A E IntoIter : Type _}
    (iterInst : iter.traits.iterator.Iterator IntoIter (result.Result A E))
    (it : IntoIter) (acc : rust_primitives.sequence.Seq A) :
    RustM ((option.Option E) × (rust_primitives.sequence.Seq A)) := do
  let (o, it1) ← iterInst.next it
  match o with
  | option.Option.None => .ok (option.Option.None, acc)
  | option.Option.Some (result.Result.Ok a) => do
    let acc1 ← rust_primitives.sequence.seq_push acc a
    result.Result.Insts.CoreIterTraitsCollectFromIteratorResult.from_iter_loop
      iterInst it1 acc1
  | option.Option.Some (result.Result.Err e) =>
    .ok (option.Option.Some e, acc)
partial_fixpoint

open Aeneas.Std (RustM) in
def result.Result.Insts.CoreIterTraitsCollectFromIteratorResult.from_iter
    {A E V T IntoIter : Type _}
    (FromIteratorInst : iter.traits.collect.FromIterator V A)
    (IntoIteratorInst : iter.traits.collect.IntoIterator T (result.Result A E) IntoIter)
    -- NOT named `iter`: that shadows the `iter` namespace, so `iter.traits.…`
    -- below would parse as a field projection (cf. `rename_iter_param` in
    -- `patch_lean.py`).
    (input : T) : RustM (result.Result V E) := do
  let it ← IntoIteratorInst.into_iter input
  let empty ← rust_primitives.sequence.seq_empty A
  let (err, acc) ←
    result.Result.Insts.CoreIterTraitsCollectFromIteratorResult.from_iter_loop
      IntoIteratorInst.iteratorIteratorInst it empty
  match err with
  | option.Option.Some e => .ok (result.Result.Err e)
  | option.Option.None => do
    let v ← FromIteratorInst.from_iter
      (iter.traits.collect.IntoIterator.Blanket
        (result.SeqIter.Insts.CoreIterTraitsIteratorIterator A)) acc
    .ok (result.Result.Ok v)

-- Argument order matches aeneas's emission at a `collect::<Result<_, _>>()`
-- call site: the error type `E` explicitly (it appears nowhere else, so it
-- cannot be inferred), then the `FromIterator` dictionary for the collection,
-- from which `A` and `V` follow.
@[reducible]
def result.Result.Insts.CoreIterTraitsCollectFromIteratorResult
    (E : Type _) {A V : Type _} (FromIteratorInst : iter.traits.collect.FromIterator V A) :
    iter.traits.collect.FromIterator (result.Result V E) (result.Result A E) := {
  from_iter := fun {_T _IntoIter : Type _} IntoIteratorInst =>
    result.Result.Insts.CoreIterTraitsCollectFromIteratorResult.from_iter
      FromIteratorInst IntoIteratorInst
}


end core

namespace alloc

/-! ## `IntoIterator` for `&Vec<T>` (aeneas's `SharedAVec`)

`(&vec).into_iter()` / iterating a `&Vec<T>` yields `&T` via a slice `Iter`. aeneas
names this shared-reference `IntoIterator` instance `alloc.SharedAVec.Insts.
CoreIterTraitsCollectIntoIteratorSharedATIter` (cf. Aeneas.Std's `SharedArray` for
`&[T; N]`). Core-models didn't provide it; supply `into_iter` by hand —
`Vec::as_slice` then the slice `Iter` constructor — matching the name/signature the
extraction calls directly. -/
open Aeneas.Std (RustM) in
def SharedAVec.Insts.CoreIterTraitsCollectIntoIteratorSharedATIter.into_iter
    {T : Type _} (v : vec.Vec T) : RustM (core.slice.iter.Iter T) := do
  let s ← vec.Vec.as_slice v
  core.slice.Slice.iter s

/-! ## `IntoIter::map` (a provided `Iterator` method)

`map` lives on the extraction-excluded `IteratorMethods` trait, so Aeneas
never synthesises the per-impl `Iterator::map` specialisation that a
downstream crate references when it writes `v.into_iter().map(f)`. We supply
it by hand, mirroring Aeneas's own builtin `Aeneas/Std/VecIter.lean` (which
this project shadows via `open Aeneas.Std hiding namespace core alloc`).

The body just builds the
`Map` adapter; iteration then runs through `Map`'s own `Iterator` instance.
`F` is the closure, `T` the item, `O` its output (the `FnMut` instance is
irrelevant to the model, hence `_`-prefixed). -/
def vec.into_iter.IntoIter.Insts.CoreIterTraitsIteratorIterator.map
  {T O F : Type _} (_FnMutInst : core.ops.function.FnMut F T O) :
  vec.into_iter.IntoIter T → F →
  Aeneas.Std.RustM (core.iter.adapters.map.Map (vec.into_iter.IntoIter T) F) :=
  fun it f => .ok { iter := it, f := f }

/-! ## `FromIterator<T>` for `VecDeque<T, Global>`-/

open Aeneas.Std (RustM) in
def collections.vec_deque.VecDequeTGlobal.Insts.CoreIterTraitsCollectFromIterator.from_iter_loop
    {T IntoIter : Type _}
    (iterInst : core.iter.traits.iterator.Iterator IntoIter T)
    (it : IntoIter) (res : collections.vec_deque.VecDeque T alloc.Global) : RustM (collections.vec_deque.VecDeque T alloc.Global) := do
  let (o, it1) ← iterInst.next it
  match o with
  | core.option.Option.None => .ok res
  | core.option.Option.Some x =>
    let res1 ← collections.vec_deque.VecDeque.push_back res x
    collections.vec_deque.VecDequeTGlobal.Insts.CoreIterTraitsCollectFromIterator.from_iter_loop iterInst it1 res1
partial_fixpoint

open Aeneas.Std (RustM) in
def collections.vec_deque.VecDequeTGlobal.Insts.CoreIterTraitsCollectFromIterator.from_iter
    {T I IntoIter : Type _}
    (IntoIteratorInst : core.iter.traits.collect.IntoIterator I T IntoIter)
    (iter : I) : RustM (collections.vec_deque.VecDeque T alloc.Global) := do
  let res ← collections.vec_deque.VecDequeTGlobal.new T
  let it ← IntoIteratorInst.into_iter iter
  collections.vec_deque.VecDequeTGlobal.Insts.CoreIterTraitsCollectFromIterator.from_iter_loop
    IntoIteratorInst.iteratorIteratorInst it res

@[reducible]
def collections.vec_deque.VecDequeTGlobal.Insts.CoreIterTraitsCollectFromIterator
  (T : Type _) :
  core.iter.traits.collect.FromIterator
    (collections.vec_deque.VecDeque T alloc.Global) T := {
  from_iter := fun {T1 Clause0_IntoIter : Type _}
    (IntoIteratorInst : core.iter.traits.collect.IntoIterator T1 T Clause0_IntoIter) =>
    collections.vec_deque.VecDequeTGlobal.Insts.CoreIterTraitsCollectFromIterator.from_iter IntoIteratorInst
}

/-! ## Real (computable) `FromIterator<T>` for `Vec<T>`

`collect::<Vec<_>>()` is idiomatic and must be executable. The Rust impl folds
via `next` into a `Vec` (`vec.Vec.push`), but Aeneas can't extract it — it hits
`type_var_id` resolving the `IntoIterator::Item` associated type (the same aeneas
bug the carve saw), so the impl stays `--exclude`d and we hand-write it, exactly
as Aeneas.Std hand-writes `alloc.vec.FromIteratorVec`. `IntoIterator` carries
`iteratorIteratorInst` (the `IntoIter: Iterator` bound) and
`FromIterator::from_iter` pins `Item = A`, so the fold type-checks and `collect`
is computable. -/
open Aeneas.Std (RustM) in
def vec.Vec.Insts.CoreIterTraitsCollectFromIterator.from_iter_loop
    {T IntoIter : Type _}
    (iterInst : core.iter.traits.iterator.Iterator IntoIter T)
    (it : IntoIter) (res : vec.Vec T) : RustM (vec.Vec T) := do
  let (o, it1) ← iterInst.next it
  match o with
  | core.option.Option.None => .ok res
  | core.option.Option.Some x =>
    let res1 ← vec.Vec.push res x
    vec.Vec.Insts.CoreIterTraitsCollectFromIterator.from_iter_loop iterInst it1 res1
partial_fixpoint

open Aeneas.Std (RustM) in
def vec.Vec.Insts.CoreIterTraitsCollectFromIterator.from_iter
    {T I IntoIter : Type _}
    (IntoIteratorInst : core.iter.traits.collect.IntoIterator I T IntoIter)
    (iter : I) : RustM (vec.Vec T) := do
  let res ← vec.Vec.new T
  let it ← IntoIteratorInst.into_iter iter
  vec.Vec.Insts.CoreIterTraitsCollectFromIterator.from_iter_loop
    IntoIteratorInst.iteratorIteratorInst it res

@[reducible]
def vec.Vec.Insts.CoreIterTraitsCollectFromIterator (T : Type _) :
    core.iter.traits.collect.FromIterator (vec.Vec T) T := {
  from_iter := fun {T1 Clause0_IntoIter : Type _}
    (IntoIteratorInst : core.iter.traits.collect.IntoIterator T1 T Clause0_IntoIter) =>
    vec.Vec.Insts.CoreIterTraitsCollectFromIterator.from_iter IntoIteratorInst
}

/-! ## `[T]::to_vec` and `Box<[T]>::into_vec`

Aeneas's builtin name map turns `<[T]>::to_vec` into a reference to
`alloc.slice.Slice.to_vec` (and similarly for `into_vec`). Our local
`alloc/` crate provides those bodies, but under the `alloc.slice.Dummy`
namespace because of the standard "you can't `impl` for a foreign slice
type" workaround. Re-export them at the std-map name so downstream
extractions land on a defined symbol.
-/

noncomputable section

@[rust_fun "alloc::slice::{[@T]}::to_vec"]
def slice.Slice.to_vec
  {T : Type _} (cloneInst : core.clone.Clone T) (s : Aeneas.Std.Slice T) :
  Aeneas.Std.RustM (vec.Vec T) :=
  slice.Dummy.to_vec cloneInst s

@[rust_fun "alloc::slice::{alloc::boxed::Box<[@T], @A>}::into_vec"]
def slice.Slice.into_vec
  {T : Type _} (s : Aeneas.Std.Slice T) : Aeneas.Std.RustM (vec.Vec T) :=
  slice.Dummy.into_vec s

end

end alloc
end CoreModels
