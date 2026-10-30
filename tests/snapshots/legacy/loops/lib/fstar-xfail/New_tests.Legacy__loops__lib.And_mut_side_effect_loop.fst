module New_tests.Legacy__loops__lib.And_mut_side_effect_loop
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let looping (array: t_Array u8 (mk_usize 5)) : t_Array u8 (mk_usize 5) =
  let array:t_Array u8 (mk_usize 5) =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      (Core_models.Slice.impl__len #u8 (array <: t_Slice u8) <: usize)
      (fun array temp_1_ ->
          let array:t_Array u8 (mk_usize 5) = array in
          let _:usize = temp_1_ in
          true)
      array
      (fun array i ->
          let array:t_Array u8 (mk_usize 5) = array in
          let i:usize = i in
          Rust_primitives.Hax.Monomorphized_update_at.update_at_usize array
            i
            (cast (i <: usize) <: u8)
          <:
          t_Array u8 (mk_usize 5))
  in
  array

let looping_2_ (array: t_Array u8 (mk_usize 5)) : t_Array u8 (mk_usize 5) =
  let (array: t_Array u8 (mk_usize 5)), (result: Prims.unit) =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      (Core_models.Slice.impl__len #u8 (array <: t_Slice u8) <: usize)
      (fun array temp_1_ ->
          let array:t_Array u8 (mk_usize 5) = array in
          let _:usize = temp_1_ in
          true)
      array
      (fun array i ->
          let array:t_Array u8 (mk_usize 5) = array in
          let i:usize = i in
          Rust_primitives.Hax.Monomorphized_update_at.update_at_usize array
            i
            (cast (i <: usize) <: u8)
          <:
          t_Array u8 (mk_usize 5)),
    ()
    <:
    (t_Array u8 (mk_usize 5) & Prims.unit)
  in
  let _:Prims.unit = admit () (* Panic freedom *) in
  let _:Prims.unit = result in
  array
