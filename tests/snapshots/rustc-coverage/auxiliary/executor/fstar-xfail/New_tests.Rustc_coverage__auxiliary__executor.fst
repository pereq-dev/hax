module New_tests.Rustc_coverage__auxiliary__executor
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

/// Dummy "executor" that just repeatedly polls a future until it's ready.
/// @fail(extraction): fstar(HAX0003, HAX0003, HAX0003, HAX0003, HAX0003), ssprove(HAX0003, HAX0003, HAX0003, HAX0003, HAX0008), coq(HAX0008, HAX0003, HAX0003, HAX0003, HAX0003), legacy-lean(HAX0003, HAX0003, HAX0003, HAX0003, HAX0003), proverif(HAX0003, HAX0003, HAX0003, HAX0003, HAX0008)
let block_on
      (#v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Future.Future.t_Future v_F)
      (future: v_F)
    : i0.f_Output =
  Rust_primitives.Hax.failure "The mutation of this &mut is not allowed here.\n\nThis is discussed in issue https://github.com/pereq-dev/hax/issues/420.\nPlease upvote or comment this issue if you see this error message.\nNote: the error was labeled with context `DirectAndMut`.\n"
    "{\n let mut future: core_models::pin::t_Pin<&mut F> = {\n {\n let mut pinned: F = { future };\n {\n unsafe {\n core_models::pin::impl_6__new_unchecked::<&mut F>(&mut (pinned))\n }\n }\n }\n };\n {\n let mut conte..."
