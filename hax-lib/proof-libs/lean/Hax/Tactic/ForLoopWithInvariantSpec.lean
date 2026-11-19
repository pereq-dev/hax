module
public import Hax.Tactic.ForLoopWithInvariant

@[expose] public section

/-! # Spec lemmas for `forLoopWithInvariant`

The user-facing `forLoopWithInvariant_spec*` lemmas reduce to the `loop_range_spec*` lemmas in
`Hax.MissingAeneas`; `Iterator::next` on the range is handled by the
`core.«%S».Insts.CoreIterRangeStep.next_spec` lemmas in `CoreModels.Spec.Core.Iter`. -/

set_option autoImplicit false
set_option mvcgen.warning false
set_option linter.deprecated.syntax false

open CoreModels Aeneas
open Aeneas.Std hiding namespace core alloc
open RustM ControlFlow Error
open Std.Do
open Std.Tactic

namespace Hax

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
