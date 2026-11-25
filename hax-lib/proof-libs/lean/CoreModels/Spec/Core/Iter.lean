module
public import CoreModels.MissingAeneas
public import CoreModels.Core.Funs
public import CoreModels.Spec.Core.Convert
import all Aeneas.Std.Scalar.OverflowingOps.Add

@[expose] public section

/-! # Specs for `core::iter::range` -/

namespace CoreModels

open Aeneas
open Aeneas.Std hiding namespace core alloc
open Std.Do WP RustM ScalarElab

set_option mvcgen.warning false
set_option linter.deprecated.syntax false

section

/- The proofs below see through the integer comparison and `Clone` implementations. -/
attribute [local spec]
  core.mkUPartialOrd
  core.mkIPartialOrd
  core.U8.Insts.CoreCmpPartialOrdU8
  core.U16.Insts.CoreCmpPartialOrdU16
  core.U32.Insts.CoreCmpPartialOrdU32
  core.U64.Insts.CoreCmpPartialOrdU64
  core.U128.Insts.CoreCmpPartialOrdU128
  core.Usize.Insts.CoreCmpPartialOrdUsize
  core.I8.Insts.CoreCmpPartialOrdI8
  core.I16.Insts.CoreCmpPartialOrdI16
  core.I32.Insts.CoreCmpPartialOrdI32
  core.I64.Insts.CoreCmpPartialOrdI64
  core.I128.Insts.CoreCmpPartialOrdI128
  core.Isize.Insts.CoreCmpPartialOrdIsize
  core.U8.Insts.CoreCloneClone.clone
  core.U16.Insts.CoreCloneClone.clone
  core.U32.Insts.CoreCloneClone.clone
  core.U64.Insts.CoreCloneClone.clone
  core.U128.Insts.CoreCloneClone.clone
  core.Usize.Insts.CoreCloneClone.clone
  core.I8.Insts.CoreCloneClone.clone
  core.I16.Insts.CoreCloneClone.clone
  core.I32.Insts.CoreCloneClone.clone
  core.I64.Insts.CoreCloneClone.clone
  core.I128.Insts.CoreCloneClone.clone
  core.Isize.Insts.CoreCloneClone.clone

section forward_checked

/- The `forward_checked` proofs also see through the integer arithmetic they step with. -/
attribute [local spec]
  core.U8.Insts.CoreIterRangeStep.forward_checked
  core.U16.Insts.CoreIterRangeStep.forward_checked
  core.U32.Insts.CoreIterRangeStep.forward_checked
  core.U64.Insts.CoreIterRangeStep.forward_checked
  core.U128.Insts.CoreIterRangeStep.forward_checked
  core.Usize.Insts.CoreIterRangeStep.forward_checked
  core.I8.Insts.CoreIterRangeStep.forward_checked
  core.I16.Insts.CoreIterRangeStep.forward_checked
  core.I32.Insts.CoreIterRangeStep.forward_checked
  core.I64.Insts.CoreIterRangeStep.forward_checked
  core.I128.Insts.CoreIterRangeStep.forward_checked
  core.Isize.Insts.CoreIterRangeStep.forward_checked
  core.U8.Insts.CoreConvertTryFromUsizeTryFromIntError.try_from
  core.U16.Insts.CoreConvertTryFromUsizeTryFromIntError.try_from
  core.U32.Insts.CoreConvertTryFromUsizeTryFromIntError.try_from
  core.U64.Insts.CoreConvertTryFromUsizeTryFromIntError.try_from
  core.num.U8.checked_add
  core.num.U8.overflowing_add
  core.num.U16.checked_add
  core.num.U16.overflowing_add
  core.num.U32.checked_add
  core.num.U32.overflowing_add
  core.num.U64.checked_add
  core.num.U64.overflowing_add
  core.num.U128.checked_add
  core.num.U128.overflowing_add
  core.num.Usize.checked_add
  core.num.Usize.overflowing_add
  core.num.I8.checked_add
  core.num.I8.overflowing_add
  core.num.I16.checked_add
  core.num.I16.overflowing_add
  core.num.I32.checked_add
  core.num.I32.overflowing_add
  core.num.I64.checked_add
  core.num.I64.overflowing_add
  core.num.I128.checked_add
  core.num.I128.overflowing_add
  core.num.Isize.checked_add
  core.num.Isize.overflowing_add
  core.num.I8.wrapping_add
  core.num.I16.wrapping_add
  core.num.I32.wrapping_add
  core.num.I64.wrapping_add
  core.num.I128.wrapping_add
  core.num.Isize.wrapping_add

private theorem IScalar.overflowing_add_of_inBounds {ty} (x y : IScalar ty)
    (h1 : -2 ^ (ty.numBits - 1) ≤ x.val + y.val) (h2 : x.val + y.val < 2 ^ (ty.numBits - 1)) :
    (IScalar.overflowing_add x y).2 = false ∧
      (IScalar.overflowing_add x y).1.val = x.val + y.val := by
  simp only [IScalar.val] at h1 h2
  have hno : ¬ BitVec.saddOverflow x.bv y.bv := by
    simp only [BitVec.saddOverflow, Bool.or_eq_true, decide_eq_true_eq, not_or]; omega
  simp only [IScalar.overflowing_add, IScalar.val]
  exact ⟨by simpa using hno, BitVec.toInt_add_of_not_saddOverflow hno⟩

/-! ## `Step::forward_checked start 1`, the step taken by `Iterator::next` -/

uscalar_no_usize theorem core.«%S».Insts.CoreIterRangeStep.forward_checked_one_spec
    {Q : PostCond (Option «%S») postShape} (start : «%S») (h : start.val < «%S».max)
    (hQ : ∀ s : «%S», s.val = start.val + 1 → PostCond.ok Q (some s)) :
    ⦃ ⌜ True ⌝ ⦄
    core.«%S».Insts.CoreIterRangeStep.forward_checked start 1#usize
    ⦃ Q ⦄ := by
  have h1 : (UScalar.cast UScalarTy.«%S» 1#usize).val = 1 := by
    simp [UScalar.cast_val_eq]
  -- Converting `1` from `usize` succeeds (`u128` casts instead and does not use these).
  have hmax : ¬ 1#usize > UScalar.cast UScalarTy.Usize core.num.«%S».MAX := by
    obtain hb | hb := System.Platform.numBits_eq <;>
      simp [global_simps, UScalar.lt_equiv, UScalar.cast_val_eq, hb, «%S».rMax]
  have hmin : ¬ 1#usize < UScalar.cast UScalarTy.Usize core.num.«%S».MIN := by
    simp [global_simps, UScalar.lt_equiv, UScalar.cast_val_eq]
  -- `start + 1` does not overflow.
  obtain ⟨hval, hno⟩ :
      (UScalar.overflowing_add start (UScalar.cast _ 1#usize)).1.val = start.val + 1 ∧
      ¬ (UScalar.overflowing_add start (UScalar.cast _ 1#usize)).2 = true := by
    have hov := UScalar.overflowing_add_eq start (UScalar.cast _ 1#usize)
    simp only [h1] at hov
    rw [ite_eq_right (by scalar_tac)] at hov
    exact ⟨hov.1, by simp [hov.2]⟩
  mvcgen
  -- The successful step is the last goal; the others return `none` in branches ruled out above.
  rotate_right
  · subst_vars
    exact hQ _ hval
  all_goals
    subst_vars
    contradiction

theorem core.Usize.Insts.CoreIterRangeStep.forward_checked_one_spec
    {Q : PostCond (Option Usize) postShape} (start : Usize) (h : start.val < Usize.max)
    (hQ : ∀ s : Usize, s.val = start.val + 1 → PostCond.ok Q (some s)) :
    ⦃ ⌜ True ⌝ ⦄
    core.Usize.Insts.CoreIterRangeStep.forward_checked start 1#usize
    ⦃ Q ⦄ := by
  -- `start + 1` does not overflow.
  obtain ⟨hval, hno⟩ :
      (UScalar.overflowing_add start 1#usize).1.val = start.val + 1 ∧
      ¬ (UScalar.overflowing_add start 1#usize).2 = true := by
    have hov := UScalar.overflowing_add_eq start 1#usize
    rw [ite_eq_right (by scalar_tac)] at hov
    exact ⟨hov.1, by simp [hov.2]⟩
  -- `mvcgen` rules out the overflow branch with `hno`.
  mvcgen
  exact hQ _ hval

-- `wrapping_add_val_eq` is only needed for the types that step with `wrapping_add`.
set_option linter.unusedSimpArgs false in
iscalar theorem core.«%S».Insts.CoreIterRangeStep.forward_checked_one_spec
    {Q : PostCond (Option «%S») postShape} (start : «%S») (h : start.val < «%S».max)
    (hQ : ∀ s : «%S», s.val = start.val + 1 → PostCond.ok Q (some s)) :
    ⦃ ⌜ True ⌝ ⦄
    core.«%S».Insts.CoreIterRangeStep.forward_checked start 1#usize
    ⦃ Q ⦄ := by
  -- `start + 1` is in bounds.
  have hbd := IScalar.hBounds start
  have hmax : start.val + 1 < 2 ^ (IScalarTy.«%S».numBits - 1) := by
    simp only [«%S».max, «%S».numBits] at h; omega
  have h1 : (UScalar.hcast IScalarTy.«%S» 1#usize).val = 1 := by
    obtain hb | hb := System.Platform.numBits_eq <;> simp [UScalar.hcast_val_eq, hb, Int.bmod_def]
  -- `i128` steps with `checked_add`, which does not overflow here.
  obtain ⟨hno, hval⟩ := IScalar.overflowing_add_of_inBounds start (UScalar.hcast _ 1#usize)
    (by omega) (by omega)
  mvcgen
  all_goals subst_vars
  -- The branches returning `none` contradict `h`; this leaves the successful step.
  all_goals try
    exfalso
    obtain hb | hb := System.Platform.numBits_eq <;>
      simp_all [global_simps, U8.rMax, U16.rMax, U32.rMax, U64.rMax, UScalar.lt_equiv,
        UScalar.cast_val_eq, UScalar.hcast_val_eq, IScalar.wrapping_add_val_eq,
        IScalar.le_equiv, Int.bmod_def, IScalarTy.numBits, «%S».max, «%S».numBits] <;>
      split_ifs at * <;> omega
  -- The step adds `1`, by `wrapping_add` (without wrapping) or by `checked_add` for `i128`.
  apply hQ
  obtain hb | hb := System.Platform.numBits_eq <;>
    simp_all [UScalar.cast_val_eq, UScalar.hcast_val_eq, IScalar.wrapping_add_val_eq,
      Int.bmod_def, IScalarTy.numBits, «%S».max, «%S».numBits] <;>
    split_ifs <;> omega

end forward_checked

/-! ## `Iterator::next` on integer ranges -/

/-- The test `Iterator::next` uses to decide whether the range is non-empty. -/
private theorem isLess_eq_true_iff {α} [LinearOrder α] (a b : α) :
    (match core.option.Option.Some
        (match compare a b with
          | .lt => core.cmp.Ordering.Less
          | .eq => core.cmp.Ordering.Equal
          | .gt => core.cmp.Ordering.Greater) with
      | some o =>
        match o with
        | core.cmp.Ordering.Less => true
        | _ => false
      | _ => false) = true ↔ a < b := by
  rcases lt_trichotomy a b with h | h | h
  · simp [compare_lt_iff_lt.mpr h, h]
  · simp [h]
  · simp [compare_gt_iff_gt.mpr h, not_lt_of_gt h]

scalar @[spec] theorem core.«%S».Insts.CoreIterRangeStep.next_spec {Q}
    (range : core.ops.range.Range «%S»)
    (h_lt : range.start.val < range.end.val → ∀ s : «%S», s.val = range.start.val + 1 →
      PostCond.ok Q (some range.start, { start := s, «end» := range.end }))
    (h_ge : range.start.val ≥ range.end.val → PostCond.ok Q (none, range)) :
    ⦃ ⌜ True ⌝ ⦄
    core.IteratorRange.next core.«%S».Insts.CoreIterRangeStep range
    ⦃ Q ⦄ := by
  mvcgen [core.IteratorRange.next, core.«%S».Insts.CoreIterRangeStep.forward_checked_one_spec]
  case vc1.h =>
    -- `forward_checked_one_spec` requires `start < max`, which follows from `start < end`.
    have := (isLess_eq_true_iff _ _).mp ‹_›
    scalar_tac
  case vc2.hQ => exact h_lt ((isLess_eq_true_iff _ _).mp ‹_›) _ ‹_›
  case vc3.hQ => exact h_ge (not_lt.mp (mt (isLess_eq_true_iff _ _).mpr ‹_›))

end

end CoreModels
