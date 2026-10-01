module New_tests.Rustc_coverage__branch__guard
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let branch_match_guard (x: Core_models.Option.t_Option u32) : Prims.unit =
  let _:Prims.unit =
    Rust_primitives.Hax.Folds.fold_range (mk_i32 0)
      (mk_i32 1)
      (fun temp_0_ temp_1_ ->
          let _:Prims.unit = temp_0_ in
          let _:i32 = temp_1_ in
          true)
      ()
      (fun temp_0_ temp_1_ ->
          let _:Prims.unit = temp_0_ in
          let _:i32 = temp_1_ in
          ())
  in
  match x <: Core_models.Option.t_Option u32 with
  | Core_models.Option.Option_Some (Rust_primitives.Integers.MkInt 0) ->
    let _:Prims.unit =
      Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
            (let list = ["zero\n"] in
              FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
              Rust_primitives.Hax.array_of_list 1 list)
          <:
          Core_models.Fmt.t_Arguments)
    in
    let _:Prims.unit = () in
    ()
  | _ ->
    match
      (match x <: Core_models.Option.t_Option u32 with
        | Core_models.Option.Option_Some x ->
          (match (x %! mk_u32 2 <: u32) =. mk_u32 0 <: bool with
            | true ->
              let _:Prims.unit =
                Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
                      (let list = ["is nonzero and even\n"] in
                        FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                        Rust_primitives.Hax.array_of_list 1 list)
                    <:
                    Core_models.Fmt.t_Arguments)
              in
              let _:Prims.unit = () in
              Core_models.Option.Option_Some () <: Core_models.Option.t_Option Prims.unit
            | _ -> Core_models.Option.Option_None <: Core_models.Option.t_Option Prims.unit)
        | _ -> Core_models.Option.Option_None <: Core_models.Option.t_Option Prims.unit)
      <:
      Core_models.Option.t_Option Prims.unit
    with
    | Core_models.Option.Option_Some x -> x
    | Core_models.Option.Option_None  ->
      match
        (match x <: Core_models.Option.t_Option u32 with
          | Core_models.Option.Option_Some x ->
            (match (x %! mk_u32 3 <: u32) =. mk_u32 0 <: bool with
              | true ->
                let _:Prims.unit =
                  Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
                        (let list = ["is nonzero and odd, but divisible by 3\n"] in
                          FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                          Rust_primitives.Hax.array_of_list 1 list)
                      <:
                      Core_models.Fmt.t_Arguments)
                in
                let _:Prims.unit = () in
                Core_models.Option.Option_Some () <: Core_models.Option.t_Option Prims.unit
              | _ -> Core_models.Option.Option_None <: Core_models.Option.t_Option Prims.unit)
          | _ -> Core_models.Option.Option_None <: Core_models.Option.t_Option Prims.unit)
        <:
        Core_models.Option.t_Option Prims.unit
      with
      | Core_models.Option.Option_Some x -> x
      | Core_models.Option.Option_None  ->
        let _:Prims.unit =
          Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
                (let list = ["something else\n"] in
                  FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                  Rust_primitives.Hax.array_of_list 1 list)
              <:
              Core_models.Fmt.t_Arguments)
        in
        let _:Prims.unit = () in
        ()

let main (_: Prims.unit) : Prims.unit =
  let _:Prims.unit =
    branch_match_guard (Core_models.Option.Option_Some (mk_u32 0) <: Core_models.Option.t_Option u32
      )
  in
  let _:Prims.unit =
    branch_match_guard (Core_models.Option.Option_Some (mk_u32 2) <: Core_models.Option.t_Option u32
      )
  in
  let _:Prims.unit =
    branch_match_guard (Core_models.Option.Option_Some (mk_u32 6) <: Core_models.Option.t_Option u32
      )
  in
  let _:Prims.unit =
    branch_match_guard (Core_models.Option.Option_Some (mk_u32 3) <: Core_models.Option.t_Option u32
      )
  in
  ()
