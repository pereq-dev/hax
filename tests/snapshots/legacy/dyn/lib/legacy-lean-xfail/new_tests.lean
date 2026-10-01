
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


namespace new_tests.legacy__dyn__lib

class Printable.AssociatedTypes (Self : Type) (S : Type) where

class Printable (Self : Type) (S : Type)
  [associatedTypes : outParam (Printable.AssociatedTypes (Self : Type) (S :
      Type))]
  where
  stringify (Self) (S) : (Self -> RustM S)

@[spec]
def Impl.stringify_hoisted (self : i32) : RustM alloc.string.String := do
  (alloc.string.ToString.to_string i32 self)

@[reducible] instance Impl.AssociatedTypes :
  Printable.AssociatedTypes i32 alloc.string.String
  where

instance Impl : Printable i32 alloc.string.String where
  stringify := (Impl.stringify_hoisted)

--  @fail(extraction): ssprove(HAX0008), coq(HAX0008)
--  @fail(extraction): proverif(HAX0008)
@[spec]
def print (a : sorry) : RustM rust_primitives.hax.Tuple0 := do
  let args : (rust_primitives.hax.Tuple1 alloc.string.String) :=
    (rust_primitives.hax.Tuple1.mk
      (← (Printable.stringify sorry alloc.string.String a)));
  let args : (RustArray core_models.fmt.rt.Argument 1) :=
    (RustArray.ofVec #v[(← (core_models.fmt.rt.Impl.new_display
                            alloc.string.String
                            (rust_primitives.hax.Tuple1._0 args)))]);
  let _ ←
    (std.io.stdio._print
      (← (core_models.fmt.rt.Impl_1.new_v1 ((2 : usize)) ((1 : usize))
        (RustArray.ofVec #v["", "\n"])
        args)));
  let _ := rust_primitives.hax.Tuple0.mk;
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.legacy__dyn__lib

