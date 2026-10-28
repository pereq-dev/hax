-- Missing core model specs, to upstream
import Aeneas
import Barrett.Extraction
open Aeneas Aeneas.Std RustM ControlFlow Error
open Std.Do Aeneas barrett CoreModels
set_option mvcgen.warning false

@[spec]
-- `MIN % -1` overflows just like `MIN / -1`, so the model panics there and the
-- spec needs to rule it out.
theorem I32_rem_euclid_spec (x y : Std.I32) (hy : y.val ≠ 0)
    (hov : ¬(x.val = I32.min ∧ y.val = -1)) :
    ⦃ ⌜ True ⌝ ⦄
    core.num.I32.rem_euclid x y
    ⦃ ⇓ r => ⌜ r.val = x.val % y.val ⌝ ⦄ := by
  hax_mvcgen [core.num.I32.rem_euclid, rust_primitives.arithmetic.rem_euclid_i32, irem_euclid]
  all_goals grind [Int.emod_nonneg x.val hy, Int.emod_lt_abs x.val hy, IScalar.val]
