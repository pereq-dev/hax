
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


namespace new_tests.legacy__proverif_projection__lib

structure Tag where
  _0 : u32

def Impl.DEFAULT : Tag := RustM.of_isOk (do (pure (Tag.mk (0 : u32)))) (by rfl)

--  The value of `DEFAULT` survives, so `.0` reduces to `nat_0`.
@[spec]
def const_projection (_ : rust_primitives.hax.Tuple0) : RustM u32 := do
  (pure (Tag._0 Impl.DEFAULT))

opaque hidden (_ : rust_primitives.hax.Tuple0) : RustM Tag

--  `hidden()` has no body in the model, so `.0` can never reduce.
@[spec]
def dead_projection (_ : rust_primitives.hax.Tuple0) : RustM u32 := do
  (pure (Tag._0 (← (hidden rust_primitives.hax.Tuple0.mk))))

@[spec]
def live_projection (t : Tag) : RustM u32 := do (pure (Tag._0 t))

end new_tests.legacy__proverif_projection__lib

