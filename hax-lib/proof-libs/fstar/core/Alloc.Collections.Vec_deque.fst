module Alloc.Collections.Vec_deque
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Rust_primitives

type t_VecDeque (v_T: Type0) (v_A: Type0) =
  | VecDeque : Rust_primitives.Sequence.t_Seq v_T -> Core_models.Marker.t_PhantomData v_A
    -> t_VecDeque v_T v_A

let impl_4__new (#v_T: Type0) (_: Prims.unit) : t_VecDeque v_T Alloc.Alloc.t_Global =
  VecDeque (Rust_primitives.Sequence.seq_empty #v_T ())
    (Core_models.Marker.PhantomData <: Core_models.Marker.t_PhantomData Alloc.Alloc.t_Global)
  <:
  t_VecDeque v_T Alloc.Alloc.t_Global

let impl_4__with_capacity (#v_T: Type0) (e_capacity: usize) : t_VecDeque v_T Alloc.Alloc.t_Global =
  impl_4__new #v_T ()

let impl_5__len (#v_T #v_A: Type0) (self: t_VecDeque v_T v_A) : usize =
  Rust_primitives.Sequence.seq_len #v_T self._0

let impl_5__pop_front (#v_T #v_A: Type0) (self: t_VecDeque v_T v_A)
    : (t_VecDeque v_T v_A & Core_models.Option.t_Option v_T) =
  let (self: t_VecDeque v_T v_A), (hax_temp_output: Core_models.Option.t_Option v_T) =
    if (impl_5__len #v_T #v_A self <: usize) =. mk_usize 0
    then
      self, (Core_models.Option.Option_None <: Core_models.Option.t_Option v_T)
      <:
      (t_VecDeque v_T v_A & Core_models.Option.t_Option v_T)
    else
      let (tmp0: Rust_primitives.Sequence.t_Seq v_T), (out: v_T) =
        Rust_primitives.Sequence.seq_remove #v_T self._0 (mk_usize 0)
      in
      let self:t_VecDeque v_T v_A = { self with _0 = tmp0 } <: t_VecDeque v_T v_A in
      self, (Core_models.Option.Option_Some out <: Core_models.Option.t_Option v_T)
      <:
      (t_VecDeque v_T v_A & Core_models.Option.t_Option v_T)
  in
  self, hax_temp_output <: (t_VecDeque v_T v_A & Core_models.Option.t_Option v_T)

let impl_5__iter (#v_T #v_A: Type0) (self: t_VecDeque v_T v_A)
    : Alloc.Collections.Vec_deque.Iter.t_Iter v_T =
  Alloc.Collections.Vec_deque.Iter.Iter
  (Rust_primitives.Sequence.seq_from_slice #v_T
      (Rust_primitives.Sequence.seq_to_slice #v_T self._0 <: t_Slice v_T))
  <:
  Alloc.Collections.Vec_deque.Iter.t_Iter v_T

let impl_5__remove (#v_T #v_A: Type0) (self: t_VecDeque v_T v_A) (index: usize)
    : (t_VecDeque v_T v_A & Core_models.Option.t_Option v_T) =
  let (self: t_VecDeque v_T v_A), (hax_temp_output: Core_models.Option.t_Option v_T) =
    if index <. (impl_5__len #v_T #v_A self <: usize)
    then
      let (tmp0: Rust_primitives.Sequence.t_Seq v_T), (out: v_T) =
        Rust_primitives.Sequence.seq_remove #v_T self._0 index
      in
      let self:t_VecDeque v_T v_A = { self with _0 = tmp0 } <: t_VecDeque v_T v_A in
      self, (Core_models.Option.Option_Some out <: Core_models.Option.t_Option v_T)
      <:
      (t_VecDeque v_T v_A & Core_models.Option.t_Option v_T)
    else
      self, (Core_models.Option.Option_None <: Core_models.Option.t_Option v_T)
      <:
      (t_VecDeque v_T v_A & Core_models.Option.t_Option v_T)
  in
  self, hax_temp_output <: (t_VecDeque v_T v_A & Core_models.Option.t_Option v_T)

let impl_5__push_back (#v_T #v_A: Type0) (self: t_VecDeque v_T v_A) (x: v_T)
    : Prims.Pure (t_VecDeque v_T v_A)
      (requires
        (Rust_primitives.Sequence.seq_len #v_T self._0 <: usize) <. Core_models.Num.impl_usize__MAX)
      (fun _ -> Prims.l_True) =
  let self:t_VecDeque v_T v_A =
    { self with _0 = Rust_primitives.Sequence.seq_push #v_T self._0 x } <: t_VecDeque v_T v_A
  in
  self

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_6 (#v_T #v_A: Type0) : Core_models.Ops.Index.t_Index (t_VecDeque v_T v_A) usize =
  {
    f_Output = v_T;
    f_index_pre
    =
    (fun (self_: t_VecDeque v_T v_A) (i: usize) -> i <. (impl_5__len #v_T #v_A self_ <: usize));
    f_index_post = (fun (self: t_VecDeque v_T v_A) (i: usize) (out: v_T) -> true);
    f_index
    =
    fun (self: t_VecDeque v_T v_A) (i: usize) -> Rust_primitives.Sequence.seq_index #v_T self._0 i
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_7': #v_T: Type0 -> #v_A: Type0
  -> Core_models.Iter.Traits.Collect.t_IntoIterator (t_VecDeque v_T v_A)

unfold
let impl_7 (#v_T #v_A: Type0) = impl_7' #v_T #v_A

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_8': #v_T: Type0
  -> Core_models.Iter.Traits.Collect.t_FromIterator (t_VecDeque v_T Alloc.Alloc.t_Global) v_T

unfold
let impl_8 (#v_T: Type0) = impl_8' #v_T

[@@ FStar.Tactics.Typeclasses.tcinstance]
let update_at_usize (#v_T #v_A: Type0)
    : Rust_primitives.Hax.update_at_tc (t_VecDeque v_T v_A) usize =
  {
    super_index = impl_6 #v_T #v_A;
    // `i` is deliberately left unannotated: the class gives it the refinement
    // `f_index_pre self i` (here `i < len self`), and annotating it `usize`
    // would drop exactly the bound `Seq.upd` needs.
    update_at = (fun self i x -> VecDeque (FStar.Seq.upd self._0 (v i) x) self._1)
  }
