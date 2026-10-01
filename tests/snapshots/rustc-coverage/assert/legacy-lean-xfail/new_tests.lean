
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


namespace new_tests.rustc_coverage__assert

--  @fail(extraction): ssprove(HAX0001)
@[spec]
def might_fail_assert (one_plus_one : u32) :
    RustM rust_primitives.hax.Tuple0 := do
  let args : (rust_primitives.hax.Tuple1 u32) :=
    (rust_primitives.hax.Tuple1.mk one_plus_one);
  let args : (RustArray core_models.fmt.rt.Argument 1) :=
    (RustArray.ofVec #v[(← (core_models.fmt.rt.Impl.new_display u32
                            (rust_primitives.hax.Tuple1._0 args)))]);
  let _ ←
    (std.io.stdio._print
      (← (core_models.fmt.rt.Impl_1.new_v1 ((2 : usize)) ((1 : usize))
        (RustArray.ofVec #v["does 1 + 1 = ", "?\n"])
        args)));
  let _ := rust_primitives.hax.Tuple0.mk;
  let _ ←
    match
      (rust_primitives.hax.Tuple2.mk (← ((1 : u32) +? (1 : u32))) one_plus_one)
    with
      | ⟨left_val, right_val⟩ => do
        (hax_lib.assert (← (left_val ==? right_val)));
  (pure rust_primitives.hax.Tuple0.mk)

--  @fail(extraction): ssprove(HAX0001), coq(HAX0001, HAX0001)
@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM (core_models.result.Result rust_primitives.hax.Tuple0 u8) := do
  let countdown : i32 := (10 : i32);
  let countdown : i32 ←
    (rust_primitives.hax.while_loop
      (fun countdown => (do (pure true) : RustM Bool))
      (fun countdown => (do (countdown >? (0 : i32)) : RustM Bool))
      (fun countdown =>
        (do
        (rust_primitives.hax.int.from_machine (0 : u32)) :
        RustM hax_lib.int.Int))
      countdown
      (fun countdown =>
        (do
        let _ ←
          if (← (countdown ==? (1 : i32))) then do
            let _ ← (might_fail_assert (3 : u32));
            (pure rust_primitives.hax.Tuple0.mk)
          else do
            if (← (countdown <? (5 : i32))) then do
              let _ ← (might_fail_assert (2 : u32));
              (pure rust_primitives.hax.Tuple0.mk)
            else do
              (pure rust_primitives.hax.Tuple0.mk);
        let countdown : i32 ← (countdown -? (1 : i32));
        (pure countdown) :
        RustM i32)));
  (pure (core_models.result.Result.Ok rust_primitives.hax.Tuple0.mk))

end new_tests.rustc_coverage__assert

