module Core_models.Bundle
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Rust_primitives

/// See [`std::array::TryFromSliceError`]
type t_TryFromSliceError = | TryFromSliceError : t_TryFromSliceError

let impl_23__map
      (#v_T: Type0)
      (v_N: usize)
      (#v_F #v_U: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Ops.Function.t_FnOnce v_F v_T)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == v_U})
      (s: t_Array v_T v_N)
      (f: (v_T -> v_U))
    : t_Array v_U v_N = Rust_primitives.Slice.array_map #v_T #v_U v_N #(v_T -> v_U) s f

/// See [`std::array::as_slice`]
let impl_23__as_slice (#v_T: Type0) (v_N: usize) (s: t_Array v_T v_N) : t_Slice v_T =
  Rust_primitives.Slice.array_as_slice #v_T v_N s

/// See [`std::array::each_ref`]
let impl_23__each_ref (#v_T: Type0) (v_N: usize) (s: t_Array v_T v_N) : t_Array v_T v_N =
  Rust_primitives.Slice.array_from_fn #v_T
    v_N
    #(usize -> v_T)
    (fun i ->
        let i:usize = i in
        Rust_primitives.Slice.array_index #v_T v_N s i <: v_T)

let from_fn = Rust_primitives.Slice.array_from_fn

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_27 (#v_T: Type0) (v_N: usize) : Core_models.Ops.Index.t_Index (t_Array v_T v_N) usize =
  {
    f_Output = v_T;
    f_index_pre = (fun (self_: t_Array v_T v_N) (i: usize) -> i <. v_N);
    f_index_post = (fun (self: t_Array v_T v_N) (i: usize) (out: v_T) -> true);
    f_index
    =
    fun (self: t_Array v_T v_N) (i: usize) -> Rust_primitives.Slice.array_index #v_T v_N self i
  }

/// See [`std::default::Default`] for `[T; N]`
[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_25
      (#v_T: Type0)
      (v_N: usize)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Default.t_Default v_T)
    : Core_models.Default.t_Default (t_Array v_T v_N) =
  {
    f_default_pre = (fun (_: Prims.unit) -> true);
    f_default_post = (fun (_: Prims.unit) (out: t_Array v_T v_N) -> true);
    f_default
    =
    fun (_: Prims.unit) ->
      Rust_primitives.Slice.array_from_fn #v_T
        v_N
        #(usize -> v_T)
        (fun e_i ->
            let e_i:usize = e_i in
            Core_models.Default.f_default #v_T #FStar.Tactics.Typeclasses.solve () <: v_T)
  }

type t_IntoIter (v_T: Type0) (v_N: usize) =
  | IntoIter : Rust_primitives.Sequence.t_Seq v_T -> t_IntoIter v_T v_N

/// See [`std::cmp::Ordering`]
type t_Ordering =
  | Ordering_Less : t_Ordering
  | Ordering_Equal : t_Ordering
  | Ordering_Greater : t_Ordering

let anon_const_Ordering_Less__anon_const_0: isize = mk_isize (-1)

let anon_const_Ordering_Equal__anon_const_0: isize = mk_isize 0

let anon_const_Ordering_Greater__anon_const_0: isize = mk_isize 1

let t_Ordering_cast_to_repr (x: t_Ordering) : isize =
  match x <: t_Ordering with
  | Ordering_Less  -> anon_const_Ordering_Less__anon_const_0
  | Ordering_Equal  -> anon_const_Ordering_Equal__anon_const_0
  | Ordering_Greater  -> anon_const_Ordering_Greater__anon_const_0

/// See [`std::cmp::Reverse`]
type t_Reverse (v_T: Type0) = | Reverse : v_T -> t_Reverse v_T

/// See [`std::cmp::Ordering::is_eq`]
let impl_58__is_eq (self: t_Ordering) : bool =
  match self <: t_Ordering with
  | Ordering_Equal  -> true
  | _ -> false

/// See [`std::cmp::Ordering::is_ne`]
let impl_58__is_ne (self: t_Ordering) : bool =
  match self <: t_Ordering with
  | Ordering_Less  | Ordering_Greater  -> true
  | _ -> false

/// See [`std::cmp::Ordering::is_lt`]
let impl_58__is_lt (self: t_Ordering) : bool =
  match self <: t_Ordering with
  | Ordering_Less  -> true
  | _ -> false

/// See [`std::cmp::Ordering::is_gt`]
let impl_58__is_gt (self: t_Ordering) : bool =
  match self <: t_Ordering with
  | Ordering_Greater  -> true
  | _ -> false

/// See [`std::cmp::Ordering::is_le`]
let impl_58__is_le (self: t_Ordering) : bool =
  match self <: t_Ordering with
  | Ordering_Less  | Ordering_Equal  -> true
  | _ -> false

/// See [`std::cmp::Ordering::is_ge`]
let impl_58__is_ge (self: t_Ordering) : bool =
  match self <: t_Ordering with
  | Ordering_Greater  | Ordering_Equal  -> true
  | _ -> false

/// See [`std::cmp::Ordering::reverse`]
let impl_58__reverse (self: t_Ordering) : t_Ordering =
  match self <: t_Ordering with
  | Ordering_Less  -> Ordering_Greater <: t_Ordering
  | Ordering_Equal  -> Ordering_Equal <: t_Ordering
  | Ordering_Greater  -> Ordering_Less <: t_Ordering

/// See [`std::cmp::Ordering::then`]
let impl_58__then (self other: t_Ordering) : t_Ordering =
  match self <: t_Ordering with
  | Ordering_Equal  -> other
  | _ -> self

/// See [`std::cmp::Ordering::then_with`]
let impl_58__then_with
      (#v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Core_models.Ops.Function.t_FnOnce v_F Prims.unit)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == t_Ordering})
      (self: t_Ordering)
      (f: v_F)
    : t_Ordering =
  match self <: t_Ordering with
  | Ordering_Equal  ->
    Core_models.Ops.Function.f_call_once #v_F
      #Prims.unit
      #FStar.Tactics.Typeclasses.solve
      f
      (() <: Prims.unit)
  | _ -> self

/// See [`std::convert::Infallible`]
type t_Infallible = | Infallible : t_Infallible

/// See [`std::iter::Enumerate`]
type t_Enumerate (v_I: Type0) = {
  f_iter:v_I;
  f_count:usize
}

let impl__new (#v_I: Type0) (iter: v_I) : t_Enumerate v_I =
  { f_iter = iter; f_count = mk_usize 0 } <: t_Enumerate v_I

/// See [`std::iter::Filter`]
type t_Filter (v_I: Type0) (v_P: Type0) = {
  f_iter:v_I;
  f_predicate:v_P
}

let impl__new__from__filter (#v_I #v_P: Type0) (iter: v_I) (predicate: v_P) : t_Filter v_I v_P =
  { f_iter = iter; f_predicate = predicate } <: t_Filter v_I v_P

/// See [`std::iter::FilterMap`]
type t_FilterMap (v_I: Type0) (v_F: Type0) = {
  f_iter:v_I;
  f_f:v_F
}

let impl__new__from__filter_map (#v_I #v_F: Type0) (iter: v_I) (f: v_F) : t_FilterMap v_I v_F =
  { f_iter = iter; f_f = f } <: t_FilterMap v_I v_F

/// See [`std::iter::Fuse`] — once the inner iterator returns `None`, always `None`.
type t_Fuse (v_I: Type0) = {
  f_iter:v_I;
  f_done:bool
}

let impl__new__from__fuse (#v_I: Type0) (iter: v_I) : t_Fuse v_I =
  { f_iter = iter; f_done = false } <: t_Fuse v_I

/// See [`std::iter::Inspect`]
type t_Inspect (v_I: Type0) (v_F: Type0) = {
  f_iter:v_I;
  f_f:v_F
}

let impl__new__from__inspect (#v_I #v_F: Type0) (iter: v_I) (f: v_F) : t_Inspect v_I v_F =
  { f_iter = iter; f_f = f } <: t_Inspect v_I v_F

/// See [`std::iter::Map`]
type t_Map (v_I: Type0) (v_F: Type0) = {
  f_iter:v_I;
  f_f:v_F
}

let impl__new__from__map (#v_I #v_F: Type0) (iter: v_I) (f: v_F) : t_Map v_I v_F =
  { f_iter = iter; f_f = f } <: t_Map v_I v_F

/// See [`std::iter::MapWhile`]
type t_MapWhile (v_I: Type0) (v_F: Type0) = {
  f_iter:v_I;
  f_f:v_F
}

let impl__new__from__map_while (#v_I #v_F: Type0) (iter: v_I) (f: v_F) : t_MapWhile v_I v_F =
  { f_iter = iter; f_f = f } <: t_MapWhile v_I v_F

/// See [`std::iter::Skip`]
type t_Skip (v_I: Type0) = {
  f_iter:v_I;
  f_n:usize
}

let impl__new__from__skip (#v_I: Type0) (iter: v_I) (n: usize) : t_Skip v_I =
  { f_iter = iter; f_n = n } <: t_Skip v_I

/// See [`std::iter::SkipWhile`]
type t_SkipWhile (v_I: Type0) (v_P: Type0) = {
  f_iter:v_I;
  f_flag:bool;
  f_predicate:v_P
}

let impl__new__from__skip_while (#v_I #v_P: Type0) (iter: v_I) (predicate: v_P)
    : t_SkipWhile v_I v_P =
  { f_iter = iter; f_flag = false; f_predicate = predicate } <: t_SkipWhile v_I v_P

/// See [`std::iter::StepBy`]
type t_StepBy (v_I: Type0) = {
  f_iter:v_I;
  f_step:usize
}

let impl__new__from__step_by (#v_I: Type0) (iter: v_I) (step: usize)
    : Prims.Pure (t_StepBy v_I) (requires step >. mk_usize 0) (fun _ -> Prims.l_True) =
  let _:Prims.unit =
    if step =. mk_usize 0 then Core_models.Panicking.Internal.panic #Prims.unit ()
  in
  { f_iter = iter; f_step = step } <: t_StepBy v_I

/// See [`std::iter::Take`]
type t_Take (v_I: Type0) = {
  f_iter:v_I;
  f_n:usize
}

let impl__new__from__take (#v_I: Type0) (iter: v_I) (n: usize) : t_Take v_I =
  { f_iter = iter; f_n = n } <: t_Take v_I

/// See [`std::iter::TakeWhile`]
type t_TakeWhile (v_I: Type0) (v_P: Type0) = {
  f_iter:v_I;
  f_flag:bool;
  f_predicate:v_P
}

let impl__new__from__take_while (#v_I #v_P: Type0) (iter: v_I) (predicate: v_P)
    : t_TakeWhile v_I v_P =
  { f_iter = iter; f_flag = false; f_predicate = predicate } <: t_TakeWhile v_I v_P

/// See [`std::iter::Zip`]
type t_Zip (v_I1: Type0) (v_I2: Type0) = {
  f_it1:v_I1;
  f_it2:v_I2
}

/// See [`std::primitive::u8::MIN`] (and similar for other unsigned integer types)
let impl_6__MIN: u8 = mk_u8 0

/// See [`std::primitive::u8::MAX`] (and similar for other unsigned integer types)
let impl_6__MAX: u8 = mk_u8 255

/// See [`std::primitive::u8::BITS`] (and similar for other unsigned integer types)
let impl_6__BITS: u32 = mk_u32 8

/// See [`std::primitive::u8::wrapping_add`] (and similar for other unsigned integer types)
let impl_6__wrapping_add (x y: u8) : u8 = Rust_primitives.Arithmetic.wrapping_add_u8 x y

/// See [`std::primitive::u8::saturating_add`] (and similar for other integer types)
let impl_6__saturating_add (x y: u8) : u8 = Rust_primitives.Arithmetic.saturating_add_u8 x y

/// See [`std::primitive::u8::overflowing_add`] (and similar for other integer types)
let impl_6__overflowing_add (x y: u8) : (u8 & bool) =
  Rust_primitives.Arithmetic.overflowing_add_u8 x y

/// See [`std::primitive::u8::wrapping_sub`] (and similar for other integer types)
let impl_6__wrapping_sub (x y: u8) : u8 = Rust_primitives.Arithmetic.wrapping_sub_u8 x y

/// See [`std::primitive::u8::saturating_sub`] (and similar for other integer types)
let impl_6__saturating_sub (x y: u8) : u8 = Rust_primitives.Arithmetic.saturating_sub_u8 x y

/// See [`std::primitive::u8::overflowing_sub`] (and similar for other integer types)
let impl_6__overflowing_sub (x y: u8) : (u8 & bool) =
  Rust_primitives.Arithmetic.overflowing_sub_u8 x y

/// See [`std::primitive::u8::wrapping_mul`] (and similar for other integer types)
let impl_6__wrapping_mul (x y: u8) : u8 = Rust_primitives.Arithmetic.wrapping_mul_u8 x y

/// See [`std::primitive::u8::saturating_mul`] (and similar for other integer types)
let impl_6__saturating_mul (x y: u8) : u8 = Rust_primitives.Arithmetic.saturating_mul_u8 x y

/// See [`std::primitive::u8::overflowing_mul`] (and similar for other integer types)
let impl_6__overflowing_mul (x y: u8) : (u8 & bool) =
  Rust_primitives.Arithmetic.overflowing_mul_u8 x y

/// See [`std::primitive::u8::overflowing_pow`] (and similar for other integer types)
let impl_6__overflowing_pow (x: u8) (exp: u32) : (u8 & bool) =
  Rust_primitives.Arithmetic.overflowing_pow_u8 x exp

/// See [`std::primitive::u8::count_ones`] (and similar for other integer types)
let impl_6__count_ones (x: u8) : u32 =
  let n:u32 = mk_u32 0 in
  let n:u32 =
    Rust_primitives.Hax.Folds.fold_range (mk_u32 0)
      (mk_u32 8)
      (fun n i ->
          let n:u32 = n in
          let i:u32 = i in
          n <=. i <: bool)
      n
      (fun n i ->
          let n:u32 = n in
          let i:u32 = i in
          if ((x >>! i <: u8) &. mk_u8 1 <: u8) =. mk_u8 1 <: bool
          then
            let n:u32 = n +! mk_u32 1 in
            n
          else n)
  in
  n

/// See [`std::primitive::u8::leading_zeros`] (and similar for other integer types)
assume
val impl_6__leading_zeros': x: u8 -> u32

unfold
let impl_6__leading_zeros = impl_6__leading_zeros'

/// See [`std::primitive::u8::from_be_bytes`] (and similar for other integer types)
assume
val impl_6__from_be_bytes': bytes: t_Array u8 (mk_usize 1) -> u8

unfold
let impl_6__from_be_bytes = impl_6__from_be_bytes'

/// See [`std::primitive::u8::from_le_bytes`] (and similar for other integer types)
assume
val impl_6__from_le_bytes': bytes: t_Array u8 (mk_usize 1) -> u8

unfold
let impl_6__from_le_bytes = impl_6__from_le_bytes'

/// See [`std::primitive::u8::to_be_bytes`] (and similar for other integer types)
assume
val impl_6__to_be_bytes': bytes: u8 -> t_Array u8 (mk_usize 1)

unfold
let impl_6__to_be_bytes = impl_6__to_be_bytes'

/// See [`std::primitive::u8::to_le_bytes`] (and similar for other integer types)
let impl_6__to_le_bytes (bytes: u8) : t_Array u8 (mk_usize 1) =
  Rust_primitives.Slice.array_from_fn #u8
    (mk_usize 1)
    #(usize -> u8)
    (fun i ->
        let i:usize = i in
        bytes >>! ((mk_u32 8 *! (cast (i <: usize) <: u32) <: u32) %! mk_u32 8 <: u32) <: u8)

/// See [`std::primitive::u8::is_power_of_two`] (and similar for other unsigned integer types)
let impl_6__is_power_of_two (x: u8) : bool =
  x <>. mk_u8 0 && (x &. (x -! mk_u8 1 <: u8) <: u8) =. mk_u8 0

/// See [`std::primitive::u8::is_multiple_of`] (and similar for other unsigned integer types)
let impl_6__is_multiple_of (x y: u8) : bool =
  if y =. mk_u8 0 then x =. mk_u8 0 else (x %! y <: u8) =. mk_u8 0

/// See [`std::primitive::u8::wrapping_neg`] (and similar for other integer types)
let impl_6__wrapping_neg (x: u8) : u8 = Rust_primitives.Arithmetic.wrapping_sub_u8 (mk_u8 0) x

/// See [`std::primitive::u8::unchecked_add`] (and similar for other integer types)
let impl_6__unchecked_add (x y: u8)
    : Prims.Pure u8
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_6__MAX <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x +! y

/// See [`std::primitive::u8::unchecked_sub`] (and similar for other integer types)
let impl_6__unchecked_sub (x y: u8) : Prims.Pure u8 (requires x >=. y) (fun _ -> Prims.l_True) =
  x -! y

/// See [`std::primitive::u8::unchecked_mul`] (and similar for other integer types)
let impl_6__unchecked_mul (x y: u8)
    : Prims.Pure u8
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) *
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_6__MAX <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x *! y

/// See [`std::primitive::u8::rem_euclid`] (and similar for other integer types)
let impl_6__rem_euclid (x y: u8) : Prims.Pure u8 (requires y <>. mk_u8 0) (fun _ -> Prims.l_True) =
  Rust_primitives.Arithmetic.rem_euclid_u8 x y

/// See [`std::primitive::u8::pow`] (and similar for other integer types)
let impl_6__pow (x: u8) (exp: u32)
    : Prims.Pure u8
      (requires (impl_6__overflowing_pow x exp <: (u8 & bool))._2 =. false)
      (fun _ -> Prims.l_True) = Rust_primitives.Arithmetic.pow_u8 x exp

[@@ "opaque_to_smt"]

/// See [`std::primitive::u8::rotate_right`] (and similar for other integer types)
let impl_6__rotate_right (x: u8) (n: u32) : u8 =
  let m:u32 = n %! mk_u32 8 in
  if m =. mk_u32 0 then x else (x >>! m <: u8) ^. (x <<! (mk_u32 8 -! m <: u32) <: u8)

[@@ "opaque_to_smt"]

/// See [`std::primitive::u8::rotate_left`] (and similar for other integer types)
let impl_6__rotate_left (x: u8) (n: u32) : u8 =
  let m:u32 = n %! mk_u32 8 in
  if m =. mk_u32 0 then x else (x <<! m <: u8) ^. (x >>! (mk_u32 8 -! m <: u32) <: u8)

/// See [`std::primitive::u8::ilog2`] (and similar for other integer types)
assume
val impl_6__ilog2': x: u8 -> Prims.Pure u32 (requires x >. mk_u8 0) (fun _ -> Prims.l_True)

unfold
let impl_6__ilog2 = impl_6__ilog2'

/// See [`std::primitive::u8::unchecked_div`] (and similar for other integer types)
let impl_6__unchecked_div (x y: u8) : Prims.Pure u8 (requires y <>. mk_u8 0) (fun _ -> Prims.l_True) =
  x /! y

/// See [`std::primitive::u8::unchecked_rem`] (and similar for other integer types)
let impl_6__unchecked_rem (x y: u8) : Prims.Pure u8 (requires y <>. mk_u8 0) (fun _ -> Prims.l_True) =
  x %! y

/// See [`std::primitive::u8::div_ceil`] (and similar for other unsigned integer types)
let impl_6__div_ceil (x y: u8) : Prims.Pure u8 (requires y <>. mk_u8 0) (fun _ -> Prims.l_True) =
  let d:u8 = x /! y in
  let r:u8 = x %! y in
  if r >. mk_u8 0 then d +! mk_u8 1 else d

/// See [`std::primitive::u8::MIN`] (and similar for other unsigned integer types)
let impl_7__MIN: u16 = mk_u16 0

/// See [`std::primitive::u8::MAX`] (and similar for other unsigned integer types)
let impl_7__MAX: u16 = mk_u16 65535

/// See [`std::primitive::u8::BITS`] (and similar for other unsigned integer types)
let impl_7__BITS: u32 = mk_u32 16

/// See [`std::primitive::u8::wrapping_add`] (and similar for other unsigned integer types)
let impl_7__wrapping_add (x y: u16) : u16 = Rust_primitives.Arithmetic.wrapping_add_u16 x y

/// See [`std::primitive::u8::saturating_add`] (and similar for other integer types)
let impl_7__saturating_add (x y: u16) : u16 = Rust_primitives.Arithmetic.saturating_add_u16 x y

/// See [`std::primitive::u8::overflowing_add`] (and similar for other integer types)
let impl_7__overflowing_add (x y: u16) : (u16 & bool) =
  Rust_primitives.Arithmetic.overflowing_add_u16 x y

/// See [`std::primitive::u8::wrapping_sub`] (and similar for other integer types)
let impl_7__wrapping_sub (x y: u16) : u16 = Rust_primitives.Arithmetic.wrapping_sub_u16 x y

/// See [`std::primitive::u8::saturating_sub`] (and similar for other integer types)
let impl_7__saturating_sub (x y: u16) : u16 = Rust_primitives.Arithmetic.saturating_sub_u16 x y

/// See [`std::primitive::u8::overflowing_sub`] (and similar for other integer types)
let impl_7__overflowing_sub (x y: u16) : (u16 & bool) =
  Rust_primitives.Arithmetic.overflowing_sub_u16 x y

/// See [`std::primitive::u8::wrapping_mul`] (and similar for other integer types)
let impl_7__wrapping_mul (x y: u16) : u16 = Rust_primitives.Arithmetic.wrapping_mul_u16 x y

/// See [`std::primitive::u8::saturating_mul`] (and similar for other integer types)
let impl_7__saturating_mul (x y: u16) : u16 = Rust_primitives.Arithmetic.saturating_mul_u16 x y

/// See [`std::primitive::u8::overflowing_mul`] (and similar for other integer types)
let impl_7__overflowing_mul (x y: u16) : (u16 & bool) =
  Rust_primitives.Arithmetic.overflowing_mul_u16 x y

/// See [`std::primitive::u8::overflowing_pow`] (and similar for other integer types)
let impl_7__overflowing_pow (x: u16) (exp: u32) : (u16 & bool) =
  Rust_primitives.Arithmetic.overflowing_pow_u16 x exp

/// See [`std::primitive::u8::count_ones`] (and similar for other integer types)
let impl_7__count_ones (x: u16) : u32 =
  let n:u32 = mk_u32 0 in
  let n:u32 =
    Rust_primitives.Hax.Folds.fold_range (mk_u32 0)
      (mk_u32 16)
      (fun n i ->
          let n:u32 = n in
          let i:u32 = i in
          n <=. i <: bool)
      n
      (fun n i ->
          let n:u32 = n in
          let i:u32 = i in
          if ((x >>! i <: u16) &. mk_u16 1 <: u16) =. mk_u16 1 <: bool
          then
            let n:u32 = n +! mk_u32 1 in
            n
          else n)
  in
  n

/// See [`std::primitive::u8::leading_zeros`] (and similar for other integer types)
assume
val impl_7__leading_zeros': x: u16 -> u32

unfold
let impl_7__leading_zeros = impl_7__leading_zeros'

/// See [`std::primitive::u8::from_be_bytes`] (and similar for other integer types)
assume
val impl_7__from_be_bytes': bytes: t_Array u8 (mk_usize 2) -> u16

unfold
let impl_7__from_be_bytes = impl_7__from_be_bytes'

/// See [`std::primitive::u8::from_le_bytes`] (and similar for other integer types)
assume
val impl_7__from_le_bytes': bytes: t_Array u8 (mk_usize 2) -> u16

unfold
let impl_7__from_le_bytes = impl_7__from_le_bytes'

/// See [`std::primitive::u8::to_be_bytes`] (and similar for other integer types)
assume
val impl_7__to_be_bytes': bytes: u16 -> t_Array u8 (mk_usize 2)

unfold
let impl_7__to_be_bytes = impl_7__to_be_bytes'

/// See [`std::primitive::u8::to_le_bytes`] (and similar for other integer types)
let impl_7__to_le_bytes (bytes: u16) : t_Array u8 (mk_usize 2) =
  Rust_primitives.Slice.array_from_fn #u8
    (mk_usize 2)
    #(usize -> u8)
    (fun i ->
        let i:usize = i in
        cast (bytes >>! ((mk_u32 8 *! (cast (i <: usize) <: u32) <: u32) %! mk_u32 16 <: u32) <: u16
          )
        <:
        u8)

/// See [`std::primitive::u8::is_power_of_two`] (and similar for other unsigned integer types)
let impl_7__is_power_of_two (x: u16) : bool =
  x <>. mk_u16 0 && (x &. (x -! mk_u16 1 <: u16) <: u16) =. mk_u16 0

/// See [`std::primitive::u8::is_multiple_of`] (and similar for other unsigned integer types)
let impl_7__is_multiple_of (x y: u16) : bool =
  if y =. mk_u16 0 then x =. mk_u16 0 else (x %! y <: u16) =. mk_u16 0

/// See [`std::primitive::u8::wrapping_neg`] (and similar for other integer types)
let impl_7__wrapping_neg (x: u16) : u16 = Rust_primitives.Arithmetic.wrapping_sub_u16 (mk_u16 0) x

/// See [`std::primitive::u8::unchecked_add`] (and similar for other integer types)
let impl_7__unchecked_add (x y: u16)
    : Prims.Pure u16
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_7__MAX <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x +! y

/// See [`std::primitive::u8::unchecked_sub`] (and similar for other integer types)
let impl_7__unchecked_sub (x y: u16) : Prims.Pure u16 (requires x >=. y) (fun _ -> Prims.l_True) =
  x -! y

/// See [`std::primitive::u8::unchecked_mul`] (and similar for other integer types)
let impl_7__unchecked_mul (x y: u16)
    : Prims.Pure u16
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) *
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_7__MAX <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x *! y

/// See [`std::primitive::u8::rem_euclid`] (and similar for other integer types)
let impl_7__rem_euclid (x y: u16) : Prims.Pure u16 (requires y <>. mk_u16 0) (fun _ -> Prims.l_True) =
  Rust_primitives.Arithmetic.rem_euclid_u16 x y

/// See [`std::primitive::u8::pow`] (and similar for other integer types)
let impl_7__pow (x: u16) (exp: u32)
    : Prims.Pure u16
      (requires (impl_7__overflowing_pow x exp <: (u16 & bool))._2 =. false)
      (fun _ -> Prims.l_True) = Rust_primitives.Arithmetic.pow_u16 x exp

[@@ "opaque_to_smt"]

/// See [`std::primitive::u8::rotate_right`] (and similar for other integer types)
let impl_7__rotate_right (x: u16) (n: u32) : u16 =
  let m:u32 = n %! mk_u32 16 in
  if m =. mk_u32 0 then x else (x >>! m <: u16) ^. (x <<! (mk_u32 16 -! m <: u32) <: u16)

[@@ "opaque_to_smt"]

/// See [`std::primitive::u8::rotate_left`] (and similar for other integer types)
let impl_7__rotate_left (x: u16) (n: u32) : u16 =
  let m:u32 = n %! mk_u32 16 in
  if m =. mk_u32 0 then x else (x <<! m <: u16) ^. (x >>! (mk_u32 16 -! m <: u32) <: u16)

/// See [`std::primitive::u8::ilog2`] (and similar for other integer types)
assume
val impl_7__ilog2': x: u16 -> Prims.Pure u32 (requires x >. mk_u16 0) (fun _ -> Prims.l_True)

unfold
let impl_7__ilog2 = impl_7__ilog2'

/// See [`std::primitive::u8::unchecked_div`] (and similar for other integer types)
let impl_7__unchecked_div (x y: u16)
    : Prims.Pure u16 (requires y <>. mk_u16 0) (fun _ -> Prims.l_True) = x /! y

/// See [`std::primitive::u8::unchecked_rem`] (and similar for other integer types)
let impl_7__unchecked_rem (x y: u16)
    : Prims.Pure u16 (requires y <>. mk_u16 0) (fun _ -> Prims.l_True) = x %! y

/// See [`std::primitive::u8::div_ceil`] (and similar for other unsigned integer types)
let impl_7__div_ceil (x y: u16) : Prims.Pure u16 (requires y <>. mk_u16 0) (fun _ -> Prims.l_True) =
  let d:u16 = x /! y in
  let r:u16 = x %! y in
  if r >. mk_u16 0 then d +! mk_u16 1 else d

/// See [`std::primitive::u8::MIN`] (and similar for other unsigned integer types)
let impl_8__MIN: u32 = mk_u32 0

/// See [`std::primitive::u8::MAX`] (and similar for other unsigned integer types)
let impl_8__MAX: u32 = mk_u32 4294967295

/// See [`std::primitive::u8::BITS`] (and similar for other unsigned integer types)
let impl_8__BITS: u32 = mk_u32 32

/// See [`std::primitive::u8::wrapping_add`] (and similar for other unsigned integer types)
let impl_8__wrapping_add (x y: u32) : u32 = Rust_primitives.Arithmetic.wrapping_add_u32 x y

/// See [`std::primitive::u8::saturating_add`] (and similar for other integer types)
let impl_8__saturating_add (x y: u32) : u32 = Rust_primitives.Arithmetic.saturating_add_u32 x y

/// See [`std::primitive::u8::overflowing_add`] (and similar for other integer types)
let impl_8__overflowing_add (x y: u32) : (u32 & bool) =
  Rust_primitives.Arithmetic.overflowing_add_u32 x y

/// See [`std::primitive::u8::wrapping_sub`] (and similar for other integer types)
let impl_8__wrapping_sub (x y: u32) : u32 = Rust_primitives.Arithmetic.wrapping_sub_u32 x y

/// See [`std::primitive::u8::saturating_sub`] (and similar for other integer types)
let impl_8__saturating_sub (x y: u32) : u32 = Rust_primitives.Arithmetic.saturating_sub_u32 x y

/// See [`std::primitive::u8::overflowing_sub`] (and similar for other integer types)
let impl_8__overflowing_sub (x y: u32) : (u32 & bool) =
  Rust_primitives.Arithmetic.overflowing_sub_u32 x y

/// See [`std::primitive::u8::wrapping_mul`] (and similar for other integer types)
let impl_8__wrapping_mul (x y: u32) : u32 = Rust_primitives.Arithmetic.wrapping_mul_u32 x y

/// See [`std::primitive::u8::saturating_mul`] (and similar for other integer types)
let impl_8__saturating_mul (x y: u32) : u32 = Rust_primitives.Arithmetic.saturating_mul_u32 x y

/// See [`std::primitive::u8::overflowing_mul`] (and similar for other integer types)
let impl_8__overflowing_mul (x y: u32) : (u32 & bool) =
  Rust_primitives.Arithmetic.overflowing_mul_u32 x y

/// See [`std::primitive::u8::overflowing_pow`] (and similar for other integer types)
let impl_8__overflowing_pow (x exp: u32) : (u32 & bool) =
  Rust_primitives.Arithmetic.overflowing_pow_u32 x exp

/// See [`std::primitive::u8::count_ones`] (and similar for other integer types)
let impl_8__count_ones (x: u32) : u32 =
  let n:u32 = mk_u32 0 in
  let n:u32 =
    Rust_primitives.Hax.Folds.fold_range (mk_u32 0)
      (mk_u32 32)
      (fun n i ->
          let n:u32 = n in
          let i:u32 = i in
          n <=. i <: bool)
      n
      (fun n i ->
          let n:u32 = n in
          let i:u32 = i in
          if ((x >>! i <: u32) &. mk_u32 1 <: u32) =. mk_u32 1 <: bool
          then
            let n:u32 = n +! mk_u32 1 in
            n
          else n)
  in
  n

/// See [`std::primitive::u8::leading_zeros`] (and similar for other integer types)
assume
val impl_8__leading_zeros': x: u32 -> u32

unfold
let impl_8__leading_zeros = impl_8__leading_zeros'

/// See [`std::primitive::u8::from_be_bytes`] (and similar for other integer types)
assume
val impl_8__from_be_bytes': bytes: t_Array u8 (mk_usize 4) -> u32

unfold
let impl_8__from_be_bytes = impl_8__from_be_bytes'

/// See [`std::primitive::u8::from_le_bytes`] (and similar for other integer types)
assume
val impl_8__from_le_bytes': bytes: t_Array u8 (mk_usize 4) -> u32

unfold
let impl_8__from_le_bytes = impl_8__from_le_bytes'

/// See [`std::primitive::u8::to_be_bytes`] (and similar for other integer types)
assume
val impl_8__to_be_bytes': bytes: u32 -> t_Array u8 (mk_usize 4)

unfold
let impl_8__to_be_bytes = impl_8__to_be_bytes'

/// See [`std::primitive::u8::to_le_bytes`] (and similar for other integer types)
let impl_8__to_le_bytes (bytes: u32) : t_Array u8 (mk_usize 4) =
  Rust_primitives.Slice.array_from_fn #u8
    (mk_usize 4)
    #(usize -> u8)
    (fun i ->
        let i:usize = i in
        cast (bytes >>! ((mk_u32 8 *! (cast (i <: usize) <: u32) <: u32) %! mk_u32 32 <: u32) <: u32
          )
        <:
        u8)

/// See [`std::primitive::u8::is_power_of_two`] (and similar for other unsigned integer types)
let impl_8__is_power_of_two (x: u32) : bool =
  x <>. mk_u32 0 && (x &. (x -! mk_u32 1 <: u32) <: u32) =. mk_u32 0

/// See [`std::primitive::u8::is_multiple_of`] (and similar for other unsigned integer types)
let impl_8__is_multiple_of (x y: u32) : bool =
  if y =. mk_u32 0 then x =. mk_u32 0 else (x %! y <: u32) =. mk_u32 0

/// See [`std::primitive::u8::wrapping_neg`] (and similar for other integer types)
let impl_8__wrapping_neg (x: u32) : u32 = Rust_primitives.Arithmetic.wrapping_sub_u32 (mk_u32 0) x

/// See [`std::primitive::u8::unchecked_add`] (and similar for other integer types)
let impl_8__unchecked_add (x y: u32)
    : Prims.Pure u32
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_8__MAX <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x +! y

/// See [`std::primitive::u8::unchecked_sub`] (and similar for other integer types)
let impl_8__unchecked_sub (x y: u32) : Prims.Pure u32 (requires x >=. y) (fun _ -> Prims.l_True) =
  x -! y

/// See [`std::primitive::u8::unchecked_mul`] (and similar for other integer types)
let impl_8__unchecked_mul (x y: u32)
    : Prims.Pure u32
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) *
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_8__MAX <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x *! y

/// See [`std::primitive::u8::rem_euclid`] (and similar for other integer types)
let impl_8__rem_euclid (x y: u32) : Prims.Pure u32 (requires y <>. mk_u32 0) (fun _ -> Prims.l_True) =
  Rust_primitives.Arithmetic.rem_euclid_u32 x y

/// See [`std::primitive::u8::pow`] (and similar for other integer types)
let impl_8__pow (x exp: u32)
    : Prims.Pure u32
      (requires (impl_8__overflowing_pow x exp <: (u32 & bool))._2 =. false)
      (fun _ -> Prims.l_True) = Rust_primitives.Arithmetic.pow_u32 x exp

[@@ "opaque_to_smt"]

/// See [`std::primitive::u8::rotate_right`] (and similar for other integer types)
let impl_8__rotate_right (x n: u32) : u32 =
  let m:u32 = n %! mk_u32 32 in
  if m =. mk_u32 0 then x else (x >>! m <: u32) ^. (x <<! (mk_u32 32 -! m <: u32) <: u32)

[@@ "opaque_to_smt"]

/// See [`std::primitive::u8::rotate_left`] (and similar for other integer types)
let impl_8__rotate_left (x n: u32) : u32 =
  let m:u32 = n %! mk_u32 32 in
  if m =. mk_u32 0 then x else (x <<! m <: u32) ^. (x >>! (mk_u32 32 -! m <: u32) <: u32)

/// See [`std::primitive::u8::ilog2`] (and similar for other integer types)
assume
val impl_8__ilog2': x: u32 -> Prims.Pure u32 (requires x >. mk_u32 0) (fun _ -> Prims.l_True)

unfold
let impl_8__ilog2 = impl_8__ilog2'

/// See [`std::primitive::u8::unchecked_div`] (and similar for other integer types)
let impl_8__unchecked_div (x y: u32)
    : Prims.Pure u32 (requires y <>. mk_u32 0) (fun _ -> Prims.l_True) = x /! y

/// See [`std::primitive::u8::unchecked_rem`] (and similar for other integer types)
let impl_8__unchecked_rem (x y: u32)
    : Prims.Pure u32 (requires y <>. mk_u32 0) (fun _ -> Prims.l_True) = x %! y

/// See [`std::primitive::u8::div_ceil`] (and similar for other unsigned integer types)
let impl_8__div_ceil (x y: u32) : Prims.Pure u32 (requires y <>. mk_u32 0) (fun _ -> Prims.l_True) =
  let d:u32 = x /! y in
  let r:u32 = x %! y in
  if r >. mk_u32 0 then d +! mk_u32 1 else d

/// See [`std::primitive::u8::MIN`] (and similar for other unsigned integer types)
let impl_9__MIN: u64 = mk_u64 0

/// See [`std::primitive::u8::MAX`] (and similar for other unsigned integer types)
let impl_9__MAX: u64 = mk_u64 18446744073709551615

/// See [`std::primitive::u8::BITS`] (and similar for other unsigned integer types)
let impl_9__BITS: u32 = mk_u32 64

/// See [`std::primitive::u8::wrapping_add`] (and similar for other unsigned integer types)
let impl_9__wrapping_add (x y: u64) : u64 = Rust_primitives.Arithmetic.wrapping_add_u64 x y

/// See [`std::primitive::u8::saturating_add`] (and similar for other integer types)
let impl_9__saturating_add (x y: u64) : u64 = Rust_primitives.Arithmetic.saturating_add_u64 x y

/// See [`std::primitive::u8::overflowing_add`] (and similar for other integer types)
let impl_9__overflowing_add (x y: u64) : (u64 & bool) =
  Rust_primitives.Arithmetic.overflowing_add_u64 x y

/// See [`std::primitive::u8::wrapping_sub`] (and similar for other integer types)
let impl_9__wrapping_sub (x y: u64) : u64 = Rust_primitives.Arithmetic.wrapping_sub_u64 x y

/// See [`std::primitive::u8::saturating_sub`] (and similar for other integer types)
let impl_9__saturating_sub (x y: u64) : u64 = Rust_primitives.Arithmetic.saturating_sub_u64 x y

/// See [`std::primitive::u8::overflowing_sub`] (and similar for other integer types)
let impl_9__overflowing_sub (x y: u64) : (u64 & bool) =
  Rust_primitives.Arithmetic.overflowing_sub_u64 x y

/// See [`std::primitive::u8::wrapping_mul`] (and similar for other integer types)
let impl_9__wrapping_mul (x y: u64) : u64 = Rust_primitives.Arithmetic.wrapping_mul_u64 x y

/// See [`std::primitive::u8::saturating_mul`] (and similar for other integer types)
let impl_9__saturating_mul (x y: u64) : u64 = Rust_primitives.Arithmetic.saturating_mul_u64 x y

/// See [`std::primitive::u8::overflowing_mul`] (and similar for other integer types)
let impl_9__overflowing_mul (x y: u64) : (u64 & bool) =
  Rust_primitives.Arithmetic.overflowing_mul_u64 x y

/// See [`std::primitive::u8::overflowing_pow`] (and similar for other integer types)
let impl_9__overflowing_pow (x: u64) (exp: u32) : (u64 & bool) =
  Rust_primitives.Arithmetic.overflowing_pow_u64 x exp

/// See [`std::primitive::u8::count_ones`] (and similar for other integer types)
let impl_9__count_ones (x: u64) : u32 =
  let n:u32 = mk_u32 0 in
  let n:u32 =
    Rust_primitives.Hax.Folds.fold_range (mk_u32 0)
      (mk_u32 64)
      (fun n i ->
          let n:u32 = n in
          let i:u32 = i in
          n <=. i <: bool)
      n
      (fun n i ->
          let n:u32 = n in
          let i:u32 = i in
          if ((x >>! i <: u64) &. mk_u64 1 <: u64) =. mk_u64 1 <: bool
          then
            let n:u32 = n +! mk_u32 1 in
            n
          else n)
  in
  n

/// See [`std::primitive::u8::leading_zeros`] (and similar for other integer types)
assume
val impl_9__leading_zeros': x: u64 -> u32

unfold
let impl_9__leading_zeros = impl_9__leading_zeros'

/// See [`std::primitive::u8::from_be_bytes`] (and similar for other integer types)
assume
val impl_9__from_be_bytes': bytes: t_Array u8 (mk_usize 8) -> u64

unfold
let impl_9__from_be_bytes = impl_9__from_be_bytes'

/// See [`std::primitive::u8::from_le_bytes`] (and similar for other integer types)
assume
val impl_9__from_le_bytes': bytes: t_Array u8 (mk_usize 8) -> u64

unfold
let impl_9__from_le_bytes = impl_9__from_le_bytes'

/// See [`std::primitive::u8::to_be_bytes`] (and similar for other integer types)
assume
val impl_9__to_be_bytes': bytes: u64 -> t_Array u8 (mk_usize 8)

unfold
let impl_9__to_be_bytes = impl_9__to_be_bytes'

/// See [`std::primitive::u8::to_le_bytes`] (and similar for other integer types)
let impl_9__to_le_bytes (bytes: u64) : t_Array u8 (mk_usize 8) =
  Rust_primitives.Slice.array_from_fn #u8
    (mk_usize 8)
    #(usize -> u8)
    (fun i ->
        let i:usize = i in
        cast (bytes >>! ((mk_u32 8 *! (cast (i <: usize) <: u32) <: u32) %! mk_u32 64 <: u32) <: u64
          )
        <:
        u8)

/// See [`std::primitive::u8::is_power_of_two`] (and similar for other unsigned integer types)
let impl_9__is_power_of_two (x: u64) : bool =
  x <>. mk_u64 0 && (x &. (x -! mk_u64 1 <: u64) <: u64) =. mk_u64 0

/// See [`std::primitive::u8::is_multiple_of`] (and similar for other unsigned integer types)
let impl_9__is_multiple_of (x y: u64) : bool =
  if y =. mk_u64 0 then x =. mk_u64 0 else (x %! y <: u64) =. mk_u64 0

/// See [`std::primitive::u8::wrapping_neg`] (and similar for other integer types)
let impl_9__wrapping_neg (x: u64) : u64 = Rust_primitives.Arithmetic.wrapping_sub_u64 (mk_u64 0) x

/// See [`std::primitive::u8::unchecked_add`] (and similar for other integer types)
let impl_9__unchecked_add (x y: u64)
    : Prims.Pure u64
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_9__MAX <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x +! y

/// See [`std::primitive::u8::unchecked_sub`] (and similar for other integer types)
let impl_9__unchecked_sub (x y: u64) : Prims.Pure u64 (requires x >=. y) (fun _ -> Prims.l_True) =
  x -! y

/// See [`std::primitive::u8::unchecked_mul`] (and similar for other integer types)
let impl_9__unchecked_mul (x y: u64)
    : Prims.Pure u64
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) *
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_9__MAX <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x *! y

/// See [`std::primitive::u8::rem_euclid`] (and similar for other integer types)
let impl_9__rem_euclid (x y: u64) : Prims.Pure u64 (requires y <>. mk_u64 0) (fun _ -> Prims.l_True) =
  Rust_primitives.Arithmetic.rem_euclid_u64 x y

/// See [`std::primitive::u8::pow`] (and similar for other integer types)
let impl_9__pow (x: u64) (exp: u32)
    : Prims.Pure u64
      (requires (impl_9__overflowing_pow x exp <: (u64 & bool))._2 =. false)
      (fun _ -> Prims.l_True) = Rust_primitives.Arithmetic.pow_u64 x exp

[@@ "opaque_to_smt"]

/// See [`std::primitive::u8::rotate_right`] (and similar for other integer types)
let impl_9__rotate_right (x: u64) (n: u32) : u64 =
  let m:u32 = n %! mk_u32 64 in
  if m =. mk_u32 0 then x else (x >>! m <: u64) ^. (x <<! (mk_u32 64 -! m <: u32) <: u64)

[@@ "opaque_to_smt"]

/// See [`std::primitive::u8::rotate_left`] (and similar for other integer types)
let impl_9__rotate_left (x: u64) (n: u32) : u64 =
  let m:u32 = n %! mk_u32 64 in
  if m =. mk_u32 0 then x else (x <<! m <: u64) ^. (x >>! (mk_u32 64 -! m <: u32) <: u64)

/// See [`std::primitive::u8::ilog2`] (and similar for other integer types)
assume
val impl_9__ilog2': x: u64 -> Prims.Pure u32 (requires x >. mk_u64 0) (fun _ -> Prims.l_True)

unfold
let impl_9__ilog2 = impl_9__ilog2'

/// See [`std::primitive::u8::unchecked_div`] (and similar for other integer types)
let impl_9__unchecked_div (x y: u64)
    : Prims.Pure u64 (requires y <>. mk_u64 0) (fun _ -> Prims.l_True) = x /! y

/// See [`std::primitive::u8::unchecked_rem`] (and similar for other integer types)
let impl_9__unchecked_rem (x y: u64)
    : Prims.Pure u64 (requires y <>. mk_u64 0) (fun _ -> Prims.l_True) = x %! y

/// See [`std::primitive::u8::div_ceil`] (and similar for other unsigned integer types)
let impl_9__div_ceil (x y: u64) : Prims.Pure u64 (requires y <>. mk_u64 0) (fun _ -> Prims.l_True) =
  let d:u64 = x /! y in
  let r:u64 = x %! y in
  if r >. mk_u64 0 then d +! mk_u64 1 else d

/// See [`std::primitive::u8::MIN`] (and similar for other unsigned integer types)
let impl_10__MIN: u128 = mk_u128 0

/// See [`std::primitive::u8::MAX`] (and similar for other unsigned integer types)
let impl_10__MAX: u128 = mk_u128 340282366920938463463374607431768211455

/// See [`std::primitive::u8::BITS`] (and similar for other unsigned integer types)
let impl_10__BITS: u32 = mk_u32 128

/// See [`std::primitive::u8::wrapping_add`] (and similar for other unsigned integer types)
let impl_10__wrapping_add (x y: u128) : u128 = Rust_primitives.Arithmetic.wrapping_add_u128 x y

/// See [`std::primitive::u8::saturating_add`] (and similar for other integer types)
let impl_10__saturating_add (x y: u128) : u128 = Rust_primitives.Arithmetic.saturating_add_u128 x y

/// See [`std::primitive::u8::overflowing_add`] (and similar for other integer types)
let impl_10__overflowing_add (x y: u128) : (u128 & bool) =
  Rust_primitives.Arithmetic.overflowing_add_u128 x y

/// See [`std::primitive::u8::wrapping_sub`] (and similar for other integer types)
let impl_10__wrapping_sub (x y: u128) : u128 = Rust_primitives.Arithmetic.wrapping_sub_u128 x y

/// See [`std::primitive::u8::saturating_sub`] (and similar for other integer types)
let impl_10__saturating_sub (x y: u128) : u128 = Rust_primitives.Arithmetic.saturating_sub_u128 x y

/// See [`std::primitive::u8::overflowing_sub`] (and similar for other integer types)
let impl_10__overflowing_sub (x y: u128) : (u128 & bool) =
  Rust_primitives.Arithmetic.overflowing_sub_u128 x y

/// See [`std::primitive::u8::wrapping_mul`] (and similar for other integer types)
let impl_10__wrapping_mul (x y: u128) : u128 = Rust_primitives.Arithmetic.wrapping_mul_u128 x y

/// See [`std::primitive::u8::saturating_mul`] (and similar for other integer types)
let impl_10__saturating_mul (x y: u128) : u128 = Rust_primitives.Arithmetic.saturating_mul_u128 x y

/// See [`std::primitive::u8::overflowing_mul`] (and similar for other integer types)
let impl_10__overflowing_mul (x y: u128) : (u128 & bool) =
  Rust_primitives.Arithmetic.overflowing_mul_u128 x y

/// See [`std::primitive::u8::overflowing_pow`] (and similar for other integer types)
let impl_10__overflowing_pow (x: u128) (exp: u32) : (u128 & bool) =
  Rust_primitives.Arithmetic.overflowing_pow_u128 x exp

/// See [`std::primitive::u8::count_ones`] (and similar for other integer types)
let impl_10__count_ones (x: u128) : u32 =
  let n:u32 = mk_u32 0 in
  let n:u32 =
    Rust_primitives.Hax.Folds.fold_range (mk_u32 0)
      (mk_u32 128)
      (fun n i ->
          let n:u32 = n in
          let i:u32 = i in
          n <=. i <: bool)
      n
      (fun n i ->
          let n:u32 = n in
          let i:u32 = i in
          if ((x >>! i <: u128) &. mk_u128 1 <: u128) =. mk_u128 1 <: bool
          then
            let n:u32 = n +! mk_u32 1 in
            n
          else n)
  in
  n

/// See [`std::primitive::u8::leading_zeros`] (and similar for other integer types)
assume
val impl_10__leading_zeros': x: u128 -> u32

unfold
let impl_10__leading_zeros = impl_10__leading_zeros'

/// See [`std::primitive::u8::from_be_bytes`] (and similar for other integer types)
assume
val impl_10__from_be_bytes': bytes: t_Array u8 (mk_usize 16) -> u128

unfold
let impl_10__from_be_bytes = impl_10__from_be_bytes'

/// See [`std::primitive::u8::from_le_bytes`] (and similar for other integer types)
assume
val impl_10__from_le_bytes': bytes: t_Array u8 (mk_usize 16) -> u128

unfold
let impl_10__from_le_bytes = impl_10__from_le_bytes'

/// See [`std::primitive::u8::to_be_bytes`] (and similar for other integer types)
assume
val impl_10__to_be_bytes': bytes: u128 -> t_Array u8 (mk_usize 16)

unfold
let impl_10__to_be_bytes = impl_10__to_be_bytes'

/// See [`std::primitive::u8::to_le_bytes`] (and similar for other integer types)
let impl_10__to_le_bytes (bytes: u128) : t_Array u8 (mk_usize 16) =
  Rust_primitives.Slice.array_from_fn #u8
    (mk_usize 16)
    #(usize -> u8)
    (fun i ->
        let i:usize = i in
        cast (bytes >>! ((mk_u32 8 *! (cast (i <: usize) <: u32) <: u32) %! mk_u32 128 <: u32)
            <:
            u128)
        <:
        u8)

/// See [`std::primitive::u8::is_power_of_two`] (and similar for other unsigned integer types)
let impl_10__is_power_of_two (x: u128) : bool =
  x <>. mk_u128 0 && (x &. (x -! mk_u128 1 <: u128) <: u128) =. mk_u128 0

/// See [`std::primitive::u8::is_multiple_of`] (and similar for other unsigned integer types)
let impl_10__is_multiple_of (x y: u128) : bool =
  if y =. mk_u128 0 then x =. mk_u128 0 else (x %! y <: u128) =. mk_u128 0

/// See [`std::primitive::u8::wrapping_neg`] (and similar for other integer types)
let impl_10__wrapping_neg (x: u128) : u128 =
  Rust_primitives.Arithmetic.wrapping_sub_u128 (mk_u128 0) x

/// See [`std::primitive::u8::unchecked_add`] (and similar for other integer types)
let impl_10__unchecked_add (x y: u128)
    : Prims.Pure u128
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_10__MAX <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x +! y

/// See [`std::primitive::u8::unchecked_sub`] (and similar for other integer types)
let impl_10__unchecked_sub (x y: u128) : Prims.Pure u128 (requires x >=. y) (fun _ -> Prims.l_True) =
  x -! y

/// See [`std::primitive::u8::unchecked_mul`] (and similar for other integer types)
let impl_10__unchecked_mul (x y: u128)
    : Prims.Pure u128
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) *
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_10__MAX <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x *! y

/// See [`std::primitive::u8::rem_euclid`] (and similar for other integer types)
let impl_10__rem_euclid (x y: u128)
    : Prims.Pure u128 (requires y <>. mk_u128 0) (fun _ -> Prims.l_True) =
  Rust_primitives.Arithmetic.rem_euclid_u128 x y

/// See [`std::primitive::u8::pow`] (and similar for other integer types)
let impl_10__pow (x: u128) (exp: u32)
    : Prims.Pure u128
      (requires (impl_10__overflowing_pow x exp <: (u128 & bool))._2 =. false)
      (fun _ -> Prims.l_True) = Rust_primitives.Arithmetic.pow_u128 x exp

[@@ "opaque_to_smt"]

/// See [`std::primitive::u8::rotate_right`] (and similar for other integer types)
let impl_10__rotate_right (x: u128) (n: u32) : u128 =
  let m:u32 = n %! mk_u32 128 in
  if m =. mk_u32 0 then x else (x >>! m <: u128) ^. (x <<! (mk_u32 128 -! m <: u32) <: u128)

[@@ "opaque_to_smt"]

/// See [`std::primitive::u8::rotate_left`] (and similar for other integer types)
let impl_10__rotate_left (x: u128) (n: u32) : u128 =
  let m:u32 = n %! mk_u32 128 in
  if m =. mk_u32 0 then x else (x <<! m <: u128) ^. (x >>! (mk_u32 128 -! m <: u32) <: u128)

/// See [`std::primitive::u8::ilog2`] (and similar for other integer types)
assume
val impl_10__ilog2': x: u128 -> Prims.Pure u32 (requires x >. mk_u128 0) (fun _ -> Prims.l_True)

unfold
let impl_10__ilog2 = impl_10__ilog2'

/// See [`std::primitive::u8::unchecked_div`] (and similar for other integer types)
let impl_10__unchecked_div (x y: u128)
    : Prims.Pure u128 (requires y <>. mk_u128 0) (fun _ -> Prims.l_True) = x /! y

/// See [`std::primitive::u8::unchecked_rem`] (and similar for other integer types)
let impl_10__unchecked_rem (x y: u128)
    : Prims.Pure u128 (requires y <>. mk_u128 0) (fun _ -> Prims.l_True) = x %! y

/// See [`std::primitive::u8::div_ceil`] (and similar for other unsigned integer types)
let impl_10__div_ceil (x y: u128)
    : Prims.Pure u128 (requires y <>. mk_u128 0) (fun _ -> Prims.l_True) =
  let d:u128 = x /! y in
  let r:u128 = x %! y in
  if r >. mk_u128 0 then d +! mk_u128 1 else d

/// See [`std::primitive::u8::MIN`] (and similar for other unsigned integer types)
let impl_11__MIN: usize = mk_usize 0

/// See [`std::primitive::u8::MAX`] (and similar for other unsigned integer types)
let impl_11__MAX: usize = Rust_primitives.Arithmetic.v_USIZE_MAX

/// See [`std::primitive::u8::BITS`] (and similar for other unsigned integer types)
let impl_11__BITS: u32 = Rust_primitives.Arithmetic.v_SIZE_BITS

/// See [`std::primitive::u8::wrapping_add`] (and similar for other unsigned integer types)
let impl_11__wrapping_add (x y: usize) : usize = Rust_primitives.Arithmetic.wrapping_add_usize x y

/// See [`std::primitive::u8::saturating_add`] (and similar for other integer types)
let impl_11__saturating_add (x y: usize) : usize =
  Rust_primitives.Arithmetic.saturating_add_usize x y

/// See [`std::primitive::u8::overflowing_add`] (and similar for other integer types)
let impl_11__overflowing_add (x y: usize) : (usize & bool) =
  Rust_primitives.Arithmetic.overflowing_add_usize x y

/// See [`std::primitive::u8::wrapping_sub`] (and similar for other integer types)
let impl_11__wrapping_sub (x y: usize) : usize = Rust_primitives.Arithmetic.wrapping_sub_usize x y

/// See [`std::primitive::u8::saturating_sub`] (and similar for other integer types)
let impl_11__saturating_sub (x y: usize) : usize =
  Rust_primitives.Arithmetic.saturating_sub_usize x y

/// See [`std::primitive::u8::overflowing_sub`] (and similar for other integer types)
let impl_11__overflowing_sub (x y: usize) : (usize & bool) =
  Rust_primitives.Arithmetic.overflowing_sub_usize x y

/// See [`std::primitive::u8::wrapping_mul`] (and similar for other integer types)
let impl_11__wrapping_mul (x y: usize) : usize = Rust_primitives.Arithmetic.wrapping_mul_usize x y

/// See [`std::primitive::u8::saturating_mul`] (and similar for other integer types)
let impl_11__saturating_mul (x y: usize) : usize =
  Rust_primitives.Arithmetic.saturating_mul_usize x y

/// See [`std::primitive::u8::overflowing_mul`] (and similar for other integer types)
let impl_11__overflowing_mul (x y: usize) : (usize & bool) =
  Rust_primitives.Arithmetic.overflowing_mul_usize x y

/// See [`std::primitive::u8::overflowing_pow`] (and similar for other integer types)
let impl_11__overflowing_pow (x: usize) (exp: u32) : (usize & bool) =
  Rust_primitives.Arithmetic.overflowing_pow_usize x exp

/// See [`std::primitive::u8::count_ones`] (and similar for other integer types)
let impl_11__count_ones (x: usize) : u32 =
  let n:u32 = mk_u32 0 in
  let n:u32 =
    Rust_primitives.Hax.Folds.fold_range (mk_u32 0)
      Rust_primitives.Arithmetic.v_SIZE_BITS
      (fun n i ->
          let n:u32 = n in
          let i:u32 = i in
          n <=. i <: bool)
      n
      (fun n i ->
          let n:u32 = n in
          let i:u32 = i in
          if ((x >>! i <: usize) &. mk_usize 1 <: usize) =. mk_usize 1 <: bool
          then
            let n:u32 = n +! mk_u32 1 in
            n
          else n)
  in
  n

/// See [`std::primitive::u8::leading_zeros`] (and similar for other integer types)
assume
val impl_11__leading_zeros': x: usize -> u32

unfold
let impl_11__leading_zeros = impl_11__leading_zeros'

/// See [`std::primitive::u8::from_be_bytes`] (and similar for other integer types)
assume
val impl_11__from_be_bytes': bytes: t_Array u8 (mk_usize 8) -> usize

unfold
let impl_11__from_be_bytes = impl_11__from_be_bytes'

/// See [`std::primitive::u8::from_le_bytes`] (and similar for other integer types)
assume
val impl_11__from_le_bytes': bytes: t_Array u8 (mk_usize 8) -> usize

unfold
let impl_11__from_le_bytes = impl_11__from_le_bytes'

/// See [`std::primitive::u8::to_be_bytes`] (and similar for other integer types)
assume
val impl_11__to_be_bytes': bytes: usize -> t_Array u8 (mk_usize 8)

unfold
let impl_11__to_be_bytes = impl_11__to_be_bytes'

/// See [`std::primitive::u8::to_le_bytes`] (and similar for other integer types)
let impl_11__to_le_bytes (bytes: usize) : t_Array u8 (mk_usize 8) =
  Rust_primitives.Slice.array_from_fn #u8
    (mk_usize 8)
    #(usize -> u8)
    (fun i ->
        let i:usize = i in
        cast (bytes >>!
            ((mk_u32 8 *! (cast (i <: usize) <: u32) <: u32) %!
              Rust_primitives.Arithmetic.v_SIZE_BITS
              <:
              u32)
            <:
            usize)
        <:
        u8)

/// See [`std::primitive::u8::is_power_of_two`] (and similar for other unsigned integer types)
let impl_11__is_power_of_two (x: usize) : bool =
  x <>. mk_usize 0 && (x &. (x -! mk_usize 1 <: usize) <: usize) =. mk_usize 0

/// See [`std::primitive::u8::is_multiple_of`] (and similar for other unsigned integer types)
let impl_11__is_multiple_of (x y: usize) : bool =
  if y =. mk_usize 0 then x =. mk_usize 0 else (x %! y <: usize) =. mk_usize 0

/// See [`std::primitive::u8::wrapping_neg`] (and similar for other integer types)
let impl_11__wrapping_neg (x: usize) : usize =
  Rust_primitives.Arithmetic.wrapping_sub_usize (mk_usize 0) x

/// See [`std::primitive::u8::unchecked_add`] (and similar for other integer types)
let impl_11__unchecked_add (x y: usize)
    : Prims.Pure usize
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_11__MAX <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x +! y

/// See [`std::primitive::u8::unchecked_sub`] (and similar for other integer types)
let impl_11__unchecked_sub (x y: usize)
    : Prims.Pure usize (requires x >=. y) (fun _ -> Prims.l_True) = x -! y

/// See [`std::primitive::u8::unchecked_mul`] (and similar for other integer types)
let impl_11__unchecked_mul (x y: usize)
    : Prims.Pure usize
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) *
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_11__MAX <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x *! y

/// See [`std::primitive::u8::rem_euclid`] (and similar for other integer types)
let impl_11__rem_euclid (x y: usize)
    : Prims.Pure usize (requires y <>. mk_usize 0) (fun _ -> Prims.l_True) =
  Rust_primitives.Arithmetic.rem_euclid_usize x y

/// See [`std::primitive::u8::pow`] (and similar for other integer types)
let impl_11__pow (x: usize) (exp: u32)
    : Prims.Pure usize
      (requires (impl_11__overflowing_pow x exp <: (usize & bool))._2 =. false)
      (fun _ -> Prims.l_True) = Rust_primitives.Arithmetic.pow_usize x exp

[@@ "opaque_to_smt"]

/// See [`std::primitive::u8::rotate_right`] (and similar for other integer types)
let impl_11__rotate_right (x: usize) (n: u32) : usize =
  let m:u32 = n %! Rust_primitives.Arithmetic.v_SIZE_BITS in
  if m =. mk_u32 0
  then x
  else (x >>! m <: usize) ^. (x <<! (Rust_primitives.Arithmetic.v_SIZE_BITS -! m <: u32) <: usize)

[@@ "opaque_to_smt"]

/// See [`std::primitive::u8::rotate_left`] (and similar for other integer types)
let impl_11__rotate_left (x: usize) (n: u32) : usize =
  let m:u32 = n %! Rust_primitives.Arithmetic.v_SIZE_BITS in
  if m =. mk_u32 0
  then x
  else (x <<! m <: usize) ^. (x >>! (Rust_primitives.Arithmetic.v_SIZE_BITS -! m <: u32) <: usize)

/// See [`std::primitive::u8::ilog2`] (and similar for other integer types)
assume
val impl_11__ilog2': x: usize -> Prims.Pure u32 (requires x >. mk_usize 0) (fun _ -> Prims.l_True)

unfold
let impl_11__ilog2 = impl_11__ilog2'

/// See [`std::primitive::u8::unchecked_div`] (and similar for other integer types)
let impl_11__unchecked_div (x y: usize)
    : Prims.Pure usize (requires y <>. mk_usize 0) (fun _ -> Prims.l_True) = x /! y

/// See [`std::primitive::u8::unchecked_rem`] (and similar for other integer types)
let impl_11__unchecked_rem (x y: usize)
    : Prims.Pure usize (requires y <>. mk_usize 0) (fun _ -> Prims.l_True) = x %! y

/// See [`std::primitive::u8::div_ceil`] (and similar for other unsigned integer types)
let impl_11__div_ceil (x y: usize)
    : Prims.Pure usize (requires y <>. mk_usize 0) (fun _ -> Prims.l_True) =
  let d:usize = x /! y in
  let r:usize = x %! y in
  if r >. mk_usize 0 then d +! mk_usize 1 else d

/// See [`std::primitive::i8::MIN`] (and similar for other signed integer types)
let impl_12__MIN: i8 = mk_i8 (-128)

/// See [`std::primitive::i8::MAX`] (and similar for other signed integer types)
let impl_12__MAX: i8 = mk_i8 127

/// See [`std::primitive::i8::BITS`] (and similar for other signed integer types)
let impl_12__BITS: u32 = mk_u32 8

let impl_12__wrapping_add (x y: i8) : i8 = Rust_primitives.Arithmetic.wrapping_add_i8 x y

/// See [`std::primitive::u8::saturating_add`] (and similar for other integer types)
let impl_12__saturating_add (x y: i8) : i8 = Rust_primitives.Arithmetic.saturating_add_i8 x y

/// See [`std::primitive::u8::overflowing_add`] (and similar for other integer types)
let impl_12__overflowing_add (x y: i8) : (i8 & bool) =
  Rust_primitives.Arithmetic.overflowing_add_i8 x y

/// See [`std::primitive::u8::wrapping_sub`] (and similar for other integer types)
let impl_12__wrapping_sub (x y: i8) : i8 = Rust_primitives.Arithmetic.wrapping_sub_i8 x y

/// See [`std::primitive::u8::saturating_sub`] (and similar for other integer types)
let impl_12__saturating_sub (x y: i8) : i8 = Rust_primitives.Arithmetic.saturating_sub_i8 x y

/// See [`std::primitive::u8::overflowing_sub`] (and similar for other integer types)
let impl_12__overflowing_sub (x y: i8) : (i8 & bool) =
  Rust_primitives.Arithmetic.overflowing_sub_i8 x y

/// See [`std::primitive::u8::wrapping_mul`] (and similar for other integer types)
let impl_12__wrapping_mul (x y: i8) : i8 = Rust_primitives.Arithmetic.wrapping_mul_i8 x y

/// See [`std::primitive::u8::saturating_mul`] (and similar for other integer types)
let impl_12__saturating_mul (x y: i8) : i8 = Rust_primitives.Arithmetic.saturating_mul_i8 x y

/// See [`std::primitive::u8::overflowing_mul`] (and similar for other integer types)
let impl_12__overflowing_mul (x y: i8) : (i8 & bool) =
  Rust_primitives.Arithmetic.overflowing_mul_i8 x y

/// See [`std::primitive::u8::overflowing_pow`] (and similar for other integer types)
let impl_12__overflowing_pow (x: i8) (exp: u32) : (i8 & bool) =
  Rust_primitives.Arithmetic.overflowing_pow_i8 x exp

/// See [`std::primitive::u8::count_ones`] (and similar for other integer types)
let impl_12__count_ones (x: i8) : u32 =
  let n:u32 = mk_u32 0 in
  let n:u32 =
    Rust_primitives.Hax.Folds.fold_range (mk_u32 0)
      (mk_u32 8)
      (fun n i ->
          let n:u32 = n in
          let i:u32 = i in
          n <=. i <: bool)
      n
      (fun n i ->
          let n:u32 = n in
          let i:u32 = i in
          if ((x >>! i <: i8) &. mk_i8 1 <: i8) =. mk_i8 1 <: bool
          then
            let n:u32 = n +! mk_u32 1 in
            n
          else n)
  in
  n

/// See [`std::primitive::i8::unsigned_abs`] (and similar for other signed integer types)
let impl_12__unsigned_abs (x: i8) : u8 =
  if x <. mk_i8 0
  then (cast (Rust_primitives.Arithmetic.neg (x +! mk_i8 1 <: i8) <: i8) <: u8) +! mk_u8 1
  else cast (x <: i8) <: u8

/// See [`std::primitive::u8::rotate_right`] (and similar for other integer types)
assume
val impl_12__rotate_right': x: i8 -> n: u32 -> i8

unfold
let impl_12__rotate_right = impl_12__rotate_right'

/// See [`std::primitive::u8::rotate_left`] (and similar for other integer types)
assume
val impl_12__rotate_left': x: i8 -> n: u32 -> i8

unfold
let impl_12__rotate_left = impl_12__rotate_left'

/// See [`std::primitive::u8::leading_zeros`] (and similar for other integer types)
assume
val impl_12__leading_zeros': x: i8 -> u32

unfold
let impl_12__leading_zeros = impl_12__leading_zeros'

/// See [`std::primitive::u8::from_be_bytes`] (and similar for other integer types)
assume
val impl_12__from_be_bytes': bytes: t_Array u8 (mk_usize 1) -> i8

unfold
let impl_12__from_be_bytes = impl_12__from_be_bytes'

/// See [`std::primitive::u8::from_le_bytes`] (and similar for other integer types)
assume
val impl_12__from_le_bytes': bytes: t_Array u8 (mk_usize 1) -> i8

unfold
let impl_12__from_le_bytes = impl_12__from_le_bytes'

/// See [`std::primitive::u8::to_be_bytes`] (and similar for other integer types)
assume
val impl_12__to_be_bytes': bytes: i8 -> t_Array u8 (mk_usize 1)

unfold
let impl_12__to_be_bytes = impl_12__to_be_bytes'

/// See [`std::primitive::u8::to_le_bytes`] (and similar for other integer types)
let impl_12__to_le_bytes (bytes: i8) : t_Array u8 (mk_usize 1) =
  Rust_primitives.Slice.array_from_fn #u8
    (mk_usize 1)
    #(usize -> u8)
    (fun i ->
        let i:usize = i in
        cast (bytes >>! ((mk_u32 8 *! (cast (i <: usize) <: u32) <: u32) %! mk_u32 8 <: u32) <: i8)
        <:
        u8)

/// See [`std::primitive::i8::signum`] (and similar for other signed integer types)
let impl_12__signum (x: i8) : i8 =
  if x >. mk_i8 0 then mk_i8 1 else if x =. mk_i8 0 then mk_i8 0 else mk_i8 (-1)

/// See [`std::primitive::i8::wrapping_neg`] (and similar for other signed integer types)
let impl_12__wrapping_neg (x: i8) : i8 = Rust_primitives.Arithmetic.wrapping_sub_i8 (mk_i8 0) x

/// See [`std::primitive::u8::unchecked_add`] (and similar for other integer types)
let impl_12__unchecked_add (x y: i8)
    : Prims.Pure i8
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_12__MAX <: Hax_lib.Int.t_Int) &&
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) >=
        (Rust_primitives.Hax.Int.from_machine impl_12__MIN <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x +! y

/// See [`std::primitive::u8::unchecked_sub`] (and similar for other integer types)
let impl_12__unchecked_sub (x y: i8)
    : Prims.Pure i8
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) -
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_12__MAX <: Hax_lib.Int.t_Int) &&
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) -
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) >=
        (Rust_primitives.Hax.Int.from_machine impl_12__MIN <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x -! y

/// See [`std::primitive::u8::unchecked_mul`] (and similar for other integer types)
let impl_12__unchecked_mul (x y: i8)
    : Prims.Pure i8
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) *
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_12__MAX <: Hax_lib.Int.t_Int) &&
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) *
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) >=
        (Rust_primitives.Hax.Int.from_machine impl_12__MIN <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x *! y

/// See [`std::primitive::u8::rem_euclid`] (and similar for other integer types)
let impl_12__rem_euclid (x y: i8)
    : Prims.Pure i8
      (requires y <>. mk_i8 0 && ~.((x =. impl_12__MIN <: bool) && (y =. mk_i8 (-1) <: bool)))
      (fun _ -> Prims.l_True) = Rust_primitives.Arithmetic.rem_euclid_i8 x y

/// See [`std::primitive::u8::pow`] (and similar for other integer types)
let impl_12__pow (x: i8) (exp: u32)
    : Prims.Pure i8
      (requires (impl_12__overflowing_pow x exp <: (i8 & bool))._2 =. false)
      (fun _ -> Prims.l_True) = Rust_primitives.Arithmetic.pow_i8 x exp

/// See [`std::primitive::i8::abs`] (and similar for other signed integer types)
let impl_12__abs (x: i8) : Prims.Pure i8 (requires x >. impl_12__MIN) (fun _ -> Prims.l_True) =
  if x <. mk_i8 0 then Rust_primitives.Arithmetic.neg x else x

/// See [`std::primitive::u8::ilog2`] (and similar for other integer types)
assume
val impl_12__ilog2': x: i8 -> Prims.Pure u32 (requires x >. mk_i8 0) (fun _ -> Prims.l_True)

unfold
let impl_12__ilog2 = impl_12__ilog2'

/// See [`std::primitive::u8::unchecked_div`] (and similar for other integer types)
let impl_12__unchecked_div (x y: i8)
    : Prims.Pure i8
      (requires y <>. mk_i8 0 && (x <>. impl_12__MIN || y <>. mk_i8 (-1)))
      (fun _ -> Prims.l_True) = x /! y

/// See [`std::primitive::u8::unchecked_rem`] (and similar for other integer types)
let impl_12__unchecked_rem (x y: i8)
    : Prims.Pure i8
      (requires y <>. mk_i8 0 && (x <>. impl_12__MIN || y <>. mk_i8 (-1)))
      (fun _ -> Prims.l_True) = x %! y

/// See [`std::primitive::i8::div_ceil`] (and similar for other signed integer types)
let impl_12__div_ceil (x y: i8)
    : Prims.Pure i8
      (requires y <>. mk_i8 0 && ~.((x =. impl_12__MIN <: bool) && (y =. mk_i8 (-1) <: bool)))
      (fun _ -> Prims.l_True) =
  let d:i8 = x /! y in
  let r:i8 = x %! y in
  if r >. mk_i8 0 && y >. mk_i8 0 || r <. mk_i8 0 && y <. mk_i8 0 then d +! mk_i8 1 else d

/// See [`std::primitive::i8::MIN`] (and similar for other signed integer types)
let impl_13__MIN: i16 = mk_i16 (-32768)

/// See [`std::primitive::i8::MAX`] (and similar for other signed integer types)
let impl_13__MAX: i16 = mk_i16 32767

/// See [`std::primitive::i8::BITS`] (and similar for other signed integer types)
let impl_13__BITS: u32 = mk_u32 16

let impl_13__wrapping_add (x y: i16) : i16 = Rust_primitives.Arithmetic.wrapping_add_i16 x y

/// See [`std::primitive::u8::saturating_add`] (and similar for other integer types)
let impl_13__saturating_add (x y: i16) : i16 = Rust_primitives.Arithmetic.saturating_add_i16 x y

/// See [`std::primitive::u8::overflowing_add`] (and similar for other integer types)
let impl_13__overflowing_add (x y: i16) : (i16 & bool) =
  Rust_primitives.Arithmetic.overflowing_add_i16 x y

/// See [`std::primitive::u8::wrapping_sub`] (and similar for other integer types)
let impl_13__wrapping_sub (x y: i16) : i16 = Rust_primitives.Arithmetic.wrapping_sub_i16 x y

/// See [`std::primitive::u8::saturating_sub`] (and similar for other integer types)
let impl_13__saturating_sub (x y: i16) : i16 = Rust_primitives.Arithmetic.saturating_sub_i16 x y

/// See [`std::primitive::u8::overflowing_sub`] (and similar for other integer types)
let impl_13__overflowing_sub (x y: i16) : (i16 & bool) =
  Rust_primitives.Arithmetic.overflowing_sub_i16 x y

/// See [`std::primitive::u8::wrapping_mul`] (and similar for other integer types)
let impl_13__wrapping_mul (x y: i16) : i16 = Rust_primitives.Arithmetic.wrapping_mul_i16 x y

/// See [`std::primitive::u8::saturating_mul`] (and similar for other integer types)
let impl_13__saturating_mul (x y: i16) : i16 = Rust_primitives.Arithmetic.saturating_mul_i16 x y

/// See [`std::primitive::u8::overflowing_mul`] (and similar for other integer types)
let impl_13__overflowing_mul (x y: i16) : (i16 & bool) =
  Rust_primitives.Arithmetic.overflowing_mul_i16 x y

/// See [`std::primitive::u8::overflowing_pow`] (and similar for other integer types)
let impl_13__overflowing_pow (x: i16) (exp: u32) : (i16 & bool) =
  Rust_primitives.Arithmetic.overflowing_pow_i16 x exp

/// See [`std::primitive::u8::count_ones`] (and similar for other integer types)
let impl_13__count_ones (x: i16) : u32 =
  let n:u32 = mk_u32 0 in
  let n:u32 =
    Rust_primitives.Hax.Folds.fold_range (mk_u32 0)
      (mk_u32 16)
      (fun n i ->
          let n:u32 = n in
          let i:u32 = i in
          n <=. i <: bool)
      n
      (fun n i ->
          let n:u32 = n in
          let i:u32 = i in
          if ((x >>! i <: i16) &. mk_i16 1 <: i16) =. mk_i16 1 <: bool
          then
            let n:u32 = n +! mk_u32 1 in
            n
          else n)
  in
  n

/// See [`std::primitive::i8::unsigned_abs`] (and similar for other signed integer types)
let impl_13__unsigned_abs (x: i16) : u16 =
  if x <. mk_i16 0
  then (cast (Rust_primitives.Arithmetic.neg (x +! mk_i16 1 <: i16) <: i16) <: u16) +! mk_u16 1
  else cast (x <: i16) <: u16

/// See [`std::primitive::u8::rotate_right`] (and similar for other integer types)
assume
val impl_13__rotate_right': x: i16 -> n: u32 -> i16

unfold
let impl_13__rotate_right = impl_13__rotate_right'

/// See [`std::primitive::u8::rotate_left`] (and similar for other integer types)
assume
val impl_13__rotate_left': x: i16 -> n: u32 -> i16

unfold
let impl_13__rotate_left = impl_13__rotate_left'

/// See [`std::primitive::u8::leading_zeros`] (and similar for other integer types)
assume
val impl_13__leading_zeros': x: i16 -> u32

unfold
let impl_13__leading_zeros = impl_13__leading_zeros'

/// See [`std::primitive::u8::from_be_bytes`] (and similar for other integer types)
assume
val impl_13__from_be_bytes': bytes: t_Array u8 (mk_usize 2) -> i16

unfold
let impl_13__from_be_bytes = impl_13__from_be_bytes'

/// See [`std::primitive::u8::from_le_bytes`] (and similar for other integer types)
assume
val impl_13__from_le_bytes': bytes: t_Array u8 (mk_usize 2) -> i16

unfold
let impl_13__from_le_bytes = impl_13__from_le_bytes'

/// See [`std::primitive::u8::to_be_bytes`] (and similar for other integer types)
assume
val impl_13__to_be_bytes': bytes: i16 -> t_Array u8 (mk_usize 2)

unfold
let impl_13__to_be_bytes = impl_13__to_be_bytes'

/// See [`std::primitive::u8::to_le_bytes`] (and similar for other integer types)
let impl_13__to_le_bytes (bytes: i16) : t_Array u8 (mk_usize 2) =
  Rust_primitives.Slice.array_from_fn #u8
    (mk_usize 2)
    #(usize -> u8)
    (fun i ->
        let i:usize = i in
        cast (bytes >>! ((mk_u32 8 *! (cast (i <: usize) <: u32) <: u32) %! mk_u32 16 <: u32) <: i16
          )
        <:
        u8)

/// See [`std::primitive::i8::signum`] (and similar for other signed integer types)
let impl_13__signum (x: i16) : i16 =
  if x >. mk_i16 0 then mk_i16 1 else if x =. mk_i16 0 then mk_i16 0 else mk_i16 (-1)

/// See [`std::primitive::i8::wrapping_neg`] (and similar for other signed integer types)
let impl_13__wrapping_neg (x: i16) : i16 = Rust_primitives.Arithmetic.wrapping_sub_i16 (mk_i16 0) x

/// See [`std::primitive::u8::unchecked_add`] (and similar for other integer types)
let impl_13__unchecked_add (x y: i16)
    : Prims.Pure i16
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_13__MAX <: Hax_lib.Int.t_Int) &&
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) >=
        (Rust_primitives.Hax.Int.from_machine impl_13__MIN <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x +! y

/// See [`std::primitive::u8::unchecked_sub`] (and similar for other integer types)
let impl_13__unchecked_sub (x y: i16)
    : Prims.Pure i16
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) -
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_13__MAX <: Hax_lib.Int.t_Int) &&
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) -
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) >=
        (Rust_primitives.Hax.Int.from_machine impl_13__MIN <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x -! y

/// See [`std::primitive::u8::unchecked_mul`] (and similar for other integer types)
let impl_13__unchecked_mul (x y: i16)
    : Prims.Pure i16
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) *
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_13__MAX <: Hax_lib.Int.t_Int) &&
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) *
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) >=
        (Rust_primitives.Hax.Int.from_machine impl_13__MIN <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x *! y

/// See [`std::primitive::u8::rem_euclid`] (and similar for other integer types)
let impl_13__rem_euclid (x y: i16)
    : Prims.Pure i16
      (requires y <>. mk_i16 0 && ~.((x =. impl_13__MIN <: bool) && (y =. mk_i16 (-1) <: bool)))
      (fun _ -> Prims.l_True) = Rust_primitives.Arithmetic.rem_euclid_i16 x y

/// See [`std::primitive::u8::pow`] (and similar for other integer types)
let impl_13__pow (x: i16) (exp: u32)
    : Prims.Pure i16
      (requires (impl_13__overflowing_pow x exp <: (i16 & bool))._2 =. false)
      (fun _ -> Prims.l_True) = Rust_primitives.Arithmetic.pow_i16 x exp

/// See [`std::primitive::i8::abs`] (and similar for other signed integer types)
let impl_13__abs (x: i16) : Prims.Pure i16 (requires x >. impl_13__MIN) (fun _ -> Prims.l_True) =
  if x <. mk_i16 0 then Rust_primitives.Arithmetic.neg x else x

/// See [`std::primitive::u8::ilog2`] (and similar for other integer types)
assume
val impl_13__ilog2': x: i16 -> Prims.Pure u32 (requires x >. mk_i16 0) (fun _ -> Prims.l_True)

unfold
let impl_13__ilog2 = impl_13__ilog2'

/// See [`std::primitive::u8::unchecked_div`] (and similar for other integer types)
let impl_13__unchecked_div (x y: i16)
    : Prims.Pure i16
      (requires y <>. mk_i16 0 && (x <>. impl_13__MIN || y <>. mk_i16 (-1)))
      (fun _ -> Prims.l_True) = x /! y

/// See [`std::primitive::u8::unchecked_rem`] (and similar for other integer types)
let impl_13__unchecked_rem (x y: i16)
    : Prims.Pure i16
      (requires y <>. mk_i16 0 && (x <>. impl_13__MIN || y <>. mk_i16 (-1)))
      (fun _ -> Prims.l_True) = x %! y

/// See [`std::primitive::i8::div_ceil`] (and similar for other signed integer types)
let impl_13__div_ceil (x y: i16)
    : Prims.Pure i16
      (requires y <>. mk_i16 0 && ~.((x =. impl_13__MIN <: bool) && (y =. mk_i16 (-1) <: bool)))
      (fun _ -> Prims.l_True) =
  let d:i16 = x /! y in
  let r:i16 = x %! y in
  if r >. mk_i16 0 && y >. mk_i16 0 || r <. mk_i16 0 && y <. mk_i16 0 then d +! mk_i16 1 else d

/// See [`std::primitive::i8::MIN`] (and similar for other signed integer types)
let impl_14__MIN: i32 = mk_i32 (-2147483648)

/// See [`std::primitive::i8::MAX`] (and similar for other signed integer types)
let impl_14__MAX: i32 = mk_i32 2147483647

/// See [`std::primitive::i8::BITS`] (and similar for other signed integer types)
let impl_14__BITS: u32 = mk_u32 32

let impl_14__wrapping_add (x y: i32) : i32 = Rust_primitives.Arithmetic.wrapping_add_i32 x y

/// See [`std::primitive::u8::saturating_add`] (and similar for other integer types)
let impl_14__saturating_add (x y: i32) : i32 = Rust_primitives.Arithmetic.saturating_add_i32 x y

/// See [`std::primitive::u8::overflowing_add`] (and similar for other integer types)
let impl_14__overflowing_add (x y: i32) : (i32 & bool) =
  Rust_primitives.Arithmetic.overflowing_add_i32 x y

/// See [`std::primitive::u8::wrapping_sub`] (and similar for other integer types)
let impl_14__wrapping_sub (x y: i32) : i32 = Rust_primitives.Arithmetic.wrapping_sub_i32 x y

/// See [`std::primitive::u8::saturating_sub`] (and similar for other integer types)
let impl_14__saturating_sub (x y: i32) : i32 = Rust_primitives.Arithmetic.saturating_sub_i32 x y

/// See [`std::primitive::u8::overflowing_sub`] (and similar for other integer types)
let impl_14__overflowing_sub (x y: i32) : (i32 & bool) =
  Rust_primitives.Arithmetic.overflowing_sub_i32 x y

/// See [`std::primitive::u8::wrapping_mul`] (and similar for other integer types)
let impl_14__wrapping_mul (x y: i32) : i32 = Rust_primitives.Arithmetic.wrapping_mul_i32 x y

/// See [`std::primitive::u8::saturating_mul`] (and similar for other integer types)
let impl_14__saturating_mul (x y: i32) : i32 = Rust_primitives.Arithmetic.saturating_mul_i32 x y

/// See [`std::primitive::u8::overflowing_mul`] (and similar for other integer types)
let impl_14__overflowing_mul (x y: i32) : (i32 & bool) =
  Rust_primitives.Arithmetic.overflowing_mul_i32 x y

/// See [`std::primitive::u8::overflowing_pow`] (and similar for other integer types)
let impl_14__overflowing_pow (x: i32) (exp: u32) : (i32 & bool) =
  Rust_primitives.Arithmetic.overflowing_pow_i32 x exp

/// See [`std::primitive::u8::count_ones`] (and similar for other integer types)
let impl_14__count_ones (x: i32) : u32 =
  let n:u32 = mk_u32 0 in
  let n:u32 =
    Rust_primitives.Hax.Folds.fold_range (mk_u32 0)
      (mk_u32 32)
      (fun n i ->
          let n:u32 = n in
          let i:u32 = i in
          n <=. i <: bool)
      n
      (fun n i ->
          let n:u32 = n in
          let i:u32 = i in
          if ((x >>! i <: i32) &. mk_i32 1 <: i32) =. mk_i32 1 <: bool
          then
            let n:u32 = n +! mk_u32 1 in
            n
          else n)
  in
  n

/// See [`std::primitive::i8::unsigned_abs`] (and similar for other signed integer types)
let impl_14__unsigned_abs (x: i32) : u32 =
  if x <. mk_i32 0
  then (cast (Rust_primitives.Arithmetic.neg (x +! mk_i32 1 <: i32) <: i32) <: u32) +! mk_u32 1
  else cast (x <: i32) <: u32

/// See [`std::primitive::u8::rotate_right`] (and similar for other integer types)
assume
val impl_14__rotate_right': x: i32 -> n: u32 -> i32

unfold
let impl_14__rotate_right = impl_14__rotate_right'

/// See [`std::primitive::u8::rotate_left`] (and similar for other integer types)
assume
val impl_14__rotate_left': x: i32 -> n: u32 -> i32

unfold
let impl_14__rotate_left = impl_14__rotate_left'

/// See [`std::primitive::u8::leading_zeros`] (and similar for other integer types)
assume
val impl_14__leading_zeros': x: i32 -> u32

unfold
let impl_14__leading_zeros = impl_14__leading_zeros'

/// See [`std::primitive::u8::from_be_bytes`] (and similar for other integer types)
assume
val impl_14__from_be_bytes': bytes: t_Array u8 (mk_usize 4) -> i32

unfold
let impl_14__from_be_bytes = impl_14__from_be_bytes'

/// See [`std::primitive::u8::from_le_bytes`] (and similar for other integer types)
assume
val impl_14__from_le_bytes': bytes: t_Array u8 (mk_usize 4) -> i32

unfold
let impl_14__from_le_bytes = impl_14__from_le_bytes'

/// See [`std::primitive::u8::to_be_bytes`] (and similar for other integer types)
assume
val impl_14__to_be_bytes': bytes: i32 -> t_Array u8 (mk_usize 4)

unfold
let impl_14__to_be_bytes = impl_14__to_be_bytes'

/// See [`std::primitive::u8::to_le_bytes`] (and similar for other integer types)
let impl_14__to_le_bytes (bytes: i32) : t_Array u8 (mk_usize 4) =
  Rust_primitives.Slice.array_from_fn #u8
    (mk_usize 4)
    #(usize -> u8)
    (fun i ->
        let i:usize = i in
        cast (bytes >>! ((mk_u32 8 *! (cast (i <: usize) <: u32) <: u32) %! mk_u32 32 <: u32) <: i32
          )
        <:
        u8)

/// See [`std::primitive::i8::signum`] (and similar for other signed integer types)
let impl_14__signum (x: i32) : i32 =
  if x >. mk_i32 0 then mk_i32 1 else if x =. mk_i32 0 then mk_i32 0 else mk_i32 (-1)

/// See [`std::primitive::i8::wrapping_neg`] (and similar for other signed integer types)
let impl_14__wrapping_neg (x: i32) : i32 = Rust_primitives.Arithmetic.wrapping_sub_i32 (mk_i32 0) x

/// See [`std::primitive::u8::unchecked_add`] (and similar for other integer types)
let impl_14__unchecked_add (x y: i32)
    : Prims.Pure i32
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_14__MAX <: Hax_lib.Int.t_Int) &&
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) >=
        (Rust_primitives.Hax.Int.from_machine impl_14__MIN <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x +! y

/// See [`std::primitive::u8::unchecked_sub`] (and similar for other integer types)
let impl_14__unchecked_sub (x y: i32)
    : Prims.Pure i32
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) -
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_14__MAX <: Hax_lib.Int.t_Int) &&
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) -
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) >=
        (Rust_primitives.Hax.Int.from_machine impl_14__MIN <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x -! y

/// See [`std::primitive::u8::unchecked_mul`] (and similar for other integer types)
let impl_14__unchecked_mul (x y: i32)
    : Prims.Pure i32
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) *
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_14__MAX <: Hax_lib.Int.t_Int) &&
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) *
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) >=
        (Rust_primitives.Hax.Int.from_machine impl_14__MIN <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x *! y

/// See [`std::primitive::u8::rem_euclid`] (and similar for other integer types)
let impl_14__rem_euclid (x y: i32)
    : Prims.Pure i32
      (requires y <>. mk_i32 0 && ~.((x =. impl_14__MIN <: bool) && (y =. mk_i32 (-1) <: bool)))
      (fun _ -> Prims.l_True) = Rust_primitives.Arithmetic.rem_euclid_i32 x y

/// See [`std::primitive::u8::pow`] (and similar for other integer types)
let impl_14__pow (x: i32) (exp: u32)
    : Prims.Pure i32
      (requires (impl_14__overflowing_pow x exp <: (i32 & bool))._2 =. false)
      (fun _ -> Prims.l_True) = Rust_primitives.Arithmetic.pow_i32 x exp

/// See [`std::primitive::i8::abs`] (and similar for other signed integer types)
let impl_14__abs (x: i32) : Prims.Pure i32 (requires x >. impl_14__MIN) (fun _ -> Prims.l_True) =
  if x <. mk_i32 0 then Rust_primitives.Arithmetic.neg x else x

/// See [`std::primitive::u8::ilog2`] (and similar for other integer types)
assume
val impl_14__ilog2': x: i32 -> Prims.Pure u32 (requires x >. mk_i32 0) (fun _ -> Prims.l_True)

unfold
let impl_14__ilog2 = impl_14__ilog2'

/// See [`std::primitive::u8::unchecked_div`] (and similar for other integer types)
let impl_14__unchecked_div (x y: i32)
    : Prims.Pure i32
      (requires y <>. mk_i32 0 && (x <>. impl_14__MIN || y <>. mk_i32 (-1)))
      (fun _ -> Prims.l_True) = x /! y

/// See [`std::primitive::u8::unchecked_rem`] (and similar for other integer types)
let impl_14__unchecked_rem (x y: i32)
    : Prims.Pure i32
      (requires y <>. mk_i32 0 && (x <>. impl_14__MIN || y <>. mk_i32 (-1)))
      (fun _ -> Prims.l_True) = x %! y

/// See [`std::primitive::i8::div_ceil`] (and similar for other signed integer types)
let impl_14__div_ceil (x y: i32)
    : Prims.Pure i32
      (requires y <>. mk_i32 0 && ~.((x =. impl_14__MIN <: bool) && (y =. mk_i32 (-1) <: bool)))
      (fun _ -> Prims.l_True) =
  let d:i32 = x /! y in
  let r:i32 = x %! y in
  if r >. mk_i32 0 && y >. mk_i32 0 || r <. mk_i32 0 && y <. mk_i32 0 then d +! mk_i32 1 else d

/// See [`std::primitive::i8::MIN`] (and similar for other signed integer types)
let impl_15__MIN: i64 = mk_i64 (-9223372036854775808)

/// See [`std::primitive::i8::MAX`] (and similar for other signed integer types)
let impl_15__MAX: i64 = mk_i64 9223372036854775807

/// See [`std::primitive::i8::BITS`] (and similar for other signed integer types)
let impl_15__BITS: u32 = mk_u32 64

let impl_15__wrapping_add (x y: i64) : i64 = Rust_primitives.Arithmetic.wrapping_add_i64 x y

/// See [`std::primitive::u8::saturating_add`] (and similar for other integer types)
let impl_15__saturating_add (x y: i64) : i64 = Rust_primitives.Arithmetic.saturating_add_i64 x y

/// See [`std::primitive::u8::overflowing_add`] (and similar for other integer types)
let impl_15__overflowing_add (x y: i64) : (i64 & bool) =
  Rust_primitives.Arithmetic.overflowing_add_i64 x y

/// See [`std::primitive::u8::wrapping_sub`] (and similar for other integer types)
let impl_15__wrapping_sub (x y: i64) : i64 = Rust_primitives.Arithmetic.wrapping_sub_i64 x y

/// See [`std::primitive::u8::saturating_sub`] (and similar for other integer types)
let impl_15__saturating_sub (x y: i64) : i64 = Rust_primitives.Arithmetic.saturating_sub_i64 x y

/// See [`std::primitive::u8::overflowing_sub`] (and similar for other integer types)
let impl_15__overflowing_sub (x y: i64) : (i64 & bool) =
  Rust_primitives.Arithmetic.overflowing_sub_i64 x y

/// See [`std::primitive::u8::wrapping_mul`] (and similar for other integer types)
let impl_15__wrapping_mul (x y: i64) : i64 = Rust_primitives.Arithmetic.wrapping_mul_i64 x y

/// See [`std::primitive::u8::saturating_mul`] (and similar for other integer types)
let impl_15__saturating_mul (x y: i64) : i64 = Rust_primitives.Arithmetic.saturating_mul_i64 x y

/// See [`std::primitive::u8::overflowing_mul`] (and similar for other integer types)
let impl_15__overflowing_mul (x y: i64) : (i64 & bool) =
  Rust_primitives.Arithmetic.overflowing_mul_i64 x y

/// See [`std::primitive::u8::overflowing_pow`] (and similar for other integer types)
let impl_15__overflowing_pow (x: i64) (exp: u32) : (i64 & bool) =
  Rust_primitives.Arithmetic.overflowing_pow_i64 x exp

/// See [`std::primitive::u8::count_ones`] (and similar for other integer types)
let impl_15__count_ones (x: i64) : u32 =
  let n:u32 = mk_u32 0 in
  let n:u32 =
    Rust_primitives.Hax.Folds.fold_range (mk_u32 0)
      (mk_u32 64)
      (fun n i ->
          let n:u32 = n in
          let i:u32 = i in
          n <=. i <: bool)
      n
      (fun n i ->
          let n:u32 = n in
          let i:u32 = i in
          if ((x >>! i <: i64) &. mk_i64 1 <: i64) =. mk_i64 1 <: bool
          then
            let n:u32 = n +! mk_u32 1 in
            n
          else n)
  in
  n

/// See [`std::primitive::i8::unsigned_abs`] (and similar for other signed integer types)
let impl_15__unsigned_abs (x: i64) : u64 =
  if x <. mk_i64 0
  then (cast (Rust_primitives.Arithmetic.neg (x +! mk_i64 1 <: i64) <: i64) <: u64) +! mk_u64 1
  else cast (x <: i64) <: u64

/// See [`std::primitive::u8::rotate_right`] (and similar for other integer types)
assume
val impl_15__rotate_right': x: i64 -> n: u32 -> i64

unfold
let impl_15__rotate_right = impl_15__rotate_right'

/// See [`std::primitive::u8::rotate_left`] (and similar for other integer types)
assume
val impl_15__rotate_left': x: i64 -> n: u32 -> i64

unfold
let impl_15__rotate_left = impl_15__rotate_left'

/// See [`std::primitive::u8::leading_zeros`] (and similar for other integer types)
assume
val impl_15__leading_zeros': x: i64 -> u32

unfold
let impl_15__leading_zeros = impl_15__leading_zeros'

/// See [`std::primitive::u8::from_be_bytes`] (and similar for other integer types)
assume
val impl_15__from_be_bytes': bytes: t_Array u8 (mk_usize 8) -> i64

unfold
let impl_15__from_be_bytes = impl_15__from_be_bytes'

/// See [`std::primitive::u8::from_le_bytes`] (and similar for other integer types)
assume
val impl_15__from_le_bytes': bytes: t_Array u8 (mk_usize 8) -> i64

unfold
let impl_15__from_le_bytes = impl_15__from_le_bytes'

/// See [`std::primitive::u8::to_be_bytes`] (and similar for other integer types)
assume
val impl_15__to_be_bytes': bytes: i64 -> t_Array u8 (mk_usize 8)

unfold
let impl_15__to_be_bytes = impl_15__to_be_bytes'

/// See [`std::primitive::u8::to_le_bytes`] (and similar for other integer types)
let impl_15__to_le_bytes (bytes: i64) : t_Array u8 (mk_usize 8) =
  Rust_primitives.Slice.array_from_fn #u8
    (mk_usize 8)
    #(usize -> u8)
    (fun i ->
        let i:usize = i in
        cast (bytes >>! ((mk_u32 8 *! (cast (i <: usize) <: u32) <: u32) %! mk_u32 64 <: u32) <: i64
          )
        <:
        u8)

/// See [`std::primitive::i8::signum`] (and similar for other signed integer types)
let impl_15__signum (x: i64) : i64 =
  if x >. mk_i64 0 then mk_i64 1 else if x =. mk_i64 0 then mk_i64 0 else mk_i64 (-1)

/// See [`std::primitive::i8::wrapping_neg`] (and similar for other signed integer types)
let impl_15__wrapping_neg (x: i64) : i64 = Rust_primitives.Arithmetic.wrapping_sub_i64 (mk_i64 0) x

/// See [`std::primitive::u8::unchecked_add`] (and similar for other integer types)
let impl_15__unchecked_add (x y: i64)
    : Prims.Pure i64
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_15__MAX <: Hax_lib.Int.t_Int) &&
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) >=
        (Rust_primitives.Hax.Int.from_machine impl_15__MIN <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x +! y

/// See [`std::primitive::u8::unchecked_sub`] (and similar for other integer types)
let impl_15__unchecked_sub (x y: i64)
    : Prims.Pure i64
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) -
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_15__MAX <: Hax_lib.Int.t_Int) &&
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) -
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) >=
        (Rust_primitives.Hax.Int.from_machine impl_15__MIN <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x -! y

/// See [`std::primitive::u8::unchecked_mul`] (and similar for other integer types)
let impl_15__unchecked_mul (x y: i64)
    : Prims.Pure i64
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) *
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_15__MAX <: Hax_lib.Int.t_Int) &&
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) *
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) >=
        (Rust_primitives.Hax.Int.from_machine impl_15__MIN <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x *! y

/// See [`std::primitive::u8::rem_euclid`] (and similar for other integer types)
let impl_15__rem_euclid (x y: i64)
    : Prims.Pure i64
      (requires y <>. mk_i64 0 && ~.((x =. impl_15__MIN <: bool) && (y =. mk_i64 (-1) <: bool)))
      (fun _ -> Prims.l_True) = Rust_primitives.Arithmetic.rem_euclid_i64 x y

/// See [`std::primitive::u8::pow`] (and similar for other integer types)
let impl_15__pow (x: i64) (exp: u32)
    : Prims.Pure i64
      (requires (impl_15__overflowing_pow x exp <: (i64 & bool))._2 =. false)
      (fun _ -> Prims.l_True) = Rust_primitives.Arithmetic.pow_i64 x exp

/// See [`std::primitive::i8::abs`] (and similar for other signed integer types)
let impl_15__abs (x: i64) : Prims.Pure i64 (requires x >. impl_15__MIN) (fun _ -> Prims.l_True) =
  if x <. mk_i64 0 then Rust_primitives.Arithmetic.neg x else x

/// See [`std::primitive::u8::ilog2`] (and similar for other integer types)
assume
val impl_15__ilog2': x: i64 -> Prims.Pure u32 (requires x >. mk_i64 0) (fun _ -> Prims.l_True)

unfold
let impl_15__ilog2 = impl_15__ilog2'

/// See [`std::primitive::u8::unchecked_div`] (and similar for other integer types)
let impl_15__unchecked_div (x y: i64)
    : Prims.Pure i64
      (requires y <>. mk_i64 0 && (x <>. impl_15__MIN || y <>. mk_i64 (-1)))
      (fun _ -> Prims.l_True) = x /! y

/// See [`std::primitive::u8::unchecked_rem`] (and similar for other integer types)
let impl_15__unchecked_rem (x y: i64)
    : Prims.Pure i64
      (requires y <>. mk_i64 0 && (x <>. impl_15__MIN || y <>. mk_i64 (-1)))
      (fun _ -> Prims.l_True) = x %! y

/// See [`std::primitive::i8::div_ceil`] (and similar for other signed integer types)
let impl_15__div_ceil (x y: i64)
    : Prims.Pure i64
      (requires y <>. mk_i64 0 && ~.((x =. impl_15__MIN <: bool) && (y =. mk_i64 (-1) <: bool)))
      (fun _ -> Prims.l_True) =
  let d:i64 = x /! y in
  let r:i64 = x %! y in
  if r >. mk_i64 0 && y >. mk_i64 0 || r <. mk_i64 0 && y <. mk_i64 0 then d +! mk_i64 1 else d

/// See [`std::primitive::i8::MIN`] (and similar for other signed integer types)
let impl_16__MIN: i128 = mk_i128 (-170141183460469231731687303715884105728)

/// See [`std::primitive::i8::MAX`] (and similar for other signed integer types)
let impl_16__MAX: i128 = mk_i128 170141183460469231731687303715884105727

/// See [`std::primitive::i8::BITS`] (and similar for other signed integer types)
let impl_16__BITS: u32 = mk_u32 128

let impl_16__wrapping_add (x y: i128) : i128 = Rust_primitives.Arithmetic.wrapping_add_i128 x y

/// See [`std::primitive::u8::saturating_add`] (and similar for other integer types)
let impl_16__saturating_add (x y: i128) : i128 = Rust_primitives.Arithmetic.saturating_add_i128 x y

/// See [`std::primitive::u8::overflowing_add`] (and similar for other integer types)
let impl_16__overflowing_add (x y: i128) : (i128 & bool) =
  Rust_primitives.Arithmetic.overflowing_add_i128 x y

/// See [`std::primitive::u8::wrapping_sub`] (and similar for other integer types)
let impl_16__wrapping_sub (x y: i128) : i128 = Rust_primitives.Arithmetic.wrapping_sub_i128 x y

/// See [`std::primitive::u8::saturating_sub`] (and similar for other integer types)
let impl_16__saturating_sub (x y: i128) : i128 = Rust_primitives.Arithmetic.saturating_sub_i128 x y

/// See [`std::primitive::u8::overflowing_sub`] (and similar for other integer types)
let impl_16__overflowing_sub (x y: i128) : (i128 & bool) =
  Rust_primitives.Arithmetic.overflowing_sub_i128 x y

/// See [`std::primitive::u8::wrapping_mul`] (and similar for other integer types)
let impl_16__wrapping_mul (x y: i128) : i128 = Rust_primitives.Arithmetic.wrapping_mul_i128 x y

/// See [`std::primitive::u8::saturating_mul`] (and similar for other integer types)
let impl_16__saturating_mul (x y: i128) : i128 = Rust_primitives.Arithmetic.saturating_mul_i128 x y

/// See [`std::primitive::u8::overflowing_mul`] (and similar for other integer types)
let impl_16__overflowing_mul (x y: i128) : (i128 & bool) =
  Rust_primitives.Arithmetic.overflowing_mul_i128 x y

/// See [`std::primitive::u8::overflowing_pow`] (and similar for other integer types)
let impl_16__overflowing_pow (x: i128) (exp: u32) : (i128 & bool) =
  Rust_primitives.Arithmetic.overflowing_pow_i128 x exp

/// See [`std::primitive::u8::count_ones`] (and similar for other integer types)
let impl_16__count_ones (x: i128) : u32 =
  let n:u32 = mk_u32 0 in
  let n:u32 =
    Rust_primitives.Hax.Folds.fold_range (mk_u32 0)
      (mk_u32 128)
      (fun n i ->
          let n:u32 = n in
          let i:u32 = i in
          n <=. i <: bool)
      n
      (fun n i ->
          let n:u32 = n in
          let i:u32 = i in
          if ((x >>! i <: i128) &. mk_i128 1 <: i128) =. mk_i128 1 <: bool
          then
            let n:u32 = n +! mk_u32 1 in
            n
          else n)
  in
  n

/// See [`std::primitive::i8::unsigned_abs`] (and similar for other signed integer types)
let impl_16__unsigned_abs (x: i128) : u128 =
  if x <. mk_i128 0
  then (cast (Rust_primitives.Arithmetic.neg (x +! mk_i128 1 <: i128) <: i128) <: u128) +! mk_u128 1
  else cast (x <: i128) <: u128

/// See [`std::primitive::u8::rotate_right`] (and similar for other integer types)
assume
val impl_16__rotate_right': x: i128 -> n: u32 -> i128

unfold
let impl_16__rotate_right = impl_16__rotate_right'

/// See [`std::primitive::u8::rotate_left`] (and similar for other integer types)
assume
val impl_16__rotate_left': x: i128 -> n: u32 -> i128

unfold
let impl_16__rotate_left = impl_16__rotate_left'

/// See [`std::primitive::u8::leading_zeros`] (and similar for other integer types)
assume
val impl_16__leading_zeros': x: i128 -> u32

unfold
let impl_16__leading_zeros = impl_16__leading_zeros'

/// See [`std::primitive::u8::from_be_bytes`] (and similar for other integer types)
assume
val impl_16__from_be_bytes': bytes: t_Array u8 (mk_usize 16) -> i128

unfold
let impl_16__from_be_bytes = impl_16__from_be_bytes'

/// See [`std::primitive::u8::from_le_bytes`] (and similar for other integer types)
assume
val impl_16__from_le_bytes': bytes: t_Array u8 (mk_usize 16) -> i128

unfold
let impl_16__from_le_bytes = impl_16__from_le_bytes'

/// See [`std::primitive::u8::to_be_bytes`] (and similar for other integer types)
assume
val impl_16__to_be_bytes': bytes: i128 -> t_Array u8 (mk_usize 16)

unfold
let impl_16__to_be_bytes = impl_16__to_be_bytes'

/// See [`std::primitive::u8::to_le_bytes`] (and similar for other integer types)
let impl_16__to_le_bytes (bytes: i128) : t_Array u8 (mk_usize 16) =
  Rust_primitives.Slice.array_from_fn #u8
    (mk_usize 16)
    #(usize -> u8)
    (fun i ->
        let i:usize = i in
        cast (bytes >>! ((mk_u32 8 *! (cast (i <: usize) <: u32) <: u32) %! mk_u32 128 <: u32)
            <:
            i128)
        <:
        u8)

/// See [`std::primitive::i8::signum`] (and similar for other signed integer types)
let impl_16__signum (x: i128) : i128 =
  if x >. mk_i128 0 then mk_i128 1 else if x =. mk_i128 0 then mk_i128 0 else mk_i128 (-1)

/// See [`std::primitive::i8::wrapping_neg`] (and similar for other signed integer types)
let impl_16__wrapping_neg (x: i128) : i128 =
  Rust_primitives.Arithmetic.wrapping_sub_i128 (mk_i128 0) x

/// See [`std::primitive::u8::unchecked_add`] (and similar for other integer types)
let impl_16__unchecked_add (x y: i128)
    : Prims.Pure i128
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_16__MAX <: Hax_lib.Int.t_Int) &&
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) >=
        (Rust_primitives.Hax.Int.from_machine impl_16__MIN <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x +! y

/// See [`std::primitive::u8::unchecked_sub`] (and similar for other integer types)
let impl_16__unchecked_sub (x y: i128)
    : Prims.Pure i128
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) -
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_16__MAX <: Hax_lib.Int.t_Int) &&
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) -
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) >=
        (Rust_primitives.Hax.Int.from_machine impl_16__MIN <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x -! y

/// See [`std::primitive::u8::unchecked_mul`] (and similar for other integer types)
let impl_16__unchecked_mul (x y: i128)
    : Prims.Pure i128
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) *
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_16__MAX <: Hax_lib.Int.t_Int) &&
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) *
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) >=
        (Rust_primitives.Hax.Int.from_machine impl_16__MIN <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x *! y

/// See [`std::primitive::u8::rem_euclid`] (and similar for other integer types)
let impl_16__rem_euclid (x y: i128)
    : Prims.Pure i128
      (requires y <>. mk_i128 0 && ~.((x =. impl_16__MIN <: bool) && (y =. mk_i128 (-1) <: bool)))
      (fun _ -> Prims.l_True) = Rust_primitives.Arithmetic.rem_euclid_i128 x y

/// See [`std::primitive::u8::pow`] (and similar for other integer types)
let impl_16__pow (x: i128) (exp: u32)
    : Prims.Pure i128
      (requires (impl_16__overflowing_pow x exp <: (i128 & bool))._2 =. false)
      (fun _ -> Prims.l_True) = Rust_primitives.Arithmetic.pow_i128 x exp

/// See [`std::primitive::i8::abs`] (and similar for other signed integer types)
let impl_16__abs (x: i128) : Prims.Pure i128 (requires x >. impl_16__MIN) (fun _ -> Prims.l_True) =
  if x <. mk_i128 0 then Rust_primitives.Arithmetic.neg x else x

/// See [`std::primitive::u8::ilog2`] (and similar for other integer types)
assume
val impl_16__ilog2': x: i128 -> Prims.Pure u32 (requires x >. mk_i128 0) (fun _ -> Prims.l_True)

unfold
let impl_16__ilog2 = impl_16__ilog2'

/// See [`std::primitive::u8::unchecked_div`] (and similar for other integer types)
let impl_16__unchecked_div (x y: i128)
    : Prims.Pure i128
      (requires y <>. mk_i128 0 && (x <>. impl_16__MIN || y <>. mk_i128 (-1)))
      (fun _ -> Prims.l_True) = x /! y

/// See [`std::primitive::u8::unchecked_rem`] (and similar for other integer types)
let impl_16__unchecked_rem (x y: i128)
    : Prims.Pure i128
      (requires y <>. mk_i128 0 && (x <>. impl_16__MIN || y <>. mk_i128 (-1)))
      (fun _ -> Prims.l_True) = x %! y

/// See [`std::primitive::i8::div_ceil`] (and similar for other signed integer types)
let impl_16__div_ceil (x y: i128)
    : Prims.Pure i128
      (requires y <>. mk_i128 0 && ~.((x =. impl_16__MIN <: bool) && (y =. mk_i128 (-1) <: bool)))
      (fun _ -> Prims.l_True) =
  let d:i128 = x /! y in
  let r:i128 = x %! y in
  if r >. mk_i128 0 && y >. mk_i128 0 || r <. mk_i128 0 && y <. mk_i128 0 then d +! mk_i128 1 else d

/// See [`std::primitive::i8::MIN`] (and similar for other signed integer types)
let impl_17__MIN: isize = Rust_primitives.Arithmetic.v_ISIZE_MIN

/// See [`std::primitive::i8::MAX`] (and similar for other signed integer types)
let impl_17__MAX: isize = Rust_primitives.Arithmetic.v_ISIZE_MAX

/// See [`std::primitive::i8::BITS`] (and similar for other signed integer types)
let impl_17__BITS: u32 = Rust_primitives.Arithmetic.v_SIZE_BITS

let impl_17__wrapping_add (x y: isize) : isize = Rust_primitives.Arithmetic.wrapping_add_isize x y

/// See [`std::primitive::u8::saturating_add`] (and similar for other integer types)
let impl_17__saturating_add (x y: isize) : isize =
  Rust_primitives.Arithmetic.saturating_add_isize x y

/// See [`std::primitive::u8::overflowing_add`] (and similar for other integer types)
let impl_17__overflowing_add (x y: isize) : (isize & bool) =
  Rust_primitives.Arithmetic.overflowing_add_isize x y

/// See [`std::primitive::u8::wrapping_sub`] (and similar for other integer types)
let impl_17__wrapping_sub (x y: isize) : isize = Rust_primitives.Arithmetic.wrapping_sub_isize x y

/// See [`std::primitive::u8::saturating_sub`] (and similar for other integer types)
let impl_17__saturating_sub (x y: isize) : isize =
  Rust_primitives.Arithmetic.saturating_sub_isize x y

/// See [`std::primitive::u8::overflowing_sub`] (and similar for other integer types)
let impl_17__overflowing_sub (x y: isize) : (isize & bool) =
  Rust_primitives.Arithmetic.overflowing_sub_isize x y

/// See [`std::primitive::u8::wrapping_mul`] (and similar for other integer types)
let impl_17__wrapping_mul (x y: isize) : isize = Rust_primitives.Arithmetic.wrapping_mul_isize x y

/// See [`std::primitive::u8::saturating_mul`] (and similar for other integer types)
let impl_17__saturating_mul (x y: isize) : isize =
  Rust_primitives.Arithmetic.saturating_mul_isize x y

/// See [`std::primitive::u8::overflowing_mul`] (and similar for other integer types)
let impl_17__overflowing_mul (x y: isize) : (isize & bool) =
  Rust_primitives.Arithmetic.overflowing_mul_isize x y

/// See [`std::primitive::u8::overflowing_pow`] (and similar for other integer types)
let impl_17__overflowing_pow (x: isize) (exp: u32) : (isize & bool) =
  Rust_primitives.Arithmetic.overflowing_pow_isize x exp

/// See [`std::primitive::u8::count_ones`] (and similar for other integer types)
let impl_17__count_ones (x: isize) : u32 =
  let n:u32 = mk_u32 0 in
  let n:u32 =
    Rust_primitives.Hax.Folds.fold_range (mk_u32 0)
      Rust_primitives.Arithmetic.v_SIZE_BITS
      (fun n i ->
          let n:u32 = n in
          let i:u32 = i in
          n <=. i <: bool)
      n
      (fun n i ->
          let n:u32 = n in
          let i:u32 = i in
          if ((x >>! i <: isize) &. mk_isize 1 <: isize) =. mk_isize 1 <: bool
          then
            let n:u32 = n +! mk_u32 1 in
            n
          else n)
  in
  n

/// See [`std::primitive::i8::unsigned_abs`] (and similar for other signed integer types)
let impl_17__unsigned_abs (x: isize) : usize =
  if x <. mk_isize 0
  then
    (cast (Rust_primitives.Arithmetic.neg (x +! mk_isize 1 <: isize) <: isize) <: usize) +!
    mk_usize 1
  else cast (x <: isize) <: usize

/// See [`std::primitive::u8::rotate_right`] (and similar for other integer types)
assume
val impl_17__rotate_right': x: isize -> n: u32 -> isize

unfold
let impl_17__rotate_right = impl_17__rotate_right'

/// See [`std::primitive::u8::rotate_left`] (and similar for other integer types)
assume
val impl_17__rotate_left': x: isize -> n: u32 -> isize

unfold
let impl_17__rotate_left = impl_17__rotate_left'

/// See [`std::primitive::u8::leading_zeros`] (and similar for other integer types)
assume
val impl_17__leading_zeros': x: isize -> u32

unfold
let impl_17__leading_zeros = impl_17__leading_zeros'

/// See [`std::primitive::u8::from_be_bytes`] (and similar for other integer types)
assume
val impl_17__from_be_bytes': bytes: t_Array u8 (mk_usize 8) -> isize

unfold
let impl_17__from_be_bytes = impl_17__from_be_bytes'

/// See [`std::primitive::u8::from_le_bytes`] (and similar for other integer types)
assume
val impl_17__from_le_bytes': bytes: t_Array u8 (mk_usize 8) -> isize

unfold
let impl_17__from_le_bytes = impl_17__from_le_bytes'

/// See [`std::primitive::u8::to_be_bytes`] (and similar for other integer types)
assume
val impl_17__to_be_bytes': bytes: isize -> t_Array u8 (mk_usize 8)

unfold
let impl_17__to_be_bytes = impl_17__to_be_bytes'

/// See [`std::primitive::u8::to_le_bytes`] (and similar for other integer types)
let impl_17__to_le_bytes (bytes: isize) : t_Array u8 (mk_usize 8) =
  Rust_primitives.Slice.array_from_fn #u8
    (mk_usize 8)
    #(usize -> u8)
    (fun i ->
        let i:usize = i in
        cast (bytes >>!
            ((mk_u32 8 *! (cast (i <: usize) <: u32) <: u32) %!
              Rust_primitives.Arithmetic.v_SIZE_BITS
              <:
              u32)
            <:
            isize)
        <:
        u8)

/// See [`std::primitive::i8::signum`] (and similar for other signed integer types)
let impl_17__signum (x: isize) : isize =
  if x >. mk_isize 0 then mk_isize 1 else if x =. mk_isize 0 then mk_isize 0 else mk_isize (-1)

/// See [`std::primitive::i8::wrapping_neg`] (and similar for other signed integer types)
let impl_17__wrapping_neg (x: isize) : isize =
  Rust_primitives.Arithmetic.wrapping_sub_isize (mk_isize 0) x

/// See [`std::primitive::u8::unchecked_add`] (and similar for other integer types)
let impl_17__unchecked_add (x y: isize)
    : Prims.Pure isize
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_17__MAX <: Hax_lib.Int.t_Int) &&
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) +
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) >=
        (Rust_primitives.Hax.Int.from_machine impl_17__MIN <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x +! y

/// See [`std::primitive::u8::unchecked_sub`] (and similar for other integer types)
let impl_17__unchecked_sub (x y: isize)
    : Prims.Pure isize
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) -
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_17__MAX <: Hax_lib.Int.t_Int) &&
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) -
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) >=
        (Rust_primitives.Hax.Int.from_machine impl_17__MIN <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x -! y

/// See [`std::primitive::u8::unchecked_mul`] (and similar for other integer types)
let impl_17__unchecked_mul (x y: isize)
    : Prims.Pure isize
      (requires
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) *
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine impl_17__MAX <: Hax_lib.Int.t_Int) &&
        ((Rust_primitives.Hax.Int.from_machine x <: Hax_lib.Int.t_Int) *
          (Rust_primitives.Hax.Int.from_machine y <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int) >=
        (Rust_primitives.Hax.Int.from_machine impl_17__MIN <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) = x *! y

/// See [`std::primitive::u8::rem_euclid`] (and similar for other integer types)
let impl_17__rem_euclid (x y: isize)
    : Prims.Pure isize
      (requires y <>. mk_isize 0 && ~.((x =. impl_17__MIN <: bool) && (y =. mk_isize (-1) <: bool)))
      (fun _ -> Prims.l_True) = Rust_primitives.Arithmetic.rem_euclid_isize x y

/// See [`std::primitive::u8::pow`] (and similar for other integer types)
let impl_17__pow (x: isize) (exp: u32)
    : Prims.Pure isize
      (requires (impl_17__overflowing_pow x exp <: (isize & bool))._2 =. false)
      (fun _ -> Prims.l_True) = Rust_primitives.Arithmetic.pow_isize x exp

/// See [`std::primitive::i8::abs`] (and similar for other signed integer types)
let impl_17__abs (x: isize) : Prims.Pure isize (requires x >. impl_17__MIN) (fun _ -> Prims.l_True) =
  if x <. mk_isize 0 then Rust_primitives.Arithmetic.neg x else x

/// See [`std::primitive::u8::ilog2`] (and similar for other integer types)
assume
val impl_17__ilog2': x: isize -> Prims.Pure u32 (requires x >. mk_isize 0) (fun _ -> Prims.l_True)

unfold
let impl_17__ilog2 = impl_17__ilog2'

/// See [`std::primitive::u8::unchecked_div`] (and similar for other integer types)
let impl_17__unchecked_div (x y: isize)
    : Prims.Pure isize
      (requires y <>. mk_isize 0 && (x <>. impl_17__MIN || y <>. mk_isize (-1)))
      (fun _ -> Prims.l_True) = x /! y

/// See [`std::primitive::u8::unchecked_rem`] (and similar for other integer types)
let impl_17__unchecked_rem (x y: isize)
    : Prims.Pure isize
      (requires y <>. mk_isize 0 && (x <>. impl_17__MIN || y <>. mk_isize (-1)))
      (fun _ -> Prims.l_True) = x %! y

/// See [`std::primitive::i8::div_ceil`] (and similar for other signed integer types)
let impl_17__div_ceil (x y: isize)
    : Prims.Pure isize
      (requires y <>. mk_isize 0 && ~.((x =. impl_17__MIN <: bool) && (y =. mk_isize (-1) <: bool)))
      (fun _ -> Prims.l_True) =
  let d:isize = x /! y in
  let r:isize = x %! y in
  if r >. mk_isize 0 && y >. mk_isize 0 || r <. mk_isize 0 && y <. mk_isize 0
  then d +! mk_isize 1
  else d

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_18__from__num: Core_models.Default.t_Default u8 =
  {
    f_default_pre = (fun (_: Prims.unit) -> true);
    f_default_post = (fun (_: Prims.unit) (out: u8) -> true);
    f_default = fun (_: Prims.unit) -> mk_u8 0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_19__from__num: Core_models.Default.t_Default u16 =
  {
    f_default_pre = (fun (_: Prims.unit) -> true);
    f_default_post = (fun (_: Prims.unit) (out: u16) -> true);
    f_default = fun (_: Prims.unit) -> mk_u16 0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_20__from__num: Core_models.Default.t_Default u32 =
  {
    f_default_pre = (fun (_: Prims.unit) -> true);
    f_default_post = (fun (_: Prims.unit) (out: u32) -> true);
    f_default = fun (_: Prims.unit) -> mk_u32 0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_21__from__num: Core_models.Default.t_Default u64 =
  {
    f_default_pre = (fun (_: Prims.unit) -> true);
    f_default_post = (fun (_: Prims.unit) (out: u64) -> true);
    f_default = fun (_: Prims.unit) -> mk_u64 0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_22__from__num: Core_models.Default.t_Default u128 =
  {
    f_default_pre = (fun (_: Prims.unit) -> true);
    f_default_post = (fun (_: Prims.unit) (out: u128) -> true);
    f_default = fun (_: Prims.unit) -> mk_u128 0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_23__from__num: Core_models.Default.t_Default usize =
  {
    f_default_pre = (fun (_: Prims.unit) -> true);
    f_default_post = (fun (_: Prims.unit) (out: usize) -> true);
    f_default = fun (_: Prims.unit) -> mk_usize 0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_24__from__num: Core_models.Default.t_Default i8 =
  {
    f_default_pre = (fun (_: Prims.unit) -> true);
    f_default_post = (fun (_: Prims.unit) (out: i8) -> true);
    f_default = fun (_: Prims.unit) -> mk_i8 0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_25__from__num: Core_models.Default.t_Default i16 =
  {
    f_default_pre = (fun (_: Prims.unit) -> true);
    f_default_post = (fun (_: Prims.unit) (out: i16) -> true);
    f_default = fun (_: Prims.unit) -> mk_i16 0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_26__from__num: Core_models.Default.t_Default i32 =
  {
    f_default_pre = (fun (_: Prims.unit) -> true);
    f_default_post = (fun (_: Prims.unit) (out: i32) -> true);
    f_default = fun (_: Prims.unit) -> mk_i32 0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_27__from__num: Core_models.Default.t_Default i64 =
  {
    f_default_pre = (fun (_: Prims.unit) -> true);
    f_default_post = (fun (_: Prims.unit) (out: i64) -> true);
    f_default = fun (_: Prims.unit) -> mk_i64 0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_28__from__num: Core_models.Default.t_Default i128 =
  {
    f_default_pre = (fun (_: Prims.unit) -> true);
    f_default_post = (fun (_: Prims.unit) (out: i128) -> true);
    f_default = fun (_: Prims.unit) -> mk_i128 0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_29__from__num: Core_models.Default.t_Default isize =
  {
    f_default_pre = (fun (_: Prims.unit) -> true);
    f_default_post = (fun (_: Prims.unit) (out: isize) -> true);
    f_default = fun (_: Prims.unit) -> mk_isize 0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_30__from__num: Core_models.Default.t_Default bool =
  {
    f_default_pre = (fun (_: Prims.unit) -> true);
    f_default_post = (fun (_: Prims.unit) (out: bool) -> true);
    f_default = fun (_: Prims.unit) -> false
  }

/// See [`std::ops::RangeTo`]
type t_RangeTo (v_T: Type0) = { f_end:v_T }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_29 (#v_T: Type0) (v_N: usize)
    : Core_models.Ops.Index.t_Index (t_Array v_T v_N) (t_RangeTo usize) =
  {
    f_Output = t_Slice v_T;
    f_index_pre = (fun (self_: t_Array v_T v_N) (i: t_RangeTo usize) -> i.f_end <=. v_N);
    f_index_post = (fun (self: t_Array v_T v_N) (i: t_RangeTo usize) (out: t_Slice v_T) -> true);
    f_index
    =
    fun (self: t_Array v_T v_N) (i: t_RangeTo usize) ->
      Rust_primitives.Slice.array_slice #v_T v_N self (mk_usize 0) i.f_end
  }

/// See [`std::ops::RangeFrom`]
type t_RangeFrom (v_T: Type0) = { f_start:v_T }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_30 (#v_T: Type0) (v_N: usize)
    : Core_models.Ops.Index.t_Index (t_Array v_T v_N) (t_RangeFrom usize) =
  {
    f_Output = t_Slice v_T;
    f_index_pre = (fun (self_: t_Array v_T v_N) (i: t_RangeFrom usize) -> i.f_start <=. v_N);
    f_index_post = (fun (self: t_Array v_T v_N) (i: t_RangeFrom usize) (out: t_Slice v_T) -> true);
    f_index
    =
    fun (self: t_Array v_T v_N) (i: t_RangeFrom usize) ->
      Rust_primitives.Slice.array_slice #v_T v_N self i.f_start v_N
  }

/// See [`std::ops::Range`]
type t_Range (v_T: Type0) = {
  f_start:v_T;
  f_end:v_T
}

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_28 (#v_T: Type0) (v_N: usize)
    : Core_models.Ops.Index.t_Index (t_Array v_T v_N) (t_Range usize) =
  {
    f_Output = t_Slice v_T;
    f_index_pre
    =
    (fun (self_: t_Array v_T v_N) (i: t_Range usize) -> i.f_start <=. i.f_end && i.f_end <=. v_N);
    f_index_post = (fun (self: t_Array v_T v_N) (i: t_Range usize) (out: t_Slice v_T) -> true);
    f_index
    =
    fun (self: t_Array v_T v_N) (i: t_Range usize) ->
      Rust_primitives.Slice.array_slice #v_T v_N self i.f_start i.f_end
  }

/// See [`std::ops::RangeFull`]
type t_RangeFull = | RangeFull : t_RangeFull

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_31 (#v_T: Type0) (v_N: usize) : Core_models.Ops.Index.t_Index (t_Array v_T v_N) t_RangeFull =
  {
    f_Output = t_Slice v_T;
    f_index_pre = (fun (self: t_Array v_T v_N) (i: t_RangeFull) -> true);
    f_index_post = (fun (self: t_Array v_T v_N) (i: t_RangeFull) (out: t_Slice v_T) -> true);
    f_index
    =
    fun (self: t_Array v_T v_N) (i: t_RangeFull) ->
      Rust_primitives.Slice.array_slice #v_T v_N self (mk_usize 0) v_N
  }

/// See [`std::ops::RangeInclusive`]
type t_RangeInclusive (v_T: Type0) = {
  f_start_:v_T;
  f_end_:v_T;
  f_exhausted:bool
}

/// See [`std::ops::RangeToInclusive`]
type t_RangeToInclusive (v_T: Type0) = { f_end:v_T }

/// See [`std::ops::Bound`]
type t_Bound (v_T: Type0) =
  | Bound_Included : v_T -> t_Bound v_T
  | Bound_Excluded : v_T -> t_Bound v_T
  | Bound_Unbounded : t_Bound v_T

/// See [`std::ops::RangeInclusive::new`]
let impl_7__new (#v_T: Type0) (start v_end: v_T) : t_RangeInclusive v_T =
  { f_start_ = start; f_end_ = v_end; f_exhausted = false } <: t_RangeInclusive v_T

/// See [`std::ops::RangeInclusive::start`]
let impl_7__start (#v_T: Type0) (self: t_RangeInclusive v_T) : v_T = self.f_start_

/// See [`std::ops::RangeInclusive::end`]
let impl_7__end (#v_T: Type0) (self: t_RangeInclusive v_T) : v_T = self.f_end_

/// See [`std::ops::RangeInclusive::into_inner`]
let impl_7__into_inner (#v_T: Type0) (self: t_RangeInclusive v_T) : (v_T & v_T) =
  self.f_start_, self.f_end_ <: (v_T & v_T)

let bound_as_ref (#v_T: Type0) (bound: t_Bound v_T) : t_Bound v_T =
  match bound <: t_Bound v_T with
  | Bound_Included x -> Bound_Included x <: t_Bound v_T
  | Bound_Excluded x -> Bound_Excluded x <: t_Bound v_T
  | Bound_Unbounded  -> Bound_Unbounded <: t_Bound v_T

/// See [`std::option::Option`]
type t_Option (v_T: Type0) =
  | Option_Some : v_T -> t_Option v_T
  | Option_None : t_Option v_T

/// See [`std::iter::Chain`]
type t_Chain (v_A: Type0) (v_B: Type0) = {
  f_a:t_Option v_A;
  f_b:v_B
}

/// See [`std::iter::FlatMap`]
type t_FlatMap (v_I: Type0) (v_U: Type0) (v_F: Type0) = {
  f_it:v_I;
  f_f:v_F;
  f_current:t_Option v_U
}

/// See [`std::primitive::u8::checked_add`] (and similar for other integer types)
let impl_6__checked_add (x y: u8) : t_Option u8 =
  let (result: u8), (overflowed: bool) = impl_6__overflowing_add x y in
  if overflowed then Option_None <: t_Option u8 else Option_Some result <: t_Option u8

/// See [`std::primitive::u8::checked_sub`] (and similar for other integer types)
let impl_6__checked_sub (x y: u8) : t_Option u8 =
  let (result: u8), (overflowed: bool) = impl_6__overflowing_sub x y in
  if overflowed then Option_None <: t_Option u8 else Option_Some result <: t_Option u8

/// See [`std::primitive::u8::checked_mul`] (and similar for other integer types)
let impl_6__checked_mul (x y: u8) : t_Option u8 =
  let (result: u8), (overflowed: bool) = impl_6__overflowing_mul x y in
  if overflowed then Option_None <: t_Option u8 else Option_Some result <: t_Option u8

/// See [`std::primitive::u8::checked_div`] (and similar for other integer types)
let impl_6__checked_div (x y: u8) : t_Option u8 =
  if y =. mk_u8 0 then Option_None <: t_Option u8 else Option_Some (x /! y) <: t_Option u8

/// See [`std::primitive::u8::checked_rem`] (and similar for other integer types)
let impl_6__checked_rem (x y: u8) : t_Option u8 =
  if y =. mk_u8 0 then Option_None <: t_Option u8 else Option_Some (x %! y) <: t_Option u8

/// See [`std::primitive::u8::checked_add`] (and similar for other integer types)
let impl_7__checked_add (x y: u16) : t_Option u16 =
  let (result: u16), (overflowed: bool) = impl_7__overflowing_add x y in
  if overflowed then Option_None <: t_Option u16 else Option_Some result <: t_Option u16

/// See [`std::primitive::u8::checked_sub`] (and similar for other integer types)
let impl_7__checked_sub (x y: u16) : t_Option u16 =
  let (result: u16), (overflowed: bool) = impl_7__overflowing_sub x y in
  if overflowed then Option_None <: t_Option u16 else Option_Some result <: t_Option u16

/// See [`std::primitive::u8::checked_mul`] (and similar for other integer types)
let impl_7__checked_mul (x y: u16) : t_Option u16 =
  let (result: u16), (overflowed: bool) = impl_7__overflowing_mul x y in
  if overflowed then Option_None <: t_Option u16 else Option_Some result <: t_Option u16

/// See [`std::primitive::u8::checked_div`] (and similar for other integer types)
let impl_7__checked_div (x y: u16) : t_Option u16 =
  if y =. mk_u16 0 then Option_None <: t_Option u16 else Option_Some (x /! y) <: t_Option u16

/// See [`std::primitive::u8::checked_rem`] (and similar for other integer types)
let impl_7__checked_rem (x y: u16) : t_Option u16 =
  if y =. mk_u16 0 then Option_None <: t_Option u16 else Option_Some (x %! y) <: t_Option u16

/// See [`std::primitive::u8::checked_add`] (and similar for other integer types)
let impl_8__checked_add (x y: u32) : t_Option u32 =
  let (result: u32), (overflowed: bool) = impl_8__overflowing_add x y in
  if overflowed then Option_None <: t_Option u32 else Option_Some result <: t_Option u32

/// See [`std::primitive::u8::checked_sub`] (and similar for other integer types)
let impl_8__checked_sub (x y: u32) : t_Option u32 =
  let (result: u32), (overflowed: bool) = impl_8__overflowing_sub x y in
  if overflowed then Option_None <: t_Option u32 else Option_Some result <: t_Option u32

/// See [`std::primitive::u8::checked_mul`] (and similar for other integer types)
let impl_8__checked_mul (x y: u32) : t_Option u32 =
  let (result: u32), (overflowed: bool) = impl_8__overflowing_mul x y in
  if overflowed then Option_None <: t_Option u32 else Option_Some result <: t_Option u32

/// See [`std::primitive::u8::checked_div`] (and similar for other integer types)
let impl_8__checked_div (x y: u32) : t_Option u32 =
  if y =. mk_u32 0 then Option_None <: t_Option u32 else Option_Some (x /! y) <: t_Option u32

/// See [`std::primitive::u8::checked_rem`] (and similar for other integer types)
let impl_8__checked_rem (x y: u32) : t_Option u32 =
  if y =. mk_u32 0 then Option_None <: t_Option u32 else Option_Some (x %! y) <: t_Option u32

/// See [`std::primitive::u8::checked_add`] (and similar for other integer types)
let impl_9__checked_add (x y: u64) : t_Option u64 =
  let (result: u64), (overflowed: bool) = impl_9__overflowing_add x y in
  if overflowed then Option_None <: t_Option u64 else Option_Some result <: t_Option u64

/// See [`std::primitive::u8::checked_sub`] (and similar for other integer types)
let impl_9__checked_sub (x y: u64) : t_Option u64 =
  let (result: u64), (overflowed: bool) = impl_9__overflowing_sub x y in
  if overflowed then Option_None <: t_Option u64 else Option_Some result <: t_Option u64

/// See [`std::primitive::u8::checked_mul`] (and similar for other integer types)
let impl_9__checked_mul (x y: u64) : t_Option u64 =
  let (result: u64), (overflowed: bool) = impl_9__overflowing_mul x y in
  if overflowed then Option_None <: t_Option u64 else Option_Some result <: t_Option u64

/// See [`std::primitive::u8::checked_div`] (and similar for other integer types)
let impl_9__checked_div (x y: u64) : t_Option u64 =
  if y =. mk_u64 0 then Option_None <: t_Option u64 else Option_Some (x /! y) <: t_Option u64

/// See [`std::primitive::u8::checked_rem`] (and similar for other integer types)
let impl_9__checked_rem (x y: u64) : t_Option u64 =
  if y =. mk_u64 0 then Option_None <: t_Option u64 else Option_Some (x %! y) <: t_Option u64

/// See [`std::primitive::u8::checked_add`] (and similar for other integer types)
let impl_10__checked_add (x y: u128) : t_Option u128 =
  let (result: u128), (overflowed: bool) = impl_10__overflowing_add x y in
  if overflowed then Option_None <: t_Option u128 else Option_Some result <: t_Option u128

/// See [`std::primitive::u8::checked_sub`] (and similar for other integer types)
let impl_10__checked_sub (x y: u128) : t_Option u128 =
  let (result: u128), (overflowed: bool) = impl_10__overflowing_sub x y in
  if overflowed then Option_None <: t_Option u128 else Option_Some result <: t_Option u128

/// See [`std::primitive::u8::checked_mul`] (and similar for other integer types)
let impl_10__checked_mul (x y: u128) : t_Option u128 =
  let (result: u128), (overflowed: bool) = impl_10__overflowing_mul x y in
  if overflowed then Option_None <: t_Option u128 else Option_Some result <: t_Option u128

/// See [`std::primitive::u8::checked_div`] (and similar for other integer types)
let impl_10__checked_div (x y: u128) : t_Option u128 =
  if y =. mk_u128 0 then Option_None <: t_Option u128 else Option_Some (x /! y) <: t_Option u128

/// See [`std::primitive::u8::checked_rem`] (and similar for other integer types)
let impl_10__checked_rem (x y: u128) : t_Option u128 =
  if y =. mk_u128 0 then Option_None <: t_Option u128 else Option_Some (x %! y) <: t_Option u128

/// See [`std::primitive::u8::checked_add`] (and similar for other integer types)
let impl_11__checked_add (x y: usize) : t_Option usize =
  let (result: usize), (overflowed: bool) = impl_11__overflowing_add x y in
  if overflowed then Option_None <: t_Option usize else Option_Some result <: t_Option usize

/// See [`std::primitive::u8::checked_sub`] (and similar for other integer types)
let impl_11__checked_sub (x y: usize) : t_Option usize =
  let (result: usize), (overflowed: bool) = impl_11__overflowing_sub x y in
  if overflowed then Option_None <: t_Option usize else Option_Some result <: t_Option usize

/// See [`std::primitive::u8::checked_mul`] (and similar for other integer types)
let impl_11__checked_mul (x y: usize) : t_Option usize =
  let (result: usize), (overflowed: bool) = impl_11__overflowing_mul x y in
  if overflowed then Option_None <: t_Option usize else Option_Some result <: t_Option usize

/// See [`std::primitive::u8::checked_div`] (and similar for other integer types)
let impl_11__checked_div (x y: usize) : t_Option usize =
  if y =. mk_usize 0 then Option_None <: t_Option usize else Option_Some (x /! y) <: t_Option usize

/// See [`std::primitive::u8::checked_rem`] (and similar for other integer types)
let impl_11__checked_rem (x y: usize) : t_Option usize =
  if y =. mk_usize 0 then Option_None <: t_Option usize else Option_Some (x %! y) <: t_Option usize

/// See [`std::primitive::u8::checked_add`] (and similar for other integer types)
let impl_12__checked_add (x y: i8) : t_Option i8 =
  let (result: i8), (overflowed: bool) = impl_12__overflowing_add x y in
  if overflowed then Option_None <: t_Option i8 else Option_Some result <: t_Option i8

/// See [`std::primitive::u8::checked_sub`] (and similar for other integer types)
let impl_12__checked_sub (x y: i8) : t_Option i8 =
  let (result: i8), (overflowed: bool) = impl_12__overflowing_sub x y in
  if overflowed then Option_None <: t_Option i8 else Option_Some result <: t_Option i8

/// See [`std::primitive::i8::checked_add_unsigned`] (and similar for other signed integer types)
let impl_12__checked_add_unsigned (x: i8) (y: u8) : t_Option i8 =
  let (result: i8), (overflowed: bool) = impl_12__overflowing_add x (cast (y <: u8) <: i8) in
  if overflowed =. (y >. (cast (impl_12__MAX <: i8) <: u8) <: bool)
  then Option_Some result <: t_Option i8
  else Option_None <: t_Option i8

/// See [`std::primitive::i8::checked_sub_unsigned`] (and similar for other signed integer types)
let impl_12__checked_sub_unsigned (x: i8) (y: u8) : t_Option i8 =
  let (result: i8), (overflowed: bool) = impl_12__overflowing_sub x (cast (y <: u8) <: i8) in
  if overflowed =. (y >. (cast (impl_12__MAX <: i8) <: u8) <: bool)
  then Option_Some result <: t_Option i8
  else Option_None <: t_Option i8

/// See [`std::primitive::u8::checked_mul`] (and similar for other integer types)
let impl_12__checked_mul (x y: i8) : t_Option i8 =
  let (result: i8), (overflowed: bool) = impl_12__overflowing_mul x y in
  if overflowed then Option_None <: t_Option i8 else Option_Some result <: t_Option i8

/// See [`std::primitive::i8::checked_div`] (and similar for other signed integer types)
let impl_12__checked_div (x y: i8) : t_Option i8 =
  if y =. mk_i8 0 || x =. impl_12__MIN && y =. mk_i8 (-1)
  then Option_None <: t_Option i8
  else Option_Some (x /! y) <: t_Option i8

/// See [`std::primitive::i8::checked_rem`] (and similar for other signed integer types)
let impl_12__checked_rem (x y: i8) : t_Option i8 =
  if y =. mk_i8 0 || x =. impl_12__MIN && y =. mk_i8 (-1)
  then Option_None <: t_Option i8
  else Option_Some (x %! y) <: t_Option i8

/// See [`std::primitive::u8::checked_add`] (and similar for other integer types)
let impl_13__checked_add (x y: i16) : t_Option i16 =
  let (result: i16), (overflowed: bool) = impl_13__overflowing_add x y in
  if overflowed then Option_None <: t_Option i16 else Option_Some result <: t_Option i16

/// See [`std::primitive::u8::checked_sub`] (and similar for other integer types)
let impl_13__checked_sub (x y: i16) : t_Option i16 =
  let (result: i16), (overflowed: bool) = impl_13__overflowing_sub x y in
  if overflowed then Option_None <: t_Option i16 else Option_Some result <: t_Option i16

/// See [`std::primitive::i8::checked_add_unsigned`] (and similar for other signed integer types)
let impl_13__checked_add_unsigned (x: i16) (y: u16) : t_Option i16 =
  let (result: i16), (overflowed: bool) = impl_13__overflowing_add x (cast (y <: u16) <: i16) in
  if overflowed =. (y >. (cast (impl_13__MAX <: i16) <: u16) <: bool)
  then Option_Some result <: t_Option i16
  else Option_None <: t_Option i16

/// See [`std::primitive::i8::checked_sub_unsigned`] (and similar for other signed integer types)
let impl_13__checked_sub_unsigned (x: i16) (y: u16) : t_Option i16 =
  let (result: i16), (overflowed: bool) = impl_13__overflowing_sub x (cast (y <: u16) <: i16) in
  if overflowed =. (y >. (cast (impl_13__MAX <: i16) <: u16) <: bool)
  then Option_Some result <: t_Option i16
  else Option_None <: t_Option i16

/// See [`std::primitive::u8::checked_mul`] (and similar for other integer types)
let impl_13__checked_mul (x y: i16) : t_Option i16 =
  let (result: i16), (overflowed: bool) = impl_13__overflowing_mul x y in
  if overflowed then Option_None <: t_Option i16 else Option_Some result <: t_Option i16

/// See [`std::primitive::i8::checked_div`] (and similar for other signed integer types)
let impl_13__checked_div (x y: i16) : t_Option i16 =
  if y =. mk_i16 0 || x =. impl_13__MIN && y =. mk_i16 (-1)
  then Option_None <: t_Option i16
  else Option_Some (x /! y) <: t_Option i16

/// See [`std::primitive::i8::checked_rem`] (and similar for other signed integer types)
let impl_13__checked_rem (x y: i16) : t_Option i16 =
  if y =. mk_i16 0 || x =. impl_13__MIN && y =. mk_i16 (-1)
  then Option_None <: t_Option i16
  else Option_Some (x %! y) <: t_Option i16

/// See [`std::primitive::u8::checked_add`] (and similar for other integer types)
let impl_14__checked_add (x y: i32) : t_Option i32 =
  let (result: i32), (overflowed: bool) = impl_14__overflowing_add x y in
  if overflowed then Option_None <: t_Option i32 else Option_Some result <: t_Option i32

/// See [`std::primitive::u8::checked_sub`] (and similar for other integer types)
let impl_14__checked_sub (x y: i32) : t_Option i32 =
  let (result: i32), (overflowed: bool) = impl_14__overflowing_sub x y in
  if overflowed then Option_None <: t_Option i32 else Option_Some result <: t_Option i32

/// See [`std::primitive::i8::checked_add_unsigned`] (and similar for other signed integer types)
let impl_14__checked_add_unsigned (x: i32) (y: u32) : t_Option i32 =
  let (result: i32), (overflowed: bool) = impl_14__overflowing_add x (cast (y <: u32) <: i32) in
  if overflowed =. (y >. (cast (impl_14__MAX <: i32) <: u32) <: bool)
  then Option_Some result <: t_Option i32
  else Option_None <: t_Option i32

/// See [`std::primitive::i8::checked_sub_unsigned`] (and similar for other signed integer types)
let impl_14__checked_sub_unsigned (x: i32) (y: u32) : t_Option i32 =
  let (result: i32), (overflowed: bool) = impl_14__overflowing_sub x (cast (y <: u32) <: i32) in
  if overflowed =. (y >. (cast (impl_14__MAX <: i32) <: u32) <: bool)
  then Option_Some result <: t_Option i32
  else Option_None <: t_Option i32

/// See [`std::primitive::u8::checked_mul`] (and similar for other integer types)
let impl_14__checked_mul (x y: i32) : t_Option i32 =
  let (result: i32), (overflowed: bool) = impl_14__overflowing_mul x y in
  if overflowed then Option_None <: t_Option i32 else Option_Some result <: t_Option i32

/// See [`std::primitive::i8::checked_div`] (and similar for other signed integer types)
let impl_14__checked_div (x y: i32) : t_Option i32 =
  if y =. mk_i32 0 || x =. impl_14__MIN && y =. mk_i32 (-1)
  then Option_None <: t_Option i32
  else Option_Some (x /! y) <: t_Option i32

/// See [`std::primitive::i8::checked_rem`] (and similar for other signed integer types)
let impl_14__checked_rem (x y: i32) : t_Option i32 =
  if y =. mk_i32 0 || x =. impl_14__MIN && y =. mk_i32 (-1)
  then Option_None <: t_Option i32
  else Option_Some (x %! y) <: t_Option i32

/// See [`std::primitive::u8::checked_add`] (and similar for other integer types)
let impl_15__checked_add (x y: i64) : t_Option i64 =
  let (result: i64), (overflowed: bool) = impl_15__overflowing_add x y in
  if overflowed then Option_None <: t_Option i64 else Option_Some result <: t_Option i64

/// See [`std::primitive::u8::checked_sub`] (and similar for other integer types)
let impl_15__checked_sub (x y: i64) : t_Option i64 =
  let (result: i64), (overflowed: bool) = impl_15__overflowing_sub x y in
  if overflowed then Option_None <: t_Option i64 else Option_Some result <: t_Option i64

/// See [`std::primitive::i8::checked_add_unsigned`] (and similar for other signed integer types)
let impl_15__checked_add_unsigned (x: i64) (y: u64) : t_Option i64 =
  let (result: i64), (overflowed: bool) = impl_15__overflowing_add x (cast (y <: u64) <: i64) in
  if overflowed =. (y >. (cast (impl_15__MAX <: i64) <: u64) <: bool)
  then Option_Some result <: t_Option i64
  else Option_None <: t_Option i64

/// See [`std::primitive::i8::checked_sub_unsigned`] (and similar for other signed integer types)
let impl_15__checked_sub_unsigned (x: i64) (y: u64) : t_Option i64 =
  let (result: i64), (overflowed: bool) = impl_15__overflowing_sub x (cast (y <: u64) <: i64) in
  if overflowed =. (y >. (cast (impl_15__MAX <: i64) <: u64) <: bool)
  then Option_Some result <: t_Option i64
  else Option_None <: t_Option i64

/// See [`std::primitive::u8::checked_mul`] (and similar for other integer types)
let impl_15__checked_mul (x y: i64) : t_Option i64 =
  let (result: i64), (overflowed: bool) = impl_15__overflowing_mul x y in
  if overflowed then Option_None <: t_Option i64 else Option_Some result <: t_Option i64

/// See [`std::primitive::i8::checked_div`] (and similar for other signed integer types)
let impl_15__checked_div (x y: i64) : t_Option i64 =
  if y =. mk_i64 0 || x =. impl_15__MIN && y =. mk_i64 (-1)
  then Option_None <: t_Option i64
  else Option_Some (x /! y) <: t_Option i64

/// See [`std::primitive::i8::checked_rem`] (and similar for other signed integer types)
let impl_15__checked_rem (x y: i64) : t_Option i64 =
  if y =. mk_i64 0 || x =. impl_15__MIN && y =. mk_i64 (-1)
  then Option_None <: t_Option i64
  else Option_Some (x %! y) <: t_Option i64

/// See [`std::primitive::u8::checked_add`] (and similar for other integer types)
let impl_16__checked_add (x y: i128) : t_Option i128 =
  let (result: i128), (overflowed: bool) = impl_16__overflowing_add x y in
  if overflowed then Option_None <: t_Option i128 else Option_Some result <: t_Option i128

/// See [`std::primitive::u8::checked_sub`] (and similar for other integer types)
let impl_16__checked_sub (x y: i128) : t_Option i128 =
  let (result: i128), (overflowed: bool) = impl_16__overflowing_sub x y in
  if overflowed then Option_None <: t_Option i128 else Option_Some result <: t_Option i128

/// See [`std::primitive::i8::checked_add_unsigned`] (and similar for other signed integer types)
let impl_16__checked_add_unsigned (x: i128) (y: u128) : t_Option i128 =
  let (result: i128), (overflowed: bool) = impl_16__overflowing_add x (cast (y <: u128) <: i128) in
  if overflowed =. (y >. (cast (impl_16__MAX <: i128) <: u128) <: bool)
  then Option_Some result <: t_Option i128
  else Option_None <: t_Option i128

/// See [`std::primitive::i8::checked_sub_unsigned`] (and similar for other signed integer types)
let impl_16__checked_sub_unsigned (x: i128) (y: u128) : t_Option i128 =
  let (result: i128), (overflowed: bool) = impl_16__overflowing_sub x (cast (y <: u128) <: i128) in
  if overflowed =. (y >. (cast (impl_16__MAX <: i128) <: u128) <: bool)
  then Option_Some result <: t_Option i128
  else Option_None <: t_Option i128

/// See [`std::primitive::u8::checked_mul`] (and similar for other integer types)
let impl_16__checked_mul (x y: i128) : t_Option i128 =
  let (result: i128), (overflowed: bool) = impl_16__overflowing_mul x y in
  if overflowed then Option_None <: t_Option i128 else Option_Some result <: t_Option i128

/// See [`std::primitive::i8::checked_div`] (and similar for other signed integer types)
let impl_16__checked_div (x y: i128) : t_Option i128 =
  if y =. mk_i128 0 || x =. impl_16__MIN && y =. mk_i128 (-1)
  then Option_None <: t_Option i128
  else Option_Some (x /! y) <: t_Option i128

/// See [`std::primitive::i8::checked_rem`] (and similar for other signed integer types)
let impl_16__checked_rem (x y: i128) : t_Option i128 =
  if y =. mk_i128 0 || x =. impl_16__MIN && y =. mk_i128 (-1)
  then Option_None <: t_Option i128
  else Option_Some (x %! y) <: t_Option i128

/// See [`std::primitive::u8::checked_add`] (and similar for other integer types)
let impl_17__checked_add (x y: isize) : t_Option isize =
  let (result: isize), (overflowed: bool) = impl_17__overflowing_add x y in
  if overflowed then Option_None <: t_Option isize else Option_Some result <: t_Option isize

/// See [`std::primitive::u8::checked_sub`] (and similar for other integer types)
let impl_17__checked_sub (x y: isize) : t_Option isize =
  let (result: isize), (overflowed: bool) = impl_17__overflowing_sub x y in
  if overflowed then Option_None <: t_Option isize else Option_Some result <: t_Option isize

/// See [`std::primitive::i8::checked_add_unsigned`] (and similar for other signed integer types)
let impl_17__checked_add_unsigned (x: isize) (y: usize) : t_Option isize =
  let (result: isize), (overflowed: bool) =
    impl_17__overflowing_add x (cast (y <: usize) <: isize)
  in
  if overflowed =. (y >. (cast (impl_17__MAX <: isize) <: usize) <: bool)
  then Option_Some result <: t_Option isize
  else Option_None <: t_Option isize

/// See [`std::primitive::i8::checked_sub_unsigned`] (and similar for other signed integer types)
let impl_17__checked_sub_unsigned (x: isize) (y: usize) : t_Option isize =
  let (result: isize), (overflowed: bool) =
    impl_17__overflowing_sub x (cast (y <: usize) <: isize)
  in
  if overflowed =. (y >. (cast (impl_17__MAX <: isize) <: usize) <: bool)
  then Option_Some result <: t_Option isize
  else Option_None <: t_Option isize

/// See [`std::primitive::u8::checked_mul`] (and similar for other integer types)
let impl_17__checked_mul (x y: isize) : t_Option isize =
  let (result: isize), (overflowed: bool) = impl_17__overflowing_mul x y in
  if overflowed then Option_None <: t_Option isize else Option_Some result <: t_Option isize

/// See [`std::primitive::i8::checked_div`] (and similar for other signed integer types)
let impl_17__checked_div (x y: isize) : t_Option isize =
  if y =. mk_isize 0 || x =. impl_17__MIN && y =. mk_isize (-1)
  then Option_None <: t_Option isize
  else Option_Some (x /! y) <: t_Option isize

/// See [`std::primitive::i8::checked_rem`] (and similar for other signed integer types)
let impl_17__checked_rem (x y: isize) : t_Option isize =
  if y =. mk_isize 0 || x =. impl_17__MIN && y =. mk_isize (-1)
  then Option_None <: t_Option isize
  else Option_Some (x %! y) <: t_Option isize

/// See [`std::option::Option::is_some_and`]
let impl__is_some_and
      (#v_T #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Ops.Function.t_FnOnce v_F v_T)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == bool})
      (self: t_Option v_T)
      (f: v_F)
    : bool =
  match self <: t_Option v_T with
  | Option_None  -> false
  | Option_Some x ->
    Core_models.Ops.Function.f_call_once #v_F #v_T #FStar.Tactics.Typeclasses.solve f (x <: v_T)

/// See [`std::option::Option::is_none_or`]
let impl__is_none_or
      (#v_T #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Ops.Function.t_FnOnce v_F v_T)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == bool})
      (self: t_Option v_T)
      (f: v_F)
    : bool =
  match self <: t_Option v_T with
  | Option_None  -> true
  | Option_Some x ->
    Core_models.Ops.Function.f_call_once #v_F #v_T #FStar.Tactics.Typeclasses.solve f (x <: v_T)

/// See [`std::option::Option::as_ref`]
let impl__as_ref (#v_T: Type0) (self: t_Option v_T) : t_Option v_T =
  match self <: t_Option v_T with
  | Option_Some x -> Option_Some x <: t_Option v_T
  | Option_None  -> Option_None <: t_Option v_T

/// See [`std::option::Option::unwrap_or`]
let impl__unwrap_or (#v_T: Type0) (self: t_Option v_T) (v_default: v_T) : v_T =
  match self <: t_Option v_T with
  | Option_Some x -> x
  | Option_None  -> v_default

/// See [`std::option::Option::unwrap_or_else`]
let impl__unwrap_or_else
      (#v_T #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Core_models.Ops.Function.t_FnOnce v_F Prims.unit)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == v_T})
      (self: t_Option v_T)
      (f: v_F)
    : v_T =
  match self <: t_Option v_T with
  | Option_Some x -> x
  | Option_None  ->
    Core_models.Ops.Function.f_call_once #v_F
      #Prims.unit
      #FStar.Tactics.Typeclasses.solve
      f
      (() <: Prims.unit)

/// See [`std::option::Option::unwrap_or_default`]
let impl__unwrap_or_default
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Default.t_Default v_T)
      (self: t_Option v_T)
    : v_T =
  match self <: t_Option v_T with
  | Option_Some x -> x
  | Option_None  -> Core_models.Default.f_default #v_T #FStar.Tactics.Typeclasses.solve ()

/// See [`std::option::Option::map`]
let impl__map
      (#v_T #v_U #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Ops.Function.t_FnOnce v_F v_T)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == v_U})
      (self: t_Option v_T)
      (f: v_F)
    : t_Option v_U =
  match self <: t_Option v_T with
  | Option_Some x ->
    Option_Some
    (Core_models.Ops.Function.f_call_once #v_F #v_T #FStar.Tactics.Typeclasses.solve f (x <: v_T))
    <:
    t_Option v_U
  | Option_None  -> Option_None <: t_Option v_U

/// See [`std::option::Option::map_or`]
let impl__map_or
      (#v_T #v_U #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Ops.Function.t_FnOnce v_F v_T)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == v_U})
      (self: t_Option v_T)
      (v_default: v_U)
      (f: v_F)
    : v_U =
  match self <: t_Option v_T with
  | Option_Some t ->
    Core_models.Ops.Function.f_call_once #v_F #v_T #FStar.Tactics.Typeclasses.solve f (t <: v_T)
  | Option_None  -> v_default

/// See [`std::option::Option::map_or_else`]
let impl__map_or_else
      (#v_T #v_U #v_D #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Ops.Function.t_FnOnce v_F v_T)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnOnce v_D Prims.unit)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == v_U})
      (#_: unit{i1.Core_models.Ops.Function.f_Output == v_U})
      (self: t_Option v_T)
      (v_default: v_D)
      (f: v_F)
    : v_U =
  match self <: t_Option v_T with
  | Option_Some t ->
    Core_models.Ops.Function.f_call_once #v_F #v_T #FStar.Tactics.Typeclasses.solve f (t <: v_T)
  | Option_None  ->
    Core_models.Ops.Function.f_call_once #v_D
      #Prims.unit
      #FStar.Tactics.Typeclasses.solve
      v_default
      (() <: Prims.unit)

/// See [`std::option::Option::map_or_default`]
let impl__map_or_default
      (#v_T #v_U #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Ops.Function.t_FnOnce v_F v_T)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: Core_models.Default.t_Default v_U)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == v_U})
      (self: t_Option v_T)
      (f: v_F)
    : v_U =
  match self <: t_Option v_T with
  | Option_Some t ->
    Core_models.Ops.Function.f_call_once #v_F #v_T #FStar.Tactics.Typeclasses.solve f (t <: v_T)
  | Option_None  -> Core_models.Default.f_default #v_U #FStar.Tactics.Typeclasses.solve ()

/// See [`std::option::Option::and_then`]
let impl__and_then
      (#v_T #v_U #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Ops.Function.t_FnOnce v_F v_T)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == t_Option v_U})
      (self: t_Option v_T)
      (f: v_F)
    : t_Option v_U =
  match self <: t_Option v_T with
  | Option_Some x ->
    Core_models.Ops.Function.f_call_once #v_F #v_T #FStar.Tactics.Typeclasses.solve f (x <: v_T)
  | Option_None  -> Option_None <: t_Option v_U

/// See [`std::option::Option::take`]
/// Note: The interface in Rust is wrong, but is good after extraction.
/// We cannot make a useful model with the right interface so we lose the executability.
let impl__take (#v_T: Type0) (self: t_Option v_T) : (t_Option v_T & t_Option v_T) =
  (Option_None <: t_Option v_T), self <: (t_Option v_T & t_Option v_T)

/// See [`std::option::Option::filter`]
assume
val impl__filter':
    #v_T: Type0 ->
    #v_P: Type0 ->
    {| i0: Core_models.Ops.Function.t_FnOnce v_P v_T |} ->
    #_: unit{i0.Core_models.Ops.Function.f_Output == bool} ->
    self: t_Option v_T ->
    predicate: v_P
  -> t_Option v_T

unfold
let impl__filter
      (#v_T #v_P: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Ops.Function.t_FnOnce v_P v_T)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == bool})
     = impl__filter' #v_T #v_P #i0 #_

/// See [`std::option::Option::or`]
let impl__or (#v_T: Type0) (self optb: t_Option v_T) : t_Option v_T =
  match self <: t_Option v_T with
  | Option_Some x -> Option_Some x <: t_Option v_T
  | Option_None  -> optb

/// See [`std::option::Option::or_else`]
let impl__or_else
      (#v_T #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Core_models.Ops.Function.t_FnOnce v_F Prims.unit)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == t_Option v_T})
      (self: t_Option v_T)
      (f: v_F)
    : t_Option v_T =
  match self <: t_Option v_T with
  | Option_Some x -> Option_Some x <: t_Option v_T
  | Option_None  ->
    Core_models.Ops.Function.f_call_once #v_F
      #Prims.unit
      #FStar.Tactics.Typeclasses.solve
      f
      (() <: Prims.unit)

/// See [`std::option::Option::xor`]
let impl__xor (#v_T: Type0) (self optb: t_Option v_T) : t_Option v_T =
  match self, optb <: (t_Option v_T & t_Option v_T) with
  | Option_Some a, Option_None  -> Option_Some a <: t_Option v_T
  | Option_None , Option_Some b -> Option_Some b <: t_Option v_T
  | _ -> Option_None <: t_Option v_T

/// See [`std::option::Option::zip`]
let impl__zip (#v_T #v_U: Type0) (self: t_Option v_T) (other: t_Option v_U) : t_Option (v_T & v_U) =
  match self, other <: (t_Option v_T & t_Option v_U) with
  | Option_Some a, Option_Some b -> Option_Some (a, b <: (v_T & v_U)) <: t_Option (v_T & v_U)
  | _ -> Option_None <: t_Option (v_T & v_U)

/// See [`std::option::Option::inspect`]
let impl__inspect
      (#v_T #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Ops.Function.t_FnOnce v_F v_T)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == Prims.unit})
      (self: t_Option v_T)
      (f: v_F)
    : t_Option v_T =
  let _:Prims.unit =
    match self <: t_Option v_T with
    | Option_Some x ->
      let _:Prims.unit =
        Core_models.Ops.Function.f_call_once #v_F #v_T #FStar.Tactics.Typeclasses.solve f (x <: v_T)
      in
      ()
    | _ -> ()
  in
  self

/// See [`std::option::Option::is_some`]
let impl__is_some (#v_T: Type0) (self: t_Option v_T)
    : Prims.Pure bool
      Prims.l_True
      (ensures
        fun res ->
          let res:bool = res in
          b2t res ==> Option_Some? self) =
  match self <: t_Option v_T with
  | Option_Some _ -> true
  | _ -> false

/// See [`std::option::Option::is_none`]
let impl__is_none (#v_T: Type0) (self: t_Option v_T) : bool =
  (impl__is_some #v_T self <: bool) =. false

/// See [`std::option::Option::expect`]
let impl__expect (#v_T: Type0) (self: t_Option v_T) (e_msg: string)
    : Prims.Pure v_T (requires impl__is_some #v_T self) (fun _ -> Prims.l_True) =
  match self <: t_Option v_T with
  | Option_Some v_val -> v_val
  | Option_None  -> Core_models.Panicking.Internal.panic #v_T ()

/// See [`std::option::Option::unwrap`]
let impl__unwrap (#v_T: Type0) (self: t_Option v_T)
    : Prims.Pure v_T (requires impl__is_some #v_T self) (fun _ -> Prims.l_True) =
  match self <: t_Option v_T with
  | Option_Some v_val -> v_val
  | Option_None  -> Core_models.Panicking.Internal.panic #v_T ()

/// See [`std::option::Option::flatten`]
let impl_1__flatten (#v_T: Type0) (self: t_Option (t_Option v_T)) : t_Option v_T =
  match self <: t_Option (t_Option v_T) with
  | Option_Some inner -> inner
  | Option_None  -> Option_None <: t_Option v_T

/// See [`std::option::Option::cloned`]
let impl_2__cloned
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Clone.t_Clone v_T)
      (self: t_Option v_T)
    : t_Option v_T =
  match self <: t_Option v_T with
  | Option_Some t ->
    Option_Some (Core_models.Clone.f_clone #v_T #FStar.Tactics.Typeclasses.solve t) <: t_Option v_T
  | Option_None  -> Option_None <: t_Option v_T

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_3__from__option (#v_T: Type0) : Core_models.Default.t_Default (t_Option v_T) =
  {
    f_default_pre = (fun (_: Prims.unit) -> true);
    f_default_post = (fun (_: Prims.unit) (out: t_Option v_T) -> true);
    f_default = fun (_: Prims.unit) -> Option_None <: t_Option v_T
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_5 (#v_T: Type0) : Core_models.Ops.Try_trait.t_Try (t_Option v_T) =
  {
    f_Output = v_T;
    f_Residual = t_Option t_Infallible;
    f_from_output_pre = (fun (output: v_T) -> true);
    f_from_output_post = (fun (output: v_T) (out: t_Option v_T) -> true);
    f_from_output = (fun (output: v_T) -> Option_Some output <: t_Option v_T);
    f_branch_pre = (fun (self: t_Option v_T) -> true);
    f_branch_post
    =
    (fun
        (self: t_Option v_T)
        (out: Core_models.Ops.Control_flow.t_ControlFlow (t_Option t_Infallible) v_T)
        ->
        true);
    f_branch
    =
    fun (self: t_Option v_T) ->
      match self <: t_Option v_T with
      | Option_Some v ->
        Core_models.Ops.Control_flow.ControlFlow_Continue v
        <:
        Core_models.Ops.Control_flow.t_ControlFlow (t_Option t_Infallible) v_T
      | Option_None  ->
        Core_models.Ops.Control_flow.ControlFlow_Break (Option_None <: t_Option t_Infallible)
        <:
        Core_models.Ops.Control_flow.t_ControlFlow (t_Option t_Infallible) v_T
  }

/// The `None` half of `?` on `Option`: rebuild `None` at the target type. The
/// residual carries `Infallible`, so the `Some` arm is unreachable.
[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_6__from__option': #v_T: Type0
  -> Core_models.Ops.Try_trait.t_FromResidual (t_Option v_T) (t_Option t_Infallible)

unfold
let impl_6__from__option (#v_T: Type0) = impl_6__from__option' #v_T

/// See [`std::result::Result`]
type t_Result (v_T: Type0) (v_E: Type0) =
  | Result_Ok : v_T -> t_Result v_T v_E
  | Result_Err : v_E -> t_Result v_T v_E

/// See [`std::primitive::u8::from_str_radix`] (and similar for other integer types)
assume
val impl_6__from_str_radix': src: string -> radix: u32
  -> Prims.Pure (t_Result u8 Core_models.Num.Error.t_ParseIntError)
      (requires radix >=. mk_u32 2 && radix <=. mk_u32 36)
      (fun _ -> Prims.l_True)

unfold
let impl_6__from_str_radix = impl_6__from_str_radix'

/// See [`std::primitive::u8::from_str_radix`] (and similar for other integer types)
assume
val impl_7__from_str_radix': src: string -> radix: u32
  -> Prims.Pure (t_Result u16 Core_models.Num.Error.t_ParseIntError)
      (requires radix >=. mk_u32 2 && radix <=. mk_u32 36)
      (fun _ -> Prims.l_True)

unfold
let impl_7__from_str_radix = impl_7__from_str_radix'

/// See [`std::primitive::u8::from_str_radix`] (and similar for other integer types)
assume
val impl_8__from_str_radix': src: string -> radix: u32
  -> Prims.Pure (t_Result u32 Core_models.Num.Error.t_ParseIntError)
      (requires radix >=. mk_u32 2 && radix <=. mk_u32 36)
      (fun _ -> Prims.l_True)

unfold
let impl_8__from_str_radix = impl_8__from_str_radix'

/// See [`std::primitive::u8::from_str_radix`] (and similar for other integer types)
assume
val impl_9__from_str_radix': src: string -> radix: u32
  -> Prims.Pure (t_Result u64 Core_models.Num.Error.t_ParseIntError)
      (requires radix >=. mk_u32 2 && radix <=. mk_u32 36)
      (fun _ -> Prims.l_True)

unfold
let impl_9__from_str_radix = impl_9__from_str_radix'

/// See [`std::primitive::u8::from_str_radix`] (and similar for other integer types)
assume
val impl_10__from_str_radix': src: string -> radix: u32
  -> Prims.Pure (t_Result u128 Core_models.Num.Error.t_ParseIntError)
      (requires radix >=. mk_u32 2 && radix <=. mk_u32 36)
      (fun _ -> Prims.l_True)

unfold
let impl_10__from_str_radix = impl_10__from_str_radix'

/// See [`std::primitive::u8::from_str_radix`] (and similar for other integer types)
assume
val impl_11__from_str_radix': src: string -> radix: u32
  -> Prims.Pure (t_Result usize Core_models.Num.Error.t_ParseIntError)
      (requires radix >=. mk_u32 2 && radix <=. mk_u32 36)
      (fun _ -> Prims.l_True)

unfold
let impl_11__from_str_radix = impl_11__from_str_radix'

/// See [`std::primitive::u8::from_str_radix`] (and similar for other integer types)
assume
val impl_12__from_str_radix': src: string -> radix: u32
  -> Prims.Pure (t_Result i8 Core_models.Num.Error.t_ParseIntError)
      (requires radix >=. mk_u32 2 && radix <=. mk_u32 36)
      (fun _ -> Prims.l_True)

unfold
let impl_12__from_str_radix = impl_12__from_str_radix'

/// See [`std::primitive::u8::from_str_radix`] (and similar for other integer types)
assume
val impl_13__from_str_radix': src: string -> radix: u32
  -> Prims.Pure (t_Result i16 Core_models.Num.Error.t_ParseIntError)
      (requires radix >=. mk_u32 2 && radix <=. mk_u32 36)
      (fun _ -> Prims.l_True)

unfold
let impl_13__from_str_radix = impl_13__from_str_radix'

/// See [`std::primitive::u8::from_str_radix`] (and similar for other integer types)
assume
val impl_14__from_str_radix': src: string -> radix: u32
  -> Prims.Pure (t_Result i32 Core_models.Num.Error.t_ParseIntError)
      (requires radix >=. mk_u32 2 && radix <=. mk_u32 36)
      (fun _ -> Prims.l_True)

unfold
let impl_14__from_str_radix = impl_14__from_str_radix'

/// See [`std::primitive::u8::from_str_radix`] (and similar for other integer types)
assume
val impl_15__from_str_radix': src: string -> radix: u32
  -> Prims.Pure (t_Result i64 Core_models.Num.Error.t_ParseIntError)
      (requires radix >=. mk_u32 2 && radix <=. mk_u32 36)
      (fun _ -> Prims.l_True)

unfold
let impl_15__from_str_radix = impl_15__from_str_radix'

/// See [`std::primitive::u8::from_str_radix`] (and similar for other integer types)
assume
val impl_16__from_str_radix': src: string -> radix: u32
  -> Prims.Pure (t_Result i128 Core_models.Num.Error.t_ParseIntError)
      (requires radix >=. mk_u32 2 && radix <=. mk_u32 36)
      (fun _ -> Prims.l_True)

unfold
let impl_16__from_str_radix = impl_16__from_str_radix'

/// See [`std::primitive::u8::from_str_radix`] (and similar for other integer types)
assume
val impl_17__from_str_radix': src: string -> radix: u32
  -> Prims.Pure (t_Result isize Core_models.Num.Error.t_ParseIntError)
      (requires radix >=. mk_u32 2 && radix <=. mk_u32 36)
      (fun _ -> Prims.l_True)

unfold
let impl_17__from_str_radix = impl_17__from_str_radix'

/// See [`std::option::Option::ok_or`]
let impl__ok_or (#v_T #v_E: Type0) (self: t_Option v_T) (err: v_E) : t_Result v_T v_E =
  match self <: t_Option v_T with
  | Option_Some v -> Result_Ok v <: t_Result v_T v_E
  | Option_None  -> Result_Err err <: t_Result v_T v_E

/// See [`std::option::Option::ok_or_else`]
let impl__ok_or_else
      (#v_T #v_E #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Core_models.Ops.Function.t_FnOnce v_F Prims.unit)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == v_E})
      (self: t_Option v_T)
      (err: v_F)
    : t_Result v_T v_E =
  match self <: t_Option v_T with
  | Option_Some v -> Result_Ok v <: t_Result v_T v_E
  | Option_None  ->
    Result_Err
    (Core_models.Ops.Function.f_call_once #v_F
        #Prims.unit
        #FStar.Tactics.Typeclasses.solve
        err
        (() <: Prims.unit))
    <:
    t_Result v_T v_E

/// See [`std::result::Result::is_ok`]
let impl__is_ok (#v_T #v_E: Type0) (self: t_Result v_T v_E) : bool =
  match self <: t_Result v_T v_E with
  | Result_Ok _ -> true
  | _ -> false

/// See [`std::result::Result::is_ok_and`]
let impl__is_ok_and
      (#v_T #v_E #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Ops.Function.t_FnOnce v_F v_T)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == bool})
      (self: t_Result v_T v_E)
      (f: v_F)
    : bool =
  match self <: t_Result v_T v_E with
  | Result_Ok t ->
    Core_models.Ops.Function.f_call_once #v_F #v_T #FStar.Tactics.Typeclasses.solve f (t <: v_T)
  | Result_Err _ -> false

/// See [`std::result::Result::is_err`]
let impl__is_err (#v_T #v_E: Type0) (self: t_Result v_T v_E) : bool =
  ~.(impl__is_ok #v_T #v_E self <: bool)

/// See [`std::result::Result::is_err_and`]
let impl__is_err_and
      (#v_T #v_E #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Ops.Function.t_FnOnce v_F v_E)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == bool})
      (self: t_Result v_T v_E)
      (f: v_F)
    : bool =
  match self <: t_Result v_T v_E with
  | Result_Ok _ -> false
  | Result_Err e ->
    Core_models.Ops.Function.f_call_once #v_F #v_E #FStar.Tactics.Typeclasses.solve f (e <: v_E)

/// See [`std::result::Result::as_ref`]
let impl__as_ref__from__result (#v_T #v_E: Type0) (self: t_Result v_T v_E) : t_Result v_T v_E =
  match self <: t_Result v_T v_E with
  | Result_Ok t -> Result_Ok t <: t_Result v_T v_E
  | Result_Err e -> Result_Err e <: t_Result v_T v_E

/// See [`std::result::Result::unwrap_or_else`]
let impl__unwrap_or_else__from__result
      (#v_T #v_E #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Ops.Function.t_FnOnce v_F v_E)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == v_T})
      (self: t_Result v_T v_E)
      (op: v_F)
    : v_T =
  match self <: t_Result v_T v_E with
  | Result_Ok t -> t
  | Result_Err e ->
    Core_models.Ops.Function.f_call_once #v_F #v_E #FStar.Tactics.Typeclasses.solve op (e <: v_E)

/// See [`std::result::Result::unwrap_or_default`]
let impl__unwrap_or_default__from__result
      (#v_T #v_E: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Default.t_Default v_T)
      (self: t_Result v_T v_E)
    : v_T =
  match self <: t_Result v_T v_E with
  | Result_Ok t -> t
  | Result_Err _ -> Core_models.Default.f_default #v_T #FStar.Tactics.Typeclasses.solve ()

/// See [`std::result::Result::map`]
let impl__map__from__result
      (#v_T #v_E #v_U #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Ops.Function.t_FnOnce v_F v_T)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == v_U})
      (self: t_Result v_T v_E)
      (op: v_F)
    : t_Result v_U v_E =
  match self <: t_Result v_T v_E with
  | Result_Ok t ->
    Result_Ok
    (Core_models.Ops.Function.f_call_once #v_F #v_T #FStar.Tactics.Typeclasses.solve op (t <: v_T))
    <:
    t_Result v_U v_E
  | Result_Err e -> Result_Err e <: t_Result v_U v_E

/// See [`std::result::Result::map_or`]
let impl__map_or__from__result
      (#v_T #v_E #v_U #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Ops.Function.t_FnOnce v_F v_T)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == v_U})
      (self: t_Result v_T v_E)
      (v_default: v_U)
      (f: v_F)
    : v_U =
  match self <: t_Result v_T v_E with
  | Result_Ok t ->
    Core_models.Ops.Function.f_call_once #v_F #v_T #FStar.Tactics.Typeclasses.solve f (t <: v_T)
  | Result_Err _ -> v_default

/// See [`std::result::Result::map_or_else`]
let impl__map_or_else__from__result
      (#v_T #v_E #v_U #v_D #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Ops.Function.t_FnOnce v_D v_E)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: Core_models.Ops.Function.t_FnOnce v_F v_T)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == v_U})
      (#_: unit{i1.Core_models.Ops.Function.f_Output == v_U})
      (self: t_Result v_T v_E)
      (v_default: v_D)
      (f: v_F)
    : v_U =
  match self <: t_Result v_T v_E with
  | Result_Ok t ->
    Core_models.Ops.Function.f_call_once #v_F #v_T #FStar.Tactics.Typeclasses.solve f (t <: v_T)
  | Result_Err e ->
    Core_models.Ops.Function.f_call_once #v_D
      #v_E
      #FStar.Tactics.Typeclasses.solve
      v_default
      (e <: v_E)

/// See [`std::result::Result::map_or_default`]
let impl__map_or_default__from__result
      (#v_T #v_E #v_U #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Ops.Function.t_FnOnce v_F v_T)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: Core_models.Default.t_Default v_U)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == v_U})
      (self: t_Result v_T v_E)
      (f: v_F)
    : v_U =
  match self <: t_Result v_T v_E with
  | Result_Ok t ->
    Core_models.Ops.Function.f_call_once #v_F #v_T #FStar.Tactics.Typeclasses.solve f (t <: v_T)
  | Result_Err _ -> Core_models.Default.f_default #v_U #FStar.Tactics.Typeclasses.solve ()

/// See [`std::result::Result::inspect`]
let impl__inspect__from__result
      (#v_T #v_E #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Ops.Function.t_FnOnce v_F v_T)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == Prims.unit})
      (self: t_Result v_T v_E)
      (f: v_F)
    : t_Result v_T v_E =
  let _:Prims.unit =
    match self <: t_Result v_T v_E with
    | Result_Ok t ->
      let _:Prims.unit =
        Core_models.Ops.Function.f_call_once #v_F #v_T #FStar.Tactics.Typeclasses.solve f (t <: v_T)
      in
      ()
    | _ -> ()
  in
  self

/// See [`std::result::Result::inspect_err`]
let impl__inspect_err
      (#v_T #v_E #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Ops.Function.t_FnOnce v_F v_E)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == Prims.unit})
      (self: t_Result v_T v_E)
      (f: v_F)
    : t_Result v_T v_E =
  let _:Prims.unit =
    match self <: t_Result v_T v_E with
    | Result_Err e ->
      let _:Prims.unit =
        Core_models.Ops.Function.f_call_once #v_F #v_E #FStar.Tactics.Typeclasses.solve f (e <: v_E)
      in
      ()
    | _ -> ()
  in
  self

/// See [`std::result::Result::ok`]
let impl__ok (#v_T #v_E: Type0) (self: t_Result v_T v_E) : t_Option v_T =
  match self <: t_Result v_T v_E with
  | Result_Ok x -> Option_Some x <: t_Option v_T
  | Result_Err _ -> Option_None <: t_Option v_T

/// See [`std::result::Result::err`]
let impl__err (#v_T #v_E: Type0) (self: t_Result v_T v_E) : t_Option v_E =
  match self <: t_Result v_T v_E with
  | Result_Ok _ -> Option_None <: t_Option v_E
  | Result_Err e -> Option_Some e <: t_Option v_E

/// See [`std::result::Result::and`]
let impl__and (#v_T #v_E #v_U: Type0) (self: t_Result v_T v_E) (res: t_Result v_U v_E)
    : t_Result v_U v_E =
  match self <: t_Result v_T v_E with
  | Result_Ok _ -> res
  | Result_Err e -> Result_Err e <: t_Result v_U v_E

/// See [`std::result::Result::and_then`]
let impl__and_then__from__result
      (#v_T #v_E #v_U #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Ops.Function.t_FnOnce v_F v_T)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == t_Result v_U v_E})
      (self: t_Result v_T v_E)
      (op: v_F)
    : t_Result v_U v_E =
  match self <: t_Result v_T v_E with
  | Result_Ok t ->
    Core_models.Ops.Function.f_call_once #v_F #v_T #FStar.Tactics.Typeclasses.solve op (t <: v_T)
  | Result_Err e -> Result_Err e <: t_Result v_U v_E

/// See [`std::result::Result::or`]
let impl__or__from__result (#v_T #v_E #v_F: Type0) (self: t_Result v_T v_E) (res: t_Result v_T v_F)
    : t_Result v_T v_F =
  match self <: t_Result v_T v_E with
  | Result_Ok t -> Result_Ok t <: t_Result v_T v_F
  | Result_Err _ -> res

/// See [`std::result::Result::or_else`]
let impl__or_else__from__result
      (#v_T #v_E #v_F #v_O: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Ops.Function.t_FnOnce v_O v_E)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == t_Result v_T v_F})
      (self: t_Result v_T v_E)
      (op: v_O)
    : t_Result v_T v_F =
  match self <: t_Result v_T v_E with
  | Result_Ok t -> Result_Ok t <: t_Result v_T v_F
  | Result_Err e ->
    Core_models.Ops.Function.f_call_once #v_O #v_E #FStar.Tactics.Typeclasses.solve op (e <: v_E)

/// See [`std::result::Result::unwrap_or`]
let impl__unwrap_or__from__result (#v_T #v_E: Type0) (self: t_Result v_T v_E) (v_default: v_T) : v_T =
  match self <: t_Result v_T v_E with
  | Result_Ok t -> t
  | Result_Err _ -> v_default

/// See [`std::result::Result::map_err`]
let impl__map_err
      (#v_T #v_E #v_F #v_O: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Ops.Function.t_FnOnce v_O v_E)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == v_F})
      (self: t_Result v_T v_E)
      (op: v_O)
    : t_Result v_T v_F =
  match self <: t_Result v_T v_E with
  | Result_Ok t -> Result_Ok t <: t_Result v_T v_F
  | Result_Err e ->
    Result_Err
    (Core_models.Ops.Function.f_call_once #v_O #v_E #FStar.Tactics.Typeclasses.solve op (e <: v_E))
    <:
    t_Result v_T v_F

/// See [`std::result::Result::expect`]
let impl__expect__from__result (#v_T #v_E: Type0) (self: t_Result v_T v_E) (e_msg: string)
    : Prims.Pure v_T (requires impl__is_ok #v_T #v_E self) (fun _ -> Prims.l_True) =
  match self <: t_Result v_T v_E with
  | Result_Ok t -> t
  | Result_Err _ -> Core_models.Panicking.Internal.panic #v_T ()

/// See [`std::result::Result::unwrap`]
let impl__unwrap__from__result (#v_T #v_E: Type0) (self: t_Result v_T v_E)
    : Prims.Pure v_T (requires impl__is_ok #v_T #v_E self) (fun _ -> Prims.l_True) =
  match self <: t_Result v_T v_E with
  | Result_Ok t -> t
  | Result_Err _ -> Core_models.Panicking.Internal.panic #v_T ()

/// See [`std::result::Result::expect_err`]
let impl__expect_err (#v_T #v_E: Type0) (self: t_Result v_T v_E) (e_msg: string)
    : Prims.Pure v_E (requires impl__is_err #v_T #v_E self) (fun _ -> Prims.l_True) =
  match self <: t_Result v_T v_E with
  | Result_Ok _ -> Core_models.Panicking.Internal.panic #v_E ()
  | Result_Err e -> e

/// See [`std::result::Result::unwrap_err`]
let impl__unwrap_err (#v_T #v_E: Type0) (self: t_Result v_T v_E)
    : Prims.Pure v_E (requires impl__is_err #v_T #v_E self) (fun _ -> Prims.l_True) =
  match self <: t_Result v_T v_E with
  | Result_Ok _ -> Core_models.Panicking.Internal.panic #v_E ()
  | Result_Err e -> e

/// See [`std::result::Result::cloned`]
let impl_1__cloned
      (#v_T #v_E: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Clone.t_Clone v_T)
      (self: t_Result v_T v_E)
    : t_Result v_T v_E =
  match self <: t_Result v_T v_E with
  | Result_Ok t ->
    Result_Ok (Core_models.Clone.f_clone #v_T #FStar.Tactics.Typeclasses.solve t)
    <:
    t_Result v_T v_E
  | Result_Err e -> Result_Err e <: t_Result v_T v_E

/// See [`std::result::Result::transpose`]
let impl_2__transpose (#v_T #v_E: Type0) (self: t_Result (t_Option v_T) v_E)
    : t_Option (t_Result v_T v_E) =
  match self <: t_Result (t_Option v_T) v_E with
  | Result_Ok (Option_Some t) ->
    Option_Some (Result_Ok t <: t_Result v_T v_E) <: t_Option (t_Result v_T v_E)
  | Result_Ok (Option_None ) -> Option_None <: t_Option (t_Result v_T v_E)
  | Result_Err e -> Option_Some (Result_Err e <: t_Result v_T v_E) <: t_Option (t_Result v_T v_E)

/// See [`std::result::Result::flatten`]
let impl_3__flatten (#v_T #v_E: Type0) (self: t_Result (t_Result v_T v_E) v_E) : t_Result v_T v_E =
  match self <: t_Result (t_Result v_T v_E) v_E with
  | Result_Ok inner -> inner
  | Result_Err e -> Result_Err e <: t_Result v_T v_E

/// Yields a `Seq` by value, for the `Result` shunt below: `V::from_iter` needs
/// an `IntoIterator<Item = A>`, and `slice::Iter` only yields references.
type t_SeqIter (v_A: Type0) = | SeqIter : Rust_primitives.Sequence.t_Seq v_A -> t_SeqIter v_A

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_6__from__result (#v_T #v_E: Type0) : Core_models.Ops.Try_trait.t_Try (t_Result v_T v_E) =
  {
    f_Output = v_T;
    f_Residual = t_Result t_Infallible v_E;
    f_from_output_pre = (fun (output: v_T) -> true);
    f_from_output_post = (fun (output: v_T) (out: t_Result v_T v_E) -> true);
    f_from_output = (fun (output: v_T) -> Result_Ok output <: t_Result v_T v_E);
    f_branch_pre = (fun (self: t_Result v_T v_E) -> true);
    f_branch_post
    =
    (fun
        (self: t_Result v_T v_E)
        (out: Core_models.Ops.Control_flow.t_ControlFlow (t_Result t_Infallible v_E) v_T)
        ->
        true);
    f_branch
    =
    fun (self: t_Result v_T v_E) ->
      match self <: t_Result v_T v_E with
      | Result_Ok v ->
        Core_models.Ops.Control_flow.ControlFlow_Continue v
        <:
        Core_models.Ops.Control_flow.t_ControlFlow (t_Result t_Infallible v_E) v_T
      | Result_Err e ->
        Core_models.Ops.Control_flow.ControlFlow_Break (Result_Err e <: t_Result t_Infallible v_E)
        <:
        Core_models.Ops.Control_flow.t_ControlFlow (t_Result t_Infallible v_E) v_T
  }

/// See [`std::cmp::PartialEq`]
class t_PartialEq (v_Self: Type0) (v_Rhs: Type0) = {
  f_eq_pre:self_: v_Self -> other: v_Rhs -> pred: Type0{true ==> pred};
  f_eq_post:v_Self -> v_Rhs -> bool -> Type0;
  f_eq:x0: v_Self -> x1: v_Rhs
    -> Prims.Pure bool (f_eq_pre x0 x1) (fun result -> f_eq_post x0 x1 result)
}

class t_Neq (v_Self: Type0) (v_Rhs: Type0) = {
  f_neq_pre:self_: v_Self -> y: v_Rhs -> pred: Type0{true ==> pred};
  f_neq_post:v_Self -> v_Rhs -> bool -> Type0;
  f_neq:x0: v_Self -> x1: v_Rhs
    -> Prims.Pure bool (f_neq_pre x0 x1) (fun result -> f_neq_post x0 x1 result)
}

/// See [`std::convert::Into`]
class t_Into (v_Self: Type0) (v_T: Type0) = {
  f_into_pre:self_: v_Self -> pred: Type0{true ==> pred};
  f_into_post:v_Self -> v_T -> Type0;
  f_into:x0: v_Self -> Prims.Pure v_T (f_into_pre x0) (fun result -> f_into_post x0 result)
}

/// See [`std::convert::From`]
class t_From (v_Self: Type0) (v_T: Type0) = {
  f_from_pre:x: v_T -> pred: Type0{true ==> pred};
  f_from_post:v_T -> v_Self -> Type0;
  f_from:x0: v_T -> Prims.Pure v_Self (f_from_pre x0) (fun result -> f_from_post x0 result)
}

/// See [`std::convert::AsRef`]
class t_AsRef (v_Self: Type0) (v_T: Type0) = {
  f_as_ref_pre:self_: v_Self -> pred: Type0{true ==> pred};
  f_as_ref_post:v_Self -> v_T -> Type0;
  f_as_ref:x0: v_Self -> Prims.Pure v_T (f_as_ref_pre x0) (fun result -> f_as_ref_post x0 result)
}

/// See [`std::convert::AsRef`] for `[T; N]`
[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_26 (#v_T: Type0) (v_N: usize) : t_AsRef (t_Array v_T v_N) (t_Slice v_T) =
  {
    f_as_ref_pre = (fun (self: t_Array v_T v_N) -> true);
    f_as_ref_post = (fun (self: t_Array v_T v_N) (out: t_Slice v_T) -> true);
    f_as_ref = fun (self: t_Array v_T v_N) -> Rust_primitives.Slice.array_as_slice #v_T v_N self
  }

/// See [`std::cmp::Eq`]
class t_Eq (v_Self: Type0) = {
  [@@@ FStar.Tactics.Typeclasses.no_method]_super_i0:t_PartialEq v_Self v_Self
}

[@@ FStar.Tactics.Typeclasses.tcinstance]
let _ = fun (v_Self:Type0) {|i: t_Eq v_Self|} -> i._super_i0

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl__from__cmp
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_PartialEq v_T v_T)
    : t_Neq v_T v_T =
  {
    f_neq_pre = (fun (self: v_T) (y: v_T) -> true);
    f_neq_post = (fun (self: v_T) (y: v_T) (out: bool) -> true);
    f_neq
    =
    fun (self: v_T) (y: v_T) ->
      (f_eq #v_T #v_T #FStar.Tactics.Typeclasses.solve self y <: bool) =. false
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_3 (#v_T: Type0) (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_PartialEq v_T v_T)
    : t_PartialEq (t_Reverse v_T) (t_Reverse v_T) =
  {
    f_eq_pre = (fun (self: t_Reverse v_T) (other: t_Reverse v_T) -> true);
    f_eq_post = (fun (self: t_Reverse v_T) (other: t_Reverse v_T) (out: bool) -> true);
    f_eq
    =
    fun (self: t_Reverse v_T) (other: t_Reverse v_T) ->
      f_eq #v_T #v_T #FStar.Tactics.Typeclasses.solve other._0 self._0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_4 (#v_T: Type0) (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Eq v_T)
    : t_Eq (t_Reverse v_T) = { _super_i0 = FStar.Tactics.Typeclasses.solve }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_10: t_PartialEq u8 u8 =
  {
    f_eq_pre = (fun (self: u8) (other: u8) -> true);
    f_eq_post = (fun (self: u8) (other: u8) (out: bool) -> true);
    f_eq = fun (self: u8) (other: u8) -> self =. other
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_11: t_Eq u8 = { _super_i0 = FStar.Tactics.Typeclasses.solve }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_12: t_PartialEq i8 i8 =
  {
    f_eq_pre = (fun (self: i8) (other: i8) -> true);
    f_eq_post = (fun (self: i8) (other: i8) (out: bool) -> true);
    f_eq = fun (self: i8) (other: i8) -> self =. other
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_13: t_Eq i8 = { _super_i0 = FStar.Tactics.Typeclasses.solve }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_14: t_PartialEq u16 u16 =
  {
    f_eq_pre = (fun (self: u16) (other: u16) -> true);
    f_eq_post = (fun (self: u16) (other: u16) (out: bool) -> true);
    f_eq = fun (self: u16) (other: u16) -> self =. other
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_15: t_Eq u16 = { _super_i0 = FStar.Tactics.Typeclasses.solve }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_16: t_PartialEq i16 i16 =
  {
    f_eq_pre = (fun (self: i16) (other: i16) -> true);
    f_eq_post = (fun (self: i16) (other: i16) (out: bool) -> true);
    f_eq = fun (self: i16) (other: i16) -> self =. other
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_17: t_Eq i16 = { _super_i0 = FStar.Tactics.Typeclasses.solve }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_18: t_PartialEq u32 u32 =
  {
    f_eq_pre = (fun (self: u32) (other: u32) -> true);
    f_eq_post = (fun (self: u32) (other: u32) (out: bool) -> true);
    f_eq = fun (self: u32) (other: u32) -> self =. other
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_19: t_Eq u32 = { _super_i0 = FStar.Tactics.Typeclasses.solve }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_20: t_PartialEq i32 i32 =
  {
    f_eq_pre = (fun (self: i32) (other: i32) -> true);
    f_eq_post = (fun (self: i32) (other: i32) (out: bool) -> true);
    f_eq = fun (self: i32) (other: i32) -> self =. other
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_21: t_Eq i32 = { _super_i0 = FStar.Tactics.Typeclasses.solve }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_22: t_PartialEq u64 u64 =
  {
    f_eq_pre = (fun (self: u64) (other: u64) -> true);
    f_eq_post = (fun (self: u64) (other: u64) (out: bool) -> true);
    f_eq = fun (self: u64) (other: u64) -> self =. other
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_23: t_Eq u64 = { _super_i0 = FStar.Tactics.Typeclasses.solve }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_24__from__cmp: t_PartialEq i64 i64 =
  {
    f_eq_pre = (fun (self: i64) (other: i64) -> true);
    f_eq_post = (fun (self: i64) (other: i64) (out: bool) -> true);
    f_eq = fun (self: i64) (other: i64) -> self =. other
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_25__from__cmp: t_Eq i64 = { _super_i0 = FStar.Tactics.Typeclasses.solve }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_26__from__cmp: t_PartialEq u128 u128 =
  {
    f_eq_pre = (fun (self: u128) (other: u128) -> true);
    f_eq_post = (fun (self: u128) (other: u128) (out: bool) -> true);
    f_eq = fun (self: u128) (other: u128) -> self =. other
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_27__from__cmp: t_Eq u128 = { _super_i0 = FStar.Tactics.Typeclasses.solve }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_28__from__cmp: t_PartialEq i128 i128 =
  {
    f_eq_pre = (fun (self: i128) (other: i128) -> true);
    f_eq_post = (fun (self: i128) (other: i128) (out: bool) -> true);
    f_eq = fun (self: i128) (other: i128) -> self =. other
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_29__from__cmp: t_Eq i128 = { _super_i0 = FStar.Tactics.Typeclasses.solve }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_30__from__cmp: t_PartialEq usize usize =
  {
    f_eq_pre = (fun (self: usize) (other: usize) -> true);
    f_eq_post = (fun (self: usize) (other: usize) (out: bool) -> true);
    f_eq = fun (self: usize) (other: usize) -> self =. other
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_31__from__cmp: t_Eq usize = { _super_i0 = FStar.Tactics.Typeclasses.solve }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_32: t_PartialEq isize isize =
  {
    f_eq_pre = (fun (self: isize) (other: isize) -> true);
    f_eq_post = (fun (self: isize) (other: isize) (out: bool) -> true);
    f_eq = fun (self: isize) (other: isize) -> self =. other
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_33: t_Eq isize = { _super_i0 = FStar.Tactics.Typeclasses.solve }

/// See [`std::cmp::PartialEq`] for `()`
[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_6: t_PartialEq Prims.unit Prims.unit =
  {
    f_eq_pre = (fun (self: Prims.unit) (other: Prims.unit) -> true);
    f_eq_post = (fun (self: Prims.unit) (other: Prims.unit) (out: bool) -> true);
    f_eq = fun (self: Prims.unit) (other: Prims.unit) -> true
  }

/// See [`std::cmp::Eq`] for `()`
[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_7: t_Eq Prims.unit = { _super_i0 = FStar.Tactics.Typeclasses.solve }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl__from__convert
      (#v_T #v_U: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_From v_U v_T)
    : t_Into v_T v_U =
  {
    f_into_pre = (fun (self: v_T) -> true);
    f_into_post = (fun (self: v_T) (out: v_U) -> true);
    f_into = fun (self: v_T) -> f_from #v_U #v_T #FStar.Tactics.Typeclasses.solve self
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_4__from__convert (#v_T: Type0) : t_From v_T v_T =
  {
    f_from_pre = (fun (x: v_T) -> true);
    f_from_post = (fun (x: v_T) (out: v_T) -> true);
    f_from = fun (x: v_T) -> x
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_5__from__convert (#v_T: Type0) : t_AsRef (t_Slice v_T) (t_Slice v_T) =
  {
    f_as_ref_pre = (fun (self: t_Slice v_T) -> true);
    f_as_ref_post = (fun (self: t_Slice v_T) (out: t_Slice v_T) -> true);
    f_as_ref = fun (self: t_Slice v_T) -> self
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_6__from__convert: t_From u16 u8 =
  {
    f_from_pre = (fun (x: u8) -> true);
    f_from_post = (fun (x: u8) (out: u16) -> true);
    f_from = fun (x: u8) -> cast (x <: u8) <: u16
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_7__from__convert: t_From u32 u8 =
  {
    f_from_pre = (fun (x: u8) -> true);
    f_from_post = (fun (x: u8) (out: u32) -> true);
    f_from = fun (x: u8) -> cast (x <: u8) <: u32
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_8__from__convert: t_From u32 u16 =
  {
    f_from_pre = (fun (x: u16) -> true);
    f_from_post = (fun (x: u16) (out: u32) -> true);
    f_from = fun (x: u16) -> cast (x <: u16) <: u32
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_9__from__convert: t_From u64 u8 =
  {
    f_from_pre = (fun (x: u8) -> true);
    f_from_post = (fun (x: u8) (out: u64) -> true);
    f_from = fun (x: u8) -> cast (x <: u8) <: u64
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_10__from__convert: t_From u64 u16 =
  {
    f_from_pre = (fun (x: u16) -> true);
    f_from_post = (fun (x: u16) (out: u64) -> true);
    f_from = fun (x: u16) -> cast (x <: u16) <: u64
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_11__from__convert: t_From u64 u32 =
  {
    f_from_pre = (fun (x: u32) -> true);
    f_from_post = (fun (x: u32) (out: u64) -> true);
    f_from = fun (x: u32) -> cast (x <: u32) <: u64
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_12__from__convert: t_From u128 u8 =
  {
    f_from_pre = (fun (x: u8) -> true);
    f_from_post = (fun (x: u8) (out: u128) -> true);
    f_from = fun (x: u8) -> cast (x <: u8) <: u128
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_13__from__convert: t_From u128 u16 =
  {
    f_from_pre = (fun (x: u16) -> true);
    f_from_post = (fun (x: u16) (out: u128) -> true);
    f_from = fun (x: u16) -> cast (x <: u16) <: u128
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_14__from__convert: t_From u128 u32 =
  {
    f_from_pre = (fun (x: u32) -> true);
    f_from_post = (fun (x: u32) (out: u128) -> true);
    f_from = fun (x: u32) -> cast (x <: u32) <: u128
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_15__from__convert: t_From u128 u64 =
  {
    f_from_pre = (fun (x: u64) -> true);
    f_from_post = (fun (x: u64) (out: u128) -> true);
    f_from = fun (x: u64) -> cast (x <: u64) <: u128
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_16__from__convert: t_From usize u8 =
  {
    f_from_pre = (fun (x: u8) -> true);
    f_from_post = (fun (x: u8) (out: usize) -> true);
    f_from = fun (x: u8) -> cast (x <: u8) <: usize
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_17__from__convert: t_From usize u16 =
  {
    f_from_pre = (fun (x: u16) -> true);
    f_from_post = (fun (x: u16) (out: usize) -> true);
    f_from = fun (x: u16) -> cast (x <: u16) <: usize
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_18__from__convert: t_From i16 i8 =
  {
    f_from_pre = (fun (x: i8) -> true);
    f_from_post = (fun (x: i8) (out: i16) -> true);
    f_from = fun (x: i8) -> cast (x <: i8) <: i16
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_19__from__convert: t_From i32 i8 =
  {
    f_from_pre = (fun (x: i8) -> true);
    f_from_post = (fun (x: i8) (out: i32) -> true);
    f_from = fun (x: i8) -> cast (x <: i8) <: i32
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_20__from__convert: t_From i32 i16 =
  {
    f_from_pre = (fun (x: i16) -> true);
    f_from_post = (fun (x: i16) (out: i32) -> true);
    f_from = fun (x: i16) -> cast (x <: i16) <: i32
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_21__from__convert: t_From i64 i8 =
  {
    f_from_pre = (fun (x: i8) -> true);
    f_from_post = (fun (x: i8) (out: i64) -> true);
    f_from = fun (x: i8) -> cast (x <: i8) <: i64
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_22__from__convert: t_From i64 i16 =
  {
    f_from_pre = (fun (x: i16) -> true);
    f_from_post = (fun (x: i16) (out: i64) -> true);
    f_from = fun (x: i16) -> cast (x <: i16) <: i64
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_23__from__convert: t_From i64 i32 =
  {
    f_from_pre = (fun (x: i32) -> true);
    f_from_post = (fun (x: i32) (out: i64) -> true);
    f_from = fun (x: i32) -> cast (x <: i32) <: i64
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_24__from__convert: t_From i128 i8 =
  {
    f_from_pre = (fun (x: i8) -> true);
    f_from_post = (fun (x: i8) (out: i128) -> true);
    f_from = fun (x: i8) -> cast (x <: i8) <: i128
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_25__from__convert: t_From i128 i16 =
  {
    f_from_pre = (fun (x: i16) -> true);
    f_from_post = (fun (x: i16) (out: i128) -> true);
    f_from = fun (x: i16) -> cast (x <: i16) <: i128
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_26__from__convert: t_From i128 i32 =
  {
    f_from_pre = (fun (x: i32) -> true);
    f_from_post = (fun (x: i32) (out: i128) -> true);
    f_from = fun (x: i32) -> cast (x <: i32) <: i128
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_27__from__convert: t_From i128 i64 =
  {
    f_from_pre = (fun (x: i64) -> true);
    f_from_post = (fun (x: i64) (out: i128) -> true);
    f_from = fun (x: i64) -> cast (x <: i64) <: i128
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_28__from__convert: t_From isize i8 =
  {
    f_from_pre = (fun (x: i8) -> true);
    f_from_post = (fun (x: i8) (out: isize) -> true);
    f_from = fun (x: i8) -> cast (x <: i8) <: isize
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_29__from__convert: t_From isize i16 =
  {
    f_from_pre = (fun (x: i16) -> true);
    f_from_post = (fun (x: i16) (out: isize) -> true);
    f_from = fun (x: i16) -> cast (x <: i16) <: isize
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_30__from__convert: t_From i16 u8 =
  {
    f_from_pre = (fun (x: u8) -> true);
    f_from_post = (fun (x: u8) (out: i16) -> true);
    f_from = fun (x: u8) -> cast (x <: u8) <: i16
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_31__from__convert: t_From i32 u8 =
  {
    f_from_pre = (fun (x: u8) -> true);
    f_from_post = (fun (x: u8) (out: i32) -> true);
    f_from = fun (x: u8) -> cast (x <: u8) <: i32
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_32__from__convert: t_From i64 u8 =
  {
    f_from_pre = (fun (x: u8) -> true);
    f_from_post = (fun (x: u8) (out: i64) -> true);
    f_from = fun (x: u8) -> cast (x <: u8) <: i64
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_33__from__convert: t_From i128 u8 =
  {
    f_from_pre = (fun (x: u8) -> true);
    f_from_post = (fun (x: u8) (out: i128) -> true);
    f_from = fun (x: u8) -> cast (x <: u8) <: i128
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_34__from__convert: t_From isize u8 =
  {
    f_from_pre = (fun (x: u8) -> true);
    f_from_post = (fun (x: u8) (out: isize) -> true);
    f_from = fun (x: u8) -> cast (x <: u8) <: isize
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_35: t_From i32 u16 =
  {
    f_from_pre = (fun (x: u16) -> true);
    f_from_post = (fun (x: u16) (out: i32) -> true);
    f_from = fun (x: u16) -> cast (x <: u16) <: i32
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_36__from__convert: t_From i64 u16 =
  {
    f_from_pre = (fun (x: u16) -> true);
    f_from_post = (fun (x: u16) (out: i64) -> true);
    f_from = fun (x: u16) -> cast (x <: u16) <: i64
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_37: t_From i128 u16 =
  {
    f_from_pre = (fun (x: u16) -> true);
    f_from_post = (fun (x: u16) (out: i128) -> true);
    f_from = fun (x: u16) -> cast (x <: u16) <: i128
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_38__from__convert: t_From i64 u32 =
  {
    f_from_pre = (fun (x: u32) -> true);
    f_from_post = (fun (x: u32) (out: i64) -> true);
    f_from = fun (x: u32) -> cast (x <: u32) <: i64
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_39: t_From i128 u32 =
  {
    f_from_pre = (fun (x: u32) -> true);
    f_from_post = (fun (x: u32) (out: i128) -> true);
    f_from = fun (x: u32) -> cast (x <: u32) <: i128
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_40__from__convert: t_From i128 u64 =
  {
    f_from_pre = (fun (x: u64) -> true);
    f_from_post = (fun (x: u64) (out: i128) -> true);
    f_from = fun (x: u64) -> cast (x <: u64) <: i128
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_133: t_From u8 bool =
  {
    f_from_pre = (fun (x: bool) -> true);
    f_from_post = (fun (x: bool) (out: u8) -> true);
    f_from = fun (x: bool) -> if x then mk_u8 1 else mk_u8 0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_134: t_From u16 bool =
  {
    f_from_pre = (fun (x: bool) -> true);
    f_from_post = (fun (x: bool) (out: u16) -> true);
    f_from = fun (x: bool) -> if x then mk_u16 1 else mk_u16 0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_135: t_From u32 bool =
  {
    f_from_pre = (fun (x: bool) -> true);
    f_from_post = (fun (x: bool) (out: u32) -> true);
    f_from = fun (x: bool) -> if x then mk_u32 1 else mk_u32 0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_136: t_From u64 bool =
  {
    f_from_pre = (fun (x: bool) -> true);
    f_from_post = (fun (x: bool) (out: u64) -> true);
    f_from = fun (x: bool) -> if x then mk_u64 1 else mk_u64 0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_137: t_From u128 bool =
  {
    f_from_pre = (fun (x: bool) -> true);
    f_from_post = (fun (x: bool) (out: u128) -> true);
    f_from = fun (x: bool) -> if x then mk_u128 1 else mk_u128 0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_138: t_From usize bool =
  {
    f_from_pre = (fun (x: bool) -> true);
    f_from_post = (fun (x: bool) (out: usize) -> true);
    f_from = fun (x: bool) -> if x then mk_usize 1 else mk_usize 0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_139: t_From i8 bool =
  {
    f_from_pre = (fun (x: bool) -> true);
    f_from_post = (fun (x: bool) (out: i8) -> true);
    f_from = fun (x: bool) -> if x then mk_i8 1 else mk_i8 0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_140: t_From i16 bool =
  {
    f_from_pre = (fun (x: bool) -> true);
    f_from_post = (fun (x: bool) (out: i16) -> true);
    f_from = fun (x: bool) -> if x then mk_i16 1 else mk_i16 0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_141: t_From i32 bool =
  {
    f_from_pre = (fun (x: bool) -> true);
    f_from_post = (fun (x: bool) (out: i32) -> true);
    f_from = fun (x: bool) -> if x then mk_i32 1 else mk_i32 0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_142: t_From i64 bool =
  {
    f_from_pre = (fun (x: bool) -> true);
    f_from_post = (fun (x: bool) (out: i64) -> true);
    f_from = fun (x: bool) -> if x then mk_i64 1 else mk_i64 0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_143: t_From i128 bool =
  {
    f_from_pre = (fun (x: bool) -> true);
    f_from_post = (fun (x: bool) (out: i128) -> true);
    f_from = fun (x: bool) -> if x then mk_i128 1 else mk_i128 0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_144: t_From isize bool =
  {
    f_from_pre = (fun (x: bool) -> true);
    f_from_post = (fun (x: bool) (out: isize) -> true);
    f_from = fun (x: bool) -> if x then mk_isize 1 else mk_isize 0
  }

/// See [`std::ops::RangeBounds`]
class t_RangeBounds (v_Self: Type0) (v_T: Type0) = {
  f_start_bound_pre:self_: v_Self -> pred: Type0{true ==> pred};
  f_start_bound_post:v_Self -> t_Bound v_T -> Type0;
  f_start_bound:x0: v_Self
    -> Prims.Pure (t_Bound v_T) (f_start_bound_pre x0) (fun result -> f_start_bound_post x0 result);
  f_end_bound_pre:self_: v_Self -> pred: Type0{true ==> pred};
  f_end_bound_post:v_Self -> t_Bound v_T -> Type0;
  f_end_bound:x0: v_Self
    -> Prims.Pure (t_Bound v_T) (f_end_bound_pre x0) (fun result -> f_end_bound_post x0 result)
}

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl__from__range (#v_T: Type0) : t_RangeBounds t_RangeFull v_T =
  {
    f_start_bound_pre = (fun (self: t_RangeFull) -> true);
    f_start_bound_post = (fun (self: t_RangeFull) (out: t_Bound v_T) -> true);
    f_start_bound = (fun (self: t_RangeFull) -> Bound_Unbounded <: t_Bound v_T);
    f_end_bound_pre = (fun (self: t_RangeFull) -> true);
    f_end_bound_post = (fun (self: t_RangeFull) (out: t_Bound v_T) -> true);
    f_end_bound = fun (self: t_RangeFull) -> Bound_Unbounded <: t_Bound v_T
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_1__from__range (#v_T: Type0) : t_RangeBounds (t_RangeFrom v_T) v_T =
  {
    f_start_bound_pre = (fun (self: t_RangeFrom v_T) -> true);
    f_start_bound_post = (fun (self: t_RangeFrom v_T) (out: t_Bound v_T) -> true);
    f_start_bound = (fun (self: t_RangeFrom v_T) -> Bound_Included self.f_start <: t_Bound v_T);
    f_end_bound_pre = (fun (self: t_RangeFrom v_T) -> true);
    f_end_bound_post = (fun (self: t_RangeFrom v_T) (out: t_Bound v_T) -> true);
    f_end_bound = fun (self: t_RangeFrom v_T) -> Bound_Unbounded <: t_Bound v_T
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_3__from__range (#v_T: Type0) : t_RangeBounds (t_RangeTo v_T) v_T =
  {
    f_start_bound_pre = (fun (self: t_RangeTo v_T) -> true);
    f_start_bound_post = (fun (self: t_RangeTo v_T) (out: t_Bound v_T) -> true);
    f_start_bound = (fun (self: t_RangeTo v_T) -> Bound_Unbounded <: t_Bound v_T);
    f_end_bound_pre = (fun (self: t_RangeTo v_T) -> true);
    f_end_bound_post = (fun (self: t_RangeTo v_T) (out: t_Bound v_T) -> true);
    f_end_bound = fun (self: t_RangeTo v_T) -> Bound_Excluded self.f_end <: t_Bound v_T
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_5__from__range (#v_T: Type0) : t_RangeBounds (t_Range v_T) v_T =
  {
    f_start_bound_pre = (fun (self: t_Range v_T) -> true);
    f_start_bound_post = (fun (self: t_Range v_T) (out: t_Bound v_T) -> true);
    f_start_bound = (fun (self: t_Range v_T) -> Bound_Included self.f_start <: t_Bound v_T);
    f_end_bound_pre = (fun (self: t_Range v_T) -> true);
    f_end_bound_post = (fun (self: t_Range v_T) (out: t_Bound v_T) -> true);
    f_end_bound = fun (self: t_Range v_T) -> Bound_Excluded self.f_end <: t_Bound v_T
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_9 (#v_T: Type0) : t_RangeBounds (t_RangeInclusive v_T) v_T =
  {
    f_start_bound_pre = (fun (self: t_RangeInclusive v_T) -> true);
    f_start_bound_post = (fun (self: t_RangeInclusive v_T) (out: t_Bound v_T) -> true);
    f_start_bound
    =
    (fun (self: t_RangeInclusive v_T) -> Bound_Included self.f_start_ <: t_Bound v_T);
    f_end_bound_pre = (fun (self: t_RangeInclusive v_T) -> true);
    f_end_bound_post = (fun (self: t_RangeInclusive v_T) (out: t_Bound v_T) -> true);
    f_end_bound
    =
    fun (self: t_RangeInclusive v_T) ->
      if self.f_exhausted
      then Bound_Excluded self.f_start_ <: t_Bound v_T
      else Bound_Included self.f_end_ <: t_Bound v_T
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_11__from__range (#v_T: Type0) : t_RangeBounds (t_RangeToInclusive v_T) v_T =
  {
    f_start_bound_pre = (fun (self: t_RangeToInclusive v_T) -> true);
    f_start_bound_post = (fun (self: t_RangeToInclusive v_T) (out: t_Bound v_T) -> true);
    f_start_bound = (fun (self: t_RangeToInclusive v_T) -> Bound_Unbounded <: t_Bound v_T);
    f_end_bound_pre = (fun (self: t_RangeToInclusive v_T) -> true);
    f_end_bound_post = (fun (self: t_RangeToInclusive v_T) (out: t_Bound v_T) -> true);
    f_end_bound = fun (self: t_RangeToInclusive v_T) -> Bound_Included self.f_end <: t_Bound v_T
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_13__from__range (#v_T: Type0) : t_RangeBounds (t_Bound v_T & t_Bound v_T) v_T =
  {
    f_start_bound_pre = (fun (self: (t_Bound v_T & t_Bound v_T)) -> true);
    f_start_bound_post = (fun (self: (t_Bound v_T & t_Bound v_T)) (out: t_Bound v_T) -> true);
    f_start_bound = (fun (self: (t_Bound v_T & t_Bound v_T)) -> bound_as_ref #v_T self._1);
    f_end_bound_pre = (fun (self: (t_Bound v_T & t_Bound v_T)) -> true);
    f_end_bound_post = (fun (self: (t_Bound v_T & t_Bound v_T)) (out: t_Bound v_T) -> true);
    f_end_bound = fun (self: (t_Bound v_T & t_Bound v_T)) -> bound_as_ref #v_T self._2
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_4__from__option
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_PartialEq v_T v_T)
    : t_PartialEq (t_Option v_T) (t_Option v_T) =
  {
    f_eq_pre = (fun (self: t_Option v_T) (other: t_Option v_T) -> true);
    f_eq_post = (fun (self: t_Option v_T) (other: t_Option v_T) (out: bool) -> true);
    f_eq
    =
    fun (self: t_Option v_T) (other: t_Option v_T) ->
      match self, other <: (t_Option v_T & t_Option v_T) with
      | Option_Some a, Option_Some b -> f_eq #v_T #v_T #FStar.Tactics.Typeclasses.solve a b
      | Option_None , Option_None  -> true
      | _ -> false
  }

/// The error half of `?`: re-inject the `Err(e)` residual, widening the error
/// via `From` (mirrors std\'s `impl<T, E, F: From<E>> ... for Result<T, F>`). `Ok`
/// is unreachable — the residual\'s payload is `Infallible`.
[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_7__from__result': #v_T: Type0 -> #v_E: Type0 -> #v_F: Type0 -> {| i0: t_From v_F v_E |}
  -> Core_models.Ops.Try_trait.t_FromResidual (t_Result v_T v_F) (t_Result t_Infallible v_E)

unfold
let impl_7__from__result
      (#v_T #v_E #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_From v_F v_E)
     = impl_7__from__result' #v_T #v_E #v_F #i0

/// See [`std::cmp::PartialOrd`]
class t_PartialOrd (v_Self: Type0) (v_Rhs: Type0) = {
  [@@@ FStar.Tactics.Typeclasses.no_method]_super_i0:t_PartialEq v_Self v_Rhs;
  f_partial_cmp_pre:self_: v_Self -> other: v_Rhs -> pred: Type0{true ==> pred};
  f_partial_cmp_post:v_Self -> v_Rhs -> t_Option t_Ordering -> Type0;
  f_partial_cmp:x0: v_Self -> x1: v_Rhs
    -> Prims.Pure (t_Option t_Ordering)
        (f_partial_cmp_pre x0 x1)
        (fun result -> f_partial_cmp_post x0 x1 result)
}

[@@ FStar.Tactics.Typeclasses.tcinstance]
let _ = fun (v_Self:Type0) (v_Rhs:Type0) {|i: t_PartialOrd v_Self v_Rhs|} -> i._super_i0

/// See [`std::convert::TryInto`]
class t_TryInto (v_Self: Type0) (v_T: Type0) = {
  [@@@ FStar.Tactics.Typeclasses.no_method]f_Error:Type0;
  f_try_into_pre:self_: v_Self -> pred: Type0{true ==> pred};
  f_try_into_post:v_Self -> t_Result v_T f_Error -> Type0;
  f_try_into:x0: v_Self
    -> Prims.Pure (t_Result v_T f_Error)
        (f_try_into_pre x0)
        (fun result -> f_try_into_post x0 result)
}

/// See [`std::convert::TryFrom`]
class t_TryFrom (v_Self: Type0) (v_T: Type0) = {
  [@@@ FStar.Tactics.Typeclasses.no_method]f_Error:Type0;
  f_try_from_pre:x: v_T -> pred: Type0{true ==> pred};
  f_try_from_post:v_T -> t_Result v_Self f_Error -> Type0;
  f_try_from:x0: v_T
    -> Prims.Pure (t_Result v_Self f_Error)
        (f_try_from_pre x0)
        (fun result -> f_try_from_post x0 result)
}

/// See [`std::iter::Iterator`]
class t_Iterator (v_Self: Type0) = {
  [@@@ FStar.Tactics.Typeclasses.no_method]f_Item:Type0;
  f_next_pre:self_: v_Self -> pred: Type0{true ==> pred};
  f_next_post:v_Self -> (v_Self & t_Option f_Item) -> Type0;
  f_next:x0: v_Self
    -> Prims.Pure (v_Self & t_Option f_Item) (f_next_pre x0) (fun result -> f_next_post x0 result)
}

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl (#v_T: Type0) (v_N: usize) : t_Iterator (t_IntoIter v_T v_N) =
  {
    f_Item = v_T;
    f_next_pre = (fun (self: t_IntoIter v_T v_N) -> true);
    f_next_post
    =
    (fun (self: t_IntoIter v_T v_N) (out1: (t_IntoIter v_T v_N & t_Option v_T)) -> true);
    f_next
    =
    fun (self: t_IntoIter v_T v_N) ->
      let (self: t_IntoIter v_T v_N), (hax_temp_output: t_Option v_T) =
        if (Rust_primitives.Sequence.seq_len #v_T self._0 <: usize) =. mk_usize 0
        then self, (Option_None <: t_Option v_T) <: (t_IntoIter v_T v_N & t_Option v_T)
        else
          let (tmp0: Rust_primitives.Sequence.t_Seq v_T), (out: v_T) =
            Rust_primitives.Sequence.seq_remove #v_T self._0 (mk_usize 0)
          in
          let self:t_IntoIter v_T v_N = { self with _0 = tmp0 } <: t_IntoIter v_T v_N in
          let res:v_T = out in
          self, (Option_Some res <: t_Option v_T) <: (t_IntoIter v_T v_N & t_Option v_T)
      in
      self, hax_temp_output <: (t_IntoIter v_T v_N & t_Option v_T)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_2 (#v_T: Type0) (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_PartialOrd v_T v_T)
    : t_PartialOrd (t_Reverse v_T) (t_Reverse v_T) =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    f_partial_cmp_pre = (fun (self: t_Reverse v_T) (other: t_Reverse v_T) -> true);
    f_partial_cmp_post
    =
    (fun (self: t_Reverse v_T) (other: t_Reverse v_T) (out: t_Option t_Ordering) -> true);
    f_partial_cmp
    =
    fun (self: t_Reverse v_T) (other: t_Reverse v_T) ->
      f_partial_cmp #v_T #v_T #FStar.Tactics.Typeclasses.solve other._0 self._0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_34: t_PartialOrd u8 u8 =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    f_partial_cmp_pre = (fun (self: u8) (other: u8) -> true);
    f_partial_cmp_post
    =
    (fun (self_: u8) (other: u8) (res: t_Option t_Ordering) ->
        match res <: t_Option t_Ordering with
        | Option_Some (Ordering_Less ) -> self_ <. other
        | Option_Some (Ordering_Equal ) -> self_ =. other
        | Option_Some (Ordering_Greater ) -> self_ >. other
        | Option_None  -> false);
    f_partial_cmp
    =
    fun (self: u8) (other: u8) ->
      if self <. other
      then Option_Some (Ordering_Less <: t_Ordering) <: t_Option t_Ordering
      else
        if self >. other
        then Option_Some (Ordering_Greater <: t_Ordering) <: t_Option t_Ordering
        else Option_Some (Ordering_Equal <: t_Ordering) <: t_Option t_Ordering
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_36: t_PartialOrd i8 i8 =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    f_partial_cmp_pre = (fun (self: i8) (other: i8) -> true);
    f_partial_cmp_post
    =
    (fun (self_: i8) (other: i8) (res: t_Option t_Ordering) ->
        match res <: t_Option t_Ordering with
        | Option_Some (Ordering_Less ) -> self_ <. other
        | Option_Some (Ordering_Equal ) -> self_ =. other
        | Option_Some (Ordering_Greater ) -> self_ >. other
        | Option_None  -> false);
    f_partial_cmp
    =
    fun (self: i8) (other: i8) ->
      if self <. other
      then Option_Some (Ordering_Less <: t_Ordering) <: t_Option t_Ordering
      else
        if self >. other
        then Option_Some (Ordering_Greater <: t_Ordering) <: t_Option t_Ordering
        else Option_Some (Ordering_Equal <: t_Ordering) <: t_Option t_Ordering
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_38: t_PartialOrd u16 u16 =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    f_partial_cmp_pre = (fun (self: u16) (other: u16) -> true);
    f_partial_cmp_post
    =
    (fun (self_: u16) (other: u16) (res: t_Option t_Ordering) ->
        match res <: t_Option t_Ordering with
        | Option_Some (Ordering_Less ) -> self_ <. other
        | Option_Some (Ordering_Equal ) -> self_ =. other
        | Option_Some (Ordering_Greater ) -> self_ >. other
        | Option_None  -> false);
    f_partial_cmp
    =
    fun (self: u16) (other: u16) ->
      if self <. other
      then Option_Some (Ordering_Less <: t_Ordering) <: t_Option t_Ordering
      else
        if self >. other
        then Option_Some (Ordering_Greater <: t_Ordering) <: t_Option t_Ordering
        else Option_Some (Ordering_Equal <: t_Ordering) <: t_Option t_Ordering
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_40: t_PartialOrd i16 i16 =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    f_partial_cmp_pre = (fun (self: i16) (other: i16) -> true);
    f_partial_cmp_post
    =
    (fun (self_: i16) (other: i16) (res: t_Option t_Ordering) ->
        match res <: t_Option t_Ordering with
        | Option_Some (Ordering_Less ) -> self_ <. other
        | Option_Some (Ordering_Equal ) -> self_ =. other
        | Option_Some (Ordering_Greater ) -> self_ >. other
        | Option_None  -> false);
    f_partial_cmp
    =
    fun (self: i16) (other: i16) ->
      if self <. other
      then Option_Some (Ordering_Less <: t_Ordering) <: t_Option t_Ordering
      else
        if self >. other
        then Option_Some (Ordering_Greater <: t_Ordering) <: t_Option t_Ordering
        else Option_Some (Ordering_Equal <: t_Ordering) <: t_Option t_Ordering
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_42: t_PartialOrd u32 u32 =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    f_partial_cmp_pre = (fun (self: u32) (other: u32) -> true);
    f_partial_cmp_post
    =
    (fun (self_: u32) (other: u32) (res: t_Option t_Ordering) ->
        match res <: t_Option t_Ordering with
        | Option_Some (Ordering_Less ) -> self_ <. other
        | Option_Some (Ordering_Equal ) -> self_ =. other
        | Option_Some (Ordering_Greater ) -> self_ >. other
        | Option_None  -> false);
    f_partial_cmp
    =
    fun (self: u32) (other: u32) ->
      if self <. other
      then Option_Some (Ordering_Less <: t_Ordering) <: t_Option t_Ordering
      else
        if self >. other
        then Option_Some (Ordering_Greater <: t_Ordering) <: t_Option t_Ordering
        else Option_Some (Ordering_Equal <: t_Ordering) <: t_Option t_Ordering
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_44: t_PartialOrd i32 i32 =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    f_partial_cmp_pre = (fun (self: i32) (other: i32) -> true);
    f_partial_cmp_post
    =
    (fun (self_: i32) (other: i32) (res: t_Option t_Ordering) ->
        match res <: t_Option t_Ordering with
        | Option_Some (Ordering_Less ) -> self_ <. other
        | Option_Some (Ordering_Equal ) -> self_ =. other
        | Option_Some (Ordering_Greater ) -> self_ >. other
        | Option_None  -> false);
    f_partial_cmp
    =
    fun (self: i32) (other: i32) ->
      if self <. other
      then Option_Some (Ordering_Less <: t_Ordering) <: t_Option t_Ordering
      else
        if self >. other
        then Option_Some (Ordering_Greater <: t_Ordering) <: t_Option t_Ordering
        else Option_Some (Ordering_Equal <: t_Ordering) <: t_Option t_Ordering
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_46: t_PartialOrd u64 u64 =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    f_partial_cmp_pre = (fun (self: u64) (other: u64) -> true);
    f_partial_cmp_post
    =
    (fun (self_: u64) (other: u64) (res: t_Option t_Ordering) ->
        match res <: t_Option t_Ordering with
        | Option_Some (Ordering_Less ) -> self_ <. other
        | Option_Some (Ordering_Equal ) -> self_ =. other
        | Option_Some (Ordering_Greater ) -> self_ >. other
        | Option_None  -> false);
    f_partial_cmp
    =
    fun (self: u64) (other: u64) ->
      if self <. other
      then Option_Some (Ordering_Less <: t_Ordering) <: t_Option t_Ordering
      else
        if self >. other
        then Option_Some (Ordering_Greater <: t_Ordering) <: t_Option t_Ordering
        else Option_Some (Ordering_Equal <: t_Ordering) <: t_Option t_Ordering
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_48: t_PartialOrd i64 i64 =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    f_partial_cmp_pre = (fun (self: i64) (other: i64) -> true);
    f_partial_cmp_post
    =
    (fun (self_: i64) (other: i64) (res: t_Option t_Ordering) ->
        match res <: t_Option t_Ordering with
        | Option_Some (Ordering_Less ) -> self_ <. other
        | Option_Some (Ordering_Equal ) -> self_ =. other
        | Option_Some (Ordering_Greater ) -> self_ >. other
        | Option_None  -> false);
    f_partial_cmp
    =
    fun (self: i64) (other: i64) ->
      if self <. other
      then Option_Some (Ordering_Less <: t_Ordering) <: t_Option t_Ordering
      else
        if self >. other
        then Option_Some (Ordering_Greater <: t_Ordering) <: t_Option t_Ordering
        else Option_Some (Ordering_Equal <: t_Ordering) <: t_Option t_Ordering
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_50: t_PartialOrd u128 u128 =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    f_partial_cmp_pre = (fun (self: u128) (other: u128) -> true);
    f_partial_cmp_post
    =
    (fun (self_: u128) (other: u128) (res: t_Option t_Ordering) ->
        match res <: t_Option t_Ordering with
        | Option_Some (Ordering_Less ) -> self_ <. other
        | Option_Some (Ordering_Equal ) -> self_ =. other
        | Option_Some (Ordering_Greater ) -> self_ >. other
        | Option_None  -> false);
    f_partial_cmp
    =
    fun (self: u128) (other: u128) ->
      if self <. other
      then Option_Some (Ordering_Less <: t_Ordering) <: t_Option t_Ordering
      else
        if self >. other
        then Option_Some (Ordering_Greater <: t_Ordering) <: t_Option t_Ordering
        else Option_Some (Ordering_Equal <: t_Ordering) <: t_Option t_Ordering
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_52: t_PartialOrd i128 i128 =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    f_partial_cmp_pre = (fun (self: i128) (other: i128) -> true);
    f_partial_cmp_post
    =
    (fun (self_: i128) (other: i128) (res: t_Option t_Ordering) ->
        match res <: t_Option t_Ordering with
        | Option_Some (Ordering_Less ) -> self_ <. other
        | Option_Some (Ordering_Equal ) -> self_ =. other
        | Option_Some (Ordering_Greater ) -> self_ >. other
        | Option_None  -> false);
    f_partial_cmp
    =
    fun (self: i128) (other: i128) ->
      if self <. other
      then Option_Some (Ordering_Less <: t_Ordering) <: t_Option t_Ordering
      else
        if self >. other
        then Option_Some (Ordering_Greater <: t_Ordering) <: t_Option t_Ordering
        else Option_Some (Ordering_Equal <: t_Ordering) <: t_Option t_Ordering
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_54: t_PartialOrd usize usize =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    f_partial_cmp_pre = (fun (self: usize) (other: usize) -> true);
    f_partial_cmp_post
    =
    (fun (self_: usize) (other: usize) (res: t_Option t_Ordering) ->
        match res <: t_Option t_Ordering with
        | Option_Some (Ordering_Less ) -> self_ <. other
        | Option_Some (Ordering_Equal ) -> self_ =. other
        | Option_Some (Ordering_Greater ) -> self_ >. other
        | Option_None  -> false);
    f_partial_cmp
    =
    fun (self: usize) (other: usize) ->
      if self <. other
      then Option_Some (Ordering_Less <: t_Ordering) <: t_Option t_Ordering
      else
        if self >. other
        then Option_Some (Ordering_Greater <: t_Ordering) <: t_Option t_Ordering
        else Option_Some (Ordering_Equal <: t_Ordering) <: t_Option t_Ordering
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_56: t_PartialOrd isize isize =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    f_partial_cmp_pre = (fun (self: isize) (other: isize) -> true);
    f_partial_cmp_post
    =
    (fun (self_: isize) (other: isize) (res: t_Option t_Ordering) ->
        match res <: t_Option t_Ordering with
        | Option_Some (Ordering_Less ) -> self_ <. other
        | Option_Some (Ordering_Equal ) -> self_ =. other
        | Option_Some (Ordering_Greater ) -> self_ >. other
        | Option_None  -> false);
    f_partial_cmp
    =
    fun (self: isize) (other: isize) ->
      if self <. other
      then Option_Some (Ordering_Less <: t_Ordering) <: t_Option t_Ordering
      else
        if self >. other
        then Option_Some (Ordering_Greater <: t_Ordering) <: t_Option t_Ordering
        else Option_Some (Ordering_Equal <: t_Ordering) <: t_Option t_Ordering
  }

/// See [`std::cmp::PartialOrd`] for `()`
[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_8: t_PartialOrd Prims.unit Prims.unit =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    f_partial_cmp_pre = (fun (self: Prims.unit) (other: Prims.unit) -> true);
    f_partial_cmp_post
    =
    (fun (self: Prims.unit) (other: Prims.unit) (out: t_Option t_Ordering) -> true);
    f_partial_cmp
    =
    fun (self: Prims.unit) (other: Prims.unit) ->
      Option_Some (Ordering_Equal <: t_Ordering) <: t_Option t_Ordering
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_1__from__convert
      (#v_T #v_U: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_From v_U v_T)
    : t_TryFrom v_U v_T =
  {
    f_Error = t_Infallible;
    f_try_from_pre = (fun (x: v_T) -> true);
    f_try_from_post = (fun (x: v_T) (out: t_Result v_U t_Infallible) -> true);
    f_try_from
    =
    fun (x: v_T) ->
      Result_Ok (f_from #v_U #v_T #FStar.Tactics.Typeclasses.solve x) <: t_Result v_U t_Infallible
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_2__from__convert
      (#v_T: Type0)
      (v_N: usize)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Core_models.Marker.t_Copy v_T)
    : t_TryFrom (t_Array v_T v_N) (t_Slice v_T) =
  {
    f_Error = t_TryFromSliceError;
    f_try_from_pre = (fun (x: t_Slice v_T) -> true);
    f_try_from_post
    =
    (fun (x: t_Slice v_T) (out: t_Result (t_Array v_T v_N) t_TryFromSliceError) -> true);
    f_try_from
    =
    fun (x: t_Slice v_T) ->
      if (Rust_primitives.Slice.slice_length #v_T x <: usize) =. v_N
      then
        Result_Ok
        (Rust_primitives.Slice.array_from_fn #v_T
            v_N
            #(usize -> v_T)
            (fun i ->
                let i:usize = i in
                Rust_primitives.Slice.slice_index #v_T x i <: v_T))
        <:
        t_Result (t_Array v_T v_N) t_TryFromSliceError
      else
        Result_Err (TryFromSliceError <: t_TryFromSliceError)
        <:
        t_Result (t_Array v_T v_N) t_TryFromSliceError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_3__from__convert
      (#v_T #v_U: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_TryFrom v_U v_T)
    : t_TryInto v_T v_U =
  {
    f_Error = i0.f_Error;
    f_try_into_pre = (fun (self: v_T) -> true);
    f_try_into_post = (fun (self: v_T) (out: t_Result v_U i0.f_Error) -> true);
    f_try_into = fun (self: v_T) -> f_try_from #v_U #v_T #FStar.Tactics.Typeclasses.solve self
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_41: t_TryFrom u8 u16 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u16) -> true);
    f_try_from_post
    =
    (fun (x: u16) (out: t_Result u8 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u16) ->
      if x >. (cast (impl_6__MAX <: u8) <: u16) || x <. (cast (impl_6__MIN <: u8) <: u16)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u8 Core_models.Num.Error.t_TryFromIntError
      else Result_Ok (cast (x <: u16) <: u8) <: t_Result u8 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_42__from__convert: t_TryFrom u8 u32 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u32) -> true);
    f_try_from_post
    =
    (fun (x: u32) (out: t_Result u8 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u32) ->
      if x >. (cast (impl_6__MAX <: u8) <: u32) || x <. (cast (impl_6__MIN <: u8) <: u32)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u8 Core_models.Num.Error.t_TryFromIntError
      else Result_Ok (cast (x <: u32) <: u8) <: t_Result u8 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_43: t_TryFrom u16 u32 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u32) -> true);
    f_try_from_post
    =
    (fun (x: u32) (out: t_Result u16 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u32) ->
      if x >. (cast (impl_7__MAX <: u16) <: u32) || x <. (cast (impl_7__MIN <: u16) <: u32)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u16 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: u32) <: u16) <: t_Result u16 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_44__from__convert: t_TryFrom u8 u64 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u64) -> true);
    f_try_from_post
    =
    (fun (x: u64) (out: t_Result u8 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u64) ->
      if x >. (cast (impl_6__MAX <: u8) <: u64) || x <. (cast (impl_6__MIN <: u8) <: u64)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u8 Core_models.Num.Error.t_TryFromIntError
      else Result_Ok (cast (x <: u64) <: u8) <: t_Result u8 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_45: t_TryFrom u16 u64 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u64) -> true);
    f_try_from_post
    =
    (fun (x: u64) (out: t_Result u16 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u64) ->
      if x >. (cast (impl_7__MAX <: u16) <: u64) || x <. (cast (impl_7__MIN <: u16) <: u64)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u16 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: u64) <: u16) <: t_Result u16 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_46__from__convert: t_TryFrom u32 u64 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u64) -> true);
    f_try_from_post
    =
    (fun (x: u64) (out: t_Result u32 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u64) ->
      if x >. (cast (impl_8__MAX <: u32) <: u64) || x <. (cast (impl_8__MIN <: u32) <: u64)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u32 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: u64) <: u32) <: t_Result u32 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_47: t_TryFrom usize u64 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u64) -> true);
    f_try_from_post
    =
    (fun (x: u64) (out: t_Result usize Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u64) ->
      if x >. (cast (impl_11__MAX <: usize) <: u64) || x <. (cast (impl_11__MIN <: usize) <: u64)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result usize Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: u64) <: usize)
        <:
        t_Result usize Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_48__from__convert: t_TryFrom u8 u128 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u128) -> true);
    f_try_from_post
    =
    (fun (x: u128) (out: t_Result u8 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u128) ->
      if x >. (cast (impl_6__MAX <: u8) <: u128) || x <. (cast (impl_6__MIN <: u8) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u8 Core_models.Num.Error.t_TryFromIntError
      else Result_Ok (cast (x <: u128) <: u8) <: t_Result u8 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_49: t_TryFrom u16 u128 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u128) -> true);
    f_try_from_post
    =
    (fun (x: u128) (out: t_Result u16 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u128) ->
      if x >. (cast (impl_7__MAX <: u16) <: u128) || x <. (cast (impl_7__MIN <: u16) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u16 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: u128) <: u16) <: t_Result u16 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_50__from__convert: t_TryFrom u32 u128 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u128) -> true);
    f_try_from_post
    =
    (fun (x: u128) (out: t_Result u32 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u128) ->
      if x >. (cast (impl_8__MAX <: u32) <: u128) || x <. (cast (impl_8__MIN <: u32) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u32 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: u128) <: u32) <: t_Result u32 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_51: t_TryFrom u64 u128 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u128) -> true);
    f_try_from_post
    =
    (fun (x: u128) (out: t_Result u64 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u128) ->
      if x >. (cast (impl_9__MAX <: u64) <: u128) || x <. (cast (impl_9__MIN <: u64) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u64 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: u128) <: u64) <: t_Result u64 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_52__from__convert: t_TryFrom usize u128 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u128) -> true);
    f_try_from_post
    =
    (fun (x: u128) (out: t_Result usize Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u128) ->
      if x >. (cast (impl_11__MAX <: usize) <: u128) || x <. (cast (impl_11__MIN <: usize) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result usize Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: u128) <: usize)
        <:
        t_Result usize Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_53: t_TryFrom u8 usize =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: usize) -> true);
    f_try_from_post
    =
    (fun (x: usize) (out: t_Result u8 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: usize) ->
      if x >. (cast (impl_6__MAX <: u8) <: usize) || x <. (cast (impl_6__MIN <: u8) <: usize)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u8 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: usize) <: u8) <: t_Result u8 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_54__from__convert: t_TryFrom u16 usize =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: usize) -> true);
    f_try_from_post
    =
    (fun (x: usize) (out: t_Result u16 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: usize) ->
      if x >. (cast (impl_7__MAX <: u16) <: usize) || x <. (cast (impl_7__MIN <: u16) <: usize)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u16 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: usize) <: u16) <: t_Result u16 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_55: t_TryFrom u32 usize =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: usize) -> true);
    f_try_from_post
    =
    (fun (x: usize) (out: t_Result u32 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: usize) ->
      if x >. (cast (impl_8__MAX <: u32) <: usize) || x <. (cast (impl_8__MIN <: u32) <: usize)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u32 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: usize) <: u32) <: t_Result u32 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_56__from__convert: t_TryFrom u64 usize =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: usize) -> true);
    f_try_from_post
    =
    (fun (x: usize) (out: t_Result u64 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: usize) ->
      if x >. (cast (impl_9__MAX <: u64) <: usize) || x <. (cast (impl_9__MIN <: u64) <: usize)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u64 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: usize) <: u64) <: t_Result u64 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_57: t_TryFrom i8 i16 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i16) -> true);
    f_try_from_post
    =
    (fun (x: i16) (out: t_Result i8 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i16) ->
      if x >. (cast (impl_12__MAX <: i8) <: i16) || x <. (cast (impl_12__MIN <: i8) <: i16)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i8 Core_models.Num.Error.t_TryFromIntError
      else Result_Ok (cast (x <: i16) <: i8) <: t_Result i8 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_58: t_TryFrom i8 i32 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i32) -> true);
    f_try_from_post
    =
    (fun (x: i32) (out: t_Result i8 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i32) ->
      if x >. (cast (impl_12__MAX <: i8) <: i32) || x <. (cast (impl_12__MIN <: i8) <: i32)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i8 Core_models.Num.Error.t_TryFromIntError
      else Result_Ok (cast (x <: i32) <: i8) <: t_Result i8 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_59: t_TryFrom i16 i32 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i32) -> true);
    f_try_from_post
    =
    (fun (x: i32) (out: t_Result i16 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i32) ->
      if x >. (cast (impl_13__MAX <: i16) <: i32) || x <. (cast (impl_13__MIN <: i16) <: i32)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i16 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i32) <: i16) <: t_Result i16 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_60: t_TryFrom i8 i64 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i64) -> true);
    f_try_from_post
    =
    (fun (x: i64) (out: t_Result i8 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i64) ->
      if x >. (cast (impl_12__MAX <: i8) <: i64) || x <. (cast (impl_12__MIN <: i8) <: i64)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i8 Core_models.Num.Error.t_TryFromIntError
      else Result_Ok (cast (x <: i64) <: i8) <: t_Result i8 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_61: t_TryFrom i16 i64 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i64) -> true);
    f_try_from_post
    =
    (fun (x: i64) (out: t_Result i16 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i64) ->
      if x >. (cast (impl_13__MAX <: i16) <: i64) || x <. (cast (impl_13__MIN <: i16) <: i64)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i16 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i64) <: i16) <: t_Result i16 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_62: t_TryFrom i32 i64 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i64) -> true);
    f_try_from_post
    =
    (fun (x: i64) (out: t_Result i32 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i64) ->
      if x >. (cast (impl_14__MAX <: i32) <: i64) || x <. (cast (impl_14__MIN <: i32) <: i64)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i32 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i64) <: i32) <: t_Result i32 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_63: t_TryFrom isize i64 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i64) -> true);
    f_try_from_post
    =
    (fun (x: i64) (out: t_Result isize Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i64) ->
      if x >. (cast (impl_17__MAX <: isize) <: i64) || x <. (cast (impl_17__MIN <: isize) <: i64)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result isize Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i64) <: isize)
        <:
        t_Result isize Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_64: t_TryFrom i8 i128 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i128) -> true);
    f_try_from_post
    =
    (fun (x: i128) (out: t_Result i8 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i128) ->
      if x >. (cast (impl_12__MAX <: i8) <: i128) || x <. (cast (impl_12__MIN <: i8) <: i128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i8 Core_models.Num.Error.t_TryFromIntError
      else Result_Ok (cast (x <: i128) <: i8) <: t_Result i8 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_65: t_TryFrom i16 i128 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i128) -> true);
    f_try_from_post
    =
    (fun (x: i128) (out: t_Result i16 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i128) ->
      if x >. (cast (impl_13__MAX <: i16) <: i128) || x <. (cast (impl_13__MIN <: i16) <: i128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i16 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i128) <: i16) <: t_Result i16 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_66: t_TryFrom i32 i128 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i128) -> true);
    f_try_from_post
    =
    (fun (x: i128) (out: t_Result i32 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i128) ->
      if x >. (cast (impl_14__MAX <: i32) <: i128) || x <. (cast (impl_14__MIN <: i32) <: i128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i32 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i128) <: i32) <: t_Result i32 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_67: t_TryFrom i64 i128 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i128) -> true);
    f_try_from_post
    =
    (fun (x: i128) (out: t_Result i64 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i128) ->
      if x >. (cast (impl_15__MAX <: i64) <: i128) || x <. (cast (impl_15__MIN <: i64) <: i128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i64 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i128) <: i64) <: t_Result i64 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_68: t_TryFrom isize i128 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i128) -> true);
    f_try_from_post
    =
    (fun (x: i128) (out: t_Result isize Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i128) ->
      if x >. (cast (impl_17__MAX <: isize) <: i128) || x <. (cast (impl_17__MIN <: isize) <: i128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result isize Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i128) <: isize)
        <:
        t_Result isize Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_69: t_TryFrom i8 isize =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: isize) -> true);
    f_try_from_post
    =
    (fun (x: isize) (out: t_Result i8 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: isize) ->
      if x >. (cast (impl_12__MAX <: i8) <: isize) || x <. (cast (impl_12__MIN <: i8) <: isize)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i8 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: isize) <: i8) <: t_Result i8 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_70: t_TryFrom i16 isize =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: isize) -> true);
    f_try_from_post
    =
    (fun (x: isize) (out: t_Result i16 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: isize) ->
      if x >. (cast (impl_13__MAX <: i16) <: isize) || x <. (cast (impl_13__MIN <: i16) <: isize)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i16 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: isize) <: i16) <: t_Result i16 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_71: t_TryFrom i32 isize =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: isize) -> true);
    f_try_from_post
    =
    (fun (x: isize) (out: t_Result i32 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: isize) ->
      if x >. (cast (impl_14__MAX <: i32) <: isize) || x <. (cast (impl_14__MIN <: i32) <: isize)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i32 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: isize) <: i32) <: t_Result i32 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_72: t_TryFrom i64 isize =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: isize) -> true);
    f_try_from_post
    =
    (fun (x: isize) (out: t_Result i64 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: isize) ->
      if x >. (cast (impl_15__MAX <: i64) <: isize) || x <. (cast (impl_15__MIN <: i64) <: isize)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i64 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: isize) <: i64) <: t_Result i64 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_73: t_TryFrom isize i32 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i32) -> true);
    f_try_from_post
    =
    (fun (x: i32) (out: t_Result isize Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i32) ->
      Result_Ok (cast (x <: i32) <: isize) <: t_Result isize Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_74: t_TryFrom i128 isize =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: isize) -> true);
    f_try_from_post
    =
    (fun (x: isize) (out: t_Result i128 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: isize) ->
      Result_Ok (cast (x <: isize) <: i128) <: t_Result i128 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_75: t_TryFrom usize u32 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u32) -> true);
    f_try_from_post
    =
    (fun (x: u32) (out: t_Result usize Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u32) ->
      Result_Ok (cast (x <: u32) <: usize) <: t_Result usize Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_76: t_TryFrom u128 usize =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: usize) -> true);
    f_try_from_post
    =
    (fun (x: usize) (out: t_Result u128 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: usize) ->
      Result_Ok (cast (x <: usize) <: u128) <: t_Result u128 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_77: t_TryFrom i8 u8 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u8) -> true);
    f_try_from_post
    =
    (fun (x: u8) (out: t_Result i8 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u8) ->
      if x >. (cast (impl_12__MAX <: i8) <: u8)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i8 Core_models.Num.Error.t_TryFromIntError
      else Result_Ok (cast (x <: u8) <: i8) <: t_Result i8 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_78: t_TryFrom i8 u16 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u16) -> true);
    f_try_from_post
    =
    (fun (x: u16) (out: t_Result i8 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u16) ->
      if x >. (cast (impl_12__MAX <: i8) <: u16)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i8 Core_models.Num.Error.t_TryFromIntError
      else Result_Ok (cast (x <: u16) <: i8) <: t_Result i8 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_79: t_TryFrom i16 u16 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u16) -> true);
    f_try_from_post
    =
    (fun (x: u16) (out: t_Result i16 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u16) ->
      if x >. (cast (impl_13__MAX <: i16) <: u16)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i16 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: u16) <: i16) <: t_Result i16 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_80: t_TryFrom i8 u32 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u32) -> true);
    f_try_from_post
    =
    (fun (x: u32) (out: t_Result i8 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u32) ->
      if x >. (cast (impl_12__MAX <: i8) <: u32)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i8 Core_models.Num.Error.t_TryFromIntError
      else Result_Ok (cast (x <: u32) <: i8) <: t_Result i8 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_81: t_TryFrom i16 u32 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u32) -> true);
    f_try_from_post
    =
    (fun (x: u32) (out: t_Result i16 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u32) ->
      if x >. (cast (impl_13__MAX <: i16) <: u32)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i16 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: u32) <: i16) <: t_Result i16 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_82: t_TryFrom i32 u32 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u32) -> true);
    f_try_from_post
    =
    (fun (x: u32) (out: t_Result i32 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u32) ->
      if x >. (cast (impl_14__MAX <: i32) <: u32)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i32 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: u32) <: i32) <: t_Result i32 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_83: t_TryFrom i8 u64 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u64) -> true);
    f_try_from_post
    =
    (fun (x: u64) (out: t_Result i8 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u64) ->
      if x >. (cast (impl_12__MAX <: i8) <: u64)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i8 Core_models.Num.Error.t_TryFromIntError
      else Result_Ok (cast (x <: u64) <: i8) <: t_Result i8 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_84: t_TryFrom i16 u64 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u64) -> true);
    f_try_from_post
    =
    (fun (x: u64) (out: t_Result i16 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u64) ->
      if x >. (cast (impl_13__MAX <: i16) <: u64)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i16 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: u64) <: i16) <: t_Result i16 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_85: t_TryFrom i32 u64 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u64) -> true);
    f_try_from_post
    =
    (fun (x: u64) (out: t_Result i32 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u64) ->
      if x >. (cast (impl_14__MAX <: i32) <: u64)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i32 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: u64) <: i32) <: t_Result i32 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_86: t_TryFrom i64 u64 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u64) -> true);
    f_try_from_post
    =
    (fun (x: u64) (out: t_Result i64 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u64) ->
      if x >. (cast (impl_15__MAX <: i64) <: u64)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i64 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: u64) <: i64) <: t_Result i64 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_87: t_TryFrom i8 u128 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u128) -> true);
    f_try_from_post
    =
    (fun (x: u128) (out: t_Result i8 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u128) ->
      if x >. (cast (impl_12__MAX <: i8) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i8 Core_models.Num.Error.t_TryFromIntError
      else Result_Ok (cast (x <: u128) <: i8) <: t_Result i8 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_88: t_TryFrom i16 u128 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u128) -> true);
    f_try_from_post
    =
    (fun (x: u128) (out: t_Result i16 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u128) ->
      if x >. (cast (impl_13__MAX <: i16) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i16 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: u128) <: i16) <: t_Result i16 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_89: t_TryFrom i32 u128 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u128) -> true);
    f_try_from_post
    =
    (fun (x: u128) (out: t_Result i32 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u128) ->
      if x >. (cast (impl_14__MAX <: i32) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i32 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: u128) <: i32) <: t_Result i32 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_90: t_TryFrom i64 u128 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u128) -> true);
    f_try_from_post
    =
    (fun (x: u128) (out: t_Result i64 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u128) ->
      if x >. (cast (impl_15__MAX <: i64) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i64 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: u128) <: i64) <: t_Result i64 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_91: t_TryFrom i128 u128 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: u128) -> true);
    f_try_from_post
    =
    (fun (x: u128) (out: t_Result i128 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: u128) ->
      if x >. (cast (impl_16__MAX <: i128) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i128 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: u128) <: i128)
        <:
        t_Result i128 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_92: t_TryFrom i8 usize =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: usize) -> true);
    f_try_from_post
    =
    (fun (x: usize) (out: t_Result i8 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: usize) ->
      if x >. (cast (impl_12__MAX <: i8) <: usize)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i8 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: usize) <: i8) <: t_Result i8 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_93: t_TryFrom i16 usize =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: usize) -> true);
    f_try_from_post
    =
    (fun (x: usize) (out: t_Result i16 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: usize) ->
      if x >. (cast (impl_13__MAX <: i16) <: usize)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i16 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: usize) <: i16) <: t_Result i16 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_94: t_TryFrom i32 usize =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: usize) -> true);
    f_try_from_post
    =
    (fun (x: usize) (out: t_Result i32 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: usize) ->
      if x >. (cast (impl_14__MAX <: i32) <: usize)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i32 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: usize) <: i32) <: t_Result i32 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_95: t_TryFrom i64 usize =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: usize) -> true);
    f_try_from_post
    =
    (fun (x: usize) (out: t_Result i64 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: usize) ->
      if x >. (cast (impl_15__MAX <: i64) <: usize)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result i64 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: usize) <: i64) <: t_Result i64 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_96: t_TryFrom isize usize =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: usize) -> true);
    f_try_from_post
    =
    (fun (x: usize) (out: t_Result isize Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: usize) ->
      if x >. (cast (impl_17__MAX <: isize) <: usize)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result isize Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: usize) <: isize)
        <:
        t_Result isize Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_97: t_TryFrom u8 i8 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i8) -> true);
    f_try_from_post
    =
    (fun (x: i8) (out: t_Result u8 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i8) ->
      if x <. mk_i8 0 || (cast (x <: i8) <: u128) >. (cast (impl_6__MAX <: u8) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u8 Core_models.Num.Error.t_TryFromIntError
      else Result_Ok (cast (x <: i8) <: u8) <: t_Result u8 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_98: t_TryFrom u16 i8 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i8) -> true);
    f_try_from_post
    =
    (fun (x: i8) (out: t_Result u16 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i8) ->
      if x <. mk_i8 0 || (cast (x <: i8) <: u128) >. (cast (impl_7__MAX <: u16) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u16 Core_models.Num.Error.t_TryFromIntError
      else Result_Ok (cast (x <: i8) <: u16) <: t_Result u16 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_99: t_TryFrom u32 i8 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i8) -> true);
    f_try_from_post
    =
    (fun (x: i8) (out: t_Result u32 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i8) ->
      if x <. mk_i8 0 || (cast (x <: i8) <: u128) >. (cast (impl_8__MAX <: u32) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u32 Core_models.Num.Error.t_TryFromIntError
      else Result_Ok (cast (x <: i8) <: u32) <: t_Result u32 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_100: t_TryFrom u64 i8 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i8) -> true);
    f_try_from_post
    =
    (fun (x: i8) (out: t_Result u64 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i8) ->
      if x <. mk_i8 0 || (cast (x <: i8) <: u128) >. (cast (impl_9__MAX <: u64) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u64 Core_models.Num.Error.t_TryFromIntError
      else Result_Ok (cast (x <: i8) <: u64) <: t_Result u64 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_101: t_TryFrom u128 i8 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i8) -> true);
    f_try_from_post
    =
    (fun (x: i8) (out: t_Result u128 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i8) ->
      if x <. mk_i8 0 || (cast (x <: i8) <: u128) >. impl_10__MAX
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u128 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i8) <: u128) <: t_Result u128 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_102: t_TryFrom usize i8 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i8) -> true);
    f_try_from_post
    =
    (fun (x: i8) (out: t_Result usize Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i8) ->
      if x <. mk_i8 0 || (cast (x <: i8) <: u128) >. (cast (impl_11__MAX <: usize) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result usize Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i8) <: usize)
        <:
        t_Result usize Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_103: t_TryFrom u8 i16 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i16) -> true);
    f_try_from_post
    =
    (fun (x: i16) (out: t_Result u8 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i16) ->
      if x <. mk_i16 0 || (cast (x <: i16) <: u128) >. (cast (impl_6__MAX <: u8) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u8 Core_models.Num.Error.t_TryFromIntError
      else Result_Ok (cast (x <: i16) <: u8) <: t_Result u8 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_104: t_TryFrom u16 i16 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i16) -> true);
    f_try_from_post
    =
    (fun (x: i16) (out: t_Result u16 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i16) ->
      if x <. mk_i16 0 || (cast (x <: i16) <: u128) >. (cast (impl_7__MAX <: u16) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u16 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i16) <: u16) <: t_Result u16 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_105: t_TryFrom u32 i16 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i16) -> true);
    f_try_from_post
    =
    (fun (x: i16) (out: t_Result u32 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i16) ->
      if x <. mk_i16 0 || (cast (x <: i16) <: u128) >. (cast (impl_8__MAX <: u32) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u32 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i16) <: u32) <: t_Result u32 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_106: t_TryFrom u64 i16 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i16) -> true);
    f_try_from_post
    =
    (fun (x: i16) (out: t_Result u64 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i16) ->
      if x <. mk_i16 0 || (cast (x <: i16) <: u128) >. (cast (impl_9__MAX <: u64) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u64 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i16) <: u64) <: t_Result u64 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_107: t_TryFrom u128 i16 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i16) -> true);
    f_try_from_post
    =
    (fun (x: i16) (out: t_Result u128 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i16) ->
      if x <. mk_i16 0 || (cast (x <: i16) <: u128) >. impl_10__MAX
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u128 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i16) <: u128) <: t_Result u128 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_108: t_TryFrom usize i16 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i16) -> true);
    f_try_from_post
    =
    (fun (x: i16) (out: t_Result usize Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i16) ->
      if x <. mk_i16 0 || (cast (x <: i16) <: u128) >. (cast (impl_11__MAX <: usize) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result usize Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i16) <: usize)
        <:
        t_Result usize Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_109: t_TryFrom u8 i32 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i32) -> true);
    f_try_from_post
    =
    (fun (x: i32) (out: t_Result u8 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i32) ->
      if x <. mk_i32 0 || (cast (x <: i32) <: u128) >. (cast (impl_6__MAX <: u8) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u8 Core_models.Num.Error.t_TryFromIntError
      else Result_Ok (cast (x <: i32) <: u8) <: t_Result u8 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_110: t_TryFrom u16 i32 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i32) -> true);
    f_try_from_post
    =
    (fun (x: i32) (out: t_Result u16 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i32) ->
      if x <. mk_i32 0 || (cast (x <: i32) <: u128) >. (cast (impl_7__MAX <: u16) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u16 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i32) <: u16) <: t_Result u16 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_111: t_TryFrom u32 i32 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i32) -> true);
    f_try_from_post
    =
    (fun (x: i32) (out: t_Result u32 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i32) ->
      if x <. mk_i32 0 || (cast (x <: i32) <: u128) >. (cast (impl_8__MAX <: u32) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u32 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i32) <: u32) <: t_Result u32 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_112: t_TryFrom u64 i32 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i32) -> true);
    f_try_from_post
    =
    (fun (x: i32) (out: t_Result u64 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i32) ->
      if x <. mk_i32 0 || (cast (x <: i32) <: u128) >. (cast (impl_9__MAX <: u64) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u64 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i32) <: u64) <: t_Result u64 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_113: t_TryFrom u128 i32 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i32) -> true);
    f_try_from_post
    =
    (fun (x: i32) (out: t_Result u128 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i32) ->
      if x <. mk_i32 0 || (cast (x <: i32) <: u128) >. impl_10__MAX
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u128 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i32) <: u128) <: t_Result u128 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_114: t_TryFrom usize i32 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i32) -> true);
    f_try_from_post
    =
    (fun (x: i32) (out: t_Result usize Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i32) ->
      if x <. mk_i32 0 || (cast (x <: i32) <: u128) >. (cast (impl_11__MAX <: usize) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result usize Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i32) <: usize)
        <:
        t_Result usize Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_115: t_TryFrom u8 i64 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i64) -> true);
    f_try_from_post
    =
    (fun (x: i64) (out: t_Result u8 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i64) ->
      if x <. mk_i64 0 || (cast (x <: i64) <: u128) >. (cast (impl_6__MAX <: u8) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u8 Core_models.Num.Error.t_TryFromIntError
      else Result_Ok (cast (x <: i64) <: u8) <: t_Result u8 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_116: t_TryFrom u16 i64 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i64) -> true);
    f_try_from_post
    =
    (fun (x: i64) (out: t_Result u16 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i64) ->
      if x <. mk_i64 0 || (cast (x <: i64) <: u128) >. (cast (impl_7__MAX <: u16) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u16 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i64) <: u16) <: t_Result u16 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_117: t_TryFrom u32 i64 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i64) -> true);
    f_try_from_post
    =
    (fun (x: i64) (out: t_Result u32 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i64) ->
      if x <. mk_i64 0 || (cast (x <: i64) <: u128) >. (cast (impl_8__MAX <: u32) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u32 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i64) <: u32) <: t_Result u32 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_118: t_TryFrom u64 i64 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i64) -> true);
    f_try_from_post
    =
    (fun (x: i64) (out: t_Result u64 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i64) ->
      if x <. mk_i64 0 || (cast (x <: i64) <: u128) >. (cast (impl_9__MAX <: u64) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u64 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i64) <: u64) <: t_Result u64 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_119: t_TryFrom u128 i64 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i64) -> true);
    f_try_from_post
    =
    (fun (x: i64) (out: t_Result u128 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i64) ->
      if x <. mk_i64 0 || (cast (x <: i64) <: u128) >. impl_10__MAX
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u128 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i64) <: u128) <: t_Result u128 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_120: t_TryFrom usize i64 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i64) -> true);
    f_try_from_post
    =
    (fun (x: i64) (out: t_Result usize Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i64) ->
      if x <. mk_i64 0 || (cast (x <: i64) <: u128) >. (cast (impl_11__MAX <: usize) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result usize Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i64) <: usize)
        <:
        t_Result usize Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_121: t_TryFrom u8 i128 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i128) -> true);
    f_try_from_post
    =
    (fun (x: i128) (out: t_Result u8 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i128) ->
      if x <. mk_i128 0 || (cast (x <: i128) <: u128) >. (cast (impl_6__MAX <: u8) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u8 Core_models.Num.Error.t_TryFromIntError
      else Result_Ok (cast (x <: i128) <: u8) <: t_Result u8 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_122: t_TryFrom u16 i128 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i128) -> true);
    f_try_from_post
    =
    (fun (x: i128) (out: t_Result u16 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i128) ->
      if x <. mk_i128 0 || (cast (x <: i128) <: u128) >. (cast (impl_7__MAX <: u16) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u16 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i128) <: u16) <: t_Result u16 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_123: t_TryFrom u32 i128 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i128) -> true);
    f_try_from_post
    =
    (fun (x: i128) (out: t_Result u32 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i128) ->
      if x <. mk_i128 0 || (cast (x <: i128) <: u128) >. (cast (impl_8__MAX <: u32) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u32 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i128) <: u32) <: t_Result u32 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_124: t_TryFrom u64 i128 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i128) -> true);
    f_try_from_post
    =
    (fun (x: i128) (out: t_Result u64 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i128) ->
      if x <. mk_i128 0 || (cast (x <: i128) <: u128) >. (cast (impl_9__MAX <: u64) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u64 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i128) <: u64) <: t_Result u64 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_125: t_TryFrom u128 i128 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i128) -> true);
    f_try_from_post
    =
    (fun (x: i128) (out: t_Result u128 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i128) ->
      if x <. mk_i128 0 || (cast (x <: i128) <: u128) >. impl_10__MAX
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u128 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i128) <: u128)
        <:
        t_Result u128 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_126: t_TryFrom usize i128 =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: i128) -> true);
    f_try_from_post
    =
    (fun (x: i128) (out: t_Result usize Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: i128) ->
      if x <. mk_i128 0 || (cast (x <: i128) <: u128) >. (cast (impl_11__MAX <: usize) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result usize Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: i128) <: usize)
        <:
        t_Result usize Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_127: t_TryFrom u8 isize =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: isize) -> true);
    f_try_from_post
    =
    (fun (x: isize) (out: t_Result u8 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: isize) ->
      if x <. mk_isize 0 || (cast (x <: isize) <: u128) >. (cast (impl_6__MAX <: u8) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u8 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: isize) <: u8) <: t_Result u8 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_128: t_TryFrom u16 isize =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: isize) -> true);
    f_try_from_post
    =
    (fun (x: isize) (out: t_Result u16 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: isize) ->
      if x <. mk_isize 0 || (cast (x <: isize) <: u128) >. (cast (impl_7__MAX <: u16) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u16 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: isize) <: u16) <: t_Result u16 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_129: t_TryFrom u32 isize =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: isize) -> true);
    f_try_from_post
    =
    (fun (x: isize) (out: t_Result u32 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: isize) ->
      if x <. mk_isize 0 || (cast (x <: isize) <: u128) >. (cast (impl_8__MAX <: u32) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u32 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: isize) <: u32) <: t_Result u32 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_130: t_TryFrom u64 isize =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: isize) -> true);
    f_try_from_post
    =
    (fun (x: isize) (out: t_Result u64 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: isize) ->
      if x <. mk_isize 0 || (cast (x <: isize) <: u128) >. (cast (impl_9__MAX <: u64) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u64 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: isize) <: u64) <: t_Result u64 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_131: t_TryFrom u128 isize =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: isize) -> true);
    f_try_from_post
    =
    (fun (x: isize) (out: t_Result u128 Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: isize) ->
      if x <. mk_isize 0 || (cast (x <: isize) <: u128) >. impl_10__MAX
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result u128 Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: isize) <: u128)
        <:
        t_Result u128 Core_models.Num.Error.t_TryFromIntError
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_132: t_TryFrom usize isize =
  {
    f_Error = Core_models.Num.Error.t_TryFromIntError;
    f_try_from_pre = (fun (x: isize) -> true);
    f_try_from_post
    =
    (fun (x: isize) (out: t_Result usize Core_models.Num.Error.t_TryFromIntError) -> true);
    f_try_from
    =
    fun (x: isize) ->
      if x <. mk_isize 0 || (cast (x <: isize) <: u128) >. (cast (impl_11__MAX <: usize) <: u128)
      then
        Result_Err
        (Core_models.Num.Error.TryFromIntError (() <: Prims.unit)
          <:
          Core_models.Num.Error.t_TryFromIntError)
        <:
        t_Result usize Core_models.Num.Error.t_TryFromIntError
      else
        Result_Ok (cast (x <: isize) <: usize)
        <:
        t_Result usize Core_models.Num.Error.t_TryFromIntError
  }

let impl__new__from__chain
      (#v_A #v_B: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_A)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Iterator v_B)
      (#_: unit{i1.f_Item == i0.f_Item})
      (a: v_A)
      (b: v_B)
    : t_Chain v_A v_B = { f_a = Option_Some a <: t_Option v_A; f_b = b } <: t_Chain v_A v_B

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_1__from__chain':
    #v_A: Type0 ->
    #v_B: Type0 ->
    {| i0: t_Iterator v_A |} ->
    {| i1: t_Iterator v_B |} ->
    #_: unit{i1.f_Item == i0.f_Item}
  -> t_Iterator (t_Chain v_A v_B)

unfold
let impl_1__from__chain
      (#v_A #v_B: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_A)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Iterator v_B)
      (#_: unit{i1.f_Item == i0.f_Item})
     = impl_1__from__chain' #v_A #v_B #i0 #i1 #_

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_1__from__enumerate
      (#v_I: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
    : t_Iterator (t_Enumerate v_I) =
  {
    f_Item = (usize & i0.f_Item);
    f_next_pre = (fun (self: t_Enumerate v_I) -> true);
    f_next_post
    =
    (fun (self: t_Enumerate v_I) (out1: (t_Enumerate v_I & t_Option (usize & i0.f_Item))) -> true);
    f_next
    =
    fun (self: t_Enumerate v_I) ->
      let (tmp0: v_I), (out: t_Option i0.f_Item) =
        f_next #v_I #FStar.Tactics.Typeclasses.solve self.f_iter
      in
      let self:t_Enumerate v_I = { self with f_iter = tmp0 } <: t_Enumerate v_I in
      let (self: t_Enumerate v_I), (hax_temp_output: t_Option (usize & i0.f_Item)) =
        match out <: t_Option i0.f_Item with
        | Option_Some a ->
          let i:usize = self.f_count in
          let _:Prims.unit = Hax_lib.v_assume (b2t (self.f_count <. impl_11__MAX <: bool)) in
          let self:t_Enumerate v_I =
            { self with f_count = self.f_count +! mk_usize 1 } <: t_Enumerate v_I
          in
          self, (Option_Some (i, a <: (usize & i0.f_Item)) <: t_Option (usize & i0.f_Item))
          <:
          (t_Enumerate v_I & t_Option (usize & i0.f_Item))
        | Option_None  ->
          self, (Option_None <: t_Option (usize & i0.f_Item))
          <:
          (t_Enumerate v_I & t_Option (usize & i0.f_Item))
      in
      self, hax_temp_output <: (t_Enumerate v_I & t_Option (usize & i0.f_Item))
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_1__from__filter':
    #v_I: Type0 ->
    #v_P: Type0 ->
    {| i0: t_Iterator v_I |} ->
    {| i1: Core_models.Ops.Function.t_FnMut v_P i0.f_Item |}
  -> t_Iterator (t_Filter v_I v_P)

unfold
let impl_1__from__filter
      (#v_I #v_P: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: Core_models.Ops.Function.t_FnMut v_P i0.f_Item)
     = impl_1__from__filter' #v_I #v_P #i0 #i1

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_1__from__filter_map':
    #v_I: Type0 ->
    #v_B: Type0 ->
    #v_F: Type0 ->
    {| i0: t_Iterator v_I |} ->
    {| i1: Core_models.Ops.Function.t_FnMut v_F i0.f_Item |}
  -> t_Iterator (t_FilterMap v_I v_F)

unfold
let impl_1__from__filter_map
      (#v_I #v_B #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
     = impl_1__from__filter_map' #v_I #v_B #v_F #i0 #i1

let impl__new__from__flat_map
      (#v_I #v_U #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Iterator v_U)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i2: Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
      (it: v_I)
      (f: v_F)
    : t_FlatMap v_I v_U v_F =
  { f_it = it; f_f = f; f_current = Option_None <: t_Option v_U } <: t_FlatMap v_I v_U v_F

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_1__from__flat_map':
    #v_I: Type0 ->
    #v_U: Type0 ->
    #v_F: Type0 ->
    {| i0: t_Iterator v_I |} ->
    {| i1: t_Iterator v_U |} ->
    {| i2: Core_models.Ops.Function.t_FnMut v_F i0.f_Item |}
  -> t_Iterator (t_FlatMap v_I v_U v_F)

unfold
let impl_1__from__flat_map
      (#v_I #v_U #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Iterator v_U)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i2: Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
     = impl_1__from__flat_map' #v_I #v_U #v_F #i0 #i1 #i2

noeq

/// See [`std::iter::Flatten`]
type t_Flatten (v_I: Type0) {| i0: t_Iterator v_I |} {| i1: t_Iterator i0.f_Item |} = {
  f_it:v_I;
  f_current:t_Option i0.f_Item
}

let impl__new__from__flatten
      (#v_I: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Iterator i0.f_Item)
      (it: v_I)
    : t_Flatten v_I = { f_it = it; f_current = Option_None <: t_Option i0.f_Item } <: t_Flatten v_I

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_1__from__flatten':
    #v_I: Type0 ->
    {| i0: t_Iterator v_I |} ->
    {| i1: t_Iterator i0.f_Item |}
  -> t_Iterator (t_Flatten v_I)

unfold
let impl_1__from__flatten
      (#v_I: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Iterator i0.f_Item)
     = impl_1__from__flatten' #v_I #i0 #i1

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_1__from__fuse': #v_I: Type0 -> {| i0: t_Iterator v_I |} -> t_Iterator (t_Fuse v_I)

unfold
let impl_1__from__fuse (#v_I: Type0) (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I) =
  impl_1__from__fuse' #v_I #i0

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_1__from__inspect':
    #v_I: Type0 ->
    #v_F: Type0 ->
    {| i0: t_Iterator v_I |} ->
    {| i1: Core_models.Ops.Function.t_FnMut v_F i0.f_Item |}
  -> t_Iterator (t_Inspect v_I v_F)

unfold
let impl_1__from__inspect
      (#v_I #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
     = impl_1__from__inspect' #v_I #v_F #i0 #i1

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_1__from__map':
    #v_I: Type0 ->
    #v_O: Type0 ->
    #v_F: Type0 ->
    {| i0: t_Iterator v_I |} ->
    {| i1: Core_models.Ops.Function.t_FnMut v_F i0.f_Item |}
  -> t_Iterator (t_Map v_I v_F)

unfold
let impl_1__from__map
      (#v_I #v_O #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
     = impl_1__from__map' #v_I #v_O #v_F #i0 #i1

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_1__from__map_while':
    #v_I: Type0 ->
    #v_B: Type0 ->
    #v_F: Type0 ->
    {| i0: t_Iterator v_I |} ->
    {| i1: Core_models.Ops.Function.t_FnMut v_F i0.f_Item |}
  -> t_Iterator (t_MapWhile v_I v_F)

unfold
let impl_1__from__map_while
      (#v_I #v_B #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
     = impl_1__from__map_while' #v_I #v_B #v_F #i0 #i1

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_1__from__skip': #v_I: Type0 -> {| i0: t_Iterator v_I |} -> t_Iterator (t_Skip v_I)

unfold
let impl_1__from__skip (#v_I: Type0) (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I) =
  impl_1__from__skip' #v_I #i0

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_1__from__skip_while':
    #v_I: Type0 ->
    #v_P: Type0 ->
    {| i0: t_Iterator v_I |} ->
    {| i1: Core_models.Ops.Function.t_FnMut v_P i0.f_Item |}
  -> t_Iterator (t_SkipWhile v_I v_P)

unfold
let impl_1__from__skip_while
      (#v_I #v_P: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: Core_models.Ops.Function.t_FnMut v_P i0.f_Item)
     = impl_1__from__skip_while' #v_I #v_P #i0 #i1

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_1__from__step_by': #v_I: Type0 -> {| i0: t_Iterator v_I |} -> t_Iterator (t_StepBy v_I)

unfold
let impl_1__from__step_by
      (#v_I: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
     = impl_1__from__step_by' #v_I #i0

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_1__from__take (#v_I: Type0) (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
    : t_Iterator (t_Take v_I) =
  {
    f_Item = i0.f_Item;
    f_next_pre = (fun (self: t_Take v_I) -> true);
    f_next_post = (fun (self: t_Take v_I) (out1: (t_Take v_I & t_Option i0.f_Item)) -> true);
    f_next
    =
    fun (self: t_Take v_I) ->
      let (self: t_Take v_I), (hax_temp_output: t_Option i0.f_Item) =
        if self.f_n <>. mk_usize 0
        then
          let self:t_Take v_I = { self with f_n = self.f_n -! mk_usize 1 } <: t_Take v_I in
          let (tmp0: v_I), (out: t_Option i0.f_Item) =
            f_next #v_I #FStar.Tactics.Typeclasses.solve self.f_iter
          in
          let self:t_Take v_I = { self with f_iter = tmp0 } <: t_Take v_I in
          self, out <: (t_Take v_I & t_Option i0.f_Item)
        else self, (Option_None <: t_Option i0.f_Item) <: (t_Take v_I & t_Option i0.f_Item)
      in
      self, hax_temp_output <: (t_Take v_I & t_Option i0.f_Item)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_1__from__take_while':
    #v_I: Type0 ->
    #v_P: Type0 ->
    {| i0: t_Iterator v_I |} ->
    {| i1: Core_models.Ops.Function.t_FnMut v_P i0.f_Item |}
  -> t_Iterator (t_TakeWhile v_I v_P)

unfold
let impl_1__from__take_while
      (#v_I #v_P: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: Core_models.Ops.Function.t_FnMut v_P i0.f_Item)
     = impl_1__from__take_while' #v_I #v_P #i0 #i1

let impl__new__from__zip
      (#v_I1 #v_I2: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I1)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Iterator v_I2)
      (it1: v_I1)
      (it2: v_I2)
    : t_Zip v_I1 v_I2 = { f_it1 = it1; f_it2 = it2 } <: t_Zip v_I1 v_I2

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_1__from__zip':
    #v_I1: Type0 ->
    #v_I2: Type0 ->
    {| i0: t_Iterator v_I1 |} ->
    {| i1: t_Iterator v_I2 |}
  -> t_Iterator (t_Zip v_I1 v_I2)

unfold
let impl_1__from__zip
      (#v_I1 #v_I2: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I1)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Iterator v_I2)
     = impl_1__from__zip' #v_I1 #v_I2 #i0 #i1

assume
val iter_fold':
    #v_I: Type0 ->
    #v_B: Type0 ->
    #v_F: Type0 ->
    {| i0: t_Iterator v_I |} ->
    {| i1: Core_models.Ops.Function.t_FnMut v_F (v_B & i0.f_Item) |} ->
    iter: v_I ->
    init: v_B ->
    f: v_F
  -> v_B

unfold
let iter_fold
      (#v_I #v_B #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F (v_B & i0.f_Item))
     = iter_fold' #v_I #v_B #v_F #i0 #i1

assume
val iter_all':
    #v_I: Type0 ->
    #v_F: Type0 ->
    {| i0: t_Iterator v_I |} ->
    {| i1: Core_models.Ops.Function.t_FnMut v_F i0.f_Item |} ->
    iter: v_I ->
    f: v_F
  -> (v_I & bool)

unfold
let iter_all
      (#v_I #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
     = iter_all' #v_I #v_F #i0 #i1

assume
val iter_any':
    #v_I: Type0 ->
    #v_F: Type0 ->
    {| i0: t_Iterator v_I |} ->
    {| i1: Core_models.Ops.Function.t_FnMut v_F i0.f_Item |} ->
    iter: v_I ->
    f: v_F
  -> (v_I & bool)

unfold
let iter_any
      (#v_I #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
     = iter_any' #v_I #v_F #i0 #i1

assume
val iter_find':
    #v_I: Type0 ->
    #v_P: Type0 ->
    {| i0: t_Iterator v_I |} ->
    {| i1: Core_models.Ops.Function.t_FnMut v_P i0.f_Item |} ->
    iter: v_I ->
    predicate: v_P
  -> (v_I & t_Option i0.f_Item)

unfold
let iter_find
      (#v_I #v_P: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: Core_models.Ops.Function.t_FnMut v_P i0.f_Item)
     = iter_find' #v_I #v_P #i0 #i1

assume
val iter_find_map':
    #v_I: Type0 ->
    #v_B: Type0 ->
    #v_F: Type0 ->
    {| i0: t_Iterator v_I |} ->
    {| i1: Core_models.Ops.Function.t_FnMut v_F i0.f_Item |} ->
    iter: v_I ->
    f: v_F
  -> (v_I & t_Option v_B)

unfold
let iter_find_map
      (#v_I #v_B #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
     = iter_find_map' #v_I #v_B #v_F #i0 #i1

assume
val iter_position':
    #v_I: Type0 ->
    #v_P: Type0 ->
    {| i0: t_Iterator v_I |} ->
    {| i1: Core_models.Ops.Function.t_FnMut v_P i0.f_Item |} ->
    iter: v_I ->
    predicate: v_P
  -> (v_I & t_Option usize)

unfold
let iter_position
      (#v_I #v_P: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: Core_models.Ops.Function.t_FnMut v_P i0.f_Item)
     = iter_position' #v_I #v_P #i0 #i1

assume
val iter_count': #v_I: Type0 -> {| i0: t_Iterator v_I |} -> iter: v_I -> usize

unfold
let iter_count (#v_I: Type0) (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I) =
  iter_count' #v_I #i0

assume
val iter_nth': #v_I: Type0 -> {| i0: t_Iterator v_I |} -> iter: v_I -> n: usize
  -> t_Option i0.f_Item

unfold
let iter_nth (#v_I: Type0) (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I) =
  iter_nth' #v_I #i0

assume
val iter_last': #v_I: Type0 -> {| i0: t_Iterator v_I |} -> iter: v_I -> t_Option i0.f_Item

unfold
let iter_last (#v_I: Type0) (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I) =
  iter_last' #v_I #i0

assume
val iter_for_each':
    #v_I: Type0 ->
    #v_F: Type0 ->
    {| i0: t_Iterator v_I |} ->
    {| i1: Core_models.Ops.Function.t_FnMut v_F i0.f_Item |} ->
    iter: v_I ->
    f: v_F
  -> Prims.unit

unfold
let iter_for_each
      (#v_I #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
     = iter_for_each' #v_I #v_F #i0 #i1

assume
val iter_reduce':
    #v_I: Type0 ->
    #v_F: Type0 ->
    {| i0: t_Iterator v_I |} ->
    {| i1: Core_models.Ops.Function.t_FnMut v_F (i0.f_Item & i0.f_Item) |} ->
    iter: v_I ->
    f: v_F
  -> t_Option i0.f_Item

unfold
let iter_reduce
      (#v_I #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F (i0.f_Item & i0.f_Item))
     = iter_reduce' #v_I #v_F #i0 #i1

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_15__from__range: t_Iterator (t_Range u8) =
  {
    f_Item = u8;
    f_next_pre = (fun (self: t_Range u8) -> true);
    f_next_post = (fun (self: t_Range u8) (out: (t_Range u8 & t_Option u8)) -> true);
    f_next
    =
    fun (self: t_Range u8) ->
      let (self: t_Range u8), (hax_temp_output: t_Option u8) =
        if self.f_start >=. self.f_end
        then self, (Option_None <: t_Option u8) <: (t_Range u8 & t_Option u8)
        else
          let res:u8 = self.f_start in
          let self:t_Range u8 = { self with f_start = self.f_start +! mk_u8 1 } <: t_Range u8 in
          self, (Option_Some res <: t_Option u8) <: (t_Range u8 & t_Option u8)
      in
      self, hax_temp_output <: (t_Range u8 & t_Option u8)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_16__from__range: t_Iterator (t_Range u16) =
  {
    f_Item = u16;
    f_next_pre = (fun (self: t_Range u16) -> true);
    f_next_post = (fun (self: t_Range u16) (out: (t_Range u16 & t_Option u16)) -> true);
    f_next
    =
    fun (self: t_Range u16) ->
      let (self: t_Range u16), (hax_temp_output: t_Option u16) =
        if self.f_start >=. self.f_end
        then self, (Option_None <: t_Option u16) <: (t_Range u16 & t_Option u16)
        else
          let res:u16 = self.f_start in
          let self:t_Range u16 = { self with f_start = self.f_start +! mk_u16 1 } <: t_Range u16 in
          self, (Option_Some res <: t_Option u16) <: (t_Range u16 & t_Option u16)
      in
      self, hax_temp_output <: (t_Range u16 & t_Option u16)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_17__from__range: t_Iterator (t_Range u32) =
  {
    f_Item = u32;
    f_next_pre = (fun (self: t_Range u32) -> true);
    f_next_post = (fun (self: t_Range u32) (out: (t_Range u32 & t_Option u32)) -> true);
    f_next
    =
    fun (self: t_Range u32) ->
      let (self: t_Range u32), (hax_temp_output: t_Option u32) =
        if self.f_start >=. self.f_end
        then self, (Option_None <: t_Option u32) <: (t_Range u32 & t_Option u32)
        else
          let res:u32 = self.f_start in
          let self:t_Range u32 = { self with f_start = self.f_start +! mk_u32 1 } <: t_Range u32 in
          self, (Option_Some res <: t_Option u32) <: (t_Range u32 & t_Option u32)
      in
      self, hax_temp_output <: (t_Range u32 & t_Option u32)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_18__from__range: t_Iterator (t_Range u64) =
  {
    f_Item = u64;
    f_next_pre = (fun (self: t_Range u64) -> true);
    f_next_post = (fun (self: t_Range u64) (out: (t_Range u64 & t_Option u64)) -> true);
    f_next
    =
    fun (self: t_Range u64) ->
      let (self: t_Range u64), (hax_temp_output: t_Option u64) =
        if self.f_start >=. self.f_end
        then self, (Option_None <: t_Option u64) <: (t_Range u64 & t_Option u64)
        else
          let res:u64 = self.f_start in
          let self:t_Range u64 = { self with f_start = self.f_start +! mk_u64 1 } <: t_Range u64 in
          self, (Option_Some res <: t_Option u64) <: (t_Range u64 & t_Option u64)
      in
      self, hax_temp_output <: (t_Range u64 & t_Option u64)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_19__from__range: t_Iterator (t_Range u128) =
  {
    f_Item = u128;
    f_next_pre = (fun (self: t_Range u128) -> true);
    f_next_post = (fun (self: t_Range u128) (out: (t_Range u128 & t_Option u128)) -> true);
    f_next
    =
    fun (self: t_Range u128) ->
      let (self: t_Range u128), (hax_temp_output: t_Option u128) =
        if self.f_start >=. self.f_end
        then self, (Option_None <: t_Option u128) <: (t_Range u128 & t_Option u128)
        else
          let res:u128 = self.f_start in
          let self:t_Range u128 =
            { self with f_start = self.f_start +! mk_u128 1 } <: t_Range u128
          in
          self, (Option_Some res <: t_Option u128) <: (t_Range u128 & t_Option u128)
      in
      self, hax_temp_output <: (t_Range u128 & t_Option u128)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_20__from__range: t_Iterator (t_Range usize) =
  {
    f_Item = usize;
    f_next_pre = (fun (self: t_Range usize) -> true);
    f_next_post = (fun (self: t_Range usize) (out: (t_Range usize & t_Option usize)) -> true);
    f_next
    =
    fun (self: t_Range usize) ->
      let (self: t_Range usize), (hax_temp_output: t_Option usize) =
        if self.f_start >=. self.f_end
        then self, (Option_None <: t_Option usize) <: (t_Range usize & t_Option usize)
        else
          let res:usize = self.f_start in
          let self:t_Range usize =
            { self with f_start = self.f_start +! mk_usize 1 } <: t_Range usize
          in
          self, (Option_Some res <: t_Option usize) <: (t_Range usize & t_Option usize)
      in
      self, hax_temp_output <: (t_Range usize & t_Option usize)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_21__from__range: t_Iterator (t_Range i8) =
  {
    f_Item = i8;
    f_next_pre = (fun (self: t_Range i8) -> true);
    f_next_post = (fun (self: t_Range i8) (out: (t_Range i8 & t_Option i8)) -> true);
    f_next
    =
    fun (self: t_Range i8) ->
      let (self: t_Range i8), (hax_temp_output: t_Option i8) =
        if self.f_start >=. self.f_end
        then self, (Option_None <: t_Option i8) <: (t_Range i8 & t_Option i8)
        else
          let res:i8 = self.f_start in
          let self:t_Range i8 = { self with f_start = self.f_start +! mk_i8 1 } <: t_Range i8 in
          self, (Option_Some res <: t_Option i8) <: (t_Range i8 & t_Option i8)
      in
      self, hax_temp_output <: (t_Range i8 & t_Option i8)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_22__from__range: t_Iterator (t_Range i16) =
  {
    f_Item = i16;
    f_next_pre = (fun (self: t_Range i16) -> true);
    f_next_post = (fun (self: t_Range i16) (out: (t_Range i16 & t_Option i16)) -> true);
    f_next
    =
    fun (self: t_Range i16) ->
      let (self: t_Range i16), (hax_temp_output: t_Option i16) =
        if self.f_start >=. self.f_end
        then self, (Option_None <: t_Option i16) <: (t_Range i16 & t_Option i16)
        else
          let res:i16 = self.f_start in
          let self:t_Range i16 = { self with f_start = self.f_start +! mk_i16 1 } <: t_Range i16 in
          self, (Option_Some res <: t_Option i16) <: (t_Range i16 & t_Option i16)
      in
      self, hax_temp_output <: (t_Range i16 & t_Option i16)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_23__from__range: t_Iterator (t_Range i32) =
  {
    f_Item = i32;
    f_next_pre = (fun (self: t_Range i32) -> true);
    f_next_post = (fun (self: t_Range i32) (out: (t_Range i32 & t_Option i32)) -> true);
    f_next
    =
    fun (self: t_Range i32) ->
      let (self: t_Range i32), (hax_temp_output: t_Option i32) =
        if self.f_start >=. self.f_end
        then self, (Option_None <: t_Option i32) <: (t_Range i32 & t_Option i32)
        else
          let res:i32 = self.f_start in
          let self:t_Range i32 = { self with f_start = self.f_start +! mk_i32 1 } <: t_Range i32 in
          self, (Option_Some res <: t_Option i32) <: (t_Range i32 & t_Option i32)
      in
      self, hax_temp_output <: (t_Range i32 & t_Option i32)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_24__from__range: t_Iterator (t_Range i64) =
  {
    f_Item = i64;
    f_next_pre = (fun (self: t_Range i64) -> true);
    f_next_post = (fun (self: t_Range i64) (out: (t_Range i64 & t_Option i64)) -> true);
    f_next
    =
    fun (self: t_Range i64) ->
      let (self: t_Range i64), (hax_temp_output: t_Option i64) =
        if self.f_start >=. self.f_end
        then self, (Option_None <: t_Option i64) <: (t_Range i64 & t_Option i64)
        else
          let res:i64 = self.f_start in
          let self:t_Range i64 = { self with f_start = self.f_start +! mk_i64 1 } <: t_Range i64 in
          self, (Option_Some res <: t_Option i64) <: (t_Range i64 & t_Option i64)
      in
      self, hax_temp_output <: (t_Range i64 & t_Option i64)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_25__from__range: t_Iterator (t_Range i128) =
  {
    f_Item = i128;
    f_next_pre = (fun (self: t_Range i128) -> true);
    f_next_post = (fun (self: t_Range i128) (out: (t_Range i128 & t_Option i128)) -> true);
    f_next
    =
    fun (self: t_Range i128) ->
      let (self: t_Range i128), (hax_temp_output: t_Option i128) =
        if self.f_start >=. self.f_end
        then self, (Option_None <: t_Option i128) <: (t_Range i128 & t_Option i128)
        else
          let res:i128 = self.f_start in
          let self:t_Range i128 =
            { self with f_start = self.f_start +! mk_i128 1 } <: t_Range i128
          in
          self, (Option_Some res <: t_Option i128) <: (t_Range i128 & t_Option i128)
      in
      self, hax_temp_output <: (t_Range i128 & t_Option i128)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_26__from__range: t_Iterator (t_Range isize) =
  {
    f_Item = isize;
    f_next_pre = (fun (self: t_Range isize) -> true);
    f_next_post = (fun (self: t_Range isize) (out: (t_Range isize & t_Option isize)) -> true);
    f_next
    =
    fun (self: t_Range isize) ->
      let (self: t_Range isize), (hax_temp_output: t_Option isize) =
        if self.f_start >=. self.f_end
        then self, (Option_None <: t_Option isize) <: (t_Range isize & t_Option isize)
        else
          let res:isize = self.f_start in
          let self:t_Range isize =
            { self with f_start = self.f_start +! mk_isize 1 } <: t_Range isize
          in
          self, (Option_Some res <: t_Option isize) <: (t_Range isize & t_Option isize)
      in
      self, hax_temp_output <: (t_Range isize & t_Option isize)
  }

let bound_le
      (#v_A #v_B: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_PartialOrd v_A v_B)
      (a: v_A)
      (b: v_B)
    : bool =
  match f_partial_cmp #v_A #v_B #FStar.Tactics.Typeclasses.solve a b <: t_Option t_Ordering with
  | Option_Some (Ordering_Less ) | Option_Some (Ordering_Equal ) -> true
  | _ -> false

let bound_lt
      (#v_A #v_B: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_PartialOrd v_A v_B)
      (a: v_A)
      (b: v_B)
    : bool =
  match f_partial_cmp #v_A #v_B #FStar.Tactics.Typeclasses.solve a b <: t_Option t_Ordering with
  | Option_Some (Ordering_Less ) -> true
  | _ -> false

let after_start
      (#v_T #v_U: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_PartialOrd v_T v_U)
      (start: t_Bound v_T)
      (item: v_U)
    : bool =
  match start <: t_Bound v_T with
  | Bound_Included start -> bound_le #v_T #v_U start item
  | Bound_Excluded start -> bound_lt #v_T #v_U start item
  | Bound_Unbounded  -> true

let before_end
      (#v_T #v_U: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_PartialOrd v_U v_T)
      (v_end: t_Bound v_T)
      (item: v_U)
    : bool =
  match v_end <: t_Bound v_T with
  | Bound_Included v_end -> bound_le #v_U #v_T item v_end
  | Bound_Excluded v_end -> bound_lt #v_U #v_T item v_end
  | Bound_Unbounded  -> true

/// See [`std::ops::Range::contains`]
let impl_2__contains
      (#v_T #v_U: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_PartialOrd v_T v_T)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_PartialOrd v_T v_U)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i2: t_PartialOrd v_U v_T)
      (self: t_Range v_T)
      (item: v_U)
    : bool = bound_le #v_T #v_U self.f_start item && bound_lt #v_U #v_T item self.f_end

/// See [`std::ops::Range::is_empty`]
let impl_2__is_empty
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_PartialOrd v_T v_T)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_PartialOrd v_T v_T)
      (self: t_Range v_T)
    : bool = ~.(bound_lt #v_T #v_T self.f_start self.f_end <: bool)

/// See [`std::ops::RangeFrom::contains`]
let impl_4__contains
      (#v_T #v_U: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_PartialOrd v_T v_T)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_PartialOrd v_T v_U)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i2: t_PartialOrd v_U v_T)
      (self: t_RangeFrom v_T)
      (item: v_U)
    : bool = bound_le #v_T #v_U self.f_start item

/// See [`std::ops::RangeTo::contains`]
let impl_6__contains
      (#v_T #v_U: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_PartialOrd v_T v_T)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_PartialOrd v_T v_U)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i2: t_PartialOrd v_U v_T)
      (self: t_RangeTo v_T)
      (item: v_U)
    : bool = bound_lt #v_U #v_T item self.f_end

/// See [`std::ops::RangeInclusive::contains`]
let impl_10__contains
      (#v_T #v_U: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_PartialOrd v_T v_T)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_PartialOrd v_T v_U)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i2: t_PartialOrd v_U v_T)
      (self: t_RangeInclusive v_T)
      (item: v_U)
    : bool =
  after_start #v_T
    #v_U
    (f_start_bound #(t_RangeInclusive v_T) #v_T #FStar.Tactics.Typeclasses.solve self <: t_Bound v_T
    )
    item &&
  before_end #v_T
    #v_U
    (f_end_bound #(t_RangeInclusive v_T) #v_T #FStar.Tactics.Typeclasses.solve self <: t_Bound v_T)
    item

/// See [`std::ops::RangeInclusive::is_empty`]
let impl_10__is_empty
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_PartialOrd v_T v_T)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_PartialOrd v_T v_T)
      (self: t_RangeInclusive v_T)
    : bool = self.f_exhausted || ~.(bound_le #v_T #v_T self.f_start_ self.f_end_ <: bool)

/// See [`std::ops::RangeToInclusive::contains`]
let impl_12__contains
      (#v_T #v_U: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_PartialOrd v_T v_T)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_PartialOrd v_T v_U)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i2: t_PartialOrd v_U v_T)
      (self: t_RangeToInclusive v_T)
      (item: v_U)
    : bool = bound_le #v_U #v_T item self.f_end

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_4__from__result (#v_A: Type0) : t_Iterator (t_SeqIter v_A) =
  {
    f_Item = v_A;
    f_next_pre = (fun (self: t_SeqIter v_A) -> true);
    f_next_post = (fun (self: t_SeqIter v_A) (out1: (t_SeqIter v_A & t_Option v_A)) -> true);
    f_next
    =
    fun (self: t_SeqIter v_A) ->
      let (self: t_SeqIter v_A), (hax_temp_output: t_Option v_A) =
        if (Rust_primitives.Sequence.seq_len #v_A self._0 <: usize) =. mk_usize 0
        then self, (Option_None <: t_Option v_A) <: (t_SeqIter v_A & t_Option v_A)
        else
          let (tmp0: Rust_primitives.Sequence.t_Seq v_A), (out: v_A) =
            Rust_primitives.Sequence.seq_remove #v_A self._0 (mk_usize 0)
          in
          let self:t_SeqIter v_A = { self with _0 = tmp0 } <: t_SeqIter v_A in
          self, (Option_Some out <: t_Option v_A) <: (t_SeqIter v_A & t_Option v_A)
      in
      self, hax_temp_output <: (t_SeqIter v_A & t_Option v_A)
  }

/// See [`std::cmp::Ord`]
class t_Ord (v_Self: Type0) = {
  [@@@ FStar.Tactics.Typeclasses.no_method]_super_i0:t_Eq v_Self;
  [@@@ FStar.Tactics.Typeclasses.no_method]_super_i1:t_PartialOrd v_Self v_Self;
  f_cmp_pre:self_: v_Self -> other: v_Self -> pred: Type0{true ==> pred};
  f_cmp_post:v_Self -> v_Self -> t_Ordering -> Type0;
  f_cmp:x0: v_Self -> x1: v_Self
    -> Prims.Pure t_Ordering (f_cmp_pre x0 x1) (fun result -> f_cmp_post x0 x1 result)
}

[@@ FStar.Tactics.Typeclasses.tcinstance]
let _ = fun (v_Self:Type0) {|i: t_Ord v_Self|} -> i._super_i0

[@@ FStar.Tactics.Typeclasses.tcinstance]
let _ = fun (v_Self:Type0) {|i: t_Ord v_Self|} -> i._super_i1

/// See [`std::iter::IntoIterator`]
class t_IntoIterator (v_Self: Type0) = {
  [@@@ FStar.Tactics.Typeclasses.no_method]f_Item:Type0;
  [@@@ FStar.Tactics.Typeclasses.no_method]f_IntoIter:Type0;
  f_IntoIter_i0:t_Iterator f_IntoIter;
  f_into_iter_pre:v_Self -> Type0;
  f_into_iter_post:v_Self -> f_IntoIter -> Type0;
  f_into_iter:x0: v_Self
    -> Prims.Pure f_IntoIter (f_into_iter_pre x0) (fun result -> f_into_iter_post x0 result)
}

class t_RangeBoundsDefaults (v_Self: Type0) (v_T: Type0) = {
  f_contains_pre:
      #v_U: Type0 ->
      {| i1: t_RangeBounds v_Self v_T |} ->
      {| i2: t_PartialOrd v_T v_U |} ->
      {| i3: t_PartialOrd v_U v_T |} ->
      self_: v_Self ->
      item: v_U
    -> pred: Type0{true ==> pred};
  f_contains_post:
      #v_U: Type0 ->
      {| i1: t_RangeBounds v_Self v_T |} ->
      {| i2: t_PartialOrd v_T v_U |} ->
      {| i3: t_PartialOrd v_U v_T |} ->
      v_Self ->
      v_U ->
      bool
    -> Type0;
  f_contains:
      #v_U: Type0 ->
      {| i1: t_RangeBounds v_Self v_T |} ->
      {| i2: t_PartialOrd v_T v_U |} ->
      {| i3: t_PartialOrd v_U v_T |} ->
      x0: v_Self ->
      x1: v_U
    -> Prims.Pure bool
        (f_contains_pre #v_U #i1 #i2 #i3 x0 x1)
        (fun result -> f_contains_post #v_U #i1 #i2 #i3 x0 x1 result)
}

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_24 (#v_T: Type0) (v_N: usize) : t_IntoIterator (t_Array v_T v_N) =
  {
    f_Item = v_T;
    f_IntoIter = t_IntoIter v_T v_N;
    f_IntoIter_i0 = FStar.Tactics.Typeclasses.solve;
    f_into_iter_pre = (fun (self: t_Array v_T v_N) -> true);
    f_into_iter_post = (fun (self: t_Array v_T v_N) (out: t_IntoIter v_T v_N) -> true);
    f_into_iter
    =
    fun (self: t_Array v_T v_N) ->
      IntoIter (Rust_primitives.Sequence.seq_from_array #v_T v_N self) <: t_IntoIter v_T v_N
  }

class t_PartialOrdDefaults (v_Self: Type0) (v_Rhs: Type0) = {
  f_lt_pre:{| i1: t_PartialOrd v_Self v_Rhs |} -> self_: v_Self -> y: v_Rhs
    -> pred: Type0{true ==> pred};
  f_lt_post:{| i1: t_PartialOrd v_Self v_Rhs |} -> v_Self -> v_Rhs -> bool -> Type0;
  f_lt:{| i1: t_PartialOrd v_Self v_Rhs |} -> x0: v_Self -> x1: v_Rhs
    -> Prims.Pure bool (f_lt_pre #i1 x0 x1) (fun result -> f_lt_post #i1 x0 x1 result);
  f_le_pre:{| i1: t_PartialOrd v_Self v_Rhs |} -> self_: v_Self -> y: v_Rhs
    -> pred: Type0{true ==> pred};
  f_le_post:{| i1: t_PartialOrd v_Self v_Rhs |} -> v_Self -> v_Rhs -> bool -> Type0;
  f_le:{| i1: t_PartialOrd v_Self v_Rhs |} -> x0: v_Self -> x1: v_Rhs
    -> Prims.Pure bool (f_le_pre #i1 x0 x1) (fun result -> f_le_post #i1 x0 x1 result);
  f_gt_pre:{| i1: t_PartialOrd v_Self v_Rhs |} -> self_: v_Self -> y: v_Rhs
    -> pred: Type0{true ==> pred};
  f_gt_post:{| i1: t_PartialOrd v_Self v_Rhs |} -> v_Self -> v_Rhs -> bool -> Type0;
  f_gt:{| i1: t_PartialOrd v_Self v_Rhs |} -> x0: v_Self -> x1: v_Rhs
    -> Prims.Pure bool (f_gt_pre #i1 x0 x1) (fun result -> f_gt_post #i1 x0 x1 result);
  f_ge_pre:{| i1: t_PartialOrd v_Self v_Rhs |} -> self_: v_Self -> y: v_Rhs
    -> pred: Type0{true ==> pred};
  f_ge_post:{| i1: t_PartialOrd v_Self v_Rhs |} -> v_Self -> v_Rhs -> bool -> Type0;
  f_ge:{| i1: t_PartialOrd v_Self v_Rhs |} -> x0: v_Self -> x1: v_Rhs
    -> Prims.Pure bool (f_ge_pre #i1 x0 x1) (fun result -> f_ge_post #i1 x0 x1 result)
}

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_1__from__cmp
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_PartialOrd v_T v_T)
    : t_PartialOrdDefaults v_T v_T =
  {
    f_lt_pre
    =
    (fun
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_PartialOrd v_T v_T)
        (self: v_T)
        (y: v_T)
        ->
        true);
    f_lt_post
    =
    (fun
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_PartialOrd v_T v_T)
        (self: v_T)
        (y: v_T)
        (out: bool)
        ->
        true);
    f_lt
    =
    (fun
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_PartialOrd v_T v_T)
        (self: v_T)
        (y: v_T)
        ->
        match
          f_partial_cmp #v_T #v_T #FStar.Tactics.Typeclasses.solve self y <: t_Option t_Ordering
        with
        | Option_Some (Ordering_Less ) -> true
        | _ -> false);
    f_le_pre
    =
    (fun
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_PartialOrd v_T v_T)
        (self: v_T)
        (y: v_T)
        ->
        true);
    f_le_post
    =
    (fun
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_PartialOrd v_T v_T)
        (self: v_T)
        (y: v_T)
        (out: bool)
        ->
        true);
    f_le
    =
    (fun
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_PartialOrd v_T v_T)
        (self: v_T)
        (y: v_T)
        ->
        match
          f_partial_cmp #v_T #v_T #FStar.Tactics.Typeclasses.solve self y <: t_Option t_Ordering
        with
        | Option_Some (Ordering_Less ) | Option_Some (Ordering_Equal ) -> true
        | _ -> false);
    f_gt_pre
    =
    (fun
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_PartialOrd v_T v_T)
        (self: v_T)
        (y: v_T)
        ->
        true);
    f_gt_post
    =
    (fun
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_PartialOrd v_T v_T)
        (self: v_T)
        (y: v_T)
        (out: bool)
        ->
        true);
    f_gt
    =
    (fun
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_PartialOrd v_T v_T)
        (self: v_T)
        (y: v_T)
        ->
        match
          f_partial_cmp #v_T #v_T #FStar.Tactics.Typeclasses.solve self y <: t_Option t_Ordering
        with
        | Option_Some (Ordering_Greater ) -> true
        | _ -> false);
    f_ge_pre
    =
    (fun
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_PartialOrd v_T v_T)
        (self: v_T)
        (y: v_T)
        ->
        true);
    f_ge_post
    =
    (fun
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_PartialOrd v_T v_T)
        (self: v_T)
        (y: v_T)
        (out: bool)
        ->
        true);
    f_ge
    =
    fun (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_PartialOrd v_T v_T) (self: v_T) (y: v_T) ->
      match
        f_partial_cmp #v_T #v_T #FStar.Tactics.Typeclasses.solve self y <: t_Option t_Ordering
      with
      | Option_Some (Ordering_Greater ) | Option_Some (Ordering_Equal ) -> true
      | _ -> false
  }

/// See [`std::cmp::max`]
let max (#v_T: Type0) (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Ord v_T) (v1 v2: v_T) : v_T =
  match f_cmp #v_T #FStar.Tactics.Typeclasses.solve v1 v2 <: t_Ordering with
  | Ordering_Greater  -> v1
  | _ -> v2

/// See [`std::cmp::min`]
let min (#v_T: Type0) (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Ord v_T) (v1 v2: v_T) : v_T =
  match f_cmp #v_T #FStar.Tactics.Typeclasses.solve v1 v2 <: t_Ordering with
  | Ordering_Greater  -> v2
  | _ -> v1

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_5__from__cmp (#v_T: Type0) (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Ord v_T)
    : t_Ord (t_Reverse v_T) =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    _super_i1 = FStar.Tactics.Typeclasses.solve;
    f_cmp_pre = (fun (self: t_Reverse v_T) (other: t_Reverse v_T) -> true);
    f_cmp_post = (fun (self: t_Reverse v_T) (other: t_Reverse v_T) (out: t_Ordering) -> true);
    f_cmp
    =
    fun (self: t_Reverse v_T) (other: t_Reverse v_T) ->
      f_cmp #v_T #FStar.Tactics.Typeclasses.solve other._0 self._0
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_35__from__cmp: t_Ord u8 =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    _super_i1 = FStar.Tactics.Typeclasses.solve;
    f_cmp_pre = (fun (self: u8) (other: u8) -> true);
    f_cmp_post
    =
    (fun (self_: u8) (other: u8) (res: t_Ordering) ->
        match res <: t_Ordering with
        | Ordering_Less  -> self_ <. other
        | Ordering_Equal  -> self_ =. other
        | Ordering_Greater  -> self_ >. other);
    f_cmp
    =
    fun (self: u8) (other: u8) ->
      if self <. other
      then Ordering_Less <: t_Ordering
      else if self >. other then Ordering_Greater <: t_Ordering else Ordering_Equal <: t_Ordering
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_37__from__cmp: t_Ord i8 =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    _super_i1 = FStar.Tactics.Typeclasses.solve;
    f_cmp_pre = (fun (self: i8) (other: i8) -> true);
    f_cmp_post
    =
    (fun (self_: i8) (other: i8) (res: t_Ordering) ->
        match res <: t_Ordering with
        | Ordering_Less  -> self_ <. other
        | Ordering_Equal  -> self_ =. other
        | Ordering_Greater  -> self_ >. other);
    f_cmp
    =
    fun (self: i8) (other: i8) ->
      if self <. other
      then Ordering_Less <: t_Ordering
      else if self >. other then Ordering_Greater <: t_Ordering else Ordering_Equal <: t_Ordering
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_39__from__cmp: t_Ord u16 =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    _super_i1 = FStar.Tactics.Typeclasses.solve;
    f_cmp_pre = (fun (self: u16) (other: u16) -> true);
    f_cmp_post
    =
    (fun (self_: u16) (other: u16) (res: t_Ordering) ->
        match res <: t_Ordering with
        | Ordering_Less  -> self_ <. other
        | Ordering_Equal  -> self_ =. other
        | Ordering_Greater  -> self_ >. other);
    f_cmp
    =
    fun (self: u16) (other: u16) ->
      if self <. other
      then Ordering_Less <: t_Ordering
      else if self >. other then Ordering_Greater <: t_Ordering else Ordering_Equal <: t_Ordering
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_41__from__cmp: t_Ord i16 =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    _super_i1 = FStar.Tactics.Typeclasses.solve;
    f_cmp_pre = (fun (self: i16) (other: i16) -> true);
    f_cmp_post
    =
    (fun (self_: i16) (other: i16) (res: t_Ordering) ->
        match res <: t_Ordering with
        | Ordering_Less  -> self_ <. other
        | Ordering_Equal  -> self_ =. other
        | Ordering_Greater  -> self_ >. other);
    f_cmp
    =
    fun (self: i16) (other: i16) ->
      if self <. other
      then Ordering_Less <: t_Ordering
      else if self >. other then Ordering_Greater <: t_Ordering else Ordering_Equal <: t_Ordering
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_43__from__cmp: t_Ord u32 =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    _super_i1 = FStar.Tactics.Typeclasses.solve;
    f_cmp_pre = (fun (self: u32) (other: u32) -> true);
    f_cmp_post
    =
    (fun (self_: u32) (other: u32) (res: t_Ordering) ->
        match res <: t_Ordering with
        | Ordering_Less  -> self_ <. other
        | Ordering_Equal  -> self_ =. other
        | Ordering_Greater  -> self_ >. other);
    f_cmp
    =
    fun (self: u32) (other: u32) ->
      if self <. other
      then Ordering_Less <: t_Ordering
      else if self >. other then Ordering_Greater <: t_Ordering else Ordering_Equal <: t_Ordering
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_45__from__cmp: t_Ord i32 =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    _super_i1 = FStar.Tactics.Typeclasses.solve;
    f_cmp_pre = (fun (self: i32) (other: i32) -> true);
    f_cmp_post
    =
    (fun (self_: i32) (other: i32) (res: t_Ordering) ->
        match res <: t_Ordering with
        | Ordering_Less  -> self_ <. other
        | Ordering_Equal  -> self_ =. other
        | Ordering_Greater  -> self_ >. other);
    f_cmp
    =
    fun (self: i32) (other: i32) ->
      if self <. other
      then Ordering_Less <: t_Ordering
      else if self >. other then Ordering_Greater <: t_Ordering else Ordering_Equal <: t_Ordering
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_47__from__cmp: t_Ord u64 =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    _super_i1 = FStar.Tactics.Typeclasses.solve;
    f_cmp_pre = (fun (self: u64) (other: u64) -> true);
    f_cmp_post
    =
    (fun (self_: u64) (other: u64) (res: t_Ordering) ->
        match res <: t_Ordering with
        | Ordering_Less  -> self_ <. other
        | Ordering_Equal  -> self_ =. other
        | Ordering_Greater  -> self_ >. other);
    f_cmp
    =
    fun (self: u64) (other: u64) ->
      if self <. other
      then Ordering_Less <: t_Ordering
      else if self >. other then Ordering_Greater <: t_Ordering else Ordering_Equal <: t_Ordering
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_49__from__cmp: t_Ord i64 =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    _super_i1 = FStar.Tactics.Typeclasses.solve;
    f_cmp_pre = (fun (self: i64) (other: i64) -> true);
    f_cmp_post
    =
    (fun (self_: i64) (other: i64) (res: t_Ordering) ->
        match res <: t_Ordering with
        | Ordering_Less  -> self_ <. other
        | Ordering_Equal  -> self_ =. other
        | Ordering_Greater  -> self_ >. other);
    f_cmp
    =
    fun (self: i64) (other: i64) ->
      if self <. other
      then Ordering_Less <: t_Ordering
      else if self >. other then Ordering_Greater <: t_Ordering else Ordering_Equal <: t_Ordering
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_51__from__cmp: t_Ord u128 =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    _super_i1 = FStar.Tactics.Typeclasses.solve;
    f_cmp_pre = (fun (self: u128) (other: u128) -> true);
    f_cmp_post
    =
    (fun (self_: u128) (other: u128) (res: t_Ordering) ->
        match res <: t_Ordering with
        | Ordering_Less  -> self_ <. other
        | Ordering_Equal  -> self_ =. other
        | Ordering_Greater  -> self_ >. other);
    f_cmp
    =
    fun (self: u128) (other: u128) ->
      if self <. other
      then Ordering_Less <: t_Ordering
      else if self >. other then Ordering_Greater <: t_Ordering else Ordering_Equal <: t_Ordering
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_53__from__cmp: t_Ord i128 =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    _super_i1 = FStar.Tactics.Typeclasses.solve;
    f_cmp_pre = (fun (self: i128) (other: i128) -> true);
    f_cmp_post
    =
    (fun (self_: i128) (other: i128) (res: t_Ordering) ->
        match res <: t_Ordering with
        | Ordering_Less  -> self_ <. other
        | Ordering_Equal  -> self_ =. other
        | Ordering_Greater  -> self_ >. other);
    f_cmp
    =
    fun (self: i128) (other: i128) ->
      if self <. other
      then Ordering_Less <: t_Ordering
      else if self >. other then Ordering_Greater <: t_Ordering else Ordering_Equal <: t_Ordering
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_55__from__cmp: t_Ord usize =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    _super_i1 = FStar.Tactics.Typeclasses.solve;
    f_cmp_pre = (fun (self: usize) (other: usize) -> true);
    f_cmp_post
    =
    (fun (self_: usize) (other: usize) (res: t_Ordering) ->
        match res <: t_Ordering with
        | Ordering_Less  -> self_ <. other
        | Ordering_Equal  -> self_ =. other
        | Ordering_Greater  -> self_ >. other);
    f_cmp
    =
    fun (self: usize) (other: usize) ->
      if self <. other
      then Ordering_Less <: t_Ordering
      else if self >. other then Ordering_Greater <: t_Ordering else Ordering_Equal <: t_Ordering
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_57__from__cmp: t_Ord isize =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    _super_i1 = FStar.Tactics.Typeclasses.solve;
    f_cmp_pre = (fun (self: isize) (other: isize) -> true);
    f_cmp_post
    =
    (fun (self_: isize) (other: isize) (res: t_Ordering) ->
        match res <: t_Ordering with
        | Ordering_Less  -> self_ <. other
        | Ordering_Equal  -> self_ =. other
        | Ordering_Greater  -> self_ >. other);
    f_cmp
    =
    fun (self: isize) (other: isize) ->
      if self <. other
      then Ordering_Less <: t_Ordering
      else if self >. other then Ordering_Greater <: t_Ordering else Ordering_Equal <: t_Ordering
  }

/// See [`std::cmp::clamp`]
let clamp
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Ord v_T)
      (value min max: v_T)
    : Prims.Pure v_T
      (requires impl_58__is_le (f_cmp #v_T #FStar.Tactics.Typeclasses.solve min max <: t_Ordering))
      (fun _ -> Prims.l_True) =
  let _:Prims.unit =
    if
      ~.(impl_58__is_le (f_cmp #v_T #FStar.Tactics.Typeclasses.solve min max <: t_Ordering) <: bool)
    then Core_models.Panicking.Internal.panic #Prims.unit ()
  in
  match f_cmp #v_T #FStar.Tactics.Typeclasses.solve value min <: t_Ordering with
  | Ordering_Less  -> min
  | Ordering_Equal  -> value
  | Ordering_Greater  ->
    match f_cmp #v_T #FStar.Tactics.Typeclasses.solve value max <: t_Ordering with
    | Ordering_Greater  -> max
    | _ -> value

/// See [`std::cmp::Ord`] for `()`
[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_9__from__cmp: t_Ord Prims.unit =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    _super_i1 = FStar.Tactics.Typeclasses.solve;
    f_cmp_pre = (fun (self: Prims.unit) (other: Prims.unit) -> true);
    f_cmp_post = (fun (self: Prims.unit) (other: Prims.unit) (out: t_Ordering) -> true);
    f_cmp = fun (self: Prims.unit) (other: Prims.unit) -> Ordering_Equal <: t_Ordering
  }

assume
val iter_min': #v_I: Type0 -> {| i0: t_Iterator v_I |} -> {| i1: t_Ord i0.f_Item |} -> iter: v_I
  -> t_Option i0.f_Item

unfold
let iter_min
      (#v_I: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Ord i0.f_Item)
     = iter_min' #v_I #i0 #i1

assume
val iter_max': #v_I: Type0 -> {| i0: t_Iterator v_I |} -> {| i1: t_Ord i0.f_Item |} -> iter: v_I
  -> t_Option i0.f_Item

unfold
let iter_max
      (#v_I: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Ord i0.f_Item)
     = iter_max' #v_I #i0 #i1

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_1 (#v_I: Type0) (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
    : t_IntoIterator v_I =
  {
    f_Item = i0.f_Item;
    f_IntoIter = v_I;
    f_IntoIter_i0 = FStar.Tactics.Typeclasses.solve;
    f_into_iter_pre = (fun (self: v_I) -> true);
    f_into_iter_post = (fun (self: v_I) (out: v_I) -> true);
    f_into_iter = fun (self: v_I) -> self
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_14__from__range (#v_T #v_R: Type0) : t_RangeBoundsDefaults v_R v_T =
  {
    f_contains_pre
    =
    (fun
        (#v_U: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_RangeBounds v_R v_T)
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_PartialOrd v_T v_U)
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i2: t_PartialOrd v_U v_T)
        (self: v_R)
        (item: v_U)
        ->
        true);
    f_contains_post
    =
    (fun
        (#v_U: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_RangeBounds v_R v_T)
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_PartialOrd v_T v_U)
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i2: t_PartialOrd v_U v_T)
        (self: v_R)
        (item: v_U)
        (out: bool)
        ->
        true);
    f_contains
    =
    fun
      (#v_U: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_RangeBounds v_R v_T)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_PartialOrd v_T v_U)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i2: t_PartialOrd v_U v_T)
      (self: v_R)
      (item: v_U)
      ->
      after_start #v_T
        #v_U
        (f_start_bound #v_R #v_T #FStar.Tactics.Typeclasses.solve self <: t_Bound v_T)
        item &&
      before_end #v_T
        #v_U
        (f_end_bound #v_R #v_T #FStar.Tactics.Typeclasses.solve self <: t_Bound v_T)
        item
  }

/// See [`std::iter::FromIterator`]
class t_FromIterator (v_Self: Type0) (v_A: Type0) = {
  f_from_iter_pre:
      #v_T: Type0 ->
      {| i1: t_IntoIterator v_T |} ->
      #_: unit{i1.f_Item == v_A} ->
      iter: v_T
    -> pred: Type0{true ==> pred};
  f_from_iter_post:
      #v_T: Type0 ->
      {| i1: t_IntoIterator v_T |} ->
      #_: unit{i1.f_Item == v_A} ->
      v_T ->
      v_Self
    -> Type0;
  f_from_iter:#v_T: Type0 -> {| i1: t_IntoIterator v_T |} -> #_: unit{i1.f_Item == v_A} -> x0: v_T
    -> Prims.Pure v_Self
        (f_from_iter_pre #v_T #i1 #_ x0)
        (fun result -> f_from_iter_post #v_T #i1 #_ x0 result)
}

/// See [`std::iter::FromIterator`] for `Result`: buffers the `Ok`s, stops at
/// the first `Err`. std threads a `&mut Option<E>` through `V::from_iter`,
/// which the model cannot, hence the `Seq`. `while` rather than an early
/// `return`, which hax cannot functionalize.
[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_5__from__result':
    #v_A: Type0 ->
    #v_E: Type0 ->
    #v_V: Type0 ->
    {| i0: t_FromIterator v_V v_A |}
  -> t_FromIterator (t_Result v_V v_E) (t_Result v_A v_E)

unfold
let impl_5__from__result
      (#v_A #v_E #v_V: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_FromIterator v_V v_A)
     = impl_5__from__result' #v_A #v_E #v_V #i0

class t_IteratorMethods (v_Self: Type0) = {
  [@@@ FStar.Tactics.Typeclasses.no_method]_super_i0:t_Iterator v_Self;
  f_fold_pre:
      #v_B: Type0 ->
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (v_B & (_super_i0).f_Item) |} ->
      v_Self ->
      v_B ->
      v_F
    -> Type0;
  f_fold_post:
      #v_B: Type0 ->
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (v_B & (_super_i0).f_Item) |} ->
      v_Self ->
      v_B ->
      v_F ->
      v_B
    -> Type0;
  f_fold:
      #v_B: Type0 ->
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (v_B & (_super_i0).f_Item) |} ->
      x0: v_Self ->
      x1: v_B ->
      x2: v_F
    -> Prims.Pure v_B
        (f_fold_pre #v_B #v_F #i1 x0 x1 x2)
        (fun result -> f_fold_post #v_B #v_F #i1 x0 x1 x2 result);
  f_all_pre:
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      v_Self ->
      v_F
    -> Type0;
  f_all_post:
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      v_Self ->
      v_F ->
      (v_Self & bool)
    -> Type0;
  f_all:
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      x0: v_Self ->
      x1: v_F
    -> Prims.Pure (v_Self & bool)
        (f_all_pre #v_F #i1 x0 x1)
        (fun result -> f_all_post #v_F #i1 x0 x1 result);
  f_map_pre:
      #v_O: Type0 ->
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      v_Self ->
      v_F
    -> Type0;
  f_map_post:
      #v_O: Type0 ->
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      v_Self ->
      v_F ->
      t_Map v_Self v_F
    -> Type0;
  f_map:
      #v_O: Type0 ->
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      x0: v_Self ->
      x1: v_F
    -> Prims.Pure (t_Map v_Self v_F)
        (f_map_pre #v_O #v_F #i1 x0 x1)
        (fun result -> f_map_post #v_O #v_F #i1 x0 x1 result);
  f_enumerate_pre:v_Self -> Type0;
  f_enumerate_post:v_Self -> t_Enumerate v_Self -> Type0;
  f_enumerate:x0: v_Self
    -> Prims.Pure (t_Enumerate v_Self)
        (f_enumerate_pre x0)
        (fun result -> f_enumerate_post x0 result);
  f_step_by_pre:self_: v_Self -> step: usize -> pred: Type0{step >. mk_usize 0 ==> pred};
  f_step_by_post:v_Self -> usize -> t_StepBy v_Self -> Type0;
  f_step_by:x0: v_Self -> x1: usize
    -> Prims.Pure (t_StepBy v_Self)
        (f_step_by_pre x0 x1)
        (fun result -> f_step_by_post x0 x1 result);
  f_take_pre:v_Self -> usize -> Type0;
  f_take_post:v_Self -> usize -> t_Take v_Self -> Type0;
  f_take:x0: v_Self -> x1: usize
    -> Prims.Pure (t_Take v_Self) (f_take_pre x0 x1) (fun result -> f_take_post x0 x1 result);
  f_skip_pre:v_Self -> usize -> Type0;
  f_skip_post:v_Self -> usize -> t_Skip v_Self -> Type0;
  f_skip:x0: v_Self -> x1: usize
    -> Prims.Pure (t_Skip v_Self) (f_skip_pre x0 x1) (fun result -> f_skip_post x0 x1 result);
  f_filter_pre:
      #v_P: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_P (_super_i0).f_Item |} ->
      v_Self ->
      v_P
    -> Type0;
  f_filter_post:
      #v_P: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_P (_super_i0).f_Item |} ->
      v_Self ->
      v_P ->
      t_Filter v_Self v_P
    -> Type0;
  f_filter:
      #v_P: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_P (_super_i0).f_Item |} ->
      x0: v_Self ->
      x1: v_P
    -> Prims.Pure (t_Filter v_Self v_P)
        (f_filter_pre #v_P #i1 x0 x1)
        (fun result -> f_filter_post #v_P #i1 x0 x1 result);
  f_filter_map_pre:
      #v_B: Type0 ->
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      v_Self ->
      v_F
    -> Type0;
  f_filter_map_post:
      #v_B: Type0 ->
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      v_Self ->
      v_F ->
      t_FilterMap v_Self v_F
    -> Type0;
  f_filter_map:
      #v_B: Type0 ->
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      x0: v_Self ->
      x1: v_F
    -> Prims.Pure (t_FilterMap v_Self v_F)
        (f_filter_map_pre #v_B #v_F #i1 x0 x1)
        (fun result -> f_filter_map_post #v_B #v_F #i1 x0 x1 result);
  f_take_while_pre:
      #v_P: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_P (_super_i0).f_Item |} ->
      v_Self ->
      v_P
    -> Type0;
  f_take_while_post:
      #v_P: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_P (_super_i0).f_Item |} ->
      v_Self ->
      v_P ->
      t_TakeWhile v_Self v_P
    -> Type0;
  f_take_while:
      #v_P: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_P (_super_i0).f_Item |} ->
      x0: v_Self ->
      x1: v_P
    -> Prims.Pure (t_TakeWhile v_Self v_P)
        (f_take_while_pre #v_P #i1 x0 x1)
        (fun result -> f_take_while_post #v_P #i1 x0 x1 result);
  f_skip_while_pre:
      #v_P: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_P (_super_i0).f_Item |} ->
      v_Self ->
      v_P
    -> Type0;
  f_skip_while_post:
      #v_P: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_P (_super_i0).f_Item |} ->
      v_Self ->
      v_P ->
      t_SkipWhile v_Self v_P
    -> Type0;
  f_skip_while:
      #v_P: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_P (_super_i0).f_Item |} ->
      x0: v_Self ->
      x1: v_P
    -> Prims.Pure (t_SkipWhile v_Self v_P)
        (f_skip_while_pre #v_P #i1 x0 x1)
        (fun result -> f_skip_while_post #v_P #i1 x0 x1 result);
  f_map_while_pre:
      #v_B: Type0 ->
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      v_Self ->
      v_F
    -> Type0;
  f_map_while_post:
      #v_B: Type0 ->
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      v_Self ->
      v_F ->
      t_MapWhile v_Self v_F
    -> Type0;
  f_map_while:
      #v_B: Type0 ->
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      x0: v_Self ->
      x1: v_F
    -> Prims.Pure (t_MapWhile v_Self v_F)
        (f_map_while_pre #v_B #v_F #i1 x0 x1)
        (fun result -> f_map_while_post #v_B #v_F #i1 x0 x1 result);
  f_inspect_pre:
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      v_Self ->
      v_F
    -> Type0;
  f_inspect_post:
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      v_Self ->
      v_F ->
      t_Inspect v_Self v_F
    -> Type0;
  f_inspect:
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      x0: v_Self ->
      x1: v_F
    -> Prims.Pure (t_Inspect v_Self v_F)
        (f_inspect_pre #v_F #i1 x0 x1)
        (fun result -> f_inspect_post #v_F #i1 x0 x1 result);
  f_fuse_pre:v_Self -> Type0;
  f_fuse_post:v_Self -> t_Fuse v_Self -> Type0;
  f_fuse:x0: v_Self
    -> Prims.Pure (t_Fuse v_Self) (f_fuse_pre x0) (fun result -> f_fuse_post x0 result);
  f_flat_map_pre:
      #v_U: Type0 ->
      #v_F: Type0 ->
      {| i1: t_Iterator v_U |} ->
      {| i2: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      v_Self ->
      v_F
    -> Type0;
  f_flat_map_post:
      #v_U: Type0 ->
      #v_F: Type0 ->
      {| i1: t_Iterator v_U |} ->
      {| i2: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      v_Self ->
      v_F ->
      t_FlatMap v_Self v_U v_F
    -> Type0;
  f_flat_map:
      #v_U: Type0 ->
      #v_F: Type0 ->
      {| i1: t_Iterator v_U |} ->
      {| i2: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      x0: v_Self ->
      x1: v_F
    -> Prims.Pure (t_FlatMap v_Self v_U v_F)
        (f_flat_map_pre #v_U #v_F #i1 #i2 x0 x1)
        (fun result -> f_flat_map_post #v_U #v_F #i1 #i2 x0 x1 result);
  f_flatten_pre:{| i1: t_Iterator (_super_i0).f_Item |} -> v_Self -> Type0;
  f_flatten_post:{| i1: t_Iterator (_super_i0).f_Item |} -> v_Self -> t_Flatten v_Self -> Type0;
  f_flatten:{| i1: t_Iterator (_super_i0).f_Item |} -> x0: v_Self
    -> Prims.Pure (t_Flatten v_Self)
        (f_flatten_pre #i1 x0)
        (fun result -> f_flatten_post #i1 x0 result);
  f_zip_pre:#v_I2: Type0 -> {| i1: t_Iterator v_I2 |} -> v_Self -> v_I2 -> Type0;
  f_zip_post:#v_I2: Type0 -> {| i1: t_Iterator v_I2 |} -> v_Self -> v_I2 -> t_Zip v_Self v_I2
    -> Type0;
  f_zip:#v_I2: Type0 -> {| i1: t_Iterator v_I2 |} -> x0: v_Self -> x1: v_I2
    -> Prims.Pure (t_Zip v_Self v_I2)
        (f_zip_pre #v_I2 #i1 x0 x1)
        (fun result -> f_zip_post #v_I2 #i1 x0 x1 result);
  f_chain_pre:
      #v_U: Type0 ->
      {| i1: t_Iterator v_U |} ->
      #_: unit{i1.f_Item == (_super_i0).f_Item} ->
      v_Self ->
      v_U
    -> Type0;
  f_chain_post:
      #v_U: Type0 ->
      {| i1: t_Iterator v_U |} ->
      #_: unit{i1.f_Item == (_super_i0).f_Item} ->
      v_Self ->
      v_U ->
      t_Chain v_Self v_U
    -> Type0;
  f_chain:
      #v_U: Type0 ->
      {| i1: t_Iterator v_U |} ->
      #_: unit{i1.f_Item == (_super_i0).f_Item} ->
      x0: v_Self ->
      x1: v_U
    -> Prims.Pure (t_Chain v_Self v_U)
        (f_chain_pre #v_U #i1 #_ x0 x1)
        (fun result -> f_chain_post #v_U #i1 #_ x0 x1 result);
  f_any_pre:
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      v_Self ->
      v_F
    -> Type0;
  f_any_post:
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      v_Self ->
      v_F ->
      (v_Self & bool)
    -> Type0;
  f_any:
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      x0: v_Self ->
      x1: v_F
    -> Prims.Pure (v_Self & bool)
        (f_any_pre #v_F #i1 x0 x1)
        (fun result -> f_any_post #v_F #i1 x0 x1 result);
  f_find_pre:
      #v_P: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_P (_super_i0).f_Item |} ->
      v_Self ->
      v_P
    -> Type0;
  f_find_post:
      #v_P: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_P (_super_i0).f_Item |} ->
      v_Self ->
      v_P ->
      (v_Self & t_Option (_super_i0).f_Item)
    -> Type0;
  f_find:
      #v_P: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_P (_super_i0).f_Item |} ->
      x0: v_Self ->
      x1: v_P
    -> Prims.Pure (v_Self & t_Option (_super_i0).f_Item)
        (f_find_pre #v_P #i1 x0 x1)
        (fun result -> f_find_post #v_P #i1 x0 x1 result);
  f_find_map_pre:
      #v_B: Type0 ->
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      v_Self ->
      v_F
    -> Type0;
  f_find_map_post:
      #v_B: Type0 ->
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      v_Self ->
      v_F ->
      (v_Self & t_Option v_B)
    -> Type0;
  f_find_map:
      #v_B: Type0 ->
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      x0: v_Self ->
      x1: v_F
    -> Prims.Pure (v_Self & t_Option v_B)
        (f_find_map_pre #v_B #v_F #i1 x0 x1)
        (fun result -> f_find_map_post #v_B #v_F #i1 x0 x1 result);
  f_position_pre:
      #v_P: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_P (_super_i0).f_Item |} ->
      v_Self ->
      v_P
    -> Type0;
  f_position_post:
      #v_P: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_P (_super_i0).f_Item |} ->
      v_Self ->
      v_P ->
      (v_Self & t_Option usize)
    -> Type0;
  f_position:
      #v_P: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_P (_super_i0).f_Item |} ->
      x0: v_Self ->
      x1: v_P
    -> Prims.Pure (v_Self & t_Option usize)
        (f_position_pre #v_P #i1 x0 x1)
        (fun result -> f_position_post #v_P #i1 x0 x1 result);
  f_count_pre:v_Self -> Type0;
  f_count_post:v_Self -> usize -> Type0;
  f_count:x0: v_Self -> Prims.Pure usize (f_count_pre x0) (fun result -> f_count_post x0 result);
  f_nth_pre:v_Self -> usize -> Type0;
  f_nth_post:v_Self -> usize -> t_Option (_super_i0).f_Item -> Type0;
  f_nth:x0: v_Self -> x1: usize
    -> Prims.Pure (t_Option (_super_i0).f_Item)
        (f_nth_pre x0 x1)
        (fun result -> f_nth_post x0 x1 result);
  f_last_pre:v_Self -> Type0;
  f_last_post:v_Self -> t_Option (_super_i0).f_Item -> Type0;
  f_last:x0: v_Self
    -> Prims.Pure (t_Option (_super_i0).f_Item)
        (f_last_pre x0)
        (fun result -> f_last_post x0 result);
  f_for_each_pre:
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      v_Self ->
      v_F
    -> Type0;
  f_for_each_post:
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      v_Self ->
      v_F ->
      Prims.unit
    -> Type0;
  f_for_each:
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F (_super_i0).f_Item |} ->
      x0: v_Self ->
      x1: v_F
    -> Prims.Pure Prims.unit
        (f_for_each_pre #v_F #i1 x0 x1)
        (fun result -> f_for_each_post #v_F #i1 x0 x1 result);
  f_reduce_pre:
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F ((_super_i0).f_Item & (_super_i0).f_Item) |} ->
      v_Self ->
      v_F
    -> Type0;
  f_reduce_post:
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F ((_super_i0).f_Item & (_super_i0).f_Item) |} ->
      v_Self ->
      v_F ->
      t_Option (_super_i0).f_Item
    -> Type0;
  f_reduce:
      #v_F: Type0 ->
      {| i1: Core_models.Ops.Function.t_FnMut v_F ((_super_i0).f_Item & (_super_i0).f_Item) |} ->
      x0: v_Self ->
      x1: v_F
    -> Prims.Pure (t_Option (_super_i0).f_Item)
        (f_reduce_pre #v_F #i1 x0 x1)
        (fun result -> f_reduce_post #v_F #i1 x0 x1 result);
  f_min_pre:{| i1: t_Ord (_super_i0).f_Item |} -> v_Self -> Type0;
  f_min_post:{| i1: t_Ord (_super_i0).f_Item |} -> v_Self -> t_Option (_super_i0).f_Item -> Type0;
  f_min:{| i1: t_Ord (_super_i0).f_Item |} -> x0: v_Self
    -> Prims.Pure (t_Option (_super_i0).f_Item)
        (f_min_pre #i1 x0)
        (fun result -> f_min_post #i1 x0 result);
  f_max_pre:{| i1: t_Ord (_super_i0).f_Item |} -> v_Self -> Type0;
  f_max_post:{| i1: t_Ord (_super_i0).f_Item |} -> v_Self -> t_Option (_super_i0).f_Item -> Type0;
  f_max:{| i1: t_Ord (_super_i0).f_Item |} -> x0: v_Self
    -> Prims.Pure (t_Option (_super_i0).f_Item)
        (f_max_pre #i1 x0)
        (fun result -> f_max_post #i1 x0 result);
  f_collect_pre:#v_B: Type0 -> {| i1: t_FromIterator v_B (_super_i0).f_Item |} -> v_Self -> Type0;
  f_collect_post:#v_B: Type0 -> {| i1: t_FromIterator v_B (_super_i0).f_Item |} -> v_Self -> v_B
    -> Type0;
  f_collect:#v_B: Type0 -> {| i1: t_FromIterator v_B (_super_i0).f_Item |} -> x0: v_Self
    -> Prims.Pure v_B (f_collect_pre #v_B #i1 x0) (fun result -> f_collect_post #v_B #i1 x0 result)
}

[@@ FStar.Tactics.Typeclasses.tcinstance]
let _ = fun (v_Self:Type0) {|i: t_IteratorMethods v_Self|} -> i._super_i0

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl__from__iterator
      (#v_I: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_Iterator v_I)
    : t_IteratorMethods v_I =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    f_fold_pre
    =
    (fun
        (#v_B: Type0)
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F (v_B & i0.f_Item))
        (self: v_I)
        (init: v_B)
        (f: v_F)
        ->
        true);
    f_fold_post
    =
    (fun
        (#v_B: Type0)
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F (v_B & i0.f_Item))
        (self: v_I)
        (init: v_B)
        (f: v_F)
        (out: v_B)
        ->
        true);
    f_fold
    =
    (fun
        (#v_B: Type0)
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F (v_B & i0.f_Item))
        (self: v_I)
        (init: v_B)
        (f: v_F)
        ->
        iter_fold #v_I #v_B #v_F self init f);
    f_all_pre
    =
    (fun
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        ->
        true);
    f_all_post
    =
    (fun
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        (out1: (v_I & bool))
        ->
        true);
    f_all
    =
    (fun
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        ->
        let (tmp0: v_I), (out: bool) = iter_all #v_I #v_F self f in
        let self:v_I = tmp0 in
        let hax_temp_output:bool = out in
        self, hax_temp_output <: (v_I & bool));
    f_map_pre
    =
    (fun
        (#v_O: Type0)
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        ->
        true);
    f_map_post
    =
    (fun
        (#v_O: Type0)
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        (out: t_Map v_I v_F)
        ->
        true);
    f_map
    =
    (fun
        (#v_O: Type0)
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        ->
        impl__new__from__map #v_I #v_F self f);
    f_enumerate_pre = (fun (self: v_I) -> true);
    f_enumerate_post = (fun (self: v_I) (out: t_Enumerate v_I) -> true);
    f_enumerate = (fun (self: v_I) -> impl__new #v_I self);
    f_step_by_pre = (fun (self_: v_I) (step: usize) -> step >. mk_usize 0);
    f_step_by_post = (fun (self: v_I) (step: usize) (out: t_StepBy v_I) -> true);
    f_step_by = (fun (self: v_I) (step: usize) -> impl__new__from__step_by #v_I self step);
    f_take_pre = (fun (self: v_I) (n: usize) -> true);
    f_take_post = (fun (self: v_I) (n: usize) (out: t_Take v_I) -> true);
    f_take = (fun (self: v_I) (n: usize) -> impl__new__from__take #v_I self n);
    f_skip_pre = (fun (self: v_I) (n: usize) -> true);
    f_skip_post = (fun (self: v_I) (n: usize) (out: t_Skip v_I) -> true);
    f_skip = (fun (self: v_I) (n: usize) -> impl__new__from__skip #v_I self n);
    f_filter_pre
    =
    (fun
        (#v_P: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_P i0.f_Item)
        (self: v_I)
        (predicate: v_P)
        ->
        true);
    f_filter_post
    =
    (fun
        (#v_P: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_P i0.f_Item)
        (self: v_I)
        (predicate: v_P)
        (out: t_Filter v_I v_P)
        ->
        true);
    f_filter
    =
    (fun
        (#v_P: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_P i0.f_Item)
        (self: v_I)
        (predicate: v_P)
        ->
        impl__new__from__filter #v_I #v_P self predicate);
    f_filter_map_pre
    =
    (fun
        (#v_B: Type0)
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        ->
        true);
    f_filter_map_post
    =
    (fun
        (#v_B: Type0)
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        (out: t_FilterMap v_I v_F)
        ->
        true);
    f_filter_map
    =
    (fun
        (#v_B: Type0)
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        ->
        impl__new__from__filter_map #v_I #v_F self f);
    f_take_while_pre
    =
    (fun
        (#v_P: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_P i0.f_Item)
        (self: v_I)
        (predicate: v_P)
        ->
        true);
    f_take_while_post
    =
    (fun
        (#v_P: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_P i0.f_Item)
        (self: v_I)
        (predicate: v_P)
        (out: t_TakeWhile v_I v_P)
        ->
        true);
    f_take_while
    =
    (fun
        (#v_P: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_P i0.f_Item)
        (self: v_I)
        (predicate: v_P)
        ->
        impl__new__from__take_while #v_I #v_P self predicate);
    f_skip_while_pre
    =
    (fun
        (#v_P: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_P i0.f_Item)
        (self: v_I)
        (predicate: v_P)
        ->
        true);
    f_skip_while_post
    =
    (fun
        (#v_P: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_P i0.f_Item)
        (self: v_I)
        (predicate: v_P)
        (out: t_SkipWhile v_I v_P)
        ->
        true);
    f_skip_while
    =
    (fun
        (#v_P: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_P i0.f_Item)
        (self: v_I)
        (predicate: v_P)
        ->
        impl__new__from__skip_while #v_I #v_P self predicate);
    f_map_while_pre
    =
    (fun
        (#v_B: Type0)
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        ->
        true);
    f_map_while_post
    =
    (fun
        (#v_B: Type0)
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        (out: t_MapWhile v_I v_F)
        ->
        true);
    f_map_while
    =
    (fun
        (#v_B: Type0)
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        ->
        impl__new__from__map_while #v_I #v_F self f);
    f_inspect_pre
    =
    (fun
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        ->
        true);
    f_inspect_post
    =
    (fun
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        (out: t_Inspect v_I v_F)
        ->
        true);
    f_inspect
    =
    (fun
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        ->
        impl__new__from__inspect #v_I #v_F self f);
    f_fuse_pre = (fun (self: v_I) -> true);
    f_fuse_post = (fun (self: v_I) (out: t_Fuse v_I) -> true);
    f_fuse = (fun (self: v_I) -> impl__new__from__fuse #v_I self);
    f_flat_map_pre
    =
    (fun
        (#v_U: Type0)
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Iterator v_U)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i2:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        ->
        true);
    f_flat_map_post
    =
    (fun
        (#v_U: Type0)
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Iterator v_U)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i2:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        (out: t_FlatMap v_I v_U v_F)
        ->
        true);
    f_flat_map
    =
    (fun
        (#v_U: Type0)
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Iterator v_U)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i2:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        ->
        impl__new__from__flat_map #v_I #v_U #v_F self f);
    f_flatten_pre
    =
    (fun (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Iterator i0.f_Item) (self: v_I) -> true);
    f_flatten_post
    =
    (fun
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Iterator i0.f_Item)
        (self: v_I)
        (out: t_Flatten v_I)
        ->
        true);
    f_flatten
    =
    (fun (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Iterator i0.f_Item) (self: v_I) ->
        impl__new__from__flatten #v_I self);
    f_zip_pre
    =
    (fun
        (#v_I2: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Iterator v_I2)
        (self: v_I)
        (it2: v_I2)
        ->
        true);
    f_zip_post
    =
    (fun
        (#v_I2: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Iterator v_I2)
        (self: v_I)
        (it2: v_I2)
        (out: t_Zip v_I v_I2)
        ->
        true);
    f_zip
    =
    (fun
        (#v_I2: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Iterator v_I2)
        (self: v_I)
        (it2: v_I2)
        ->
        impl__new__from__zip #v_I #v_I2 self it2);
    f_chain_pre
    =
    (fun
        (#v_U: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Iterator v_U)
        (self: v_I)
        (other: v_U)
        ->
        true);
    f_chain_post
    =
    (fun
        (#v_U: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Iterator v_U)
        (self: v_I)
        (other: v_U)
        (out: t_Chain v_I v_U)
        ->
        true);
    f_chain
    =
    (fun
        (#v_U: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Iterator v_U)
        (self: v_I)
        (other: v_U)
        ->
        impl__new__from__chain #v_I #v_U self other);
    f_any_pre
    =
    (fun
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        ->
        true);
    f_any_post
    =
    (fun
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        (out1: (v_I & bool))
        ->
        true);
    f_any
    =
    (fun
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        ->
        let (tmp0: v_I), (out: bool) = iter_any #v_I #v_F self f in
        let self:v_I = tmp0 in
        let hax_temp_output:bool = out in
        self, hax_temp_output <: (v_I & bool));
    f_find_pre
    =
    (fun
        (#v_P: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_P i0.f_Item)
        (self: v_I)
        (predicate: v_P)
        ->
        true);
    f_find_post
    =
    (fun
        (#v_P: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_P i0.f_Item)
        (self: v_I)
        (predicate: v_P)
        (out1: (v_I & t_Option i0.f_Item))
        ->
        true);
    f_find
    =
    (fun
        (#v_P: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_P i0.f_Item)
        (self: v_I)
        (predicate: v_P)
        ->
        let (tmp0: v_I), (out: t_Option i0.f_Item) = iter_find #v_I #v_P self predicate in
        let self:v_I = tmp0 in
        let hax_temp_output:t_Option i0.f_Item = out in
        self, hax_temp_output <: (v_I & t_Option i0.f_Item));
    f_find_map_pre
    =
    (fun
        (#v_B: Type0)
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        ->
        true);
    f_find_map_post
    =
    (fun
        (#v_B: Type0)
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        (out1: (v_I & t_Option v_B))
        ->
        true);
    f_find_map
    =
    (fun
        (#v_B: Type0)
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        ->
        let (tmp0: v_I), (out: t_Option v_B) = iter_find_map #v_I #v_B #v_F self f in
        let self:v_I = tmp0 in
        let hax_temp_output:t_Option v_B = out in
        self, hax_temp_output <: (v_I & t_Option v_B));
    f_position_pre
    =
    (fun
        (#v_P: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_P i0.f_Item)
        (self: v_I)
        (predicate: v_P)
        ->
        true);
    f_position_post
    =
    (fun
        (#v_P: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_P i0.f_Item)
        (self: v_I)
        (predicate: v_P)
        (out1: (v_I & t_Option usize))
        ->
        true);
    f_position
    =
    (fun
        (#v_P: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_P i0.f_Item)
        (self: v_I)
        (predicate: v_P)
        ->
        let (tmp0: v_I), (out: t_Option usize) = iter_position #v_I #v_P self predicate in
        let self:v_I = tmp0 in
        let hax_temp_output:t_Option usize = out in
        self, hax_temp_output <: (v_I & t_Option usize));
    f_count_pre = (fun (self: v_I) -> true);
    f_count_post = (fun (self: v_I) (out: usize) -> true);
    f_count = (fun (self: v_I) -> iter_count #v_I self);
    f_nth_pre = (fun (self: v_I) (n: usize) -> true);
    f_nth_post = (fun (self: v_I) (n: usize) (out: t_Option i0.f_Item) -> true);
    f_nth = (fun (self: v_I) (n: usize) -> iter_nth #v_I self n);
    f_last_pre = (fun (self: v_I) -> true);
    f_last_post = (fun (self: v_I) (out: t_Option i0.f_Item) -> true);
    f_last = (fun (self: v_I) -> iter_last #v_I self);
    f_for_each_pre
    =
    (fun
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        ->
        true);
    f_for_each_post
    =
    (fun
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        (out: Prims.unit)
        ->
        true);
    f_for_each
    =
    (fun
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F i0.f_Item)
        (self: v_I)
        (f: v_F)
        ->
        iter_for_each #v_I #v_F self f);
    f_reduce_pre
    =
    (fun
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F (i0.f_Item & i0.f_Item))
        (self: v_I)
        (f: v_F)
        ->
        true);
    f_reduce_post
    =
    (fun
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F (i0.f_Item & i0.f_Item))
        (self: v_I)
        (f: v_F)
        (out: t_Option i0.f_Item)
        ->
        true);
    f_reduce
    =
    (fun
        (#v_F: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Core_models.Ops.Function.t_FnMut v_F (i0.f_Item & i0.f_Item))
        (self: v_I)
        (f: v_F)
        ->
        iter_reduce #v_I #v_F self f);
    f_min_pre
    =
    (fun (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Ord i0.f_Item) (self: v_I) -> true);
    f_min_post
    =
    (fun
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Ord i0.f_Item)
        (self: v_I)
        (out: t_Option i0.f_Item)
        ->
        true);
    f_min
    =
    (fun (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Ord i0.f_Item) (self: v_I) ->
        iter_min #v_I self);
    f_max_pre
    =
    (fun (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Ord i0.f_Item) (self: v_I) -> true);
    f_max_post
    =
    (fun
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Ord i0.f_Item)
        (self: v_I)
        (out: t_Option i0.f_Item)
        ->
        true);
    f_max
    =
    (fun (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Ord i0.f_Item) (self: v_I) ->
        iter_max #v_I self);
    f_collect_pre
    =
    (fun
        (#v_B: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_FromIterator v_B i0.f_Item)
        (self: v_I)
        ->
        true);
    f_collect_post
    =
    (fun
        (#v_B: Type0)
        (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_FromIterator v_B i0.f_Item)
        (self: v_I)
        (out: v_B)
        ->
        true);
    f_collect
    =
    fun
      (#v_B: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_FromIterator v_B i0.f_Item)
      (self: v_I)
      ->
      f_from_iter #v_B #i0.f_Item #FStar.Tactics.Typeclasses.solve #v_I self
  }
