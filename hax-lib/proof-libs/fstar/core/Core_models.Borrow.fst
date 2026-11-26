module Core_models.Borrow
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Rust_primitives

/// See [`std::borrow::Borrow`]
class t_Borrow (v_Self: Type0) (v_Borrowed: Type0) = {
  f_borrow_pre:v_Self -> Type0;
  f_borrow_post:v_Self -> v_Borrowed -> Type0;
  f_borrow:x0: v_Self
    -> Prims.Pure v_Borrowed (f_borrow_pre x0) (fun result -> f_borrow_post x0 result)
}

/// See [`std::borrow::Borrow`] for `&T`
[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl (#v_T: Type0) : t_Borrow v_T v_T =
  {
    f_borrow_pre = (fun (self: v_T) -> true);
    f_borrow_post = (fun (self: v_T) (out: v_T) -> true);
    f_borrow = fun (self: v_T) -> self
  }
