
-- Legacy lean backend for Hax
-- The Hax prelude library can be found in hax/proof-libs/legacy-lean
import Hax
import Std.Tactic.Do
import Std.Do.Triple
import Std.Tactic.Do.Syntax
open Std.Do
open Std.Tactic

set_option mvcgen.warning false
set_option linter.unusedVariables false


namespace new_tests.legacy__proverif_integers__lib

@[spec]
def small (_ : rust_primitives.hax.Tuple0) : RustM u32 := do (pure (3 : u32))

@[spec]
def big (_ : rust_primitives.hax.Tuple0) : RustM u64 := do
  (pure (18446744073709551615 : u64))

@[spec]
def negative (_ : rust_primitives.hax.Tuple0) : RustM i32 := do
  (pure (-5 : i32))

@[spec]
def increment (n : u32) : RustM u32 := do (n +? (1 : u32))

@[spec]
def decrement (n : u32) : RustM u32 := do (n -? (1 : u32))

@[spec]
def opaque_sum (a : u32) (b : u32) : RustM u32 := do (a +? b)

@[spec]
def head (xs : (RustSlice u8)) : RustM u8 := do xs[(0 : usize)]_?

end new_tests.legacy__proverif_integers__lib

