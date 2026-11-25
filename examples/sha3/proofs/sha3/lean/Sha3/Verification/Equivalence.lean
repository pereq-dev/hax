import Sha3.Extraction
import Hax

namespace sha3

open Std.Do Aeneas implementation
open Aeneas.Std hiding namespace core alloc
open CoreModels
set_option mvcgen.warning false

abbrev inst := U64.Insts.Sha3ImplementationKeccakItem1

/-- `_ensures_theta` clones the state: the clone of a state of `U64`s is that state. -/
@[spec]
theorem clone_spec (st : KeccakState Std.U64 1#usize) :
    ⦃ ⌜ True ⌝ ⦄
    KeccakState.Insts.CoreCloneClone.clone core.U64.Insts.CoreCloneClone inst st
    ⦃ ⇓ r => ⌜ r = st ⌝ ⦄ := by
  hax_mvcgen [KeccakState.Insts.CoreCloneClone.clone]
  subst_vars
  rfl

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
    _requires_iota at *
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
  _ensures_chi _ensures_iota _ensures_pi _ensures_round _ensures_theta

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
