
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


namespace new_tests.rustc_coverage__attr__trait_impl_inherit

--  @fail(extraction): ssprove(HAX0008), fstar(HAX0008), coq(HAX0008)
class T.AssociatedTypes (Self : Type) where

class T (Self : Type)
  [associatedTypes : outParam (T.AssociatedTypes (Self : Type))]
  where
  f (Self) (self : Self) :RustM rust_primitives.hax.Tuple0 := do
    let _ ←
      (std.io.stdio._print
        (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
          (RustArray.ofVec #v["default\n"]))));
    let _ := rust_primitives.hax.Tuple0.mk;
    (pure rust_primitives.hax.Tuple0.mk)

structure S where
  -- no fields

@[spec]
def Impl.f_hoisted (self : S) : RustM rust_primitives.hax.Tuple0 := do
  let _ ←
    (std.io.stdio._print
      (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
        (RustArray.ofVec #v["impl S\n"]))));
  let _ := rust_primitives.hax.Tuple0.mk;
  (pure rust_primitives.hax.Tuple0.mk)

@[reducible] instance Impl.AssociatedTypes : T.AssociatedTypes S where

instance Impl : T S where
  f := (Impl.f_hoisted)

@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ ← (T.f S S.mk);
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.rustc_coverage__attr__trait_impl_inherit

