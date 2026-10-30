
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


namespace new_tests.legacy__side_effects__lib

--  Helper function
@[spec]
def add3 (x : u32) (y : u32) (z : u32) : RustM u32 := do
  (core_models.num.Impl_8.wrapping_add
    (← (core_models.num.Impl_8.wrapping_add x y))
    z)

--  Exercise local mutation with control flow and loops
@[spec]
def local_mutation (x : u32) : RustM u32 := do
  let y : u32 := (0 : u32);
  let x : u32 ← (core_models.num.Impl_8.wrapping_add x (1 : u32));
  if (← (x >? (3 : u32))) then do
    let x : u32 ← (core_models.num.Impl_8.wrapping_sub x (3 : u32));
    let y : u32 ← (x /? (2 : u32));
    let y : u32 ← (core_models.num.Impl_8.wrapping_add y (2 : u32));
    let y : u32 ←
      (core_models.iter.traits.iterator.Iterator.fold
        (← (core_models.iter.traits.collect.IntoIterator.into_iter
          (core_models.ops.range.Range u32)
          (core_models.ops.range.Range.mk
            (start := (0 : u32))
            (_end := (10 : u32)))))
        y
        (fun y i =>
          (do (core_models.num.Impl_8.wrapping_add x i) : RustM u32)));
    (core_models.num.Impl_8.wrapping_add x y)
  else do
    let ⟨⟨x, y⟩, hoist7⟩ ←
      match x with
        | 12 => do
          let y : u32 ← (core_models.num.Impl_8.wrapping_add x y);
          (pure (rust_primitives.hax.Tuple2.mk
            (rust_primitives.hax.Tuple2.mk x y)
            (3 : u32)))
        | 13 => do
          let x : u32 ← (core_models.num.Impl_8.wrapping_add x (1 : u32));
          (pure (rust_primitives.hax.Tuple2.mk
            (rust_primitives.hax.Tuple2.mk x y)
            (← (add3
              x
              (← (core_models.num.Impl_8.wrapping_add (123 : u32) x))
              x))))
        | _ => do
          (pure (rust_primitives.hax.Tuple2.mk
            (rust_primitives.hax.Tuple2.mk x y)
            (0 : u32)));
    let x : u32 := hoist7;
    (core_models.num.Impl_8.wrapping_add x y)

--  Exercise early returns with control flow and loops
@[spec]
def early_returns (x : u32) : RustM u32 := do
  if (← (x >? (3 : u32))) then do
    (pure (0 : u32))
  else do
    if (← (x >? (30 : u32))) then do
      match true with
        | true => do (pure (34 : u32))
        | _ => do
          let ⟨x, hoist11⟩ := (rust_primitives.hax.Tuple2.mk x (3 : u32));
          (core_models.num.Impl_8.wrapping_add
            (← (core_models.num.Impl_8.wrapping_add (123 : u32) hoist11))
            x)
    else do
      let x : u32 ← (x +? (9 : u32));
      let ⟨x, hoist11⟩ :=
        (rust_primitives.hax.Tuple2.mk x (← (x +? (1 : u32))));
      (core_models.num.Impl_8.wrapping_add
        (← (core_models.num.Impl_8.wrapping_add (123 : u32) hoist11))
        x)

@[spec]
def simplifiable_return (c1 : Bool) (c2 : Bool) (c3 : Bool) : RustM i32 := do
  let x : i32 := (0 : i32);
  if c1 then do
    if c2 then do
      let x : i32 ← (x +? (10 : i32));
      if c3 then do (pure (1 : i32)) else do (x +? (1 : i32))
    else do
      (x +? (1 : i32))
  else do
    (pure x)

@[spec]
def simplifiable_question_mark
    (c : Bool)
    (x : (core_models.option.Option i32)) :
    RustM (core_models.option.Option i32) := do
  if c then do
    match x with
      | (core_models.option.Option.Some  hoist16) => do
        let a : i32 ← (hoist16 +? (10 : i32));
        let b : i32 := (20 : i32);
        (pure (core_models.option.Option.Some (← (a +? b))))
      | (core_models.option.Option.None ) => do
        (pure core_models.option.Option.None)
  else do
    let a : i32 := (0 : i32);
    let b : i32 := (20 : i32);
    (pure (core_models.option.Option.Some (← (a +? b))))

--  Question mark without error coercion
@[spec]
def direct_result_question_mark
    (y : (core_models.result.Result rust_primitives.hax.Tuple0 u32)) :
    RustM (core_models.result.Result i8 u32) := do
  match y with
    | (core_models.result.Result.Ok  _) => do
      (pure (core_models.result.Result.Ok (0 : i8)))
    | (core_models.result.Result.Err  err) => do
      (pure (core_models.result.Result.Err err))

--  Question mark with an error coercion
@[spec]
def direct_result_question_mark_coercion
    (y : (core_models.result.Result i8 u16)) :
    RustM (core_models.result.Result i8 u32) := do
  match y with
    | (core_models.result.Result.Ok  hoist17) => do
      (pure (core_models.result.Result.Ok hoist17))
    | (core_models.result.Result.Err  err) => do
      (pure (core_models.result.Result.Err
        (← (core_models.convert.From._from u32 u16 err))))

--  Test question mark on `Option`s with some control flow
@[spec]
def options
    (x : (core_models.option.Option u8))
    (y : (core_models.option.Option u8))
    (z : (core_models.option.Option u64)) :
    RustM (core_models.option.Option u8) := do
  match x with
    | (core_models.option.Option.Some  hoist21) => do
      if (← (hoist21 >? (10 : u8))) then do
        match x with
          | (core_models.option.Option.Some  hoist23) => do
            match
              (core_models.option.Option.Some
                (← (core_models.num.Impl_6.wrapping_add hoist23 (3 : u8))))
            with
              | (core_models.option.Option.Some  hoist29) => do
                match hoist29 with
                  | 3 => do
                    match core_models.option.Option.None with
                      | (core_models.option.Option.Some  some) => do
                        let v : u8 := some;
                        match x with
                          | (core_models.option.Option.Some  hoist30) => do
                            match y with
                              | (core_models.option.Option.Some  hoist31) => do
                                (pure (core_models.option.Option.Some
                                  (← (core_models.num.Impl_6.wrapping_add
                                    (← (core_models.num.Impl_6.wrapping_add
                                      v
                                      hoist30))
                                    hoist31))))
                              | (core_models.option.Option.None ) => do
                                (pure core_models.option.Option.None)
                          | (core_models.option.Option.None ) => do
                            (pure core_models.option.Option.None)
                      | (core_models.option.Option.None ) => do
                        (pure core_models.option.Option.None)
                  | 4 => do
                    match z with
                      | (core_models.option.Option.Some  hoist18) => do
                        let v : u8 ←
                          ((4 : u8)
                            +? (← if (← (hoist18 >? (4 : u64))) then do
                              (pure (0 : u8))
                            else do
                              (pure (3 : u8))));
                        match x with
                          | (core_models.option.Option.Some  hoist30) => do
                            match y with
                              | (core_models.option.Option.Some  hoist31) => do
                                (pure (core_models.option.Option.Some
                                  (← (core_models.num.Impl_6.wrapping_add
                                    (← (core_models.num.Impl_6.wrapping_add
                                      v
                                      hoist30))
                                    hoist31))))
                              | (core_models.option.Option.None ) => do
                                (pure core_models.option.Option.None)
                          | (core_models.option.Option.None ) => do
                            (pure core_models.option.Option.None)
                      | (core_models.option.Option.None ) => do
                        (pure core_models.option.Option.None)
                  | _ => do
                    let v : u8 := (12 : u8);
                    match x with
                      | (core_models.option.Option.Some  hoist30) => do
                        match y with
                          | (core_models.option.Option.Some  hoist31) => do
                            (pure (core_models.option.Option.Some
                              (← (core_models.num.Impl_6.wrapping_add
                                (← (core_models.num.Impl_6.wrapping_add
                                  v
                                  hoist30))
                                hoist31))))
                          | (core_models.option.Option.None ) => do
                            (pure core_models.option.Option.None)
                      | (core_models.option.Option.None ) => do
                        (pure core_models.option.Option.None)
              | (core_models.option.Option.None ) => do
                (pure core_models.option.Option.None)
          | (core_models.option.Option.None ) => do
            (pure core_models.option.Option.None)
      else do
        match x with
          | (core_models.option.Option.Some  hoist26) => do
            match y with
              | (core_models.option.Option.Some  hoist25) => do
                match
                  (core_models.option.Option.Some
                    (← (core_models.num.Impl_6.wrapping_add hoist26 hoist25)))
                with
                  | (core_models.option.Option.Some  hoist29) => do
                    match hoist29 with
                      | 3 => do
                        match core_models.option.Option.None with
                          | (core_models.option.Option.Some  some) => do
                            let v : u8 := some;
                            match x with
                              | (core_models.option.Option.Some  hoist30) => do
                                match y with
                                  | (core_models.option.Option.Some  hoist31) =>
                                    do
                                    (pure (core_models.option.Option.Some
                                      (← (core_models.num.Impl_6.wrapping_add
                                        (← (core_models.num.Impl_6.wrapping_add
                                          v
                                          hoist30))
                                        hoist31))))
                                  | (core_models.option.Option.None ) => do
                                    (pure core_models.option.Option.None)
                              | (core_models.option.Option.None ) => do
                                (pure core_models.option.Option.None)
                          | (core_models.option.Option.None ) => do
                            (pure core_models.option.Option.None)
                      | 4 => do
                        match z with
                          | (core_models.option.Option.Some  hoist18) => do
                            let v : u8 ←
                              ((4 : u8)
                                +? (← if (← (hoist18 >? (4 : u64))) then do
                                  (pure (0 : u8))
                                else do
                                  (pure (3 : u8))));
                            match x with
                              | (core_models.option.Option.Some  hoist30) => do
                                match y with
                                  | (core_models.option.Option.Some  hoist31) =>
                                    do
                                    (pure (core_models.option.Option.Some
                                      (← (core_models.num.Impl_6.wrapping_add
                                        (← (core_models.num.Impl_6.wrapping_add
                                          v
                                          hoist30))
                                        hoist31))))
                                  | (core_models.option.Option.None ) => do
                                    (pure core_models.option.Option.None)
                              | (core_models.option.Option.None ) => do
                                (pure core_models.option.Option.None)
                          | (core_models.option.Option.None ) => do
                            (pure core_models.option.Option.None)
                      | _ => do
                        let v : u8 := (12 : u8);
                        match x with
                          | (core_models.option.Option.Some  hoist30) => do
                            match y with
                              | (core_models.option.Option.Some  hoist31) => do
                                (pure (core_models.option.Option.Some
                                  (← (core_models.num.Impl_6.wrapping_add
                                    (← (core_models.num.Impl_6.wrapping_add
                                      v
                                      hoist30))
                                    hoist31))))
                              | (core_models.option.Option.None ) => do
                                (pure core_models.option.Option.None)
                          | (core_models.option.Option.None ) => do
                            (pure core_models.option.Option.None)
                  | (core_models.option.Option.None ) => do
                    (pure core_models.option.Option.None)
              | (core_models.option.Option.None ) => do
                (pure core_models.option.Option.None)
          | (core_models.option.Option.None ) => do
            (pure core_models.option.Option.None)
    | (core_models.option.Option.None ) => do
      (pure core_models.option.Option.None)

--  Test question mark on `Result`s with local mutation
@[spec]
def question_mark (x : u32) : RustM (core_models.result.Result u32 u32) := do
  if (← (x >? (40 : u32))) then do
    let y : u32 := (0 : u32);
    let x : u32 ← (core_models.num.Impl_8.wrapping_add x (3 : u32));
    let y : u32 ← (core_models.num.Impl_8.wrapping_add x y);
    let x : u32 ← (core_models.num.Impl_8.wrapping_add x y);
    if (← (x >? (90 : u32))) then do
      match (core_models.result.Result.Err (12 : u8)) with
        | (core_models.result.Result.Ok  ok) => do
          (pure (core_models.result.Result.Ok
            (← (core_models.num.Impl_8.wrapping_add (3 : u32) x))))
        | (core_models.result.Result.Err  err) => do
          (pure (core_models.result.Result.Err
            (← (core_models.convert.From._from u32 u8 err))))
    else do
      (pure (core_models.result.Result.Ok
        (← (core_models.num.Impl_8.wrapping_add (3 : u32) x))))
  else do
    (pure (core_models.result.Result.Ok
      (← (core_models.num.Impl_8.wrapping_add (3 : u32) x))))

structure A where
  -- no fields

structure B where
  -- no fields

--  Combine `?` and early return
@[spec]
def monad_lifting (x : u8) : RustM (core_models.result.Result A B) := do
  if (← (x >? (123 : u8))) then do
    match (core_models.result.Result.Err B.mk) with
      | (core_models.result.Result.Ok  hoist35) => do
        (pure (core_models.result.Result.Ok hoist35))
      | (core_models.result.Result.Err  err) => do
        (pure (core_models.result.Result.Err err))
  else do
    (pure (core_models.result.Result.Ok A.mk))

structure Bar where
  a : Bool
  b : (rust_primitives.hax.Tuple2
      (RustArray (rust_primitives.hax.Tuple2 Bool Bool) 6)
      Bool)

structure Foo where
  x : Bool
  y : (rust_primitives.hax.Tuple2 Bool (alloc.vec.Vec Bar alloc.alloc.Global))
  z : (RustArray Bar 6)
  bar : Bar

--  Test assignation on non-trivial places
--  @fail(extraction): coq(HAX0002, HAX0002), ssprove(HAX0001)
@[spec]
def assign_non_trivial_lhs (foo : Foo) : RustM Foo := do
  let foo : Foo := {foo with x := true};
  let foo : Foo := {foo with bar := {(Foo.bar foo) with a := true}};
  let foo : Foo :=
    {foo
    with bar := {(Foo.bar foo)
    with b := {(Bar.b (Foo.bar foo))
    with _0 := (← (rust_primitives.hax.monomorphized_update_at.update_at_usize
      (rust_primitives.hax.Tuple2._0 (Bar.b (Foo.bar foo)))
      (3 : usize)
      {(← (rust_primitives.hax.Tuple2._0 (Bar.b (Foo.bar foo)))[(3 : usize)]_?)
      with _1 := true}))}}};
  let foo : Foo :=
    {foo
    with z := (← (rust_primitives.hax.monomorphized_update_at.update_at_usize
      (Foo.z foo)
      (3 : usize)
      {(← (Foo.z foo)[(3 : usize)]_?) with a := true}))};
  let foo : Foo :=
    {foo
    with y := {(Foo.y foo)
    with _1 := (← (alloc.slice.Impl.to_vec
      (← (rust_primitives.hax.monomorphized_update_at.update_at_usize
        (← (alloc.vec.Impl_1.as_slice
          (rust_primitives.hax.Tuple2._1 (Foo.y foo))))
        (3 : usize)
        {(← (rust_primitives.hax.Tuple2._1 (Foo.y foo))[(3 : usize)]_?)
        with b := {(Bar.b
          (← (rust_primitives.hax.Tuple2._1 (Foo.y foo))[(3 : usize)]_?))
        with _0 := (←
        (rust_primitives.hax.monomorphized_update_at.update_at_usize
          (rust_primitives.hax.Tuple2._0
            (Bar.b
              (← (rust_primitives.hax.Tuple2._1 (Foo.y foo))[(3 : usize)]_?)))
          (5 : usize)
          {(← (rust_primitives.hax.Tuple2._0
              (Bar.b
                (← (rust_primitives.hax.Tuple2._1 (Foo.y foo))[
                  (3 : usize)
                  ]_?)))[
            (5 : usize)
            ]_?)
          with _0 := true}))}}))))}};
  (pure foo)

end new_tests.legacy__side_effects__lib


namespace new_tests.legacy__side_effects__lib.issue_1083

class MyFrom.AssociatedTypes (Self : Type) (T : Type) where

class MyFrom (Self : Type) (T : Type)
  [associatedTypes : outParam (MyFrom.AssociatedTypes (Self : Type) (T : Type))]
  where
  my_from (Self) (T) : (T -> RustM Self)

@[spec]
def Impl.my_from_hoisted (x : u8) : RustM u16 := do
  (rust_primitives.hax.cast_op x : RustM u16)

@[reducible] instance Impl.AssociatedTypes : MyFrom.AssociatedTypes u16 u8 where

instance Impl : MyFrom u16 u8 where
  my_from := (Impl.my_from_hoisted)

@[spec]
def f (x : u8) : RustM (core_models.result.Result u16 u16) := do
  match
    (← (core_models.ops.try_trait.Try.branch
      (core_models.result.Result rust_primitives.hax.Never u8)
      (core_models.result.Result.Err (1 : u8))))
  with
    | (core_models.ops.control_flow.ControlFlow.Break  residual) => do
      (core_models.ops.try_trait.FromResidual.from_residual
        (core_models.result.Result u16 u16)
        (core_models.result.Result core_models.convert.Infallible u8) residual)
    | (core_models.ops.control_flow.ControlFlow.Continue  val) => do
      let _ := val;
      (pure (core_models.result.Result.Ok (← (MyFrom.my_from u16 u8 x))))

end new_tests.legacy__side_effects__lib.issue_1083


namespace new_tests.legacy__side_effects__lib.issue_1089

@[spec]
def test
    (x : (core_models.option.Option i32))
    (y : (core_models.option.Option i32)) :
    RustM (core_models.option.Option i32) := do
  match
    (← (core_models.option.Impl.map
      i32
      (core_models.option.Option i32)
      (i32 -> RustM (core_models.option.Option i32))
      x
      (fun i =>
        (do
        match y with
          | (core_models.option.Option.Some  hoist38) => do
            (pure (core_models.option.Option.Some (← (i +? hoist38))))
          | (core_models.option.Option.None ) => do
            (pure core_models.option.Option.None) :
        RustM (core_models.option.Option i32)))))
  with
    | (core_models.option.Option.Some  some) => do (pure some)
    | (core_models.option.Option.None ) => do
      (pure core_models.option.Option.None)

end new_tests.legacy__side_effects__lib.issue_1089


namespace new_tests.legacy__side_effects__lib.nested_return

@[spec]
def other_fun (rng : i8) :
    RustM
    (rust_primitives.hax.Tuple2
      i8
      (core_models.result.Result
        rust_primitives.hax.Tuple0
        rust_primitives.hax.Tuple0))
    := do
  let
    hax_temp_output : (core_models.result.Result
      rust_primitives.hax.Tuple0
      rust_primitives.hax.Tuple0) :=
    (core_models.result.Result.Ok rust_primitives.hax.Tuple0.mk);
  (pure (rust_primitives.hax.Tuple2.mk rng hax_temp_output))

@[spec]
def fun (rng : i8) :
    RustM
    (rust_primitives.hax.Tuple2
      i8
      (core_models.result.Result
        rust_primitives.hax.Tuple0
        rust_primitives.hax.Tuple0))
    := do
  let ⟨tmp0, out⟩ ← (other_fun rng);
  let rng : i8 := tmp0;
  match out with
    | (core_models.result.Result.Ok  hoist41) => do
      (pure (rust_primitives.hax.Tuple2.mk
        rng
        (core_models.result.Result.Ok hoist41)))
    | (core_models.result.Result.Err  err) => do
      (pure (rust_primitives.hax.Tuple2.mk
        rng
        (core_models.result.Result.Err err)))

end new_tests.legacy__side_effects__lib.nested_return


namespace new_tests.legacy__side_effects__lib.issue_1300

@[spec]
def fun (_ : rust_primitives.hax.Tuple0) :
    RustM (core_models.result.Result rust_primitives.hax.Tuple0 u8) := do
  match
    (← (core_models.iter.traits.iterator.Iterator.collect
      (core_models.iter.adapters.map.Map
        (core_models.slice.iter.Iter u8)
        (u8 ->
        RustM (core_models.result.Result
          (rust_primitives.hax.Tuple2 u8 (RustArray u8 32))
          u8)))
      (core_models.result.Result
        (alloc.vec.Vec
          (rust_primitives.hax.Tuple2 u8 (RustArray u8 32))
          alloc.alloc.Global)
        u8)
      (← (core_models.iter.traits.iterator.Iterator.map
        (core_models.slice.iter.Iter u8)
        (core_models.result.Result
          (rust_primitives.hax.Tuple2 u8 (RustArray u8 32))
          u8)
        (u8 ->
        RustM (core_models.result.Result
          (rust_primitives.hax.Tuple2 u8 (RustArray u8 32))
          u8))
        (← (core_models.slice.Impl.iter u8
          (← (rust_primitives.unsize
            (← (rust_primitives.hax.repeat (0 : u8) (5 : usize)))))))
        (fun prev =>
          (do
          match
            (core_models.result.Result.Ok
              (← (rust_primitives.hax.repeat (0 : u8) (32 : usize))))
          with
            | (core_models.result.Result.Ok  hoist45) => do
              (pure (core_models.result.Result.Ok
                (rust_primitives.hax.Tuple2.mk prev hoist45)))
            | (core_models.result.Result.Err  err) => do
              (pure (core_models.result.Result.Err err)) :
          RustM
          (core_models.result.Result
            (rust_primitives.hax.Tuple2 u8 (RustArray u8 32))
            u8)))))))
  with
    | (core_models.result.Result.Ok  val) => do
      (pure (core_models.result.Result.Ok rust_primitives.hax.Tuple0.mk))
    | (core_models.result.Result.Err  err) => do
      (pure (core_models.result.Result.Err err))

end new_tests.legacy__side_effects__lib.issue_1300


namespace new_tests.legacy__side_effects__lib.issue_1299

structure Foo where
  y : u8

structure S where
  g : Foo

structure OtherS where
  g : (core_models.option.Option Foo)

@[spec]
def Impl._from (i : Foo) : RustM Foo := do
  (pure (Foo.mk (y := (← (core_models.clone.Clone.clone u8 (Foo.y i))))))

structure Error where
  -- no fields

@[spec]
def Impl_1._from (i : OtherS) : RustM (core_models.result.Result S Error) := do
  match
    (← (core_models.option.Impl.ok_or Foo Error
      (← (core_models.option.Impl.as_ref Foo (OtherS.g i)))
      Error.mk))
  with
    | (core_models.result.Result.Ok  hoist47) => do
      (pure (core_models.result.Result.Ok
        (S.mk (g := (← (Impl._from hoist47))))))
    | (core_models.result.Result.Err  err) => do
      (pure (core_models.result.Result.Err err))

end new_tests.legacy__side_effects__lib.issue_1299

