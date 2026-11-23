module New_tests.Legacy__char_literals__lib
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let v_ASCII: t_Array FStar.Char.char (mk_usize 6) =
  let list = ['a'; '\0'; '\''; '"'; '\n'; ''] in
  FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 6);
  Rust_primitives.Hax.array_of_list 6 list

let v_TWO_BYTES: t_Array FStar.Char.char (mk_usize 4) =
  let list = [''; 'é'; 'Ã'; '߿'] in
  FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 4);
  Rust_primitives.Hax.array_of_list 4 list

let v_THREE_BYTES: t_Array FStar.Char.char (mk_usize 4) =
  let list = ['ࠀ'; '中'; ''; '￿'] in
  FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 4);
  Rust_primitives.Hax.array_of_list 4 list

let v_FOUR_BYTES: t_Array FStar.Char.char (mk_usize 3) =
  let list = ['𐀀'; '🦀'; '􏿿'] in
  FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 3);
  Rust_primitives.Hax.array_of_list 3 list

/// https://github.com/pereq-dev/hax/issues/2396
let distinct_chars (_: Prims.unit)
    : Prims.Pure bool
      Prims.l_True
      (ensures
        fun r ->
          let r:bool = r in
          r =. false) = 'é' =. 'Ã'

let classify (c: FStar.Char.char) : u8 =
  match c <: FStar.Char.char with
  | 'a' -> mk_u8 0
  | 'é' -> mk_u8 1
  | '中' -> mk_u8 2
  | '🦀' -> mk_u8 3
  | _ -> mk_u8 4
