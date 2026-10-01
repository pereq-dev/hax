
-- Legacy lean backend for Hax
-- The Hax prelude library can be found in hax/proof-libs/legacy-lean
import Hax
import Std.Tactic.Do
import Std.Do.Triple
import Std.Tactic.Do.Syntax
open Std.Do
open Std.Tactic

set_option mvcgen.warning false
set_option linter.unusedVariables false


namespace new_tests.legacy__lean_tests__lib.array

@[spec]
def f (N : usize) (x : (RustArray u8 N)) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def g (N : usize) (x : (RustArray u8 N)) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ ← (f (N) x);
  let _ ←
    (f ((10 : usize)) (← (rust_primitives.hax.repeat (0 : u8) (10 : usize))));
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.legacy__lean_tests__lib.array


namespace new_tests.legacy__lean_tests__lib.associated_types.basic

@[spec]
def Impl.first_hoisted (self : Bool) : RustM i32 := do (pure (3 : i32))

end new_tests.legacy__lean_tests__lib.associated_types.basic


namespace new_tests.legacy__lean_tests__lib.associated_types.projection

class T1.AssociatedTypes (Self : Type) where
  A1 : Type

attribute [reducible] T1.AssociatedTypes.A1

abbrev T1.A1 :=
  T1.AssociatedTypes.A1

class T1 (Self : Type)
  [associatedTypes : outParam (T1.AssociatedTypes (Self : Type))]
  where

end new_tests.legacy__lean_tests__lib.associated_types.projection


namespace new_tests.legacy__lean_tests__lib.associated_types.multiple_associated_types

@[spec]
def Impl.first_hoisted (self : (rust_primitives.hax.Tuple2 i32 Bool)) :
    RustM i32 := do
  (pure (rust_primitives.hax.Tuple2._0 self))

@[spec]
def Impl.second_hoisted (self : (rust_primitives.hax.Tuple2 i32 Bool)) :
    RustM Bool := do
  (pure (rust_primitives.hax.Tuple2._1 self))

end new_tests.legacy__lean_tests__lib.associated_types.multiple_associated_types


namespace new_tests.legacy__lean_tests__lib.associated_types.multiple_projections

class FnOnce.AssociatedTypes (Self : Type) (T : Type) where
  Output : Type

attribute [reducible] FnOnce.AssociatedTypes.Output

abbrev FnOnce.Output :=
  FnOnce.AssociatedTypes.Output

class FnOnce (Self : Type) (T : Type)
  [associatedTypes : outParam (FnOnce.AssociatedTypes (Self : Type) (T : Type))]
  where

@[spec]
def func
    (T : Type)
    (U : Type)
    (D : Type)
    (F : Type)
    [trait_constr_func_associated_type_i0 : FnOnce.AssociatedTypes F T]
    [trait_constr_func_i0 : FnOnce
      F
      T
      (associatedTypes := {
        show FnOnce.AssociatedTypes F T
        by infer_instance
        with Output := U})]
    [trait_constr_func_associated_type_i1 : FnOnce.AssociatedTypes D T]
    [trait_constr_func_i1 : FnOnce
      D
      T
      (associatedTypes := {
        show FnOnce.AssociatedTypes D T
        by infer_instance
        with Output := U})]
    (d : D)
    (f : F)
    (u : U) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.legacy__lean_tests__lib.associated_types.multiple_projections


namespace new_tests.legacy__lean_tests__lib.binops

@[spec]
def noop (x : i32) : RustM i32 := do (pure x)

@[spec]
def neg_int (x : i32) : RustM i32 := do (-? x)

@[spec]
def not_int (x : i32) : RustM i32 := do (~? x)

@[spec]
def not_bool (x : Bool) : RustM Bool := do (!? x)

@[spec]
def index (x : (RustArray i32 1)) : RustM i32 := do x[(0 : usize)]_?

@[spec]
def add_int (x : i32) (y : i32) : RustM i32 := do (x +? y)

@[spec]
def sub_int (x : i32) (y : i32) : RustM i32 := do (x -? y)

@[spec]
def mul_int (x : i32) (y : i32) : RustM i32 := do (x *? y)

@[spec]
def div_int (x : i32) (y : i32) : RustM i32 := do (x /? y)

@[spec]
def rem_int (x : i32) (y : i32) : RustM i32 := do (x %? y)

@[spec]
def shr_int (x : i32) (y : i32) : RustM i32 := do (x >>>? y)

@[spec]
def shl_int (x : i32) (y : i32) : RustM i32 := do (x <<<? y)

@[spec]
def bitand_int (x : i32) (y : i32) : RustM i32 := do (x &&&? y)

@[spec]
def bitand_bool (x : Bool) (y : Bool) : RustM Bool := do (x &&? y)

@[spec]
def bitor_int (x : i32) (y : i32) : RustM i32 := do (x |||? y)

@[spec]
def bitor_bool (x : Bool) (y : Bool) : RustM Bool := do (x ||? y)

@[spec]
def bitxor_int (x : i32) (y : i32) : RustM i32 := do (x ^^^? y)

@[spec]
def bitxor_bool (x : Bool) (y : Bool) : RustM Bool := do (x ^^? y)

@[spec]
def logical_op_and (x : Bool) (y : Bool) : RustM Bool := do (x &&? y)

@[spec]
def logical_op_or (x : Bool) (y : Bool) : RustM Bool := do (x ||? y)

@[spec]
def eq_int (x : i32) (y : i32) : RustM Bool := do (x ==? y)

@[spec]
def eq_bool (x : Bool) (y : Bool) : RustM Bool := do (x ==? y)

@[spec]
def neq_int (x : i32) (y : i32) : RustM Bool := do (x !=? y)

@[spec]
def neq_bool (x : Bool) (y : Bool) : RustM Bool := do (x !=? y)

@[spec]
def lt_int (x : i32) (y : i32) : RustM Bool := do (x <? y)

@[spec]
def le_int (x : i32) (y : i32) : RustM Bool := do (x <=? y)

@[spec]
def gt_int (x : i32) (y : i32) : RustM Bool := do (x >? y)

@[spec]
def ge_int (x : i32) (y : i32) : RustM Bool := do (x >=? y)

structure S where
  -- no fields

@[spec]
def Impl.not_hoisted (self : S) : RustM S := do (pure self)

@[reducible] instance Impl.AssociatedTypes :
  core_models.ops.bit.Not.AssociatedTypes S
  where
  Output := S

instance Impl : core_models.ops.bit.Not S where
  not := (Impl.not_hoisted)

@[spec]
def Impl_1.add_hoisted (self : S) (rhs : S) : RustM S := do (pure self)

@[reducible] instance Impl_1.AssociatedTypes :
  core_models.ops.arith.Add.AssociatedTypes S S
  where
  Output := S

instance Impl_1 : core_models.ops.arith.Add S S where
  add := (Impl_1.add_hoisted)

@[spec]
def not_s (x : S) : RustM S := do (core_models.ops.bit.Not.not S x)

@[spec]
def add_s (x : S) (y : S) : RustM S := do
  (core_models.ops.arith.Add.add S S x y)

end new_tests.legacy__lean_tests__lib.binops


namespace new_tests.legacy__lean_tests__lib.casts

--  Returns true if all casting edge cases behave as expected.
def casting_edge_cases (_dummy : Bool) : RustM Bool := do
  let case1 : Bool ←
    ((← (rust_primitives.hax.cast_op (256 : u16) : RustM u8)) ==? (0 : u8));
  let case2 : Bool ←
    ((← (rust_primitives.hax.cast_op (-1 : i16) : RustM u8)) ==? (255 : u8));
  let case3 : Bool ←
    ((← (rust_primitives.hax.cast_op (-1 : i8) : RustM i16)) ==? (-1 : i16));
  let case4 : Bool ←
    ((← (rust_primitives.hax.cast_op (128 : u8) : RustM i8)) ==? (-128 : i8));
  let case5 : Bool ←
    ((← (rust_primitives.hax.cast_op (4294967295 : u32) : RustM i32))
      ==? (-1 : i32));
  ((← ((← ((← (case1 &&? case2)) &&? case3)) &&? case4)) &&? case5)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def casting_edge_cases.spec (_dummy : Bool) :
    Spec
      (requires := do pure True)
      (ensures := fun result => do (pure result))
      (casting_edge_cases (_dummy : Bool)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [casting_edge_cases] <;> bv_decide
}

--  https://github.com/pereq-dev/hax/issues/1912
@[spec]
def shift_after_cast (x : u16) (n : u8) : RustM u32 := do
  ((← (rust_primitives.hax.cast_op x : RustM u32))
    <<<? (← (rust_primitives.hax.cast_op n : RustM u32)))

--  https://github.com/pereq-dev/hax/issues/1911
@[spec]
def add_after_cast (a : u8) (b : u8) (c : u8) : RustM u16 := do
  ((← ((← (rust_primitives.hax.cast_op a : RustM u16))
      +? (← (rust_primitives.hax.cast_op b : RustM u16))))
    +? (← (rust_primitives.hax.cast_op c : RustM u16)))

end new_tests.legacy__lean_tests__lib.casts


namespace new_tests.legacy__lean_tests__lib.comments

--  Single line doc comment
@[spec]
def f (_ : rust_primitives.hax.Tuple0) : RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

/--
   Block doc-comment : Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum rutrum
  orci ac tellus ullamcorper sollicitudin. Sed fringilla mi id arcu suscipit rhoncus. Pellentesque et
  metus a ante feugiat lobortis. Nam a mauris eget nisl congue egestas. Duis et gravida
  nulla. Curabitur mattis leo vel molestie posuere. Etiam malesuada et augue eget
  varius. Pellentesque quis tincidunt erat. Vestibulum id consectetur turpis. Cras elementum magna id
  urna volutpat fermentum. In vel erat quis nunc rhoncus porta. Aliquam sed pellentesque
  tellus. Quisque odio diam, mollis ut venenatis non, scelerisque at nulla. Nunc urna ante, tristique
  quis nisi quis, congue maximus nisl. Curabitur non efficitur odio. 
  -/
@[spec]
def heavily_documented (_ : rust_primitives.hax.Tuple0) : RustM u32 := do
  (pure (4 : u32))

end new_tests.legacy__lean_tests__lib.comments


namespace new_tests.legacy__lean_tests__lib.constants

def C1 : u32 := (5678 : u32)

def C2 : u32 := RustM.of_isOk (do (C1 +? (1 : u32))) (by rfl)

def C3 : u32 :=
  RustM.of_isOk
    (do if true then do (pure (890 : u32)) else do ((9 : u32) /? (0 : u32)))
    (by rfl)

@[spec]
def computation (x : u32) : RustM u32 := do ((← (x +? x)) +? (1 : u32))

def C4 : u32 := RustM.of_isOk (do ((← (computation C1)) +? C2)) (by rfl)

def C5 : (rust_primitives.hax.Tuple2 u32 u32) :=
  RustM.of_isOk
    (do
    (pure (rust_primitives.hax.Tuple2.mk
      (← ((0 : u32) +? (0 : u32)))
      (0 : u32))))
    (by rfl)

def C6 : (RustArray u32 1) :=
  RustM.of_isOk (do (pure (RustArray.ofVec #v[(0 : u32)]))) (by rfl)

@[spec]
def test (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let x : u32 ← (C1 +? (1 : u32));
  let y : u32 ← (C2 +? C3);
  let z : u32 ← (C4 -? C3);
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.legacy__lean_tests__lib.constants


namespace new_tests.legacy__lean_tests__lib.constants.const_parameters

--  Function with const parameter
@[spec]
def f (N : usize) (_ : rust_primitives.hax.Tuple0) : RustM usize := do (pure N)

def N0 : usize := (1 : usize)

def N1 : usize := (10 : usize)

@[spec]
def test (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ ←
    ((← (f ((9 : usize)) rust_primitives.hax.Tuple0.mk))
      +? (← (f ((10 : usize)) rust_primitives.hax.Tuple0.mk)));
  (pure rust_primitives.hax.Tuple0.mk)

--  Trait definition
class T.AssociatedTypes (Self : Type) (N_TRAIT : usize) where

class T (Self : Type) (N_TRAIT : usize)
  [associatedTypes : outParam (T.AssociatedTypes (Self : Type) (N_TRAIT :
      usize))]
  where
  f (Self) (N_TRAIT) (N_FIELD : usize) : (Self -> RustM usize)

--  Struct definition
structure S (N : usize) where
  _0 : u32

@[spec]
def Impl.f_hoisted (N_TRAIT : usize) (N_FIELD : usize) (self : (S (N_TRAIT))) :
    RustM usize := do
  (N_TRAIT -? N_FIELD)

@[reducible] instance Impl.AssociatedTypes (N_TRAIT : usize) :
  T.AssociatedTypes (S (N_TRAIT)) (N_TRAIT)
  where

instance Impl (N_TRAIT : usize) : T (S (N_TRAIT)) (N_TRAIT) where
  f := fun  (N_FIELD : usize) => (Impl.f_hoisted (N_TRAIT) (N_FIELD))

@[spec]
def test2
    (N2 : usize)
    (A : Type)
    [trait_constr_test2_associated_type_i0 : T.AssociatedTypes A (N2)]
    [trait_constr_test2_i0 : T A (N2) ]
    (x : A) :
    RustM usize := do
  let s : (S ((10 : usize))) := (S.mk (9 : u32));
  let _ ←
    ((← (T.f (S ((10 : usize))) ((10 : usize)) ((1 : usize)) s))
      +? (← (T.f A (N2) ((11 : usize)) x)));
  let s : (S ((3 : usize))) := (S.mk (9 : u32));
  (T.f A (N2) ((4 : usize)) x)

end new_tests.legacy__lean_tests__lib.constants.const_parameters


namespace new_tests.legacy__lean_tests__lib.enums

inductive E : Type
| V1 : E
| V2 : E
| V3 : usize -> E
| V4 : usize -> usize -> usize -> E
| V5 (f1 : usize) (f2 : usize) : E
| V6 (f1 : usize) (f2 : usize) : E

end new_tests.legacy__lean_tests__lib.enums


namespace new_tests.legacy__lean_tests__lib.floats

--  @fail(extraction): ssprove(HAX0001)
--  @fail(extraction): ssprove(HAX0001)
def N : f32 := (1.0 : f32)

--  @fail(extraction): ssprove(HAX0001)
@[spec]
def test (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let l0 : f64 := (1.0 : f64);
  let l1 : f64 := (0.9 : f64);
  let l2 : f32 := (5.0 : f32);
  let l5 : f32 := N;
  (pure rust_primitives.hax.Tuple0.mk)

--  @fail(extraction): ssprove(HAX0001)
@[spec]
def f (x : f64) (y : f32) : RustM f32 := do (pure y)

end new_tests.legacy__lean_tests__lib.floats


namespace new_tests.legacy__lean_tests__lib.ite

@[spec]
def test1 (_ : rust_primitives.hax.Tuple0) : RustM i32 := do
  let x : i32 ← if true then do (pure (0 : i32)) else do (pure (1 : i32));
  if false then do (pure (2 : i32)) else do (pure (3 : i32))

@[spec]
def test2 (b : Bool) : RustM i32 := do
  let x : i32 ← if b then do (pure (0 : i32)) else do (pure (9 : i32));
  let y : i32 := (0 : i32);
  let y : i32 ←
    if true then do
      ((← (y +? x)) +? (1 : i32))
    else do
      ((← (y -? x)) -? (1 : i32));
  if b then do
    let z : i32 ← (y +? y);
    ((← (z +? y)) +? x)
  else do
    let z : i32 ← (y -? x);
    ((← (z +? y)) +? x)

end new_tests.legacy__lean_tests__lib.ite


namespace new_tests.legacy__lean_tests__lib.loops

@[spec]
def loop1 (_ : rust_primitives.hax.Tuple0) : RustM u32 := do
  let x : u32 := (0 : u32);
  let x : u32 ←
    (rust_primitives.hax.folds.fold_range
      (1 : u32)
      (10 : u32)
      (fun x _ => (do (pure true) : RustM Bool))
      x
      (fun x i => (do (x +? i) : RustM u32)));
  (pure x)

@[spec]
def loop2 (_ : rust_primitives.hax.Tuple0) : RustM u32 := do
  let x : u32 := (0 : u32);
  match
    (← (rust_primitives.hax.folds.fold_range_return
      (1 : u32)
      (10 : u32)
      (fun x _ => (do (pure true) : RustM Bool))
      x
      (fun x i =>
        (do
        if (← (i ==? (5 : u32))) then do
          (pure (core_models.ops.control_flow.ControlFlow.Break
            (core_models.ops.control_flow.ControlFlow.Break x)))
        else do
          (pure (core_models.ops.control_flow.ControlFlow.Continue
            (← (x +? i)))) :
        RustM
        (core_models.ops.control_flow.ControlFlow
          (core_models.ops.control_flow.ControlFlow
            u32
            (rust_primitives.hax.Tuple2 rust_primitives.hax.Tuple0 u32))
          u32)))))
  with
    | (core_models.ops.control_flow.ControlFlow.Break  ret) => do (pure ret)
    | (core_models.ops.control_flow.ControlFlow.Continue  x) => do (pure x)

--  For-loop with a spec
def for_loop_with_spec (y : u64) : RustM u64 := do
  let x : u64 := y;
  let x : u64 ←
    (rust_primitives.hax.folds.fold_range
      (0 : u64)
      y
      (fun x i => (do (x >? (0 : u64)) : RustM Bool))
      x
      (fun x i =>
        (do
        if (← ((← (x %? (5 : u64))) ==? (0 : u64))) then do
          let x : u64 := (200 : u64);
          (pure x)
        else do
          let x : u64 ← (x %? (5 : u64));
          (pure x) :
        RustM u64)));
  (pure x)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def for_loop_with_spec.spec (y : u64) :
    Spec
      (requires := do (y >? (0 : u64)))
      (ensures := fun res => do (res >? (0 : u64)))
      (for_loop_with_spec (y : u64)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [for_loop_with_spec] <;> bv_decide
}

--  while-loop
--  @fail(extraction): coq(HAX0001, HAX0001), ssprove(HAX0001)
def while_loop1 (s : u32) : RustM u32 := do
  let x : u32 := s;
  let x : u32 ←
    (rust_primitives.hax.while_loop
      (fun x => (do (pure true) : RustM Bool))
      (fun x => (do (x >? (0 : u32)) : RustM Bool))
      (fun x =>
        (do (rust_primitives.hax.int.from_machine x) : RustM hax_lib.int.Int))
      x
      (fun x => (do let x : u32 ← (x -? (1 : u32)); (pure x) : RustM u32)));
  (pure x)

set_option hax_mvcgen.specset "int" in
@[hax_spec]
def while_loop1.spec (s : u32) :
    Spec
      (requires := do pure True)
      (ensures := fun r => do (r ==? (0 : u32)))
      (while_loop1 (s : u32)) := {
  pureRequires := by hax_construct_pure <;> grind
  pureEnsures := by hax_construct_pure <;> grind
  contract := by hax_mvcgen [while_loop1] <;> grind
}

end new_tests.legacy__lean_tests__lib.loops


namespace new_tests.legacy__lean_tests__lib.loops.errors

inductive Error : Type
| Foo : Error
| Bar : u32 -> Error

@[spec]
def loop3 (_ : rust_primitives.hax.Tuple0) :
    RustM (core_models.result.Result u32 Error) := do
  let x : u32 := (0 : u32);
  let _end : u32 := (10 : u32);
  match
    (← (rust_primitives.hax.folds.fold_range_return
      (1 : u32)
      _end
      (fun x _ => (do (pure true) : RustM Bool))
      x
      (fun x i =>
        (do
        if (← (i ==? (5 : u32))) then do
          (pure (core_models.ops.control_flow.ControlFlow.Break
            (core_models.ops.control_flow.ControlFlow.Break
              (core_models.result.Result.Err Error.Foo))))
        else do
          (pure (core_models.ops.control_flow.ControlFlow.Continue
            (← (x +? (5 : u32))))) :
        RustM
        (core_models.ops.control_flow.ControlFlow
          (core_models.ops.control_flow.ControlFlow
            (core_models.result.Result u32 Error)
            (rust_primitives.hax.Tuple2 rust_primitives.hax.Tuple0 u32))
          u32)))))
  with
    | (core_models.ops.control_flow.ControlFlow.Break  ret) => do (pure ret)
    | (core_models.ops.control_flow.ControlFlow.Continue  x) => do
      (pure (core_models.result.Result.Ok x))

@[spec]
def loop4 (_ : rust_primitives.hax.Tuple0) :
    RustM
    (core_models.result.Result (rust_primitives.hax.Tuple2 u32 u32) Error)
    := do
  let e : u32 := (0 : u32);
  let f : (rust_primitives.hax.Tuple0 -> RustM u32) :=
    (fun ⟨⟩ => (do (pure (42 : u32)) : RustM u32));
  match
    (← (rust_primitives.hax.folds.fold_range_return
      (0 : u32)
      (← (core_models.ops.function.Fn.call
        (rust_primitives.hax.Tuple0 -> RustM u32)
        (rust_primitives.hax.Tuple1 rust_primitives.hax.Tuple0)
        f
        (rust_primitives.hax.Tuple1.mk rust_primitives.hax.Tuple0.mk)))
      (fun e _ => (do (pure true) : RustM Bool))
      e
      (fun e i =>
        (do
        if (← (i >? (10 : u32))) then do
          (pure (core_models.ops.control_flow.ControlFlow.Break
            (core_models.ops.control_flow.ControlFlow.Break
              (core_models.result.Result.Err (Error.Bar e)))))
        else do
          (pure (core_models.ops.control_flow.ControlFlow.Continue
            (← (e +? i)))) :
        RustM
        (core_models.ops.control_flow.ControlFlow
          (core_models.ops.control_flow.ControlFlow
            (core_models.result.Result
              (rust_primitives.hax.Tuple2 u32 u32)
              Error)
            (rust_primitives.hax.Tuple2 rust_primitives.hax.Tuple0 u32))
          u32)))))
  with
    | (core_models.ops.control_flow.ControlFlow.Break  ret) => do (pure ret)
    | (core_models.ops.control_flow.ControlFlow.Continue  e) => do
      (pure (core_models.result.Result.Ok (rust_primitives.hax.Tuple2.mk e e)))

end new_tests.legacy__lean_tests__lib.loops.errors


namespace new_tests.legacy__lean_tests__lib.matching

--  @fail(extraction): ssprove(HAX0001)
@[spec]
def test_const_matching (x : u32) (c : Char) (s : String) (b : Bool) :
    RustM u32 := do
  let x : u32 ←
    match x with | 0 => do (pure (42 : u32)) | _ => do (pure (0 : u32));
  let c : u32 ←
    match c with | 'a' => do (pure (42 : u32)) | _ => do (pure (0 : u32));
  let s : u32 ←
    match s with | "Hello" => do (pure (42 : u32)) | _ => do (pure (0 : u32));
  let b : u32 ←
    match b with | true => do (pure (42 : u32)) | false => do (pure (0 : u32));
  ((← ((← (x +? c)) +? s)) +? b)

--  @fail(extraction): fstar(HAX0008), ssprove(HAX0008), coq(HAX0008)
@[spec]
def test_binding_subpattern_matching
    (x : (rust_primitives.hax.Tuple2 u8 (rust_primitives.hax.Tuple2 u8 u8))) :
    RustM u8 := do
  match x with
    | ⟨0, pair@⟨a, b⟩⟩ => do
      ((← ((← (a +? b)) +? (rust_primitives.hax.Tuple2._0 pair)))
        +? (rust_primitives.hax.Tuple2._1 pair))
    | _ => do (pure (0 : u8))

inductive test_ellipsis_records.E : Type
| C (f1 : u8) (f2 : u8) (f3 : u8) (f4 : u8) : test_ellipsis_records.E

@[spec]
def test_ellipsis_records (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let c : test_ellipsis_records.E :=
    (test_ellipsis_records.E.C
      (f1 := (1 : u8))
      (f2 := (2 : u8))
      (f3 := (3 : u8))
      (f4 := (4 : u8)));
  let _ ←
    match c with | (test_ellipsis_records.E.C _ ..) => do (hax_lib.assert true);
  let _ ←
    match c with
      | (test_ellipsis_records.E.C (f1 := f1) ..) => do
        (hax_lib.assert (← (f1 ==? (1 : u8))));
  let _ ←
    match c with
      | (test_ellipsis_records.E.C (f1 := f1) (f2 := f2) ..) => do
        (hax_lib.assert (← ((← (f1 ==? (1 : u8))) &&? (← (f2 ==? (2 : u8))))));
  let _ ←
    match c with
      | (test_ellipsis_records.E.C (f2 := f2) (f4 := f4) ..) => do
        (hax_lib.assert (← ((← (f2 ==? (2 : u8))) &&? (← (f4 ==? (4 : u8))))));
  let _ ←
    match c with
      | (test_ellipsis_records.E.C  (f1 := f1) (f2 := f2) (f3 := f3) (f4 := f4))
        => do
        (hax_lib.assert
          (← ((← ((← ((← (f1 ==? (1 : u8))) &&? (← (f2 ==? (2 : u8)))))
              &&? (← (f3 ==? (3 : u8)))))
            &&? (← (f4 ==? (4 : u8))))));
  (pure rust_primitives.hax.Tuple0.mk)

structure test_ellipsis_structs.S where
  f1 : u8
  f2 : u8
  f3 : u8
  f4 : u8

@[spec]
def test_ellipsis_structs (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let c : test_ellipsis_structs.S :=
    (test_ellipsis_structs.S.mk
      (f1 := (1 : u8))
      (f2 := (2 : u8))
      (f3 := (3 : u8))
      (f4 := (4 : u8)));
  let _ ← match c with | _ => do (hax_lib.assert true);
  let _ ←
    match c with | {f1 := f1, ..} => do (hax_lib.assert (← (f1 ==? (1 : u8))));
  let _ ←
    match c with
      | {f1 := f1, f2 := f2, ..} => do
        (hax_lib.assert (← ((← (f1 ==? (1 : u8))) &&? (← (f2 ==? (2 : u8))))));
  let _ ←
    match c with
      | {f2 := f2, f4 := f4, ..} => do
        (hax_lib.assert (← ((← (f2 ==? (2 : u8))) &&? (← (f4 ==? (4 : u8))))));
  let _ ←
    match c with
      | {f1 := f1, f2 := f2, f3 := f3, f4 := f4} => do
        (hax_lib.assert
          (← ((← ((← ((← (f1 ==? (1 : u8))) &&? (← (f2 ==? (2 : u8)))))
              &&? (← (f3 ==? (3 : u8)))))
            &&? (← (f4 ==? (4 : u8))))));
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def test_ellipsis_bare_tuples (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let t : (rust_primitives.hax.Tuple4 u8 u8 u8 u8) :=
    (rust_primitives.hax.Tuple4.mk (1 : u8) (2 : u8) (3 : u8) (4 : u8));
  let _ ← match t with | ⟨_, _, _, _⟩ => do (hax_lib.assert true);
  let _ ←
    match t with | ⟨a, _, _, _⟩ => do (hax_lib.assert (← (a ==? (1 : u8))));
  let _ ←
    match t with
      | ⟨a, b, _, _⟩ => do
        (hax_lib.assert (← ((← (a ==? (1 : u8))) &&? (← (b ==? (2 : u8))))));
  let _ ←
    match t with | ⟨_, _, _, d⟩ => do (hax_lib.assert (← (d ==? (4 : u8))));
  let _ ←
    match t with
      | ⟨_, _, c, d⟩ => do
        (hax_lib.assert (← ((← (c ==? (3 : u8))) &&? (← (d ==? (4 : u8))))));
  let _ ←
    match t with
      | ⟨a, _, _, d⟩ => do
        (hax_lib.assert (← ((← (a ==? (1 : u8))) &&? (← (d ==? (4 : u8))))));
  let _ ←
    match t with
      | ⟨a, b, c, d⟩ => do
        (hax_lib.assert
          (← ((← ((← ((← (a ==? (1 : u8))) &&? (← (b ==? (2 : u8)))))
              &&? (← (c ==? (3 : u8)))))
            &&? (← (d ==? (4 : u8))))));
  (pure rust_primitives.hax.Tuple0.mk)

inductive test_ellipsis_tuples.F : Type
| D : u8 -> u8 -> u8 -> u8 -> test_ellipsis_tuples.F

@[spec]
def test_ellipsis_tuples (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let d : test_ellipsis_tuples.F :=
    (test_ellipsis_tuples.F.D (1 : u8) (2 : u8) (3 : u8) (4 : u8));
  let _ ←
    match d with
      | (test_ellipsis_tuples.F.D  _ _ _ _) => do (hax_lib.assert true);
  let _ ←
    match d with
      | (test_ellipsis_tuples.F.D  a _ _ _) => do
        (hax_lib.assert (← (a ==? (1 : u8))));
  let _ ←
    match d with
      | (test_ellipsis_tuples.F.D  a b _ _) => do
        (hax_lib.assert (← ((← (a ==? (1 : u8))) &&? (← (b ==? (2 : u8))))));
  let _ ←
    match d with
      | (test_ellipsis_tuples.F.D  _ _ _ d) => do
        (hax_lib.assert (← (d ==? (4 : u8))));
  let _ ←
    match d with
      | (test_ellipsis_tuples.F.D  _ _ c d) => do
        (hax_lib.assert (← ((← (c ==? (3 : u8))) &&? (← (d ==? (4 : u8))))));
  let _ ←
    match d with
      | (test_ellipsis_tuples.F.D  a _ _ d) => do
        (hax_lib.assert (← ((← (a ==? (1 : u8))) &&? (← (d ==? (4 : u8))))));
  let _ ←
    match d with
      | (test_ellipsis_tuples.F.D  a b c d) => do
        (hax_lib.assert
          (← ((← ((← ((← (a ==? (1 : u8))) &&? (← (b ==? (2 : u8)))))
              &&? (← (c ==? (3 : u8)))))
            &&? (← (d ==? (4 : u8))))));
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.legacy__lean_tests__lib.matching


namespace new_tests.legacy__lean_tests__lib.monadic

structure S where
  f : u32

@[spec]
def test (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ := (9 : i32);
  let _ ← ((9 : i32) +? (9 : i32));
  let _ := (S.mk (f := (9 : u32)));
  let _ := (S.mk (f := (← ((9 : u32) +? (9 : u32)))));
  let _ := (S.f (S.mk (f := (← ((9 : u32) +? (9 : u32))))));
  let _ ← ((S.f (S.mk (f := (← ((9 : u32) +? (9 : u32)))))) +? (9 : u32));
  let _ ←
    if true then do ((3 : i32) +? (4 : i32)) else do ((3 : i32) -? (4 : i32));
  let _ ←
    if (← ((← ((9 : i32) +? (9 : i32))) ==? (0 : i32))) then do
      ((3 : i32) +? (4 : i32))
    else do
      ((3 : i32) -? (4 : i32));
  let _ ←
    if true then do
      let x : i32 := (9 : i32);
      let _ ← ((3 : i32) +? x);
      (pure rust_primitives.hax.Tuple0.mk)
    else do
      let y : i32 := (19 : i32);
      let _ ← ((← ((3 : i32) +? y)) -? (4 : i32));
      (pure rust_primitives.hax.Tuple0.mk);
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.legacy__lean_tests__lib.monadic


namespace new_tests.legacy__lean_tests__lib.monadic.trait_constants

class Foo.AssociatedTypes (Self : Type) where

class Foo (Self : Type)
  [associatedTypes : outParam (Foo.AssociatedTypes (Self : Type))]
  where
  F (Self) : u32

class Bar.AssociatedTypes (Self : Type) where

class Bar (Self : Type)
  [associatedTypes : outParam (Bar.AssociatedTypes (Self : Type))]
  where
  B (Self) : u32

structure Baz where
  -- no fields

def Impl.F_hoisted : u32 := (1 : u32)

@[reducible] instance Impl.AssociatedTypes : Foo.AssociatedTypes Baz where

instance Impl : Foo Baz where
  F := (Impl.F_hoisted)

def Impl_1.B_hoisted : u32 :=
  RustM.of_isOk (do ((Foo.F Baz) -? (1 : u32))) (by rfl)

@[reducible] instance Impl_1.AssociatedTypes : Bar.AssociatedTypes Baz where

instance Impl_1 : Bar Baz where
  B := (Impl_1.B_hoisted)

end new_tests.legacy__lean_tests__lib.monadic.trait_constants


namespace new_tests.legacy__lean_tests__lib.nested_control_flow

@[spec]
def nested_control_flow (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let x1 : i32 ←
    ((1 : i32)
      +? (← if true then do (pure (0 : i32)) else do (pure (1 : i32))));
  let x2 : i32 ←
    ((1 : i32)
      +? (← match (rust_primitives.hax.Tuple2.mk (1 : i32) (2 : i32)) with
        | _ => do (pure (0 : i32))));
  let x : i32 := (9 : i32);
  let x3 : i32 ← ((1 : i32) +? (← (x +? (1 : i32))));
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def explicit_hoisting (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let x1_tmp : i32 ← if true then do (pure (0 : i32)) else do (pure (1 : i32));
  let x1 : i32 ← ((1 : i32) +? x1_tmp);
  let x2_tmp : i32 ←
    match (rust_primitives.hax.Tuple2.mk (1 : i32) (2 : i32)) with
      | _ => do (pure (0 : i32));
  let x2 : i32 ← ((1 : i32) +? x2_tmp);
  let x3_tmp_x : i32 := (9 : i32);
  let x3_tmp : i32 ← (x3_tmp_x +? (1 : i32));
  let x3 : i32 ← ((1 : i32) +? x3_tmp);
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def complex_nesting (_ : rust_primitives.hax.Tuple0) :
    RustM
    (rust_primitives.hax.Tuple2
      rust_primitives.hax.Tuple0
      rust_primitives.hax.Tuple0)
    := do
  let x1 : i32 ←
    if true then do
      let y : i32 ←
        if false then do
          let z : i32 ←
            match rust_primitives.hax.Tuple0.mk with | _ => do (pure (9 : i32));
          let z : i32 ← ((1 : i32) +? z);
          (z +? (1 : i32))
        else do
          let z : i32 := (9 : i32);
          let z : i32 ← (z +? (1 : i32));
          (pure z);
      let y : i32 ← (y +? (1 : i32));
      (y +? (1 : i32))
    else do
      (pure (0 : i32));
  let x1 : i32 ← (x1 +? (1 : i32));
  let x2 : i32 ←
    match (core_models.option.Option.Some (89 : i32)) with
      | (core_models.option.Option.Some  a) => do
        let y : i32 ← ((1 : i32) +? a);
        let y : i32 ← (y +? (1 : i32));
        if (← (y ==? (0 : i32))) then do
          let z : i32 := (9 : i32);
          let z : i32 ← ((← (z +? y)) +? (1 : i32));
          (pure z)
        else do
          (pure (10 : i32))
      | (core_models.option.Option.None ) => do
        let y : i32 ←
          if false then do
            (pure (9 : i32))
          else do
            let z : i32 := (9 : i32);
            let z : i32 ← (z +? (1 : i32));
            (z +? (9 : i32));
        let y : i32 ← (y +? (1 : i32));
        (pure y);
  (pure (rust_primitives.hax.Tuple2.mk
    rust_primitives.hax.Tuple0.mk
    rust_primitives.hax.Tuple0.mk))

end new_tests.legacy__lean_tests__lib.nested_control_flow


namespace new_tests.legacy__lean_tests__lib.opaque

opaque an_opaque_fn (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0

class T.AssociatedTypes (Self : Type) where
  A : Type

attribute [reducible] T.AssociatedTypes.A

abbrev T.A :=
  T.AssociatedTypes.A

class T (Self : Type)
  [associatedTypes : outParam (T.AssociatedTypes (Self : Type))]
  where
  f (Self) : (rust_primitives.hax.Tuple0 -> RustM rust_primitives.hax.Tuple0)

structure S where
  -- no fields

@[instance] opaque Impl.AssociatedTypes :
  T.AssociatedTypes S :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl : T S := by constructor <;> exact Inhabited.default

opaque OpaqueStruct : Type

end new_tests.legacy__lean_tests__lib.opaque


namespace new_tests.legacy__lean_tests__lib.specs

def test (x : u8) : RustM u8 := do (pure x)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def test.spec (x : u8) :
    Spec
      (requires := do (x >? (0 : u8)))
      (ensures := fun r => do (r ==? x))
      (test (x : u8)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [test] <;> bv_decide
}

def use_previous_result (x : u8) : RustM u8 := do (test x)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def use_previous_result.spec (x : u8) :
    Spec
      (requires := do (x >? (0 : u8)))
      (ensures := fun r => do (r ==? x))
      (use_previous_result (x : u8)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [use_previous_result] <;> bv_decide
}

def test_proof (x : u8) : RustM u8 := do (pure x)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def test_proof.spec (x : u8) :
    Spec
      (requires := do (x >? (0 : u8)))
      (ensures := fun r => do (r ==? x))
      (test_proof (x : u8)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by unfold lean_tests.specs.test_proof; hax_bv_decide
}

def square (x : u8) : RustM u8 := do (x *? x)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def square.spec (x : u8) :
    Spec
      (requires := do (x <? (16 : u8)))
      (ensures := fun res => do (res >=? x))
      (square (x : u8)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [square] <;> bv_decide
}

def forall_and_exists (x : u8) : RustM u8 := do (pure x)

set_option hax_mvcgen.specset "int" in
@[hax_spec]
def forall_and_exists.spec (x : u8) :
    Spec
      (requires := do
        (hax_lib.prop.constructors.forall
          (fun i =>
            (do
            (hax_lib.prop.constructors.implies
              (← (hax_lib.prop.constructors.from_bool (← (i <? (20 : u8)))))
              (← (hax_lib.prop.constructors.from_bool (← (x >? i))))) :
            RustM hax_lib.prop.Prop))))
      (ensures := fun
          r => do
          (hax_lib.prop.constructors.not
            (← (hax_lib.prop.constructors.exists
              (fun i =>
                (do
                (hax_lib.prop.constructors.not
                  (← (hax_lib.prop.constructors.implies
                    (← (hax_lib.prop.constructors.from_bool
                      (← (i <? (20 : u8)))))
                    (← (hax_lib.prop.constructors.from_bool (← (r >? i))))))) :
                RustM hax_lib.prop.Prop))))))
      (forall_and_exists (x : u8)) := {
  pureRequires := by hax_construct_pure <;> grind
  pureEnsures := by hax_construct_pure <;> grind
  contract := by hax_mvcgen [forall_and_exists] <;> grind
}

--  Test function without arguments
--  https://github.com/pereq-dev/hax/issues/1856
def fn_without_args (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def fn_without_args.spec (_ : rust_primitives.hax.Tuple0) :
    Spec
      (requires := do pure True)
      (ensures := fun _ => do (pure true))
      (fn_without_args ⟨⟩) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [fn_without_args] <;> bv_decide
}

end new_tests.legacy__lean_tests__lib.specs


namespace new_tests.legacy__lean_tests__lib.specs.issue_1852

structure T where
  -- no fields

@[spec]
def Impl.test (self : T) : RustM Bool := do (pure true)

def Impl.func (self : T) : RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def Impl.func.spec (self : T) :
    Spec
      (requires := do (Impl.test self))
      (ensures := fun _ => pure True)
      (Impl.func (self : T)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [Impl.func] <;> bv_decide
}

end new_tests.legacy__lean_tests__lib.specs.issue_1852


namespace new_tests.legacy__lean_tests__lib.specs

def custom_pure_proofs (x : u8) : RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def custom_pure_proofs.spec (x : u8) :
    Spec
      (requires := do (pure true))
      (ensures := fun r => do (pure true))
      (custom_pure_proofs (x : u8)) := {
  pureRequires := ⟨True, by mvcgen⟩
  pureEnsures := ⟨fun _ => True, by intros; mvcgen⟩
  contract := by hax_mvcgen [custom_pure_proofs] <;> bv_decide
}

end new_tests.legacy__lean_tests__lib.specs


namespace new_tests.legacy__lean_tests__lib.specs.issue_1945

def mktuple (a : i32) : RustM Bool := do let x : i32 := a; (a ==? (0 : i32))

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def mktuple.spec (a : i32) :
    Spec
      (requires := do let x : i32 := a; (a ==? (0 : i32)))
      (ensures := fun _ => pure True)
      (mktuple (a : i32)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [mktuple] <;> bv_decide
}

end new_tests.legacy__lean_tests__lib.specs.issue_1945


namespace new_tests.legacy__lean_tests__lib.structs

structure T0 where
  -- no fields

structure T1 (A : Type) where
  _0 : A

structure T2 (A : Type) (B : Type) where
  _0 : A
  _1 : B

structure T3 (A : Type) (B : Type) (C : Type) where
  _0 : A
  _1 : B
  _2 : C

structure T3p (A : Type) (B : Type) (C : Type) where
  _0 : A
  _1 : (T2 B C)

@[spec]
def tuple_structs (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let t0 : T0 := T0.mk;
  let t1 : (T1 i32) := (T1.mk (1 : i32));
  let t2 : (T2 i32 i32) := (T2.mk (1 : i32) (2 : i32));
  let t3 : (T3 T0 (T1 i32) (T2 i32 i32)) :=
    (T3.mk T0.mk (T1.mk (1 : i32)) (T2.mk (1 : i32) (2 : i32)));
  let t3p : (T3p T0 (T1 i32) (T2 i32 i32)) :=
    (T3p.mk T0.mk (T2.mk (T1.mk (1 : i32)) (T2.mk (1 : i32) (2 : i32))));
  let ⟨⟩ := t0;
  let ⟨u1⟩ := t1;
  let ⟨u2, u3⟩ := t2;
  let ⟨⟨⟩, ⟨_⟩, ⟨_, _⟩⟩ := t3;
  let ⟨⟨⟩, ⟨⟨_⟩, ⟨_, _⟩⟩⟩ := t3p;
  let _ := (T1._0 t1);
  let _ := (T2._0 t2);
  let _ := (T2._1 t2);
  let _ := (T3._0 t3);
  let _ := (T3._1 t3);
  let _ := (T3._2 t3);
  let _ := (T2._1 (T3._2 t3));
  let _ := (T3p._0 t3p);
  let _ := (T3p._1 t3p);
  let _ := (T2._0 (T2._1 (T3p._1 t3p)));
  let _ := (T2._0 (T3p._1 t3p));
  let _ := (T2._1 (T3p._1 t3p));
  let _ ← match t0 with | ⟨⟩ => do (pure rust_primitives.hax.Tuple0.mk);
  let _ ← match t1 with | ⟨u1⟩ => do (pure rust_primitives.hax.Tuple0.mk);
  let _ ← match t2 with | ⟨u2, u3⟩ => do (pure rust_primitives.hax.Tuple0.mk);
  let _ ←
    match t3 with
      | ⟨⟨⟩, ⟨u1⟩, ⟨u2, u3⟩⟩ => do (pure rust_primitives.hax.Tuple0.mk);
  let _ ←
    match t3p with
      | ⟨⟨⟩, ⟨⟨u1⟩, ⟨u2, u3⟩⟩⟩ => do (pure rust_primitives.hax.Tuple0.mk);
  (pure rust_primitives.hax.Tuple0.mk)

structure S1 where
  f1 : usize
  f2 : usize

structure S2 where
  f1 : S1
  f2 : usize

structure S3 where
  _end : usize
  _def : usize
  _theorem : usize
  _structure : usize
  _inductive : usize

--  @fail(extraction): ssprove(HAX0001)
@[spec]
def normal_structs (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let s1 : S1 := (S1.mk (f1 := (0 : usize)) (f2 := (1 : usize)));
  let s2 : S2 :=
    (S2.mk
      (f1 := (S1.mk (f1 := (2 : usize)) (f2 := (3 : usize))))
      (f2 := (4 : usize)));
  let s3 : S3 :=
    (S3.mk
      (_end := (0 : usize))
      (_def := (0 : usize))
      (_theorem := (0 : usize))
      (_structure := (0 : usize))
      (_inductive := (0 : usize)));
  let {f1 := f1, f2 := f2} := s1;
  let {f1 := f1, f2 := other_name_for_f2} := s1;
  let {f1 := {f1 := f1, f2 := f2}, f2 := other_name_for_f2} := s2;
  let
    {_end := _end,
     _def := _def,
     _theorem := _theorem,
     _structure := _structure,
     _inductive := _inductive} :=
    s3;
  let _ := (rust_primitives.hax.Tuple2.mk (S1.f1 s1) (S1.f2 s1));
  let _ :=
    (rust_primitives.hax.Tuple8.mk
      (S1.f1 s1)
      (S1.f2 s1)
      (S1.f1 (S2.f1 s2))
      (S1.f2 (S2.f1 s2))
      (S2.f2 s2)
      (S3._end s3)
      (S3._def s3)
      (S3._theorem s3));
  let _ ←
    match s1 with
      | {f1 := f1, f2 := f2} => do (pure rust_primitives.hax.Tuple0.mk);
  let _ ←
    match s2 with
      | {f1 := {f1 := f1, f2 := other_name_for_f2}, f2 := f2} => do
        (pure rust_primitives.hax.Tuple0.mk);
  match s3 with
    | {_end := _end,
       _def := _def,
       _theorem := _theorem,
       _structure := _structure,
       _inductive := _inductive} => do
      (pure rust_primitives.hax.Tuple0.mk)

end new_tests.legacy__lean_tests__lib.structs


namespace new_tests.legacy__lean_tests__lib.structs.miscellaneous

structure S where
  f : i32

@[spec]
def test_tuples (_ : rust_primitives.hax.Tuple0) :
    RustM (rust_primitives.hax.Tuple2 i32 i32) := do
  let lit : i32 := (1 : i32);
  let constr : S := (S.mk (f := (42 : i32)));
  let proj : i32 := (S.f constr);
  let ite : (rust_primitives.hax.Tuple2 i32 i32) ←
    if true then do
      (pure (rust_primitives.hax.Tuple2.mk (1 : i32) (2 : i32)))
    else do
      let z : i32 ← ((1 : i32) +? (2 : i32));
      (pure (rust_primitives.hax.Tuple2.mk z z));
  (pure (rust_primitives.hax.Tuple2.mk (1 : i32) (2 : i32)))

end new_tests.legacy__lean_tests__lib.structs.miscellaneous


namespace new_tests.legacy__lean_tests__lib.structs.base_expressions

structure S where
  f1 : u32
  f2 : u32
  f3 : u32

@[spec]
def test (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let s1 : S := (S.mk (f1 := (1 : u32)) (f2 := (2 : u32)) (f3 := (3 : u32)));
  let _ := {s1 with f1 := (0 : u32)};
  let _ := {s1 with f2 := (0 : u32)};
  let _ := {s1 with f3 := (0 : u32)};
  let _ := {s1 with f1 := (0 : u32), f2 := (1 : u32)};
  let _ := {s1 with f2 := (0 : u32), f3 := (1 : u32)};
  let _ := {s1 with f3 := (0 : u32), f1 := (2 : u32)};
  let _ := {s1 with f1 := (0 : u32), f2 := (1 : u32), f3 := (0 : u32)};
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.legacy__lean_tests__lib.structs.base_expressions


namespace new_tests.legacy__lean_tests__lib.traits.basic

class T1.AssociatedTypes (Self : Type) where

class T1 (Self : Type)
  [associatedTypes : outParam (T1.AssociatedTypes (Self : Type))]
  where
  f1 (Self) : (Self -> RustM usize)
  f2 (Self) : (Self -> Self -> RustM usize)

structure S where
  -- no fields

@[spec]
def Impl.f1_hoisted (self : S) : RustM usize := do (pure (42 : usize))

@[spec]
def Impl.f2_hoisted (self : S) (other : S) : RustM usize := do
  (pure (43 : usize))

@[reducible] instance Impl.AssociatedTypes : T1.AssociatedTypes S where

instance Impl : T1 S where
  f1 := (Impl.f1_hoisted)
  f2 := (Impl.f2_hoisted)

@[spec]
def f
    (T : Type)
    [trait_constr_f_associated_type_i0 : T1.AssociatedTypes T]
    [trait_constr_f_i0 : T1 T ]
    (x : T) :
    RustM usize := do
  ((← (T1.f1 T x)) +? (← (T1.f2 T x x)))

end new_tests.legacy__lean_tests__lib.traits.basic


namespace new_tests.legacy__lean_tests__lib.traits.bounds

class T1.AssociatedTypes (Self : Type) where

class T1 (Self : Type)
  [associatedTypes : outParam (T1.AssociatedTypes (Self : Type))]
  where
  f1 (Self) : (Self -> RustM usize)

class T2.AssociatedTypes (Self : Type) where

class T2 (Self : Type)
  [associatedTypes : outParam (T2.AssociatedTypes (Self : Type))]
  where
  f2 (Self) : (Self -> RustM usize)

class Test.AssociatedTypes (Self : Type) (T : Type) where
  [trait_constr_Test_i0 : T2.AssociatedTypes Self]
  [trait_constr_Test_i1 : T1.AssociatedTypes T]

attribute [instance_reducible, instance]
  Test.AssociatedTypes.trait_constr_Test_i0

attribute [instance_reducible, instance]
  Test.AssociatedTypes.trait_constr_Test_i1

class Test (Self : Type) (T : Type)
  [associatedTypes : outParam (Test.AssociatedTypes (Self : Type) (T : Type))]
  where
  [trait_constr_Test_i0 : T2 Self]
  [trait_constr_Test_i1 : T1 T]
  f_test (Self) (T) : (Self -> T -> RustM usize)

attribute [instance_reducible, instance] Test.trait_constr_Test_i0

attribute [instance_reducible, instance] Test.trait_constr_Test_i1

structure S1 where
  -- no fields

@[spec]
def Impl.f1_hoisted (self : S1) : RustM usize := do (pure (0 : usize))

@[reducible] instance Impl.AssociatedTypes : T1.AssociatedTypes S1 where

instance Impl : T1 S1 where
  f1 := (Impl.f1_hoisted)

structure S2 where
  -- no fields

@[spec]
def Impl_1.f2_hoisted (self : S2) : RustM usize := do (pure (1 : usize))

@[reducible] instance Impl_1.AssociatedTypes : T2.AssociatedTypes S2 where

instance Impl_1 : T2 S2 where
  f2 := (Impl_1.f2_hoisted)

@[spec]
def Impl_2.f_test_hoisted (self : S2) (x : S1) : RustM usize := do
  ((← ((← (T1.f1 S1 x)) +? (← (T2.f2 S2 self)))) +? (1 : usize))

@[reducible] instance Impl_2.AssociatedTypes : Test.AssociatedTypes S2 S1 where

instance Impl_2 : Test S2 S1 where
  f_test := (Impl_2.f_test_hoisted)

@[spec]
def test (x1 : S1) (x2 : S2) : RustM usize := do
  ((← (Test.f_test S2 S1 x2 x1)) +? (← (T1.f1 S1 x1)))

end new_tests.legacy__lean_tests__lib.traits.bounds


namespace new_tests.legacy__lean_tests__lib.traits.associated_types

class Foo.AssociatedTypes (Self : Type) (T : Type) where

class Foo (Self : Type) (T : Type)
  [associatedTypes : outParam (Foo.AssociatedTypes (Self : Type) (T : Type))]
  where

class Bar.AssociatedTypes (Self : Type) where

class Bar (Self : Type)
  [associatedTypes : outParam (Bar.AssociatedTypes (Self : Type))]
  where

structure S where
  -- no fields

@[spec]
def Impl.f_hoisted (self : S) (x : i32) : RustM i32 := do (pure (2121 : i32))

@[spec]
def Impl_1.f_hoisted (self : S) (x : S) : RustM usize := do (pure (21 : usize))

@[reducible] instance Impl_2.AssociatedTypes : Bar.AssociatedTypes i16 where

instance Impl_2 : Bar i16 where

@[reducible] instance Impl_3.AssociatedTypes (A : Type) :
  Foo.AssociatedTypes (rust_primitives.hax.Tuple2 u32 A) i16
  where

instance Impl_3 (A : Type) : Foo (rust_primitives.hax.Tuple2 u32 A) i16 where

@[spec]
def Impl_4.f_hoisted
    (A : Type)
    [trait_constr_f_hoisted_associated_type_i0 : Bar.AssociatedTypes A]
    [trait_constr_f_hoisted_i0 : Bar A ]
    (self : S)
    (x : i16)
    (y : (rust_primitives.hax.Tuple2 u32 A)) :
    RustM usize := do
  (pure (12 : usize))

class Chain0.AssociatedTypes (Self : Type) where

class Chain0 (Self : Type)
  [associatedTypes : outParam (Chain0.AssociatedTypes (Self : Type))]
  where

@[reducible] instance Impl_5.AssociatedTypes : Chain0.AssociatedTypes u8 where

instance Impl_5 : Chain0 u8 where

@[spec]
def Impl_8.f_hoisted (_ : rust_primitives.hax.Tuple0) : RustM u8 := do
  (pure (0 : u8))

end new_tests.legacy__lean_tests__lib.traits.associated_types


namespace new_tests.legacy__lean_tests__lib.traits.overlapping_methods

class T1.AssociatedTypes (Self : Type) where

class T1 (Self : Type)
  [associatedTypes : outParam (T1.AssociatedTypes (Self : Type))]
  where
  f (Self) : (Self -> RustM usize)

class T2.AssociatedTypes (Self : Type) where

class T2 (Self : Type)
  [associatedTypes : outParam (T2.AssociatedTypes (Self : Type))]
  where
  f (Self) : (Self -> RustM usize)

class T3.AssociatedTypes (Self : Type) where

class T3 (Self : Type)
  [associatedTypes : outParam (T3.AssociatedTypes (Self : Type))]
  where
  f (Self) : (Self -> RustM usize)

@[spec]
def Impl.f_hoisted (self : u32) : RustM usize := do (pure (0 : usize))

@[reducible] instance Impl.AssociatedTypes : T1.AssociatedTypes u32 where

instance Impl : T1 u32 where
  f := (Impl.f_hoisted)

@[spec]
def Impl_1.f_hoisted (self : u32) : RustM usize := do (pure (1 : usize))

@[reducible] instance Impl_1.AssociatedTypes : T2.AssociatedTypes u32 where

instance Impl_1 : T2 u32 where
  f := (Impl_1.f_hoisted)

@[spec]
def Impl_2.f_hoisted (self : u32) : RustM usize := do (pure (2 : usize))

@[reducible] instance Impl_2.AssociatedTypes : T3.AssociatedTypes u32 where

instance Impl_2 : T3 u32 where
  f := (Impl_2.f_hoisted)

@[spec]
def test (_ : rust_primitives.hax.Tuple0) : RustM usize := do
  let x : u32 := (9 : u32);
  ((← ((← (T1.f u32 x)) +? (← (T2.f u32 x)))) +? (← (T3.f u32 x)))

end new_tests.legacy__lean_tests__lib.traits.overlapping_methods


namespace new_tests.legacy__lean_tests__lib.traits.inheritance

class T1.AssociatedTypes (Self : Type) where

class T1 (Self : Type)
  [associatedTypes : outParam (T1.AssociatedTypes (Self : Type))]
  where
  f1 (Self) : (Self -> RustM usize)

class T2.AssociatedTypes (Self : Type) where

class T2 (Self : Type)
  [associatedTypes : outParam (T2.AssociatedTypes (Self : Type))]
  where
  f2 (Self) : (Self -> RustM usize)

class T3.AssociatedTypes (Self : Type) where
  [trait_constr_T3_i0 : T2.AssociatedTypes Self]
  [trait_constr_T3_i1 : T1.AssociatedTypes Self]

attribute [instance_reducible, instance] T3.AssociatedTypes.trait_constr_T3_i0

attribute [instance_reducible, instance] T3.AssociatedTypes.trait_constr_T3_i1

class T3 (Self : Type)
  [associatedTypes : outParam (T3.AssociatedTypes (Self : Type))]
  where
  [trait_constr_T3_i0 : T2 Self]
  [trait_constr_T3_i1 : T1 Self]
  f3 (Self) : (Self -> RustM usize)

attribute [instance_reducible, instance] T3.trait_constr_T3_i0

attribute [instance_reducible, instance] T3.trait_constr_T3_i1

class Tp1.AssociatedTypes (Self : Type) where

class Tp1 (Self : Type)
  [associatedTypes : outParam (Tp1.AssociatedTypes (Self : Type))]
  where
  f1 (Self) : (Self -> RustM usize)

class Tp2.AssociatedTypes (Self : Type) where
  [trait_constr_Tp2_i0 : Tp1.AssociatedTypes Self]
  [trait_constr_Tp2_i1 : T3.AssociatedTypes Self]

attribute [instance_reducible, instance] Tp2.AssociatedTypes.trait_constr_Tp2_i0

attribute [instance_reducible, instance] Tp2.AssociatedTypes.trait_constr_Tp2_i1

class Tp2 (Self : Type)
  [associatedTypes : outParam (Tp2.AssociatedTypes (Self : Type))]
  where
  [trait_constr_Tp2_i0 : Tp1 Self]
  [trait_constr_Tp2_i1 : T3 Self]
  fp2 (Self) : (Self -> RustM usize)

attribute [instance_reducible, instance] Tp2.trait_constr_Tp2_i0

attribute [instance_reducible, instance] Tp2.trait_constr_Tp2_i1

structure S where
  -- no fields

@[spec]
def Impl.f1_hoisted (self : S) : RustM usize := do (pure (1 : usize))

@[reducible] instance Impl.AssociatedTypes : T1.AssociatedTypes S where

instance Impl : T1 S where
  f1 := (Impl.f1_hoisted)

@[spec]
def Impl_1.f2_hoisted (self : S) : RustM usize := do (pure (2 : usize))

@[reducible] instance Impl_1.AssociatedTypes : T2.AssociatedTypes S where

instance Impl_1 : T2 S where
  f2 := (Impl_1.f2_hoisted)

@[spec]
def Impl_2.f3_hoisted (self : S) : RustM usize := do (pure (3 : usize))

@[reducible] instance Impl_2.AssociatedTypes : T3.AssociatedTypes S where

instance Impl_2 : T3 S where
  f3 := (Impl_2.f3_hoisted)

@[spec]
def Impl_3.f1_hoisted (self : S) : RustM usize := do (pure (10 : usize))

@[reducible] instance Impl_3.AssociatedTypes : Tp1.AssociatedTypes S where

instance Impl_3 : Tp1 S where
  f1 := (Impl_3.f1_hoisted)

@[spec]
def Impl_4.fp2_hoisted (self : S) : RustM usize := do
  ((← ((← ((← (Tp1.f1 S self)) +? (← (T1.f1 S self)))) +? (← (T2.f2 S self))))
    +? (← (T3.f3 S self)))

@[reducible] instance Impl_4.AssociatedTypes : Tp2.AssociatedTypes S where

instance Impl_4 : Tp2 S where
  fp2 := (Impl_4.fp2_hoisted)

@[spec]
def test (_ : rust_primitives.hax.Tuple0) : RustM usize := do
  let s : S := S.mk;
  ((← (T3.f3 S s)) +? (1 : usize))

end new_tests.legacy__lean_tests__lib.traits.inheritance


namespace new_tests.legacy__lean_tests__lib.traits.default

--  @fail(extraction): ssprove(HAX0008), coq(HAX0008), fstar(HAX0008)
class Easy.AssociatedTypes (Self : Type) where

class Easy (Self : Type)
  [associatedTypes : outParam (Easy.AssociatedTypes (Self : Type))]
  where
  dft (Self) (self : Self) :RustM usize := do (pure (32 : usize))

@[spec]
def Impl.dft_hoisted (self : usize) : RustM usize := do (self +? (1 : usize))

@[reducible] instance Impl.AssociatedTypes : Easy.AssociatedTypes usize where

instance Impl : Easy usize where
  dft := (Impl.dft_hoisted)

@[reducible] instance Impl_1.AssociatedTypes : Easy.AssociatedTypes u32 where

instance Impl_1 : Easy u32 where

--  @fail(extraction): ssprove(HAX0008), coq(HAX0008), fstar(HAX0008)
class T1.AssociatedTypes (Self : Type) where

class T1 (Self : Type)
  [associatedTypes : outParam (T1.AssociatedTypes (Self : Type))]
  where
  f1 (Self) : (Self -> RustM usize)
  f2 (Self) (self : Self) :RustM usize := do (pure (1 : usize))
  f3 (Self) (A : Type) (self : Self) (x : A) :RustM usize := do
    (pure (1 : usize))
  f4 (Self)
    (A : Type)
    [trait_constr_f4_associated_type_i1 : Easy.AssociatedTypes A]
    [trait_constr_f4_i1 : Easy A ] (self : Self) (x : A) :RustM usize := do
    ((← (Easy.dft A x)) +? (1 : usize))

structure S (A : Type) where
  _0 : usize
  _1 : A

@[spec]
def Impl_2.f1_hoisted (self : (S usize)) : RustM usize := do
  ((S._0 self) +? (S._1 self))

@[spec]
def Impl_2.f2_hoisted (self : (S usize)) : RustM usize := do (pure (S._1 self))

@[reducible] instance Impl_2.AssociatedTypes :
  T1.AssociatedTypes (S usize)
  where

instance Impl_2 : T1 (S usize) where
  f1 := (Impl_2.f1_hoisted)
  f2 := (Impl_2.f2_hoisted)

@[spec]
def Impl_3.f1_hoisted (self : (S Bool)) : RustM usize := do
  if (S._1 self) then do (pure (S._0 self)) else do (pure (9 : usize))

@[spec]
def Impl_3.f2_hoisted (self : (S Bool)) : RustM usize := do
  ((S._0 self) +? (1 : usize))

@[reducible] instance Impl_3.AssociatedTypes : T1.AssociatedTypes (S Bool) where

instance Impl_3 : T1 (S Bool) where
  f1 := (Impl_3.f1_hoisted)
  f2 := (Impl_3.f2_hoisted)

@[spec]
def Impl_4.f1_hoisted (self : (S alloc.string.String)) : RustM usize := do
  (pure (0 : usize))

@[reducible] instance Impl_4.AssociatedTypes :
  T1.AssociatedTypes (S alloc.string.String)
  where

instance Impl_4 : T1 (S alloc.string.String) where
  f1 := (Impl_4.f1_hoisted)

end new_tests.legacy__lean_tests__lib.traits.default


namespace new_tests.legacy__lean_tests__lib.traits.trait_level_args

class T1.AssociatedTypes (Self : Type) (A : Type) (B : Type) where

class T1 (Self : Type) (A : Type) (B : Type)
  [associatedTypes : outParam (T1.AssociatedTypes (Self : Type) (A : Type) (B :
      Type))]
  where
  f1 (Self) (A) (B) (C : Type) (D : Type) :
    (Self -> RustM rust_primitives.hax.Tuple0)
  f2 (Self) (A) (B) (C : Type) (D : Type) :
    (Self -> A -> RustM rust_primitives.hax.Tuple0)
  f3 (Self) (A) (B) (C : Type) (D : Type) :
    (Self -> A -> B -> RustM rust_primitives.hax.Tuple0)

@[spec]
def Impl.f1_hoisted (C : Type) (D : Type) (self : usize) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def Impl.f2_hoisted (C : Type) (D : Type) (self : usize) (x : u32) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def Impl.f3_hoisted (C : Type) (D : Type) (self : usize) (x : u32) (y : u64) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

@[reducible] instance Impl.AssociatedTypes :
  T1.AssociatedTypes usize u32 u64
  where

instance Impl : T1 usize u32 u64 where
  f1 := fun  (C : Type) (D : Type) => (Impl.f1_hoisted C D)
  f2 := fun  (C : Type) (D : Type) => (Impl.f2_hoisted C D)
  f3 := fun  (C : Type) (D : Type) => (Impl.f3_hoisted C D)

@[spec]
def test
    (A : Type)
    (B : Type)
    (C : Type)
    (D : Type)
    (U : Type)
    [trait_constr_test_associated_type_i0 : T1.AssociatedTypes U A B]
    [trait_constr_test_i0 : T1 U A B ]
    (x : U)
    (a : A)
    (b : B) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ ← (T1.f1 U A B C D x);
  let _ ← (T1.f2 U A B C D x a);
  let _ ← (T1.f3 U A B C D x a b);
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.legacy__lean_tests__lib.traits.trait_level_args


namespace new_tests.legacy__lean_tests__lib.traits.trait_with_constraints

class T1.AssociatedTypes (Self : Type) where

class T1 (Self : Type)
  [associatedTypes : outParam (T1.AssociatedTypes (Self : Type))]
  where

class T2.AssociatedTypes (Self : Type) where

class T2 (Self : Type)
  [associatedTypes : outParam (T2.AssociatedTypes (Self : Type))]
  where
  func (Self)
    [trait_constr_func_associated_type_i1 : T1.AssociatedTypes Self]
    [trait_constr_func_i1 : T1 Self ] :
    (Self -> RustM Bool)

@[spec]
def Impl.func_hoisted
    (A : Type)
    [trait_constr_func_hoisted_associated_type_i0 : T1.AssociatedTypes A]
    [trait_constr_func_hoisted_i0 : T1 A ]
    [trait_constr_func_hoisted_associated_type_i1 : T1.AssociatedTypes A]
    [trait_constr_func_hoisted_i1 : T1 A ]
    (self : A) :
    RustM Bool := do
  (pure true)

@[reducible] instance Impl.AssociatedTypes
  (A : Type)
  [trait_constr_Impl_associated_type_i0 : T1.AssociatedTypes A]
  [trait_constr_Impl_i0 : T1 A ] :
  T2.AssociatedTypes A
  where

instance Impl
  (A : Type)
  [trait_constr_Impl_associated_type_i0 : T1.AssociatedTypes A]
  [trait_constr_Impl_i0 : T1 A ] :
  T2 A
  where
  func := (Impl.func_hoisted A)

end new_tests.legacy__lean_tests__lib.traits.trait_with_constraints


namespace new_tests.legacy__lean_tests__lib.traits.associated_constant

class Foo.AssociatedTypes (Self : Type) where

class Foo (Self : Type)
  [associatedTypes : outParam (Foo.AssociatedTypes (Self : Type))]
  where
  f (Self) : Bool
  x (Self) :u8 := (0 : u8)

structure Bar where
  -- no fields

def Impl.f_hoisted : Bool := true

def Impl.x_hoisted : u8 := RustM.of_isOk (do ((1 : u8) +? (1 : u8))) (by rfl)

@[reducible] instance Impl.AssociatedTypes : Foo.AssociatedTypes Bar where

instance Impl : Foo Bar where
  f := (Impl.f_hoisted)
  x := (Impl.x_hoisted)

class Baz.AssociatedTypes (Self : Type) where

class Baz (Self : Type)
  [associatedTypes : outParam (Baz.AssociatedTypes (Self : Type))]
  where
  One (Self) :u32 := (1 : u32)

@[spec]
def foo
    (F : Type)
    [trait_constr_foo_associated_type_i0 : Baz.AssociatedTypes F]
    [trait_constr_foo_i0 : Baz F ]
    (n : u32) :
    RustM u32 := do
  (n +? (Baz.One F))

end new_tests.legacy__lean_tests__lib.traits.associated_constant


namespace new_tests.legacy__lean_tests__lib.traits.methods_hoisting

class T0.AssociatedTypes (Self : Type) where

class T0 (Self : Type)
  [associatedTypes : outParam (T0.AssociatedTypes (Self : Type))]
  where

@[spec]
def Impl.g_hoisted
    (U : Type)
    (T : Type)
    [trait_constr_g_hoisted_associated_type_i0 : T0.AssociatedTypes U]
    [trait_constr_g_hoisted_i0 : T0 U ]
    (x : T) :
    RustM T := do
  (pure x)

def Impl.AC_hoisted
  (U : Type)
  [trait_constr_AC_hoisted_associated_type_i0 : T0.AssociatedTypes U]
  [trait_constr_AC_hoisted_i0 : T0 U ] :
  u16
  :=
  (5 : u16)

@[spec]
def Impl.f_hoisted
    (U : Type)
    (T : Type)
    (C : u8)
    [trait_constr_f_hoisted_associated_type_i0 : T0.AssociatedTypes U]
    [trait_constr_f_hoisted_i0 : T0 U ]
    [trait_constr_f_hoisted_associated_type_i1 : T0.AssociatedTypes T]
    [trait_constr_f_hoisted_i1 : T0 T ]
    (x : T)
    (y : U) :
    RustM (rust_primitives.hax.Tuple5 T U u8 u64 u16) := do
  (pure (rust_primitives.hax.Tuple5.mk
    (← (Impl.g_hoisted U T x))
    y
    (← ((2 : u8) +? C))
    (3 : u64)
    (Impl.AC_hoisted U)))

end new_tests.legacy__lean_tests__lib.traits.methods_hoisting


namespace new_tests.legacy__lean_tests__lib.types

abbrev UsizeAlias : Type := usize

abbrev MyOption (A : Type) : Type := (core_models.option.Option A)

abbrev MyResult (A : Type) (B : Type) :
  Type :=
  (core_models.result.Result (core_models.option.Option A) B)

abbrev ErrorMonad (A : Type) (E : Type) :
  Type :=
  (core_models.result.Result A E)

abbrev StateMonad (A : Type) (S : Type) :
  Type :=
  (rust_primitives.hax.Tuple2 A S)

abbrev ESMonad (A : Type) (S : Type) (E : Type) :
  Type :=
  (rust_primitives.hax.Tuple2 (core_models.result.Result A E) S)

end new_tests.legacy__lean_tests__lib.types


namespace new_tests.legacy__lean_tests__lib

def FORTYTWO : usize := (42 : usize)

def MINUS_FORTYTWO : isize := (-42 : isize)

@[spec]
def returns42 (_ : rust_primitives.hax.Tuple0) : RustM usize := do
  (pure FORTYTWO)

@[spec]
def add_two_numbers (x : usize) (y : usize) : RustM usize := do (x +? y)

@[spec]
def letBinding (x : usize) (y : usize) : RustM usize := do
  let useless : rust_primitives.hax.Tuple0 := rust_primitives.hax.Tuple0.mk;
  let result1 : usize ← (x +? y);
  let result2 : usize ← (result1 +? (2 : usize));
  (result2 +? (1 : usize))

@[spec]
def closure (_ : rust_primitives.hax.Tuple0) : RustM i32 := do
  let x : i32 := (41 : i32);
  let f1 : (i32 -> RustM i32) := (fun y => (do (y +? x) : RustM i32));
  let f2 : (i32 -> i32 -> RustM i32) :=
    (fun y z => (do ((← (y +? x)) +? z) : RustM i32));
  let res1 : i32 ←
    (core_models.ops.function.Fn.call
      (i32 -> RustM i32)
      (rust_primitives.hax.Tuple1 i32)
      f1
      (rust_primitives.hax.Tuple1.mk (1 : i32)));
  let res2 : i32 ←
    (core_models.ops.function.Fn.call
      (i32 -> i32 -> RustM i32)
      (rust_primitives.hax.Tuple2 i32 i32)
      f2
      (rust_primitives.hax.Tuple2.mk (2 : i32) (3 : i32)));
  (res1 +? res2)

example : Nat := 42

@[spec]
def test_before_verbatime_single_line (x : u8) : RustM u8 := do (pure (42 : u8))


def multiline : Unit := ()


@[spec]
def test_before_verbatim_multi_line (x : u8) : RustM u8 := do (pure (32 : u8))

def NULL_CHAR : Char := ' '

--  Test string literals with escape sequences
@[spec]
def string_escapes (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let _empty : String := "";
  let _plain : String := "hello world";
  let _with_quotes : String := "she said \"hello\"";
  let _with_single_quote : String := "it\'s fine";
  let _with_backslash : String := "path\\to\\file";
  let _with_newline : String := "line1\nline2";
  let _with_tab : String := "col1\tcol2";
  let _with_carriage_return : String := "before\rafter";
  let _mixed : String := "say \"hello\"\nand\t\'goodbye\'\\end";
  let _carriage_return : String := "carriage\rreturn";
  let _control_chars : String := "null\x00byte bell\x07char font\x1b[0mreset";
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.legacy__lean_tests__lib


namespace new_tests.legacy__lean_tests__lib.associated_types.basic

class Iterable.AssociatedTypes (Self : Type) where
  Item : Type

attribute [reducible] Iterable.AssociatedTypes.Item

abbrev Iterable.Item :=
  Iterable.AssociatedTypes.Item

class Iterable (Self : Type)
  [associatedTypes : outParam (Iterable.AssociatedTypes (Self : Type))]
  where
  first (Self) : (Self -> RustM associatedTypes.Item)

end new_tests.legacy__lean_tests__lib.associated_types.basic


namespace new_tests.legacy__lean_tests__lib.associated_types.projection

class T2.AssociatedTypes (Self : Type) where
  A2 : Type

attribute [reducible] T2.AssociatedTypes.A2

abbrev T2.A2 :=
  T2.AssociatedTypes.A2

class T2 (Self : Type)
  [associatedTypes : outParam (T2.AssociatedTypes (Self : Type))]
  where
  [trait_constr_A2_associated_type_i1 : T1.AssociatedTypes associatedTypes.A2]
  [trait_constr_A2_i1 : T1 associatedTypes.A2 ]
  f (Self) : (rust_primitives.hax.Tuple0 -> RustM (T1.A1 associatedTypes.A2))

end new_tests.legacy__lean_tests__lib.associated_types.projection


namespace new_tests.legacy__lean_tests__lib.associated_types.multiple_associated_types

class Pair.AssociatedTypes (Self : Type) where
  First : Type
  Second : Type

attribute [reducible] Pair.AssociatedTypes.First

attribute [reducible] Pair.AssociatedTypes.Second

abbrev Pair.First :=
  Pair.AssociatedTypes.First

abbrev Pair.Second :=
  Pair.AssociatedTypes.Second

class Pair (Self : Type)
  [associatedTypes : outParam (Pair.AssociatedTypes (Self : Type))]
  where
  first (Self) : (Self -> RustM associatedTypes.First)
  second (Self) : (Self -> RustM associatedTypes.Second)

end new_tests.legacy__lean_tests__lib.associated_types.multiple_associated_types


namespace new_tests.legacy__lean_tests__lib.enums

inductive MyList (T : Type) : Type
| Nil : MyList (T : Type)
| Cons (hd : T) (tl : (MyList T)) : MyList (T : Type)

end new_tests.legacy__lean_tests__lib.enums


namespace new_tests.legacy__lean_tests__lib.recursion

@[spec]
def factorial (n : u32) : RustM u32 := do
  if (← (n ==? (0 : u32))) then do
    (pure (1 : u32))
  else do
    (n *? (← (factorial (← (n -? (1 : u32))))))
partial_fixpoint

end new_tests.legacy__lean_tests__lib.recursion


namespace new_tests.legacy__lean_tests__lib.traits.associated_types

class T1.AssociatedTypes (Self : Type) where
  T : Type

attribute [reducible] T1.AssociatedTypes.T

abbrev T1.T :=
  T1.AssociatedTypes.T

class T1 (Self : Type)
  [associatedTypes : outParam (T1.AssociatedTypes (Self : Type))]
  where
  f (Self) : (Self -> associatedTypes.T -> RustM associatedTypes.T)

class T3.AssociatedTypes (Self : Type) where
  T : Type
  Tp : Type

attribute [reducible] T3.AssociatedTypes.T

attribute [reducible] T3.AssociatedTypes.Tp

abbrev T3.T :=
  T3.AssociatedTypes.T

abbrev T3.Tp :=
  T3.AssociatedTypes.Tp

class T3 (Self : Type)
  [associatedTypes : outParam (T3.AssociatedTypes (Self : Type))]
  where
  [trait_constr_T_associated_type_i1 : Bar.AssociatedTypes associatedTypes.T]
  [trait_constr_T_i1 : Bar associatedTypes.T ]
  (A : Type)
  [trait_constr_Tp_associated_type_i1 : Foo.AssociatedTypes
    associatedTypes.Tp
    associatedTypes.T]
  [trait_constr_Tp_i1 : Foo associatedTypes.Tp associatedTypes.T ]
  f (Self)
    (A : Type)
    [trait_constr_f_associated_type_i1 : Bar.AssociatedTypes A]
    [trait_constr_f_i1 : Bar A ] :
    (Self -> associatedTypes.T -> associatedTypes.Tp -> RustM usize)

class Chain1.AssociatedTypes (Self : Type) where
  A : Type
  B : Type

attribute [reducible] Chain1.AssociatedTypes.A

attribute [reducible] Chain1.AssociatedTypes.B

abbrev Chain1.A :=
  Chain1.AssociatedTypes.A

abbrev Chain1.B :=
  Chain1.AssociatedTypes.B

class Chain1 (Self : Type)
  [associatedTypes : outParam (Chain1.AssociatedTypes (Self : Type))]
  where
  [trait_constr_A_associated_type_i1 : Chain0.AssociatedTypes associatedTypes.A]
  [trait_constr_A_i1 : Chain0 associatedTypes.A ]
  [trait_constr_B_associated_type_i1 : Chain0.AssociatedTypes associatedTypes.B]
  [trait_constr_B_i1 : Chain0 associatedTypes.B ]

end new_tests.legacy__lean_tests__lib.traits.associated_types


namespace new_tests.legacy__lean_tests__lib.traits.methods_hoisting

class T1.AssociatedTypes (Self : Type) (U : Type) where
  A : Type

attribute [reducible] T1.AssociatedTypes.A

abbrev T1.A :=
  T1.AssociatedTypes.A

class T1 (Self : Type) (U : Type)
  [associatedTypes : outParam (T1.AssociatedTypes (Self : Type) (U : Type))]
  where
  f (Self) (U)
    (T : Type)
    (C : u8)
    [trait_constr_f_associated_type_i1 : T0.AssociatedTypes T]
    [trait_constr_f_i1 : T0 T ] :
    (T ->
    U ->
    RustM (rust_primitives.hax.Tuple5 T U Self associatedTypes.A u16))
  g (Self) (U) (T : Type) : (T -> RustM T)
  AC (Self) (U) : u16

end new_tests.legacy__lean_tests__lib.traits.methods_hoisting


namespace new_tests.legacy__lean_tests__lib.associated_types.basic

@[spec]
def just_the_first
    (I : Type)
    [trait_constr_just_the_first_associated_type_i0 : Iterable.AssociatedTypes
      I]
    [trait_constr_just_the_first_i0 : Iterable I ]
    (iter : I) :
    RustM (Iterable.Item I) := do
  (Iterable.first I iter)

@[spec]
def first_plus_1
    (I : Type)
    [trait_constr_first_plus_1_associated_type_i0 : Iterable.AssociatedTypes I]
    [trait_constr_first_plus_1_i0 : Iterable
      I
      (associatedTypes := {
        show Iterable.AssociatedTypes I
        by infer_instance
        with Item := i32})]
    (iter : I) :
    RustM i32 := do
  ((← (Iterable.first I iter)) +? (1 : i32))

@[reducible] instance Impl.AssociatedTypes : Iterable.AssociatedTypes Bool where
  Item := i32

instance Impl : Iterable Bool where
  first := (Impl.first_hoisted)

@[spec]
def a (_ : rust_primitives.hax.Tuple0) : RustM rust_primitives.hax.Tuple0 := do
  let _ ← (first_plus_1 Bool true);
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.legacy__lean_tests__lib.associated_types.basic


namespace new_tests.legacy__lean_tests__lib.associated_types.multiple_associated_types

@[spec]
def get_both
    (P : Type)
    [trait_constr_get_both_associated_type_i0 : Pair.AssociatedTypes P]
    [trait_constr_get_both_i0 : Pair P ]
    (pair : P) :
    RustM (rust_primitives.hax.Tuple2 (Pair.First P) (Pair.Second P)) := do
  (pure (rust_primitives.hax.Tuple2.mk
    (← (Pair.first P pair))
    (← (Pair.second P pair))))

--  @fail(extraction): ssprove(HAX0001)
@[reducible] instance Impl.AssociatedTypes :
  Pair.AssociatedTypes (rust_primitives.hax.Tuple2 i32 Bool)
  where
  First := i32
  Second := Bool

instance Impl : Pair (rust_primitives.hax.Tuple2 i32 Bool) where
  first := (Impl.first_hoisted)
  second := (Impl.second_hoisted)

@[spec]
def b (_ : rust_primitives.hax.Tuple0) : RustM rust_primitives.hax.Tuple0 := do
  let pair : (rust_primitives.hax.Tuple2 i32 Bool) :=
    (rust_primitives.hax.Tuple2.mk (42 : i32) true);
  let both : (rust_primitives.hax.Tuple2 i32 Bool) ←
    (get_both (rust_primitives.hax.Tuple2 i32 Bool) pair);
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def get_first_as_i32
    (P : Type)
    [trait_constr_get_first_as_i32_associated_type_i0 : Pair.AssociatedTypes P]
    [trait_constr_get_first_as_i32_i0 : Pair
      P
      (associatedTypes := {
        show Pair.AssociatedTypes P
        by infer_instance
        with First := i32})]
    (pair : P) :
    RustM i32 := do
  (Pair.first P pair)

end new_tests.legacy__lean_tests__lib.associated_types.multiple_associated_types


namespace new_tests.legacy__lean_tests__lib.enums

--  @fail(extraction): ssprove(HAX0001)
@[spec]
def enums (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let e_v1 : E := E.V1;
  let e_v2 : E := E.V2;
  let e_v3 : E := (E.V3 (23 : usize));
  let e_v4 : E := (E.V4 (23 : usize) (12 : usize) (1 : usize));
  let e_v5 : E := (E.V5 (f1 := (23 : usize)) (f2 := (43 : usize)));
  let e_v6 : E := (E.V6 (f1 := (12 : usize)) (f2 := (13 : usize)));
  let nil : (MyList usize) := MyList.Nil;
  let cons_1 : (MyList usize) := (MyList.Cons (hd := (1 : usize)) (tl := nil));
  let cons_2_1 : (MyList usize) :=
    (MyList.Cons (hd := (2 : usize)) (tl := cons_1));
  match e_v1 with
    | (E.V1 ) => do (pure rust_primitives.hax.Tuple0.mk)
    | (E.V2 ) => do (pure rust_primitives.hax.Tuple0.mk)
    | (E.V3  _) => do (pure rust_primitives.hax.Tuple0.mk)
    | (E.V4  x1 x2 x3) => do
      let y1 : usize ← (x1 +? x2);
      let y2 : usize ← (y1 -? x2);
      let y3 : usize ← (y2 +? x3);
      (pure rust_primitives.hax.Tuple0.mk)
    | (E.V5  (f1 := f1) (f2 := f2)) => do (pure rust_primitives.hax.Tuple0.mk)
    | (E.V6  (f1 := f1) (f2 := other_name_for_f2)) => do
      (pure rust_primitives.hax.Tuple0.mk)

end new_tests.legacy__lean_tests__lib.enums


namespace new_tests.legacy__lean_tests__lib.traits.associated_types

@[reducible] instance Impl.AssociatedTypes : T1.AssociatedTypes S where
  T := i32

instance Impl : T1 S where
  f := (Impl.f_hoisted)

@[reducible] instance Impl_4.AssociatedTypes : T3.AssociatedTypes S where
  T := i16
  Tp := (rust_primitives.hax.Tuple2 u32 A)

instance Impl_4 : T3 S where
  f :=
    fun
      
      (A : Type)
      [trait_constr__associated_type_i0 : Bar.AssociatedTypes A]
      [trait_constr__i0 : Bar A ]
      =>
    (Impl_4.f_hoisted A)

class Chain2.AssociatedTypes (Self : Type) where
  [trait_constr_Chain2_i0 : Chain1.AssociatedTypes Self]

attribute [instance_reducible, instance]
  Chain2.AssociatedTypes.trait_constr_Chain2_i0

class Chain2 (Self : Type)
  [associatedTypes : outParam (Chain2.AssociatedTypes (Self : Type))]
  where
  [trait_constr_Chain2_i0 : Chain1 Self]

attribute [instance_reducible, instance] Chain2.trait_constr_Chain2_i0

class Chain3.AssociatedTypes (Self : Type) where
  [trait_constr_Chain3_i0 : Chain2.AssociatedTypes Self]

attribute [instance_reducible, instance]
  Chain3.AssociatedTypes.trait_constr_Chain3_i0

class Chain3 (Self : Type)
  [associatedTypes : outParam (Chain3.AssociatedTypes (Self : Type))]
  where
  [trait_constr_Chain3_i0 : Chain2 Self]
  f (Self) : (rust_primitives.hax.Tuple0 -> RustM (Chain1.A Self))

attribute [instance_reducible, instance] Chain3.trait_constr_Chain3_i0

@[reducible] instance Impl_6.AssociatedTypes : Chain1.AssociatedTypes u8 where
  A := u8
  B := u8

instance Impl_6 : Chain1 u8 where

@[reducible] instance Impl_7.AssociatedTypes : Chain2.AssociatedTypes u8 where

instance Impl_7 : Chain2 u8 where

@[reducible] instance Impl_8.AssociatedTypes : Chain3.AssociatedTypes u8 where

instance Impl_8 : Chain3 u8 where
  f := (Impl_8.f_hoisted)

end new_tests.legacy__lean_tests__lib.traits.associated_types


namespace new_tests.legacy__lean_tests__lib.traits.methods_hoisting

@[reducible] instance Impl.AssociatedTypes
  (U : Type)
  [trait_constr_Impl_associated_type_i0 : T0.AssociatedTypes U]
  [trait_constr_Impl_i0 : T0 U ] :
  T1.AssociatedTypes u8 U
  where
  A := u64

instance Impl
  (U : Type)
  [trait_constr_Impl_associated_type_i0 : T0.AssociatedTypes U]
  [trait_constr_Impl_i0 : T0 U ] :
  T1 u8 U
  where
  f :=
    fun
      
      (T : Type)
      (C : u8)
      [trait_constr__associated_type_i1 : T0.AssociatedTypes T]
      [trait_constr__i1 : T0 T ]
      =>
    (Impl.f_hoisted U T (C))
  g := fun  (T : Type) => (Impl.g_hoisted U T)
  AC := (Impl.AC_hoisted U)

end new_tests.legacy__lean_tests__lib.traits.methods_hoisting


namespace new_tests.legacy__lean_tests__lib.traits.associated_types

class T2.AssociatedTypes (Self : Type) where
  T : Type

attribute [reducible] T2.AssociatedTypes.T

abbrev T2.T :=
  T2.AssociatedTypes.T

class T2 (Self : Type)
  [associatedTypes : outParam (T2.AssociatedTypes (Self : Type))]
  where
  [trait_constr_T_associated_type_i1 : T1.AssociatedTypes associatedTypes.T]
  [trait_constr_T_i1 : T1 associatedTypes.T ]
  f (Self) : (Self -> associatedTypes.T -> RustM usize)

@[reducible] instance Impl_1.AssociatedTypes : T2.AssociatedTypes S where
  T := S

instance Impl_1 : T2 S where
  f := (Impl_1.f_hoisted)

end new_tests.legacy__lean_tests__lib.traits.associated_types

