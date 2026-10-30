
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


namespace new_tests.rustc_coverage__simple_loop

--  @fail(extraction): fstar(HAX0001), coq(HAX0001, HAX0001), legacy-lean(HAX0001), ssprove(HAX0001)
@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM (rust_primitives.hax.Tuple2 i32 rust_primitives.hax.Tuple0) := do
  let is_true : Bool ←
    ((← (core_models.iter.traits.exact_size.ExactSizeIterator.len
        std.env.Args (← (std.env.args rust_primitives.hax.Tuple0.mk))))
      ==? (1 : usize));
  let countdown : i32 := (0 : i32);
  let countdown : i32 ←
    if is_true then do
      let countdown : i32 := (10 : i32);
      (pure countdown)
    else do
      (pure countdown);
  (pure (rust_primitives.hax.Tuple2.mk sorry rust_primitives.hax.Tuple0.mk))

end new_tests.rustc_coverage__simple_loop

