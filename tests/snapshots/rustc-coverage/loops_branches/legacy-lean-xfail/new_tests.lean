
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


namespace new_tests.rustc_coverage__loops_branches

structure DebugTest where
  -- no fields

@[spec]
def Impl.fmt_hoisted (self : DebugTest) (f : core_models.fmt.Formatter) :
    RustM
    (rust_primitives.hax.Tuple2
      core_models.fmt.Formatter
      (core_models.result.Result
        rust_primitives.hax.Tuple0
        core_models.fmt.Error))
    := do
  if true then do
    let _ ←
      if false then do
        (rust_primitives.hax.while_loop
          (fun _ => (do (pure true) : RustM Bool))
          (fun _ => (do (pure true) : RustM Bool))
          (fun _ =>
            (do
            (rust_primitives.hax.int.from_machine (0 : u32)) :
            RustM hax_lib.int.Int))
          rust_primitives.hax.Tuple0.mk
          (fun _ =>
            (do
            (pure rust_primitives.hax.Tuple0.mk) :
            RustM rust_primitives.hax.Tuple0)))
      else do
        (pure rust_primitives.hax.Tuple0.mk);
    let ⟨tmp0, out⟩ ←
      (core_models.fmt.Impl_11.write_fmt
        f
        (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
          (RustArray.ofVec #v["cool"]))));
    let f : core_models.fmt.Formatter := tmp0;
    match out with
      | (core_models.result.Result.Ok  _) => do
        match
          (← (rust_primitives.hax.folds.fold_range_return
            (0 : i32)
            (10 : i32)
            (fun f _ => (do (pure true) : RustM Bool))
            f
            (fun f i =>
              (do
              if true then do
                let _ ←
                  if false then do
                    (rust_primitives.hax.while_loop
                      (fun _ => (do (pure true) : RustM Bool))
                      (fun _ => (do (pure true) : RustM Bool))
                      (fun _ =>
                        (do
                        (rust_primitives.hax.int.from_machine (0 : u32)) :
                        RustM hax_lib.int.Int))
                      rust_primitives.hax.Tuple0.mk
                      (fun _ =>
                        (do
                        (pure rust_primitives.hax.Tuple0.mk) :
                        RustM rust_primitives.hax.Tuple0)))
                  else do
                    (pure rust_primitives.hax.Tuple0.mk);
                let ⟨tmp0, out⟩ ←
                  (core_models.fmt.Impl_11.write_fmt
                    f
                    (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
                      (RustArray.ofVec #v["cool"]))));
                let f : core_models.fmt.Formatter := tmp0;
                match out with
                  | (core_models.result.Result.Ok  _) => do
                    (pure (core_models.ops.control_flow.ControlFlow.Continue f))
                  | (core_models.result.Result.Err  err) => do
                    (pure (core_models.ops.control_flow.ControlFlow.Break
                      (core_models.ops.control_flow.ControlFlow.Break
                        (rust_primitives.hax.Tuple2.mk
                          f
                          (core_models.result.Result.Err err)))))
              else do
                (pure (core_models.ops.control_flow.ControlFlow.Continue f)) :
              RustM
              (core_models.ops.control_flow.ControlFlow
                (core_models.ops.control_flow.ControlFlow
                  (rust_primitives.hax.Tuple2
                    core_models.fmt.Formatter
                    (core_models.result.Result
                      rust_primitives.hax.Tuple0
                      core_models.fmt.Error))
                  (rust_primitives.hax.Tuple2
                    rust_primitives.hax.Tuple0
                    core_models.fmt.Formatter))
                core_models.fmt.Formatter)))))
        with
          | (core_models.ops.control_flow.ControlFlow.Break  ret) => do
            (pure ret)
          | (core_models.ops.control_flow.ControlFlow.Continue  f) => do
            let
              hax_temp_output : (core_models.result.Result
                rust_primitives.hax.Tuple0
                core_models.fmt.Error) :=
              (core_models.result.Result.Ok rust_primitives.hax.Tuple0.mk);
            (pure (rust_primitives.hax.Tuple2.mk f hax_temp_output))
      | (core_models.result.Result.Err  err) => do
        (pure (rust_primitives.hax.Tuple2.mk
          f
          (core_models.result.Result.Err err)))
  else do
    match
      (← (rust_primitives.hax.folds.fold_range_return
        (0 : i32)
        (10 : i32)
        (fun f _ => (do (pure true) : RustM Bool))
        f
        (fun f i =>
          (do
          if true then do
            let _ ←
              if false then do
                (rust_primitives.hax.while_loop
                  (fun _ => (do (pure true) : RustM Bool))
                  (fun _ => (do (pure true) : RustM Bool))
                  (fun _ =>
                    (do
                    (rust_primitives.hax.int.from_machine (0 : u32)) :
                    RustM hax_lib.int.Int))
                  rust_primitives.hax.Tuple0.mk
                  (fun _ =>
                    (do
                    (pure rust_primitives.hax.Tuple0.mk) :
                    RustM rust_primitives.hax.Tuple0)))
              else do
                (pure rust_primitives.hax.Tuple0.mk);
            let ⟨tmp0, out⟩ ←
              (core_models.fmt.Impl_11.write_fmt
                f
                (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
                  (RustArray.ofVec #v["cool"]))));
            let f : core_models.fmt.Formatter := tmp0;
            match out with
              | (core_models.result.Result.Ok  _) => do
                (pure (core_models.ops.control_flow.ControlFlow.Continue f))
              | (core_models.result.Result.Err  err) => do
                (pure (core_models.ops.control_flow.ControlFlow.Break
                  (core_models.ops.control_flow.ControlFlow.Break
                    (rust_primitives.hax.Tuple2.mk
                      f
                      (core_models.result.Result.Err err)))))
          else do
            (pure (core_models.ops.control_flow.ControlFlow.Continue f)) :
          RustM
          (core_models.ops.control_flow.ControlFlow
            (core_models.ops.control_flow.ControlFlow
              (rust_primitives.hax.Tuple2
                core_models.fmt.Formatter
                (core_models.result.Result
                  rust_primitives.hax.Tuple0
                  core_models.fmt.Error))
              (rust_primitives.hax.Tuple2
                rust_primitives.hax.Tuple0
                core_models.fmt.Formatter))
            core_models.fmt.Formatter)))))
    with
      | (core_models.ops.control_flow.ControlFlow.Break  ret) => do (pure ret)
      | (core_models.ops.control_flow.ControlFlow.Continue  f) => do
        let
          hax_temp_output : (core_models.result.Result
            rust_primitives.hax.Tuple0
            core_models.fmt.Error) :=
          (core_models.result.Result.Ok rust_primitives.hax.Tuple0.mk);
        (pure (rust_primitives.hax.Tuple2.mk f hax_temp_output))

--  @fail(extraction): coq(HAX0001, HAX0001, HAX0001, HAX0001), ssprove(HAX0001, HAX0001)
@[reducible] instance Impl.AssociatedTypes :
  core_models.fmt.Debug.AssociatedTypes DebugTest
  where

instance Impl : core_models.fmt.Debug DebugTest where
  fmt := (Impl.fmt_hoisted)

structure DisplayTest where
  -- no fields

@[spec]
def Impl_1.fmt_hoisted (self : DisplayTest) (f : core_models.fmt.Formatter) :
    RustM
    (rust_primitives.hax.Tuple2
      core_models.fmt.Formatter
      (core_models.result.Result
        rust_primitives.hax.Tuple0
        core_models.fmt.Error))
    := do
  if false then do
    match
      (← (rust_primitives.hax.folds.fold_range_return
        (0 : i32)
        (10 : i32)
        (fun f _ => (do (pure true) : RustM Bool))
        f
        (fun f i =>
          (do
          if false then do
            (pure (core_models.ops.control_flow.ControlFlow.Continue f))
          else do
            let _ ←
              if false then do
                (rust_primitives.hax.while_loop
                  (fun _ => (do (pure true) : RustM Bool))
                  (fun _ => (do (pure true) : RustM Bool))
                  (fun _ =>
                    (do
                    (rust_primitives.hax.int.from_machine (0 : u32)) :
                    RustM hax_lib.int.Int))
                  rust_primitives.hax.Tuple0.mk
                  (fun _ =>
                    (do
                    (pure rust_primitives.hax.Tuple0.mk) :
                    RustM rust_primitives.hax.Tuple0)))
              else do
                (pure rust_primitives.hax.Tuple0.mk);
            let ⟨tmp0, out⟩ ←
              (core_models.fmt.Impl_11.write_fmt
                f
                (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
                  (RustArray.ofVec #v["cool"]))));
            let f : core_models.fmt.Formatter := tmp0;
            match out with
              | (core_models.result.Result.Ok  _) => do
                (pure (core_models.ops.control_flow.ControlFlow.Continue f))
              | (core_models.result.Result.Err  err) => do
                (pure (core_models.ops.control_flow.ControlFlow.Break
                  (core_models.ops.control_flow.ControlFlow.Break
                    (rust_primitives.hax.Tuple2.mk
                      f
                      (core_models.result.Result.Err err))))) :
          RustM
          (core_models.ops.control_flow.ControlFlow
            (core_models.ops.control_flow.ControlFlow
              (rust_primitives.hax.Tuple2
                core_models.fmt.Formatter
                (core_models.result.Result
                  rust_primitives.hax.Tuple0
                  core_models.fmt.Error))
              (rust_primitives.hax.Tuple2
                rust_primitives.hax.Tuple0
                core_models.fmt.Formatter))
            core_models.fmt.Formatter)))))
    with
      | (core_models.ops.control_flow.ControlFlow.Break  ret) => do (pure ret)
      | (core_models.ops.control_flow.ControlFlow.Continue  f) => do
        let
          hax_temp_output : (core_models.result.Result
            rust_primitives.hax.Tuple0
            core_models.fmt.Error) :=
          (core_models.result.Result.Ok rust_primitives.hax.Tuple0.mk);
        (pure (rust_primitives.hax.Tuple2.mk f hax_temp_output))
  else do
    let _ ←
      if false then do
        (rust_primitives.hax.while_loop
          (fun _ => (do (pure true) : RustM Bool))
          (fun _ => (do (pure true) : RustM Bool))
          (fun _ =>
            (do
            (rust_primitives.hax.int.from_machine (0 : u32)) :
            RustM hax_lib.int.Int))
          rust_primitives.hax.Tuple0.mk
          (fun _ =>
            (do
            (pure rust_primitives.hax.Tuple0.mk) :
            RustM rust_primitives.hax.Tuple0)))
      else do
        (pure rust_primitives.hax.Tuple0.mk);
    let ⟨tmp0, out⟩ ←
      (core_models.fmt.Impl_11.write_fmt
        f
        (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
          (RustArray.ofVec #v["cool"]))));
    let f : core_models.fmt.Formatter := tmp0;
    match out with
      | (core_models.result.Result.Ok  _) => do
        match
          (← (rust_primitives.hax.folds.fold_range_return
            (0 : i32)
            (10 : i32)
            (fun f _ => (do (pure true) : RustM Bool))
            f
            (fun f i =>
              (do
              if false then do
                (pure (core_models.ops.control_flow.ControlFlow.Continue f))
              else do
                let _ ←
                  if false then do
                    (rust_primitives.hax.while_loop
                      (fun _ => (do (pure true) : RustM Bool))
                      (fun _ => (do (pure true) : RustM Bool))
                      (fun _ =>
                        (do
                        (rust_primitives.hax.int.from_machine (0 : u32)) :
                        RustM hax_lib.int.Int))
                      rust_primitives.hax.Tuple0.mk
                      (fun _ =>
                        (do
                        (pure rust_primitives.hax.Tuple0.mk) :
                        RustM rust_primitives.hax.Tuple0)))
                  else do
                    (pure rust_primitives.hax.Tuple0.mk);
                let ⟨tmp0, out⟩ ←
                  (core_models.fmt.Impl_11.write_fmt
                    f
                    (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
                      (RustArray.ofVec #v["cool"]))));
                let f : core_models.fmt.Formatter := tmp0;
                match out with
                  | (core_models.result.Result.Ok  _) => do
                    (pure (core_models.ops.control_flow.ControlFlow.Continue f))
                  | (core_models.result.Result.Err  err) => do
                    (pure (core_models.ops.control_flow.ControlFlow.Break
                      (core_models.ops.control_flow.ControlFlow.Break
                        (rust_primitives.hax.Tuple2.mk
                          f
                          (core_models.result.Result.Err err))))) :
              RustM
              (core_models.ops.control_flow.ControlFlow
                (core_models.ops.control_flow.ControlFlow
                  (rust_primitives.hax.Tuple2
                    core_models.fmt.Formatter
                    (core_models.result.Result
                      rust_primitives.hax.Tuple0
                      core_models.fmt.Error))
                  (rust_primitives.hax.Tuple2
                    rust_primitives.hax.Tuple0
                    core_models.fmt.Formatter))
                core_models.fmt.Formatter)))))
        with
          | (core_models.ops.control_flow.ControlFlow.Break  ret) => do
            (pure ret)
          | (core_models.ops.control_flow.ControlFlow.Continue  f) => do
            let
              hax_temp_output : (core_models.result.Result
                rust_primitives.hax.Tuple0
                core_models.fmt.Error) :=
              (core_models.result.Result.Ok rust_primitives.hax.Tuple0.mk);
            (pure (rust_primitives.hax.Tuple2.mk f hax_temp_output))
      | (core_models.result.Result.Err  err) => do
        (pure (rust_primitives.hax.Tuple2.mk
          f
          (core_models.result.Result.Err err)))

--  @fail(extraction): coq(HAX0001, HAX0001, HAX0001, HAX0001), ssprove(HAX0001, HAX0001)
@[reducible] instance Impl_1.AssociatedTypes :
  core_models.fmt.Display.AssociatedTypes DisplayTest
  where

instance Impl_1 : core_models.fmt.Display DisplayTest where
  fmt := (Impl_1.fmt_hoisted)

--  @fail(extraction): ssprove(HAX0001)
@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let debug_test : DebugTest := DebugTest.mk;
  let args : (rust_primitives.hax.Tuple1 DebugTest) :=
    (rust_primitives.hax.Tuple1.mk debug_test);
  let args : (RustArray core_models.fmt.rt.Argument 1) :=
    (RustArray.ofVec #v[(← (core_models.fmt.rt.Impl.new_debug DebugTest
                            (rust_primitives.hax.Tuple1._0 args)))]);
  let _ ←
    (std.io.stdio._print
      (← (core_models.fmt.rt.Impl_1.new_v1 ((2 : usize)) ((1 : usize))
        (RustArray.ofVec #v["", "\n"])
        args)));
  let _ := rust_primitives.hax.Tuple0.mk;
  let display_test : DisplayTest := DisplayTest.mk;
  let args : (rust_primitives.hax.Tuple1 DisplayTest) :=
    (rust_primitives.hax.Tuple1.mk display_test);
  let args : (RustArray core_models.fmt.rt.Argument 1) :=
    (RustArray.ofVec #v[(← (core_models.fmt.rt.Impl.new_display DisplayTest
                            (rust_primitives.hax.Tuple1._0 args)))]);
  let _ ←
    (std.io.stdio._print
      (← (core_models.fmt.rt.Impl_1.new_v1 ((2 : usize)) ((1 : usize))
        (RustArray.ofVec #v["", "\n"])
        args)));
  let _ := rust_primitives.hax.Tuple0.mk;
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.rustc_coverage__loops_branches

