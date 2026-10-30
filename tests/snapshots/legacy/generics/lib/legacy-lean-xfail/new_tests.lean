
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


namespace new_tests.legacy__generics__lib

@[spec]
def dup
    (T : Type)
    [trait_constr_dup_associated_type_i0 :
      core_models.clone.Clone.AssociatedTypes
      T]
    [trait_constr_dup_i0 : core_models.clone.Clone T ]
    (x : T) :
    RustM (rust_primitives.hax.Tuple2 T T) := do
  (pure (rust_primitives.hax.Tuple2.mk
    (← (core_models.clone.Clone.clone T x))
    (← (core_models.clone.Clone.clone T x))))

@[spec]
def foo (LEN : usize) (arr : (RustArray usize LEN)) : RustM usize := do
  let acc : usize ← (LEN +? (9 : usize));
  let acc : usize ←
    (rust_primitives.hax.folds.fold_range
      (0 : usize)
      LEN
      (fun acc _ => (do (pure true) : RustM Bool))
      acc
      (fun acc i => (do (acc +? (← arr[i]_?)) : RustM usize)));
  (pure acc)

@[spec]
def repeat
    (LEN : usize)
    (T : Type)
    [trait_constr_repeat_associated_type_i0 :
      core_models.marker.Copy.AssociatedTypes
      T]
    [trait_constr_repeat_i0 : core_models.marker.Copy T ]
    (x : T) :
    RustM (RustArray T LEN) := do
  (rust_primitives.hax.repeat x LEN)

@[spec]
def f (N : usize) (x : usize) : RustM usize := do ((← (N +? N)) +? x)

@[spec]
def call_f (_ : rust_primitives.hax.Tuple0) : RustM usize := do
  ((← (f ((10 : usize)) (3 : usize))) +? (3 : usize))

@[spec]
def g
    (N : usize)
    (T : Type)
    [trait_constr_g_associated_type_i0 :
      core_models.convert.Into.AssociatedTypes
      T
      (RustArray usize N)]
    [trait_constr_g_i0 : core_models.convert.Into T (RustArray usize N) ]
    (arr : T) :
    RustM usize := do
  ((← (core_models.option.Impl.unwrap_or usize
      (← (core_models.iter.traits.iterator.Iterator.max
        (core_models.array.iter.IntoIter usize (N))
        (← (core_models.iter.traits.collect.IntoIterator.into_iter
          (RustArray usize N)
          (← (core_models.convert.Into.into T (RustArray usize N) arr))))))
      N))
    +? N)

@[spec]
def call_g (_ : rust_primitives.hax.Tuple0) : RustM usize := do
  ((← (g ((3 : usize)) (RustArray usize 3)
      (RustArray.ofVec #v[(42 : usize), (3 : usize), (49 : usize)])))
    +? (3 : usize))

class Foo.AssociatedTypes (Self : Type) where

class Foo (Self : Type)
  [associatedTypes : outParam (Foo.AssociatedTypes (Self : Type))]
  where
  const_add (Self) (N : usize) : (Self -> RustM usize)

@[spec]
def Impl.const_add_hoisted (N : usize) (self : usize) : RustM usize := do
  (self +? N)

@[reducible] instance Impl.AssociatedTypes : Foo.AssociatedTypes usize where

instance Impl : Foo usize where
  const_add := fun  (N : usize) => (Impl.const_add_hoisted (N))

structure Bar where
  -- no fields

@[spec]
def Impl_1.inherent_impl_generics (T : Type) (N : usize) (x : (RustArray T N)) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.legacy__generics__lib


namespace new_tests.legacy__generics__lib.defaults_generics

structure Defaults (T : Type) (N : usize) where
  _0 : (RustArray T N)

@[spec]
def f (_ : (Defaults rust_primitives.hax.Tuple0 ((2 : usize)))) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.legacy__generics__lib.defaults_generics


namespace new_tests.legacy__generics__lib.impl_generics

structure Test where
  -- no fields

@[spec]
def Impl.set_ciphersuites
    (S : Type)
    (impl_IntoIterator_Item_=_S_ : Type)
    [trait_constr_set_ciphersuites_associated_type_i0 :
      core_models.convert.AsRef.AssociatedTypes
      S
      String]
    [trait_constr_set_ciphersuites_i0 : core_models.convert.AsRef S String ]
    [trait_constr_set_ciphersuites_associated_type_i1 :
      core_models.iter.traits.collect.IntoIterator.AssociatedTypes
      impl_IntoIterator_Item_=_S_]
    [trait_constr_set_ciphersuites_i1 :
      core_models.iter.traits.collect.IntoIterator
      impl_IntoIterator_Item_=_S_
      (associatedTypes := {
        show
          core_models.iter.traits.collect.IntoIterator.AssociatedTypes
          impl_IntoIterator_Item_=_S_
        by infer_instance
        with Item := S})]
    (self : Test)
    (ciphers : impl_IntoIterator_Item_=_S_) :
    RustM
    (core_models.result.Result
      rust_primitives.hax.Tuple0
      rust_primitives.hax.Tuple0)
    := do
  (pure (core_models.result.Result.Ok rust_primitives.hax.Tuple0.mk))

@[spec]
def Impl.set_alpn_protocols
    (S : Type)
    (impl_IntoIterator_Item_=_S_ : Type)
    [trait_constr_set_alpn_protocols_associated_type_i0 :
      core_models.convert.AsRef.AssociatedTypes
      S
      String]
    [trait_constr_set_alpn_protocols_i0 : core_models.convert.AsRef S String ]
    [trait_constr_set_alpn_protocols_associated_type_i1 :
      core_models.iter.traits.collect.IntoIterator.AssociatedTypes
      impl_IntoIterator_Item_=_S_]
    [trait_constr_set_alpn_protocols_i1 :
      core_models.iter.traits.collect.IntoIterator
      impl_IntoIterator_Item_=_S_
      (associatedTypes := {
        show
          core_models.iter.traits.collect.IntoIterator.AssociatedTypes
          impl_IntoIterator_Item_=_S_
        by infer_instance
        with Item := S})]
    (self : Test)
    (_protocols : impl_IntoIterator_Item_=_S_) :
    RustM
    (core_models.result.Result
      rust_primitives.hax.Tuple0
      rust_primitives.hax.Tuple0)
    := do
  (pure (core_models.result.Result.Ok rust_primitives.hax.Tuple0.mk))

end new_tests.legacy__generics__lib.impl_generics


namespace new_tests.legacy__generics__lib.assoc_const_param

structure Test (N : usize) where
  -- no fields

def Impl.A (N : usize) : (Test (N)) :=
  RustM.of_isOk (do (pure Test.mk)) (by rfl)

@[spec]
def test (_ : rust_primitives.hax.Tuple0) : RustM (Test ((1 : usize))) := do
  (Impl.A (1 : usize))

end new_tests.legacy__generics__lib.assoc_const_param

