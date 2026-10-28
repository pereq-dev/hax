
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


namespace new_tests.legacy__attributes__lib

def u32_max : u32 := (90000 : u32)

--  A doc comment on `add3`
-- another doc comment on add3
def add3 (x : u32) (y : u32) (z : u32) : RustM u32 := do ((← (x +? y)) +? z)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def add3.spec (x : u32) (y : u32) (z : u32) :
    Spec
      (requires := do
        ((← ((← ((← (x >? (10 : u32))) &&? (← (y >? (10 : u32)))))
            &&? (← (z >? (10 : u32)))))
          &&? (← ((← ((← (x +? y)) +? z)) <? u32_max))))
      (ensures := fun
          result => do
          (hax_lib.prop.constructors.implies
            (← (hax_lib.prop.constructors.from_bool true))
            (← (hax_lib.prop.constructors.from_bool
              (← (result >? (32 : u32)))))))
      (add3 (x : u32) (y : u32) (z : u32)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [add3] <;> bv_decide
}

def swap_and_mut_req_ens (x : u32) (y : u32) :
    RustM (rust_primitives.hax.Tuple3 u32 u32 u32) := do
  let x0 : u32 := x;
  let x : u32 := y;
  let y : u32 := x0;
  let hax_temp_output : u32 ← (x +? y);
  (pure (rust_primitives.hax.Tuple3.mk x y hax_temp_output))

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def swap_and_mut_req_ens.spec (x : u32) (y : u32) :
    Spec
      (requires := do ((← (x <? (40 : u32))) &&? (← (y <? (300 : u32)))))
      (ensures := fun
          ⟨x_future, y_future, result⟩ => do
          ((← ((← (x_future ==? y)) &&? (← (y_future ==? x))))
            &&? (← (result ==? (← (x +? y))))))
      (swap_and_mut_req_ens (x : u32) (y : u32)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [swap_and_mut_req_ens] <;> bv_decide
}

def issue_844 (_x : u8) : RustM u8 := do (pure _x)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def issue_844.spec (_x : u8) :
    Spec
      (requires := do pure True)
      (ensures := fun _x_future => do (pure true))
      (issue_844 (_x : u8)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [issue_844] <;> bv_decide
}

end new_tests.legacy__attributes__lib


namespace new_tests.legacy__attributes__lib.ensures_on_arity_zero_fns

def doing_nothing (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def doing_nothing.spec (_ : rust_primitives.hax.Tuple0) :
    Spec
      (requires := do (pure true))
      (ensures := fun _x => do (pure true))
      (doing_nothing ⟨⟩) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [doing_nothing] <;> bv_decide
}

def basically_a_constant (_ : rust_primitives.hax.Tuple0) : RustM u8 := do
  (pure (127 : u8))

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def basically_a_constant.spec (_ : rust_primitives.hax.Tuple0) :
    Spec
      (requires := do (pure true))
      (ensures := fun x => do (x >? (100 : u8)))
      (basically_a_constant ⟨⟩) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [basically_a_constant] <;> bv_decide
}

end new_tests.legacy__attributes__lib.ensures_on_arity_zero_fns


namespace new_tests.legacy__attributes__lib

def add3_lemma (x : u32) : RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def add3_lemma.spec (x : u32) :
    Spec
      (requires := do pure True)
      (ensures := fun
          _ => do
          ((← ((← (x <=? (10 : u32)))
              ||? (← (x >=? (← (u32_max /? (3 : u32)))))))
            ||? (← ((← (add3 x x x)) ==? (← (x *? (3 : u32)))))))
      (add3_lemma (x : u32)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [add3_lemma] <;> bv_decide
}

@[spec]
def dummy_function (x : u32) : RustM u32 := do (pure x)

def apply_dummy_function_lemma (x : u32) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def apply_dummy_function_lemma.spec (x : u32) :
    Spec
      (requires := do pure True)
      (ensures := fun _ => do (x ==? (← (dummy_function x))))
      (apply_dummy_function_lemma (x : u32)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [apply_dummy_function_lemma] <;> bv_decide
}

end new_tests.legacy__attributes__lib


namespace new_tests.legacy__attributes__lib.postprocess_with

@[spec]
def f (_ : rust_primitives.hax.Tuple0) : RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.legacy__attributes__lib.postprocess_with


namespace new_tests.legacy__attributes__lib.postprocess_with.somewhere

@[spec]
def some_hypothetical_tactic (some_param : u8) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.legacy__attributes__lib.postprocess_with.somewhere


namespace new_tests.legacy__attributes__lib.postprocess_with

@[spec]
def g (_ : rust_primitives.hax.Tuple0) : RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.legacy__attributes__lib.postprocess_with


namespace new_tests.legacy__attributes__lib

structure Foo where
  x : u32
  y : u32
  z : u32

--  Regression test for https://github.com/pereq-dev/hax/issues/899: a refinement
--  on a struct field may mention a const generic (here `LEN`), both in a field
--  type and in the refinement formula itself.
structure FooConstGeneric (LEN : usize) where
  indices : (RustArray u8 LEN)
  length : usize

@[spec]
def props (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ ← (hax_lib.assume (← (hax_lib.prop.Impl.from_bool true)));
  let _ ← (hax_lib.assert_prop (← (hax_lib.prop.Impl.from_bool true)));
  let _ := rust_primitives.hax.Tuple0.mk;
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.legacy__attributes__lib


namespace new_tests.legacy__attributes__lib.refined_arithmetic

structure Foo where
  _0 : u8

def Impl.add_hoisted (self : Foo) (rhs : Foo) : RustM Foo := do
  (pure (Foo.mk (← ((Foo._0 self) +? (Foo._0 rhs)))))

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def Impl.add_hoisted.spec (self : Foo) (rhs : Foo) :
    Spec
      (requires := do ((Foo._0 self) <? (← ((255 : u8) -? (Foo._0 rhs)))))
      (ensures := fun _ => pure True)
      (Impl.add_hoisted (self : Foo) (rhs : Foo)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [Impl.add_hoisted] <;> bv_decide
}

@[reducible] instance Impl.AssociatedTypes :
  core_models.ops.arith.Add.AssociatedTypes Foo Foo
  where
  Output := Foo

instance Impl : core_models.ops.arith.Add Foo Foo where
  add := (Impl.add_hoisted)

def Impl_1.mul_hoisted (self : Foo) (rhs : Foo) : RustM Foo := do
  (pure (Foo.mk (← ((Foo._0 self) *? (Foo._0 rhs)))))

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def Impl_1.mul_hoisted.spec (self : Foo) (rhs : Foo) :
    Spec
      (requires := do
        ((← ((Foo._0 rhs) ==? (0 : u8)))
          ||? (← ((Foo._0 self) <? (← ((255 : u8) /? (Foo._0 rhs)))))))
      (ensures := fun _ => pure True)
      (Impl_1.mul_hoisted (self : Foo) (rhs : Foo)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [Impl_1.mul_hoisted] <;> bv_decide
}

@[reducible] instance Impl_1.AssociatedTypes :
  core_models.ops.arith.Mul.AssociatedTypes Foo Foo
  where
  Output := Foo

instance Impl_1 : core_models.ops.arith.Mul Foo Foo where
  mul := (Impl_1.mul_hoisted)

end new_tests.legacy__attributes__lib.refined_arithmetic


namespace new_tests.legacy__attributes__lib.refined_indexes

def MAX : usize := (10 : usize)

structure MyArray where
  _0 : (RustArray u8 10)

def Impl_1.index_hoisted (self : MyArray) (index : usize) : RustM u8 := do
  (core_models.legacy__attributes__lib.refined_indexes.Impl_1.index_hoisted
    usize self index)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def Impl_1.index_hoisted.spec (self : MyArray) (index : usize) :
    Spec
      (requires := do (index <? MAX))
      (ensures := fun _ => pure True)
      (Impl_1.index_hoisted (self : MyArray) (index : usize)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [Impl_1.index_hoisted] <;> bv_decide
}

@[reducible] instance Impl_1.AssociatedTypes :
  core_models.ops.index.Index.AssociatedTypes MyArray usize
  where
  Output := u8

instance Impl_1 : core_models.ops.index.Index MyArray usize where
  index := (Impl_1.index_hoisted)

--  Triple dash comment
/--
   Multiline double star comment Maecenas blandit accumsan feugiat.
      Done vitae ullamcorper est.
      Curabitur id dui eget sem viverra interdum. 
  -/
@[spec]
def mutation_example
    (use_generic_update_at : MyArray)
    (use_specialized_update_at : (RustSlice u8))
    (specialized_as_well : (alloc.vec.Vec u8 alloc.alloc.Global)) :
    RustM
    (rust_primitives.hax.Tuple3
      MyArray
      (RustSlice u8)
      (alloc.vec.Vec u8 alloc.alloc.Global))
    := do
  let use_generic_update_at : MyArray ←
    (rust_primitives.hax.update_at use_generic_update_at (2 : usize) (0 : u8));
  let use_specialized_update_at : (RustSlice u8) ←
    (rust_primitives.hax.monomorphized_update_at.update_at_usize
      use_specialized_update_at
      (2 : usize)
      (0 : u8));
  let specialized_as_well : (alloc.vec.Vec u8 alloc.alloc.Global) ←
    (alloc.slice.Impl.to_vec
      (← (rust_primitives.hax.monomorphized_update_at.update_at_usize
        (← (alloc.vec.Impl_1.as_slice specialized_as_well))
        (2 : usize)
        (0 : u8))));
  (pure (rust_primitives.hax.Tuple3.mk
    use_generic_update_at
    use_specialized_update_at
    specialized_as_well))

end new_tests.legacy__attributes__lib.refined_indexes


namespace new_tests.legacy__attributes__lib.newtype_pattern

def MAX : usize := (10 : usize)

structure SafeIndex where
  i : usize

@[spec]
def Impl.new (i : usize) : RustM (core_models.option.Option SafeIndex) := do
  if (← (i <? MAX)) then do
    (pure (core_models.option.Option.Some (SafeIndex.mk (i := i))))
  else do
    (pure core_models.option.Option.None)

@[spec]
def Impl.as_usize (self : SafeIndex) : RustM usize := do
  (pure (SafeIndex.i self))

@[spec]
def Impl_1.index_hoisted (T : Type)
    (self : (RustArray T 10))
    (index : SafeIndex) :
    RustM T := do
  self[(SafeIndex.i index)]_?

@[reducible] instance Impl_1.AssociatedTypes (T : Type) :
  core_models.ops.index.Index.AssociatedTypes (RustArray T 10) SafeIndex
  where
  Output := T

instance Impl_1 (T : Type) :
  core_models.ops.index.Index (RustArray T 10) SafeIndex
  where
  index := (Impl_1.index_hoisted T)

end new_tests.legacy__attributes__lib.newtype_pattern


namespace new_tests.legacy__attributes__lib

@[spec]
def inlined_code (foo : Foo) : RustM rust_primitives.hax.Tuple0 := do
  let v_a : i32 := (13 : i32);
  let _ := rust_primitives.hax.Tuple0.mk;
  (pure rust_primitives.hax.Tuple0.mk)

def inlined_code.V : u8 := (12 : u8)

@[spec]
def mutliple_before_after (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def some_function (_ : rust_primitives.hax.Tuple0) :
    RustM alloc.string.String := do
  (core_models.convert.From._from alloc.string.String String "hello from Rust")

end new_tests.legacy__attributes__lib


namespace new_tests.legacy__attributes__lib.future_self

structure Dummy where
  -- no fields

@[instance] opaque Impl_1.AssociatedTypes :
  core_models.marker.StructuralPartialEq.AssociatedTypes Dummy :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_1 :
  core_models.marker.StructuralPartialEq Dummy :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_2.AssociatedTypes :
  core_models.cmp.PartialEq.AssociatedTypes Dummy Dummy :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_2 :
  core_models.cmp.PartialEq Dummy Dummy :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl.AssociatedTypes :
  core_models.cmp.Eq.AssociatedTypes Dummy :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl :
  core_models.cmp.Eq Dummy :=
  by constructor <;> exact Inhabited.default

def Impl_3.f (self : Dummy) : RustM Dummy := do (pure self)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def Impl_3.f.spec (self : Dummy) :
    Spec
      (requires := do pure True)
      (ensures := fun
          self__future => do
          (core_models.cmp.PartialEq.eq Dummy Dummy self__future self))
      (Impl_3.f (self : Dummy)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [Impl_3.f] <;> bv_decide
}

end new_tests.legacy__attributes__lib.future_self


namespace new_tests.legacy__attributes__lib.future_and_result_order

--  One `&mut` input, non-unit result: `(u32, bool)` in the engine lanes,
--  `(bool, u32)` in the aeneas/Lean lane.
def one_mut_and_result (x : u32) :
    RustM (rust_primitives.hax.Tuple2 u32 Bool) := do
  let x : u32 ← (core_models.num.Impl_8.wrapping_add x (1 : u32));
  let hax_temp_output : Bool ← (x ==? (0 : u32));
  (pure (rust_primitives.hax.Tuple2.mk x hax_temp_output))

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def one_mut_and_result.spec (x : u32) :
    Spec
      (requires := do pure True)
      (ensures := fun
          ⟨x_future, result⟩ => do
          ((← (x_future
              ==? (← (core_models.num.Impl_8.wrapping_add x (1 : u32)))))
            &&? (← (result ==? (← (x_future ==? (0 : u32)))))))
      (one_mut_and_result (x : u32)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [one_mut_and_result] <;> bv_decide
}

--  Two `&mut` inputs: pins the futures\' order relative to each other as
--  well as relative to the result. `(u8, u64, bool)` in the engine lanes,
--  `(bool, u8, u64)` in the aeneas/Lean lane.
def two_mut_and_result (a : u8) (b : u64) :
    RustM (rust_primitives.hax.Tuple3 u8 u64 Bool) := do
  let a : u8 ← (core_models.num.Impl_6.wrapping_add a (1 : u8));
  let b : u64 ← (core_models.num.Impl_9.wrapping_add b (2 : u64));
  let hax_temp_output : Bool ← (a ==? (0 : u8));
  (pure (rust_primitives.hax.Tuple3.mk a b hax_temp_output))

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def two_mut_and_result.spec (a : u8) (b : u64) :
    Spec
      (requires := do pure True)
      (ensures := fun
          ⟨a_future, b_future, result⟩ => do
          ((← ((← (a_future
                ==? (← (core_models.num.Impl_6.wrapping_add a (1 : u8)))))
              &&? (← (b_future
                ==? (← (core_models.num.Impl_9.wrapping_add b (2 : u64)))))))
            &&? (← (result ==? (← (a_future ==? (0 : u8)))))))
      (two_mut_and_result (a : u8) (b : u64)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [two_mut_and_result] <;> bv_decide
}

--  One `&mut` input, unit result: the result binder is dropped altogether,
--  so what is left is futures-only and the reordering is a no-op — both
--  lanes agree here.
def one_mut_no_result (x : u32) : RustM u32 := do
  let x : u32 ← (core_models.num.Impl_8.wrapping_add x (1 : u32));
  (pure x)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def one_mut_no_result.spec (x : u32) :
    Spec
      (requires := do pure True)
      (ensures := fun
          x_future => do
          (x_future ==? (← (core_models.num.Impl_8.wrapping_add x (1 : u32)))))
      (one_mut_no_result (x : u32)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [one_mut_no_result] <;> bv_decide
}

end new_tests.legacy__attributes__lib.future_and_result_order


namespace new_tests.legacy__attributes__lib.replace_body

@[spec]
def f (x : u8) (y : u8) : RustM u8 := do ((1 : u8) +? (2 : u8))

structure Foo where
  -- no fields

@[spec]
def Impl.assoc_fn (self : Foo) (x : u8) : RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def Impl_1.to_string_hoisted (self : Foo) : RustM alloc.string.String := do
  (core_models.convert.Into.into String alloc.string.String "Hello")

@[reducible] instance Impl_1.AssociatedTypes :
  alloc.string.ToString.AssociatedTypes Foo
  where

instance Impl_1 : alloc.string.ToString Foo where
  to_string := (Impl_1.to_string_hoisted)

end new_tests.legacy__attributes__lib.replace_body


namespace new_tests.legacy__attributes__lib.pre_post_on_traits_and_impls

structure ViaAdd where
  -- no fields

structure ViaMul where
  -- no fields

def Impl.double_hoisted (x : u8) : RustM u8 := do (x +? x)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def Impl.double_hoisted.spec (x : u8) :
    Spec
      (requires := do
        (rust_primitives.hax.int.le
          (← (rust_primitives.hax.int.from_machine x))
          (← (hax_lib.int.Impl_7._unsafe_from_str "127"))))
      (ensures := fun
          result => do
          (rust_primitives.hax.int.eq
            (← (rust_primitives.hax.int.mul
              (← (rust_primitives.hax.int.from_machine x))
              (← (hax_lib.int.Impl_7._unsafe_from_str "2"))))
            (← (rust_primitives.hax.int.from_machine result))))
      (Impl.double_hoisted (x : u8)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [Impl.double_hoisted] <;> bv_decide
}

def Impl_1.double_hoisted (x : u8) : RustM u8 := do (x *? (2 : u8))

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def Impl_1.double_hoisted.spec (x : u8) :
    Spec
      (requires := do
        (rust_primitives.hax.int.le
          (← (rust_primitives.hax.int.from_machine x))
          (← (hax_lib.int.Impl_7._unsafe_from_str "127"))))
      (ensures := fun
          result => do
          (rust_primitives.hax.int.eq
            (← (rust_primitives.hax.int.mul
              (← (rust_primitives.hax.int.from_machine x))
              (← (hax_lib.int.Impl_7._unsafe_from_str "2"))))
            (← (rust_primitives.hax.int.from_machine result))))
      (Impl_1.double_hoisted (x : u8)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [Impl_1.double_hoisted] <;> bv_decide
}

end new_tests.legacy__attributes__lib.pre_post_on_traits_and_impls


namespace new_tests.legacy__attributes__lib.int_model

structure Int where
  _0 : u128

@[instance] opaque Impl_2.AssociatedTypes :
  core_models.clone.Clone.AssociatedTypes Int :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_2 :
  core_models.clone.Clone Int :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_1.AssociatedTypes :
  core_models.marker.Copy.AssociatedTypes Int :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_1 :
  core_models.marker.Copy Int :=
  by constructor <;> exact Inhabited.default

@[spec]
def add (x : Int) (y : Int) : RustM Int := do
  (pure (Int.mk (← ((Int._0 x) +? (Int._0 y)))))

@[spec]
def Impl.sub_hoisted (self : Int) (other : Int) : RustM Int := do
  (pure (Int.mk (← ((Int._0 self) +? (Int._0 other)))))

@[reducible] instance Impl.AssociatedTypes :
  core_models.ops.arith.Sub.AssociatedTypes Int Int
  where
  Output := Int

instance Impl : core_models.ops.arith.Sub Int Int where
  sub := (Impl.sub_hoisted)

end new_tests.legacy__attributes__lib.int_model


namespace new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__BoundedU8

abbrev BoundedU8 (MIN : u8) (MAX : u8) : Type := u8

end new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__BoundedU8


namespace new_tests.legacy__attributes__lib.refinement_types

@[spec]
def bounded_u8
    (x :
    (new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__BoundedU8.BoundedU8
      ((12 : u8))
      ((15 : u8))))
    (y :
    (new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__BoundedU8.BoundedU8
      ((10 : u8))
      ((11 : u8)))) :
    RustM
    (new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__BoundedU8.BoundedU8
      ((1 : u8))
      ((23 : u8)))
    := do
  (pure ((← ((x : u8) +? (y : u8))) :
  (new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__BoundedU8.BoundedU8
    ((1 : u8))
    ((23 : u8)))))

end new_tests.legacy__attributes__lib.refinement_types


namespace new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__Even

--  Even `u8` numbers. Constructing pub Even values triggers static
--  proofs in the extraction.
abbrev Even : Type := u8

end new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__Even


namespace new_tests.legacy__attributes__lib.refinement_types

def double (x : u8) :
    RustM
    new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__Even.Even
    := do
  (pure ((← (x +? x)) :
  new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__Even.Even))

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def double.spec (x : u8) :
    Spec
      (requires := do (x <? (127 : u8)))
      (ensures := fun _ => pure True)
      (double (x : u8)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [double] <;> bv_decide
}

def double_refine (x : u8) :
    RustM
    new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__Even.Even
    := do
  (pure ((← (x +? x)) :
  new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__Even.Even))

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def double_refine.spec (x : u8) :
    Spec
      (requires := do (x <? (127 : u8)))
      (ensures := fun _ => pure True)
      (double_refine (x : u8)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [double_refine] <;> bv_decide
}

end new_tests.legacy__attributes__lib.refinement_types


namespace new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__NoE

--  A string that contains no space.
abbrev NoE : Type := alloc.string.String

end new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__NoE


namespace new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__ModInverse

--  A modular mutliplicative inverse
abbrev ModInverse (MOD : u32) : Type := u32

end new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__ModInverse


namespace new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__FieldElement

--  A field element
abbrev FieldElement : Type := u16

end new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__FieldElement


namespace new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__CompressionFactor

--  Example of a specific constraint on a value
abbrev CompressionFactor : Type := u8

end new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__CompressionFactor


namespace new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__BoundedAbsI16

abbrev BoundedAbsI16 (B : usize) : Type := i16

@[instance] opaque Impl.AssociatedTypes (B : usize) :
  core_models.clone.Clone.AssociatedTypes (BoundedAbsI16 (B)) :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl (B : usize) :
  core_models.clone.Clone (BoundedAbsI16 (B)) :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_1.AssociatedTypes (B : usize) :
  core_models.marker.Copy.AssociatedTypes (BoundedAbsI16 (B)) :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_1 (B : usize) :
  core_models.marker.Copy (BoundedAbsI16 (B)) :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_3.AssociatedTypes (B : usize) :
  core_models.marker.StructuralPartialEq.AssociatedTypes (BoundedAbsI16 (B)) :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_3 (B : usize) :
  core_models.marker.StructuralPartialEq (BoundedAbsI16 (B)) :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_4.AssociatedTypes (B : usize) :
  core_models.cmp.PartialEq.AssociatedTypes
  (BoundedAbsI16 (B))
  (BoundedAbsI16 (B)) :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_4 (B : usize) :
  core_models.cmp.PartialEq (BoundedAbsI16 (B)) (BoundedAbsI16 (B)) :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_2.AssociatedTypes (B : usize) :
  core_models.cmp.Eq.AssociatedTypes (BoundedAbsI16 (B)) :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_2 (B : usize) :
  core_models.cmp.Eq (BoundedAbsI16 (B)) :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_6.AssociatedTypes (B : usize) :
  core_models.cmp.PartialOrd.AssociatedTypes
  (BoundedAbsI16 (B))
  (BoundedAbsI16 (B)) :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_6 (B : usize) :
  core_models.cmp.PartialOrd (BoundedAbsI16 (B)) (BoundedAbsI16 (B)) :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_5.AssociatedTypes (B : usize) :
  core_models.cmp.Ord.AssociatedTypes (BoundedAbsI16 (B)) :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_5 (B : usize) :
  core_models.cmp.Ord (BoundedAbsI16 (B)) :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_7.AssociatedTypes (B : usize) :
  core_models.hash.Hash.AssociatedTypes (BoundedAbsI16 (B)) :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_7 (B : usize) :
  core_models.hash.Hash (BoundedAbsI16 (B)) :=
  by constructor <;> exact Inhabited.default

end new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__BoundedAbsI16


namespace new_tests.legacy__attributes__lib.refinement_types

def double_abs_i16 (N : usize) (M : usize)
    (x :
    (new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__BoundedAbsI16.BoundedAbsI16
      (N))) :
    RustM
    (new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__BoundedAbsI16.BoundedAbsI16
      (M))
    := do
  (pure ((← (core_models.ops.arith.Mul.mul
    (new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__BoundedAbsI16.BoundedAbsI16
      (N))
    i16 x (2 : i16))) :
  (new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__BoundedAbsI16.BoundedAbsI16
    (M))))

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def
      double_abs_i16.spec (N : usize) (M : usize)
      (x :
      (new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__BoundedAbsI16.BoundedAbsI16
        (N))) :
    Spec
      (requires := do
        ((← (rust_primitives.hax.int.lt
            (← (rust_primitives.hax.int.from_machine M))
            (← (hax_lib.int.Impl_7._unsafe_from_str "32768"))))
          &&? (← (rust_primitives.hax.int.eq
            (← (rust_primitives.hax.int.from_machine M))
            (← (rust_primitives.hax.int.mul
              (← (rust_primitives.hax.int.from_machine N))
              (← (hax_lib.int.Impl_7._unsafe_from_str "2"))))))))
      (ensures := fun _ => pure True)
      (double_abs_i16
        (N : usize)
        (M : usize)
        (x :
        (new_tests.legacy__attributes__lib.refinement_types.hax__autogenerated_refinement__BoundedAbsI16.BoundedAbsI16
          (N)))) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [double_abs_i16] <;> bv_decide
}

end new_tests.legacy__attributes__lib.refinement_types


namespace new_tests.legacy__attributes__lib.nested_refinement_elim.hax__autogenerated_refinement__DummyRefinement

abbrev DummyRefinement : Type := u16

end new_tests.legacy__attributes__lib.nested_refinement_elim.hax__autogenerated_refinement__DummyRefinement


namespace new_tests.legacy__attributes__lib.nested_refinement_elim

@[spec]
def elim_twice
    (x :
    new_tests.legacy__attributes__lib.nested_refinement_elim.hax__autogenerated_refinement__DummyRefinement.DummyRefinement)
    :
    RustM u16 := do
  (pure (((x : u16) :
  new_tests.legacy__attributes__lib.nested_refinement_elim.hax__autogenerated_refinement__DummyRefinement.DummyRefinement)
  : u16))

end new_tests.legacy__attributes__lib.nested_refinement_elim


namespace new_tests.legacy__attributes__lib.inlined_code_ensures_requires

def increment_array (v : (RustArray u8 4)) : RustM (RustArray u8 4) := do
  let v : (RustArray u8 4) ←
    (rust_primitives.hax.monomorphized_update_at.update_at_usize
      v
      (0 : usize)
      (← ((← v[(0 : usize)]_?) +? (1 : u8))));
  let v : (RustArray u8 4) ←
    (rust_primitives.hax.monomorphized_update_at.update_at_usize
      v
      (1 : usize)
      (← ((← v[(1 : usize)]_?) +? (1 : u8))));
  let v : (RustArray u8 4) ←
    (rust_primitives.hax.monomorphized_update_at.update_at_usize
      v
      (2 : usize)
      (← ((← v[(2 : usize)]_?) +? (1 : u8))));
  let v : (RustArray u8 4) ←
    (rust_primitives.hax.monomorphized_update_at.update_at_usize
      v
      (3 : usize)
      (← ((← v[(3 : usize)]_?) +? (1 : u8))));
  (pure v)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def increment_array.spec (v : (RustArray u8 4)) :
    Spec
      (requires := do (hax_lib.prop.Impl.from_bool true))
      (ensures := fun
          v_future => do
          let future_v : (RustArray u8 4) := v_future;
          (hax_lib.prop.Impl.from_bool true))
      (increment_array (v : (RustArray u8 4))) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [increment_array] <;> bv_decide
}

end new_tests.legacy__attributes__lib.inlined_code_ensures_requires


namespace new_tests.legacy__attributes__lib.verifcation_status

@[spec]
def a_function_which_only_laxes (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  (hax_lib.assert false)

def a_panicfree_function (_ : rust_primitives.hax.Tuple0) : RustM u8 := do
  let a : u8 := (3 : u8);
  let b : u8 := (6 : u8);
  let result : u8 ← (a +? b);
  let _ := rust_primitives.hax.Tuple0.mk;
  (pure result)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def a_panicfree_function.spec (_ : rust_primitives.hax.Tuple0) :
    Spec
      (requires := do pure True)
      (ensures := fun x => do (pure false))
      (a_panicfree_function ⟨⟩) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [a_panicfree_function] <;> bv_decide
}

def another_panicfree_function (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let not_much : i32 := (0 : i32);
  let nothing : i32 := (0 : i32);
  let still_not_much : i32 ← (not_much +? nothing);
  (pure rust_primitives.hax.Tuple0.mk)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def another_panicfree_function.spec (_ : rust_primitives.hax.Tuple0) :
    Spec
      (requires := do pure True)
      (ensures := fun x => do (pure false))
      (another_panicfree_function ⟨⟩) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [another_panicfree_function] <;> bv_decide
}

end new_tests.legacy__attributes__lib.verifcation_status


namespace new_tests.legacy__attributes__lib.requires_mut

def Impl.f_hoisted (x : u8) (y : u8) :
    RustM (rust_primitives.hax.Tuple2 u8 u8) := do
  let y : u8 ← (y +? x);
  let hax_temp_output : u8 := y;
  (pure (rust_primitives.hax.Tuple2.mk y hax_temp_output))

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def Impl.f_hoisted.spec (x : u8) (y : u8) :
    Spec
      (requires := do
        (rust_primitives.hax.int.lt
          (← (rust_primitives.hax.int.add
            (← (rust_primitives.hax.int.from_machine x))
            (← (rust_primitives.hax.int.from_machine y))))
          (← (hax_lib.int.Impl_7._unsafe_from_str "254"))))
      (ensures := fun
          ⟨y_future, output_variable⟩ => do
          (output_variable ==? y_future))
      (Impl.f_hoisted (x : u8) (y : u8)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [Impl.f_hoisted] <;> bv_decide
}

def Impl.g_hoisted (x : u8) (y : u8) : RustM u8 := do (pure y)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def Impl.g_hoisted.spec (x : u8) (y : u8) :
    Spec
      (requires := do (pure true))
      (ensures := fun output_variable => do (output_variable ==? y))
      (Impl.g_hoisted (x : u8) (y : u8)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [Impl.g_hoisted] <;> bv_decide
}

def Impl.h_hoisted (x : u8) (y : u8) : RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def Impl.h_hoisted.spec (x : u8) (y : u8) :
    Spec
      (requires := do (pure true))
      (ensures := fun
          output_variable => do
          (core_models.cmp.PartialEq.eq
            rust_primitives.hax.Tuple0
            rust_primitives.hax.Tuple0
            output_variable
            rust_primitives.hax.Tuple0.mk))
      (Impl.h_hoisted (x : u8) (y : u8)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [Impl.h_hoisted] <;> bv_decide
}

def Impl.i_hoisted (x : u8) (y : u8) : RustM u8 := do
  let _ := rust_primitives.hax.Tuple0.mk;
  (pure y)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def Impl.i_hoisted.spec (x : u8) (y : u8) :
    Spec
      (requires := do (pure true))
      (ensures := fun y_future => do (y_future ==? y))
      (Impl.i_hoisted (x : u8) (y : u8)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [Impl.i_hoisted] <;> bv_decide
}

end new_tests.legacy__attributes__lib.requires_mut


namespace new_tests.legacy__attributes__lib.issue_2089

class Super.AssociatedTypes (Self : Type) where
  B : Type

attribute [reducible] Super.AssociatedTypes.B

abbrev Super.B :=
  Super.AssociatedTypes.B

class Super (Self : Type)
  [associatedTypes : outParam (Super.AssociatedTypes (Self : Type))]
  where

structure S where
  -- no fields

@[reducible] instance Impl.AssociatedTypes : Super.AssociatedTypes S where
  B := u8

instance Impl : Super S where

def Impl_1.C_hoisted : u8 := (1 : u8)

def Impl_1.f_hoisted (x : u32) : RustM u8 := do (pure (0 : u8))

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def Impl_1.f_hoisted.spec (x : u32) :
    Spec
      (requires := do (pure true))
      (ensures := fun _ => pure True)
      (Impl_1.f_hoisted (x : u32)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [Impl_1.f_hoisted] <;> bv_decide
}

def Impl_1.g_hoisted (self : S) (x : u32) : RustM u32 := do (pure x)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def Impl_1.g_hoisted.spec (self : S) (x : u32) :
    Spec
      (requires := do pure True)
      (ensures := fun result => do (pure true))
      (Impl_1.g_hoisted (self : S) (x : u32)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [Impl_1.g_hoisted] <;> bv_decide
}

def Impl_1.h_hoisted
    (Y : Type)
    [trait_constr_h_hoisted_associated_type_i0 :
      core_models.convert.Into.AssociatedTypes
      Y
      u32]
    [trait_constr_h_hoisted_i0 : core_models.convert.Into Y u32 ]
    (x : u8)
    (y : Y)
    (z : u16) :
    RustM u32 := do
  (core_models.convert.Into.into Y u32 y)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def
      Impl_1.h_hoisted.spec
      (Y : Type)
      [trait_constr_h_hoisted_associated_type_i0 :
        core_models.convert.Into.AssociatedTypes
        Y
        u32]
      [trait_constr_h_hoisted_i0 : core_models.convert.Into Y u32 ]
      (x : u8)
      (y : Y)
      (z : u16) :
    Spec
      (requires := do (pure true))
      (ensures := fun result => do (pure true))
      (Impl_1.h_hoisted (Y : Type) (x : u8) (y : Y) (z : u16)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [Impl_1.h_hoisted] <;> bv_decide
}

def Impl_1.plain_hoisted (x : u8) : RustM u8 := do (pure x)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def Impl_1.plain_hoisted.spec (x : u8) :
    Spec
      (requires := do (x >? (0 : u8)))
      (ensures := fun _ => pure True)
      (Impl_1.plain_hoisted (x : u8)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [Impl_1.plain_hoisted] <;> bv_decide
}

end new_tests.legacy__attributes__lib.issue_2089


namespace new_tests.legacy__attributes__lib.props

@[spec]
def f (x : hax_lib.prop.Prop) (y : Bool) : RustM hax_lib.prop.Prop := do
  let xprop : hax_lib.prop.Prop ← (hax_lib.prop.constructors.from_bool y);
  let p : hax_lib.prop.Prop ←
    (hax_lib.prop.constructors.and
      (← (hax_lib.prop.constructors.and
        (← (hax_lib.prop.constructors.and
          (← (hax_lib.prop.constructors.from_bool y))
          xprop))
        (← (hax_lib.prop.constructors.from_bool y))))
      (← (hax_lib.prop.constructors.from_bool y)));
  (hax_lib.prop.constructors.not
    (← (hax_lib.prop.constructors.implies
      (← (hax_lib.prop.constructors.or
        p
        (← (hax_lib.prop.constructors.from_bool y))))
      (← (hax_lib.prop.constructors.and
        (← (hax_lib.prop.constructors.forall
          (fun x =>
            (do
            (hax_lib.prop.constructors.from_bool
              (← (x <=? core_models.num.Impl_6.MAX))) :
            RustM hax_lib.prop.Prop))))
        (← (hax_lib.prop.constructors.exists
          (fun x =>
            (do
            (hax_lib.prop.constructors.from_bool (← (x >? (300 : u16)))) :
            RustM hax_lib.prop.Prop)))))))))

end new_tests.legacy__attributes__lib.props


namespace new_tests.legacy__attributes__lib.reorder

structure Foo where
  field_3 : u8
  field_4 : u8
  field_2 : u8
  field_1 : u8

inductive Bar : Type
| A (a_field_3 : u8) (a_field_1 : u8) (a_field_2 : u8) : Bar
| B (b_field_1 : u8) (b_field_3 : u8) (b_field_2 : u8) : Bar

end new_tests.legacy__attributes__lib.reorder


namespace new_tests.legacy__attributes__lib.issue_1276

structure S where
  _0 : u8

def Impl.f (self : S) (self_ : u8) (self_0 : u8) (self_1 : u8) (self_2 : u8) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def
      Impl.f.spec
      (self : S)
      (self_ : u8)
      (self_0 : u8)
      (self_1 : u8)
      (self_2 : u8) :
    Spec
      (requires := do
        ((← ((← ((S._0 self) ==? (0 : u8))) &&? (← (self_ ==? self_1))))
          &&? (← (self_2 ==? (9 : u8)))))
      (ensures := fun _ => pure True)
      (Impl.f (self : S) (self_ : u8) (self_0 : u8) (self_1 : u8) (self_2 : u8))
    := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [Impl.f] <;> bv_decide
}

end new_tests.legacy__attributes__lib.issue_1276


namespace new_tests.legacy__attributes__lib.issue_evit_57

structure Foo where
  -- no fields

def Impl.f (self : Foo) : RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def Impl.f.spec (self : Foo) :
    Spec
      (requires := do (pure true))
      (ensures := fun _ => pure True)
      (Impl.f (self : Foo)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [Impl.f] <;> bv_decide
}

end new_tests.legacy__attributes__lib.issue_evit_57


namespace new_tests.legacy__attributes__lib

@[spec]
def fib (x : usize) : RustM usize := do
  if (← (x <=? (2 : usize))) then do
    (pure x)
  else do
    (core_models.num.Impl_11.wrapping_add
      (← (fib (← (x -? (1 : usize)))))
      (← (fib (← (x -? (2 : usize))))))
partial_fixpoint

end new_tests.legacy__attributes__lib


namespace new_tests.legacy__attributes__lib.issue_1266

class T.AssociatedTypes (Self : Type) where

class T (Self : Type)
  [associatedTypes : outParam (T.AssociatedTypes (Self : Type))]
  where
  v (Self) : (Self -> RustM Self)

end new_tests.legacy__attributes__lib.issue_1266


namespace new_tests.legacy__attributes__lib.pre_post_on_traits_and_impls

class Operation.AssociatedTypes (Self : Type) where

class Operation (Self : Type)
  [associatedTypes : outParam (Operation.AssociatedTypes (Self : Type))]
  where
  double (Self) : (u8 -> RustM u8)

@[reducible] instance Impl.AssociatedTypes :
  Operation.AssociatedTypes ViaAdd
  where

instance Impl : Operation ViaAdd where
  double := (Impl.double_hoisted)

@[reducible] instance Impl_1.AssociatedTypes :
  Operation.AssociatedTypes ViaMul
  where

instance Impl_1 : Operation ViaMul where
  double := (Impl_1.double_hoisted)

class TraitWithRequiresAndEnsures.AssociatedTypes (Self : Type) where

class TraitWithRequiresAndEnsures (Self : Type)
  [associatedTypes : outParam (TraitWithRequiresAndEnsures.AssociatedTypes (Self
      : Type))]
  where
  method (Self) : (Self -> u8 -> RustM u8)

@[spec]
def test
    (T : Type)
    [trait_constr_test_associated_type_i0 :
      TraitWithRequiresAndEnsures.AssociatedTypes
      T]
    [trait_constr_test_i0 : TraitWithRequiresAndEnsures T ]
    (x : T) :
    RustM u8 := do
  ((← (TraitWithRequiresAndEnsures.method T x (99 : u8))) -? (88 : u8))

end new_tests.legacy__attributes__lib.pre_post_on_traits_and_impls


namespace new_tests.legacy__attributes__lib.requires_mut

class Foo.AssociatedTypes (Self : Type) where

class Foo (Self : Type)
  [associatedTypes : outParam (Foo.AssociatedTypes (Self : Type))]
  where
  f (Self) : (u8 -> u8 -> RustM (rust_primitives.hax.Tuple2 u8 u8))
  g (Self) : (u8 -> u8 -> RustM u8)
  h (Self) : (u8 -> u8 -> RustM rust_primitives.hax.Tuple0)
  i (Self) : (u8 -> u8 -> RustM u8)

@[reducible] instance Impl.AssociatedTypes :
  Foo.AssociatedTypes rust_primitives.hax.Tuple0
  where

instance Impl : Foo rust_primitives.hax.Tuple0 where
  f := (Impl.f_hoisted)
  g := (Impl.g_hoisted)
  h := (Impl.h_hoisted)
  i := (Impl.i_hoisted)

end new_tests.legacy__attributes__lib.requires_mut


namespace new_tests.legacy__attributes__lib.issue_2089

class T.AssociatedTypes (Self : Type) (X : Type) where
  [trait_constr_T_i0 : Super.AssociatedTypes Self]
  A : Type

attribute [instance_reducible, instance] T.AssociatedTypes.trait_constr_T_i0

attribute [reducible] T.AssociatedTypes.A

abbrev T.A :=
  T.AssociatedTypes.A

class T (Self : Type) (X : Type)
  [associatedTypes : outParam (T.AssociatedTypes (Self : Type) (X : Type))]
  where
  [trait_constr_T_i0 : Super Self]
  C (Self) (X) : u8
  f (Self) (X) : (associatedTypes.A -> RustM u8)
  g (Self) (X) : (Self -> associatedTypes.A -> RustM associatedTypes.A)
  h (Self) (X)
    (Y : Type)
    [trait_constr_h_associated_type_i1 :
      core_models.convert.Into.AssociatedTypes
      Y
      associatedTypes.A]
    [trait_constr_h_i1 : core_models.convert.Into Y associatedTypes.A ] :
    ((Super.B Self) -> Y -> X -> RustM associatedTypes.A)
  plain (Self) (X) : (u8 -> RustM u8)

attribute [instance_reducible, instance] T.trait_constr_T_i0

@[reducible] instance Impl_1.AssociatedTypes : T.AssociatedTypes S u16 where
  A := u32

instance Impl_1 : T S u16 where
  C := (Impl_1.C_hoisted)
  f := (Impl_1.f_hoisted)
  g := (Impl_1.g_hoisted)
  h :=
    fun
      
      (Y : Type)
      [trait_constr__associated_type_i0 :
        core_models.convert.Into.AssociatedTypes
        Y
        u32]
      [trait_constr__i0 : core_models.convert.Into Y u32 ]
      =>
    (Impl_1.h_hoisted Y)
  plain := (Impl_1.plain_hoisted)

end new_tests.legacy__attributes__lib.issue_2089

