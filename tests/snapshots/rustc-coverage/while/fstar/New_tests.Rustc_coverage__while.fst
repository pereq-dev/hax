module New_tests.Rustc_coverage__while
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

/// @fail(extraction): ssprove(HAX0001), coq(HAX0001, HAX0001)
let main (_: Prims.unit) : (Prims.unit & Prims.unit) =
  let num:i32 = mk_i32 9 in
  Rust_primitives.Hax.while_loop (fun temp_0_ ->
        let _:Prims.unit = temp_0_ in
        true)
    (fun temp_0_ ->
        let _:Prims.unit = temp_0_ in
        num >=. mk_i32 10 <: bool)
    (fun temp_0_ ->
        let _:Prims.unit = temp_0_ in
        Rust_primitives.Hax.Int.from_machine (mk_u32 0) <: Hax_lib.Int.t_Int)
    ()
    (fun temp_0_ ->
        let _:Prims.unit = temp_0_ in
        ()),
  ()
  <:
  (Prims.unit & Prims.unit)
