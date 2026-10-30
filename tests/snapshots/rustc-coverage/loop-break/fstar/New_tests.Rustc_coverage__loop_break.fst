module New_tests.Rustc_coverage__loop_break
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

/// @fail(extraction): legacy-lean(HAX0001), fstar(HAX0001), coq(HAX0001, HAX0001), ssprove(HAX0001)
let main (_: Prims.unit) : (Prims.unit & Prims.unit) =
  Rust_primitives.Hax.failure "something is not implemented yet.\nUnhandled loop kind\n\nThis is discussed in issue https://github.com/pereq-dev/hax/issues/933.\nPlease upvote or comment this issue if you see this error message.\nNote: the error was labeled with context `FunctionalizeLoops`.\n"
    "{\n loop {\n (if core_models::hint::black_box::<bool>(true) {\n core_models::ops::control_flow::ControlFlow_Break(\n Tuple2(Tuple0, Tuple0()),\n )\n } else {\n core_models::ops::control_flow::ControlFlow_Con..."
  ,
  ()
  <:
  (Prims.unit & Prims.unit)
