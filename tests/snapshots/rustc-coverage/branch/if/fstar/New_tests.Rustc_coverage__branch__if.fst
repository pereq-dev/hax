module New_tests.Rustc_coverage__branch__if
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let say (message: string) : Prims.unit =
  let _:string = Core_models.Hint.black_box #string message in
  ()

let branch_not (a: bool) : Prims.unit =
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
  let _:Prims.unit = if a then say "a" in
  let _:Prims.unit =
    if ~.a
    then
      let _:Prims.unit = say "not a" in
      ()
  in
  let _:Prims.unit =
    if ~.(~.a <: bool)
    then
      let _:Prims.unit = say "not not a" in
      ()
  in
  if ~.(~.(~.a <: bool) <: bool)
  then
    let _:Prims.unit = say "not not not a" in
    ()

let branch_not_as (a: bool) : Prims.unit =
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
  let _:Prims.unit =
    if ~.a
    then
      let _:Prims.unit = say "not (a as bool)" in
      ()
  in
  let _:Prims.unit =
    if ~.(~.a <: bool)
    then
      let _:Prims.unit = say "not not (a as bool)" in
      ()
  in
  if ~.(~.(~.a <: bool) <: bool)
  then
    let _:Prims.unit = say "not not (a as bool)" in
    ()

let branch_and (a b: bool) : Prims.unit =
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
  if a && b
  then
    let _:Prims.unit = say "both" in
    ()
  else
    let _:Prims.unit = say "not both" in
    ()

let branch_or (a b: bool) : Prims.unit =
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
  if a || b
  then
    let _:Prims.unit = say "either" in
    ()
  else
    let _:Prims.unit = say "neither" in
    ()

let main (_: Prims.unit) : (Prims.unit & Prims.unit) =
  let _:Prims.unit =
    Core_models.Iter.Traits.Iterator.f_fold (Core_models.Iter.Traits.Collect.f_into_iter #(t_Array
              bool (mk_usize 3))
          #FStar.Tactics.Typeclasses.solve
          (let list = [false; true; true] in
            FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 3);
            Rust_primitives.Hax.array_of_list 3 list)
        <:
        Core_models.Array.Iter.t_IntoIter bool (mk_usize 3))
      ()
      (fun temp_0_ a ->
          let _:Prims.unit = temp_0_ in
          let a:bool = a in
          let _:Prims.unit = branch_not a in
          let _:Prims.unit = branch_not_as a in
          ())
  in
  Core_models.Iter.Traits.Iterator.f_fold (Core_models.Iter.Traits.Collect.f_into_iter #(t_Array
            bool (mk_usize 5))
        #FStar.Tactics.Typeclasses.solve
        (let list = [false; true; true; true; true] in
          FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 5);
          Rust_primitives.Hax.array_of_list 5 list)
      <:
      Core_models.Array.Iter.t_IntoIter bool (mk_usize 5))
    ()
    (fun temp_0_ a ->
        let _:Prims.unit = temp_0_ in
        let a:bool = a in
        Core_models.Iter.Traits.Iterator.f_fold (Core_models.Iter.Traits.Collect.f_into_iter #(t_Array
                  bool (mk_usize 3))
              #FStar.Tactics.Typeclasses.solve
              (let list = [false; true; true] in
                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 3);
                Rust_primitives.Hax.array_of_list 3 list)
            <:
            Core_models.Array.Iter.t_IntoIter bool (mk_usize 3))
          ()
          (fun temp_0_ b ->
              let _:Prims.unit = temp_0_ in
              let b:bool = b in
              let _:Prims.unit = branch_and a b in
              let _:Prims.unit = branch_or a b in
              ())
        <:
        Prims.unit),
  ()
  <:
  (Prims.unit & Prims.unit)
