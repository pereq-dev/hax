
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


namespace new_tests.rustc_coverage__continue

--  @fail(extraction): coq(HAX0008, HAX0008, HAX0008, HAX0008, HAX0001), ssprove(HAX0008, HAX0008, HAX0008, HAX0008, HAX0001)
@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let is_true : Bool ←
    ((← (core_models.iter.traits.exact_size.ExactSizeIterator.len
        std.env.Args (← (std.env.args rust_primitives.hax.Tuple0.mk))))
      ==? (1 : usize));
  let x : i32 := (0 : i32);
  let x : i32 ←
    (rust_primitives.hax.folds.fold_range
      (0 : i32)
      (10 : i32)
      (fun x _ => (do (pure true) : RustM Bool))
      x
      (fun x _ =>
        (do
        match is_true with
          | true => do (pure x)
          | _ => do let x : i32 := (1 : i32); (pure (3 : i32)) :
        RustM i32)));
  let x : i32 ←
    (rust_primitives.hax.folds.fold_range
      (0 : i32)
      (10 : i32)
      (fun x _ => (do (pure true) : RustM Bool))
      x
      (fun x _ =>
        (do
        match is_true with
          | false => do let x : i32 := (1 : i32); (pure (3 : i32))
          | _ => do (pure x) :
        RustM i32)));
  let x : i32 ←
    (rust_primitives.hax.folds.fold_range
      (0 : i32)
      (10 : i32)
      (fun x _ => (do (pure true) : RustM Bool))
      x
      (fun x _ =>
        (do
        match is_true with
          | true => do let x : i32 := (1 : i32); (pure (3 : i32))
          | _ => do (pure x) :
        RustM i32)));
  let x : i32 ←
    (rust_primitives.hax.folds.fold_range
      (0 : i32)
      (10 : i32)
      (fun x _ => (do (pure true) : RustM Bool))
      x
      (fun x _ =>
        (do if is_true then do (pure x) else do (pure (3 : i32)) : RustM i32)));
  let x : i32 ←
    (rust_primitives.hax.folds.fold_range
      (0 : i32)
      (10 : i32)
      (fun x _ => (do (pure true) : RustM Bool))
      x
      (fun x _ =>
        (do
        let x : i32 ←
          match is_true with
            | false => do let x : i32 := (1 : i32); (pure x)
            | _ => do let _ := x; (pure x);
        let x : i32 := (3 : i32);
        (pure x) :
        RustM i32)));
  let x : i32 ←
    (rust_primitives.hax.folds.fold_range_cf
      (0 : i32)
      (10 : i32)
      (fun x _ => (do (pure true) : RustM Bool))
      x
      (fun x _ =>
        (do
        match is_true with
          | false => do
            let x : i32 := (1 : i32);
            (pure (core_models.ops.control_flow.ControlFlow.Continue (3 : i32)))
          | _ => do
            (pure (core_models.ops.control_flow.ControlFlow.Break
              (rust_primitives.hax.Tuple2.mk rust_primitives.hax.Tuple0.mk x)))
        :
        RustM
        (core_models.ops.control_flow.ControlFlow
          (rust_primitives.hax.Tuple2 rust_primitives.hax.Tuple0 i32)
          i32))));
  let _ := x;
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.rustc_coverage__continue

