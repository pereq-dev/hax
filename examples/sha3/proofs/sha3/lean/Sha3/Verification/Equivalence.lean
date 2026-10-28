import Sha3.Extraction
import Hax

namespace sha3

open Std.Do Aeneas implementation
open Aeneas.Std hiding namespace core alloc
open CoreModels
set_option mvcgen.warning false

abbrev inst := U64.Insts.Sha3ImplementationKeccakItem1

/-! ## Helper lemmas -/

open Std.Do Std core in
private theorem allM_zip_beq :
    ∀ (xs ys : List U64), xs.length = ys.length →
    List.allM (fun ((x, y) : U64 × U64) => (RustM.ok (x == y) : RustM Bool))
      (List.zip xs ys) = RustM.ok (xs == ys) := by
  intro xs
  induction xs with
  | nil =>
    intro ys h
    cases ys with
    | nil => rfl
    | cons => simp at h
  | cons x xs ih =>
    intro ys h
    cases ys with
    | nil => simp at h
    | cons y ys =>
      simp only [List.length_cons] at h
      have h' : xs.length = ys.length := by omega
      simp only [List.zip_cons_cons]
      simp only [List.allM, bind_tc_ok, ih ys h']
      cases hxy : (x == y)
      · simp [hxy, pure]
      · simp [hxy]

private theorem array_index_U64_eq {N : Usize} (a : Array U64 N) (i : Usize)
    (h : i.val < N.val) :
    rust_primitives.slice.array_index a i = .ok a.val[i.val]! := by
  have h' : i.val < a.val.length := by rw [a.property]; exact h
  have hSpec := Aeneas.Std.WP.spec_of_partialSpec (Slice.index_usize_spec (Array.to_slice a) i)
    (by rintro ⟨⟩ <;> simp_all) (by simp)
  obtain ⟨y, hy, _, hyVal⟩ := Aeneas.Std.WP.spec_imp_exists hSpec
  simp only [rust_primitives.slice.array_index, hy, hyVal, Array.val_to_slice,
    getElem!_pos a.val i.val h']

private theorem list_eq_of_pointwise {α} [Inhabited α] [DecidableEq α] (xs ys : List α)
    (hLen : xs.length = ys.length)
    (h : ∀ k, k < xs.length → xs[k]! = ys[k]!) :
    xs = ys := by
  induction xs generalizing ys with
  | nil =>
    cases ys with
    | nil => rfl
    | cons _ _ => simp at hLen
  | cons x xs ih =>
    cases ys with
    | nil => simp at hLen
    | cons y ys =>
      have h0 := h 0 (by simp)
      simp only [List.getElem!_cons_zero] at h0
      have hLen' : xs.length = ys.length := by simp at hLen; omega
      have ih' := ih ys hLen' (by
        intro k hk
        have := h (k + 1) (by simp; omega)
        simp only [List.getElem!_cons_succ] at this
        exact this)
      simp [h0, ih']

private theorem add_one_usize (j : Usize) (h : j.val + 1 ≤ Usize.max) :
    ∃ k : Usize, (j + 1#usize : RustM Usize) = .ok k ∧ k.val = j.val + 1 := by
  have hSpec : (j + 1#usize : RustM Usize) ⦃ k => k.val = j.val + 1#usize.val ⦄ := by
    apply UScalar.add_spec.step_spec
    scalar_tac
  obtain ⟨k, hk, hkVal⟩ := Aeneas.Std.WP.spec_imp_exists hSpec
  refine ⟨k, hk, ?_⟩
  rw [hkVal]; rfl

private theorem eq_loop_correct {N : Usize} (a0 a1 : Array U64 N) (i : Usize)
    (hi : i.val ≤ N.val)
    (hPrev : ∀ k, k < i.val → a0.val[k]! = a1.val[k]!) :
    Aeneas.Std.WP.spec
      (core.Array.Insts.CoreCmpPartialEqArray.eq_loop
        core.U64.Insts.CoreCmpPartialEqU64 a0 a1 i)
      (fun r => r = (a0.val == a1.val)) := by
  unfold core.Array.Insts.CoreCmpPartialEqArray.eq_loop
  refine Aeneas.Std.loop.spec_decr_nat
    (measure := fun (j : Usize) => N.val - j.val)
    (inv := fun (j : Usize) => j.val ≤ N.val ∧ ∀ k, k < j.val → a0.val[k]! = a1.val[k]!)
    (post := fun r => r = (a0.val == a1.val))
    (hBody := ?_) (hInv := ⟨hi, hPrev⟩)
  rintro j ⟨hjN, hPrevJ⟩
  unfold core.Array.Insts.CoreCmpPartialEqArray.eq_loop.body
  by_cases hjLt : j.val < N.val
  · -- j < N case
    have hjLt' : (j < N : Prop) := hjLt
    rw [ite_eq_left hjLt']
    rw [array_index_U64_eq a0 j hjLt, bind_ok]
    show Aeneas.Std.WP.spec (do
      let t1 ← rust_primitives.slice.array_index a1 j
      let b ← core.U64.Insts.CoreCmpPartialEqU64.eq a0.val[j.val]! t1
      if b then let i1 ← j + 1#usize; RustM.ok (ControlFlow.cont i1)
      else RustM.ok (ControlFlow.done false)) _
    rw [array_index_U64_eq a1 j hjLt, bind_ok]
    show Aeneas.Std.WP.spec (do
      let b ← core.U64.Insts.CoreCmpPartialEqU64.eq a0.val[j.val]! a1.val[j.val]!
      if b then let i1 ← j + 1#usize; RustM.ok (ControlFlow.cont i1)
      else RustM.ok (ControlFlow.done false)) _
    have hcmp : core.U64.Insts.CoreCmpPartialEqU64.eq a0.val[j.val]! a1.val[j.val]!
        = .ok (a0.val[j.val]! == a1.val[j.val]!) := rfl
    rw [hcmp, bind_ok]
    show Aeneas.Std.WP.spec
      (if (a0.val[j.val]! == a1.val[j.val]!)
       then (do let i1 ← j + 1#usize; RustM.ok (ControlFlow.cont i1))
       else RustM.ok (ControlFlow.done false)) _
    have hj1Bound : j.val + 1 ≤ Usize.max := by
      have hN := N.hBounds
      have : N.val ≤ Usize.max := by scalar_tac
      omega
    by_cases hbeq : a0.val[j.val]! = a1.val[j.val]!
    · -- equal
      have hbeq' : (a0.val[j.val]! == a1.val[j.val]!) = true := beq_iff_eq.mpr hbeq
      rw [ite_eq_left hbeq']
      obtain ⟨jp1, hjp1Eq, hjp1Val⟩ := add_one_usize j hj1Bound
      rw [hjp1Eq, bind_ok, Aeneas.Std.WP.spec_ok]
      refine ⟨⟨?_, ?_⟩, ?_⟩
      · rw [hjp1Val]; omega
      · intro k hk
        rw [hjp1Val] at hk
        by_cases hkj : k < j.val
        · exact hPrevJ k hkj
        · have : k = j.val := by omega
          subst this; exact hbeq
      · show N.val - jp1.val < N.val - j.val
        rw [hjp1Val]; omega
    · -- not equal
      have hbeq' : (a0.val[j.val]! == a1.val[j.val]!) = false := beq_eq_false_iff_ne.mpr hbeq
      rw [ite_eq_right (by rw [hbeq']; simp), Aeneas.Std.WP.spec_ok]
      show (false : Bool) = (a0.val == a1.val)
      have hne : a0.val ≠ a1.val := fun hEq => hbeq (by rw [hEq])
      simp [hne]
  · -- j ≥ N: j = N
    have hjLt' : ¬ (j < N : Prop) := hjLt
    rw [ite_eq_right hjLt']
    simp only [Aeneas.Std.WP.spec_ok]
    have hjEq : j.val = N.val := by omega
    have hLen0 : a0.val.length = N.val := a0.property
    have hLen1 : a1.val.length = N.val := a1.property
    have hListEq : a0.val = a1.val := by
      apply list_eq_of_pointwise
      · rw [hLen0, hLen1]
      · intro k hk
        rw [hLen0] at hk
        apply hPrevJ; rw [hjEq]; exact hk
    simp [hListEq]

open Std.Do in
@[spec]
theorem array.equality.PartialEqArray.eq_spec {N : Usize} (a0 : Array U64 N) (a1 : Array U64 N)
    (h : (Q.1 (a0.val == a1.val)).down) :
    ⦃ ⌜ True ⌝ ⦄
    core.Array.Insts.CoreCmpPartialEqArray.eq core.U64.Insts.CoreCmpPartialEqU64 a0 a1
    ⦃ Q ⦄ := by
  have hSpec : Aeneas.Std.WP.spec
      (core.Array.Insts.CoreCmpPartialEqArray.eq
        core.U64.Insts.CoreCmpPartialEqU64 a0 a1)
      (fun r => r = (a0.val == a1.val)) := by
    unfold core.Array.Insts.CoreCmpPartialEqArray.eq
    apply eq_loop_correct a0 a1 0#usize (by simp) (fun k hk => by simp at hk)
  obtain ⟨b, hx, hb⟩ := Aeneas.Std.WP.spec_imp_exists hSpec
  rw [hx]
  apply RustM.ok_spec
  rw [hb]; exact h

attribute [local spec] uncurry

/-- `_ensures_theta` clones the state: the clone of a state of `U64`s is that state. -/
private theorem array_map_id {T F : Type} {N : Usize}
    (inst : core.ops.function.FnMut F T T) (a : Std.Array T N) (f : F)
    (hid : ∀ x, inst.call_mut f x = .ok (x, f)) :
    rust_primitives.slice.array_map inst a f = .ok a := by
  have hpure : ∀ x ∈ a.val, ⦃ ⌜ True ⌝ ⦄ inst.call_mut f x ⦃ ⇓ r => ⌜ r.2 = f ⌝ ⦄ :=
    fun x _ => Aeneas.Std.triple_iff_exists_ok.2 ⟨(x, f), hid x, rfl⟩
  obtain ⟨b, hb, hpt⟩ :=
    Aeneas.Std.triple_iff_exists_ok.1 (rust_primitives.slice.array_map_spec inst a f hpure)
  have ha := a.property
  have hbp := b.property
  have : b = a := by
    apply Aeneas.Std.Array.ext
    apply List.ext_getElem (by omega)
    intro i _ _
    obtain ⟨y, hy, hyi⟩ := Aeneas.Std.triple_iff_exists_ok.1 (hpt i (by omega))
    rw [hid] at hy
    simp only [RustM.ok.injEq] at hy
    subst hy
    exact hyi.symm
  rw [hb, this]

@[spec]
theorem clone_spec (st : KeccakState Std.U64 1#usize) :
    ⦃ ⌜ True ⌝ ⦄
    KeccakState.Insts.CoreCloneClone.clone core.U64.Insts.CoreCloneClone inst st
    ⦃ ⇓ r => ⌜ r = st ⌝ ⦄ := by
  have h : KeccakState.Insts.CoreCloneClone.clone core.U64.Insts.CoreCloneClone inst st
      = .ok st := by
    unfold KeccakState.Insts.CoreCloneClone.clone core.Array.Insts.CoreCloneClone.clone
      core.array.Array.map
    rw [array_map_id]
    · simp
    · intro x
      simp [core.array.CloneArray.clone.closure.Insts.CoreOpsFunctionFnMutTupleTT.call_mut,
        core.U64.Insts.CoreCloneClone.clone]
  rw [h]
  apply RustM.ok_spec
  simp [PostCond.ok]

/-
We prove two examples of equivalences between implementation and specification:

## Part 1: Equivalence of `iota`

Implementation and specification of iota yield the same state array, provided that the argument
`i` is small enough. (Otherwise both would access the round constants array out of bounds.)
-/

@[spec]
theorem iota_spec
  (st : KeccakState Std.U64 1#usize) (i : Std.Usize)
  (h : (_requires_iota st i).holds):
    ⦃ ⌜ True ⌝ ⦄
    KeccakState.iota inst st i
    ⦃ ⇓ r => ⌜ (_ensures_iota st i r).holds ⌝ ⦄ := by
  -- Unfold all definitions
  unfold KeccakState.iota KeccakState.Insts.CoreOpsIndexIndexPairUsizeUsizeT.index get_ij
    _requires_iota core.slice.Slice.len at *
  dsimp only [U64.Insts.Sha3ImplementationKeccakItem1.xor_constant, _veorq_n_u64, KeccakState.set, set_ij, _ensures_iota,
    specification.iota]
  -- Call `hax_mvcgen`. This tactic will run `mvcgen` on triples in both hypotheses and in goals
  hax_mvcgen
  · have hr : ROUNDCONSTANTS = specification.ROUND_CONSTANTS := by
      unfold ROUNDCONSTANTS specification.ROUND_CONSTANTS; rfl
    grind
  all_goals grind

/-
## Part 2: Equivalence of `round`

To demonstrate how we can incrementally prove equivalence of functions, we skip verification
of `theta`, `rho`, `pi`, and `chi` and show how we can prove equivalence of the `round`
function without unfolding the underlying greek-letter functions.
-/

/- ### Skip proof of `theta`, `rho`, `pi`, and `chi`: -/

@[spec]
theorem theta_spec
  (st : KeccakState Std.U64 1#usize)
  (h : (_requires_theta st).holds) :
    ⦃ ⌜ True ⌝ ⦄
    KeccakState.theta inst st
    ⦃ ⇓ r => ⌜ (_ensures_theta st r.2 r.1).holds ⌝ ⦄ := by
  sorry

@[spec]
theorem pi_spec
  (st : KeccakState Std.U64 1#usize)
  (h : (_requires_pi st).holds) :
    ⦃ ⌜ True ⌝ ⦄
    KeccakState.pi inst st
    ⦃ ⇓ r => ⌜ (_ensures_pi st r).holds ⌝ ⦄ := by
  sorry

@[spec]
theorem chi_spec
  (st : KeccakState Std.U64 1#usize)
  (h : (_requires_chi st).holds) :
    ⦃ ⌜ True ⌝ ⦄
    KeccakState.chi inst st
    ⦃ ⇓ r => ⌜ (_ensures_chi st r).holds ⌝ ⦄ := by
  sorry

theorem rho_spec
  (st : KeccakState Std.U64 1#usize) :
    ⦃ ⌜ True ⌝ ⦄
    KeccakState.rho inst st t
    ⦃ ⇓ r => ⌜ True ⌝ ⦄ := by
  sorry

/- ### Panic-freedom of the specification

We will also ignore the proofs that the specifications of `theta`, `rho`, `pi`,
`chi`, and `iota` do not panic:
-/

theorem specification_theta_spec (st) :
    ⦃ ⌜ True ⌝ ⦄
    specification.theta st
    ⦃ ⇓ r => ⌜ True ⌝ ⦄ := by
  sorry

theorem specification_rho_spec (st) :
    ⦃ ⌜ True ⌝ ⦄
    specification.rho st
    ⦃ ⇓ r => ⌜ True ⌝ ⦄ := by
  sorry

theorem specification_pi_spec (st) :
    ⦃ ⌜ True ⌝ ⦄
    specification.pi st
    ⦃ ⇓ r => ⌜ True ⌝ ⦄ := by
  sorry

theorem specification_chi_spec (st) :
    ⦃ ⌜ True ⌝ ⦄
    specification.chi st
    ⦃ ⇓ r => ⌜ True ⌝ ⦄ := by
  sorry

theorem specification_iota_spec (st i) :
    ⦃ ⌜ True ⌝ ⦄
    specification.iota st i
    ⦃ ⇓ r => ⌜ True ⌝ ⦄ := by
  sorry

/- ### Encapsulation specifications

When running `mvcgen` on a program, we need spec lemmas for all contained functions. For the
specification functions and for the implementation `rho`, we do not want to provide actual
pre- and postconditions. We want to abstract over whatever these functions are doing. For that
purpose, we can use `core.RustM.toPure_spec` to encapsulate their definitions and keep them
opaque to mvcgen. It produces spec lemmas such as:
```
⦃⌜True⌝⦄ rho st t ⦃ ⇓r => ⌜r = (rho st t).toPure⌝ ⦄
```
In essence, this states that `rho` does whatever `rho` does, and it prevents `mvcgen` from getting
stuck at `rho` while putting some useful information into the verification conditions.

-/

-- Some `Inhabited` instances are currently required for `core.RustM.toPure_spec`
instance : Inhabited (KeccakState Std.U64 1#usize) := ⟨{st := default}⟩

section
-- These aliases take their statement from `toPure_spec`; a `theorem` would have to restate it.
set_option linter.defProp false

@[spec]
def rho_spec' {st} {t} := Std.RustM.toPure_spec (KeccakState.rho inst st t) (rho_spec st)

@[spec]
def specification_theta_spec' {st} := Std.RustM.toPure_spec (specification.theta st) (specification_theta_spec st)

@[spec]
def specification_rho_spec' {st} := Std.RustM.toPure_spec (specification.rho st) (specification_rho_spec st)

@[spec]
def specification_pi_spec' {st} := Std.RustM.toPure_spec (specification.pi st) (specification_pi_spec st)

@[spec]
def specification_chi_spec' {st} := Std.RustM.toPure_spec (specification.chi st) (specification_chi_spec st)

@[spec]
def specification_iota_spec' {st i} := Std.RustM.toPure_spec (specification.iota st i) (specification_iota_spec st i)

end

/- ### Equivalence proof

Now we can prove equivalence for `round`:
-/

attribute [spec]
  _requires_chi _requires_iota _requires_pi _requires_round _requires_theta
  _ensures_chi _ensures_iota _ensures_pi _ensures_round _ensures_theta core.slice.Slice.len

@[spec]
theorem round_spec
  (st : KeccakState Std.U64 1#usize) (i : Std.Usize)
  (h : (_requires_round st i).holds) :
    ⦃ ⌜ True ⌝ ⦄
    KeccakState.round inst st i
    ⦃ ⇓ r => ⌜ (_ensures_round st i r).holds ⌝ ⦄ := by
  unfold KeccakState.round at *
  hax_mvcgen
  simp_all [← Aeneas.Std.Array.eq_iff]

end sha3
