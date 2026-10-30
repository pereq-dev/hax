module New_tests.Rustc_coverage__branch__lazy_boolean
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

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
  let c:bool = a && b in
  let _:bool = Core_models.Hint.black_box #bool c in
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
  let c:bool = a || b in
  let _:bool = Core_models.Hint.black_box #bool c in
  ()

let chain (x: u32) : Prims.unit =
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
  let c:bool = x >. mk_u32 1 && x >. mk_u32 2 && x >. mk_u32 4 && x >. mk_u32 8 in
  let _:bool = Core_models.Hint.black_box #bool c in
  let d:bool = x <. mk_u32 1 || x <. mk_u32 2 || x <. mk_u32 4 || x <. mk_u32 8 in
  let _:bool = Core_models.Hint.black_box #bool d in
  ()

let nested_mixed (x: u32) : Prims.unit =
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
  let c:bool = (x <. mk_u32 4 || x >=. mk_u32 9) && (x <. mk_u32 2 || x >=. mk_u32 10) in
  let _:bool = Core_models.Hint.black_box #bool c in
  let d:bool = x <. mk_u32 4 && x <. mk_u32 1 || x >=. mk_u32 8 && x >=. mk_u32 10 in
  let _:bool = Core_models.Hint.black_box #bool d in
  ()

let main (_: Prims.unit) : (Prims.unit & Prims.unit) =
  let _:Prims.unit =
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
          Prims.unit)
  in
  Rust_primitives.Hax.Folds.fold_range (mk_u32 0)
    (mk_u32 16)
    (fun temp_0_ temp_1_ ->
        let _:Prims.unit = temp_0_ in
        let _:u32 = temp_1_ in
        true)
    ()
    (fun temp_0_ x ->
        let _:Prims.unit = temp_0_ in
        let x:u32 = x in
        let _:Prims.unit = chain x in
        let _:Prims.unit = nested_mixed x in
        ()),
  ()
  <:
  (Prims.unit & Prims.unit)
