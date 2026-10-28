module New_tests.Rustc_coverage__generic_unused_impl
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

type t_W (v_T: Type0) = | W : v_T -> t_W v_T

let main (_: Prims.unit) : Prims.unit = ()

class t_Foo (v_Self: Type0) = {
  [@@@ FStar.Tactics.Typeclasses.no_method]f_Assoc:Type0;
  f_from_pre:f_Assoc -> Type0;
  f_from_post:f_Assoc -> v_Self -> Type0;
  f_from:x0: f_Assoc -> Prims.Pure v_Self (f_from_pre x0) (fun result -> f_from_post x0 result)
}

/// @fail(extraction): fstar(HAX0001), coq(HAX0001), proverif(HAX0001), ssprove(HAX0001), legacy-lean(HAX0001)
[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl (#v_T: Type0) (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Foo v_T)
    : Core_models.Convert.t_From (t_W v_T) (t_Array i0.f_Assoc (mk_usize 1)) =
  {
    f_from_pre = (fun (from: t_Array i0.f_Assoc (mk_usize 1)) -> true);
    f_from_post = (fun (from: t_Array i0.f_Assoc (mk_usize 1)) (out: t_W v_T) -> true);
    f_from
    =
    fun (from: t_Array i0.f_Assoc (mk_usize 1)) ->
      Rust_primitives.Hax.failure "something is not implemented yet.\nPat:Array\n\nThis is discussed in issue https://github.com/pereq-dev/hax/issues/804.\nPlease upvote or comment this issue if you see this error message.\nNote: the error was labeled with context `AST import`.\n"
        ""
  }
