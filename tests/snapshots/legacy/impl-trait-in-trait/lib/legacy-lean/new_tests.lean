
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


namespace new_tests.legacy__impl_trait_in_trait__lib

structure Foo where
  _0 : u8

@[spec]
def Impl.stream_hoisted (self : Foo) : RustM u8 := do (pure (Foo._0 self))

class Streamer.AssociatedTypes (Self : Type) where
  stream_impl_trait : Type

attribute [reducible] Streamer.AssociatedTypes.stream_impl_trait

abbrev Streamer.stream_impl_trait :=
  Streamer.AssociatedTypes.stream_impl_trait

class Streamer (Self : Type)
  [associatedTypes : outParam (Streamer.AssociatedTypes (Self : Type))]
  where
  stream (Self) : (Self -> RustM associatedTypes.stream_impl_trait)

@[reducible] instance Impl.AssociatedTypes : Streamer.AssociatedTypes Foo where
  stream_impl_trait := u8

instance Impl : Streamer Foo where
  stream := (Impl.stream_hoisted)

end new_tests.legacy__impl_trait_in_trait__lib

