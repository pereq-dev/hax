
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


namespace new_tests.legacy__cli__interface_only__lib

--  This item contains unsafe blocks and raw references, two features
--  not supported by hax. Thanks to the `-i` flag and the `+:`
--  modifier, `f` is still extractable as an interface.
-- 
--  Expressions within type are still extracted, as well as pre- and
--  post-conditions.
--  @fail(extraction): ssprove(HAX0008, HAX0008, HAX0008, HAX0008), coq(HAX0008, HAX0008, HAX0008, HAX0008), fstar(HAX0008, HAX0008, HAX0008, HAX0008)
--  @fail(extraction): legacy-lean(HAX0008, HAX0008, HAX0008, HAX0008)
--  @fail(extraction): proverif(HAX0008, HAX0008, HAX0008, HAX0008)
def f (x : u8) : RustM (RustArray u8 4) := do (pure sorry)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def f.spec (x : u8) :
    Spec
      (requires := do (x <? (254 : u8)))
      (ensures := fun r => do ((← r[(0 : usize)]_?) >? x))
      (f (x : u8)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [f] <;> bv_decide
}

structure Bar where
  -- no fields

@[spec]
def Impl.from_hoisted sorry : RustM Bar := do (pure Bar.mk)

--  Non-inherent implementations are extracted, their bodies are not
--  dropped. This might be a bit surprising: see
--  https://github.com/pereq-dev/hax/issues/616.
@[reducible] instance Impl.AssociatedTypes :
  core_models.convert.From.AssociatedTypes Bar rust_primitives.hax.Tuple0
  where

instance Impl : core_models.convert.From Bar rust_primitives.hax.Tuple0 where
  _from := (Impl.from_hoisted)

@[spec]
def Impl_1._from._from (_ : u8) : RustM Bar := do (pure Bar.mk)

@[spec]
def Impl_1.from_hoisted (x : u8) : RustM Bar := do (Impl_1._from._from x)

--  If you need to drop the body of a method, please hoist it:
@[reducible] instance Impl_1.AssociatedTypes :
  core_models.convert.From.AssociatedTypes Bar u8
  where

instance Impl_1 : core_models.convert.From Bar u8 where
  _from := (Impl_1.from_hoisted)

structure Holder (T : Type) where
  value : (alloc.vec.Vec T alloc.alloc.Global)

@[spec]
def Impl_2.from_hoisted (T : Type) sorry : RustM (Holder T) := do
  (pure (Holder.mk
    (value := (← (alloc.vec.Impl.new T rust_primitives.hax.Tuple0.mk)))))

@[reducible] instance Impl_2.AssociatedTypes (T : Type) :
  core_models.convert.From.AssociatedTypes (Holder T) rust_primitives.hax.Tuple0
  where

instance Impl_2 (T : Type) :
  core_models.convert.From (Holder T) rust_primitives.hax.Tuple0
  where
  _from := (Impl_2.from_hoisted T)

structure Param (SIZE : usize) where
  value : (RustArray u8 SIZE)

@[spec]
def Impl_3.from_hoisted (SIZE : usize) sorry : RustM (Param (SIZE)) := do
  (pure (Param.mk (value := (← (rust_primitives.hax.repeat (0 : u8) SIZE)))))

@[reducible] instance Impl_3.AssociatedTypes (SIZE : usize) :
  core_models.convert.From.AssociatedTypes
  (Param (SIZE))
  rust_primitives.hax.Tuple0
  where

instance Impl_3 (SIZE : usize) :
  core_models.convert.From (Param (SIZE)) rust_primitives.hax.Tuple0
  where
  _from := (Impl_3.from_hoisted (SIZE))

@[spec]
def f_generic (X : usize) (U : Type) (_x : U) : RustM (Param (X)) := do
  (pure (Param.mk (value := (← (rust_primitives.hax.repeat (0 : u8) X)))))

class T.AssociatedTypes (Self : Type) where
  Assoc : Type

attribute [reducible] T.AssociatedTypes.Assoc

abbrev T.Assoc :=
  T.AssociatedTypes.Assoc

class T (Self : Type)
  [associatedTypes : outParam (T.AssociatedTypes (Self : Type))]
  where
  d (Self) : (rust_primitives.hax.Tuple0 -> RustM rust_primitives.hax.Tuple0)

@[spec]
def Impl_4.d_hoisted (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

--  Impls with associated types are not erased
@[reducible] instance Impl_4.AssociatedTypes : T.AssociatedTypes u8 where
  Assoc := u8

instance Impl_4 : T u8 where
  d := (Impl_4.d_hoisted)

class T2.AssociatedTypes (Self : Type) where

class T2 (Self : Type)
  [associatedTypes : outParam (T2.AssociatedTypes (Self : Type))]
  where
  d (Self) : (rust_primitives.hax.Tuple0 -> RustM rust_primitives.hax.Tuple0)

def Impl_5.d_hoisted (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def Impl_5.d_hoisted.spec (_ : rust_primitives.hax.Tuple0) :
    Spec
      (requires := do (pure false))
      (ensures := fun _ => pure True)
      (Impl_5.d_hoisted ⟨⟩) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [Impl_5.d_hoisted] <;> bv_decide
}

--  Items can be forced to be transparent
@[reducible] instance Impl_5.AssociatedTypes : T2.AssociatedTypes u8 where

instance Impl_5 : T2 u8 where
  d := (Impl_5.d_hoisted)

def padlen (b : (RustSlice u8)) (n : usize) : RustM usize := do
  if
  (← ((← (n >? (0 : usize)))
    &&? (← ((← b[(← (n -? (1 : usize)))]_?) ==? (0 : u8))))) then do
    ((1 : usize) +? (← (padlen b (← (n -? (1 : usize))))))
  else do
    (pure (0 : usize))

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def padlen.spec (b : (RustSlice u8)) (n : usize) :
    Spec
      (requires := do ((← (core_models.slice.Impl.len u8 b)) >=? n))
      (ensures := fun out => do (out <=? n))
      (padlen (b : (RustSlice u8)) (n : usize)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [padlen] <;> bv_decide
}
partial_fixpoint

end new_tests.legacy__cli__interface_only__lib

