module New_tests.Rustc_coverage__unused
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

/// @fail(extraction): ssprove(HAX0001), coq(HAX0001, HAX0001)
let foo (#v_T: Type0) (x: v_T) : (i32 & Prims.unit) =
  let i:i32 = mk_i32 0 in
  Rust_primitives.Hax.while_loop (fun i ->
        let i:i32 = i in
        true)
    (fun i ->
        let i:i32 = i in
        i <. mk_i32 10 <: bool)
    (fun i ->
        let i:i32 = i in
        Rust_primitives.Hax.Int.from_machine (mk_u32 0) <: Hax_lib.Int.t_Int)
    i
    (fun i ->
        let i:i32 = i in
        let _:bool = i <>. mk_i32 0 || i <>. mk_i32 0 in
        let i:i32 = i +! mk_i32 1 in
        i),
  ()
  <:
  (i32 & Prims.unit)

/// @fail(extraction): ssprove(HAX0001), coq(HAX0001, HAX0001)
let unused_template_func (#v_T: Type0) (x: v_T) : (i32 & Prims.unit) =
  let i:i32 = mk_i32 0 in
  Rust_primitives.Hax.while_loop (fun i ->
        let i:i32 = i in
        true)
    (fun i ->
        let i:i32 = i in
        i <. mk_i32 10 <: bool)
    (fun i ->
        let i:i32 = i in
        Rust_primitives.Hax.Int.from_machine (mk_u32 0) <: Hax_lib.Int.t_Int)
    i
    (fun i ->
        let i:i32 = i in
        let _:bool = i <>. mk_i32 0 || i <>. mk_i32 0 in
        let i:i32 = i +! mk_i32 1 in
        i),
  ()
  <:
  (i32 & Prims.unit)

let unused_func (a: u32) : Prims.unit =
  if a <>. mk_u32 0
  then
    let a:u32 = a +! mk_u32 1 in
    ()

let unused_func2 (a: u32) : Prims.unit =
  if a <>. mk_u32 0
  then
    let a:u32 = a +! mk_u32 1 in
    ()

let unused_func3 (a: u32) : Prims.unit =
  if a <>. mk_u32 0
  then
    let a:u32 = a +! mk_u32 1 in
    ()

/// @fail(extraction): ssprove(HAX0001)
let main (_: Prims.unit) : Core_models.Result.t_Result Prims.unit u8 =
  let _:Prims.unit = foo #u32 (mk_u32 0) in
  let _:Prims.unit = foo #float (mk_float "0.0") in
  Core_models.Result.Result_Ok (() <: Prims.unit) <: Core_models.Result.t_Result Prims.unit u8
