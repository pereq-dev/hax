module New_tests.Legacy__char_literal_d7ff__lib
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

/// @fail(extraction): fstar(HAX0001)
let v_BEFORE_SURROGATES: FStar.Char.char = "failure"
