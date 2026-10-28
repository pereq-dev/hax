module New_tests.Legacy__trait_param_bounds__lib
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

class t_Item (v_Self: Type0) = {
  f_get_pre:v_Self -> Type0;
  f_get_post:v_Self -> u8 -> Type0;
  f_get:x0: v_Self -> Prims.Pure u8 (f_get_pre x0) (fun result -> f_get_post x0 result)
}

class t_Other (v_Self: Type0) = { __marker_trait_t_Other:Prims.unit }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl: t_Item u64 =
  {
    f_get_pre = (fun (self: u64) -> true);
    f_get_post = (fun (self: u64) (out: u8) -> true);
    f_get = fun (self: u64) -> mk_u8 0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_Item_for_u16: t_Item u16 =
  {
    f_get_pre = (fun (self: u16) -> true);
    f_get_post = (fun (self: u16) (out: u8) -> true);
    f_get = fun (self: u16) -> mk_u8 1
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_Other_for_u32: t_Other u32 = { __marker_trait_t_Other = () }

/// Bound on a parameter.
class t_Squeeze (v_Self: Type0) (v_T: Type0) = {
  [@@@ FStar.Tactics.Typeclasses.no_method]_super_i0:t_Item v_T;
  f_squeeze_pre:v_Self -> v_T -> Type0;
  f_squeeze_post:v_Self -> v_T -> u8 -> Type0;
  f_squeeze:x0: v_Self -> x1: v_T
    -> Prims.Pure u8 (f_squeeze_pre x0 x1) (fun result -> f_squeeze_post x0 x1 result)
}

[@@ FStar.Tactics.Typeclasses.tcinstance]
let _ = fun (v_Self:Type0) (v_T:Type0) {|i: t_Squeeze v_Self v_T|} -> i._super_i0

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_3: t_Squeeze u8 u64 =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    f_squeeze_pre = (fun (self: u8) (x: u64) -> true);
    f_squeeze_post = (fun (self: u8) (x: u64) (out: u8) -> true);
    f_squeeze = fun (self: u8) (x: u64) -> f_get #u64 #FStar.Tactics.Typeclasses.solve x
  }

/// Generic impl.
type t_W = | W : t_W

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_4 (#v_T: Type0) (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Item v_T)
    : t_Squeeze t_W v_T =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    f_squeeze_pre = (fun (self: t_W) (x: v_T) -> true);
    f_squeeze_post = (fun (self: t_W) (x: v_T) (out: u8) -> true);
    f_squeeze = fun (self: t_W) (x: v_T) -> f_get #v_T #FStar.Tactics.Typeclasses.solve x
  }

/// Supertrait mixed with parameter bounds, inline and in a `where` clause.
class t_Mixed (v_Self: Type0) (v_A: Type0) (v_B: Type0) = {
  [@@@ FStar.Tactics.Typeclasses.no_method]_super_i0:t_Other v_Self;
  [@@@ FStar.Tactics.Typeclasses.no_method]_super_i1:t_Item v_A;
  [@@@ FStar.Tactics.Typeclasses.no_method]_super_i2:t_Item v_B;
  [@@@ FStar.Tactics.Typeclasses.no_method]_super_i3:Core_models.Clone.t_Clone v_B;
  f_mixed_pre:v_Self -> v_A -> v_B -> Type0;
  f_mixed_post:v_Self -> v_A -> v_B -> bool -> Type0;
  f_mixed:x0: v_Self -> x1: v_A -> x2: v_B
    -> Prims.Pure bool (f_mixed_pre x0 x1 x2) (fun result -> f_mixed_post x0 x1 x2 result)
}

[@@ FStar.Tactics.Typeclasses.tcinstance]
let _ = fun (v_Self:Type0) (v_A:Type0) (v_B:Type0) {|i: t_Mixed v_Self v_A v_B|} -> i._super_i0

[@@ FStar.Tactics.Typeclasses.tcinstance]
let _ = fun (v_Self:Type0) (v_A:Type0) (v_B:Type0) {|i: t_Mixed v_Self v_A v_B|} -> i._super_i1

[@@ FStar.Tactics.Typeclasses.tcinstance]
let _ = fun (v_Self:Type0) (v_A:Type0) (v_B:Type0) {|i: t_Mixed v_Self v_A v_B|} -> i._super_i2

[@@ FStar.Tactics.Typeclasses.tcinstance]
let _ = fun (v_Self:Type0) (v_A:Type0) (v_B:Type0) {|i: t_Mixed v_Self v_A v_B|} -> i._super_i3

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_5: t_Mixed u32 u64 u16 =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    _super_i1 = FStar.Tactics.Typeclasses.solve;
    _super_i2 = FStar.Tactics.Typeclasses.solve;
    _super_i3 = FStar.Tactics.Typeclasses.solve;
    f_mixed_pre = (fun (self: u32) (a: u64) (b: u16) -> true);
    f_mixed_post = (fun (self: u32) (a: u64) (b: u16) (out: bool) -> true);
    f_mixed
    =
    fun (self: u32) (a: u64) (b: u16) ->
      (f_get #u64 #FStar.Tactics.Typeclasses.solve a <: u8) =.
      (f_get #u16 #FStar.Tactics.Typeclasses.solve b <: u8)
  }

/// Supertrait in a `where` clause, after a parameter bound.
class t_WhereSuper (v_Self: Type0) (v_A: Type0) = {
  [@@@ FStar.Tactics.Typeclasses.no_method]_super_i0:t_Item v_A;
  [@@@ FStar.Tactics.Typeclasses.no_method]_super_i1:t_Other v_Self;
  f_where_super_pre:v_Self -> v_A -> Type0;
  f_where_super_post:v_Self -> v_A -> u8 -> Type0;
  f_where_super:x0: v_Self -> x1: v_A
    -> Prims.Pure u8 (f_where_super_pre x0 x1) (fun result -> f_where_super_post x0 x1 result)
}

[@@ FStar.Tactics.Typeclasses.tcinstance]
let _ = fun (v_Self:Type0) (v_A:Type0) {|i: t_WhereSuper v_Self v_A|} -> i._super_i0

[@@ FStar.Tactics.Typeclasses.tcinstance]
let _ = fun (v_Self:Type0) (v_A:Type0) {|i: t_WhereSuper v_Self v_A|} -> i._super_i1

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_6: t_WhereSuper u32 u64 =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    _super_i1 = FStar.Tactics.Typeclasses.solve;
    f_where_super_pre = (fun (self: u32) (a: u64) -> true);
    f_where_super_post = (fun (self: u32) (a: u64) (out: u8) -> true);
    f_where_super = fun (self: u32) (a: u64) -> f_get #u64 #FStar.Tactics.Typeclasses.solve a
  }

/// Bound on an associated type of a supertrait.
class t_HasAssoc (v_Self: Type0) = { [@@@ FStar.Tactics.Typeclasses.no_method]f_X:Type0 }

class t_AssocBound (v_Self: Type0) = {
  [@@@ FStar.Tactics.Typeclasses.no_method]_super_i0:t_HasAssoc v_Self;
  [@@@ FStar.Tactics.Typeclasses.no_method]_super_i1:Core_models.Clone.t_Clone (_super_i0).f_X;
  f_assoc_bound_pre:v_Self -> Type0;
  f_assoc_bound_post:v_Self -> u8 -> Type0;
  f_assoc_bound:x0: v_Self
    -> Prims.Pure u8 (f_assoc_bound_pre x0) (fun result -> f_assoc_bound_post x0 result)
}

[@@ FStar.Tactics.Typeclasses.tcinstance]
let _ = fun (v_Self:Type0) {|i: t_AssocBound v_Self|} -> i._super_i0

[@@ FStar.Tactics.Typeclasses.tcinstance]
let _ = fun (v_Self:Type0) {|i: t_AssocBound v_Self|} -> i._super_i1

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_HasAssoc_for_u32: t_HasAssoc u32 = { f_X = u64 }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_AssocBound_for_u32: t_AssocBound u32 =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    _super_i1 = FStar.Tactics.Typeclasses.solve;
    f_assoc_bound_pre = (fun (self: u32) -> true);
    f_assoc_bound_post = (fun (self: u32) (out: u8) -> true);
    f_assoc_bound = fun (self: u32) -> mk_u8 2
  }

/// Trait without items.
class t_Marker (v_Self: Type0) (v_T: Type0) = {
  [@@@ FStar.Tactics.Typeclasses.no_method]_super_i0:t_Item v_T
}

[@@ FStar.Tactics.Typeclasses.tcinstance]
let _ = fun (v_Self:Type0) (v_T:Type0) {|i: t_Marker v_Self v_T|} -> i._super_i0

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_9: t_Marker u8 u64 = { _super_i0 = FStar.Tactics.Typeclasses.solve }

let uuse_mixed
      (#v_S #v_A #v_B: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Mixed v_S v_A v_B)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Item v_A)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i2: t_Item v_B)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i3: Core_models.Clone.t_Clone v_B)
      (s: v_S)
      (a: v_A)
      (b: v_B)
    : bool = f_mixed #v_S #v_A #v_B #FStar.Tactics.Typeclasses.solve s a b

let call_use_mixed (_: Prims.unit) : bool =
  uuse_mixed #u32 #u64 #u16 (mk_u32 0) (mk_u64 0) (mk_u16 0)
