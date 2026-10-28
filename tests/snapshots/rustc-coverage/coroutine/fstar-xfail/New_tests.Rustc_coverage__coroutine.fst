module New_tests.Rustc_coverage__coroutine
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let get_u32 (v_val: bool) : Core_models.Result.t_Result u32 Alloc.String.t_String =
  if v_val
  then
    Core_models.Result.Result_Ok (mk_u32 1) <: Core_models.Result.t_Result u32 Alloc.String.t_String
  else
    Core_models.Result.Result_Err
    (Core_models.Convert.f_from #Alloc.String.t_String
        #string
        #FStar.Tactics.Typeclasses.solve
        "some error")
    <:
    Core_models.Result.t_Result u32 Alloc.String.t_String

/// @fail(extraction): proverif(HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001), coq(HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001), ssprove(HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001), legacy-lean(HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001), fstar(HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001)
let main (_: Prims.unit) : Prims.unit =
  Rust_primitives.Hax.failure "something is not implemented yet.\nGot type `Coroutine`: coroutines are not supported by hax\n\nThis is discussed in issue https://github.com/pereq-dev/hax/issues/924.\nPlease upvote or comment this issue if you see this error message.\nNote: the error was labeled with context `AST import`.\n"
    ""
