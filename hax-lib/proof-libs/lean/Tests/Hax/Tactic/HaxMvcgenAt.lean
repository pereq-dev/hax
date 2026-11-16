module
public import Hax

@[expose] public section

/-! # Tests for `Hax.Tactic.HaxMvcgenAt` -/

set_option mvcgen.warning false
-- Metavariable indices are not stable under unrelated edits, so print them as `?_`.
set_option pp.mvars false

open CoreModels Aeneas
open Aeneas.Std hiding namespace core alloc
open RustM
open Std.Do
open Std.Tactic

namespace Hax.Tests

/-! ## Basic functionality -/

private def wrapped (x : U32) : RustM U32 := ok x

@[local spec]
private theorem wrapped_spec (x : U32) :
    ⦃ ⌜ True ⌝ ⦄ wrapped x ⦃ ⇓ r => ⌜ r = x ⌝ ⦄ := by
  simp [wrapped, Triple, WP.wp, PredTrans.apply]

example (h : ⦃ ⌜ True ⌝ ⦄ (do let x ← wrapped 1#u32; ok x) ⦃ ⇓ r => ⌜ r.val = 1 ⌝ ⦄) :
    (1#u32 : U32).val = 1 := by
  hax_mvcgen at h
  simp

/-! ## Programs without a successful execution path -/

private def alwaysFails (_ : U32) : RustM U32 := fail .panic

-- A total-correctness triple about a program that never succeeds is contradictory, so
-- `hax_mvcgen at` closes the goal.
#guard_msgs in
example (h : ⦃ ⌜ True ⌝ ⦄ alwaysFails 1#u32 ⦃ ⇓ r => ⌜ r.val = 0 ⌝ ⦄) : 1 = 2 := by
  hax_mvcgen [alwaysFails] at h

#guard_msgs in
example (h : ⦃ ⌜ True ⌝ ⦄ alwaysFails 1#u32 ⦃ ⇓ r => ⌜ r.val = 0 ⌝ ⦄) : 1 = 2 := by
  hax_mvcgen [alwaysFails]

-- Under binders, the hypothesis is replaced by one stating `∀ (p : Prop), p`.
/--
trace: h : ∀ (x : U32) (p : Prop), p
⊢ 1 = 2
-/
#guard_msgs in
example (h : ∀ x, ⦃ ⌜ True ⌝ ⦄ alwaysFails x ⦃ ⇓ r => ⌜ r.val = 0 ⌝ ⦄) : 1 = 2 := by
  hax_mvcgen [alwaysFails] at h
  trace_state
  exact h 0#u32 _

end Hax.Tests


/-! ## Reporting missing `@[spec]` lemmas -/

private opaque unspecced : U32 → RustM U32

/--
error: Tactic `hax_mvcgen` failed: Failed to process hypothesis h. Usually this error is due to missing specs for functions contained in the program. This is likely because unspecced is missing a @[spec] lemma.

Remaining goal:
(wp⟦unspecced 1#u32⟧ (fun a => wp⟦ok a⟧ (PostCond.mayThrow fun r => { down := ↑r = 1 → ?_ }), ExceptConds.true)).down

h :
  ⦃⌜True⌝⦄ do
    let x ← unspecced 1#u32
    ok x ⦃PostCond.noThrow fun r => ⌜↑r = 1⌝⦄
⊢ True
-/
#guard_msgs in
example (h : ⦃ ⌜ True ⌝ ⦄ (do let x ← unspecced 1#u32; ok x) ⦃ ⇓ r => ⌜ r.val = 1 ⌝ ⦄) :
    True := by
  hax_mvcgen at h
