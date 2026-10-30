
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


namespace new_tests.rustc_coverage__color

@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM
    (rust_primitives.hax.Tuple2
      rust_primitives.hax.Tuple0
      rust_primitives.hax.Tuple0)
    := do
  (pure (rust_primitives.hax.Tuple2.mk
    (← (rust_primitives.hax.folds.fold_range
      (0 : i32)
      (0 : i32)
      (fun _ _ => (do (pure true) : RustM Bool))
      rust_primitives.hax.Tuple0.mk
      (fun _ _i =>
        (do
        (pure rust_primitives.hax.Tuple0.mk) :
        RustM rust_primitives.hax.Tuple0))))
    rust_primitives.hax.Tuple0.mk))

end new_tests.rustc_coverage__color

