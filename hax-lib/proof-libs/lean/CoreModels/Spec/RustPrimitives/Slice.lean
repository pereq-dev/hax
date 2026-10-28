module
public import CoreModels.Core.Funs
public import CoreModels.MissingAeneas

@[expose] public section

/-! # Specs for `rust_primitives::slice` -/

namespace CoreModels

open Aeneas
open Aeneas.Std hiding namespace core alloc
open Std.Do WP RustM

set_option mvcgen.warning false
set_option linter.deprecated.syntax false


/-! ## `array_from_fn` -/

@[spec]
theorem rust_primitives.slice.array_from_fn_go_spec
    {T F : Type}
    (inst : core.ops.function.FnMut F Std.Usize T) (c : F) (n : Nat)
    (hpure : ∀ k, k < n → ∀ c', c' = c →
      ⦃ ⌜ True ⌝ ⦄ inst.call_mut c' ⟨BitVec.ofNat _ k⟩ ⦃ ⇓ r => ⌜ r.2 = c ⌝ ⦄) :
    ⦃ ⌜ True ⌝ ⦄
    rust_primitives.slice.array_from_fn_go inst c n
    ⦃ ⇓ (rl, rc) => ⌜ rc = c ∧ ∃ h : rl.length = n, ∀ i, (hi : i < n) →
                ⦃ ⌜ True ⌝ ⦄ inst.call_mut c ⟨BitVec.ofNat _ i⟩
                          ⦃ ⇓ r' => ⌜ rl[i] = r'.1 ⌝ ⦄ ⌝ ⦄ := by
  induction n generalizing c with
  | zero =>
    mvcgen [rust_primitives.slice.array_from_fn_go]
    refine ⟨trivial, rfl, ?_⟩
    intro i hi; exact absurd hi (by simp)
  | succ n ih =>
    -- Enrich `hpure` mvcgen's VC still contains the fact that the value came from `call_mut`:
    have hpure' := fun k hk c' hc' => triple_with_self (hpure k hk c' hc')
    mvcgen [rust_primitives.slice.array_from_fn_go, ih, hpure']
    case vc6 =>
      rename_i r_rec h_rec r_call h_call
      obtain ⟨h_receq, h_reclen, h_recpost⟩ := h_rec
      obtain ⟨h_call2, h_callself⟩ := h_call
      refine ⟨h_call2, by simp [h_reclen], ?_⟩
      intro i hi
      rcases Nat.lt_succ_iff_lt_or_eq.mp hi with hlt | heq
      · -- `i < n`: the `i`-th element comes from the recursion.
        mvcgen [h_recpost]
        grind
      · -- `i = n`: the last element is `r_call.1`, pinned by `h_callself`.
        subst heq
        rw [← h_receq]
        mvcgen [h_callself]
        grind
    all_goals grind

/-- This spec assumes that the closure is not mutated. If the closure was mutated,
we would need a more complex spec that would require the user to provide an invariant. -/
@[spec]
theorem rust_primitives.slice.array_from_fn_spec
    {T F : Type} [Inhabited T] (N : Std.Usize)
    (inst : core.ops.function.FnMut F Std.Usize T) (c : F)
    (hpure : ∀ k : Nat, k < N.val →
      ⦃ ⌜ True ⌝ ⦄ inst.call_mut c ⟨BitVec.ofNat _ k⟩ ⦃ ⇓ r => ⌜ r.2 = c ⌝ ⦄) :
    ⦃ ⌜ True ⌝ ⦄
    rust_primitives.slice.array_from_fn N inst c
    ⦃ ⇓ a => ⌜ ∀ i : Nat, (hi : i < N.val) →
                ⦃ ⌜ True ⌝ ⦄ inst.call_mut c ⟨BitVec.ofNat _ i⟩
                          ⦃ ⇓ r => ⌜ r.1 = a.val[i]'(by have := a.property; omega) ⌝ ⦄ ⌝ ⦄ := by
  -- We enrich `hpure` by universally quantifying over the `call_mut` argument instead of fixing
  -- it to `c`:
  have hpure' : ∀ k, k < N.val → ∀ c', c' = c →
      ⦃ ⌜ True ⌝ ⦄ inst.call_mut c' ⟨BitVec.ofNat _ k⟩ ⦃ ⇓ r => ⌜ r.2 = c ⌝ ⦄ :=
    fun k hk c' hc' => by subst hc'; exact hpure k hk
  mvcgen [rust_primitives.slice.array_from_fn, hpure']
  · -- then-branch
    rename_i r hlen hconj
    obtain ⟨_, _, hpost⟩ := hconj
    intro i hi
    have hp := hpost i hi
    mvcgen [hp]
    grind
  · -- else-branch is impossible: the worker's length equals `N`.
    grind

/-! ## `slice_contains` -/

@[spec]
theorem rust_primitives.slice.slice_contains_go_spec {T : Type}
    (inst : core.cmp.PartialEq T T) (x : T) (l : List T)
    (hpure : ∀ y ∈ l, ⦃ ⌜ True ⌝ ⦄ inst.eq y x ⦃ ⇓ _ => ⌜ True ⌝ ⦄) :
    ⦃ ⌜ True ⌝ ⦄
    rust_primitives.slice.slice_contains_go inst x l
    ⦃ ⇓ r => ⌜ (r = true → ∃ y ∈ l, ⦃ ⌜ True ⌝ ⦄ inst.eq y x ⦃ ⇓ b => ⌜ b = true ⌝ ⦄) ∧
              (r = false → ∀ y ∈ l, ⦃ ⌜ True ⌝ ⦄ inst.eq y x ⦃ ⇓ b => ⌜ b = false ⌝ ⦄) ⌝ ⦄ := by
  induction l with
  | nil =>
    mvcgen [rust_primitives.slice.slice_contains_go]
    simp
  | cons y ys ih =>
    -- Enrich `hpure` mvcgen's VC still contains the fact that the value came from `inst.eq`:
    have hpure' := triple_with_self (hpure y (by simp))
    have ih := ih (fun z hz => hpure z (by simp [hz]))
    mvcgen [rust_primitives.slice.slice_contains_go, hpure', ih]
      <;> grind

@[spec]
theorem rust_primitives.slice.slice_contains_spec {T : Type}
    (inst : core.cmp.PartialEq T T) (s : Slice T) (x : T)
    (hok : ∀ y ∈ s.val, ⦃ ⌜ True ⌝ ⦄ inst.eq y x ⦃ ⇓ _ => ⌜ True ⌝ ⦄) :
    ⦃ ⌜ True ⌝ ⦄
    rust_primitives.slice.slice_contains inst s x
    ⦃ ⇓ r => ⌜ (r = true → ∃ y ∈ s.val, ⦃ ⌜ True ⌝ ⦄ inst.eq y x ⦃ ⇓ b => ⌜ b = true ⌝ ⦄) ∧
              (r = false → ∀ y ∈ s.val, ⦃ ⌜ True ⌝ ⦄ inst.eq y x ⦃ ⇓ b => ⌜ b = false ⌝ ⦄) ⌝ ⦄ :=
  rust_primitives.slice.slice_contains_go_spec inst x s.val hok

/-! ## `array_map` -/

@[spec]
theorem rust_primitives.slice.array_map_go_spec {T U F : Type}
    (inst : core.ops.function.Fn F T U) (f : F) (l : List T)
    (hpure : ∀ x ∈ l, ⦃ ⌜ True ⌝ ⦄ inst.call f x ⦃ ⇓ _ => ⌜ True ⌝ ⦄) :
    ⦃ ⌜ True ⌝ ⦄
    rust_primitives.slice.array_map_go inst f l
    ⦃ ⇓ rl => ⌜ ∃ _ : rl.length = l.length,
                ∀ i, (hi : i < l.length) →
                  ⦃ ⌜ True ⌝ ⦄ inst.call f l[i] ⦃ ⇓ y => ⌜ y = rl[i] ⌝ ⦄ ⌝ ⦄ := by
  induction l with
  | nil =>
    mvcgen [rust_primitives.slice.array_map_go]
    exact ⟨rfl, fun i hi => absurd hi (by simp)⟩
  | cons x xs ih =>
    -- Enrich `hpure` mvcgen's VC still contains the fact that the value came from `inst.call`:
    have hpure' := triple_with_self (hpure x (by simp))
    have ih := ih (fun z hz => hpure z (by simp [hz]))
    mvcgen [rust_primitives.slice.array_map_go, hpure', ih]
    obtain ⟨_, _⟩ := ‹∃ _, _›
    refine ⟨by grind, ?_⟩
    intro i hi
    rcases i with _ | i <;> simp_all

/-- The fold underlying `array_map`, equationally: with a closure that leaves its
state alone the fold succeeds and builds `acc` followed by the pointwise images. -/
private theorem foldlM_map_ok {T U F : Type}
    (inst : core.ops.function.FnMut F T U) (f : F) :
    ∀ (l : List T) (acc : List U),
    (∀ x ∈ l, ∃ y, inst.call_mut f x = ok y ∧ y.2 = f) →
    ∃ res : List U × F,
      l.foldlM (fun (s : List U × F) (x : T) => do
          let r ← inst.call_mut s.2 x
          ok (s.1 ++ [r.1], r.2)) (acc, f) = ok res ∧
      res.2 = f ∧ res.1.length = acc.length + l.length ∧
      (∀ j, j < acc.length → res.1[j]? = acc[j]?) ∧
      (∀ i, (hi : i < l.length) →
        ∃ y, inst.call_mut f l[i] = ok y ∧ res.1[acc.length + i]? = some y.1) := by
  intro l
  induction l with
  | nil =>
    intro acc _
    exact ⟨(acc, f), rfl, rfl, by simp, fun j _ => rfl, fun i hi => absurd hi (by simp)⟩
  | cons x xs ih =>
    intro acc hpure
    obtain ⟨y, hy, hyf⟩ := hpure x (by simp)
    obtain ⟨v, f'⟩ := y
    simp only at hyf
    rw [hyf] at hy
    obtain ⟨res, hres, hres2, hlen, hpre, hpt⟩ :=
      ih (acc ++ [v]) (fun z hz => hpure z (by simp [hz]))
    refine ⟨res, ?_, hres2, ?_, ?_, ?_⟩
    · simp only [List.foldlM_cons, hy, bind_tc_eq, bind_ok]
      exact hres
    · simp at hlen ⊢
      omega
    · intro j hj
      rw [hpre j (by simp; omega), List.getElem?_append_left hj]
    · intro i hi
      rcases i with _ | i
      · refine ⟨(v, f), by simpa using hy, ?_⟩
        have h := hpre acc.length (by simp)
        simpa using h
      · obtain ⟨y', hy', hidx⟩ := hpt i (by simpa using hi)
        refine ⟨y', by simpa using hy', ?_⟩
        have hshift : acc.length + (i + 1) = (acc ++ [v]).length + i := by simp; omega
        rw [hshift]; exact hidx

@[spec]
theorem rust_primitives.slice.array_map_spec {T U F : Type} {N : Std.Usize}
    (inst : core.ops.function.FnMut F T U) (a : Array T N) (f : F)
    (hpure : ∀ x ∈ a.val, ⦃ ⌜ True ⌝ ⦄ inst.call_mut f x ⦃ ⇓ r => ⌜ r.2 = f ⌝ ⦄) :
    ⦃ ⌜ True ⌝ ⦄
    rust_primitives.slice.array_map inst a f
    ⦃ ⇓ b => ⌜ ∀ i, (hi : i < N.val) →
                ⦃ ⌜ True ⌝ ⦄ inst.call_mut f (a.val[i]'(by have := a.property; omega))
                  ⦃ ⇓ y => ⌜ y.1 = b.val[i]'(by have := b.property; omega) ⌝ ⦄ ⌝ ⦄ := by
  have ha := a.property
  have hpure' : ∀ x ∈ a.val, ∃ y, inst.call_mut f x = ok y ∧ y.2 = f :=
    fun x hx => Aeneas.Std.triple_iff_exists_ok.1 (hpure x hx)
  obtain ⟨res, hres, _, hlen, _, hpt⟩ := foldlM_map_ok inst f a.val [] hpure'
  simp only [List.length_nil, Nat.zero_add] at hlen hpt
  rw [Aeneas.Std.triple_iff_exists_ok]
  refine ⟨Array.from res.1 (by omega), ?_, ?_⟩
  · simp only [rust_primitives.slice.array_map]
    split <;> rename_i h <;> erw [hres] at h <;> simp at h
    subst h; rfl
  · intro i hi
    obtain ⟨y, hy, hidx⟩ := hpt i (by omega)
    rw [Aeneas.Std.triple_iff_exists_ok]
    refine ⟨y, hy, ?_⟩
    have hlt : i < res.1.length := by omega
    rw [List.getElem?_eq_getElem hlt] at hidx
    simpa using (Option.some.inj hidx).symm

end CoreModels
