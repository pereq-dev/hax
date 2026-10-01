
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


namespace new_tests.rustc_coverage__nested_loops

--  @fail(extraction): ssprove(HAX0001, HAX0001, HAX0001), coq(HAX0001, HAX0001, HAX0001, HAX0001)
@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM (rust_primitives.hax.Tuple2 i32 rust_primitives.hax.Tuple0) := do
  let is_true : Bool ←
    ((← (core_models.iter.traits.exact_size.ExactSizeIterator.len
        std.env.Args (← (std.env.args rust_primitives.hax.Tuple0.mk))))
      ==? (1 : usize));
  let countdown : i32 := (10 : i32);
  (pure (rust_primitives.hax.Tuple2.mk
    (← (rust_primitives.hax.while_loop
      (fun countdown => (do (pure true) : RustM Bool))
      (fun countdown => (do (countdown >? (0 : i32)) : RustM Bool))
      (fun countdown =>
        (do
        (rust_primitives.hax.int.from_machine (0 : u32)) :
        RustM hax_lib.int.Int))
      countdown
      (fun countdown =>
        (do
        let a : i32 := (100 : i32);
        let b : i32 := (100 : i32);
        let ⟨a, b⟩ ←
          (rust_primitives.hax.folds.fold_range_cf
            (0 : i32)
            (50 : i32)
            (fun ⟨a, b⟩ _ => (do (pure true) : RustM Bool))
            (rust_primitives.hax.Tuple2.mk a b)
            (fun ⟨a, b⟩ _ =>
              (do
              if (← (a <? (30 : i32))) then do
                (pure (core_models.ops.control_flow.ControlFlow.Break
                  (rust_primitives.hax.Tuple2.mk
                    rust_primitives.hax.Tuple0.mk
                    (rust_primitives.hax.Tuple2.mk a b))))
              else do
                let a : i32 ← (a -? (5 : i32));
                let b : i32 ← (b -? (5 : i32));
                if (← (b <? (90 : i32))) then do
                  let a : i32 ← (a -? (10 : i32));
                  if is_true then do
                    (pure (core_models.ops.control_flow.ControlFlow.Break
                      (rust_primitives.hax.Tuple2.mk
                        rust_primitives.hax.Tuple0.mk
                        (rust_primitives.hax.Tuple2.mk a b))))
                  else do
                    let a : i32 ← (a -? (2 : i32));
                    (pure (core_models.ops.control_flow.ControlFlow.Continue
                      (rust_primitives.hax.Tuple2.mk a b)))
                else do
                  (pure (core_models.ops.control_flow.ControlFlow.Continue
                    (rust_primitives.hax.Tuple2.mk a b))) :
              RustM
              (core_models.ops.control_flow.ControlFlow
                (rust_primitives.hax.Tuple2
                  rust_primitives.hax.Tuple0
                  (rust_primitives.hax.Tuple2 i32 i32))
                (rust_primitives.hax.Tuple2 i32 i32)))));
        (countdown -? (1 : i32)) :
        RustM i32))))
    rust_primitives.hax.Tuple0.mk))

end new_tests.rustc_coverage__nested_loops

