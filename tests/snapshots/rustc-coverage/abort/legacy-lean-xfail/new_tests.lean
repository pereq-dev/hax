
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


namespace new_tests.rustc_coverage__abort

@[spec]
def might_abort (should_abort : Bool) : RustM rust_primitives.hax.Tuple0 := do
  if should_abort then do
    let _ ←
      (std.io.stdio._print
        (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
          (RustArray.ofVec #v["aborting...\n"]))));
    let _ := rust_primitives.hax.Tuple0.mk;
    (rust_primitives.hax.never_to_any
      (← (core_models.panicking.panic_fmt
        (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
          (RustArray.ofVec #v["panics and aborts"]))))))
  else do
    let _ ←
      (std.io.stdio._print
        (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
          (RustArray.ofVec #v["Don\'t Panic\n"]))));
    let _ := rust_primitives.hax.Tuple0.mk;
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
          if (← (countdown <? (5 : i32))) then do
            let _ ← (might_abort false);
            (pure rust_primitives.hax.Tuple0.mk)
          else do
            (pure rust_primitives.hax.Tuple0.mk);
        let _ ←
          if (← (countdown <? (5 : i32))) then do
            let _ ← (might_abort false);
            (pure rust_primitives.hax.Tuple0.mk)
          else do
            (pure rust_primitives.hax.Tuple0.mk);
        let _ ←
          if (← (countdown <? (5 : i32))) then do
            let _ ← (might_abort false);
            (pure rust_primitives.hax.Tuple0.mk)
          else do
            (pure rust_primitives.hax.Tuple0.mk);
        let countdown : i32 ← (countdown -? (1 : i32));
        (pure countdown) :
        RustM i32)));
  (pure (core_models.result.Result.Ok rust_primitives.hax.Tuple0.mk))

end new_tests.rustc_coverage__abort

