
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


namespace new_tests.rustc_coverage__mcdc__condition_limit

--  @fail(extraction): coq(HAX0001), fstar(HAX0001), ssprove(HAX0001), legacy-lean(HAX0001)
--  @fail(extraction): proverif(HAX0001)
@[spec]
def accept_7_conditions (bool_arr : (RustArray Bool 7)) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure sorry)

@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ ←
    (accept_7_conditions (← (rust_primitives.hax.repeat false (7 : usize))));
  let _ ←
    (accept_7_conditions (← (rust_primitives.hax.repeat true (7 : usize))));
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.rustc_coverage__mcdc__condition_limit

