module New_tests.Rustc_coverage__while_early_ret
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

/// @fail(extraction): ssprove(HAX0001), coq(HAX0001, HAX0001)
let main (_: Prims.unit) : Core_models.Result.t_Result Prims.unit u8 =
  let countdown:i32 = mk_i32 10 in
  match
    Rust_primitives.Hax.while_loop_return (fun countdown ->
          let countdown:i32 = countdown in
          true)
      (fun countdown ->
          let countdown:i32 = countdown in
          countdown >. mk_i32 0 <: bool)
      (fun countdown ->
          let countdown:i32 = countdown in
          Rust_primitives.Hax.Int.from_machine (mk_u32 0) <: Hax_lib.Int.t_Int)
      countdown
      (fun countdown ->
          let countdown:i32 = countdown in
          if countdown <. mk_i32 5 <: bool
          then
            Core_models.Ops.Control_flow.ControlFlow_Break
            (Core_models.Ops.Control_flow.ControlFlow_Break
              (if countdown >. mk_i32 8 <: bool
                then
                  Core_models.Result.Result_Ok (() <: Prims.unit)
                  <:
                  Core_models.Result.t_Result Prims.unit u8
                else
                  Core_models.Result.Result_Err (mk_u8 1)
                  <:
                  Core_models.Result.t_Result Prims.unit u8)
              <:
              Core_models.Ops.Control_flow.t_ControlFlow (Core_models.Result.t_Result Prims.unit u8)
                (Prims.unit & i32))
            <:
            Core_models.Ops.Control_flow.t_ControlFlow
              (Core_models.Ops.Control_flow.t_ControlFlow
                  (Core_models.Result.t_Result Prims.unit u8) (Prims.unit & i32)) i32
          else
            Core_models.Ops.Control_flow.ControlFlow_Continue (countdown -! mk_i32 1 <: i32)
            <:
            Core_models.Ops.Control_flow.t_ControlFlow
              (Core_models.Ops.Control_flow.t_ControlFlow
                  (Core_models.Result.t_Result Prims.unit u8) (Prims.unit & i32)) i32)
    <:
    Core_models.Ops.Control_flow.t_ControlFlow (Core_models.Result.t_Result Prims.unit u8) i32
  with
  | Core_models.Ops.Control_flow.ControlFlow_Break ret -> ret
  | Core_models.Ops.Control_flow.ControlFlow_Continue countdown ->
    Core_models.Result.Result_Ok (() <: Prims.unit) <: Core_models.Result.t_Result Prims.unit u8
