
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


namespace new_tests.rustc_coverage__while

--  @fail(extraction): ssprove(HAX0001), coq(HAX0001, HAX0001)
@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM
    (rust_primitives.hax.Tuple2
      rust_primitives.hax.Tuple0
      rust_primitives.hax.Tuple0)
    := do
  let num : i32 := (9 : i32);
  (pure (rust_primitives.hax.Tuple2.mk
    (← (rust_primitives.hax.while_loop
      (fun _ => (do (pure true) : RustM Bool))
      (fun _ => (do (num >=? (10 : i32)) : RustM Bool))
      (fun _ =>
        (do
        (rust_primitives.hax.int.from_machine (0 : u32)) :
        RustM hax_lib.int.Int))
      rust_primitives.hax.Tuple0.mk
      (fun _ =>
        (do
        (pure rust_primitives.hax.Tuple0.mk) :
        RustM rust_primitives.hax.Tuple0))))
    rust_primitives.hax.Tuple0.mk))

end new_tests.rustc_coverage__while

