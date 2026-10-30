
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


namespace new_tests.rustc_coverage__branch__if

@[spec]
def say (message : String) : RustM rust_primitives.hax.Tuple0 := do
  let _ ← (core_models.hint.black_box String message);
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def branch_not (a : Bool) : RustM rust_primitives.hax.Tuple0 := do
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
  let _ ← if a then do (say "a") else do (pure rust_primitives.hax.Tuple0.mk);
  let _ ←
    if (← (!? a)) then do
      let _ ← (say "not a");
      (pure rust_primitives.hax.Tuple0.mk)
    else do
      (pure rust_primitives.hax.Tuple0.mk);
  let _ ←
    if (← (!? (← (!? a)))) then do
      let _ ← (say "not not a");
      (pure rust_primitives.hax.Tuple0.mk)
    else do
      (pure rust_primitives.hax.Tuple0.mk);
  if (← (!? (← (!? (← (!? a)))))) then do
    let _ ← (say "not not not a");
    (pure rust_primitives.hax.Tuple0.mk)
  else do
    (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def branch_not_as (a : Bool) : RustM rust_primitives.hax.Tuple0 := do
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
    if (← (!? a)) then do
      let _ ← (say "not (a as bool)");
      (pure rust_primitives.hax.Tuple0.mk)
    else do
      (pure rust_primitives.hax.Tuple0.mk);
  let _ ←
    if (← (!? (← (!? a)))) then do
      let _ ← (say "not not (a as bool)");
      (pure rust_primitives.hax.Tuple0.mk)
    else do
      (pure rust_primitives.hax.Tuple0.mk);
  if (← (!? (← (!? (← (!? a)))))) then do
    let _ ← (say "not not (a as bool)");
    (pure rust_primitives.hax.Tuple0.mk)
  else do
    (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def branch_and (a : Bool) (b : Bool) : RustM rust_primitives.hax.Tuple0 := do
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
  if (← (a &&? b)) then do
    let _ ← (say "both");
    (pure rust_primitives.hax.Tuple0.mk)
  else do
    let _ ← (say "not both");
    (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def branch_or (a : Bool) (b : Bool) : RustM rust_primitives.hax.Tuple0 := do
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
  if (← (a ||? b)) then do
    let _ ← (say "either");
    (pure rust_primitives.hax.Tuple0.mk)
  else do
    let _ ← (say "neither");
    (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM
    (rust_primitives.hax.Tuple2
      rust_primitives.hax.Tuple0
      rust_primitives.hax.Tuple0)
    := do
  let _ ←
    (core_models.iter.traits.iterator.Iterator.fold
      (← (core_models.iter.traits.collect.IntoIterator.into_iter
        (RustArray Bool 3) (RustArray.ofVec #v[false, true, true])))
      rust_primitives.hax.Tuple0.mk
      (fun _ a =>
        (do
        let _ ← (branch_not a);
        let _ ← (branch_not_as a);
        (pure rust_primitives.hax.Tuple0.mk) :
        RustM rust_primitives.hax.Tuple0)));
  (pure (rust_primitives.hax.Tuple2.mk
    (← (core_models.iter.traits.iterator.Iterator.fold
      (← (core_models.iter.traits.collect.IntoIterator.into_iter
        (RustArray Bool 5) (RustArray.ofVec #v[false, true, true, true, true])))
      rust_primitives.hax.Tuple0.mk
      (fun _ a =>
        (do
        (core_models.iter.traits.iterator.Iterator.fold
          (← (core_models.iter.traits.collect.IntoIterator.into_iter
            (RustArray Bool 3) (RustArray.ofVec #v[false, true, true])))
          rust_primitives.hax.Tuple0.mk
          (fun _ b =>
            (do
            let _ ← (branch_and a b);
            let _ ← (branch_or a b);
            (pure rust_primitives.hax.Tuple0.mk) :
            RustM rust_primitives.hax.Tuple0))) :
        RustM rust_primitives.hax.Tuple0))))
    rust_primitives.hax.Tuple0.mk))

end new_tests.rustc_coverage__branch__if

