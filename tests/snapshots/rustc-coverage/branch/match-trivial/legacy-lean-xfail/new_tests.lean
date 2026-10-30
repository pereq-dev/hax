
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


namespace new_tests.rustc_coverage__branch__match_trivial

inductive Uninhabited : Type


@[spec]
def Uninhabited_cast_to_repr (x : Uninhabited) :
    RustM rust_primitives.hax.Never := do
  match x with 

inductive Trivial : Type
| Value : Trivial

@[spec]
def Trivial_cast_to_repr (x : Trivial) : RustM isize := do
  match x with | (Trivial.Value ) => do (pure (0 : isize))

@[spec]
def consume (T : Type) (x : T) : RustM rust_primitives.hax.Tuple0 := do
  let _ ← (core_models.hint.black_box T x);
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def _uninhabited (x : Uninhabited) : RustM rust_primitives.hax.Tuple0 := do
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
  let _ ← (rust_primitives.hax.never_to_any (← match x with ));
  (rust_primitives.hax.never_to_any (← (consume String "done")))

@[spec]
def trivial (x : Trivial) : RustM rust_primitives.hax.Tuple0 := do
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
  let _ ← match x with | (Trivial.Value ) => do (consume String "trivial");
  let _ ← (consume String "done");
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ ← (trivial Trivial.Value);
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.rustc_coverage__branch__match_trivial

