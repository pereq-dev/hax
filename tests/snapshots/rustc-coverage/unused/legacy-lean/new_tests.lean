
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


namespace new_tests.rustc_coverage__unused

--  @fail(extraction): ssprove(HAX0001), coq(HAX0001, HAX0001)
@[spec]
def foo (T : Type) (x : T) :
    RustM (rust_primitives.hax.Tuple2 i32 rust_primitives.hax.Tuple0) := do
  let i : i32 := (0 : i32);
  (pure (rust_primitives.hax.Tuple2.mk
    (← (rust_primitives.hax.while_loop
      (fun i => (do (pure true) : RustM Bool))
      (fun i => (do (i <? (10 : i32)) : RustM Bool))
      (fun i =>
        (do
        (rust_primitives.hax.int.from_machine (0 : u32)) :
        RustM hax_lib.int.Int))
      i
      (fun i =>
        (do
        let _ ← ((← (i !=? (0 : i32))) ||? (← (i !=? (0 : i32))));
        let i : i32 ← (i +? (1 : i32));
        (pure i) :
        RustM i32))))
    rust_primitives.hax.Tuple0.mk))

--  @fail(extraction): ssprove(HAX0001), coq(HAX0001, HAX0001)
@[spec]
def unused_template_func (T : Type) (x : T) :
    RustM (rust_primitives.hax.Tuple2 i32 rust_primitives.hax.Tuple0) := do
  let i : i32 := (0 : i32);
  (pure (rust_primitives.hax.Tuple2.mk
    (← (rust_primitives.hax.while_loop
      (fun i => (do (pure true) : RustM Bool))
      (fun i => (do (i <? (10 : i32)) : RustM Bool))
      (fun i =>
        (do
        (rust_primitives.hax.int.from_machine (0 : u32)) :
        RustM hax_lib.int.Int))
      i
      (fun i =>
        (do
        let _ ← ((← (i !=? (0 : i32))) ||? (← (i !=? (0 : i32))));
        let i : i32 ← (i +? (1 : i32));
        (pure i) :
        RustM i32))))
    rust_primitives.hax.Tuple0.mk))

@[spec]
def unused_func (a : u32) : RustM rust_primitives.hax.Tuple0 := do
  if (← (a !=? (0 : u32))) then do
    let a : u32 ← (a +? (1 : u32));
    (pure rust_primitives.hax.Tuple0.mk)
  else do
    (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def unused_func2 (a : u32) : RustM rust_primitives.hax.Tuple0 := do
  if (← (a !=? (0 : u32))) then do
    let a : u32 ← (a +? (1 : u32));
    (pure rust_primitives.hax.Tuple0.mk)
  else do
    (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def unused_func3 (a : u32) : RustM rust_primitives.hax.Tuple0 := do
  if (← (a !=? (0 : u32))) then do
    let a : u32 ← (a +? (1 : u32));
    (pure rust_primitives.hax.Tuple0.mk)
  else do
    (pure rust_primitives.hax.Tuple0.mk)

--  @fail(extraction): ssprove(HAX0001)
@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM (core_models.result.Result rust_primitives.hax.Tuple0 u8) := do
  let _ ← (foo u32 (0 : u32));
  let _ ← (foo f32 (0.0 : f32));
  (pure (core_models.result.Result.Ok rust_primitives.hax.Tuple0.mk))

end new_tests.rustc_coverage__unused

