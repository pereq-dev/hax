module
public import Aeneas
public import CoreModels.Core
public import Hax.MissingLean
public import CoreModels.MissingAeneas

@[expose] public section

/-!

# Missing Aeneas

Material that should eventually be upstreamed into Aeneas.

-/

namespace Aeneas.Std
open Std.Do

abbrev RustM.holds (x : RustM Prop) : Prop := ⦃ ⌜ True ⌝ ⦄ x ⦃ ⇓ p => ⌜ p ⌝ ⦄

theorem RustM.deterministic (f : RustM α) [Inhabited α]:
    ∃ a, ⦃ ⌜ True ⌝ ⦄ f ⦃ ⇓?r =>  ⌜ r = a ⌝ ⦄ := by
  cases f with
  | ret a => simp [Triple, WP.wp, PredTrans.apply]
  | vis i k => cases i; simp [Triple, WP.wp, PredTrans.apply]
  | div => simp [Triple, WP.wp, PredTrans.apply]

noncomputable def RustM.toPure (f : RustM α) [Inhabited α] : α :=
  f.deterministic.choose

theorem RustM.toPure_spec_mayThrow (f : RustM α) [Inhabited α] :
    ⦃ ⌜ True ⌝ ⦄ f ⦃ ⇓?r =>  ⌜ r = f.toPure ⌝ ⦄ :=
  f.deterministic.choose_spec

theorem RustM.toPure_spec (f : RustM α) [Inhabited α]
    (h : ⦃ P ⦄ f ⦃ Q ⦄) :
    ⦃ P ⦄ f ⦃ (fun r =>  ⌜ r = f.toPure ⌝, Q.2) ⦄ := by
  have := h.and f (toPure_spec_mayThrow f)
  apply Triple.of_entails_right _ (Triple.of_entails_left _ this _)
    <;> simp

attribute [spec] Function.uncurry lift

/- Aeneas translates the coercion of an array to a slice into a call to this model. -/
attribute [spec] CoreModels.core.array.Array.as_slice

/-- A total-correctness `spec` on `RustM` means the program returns: failure is the only
effect, and both failure and divergence falsify `spec`. -/
theorem WP.spec_imp_exists {α : Type u} {m : RustM α} {P : WP.Post α} :
    WP.spec m P → (∃ y, m = .ok y ∧ P y) := by
  cases m with
  | ret y => intro h; exact ⟨y, rfl, by simpa using h⟩
  | vis i k => intro h; exact ((WP.spec_vis i k).1 h).elim
  | div => intro h; exact (WP.spec_div.1 h).elim

end Aeneas.Std

open CoreModels Aeneas
open Aeneas.Std hiding namespace core alloc
open RustM ControlFlow Error
open Std.Do

namespace Hax

/-! ## Loop-over-range specs

`loop`-over-`core.ops.range.Range` characterisations of a Rust `for i in s..e` loop,
signed (`IScalar ty`) and unsigned (`Usize`). These belong in Aeneas proper; they live
here until they can be upstreamed. -/

section loop_range_helpers

variable {α : Type}

theorem triple_noThrow_elim {x : RustM α} {Q : α → Assertion RustM.postShape}
    (h : ⦃ ⌜ True ⌝ ⦄ x ⦃ PostCond.noThrow Q ⦄) {v : α} (hv : x = ok v) :
    (Q v).down := by
  subst hv; simpa [Triple, WP.wp, PredTrans.apply] using h

theorem triple_noThrow_exists_ok {x : RustM α} {Q : α → Assertion RustM.postShape}
    (h : ⦃ ⌜ True ⌝ ⦄ x ⦃ PostCond.noThrow Q ⦄) : ∃ v, x = ok v := by
  cases x with
  | ret v => exact ⟨v, rfl⟩
  | vis i k => cases i; exact absurd h (by simp [Triple, WP.wp, PredTrans.apply])
  | div => exact absurd h (by simp [Triple, WP.wp, PredTrans.apply])

theorem triple_of_ok {x : RustM α} {v : α} {P : α → Prop}
    (hx : x = ok v) (hp : P v) :
    (⦃ ⌜ True ⌝ ⦄ x ⦃ ⇓ r => ⌜ P r ⌝ ⦄) := by
  subst hx; simp [Triple, WP.wp, hp, PredTrans.apply]

end loop_range_helpers

/-- Loop-over-range spec, signed index type; induction on `(e.val - start.val).toNat`. -/
theorem loop_range_spec {ty : IScalarTy} {β : Type}
    (body : (core.ops.range.Range (IScalar ty) × β) →
      RustM (ControlFlow (core.ops.range.Range (IScalar ty) × β) β))
    (init : β) (s e : IScalar ty) (inv : IScalar ty → β → RustM Prop)
    (h_le : s.val ≤ e.val)
    (h_init : (inv s init).holds)
    (h_step : ∀ acc (i : IScalar ty), s.val ≤ i.val → i.val ≤ e.val →
      (inv i acc).holds →
      ⦃ ⌜ True ⌝ ⦄
      body ({ start := i, «end» := e }, acc)
      ⦃ ⇓ r => match r with
        | .cont (iter', acc') =>
          ⌜ i.val < e.val ∧ iter'.«end» = e ∧ iter'.start.val = i.val + 1
            ∧ (inv iter'.start acc').holds ⌝
        | .done y => ⌜ (inv e y).holds ⌝ ⦄) :
    ⦃ ⌜ True ⌝ ⦄
    loop body ({ start := s, «end» := e }, init)
    ⦃ ⇓ r => ⌜ (inv e r).holds ⌝ ⦄ := by
  suffices gen : ∀ (n : Nat) (acc : β) (start : IScalar ty),
    (e.val - start.val).toNat = n →
    s.val ≤ start.val → start.val ≤ e.val →
    (inv start acc).holds →
    ⦃ ⌜ True ⌝ ⦄ loop body ({ start := start, «end» := e }, acc)
    ⦃ ⇓ r => ⌜ (inv e r).holds ⌝ ⦄ by
    exact gen _ init s rfl (le_refl _) h_le h_init
  intro n
  induction n with
  | zero =>
    intro acc start hn hs_le hse_le hinv
    have hs := h_step acc start hs_le hse_le hinv
    obtain ⟨r, hbody⟩ := triple_noThrow_exists_ok hs
    have hpost := triple_noThrow_elim hs hbody
    rw [loop.eq_def, hbody, bind_ok]
    match r with
    | .cont (iter', acc') =>
      simp at hpost; exact absurd hpost.1 (by omega)
    | .done y =>
      simp at hpost; exact triple_of_ok rfl hpost
  | succ n ih =>
    intro acc start hn hs_le hse_le hinv
    have hs := h_step acc start hs_le hse_le hinv
    obtain ⟨r, hbody⟩ := triple_noThrow_exists_ok hs
    have hpost := triple_noThrow_elim hs hbody
    rw [loop.eq_def, hbody, bind_ok]
    match r with
    | .done y =>
      simp at hpost; exact triple_of_ok rfl hpost
    | .cont (iter', acc') =>
      simp at hpost
      obtain ⟨hlt, hend, hstart, hinv'⟩ := hpost
      have hiter : iter' = { start := iter'.start, «end» := e } := by
        cases iter'; cases hend; rfl
      rw [hiter]
      exact ih acc' iter'.start
        (by rw [hstart]; omega) (by rw [hstart]; omega) (by rw [hstart]; omega) hinv'

set_option maxHeartbeats 2000000 in
/-- Loop-over-range spec, unsigned `Usize` index type; induction on `e.val - start.val`. -/
theorem loop_range_spec_unsigned {β : Type}
    (body : (core.ops.range.Range Usize × β) →
      RustM (ControlFlow (core.ops.range.Range Usize × β) β))
    (init : β) (s e : Usize) (inv : Usize → β → RustM Prop)
    (h_le : s.val ≤ e.val)
    (h_init : (inv s init).holds)
    (h_step : ∀ acc (i : Usize), s.val ≤ i.val → i.val ≤ e.val →
      (inv i acc).holds →
      ⦃ ⌜ True ⌝ ⦄
      body ({ start := i, «end» := e }, acc)
      ⦃ ⇓ r => match r with
        | .cont (iter', acc') =>
          ⌜ i.val < e.val ∧ iter'.«end» = e ∧ iter'.start.val = i.val + 1
            ∧ (inv iter'.start acc').holds ⌝
        | .done y => ⌜ (inv e y).holds ⌝ ⦄) :
    ⦃ ⌜ True ⌝ ⦄
    loop body ({ start := s, «end» := e }, init)
    ⦃ ⇓ r => ⌜ (inv e r).holds ⌝ ⦄ := by
  suffices gen : ∀ (n : Nat) (acc : β) (start : Usize),
    e.val - start.val = n →
    s.val ≤ start.val → start.val ≤ e.val →
    (inv start acc).holds →
    ⦃ ⌜ True ⌝ ⦄ loop body ({ start := start, «end» := e }, acc)
    ⦃ ⇓ r => ⌜ (inv e r).holds ⌝ ⦄ by
    exact gen _ init s rfl (Nat.le_refl _) h_le h_init
  intro n
  induction n with
  | zero =>
    intro acc start hn hs_le hse_le hinv
    have hs := h_step acc start hs_le hse_le hinv
    obtain ⟨r, hbody⟩ := triple_noThrow_exists_ok hs
    have hpost := triple_noThrow_elim hs hbody
    rw [loop.eq_def, hbody, bind_ok]
    match r with
    | .cont (iter', acc') =>
      simp at hpost; exact absurd hpost.1 (by omega)
    | .done y =>
      simp at hpost; exact triple_of_ok rfl hpost
  | succ n ih =>
    intro acc start hn hs_le hse_le hinv
    have hs := h_step acc start hs_le hse_le hinv
    obtain ⟨r, hbody⟩ := triple_noThrow_exists_ok hs
    have hpost := triple_noThrow_elim hs hbody
    rw [loop.eq_def, hbody, bind_ok]
    match r with
    | .done y =>
      simp at hpost; exact triple_of_ok rfl hpost
    | .cont (iter', acc') =>
      simp at hpost
      obtain ⟨hlt, hend, hstart, hinv'⟩ := hpost
      have hiter : iter' = { start := iter'.start, «end» := e } := by
        cases iter'; cases hend; rfl
      rw [hiter]
      exact ih acc' iter'.start
        (by rw [hstart]; omega) (by rw [hstart]; omega) (by rw [hstart]; omega) hinv'

end Hax
