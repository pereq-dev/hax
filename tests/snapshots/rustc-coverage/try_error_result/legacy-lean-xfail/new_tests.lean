
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


namespace new_tests.rustc_coverage__try_error_result

@[spec]
def call (return_error : Bool) :
    RustM
    (core_models.result.Result
      rust_primitives.hax.Tuple0
      rust_primitives.hax.Tuple0)
    := do
  if return_error then do
    (pure (core_models.result.Result.Err rust_primitives.hax.Tuple0.mk))
  else do
    (pure (core_models.result.Result.Ok rust_primitives.hax.Tuple0.mk))

@[spec]
def test1 (_ : rust_primitives.hax.Tuple0) :
    RustM
    (core_models.result.Result
      rust_primitives.hax.Tuple0
      rust_primitives.hax.Tuple0)
    := do
  let countdown : i32 := (10 : i32);
  match
    (← (rust_primitives.hax.folds.fold_range_return
      (0 : i32)
      (10 : i32)
      (fun countdown _ => (do (pure true) : RustM Bool))
      countdown
      (fun countdown _ =>
        (do
        let countdown : i32 ← (countdown -? (1 : i32));
        if (← (countdown <? (5 : i32))) then do
          match (← (call true)) with
            | (core_models.result.Result.Ok  _) => do
              match (← (call false)) with
                | (core_models.result.Result.Ok  _) => do
                  (pure (core_models.ops.control_flow.ControlFlow.Continue
                    countdown))
                | (core_models.result.Result.Err  err) => do
                  (pure (core_models.ops.control_flow.ControlFlow.Break
                    (core_models.ops.control_flow.ControlFlow.Break
                      (core_models.result.Result.Err err))))
            | (core_models.result.Result.Err  err) => do
              (pure (core_models.ops.control_flow.ControlFlow.Break
                (core_models.ops.control_flow.ControlFlow.Break
                  (core_models.result.Result.Err err))))
        else do
          match (← (call false)) with
            | (core_models.result.Result.Ok  _) => do
              (pure (core_models.ops.control_flow.ControlFlow.Continue
                countdown))
            | (core_models.result.Result.Err  err) => do
              (pure (core_models.ops.control_flow.ControlFlow.Break
                (core_models.ops.control_flow.ControlFlow.Break
                  (core_models.result.Result.Err err)))) :
        RustM
        (core_models.ops.control_flow.ControlFlow
          (core_models.ops.control_flow.ControlFlow
            (core_models.result.Result
              rust_primitives.hax.Tuple0
              rust_primitives.hax.Tuple0)
            (rust_primitives.hax.Tuple2 rust_primitives.hax.Tuple0 i32))
          i32)))))
  with
    | (core_models.ops.control_flow.ControlFlow.Break  ret) => do (pure ret)
    | (core_models.ops.control_flow.ControlFlow.Continue  countdown) => do
      (pure (core_models.result.Result.Ok rust_primitives.hax.Tuple0.mk))

structure Thing1 where
  -- no fields

structure Thing2 where
  -- no fields

@[spec]
def Impl.get_thing_2 (self : Thing1) (return_error : Bool) :
    RustM (core_models.result.Result Thing2 rust_primitives.hax.Tuple0) := do
  if return_error then do
    (pure (core_models.result.Result.Err rust_primitives.hax.Tuple0.mk))
  else do
    (pure (core_models.result.Result.Ok Thing2.mk))

@[spec]
def Impl_1.call (self : Thing2) (return_error : Bool) :
    RustM (core_models.result.Result u32 rust_primitives.hax.Tuple0) := do
  if return_error then do
    (pure (core_models.result.Result.Err rust_primitives.hax.Tuple0.mk))
  else do
    (pure (core_models.result.Result.Ok (57 : u32)))

@[spec]
def test2 (_ : rust_primitives.hax.Tuple0) :
    RustM
    (core_models.result.Result
      rust_primitives.hax.Tuple0
      rust_primitives.hax.Tuple0)
    := do
  let thing1 : Thing1 := Thing1.mk;
  let countdown : i32 := (10 : i32);
  match
    (← (rust_primitives.hax.folds.fold_range_return
      (0 : i32)
      (10 : i32)
      (fun countdown _ => (do (pure true) : RustM Bool))
      countdown
      (fun countdown _ =>
        (do
        let countdown : i32 ← (countdown -? (1 : i32));
        if (← (countdown <? (5 : i32))) then do
          match (← (Impl.get_thing_2 thing1 false)) with
            | (core_models.result.Result.Ok  hoist1) => do
              let _ ←
                (core_models.result.Impl.expect_err
                  u32
                  rust_primitives.hax.Tuple0
                  (← (Impl_1.call hoist1 true))
                  "call should fail");
              match (← (Impl.get_thing_2 thing1 false)) with
                | (core_models.result.Result.Ok  hoist3) => do
                  let _ ←
                    (core_models.result.Impl.expect_err
                      u32
                      rust_primitives.hax.Tuple0
                      (← (Impl_1.call hoist3 true))
                      "call should fail");
                  match (← (Impl.get_thing_2 thing1 true)) with
                    | (core_models.result.Result.Ok  hoist5) => do
                      match (← (Impl_1.call hoist5 true)) with
                        | (core_models.result.Result.Ok  val) => do
                          let _ ←
                            match
                              (rust_primitives.hax.Tuple2.mk val (57 : u32))
                            with
                              | ⟨left_val, right_val⟩ => do
                                (hax_lib.assert (← (left_val ==? right_val)));
                          match (← (Impl.get_thing_2 thing1 true)) with
                            | (core_models.result.Result.Ok  hoist7) => do
                              match (← (Impl_1.call hoist7 false)) with
                                | (core_models.result.Result.Ok  val) => do
                                  let _ ←
                                    match
                                      (rust_primitives.hax.Tuple2.mk
                                        val
                                        (57 : u32))
                                    with
                                      | ⟨left_val, right_val⟩ => do
                                        (hax_lib.assert
                                          (← (left_val ==? right_val)));
                                  (pure
                                  (core_models.ops.control_flow.ControlFlow.Continue
                                    countdown))
                                | (core_models.result.Result.Err  err) => do
                                  (pure
                                  (core_models.ops.control_flow.ControlFlow.Break
                                    (core_models.ops.control_flow.ControlFlow.Break
                                      (core_models.result.Result.Err err))))
                            | (core_models.result.Result.Err  err) => do
                              (pure
                              (core_models.ops.control_flow.ControlFlow.Break
                                (core_models.ops.control_flow.ControlFlow.Break
                                  (core_models.result.Result.Err err))))
                        | (core_models.result.Result.Err  err) => do
                          (pure (core_models.ops.control_flow.ControlFlow.Break
                            (core_models.ops.control_flow.ControlFlow.Break
                              (core_models.result.Result.Err err))))
                    | (core_models.result.Result.Err  err) => do
                      (pure (core_models.ops.control_flow.ControlFlow.Break
                        (core_models.ops.control_flow.ControlFlow.Break
                          (core_models.result.Result.Err err))))
                | (core_models.result.Result.Err  err) => do
                  (pure (core_models.ops.control_flow.ControlFlow.Break
                    (core_models.ops.control_flow.ControlFlow.Break
                      (core_models.result.Result.Err err))))
            | (core_models.result.Result.Err  err) => do
              (pure (core_models.ops.control_flow.ControlFlow.Break
                (core_models.ops.control_flow.ControlFlow.Break
                  (core_models.result.Result.Err err))))
        else do
          match (← (Impl.get_thing_2 thing1 false)) with
            | (core_models.result.Result.Ok  hoist9) => do
              match (← (Impl_1.call hoist9 false)) with
                | (core_models.result.Result.Ok  val) => do
                  let _ ←
                    match (rust_primitives.hax.Tuple2.mk val (57 : u32)) with
                      | ⟨left_val, right_val⟩ => do
                        (hax_lib.assert (← (left_val ==? right_val)));
                  match (← (Impl.get_thing_2 thing1 false)) with
                    | (core_models.result.Result.Ok  hoist11) => do
                      match (← (Impl_1.call hoist11 false)) with
                        | (core_models.result.Result.Ok  val) => do
                          let _ ←
                            match
                              (rust_primitives.hax.Tuple2.mk val (57 : u32))
                            with
                              | ⟨left_val, right_val⟩ => do
                                (hax_lib.assert (← (left_val ==? right_val)));
                          match (← (Impl.get_thing_2 thing1 false)) with
                            | (core_models.result.Result.Ok  hoist13) => do
                              match (← (Impl_1.call hoist13 false)) with
                                | (core_models.result.Result.Ok  val) => do
                                  let _ ←
                                    match
                                      (rust_primitives.hax.Tuple2.mk
                                        val
                                        (57 : u32))
                                    with
                                      | ⟨left_val, right_val⟩ => do
                                        (hax_lib.assert
                                          (← (left_val ==? right_val)));
                                  (pure
                                  (core_models.ops.control_flow.ControlFlow.Continue
                                    countdown))
                                | (core_models.result.Result.Err  err) => do
                                  (pure
                                  (core_models.ops.control_flow.ControlFlow.Break
                                    (core_models.ops.control_flow.ControlFlow.Break
                                      (core_models.result.Result.Err err))))
                            | (core_models.result.Result.Err  err) => do
                              (pure
                              (core_models.ops.control_flow.ControlFlow.Break
                                (core_models.ops.control_flow.ControlFlow.Break
                                  (core_models.result.Result.Err err))))
                        | (core_models.result.Result.Err  err) => do
                          (pure (core_models.ops.control_flow.ControlFlow.Break
                            (core_models.ops.control_flow.ControlFlow.Break
                              (core_models.result.Result.Err err))))
                    | (core_models.result.Result.Err  err) => do
                      (pure (core_models.ops.control_flow.ControlFlow.Break
                        (core_models.ops.control_flow.ControlFlow.Break
                          (core_models.result.Result.Err err))))
                | (core_models.result.Result.Err  err) => do
                  (pure (core_models.ops.control_flow.ControlFlow.Break
                    (core_models.ops.control_flow.ControlFlow.Break
                      (core_models.result.Result.Err err))))
            | (core_models.result.Result.Err  err) => do
              (pure (core_models.ops.control_flow.ControlFlow.Break
                (core_models.ops.control_flow.ControlFlow.Break
                  (core_models.result.Result.Err err)))) :
        RustM
        (core_models.ops.control_flow.ControlFlow
          (core_models.ops.control_flow.ControlFlow
            (core_models.result.Result
              rust_primitives.hax.Tuple0
              rust_primitives.hax.Tuple0)
            (rust_primitives.hax.Tuple2 rust_primitives.hax.Tuple0 i32))
          i32)))))
  with
    | (core_models.ops.control_flow.ControlFlow.Break  ret) => do (pure ret)
    | (core_models.ops.control_flow.ControlFlow.Continue  countdown) => do
      (pure (core_models.result.Result.Ok rust_primitives.hax.Tuple0.mk))

@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM
    (core_models.result.Result
      rust_primitives.hax.Tuple0
      rust_primitives.hax.Tuple0)
    := do
  let _ ←
    (core_models.result.Impl.expect_err
      rust_primitives.hax.Tuple0
      rust_primitives.hax.Tuple0
      (← (test1 rust_primitives.hax.Tuple0.mk))
      "test1 should fail");
  match (← (test2 rust_primitives.hax.Tuple0.mk)) with
    | (core_models.result.Result.Ok  _) => do
      (pure (core_models.result.Result.Ok rust_primitives.hax.Tuple0.mk))
    | (core_models.result.Result.Err  err) => do
      (pure (core_models.result.Result.Err err))

end new_tests.rustc_coverage__try_error_result

