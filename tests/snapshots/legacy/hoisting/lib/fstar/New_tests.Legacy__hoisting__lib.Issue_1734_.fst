module New_tests.Legacy__hoisting__lib.Issue_1734_
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let hoisted_let (x: u8) : u8 =
  let x_1_:i32 = mk_i32 9 in
  mk_u8 0 +! x

let hoisted_let_rhs (x: u8) : u8 =
  let x_1_:u8 = Core_models.Num.impl_u8__wrapping_add x (mk_u8 1) in
  let y:u8 = Core_models.Num.impl_u8__wrapping_mul x_1_ (mk_u8 2) in
  Core_models.Num.impl_u8__wrapping_add y x
