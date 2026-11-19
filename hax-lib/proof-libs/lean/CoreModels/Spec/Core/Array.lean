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


/-! ## `repeat` -/

/-- Cloning every element of a list with an identity `clone` returns the list. -/
private theorem mapM_clone_spec {T : Type} (inst : core.clone.Clone T)
    (hclone : ∀ y, ⦃ ⌜ True ⌝ ⦄ inst.clone y ⦃ ⇓ r => ⌜ r = y ⌝ ⦄) (l : List T) :
    ⦃ ⌜ True ⌝ ⦄ l.mapM inst.clone ⦃ ⇓ r => ⌜ r = l ⌝ ⦄ := by
  induction l with
  | nil => mvcgen
  | cons y l ih =>
    rw [List.mapM_cons]
    mvcgen [hclone, ih]
    subst_vars
    rfl

/-- With an identity `clone`, `seq_create` returns `n` copies of `x`. -/
private theorem seq_create_spec {T : Type} (inst : core.clone.Clone T)
    (hclone : ∀ y, ⦃ ⌜ True ⌝ ⦄ inst.clone y ⦃ ⇓ r => ⌜ r = y ⌝ ⦄) (x : T) (n : Std.Usize) :
    ⦃ ⌜ True ⌝ ⦄
    rust_primitives.sequence.seq_create inst x n
    ⦃ ⇓ s => ⌜ s.val = List.replicate n.val x ⌝ ⦄ := by
  obtain ⟨_, hmap, rfl⟩ :=
    triple_iff_exists_ok.1 (mapM_clone_spec inst hclone (List.replicate (n.val - 1) x))
  unfold rust_primitives.sequence.seq_create
  simp only [hmap, RustM.match.ok]
  mvcgen
  · -- `n = 0`: the empty sequence.
    rename_i hn
    exact RustM.ok_spec (by simp [hn, PostCond.ok, Slice.new_val]) trivial
  · -- `n - 1` clones followed by `x` itself.
    refine RustM.ok_spec ?_ trivial
    simp only [PostCond.ok, Slice.from_val, ← List.replicate_succ']
    show List.replicate (n.val - 1 + 1) x = List.replicate n.val x
    congr 1
    omega
  · -- `n` copies fit in a `usize`.
    rename_i hlen
    exact absurd (by simp; scalar_tac) hlen

/-- The closure of `array::repeat` hands out the first of the remaining copies. -/
private theorem repeat_call_mut_spec {T : Type} (N : Std.Usize) (inst : core.clone.Clone T)
    (x : T) (k : Nat) (hk : k < N.val) (c : core.array.repeat.closure T N)
    (hc : c.val = List.replicate (N.val - k) x) (i : Std.Usize) :
    ⦃ ⌜ True ⌝ ⦄
    core.array.repeat.closure.Insts.CoreOpsFunctionFnMutTupleUsizeT.call_mut inst c i
    ⦃ ⇓ r => ⌜ r.1 = x ∧ r.2.val = List.replicate (N.val - (k + 1)) x ⌝ ⦄ := by
  mvcgen [core.array.repeat.closure.Insts.CoreOpsFunctionFnMutTupleUsizeT.call_mut,
    rust_primitives.sequence.seq_remove]
  · -- Removing the first copy leaves one copy fewer.
    simp [hc, Slice.from_val, List.drop_replicate]
    omega
  · -- `k < N`, so there is a copy left to remove.
    rename_i hempty
    exact absurd (by simp [hc]; omega) hempty

/-- `[x; N]`: for an element type whose `clone` returns its argument, the array holds `N` copies
of `x`. -/
theorem core.array.repeat_spec {T : Type} (N : Std.Usize) (inst : core.clone.Clone T)
    (hclone : ∀ y, ⦃ ⌜ True ⌝ ⦄ inst.clone y ⦃ ⇓ r => ⌜ r = y ⌝ ⦄) (x : T) :
    ⦃ ⌜ True ⌝ ⦄ core.array.repeat N inst x ⦃ ⇓ r => ⌜ r = Array.repeat N x ⌝ ⦄ := by
  have hseq := seq_create_spec inst hclone x N
  have hfrom : ∀ s : rust_primitives.sequence.Seq T, s.val = List.replicate N.val x →
      ⦃ ⌜ True ⌝ ⦄
      rust_primitives.slice.array_from_fn N
        (core.array.repeat.closure.Insts.CoreOpsFunctionFnMutTupleUsizeT N inst) s
      ⦃ ⇓ a => ⌜ ∀ i : Nat, (hi : i < N.val) →
                  a.val[i]'(by have := a.property; omega) = x ⌝ ⦄ := fun s hs =>
    rust_primitives.slice.array_from_fn_inv_spec N _
      (fun k c => c.val = List.replicate (N.val - k) x) (fun _ y => y = x) s (by simpa using hs)
      (fun k hk c hc => repeat_call_mut_spec N inst x k hk c hc ⟨BitVec.ofNat _ k⟩)
  mvcgen [core.array.repeat, -rust_primitives.sequence.seq_create,
    -rust_primitives.slice.array_from_fn_spec, hseq, hfrom]
  · -- Every element is `x`.
    rename_i a
    intro h
    apply Aeneas.Std.Array.ext
    apply List.ext_getElem (by rw [a.property, Array.repeat_val, List.length_replicate])
    intro i h1 _
    rw [h i (by rw [a.property] at h1; exact h1)]
    simp [Array.repeat_val]
  · -- `seq_create` returns `N` copies.
    exact id

uscalar @[spec] theorem core.array.«%S».repeat_spec (N : Std.Usize) (x : «%S») :
    ⦃ ⌜ True ⌝ ⦄
    core.array.repeat N core.«%S».Insts.CoreCloneClone x
    ⦃ ⇓ r => ⌜ r = Array.repeat N x ⌝ ⦄ :=
  core.array.repeat_spec N _ (fun _ => by mvcgen [core.«%S».Insts.CoreCloneClone.clone]) x

iscalar @[spec] theorem core.array.«%S».repeat_spec (N : Std.Usize) (x : «%S») :
    ⦃ ⌜ True ⌝ ⦄
    core.array.repeat N core.«%S».Insts.CoreCloneClone x
    ⦃ ⇓ r => ⌜ r = Array.repeat N x ⌝ ⦄ :=
  core.array.repeat_spec N _ (fun _ => by mvcgen [core.«%S».Insts.CoreCloneClone.clone]) x

@[spec] theorem core.array.Bool.repeat_spec (N : Std.Usize) (x : Bool) :
    ⦃ ⌜ True ⌝ ⦄
    core.array.repeat N core.Bool.Insts.CoreCloneClone x
    ⦃ ⇓ r => ⌜ r = Array.repeat N x ⌝ ⦄ :=
  core.array.repeat_spec N _ (fun _ => by mvcgen [core.Bool.Insts.CoreCloneClone.clone]) x

end CoreModels
