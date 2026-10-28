import Barrett.Extraction
import Barrett.Verification.MissingSpecs
import Hax
open CoreModels Aeneas
open Aeneas.Std hiding namespace core alloc
open RustM ControlFlow Error
open Std.Do

set_option mvcgen.warning false
set_option hax_mvcgen.warnings false

namespace barrett

attribute [local spec] barrett_reduce core.I64.Insts.CoreConvertFromI32.from

attribute [local grind =]
  Int32.toInt_toInt64 Int64.ofBitVec_int32ToBitVec
  Int64.toInt_inj Int32.toInt_inj
  Int64.le_iff_toInt_le Int32.lt_iff_toInt_lt
  Int.shiftRight_eq_div_pow

local grind_pattern Int32.toInt64_ofNat => (@OfNat.ofNat Int32 n _).toInt64

open Std

/-- Correctness of the signed Barrett reduction -/
theorem barrett_reduce.spec.proof (value : Std.I32) : barrett_reduce.spec value := by
  unfold spec
  hax_mvcgen
    <;> try simp [BARRETT_MULTIPLIER, BARRETT_R, BARRETT_SHIFT, FIELD_MODULUS] at *
  all_goals try grind

end barrett
