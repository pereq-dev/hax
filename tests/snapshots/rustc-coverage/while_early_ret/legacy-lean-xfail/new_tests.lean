
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


namespace new_tests.rustc_coverage__while_early_ret

--  @fail(extraction): ssprove(HAX0001), coq(HAX0001, HAX0001)
@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM (core_models.result.Result rust_primitives.hax.Tuple0 u8) := do
  let countdown : i32 := (10 : i32);
  match
    (← (rust_primitives.hax.while_loop_return
      (fun countdown => (do (pure true) : RustM Bool))
      (fun countdown => (do (countdown >? (0 : i32)) : RustM Bool))
      (fun countdown =>
        (do
        (rust_primitives.hax.int.from_machine (0 : u32)) :
        RustM hax_lib.int.Int))
      countdown
      (fun countdown =>
        (do
        if (← (countdown <? (5 : i32))) then do
          (pure (core_models.ops.control_flow.ControlFlow.Break
            (core_models.ops.control_flow.ControlFlow.Break
              (← if (← (countdown >? (8 : i32))) then do
                (pure (core_models.result.Result.Ok
                  rust_primitives.hax.Tuple0.mk))
              else do
                (pure (core_models.result.Result.Err (1 : u8)))))))
        else do
          (pure (core_models.ops.control_flow.ControlFlow.Continue
            (← (countdown -? (1 : i32))))) :
        RustM
        (core_models.ops.control_flow.ControlFlow
          (core_models.ops.control_flow.ControlFlow
            (core_models.result.Result rust_primitives.hax.Tuple0 u8)
            (rust_primitives.hax.Tuple2 rust_primitives.hax.Tuple0 i32))
          i32)))))
  with
    | (core_models.ops.control_flow.ControlFlow.Break  ret) => do (pure ret)
    | (core_models.ops.control_flow.ControlFlow.Continue  countdown) => do
      (pure (core_models.result.Result.Ok rust_primitives.hax.Tuple0.mk))

end new_tests.rustc_coverage__while_early_ret

