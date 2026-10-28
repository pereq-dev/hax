module New_tests.Rustc_coverage__mcdc__condition_limit
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

/// @fail(extraction): coq(HAX0001), proverif(HAX0001), fstar(HAX0001), ssprove(HAX0001), legacy-lean(HAX0001)
let accept_7_conditions (bool_arr: t_Array bool (mk_usize 7)) : Prims.unit =
  Rust_primitives.Hax.failure "something is not implemented yet.\nPat:Array\n\nThis is discussed in issue https://github.com/pereq-dev/hax/issues/804.\nPlease upvote or comment this issue if you see this error message.\nNote: the error was labeled with context `AST import`.\n"
    ""

let main (_: Prims.unit) : Prims.unit =
  let _:Prims.unit =
    accept_7_conditions (Rust_primitives.Hax.repeat false (mk_usize 7) <: t_Array bool (mk_usize 7))
  in
  let _:Prims.unit =
    accept_7_conditions (Rust_primitives.Hax.repeat true (mk_usize 7) <: t_Array bool (mk_usize 7))
  in
  ()
