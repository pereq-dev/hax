module New_tests.Legacy__side_effects__lib
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

/// Helper function
let add3 (x y z: u32) : u32 =
  Core_models.Num.impl_u32__wrapping_add (Core_models.Num.impl_u32__wrapping_add x y <: u32) z

/// Exercise local mutation with control flow and loops
let local_mutation (x: u32) : u32 =
  let y:u32 = mk_u32 0 in
  let x:u32 = Core_models.Num.impl_u32__wrapping_add x (mk_u32 1) in
  if x >. mk_u32 3
  then
    let x:u32 = Core_models.Num.impl_u32__wrapping_sub x (mk_u32 3) in
    let y:u32 = x /! mk_u32 2 in
    let y:u32 = Core_models.Num.impl_u32__wrapping_add y (mk_u32 2) in
    let y:u32 =
      Core_models.Iter.Traits.Iterator.f_fold (Core_models.Iter.Traits.Collect.f_into_iter #(Core_models.Ops.Range.t_Range
              u32)
            #FStar.Tactics.Typeclasses.solve
            ({ Core_models.Ops.Range.f_start = mk_u32 0; Core_models.Ops.Range.f_end = mk_u32 10 }
              <:
              Core_models.Ops.Range.t_Range u32)
          <:
          Core_models.Ops.Range.t_Range u32)
        y
        (fun y i ->
            let y:u32 = y in
            let i:u32 = i in
            Core_models.Num.impl_u32__wrapping_add x i <: u32)
    in
    Core_models.Num.impl_u32__wrapping_add x y
  else
    let ((x: u32), (y: u32)), (hoist7: u32) =
      match x <: u32 with
      | Rust_primitives.Integers.MkInt 12 ->
        let y:u32 = Core_models.Num.impl_u32__wrapping_add x y in
        (x, y <: (u32 & u32)), mk_u32 3 <: ((u32 & u32) & u32)
      | Rust_primitives.Integers.MkInt 13 ->
        let x:u32 = Core_models.Num.impl_u32__wrapping_add x (mk_u32 1) in
        (x, y <: (u32 & u32)),
        add3 x (Core_models.Num.impl_u32__wrapping_add (mk_u32 123) x <: u32) x
        <:
        ((u32 & u32) & u32)
      | _ -> (x, y <: (u32 & u32)), mk_u32 0 <: ((u32 & u32) & u32)
    in
    let x:u32 = hoist7 in
    Core_models.Num.impl_u32__wrapping_add x y

/// Exercise early returns with control flow and loops
let early_returns (x: u32) : u32 =
  if x >. mk_u32 3
  then mk_u32 0
  else
    if x >. mk_u32 30
    then
      match true <: bool with
      | true -> mk_u32 34
      | _ ->
        let (x: u32), (hoist11: u32) = x, mk_u32 3 <: (u32 & u32) in
        Core_models.Num.impl_u32__wrapping_add (Core_models.Num.impl_u32__wrapping_add (mk_u32 123)
              hoist11
            <:
            u32)
          x
    else
      let x:u32 = x +! mk_u32 9 in
      let (x: u32), (hoist11: u32) = x, x +! mk_u32 1 <: (u32 & u32) in
      Core_models.Num.impl_u32__wrapping_add (Core_models.Num.impl_u32__wrapping_add (mk_u32 123)
            hoist11
          <:
          u32)
        x

let simplifiable_return (c1 c2 c3: bool) : i32 =
  let x:i32 = mk_i32 0 in
  if c1
  then
    if c2
    then
      let x:i32 = x +! mk_i32 10 in
      if c3 then mk_i32 1 else x +! mk_i32 1
    else x +! mk_i32 1
  else x

let simplifiable_question_mark (c: bool) (x: Core_models.Option.t_Option i32)
    : Core_models.Option.t_Option i32 =
  if c
  then
    match x <: Core_models.Option.t_Option i32 with
    | Core_models.Option.Option_Some hoist16 ->
      let a:i32 = hoist16 +! mk_i32 10 in
      let b:i32 = mk_i32 20 in
      Core_models.Option.Option_Some (a +! b) <: Core_models.Option.t_Option i32
    | Core_models.Option.Option_None  ->
      Core_models.Option.Option_None <: Core_models.Option.t_Option i32
  else
    let a:i32 = mk_i32 0 in
    let b:i32 = mk_i32 20 in
    Core_models.Option.Option_Some (a +! b) <: Core_models.Option.t_Option i32

/// Question mark without error coercion
let direct_result_question_mark (y: Core_models.Result.t_Result Prims.unit u32)
    : Core_models.Result.t_Result i8 u32 =
  match y <: Core_models.Result.t_Result Prims.unit u32 with
  | Core_models.Result.Result_Ok _ ->
    Core_models.Result.Result_Ok (mk_i8 0) <: Core_models.Result.t_Result i8 u32
  | Core_models.Result.Result_Err err ->
    Core_models.Result.Result_Err err <: Core_models.Result.t_Result i8 u32

/// Question mark with an error coercion
let direct_result_question_mark_coercion (y: Core_models.Result.t_Result i8 u16)
    : Core_models.Result.t_Result i8 u32 =
  match y <: Core_models.Result.t_Result i8 u16 with
  | Core_models.Result.Result_Ok hoist17 ->
    Core_models.Result.Result_Ok hoist17 <: Core_models.Result.t_Result i8 u32
  | Core_models.Result.Result_Err err ->
    Core_models.Result.Result_Err
    (Core_models.Convert.f_from #u32 #u16 #FStar.Tactics.Typeclasses.solve err)
    <:
    Core_models.Result.t_Result i8 u32

/// Test question mark on `Option`s with some control flow
let options (x y: Core_models.Option.t_Option u8) (z: Core_models.Option.t_Option u64)
    : Core_models.Option.t_Option u8 =
  match x <: Core_models.Option.t_Option u8 with
  | Core_models.Option.Option_Some hoist21 ->
    if hoist21 >. mk_u8 10
    then
      match x <: Core_models.Option.t_Option u8 with
      | Core_models.Option.Option_Some hoist23 ->
        (match
            Core_models.Option.Option_Some (Core_models.Num.impl_u8__wrapping_add hoist23 (mk_u8 3))
            <:
            Core_models.Option.t_Option u8
          with
          | Core_models.Option.Option_Some hoist29 ->
            (match hoist29 <: u8 with
              | Rust_primitives.Integers.MkInt 3 ->
                (match Core_models.Option.Option_None <: Core_models.Option.t_Option u8 with
                  | Core_models.Option.Option_Some some ->
                    let v:u8 = some in
                    (match x <: Core_models.Option.t_Option u8 with
                      | Core_models.Option.Option_Some hoist30 ->
                        (match y <: Core_models.Option.t_Option u8 with
                          | Core_models.Option.Option_Some hoist31 ->
                            Core_models.Option.Option_Some
                            (Core_models.Num.impl_u8__wrapping_add (Core_models.Num.impl_u8__wrapping_add
                                    v
                                    hoist30
                                  <:
                                  u8)
                                hoist31)
                            <:
                            Core_models.Option.t_Option u8
                          | Core_models.Option.Option_None  ->
                            Core_models.Option.Option_None <: Core_models.Option.t_Option u8)
                      | Core_models.Option.Option_None  ->
                        Core_models.Option.Option_None <: Core_models.Option.t_Option u8)
                  | Core_models.Option.Option_None  ->
                    Core_models.Option.Option_None <: Core_models.Option.t_Option u8)
              | Rust_primitives.Integers.MkInt 4 ->
                (match z <: Core_models.Option.t_Option u64 with
                  | Core_models.Option.Option_Some hoist18 ->
                    let v:u8 =
                      mk_u8 4 +! (if hoist18 >. mk_u64 4 <: bool then mk_u8 0 else mk_u8 3)
                    in
                    (match x <: Core_models.Option.t_Option u8 with
                      | Core_models.Option.Option_Some hoist30 ->
                        (match y <: Core_models.Option.t_Option u8 with
                          | Core_models.Option.Option_Some hoist31 ->
                            Core_models.Option.Option_Some
                            (Core_models.Num.impl_u8__wrapping_add (Core_models.Num.impl_u8__wrapping_add
                                    v
                                    hoist30
                                  <:
                                  u8)
                                hoist31)
                            <:
                            Core_models.Option.t_Option u8
                          | Core_models.Option.Option_None  ->
                            Core_models.Option.Option_None <: Core_models.Option.t_Option u8)
                      | Core_models.Option.Option_None  ->
                        Core_models.Option.Option_None <: Core_models.Option.t_Option u8)
                  | Core_models.Option.Option_None  ->
                    Core_models.Option.Option_None <: Core_models.Option.t_Option u8)
              | _ ->
                let v:u8 = mk_u8 12 in
                match x <: Core_models.Option.t_Option u8 with
                | Core_models.Option.Option_Some hoist30 ->
                  (match y <: Core_models.Option.t_Option u8 with
                    | Core_models.Option.Option_Some hoist31 ->
                      Core_models.Option.Option_Some
                      (Core_models.Num.impl_u8__wrapping_add (Core_models.Num.impl_u8__wrapping_add v
                              hoist30
                            <:
                            u8)
                          hoist31)
                      <:
                      Core_models.Option.t_Option u8
                    | Core_models.Option.Option_None  ->
                      Core_models.Option.Option_None <: Core_models.Option.t_Option u8)
                | Core_models.Option.Option_None  ->
                  Core_models.Option.Option_None <: Core_models.Option.t_Option u8)
          | Core_models.Option.Option_None  ->
            Core_models.Option.Option_None <: Core_models.Option.t_Option u8)
      | Core_models.Option.Option_None  ->
        Core_models.Option.Option_None <: Core_models.Option.t_Option u8
    else
      (match x <: Core_models.Option.t_Option u8 with
        | Core_models.Option.Option_Some hoist26 ->
          (match y <: Core_models.Option.t_Option u8 with
            | Core_models.Option.Option_Some hoist25 ->
              (match
                  Core_models.Option.Option_Some
                  (Core_models.Num.impl_u8__wrapping_add hoist26 hoist25)
                  <:
                  Core_models.Option.t_Option u8
                with
                | Core_models.Option.Option_Some hoist29 ->
                  (match hoist29 <: u8 with
                    | Rust_primitives.Integers.MkInt 3 ->
                      (match Core_models.Option.Option_None <: Core_models.Option.t_Option u8 with
                        | Core_models.Option.Option_Some some ->
                          let v:u8 = some in
                          (match x <: Core_models.Option.t_Option u8 with
                            | Core_models.Option.Option_Some hoist30 ->
                              (match y <: Core_models.Option.t_Option u8 with
                                | Core_models.Option.Option_Some hoist31 ->
                                  Core_models.Option.Option_Some
                                  (Core_models.Num.impl_u8__wrapping_add (Core_models.Num.impl_u8__wrapping_add
                                          v
                                          hoist30
                                        <:
                                        u8)
                                      hoist31)
                                  <:
                                  Core_models.Option.t_Option u8
                                | Core_models.Option.Option_None  ->
                                  Core_models.Option.Option_None <: Core_models.Option.t_Option u8)
                            | Core_models.Option.Option_None  ->
                              Core_models.Option.Option_None <: Core_models.Option.t_Option u8)
                        | Core_models.Option.Option_None  ->
                          Core_models.Option.Option_None <: Core_models.Option.t_Option u8)
                    | Rust_primitives.Integers.MkInt 4 ->
                      (match z <: Core_models.Option.t_Option u64 with
                        | Core_models.Option.Option_Some hoist18 ->
                          let v:u8 =
                            mk_u8 4 +! (if hoist18 >. mk_u64 4 <: bool then mk_u8 0 else mk_u8 3)
                          in
                          (match x <: Core_models.Option.t_Option u8 with
                            | Core_models.Option.Option_Some hoist30 ->
                              (match y <: Core_models.Option.t_Option u8 with
                                | Core_models.Option.Option_Some hoist31 ->
                                  Core_models.Option.Option_Some
                                  (Core_models.Num.impl_u8__wrapping_add (Core_models.Num.impl_u8__wrapping_add
                                          v
                                          hoist30
                                        <:
                                        u8)
                                      hoist31)
                                  <:
                                  Core_models.Option.t_Option u8
                                | Core_models.Option.Option_None  ->
                                  Core_models.Option.Option_None <: Core_models.Option.t_Option u8)
                            | Core_models.Option.Option_None  ->
                              Core_models.Option.Option_None <: Core_models.Option.t_Option u8)
                        | Core_models.Option.Option_None  ->
                          Core_models.Option.Option_None <: Core_models.Option.t_Option u8)
                    | _ ->
                      let v:u8 = mk_u8 12 in
                      match x <: Core_models.Option.t_Option u8 with
                      | Core_models.Option.Option_Some hoist30 ->
                        (match y <: Core_models.Option.t_Option u8 with
                          | Core_models.Option.Option_Some hoist31 ->
                            Core_models.Option.Option_Some
                            (Core_models.Num.impl_u8__wrapping_add (Core_models.Num.impl_u8__wrapping_add
                                    v
                                    hoist30
                                  <:
                                  u8)
                                hoist31)
                            <:
                            Core_models.Option.t_Option u8
                          | Core_models.Option.Option_None  ->
                            Core_models.Option.Option_None <: Core_models.Option.t_Option u8)
                      | Core_models.Option.Option_None  ->
                        Core_models.Option.Option_None <: Core_models.Option.t_Option u8)
                | Core_models.Option.Option_None  ->
                  Core_models.Option.Option_None <: Core_models.Option.t_Option u8)
            | Core_models.Option.Option_None  ->
              Core_models.Option.Option_None <: Core_models.Option.t_Option u8)
        | Core_models.Option.Option_None  ->
          Core_models.Option.Option_None <: Core_models.Option.t_Option u8)
  | Core_models.Option.Option_None  ->
    Core_models.Option.Option_None <: Core_models.Option.t_Option u8

/// Test question mark on `Result`s with local mutation
let question_mark (x: u32) : Core_models.Result.t_Result u32 u32 =
  if x >. mk_u32 40
  then
    let y:u32 = mk_u32 0 in
    let x:u32 = Core_models.Num.impl_u32__wrapping_add x (mk_u32 3) in
    let y:u32 = Core_models.Num.impl_u32__wrapping_add x y in
    let x:u32 = Core_models.Num.impl_u32__wrapping_add x y in
    if x >. mk_u32 90
    then
      match
        Core_models.Result.Result_Err (mk_u8 12) <: Core_models.Result.t_Result Prims.unit u8
      with
      | Core_models.Result.Result_Ok ok ->
        Core_models.Result.Result_Ok (Core_models.Num.impl_u32__wrapping_add (mk_u32 3) x)
        <:
        Core_models.Result.t_Result u32 u32
      | Core_models.Result.Result_Err err ->
        Core_models.Result.Result_Err
        (Core_models.Convert.f_from #u32 #u8 #FStar.Tactics.Typeclasses.solve err)
        <:
        Core_models.Result.t_Result u32 u32
    else
      Core_models.Result.Result_Ok (Core_models.Num.impl_u32__wrapping_add (mk_u32 3) x)
      <:
      Core_models.Result.t_Result u32 u32
  else
    Core_models.Result.Result_Ok (Core_models.Num.impl_u32__wrapping_add (mk_u32 3) x)
    <:
    Core_models.Result.t_Result u32 u32

type t_A = | A : t_A

type t_B = | B : t_B

/// Combine `?` and early return
let monad_lifting (x: u8) : Core_models.Result.t_Result t_A t_B =
  if x >. mk_u8 123
  then
    match Core_models.Result.Result_Err (B <: t_B) <: Core_models.Result.t_Result t_A t_B with
    | Core_models.Result.Result_Ok hoist35 ->
      Core_models.Result.Result_Ok hoist35 <: Core_models.Result.t_Result t_A t_B
    | Core_models.Result.Result_Err err ->
      Core_models.Result.Result_Err err <: Core_models.Result.t_Result t_A t_B
  else Core_models.Result.Result_Ok (A <: t_A) <: Core_models.Result.t_Result t_A t_B

type t_Bar = {
  f_a:bool;
  f_b:(t_Array (bool & bool) (mk_usize 6) & bool)
}

type t_Foo = {
  f_x:bool;
  f_y:(bool & Alloc.Vec.t_Vec t_Bar Alloc.Alloc.t_Global);
  f_z:t_Array t_Bar (mk_usize 6);
  f_bar:t_Bar
}

/// Test assignation on non-trivial places
/// @fail(extraction): coq(HAX0002, HAX0002), ssprove(HAX0001)
let assign_non_trivial_lhs (foo: t_Foo) : t_Foo =
  let foo:t_Foo = { foo with f_x = true } <: t_Foo in
  let foo:t_Foo = { foo with f_bar = { foo.f_bar with f_a = true } <: t_Bar } <: t_Foo in
  let foo:t_Foo =
    {
      foo with
      f_bar
      =
      {
        foo.f_bar with
        f_b
        =
        {
          foo.f_bar.f_b with
          _1
          =
          Rust_primitives.Hax.Monomorphized_update_at.update_at_usize foo.f_bar.f_b._1
            (mk_usize 3)
            ({ (foo.f_bar.f_b._1.[ mk_usize 3 ] <: (bool & bool)) with _2 = true } <: (bool & bool))
        }
        <:
        (t_Array (bool & bool) (mk_usize 6) & bool)
      }
      <:
      t_Bar
    }
    <:
    t_Foo
  in
  let foo:t_Foo =
    {
      foo with
      f_z
      =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_usize foo.f_z
        (mk_usize 3)
        ({ (foo.f_z.[ mk_usize 3 ] <: t_Bar) with f_a = true } <: t_Bar)
    }
    <:
    t_Foo
  in
  let foo:t_Foo =
    {
      foo with
      f_y
      =
      {
        foo.f_y with
        _2
        =
        Alloc.Slice.impl__to_vec (Rust_primitives.Hax.Monomorphized_update_at.update_at_usize (Alloc.Vec.impl_1__as_slice
                  foo.f_y._2
                <:
                t_Slice t_Bar)
              (mk_usize 3)
              ({
                  (foo.f_y._2.[ mk_usize 3 ] <: t_Bar) with
                  f_b
                  =
                  {
                    (foo.f_y._2.[ mk_usize 3 ] <: t_Bar).f_b with
                    _1
                    =
                    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize (foo.f_y._2.[ mk_usize
                          3 ]
                        <:
                        t_Bar)
                        .f_b
                        ._1
                      (mk_usize 5)
                      ({
                          ((foo.f_y._2.[ mk_usize 3 ] <: t_Bar).f_b._1.[ mk_usize 5 ]
                            <:
                            (bool & bool)) with
                          _1 = true
                        }
                        <:
                        (bool & bool))
                    <:
                    t_Array (bool & bool) (mk_usize 6)
                  }
                  <:
                  (t_Array (bool & bool) (mk_usize 6) & bool)
                }
                <:
                t_Bar)
            <:
            t_Slice t_Bar)
      }
      <:
      (bool & Alloc.Vec.t_Vec t_Bar Alloc.Alloc.t_Global)
    }
    <:
    t_Foo
  in
  foo
