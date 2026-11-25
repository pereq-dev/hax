module
public import CoreModels.MissingAeneas
public import CoreModels.Core.Funs
public import CoreModels.Spec.RustPrimitives.Slice

@[expose] public section

namespace CoreModels

open Aeneas
open Aeneas.Std hiding namespace core alloc
open Std.Do WP RustM ScalarElab

set_option mvcgen.warning false
set_option linter.deprecated.syntax false

attribute [spec]
  CoreModels.core.num.U8.from_le_bytes
  CoreModels.core.num.U16.from_le_bytes
  CoreModels.core.num.U32.from_le_bytes
  CoreModels.core.num.U64.from_le_bytes
  CoreModels.core.num.U128.from_le_bytes
  CoreModels.core.num.Usize.from_le_bytes

/-! ## `to_le_bytes`

The model builds the array with `array::from_fn` and a closure that shifts out one byte at a time.
The specs below state the result as Aeneas's `to_le_bytes` instead, so that it can be combined
with `from_le_bytes` and the `BitVec.toLEBytes` lemmas. -/

/-- Byte `i` of the little-endian encoding of `b`. -/
private theorem BitVec.setWidth_ushiftRight_eq_toLEBytes {w : Nat} (b : BitVec w) (i : Nat)
    (hi : i < b.toLEBytes.length) :
    (b >>> (8 * i)).setWidth 8 = b.toLEBytes[i] := by
  ext j hj
  by_cases h : 8 * i + j < w
  · rw [BitVec.getElem_toLEBytes_eq_getElem b i j hi hj h]
    simp only [BitVec.getElem_setWidth, BitVec.getLsbD_ushiftRight]
    exact BitVec.getLsbD_eq_getElem h
  · rw [BitVec.getElem_toLEBytes_eq_false b i j hi hj (by omega)]
    simp [BitVec.getElem_setWidth, BitVec.getLsbD_ushiftRight]
    exact BitVec.getLsbD_of_ge _ _ (by omega)

uscalar_no_usize
/-- The closure passed to `array::from_fn` computes byte `i` and leaves the value unchanged. -/
theorem core.num.«%S».to_le_bytes.closure.call_mut_spec (x : «%S») (i : Usize)
    (hi : i.val < %Size) :
    ⦃ ⌜ True ⌝ ⦄
    core.num.«%S».to_le_bytes.closure.Insts.CoreOpsFunctionFnMutTupleUsizeU8.call_mut x i
    ⦃ ⇓ r => ⌜ r.2 = x ∧ r.1.bv = (x.bv >>> (8 * i.val)).setWidth 8 ⌝ ⦄ := by
  have hc : (UScalar.cast UScalarTy.U32 i).val = i.val := by
    rw [UScalar.cast_val_eq]; apply Nat.mod_eq_of_lt; simp; omega
  have h8 : 8 * i.val % %BitWidth = 8 * i.val := Nat.mod_eq_of_lt (by omega)
  mvcgen [core.num.«%S».to_le_bytes.closure.Insts.CoreOpsFunctionFnMutTupleUsizeU8.call_mut]
  · subst_vars
    simp only [U8.bv, UScalar.cast] at *
    grind
  · subst_vars
    scalar_tac
  · subst_vars
    scalar_tac

uscalar_no_usize
/-- `to_le_bytes` computes Aeneas's `to_le_bytes`. -/
@[spec] theorem core.num.«%S».to_le_bytes_spec (x : «%S») :
    ⦃ ⌜ True ⌝ ⦄ core.num.«%S».to_le_bytes x
    ⦃ ⇓ r => ⌜ r = Aeneas.Std.core.num.«%S».to_le_bytes x ⌝ ⦄ := by
  have hidx : ∀ k, k < %Size →
      (⟨BitVec.ofNat UScalarTy.Usize.numBits k⟩ : Usize).val = k := by
    intro k hk
    obtain hb | hb := System.Platform.numBits_eq <;>
      simp [UScalar.val, hb] <;> omega
  mvcgen [core.num.«%S».to_le_bytes]
  · rename_i k hk
    have hk : k < %Size := by simpa using hk
    have hs := core.num.«%S».to_le_bytes.closure.call_mut_spec x _ ((hidx k hk).symm ▸ hk)
    mvcgen [hs]
    grind
  · intro h
    apply Aeneas.Std.Array.ext
    apply List.ext_getElem
    · simp [Std.core.num.«%S».to_le_bytes, BitVec.toLEBytes_length]
    · intro i h1 h2
      have hi : i < %Size := by simpa using h1
      apply triple_in_hypothesis _ (h i (by simpa using hi))
      have hs := core.num.«%S».to_le_bytes.closure.call_mut_spec x _ ((hidx i hi).symm ▸ hi)
      mvcgen [hs]
      rename_i r
      intro _ hbv heq
      rw [← heq]
      rw [hidx i hi, BitVec.setWidth_ushiftRight_eq_toLEBytes _ _
        (by simp [BitVec.toLEBytes_length]; omega)] at hbv
      simp only [Std.core.num.«%S».to_le_bytes, Array.from_val, List.getElem_map]
      rw [← hbv]

/-- The `usize` closure passed to `array::from_fn`, on a 64-bit platform. -/
theorem core.num.Usize.to_le_bytes.closure.call_mut_spec (x : Usize) (i : Usize)
    (h64 : System.Platform.numBits = 64) (hi : i.val < 8) :
    ⦃ ⌜ True ⌝ ⦄
    core.num.Usize.to_le_bytes.closure.Insts.CoreOpsFunctionFnMutTupleUsizeU8.call_mut x i
    ⦃ ⇓ r => ⌜ r.2 = x ∧ r.1.bv = (x.bv >>> (8 * i.val)).setWidth 8 ⌝ ⦄ := by
  have hc : (UScalar.cast UScalarTy.U32 i).val = i.val := by
    rw [UScalar.cast_val_eq]; apply Nat.mod_eq_of_lt; simp; omega
  have hbits : (Std.core.num.Usize.BITS).val = 64 := by
    simp [Std.core.num.Usize.BITS, h64]
  mvcgen [core.num.Usize.to_le_bytes.closure.Insts.CoreOpsFunctionFnMutTupleUsizeU8.call_mut]
  · subst_vars
    simp only [U8.bv, UScalar.cast] at *
    grind
  · subst_vars
    scalar_tac
  · subst_vars
    scalar_tac
  · subst_vars
    scalar_tac

/-- `usize::to_le_bytes` on a 64-bit platform.

Core models assume 64-bit, so this spec does, too. It is not a `[spec]`, because `mvcgen` would
leave `System.Platform.numBits = 64` as a goal that cannot be proven. Use it explicitly instead:
`have h := core.num.Usize.to_le_bytes_spec x h64` followed by `mvcgen [h]`. -/
theorem core.num.Usize.to_le_bytes_spec (x : Usize)
    (h64 : System.Platform.numBits = 64) :
    ⦃ ⌜ True ⌝ ⦄ core.num.Usize.to_le_bytes x
    ⦃ ⇓ r => ⌜ r.val = x.bv.toLEBytes.map (@UScalar.mk UScalarTy.U8) ⌝ ⦄ := by
  have hidx : ∀ k, k < 8 →
      (⟨BitVec.ofNat UScalarTy.Usize.numBits k⟩ : Usize).val = k := by
    intro k hk
    simp [UScalar.val, h64]; omega
  have hlen : x.bv.toLEBytes.length = 8 := by
    simp [BitVec.toLEBytes_length, h64]
  mvcgen [core.num.Usize.to_le_bytes]
  · rename_i k hk
    have hk : k < 8 := by simpa using hk
    have hs :=
      core.num.Usize.to_le_bytes.closure.call_mut_spec x _ h64 ((hidx k hk).symm ▸ hk)
    mvcgen [hs]
    grind
  · intro h
    apply List.ext_getElem
    · simp [hlen]
    · intro i h1 h2
      have hi : i < 8 := by simpa using h1
      apply triple_in_hypothesis _ (h i (by simpa using hi))
      have hs :=
        core.num.Usize.to_le_bytes.closure.call_mut_spec x _ h64 ((hidx i hi).symm ▸ hi)
      mvcgen [hs]
      rename_i r
      intro _ hbv heq
      rw [← heq]
      rw [hidx i hi, BitVec.setWidth_ushiftRight_eq_toLEBytes _ _ (by omega)] at hbv
      simp only [List.getElem_map]
      rw [← hbv]

/-! ## `rem_euclid` -/

/-- The value `urem_euclid` returns when `y ≠ 0`. -/
private theorem urem_euclid_val {ty : UScalarTy} (x y : UScalar ty) :
    (⟨BitVec.ofNat _ (x.val % y.val)⟩ : UScalar ty).val = x.val % y.val := by
  simp only [UScalar.val, BitVec.toNat_ofNat]
  exact Nat.mod_eq_of_lt (Nat.lt_of_le_of_lt (Nat.mod_le _ _) x.hBounds)

/-- The value `irem_euclid` returns when `y ≠ 0`. -/
private theorem irem_euclid_val {ty : IScalarTy} (x y : IScalar ty) (hy : y.val ≠ 0) :
    (⟨BitVec.ofInt _ (x.val % y.val)⟩ : IScalar ty).val = x.val % y.val := by
  have hx := y.hBounds
  have hnn := Int.emod_nonneg x.val hy
  have hlt := Int.emod_lt x.val hy
  have hpos : 0 < ty.numBits := by cases ty <;> simp [IScalarTy.numBits]
  have hpow : (2 : Int) ^ ty.numBits = 2 * 2 ^ (ty.numBits - 1) := by
    rw [← pow_succ']; congr 1; omega
  simp only [IScalar.val, BitVec.toInt_ofInt] at *
  rw [Int.bmod_def]
  push_cast
  rw [hpow, Int.emod_eq_of_lt hnn (by omega)]
  split_ifs <;> omega

uscalar @[spec] theorem core.num.«%S».rem_euclid_spec (x y : «%S») (hy : y.val ≠ 0) :
    ⦃ ⌜ True ⌝ ⦄ core.num.«%S».rem_euclid x y ⦃ ⇓ r => ⌜ r.val = x.val % y.val ⌝ ⦄ := by
  mvcgen [core.num.«%S».rem_euclid, urem_euclid]
  exact urem_euclid_val x y

iscalar @[spec] theorem core.num.«%S».rem_euclid_spec (x y : «%S») (hy : y.val ≠ 0)
    (hov : ¬ (x.val = «%S».min ∧ y.val = -1)) :
    ⦃ ⌜ True ⌝ ⦄ core.num.«%S».rem_euclid x y ⦃ ⇓ r => ⌜ r.val = x.val % y.val ⌝ ⦄ := by
  -- `irem_euclid` states the `MIN % -1` overflow with `IScalar.min`.
  have hov : ¬ (x.val = IScalar.min IScalarTy.«%S» ∧ y.val = -1) := by
    simpa [«%S».min, IScalar.min, «%S».numBits] using hov
  mvcgen [core.num.«%S».rem_euclid, irem_euclid]
  exact irem_euclid_val x y hy

end CoreModels
