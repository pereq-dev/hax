module New_tests.Rustc_coverage__abort
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let might_abort (should_abort: bool) : Prims.unit =
  if should_abort
  then
    let _:Prims.unit =
      Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
            (let list = ["aborting...\n"] in
              FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
              Rust_primitives.Hax.array_of_list 1 list)
          <:
          Core_models.Fmt.t_Arguments)
    in
    let _:Prims.unit = () in
    Rust_primitives.Hax.never_to_any (Core_models.Panicking.panic_fmt (Core_models.Fmt.Rt.impl_1__new_const
              (mk_usize 1)
              (let list = ["panics and aborts"] in
                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                Rust_primitives.Hax.array_of_list 1 list)
            <:
            Core_models.Fmt.t_Arguments)
        <:
        Rust_primitives.Hax.t_Never)
  else
    let _:Prims.unit =
      Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
            (let list = ["Don't Panic\n"] in
              FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
              Rust_primitives.Hax.array_of_list 1 list)
          <:
          Core_models.Fmt.t_Arguments)
    in
    let _:Prims.unit = () in
    ()

/// @fail(extraction): ssprove(HAX0001), coq(HAX0001, HAX0001)
let main (_: Prims.unit) : Core_models.Result.t_Result Prims.unit u8 =
  let countdown:i32 = mk_i32 10 in
  let countdown:i32 =
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
          let _:Prims.unit =
            if countdown <. mk_i32 5
            then
              let _:Prims.unit = might_abort false in
              ()
          in
          let _:Prims.unit =
            if countdown <. mk_i32 5
            then
              let _:Prims.unit = might_abort false in
              ()
          in
          let _:Prims.unit =
            if countdown <. mk_i32 5
            then
              let _:Prims.unit = might_abort false in
              ()
          in
          let countdown:i32 = countdown -! mk_i32 1 in
          countdown)
  in
  Core_models.Result.Result_Ok (() <: Prims.unit) <: Core_models.Result.t_Result Prims.unit u8
