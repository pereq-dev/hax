
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


namespace new_tests.legacy__hoisting__lib.issue_2336

@[spec]
def change (value : Bool) : RustM (rust_primitives.hax.Tuple2 Bool Bool) := do
  let value : Bool := true;
  let hax_temp_output : Bool := false;
  (pure (rust_primitives.hax.Tuple2.mk value hax_temp_output))

@[spec]
def or_mut_ref (first : Bool) (value : Bool) :
    RustM (rust_primitives.hax.Tuple2 Bool Bool) := do
  let ⟨value, hax_temp_output⟩ ←
    if first then do
      (pure (rust_primitives.hax.Tuple2.mk value true))
    else do
      let ⟨tmp0, out⟩ ← (change value);
      let value : Bool := tmp0;
      (pure (rust_primitives.hax.Tuple2.mk value out));
  (pure (rust_primitives.hax.Tuple2.mk value hax_temp_output))

@[spec]
def and_mut_ref (first : Bool) (value : Bool) :
    RustM (rust_primitives.hax.Tuple2 Bool Bool) := do
  let ⟨value, hax_temp_output⟩ ←
    if first then do
      let ⟨tmp0, out⟩ ← (change value);
      let value : Bool := tmp0;
      (pure (rust_primitives.hax.Tuple2.mk value out))
    else do
      (pure (rust_primitives.hax.Tuple2.mk value false));
  (pure (rust_primitives.hax.Tuple2.mk value hax_temp_output))

@[spec]
def or_assign (first : Bool) : RustM u8 := do
  let x : u8 := (0 : u8);
  let x : u8 ←
    if first then do (pure x) else do let x : u8 := (1 : u8); (pure x);
  (pure x)

@[spec]
def fallible (b : Bool) : RustM (core_models.result.Result Bool u8) := do
  if b then do
    (pure (core_models.result.Result.Ok true))
  else do
    (pure (core_models.result.Result.Err (1 : u8)))

@[spec]
def or_question_mark (first : Bool) :
    RustM (core_models.result.Result Bool u8) := do
  if first then do
    (pure (core_models.result.Result.Ok true))
  else do
    match (← (fallible false)) with
      | (core_models.result.Result.Ok  ok) => do
        (pure (core_models.result.Result.Ok ok))
      | (core_models.result.Result.Err  err) => do
        (pure (core_models.result.Result.Err err))

@[spec]
def or_return (first : Bool) : RustM u8 := do
  if first then do let _ := true; (pure (0 : u8)) else do (pure (1 : u8))

@[spec]
def and_local_let (i : usize) (v : (RustSlice u8)) (value : Bool) :
    RustM (rust_primitives.hax.Tuple2 Bool Bool) := do
  let ⟨value, hax_temp_output⟩ ←
    if (← (i <? (← (core_models.slice.Impl.len u8 v)))) then do
      let x : u8 ← v[i]_?;
      if (← (x >? (0 : u8))) then do
        (pure (rust_primitives.hax.Tuple2.mk value true))
      else do
        let ⟨tmp0, out⟩ ← (change value);
        let value : Bool := tmp0;
        (pure (rust_primitives.hax.Tuple2.mk value out))
    else do
      (pure (rust_primitives.hax.Tuple2.mk value false));
  (pure (rust_primitives.hax.Tuple2.mk value hax_temp_output))

@[spec]
def if_condition (first : Bool) (value : Bool) :
    RustM (rust_primitives.hax.Tuple2 Bool u8) := do
  let ⟨value, hoist3⟩ ←
    if first then do
      (pure (rust_primitives.hax.Tuple2.mk value true))
    else do
      let ⟨tmp0, out⟩ ← (change value);
      let value : Bool := tmp0;
      (pure (rust_primitives.hax.Tuple2.mk value out));
  let hax_temp_output : u8 ←
    if hoist3 then do (pure (1 : u8)) else do (pure (0 : u8));
  (pure (rust_primitives.hax.Tuple2.mk value hax_temp_output))

--  @fail(extraction): proverif(HAX0008)
@[spec]
def match_guard (o : (core_models.option.Option u8)) (value : Bool) :
    RustM (rust_primitives.hax.Tuple2 Bool u8) := do
  let ⟨value, hoist5⟩ ←
    match o with
      | (core_models.option.Option.Some  x) => do
        let ⟨value, hoist4⟩ ←
          if (← (x >? (3 : u8))) then do
            (pure (rust_primitives.hax.Tuple2.mk value true))
          else do
            let ⟨tmp0, out⟩ ← (change value);
            let value : Bool := tmp0;
            (pure (rust_primitives.hax.Tuple2.mk value out));
        match hoist4 with
          | true => do
            (pure (rust_primitives.hax.Tuple2.mk
              value
              (core_models.option.Option.Some x)))
          | _ => do
            (pure (rust_primitives.hax.Tuple2.mk
              value
              core_models.option.Option.None))
      | _ => do
        (pure (rust_primitives.hax.Tuple2.mk
          value
          core_models.option.Option.None));
  let hax_temp_output : u8 ←
    match hoist5 with
      | (core_models.option.Option.Some  x) => do (pure x)
      | (core_models.option.Option.None ) => do (pure (0 : u8));
  (pure (rust_primitives.hax.Tuple2.mk value hax_temp_output))

@[spec]
def pure_operands (a : Bool) (b : Bool) (c : u8) : RustM Bool := do
  (a &&? (← (b ||? (← (c >? (3 : u8))))))

@[spec]
def left_mut_ref (second : Bool) (value : Bool) :
    RustM (rust_primitives.hax.Tuple2 Bool Bool) := do
  let ⟨tmp0, out⟩ ← (change value);
  let value : Bool := tmp0;
  let hax_temp_output : Bool ← (out ||? second);
  (pure (rust_primitives.hax.Tuple2.mk value hax_temp_output))

@[spec]
def both_mut_refs (a : Bool) (b : Bool) :
    RustM (rust_primitives.hax.Tuple3 Bool Bool Bool) := do
  let ⟨tmp0, out⟩ ← (change a);
  let a : Bool := tmp0;
  let ⟨b, hax_temp_output⟩ ←
    if out then do
      let ⟨tmp0, out⟩ ← (change b);
      let b : Bool := tmp0;
      (pure (rust_primitives.hax.Tuple2.mk b out))
    else do
      (pure (rust_primitives.hax.Tuple2.mk b false));
  (pure (rust_primitives.hax.Tuple3.mk a b hax_temp_output))

@[spec]
def chained (first : Bool) (second : Bool) (value : Bool) :
    RustM (rust_primitives.hax.Tuple2 Bool Bool) := do
  let ⟨value, hax_temp_output⟩ ←
    if (← (first ||? second)) then do
      (pure (rust_primitives.hax.Tuple2.mk value true))
    else do
      let ⟨tmp0, out⟩ ← (change value);
      let value : Bool := tmp0;
      (pure (rust_primitives.hax.Tuple2.mk value out));
  (pure (rust_primitives.hax.Tuple2.mk value hax_temp_output))

end new_tests.legacy__hoisting__lib.issue_2336


namespace new_tests.legacy__hoisting__lib.while_condition

--  @fail(extraction): coq(HAX0001, HAX0001), ssprove(HAX0001), proverif(HAX0008)
@[spec]
def local_let (_ : rust_primitives.hax.Tuple0) : RustM u8 := do
  let n : u8 := (0 : u8);
  let n : u8 ←
    (rust_primitives.hax.while_loop
      (fun n => (do (pure true) : RustM Bool))
      (fun n =>
        (do
        let y : u8 ← (core_models.num.Impl_6.wrapping_add n (1 : u8));
        (y <? (4 : u8)) :
        RustM Bool))
      (fun n =>
        (do
        (rust_primitives.hax.int.from_machine (0 : u32)) :
        RustM hax_lib.int.Int))
      n
      (fun n => (do let n : u8 ← (n +? (1 : u8)); (pure n) : RustM u8)));
  (pure n)

--  @fail(extraction): coq(HAX0001, HAX0001), ssprove(HAX0001), proverif(HAX0008)
@[spec]
def local_mut (_ : rust_primitives.hax.Tuple0) : RustM u8 := do
  let n : u8 := (0 : u8);
  let n : u8 ←
    (rust_primitives.hax.while_loop
      (fun n => (do (pure true) : RustM Bool))
      (fun n =>
        (do
        let y : u8 := n;
        let y : u8 ← (core_models.num.Impl_6.wrapping_add y (1 : u8));
        (y <? (4 : u8)) :
        RustM Bool))
      (fun n =>
        (do
        (rust_primitives.hax.int.from_machine (0 : u32)) :
        RustM hax_lib.int.Int))
      n
      (fun n => (do let n : u8 ← (n +? (1 : u8)); (pure n) : RustM u8)));
  (pure n)

--  @fail(extraction): coq(HAX0001, HAX0001, HAX0008), ssprove(HAX0008, HAX0001), proverif(HAX0008, HAX0008)
opaque inner_loop (n : u8) : RustM u8

@[spec]
def dec (x : u8) : RustM (rust_primitives.hax.Tuple2 u8 Bool) := do
  let ⟨x, hax_temp_output⟩ ←
    if (← (x ==? (0 : u8))) then do
      (pure (rust_primitives.hax.Tuple2.mk x false))
    else do
      let x : u8 ← (x -? (1 : u8));
      (pure (rust_primitives.hax.Tuple2.mk x true));
  (pure (rust_primitives.hax.Tuple2.mk x hax_temp_output))

@[spec]
def fallible (b : Bool) : RustM (core_models.result.Result Bool u8) := do
  if b then do
    (pure (core_models.result.Result.Ok true))
  else do
    (pure (core_models.result.Result.Err (1 : u8)))

end new_tests.legacy__hoisting__lib.while_condition

