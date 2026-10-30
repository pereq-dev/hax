
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


namespace new_tests.rustc_coverage__overflow

--  @fail(extraction): ssprove(HAX0001)
@[spec]
def might_overflow (to_add : u32) : RustM u32 := do
  let _ ←
    if (← (to_add >? (5 : u32))) then do
      let _ ←
        (std.io.stdio._print
          (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
            (RustArray.ofVec #v["this will probably overflow\n"]))));
      let _ := rust_primitives.hax.Tuple0.mk;
      (pure rust_primitives.hax.Tuple0.mk)
    else do
      (pure rust_primitives.hax.Tuple0.mk);
  let add_to : u32 ← (core_models.num.Impl_8.MAX -? (5 : u32));
  let args : (rust_primitives.hax.Tuple2 u32 u32) :=
    (rust_primitives.hax.Tuple2.mk add_to to_add);
  let args : (RustArray core_models.fmt.rt.Argument 2) :=
    (RustArray.ofVec #v[(← (core_models.fmt.rt.Impl.new_display u32
                            (rust_primitives.hax.Tuple2._0 args))),
                          (← (core_models.fmt.rt.Impl.new_display u32
                            (rust_primitives.hax.Tuple2._1 args)))]);
  let _ ←
    (std.io.stdio._print
      (← (core_models.fmt.rt.Impl_1.new_v1 ((3 : usize)) ((2 : usize))
        (RustArray.ofVec #v["does ", " + ", " overflow?\n"])
        args)));
  let _ := rust_primitives.hax.Tuple0.mk;
  let result : u32 ← (to_add +? add_to);
  let _ ←
    (std.io.stdio._print
      (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
        (RustArray.ofVec #v["continuing after overflow check\n"]))));
  let _ := rust_primitives.hax.Tuple0.mk;
  (pure result)

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
            let result : u32 ← (might_overflow (10 : u32));
            let args : (rust_primitives.hax.Tuple1 u32) :=
              (rust_primitives.hax.Tuple1.mk result);
            let args : (RustArray core_models.fmt.rt.Argument 1) :=
              (RustArray.ofVec #v[(← (core_models.fmt.rt.Impl.new_display u32
                                      (rust_primitives.hax.Tuple1._0 args)))]);
            let _ ←
              (std.io.stdio._print
                (← (core_models.fmt.rt.Impl_1.new_v1 ((2 : usize)) ((1 : usize))
                  (RustArray.ofVec #v["Result: ", "\n"])
                  args)));
            let _ := rust_primitives.hax.Tuple0.mk;
            (pure rust_primitives.hax.Tuple0.mk)
          else do
            if (← (countdown <? (5 : i32))) then do
              let result : u32 ← (might_overflow (1 : u32));
              let args : (rust_primitives.hax.Tuple1 u32) :=
                (rust_primitives.hax.Tuple1.mk result);
              let args : (RustArray core_models.fmt.rt.Argument 1) :=
                (RustArray.ofVec #v[(← (core_models.fmt.rt.Impl.new_display u32
                                        (rust_primitives.hax.Tuple1._0
                                          args)))]);
              let _ ←
                (std.io.stdio._print
                  (← (core_models.fmt.rt.Impl_1.new_v1
                    ((2 : usize))
                    ((1 : usize))
                    (RustArray.ofVec #v["Result: ", "\n"])
                    args)));
              let _ := rust_primitives.hax.Tuple0.mk;
              (pure rust_primitives.hax.Tuple0.mk)
            else do
              (pure rust_primitives.hax.Tuple0.mk);
        let countdown : i32 ← (countdown -? (1 : i32));
        (pure countdown) :
        RustM i32)));
  (pure (core_models.result.Result.Ok rust_primitives.hax.Tuple0.mk))

end new_tests.rustc_coverage__overflow

