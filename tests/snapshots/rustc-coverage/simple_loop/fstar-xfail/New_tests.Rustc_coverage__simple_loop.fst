module New_tests.Rustc_coverage__simple_loop
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Std.Env in
  ()

/// @fail(extraction): fstar(HAX0001), coq(HAX0001, HAX0001), legacy-lean(HAX0001), proverif(HAX0008), ssprove(HAX0001)
let main (_: Prims.unit) : (i32 & Prims.unit) =
  let is_true:bool =
    (Core_models.Iter.Traits.Exact_size.f_len #Std.Env.t_Args
        #FStar.Tactics.Typeclasses.solve
        (Std.Env.args () <: Std.Env.t_Args)
      <:
      usize) =.
    mk_usize 1
  in
  let countdown:i32 = mk_i32 0 in
  let countdown:i32 =
    if is_true
    then
      let countdown:i32 = mk_i32 10 in
      countdown
    else countdown
  in
  Rust_primitives.Hax.failure "something is not implemented yet.\nUnhandled loop kind\n\nThis is discussed in issue https://github.com/pereq-dev/hax/issues/933.\nPlease upvote or comment this issue if you see this error message.\nNote: the error was labeled with context `FunctionalizeLoops`.\n"
    "{\n (loop {\n |countdown| {\n (if rust_primitives::hax::machine_int::eq(countdown, 0) {\n core_models::ops::control_flow::ControlFlow_Break(\n Tuple2(Tuple0, countdown),\n )\n } else {\n core_models::ops::con..."
  ,
  ()
  <:
  (i32 & Prims.unit)
