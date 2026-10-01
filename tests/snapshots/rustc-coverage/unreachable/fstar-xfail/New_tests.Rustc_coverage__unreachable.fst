module New_tests.Rustc_coverage__unreachable
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

/// @fail(extraction): ssprove(HAX0008), coq(HAX0008)
/// @fail(extraction): proverif(HAX0008)
let v_UNREACHABLE_CLOSURE:  Prims.unit -> Prims.unit =
  fun temp_0_ ->
    let _:Prims.unit = temp_0_ in
    Rust_primitives.Hax.never_to_any (Core_models.Hint.unreachable_unchecked ()
        <:
        Rust_primitives.Hax.t_Never)

/// @fail(extraction): coq(HAX0008), ssprove(HAX0008)
/// @fail(extraction): proverif(HAX0008)
let unreachable_function (_: Prims.unit) : Prims.unit =
  Rust_primitives.Hax.never_to_any (Core_models.Hint.unreachable_unchecked ()
      <:
      Rust_primitives.Hax.t_Never)

/// @fail(extraction): ssprove(HAX0008), coq(HAX0008)
/// @fail(extraction): proverif(HAX0008)
let unreachable_intrinsic (_: Prims.unit) : Prims.unit =
  Rust_primitives.Hax.never_to_any (Core_models.Intrinsics.unreachable ()
      <:
      Rust_primitives.Hax.t_Never)

let main (_: Prims.unit) : Prims.unit =
  let _:Prims.unit =
    if Core_models.Hint.black_box #bool false
    then
      let _:Prims.unit = v_UNREACHABLE_CLOSURE () in
      ()
  in
  let _:Prims.unit =
    if Core_models.Hint.black_box #bool false
    then
      let _:Prims.unit = unreachable_function () in
      ()
  in
  if Core_models.Hint.black_box #bool false
  then
    let _:Prims.unit = unreachable_intrinsic () in
    ()
