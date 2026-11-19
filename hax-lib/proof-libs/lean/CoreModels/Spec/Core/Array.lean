module
public import CoreModels.MissingAeneas
public import CoreModels.Core.Funs
public import CoreModels.Spec.Core.Slice
public import CoreModels.Spec.RustPrimitives.Slice

@[expose] public section

namespace CoreModels

open Aeneas
open Aeneas.Std hiding namespace core alloc
open Std.Do WP RustM
set_option mvcgen.warning false
set_option linter.deprecated.syntax false

open ScalarElab

uscalar @[spec] theorem «%S».Array.eq_spec {N : Std.Usize} {Q}
  (a : Array «%S» N) (b : Array «%S» N) (h : PostCond.ok Q (a.val == b.val)) :
  ⦃ ⌜ True ⌝ ⦄
  core.Array.Insts.CoreCmpPartialEqArray.eq core.«%S».Insts.CoreCmpPartialEq'S a b
  ⦃ Q ⦄ := by
  mvcgen -trivial [core.Array.Insts.CoreCmpPartialEqArray.eq,
    core.Array.Insts.CoreCmpPartialEqArray.eq_loop,
    core.Array.Insts.CoreCmpPartialEqArray.eq_loop.body, rust_primitives.slice.array_index,
    core.«%S».Insts.CoreCmpPartialEq'S]
  case vc1.γ => exact Nat
  case vc4.termination => exact fun i => N.val - i.val
  case vc3.rel => exact (· < ·)
  case vc5.hwf => exact wellFounded_lt
  case vc2.inv => exact fun i => a.val.take i.val = b.val.take i.val
  case vc7.hQ =>
    constructor
    · simp_all [@List.take_add, @List.take_one, -List.take_append_getElem]
    · grind
  case vc9.hQ => convert h; grind
  case vc12.hQ => convert h; grind [List.take_eq_self_iff, List.Vector.length_val]
  all_goals grind

iscalar @[spec] theorem «%S».Array.eq_spec {N : Std.Usize} {Q}
  (a : Array «%S» N) (b : Array «%S» N) (h : PostCond.ok Q (a.val == b.val)) :
  ⦃ ⌜ True ⌝ ⦄
  core.Array.Insts.CoreCmpPartialEqArray.eq core.«%S».Insts.CoreCmpPartialEq'S a b
  ⦃ Q ⦄ := by
  mvcgen -trivial [core.Array.Insts.CoreCmpPartialEqArray.eq,
    core.Array.Insts.CoreCmpPartialEqArray.eq_loop,
    core.Array.Insts.CoreCmpPartialEqArray.eq_loop.body, rust_primitives.slice.array_index,
    core.«%S».Insts.CoreCmpPartialEq'S]
  case vc1.γ => exact Nat
  case vc4.termination => exact fun i => N.val - i.val
  case vc3.rel => exact (· < ·)
  case vc5.hwf => exact wellFounded_lt
  case vc2.inv => exact fun i => a.val.take i.val = b.val.take i.val
  case vc7.hQ =>
    constructor
    · simp_all [@List.take_add, @List.take_one, -List.take_append_getElem]
    · grind
  case vc9.hQ => convert h; grind
  case vc12.hQ => convert h; grind [List.take_eq_self_iff, List.Vector.length_val]
  all_goals grind

@[spec] theorem Bool.Array.eq_spec {N : Std.Usize} {Q}
  (a : Array Bool N) (b : Array Bool N) (h : PostCond.ok Q (a.val == b.val)) :
  ⦃ ⌜ True ⌝ ⦄
  core.Array.Insts.CoreCmpPartialEqArray.eq core.Bool.Insts.CoreCmpPartialEqBool a b
  ⦃ Q ⦄ := by
  mvcgen -trivial [core.Array.Insts.CoreCmpPartialEqArray.eq,
    core.Array.Insts.CoreCmpPartialEqArray.eq_loop,
    core.Array.Insts.CoreCmpPartialEqArray.eq_loop.body, rust_primitives.slice.array_index,
    core.Bool.Insts.CoreCmpPartialEqBool]
  case vc1.γ => exact Nat
  case vc4.termination => exact fun i => N.val - i.val
  case vc3.rel => exact (· < ·)
  case vc5.hwf => exact wellFounded_lt
  case vc2.inv => exact fun i => a.val.take i.val = b.val.take i.val
  case vc7.hQ =>
    constructor
    · simp_all [@List.take_add, @List.take_one, -List.take_append_getElem]
    · grind
  case vc9.hQ => convert h; grind
  case vc12.hQ => convert h; grind [List.take_eq_self_iff, List.Vector.length_val]
  all_goals grind

@[spec]
theorem Array.index_range_spec
      {T : Type} {N : Std.Usize} (arr : Std.Array T N)
      (r : core.ops.range.Range Std.Usize)
      (h0 : r.start.val ≤  r.end.val)
      (h1 : r.end.val ≤ N.val) :
    ⦃ ⌜ True ⌝ ⦄
    core.Array.Insts.CoreOpsIndexIndex.index
      (core.Shared0Slice.Insts.CoreOpsIndexIndexRangeUsizeSlice T)
      arr r
    ⦃ ⇓ r' => ⌜ r'.val = arr.val.slice r.start.val r.end.val ∧
                r'.val.length + r.start.val = r.end.val ⌝ ⦄ := by
  mvcgen [core.Array.Insts.CoreOpsIndexIndex.index, core.array.Array.as_slice,
      rust_primitives.slice.array_as_slice]
    <;> grind

attribute [spec] core.array.from_fn core.Array.Insts.CoreCmpPartialEqArray.ne

/-! ## `clone` -/

/-- Cloning an array clones each element; for an element type whose `clone` returns its
argument, the array is returned unchanged. -/
theorem core.Array.Insts.CoreCloneClone.clone_spec {T : Type} {N : Std.Usize}
    (inst : core.clone.Clone T)
    (hclone : ∀ x, ⦃ ⌜ True ⌝ ⦄ inst.clone x ⦃ ⇓ r => ⌜ r = x ⌝ ⦄) (arr : Array T N) :
    ⦃ ⌜ True ⌝ ⦄ core.Array.Insts.CoreCloneClone.clone inst arr ⦃ ⇓ r => ⌜ r = arr ⌝ ⦄ := by
  mvcgen [core.Array.Insts.CoreCloneClone.clone, core.array.Array.map,
    core.array.CloneArray.clone.closure.Insts.CoreOpsFunctionFnMutTupleTT.call_mut, hclone]
  rename_i res
  intro h
  apply Aeneas.Std.Array.ext
  apply List.ext_getElem (by rw [res.property, arr.property])
  intro i h1 h2
  apply triple_in_hypothesis _ (h i (by have := arr.property; omega))
  mvcgen [core.array.CloneArray.clone.closure.Insts.CoreOpsFunctionFnMutTupleTT.call_mut, hclone]
  intro h'
  rw [← h']
  assumption

/-! Cloning an array of integers or `bool`s returns the same array. -/

uscalar @[spec] theorem core.Array.Insts.CoreCloneClone.«%S».clone_spec {N : Std.Usize}
    (arr : Array «%S» N) :
    ⦃ ⌜ True ⌝ ⦄
    core.Array.Insts.CoreCloneClone.clone core.«%S».Insts.CoreCloneClone arr
    ⦃ ⇓ r => ⌜ r = arr ⌝ ⦄ :=
  core.Array.Insts.CoreCloneClone.clone_spec _
    (fun x => by mvcgen [core.«%S».Insts.CoreCloneClone.clone]) arr

iscalar @[spec] theorem core.Array.Insts.CoreCloneClone.«%S».clone_spec {N : Std.Usize}
    (arr : Array «%S» N) :
    ⦃ ⌜ True ⌝ ⦄
    core.Array.Insts.CoreCloneClone.clone core.«%S».Insts.CoreCloneClone arr
    ⦃ ⇓ r => ⌜ r = arr ⌝ ⦄ :=
  core.Array.Insts.CoreCloneClone.clone_spec _
    (fun x => by mvcgen [core.«%S».Insts.CoreCloneClone.clone]) arr

@[spec] theorem core.Array.Insts.CoreCloneClone.Bool.clone_spec {N : Std.Usize}
    (arr : Array Bool N) :
    ⦃ ⌜ True ⌝ ⦄
    core.Array.Insts.CoreCloneClone.clone core.Bool.Insts.CoreCloneClone arr
    ⦃ ⇓ r => ⌜ r = arr ⌝ ⦄ :=
  core.Array.Insts.CoreCloneClone.clone_spec _
    (fun x => by mvcgen [core.Bool.Insts.CoreCloneClone.clone]) arr


end CoreModels
