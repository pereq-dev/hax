
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


namespace new_tests.rustc_coverage__if_not

@[spec]
def if_not (cond : Bool) : RustM rust_primitives.hax.Tuple0 := do
  let _ ←
    if (← (!? cond)) then do
      let _ ←
        (std.io.stdio._print
          (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
            (RustArray.ofVec #v["cond was false\n"]))));
      let _ := rust_primitives.hax.Tuple0.mk;
      (pure rust_primitives.hax.Tuple0.mk)
    else do
      (pure rust_primitives.hax.Tuple0.mk);
  let _ ←
    if (← (!? cond)) then do
      let _ ←
        (std.io.stdio._print
          (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
            (RustArray.ofVec #v["cond was false\n"]))));
      let _ := rust_primitives.hax.Tuple0.mk;
      (pure rust_primitives.hax.Tuple0.mk)
    else do
      (pure rust_primitives.hax.Tuple0.mk);
  if (← (!? cond)) then do
    let _ ←
      (std.io.stdio._print
        (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
          (RustArray.ofVec #v["cond was false\n"]))));
    let _ := rust_primitives.hax.Tuple0.mk;
    (pure rust_primitives.hax.Tuple0.mk)
  else do
    let _ ←
      (std.io.stdio._print
        (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
          (RustArray.ofVec #v["cond was true\n"]))));
    let _ := rust_primitives.hax.Tuple0.mk;
    (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM
    (rust_primitives.hax.Tuple2
      rust_primitives.hax.Tuple0
      rust_primitives.hax.Tuple0)
    := do
  let _ ←
    (rust_primitives.hax.folds.fold_range
      (0 : i32)
      (8 : i32)
      (fun _ _ => (do (pure true) : RustM Bool))
      rust_primitives.hax.Tuple0.mk
      (fun _ _ =>
        (do
        (if_not (← (core_models.hint.black_box Bool true))) :
        RustM rust_primitives.hax.Tuple0)));
  (pure (rust_primitives.hax.Tuple2.mk
    (← (rust_primitives.hax.folds.fold_range
      (0 : i32)
      (4 : i32)
      (fun _ _ => (do (pure true) : RustM Bool))
      rust_primitives.hax.Tuple0.mk
      (fun _ _ =>
        (do
        (if_not (← (core_models.hint.black_box Bool false))) :
        RustM rust_primitives.hax.Tuple0))))
    rust_primitives.hax.Tuple0.mk))

end new_tests.rustc_coverage__if_not

