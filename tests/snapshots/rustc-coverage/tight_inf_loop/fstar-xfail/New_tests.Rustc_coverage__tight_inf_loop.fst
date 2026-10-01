module New_tests.Rustc_coverage__tight_inf_loop
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

/// @fail(extraction): legacy-lean(HAX0001), coq(HAX0001), fstar(HAX0001)
let main (_: Prims.unit) : Prims.unit =
  if false
  then
    Rust_primitives.Hax.never_to_any ((Rust_primitives.Hax.failure "something is not implemented yet.\nUnhandled loop kind\n\nThis is discussed in issue https://github.com/pereq-dev/hax/issues/933.\nPlease upvote or comment this issue if you see this error message.\nNote: the error was labeled with context `FunctionalizeLoops`.\n"
            "{\n loop {\n Tuple0\n }\n }"
          <:
          Prims.unit),
        ()
        <:
        (Prims.unit & Prims.unit))
