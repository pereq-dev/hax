module New_tests.Legacy__range_inclusive_signed__lib
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

/// @fail(extraction): proverif(HAX0008)
let signed_range (_: Prims.unit) : i32 =
  let sum:i32 = mk_i32 0 in
  let sum:i32 =
    Rust_primitives.Hax.Folds.fold_range_inclusive (mk_i32 (-3))
      (mk_i32 3)
      (fun sum temp_1_ ->
          let sum:i32 = sum in
          let _:i32 = temp_1_ in
          true)
      sum
      (fun sum i ->
          let sum:i32 = sum in
          let i:i32 = i in
          sum +! i <: i32)
  in
  sum
