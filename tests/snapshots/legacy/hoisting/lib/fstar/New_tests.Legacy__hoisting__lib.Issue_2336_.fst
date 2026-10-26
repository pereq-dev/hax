module New_tests.Legacy__hoisting__lib.Issue_2336_
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let change (value: bool) : (bool & bool) =
  let value:bool = true in
  let hax_temp_output:bool = false in
  value, hax_temp_output <: (bool & bool)

let or_mut_ref (first value: bool) : (bool & bool) =
  let (value: bool), (hax_temp_output: bool) =
    if first
    then value, true <: (bool & bool)
    else
      let (tmp0: bool), (out: bool) = change value in
      let value:bool = tmp0 in
      value, out <: (bool & bool)
  in
  value, hax_temp_output <: (bool & bool)

let and_mut_ref (first value: bool) : (bool & bool) =
  let (value: bool), (hax_temp_output: bool) =
    if first
    then
      let (tmp0: bool), (out: bool) = change value in
      let value:bool = tmp0 in
      value, out <: (bool & bool)
    else value, false <: (bool & bool)
  in
  value, hax_temp_output <: (bool & bool)

let or_assign (first: bool) : u8 =
  let x:u8 = mk_u8 0 in
  let x:u8 =
    if first
    then x
    else
      let x:u8 = mk_u8 1 in
      x
  in
  x

let fallible (b: bool) : Core_models.Result.t_Result bool u8 =
  if b
  then Core_models.Result.Result_Ok true <: Core_models.Result.t_Result bool u8
  else Core_models.Result.Result_Err (mk_u8 1) <: Core_models.Result.t_Result bool u8

let or_question_mark (first: bool) : Core_models.Result.t_Result bool u8 =
  if first
  then Core_models.Result.Result_Ok true <: Core_models.Result.t_Result bool u8
  else
    match fallible false <: Core_models.Result.t_Result bool u8 with
    | Core_models.Result.Result_Ok ok ->
      Core_models.Result.Result_Ok ok <: Core_models.Result.t_Result bool u8
    | Core_models.Result.Result_Err err ->
      Core_models.Result.Result_Err err <: Core_models.Result.t_Result bool u8

let or_return (first: bool) : u8 =
  if first
  then
    let _:bool = true in
    mk_u8 0
  else mk_u8 1

let and_local_let (i: usize) (v: t_Slice u8) (value: bool) : (bool & bool) =
  let (value: bool), (hax_temp_output: bool) =
    if i <. (Core_models.Slice.impl__len #u8 v <: usize)
    then
      let x:u8 = v.[ i ] in
      if x >. mk_u8 0
      then value, true <: (bool & bool)
      else
        let (tmp0: bool), (out: bool) = change value in
        let value:bool = tmp0 in
        value, out <: (bool & bool)
    else value, false <: (bool & bool)
  in
  value, hax_temp_output <: (bool & bool)

let if_condition (first value: bool) : (bool & u8) =
  let (value: bool), (hoist3: bool) =
    if first
    then value, true <: (bool & bool)
    else
      let (tmp0: bool), (out: bool) = change value in
      let value:bool = tmp0 in
      value, out <: (bool & bool)
  in
  let hax_temp_output:u8 = if hoist3 then mk_u8 1 else mk_u8 0 in
  value, hax_temp_output <: (bool & u8)

/// @fail(extraction): proverif(HAX0008)
let match_guard (o: Core_models.Option.t_Option u8) (value: bool) : (bool & u8) =
  let (value: bool), (hoist5: Core_models.Option.t_Option u8) =
    match o <: Core_models.Option.t_Option u8 with
    | Core_models.Option.Option_Some x ->
      let (value: bool), (hoist4: bool) =
        if x >. mk_u8 3
        then value, true <: (bool & bool)
        else
          let (tmp0: bool), (out: bool) = change value in
          let value:bool = tmp0 in
          value, out <: (bool & bool)
      in
      (match hoist4 <: bool with
        | true ->
          value, (Core_models.Option.Option_Some x <: Core_models.Option.t_Option u8)
          <:
          (bool & Core_models.Option.t_Option u8)
        | _ ->
          value, (Core_models.Option.Option_None <: Core_models.Option.t_Option u8)
          <:
          (bool & Core_models.Option.t_Option u8))
    | _ ->
      value, (Core_models.Option.Option_None <: Core_models.Option.t_Option u8)
      <:
      (bool & Core_models.Option.t_Option u8)
  in
  let hax_temp_output:u8 =
    match hoist5 <: Core_models.Option.t_Option u8 with
    | Core_models.Option.Option_Some x -> x
    | Core_models.Option.Option_None  -> mk_u8 0
  in
  value, hax_temp_output <: (bool & u8)

let pure_operands (a b: bool) (c: u8) : bool = a && (b || c >. mk_u8 3)

let left_mut_ref (second value: bool) : (bool & bool) =
  let (tmp0: bool), (out: bool) = change value in
  let value:bool = tmp0 in
  let hax_temp_output:bool = out || second in
  value, hax_temp_output <: (bool & bool)

let both_mut_refs (a b: bool) : (bool & bool & bool) =
  let (tmp0: bool), (out: bool) = change a in
  let a:bool = tmp0 in
  let (b: bool), (hax_temp_output: bool) =
    if out
    then
      let (tmp0: bool), (out: bool) = change b in
      let b:bool = tmp0 in
      b, out <: (bool & bool)
    else b, false <: (bool & bool)
  in
  a, b, hax_temp_output <: (bool & bool & bool)

let chained (first second value: bool) : (bool & bool) =
  let (value: bool), (hax_temp_output: bool) =
    if first || second
    then value, true <: (bool & bool)
    else
      let (tmp0: bool), (out: bool) = change value in
      let value:bool = tmp0 in
      value, out <: (bool & bool)
  in
  value, hax_temp_output <: (bool & bool)
