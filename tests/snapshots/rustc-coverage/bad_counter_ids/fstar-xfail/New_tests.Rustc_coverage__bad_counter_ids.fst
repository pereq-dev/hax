module New_tests.Rustc_coverage__bad_counter_ids
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

type t_Foo = | Foo : u32 -> t_Foo

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl': Core_models.Fmt.t_Debug t_Foo

unfold
let impl = impl'

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_1': Core_models.Marker.t_StructuralPartialEq t_Foo

unfold
let impl_1 = impl_1'

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_2': Core_models.Cmp.t_PartialEq t_Foo t_Foo

unfold
let impl_2 = impl_2'

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_3': Core_models.Cmp.t_Eq t_Foo

unfold
let impl_3 = impl_3'

let eq_good (_: Prims.unit) : Prims.unit =
  let _:Prims.unit =
    Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
          (let list = ["a\n"] in
            FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
            Rust_primitives.Hax.array_of_list 1 list)
        <:
        Core_models.Fmt.t_Arguments)
  in
  let _:Prims.unit = () in
  let _:Prims.unit =
    match (Foo (mk_u32 1) <: t_Foo), (Foo (mk_u32 1) <: t_Foo) <: (t_Foo & t_Foo) with
    | left_val, right_val -> Hax_lib.v_assert (left_val =. right_val <: bool)
  in
  ()

let eq_good_message (_: Prims.unit) : Prims.unit =
  let _:Prims.unit =
    Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
          (let list = ["b\n"] in
            FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
            Rust_primitives.Hax.array_of_list 1 list)
        <:
        Core_models.Fmt.t_Arguments)
  in
  let _:Prims.unit = () in
  let _:Prims.unit =
    match (Foo (mk_u32 1) <: t_Foo), (Foo (mk_u32 1) <: t_Foo) <: (t_Foo & t_Foo) with
    | left_val, right_val -> Hax_lib.v_assert (left_val =. right_val <: bool)
  in
  ()

let ne_good (_: Prims.unit) : Prims.unit =
  let _:Prims.unit =
    Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
          (let list = ["c\n"] in
            FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
            Rust_primitives.Hax.array_of_list 1 list)
        <:
        Core_models.Fmt.t_Arguments)
  in
  let _:Prims.unit = () in
  let _:Prims.unit =
    match (Foo (mk_u32 1) <: t_Foo), (Foo (mk_u32 3) <: t_Foo) <: (t_Foo & t_Foo) with
    | left_val, right_val -> Hax_lib.v_assert (~.(left_val =. right_val <: bool) <: bool)
  in
  ()

let ne_good_message (_: Prims.unit) : Prims.unit =
  let _:Prims.unit =
    Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
          (let list = ["d\n"] in
            FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
            Rust_primitives.Hax.array_of_list 1 list)
        <:
        Core_models.Fmt.t_Arguments)
  in
  let _:Prims.unit = () in
  let _:Prims.unit =
    match (Foo (mk_u32 1) <: t_Foo), (Foo (mk_u32 3) <: t_Foo) <: (t_Foo & t_Foo) with
    | left_val, right_val -> Hax_lib.v_assert (~.(left_val =. right_val <: bool) <: bool)
  in
  ()

let eq_bad (_: Prims.unit) : Prims.unit =
  let _:Prims.unit =
    Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
          (let list = ["e\n"] in
            FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
            Rust_primitives.Hax.array_of_list 1 list)
        <:
        Core_models.Fmt.t_Arguments)
  in
  let _:Prims.unit = () in
  let _:Prims.unit =
    match (Foo (mk_u32 1) <: t_Foo), (Foo (mk_u32 3) <: t_Foo) <: (t_Foo & t_Foo) with
    | left_val, right_val -> Hax_lib.v_assert (left_val =. right_val <: bool)
  in
  ()

let eq_bad_message (_: Prims.unit) : Prims.unit =
  let _:Prims.unit =
    Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
          (let list = ["f\n"] in
            FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
            Rust_primitives.Hax.array_of_list 1 list)
        <:
        Core_models.Fmt.t_Arguments)
  in
  let _:Prims.unit = () in
  let _:Prims.unit =
    match (Foo (mk_u32 1) <: t_Foo), (Foo (mk_u32 3) <: t_Foo) <: (t_Foo & t_Foo) with
    | left_val, right_val -> Hax_lib.v_assert (left_val =. right_val <: bool)
  in
  ()

let ne_bad (_: Prims.unit) : Prims.unit =
  let _:Prims.unit =
    Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
          (let list = ["g\n"] in
            FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
            Rust_primitives.Hax.array_of_list 1 list)
        <:
        Core_models.Fmt.t_Arguments)
  in
  let _:Prims.unit = () in
  let _:Prims.unit =
    match (Foo (mk_u32 1) <: t_Foo), (Foo (mk_u32 1) <: t_Foo) <: (t_Foo & t_Foo) with
    | left_val, right_val -> Hax_lib.v_assert (~.(left_val =. right_val <: bool) <: bool)
  in
  ()

let ne_bad_message (_: Prims.unit) : Prims.unit =
  let _:Prims.unit =
    Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
          (let list = ["h\n"] in
            FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
            Rust_primitives.Hax.array_of_list 1 list)
        <:
        Core_models.Fmt.t_Arguments)
  in
  let _:Prims.unit = () in
  let _:Prims.unit =
    match (Foo (mk_u32 1) <: t_Foo), (Foo (mk_u32 1) <: t_Foo) <: (t_Foo & t_Foo) with
    | left_val, right_val -> Hax_lib.v_assert (~.(left_val =. right_val <: bool) <: bool)
  in
  ()

/// @fail(extraction): coq(HAX0008, HAX0008, HAX0008, HAX0008), ssprove(HAX0008, HAX0008, HAX0008, HAX0008)
/// @fail(extraction): proverif(HAX0008, HAX0008, HAX0008, HAX0008)
let main (_: Prims.unit) : Prims.unit =
  let _:Prims.unit = eq_good () in
  let _:Prims.unit = eq_good_message () in
  let _:Prims.unit = ne_good () in
  let _:Prims.unit = ne_good_message () in
  let _:Prims.unit =
    Hax_lib.v_assert (Core_models.Result.impl__is_err #Prims.unit
          #(dyn 2 (fun z -> Core_models.Any.t_Any z) (fun z -> Core_models.Marker.t_Send z))
          (Std.Panic.catch_unwind #(Prims.unit -> Prims.unit) #Prims.unit eq_bad
            <:
            Core_models.Result.t_Result Prims.unit
              (dyn 2 (fun z -> Core_models.Any.t_Any z) (fun z -> Core_models.Marker.t_Send z)))
        <:
        bool)
  in
  let _:Prims.unit =
    Hax_lib.v_assert (Core_models.Result.impl__is_err #Prims.unit
          #(dyn 2 (fun z -> Core_models.Any.t_Any z) (fun z -> Core_models.Marker.t_Send z))
          (Std.Panic.catch_unwind #(Prims.unit -> Prims.unit) #Prims.unit eq_bad_message
            <:
            Core_models.Result.t_Result Prims.unit
              (dyn 2 (fun z -> Core_models.Any.t_Any z) (fun z -> Core_models.Marker.t_Send z)))
        <:
        bool)
  in
  let _:Prims.unit =
    Hax_lib.v_assert (Core_models.Result.impl__is_err #Prims.unit
          #(dyn 2 (fun z -> Core_models.Any.t_Any z) (fun z -> Core_models.Marker.t_Send z))
          (Std.Panic.catch_unwind #(Prims.unit -> Prims.unit) #Prims.unit ne_bad
            <:
            Core_models.Result.t_Result Prims.unit
              (dyn 2 (fun z -> Core_models.Any.t_Any z) (fun z -> Core_models.Marker.t_Send z)))
        <:
        bool)
  in
  let _:Prims.unit =
    Hax_lib.v_assert (Core_models.Result.impl__is_err #Prims.unit
          #(dyn 2 (fun z -> Core_models.Any.t_Any z) (fun z -> Core_models.Marker.t_Send z))
          (Std.Panic.catch_unwind #(Prims.unit -> Prims.unit) #Prims.unit ne_bad_message
            <:
            Core_models.Result.t_Result Prims.unit
              (dyn 2 (fun z -> Core_models.Any.t_Any z) (fun z -> Core_models.Marker.t_Send z)))
        <:
        bool)
  in
  ()
