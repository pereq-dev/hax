
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


namespace new_tests.rustc_coverage__branch__if_let

@[spec]
def say (message : String) : RustM rust_primitives.hax.Tuple0 := do
  let _ ← (core_models.hint.black_box String message);
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def if_let (input : (core_models.option.Option String)) :
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
  let _ ←
    match input with
      | (core_models.option.Option.Some  x) => do
        let _ ← (say x);
        (pure rust_primitives.hax.Tuple0.mk)
      | _ => do let _ ← (say "none"); (pure rust_primitives.hax.Tuple0.mk);
  let _ ← (say "done");
  (pure rust_primitives.hax.Tuple0.mk)

--  @fail(extraction): coq(HAX0001, HAX0001), ssprove(HAX0001, HAX0001), legacy-lean(HAX0001, HAX0001), fstar(HAX0001, HAX0001)
--  @fail(extraction): proverif(HAX0001, HAX0001)
@[spec]
def if_let_chain
    (a : (core_models.option.Option String))
    (b : (core_models.option.Option String)) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ ←
    if (← (sorry &&? sorry)) then do
      let _ ← (say x);
      let _ ← (say y);
      (pure rust_primitives.hax.Tuple0.mk)
    else do
      let _ ← (say "not both");
      (pure rust_primitives.hax.Tuple0.mk);
  let _ ← (say "done");
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ ← (if_let (core_models.option.Option.Some "x"));
  let _ ← (if_let (core_models.option.Option.Some "x"));
  let _ ← (if_let core_models.option.Option.None);
  let _ ←
    (rust_primitives.hax.folds.fold_range
      (0 : i32)
      (8 : i32)
      (fun _ _ => (do (pure true) : RustM Bool))
      rust_primitives.hax.Tuple0.mk
      (fun _ _ =>
        (do
        (if_let_chain
          (core_models.option.Option.Some "a")
          (core_models.option.Option.Some "b")) :
        RustM rust_primitives.hax.Tuple0)));
  let _ ←
    (rust_primitives.hax.folds.fold_range
      (0 : i32)
      (4 : i32)
      (fun _ _ => (do (pure true) : RustM Bool))
      rust_primitives.hax.Tuple0.mk
      (fun _ _ =>
        (do
        (if_let_chain
          (core_models.option.Option.Some "a")
          core_models.option.Option.None) :
        RustM rust_primitives.hax.Tuple0)));
  let _ ←
    (rust_primitives.hax.folds.fold_range
      (0 : i32)
      (2 : i32)
      (fun _ _ => (do (pure true) : RustM Bool))
      rust_primitives.hax.Tuple0.mk
      (fun _ _ =>
        (do
        (if_let_chain
          core_models.option.Option.None
          (core_models.option.Option.Some "b")) :
        RustM rust_primitives.hax.Tuple0)));
  let _ ←
    (if_let_chain
      core_models.option.Option.None
      core_models.option.Option.None);
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.rustc_coverage__branch__if_let

