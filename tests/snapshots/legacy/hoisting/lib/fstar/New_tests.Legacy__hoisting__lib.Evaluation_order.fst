module New_tests.Legacy__hoisting__lib.Evaluation_order
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let add (a b: u8) : u8 = Core_models.Num.impl_u8__wrapping_add a b

let bump (v: u8) : (u8 & u8) =
  let v:u8 = Core_models.Num.impl_u8__wrapping_add v (mk_u8 1) in
  let hax_temp_output:u8 = mk_u8 1 in
  v, hax_temp_output <: (u8 & u8)

let set (v: u8) : (u8 & u8) =
  let v:u8 = mk_u8 7 in
  let hax_temp_output:u8 = mk_u8 2 in
  v, hax_temp_output <: (u8 & u8)

let read_then_assign (_: Prims.unit) : u8 =
  let v:u8 = mk_u8 0 in
  let hoist11:u8 = v in
  let v:u8 = mk_u8 5 in
  add hoist11 (mk_u8 1)

let read_then_mut_ref (v: u8) : (u8 & u8) =
  let hoist13:u8 = v in
  let (tmp0: u8), (out: u8) = bump v in
  let v:u8 = tmp0 in
  let hax_temp_output:u8 = add hoist13 out in
  v, hax_temp_output <: (u8 & u8)

let two_mut_refs (a b: u8) : (u8 & u8 & u8) =
  let (tmp0: u8), (out: u8) = bump a in
  let a:u8 = tmp0 in
  let hoist15:u8 = out in
  let (tmp0: u8), (out: u8) = set b in
  let b:u8 = tmp0 in
  let hax_temp_output:u8 = add hoist15 out in
  a, b, hax_temp_output <: (u8 & u8 & u8)
