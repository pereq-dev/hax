
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


namespace new_tests.rustc_coverage__coroutine

@[spec]
def get_u32 (val : Bool) :
    RustM (core_models.result.Result u32 alloc.string.String) := do
  if val then do
    (pure (core_models.result.Result.Ok (1 : u32)))
  else do
    (pure (core_models.result.Result.Err
      (← (core_models.convert.From._from
        alloc.string.String
        String "some error"))))

--  @fail(extraction): coq(HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001), ssprove(HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001), legacy-lean(HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001), fstar(HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001)
--  @fail(extraction): proverif(HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001)
@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure sorry)

end new_tests.rustc_coverage__coroutine

