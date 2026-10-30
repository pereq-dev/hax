
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


namespace new_tests.legacy__mut_ref_functionalization__lib

structure S where
  b : (RustArray u8 5)

@[spec]
def foo (lhs : S) (rhs : S) : RustM S := do
  let lhs : S ←
    (rust_primitives.hax.folds.fold_range
      (0 : usize)
      (1 : usize)
      (fun lhs _ => (do (pure true) : RustM Bool))
      lhs
      (fun lhs i =>
        (do
        (pure {lhs
        with b := (←
        (rust_primitives.hax.monomorphized_update_at.update_at_usize
          (S.b lhs)
          i
          (← ((← (S.b lhs)[i]_?) +? (← (S.b rhs)[i]_?)))))}) :
        RustM S)));
  (pure lhs)

@[spec]
def Impl.update (self : S) (x : u8) : RustM S := do
  let self : S :=
    {self
    with b := (← (rust_primitives.hax.monomorphized_update_at.update_at_usize
      (S.b self)
      (0 : usize)
      x))};
  (pure self)

@[spec]
def index_mutation
    (x : (core_models.ops.range.Range usize))
    (a : (RustSlice u8)) :
    RustM rust_primitives.hax.Tuple0 := do
  let v : (alloc.vec.Vec u8 alloc.alloc.Global) ←
    (alloc.slice.Impl.into_vec u8 alloc.alloc.Global
      (← (rust_primitives.unsize (RustArray.ofVec #v[(1 : u8)]))));
  let v : (alloc.vec.Vec u8 alloc.alloc.Global) ←
    (alloc.slice.Impl.to_vec
      (← (rust_primitives.hax.monomorphized_update_at.update_at_range
        (← (alloc.vec.Impl_1.as_slice v))
        x
        (← (core_models.slice.Impl.copy_from_slice u8 (← v[x]_?) a)))));
  let v : (alloc.vec.Vec u8 alloc.alloc.Global) ←
    (alloc.slice.Impl.to_vec
      (← (rust_primitives.hax.monomorphized_update_at.update_at_usize
        (← (alloc.vec.Impl_1.as_slice v))
        (1 : usize)
        (3 : u8))));
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def index_mutation_unsize (x : (RustArray u8 12)) : RustM u8 := do
  let x : (RustArray u8 12) ←
    (rust_primitives.hax.monomorphized_update_at.update_at_range
      x
      (core_models.ops.range.Range.mk
        (start := (4 : usize))
        (_end := (5 : usize)))
      (← (core_models.slice.Impl.copy_from_slice u8
        (← x[
          (core_models.ops.range.Range.mk
            (start := (4 : usize))
            (_end := (5 : usize)))
          ]_?)
        (← (rust_primitives.unsize
          (RustArray.ofVec #v[(1 : u8), (2 : u8)]))))));
  (pure (42 : u8))

@[spec]
def build_vec (_ : rust_primitives.hax.Tuple0) :
    RustM (alloc.vec.Vec u8 alloc.alloc.Global) := do
  (alloc.slice.Impl.into_vec u8 alloc.alloc.Global
    (← (rust_primitives.unsize
      (RustArray.ofVec #v[(1 : u8), (2 : u8), (3 : u8)]))))

@[spec]
def test_append (_ : rust_primitives.hax.Tuple0) :
    RustM (alloc.vec.Vec u8 alloc.alloc.Global) := do
  let vec1 : (alloc.vec.Vec u8 alloc.alloc.Global) ←
    (alloc.vec.Impl.new u8 rust_primitives.hax.Tuple0.mk);
  let vec2 : (alloc.vec.Vec u8 alloc.alloc.Global) ←
    (alloc.slice.Impl.into_vec u8 alloc.alloc.Global
      (← (rust_primitives.unsize
        (RustArray.ofVec #v[(1 : u8), (2 : u8), (3 : u8)]))));
  let ⟨tmp0, tmp1⟩ ← (alloc.vec.Impl_1.append u8 alloc.alloc.Global vec1 vec2);
  let vec1 : (alloc.vec.Vec u8 alloc.alloc.Global) := tmp0;
  let vec2 : (alloc.vec.Vec u8 alloc.alloc.Global) := tmp1;
  let _ := rust_primitives.hax.Tuple0.mk;
  let vec1 : (alloc.vec.Vec u8 alloc.alloc.Global) ←
    (alloc.vec.Impl_1.append u8 alloc.alloc.Global
      vec1
      (← (build_vec rust_primitives.hax.Tuple0.mk)));
  (pure vec1)

@[spec]
def f (_ : rust_primitives.hax.Tuple0) :
    RustM (alloc.vec.Vec u8 alloc.alloc.Global) := do
  let vec : (alloc.vec.Vec u8 alloc.alloc.Global) ←
    (alloc.vec.Impl.new u8 rust_primitives.hax.Tuple0.mk);
  let vec : (alloc.vec.Vec u8 alloc.alloc.Global) ←
    (alloc.vec.Impl_1.push u8 alloc.alloc.Global vec (1 : u8));
  let vec : (alloc.vec.Vec u8 alloc.alloc.Global) ←
    (alloc.vec.Impl_1.push u8 alloc.alloc.Global vec (2 : u8));
  let vec : (alloc.vec.Vec u8 alloc.alloc.Global) ←
    (alloc.slice.Impl.to_vec
      (← (core_models.slice.Impl.swap u8
        (← (alloc.vec.Impl_1.as_slice vec))
        (0 : usize)
        (1 : usize))));
  let vec : (alloc.vec.Vec u8 alloc.alloc.Global) ←
    (alloc.slice.Impl.to_vec
      (← (core_models.slice.Impl.swap u8
        (← (alloc.vec.Impl_1.as_slice vec))
        (0 : usize)
        (1 : usize))));
  (pure vec)

structure Foo where
  field : (alloc.vec.Vec u8 alloc.alloc.Global)

structure Pair (T : Type) where
  a : T
  b : Foo

@[spec]
def g (x : (Pair (alloc.vec.Vec u8 alloc.alloc.Global))) :
    RustM (alloc.vec.Vec u8 alloc.alloc.Global) := do
  let x : (Pair (alloc.vec.Vec u8 alloc.alloc.Global)) := x;
  let x : (Pair (alloc.vec.Vec u8 alloc.alloc.Global)) ←
    (rust_primitives.hax.folds.fold_range
      (1 : u8)
      (10 : u8)
      (fun x _ => (do (pure true) : RustM Bool))
      x
      (fun x i =>
        (do
        (pure {x
        with a := (← (alloc.vec.Impl_1.push u8 alloc.alloc.Global
          (Pair.a x)
          i))}) :
        RustM (Pair (alloc.vec.Vec u8 alloc.alloc.Global)))));
  let x : (Pair (alloc.vec.Vec u8 alloc.alloc.Global)) :=
    {x
    with a := (← (alloc.slice.Impl.to_vec
      (← (core_models.slice.Impl.swap u8
        (← (alloc.vec.Impl_1.as_slice (Pair.a x)))
        (0 : usize)
        (1 : usize)))))};
  let x : (Pair (alloc.vec.Vec u8 alloc.alloc.Global)) :=
    {x
    with b := {(Pair.b x)
    with field := (← (alloc.slice.Impl.to_vec
      (← (core_models.slice.Impl.swap u8
        (← (alloc.vec.Impl_1.as_slice (Foo.field (Pair.b x))))
        (0 : usize)
        (1 : usize)))))}};
  (pure (Pair.a x))

@[spec]
def h (x : u8) : RustM u8 := do let x : u8 ← (x +? (10 : u8)); (pure x)

structure Bar where
  a : u8
  b : u8

@[spec]
def i (bar : Bar) : RustM (rust_primitives.hax.Tuple2 Bar u8) := do
  let bar : Bar := {bar with b := (← ((Bar.b bar) +? (Bar.a bar)))};
  let bar : Bar := {bar with a := (← (h (Bar.a bar)))};
  let hax_temp_output : u8 ← ((Bar.a bar) +? (Bar.b bar));
  (pure (rust_primitives.hax.Tuple2.mk bar hax_temp_output))

@[spec]
def j (x : Bar) : RustM (rust_primitives.hax.Tuple2 Bar u8) := do
  let out : u8 := (123 : u8);
  let ⟨tmp0, out1⟩ ← (i x);
  let x : Bar := tmp0;
  let hax_temp_output : u8 ← (out1 +? out);
  (pure (rust_primitives.hax.Tuple2.mk x hax_temp_output))

@[spec]
def k
    (vec : (alloc.vec.Vec u8 alloc.alloc.Global))
    (arg_1_wild3 : u16)
    (arg_1_wild : u8)
    (arg_3_wild2 : rust_primitives.hax.Tuple0) :
    RustM
    (rust_primitives.hax.Tuple4
      (alloc.vec.Vec u8 alloc.alloc.Global)
      u16
      rust_primitives.hax.Tuple0
      u64)
    := do
  let arg_1_wild2 : u8 ← vec[(1 : usize)]_?;
  let arg_3_wild : u8 ← vec[(2 : usize)]_?;
  let arg_1_wild1 : u8 ← vec[(3 : usize)]_?;
  let arg_3_wild1 : u8 ← vec[(4 : usize)]_?;
  let vec : (alloc.vec.Vec u8 alloc.alloc.Global) ←
    (alloc.slice.Impl.to_vec
      (← (rust_primitives.hax.monomorphized_update_at.update_at_usize
        (← (alloc.vec.Impl_1.as_slice vec))
        (0 : usize)
        (← ((← ((← ((← (arg_1_wild +? arg_3_wild)) +? arg_1_wild1))
            +? arg_3_wild1))
          +? arg_1_wild)))));
  let hax_temp_output : u64 := (12345 : u64);
  (pure (rust_primitives.hax.Tuple4.mk
    vec
    arg_1_wild3
    arg_3_wild2
    hax_temp_output))

class FooTrait.AssociatedTypes (Self : Type) where

class FooTrait (Self : Type)
  [associatedTypes : outParam (FooTrait.AssociatedTypes (Self : Type))]
  where
  z (Self) : (Self -> RustM Self)

@[spec]
def Impl_1.z_hoisted (self : Foo) : RustM Foo := do (pure self)

@[reducible] instance Impl_1.AssociatedTypes :
  FooTrait.AssociatedTypes Foo
  where

instance Impl_1 : FooTrait Foo where
  z := (Impl_1.z_hoisted)

@[spec]
def array (x : (RustArray u8 10)) : RustM (RustArray u8 10) := do
  let x : (RustArray u8 10) ←
    (rust_primitives.hax.monomorphized_update_at.update_at_usize
      x
      (1 : usize)
      (← x[(2 : usize)]_?));
  (pure x)

end new_tests.legacy__mut_ref_functionalization__lib

