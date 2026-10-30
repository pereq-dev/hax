
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


namespace new_tests.legacy__proverif_generics__lib

@[spec]
def id (T : Type) (x : T) : RustM T := do (pure x)

structure Token where
  bytes : u8

class Cloneable.AssociatedTypes (Self : Type) where

class Cloneable (Self : Type)
  [associatedTypes : outParam (Cloneable.AssociatedTypes (Self : Type))]
  where
  clone_it (Self) : (Self -> RustM Self)

@[spec]
def Impl.clone_it_hoisted (self : Token) : RustM Token := do
  (pure (Token.mk (bytes := (Token.bytes self))))

@[reducible] instance Impl.AssociatedTypes :
  Cloneable.AssociatedTypes Token
  where

instance Impl : Cloneable Token where
  clone_it := (Impl.clone_it_hoisted)

@[spec]
def duplicate (t : Token) : RustM Token := do (Cloneable.clone_it Token t)

@[spec]
def Impl_1.fresh (b : u8) : RustM Token := do (pure (Token.mk (bytes := b)))

@[spec]
def make_token (_ : rust_primitives.hax.Tuple0) : RustM Token := do
  (Impl_1.fresh (42 : u8))

@[spec]
def pass_through (t : Token) : RustM Token := do (id Token t)

end new_tests.legacy__proverif_generics__lib

