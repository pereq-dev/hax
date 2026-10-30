module New_tests.Legacy__guards__lib
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let if_let_guard (x: Core_models.Option.t_Option (Core_models.Result.t_Result i32 i32)) : i32 =
  match x <: Core_models.Option.t_Option (Core_models.Result.t_Result i32 i32) with
  | Core_models.Option.Option_None  -> mk_i32 0
  | _ ->
    match
      (match x <: Core_models.Option.t_Option (Core_models.Result.t_Result i32 i32) with
        | Core_models.Option.Option_Some v ->
          (match v <: Core_models.Result.t_Result i32 i32 with
            | Core_models.Result.Result_Ok y ->
              Core_models.Option.Option_Some y <: Core_models.Option.t_Option i32
            | _ -> Core_models.Option.Option_None <: Core_models.Option.t_Option i32)
        | _ -> Core_models.Option.Option_None <: Core_models.Option.t_Option i32)
      <:
      Core_models.Option.t_Option i32
    with
    | Core_models.Option.Option_Some x -> x
    | Core_models.Option.Option_None  ->
      match x <: Core_models.Option.t_Option (Core_models.Result.t_Result i32 i32) with
      | Core_models.Option.Option_Some (Core_models.Result.Result_Err y) -> y
      | _ -> mk_i32 1

let equivalent (x: Core_models.Option.t_Option (Core_models.Result.t_Result i32 i32)) : i32 =
  match x <: Core_models.Option.t_Option (Core_models.Result.t_Result i32 i32) with
  | Core_models.Option.Option_None  -> mk_i32 0
  | _ ->
    match
      (match x <: Core_models.Option.t_Option (Core_models.Result.t_Result i32 i32) with
        | Core_models.Option.Option_Some v ->
          (match v <: Core_models.Result.t_Result i32 i32 with
            | Core_models.Result.Result_Ok y ->
              Core_models.Option.Option_Some y <: Core_models.Option.t_Option i32
            | _ -> Core_models.Option.Option_None <: Core_models.Option.t_Option i32)
        | _ -> Core_models.Option.Option_None <: Core_models.Option.t_Option i32)
      <:
      Core_models.Option.t_Option i32
    with
    | Core_models.Option.Option_Some y -> y
    | Core_models.Option.Option_None  ->
      match x <: Core_models.Option.t_Option (Core_models.Result.t_Result i32 i32) with
      | Core_models.Option.Option_Some (Core_models.Result.Result_Err y) -> y
      | _ -> mk_i32 1

let multiple_guards (x: Core_models.Option.t_Option (Core_models.Result.t_Result i32 i32)) : i32 =
  match x <: Core_models.Option.t_Option (Core_models.Result.t_Result i32 i32) with
  | Core_models.Option.Option_None  -> mk_i32 0
  | _ ->
    match
      (match x <: Core_models.Option.t_Option (Core_models.Result.t_Result i32 i32) with
        | Core_models.Option.Option_Some (Core_models.Result.Result_Ok v) ->
          (match
              Core_models.Option.Option_Some (v +! mk_i32 1) <: Core_models.Option.t_Option i32
            with
            | Core_models.Option.Option_Some (Rust_primitives.Integers.MkInt 1) ->
              Core_models.Option.Option_Some (mk_i32 0) <: Core_models.Option.t_Option i32
            | _ -> Core_models.Option.Option_None <: Core_models.Option.t_Option i32)
        | _ -> Core_models.Option.Option_None <: Core_models.Option.t_Option i32)
      <:
      Core_models.Option.t_Option i32
    with
    | Core_models.Option.Option_Some x -> x
    | Core_models.Option.Option_None  ->
      match
        (match x <: Core_models.Option.t_Option (Core_models.Result.t_Result i32 i32) with
          | Core_models.Option.Option_Some v ->
            (match v <: Core_models.Result.t_Result i32 i32 with
              | Core_models.Result.Result_Ok y ->
                Core_models.Option.Option_Some y <: Core_models.Option.t_Option i32
              | _ -> Core_models.Option.Option_None <: Core_models.Option.t_Option i32)
          | _ -> Core_models.Option.Option_None <: Core_models.Option.t_Option i32)
        <:
        Core_models.Option.t_Option i32
      with
      | Core_models.Option.Option_Some x -> x
      | Core_models.Option.Option_None  ->
        match x <: Core_models.Option.t_Option (Core_models.Result.t_Result i32 i32) with
        | Core_models.Option.Option_Some (Core_models.Result.Result_Err y) -> y
        | _ -> mk_i32 1

let if_guard (x: Core_models.Option.t_Option i32) : i32 =
  match
    (match x <: Core_models.Option.t_Option i32 with
      | Core_models.Option.Option_Some v ->
        (match v >. mk_i32 0 <: bool with
          | true -> Core_models.Option.Option_Some v <: Core_models.Option.t_Option i32
          | _ -> Core_models.Option.Option_None <: Core_models.Option.t_Option i32)
      | _ -> Core_models.Option.Option_None <: Core_models.Option.t_Option i32)
    <:
    Core_models.Option.t_Option i32
  with
  | Core_models.Option.Option_Some x -> x
  | Core_models.Option.Option_None  -> mk_i32 0
