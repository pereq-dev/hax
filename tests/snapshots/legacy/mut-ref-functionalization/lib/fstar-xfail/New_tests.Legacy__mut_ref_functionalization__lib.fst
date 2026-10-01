module New_tests.Legacy__mut_ref_functionalization__lib
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

type t_S = { f_b:t_Array u8 (mk_usize 5) }

let foo (lhs rhs: t_S) : t_S =
  let lhs:t_S =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      (mk_usize 1)
      (fun lhs temp_1_ ->
          let lhs:t_S = lhs in
          let _:usize = temp_1_ in
          true)
      lhs
      (fun lhs i ->
          let lhs:t_S = lhs in
          let i:usize = i in
          {
            lhs with
            f_b
            =
            Rust_primitives.Hax.Monomorphized_update_at.update_at_usize lhs.f_b
              i
              ((lhs.f_b.[ i ] <: u8) +! (rhs.f_b.[ i ] <: u8) <: u8)
            <:
            t_Array u8 (mk_usize 5)
          }
          <:
          t_S)
  in
  lhs

let impl_S__update (self: t_S) (x: u8) : t_S =
  let self:t_S =
    {
      self with
      f_b = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize self.f_b (mk_usize 0) x
    }
    <:
    t_S
  in
  self

let index_mutation (x: Core_models.Ops.Range.t_Range usize) (a: t_Slice u8) : Prims.unit =
  let v:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global =
    Alloc.Slice.impl__into_vec #u8
      #Alloc.Alloc.t_Global
      ((let list = [mk_u8 1] in
          FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
          Rust_primitives.Hax.array_of_list 1 list)
        <:
        t_Slice u8)
  in
  let v:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global =
    Alloc.Slice.impl__to_vec (Rust_primitives.Hax.Monomorphized_update_at.update_at_range (Alloc.Vec.impl_1__as_slice
              v
            <:
            t_Slice u8)
          x
          (Core_models.Slice.impl__copy_from_slice #u8 (v.[ x ] <: t_Slice u8) a <: t_Slice u8)
        <:
        t_Slice u8)
  in
  let v:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global =
    Alloc.Slice.impl__to_vec (Rust_primitives.Hax.Monomorphized_update_at.update_at_usize (Alloc.Vec.impl_1__as_slice
              v
            <:
            t_Slice u8)
          (mk_usize 1)
          (mk_u8 3)
        <:
        t_Slice u8)
  in
  ()

let index_mutation_unsize (x: t_Array u8 (mk_usize 12)) : u8 =
  let x:t_Array u8 (mk_usize 12) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range x
      ({ Core_models.Ops.Range.f_start = mk_usize 4; Core_models.Ops.Range.f_end = mk_usize 5 }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (x.[ {
                Core_models.Ops.Range.f_start = mk_usize 4;
                Core_models.Ops.Range.f_end = mk_usize 5
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
          ((let list = [mk_u8 1; mk_u8 2] in
              FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 2);
              Rust_primitives.Hax.array_of_list 2 list)
            <:
            t_Slice u8)
        <:
        t_Slice u8)
  in
  mk_u8 42

let build_vec (_: Prims.unit) : Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global =
  Alloc.Slice.impl__into_vec #u8
    #Alloc.Alloc.t_Global
    ((let list = [mk_u8 1; mk_u8 2; mk_u8 3] in
        FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 3);
        Rust_primitives.Hax.array_of_list 3 list)
      <:
      t_Slice u8)

let test_append (_: Prims.unit) : Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global =
  let vec1:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global = Alloc.Vec.impl__new #u8 () in
  let vec2:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global =
    Alloc.Slice.impl__into_vec #u8
      #Alloc.Alloc.t_Global
      ((let list = [mk_u8 1; mk_u8 2; mk_u8 3] in
          FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 3);
          Rust_primitives.Hax.array_of_list 3 list)
        <:
        t_Slice u8)
  in
  let
  (tmp0: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global), (tmp1: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global) =
    Alloc.Vec.impl_1__append #u8 #Alloc.Alloc.t_Global vec1 vec2
  in
  let vec1:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global = tmp0 in
  let vec2:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global = tmp1 in
  let _:Prims.unit = () in
  let vec1:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global =
    Alloc.Vec.impl_1__append #u8
      #Alloc.Alloc.t_Global
      vec1
      (build_vec () <: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
  in
  vec1

let f (_: Prims.unit) : Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global =
  let vec:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global = Alloc.Vec.impl__new #u8 () in
  let vec:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global =
    Alloc.Vec.impl_1__push #u8 #Alloc.Alloc.t_Global vec (mk_u8 1)
  in
  let vec:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global =
    Alloc.Vec.impl_1__push #u8 #Alloc.Alloc.t_Global vec (mk_u8 2)
  in
  let vec:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global =
    Alloc.Slice.impl__to_vec (Core_models.Slice.impl__swap #u8
          (Alloc.Vec.impl_1__as_slice vec <: t_Slice u8)
          (mk_usize 0)
          (mk_usize 1)
        <:
        t_Slice u8)
  in
  let vec:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global =
    Alloc.Slice.impl__to_vec (Core_models.Slice.impl__swap #u8
          (Alloc.Vec.impl_1__as_slice vec <: t_Slice u8)
          (mk_usize 0)
          (mk_usize 1)
        <:
        t_Slice u8)
  in
  vec

type t_Foo = { f_field:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global }

type t_Pair (v_T: Type0) = {
  f_a:v_T;
  f_b:t_Foo
}

let g (x: t_Pair (Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global))
    : Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global =
  let x:t_Pair (Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global) = x in
  let x:t_Pair (Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global) =
    Rust_primitives.Hax.Folds.fold_range (mk_u8 1)
      (mk_u8 10)
      (fun x temp_1_ ->
          let x:t_Pair (Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global) = x in
          let _:u8 = temp_1_ in
          true)
      x
      (fun x i ->
          let x:t_Pair (Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global) = x in
          let i:u8 = i in
          {
            x with
            f_a
            =
            Alloc.Vec.impl_1__push #u8 #Alloc.Alloc.t_Global x.f_a i
            <:
            Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global
          }
          <:
          t_Pair (Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global))
  in
  let x:t_Pair (Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global) =
    {
      x with
      f_a
      =
      Alloc.Slice.impl__to_vec (Core_models.Slice.impl__swap #u8
            (Alloc.Vec.impl_1__as_slice x.f_a <: t_Slice u8)
            (mk_usize 0)
            (mk_usize 1)
          <:
          t_Slice u8)
    }
    <:
    t_Pair (Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
  in
  let x:t_Pair (Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global) =
    {
      x with
      f_b
      =
      {
        x.f_b with
        f_field
        =
        Alloc.Slice.impl__to_vec (Core_models.Slice.impl__swap #u8
              (Alloc.Vec.impl_1__as_slice x.f_b.f_field <: t_Slice u8)
              (mk_usize 0)
              (mk_usize 1)
            <:
            t_Slice u8)
      }
      <:
      t_Foo
    }
    <:
    t_Pair (Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
  in
  x.f_a

let h (x: u8) : u8 =
  let x:u8 = x +! mk_u8 10 in
  x

type t_Bar = {
  f_a:u8;
  f_b:u8
}

let i (bar: t_Bar) : (t_Bar & u8) =
  let bar:t_Bar = { bar with f_b = bar.f_b +! bar.f_a } <: t_Bar in
  let bar:t_Bar = { bar with f_a = h bar.f_a } <: t_Bar in
  let hax_temp_output:u8 = bar.f_a +! bar.f_b in
  bar, hax_temp_output <: (t_Bar & u8)

let j (x: t_Bar) : (t_Bar & u8) =
  let out:u8 = mk_u8 123 in
  let (tmp0: t_Bar), (out1: u8) = i x in
  let x:t_Bar = tmp0 in
  let hax_temp_output:u8 = out1 +! out in
  x, hax_temp_output <: (t_Bar & u8)

let k
      (vec: Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
      (arg_1_wild3: u16)
      (arg_1_wild: u8)
      (arg_3_wild2: Prims.unit)
    : (Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global & u16 & Prims.unit & u64) =
  let arg_1_wild2:u8 = vec.[ mk_usize 1 ] in
  let arg_3_wild:u8 = vec.[ mk_usize 2 ] in
  let arg_1_wild1:u8 = vec.[ mk_usize 3 ] in
  let arg_3_wild1:u8 = vec.[ mk_usize 4 ] in
  let vec:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global =
    Alloc.Slice.impl__to_vec (Rust_primitives.Hax.Monomorphized_update_at.update_at_usize (Alloc.Vec.impl_1__as_slice
              vec
            <:
            t_Slice u8)
          (mk_usize 0)
          ((((arg_1_wild +! arg_3_wild <: u8) +! arg_1_wild1 <: u8) +! arg_3_wild1 <: u8) +!
            arg_1_wild
            <:
            u8)
        <:
        t_Slice u8)
  in
  let hax_temp_output:u64 = mk_u64 12345 in
  vec, arg_1_wild3, arg_3_wild2, hax_temp_output
  <:
  (Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global & u16 & Prims.unit & u64)

class t_FooTrait (v_Self: Type0) = {
  f_z_pre:v_Self -> Type0;
  f_z_post:v_Self -> v_Self -> Type0;
  f_z:x0: v_Self -> Prims.Pure v_Self (f_z_pre x0) (fun result -> f_z_post x0 result)
}

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_FooTrait_for_Foo: t_FooTrait t_Foo =
  {
    f_z_pre = (fun (self: t_Foo) -> true);
    f_z_post = (fun (self: t_Foo) (out: t_Foo) -> true);
    f_z = fun (self: t_Foo) -> self
  }

let array (x: t_Array u8 (mk_usize 10)) : t_Array u8 (mk_usize 10) =
  let x:t_Array u8 (mk_usize 10) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize x
      (mk_usize 1)
      (x.[ mk_usize 2 ] <: u8)
  in
  x
