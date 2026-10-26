
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


namespace new_tests.legacy__range_inclusive_signed__lib

--  @fail(extraction): proverif(HAX0008)
@[spec]
def signed_range (_ : rust_primitives.hax.Tuple0) : RustM i32 := do
  let sum : i32 := (0 : i32);
  let sum : i32 ←
    (rust_primitives.hax.folds.fold_range_inclusive
      (-3 : i32)
      (3 : i32)
      (fun sum _ => (do (pure true) : RustM Bool))
      sum
      (fun sum i => (do (sum +? i) : RustM i32)));
  (pure sum)

end new_tests.legacy__range_inclusive_signed__lib

