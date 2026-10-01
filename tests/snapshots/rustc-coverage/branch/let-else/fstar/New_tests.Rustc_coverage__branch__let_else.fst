module New_tests.Rustc_coverage__branch__let_else
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let say (message: string) : Prims.unit =
  let _:string = Core_models.Hint.black_box #string message in
  ()

let let_else (value: Core_models.Option.t_Option string) : Prims.unit =
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
  match value <: Core_models.Option.t_Option string with
  | Core_models.Option.Option_Some x ->
    let _:Prims.unit = say x in
    ()
  | _ ->
    let _:Prims.unit = say "none" in
    ()

let main (_: Prims.unit) : Prims.unit =
  let _:Prims.unit =
    let_else (Core_models.Option.Option_Some "x" <: Core_models.Option.t_Option string)
  in
  let _:Prims.unit =
    let_else (Core_models.Option.Option_Some "x" <: Core_models.Option.t_Option string)
  in
  let _:Prims.unit =
    let_else (Core_models.Option.Option_None <: Core_models.Option.t_Option string)
  in
  ()
