
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


namespace new_tests.rustc_coverage__unicode

@[spec]
def 申し訳ございません (_ : rust_primitives.hax.Tuple0) : RustM Bool := do
  (core_models.hint.black_box Bool false)

@[spec]
def サビ (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

--  @fail(extraction): ssprove(HAX0001)
@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ ←
    (core_models.iter.traits.iterator.Iterator.fold
      (← (core_models.iter.traits.collect.IntoIterator.into_iter
        (core_models.ops.range.RangeInclusive Char)
        (← (core_models.ops.range.Impl_7.new Char 'А' 'Я'))))
      rust_primitives.hax.Tuple0.mk
      (fun _ _İ =>
        (do
        (pure rust_primitives.hax.Tuple0.mk) :
        RustM rust_primitives.hax.Tuple0)));
  let _ ←
    if
    (← ((← (申し訳ございません rust_primitives.hax.Tuple0.mk))
      &&? (← (申し訳ございません rust_primitives.hax.Tuple0.mk)))) then do
      let _ ←
        (std.io.stdio._print
          (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
            (RustArray.ofVec #v["true\n"]))));
      let _ := rust_primitives.hax.Tuple0.mk;
      (pure rust_primitives.hax.Tuple0.mk)
    else do
      (pure rust_primitives.hax.Tuple0.mk);
  let _ ← (サビ rust_primitives.hax.Tuple0.mk);
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def 他 (_ : rust_primitives.hax.Tuple0) : RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.rustc_coverage__unicode

