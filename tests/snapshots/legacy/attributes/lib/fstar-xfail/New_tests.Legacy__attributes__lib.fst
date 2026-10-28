module New_tests.Legacy__attributes__lib
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let u32_max: u32 = mk_u32 90000

/// A doc comment on `add3`
///another doc comment on add3
let add3 (x y z: u32)
    : Prims.Pure u32
      (requires
        x >. mk_u32 10 && y >. mk_u32 10 && z >. mk_u32 10 &&
        ((x +! y <: u32) +! z <: u32) <. u32_max)
      (ensures
        fun result ->
          let result:u32 = result in
          b2t true ==> b2t (result >. mk_u32 32 <: bool)) = (x +! y <: u32) +! z

let swap_and_mut_req_ens (x y: u32)
    : Prims.Pure (u32 & u32 & u32)
      (requires x <. mk_u32 40 && y <. mk_u32 300)
      (ensures
        fun temp_0_ ->
          let (x_future: u32), (y_future: u32), (result: u32) = temp_0_ in
          x_future =. y && y_future =. x && result =. (x +! y <: u32)) =
  let x0:u32 = x in
  let x:u32 = y in
  let y:u32 = x0 in
  let hax_temp_output:u32 = x +! y in
  x, y, hax_temp_output <: (u32 & u32 & u32)

let issue_844_ (e_x: u8)
    : Prims.Pure u8
      Prims.l_True
      (ensures
        fun e_x_future ->
          let e_x_future:u8 = e_x_future in
          true) = e_x

let add3_lemma (x: u32)
    : Lemma
    (ensures
      x <=. mk_u32 10 || x >=. (u32_max /! mk_u32 3 <: u32) ||
      (add3 x x x <: u32) =. (x *! mk_u32 3 <: u32)) = ()

let dummy_function (x: u32) : u32 = x

let apply_dummy_function_lemma (x: u32) : Lemma (ensures x =. (dummy_function x <: u32)) [SMTPat x] =
  ()

type t_Foo = {
  f_x:u32;
  f_y:f_y: u32{b2t (f_y >. mk_u32 3 <: bool)};
  f_z:f_z: u32{b2t (((f_y +! f_x <: u32) +! f_z <: u32) >. mk_u32 3 <: bool)}
}

/// Regression test for https://github.com/pereq-dev/hax/issues/899: a refinement
/// on a struct field may mention a const generic (here `LEN`), both in a field
/// type and in the refinement formula itself.
type t_FooConstGeneric (v_LEN: usize) = {
  f_indices:f_indices:
  t_Array u8 v_LEN
    { forall (i: usize).
        b2t (i <. (Core_models.Slice.impl__len #u8 (f_indices <: t_Slice u8) <: usize) <: bool) ==>
        b2t ((cast (f_indices.[ i ] <: u8) <: usize) <. mk_usize 2 <: bool) };
  f_length:f_length: usize{b2t (f_length <. v_LEN <: bool)}
}

let props (_: Prims.unit) : Prims.unit =
  let _:Prims.unit = Hax_lib.v_assume True in
  let _:Prims.unit = Hax_lib.assert_prop True in
  let _:Prims.unit = () in
  ()

let inlined_code__v_V: u8 = mk_u8 12

let before_inlined_code = "example before"

let inlined_code (foo: t_Foo) : Prims.unit =
  let vv_a:i32 = mk_i32 13 in
  let _:Prims.unit =
    let x = foo.f_x in
    let { f_x = _ ; f_y = y ; f_z = _ } = foo in
    add3 ((fun _ -> 3ul) foo) vv_a inlined_code__v_V y
  in
  ()

let inlined_code_after = "example after"

let before_1 = "example before 1"

let before_2 = "example before 2"

let before_3 = "example before 3"

let mutliple_before_after (_: Prims.unit) : Prims.unit = ()

let after 1 = "example after 1"

let after 2 = "example after 2"

let after 3 = "example after 3"

unfold let some_function _ = "hello from F*"

let rec fib (x: usize) : Prims.Tot usize (decreases x) =
  if x <=. mk_usize 2
  then x
  else
    Core_models.Num.impl_usize__wrapping_add (fib (x -! mk_usize 1 <: usize) <: usize)
      (fib (x -! mk_usize 2 <: usize) <: usize)
