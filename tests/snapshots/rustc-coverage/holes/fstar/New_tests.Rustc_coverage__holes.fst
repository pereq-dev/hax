module New_tests.Rustc_coverage__holes
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

/// @fail(extraction): coq(HAX0001), legacy-lean(HAX0001), ssprove(HAX0001), proverif(HAX0001), fstar(HAX0001)
let main (_: Prims.unit) : Prims.unit =
  Rust_primitives.Hax.failure "something is not implemented yet.\nGot type `Coroutine`: coroutines are not supported by hax\n\nThis is discussed in issue https://github.com/pereq-dev/hax/issues/924.\nPlease upvote or comment this issue if you see this error message.\nNote: the error was labeled with context `AST import`.\n"
    ""

let main__v_MY_STATIC: Prims.unit = () <: Prims.unit

let main__v_MY_CONST: Prims.unit = () <: Prims.unit

let main__e_unused_fn (_: Prims.unit) : Prims.unit = ()

type main__t_MyStruct = {
  main__f_e_x:u32;
  main__f_e_y:u32
}

let main__impl__e_method (self: main__t_MyStruct) : Prims.unit = ()

class main__t_MyTrait (v_Self: Type0) = { __marker_trait_main__t_MyTrait:Prims.unit }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let main__impl_1: main__t_MyTrait main__t_MyStruct = { __marker_trait_main__t_MyTrait = () }
