module
public import CoreModels.MissingAeneas
public import CoreModels.Spec.RustPrimitives.Slice

@[expose] public section

namespace CoreModels

open Aeneas
open Aeneas.Std hiding namespace core alloc
open Std.Do WP RustM ScalarElab

set_option mvcgen.warning false
set_option linter.deprecated.syntax false

/- The proof below reads each element through the closure passed to `array::from_fn`. -/
attribute [local spec]
  core.convert.TryFromArrayShared0SliceTryFromSliceError.try_from.closure.Insts.CoreOpsFunctionFnMutTupleUsizeT.call_mut

/-- `<[T; N]>::try_from(s)` succeeds exactly when `s` has length `N`. -/
@[spec]
theorem core.Array.Insts.CoreConvertTryFromShared0SliceTryFromSliceError.try_from_spec
    {T : Type} {N : Std.Usize} (cpy : core.marker.Copy T)
    (s : Slice T) :
    ⦃ ⌜ True ⌝ ⦄
    core.Array.Insts.CoreConvertTryFromShared0SliceTryFromSliceError.try_from
      N cpy s
    ⦃ ⇓ r => ⌜ r = if hlen : s.val.length = N.val
                   then core.result.Result.Ok (Std.Array.make N s.val (by simp [hlen]))
                   else core.result.Result.Err () ⌝ ⦄ := by
  mvcgen [core.Array.Insts.CoreConvertTryFromShared0SliceTryFromSliceError.try_from,
    core.slice.Slice.len]
  · grind [UScalar.val]
  · rename_i _ a hapost
    have hlen : s.val.length = N.val := by grind [Slice.len_val, UScalar.eq_equiv]
    rw [dite_eq_left hlen]
    congr
    apply Aeneas.Std.Array.ext
    apply List.ext_getElem
    · rw [a.property, Array.make_val]; exact hlen.symm
    · intro i h1 h2
      apply triple_in_hypothesis _ (hapost i (a.property ▸ h1))
      mvcgen; grind [UScalar.val, Array.make]
  · grind [Slice.len_val, UScalar.eq_equiv]

/- The blanket conversion impls forward to another conversion, or return their argument. -/
attribute [spec]
  core.convert.From.Blanket.from
  core.convert.Into.Blanket.into
  core.convert.TryFromUTInfallible.Blanket.try_from
  core.convert.TryInto.Blanket.try_into

/-! ## Lossless integer conversions -/

/-- Casting an unsigned integer to a wider signed type keeps its value. -/
private theorem UScalar.hcast_val_of_lt {src : UScalarTy} (tgt : IScalarTy) (x : UScalar src)
    (h : src.numBits < tgt.numBits) : (UScalar.hcast tgt x).val = x.val := by
  have hx : (x.val : Int) < 2 ^ src.numBits := by exact_mod_cast x.hBounds
  have hle : (2 : Int) ^ src.numBits ≤ 2 ^ (tgt.numBits - 1) :=
    pow_le_pow_right₀ (by decide) (by omega)
  have hpos : (0 : Int) ≤ 2 ^ (tgt.numBits - 1) := by positivity
  have htgt : tgt.numBits = tgt.numBits - 1 + 1 := by omega
  rw [UScalar.hcast_val_eq, htgt]
  apply Arith.Int.bmod_pow2_eq_of_inBounds <;> omega

/-- `from_spec kind tgt src` states that `tgt::from(x : src)` keeps the value of `x`, where
`kind` says which cast the conversion uses. -/
local syntax "from_spec " ident ident ident : command

open Lean in
macro_rules
  | `(from_spec $kind $tgt $src) => do
    let inst := Name.mkStr (Name.mkStr (Name.mkStr `core tgt.getId.toString) "Insts")
      s!"CoreConvertFrom{src.getId.toString}"
    let f := mkIdent (Name.mkStr inst "from")
    let thm := mkIdent (Name.mkStr inst "from_spec")
    let castLemma ← match kind.getId.toString with
      | "unsigned" => `(term| UScalar.cast_val_mod_pow_greater_numBits_eq)
      | "signed" => `(term| IScalar.val_mod_pow_greater_numBits)
      | _ => `(term| UScalar.hcast_val_of_lt)
    `(@[spec] theorem $thm (x : $src) :
          ⦃ ⌜ True ⌝ ⦄ $f x ⦃ ⇓ r => ⌜ r.val = x.val ⌝ ⦄ := by
        mvcgen [$f:ident] <;> exact $castLemma _ x (by
          obtain hb | hb := System.Platform.numBits_eq <;>
            simp [UScalarTy.numBits, IScalarTy.numBits, hb]))

from_spec unsigned U16 U8
from_spec unsigned U32 U8
from_spec unsigned U32 U16
from_spec unsigned U64 U8
from_spec unsigned U64 U16
from_spec unsigned U64 U32
from_spec unsigned U128 U8
from_spec unsigned U128 U16
from_spec unsigned U128 U32
from_spec unsigned U128 U64
from_spec unsigned Usize U8
from_spec unsigned Usize U16
from_spec signed I16 I8
from_spec signed I32 I8
from_spec signed I32 I16
from_spec signed I64 I8
from_spec signed I64 I16
from_spec signed I64 I32
from_spec signed I128 I8
from_spec signed I128 I16
from_spec signed I128 I32
from_spec signed I128 I64
from_spec signed Isize I8
from_spec signed Isize I16
from_spec mixed I16 U8
from_spec mixed I32 U8
from_spec mixed I64 U8
from_spec mixed I128 U8
from_spec mixed Isize U8
from_spec mixed I32 U16
from_spec mixed I64 U16
from_spec mixed I128 U16
from_spec mixed I64 U32
from_spec mixed I128 U32
from_spec mixed I128 U64

/-! ## Conversions from `bool` -/

uscalar @[spec] theorem core.«%S».Insts.CoreConvertFromBool.from_spec (b : Bool) :
    ⦃ ⌜ True ⌝ ⦄
    core.«%S».Insts.CoreConvertFromBool.from b
    ⦃ ⇓ r => ⌜ r.val = b.toNat ⌝ ⦄ := by
  mvcgen [core.«%S».Insts.CoreConvertFromBool.from]
  · simp_all
  · simp_all

iscalar @[spec] theorem core.«%S».Insts.CoreConvertFromBool.from_spec (b : Bool) :
    ⦃ ⌜ True ⌝ ⦄
    core.«%S».Insts.CoreConvertFromBool.from b
    ⦃ ⇓ r => ⌜ r.val = b.toInt ⌝ ⦄ := by
  mvcgen [core.«%S».Insts.CoreConvertFromBool.from]
  · simp_all
  · simp_all

end CoreModels
