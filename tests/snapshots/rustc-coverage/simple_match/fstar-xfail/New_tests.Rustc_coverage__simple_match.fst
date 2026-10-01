module New_tests.Rustc_coverage__simple_match
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Std.Env in
  ()

/// @fail(extraction): fstar(HAX0001), legacy-lean(HAX0001), ssprove(HAX0001), coq(HAX0001)
/// @fail(extraction): proverif(HAX0001)
let main (_: Prims.unit) : (Prims.unit & Prims.unit) =
  let is_true:bool =
    (Core_models.Iter.Traits.Exact_size.f_len #Std.Env.t_Args
        #FStar.Tactics.Typeclasses.solve
        (Std.Env.args () <: Std.Env.t_Args)
      <:
      usize) =.
    mk_usize 1
  in
  let countdown:i32 = mk_i32 1 in
  let countdown:i32 =
    if is_true
    then
      let countdown:i32 = mk_i32 0 in
      countdown
    else countdown
  in
  Rust_primitives.Hax.Folds.fold_range (mk_i32 0)
    (mk_i32 2)
    (fun temp_0_ temp_1_ ->
        let _:Prims.unit = temp_0_ in
        let _:i32 = temp_1_ in
        true)
    ()
    (fun temp_0_ temp_1_ ->
        let _:Prims.unit = temp_0_ in
        let _:i32 = temp_1_ in
        Rust_primitives.Hax.failure "something is not implemented yet.\nSorry, Hax does not support declare-first let bindings (see https://doc.rust-lang.org/rust-by-example/variable_bindings/declare.html) for now.\n\nThis is discussed in issue https://github.com/pereq-dev/hax/issues/156.\nPlease upvote or comment this issue if you see this error message.\nNote: the error was labeled with context `AST import`.\n"
          ""
        <:
        Prims.unit),
  ()
  <:
  (Prims.unit & Prims.unit)
