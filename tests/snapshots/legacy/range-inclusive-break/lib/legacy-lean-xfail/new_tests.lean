
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


namespace new_tests.legacy__range_inclusive_break__lib

--  @fail(extraction): coq(HAX0001), ssprove(HAX0001), proverif(HAX0008)
@[spec]
def with_break (n : u8) : RustM u64 := do
  let count : u64 := (0 : u64);
  (rust_primitives.hax.folds.fold_range_inclusive_cf
    (0 : u8)
    n
    (fun count _ => (do (pure true) : RustM Bool))
    count
    (fun count i =>
      (do
      if (← (i ==? (3 : u8))) then do
        (pure (core_models.ops.control_flow.ControlFlow.Break
          (rust_primitives.hax.Tuple2.mk rust_primitives.hax.Tuple0.mk count)))
      else do
        (pure (core_models.ops.control_flow.ControlFlow.Continue
          (← (count +? (1 : u64))))) :
      RustM
      (core_models.ops.control_flow.ControlFlow
        (rust_primitives.hax.Tuple2 rust_primitives.hax.Tuple0 u64)
        u64))))

end new_tests.legacy__range_inclusive_break__lib

