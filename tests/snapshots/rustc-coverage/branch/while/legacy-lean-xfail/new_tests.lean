
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


namespace new_tests.rustc_coverage__branch__while

--  @fail(extraction): ssprove(HAX0001), coq(HAX0001, HAX0001)
@[spec]
def while_cond (_ : rust_primitives.hax.Tuple0) :
    RustM (rust_primitives.hax.Tuple2 i32 rust_primitives.hax.Tuple0) := do
  let _ ←
    (rust_primitives.hax.folds.fold_range
      (0 : i32)
      (1 : i32)
      (fun _ _ => (do (pure true) : RustM Bool))
      rust_primitives.hax.Tuple0.mk
      (fun _ _ =>
        (do
        (pure rust_primitives.hax.Tuple0.mk) :
        RustM rust_primitives.hax.Tuple0)));
  let a : i32 := (8 : i32);
  (pure (rust_primitives.hax.Tuple2.mk
    (← (rust_primitives.hax.while_loop
      (fun a => (do (pure true) : RustM Bool))
      (fun a => (do (a >? (0 : i32)) : RustM Bool))
      (fun a =>
        (do
        (rust_primitives.hax.int.from_machine (0 : u32)) :
        RustM hax_lib.int.Int))
      a
      (fun a => (do let a : i32 ← (a -? (1 : i32)); (pure a) : RustM i32))))
    rust_primitives.hax.Tuple0.mk))

--  @fail(extraction): ssprove(HAX0001), coq(HAX0001, HAX0001)
@[spec]
def while_cond_not (_ : rust_primitives.hax.Tuple0) :
    RustM (rust_primitives.hax.Tuple2 i32 rust_primitives.hax.Tuple0) := do
  let _ ←
    (rust_primitives.hax.folds.fold_range
      (0 : i32)
      (1 : i32)
      (fun _ _ => (do (pure true) : RustM Bool))
      rust_primitives.hax.Tuple0.mk
      (fun _ _ =>
        (do
        (pure rust_primitives.hax.Tuple0.mk) :
        RustM rust_primitives.hax.Tuple0)));
  let a : i32 := (8 : i32);
  (pure (rust_primitives.hax.Tuple2.mk
    (← (rust_primitives.hax.while_loop
      (fun a => (do (pure true) : RustM Bool))
      (fun a => (do (!? (← (a ==? (0 : i32)))) : RustM Bool))
      (fun a =>
        (do
        (rust_primitives.hax.int.from_machine (0 : u32)) :
        RustM hax_lib.int.Int))
      a
      (fun a => (do let a : i32 ← (a -? (1 : i32)); (pure a) : RustM i32))))
    rust_primitives.hax.Tuple0.mk))

--  @fail(extraction): coq(HAX0001, HAX0001), ssprove(HAX0001)
@[spec]
def while_op_and (_ : rust_primitives.hax.Tuple0) :
    RustM
    (rust_primitives.hax.Tuple2
      (rust_primitives.hax.Tuple2 i32 i32)
      rust_primitives.hax.Tuple0)
    := do
  let _ ←
    (rust_primitives.hax.folds.fold_range
      (0 : i32)
      (1 : i32)
      (fun _ _ => (do (pure true) : RustM Bool))
      rust_primitives.hax.Tuple0.mk
      (fun _ _ =>
        (do
        (pure rust_primitives.hax.Tuple0.mk) :
        RustM rust_primitives.hax.Tuple0)));
  let a : i32 := (8 : i32);
  let b : i32 := (4 : i32);
  (pure (rust_primitives.hax.Tuple2.mk
    (← (rust_primitives.hax.while_loop
      (fun ⟨a, b⟩ => (do (pure true) : RustM Bool))
      (fun ⟨a, b⟩ =>
        (do ((← (a >? (0 : i32))) &&? (← (b >? (0 : i32)))) : RustM Bool))
      (fun ⟨a, b⟩ =>
        (do
        (rust_primitives.hax.int.from_machine (0 : u32)) :
        RustM hax_lib.int.Int))
      (rust_primitives.hax.Tuple2.mk a b)
      (fun ⟨a, b⟩ =>
        (do
        let a : i32 ← (a -? (1 : i32));
        let b : i32 ← (b -? (1 : i32));
        (pure (rust_primitives.hax.Tuple2.mk a b)) :
        RustM (rust_primitives.hax.Tuple2 i32 i32)))))
    rust_primitives.hax.Tuple0.mk))

--  @fail(extraction): coq(HAX0001, HAX0001), ssprove(HAX0001)
@[spec]
def while_op_or (_ : rust_primitives.hax.Tuple0) :
    RustM
    (rust_primitives.hax.Tuple2
      (rust_primitives.hax.Tuple2 i32 i32)
      rust_primitives.hax.Tuple0)
    := do
  let _ ←
    (rust_primitives.hax.folds.fold_range
      (0 : i32)
      (1 : i32)
      (fun _ _ => (do (pure true) : RustM Bool))
      rust_primitives.hax.Tuple0.mk
      (fun _ _ =>
        (do
        (pure rust_primitives.hax.Tuple0.mk) :
        RustM rust_primitives.hax.Tuple0)));
  let a : i32 := (4 : i32);
  let b : i32 := (8 : i32);
  (pure (rust_primitives.hax.Tuple2.mk
    (← (rust_primitives.hax.while_loop
      (fun ⟨a, b⟩ => (do (pure true) : RustM Bool))
      (fun ⟨a, b⟩ =>
        (do ((← (a >? (0 : i32))) ||? (← (b >? (0 : i32)))) : RustM Bool))
      (fun ⟨a, b⟩ =>
        (do
        (rust_primitives.hax.int.from_machine (0 : u32)) :
        RustM hax_lib.int.Int))
      (rust_primitives.hax.Tuple2.mk a b)
      (fun ⟨a, b⟩ =>
        (do
        let a : i32 ← (a -? (1 : i32));
        let b : i32 ← (b -? (1 : i32));
        (pure (rust_primitives.hax.Tuple2.mk a b)) :
        RustM (rust_primitives.hax.Tuple2 i32 i32)))))
    rust_primitives.hax.Tuple0.mk))

@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ ← (while_cond rust_primitives.hax.Tuple0.mk);
  let _ ← (while_cond_not rust_primitives.hax.Tuple0.mk);
  let _ ← (while_op_and rust_primitives.hax.Tuple0.mk);
  let _ ← (while_op_or rust_primitives.hax.Tuple0.mk);
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.rustc_coverage__branch__while

