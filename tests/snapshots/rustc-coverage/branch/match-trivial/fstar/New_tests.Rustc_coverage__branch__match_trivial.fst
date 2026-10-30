module New_tests.Rustc_coverage__branch__match_trivial
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

type t_Uninhabited =

let t_Uninhabited_cast_to_repr (x: t_Uninhabited) : Rust_primitives.Hax.t_Never =
  match x <: t_Uninhabited with

type t_Trivial = | Trivial_Value : t_Trivial

let t_Trivial_cast_to_repr (x: t_Trivial) : isize =
  match x <: t_Trivial with | Trivial_Value  -> mk_isize 0

let consume (#v_T: Type0) (x: v_T) : Prims.unit =
  let _:v_T = Core_models.Hint.black_box #v_T x in
  ()

let e_uninhabited (x: t_Uninhabited) : Prims.unit =
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
  let _:Prims.unit = Rust_primitives.Hax.never_to_any (match x <: t_Uninhabited with ) in
  Rust_primitives.Hax.never_to_any (consume #string "done" <: Prims.unit)

let trivial (x: t_Trivial) : Prims.unit =
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
  let _:Prims.unit = match x <: t_Trivial with | Trivial_Value  -> consume #string "trivial" in
  let _:Prims.unit = consume #string "done" in
  ()

let main (_: Prims.unit) : Prims.unit =
  let _:Prims.unit = trivial (Trivial_Value <: t_Trivial) in
  ()
