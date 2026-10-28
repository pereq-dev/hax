import Hax.MissingLean
import Lean.Meta.Tactic.Simp.BuiltinSimprocs.UInt

/-!
# USize64

We define a type `USize64` to represent Rust's `usize` type. It is simply a copy of `UInt64`.
This file aims to collect all definitions, lemmas, and type class instances about `UInt64` from
Lean's standard library and to state them for `USize64`.

The regular `USize` type does not work for us because of https://github.com/pereq-dev/hax/issues/1702.
-/

/-- A copy of `UInt64`, which we use to represent Rust's `usize` type. -/
structure USize64 where ofBitVec :: toBitVec : BitVec 64

@[reducible] def USize64.size : Nat := UInt64.size
def USize64.ofNat (n : @& Nat) : USize64 := ⟨BitVec.ofNat 64 n⟩
def USize64.toNat (n : USize64) : Nat := n.toBitVec.toNat
def USize64.toFin (x : USize64) : Fin UInt64.size := x.toBitVec.toFin

def USize64.ofNatLT (n : @& Nat) (h : LT.lt n USize64.size) : USize64 where
  toBitVec := BitVec.ofNatLT n h

def USize64.decEq (a b : USize64) : Decidable (Eq a b) :=
  match a, b with
  | ⟨n⟩, ⟨m⟩ =>
    dite (Eq n m)
      (fun h => isTrue (h ▸ rfl))
      (fun h => isFalse (fun h' => USize64.noConfusion h' (fun h' => absurd h' h)))

abbrev Nat.toUSize64 := USize64.ofNat

namespace USize64

instance : DecidableEq USize64 := USize64.decEq

instance : Inhabited USize64 where
  default := USize64.ofNatLT 0 (of_decide_eq_true rfl)

instance {n} : OfNat USize64 n := ⟨⟨OfNat.ofNat n⟩⟩

end USize64

@[inline] def USize64.ofFin (a : Fin USize64.size) : USize64 := ⟨⟨a⟩⟩

def USize64.ofInt (x : Int) : USize64 := ofNat (x % 2 ^ 64).toNat

@[simp] theorem USize64.le_size : 2 ^ 32 ≤ USize64.size := by simp [USize64.size, UInt64.size]
@[simp] theorem USize64.size_le : USize64.size ≤ 2 ^ 64 := by simp [USize64.size, UInt64.size]

protected def USize64.add (a b : USize64) : USize64 := ⟨a.toBitVec + b.toBitVec⟩
protected def USize64.sub (a b : USize64) : USize64 := ⟨a.toBitVec - b.toBitVec⟩
protected def USize64.mul (a b : USize64) : USize64 := ⟨a.toBitVec * b.toBitVec⟩
protected def USize64.div (a b : USize64) : USize64 := ⟨a.toBitVec / b.toBitVec⟩
protected def USize64.pow (x : USize64) (n : Nat) : USize64 := ⟨x.toBitVec ^ n⟩
protected def USize64.mod (a b : USize64) : USize64 := ⟨a.toBitVec % b.toBitVec⟩

protected def USize64.land (a b : USize64) : USize64 := ⟨a.toBitVec &&& b.toBitVec⟩
protected def USize64.lor (a b : USize64) : USize64 := ⟨a.toBitVec ||| b.toBitVec⟩
protected def USize64.xor (a b : USize64) : USize64 := ⟨a.toBitVec ^^^ b.toBitVec⟩
protected def USize64.shiftLeft (a b : USize64) : USize64 := ⟨a.toBitVec <<< (USize64.mod b 64).toBitVec⟩
protected def USize64.shiftRight (a b : USize64) : USize64 := ⟨a.toBitVec >>> (USize64.mod b 64).toBitVec⟩
protected def USize64.lt (a b : USize64) : Prop := a.toBitVec < b.toBitVec
protected def USize64.le (a b : USize64) : Prop := a.toBitVec ≤ b.toBitVec

instance : Add USize64       := ⟨USize64.add⟩
instance : Sub USize64       := ⟨USize64.sub⟩
instance : Mul USize64       := ⟨USize64.mul⟩
instance : Pow USize64 Nat   := ⟨USize64.pow⟩
instance : Mod USize64       := ⟨USize64.mod⟩

instance : HMod USize64 Nat USize64 := ⟨fun x n => ⟨x.toBitVec % n⟩⟩

instance : Div USize64       := ⟨USize64.div⟩
instance : LT USize64        := ⟨USize64.lt⟩
instance : LE USize64        := ⟨USize64.le⟩

protected def USize64.complement (a : USize64) : USize64 := ⟨~~~a.toBitVec⟩
protected def USize64.neg (a : USize64) : USize64 := ⟨-a.toBitVec⟩

instance : Complement USize64 := ⟨USize64.complement⟩
instance : Neg USize64 := ⟨USize64.neg⟩
instance : AndOp USize64     := ⟨USize64.land⟩
instance : OrOp USize64      := ⟨USize64.lor⟩
instance : XorOp USize64       := ⟨USize64.xor⟩
instance : ShiftLeft USize64  := ⟨USize64.shiftLeft⟩
instance : ShiftRight USize64 := ⟨USize64.shiftRight⟩

def USize64.ofNat32 (n : @& Nat) (h : n < 4294967296) : USize64 :=
  USize64.ofNatLT n (Nat.lt_of_lt_of_le h USize64.le_size)
def UInt8.toUSize64 (a : UInt8) : USize64 :=
  USize64.ofNat32 a.toBitVec.toNat (Nat.lt_trans a.toBitVec.isLt (by decide))
def USize64.toUInt8 (a : USize64) : UInt8 := a.toNat.toUInt8
def UInt16.toUSize64 (a : UInt16) : USize64 :=
  USize64.ofNat32 a.toBitVec.toNat (Nat.lt_trans a.toBitVec.isLt (by decide))
def USize64.toUInt16 (a : USize64) : UInt16 := a.toNat.toUInt16
def UInt32.toUSize64 (a : UInt32) : USize64 := USize64.ofNat32 a.toBitVec.toNat a.toBitVec.isLt
def USize64.toUInt32 (a : USize64) : UInt32 := a.toNat.toUInt32
def UInt64.toUSize64 (a : UInt64) : USize64 := a.toNat.toUSize64
def USize64.toUInt64 (a : USize64) : UInt64 := a.toNat.toUInt64
def USize64.toUSize (a : USize64) : USize := a.toNat.toUSize

def USize64.toInt8 (a : USize64) : Int8 := a.toNat.toInt8
def USize64.toInt16 (a : USize64) : Int16 := a.toNat.toInt16
def USize64.toInt32 (a : USize64) : Int32 := a.toNat.toInt32
def USize64.toInt64 (a : USize64) : Int64 := a.toNat.toInt64
def USize64.toISize (a : USize64) : ISize := a.toNat.toISize

def Int8.toUSize64 (a : Int8) : USize64 := USize64.ofInt a.toInt
def Int16.toUSize64 (a : Int16) : USize64 := USize64.ofInt a.toInt
def Int32.toUSize64 (a : Int32) : USize64 := USize64.ofInt a.toInt
def Int64.toUSize64 (a : Int64) : USize64 := USize64.ofInt a.toInt
def ISize.toUSize64 (a : ISize) : USize64 := USize64.ofInt a.toInt

def Bool.toUSize64 (b : Bool) : USize64 := if b then 1 else 0
def USize64.decLt (a b : USize64) : Decidable (a < b) :=
  inferInstanceAs (Decidable (a.toBitVec < b.toBitVec))

def USize64.decLe (a b : USize64) : Decidable (a ≤ b) :=
  inferInstanceAs (Decidable (a.toBitVec ≤ b.toBitVec))

attribute [instance_reducible, instance] USize64.decLt USize64.decLe

instance : Max USize64 := maxOfLe
instance : Min USize64 := minOfLe

instance {α} : GetElem (Array α) USize64 α fun xs i => i.toNat < xs.size where
  getElem xs i h := xs[i.toNat]

open Std Lean in
set_option autoImplicit true in
declare_uint_theorems USize64 64

theorem USize64.uaddOverflow_iff (x y : USize64) :
    BitVec.uaddOverflow x.toBitVec y.toBitVec ↔ x.toNat + y.toNat ≥ 2 ^ 64 :=
  by simp [BitVec.uaddOverflow]

theorem USize64.umulOverflow_iff (x y : USize64) :
    BitVec.umulOverflow x.toBitVec y.toBitVec ↔ x.toNat * y.toNat ≥ 2 ^ 64 :=
  by simp [BitVec.umulOverflow]

attribute [grind =] USize64.toNat_toBitVec
attribute [grind =] USize64.toNat_ofNat_of_lt
attribute [grind =] USize64.toNat_ofNat_of_lt'
grind_pattern USize64.toBitVec_ofNat => USize64.toBitVec (OfNat.ofNat n)

additional_uint_decls USize64 64

@[simp] theorem USize64.toNat_lt (n : USize64) : n.toNat < 2 ^ 64 := n.toFin.isLt

theorem USize64.le_self_add {a b : USize64} (h : a.toNat + b.toNat < 2 ^ 64) :
    a ≤ a + b := by
  rw [le_iff_toNat_le, USize64.toNat_add_of_lt h]
  exact Nat.le_add_right a.toNat b.toNat

theorem USize64.add_le_of_le {a b c : USize64} (habc : a + b ≤ c) (hab : a.toNat + b.toNat < 2 ^ 64):
    a ≤ c := by
  rw [USize64.le_iff_toNat_le, USize64.toNat_add_of_lt hab] at *
  omega

/-!
## Init.Data.UInt.Lemmas
-/

protected theorem USize64.add_assoc (a b c : USize64) : a + b + c = a + (b + c) :=
  USize64.toBitVec_inj.1 (BitVec.add_assoc _ _ _)

protected theorem USize64.add_comm (a b : USize64) : a + b = b + a := USize64.toBitVec_inj.1 (BitVec.add_comm _ _)

@[simp] protected theorem USize64.add_zero (a : USize64) : a + 0 = a := USize64.toBitVec_inj.1 (BitVec.add_zero _)

protected theorem USize64.add_left_neg (a : USize64) : -a + a = 0 := USize64.toBitVec_inj.1 (BitVec.add_left_neg _)

protected theorem USize64.mul_assoc (a b c : USize64) : a * b * c = a * (b * c) := USize64.toBitVec_inj.1 (BitVec.mul_assoc _ _ _)

@[simp] theorem USize64.mul_one (a : USize64) : a * 1 = a := USize64.toBitVec_inj.1 (BitVec.mul_one _)

@[simp] theorem USize64.one_mul (a : USize64) : 1 * a = a := USize64.toBitVec_inj.1 (BitVec.one_mul _)

protected theorem USize64.mul_comm (a b : USize64) : a * b = b * a := USize64.toBitVec_inj.1 (BitVec.mul_comm _ _)

@[simp] theorem USize64.mul_zero {a : USize64} : a * 0 = 0 := USize64.toBitVec_inj.1 BitVec.mul_zero

@[simp] theorem USize64.zero_mul {a : USize64} : 0 * a = 0 := USize64.toBitVec_inj.1 BitVec.zero_mul

protected theorem USize64.sub_eq_add_neg (a b : USize64) : a - b = a + (-b) := USize64.toBitVec_inj.1 (BitVec.sub_eq_add_neg _ _)

@[simp] protected theorem USize64.pow_zero (x : USize64) : x ^ 0 = 1 := (rfl)

protected theorem USize64.pow_succ (x : USize64) (n : Nat) : x ^ (n + 1) = x ^ n * x := (rfl)

theorem USize64.ofNat_eq_iff_mod_eq_toNat (a : Nat) (b : USize64) : USize64.ofNat a = b ↔ a % 2 ^ 64 = b.toNat := by
  simp [← USize64.toNat_inj]

@[simp] theorem USize64.ofNat_add (a b : Nat) : USize64.ofNat (a + b) = USize64.ofNat a + USize64.ofNat b := by
  simp [USize64.ofNat_eq_iff_mod_eq_toNat]

theorem USize64.ofNat_mod_size (x : Nat) : ofNat (x % 2 ^ 64) = ofNat x := by
  simp [ofNat, BitVec.ofNat, Fin.ofNat]

@[simp] theorem USize64.ofNat_mul (a b : Nat) : USize64.ofNat (a * b) = USize64.ofNat a * USize64.ofNat b := by
  simp [USize64.ofNat_eq_iff_mod_eq_toNat]

@[simp] theorem USize64.ofInt_mul (x y : Int) : ofInt (x * y) = ofInt x * ofInt y := by
  dsimp only [USize64.ofInt]
  rw [Int.mul_emod]
  have h₁ : 0 ≤ x % 2 ^ 64 := Int.emod_nonneg _ (by decide)
  have h₂ : 0 ≤ y % 2 ^ 64 := Int.emod_nonneg _ (by decide)
  have h₃ : 0 ≤ (x % 2 ^ 64) * (y % 2 ^ 64) := Int.mul_nonneg h₁ h₂
  rw [Int.toNat_emod h₃ (by decide), Int.toNat_mul h₁ h₂]
  have : (2 ^ 64 : Int).toNat = 2 ^ 64 := (rfl)
  rw [this, USize64.ofNat_mod_size, USize64.ofNat_mul]

@[simp] theorem USize64.ofInt_neg_one : ofInt (-1) = -1 := (rfl)

theorem USize64.toBitVec_one : toBitVec 1 = 1#64 := (rfl)

theorem USize64.neg_eq_neg_one_mul (a : USize64) : -a = -1 * a := by
  apply USize64.toBitVec_inj.1
  rw [USize64.toBitVec_neg, USize64.toBitVec_mul, USize64.toBitVec_neg, USize64.toBitVec_one, BitVec.neg_eq_neg_one_mul]

@[simp] protected theorem USize64.ofInt_neg (x : Int) : ofInt (-x) = -ofInt x := by
  rw [Int.neg_eq_neg_one_mul, ofInt_mul, ofInt_neg_one, ← USize64.neg_eq_neg_one_mul]

protected theorem USize64.mul_add {a b c : USize64} : a * (b + c) = a * b + a * c :=
    USize64.toBitVec_inj.1 BitVec.mul_add

protected theorem USize64.add_mul {a b c : USize64} : (a + b) * c = a * c + b * c := by
  rw [USize64.mul_comm, USize64.mul_add, USize64.mul_comm a c, USize64.mul_comm c b]

protected theorem USize64.neg_mul (a b : USize64) : -a * b = -(a * b) := USize64.toBitVec_inj.1 (BitVec.neg_mul _ _)

@[simp] protected theorem USize64.add_sub_cancel (a b : USize64) : a + b - b = a := USize64.toBitVec_inj.1 (BitVec.add_sub_cancel _ _)

theorem USize64.ofNat_sub {a b : Nat} (hab : b ≤ a) : USize64.ofNat (a - b) = USize64.ofNat a - USize64.ofNat b := by
  rw [(Nat.sub_add_cancel hab ▸ USize64.ofNat_add (a - b) b :), USize64.add_sub_cancel]

@[simp] protected theorem USize64.sub_add_cancel (a b : USize64) : a - b + b = a :=
  USize64.toBitVec_inj.1 (BitVec.sub_add_cancel _ _)

theorem USize64.le_ofNat_iff {n : USize64} {m : Nat} (h : m < size) : n ≤ ofNat m ↔ n.toNat ≤ m := by
  rw [le_iff_toNat_le, toNat_ofNat_of_lt' h]

/-!
## Grind's ToInt

For grind to use integer arithmetic on `USize64`, we need the following instances, inspired by
the modules `Init.GrindInstances.ToInt` and `Init.GrindInstances.Ring.UInt`.
-/

namespace Lean.Grind

instance : ToInt USize64 (.uint 64) where
  toInt x := (x.toNat : Int)
  toInt_inj x y w := USize64.toNat_inj.mp (Int.ofNat_inj.mp w)
  toInt_mem x := by simpa using Int.lt_toNat.mp (USize64.toNat_lt x)

@[simp] theorem toInt_usize64 (x : USize64) : ToInt.toInt x = (x.toNat : Int) := rfl

instance : ToInt.Zero USize64 (.uint 64) where
  toInt_zero := by simp

instance : ToInt.OfNat USize64 (.uint 64) where
  toInt_ofNat x := by simp; rfl

instance : ToInt.Add USize64 (.uint 64) where
  toInt_add x y := by simp

instance : ToInt.Mul USize64 (.uint 64) where
  toInt_mul x y := by simp

instance : ToInt.Mod USize64 (.uint 64) where
  toInt_mod x y := by simp

instance : ToInt.Div USize64 (.uint 64) where
  toInt_div x y := by simp

instance : ToInt.LE USize64 (.uint 64) where
  le_iff x y := by simpa using USize64.le_iff_toBitVec_le

instance : ToInt.LT USize64 (.uint 64) where
  lt_iff x y := by simpa using USize64.lt_iff_toBitVec_lt


@[expose]
def USize64.natCast : NatCast USize64 where
  natCast x := USize64.ofNat x

@[expose]
def USize64.intCast : IntCast USize64 where
  intCast x := USize64.ofInt x

attribute [local instance_reducible, local instance] USize64.natCast USize64.intCast

theorem USize64.intCast_ofNat (x : Nat) : (OfNat.ofNat (α := Int) x : USize64) = OfNat.ofNat x := by
    change USize64.ofInt (OfNat.ofNat x) = OfNat.ofNat x
    rw [USize64.ofInt]
    rw [Int.toNat_emod (Int.zero_le_ofNat x) (by decide)]
    erw [Int.toNat_natCast]
    rw [Int.toNat_pow_of_nonneg (by decide)]
    simp +instances only [USize64.ofNat, BitVec.ofNat, Fin.Internal.ofNat_eq_ofNat, Fin.ofNat, Int.reduceToNat, Nat.dvd_refl,
      Nat.mod_mod_of_dvd]
    try rfl

theorem USize64.intCast_neg (x : Int) : ((-x : Int) : USize64) = - (x : USize64) :=
  USize64.ofInt_neg _

instance : CommRing USize64 where
  nsmul := ⟨(· * ·)⟩
  zsmul := ⟨(· * ·)⟩
  add_assoc := USize64.add_assoc
  add_comm := USize64.add_comm
  add_zero := USize64.add_zero
  neg_add_cancel := USize64.add_left_neg
  mul_assoc := USize64.mul_assoc
  mul_comm := USize64.mul_comm
  mul_one := USize64.mul_one
  one_mul := USize64.one_mul
  left_distrib _ _ _ := USize64.mul_add
  right_distrib _ _ _ := USize64.add_mul
  zero_mul _ := USize64.zero_mul
  mul_zero _ := USize64.mul_zero
  sub_eq_add_neg := USize64.sub_eq_add_neg
  pow_zero := USize64.pow_zero
  pow_succ := USize64.pow_succ
  ofNat_succ x := USize64.ofNat_add x 1
  intCast_neg := USize64.ofInt_neg
  intCast_ofNat := USize64.intCast_ofNat
  neg_zsmul i a := by
    change (-i : Int) * a = - (i * a)
    simp [USize64.intCast_neg, USize64.neg_mul]
  zsmul_natCast_eq_nsmul n a := congrArg (· * a) (USize64.intCast_ofNat _)

instance : IsCharP USize64 18446744073709551616 := IsCharP.mk' _ _
  (ofNat_eq_zero_iff := fun x => by
    have : OfNat.ofNat x = USize64.ofNat x := rfl
    simp [this, USize64.ofNat_eq_iff_mod_eq_toNat]
    )

instance : ToInt.Pow USize64 (.uint 64) := ToInt.pow_of_semiring (by simp)


end Lean.Grind


/-!
## Simp-Procs

Grind and simp use some simplification procedures for UInts. They are defined in
`Lean.Meta.Tactic.Simp.BuiltinSimprocs.UInt` and replicated here.
-/

namespace USize64
open Lean Meta Simp

instance : ToExpr USize64 where
  toTypeExpr := mkConst ``USize64
  toExpr a :=
    let r := mkRawNatLit a.toNat
    mkApp3 (.const ``OfNat.ofNat [0]) (mkConst ``USize64) r
      (.app (.const ``USize64.instOfNat []) r)

def fromExpr (e : Expr) : SimpM (Option USize64) := do
  let some (n, _) ← getOfNatValue? e `USize64 | return none
  return USize64.ofNat n

@[inline] def reduceBin (declName : Name) (arity : Nat) (op : USize64 → USize64 → USize64) (e : Expr) : SimpM DStep := do
  unless e.isAppOfArity declName arity do return .continue
  let some n ← (fromExpr e.appFn!.appArg!) | return .continue
  let some m ← (fromExpr e.appArg!) | return .continue
  return .done <| toExpr (op n m)

@[inline] def reduceBinPred (declName : Name) (arity : Nat) (op : USize64 → USize64 → Bool) (e : Expr) : SimpM Step := do
  unless e.isAppOfArity declName arity do return .continue
  let some n ← (fromExpr e.appFn!.appArg!) | return .continue
  let some m ← (fromExpr e.appArg!) | return .continue
  evalPropStep e (op n m)

@[inline] def reduceBoolPred (declName : Name) (arity : Nat) (op : USize64 → USize64 → Bool) (e : Expr) : SimpM DStep := do
  unless e.isAppOfArity declName arity do return .continue
  let some n ← (fromExpr e.appFn!.appArg!) | return .continue
  let some m ← (fromExpr e.appArg!) | return .continue
  return .done <| toExpr (op n m)

dsimproc [simp, seval] reduceAdd ((_ + _ : USize64)) := reduceBin ``HAdd.hAdd 6 (· + ·)
dsimproc [simp, seval] reduceMul ((_ * _ : USize64)) := reduceBin ``HMul.hMul 6 (· * ·)
dsimproc [simp, seval] reduceSub ((_ - _ : USize64)) := reduceBin ``HSub.hSub 6 (· - ·)
dsimproc [simp, seval] reduceDiv ((_ / _ : USize64)) := reduceBin ``HDiv.hDiv 6 (· / ·)
dsimproc [simp, seval] reduceMod ((_ % _ : USize64)) := reduceBin ``HMod.hMod 6 (· % ·)

simproc [simp, seval] reduceLT  (( _ : USize64) < _)  := reduceBinPred ``LT.lt 4 (. < .)
simproc [simp, seval] reduceLE  (( _ : USize64) ≤ _)  := reduceBinPred ``LE.le 4 (. ≤ .)
simproc [simp, seval] reduceGT  (( _ : USize64) > _)  := reduceBinPred ``GT.gt 4 (. > .)
simproc [simp, seval] reduceGE  (( _ : USize64) ≥ _)  := reduceBinPred ``GE.ge 4 (. ≥ .)
simproc [simp, seval] reduceEq  (( _ : USize64) = _)  := reduceBinPred ``Eq 3 (. = .)
simproc [simp, seval] reduceNe  (( _ : USize64) ≠ _)  := reduceBinPred ``Ne 3 (. ≠ .)
dsimproc [simp, seval] reduceBEq  (( _ : USize64) == _)  := reduceBoolPred ``BEq.beq 4 (. == .)
dsimproc [simp, seval] reduceBNe  (( _ : USize64) != _)  := reduceBoolPred ``bne 4 (. != .)

dsimproc [simp, seval] reduceOfNatLT (USize64.ofNatLT _ _) := fun e => do
  unless e.isAppOfArity `USize64.ofNatLT 2 do return .continue
  let some value ← Nat.fromExpr? e.appFn!.appArg! | return .continue
  let value := USize64.ofNat value
  return .done <| toExpr value

dsimproc [simp, seval] reduceOfNat (USize64.ofNat _) := fun e => do
  unless e.isAppOfArity `USize64.ofNat 1 do return .continue
  let some value ← Nat.fromExpr? e.appArg! | return .continue
  let value := USize64.ofNat value
  return .done <| toExpr value

dsimproc [simp, seval] reduceToNat (USize64.toNat _) := fun e => do
  unless e.isAppOfArity `USize64.toNat 1 do return .continue
  let some v ← (fromExpr e.appArg!) | return .continue
  let n := USize64.toNat v
  return .done <| toExpr n

/-- Return `.done` for UInt values. We don't want to unfold in the symbolic evaluator. -/
dsimproc [seval] isValue ((OfNat.ofNat _ : USize64)) := fun e => do
  unless (e.isAppOfArity ``OfNat.ofNat 3) do return .continue
  return .done e

end USize64

/-
## Lemmas from `Init.SizeOfLemmas`:
-/

@[simp] protected theorem USize64.sizeOf (a : USize64) : sizeOf a = a.toNat + 3 := by
  cases a; simp +arith [USize64.toNat, BitVec.toNat, -BitVec.val_toFin]

/-
## Lemmas from `MissingLean`:
-/

theorem USize64.ofNat_eq_of_toNat_eq {a : Nat} {b : USize64} (h : b.toNat = a) : ofNat a = b := by
  subst_vars; exact USize64.ofNat_toNat

theorem USize64.sub_add_eq {a b c : USize64} : a - (b + c) = a - b - c := by grind

theorem USize64.sub_succ_lt_self (a b : USize64) (h : a < b) :
    (b - (a + 1)).toNat < (b - a).toNat := by
  rw [sub_add_eq]
  rw [USize64.toNat_sub_of_le]
  try simp only [USize.toNat_one]
  apply Nat.sub_one_lt_of_lt
  · change (0 : USize64).toNat < (b - a).toNat
    rw [← lt_iff_toNat_lt]
    grind
  · grind
