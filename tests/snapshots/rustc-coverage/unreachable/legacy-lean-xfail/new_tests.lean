
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


namespace new_tests.rustc_coverage__unreachable

--  @fail(extraction): ssprove(HAX0008), coq(HAX0008)
--  @fail(extraction): proverif(HAX0008)
def UNREACHABLE_CLOSURE :
  (rust_primitives.hax.Tuple0 -> RustM rust_primitives.hax.Tuple0)
  :=
  RustM.of_isOk
    (do
    (pure (fun _ =>
      (do
      (rust_primitives.hax.never_to_any
        (← (core_models.hint.unreachable_unchecked
          rust_primitives.hax.Tuple0.mk))) :
      RustM rust_primitives.hax.Tuple0))))
    (by rfl)

--  @fail(extraction): coq(HAX0008), ssprove(HAX0008)
--  @fail(extraction): proverif(HAX0008)
@[spec]
def unreachable_function (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  (rust_primitives.hax.never_to_any
    (← (core_models.hint.unreachable_unchecked rust_primitives.hax.Tuple0.mk)))

--  @fail(extraction): ssprove(HAX0008), coq(HAX0008)
--  @fail(extraction): proverif(HAX0008)
@[spec]
def unreachable_intrinsic (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  (rust_primitives.hax.never_to_any
    (← (core_models.intrinsics.unreachable rust_primitives.hax.Tuple0.mk)))

@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ ←
    if (← (core_models.hint.black_box Bool false)) then do
      let _ ← (UNREACHABLE_CLOSURE rust_primitives.hax.Tuple0.mk);
      (pure rust_primitives.hax.Tuple0.mk)
    else do
      (pure rust_primitives.hax.Tuple0.mk);
  let _ ←
    if (← (core_models.hint.black_box Bool false)) then do
      let _ ← (unreachable_function rust_primitives.hax.Tuple0.mk);
      (pure rust_primitives.hax.Tuple0.mk)
    else do
      (pure rust_primitives.hax.Tuple0.mk);
  if (← (core_models.hint.black_box Bool false)) then do
    let _ ← (unreachable_intrinsic rust_primitives.hax.Tuple0.mk);
    (pure rust_primitives.hax.Tuple0.mk)
  else do
    (pure rust_primitives.hax.Tuple0.mk)

end new_tests.rustc_coverage__unreachable

