module New_tests.Rustc_coverage__closure
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

/// @fail(extraction): coq(HAX0006, HAX0003), proverif(HAX0006, HAX0003), fstar(HAX0006, HAX0003), ssprove(HAX0006, HAX0003), legacy-lean(HAX0003, HAX0006)
let main (_: Prims.unit) : Prims.unit =
  Rust_primitives.Hax.failure "The bindings [\"countdown\"] cannot be mutated here: they don't belong to the closure scope, and this is not allowed.\n\nThis is discussed in issue https://github.com/pereq-dev/hax/issues/1060.\nPlease upvote or comment this issue if you see this error message.\nNote: the error was labeled with context `LocalMutation`.\n"
    "{\n let is_true: bool = {\n rust_primitives::hax::machine_int::eq(\n core_models::iter::traits::exact_size::f_len(std::env::args(Tuple0)),\n 1,\n )\n };\n {\n let is_false: bool = { core_models::ops::bit::f_n..."
