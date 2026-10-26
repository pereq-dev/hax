module New_tests.Legacy__range_inclusive_break__lib
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

/// @fail(extraction): coq(HAX0001), ssprove(HAX0001), proverif(HAX0008)
let with_break (n: u8) : u64 =
  let count:u64 = mk_u64 0 in
  Rust_primitives.Hax.Folds.fold_range_inclusive_cf (mk_u8 0)
    n
    (fun count temp_1_ ->
        let count:u64 = count in
        let _:u8 = temp_1_ in
        true)
    count
    (fun count i ->
        let count:u64 = count in
        let i:u8 = i in
        if i =. mk_u8 3 <: bool
        then
          Core_models.Ops.Control_flow.ControlFlow_Break ((), count <: (Prims.unit & u64))
          <:
          Core_models.Ops.Control_flow.t_ControlFlow (Prims.unit & u64) u64
        else
          Core_models.Ops.Control_flow.ControlFlow_Continue (count +! mk_u64 1 <: u64)
          <:
          Core_models.Ops.Control_flow.t_ControlFlow (Prims.unit & u64) u64)
