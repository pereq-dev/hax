module New_tests.Legacy__loops__lib.Recognized_loops
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let range (_: Prims.unit) : (u64 & Prims.unit) =
  let count:u64 = mk_u64 0 in
  Rust_primitives.Hax.Folds.fold_range (mk_u8 0)
    (mk_u8 10)
    (fun count i ->
        let count:u64 = count in
        let i:u8 = i in
        i <=. mk_u8 10 <: bool)
    count
    (fun count i ->
        let count:u64 = count in
        let i:u8 = i in
        let count:u64 = count +! mk_u64 1 in
        count),
  ()
  <:
  (u64 & Prims.unit)

let range_step_by (_: Prims.unit) : (u64 & Prims.unit) =
  let count:u64 = mk_u64 0 in
  Rust_primitives.Hax.Folds.fold_range_step_by (mk_u8 0)
    (mk_u8 10)
    (mk_usize 2)
    (fun count i ->
        let count:u64 = count in
        let i:u8 = i in
        i <=. mk_u8 10 <: bool)
    count
    (fun count i ->
        let count:u64 = count in
        let i:u8 = i in
        let count:u64 = count +! mk_u64 1 in
        count),
  ()
  <:
  (u64 & Prims.unit)

let enumerated_slice (#v_T: Type0) (slice: t_Slice v_T) : (u64 & Prims.unit) =
  let count:u64 = mk_u64 0 in
  Rust_primitives.Hax.Folds.fold_enumerated_slice slice
    (fun count i ->
        let count:u64 = count in
        let i:usize = i in
        i <=. mk_usize 10 <: bool)
    count
    (fun count i ->
        let count:u64 = count in
        let i:(usize & v_T) = i in
        let count:u64 = count +! mk_u64 2 in
        count),
  ()
  <:
  (u64 & Prims.unit)

let enumerated_chunked_slice (#v_T: Type0) (slice: t_Slice v_T) : (u64 & Prims.unit) =
  let count:u64 = mk_u64 0 in
  Rust_primitives.Hax.Folds.fold_enumerated_chunked_slice (mk_usize 3)
    slice
    (fun count i ->
        let count:u64 = count in
        let i:usize = i in
        i <= Core_models.Slice.impl__len #v_T slice)
    count
    (fun count i ->
        let count:u64 = count in
        let i:(usize & t_Slice v_T) = i in
        let count:u64 = count +! mk_u64 3 in
        count),
  ()
  <:
  (u64 & Prims.unit)
