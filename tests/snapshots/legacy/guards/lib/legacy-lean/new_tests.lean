
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


namespace new_tests.legacy__guards__lib

@[spec]
def if_let_guard
    (x : (core_models.option.Option (core_models.result.Result i32 i32))) :
    RustM i32 := do
  match x with
    | (core_models.option.Option.None ) => do (pure (0 : i32))
    | _ => do
      match
        (← match x with
          | (core_models.option.Option.Some  v) => do
            match v with
              | (core_models.result.Result.Ok  y) => do
                (pure (core_models.option.Option.Some y))
              | _ => do (pure core_models.option.Option.None)
          | _ => do (pure core_models.option.Option.None))
      with
        | (core_models.option.Option.Some  x) => do (pure x)
        | (core_models.option.Option.None ) => do
          match x with
            | (core_models.option.Option.Some
                 (core_models.result.Result.Err  y)) => do
              (pure y)
            | _ => do (pure (1 : i32))

@[spec]
def equivalent
    (x : (core_models.option.Option (core_models.result.Result i32 i32))) :
    RustM i32 := do
  match x with
    | (core_models.option.Option.None ) => do (pure (0 : i32))
    | _ => do
      match
        (← match x with
          | (core_models.option.Option.Some  v) => do
            match v with
              | (core_models.result.Result.Ok  y) => do
                (pure (core_models.option.Option.Some y))
              | _ => do (pure core_models.option.Option.None)
          | _ => do (pure core_models.option.Option.None))
      with
        | (core_models.option.Option.Some  y) => do (pure y)
        | (core_models.option.Option.None ) => do
          match x with
            | (core_models.option.Option.Some
                 (core_models.result.Result.Err  y)) => do
              (pure y)
            | _ => do (pure (1 : i32))

@[spec]
def multiple_guards
    (x : (core_models.option.Option (core_models.result.Result i32 i32))) :
    RustM i32 := do
  match x with
    | (core_models.option.Option.None ) => do (pure (0 : i32))
    | _ => do
      match
        (← match x with
          | (core_models.option.Option.Some  (core_models.result.Result.Ok  v))
            => do
            match (core_models.option.Option.Some (← (v +? (1 : i32)))) with
              | (core_models.option.Option.Some  1) => do
                (pure (core_models.option.Option.Some (0 : i32)))
              | _ => do (pure core_models.option.Option.None)
          | _ => do (pure core_models.option.Option.None))
      with
        | (core_models.option.Option.Some  x) => do (pure x)
        | (core_models.option.Option.None ) => do
          match
            (← match x with
              | (core_models.option.Option.Some  v) => do
                match v with
                  | (core_models.result.Result.Ok  y) => do
                    (pure (core_models.option.Option.Some y))
                  | _ => do (pure core_models.option.Option.None)
              | _ => do (pure core_models.option.Option.None))
          with
            | (core_models.option.Option.Some  x) => do (pure x)
            | (core_models.option.Option.None ) => do
              match x with
                | (core_models.option.Option.Some
                     (core_models.result.Result.Err  y)) => do
                  (pure y)
                | _ => do (pure (1 : i32))

@[spec]
def if_guard (x : (core_models.option.Option i32)) : RustM i32 := do
  match
    (← match x with
      | (core_models.option.Option.Some  v) => do
        match (← (v >? (0 : i32))) with
          | true => do (pure (core_models.option.Option.Some v))
          | _ => do (pure core_models.option.Option.None)
      | _ => do (pure core_models.option.Option.None))
  with
    | (core_models.option.Option.Some  x) => do (pure x)
    | (core_models.option.Option.None ) => do (pure (0 : i32))

end new_tests.legacy__guards__lib

