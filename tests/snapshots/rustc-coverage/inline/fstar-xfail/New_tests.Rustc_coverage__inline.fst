module New_tests.Rustc_coverage__inline
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let length (#v_T: Type0) (xs: t_Slice v_T) : usize = Core_models.Slice.impl__len #v_T xs

let swap
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Marker.t_Copy v_T)
      (xs: t_Slice v_T)
      (i j: usize)
    : t_Slice v_T =
  let t:v_T = xs.[ i ] in
  let xs:t_Slice v_T =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize xs i (xs.[ j ] <: v_T)
  in
  let xs:t_Slice v_T = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize xs j t in
  xs

/// @fail(extraction): ssprove(HAX0001)
let display
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Fmt.t_Display v_T)
      (xs: t_Slice v_T)
    : Prims.unit =
  let _:Prims.unit =
    Core_models.Iter.Traits.Iterator.f_fold (Core_models.Iter.Traits.Collect.f_into_iter #(t_Slice
            v_T)
          #FStar.Tactics.Typeclasses.solve
          xs
        <:
        Core_models.Slice.Iter.t_Iter v_T)
      ()
      (fun temp_0_ x ->
          let _:Prims.unit = temp_0_ in
          let x:v_T = x in
          let args:v_T = x <: v_T in
          let args:t_Array Core_models.Fmt.Rt.t_Argument (mk_usize 1) =
            let list = [Core_models.Fmt.Rt.impl__new_display #v_T args] in
            FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
            Rust_primitives.Hax.array_of_list 1 list
          in
          let _:Prims.unit =
            Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_v1 (mk_usize 1)
                  (mk_usize 1)
                  (let list = [""] in
                    FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                    Rust_primitives.Hax.array_of_list 1 list)
                  args
                <:
                Core_models.Fmt.t_Arguments)
          in
          ())
  in
  let _:Prims.unit =
    Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
          (let list = ["\n"] in
            FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
            Rust_primitives.Hax.array_of_list 1 list)
        <:
        Core_models.Fmt.t_Arguments)
  in
  let _:Prims.unit = () in
  ()

let error (_: Prims.unit) : Prims.unit =
  Rust_primitives.Hax.never_to_any (Core_models.Panicking.panic_fmt (Core_models.Fmt.Rt.impl_1__new_const
            (mk_usize 1)
            (let list = ["error"] in
              FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
              Rust_primitives.Hax.array_of_list 1 list)
          <:
          Core_models.Fmt.t_Arguments)
      <:
      Rust_primitives.Hax.t_Never)

let rec permutate
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Marker.t_Copy v_T)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: Core_models.Fmt.t_Display v_T)
      (xs: t_Slice v_T)
      (k: usize)
    : t_Slice v_T =
  let n:usize = length #v_T xs in
  let xs:t_Slice v_T =
    if k =. n
    then
      let _:Prims.unit = display #v_T xs in
      xs
    else
      if k <. n
      then
        Rust_primitives.Hax.Folds.fold_range k
          n
          (fun xs temp_1_ ->
              let xs:t_Slice v_T = xs in
              let _:usize = temp_1_ in
              true)
          xs
          (fun xs i ->
              let xs:t_Slice v_T = xs in
              let i:usize = i in
              let xs:t_Slice v_T = swap #v_T xs i k in
              let xs:t_Slice v_T = permutate #v_T xs (k +! mk_usize 1 <: usize) in
              let xs:t_Slice v_T = swap #v_T xs i k in
              xs)
      else
        let _:Prims.unit = error () in
        xs
  in
  xs

let permutations
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Marker.t_Copy v_T)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: Core_models.Fmt.t_Display v_T)
      (xs: t_Slice v_T)
    : Prims.unit =
  let ys:Alloc.Vec.t_Vec v_T Alloc.Alloc.t_Global =
    Alloc.Borrow.f_to_owned #(t_Slice v_T) #FStar.Tactics.Typeclasses.solve xs
  in
  let ys:Alloc.Vec.t_Vec v_T Alloc.Alloc.t_Global =
    Alloc.Slice.impl__to_vec (permutate #v_T
          (Alloc.Vec.impl_1__as_slice ys <: t_Slice v_T)
          (mk_usize 0)
        <:
        t_Slice v_T)
  in
  ()

/// @fail(extraction): ssprove(HAX0001)
let main (_: Prims.unit) : Prims.unit =
  let _:Prims.unit =
    permutations #FStar.Char.char
      ((let list = ['a'; 'b'; 'c'] in
          FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 3);
          Rust_primitives.Hax.array_of_list 3 list)
        <:
        t_Slice FStar.Char.char)
  in
  ()
