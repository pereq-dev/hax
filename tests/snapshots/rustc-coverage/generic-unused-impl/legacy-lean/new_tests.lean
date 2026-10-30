
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


namespace new_tests.rustc_coverage__generic_unused_impl

structure W (T : Type) where
  _0 : T

@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

class Foo.AssociatedTypes (Self : Type) where
  Assoc : Type

attribute [reducible] Foo.AssociatedTypes.Assoc

abbrev Foo.Assoc :=
  Foo.AssociatedTypes.Assoc

class Foo (Self : Type)
  [associatedTypes : outParam (Foo.AssociatedTypes (Self : Type))]
  where
  _from (Self) : (associatedTypes.Assoc -> RustM Self)

@[spec]
def Impl.from_hoisted
    (T : Type)
    [trait_constr_from_hoisted_associated_type_i0 : Foo.AssociatedTypes T]
    [trait_constr_from_hoisted_i0 : Foo T ]
    (_from : (RustArray (Foo.Assoc T) 1)) :
    RustM (W T) := do
  (pure sorry)

--  @fail(extraction): fstar(HAX0001), coq(HAX0001), ssprove(HAX0001), legacy-lean(HAX0001)
--  @fail(extraction): proverif(HAX0001)
@[reducible] instance Impl.AssociatedTypes
  (T : Type)
  [trait_constr_Impl_associated_type_i0 : Foo.AssociatedTypes T]
  [trait_constr_Impl_i0 : Foo T ] :
  core_models.convert.From.AssociatedTypes (W T) (RustArray (Foo.Assoc T) 1)
  where

instance Impl
  (T : Type)
  [trait_constr_Impl_associated_type_i0 : Foo.AssociatedTypes T]
  [trait_constr_Impl_i0 : Foo T ] :
  core_models.convert.From (W T) (RustArray (Foo.Assoc T) 1)
  where
  _from := (Impl.from_hoisted T)

end new_tests.rustc_coverage__generic_unused_impl

