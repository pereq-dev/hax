module New_tests.Legacy__range_inclusive__lib
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

/// `end + 1` does not fit in a `u8` here.
/// @fail(extraction): proverif(HAX0008)
let full_u8_range (_: Prims.unit) : u64 =
  let count:u64 = mk_u64 0 in
  let count:u64 =
    Rust_primitives.Hax.Folds.fold_range_inclusive (mk_u8 0)
      (mk_u8 255)
      (fun count temp_1_ ->
          let count:u64 = count in
          let _:u8 = temp_1_ in
          true)
      count
      (fun count i ->
          let count:u64 = count in
          let i:u8 = i in
          count +! mk_u64 1 <: u64)
  in
  count

/// @fail(extraction): proverif(HAX0008)
let count_up_to (n: u8)
    : Prims.Pure u8
      (requires n <. mk_u8 255)
      (ensures
        fun res ->
          let res:u8 = res in
          n <. res) =
  let count:u8 = mk_u8 0 in
  let count:u8 =
    Rust_primitives.Hax.Folds.fold_range_inclusive (mk_u8 0)
      n
      (fun count i ->
          let count:u8 = count in
          let i:u8 = i in
          count =. i <: bool)
      count
      (fun count i ->
          let count:u8 = count in
          let i:u8 = i in
          let count:u8 = count +! mk_u8 1 in
          count)
  in
  count

/// @fail(extraction): proverif(HAX0008)
let empty_range (_: Prims.unit) : u64 =
  let count:u64 = mk_u64 0 in
  let count:u64 =
    Rust_primitives.Hax.Folds.fold_range_inclusive (mk_u8 5)
      (mk_u8 3)
      (fun count temp_1_ ->
          let count:u64 = count in
          let _:u8 = temp_1_ in
          true)
      count
      (fun count i ->
          let count:u64 = count in
          let i:u8 = i in
          count +! mk_u64 1 <: u64)
  in
  count

/// @fail(extraction): proverif(HAX0008)
let single_element (_: Prims.unit) : u64 =
  let count:u64 = mk_u64 0 in
  let count:u64 =
    Rust_primitives.Hax.Folds.fold_range_inclusive (mk_u8 3)
      (mk_u8 3)
      (fun count temp_1_ ->
          let count:u64 = count in
          let _:u8 = temp_1_ in
          true)
      count
      (fun count i ->
          let count:u64 = count in
          let i:u8 = i in
          count +! mk_u64 1 <: u64)
  in
  count

/// @fail(extraction): proverif(HAX0008)
let with_return (n: u8) : u8 =
  match
    Rust_primitives.Hax.Folds.fold_range_inclusive_return (mk_u8 0)
      n
      (fun temp_0_ temp_1_ ->
          let _:Prims.unit = temp_0_ in
          let _:u8 = temp_1_ in
          true)
      ()
      (fun temp_0_ i ->
          let _:Prims.unit = temp_0_ in
          let i:u8 = i in
          if i =. mk_u8 7 <: bool
          then
            Core_models.Ops.Control_flow.ControlFlow_Break
            (Core_models.Ops.Control_flow.ControlFlow_Break i
              <:
              Core_models.Ops.Control_flow.t_ControlFlow u8 (Prims.unit & Prims.unit))
            <:
            Core_models.Ops.Control_flow.t_ControlFlow
              (Core_models.Ops.Control_flow.t_ControlFlow u8 (Prims.unit & Prims.unit)) Prims.unit
          else
            Core_models.Ops.Control_flow.ControlFlow_Continue ()
            <:
            Core_models.Ops.Control_flow.t_ControlFlow
              (Core_models.Ops.Control_flow.t_ControlFlow u8 (Prims.unit & Prims.unit)) Prims.unit)
    <:
    Core_models.Ops.Control_flow.t_ControlFlow u8 Prims.unit
  with
  | Core_models.Ops.Control_flow.ControlFlow_Break ret -> ret
  | Core_models.Ops.Control_flow.ControlFlow_Continue _ -> mk_u8 0
