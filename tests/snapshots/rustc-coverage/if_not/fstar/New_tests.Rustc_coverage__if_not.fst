module New_tests.Rustc_coverage__if_not
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let if_not (cond: bool) : Prims.unit =
  let _:Prims.unit =
    if ~.cond
    then
      let _:Prims.unit =
        Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
              (let list = ["cond was false\n"] in
                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                Rust_primitives.Hax.array_of_list 1 list)
            <:
            Core_models.Fmt.t_Arguments)
      in
      let _:Prims.unit = () in
      ()
  in
  let _:Prims.unit =
    if ~.cond
    then
      let _:Prims.unit =
        Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
              (let list = ["cond was false\n"] in
                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                Rust_primitives.Hax.array_of_list 1 list)
            <:
            Core_models.Fmt.t_Arguments)
      in
      let _:Prims.unit = () in
      ()
  in
  if ~.cond
  then
    let _:Prims.unit =
      Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
            (let list = ["cond was false\n"] in
              FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
              Rust_primitives.Hax.array_of_list 1 list)
          <:
          Core_models.Fmt.t_Arguments)
    in
    let _:Prims.unit = () in
    ()
  else
    let _:Prims.unit =
      Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
            (let list = ["cond was true\n"] in
              FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
              Rust_primitives.Hax.array_of_list 1 list)
          <:
          Core_models.Fmt.t_Arguments)
    in
    let _:Prims.unit = () in
    ()

let main (_: Prims.unit) : (Prims.unit & Prims.unit) =
  let _:Prims.unit =
    Rust_primitives.Hax.Folds.fold_range (mk_i32 0)
      (mk_i32 8)
      (fun temp_0_ temp_1_ ->
          let _:Prims.unit = temp_0_ in
          let _:i32 = temp_1_ in
          true)
      ()
      (fun temp_0_ temp_1_ ->
          let _:Prims.unit = temp_0_ in
          let _:i32 = temp_1_ in
          if_not (Core_models.Hint.black_box #bool true <: bool) <: Prims.unit)
  in
  Rust_primitives.Hax.Folds.fold_range (mk_i32 0)
    (mk_i32 4)
    (fun temp_0_ temp_1_ ->
        let _:Prims.unit = temp_0_ in
        let _:i32 = temp_1_ in
        true)
    ()
    (fun temp_0_ temp_1_ ->
        let _:Prims.unit = temp_0_ in
        let _:i32 = temp_1_ in
        if_not (Core_models.Hint.black_box #bool false <: bool) <: Prims.unit),
  ()
  <:
  (Prims.unit & Prims.unit)
