module New_tests.Legacy__lean_ident_sanitize__lib
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let structure (_: Prims.unit) : u8 = mk_u8 0

let theorem (_: Prims.unit) : u8 = mk_u8 1

let deriving (_: Prims.unit) : u8 = mk_u8 2

let def (_: Prims.unit) : u8 = mk_u8 3
