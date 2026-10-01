module New_tests.Rustc_coverage__branch__no_mir_spans
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

/// @fail(extraction): ssprove(HAX0001), coq(HAX0001, HAX0001)
let while_cond (_: Prims.unit) : (i32 & Prims.unit) =
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
  let a:i32 = mk_i32 8 in
  Rust_primitives.Hax.while_loop (fun a ->
        let a:i32 = a in
        true)
    (fun a ->
        let a:i32 = a in
        a >. mk_i32 0 <: bool)
    (fun a ->
        let a:i32 = a in
        Rust_primitives.Hax.Int.from_machine (mk_u32 0) <: Hax_lib.Int.t_Int)
    a
    (fun a ->
        let a:i32 = a in
        let a:i32 = a -! mk_i32 1 in
        a),
  ()
  <:
  (i32 & Prims.unit)

/// @fail(extraction): ssprove(HAX0001), coq(HAX0001, HAX0001)
let while_cond_not (_: Prims.unit) : (i32 & Prims.unit) =
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
  let a:i32 = mk_i32 8 in
  Rust_primitives.Hax.while_loop (fun a ->
        let a:i32 = a in
        true)
    (fun a ->
        let a:i32 = a in
        ~.(a =. mk_i32 0 <: bool) <: bool)
    (fun a ->
        let a:i32 = a in
        Rust_primitives.Hax.Int.from_machine (mk_u32 0) <: Hax_lib.Int.t_Int)
    a
    (fun a ->
        let a:i32 = a in
        let a:i32 = a -! mk_i32 1 in
        a),
  ()
  <:
  (i32 & Prims.unit)

/// @fail(extraction): ssprove(HAX0001), coq(HAX0001, HAX0001)
let while_op_and (_: Prims.unit) : ((i32 & i32) & Prims.unit) =
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
  let a:i32 = mk_i32 8 in
  let b:i32 = mk_i32 4 in
  Rust_primitives.Hax.while_loop (fun temp_0_ ->
        let (a: i32), (b: i32) = temp_0_ in
        true)
    (fun temp_0_ ->
        let (a: i32), (b: i32) = temp_0_ in
        (a >. mk_i32 0 <: bool) && (b >. mk_i32 0 <: bool))
    (fun temp_0_ ->
        let (a: i32), (b: i32) = temp_0_ in
        Rust_primitives.Hax.Int.from_machine (mk_u32 0) <: Hax_lib.Int.t_Int)
    (a, b <: (i32 & i32))
    (fun temp_0_ ->
        let (a: i32), (b: i32) = temp_0_ in
        let a:i32 = a -! mk_i32 1 in
        let b:i32 = b -! mk_i32 1 in
        a, b <: (i32 & i32)),
  ()
  <:
  ((i32 & i32) & Prims.unit)

/// @fail(extraction): coq(HAX0001, HAX0001), ssprove(HAX0001)
let while_op_or (_: Prims.unit) : ((i32 & i32) & Prims.unit) =
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
  let a:i32 = mk_i32 4 in
  let b:i32 = mk_i32 8 in
  Rust_primitives.Hax.while_loop (fun temp_0_ ->
        let (a: i32), (b: i32) = temp_0_ in
        true)
    (fun temp_0_ ->
        let (a: i32), (b: i32) = temp_0_ in
        (a >. mk_i32 0 <: bool) || (b >. mk_i32 0 <: bool))
    (fun temp_0_ ->
        let (a: i32), (b: i32) = temp_0_ in
        Rust_primitives.Hax.Int.from_machine (mk_u32 0) <: Hax_lib.Int.t_Int)
    (a, b <: (i32 & i32))
    (fun temp_0_ ->
        let (a: i32), (b: i32) = temp_0_ in
        let a:i32 = a -! mk_i32 1 in
        let b:i32 = b -! mk_i32 1 in
        a, b <: (i32 & i32)),
  ()
  <:
  ((i32 & i32) & Prims.unit)

let main (_: Prims.unit) : Prims.unit =
  let _:Prims.unit = while_cond () in
  let _:Prims.unit = while_cond_not () in
  let _:Prims.unit = while_op_and () in
  let _:Prims.unit = while_op_or () in
  ()
