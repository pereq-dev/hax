module New_tests.Rustc_coverage__overflow
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

/// @fail(extraction): ssprove(HAX0001)
let might_overflow (to_add: u32) : u32 =
  let _:Prims.unit =
    if to_add >. mk_u32 5
    then
      let _:Prims.unit =
        Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
              (let list = ["this will probably overflow\n"] in
                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                Rust_primitives.Hax.array_of_list 1 list)
            <:
            Core_models.Fmt.t_Arguments)
      in
      let _:Prims.unit = () in
      ()
  in
  let add_to:u32 = Core_models.Num.impl_u32__MAX -! mk_u32 5 in
  let args:(u32 & u32) = add_to, to_add <: (u32 & u32) in
  let args:t_Array Core_models.Fmt.Rt.t_Argument (mk_usize 2) =
    let list =
      [
        Core_models.Fmt.Rt.impl__new_display #u32 args._1;
        Core_models.Fmt.Rt.impl__new_display #u32 args._2
      ]
    in
    FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 2);
    Rust_primitives.Hax.array_of_list 2 list
  in
  let _:Prims.unit =
    Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_v1 (mk_usize 3)
          (mk_usize 2)
          (let list = ["does "; " + "; " overflow?\n"] in
            FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 3);
            Rust_primitives.Hax.array_of_list 3 list)
          args
        <:
        Core_models.Fmt.t_Arguments)
  in
  let _:Prims.unit = () in
  let result:u32 = to_add +! add_to in
  let _:Prims.unit =
    Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
          (let list = ["continuing after overflow check\n"] in
            FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
            Rust_primitives.Hax.array_of_list 1 list)
        <:
        Core_models.Fmt.t_Arguments)
  in
  let _:Prims.unit = () in
  result

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
            if countdown =. mk_i32 1
            then
              let result:u32 = might_overflow (mk_u32 10) in
              let args:u32 = result <: u32 in
              let args:t_Array Core_models.Fmt.Rt.t_Argument (mk_usize 1) =
                let list = [Core_models.Fmt.Rt.impl__new_display #u32 args] in
                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                Rust_primitives.Hax.array_of_list 1 list
              in
              let _:Prims.unit =
                Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_v1 (mk_usize 2)
                      (mk_usize 1)
                      (let list = ["Result: "; "\n"] in
                        FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 2);
                        Rust_primitives.Hax.array_of_list 2 list)
                      args
                    <:
                    Core_models.Fmt.t_Arguments)
              in
              let _:Prims.unit = () in
              ()
            else
              if countdown <. mk_i32 5
              then
                let result:u32 = might_overflow (mk_u32 1) in
                let args:u32 = result <: u32 in
                let args:t_Array Core_models.Fmt.Rt.t_Argument (mk_usize 1) =
                  let list = [Core_models.Fmt.Rt.impl__new_display #u32 args] in
                  FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                  Rust_primitives.Hax.array_of_list 1 list
                in
                let _:Prims.unit =
                  Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_v1 (mk_usize 2)
                        (mk_usize 1)
                        (let list = ["Result: "; "\n"] in
                          FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 2);
                          Rust_primitives.Hax.array_of_list 2 list)
                        args
                      <:
                      Core_models.Fmt.t_Arguments)
                in
                let _:Prims.unit = () in
                ()
          in
          let countdown:i32 = countdown -! mk_i32 1 in
          countdown)
  in
  Core_models.Result.Result_Ok (() <: Prims.unit) <: Core_models.Result.t_Result Prims.unit u8
