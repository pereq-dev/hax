
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


namespace new_tests.rustc_coverage__branch__guard

@[spec]
def branch_match_guard (x : (core_models.option.Option u32)) :
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
  match x with
    | (core_models.option.Option.Some  0) => do
      let _ ←
        (std.io.stdio._print
          (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
            (RustArray.ofVec #v["zero\n"]))));
      let _ := rust_primitives.hax.Tuple0.mk;
      (pure rust_primitives.hax.Tuple0.mk)
    | _ => do
      match
        (← match x with
          | (core_models.option.Option.Some  x) => do
            match (← ((← (x %? (2 : u32))) ==? (0 : u32))) with
              | true => do
                let _ ←
                  (std.io.stdio._print
                    (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
                      (RustArray.ofVec #v["is nonzero and even\n"]))));
                let _ := rust_primitives.hax.Tuple0.mk;
                (pure (core_models.option.Option.Some
                  rust_primitives.hax.Tuple0.mk))
              | _ => do (pure core_models.option.Option.None)
          | _ => do (pure core_models.option.Option.None))
      with
        | (core_models.option.Option.Some  x) => do (pure x)
        | (core_models.option.Option.None ) => do
          match
            (← match x with
              | (core_models.option.Option.Some  x) => do
                match (← ((← (x %? (3 : u32))) ==? (0 : u32))) with
                  | true => do
                    let _ ←
                      (std.io.stdio._print
                        (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
                          (RustArray.ofVec #v["is nonzero and odd, but divisible by 3\n"]))));
                    let _ := rust_primitives.hax.Tuple0.mk;
                    (pure (core_models.option.Option.Some
                      rust_primitives.hax.Tuple0.mk))
                  | _ => do (pure core_models.option.Option.None)
              | _ => do (pure core_models.option.Option.None))
          with
            | (core_models.option.Option.Some  x) => do (pure x)
            | (core_models.option.Option.None ) => do
              let _ ←
                (std.io.stdio._print
                  (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
                    (RustArray.ofVec #v["something else\n"]))));
              let _ := rust_primitives.hax.Tuple0.mk;
              (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ ← (branch_match_guard (core_models.option.Option.Some (0 : u32)));
  let _ ← (branch_match_guard (core_models.option.Option.Some (2 : u32)));
  let _ ← (branch_match_guard (core_models.option.Option.Some (6 : u32)));
  let _ ← (branch_match_guard (core_models.option.Option.Some (3 : u32)));
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.rustc_coverage__branch__guard

