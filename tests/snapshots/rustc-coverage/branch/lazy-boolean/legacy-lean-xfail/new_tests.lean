
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


namespace new_tests.rustc_coverage__branch__lazy_boolean

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
  let c : Bool ← (a &&? b);
  let _ ← (core_models.hint.black_box Bool c);
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
  let c : Bool ← (a ||? b);
  let _ ← (core_models.hint.black_box Bool c);
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def chain (x : u32) : RustM rust_primitives.hax.Tuple0 := do
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
  let c : Bool ←
    ((← ((← ((← (x >? (1 : u32))) &&? (← (x >? (2 : u32)))))
        &&? (← (x >? (4 : u32)))))
      &&? (← (x >? (8 : u32))));
  let _ ← (core_models.hint.black_box Bool c);
  let d : Bool ←
    ((← ((← ((← (x <? (1 : u32))) ||? (← (x <? (2 : u32)))))
        ||? (← (x <? (4 : u32)))))
      ||? (← (x <? (8 : u32))));
  let _ ← (core_models.hint.black_box Bool d);
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def nested_mixed (x : u32) : RustM rust_primitives.hax.Tuple0 := do
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
  let c : Bool ←
    ((← ((← (x <? (4 : u32))) ||? (← (x >=? (9 : u32)))))
      &&? (← ((← (x <? (2 : u32))) ||? (← (x >=? (10 : u32))))));
  let _ ← (core_models.hint.black_box Bool c);
  let d : Bool ←
    ((← ((← (x <? (4 : u32))) &&? (← (x <? (1 : u32)))))
      ||? (← ((← (x >=? (8 : u32))) &&? (← (x >=? (10 : u32))))));
  let _ ← (core_models.hint.black_box Bool d);
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
        RustM rust_primitives.hax.Tuple0)));
  (pure (rust_primitives.hax.Tuple2.mk
    (← (rust_primitives.hax.folds.fold_range
      (0 : u32)
      (16 : u32)
      (fun _ _ => (do (pure true) : RustM Bool))
      rust_primitives.hax.Tuple0.mk
      (fun _ x =>
        (do
        let _ ← (chain x);
        let _ ← (nested_mixed x);
        (pure rust_primitives.hax.Tuple0.mk) :
        RustM rust_primitives.hax.Tuple0))))
    rust_primitives.hax.Tuple0.mk))

end new_tests.rustc_coverage__branch__lazy_boolean

