module New_tests.Legacy__generics__lib
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let dup
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Clone.t_Clone v_T)
      (x: v_T)
    : (v_T & v_T) =
  Core_models.Clone.f_clone #v_T #FStar.Tactics.Typeclasses.solve x,
  Core_models.Clone.f_clone #v_T #FStar.Tactics.Typeclasses.solve x
  <:
  (v_T & v_T)

let foo (v_LEN: usize) (arr: t_Array usize v_LEN) : usize =
  let acc:usize = v_LEN +! mk_usize 9 in
  let acc:usize =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      v_LEN
      (fun acc temp_1_ ->
          let acc:usize = acc in
          let _:usize = temp_1_ in
          true)
      acc
      (fun acc i ->
          let acc:usize = acc in
          let i:usize = i in
          acc +! (arr.[ i ] <: usize) <: usize)
  in
  acc

let repeat
      (v_LEN: usize)
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Marker.t_Copy v_T)
      (x: v_T)
    : t_Array v_T v_LEN = Rust_primitives.Hax.repeat x v_LEN

let f (v_N x: usize) : usize = (v_N +! v_N <: usize) +! x

let call_f (_: Prims.unit) : usize = (f (mk_usize 10) (mk_usize 3) <: usize) +! mk_usize 3

let g
      (v_N: usize)
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Core_models.Convert.t_Into v_T (t_Array usize v_N))
      (arr: v_T)
    : usize =
  (Core_models.Option.impl__unwrap_or #usize
      (Core_models.Iter.Traits.Iterator.f_max #(Core_models.Array.Iter.t_IntoIter usize v_N)
          #FStar.Tactics.Typeclasses.solve
          (Core_models.Iter.Traits.Collect.f_into_iter #(t_Array usize v_N)
              #FStar.Tactics.Typeclasses.solve
              (Core_models.Convert.f_into #v_T
                  #(t_Array usize v_N)
                  #FStar.Tactics.Typeclasses.solve
                  arr
                <:
                t_Array usize v_N)
            <:
            Core_models.Array.Iter.t_IntoIter usize v_N)
        <:
        Core_models.Option.t_Option usize)
      v_N
    <:
    usize) +!
  v_N

let call_g (_: Prims.unit) : usize =
  (g (mk_usize 3)
      #(t_Array usize (mk_usize 3))
      (let list = [mk_usize 42; mk_usize 3; mk_usize 49] in
        FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 3);
        Rust_primitives.Hax.array_of_list 3 list)
    <:
    usize) +!
  mk_usize 3

class t_Foo (v_Self: Type0) = {
  f_const_add_pre:v_N: usize -> v_Self -> Type0;
  f_const_add_post:v_N: usize -> v_Self -> usize -> Type0;
  f_const_add:v_N: usize -> x0: v_Self
    -> Prims.Pure usize (f_const_add_pre v_N x0) (fun result -> f_const_add_post v_N x0 result)
}

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl: t_Foo usize =
  {
    f_const_add_pre = (fun (v_N: usize) (self: usize) -> true);
    f_const_add_post = (fun (v_N: usize) (self: usize) (out: usize) -> true);
    f_const_add = fun (v_N: usize) (self: usize) -> self +! v_N
  }

type t_Bar = | Bar : t_Bar

let impl_Bar__inherent_impl_generics (#v_T: Type0) (v_N: usize) (x: t_Array v_T v_N) : Prims.unit =
  ()
