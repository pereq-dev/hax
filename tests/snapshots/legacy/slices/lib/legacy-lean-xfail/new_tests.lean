
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


namespace new_tests.legacy__slices__lib

def VERSION : (RustSlice u8) :=
  RustM.of_isOk
    (do (rust_primitives.unsize (RustArray.ofVec #v[(118 : u8), (49 : u8)])))
    (by rfl)

@[spec]
def do_something (_ : (RustSlice u8)) : RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def unsized (_ : (RustArray (RustSlice u8) 1)) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def sized (x : (RustArray (RustArray u8 4) 1)) :
    RustM rust_primitives.hax.Tuple0 := do
  (unsized
    (RustArray.ofVec #v[(← (rust_primitives.unsize (← x[(0 : usize)]_?)))]))

end new_tests.legacy__slices__lib

