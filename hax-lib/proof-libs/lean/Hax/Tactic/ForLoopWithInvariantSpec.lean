module
public import Hax.Tactic.ForLoopWithInvariant

@[expose] public section

/-! # Spec lemmas for `forLoopWithInvariant`

`IteratorRange_next_spec*` characterise `Iterator::next` for the signed (`I32`) and
unsigned (`Usize`) `Step` dictionaries; the user-facing `forLoopWithInvariant_spec*`
lemmas reduce to the `loop_range_spec*` lemmas in `Hax.MissingAeneas`. -/

set_option autoImplicit false
set_option mvcgen.warning false
set_option linter.deprecated.syntax false

open CoreModels Aeneas
open Aeneas.Std hiding namespace core alloc
open RustM ControlFlow Error
open Std.Do
open Std.Tactic

namespace Hax

/-! ## Iterator `next` spec (signed, `I32`)

The signed `forward_checked` steps via `wrapping_add`; `i.val < e.val ≤ I32.max` rules
out the wrap. -/

section next_spec_helpers

private theorem hcast_cast_one_val :
    (UScalar.hcast IScalarTy.I32 (UScalar.cast UScalarTy.U32 (1#usize))).val = 1 := by
  simp only [UScalar.hcast, IScalar.val, BitVec.toInt_setWidth]; grind

/-- `wrapping_add i 1 = i + 1` when `i < i32::MAX` (no wrap). -/
private theorem i32_wrapping_add_one_val (i : I32)
    (h1 : -2147483648 ≤ i.val) (h2 : i.val ≤ 2147483646) :
    (i.wrapping_add (UScalar.hcast IScalarTy.I32 (UScalar.cast UScalarTy.U32 1#usize))).val
      = i.val + 1 := by
  simp only [I32.wrapping_add_val_eq, hcast_cast_one_val, Nat.reducePow]
  grind

end next_spec_helpers

@[spec]
theorem IteratorRange_next_spec (i e : I32) {Q}
    (h_lt : (h : i.val < e.val) →
      ∀ (s : I32), s.val = i.val + 1 →
        (Q.1 (some i, { start := s, «end» := e })).down)
    (h_ge : i.val ≥ e.val →
      (Q.1 (none, { start := i, «end» := e })).down) :
    ⦃ ⌜ True ⌝ ⦄
    core.IteratorRange.next core.I32.Insts.CoreIterRangeStep
      { start := i, «end» := e }
    ⦃ Q ⦄ := by
  unfold core.IteratorRange.next core.I32.Insts.CoreIterRangeStep
  by_cases h : i.val < e.val
  · have h_lt' := h_lt h
    simp_all [compare, compareOfLessAndEq,
      core.I32.Insts.CoreCmpPartialOrdI32, core.mkIPartialOrd,
      core.I32.Insts.CoreCloneClone.clone,
      core.I32.Insts.CoreIterRangeStep.forward_checked,
      core.U32.Insts.CoreConvertTryFromUsizeTryFromIntError.try_from,
      core.num.U32.MAX, core.num.U32.MIN,
      core.num.I32.wrapping_add, rust_primitives.arithmetic.wrapping_add_i32]
    mvcgen
    all_goals first
      | refine h_lt' _ ?_
        exact i32_wrapping_add_one_val i (by scalar_tac) (by scalar_tac)
      | simp_all only [U32.rMax, hcast_cast_one_val]
        first | scalar_tac | grind
  · have h_ge' := h_ge (by omega)
    simp only [compare, compareOfLessAndEq,
      core.I32.Insts.CoreCmpPartialOrdI32, core.mkIPartialOrd]
    mvcgen
    have hlt : ¬ (i.val < e.val) := by omega
    by_cases hie : i.val = e.val <;> simp_all

/-! ## Iterator `next` spec (unsigned, `Usize`) -/

@[spec]
theorem IteratorRange_next_spec_usize (i e : Usize) {Q}
    (h_lt : (h : i.val < e.val) →
      ∀ (s : Usize), s.val = i.val + 1 →
        (Q.1 (some i, { start := s, «end» := e })).down)
    (h_ge : i.val ≥ e.val →
      (Q.1 (none, { start := i, «end» := e })).down) :
    ⦃ ⌜ True ⌝ ⦄
    core.IteratorRange.next core.Usize.Insts.CoreIterRangeStep
      { start := i, «end» := e }
    ⦃ Q ⦄ := by
  unfold core.IteratorRange.next core.Usize.Insts.CoreIterRangeStep
  by_cases h : i.val < e.val
  · simp_all [compare, compareOfLessAndEq,
      core.Usize.Insts.CoreCmpPartialOrdUsize, core.mkUPartialOrd,
      core.Usize.Insts.CoreCloneClone.clone,
      core.Usize.Insts.CoreIterRangeStep.forward_checked,
      core.convert.TryFromUTInfallible.Blanket.try_from,
      core.convert.From.Blanket.from,
      core.num.Usize.checked_add, core.num.Usize.overflowing_add,
      rust_primitives.arithmetic.overflowing_add_usize]
    mvcgen [uncurry]
      <;> grind [UScalar.overflowing_add_eq i (1#usize)]
  · have h_ge' := h_ge (Nat.le_of_not_lt h)
    simp_all [core.Usize.Insts.CoreCmpPartialOrdUsize, core.mkUPartialOrd]
    mvcgen; grind

/-! ## User-facing `@[spec]` lemma (signed, `I32`) -/

@[spec]
theorem forLoopWithInvariant_spec {β : Type}
    (body : I32 → β → RustM β)
    (init : β) (s e : I32) (inv : I32 → β → RustM Prop)
    (h_le : s.val ≤ e.val)
    (h_init : (inv s init).holds)
    (h_step : ∀ acc (i : I32), s.val ≤ i.val → i.val < e.val →
      (inv i acc).holds →
      ⦃ ⌜ True ⌝ ⦄
      body i acc
      ⦃ ⇓ r => ⌜ ∀ (i' : I32), i'.val = i.val + 1 → (inv i' r).holds ⌝ ⦄) :
    ⦃ ⌜ True ⌝ ⦄
    Hax.forLoopWithInvariant core.I32.Insts.CoreIterRangeStep inv body
      { start := s, «end» := e } init
    ⦃ ⇓ r => ⌜ (inv e r).holds ⌝ ⦄ := by
  unfold Hax.forLoopWithInvariant
  apply loop_range_spec _ init s e inv h_le h_init
  intro acc i hsi hie hinv
  unfold  RustM.holds at hinv
  mvcgen [hinv, uncurry, h_step]
    <;> grind only [ScalarTac.IScalar.bounds, IScalar.eq_equiv]

/-! ## User-facing `@[spec]` lemma (unsigned, `Usize`) -/

@[spec]
theorem forLoopWithInvariant_spec_usize {β : Type}
    (body : Usize → β → RustM β)
    (init : β) (s e : Usize) (inv : Usize → β → RustM Prop)
    (h_le : s.val ≤ e.val)
    (h_init : (inv s init).holds)
    (h_step : ∀ acc (i : Usize), s.val ≤ i.val → i.val < e.val →
      (inv i acc).holds →
      ⦃ ⌜ True ⌝ ⦄
      body i acc
      ⦃ ⇓ r => ⌜ ∀ (i' : Usize), i'.val = i.val + 1 → (inv i' r).holds ⌝ ⦄) :
    ⦃ ⌜ True ⌝ ⦄
    Hax.forLoopWithInvariant core.Usize.Insts.CoreIterRangeStep inv body
      { start := s, «end» := e } init
    ⦃ ⇓ r => ⌜ (inv e r).holds ⌝ ⦄ := by
  unfold Hax.forLoopWithInvariant
  apply loop_range_spec_unsigned _ init s e inv h_le h_init
  intro acc i hsi hie hinv
  unfold  RustM.holds at hinv
  mvcgen [hinv, uncurry, h_step]
  · grind
  · have : i = e := by grind
    grind

end Hax
