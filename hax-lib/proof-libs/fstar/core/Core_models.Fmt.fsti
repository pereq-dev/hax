module Core_models.Fmt
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Rust_primitives

/// See [`std::fmt::Error`]
type t_Error = | Error : t_Error

/// See [`std::fmt::Formatter`]
/// DEVIATION(std): the model's formatter has no output, so `Debug` and
/// `Display` only model whether formatting succeeds.
type t_Formatter = | Formatter : t_Formatter

/// See [`std::fmt::Display`]
class t_Display (v_Self: Type0) = {
  f_fmt_pre:v_Self -> t_Formatter -> Type0;
  f_fmt_post:v_Self -> t_Formatter -> (t_Formatter & Core_models.Result.t_Result Prims.unit t_Error)
    -> Type0;
  f_fmt:x0: v_Self -> x1: t_Formatter
    -> Prims.Pure (t_Formatter & Core_models.Result.t_Result Prims.unit t_Error)
        (f_fmt_pre x0 x1)
        (fun result -> f_fmt_post x0 x1 result)
}

/// See [`std::fmt::Debug`]
class t_Debug (v_Self: Type0) = {
  f_dbg_fmt_pre:v_Self -> t_Formatter -> Type0;
  f_dbg_fmt_post:
      v_Self ->
      t_Formatter ->
      (t_Formatter & Core_models.Result.t_Result Prims.unit t_Error)
    -> Type0;
  f_dbg_fmt:x0: v_Self -> x1: t_Formatter
    -> Prims.Pure (t_Formatter & Core_models.Result.t_Result Prims.unit t_Error)
        (f_dbg_fmt_pre x0 x1)
        (fun result -> f_dbg_fmt_post x0 x1 result)
}

///See [`std::fmt::Formatter::debug_struct_field1_finish`]
val impl_Formatter__debug_struct_field1_finish
      (#v_T1: Type0)
      {| i0: t_Debug v_T1 |}
      (self: t_Formatter)
      (struct_name name1: string)
      (value1: v_T1)
    : Prims.Pure (t_Formatter & Core_models.Result.t_Result Prims.unit t_Error)
      Prims.l_True
      (fun _ -> Prims.l_True)

///See [`std::fmt::Formatter::debug_struct_field2_finish`]
val impl_Formatter__debug_struct_field2_finish
      (#v_T1 #v_T2: Type0)
      {| i0: t_Debug v_T1 |}
      {| i1: t_Debug v_T2 |}
      (self: t_Formatter)
      (struct_name name1: string)
      (value1: v_T1)
      (name2: string)
      (value2: v_T2)
    : Prims.Pure (t_Formatter & Core_models.Result.t_Result Prims.unit t_Error)
      Prims.l_True
      (fun _ -> Prims.l_True)

///See [`std::fmt::Formatter::debug_struct_field3_finish`]
val impl_Formatter__debug_struct_field3_finish
      (#v_T1 #v_T2 #v_T3: Type0)
      {| i0: t_Debug v_T1 |}
      {| i1: t_Debug v_T2 |}
      {| i2: t_Debug v_T3 |}
      (self: t_Formatter)
      (struct_name name1: string)
      (value1: v_T1)
      (name2: string)
      (value2: v_T2)
      (name3: string)
      (value3: v_T3)
    : Prims.Pure (t_Formatter & Core_models.Result.t_Result Prims.unit t_Error)
      Prims.l_True
      (fun _ -> Prims.l_True)

///See [`std::fmt::Formatter::debug_struct_field4_finish`]
val impl_Formatter__debug_struct_field4_finish
      (#v_T1 #v_T2 #v_T3 #v_T4: Type0)
      {| i0: t_Debug v_T1 |}
      {| i1: t_Debug v_T2 |}
      {| i2: t_Debug v_T3 |}
      {| i3: t_Debug v_T4 |}
      (self: t_Formatter)
      (struct_name name1: string)
      (value1: v_T1)
      (name2: string)
      (value2: v_T2)
      (name3: string)
      (value3: v_T3)
      (name4: string)
      (value4: v_T4)
    : Prims.Pure (t_Formatter & Core_models.Result.t_Result Prims.unit t_Error)
      Prims.l_True
      (fun _ -> Prims.l_True)

///See [`std::fmt::Formatter::debug_struct_field5_finish`]
val impl_Formatter__debug_struct_field5_finish
      (#v_T1 #v_T2 #v_T3 #v_T4 #v_T5: Type0)
      {| i0: t_Debug v_T1 |}
      {| i1: t_Debug v_T2 |}
      {| i2: t_Debug v_T3 |}
      {| i3: t_Debug v_T4 |}
      {| i4: t_Debug v_T5 |}
      (self: t_Formatter)
      (struct_name name1: string)
      (value1: v_T1)
      (name2: string)
      (value2: v_T2)
      (name3: string)
      (value3: v_T3)
      (name4: string)
      (value4: v_T4)
      (name5: string)
      (value5: v_T5)
    : Prims.Pure (t_Formatter & Core_models.Result.t_Result Prims.unit t_Error)
      Prims.l_True
      (fun _ -> Prims.l_True)

/// See [`std::fmt::Formatter::debug_struct_fields_finish`]
/// Real `core` asserts that the two slices have the same length; the model
/// keeps that panic, since it is the only observable behaviour left.
val impl_Formatter__debug_struct_fields_finish
      (#v_T: Type0)
      {| i0: t_Debug v_T |}
      (self: t_Formatter)
      (struct_name: string)
      (names: t_Slice string)
      (values: t_Slice v_T)
    : Prims.Pure (t_Formatter & Core_models.Result.t_Result Prims.unit t_Error)
      Prims.l_True
      (fun _ -> Prims.l_True)

/// See [`std::fmt::Formatter::debug_tuple_field1_finish`]
val impl_Formatter__debug_tuple_field1_finish
      (#v_T1: Type0)
      {| i0: t_Debug v_T1 |}
      (self: t_Formatter)
      (struct_name: string)
      (value1: v_T1)
    : Prims.Pure (t_Formatter & Core_models.Result.t_Result Prims.unit t_Error)
      Prims.l_True
      (fun _ -> Prims.l_True)

/// See [`std::fmt::Arguments`]
type t_Arguments = | Arguments : Prims.unit -> t_Arguments

[@@ FStar.Tactics.Typeclasses.tcinstance]
val impl_1 (#v_T: Type0) : t_Debug v_T

[@@ FStar.Tactics.Typeclasses.tcinstance]
val impl_Display_for_u8:t_Display u8

[@@ FStar.Tactics.Typeclasses.tcinstance]
val impl_Display_for_u16:t_Display u16

[@@ FStar.Tactics.Typeclasses.tcinstance]
val impl_Display_for_u32:t_Display u32

[@@ FStar.Tactics.Typeclasses.tcinstance]
val impl_Display_for_u64:t_Display u64

[@@ FStar.Tactics.Typeclasses.tcinstance]
val impl_Display_for_u128:t_Display u128

[@@ FStar.Tactics.Typeclasses.tcinstance]
val impl_Display_for_usize:t_Display usize

[@@ FStar.Tactics.Typeclasses.tcinstance]
val impl_Display_for_i8:t_Display i8

[@@ FStar.Tactics.Typeclasses.tcinstance]
val impl_Display_for_i16:t_Display i16

[@@ FStar.Tactics.Typeclasses.tcinstance]
val impl_Display_for_i32:t_Display i32

[@@ FStar.Tactics.Typeclasses.tcinstance]
val impl_Display_for_i64:t_Display i64

[@@ FStar.Tactics.Typeclasses.tcinstance]
val impl_Display_for_i128:t_Display i128

[@@ FStar.Tactics.Typeclasses.tcinstance]
val impl_Display_for_isize:t_Display isize

/// See [`std::fmt::Formatter::write_str`]
val impl_11__write_str (self: t_Formatter) (data: string)
    : Prims.Pure (t_Formatter & Core_models.Result.t_Result Prims.unit t_Error)
      Prims.l_True
      (fun _ -> Prims.l_True)

val impl_12__write_fmt (f: t_Formatter) (args: t_Arguments)
    : Prims.Pure (t_Formatter & Core_models.Result.t_Result Prims.unit t_Error)
      Prims.l_True
      (fun _ -> Prims.l_True)

/// Not a real `std::fmt::Arguments` method: the carve lowers panic/assert
/// messages to `Arguments::from_str(msg)` and discards the result, so this
/// exists to make those call sites resolve.
/// Opaque: real `core` builds an `Arguments` that carries the formatted
/// message, while the model\'s is a payload-free phantom, so the body is not
/// a model of what `core` does — only enough to have a value.
val impl_12__from_str (e_s: string) : Prims.Pure t_Arguments Prims.l_True (fun _ -> Prims.l_True)
