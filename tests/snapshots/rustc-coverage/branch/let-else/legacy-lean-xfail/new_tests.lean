
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


namespace new_tests.rustc_coverage__branch__let_else

@[spec]
def say (message : String) : RustM rust_primitives.hax.Tuple0 := do
  let _ ← (core_models.hint.black_box String message);
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def let_else (value : (core_models.option.Option String)) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ ←
    (rust_primitives.hax.folds.fold_range
      (0 : i32)
      (1 : i32)
      (fun _ _ => (do (pure true) : RustM Bool))
      rust_primitives.hax.Tuple0.mk
      (fun _ _ =>
        (do
        (pure rust_primitives.hax.Tuple0.mk) :
        RustM rust_primitives.hax.Tuple0)));
  match value with
    | (core_models.option.Option.Some  x) => do
      let _ ← (say x);
      (pure rust_primitives.hax.Tuple0.mk)
    | _ => do let _ ← (say "none"); (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ ← (let_else (core_models.option.Option.Some "x"));
  let _ ← (let_else (core_models.option.Option.Some "x"));
  let _ ← (let_else core_models.option.Option.None);
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.rustc_coverage__branch__let_else

