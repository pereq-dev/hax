module New_tests.Rustc_coverage__issue_93054_
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

type t_Never =

let t_Never_cast_to_repr (x: t_Never) : Rust_primitives.Hax.t_Never = match x <: t_Never with

let impl_Never__bar (self: t_Never) : Prims.unit =
  Rust_primitives.Hax.never_to_any (match self <: t_Never with )

/// @fail(extraction): fstar(HAX0001), ssprove(HAX0001), legacy-lean(HAX0001), coq(HAX0001)
/// @fail(extraction): proverif(HAX0001)
let foo2 (never: t_Never) : Rust_primitives.Hax.t_Failure "" =
  Rust_primitives.Hax.failure "something is not implemented yet.\nGot type `Coroutine`: coroutines are not supported by hax\n\nThis is discussed in issue https://github.com/pereq-dev/hax/issues/924.\nPlease upvote or comment this issue if you see this error message.\nNote: the error was labeled with context `AST import`.\n"
    ""

let make (_: Prims.unit) : Core_models.Option.t_Option t_Never =
  Core_models.Option.Option_None <: Core_models.Option.t_Option t_Never

let impl_Never__foo (self: t_Never) : Prims.unit =
  let _:Prims.unit = Rust_primitives.Hax.never_to_any (match self <: t_Never with ) in
  Rust_primitives.Hax.never_to_any (Core_models.Option.impl__map #t_Never
        #Rust_primitives.Hax.t_Never
        #(t_Never -> Rust_primitives.Hax.t_Never)
        (make () <: Core_models.Option.t_Option t_Never)
        (fun never ->
            let never:t_Never = never in
            match never <: t_Never with )
      <:
      Core_models.Option.t_Option Rust_primitives.Hax.t_Never)

let main (_: Prims.unit) : Prims.unit = ()
