
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


namespace new_tests.rustc_coverage__auxiliary__executor

--  Dummy "executor" that just repeatedly polls a future until it's ready.
--  @fail(extraction): fstar(HAX0003, HAX0003, HAX0003, HAX0003, HAX0003), ssprove(HAX0003, HAX0003, HAX0003, HAX0003, HAX0008), coq(HAX0008, HAX0003, HAX0003, HAX0003, HAX0003), legacy-lean(HAX0003, HAX0003, HAX0003, HAX0003, HAX0003)
--  @fail(extraction): proverif(HAX0003, HAX0003, HAX0003, HAX0003, HAX0008)
@[spec]
def block_on
    (F : Type)
    [trait_constr_block_on_associated_type_i0 :
      core_models.future.future.Future.AssociatedTypes
      F]
    [trait_constr_block_on_i0 : core_models.future.future.Future F ]
    (future : F) :
    RustM (core_models.future.future.Future.Output F) := do
  (pure sorry)

end new_tests.rustc_coverage__auxiliary__executor

