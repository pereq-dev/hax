module New_tests.Legacy__literals__lib
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let math_integers (x: Hax_lib.Int.t_Int)
    : Prims.Pure u8
      (requires x > (0 <: Hax_lib.Int.t_Int) && x < (16 <: Hax_lib.Int.t_Int))
      (fun _ -> Prims.l_True) =
  let _:Hax_lib.Int.t_Int = Rust_primitives.Hax.Int.from_machine (mk_usize 3) in
  let e_neg_dec:Hax_lib.Int.t_Int = (-340282366920938463463374607431768211455000) in
  let e_pos_dec:Hax_lib.Int.t_Int = 340282366920938463463374607431768211455000 in
  let e_neg_hex:Hax_lib.Int.t_Int = (-340282366920938463463374607431768211455000) in
  let e_pos_hex:Hax_lib.Int.t_Int = 340282366920938463463374607431768211455000 in
  let e_neg_octal:Hax_lib.Int.t_Int = (-340282366920938463463374607431768211455000) in
  let e_pos_octal:Hax_lib.Int.t_Int = 340282366920938463463374607431768211455000 in
  let e_neg_bin:Hax_lib.Int.t_Int = (-340282366920938463463374607431768211455000) in
  let e_pos_bin:Hax_lib.Int.t_Int = 340282366920938463463374607431768211455000 in
  let _:bool =
    ((-340282366920938463463374607431768211455000) <: Hax_lib.Int.t_Int) >
    (340282366920938463463374607431768211455000 <: Hax_lib.Int.t_Int)
  in
  let _:bool = x < x in
  let _:bool = x >= x in
  let _:bool = x <= x in
  let _:bool = x <> x in
  let _:bool = x = x in
  let _:Hax_lib.Int.t_Int = x + x in
  let _:Hax_lib.Int.t_Int = x - x in
  let _:Hax_lib.Int.t_Int = x * x in
  let _:Hax_lib.Int.t_Int = Rust_primitives.Hax.Int.div x x in
  let _:i16 = Hax_lib.Int.impl_Int__to_i16 x in
  let _:i32 = Hax_lib.Int.impl_Int__to_i32 x in
  let _:i64 = Hax_lib.Int.impl_Int__to_i64 x in
  let _:i128 = Hax_lib.Int.impl_Int__to_i128 x in
  let _:isize = Hax_lib.Int.impl_Int__to_isize x in
  let _:u16 = Hax_lib.Int.impl_Int__to_u16 x in
  let _:u32 = Hax_lib.Int.impl_Int__to_u32 x in
  let _:u64 = Hax_lib.Int.impl_Int__to_u64 x in
  let _:u128 = Hax_lib.Int.impl_Int__to_u128 x in
  let _:usize = Hax_lib.Int.impl_Int__to_usize x in
  Hax_lib.Int.impl_Int__to_u8 (x + (x * x <: Hax_lib.Int.t_Int) <: Hax_lib.Int.t_Int)

let panic_with_msg (_: Prims.unit) : Prims.unit =
  Rust_primitives.Hax.never_to_any (Core_models.Panicking.panic_fmt (Core_models.Fmt.Rt.impl_1__new_const
            (mk_usize 1)
            (let list = ["with msg"] in
              FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
              Rust_primitives.Hax.array_of_list 1 list)
          <:
          Core_models.Fmt.t_Arguments)
      <:
      Rust_primitives.Hax.t_Never)

type t_Foo = { f_field:u8 }

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl': Core_models.Marker.t_StructuralPartialEq t_Foo

unfold
let impl = impl'

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_1': Core_models.Cmp.t_PartialEq t_Foo t_Foo

unfold
let impl_1 = impl_1'

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_2': Core_models.Cmp.t_Eq t_Foo

unfold
let impl_2 = impl_2'

let v_CONSTANT: t_Foo = { f_field = mk_u8 3 } <: t_Foo

let numeric (_: Prims.unit) : Prims.unit =
  let _:usize = mk_usize 123 in
  let _:isize = mk_isize (-42) in
  let _:isize = mk_isize 42 in
  let _:i32 = mk_i32 (-42) in
  let _:u128 = mk_u128 22222222222222222222 in
  ()

/// @fail(extraction): ssprove(HAX0001)
let patterns (_: Prims.unit) : Prims.unit =
  let _:Prims.unit =
    match mk_u8 1 <: u8 with
    | Rust_primitives.Integers.MkInt 2 -> () <: Prims.unit
    | _ -> () <: Prims.unit
  in
  let _:Prims.unit =
    match
      "hello",
      (mk_i32 123,
        (let list = ["a"; "b"] in
          FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 2);
          Rust_primitives.Hax.array_of_list 2 list)
        <:
        (i32 & t_Array string (mk_usize 2)))
      <:
      (string & (i32 & t_Array string (mk_usize 2)))
    with
    | "hello", (Rust_primitives.Integers.MkInt 123, e_todo) -> () <: Prims.unit
    | _ -> () <: Prims.unit
  in
  let _:Prims.unit =
    match { f_field = mk_u8 4 } <: t_Foo with
    | { f_field = Rust_primitives.Integers.MkInt 3 } -> () <: Prims.unit
    | _ -> () <: Prims.unit
  in
  ()

let casts (x8: u8) (x16: u16) (x32: u32) (x64: u64) (xs: usize) : Prims.unit =
  let _:u64 =
    ((((cast (x8 <: u8) <: u64) +! (cast (x16 <: u16) <: u64) <: u64) +! (cast (x32 <: u32) <: u64)
        <:
        u64) +!
      x64
      <:
      u64) +!
    (cast (xs <: usize) <: u64)
  in
  let _:u32 =
    ((((cast (x8 <: u8) <: u32) +! (cast (x16 <: u16) <: u32) <: u32) +! x32 <: u32) +!
      (cast (x64 <: u64) <: u32)
      <:
      u32) +!
    (cast (xs <: usize) <: u32)
  in
  let _:u16 =
    ((((cast (x8 <: u8) <: u16) +! x16 <: u16) +! (cast (x32 <: u32) <: u16) <: u16) +!
      (cast (x64 <: u64) <: u16)
      <:
      u16) +!
    (cast (xs <: usize) <: u16)
  in
  let _:u8 =
    (((x8 +! (cast (x16 <: u16) <: u8) <: u8) +! (cast (x32 <: u32) <: u8) <: u8) +!
      (cast (x64 <: u64) <: u8)
      <:
      u8) +!
    (cast (xs <: usize) <: u8)
  in
  let _:i64 =
    ((((cast (x8 <: u8) <: i64) +! (cast (x16 <: u16) <: i64) <: i64) +! (cast (x32 <: u32) <: i64)
        <:
        i64) +!
      (cast (x64 <: u64) <: i64)
      <:
      i64) +!
    (cast (xs <: usize) <: i64)
  in
  let _:i32 =
    ((((cast (x8 <: u8) <: i32) +! (cast (x16 <: u16) <: i32) <: i32) +! (cast (x32 <: u32) <: i32)
        <:
        i32) +!
      (cast (x64 <: u64) <: i32)
      <:
      i32) +!
    (cast (xs <: usize) <: i32)
  in
  let _:i16 =
    ((((cast (x8 <: u8) <: i16) +! (cast (x16 <: u16) <: i16) <: i16) +! (cast (x32 <: u32) <: i16)
        <:
        i16) +!
      (cast (x64 <: u64) <: i16)
      <:
      i16) +!
    (cast (xs <: usize) <: i16)
  in
  let _:i8 =
    ((((cast (x8 <: u8) <: i8) +! (cast (x16 <: u16) <: i8) <: i8) +! (cast (x32 <: u32) <: i8)
        <:
        i8) +!
      (cast (x64 <: u64) <: i8)
      <:
      i8) +!
    (cast (xs <: usize) <: i8)
  in
  ()

let empty_array (_: Prims.unit) : Prims.unit =
  let _:t_Slice u8 =
    (let list:Prims.list u8 = [] in
      FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 0);
      Rust_primitives.Hax.array_of_list 0 list)
    <:
    t_Slice u8
  in
  ()

/// https://github.com/pereq-dev/hax/issues/500
let fn_pointer_cast (_: Prims.unit) : Prims.unit =
  let (f: (u32 -> u32)): u32 -> u32 = fun x -> x in
  ()

/// @fail(extraction): ssprove(HAX0001)
let null: FStar.Char.char = '\0'

let strings (_: Prims.unit) : Prims.unit =
  let _:string = "hello" in
  let _:string = "hello\"world" in
  let _:string = "it's" in
  let _:string = "back\slash" in
  let _:string = "line\nbreak" in
  let _:string = "carriage\rreturn" in
  let _:string = "tab\there" in
  let _:string = "null\0byte" in
  let _:string = "bellchar" in
  let _:string = "[0m" in
  let _:string = "🦀" in
  ()
