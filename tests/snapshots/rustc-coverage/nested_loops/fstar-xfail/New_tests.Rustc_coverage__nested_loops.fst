module New_tests.Rustc_coverage__nested_loops
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Std.Env in
  ()

/// @fail(extraction): ssprove(HAX0001, HAX0001, HAX0001), coq(HAX0001, HAX0001, HAX0001, HAX0001)
let main (_: Prims.unit) : (i32 & Prims.unit) =
  let is_true:bool =
    (Core_models.Iter.Traits.Exact_size.f_len #Std.Env.t_Args
        #FStar.Tactics.Typeclasses.solve
        (Std.Env.args () <: Std.Env.t_Args)
      <:
      usize) =.
    mk_usize 1
  in
  let countdown:i32 = mk_i32 10 in
  Rust_primitives.Hax.while_loop (fun countdown ->
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
        let a:i32 = mk_i32 100 in
        let b:i32 = mk_i32 100 in
        let (a: i32), (b: i32) =
          Rust_primitives.Hax.Folds.fold_range_cf (mk_i32 0)
            (mk_i32 50)
            (fun temp_0_ temp_1_ ->
                let (a: i32), (b: i32) = temp_0_ in
                let _:i32 = temp_1_ in
                true)
            (a, b <: (i32 & i32))
            (fun temp_0_ temp_1_ ->
                let (a: i32), (b: i32) = temp_0_ in
                let _:i32 = temp_1_ in
                if a <. mk_i32 30 <: bool
                then
                  Core_models.Ops.Control_flow.ControlFlow_Break
                  ((), (a, b <: (i32 & i32)) <: (Prims.unit & (i32 & i32)))
                  <:
                  Core_models.Ops.Control_flow.t_ControlFlow (Prims.unit & (i32 & i32)) (i32 & i32)
                else
                  let a:i32 = a -! mk_i32 5 in
                  let b:i32 = b -! mk_i32 5 in
                  if b <. mk_i32 90
                  then
                    let a:i32 = a -! mk_i32 10 in
                    if is_true
                    then
                      Core_models.Ops.Control_flow.ControlFlow_Break
                      ((), (a, b <: (i32 & i32)) <: (Prims.unit & (i32 & i32)))
                      <:
                      Core_models.Ops.Control_flow.t_ControlFlow (Prims.unit & (i32 & i32))
                        (i32 & i32)
                    else
                      let a:i32 = a -! mk_i32 2 in
                      Core_models.Ops.Control_flow.ControlFlow_Continue (a, b <: (i32 & i32))
                      <:
                      Core_models.Ops.Control_flow.t_ControlFlow (Prims.unit & (i32 & i32))
                        (i32 & i32)
                  else
                    Core_models.Ops.Control_flow.ControlFlow_Continue (a, b <: (i32 & i32))
                    <:
                    Core_models.Ops.Control_flow.t_ControlFlow (Prims.unit & (i32 & i32))
                      (i32 & i32))
        in
        countdown -! mk_i32 1),
  ()
  <:
  (i32 & Prims.unit)
