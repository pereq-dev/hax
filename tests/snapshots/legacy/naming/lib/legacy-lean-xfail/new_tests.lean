
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


namespace new_tests.legacy__naming__lib

inductive Foo : Type
| A : Foo
| B (x : usize) : Foo

inductive Foo2 : Type
| A : Foo2
| B (x : usize) : Foo2

structure B where
  -- no fields

structure C where
  x : usize

structure X where
  -- no fields

@[spec]
def mk_c (_ : rust_primitives.hax.Tuple0) : RustM C := do
  let _ := (Foo.B (x := (3 : usize)));
  let _ := X.mk;
  (pure (C.mk (x := (3 : usize))))

@[spec]
def Impl.f (self : Foo) : RustM Foo := do (pure Foo.A)

@[spec]
def Impl_1.f (self : B) : RustM B := do (pure B.mk)

structure Foobar where
  a : Foo

@[spec]
def f.g (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def f.g.Impl.g (self : B) : RustM usize := do (pure (0 : usize))

inductive f.g.Impl.g.Foo : Type
| A : f.g.Impl.g.Foo
| B (x : usize) : f.g.Impl.g.Foo

@[spec]
def f.g.Impl_1.g (self : Foo) : RustM usize := do (pure (1 : usize))

@[spec]
def f (x : Foobar) : RustM usize := do (f.g.Impl_1.g (Foobar.a x))

@[spec]
def f.g.Impl_1.g.hello.h (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def reserved_names (val : u8) (noeq : u8) (of : u8) : RustM u8 := do
  ((← (val +? noeq)) +? of)

structure Arity1 (T : Type) where
  _0 : T

class T1.AssociatedTypes (Self : Type) where

class T1 (Self : Type)
  [associatedTypes : outParam (T1.AssociatedTypes (Self : Type))]
  where

@[reducible] instance Impl_2.AssociatedTypes : T1.AssociatedTypes Foo where

instance Impl_2 : T1 Foo where

@[reducible] instance Impl_3.AssociatedTypes :
  T1.AssociatedTypes (rust_primitives.hax.Tuple2 Foo u8)
  where

instance Impl_3 : T1 (rust_primitives.hax.Tuple2 Foo u8) where

class T2_for_a.AssociatedTypes (Self : Type) where

class T2_for_a (Self : Type)
  [associatedTypes : outParam (T2_for_a.AssociatedTypes (Self : Type))]
  where

@[reducible] instance Impl_4.AssociatedTypes :
  T2_for_a.AssociatedTypes (Arity1 (rust_primitives.hax.Tuple2 Foo u8))
  where

instance Impl_4 : T2_for_a (Arity1 (rust_primitives.hax.Tuple2 Foo u8)) where

class T3_e_for_a.AssociatedTypes (Self : Type) where

class T3_e_for_a (Self : Type)
  [associatedTypes : outParam (T3_e_for_a.AssociatedTypes (Self : Type))]
  where

@[reducible] instance Impl_5.AssociatedTypes :
  T3_e_for_a.AssociatedTypes Foo
  where

instance Impl_5 : T3_e_for_a Foo where

structure StructA where
  a : usize

structure StructB where
  a : usize
  b : usize

structure StructC where
  a : usize

structure StructD where
  a : usize
  b : usize

@[spec]
def construct_structs (a : usize) (b : usize) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ := (StructA.mk (a := a));
  let _ := (StructB.mk (a := a) (b := b));
  let _ := (StructC.mk (a := a));
  let _ := (StructD.mk (a := a) (b := b));
  (pure rust_primitives.hax.Tuple0.mk)

def INHERENT_CONSTANT : usize := (3 : usize)

class FooTrait.AssociatedTypes (Self : Type) where

class FooTrait (Self : Type)
  [associatedTypes : outParam (FooTrait.AssociatedTypes (Self : Type))]
  where
  ASSOCIATED_CONSTANT (Self) : usize

@[spec]
def constants
    (T : Type)
    [trait_constr_constants_associated_type_i0 : FooTrait.AssociatedTypes T]
    [trait_constr_constants_i0 : FooTrait T ]
    (_ : rust_primitives.hax.Tuple0) :
    RustM usize := do
  ((FooTrait.ASSOCIATED_CONSTANT T) +? INHERENT_CONSTANT)

end new_tests.legacy__naming__lib


namespace new_tests.legacy__naming__lib.ambiguous_names

--  @fail(extraction): ssprove(HAX0001)
@[spec]
def debug (label : u32) (value : u32) : RustM rust_primitives.hax.Tuple0 := do
  let args : (rust_primitives.hax.Tuple2 u32 u32) :=
    (rust_primitives.hax.Tuple2.mk label value);
  let args : (RustArray core_models.fmt.rt.Argument 2) :=
    (RustArray.ofVec #v[(← (core_models.fmt.rt.Impl.new_display u32
                            (rust_primitives.hax.Tuple2._0 args))),
                          (← (core_models.fmt.rt.Impl.new_display u32
                            (rust_primitives.hax.Tuple2._1 args)))]);
  let _ ←
    (std.io.stdio._print
      (← (core_models.fmt.rt.Impl_1.new_v1 ((3 : usize)) ((2 : usize))
        (RustArray.ofVec #v["[", "] a=", "\n"])
        args)));
  (pure rust_primitives.hax.Tuple0.mk)

--  `f` stacks mutliple let bindings declaring different `a`s.
@[spec]
def f (_ : rust_primitives.hax.Tuple0) : RustM rust_primitives.hax.Tuple0 := do
  let a_1 : u32 := (104 : u32);
  let a_2 : u32 := (205 : u32);
  let a_3 : u32 := (306 : u32);
  let a : u32 := (123 : u32);
  let _ ← (debug (3 : u32) a_3);
  let _ ← (debug (2 : u32) a_2);
  let _ ← (debug (1 : u32) a_1);
  (debug (4 : u32) a)

--  `f` is expanded into `f_expand` below, while the execution of `f` gives:
-- 
--  ```plaintext
--   [3] a=306
--   [2] a=205
--   [1] a=104
--   [last] a=123
--  ```
@[spec]
def f_expand (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let a : i32 := (104 : i32);
  let a : i32 := (205 : i32);
  let a : i32 := (306 : i32);
  let a : u32 := (123 : u32);
  let _ ← (debug (3 : u32) a);
  let _ ← (debug (2 : u32) a);
  let _ ← (debug (1 : u32) a);
  (debug (0 : u32) a)

end new_tests.legacy__naming__lib.ambiguous_names


namespace new_tests.legacy__naming__lib

--  From issue https://github.com/pereq-dev/hax/issues/839
@[spec]
def string_shadows (string : String) (n : String) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.legacy__naming__lib


namespace new_tests.legacy__naming__lib.functions_defined_in_trait_impls

structure A where
  -- no fields

@[spec]
def Impl.eq_hoisted (self : A) (other : A) : RustM Bool := do
  (rust_primitives.hax.never_to_any
    (← (core_models.panicking.panic "explicit panic")))

@[reducible] instance Impl.AssociatedTypes :
  core_models.cmp.PartialEq.AssociatedTypes A A
  where

instance Impl : core_models.cmp.PartialEq A A where
  eq := (Impl.eq_hoisted)

structure B where
  -- no fields

@[spec]
def Impl_1.eq_hoisted (self : B) (other : B) : RustM Bool := do
  (rust_primitives.hax.never_to_any
    (← (core_models.panicking.panic "explicit panic")))

@[reducible] instance Impl_1.AssociatedTypes :
  core_models.cmp.PartialEq.AssociatedTypes B B
  where

instance Impl_1 : core_models.cmp.PartialEq B B where
  eq := (Impl_1.eq_hoisted)

end new_tests.legacy__naming__lib.functions_defined_in_trait_impls


namespace new_tests.legacy__naming__lib

--  From issue https://github.com/pereq-dev/hax/issues/1450
@[spec]
def items_under_closures (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ : (rust_primitives.hax.Tuple0 -> RustM rust_primitives.hax.Tuple0) :=
    (fun _ =>
      (do
      (pure rust_primitives.hax.Tuple0.mk) : RustM rust_primitives.hax.Tuple0));
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def items_under_closures.Closure.nested_function
    (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

structure items_under_closures.Closure.NestedStruct where
  -- no fields

@[spec]
def items_under_closures.nested_function (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

structure items_under_closures.NestedStruct where
  -- no fields

end new_tests.legacy__naming__lib

