
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


namespace new_tests.legacy__range_inclusive__lib

--  `end + 1` does not fit in a `u8` here.
--  @fail(extraction): proverif(HAX0008)
@[spec]
def full_u8_range (_ : rust_primitives.hax.Tuple0) : RustM u64 := do
  let count : u64 := (0 : u64);
  let count : u64 ←
    (rust_primitives.hax.folds.fold_range_inclusive
      (0 : u8)
      (255 : u8)
      (fun count _ => (do (pure true) : RustM Bool))
      count
      (fun count i => (do (count +? (1 : u64)) : RustM u64)));
  (pure count)

--  @fail(extraction): proverif(HAX0008)
def count_up_to (n : u8) : RustM u8 := do
  let count : u8 := (0 : u8);
  let count : u8 ←
    (rust_primitives.hax.folds.fold_range_inclusive
      (0 : u8)
      n
      (fun count i => (do (count ==? i) : RustM Bool))
      count
      (fun count i =>
        (do let count : u8 ← (count +? (1 : u8)); (pure count) : RustM u8)));
  (pure count)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def count_up_to.spec (n : u8) :
    Spec
      (requires := do (n <? (255 : u8)))
      (ensures := fun res => do (n <? res))
      (count_up_to (n : u8)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [count_up_to] <;> bv_decide
}

--  @fail(extraction): proverif(HAX0008)
@[spec]
def empty_range (_ : rust_primitives.hax.Tuple0) : RustM u64 := do
  let count : u64 := (0 : u64);
  let count : u64 ←
    (rust_primitives.hax.folds.fold_range_inclusive
      (5 : u8)
      (3 : u8)
      (fun count _ => (do (pure true) : RustM Bool))
      count
      (fun count i => (do (count +? (1 : u64)) : RustM u64)));
  (pure count)

--  @fail(extraction): proverif(HAX0008)
@[spec]
def single_element (_ : rust_primitives.hax.Tuple0) : RustM u64 := do
  let count : u64 := (0 : u64);
  let count : u64 ←
    (rust_primitives.hax.folds.fold_range_inclusive
      (3 : u8)
      (3 : u8)
      (fun count _ => (do (pure true) : RustM Bool))
      count
      (fun count i => (do (count +? (1 : u64)) : RustM u64)));
  (pure count)

--  @fail(extraction): proverif(HAX0008)
@[spec]
def with_return (n : u8) : RustM u8 := do
  match
    (← (rust_primitives.hax.folds.fold_range_inclusive_return
      (0 : u8)
      n
      (fun _ _ => (do (pure true) : RustM Bool))
      rust_primitives.hax.Tuple0.mk
      (fun _ i =>
        (do
        if (← (i ==? (7 : u8))) then do
          (pure (core_models.ops.control_flow.ControlFlow.Break
            (core_models.ops.control_flow.ControlFlow.Break i)))
        else do
          (pure (core_models.ops.control_flow.ControlFlow.Continue
            rust_primitives.hax.Tuple0.mk)) :
        RustM
        (core_models.ops.control_flow.ControlFlow
          (core_models.ops.control_flow.ControlFlow
            u8
            (rust_primitives.hax.Tuple2
              rust_primitives.hax.Tuple0
              rust_primitives.hax.Tuple0))
          rust_primitives.hax.Tuple0)))))
  with
    | (core_models.ops.control_flow.ControlFlow.Break  ret) => do (pure ret)
    | (core_models.ops.control_flow.ControlFlow.Continue  _) => do
      (pure (0 : u8))

end new_tests.legacy__range_inclusive__lib

