
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


namespace new_tests.legacy__lean_ident_sanitize__lib

@[spec]
def _structure (_ : rust_primitives.hax.Tuple0) : RustM u8 := do (pure (0 : u8))

@[spec]
def _theorem (_ : rust_primitives.hax.Tuple0) : RustM u8 := do (pure (1 : u8))

@[spec]
def _deriving (_ : rust_primitives.hax.Tuple0) : RustM u8 := do (pure (2 : u8))

@[spec]
def _def (_ : rust_primitives.hax.Tuple0) : RustM u8 := do (pure (3 : u8))

end new_tests.legacy__lean_ident_sanitize__lib

