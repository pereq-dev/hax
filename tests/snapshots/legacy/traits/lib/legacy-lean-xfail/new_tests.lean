
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


namespace new_tests.legacy__traits__lib

class SuperTrait.AssociatedTypes (Self : Type) where
  [trait_constr_SuperTrait_i0 : core_models.clone.Clone.AssociatedTypes Self]

attribute [instance_reducible, instance]
  SuperTrait.AssociatedTypes.trait_constr_SuperTrait_i0

class SuperTrait (Self : Type)
  [associatedTypes : outParam (SuperTrait.AssociatedTypes (Self : Type))]
  where
  [trait_constr_SuperTrait_i0 : core_models.clone.Clone Self]
  function_of_super_trait (Self) : (Self -> RustM u32)

attribute [instance_reducible, instance] SuperTrait.trait_constr_SuperTrait_i0

@[spec]
def Impl.function_of_super_trait_hoisted (self : i32) : RustM u32 := do
  (rust_primitives.hax.cast_op
    (← (core_models.num.Impl_2.abs self)) :
    RustM u32)

@[reducible] instance Impl.AssociatedTypes :
  SuperTrait.AssociatedTypes i32
  where

instance Impl : SuperTrait i32 where
  function_of_super_trait := (Impl.function_of_super_trait_hoisted)

def Impl_1.N_hoisted : usize := (32 : usize)

@[spec]
def Impl_1.assoc_f_hoisted (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def Impl_1.method_f_hoisted (self : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  (Impl_1.assoc_f_hoisted rust_primitives.hax.Tuple0.mk)

@[spec]
def Impl_1.assoc_type_hoisted (_ : i32) : RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

structure Struct where
  -- no fields

class Bar.AssociatedTypes (Self : Type) where

class Bar (Self : Type)
  [associatedTypes : outParam (Bar.AssociatedTypes (Self : Type))]
  where
  bar (Self) : (Self -> RustM rust_primitives.hax.Tuple0)

@[spec]
def Impl_2.method
    (T : Type)
    [trait_constr_method_associated_type_i0 : Bar.AssociatedTypes T]
    [trait_constr_method_i0 : Bar T ]
    (x : T) :
    RustM rust_primitives.hax.Tuple0 := do
  (Bar.bar T x)

@[spec]
def closure_impl_expr
    (I : Type)
    [trait_constr_closure_impl_expr_associated_type_i0 :
      core_models.iter.traits.iterator.Iterator.AssociatedTypes
      I]
    [trait_constr_closure_impl_expr_i0 :
      core_models.iter.traits.iterator.Iterator
      I
      (associatedTypes := {
        show core_models.iter.traits.iterator.Iterator.AssociatedTypes I
        by infer_instance
        with Item := rust_primitives.hax.Tuple0})]
    (it : I) :
    RustM (alloc.vec.Vec rust_primitives.hax.Tuple0 alloc.alloc.Global) := do
  (core_models.iter.traits.iterator.Iterator.collect
    (core_models.iter.adapters.map.Map
      I
      (rust_primitives.hax.Tuple0 -> RustM rust_primitives.hax.Tuple0))
    (alloc.vec.Vec rust_primitives.hax.Tuple0 alloc.alloc.Global)
    (← (core_models.iter.traits.iterator.Iterator.map
      I
      rust_primitives.hax.Tuple0
      (rust_primitives.hax.Tuple0 -> RustM rust_primitives.hax.Tuple0)
      it
      (fun x => (do (pure x) : RustM rust_primitives.hax.Tuple0)))))

--  @fail(extraction): legacy-lean(HAX0001)
@[spec]
def closure_impl_expr_fngen
    (I : Type)
    (F : Type)
    [trait_constr_closure_impl_expr_fngen_associated_type_i0 :
      core_models.iter.traits.iterator.Iterator.AssociatedTypes
      I]
    [trait_constr_closure_impl_expr_fngen_i0 :
      core_models.iter.traits.iterator.Iterator
      I
      (associatedTypes := {
        show core_models.iter.traits.iterator.Iterator.AssociatedTypes I
        by infer_instance
        with Item := rust_primitives.hax.Tuple0, sorry})]
    [trait_constr_closure_impl_expr_fngen_associated_type_i1 :
      core_models.ops.function.FnMut.AssociatedTypes
      F
      (rust_primitives.hax.Tuple1 rust_primitives.hax.Tuple0)]
    [trait_constr_closure_impl_expr_fngen_i1 : core_models.ops.function.FnMut
      F
      (rust_primitives.hax.Tuple1 rust_primitives.hax.Tuple0)
      (associatedTypes := {
        show
          core_models.ops.function.FnMut.AssociatedTypes
          F
          (rust_primitives.hax.Tuple1 rust_primitives.hax.Tuple0)
        by infer_instance
        with sorry})]
    (it : I)
    (f : F) :
    RustM (alloc.vec.Vec rust_primitives.hax.Tuple0 alloc.alloc.Global) := do
  (core_models.iter.traits.iterator.Iterator.collect
    (core_models.iter.adapters.map.Map I F)
    (alloc.vec.Vec rust_primitives.hax.Tuple0 alloc.alloc.Global)
    (← (core_models.iter.traits.iterator.Iterator.map
      I rust_primitives.hax.Tuple0 F it f)))

inductive Error : Type
| Fail : Error

@[spec]
def Error_cast_to_repr (x : Error) : RustM isize := do
  match x with | (Error.Fail ) => do (pure (0 : isize))

@[spec]
def Impl_3.for_application_callback (_ : rust_primitives.hax.Tuple0) :
    RustM (rust_primitives.hax.Tuple0 -> RustM Error) := do
  (pure (fun _ => (do (pure Error.Fail) : RustM Error)))

@[spec]
def iter_option (T : Type) (x : (core_models.option.Option T)) :
    RustM (core_models.option.IntoIter T) := do
  (core_models.iter.traits.collect.IntoIterator.into_iter
    (core_models.option.Option T) (← (core_models.option.Impl.as_ref T x)))

@[spec]
def use_impl_trait (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let iter : (core_models.option.IntoIter Bool) ←
    (iter_option Bool (core_models.option.Option.Some false));
  let ⟨tmp0, out⟩ ←
    (core_models.iter.traits.iterator.Iterator.next
      (core_models.option.IntoIter Bool) iter);
  let iter : (core_models.option.IntoIter Bool) := tmp0;
  let _ := out;
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.legacy__traits__lib


namespace new_tests.legacy__traits__lib.for_clauses

class Foo.AssociatedTypes (Self : Type) (T : Type) where

class Foo (Self : Type) (T : Type)
  [associatedTypes : outParam (Foo.AssociatedTypes (Self : Type) (T : Type))]
  where
  to_t (Self) (T) : (Self -> RustM T)

--  @fail(extraction): proverif(HAX0001)
@[spec]
def _f
    (X : Type)
    [trait_constr__f_associated_type_i0 : Foo.AssociatedTypes X u8]
    [trait_constr__f_i0 : Foo X u8 ]
    (x : X) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ ← (Foo.to_t X u8 x);
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.legacy__traits__lib.for_clauses


namespace new_tests.legacy__traits__lib.for_clauses.issue_495

--  @fail(extraction): proverif(HAX0001)
@[spec]
def original_function_from_495 (list : (alloc.vec.Vec u8 alloc.alloc.Global)) :
    RustM rust_primitives.hax.Tuple0 := do
  let _indices : (alloc.vec.Vec u8 alloc.alloc.Global) ←
    (core_models.iter.traits.iterator.Iterator.collect
      (core_models.iter.adapters.filter.Filter
        (core_models.ops.range.Range u8)
        (u8 -> RustM Bool)) (alloc.vec.Vec u8 alloc.alloc.Global)
      (← (core_models.iter.traits.iterator.Iterator.filter
        (core_models.ops.range.Range u8) (u8 -> RustM Bool)
        (core_models.ops.range.Range.mk (start := (0 : u8)) (_end := (5 : u8)))
        (fun i =>
          (do
          let ⟨_, out⟩ ←
            (core_models.iter.traits.iterator.Iterator.any
              (core_models.slice.iter.Iter u8) (u8 -> RustM Bool)
              (← (core_models.slice.Impl.iter u8
                (← (core_models.ops.deref.Deref.deref
                  (alloc.vec.Vec u8 alloc.alloc.Global) list))))
              (fun n => (do (n ==? i) : RustM Bool)));
          (pure out) :
          RustM Bool)))));
  (pure rust_primitives.hax.Tuple0.mk)

--  @fail(extraction): proverif(HAX0001)
@[spec]
def minimized_1 (list : (alloc.vec.Vec u8 alloc.alloc.Global)) :
    RustM (alloc.vec.Vec u8 alloc.alloc.Global) := do
  (core_models.iter.traits.iterator.Iterator.collect
    (core_models.iter.adapters.filter.Filter
      (core_models.ops.range.Range u8)
      (u8 -> RustM Bool)) (alloc.vec.Vec u8 alloc.alloc.Global)
    (← (core_models.iter.traits.iterator.Iterator.filter
      (core_models.ops.range.Range u8) (u8 -> RustM Bool)
      (core_models.ops.range.Range.mk (start := (0 : u8)) (_end := (5 : u8)))
      (fun _ => (do (pure true) : RustM Bool)))))

@[spec]
def minimized_2
    (it :
    (core_models.iter.adapters.filter.Filter
      (core_models.ops.range.Range u8)
      (u8 -> RustM Bool))) :
    RustM rust_primitives.hax.Tuple0 := do
  let _indices : (alloc.vec.Vec u8 alloc.alloc.Global) ←
    (core_models.iter.traits.iterator.Iterator.collect
      (core_models.iter.adapters.filter.Filter
        (core_models.ops.range.Range u8)
        (u8 -> RustM Bool)) (alloc.vec.Vec u8 alloc.alloc.Global) it);
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.legacy__traits__lib.for_clauses.issue_495


namespace new_tests.legacy__traits__lib.for_clauses.issue_495.minimized_3

class Trait.AssociatedTypes (Self : Type) where

class Trait (Self : Type)
  [associatedTypes : outParam (Trait.AssociatedTypes (Self : Type))]
  where

--  @fail(extraction): legacy-lean(HAX0001)
@[reducible] instance Impl.AssociatedTypes
  (P : Type)
  [trait_constr_Impl_associated_type_i0 :
    core_models.ops.function.FnMut.AssociatedTypes
    P
    (rust_primitives.hax.Tuple1 u8)]
  [trait_constr_Impl_i0 : core_models.ops.function.FnMut
    P
    (rust_primitives.hax.Tuple1 u8)
    (associatedTypes := {
      show
        core_models.ops.function.FnMut.AssociatedTypes
        P
        (rust_primitives.hax.Tuple1 u8)
      by infer_instance
      with sorry})] :
  Trait.AssociatedTypes P
  where

instance Impl
  (P : Type)
  [trait_constr_Impl_associated_type_i0 :
    core_models.ops.function.FnMut.AssociatedTypes
    P
    (rust_primitives.hax.Tuple1 u8)]
  [trait_constr_Impl_i0 : core_models.ops.function.FnMut
    P
    (rust_primitives.hax.Tuple1 u8)
    (associatedTypes := {
      show
        core_models.ops.function.FnMut.AssociatedTypes
        P
        (rust_primitives.hax.Tuple1 u8)
      by infer_instance
      with sorry})] :
  Trait P
  where

end new_tests.legacy__traits__lib.for_clauses.issue_495.minimized_3


namespace new_tests.legacy__traits__lib.unconstrainted_types_issue_677

class PolyOp.AssociatedTypes (Self : Type) where

class PolyOp (Self : Type)
  [associatedTypes : outParam (PolyOp.AssociatedTypes (Self : Type))]
  where
  op (Self) : (u32 -> u32 -> RustM u32)

structure Plus where
  -- no fields

@[spec]
def Impl.op_hoisted (x : u32) (y : u32) : RustM u32 := do (x +? y)

@[reducible] instance Impl.AssociatedTypes : PolyOp.AssociatedTypes Plus where

instance Impl : PolyOp Plus where
  op := (Impl.op_hoisted)

structure Times where
  -- no fields

@[spec]
def Impl_1.op_hoisted (x : u32) (y : u32) : RustM u32 := do (x *? y)

@[reducible] instance Impl_1.AssociatedTypes :
  PolyOp.AssociatedTypes Times
  where

instance Impl_1 : PolyOp Times where
  op := (Impl_1.op_hoisted)

--  @fail(extraction): proverif(HAX0001)
@[spec]
def twice
    (OP : Type)
    [trait_constr_twice_associated_type_i0 : PolyOp.AssociatedTypes OP]
    [trait_constr_twice_i0 : PolyOp OP ]
    (x : u32) :
    RustM u32 := do
  (PolyOp.op OP x x)

@[spec]
def both (x : u32) : RustM (rust_primitives.hax.Tuple2 u32 u32) := do
  (pure (rust_primitives.hax.Tuple2.mk (← (twice Plus x)) (← (twice Times x))))

end new_tests.legacy__traits__lib.unconstrainted_types_issue_677


namespace new_tests.legacy__traits__lib.implicit_dependencies_issue_667.trait_definition

class MyTrait.AssociatedTypes (Self : Type) where

class MyTrait (Self : Type)
  [associatedTypes : outParam (MyTrait.AssociatedTypes (Self : Type))]
  where
  my_method (Self) : (Self -> RustM rust_primitives.hax.Tuple0)

end new_tests.legacy__traits__lib.implicit_dependencies_issue_667.trait_definition


namespace new_tests.legacy__traits__lib.implicit_dependencies_issue_667.define_type

structure MyType where
  -- no fields

end new_tests.legacy__traits__lib.implicit_dependencies_issue_667.define_type


namespace new_tests.legacy__traits__lib.implicit_dependencies_issue_667.impl_type

@[spec]
def Impl.my_method_hoisted
    (self :
    new_tests.legacy__traits__lib.implicit_dependencies_issue_667.define_type.MyType)
    :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

@[reducible] instance Impl.AssociatedTypes :
  new_tests.legacy__traits__lib.implicit_dependencies_issue_667.trait_definition.MyTrait.AssociatedTypes
  new_tests.legacy__traits__lib.implicit_dependencies_issue_667.define_type.MyType
  where

instance Impl :
  new_tests.legacy__traits__lib.implicit_dependencies_issue_667.trait_definition.MyTrait
  new_tests.legacy__traits__lib.implicit_dependencies_issue_667.define_type.MyType
  where
  my_method := (Impl.my_method_hoisted)

end new_tests.legacy__traits__lib.implicit_dependencies_issue_667.impl_type


namespace new_tests.legacy__traits__lib.implicit_dependencies_issue_667.use_type

@[spec]
def some_function
    (x :
    new_tests.legacy__traits__lib.implicit_dependencies_issue_667.define_type.MyType)
    :
    RustM rust_primitives.hax.Tuple0 := do
  (new_tests.legacy__traits__lib.implicit_dependencies_issue_667.trait_definition.MyTrait.my_method
    new_tests.legacy__traits__lib.implicit_dependencies_issue_667.define_type.MyType
    x)

end new_tests.legacy__traits__lib.implicit_dependencies_issue_667.use_type


namespace new_tests.legacy__traits__lib.interlaced_consts_types

structure Bar (FooConst : usize) (FooType : Type) where
  _0 : (RustArray FooType FooConst)

class Foo.AssociatedTypes (Self : Type) (FooConst : usize) (FooType : Type)
  where

class Foo (Self : Type) (FooConst : usize) (FooType : Type)
  [associatedTypes : outParam (Foo.AssociatedTypes (Self : Type) (FooConst :
      usize) (FooType : Type))]
  where
  _fun (Self) (FooConst) (FooType) (FunConst : usize) (FunType : Type) :
    ((RustArray FooType FooConst) ->
    (RustArray FunType FunConst) ->
    RustM rust_primitives.hax.Tuple0)

@[spec]
def Impl.fun_hoisted
    (FooConst : usize)
    (FooType : Type)
    (SelfType : Type)
    (FunConst : usize)
    (FunType : Type)
    (x : (RustArray FooType FooConst))
    (y : (RustArray FunType FunConst)) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

@[reducible] instance Impl.AssociatedTypes
  (FooConst : usize)
  (FooType : Type)
  (SelfType : Type) :
  Foo.AssociatedTypes SelfType (FooConst) FooType
  where

instance Impl (FooConst : usize) (FooType : Type) (SelfType : Type) :
  Foo SelfType (FooConst) FooType
  where
  _fun := fun  (FunConst : usize) (FunType : Type) =>
    (Impl.fun_hoisted (FooConst) FooType SelfType (FunConst) FunType)

end new_tests.legacy__traits__lib.interlaced_consts_types


namespace new_tests.legacy__traits__lib.implicit_explicit_calling_conventions

structure _Type (TypeArg : Type) (ConstArg : usize) where
  field : (RustArray TypeArg ConstArg)

class Trait.AssociatedTypes (Self : Type) (TypeArg : Type) (ConstArg : usize)
  where

class Trait (Self : Type) (TypeArg : Type) (ConstArg : usize)
  [associatedTypes : outParam (Trait.AssociatedTypes (Self : Type) (TypeArg :
      Type) (ConstArg : usize))]
  where
  method (Self) (TypeArg) (ConstArg)
    (MethodTypeArg : Type)
    (MethodConstArg : usize) :
    (Self ->
    TypeArg ->
    (_Type TypeArg (ConstArg)) ->
    RustM rust_primitives.hax.Tuple0)
  associated_function (Self) (TypeArg) (ConstArg)
    (MethodTypeArg : Type)
    (MethodConstArg : usize) :
    (Self ->
    TypeArg ->
    (_Type TypeArg (ConstArg)) ->
    RustM rust_primitives.hax.Tuple0)

@[spec]
def Impl.method_hoisted
    (TypeArg : Type)
    (ConstArg : usize)
    (MethodTypeArg : Type)
    (MethodConstArg : usize)
    (self : rust_primitives.hax.Tuple0)
    (value_TypeArg : TypeArg)
    (value_Type : (_Type TypeArg (ConstArg))) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def Impl.associated_function_hoisted
    (TypeArg : Type)
    (ConstArg : usize)
    (MethodTypeArg : Type)
    (MethodConstArg : usize)
    (_self : rust_primitives.hax.Tuple0)
    (value_TypeArg : TypeArg)
    (value_Type : (_Type TypeArg (ConstArg))) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

@[reducible] instance Impl.AssociatedTypes (TypeArg : Type) (ConstArg : usize) :
  Trait.AssociatedTypes rust_primitives.hax.Tuple0 TypeArg (ConstArg)
  where

instance Impl (TypeArg : Type) (ConstArg : usize) :
  Trait rust_primitives.hax.Tuple0 TypeArg (ConstArg)
  where
  method := fun  (MethodTypeArg : Type) (MethodConstArg : usize) =>
    (Impl.method_hoisted TypeArg (ConstArg) MethodTypeArg (MethodConstArg))
  associated_function := fun  (MethodTypeArg : Type) (MethodConstArg : usize) =>
    (Impl.associated_function_hoisted
      TypeArg
      (ConstArg)
      MethodTypeArg
      (MethodConstArg))

--  @fail(extraction): proverif(HAX0001)
@[spec]
def method_caller
    (MethodTypeArg : Type)
    (TypeArg : Type)
    (ConstArg : usize)
    (MethodConstArg : usize)
    (ImplTrait : Type)
    [trait_constr_method_caller_associated_type_i0 : Trait.AssociatedTypes
      ImplTrait
      TypeArg
      (ConstArg)]
    [trait_constr_method_caller_i0 : Trait ImplTrait TypeArg (ConstArg) ]
    (x : ImplTrait)
    (value_TypeArg : TypeArg)
    (value_Type : (_Type TypeArg (ConstArg))) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ ←
    (Trait.method
      ImplTrait
      TypeArg
      (ConstArg) MethodTypeArg (MethodConstArg) x value_TypeArg value_Type);
  (pure rust_primitives.hax.Tuple0.mk)

--  @fail(extraction): proverif(HAX0001)
@[spec]
def associated_function_caller
    (MethodTypeArg : Type)
    (TypeArg : Type)
    (ConstArg : usize)
    (MethodConstArg : usize)
    (ImplTrait : Type)
    [trait_constr_associated_function_caller_associated_type_i0 :
      Trait.AssociatedTypes
      ImplTrait
      TypeArg
      (ConstArg)]
    [trait_constr_associated_function_caller_i0 : Trait
      ImplTrait
      TypeArg
      (ConstArg)
      ]
    (x : ImplTrait)
    (value_TypeArg : TypeArg)
    (value_Type : (_Type TypeArg (ConstArg))) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ ←
    (Trait.associated_function
      ImplTrait
      TypeArg
      (ConstArg) MethodTypeArg (MethodConstArg) x value_TypeArg value_Type);
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.legacy__traits__lib.implicit_explicit_calling_conventions


namespace new_tests.legacy__traits__lib.type_alias_bounds_issue_707

structure StructWithGenericBounds
  (T : Type)
  [trait_constr_StructWithGenericBounds_associated_type_i0 :
    core_models.clone.Clone.AssociatedTypes
    T]
  [trait_constr_StructWithGenericBounds_i0 : core_models.clone.Clone T ]
  where
  _0 : T

abbrev SynonymA (T : Type) : Type := (StructWithGenericBounds T)

abbrev SynonymB (T : Type) :
  Type :=
  (StructWithGenericBounds (rust_primitives.hax.Tuple2 T T))

end new_tests.legacy__traits__lib.type_alias_bounds_issue_707


namespace new_tests.legacy__traits__lib.block_size

class BlockSizeUser.AssociatedTypes (Self : Type) where
  BlockSize : Type

attribute [reducible] BlockSizeUser.AssociatedTypes.BlockSize

abbrev BlockSizeUser.BlockSize :=
  BlockSizeUser.AssociatedTypes.BlockSize

class BlockSizeUser (Self : Type)
  [associatedTypes : outParam (BlockSizeUser.AssociatedTypes (Self : Type))]
  where

class ParBlocksSizeUser.AssociatedTypes (Self : Type) where
  [trait_constr_ParBlocksSizeUser_i0 : BlockSizeUser.AssociatedTypes Self]

attribute [instance_reducible, instance]
  ParBlocksSizeUser.AssociatedTypes.trait_constr_ParBlocksSizeUser_i0

class ParBlocksSizeUser (Self : Type)
  [associatedTypes : outParam (ParBlocksSizeUser.AssociatedTypes (Self : Type))]
  where
  [trait_constr_ParBlocksSizeUser_i0 : BlockSizeUser Self]

attribute [instance_reducible, instance]
  ParBlocksSizeUser.trait_constr_ParBlocksSizeUser_i0

class BlockBackend.AssociatedTypes (Self : Type) where
  [trait_constr_BlockBackend_i0 : ParBlocksSizeUser.AssociatedTypes Self]

attribute [instance_reducible, instance]
  BlockBackend.AssociatedTypes.trait_constr_BlockBackend_i0

class BlockBackend (Self : Type)
  [associatedTypes : outParam (BlockBackend.AssociatedTypes (Self : Type))]
  where
  [trait_constr_BlockBackend_i0 : ParBlocksSizeUser Self]
  proc_block (Self) :
    ((alloc.vec.Vec (BlockSizeUser.BlockSize Self) alloc.alloc.Global) ->
    RustM rust_primitives.hax.Tuple0)

attribute [instance_reducible, instance]
  BlockBackend.trait_constr_BlockBackend_i0

end new_tests.legacy__traits__lib.block_size


namespace new_tests.legacy__traits__lib.default_traits_parameters

class Bar.AssociatedTypes (Self : Type) (T : Type) where

class Bar (Self : Type) (T : Type)
  [associatedTypes : outParam (Bar.AssociatedTypes (Self : Type) (T : Type))]
  where

end new_tests.legacy__traits__lib.default_traits_parameters


namespace new_tests.legacy__traits__lib.impl_expr_in_goal

class T1.AssociatedTypes (Self : Type) where
  Assoc : Type

attribute [reducible] T1.AssociatedTypes.Assoc

abbrev T1.Assoc :=
  T1.AssociatedTypes.Assoc

class T1 (Self : Type)
  [associatedTypes : outParam (T1.AssociatedTypes (Self : Type))]
  where

class T2.AssociatedTypes (Self : Type) where

class T2 (Self : Type)
  [associatedTypes : outParam (T2.AssociatedTypes (Self : Type))]
  where

@[reducible] instance Impl.AssociatedTypes
  (U : Type)
  [trait_constr_Impl_associated_type_i0 : T1.AssociatedTypes U]
  [trait_constr_Impl_i0 : T1 U ]
  [trait_constr_Impl_associated_type_i1 : T2.AssociatedTypes (T1.Assoc U)]
  [trait_constr_Impl_i1 : T2 (T1.Assoc U) ] :
  T2.AssociatedTypes U
  where

instance Impl
  (U : Type)
  [trait_constr_Impl_associated_type_i0 : T1.AssociatedTypes U]
  [trait_constr_Impl_i0 : T1 U ]
  [trait_constr_Impl_associated_type_i1 : T2.AssociatedTypes (T1.Assoc U)]
  [trait_constr_Impl_i1 : T2 (T1.Assoc U) ] :
  T2 U
  where

end new_tests.legacy__traits__lib.impl_expr_in_goal


namespace new_tests.legacy__traits__lib.implement_arithmetic_trait

structure Wrapped where
  _0 : i32

@[spec]
def Impl.add_hoisted (self : Wrapped) (rhs : Wrapped) : RustM Wrapped := do
  (pure (Wrapped.mk (← ((Wrapped._0 self) +? (Wrapped._0 rhs)))))

@[reducible] instance Impl.AssociatedTypes :
  core_models.ops.arith.Add.AssociatedTypes Wrapped Wrapped
  where
  Output := Wrapped

instance Impl : core_models.ops.arith.Add Wrapped Wrapped where
  add := (Impl.add_hoisted)

@[spec]
def test (x : Wrapped) (y : Wrapped) : RustM Wrapped := do
  (core_models.ops.arith.Add.add Wrapped Wrapped x y)

end new_tests.legacy__traits__lib.implement_arithmetic_trait


namespace new_tests.legacy__traits__lib.typenum_perf

abbrev I20 :
  Type :=
  (typenum.uint.UInt
    (typenum.uint.UInt
      (typenum.uint.UInt
        (typenum.uint.UInt
          (typenum.uint.UInt
            (typenum.uint.UInt
              (typenum.uint.UInt
                (typenum.uint.UInt
                  (typenum.uint.UInt
                    (typenum.uint.UInt
                      (typenum.uint.UInt
                        (typenum.uint.UInt
                          (typenum.uint.UInt
                            (typenum.uint.UInt
                              (typenum.uint.UInt
                                (typenum.uint.UInt
                                  (typenum.uint.UInt
                                    (typenum.uint.UInt
                                      (typenum.uint.UInt
                                        (typenum.uint.UInt
                                          typenum.uint.UTerm
                                          typenum.bit.B1)
                                        typenum.bit.B1)
                                      typenum.bit.B1)
                                    typenum.bit.B1)
                                  typenum.bit.B1)
                                typenum.bit.B1)
                              typenum.bit.B1)
                            typenum.bit.B1)
                          typenum.bit.B1)
                        typenum.bit.B1)
                      typenum.bit.B1)
                    typenum.bit.B1)
                  typenum.bit.B1)
                typenum.bit.B1)
              typenum.bit.B1)
            typenum.bit.B1)
          typenum.bit.B1)
        typenum.bit.B1)
      typenum.bit.B1)
    typenum.bit.B1)

abbrev I19 :
  Type :=
  (typenum.uint.UInt
    (typenum.uint.UInt
      (typenum.uint.UInt
        (typenum.uint.UInt
          (typenum.uint.UInt
            (typenum.uint.UInt
              (typenum.uint.UInt
                (typenum.uint.UInt
                  (typenum.uint.UInt
                    (typenum.uint.UInt
                      (typenum.uint.UInt
                        (typenum.uint.UInt
                          (typenum.uint.UInt
                            (typenum.uint.UInt
                              (typenum.uint.UInt
                                (typenum.uint.UInt
                                  (typenum.uint.UInt
                                    (typenum.uint.UInt
                                      (typenum.uint.UInt
                                        typenum.uint.UTerm
                                        typenum.bit.B1)
                                      typenum.bit.B1)
                                    typenum.bit.B1)
                                  typenum.bit.B1)
                                typenum.bit.B1)
                              typenum.bit.B1)
                            typenum.bit.B1)
                          typenum.bit.B1)
                        typenum.bit.B1)
                      typenum.bit.B1)
                    typenum.bit.B1)
                  typenum.bit.B1)
                typenum.bit.B1)
              typenum.bit.B1)
            typenum.bit.B1)
          typenum.bit.B1)
        typenum.bit.B1)
      typenum.bit.B1)
    typenum.bit.B1)

abbrev I18 :
  Type :=
  (typenum.uint.UInt
    (typenum.uint.UInt
      (typenum.uint.UInt
        (typenum.uint.UInt
          (typenum.uint.UInt
            (typenum.uint.UInt
              (typenum.uint.UInt
                (typenum.uint.UInt
                  (typenum.uint.UInt
                    (typenum.uint.UInt
                      (typenum.uint.UInt
                        (typenum.uint.UInt
                          (typenum.uint.UInt
                            (typenum.uint.UInt
                              (typenum.uint.UInt
                                (typenum.uint.UInt
                                  (typenum.uint.UInt
                                    (typenum.uint.UInt
                                      typenum.uint.UTerm
                                      typenum.bit.B1)
                                    typenum.bit.B1)
                                  typenum.bit.B1)
                                typenum.bit.B1)
                              typenum.bit.B1)
                            typenum.bit.B1)
                          typenum.bit.B1)
                        typenum.bit.B1)
                      typenum.bit.B1)
                    typenum.bit.B1)
                  typenum.bit.B1)
                typenum.bit.B1)
              typenum.bit.B1)
            typenum.bit.B1)
          typenum.bit.B1)
        typenum.bit.B1)
      typenum.bit.B1)
    typenum.bit.B1)

abbrev I17 :
  Type :=
  (typenum.uint.UInt
    (typenum.uint.UInt
      (typenum.uint.UInt
        (typenum.uint.UInt
          (typenum.uint.UInt
            (typenum.uint.UInt
              (typenum.uint.UInt
                (typenum.uint.UInt
                  (typenum.uint.UInt
                    (typenum.uint.UInt
                      (typenum.uint.UInt
                        (typenum.uint.UInt
                          (typenum.uint.UInt
                            (typenum.uint.UInt
                              (typenum.uint.UInt
                                (typenum.uint.UInt
                                  (typenum.uint.UInt
                                    typenum.uint.UTerm
                                    typenum.bit.B1)
                                  typenum.bit.B1)
                                typenum.bit.B1)
                              typenum.bit.B1)
                            typenum.bit.B1)
                          typenum.bit.B1)
                        typenum.bit.B1)
                      typenum.bit.B1)
                    typenum.bit.B1)
                  typenum.bit.B1)
                typenum.bit.B1)
              typenum.bit.B1)
            typenum.bit.B1)
          typenum.bit.B1)
        typenum.bit.B1)
      typenum.bit.B1)
    typenum.bit.B1)

abbrev I16 :
  Type :=
  (typenum.uint.UInt
    (typenum.uint.UInt
      (typenum.uint.UInt
        (typenum.uint.UInt
          (typenum.uint.UInt
            (typenum.uint.UInt
              (typenum.uint.UInt
                (typenum.uint.UInt
                  (typenum.uint.UInt
                    (typenum.uint.UInt
                      (typenum.uint.UInt
                        (typenum.uint.UInt
                          (typenum.uint.UInt
                            (typenum.uint.UInt
                              (typenum.uint.UInt
                                (typenum.uint.UInt
                                  typenum.uint.UTerm
                                  typenum.bit.B1)
                                typenum.bit.B1)
                              typenum.bit.B1)
                            typenum.bit.B1)
                          typenum.bit.B1)
                        typenum.bit.B1)
                      typenum.bit.B1)
                    typenum.bit.B1)
                  typenum.bit.B1)
                typenum.bit.B1)
              typenum.bit.B1)
            typenum.bit.B1)
          typenum.bit.B1)
        typenum.bit.B1)
      typenum.bit.B1)
    typenum.bit.B1)

abbrev I15 :
  Type :=
  (typenum.uint.UInt
    (typenum.uint.UInt
      (typenum.uint.UInt
        (typenum.uint.UInt
          (typenum.uint.UInt
            (typenum.uint.UInt
              (typenum.uint.UInt
                (typenum.uint.UInt
                  (typenum.uint.UInt
                    (typenum.uint.UInt
                      (typenum.uint.UInt
                        (typenum.uint.UInt
                          (typenum.uint.UInt
                            (typenum.uint.UInt
                              (typenum.uint.UInt
                                typenum.uint.UTerm
                                typenum.bit.B1)
                              typenum.bit.B1)
                            typenum.bit.B1)
                          typenum.bit.B1)
                        typenum.bit.B1)
                      typenum.bit.B1)
                    typenum.bit.B1)
                  typenum.bit.B1)
                typenum.bit.B1)
              typenum.bit.B1)
            typenum.bit.B1)
          typenum.bit.B1)
        typenum.bit.B1)
      typenum.bit.B1)
    typenum.bit.B1)

abbrev I14 :
  Type :=
  (typenum.uint.UInt
    (typenum.uint.UInt
      (typenum.uint.UInt
        (typenum.uint.UInt
          (typenum.uint.UInt
            (typenum.uint.UInt
              (typenum.uint.UInt
                (typenum.uint.UInt
                  (typenum.uint.UInt
                    (typenum.uint.UInt
                      (typenum.uint.UInt
                        (typenum.uint.UInt
                          (typenum.uint.UInt
                            (typenum.uint.UInt
                              typenum.uint.UTerm
                              typenum.bit.B1)
                            typenum.bit.B1)
                          typenum.bit.B1)
                        typenum.bit.B1)
                      typenum.bit.B1)
                    typenum.bit.B1)
                  typenum.bit.B1)
                typenum.bit.B1)
              typenum.bit.B1)
            typenum.bit.B1)
          typenum.bit.B1)
        typenum.bit.B1)
      typenum.bit.B1)
    typenum.bit.B1)

abbrev I13 :
  Type :=
  (typenum.uint.UInt
    (typenum.uint.UInt
      (typenum.uint.UInt
        (typenum.uint.UInt
          (typenum.uint.UInt
            (typenum.uint.UInt
              (typenum.uint.UInt
                (typenum.uint.UInt
                  (typenum.uint.UInt
                    (typenum.uint.UInt
                      (typenum.uint.UInt
                        (typenum.uint.UInt
                          (typenum.uint.UInt typenum.uint.UTerm typenum.bit.B1)
                          typenum.bit.B1)
                        typenum.bit.B1)
                      typenum.bit.B1)
                    typenum.bit.B1)
                  typenum.bit.B1)
                typenum.bit.B1)
              typenum.bit.B1)
            typenum.bit.B1)
          typenum.bit.B1)
        typenum.bit.B1)
      typenum.bit.B1)
    typenum.bit.B1)

abbrev I12 :
  Type :=
  (typenum.uint.UInt
    (typenum.uint.UInt
      (typenum.uint.UInt
        (typenum.uint.UInt
          (typenum.uint.UInt
            (typenum.uint.UInt
              (typenum.uint.UInt
                (typenum.uint.UInt
                  (typenum.uint.UInt
                    (typenum.uint.UInt
                      (typenum.uint.UInt
                        (typenum.uint.UInt typenum.uint.UTerm typenum.bit.B1)
                        typenum.bit.B1)
                      typenum.bit.B1)
                    typenum.bit.B1)
                  typenum.bit.B1)
                typenum.bit.B1)
              typenum.bit.B1)
            typenum.bit.B1)
          typenum.bit.B1)
        typenum.bit.B1)
      typenum.bit.B1)
    typenum.bit.B1)

abbrev I11 :
  Type :=
  (typenum.uint.UInt
    (typenum.uint.UInt
      (typenum.uint.UInt
        (typenum.uint.UInt
          (typenum.uint.UInt
            (typenum.uint.UInt
              (typenum.uint.UInt
                (typenum.uint.UInt
                  (typenum.uint.UInt
                    (typenum.uint.UInt
                      (typenum.uint.UInt typenum.uint.UTerm typenum.bit.B1)
                      typenum.bit.B1)
                    typenum.bit.B1)
                  typenum.bit.B1)
                typenum.bit.B1)
              typenum.bit.B1)
            typenum.bit.B1)
          typenum.bit.B1)
        typenum.bit.B1)
      typenum.bit.B1)
    typenum.bit.B1)

abbrev I10 :
  Type :=
  (typenum.uint.UInt
    (typenum.uint.UInt
      (typenum.uint.UInt
        (typenum.uint.UInt
          (typenum.uint.UInt
            (typenum.uint.UInt
              (typenum.uint.UInt
                (typenum.uint.UInt
                  (typenum.uint.UInt
                    (typenum.uint.UInt typenum.uint.UTerm typenum.bit.B1)
                    typenum.bit.B1)
                  typenum.bit.B1)
                typenum.bit.B1)
              typenum.bit.B1)
            typenum.bit.B1)
          typenum.bit.B1)
        typenum.bit.B1)
      typenum.bit.B1)
    typenum.bit.B1)

abbrev I9 :
  Type :=
  (typenum.uint.UInt
    (typenum.uint.UInt
      (typenum.uint.UInt
        (typenum.uint.UInt
          (typenum.uint.UInt
            (typenum.uint.UInt
              (typenum.uint.UInt
                (typenum.uint.UInt
                  (typenum.uint.UInt typenum.uint.UTerm typenum.bit.B1)
                  typenum.bit.B1)
                typenum.bit.B1)
              typenum.bit.B1)
            typenum.bit.B1)
          typenum.bit.B1)
        typenum.bit.B1)
      typenum.bit.B1)
    typenum.bit.B1)

abbrev I8 :
  Type :=
  (typenum.uint.UInt
    (typenum.uint.UInt
      (typenum.uint.UInt
        (typenum.uint.UInt
          (typenum.uint.UInt
            (typenum.uint.UInt
              (typenum.uint.UInt
                (typenum.uint.UInt typenum.uint.UTerm typenum.bit.B1)
                typenum.bit.B1)
              typenum.bit.B1)
            typenum.bit.B1)
          typenum.bit.B1)
        typenum.bit.B1)
      typenum.bit.B1)
    typenum.bit.B1)

abbrev I7 :
  Type :=
  (typenum.uint.UInt
    (typenum.uint.UInt
      (typenum.uint.UInt
        (typenum.uint.UInt
          (typenum.uint.UInt
            (typenum.uint.UInt
              (typenum.uint.UInt typenum.uint.UTerm typenum.bit.B1)
              typenum.bit.B1)
            typenum.bit.B1)
          typenum.bit.B1)
        typenum.bit.B1)
      typenum.bit.B1)
    typenum.bit.B1)

abbrev I6 :
  Type :=
  (typenum.uint.UInt
    (typenum.uint.UInt
      (typenum.uint.UInt
        (typenum.uint.UInt
          (typenum.uint.UInt
            (typenum.uint.UInt typenum.uint.UTerm typenum.bit.B1)
            typenum.bit.B1)
          typenum.bit.B1)
        typenum.bit.B1)
      typenum.bit.B1)
    typenum.bit.B1)

abbrev I5 :
  Type :=
  (typenum.uint.UInt
    (typenum.uint.UInt
      (typenum.uint.UInt
        (typenum.uint.UInt
          (typenum.uint.UInt typenum.uint.UTerm typenum.bit.B1)
          typenum.bit.B1)
        typenum.bit.B1)
      typenum.bit.B1)
    typenum.bit.B1)

abbrev I4 :
  Type :=
  (typenum.uint.UInt
    (typenum.uint.UInt
      (typenum.uint.UInt
        (typenum.uint.UInt typenum.uint.UTerm typenum.bit.B1)
        typenum.bit.B1)
      typenum.bit.B1)
    typenum.bit.B1)

abbrev I3 :
  Type :=
  (typenum.uint.UInt
    (typenum.uint.UInt
      (typenum.uint.UInt typenum.uint.UTerm typenum.bit.B1)
      typenum.bit.B1)
    typenum.bit.B1)

abbrev I2 :
  Type :=
  (typenum.uint.UInt
    (typenum.uint.UInt typenum.uint.UTerm typenum.bit.B1)
    typenum.bit.B1)

abbrev I1 : Type := (typenum.uint.UInt typenum.uint.UTerm typenum.bit.B1)

abbrev I0 : Type := typenum.uint.UTerm

@[spec]
def _f
    (T : Type)
    [trait_constr__f_associated_type_i0 :
      typenum.type_operators.IsLess.AssociatedTypes
      T
      (typenum.uint.UInt
        (typenum.uint.UInt
          (typenum.uint.UInt
            (typenum.uint.UInt
              (typenum.uint.UInt
                (typenum.uint.UInt
                  (typenum.uint.UInt
                    (typenum.uint.UInt
                      (typenum.uint.UInt
                        (typenum.uint.UInt
                          (typenum.uint.UInt
                            (typenum.uint.UInt
                              (typenum.uint.UInt
                                (typenum.uint.UInt
                                  (typenum.uint.UInt
                                    (typenum.uint.UInt
                                      (typenum.uint.UInt
                                        (typenum.uint.UInt
                                          (typenum.uint.UInt
                                            (typenum.uint.UInt
                                              typenum.uint.UTerm
                                              typenum.bit.B1)
                                            typenum.bit.B1)
                                          typenum.bit.B1)
                                        typenum.bit.B1)
                                      typenum.bit.B1)
                                    typenum.bit.B1)
                                  typenum.bit.B1)
                                typenum.bit.B1)
                              typenum.bit.B1)
                            typenum.bit.B1)
                          typenum.bit.B1)
                        typenum.bit.B1)
                      typenum.bit.B1)
                    typenum.bit.B1)
                  typenum.bit.B1)
                typenum.bit.B1)
              typenum.bit.B1)
            typenum.bit.B1)
          typenum.bit.B1)
        typenum.bit.B1)]
    [trait_constr__f_i0 : typenum.type_operators.IsLess
      T
      (typenum.uint.UInt
        (typenum.uint.UInt
          (typenum.uint.UInt
            (typenum.uint.UInt
              (typenum.uint.UInt
                (typenum.uint.UInt
                  (typenum.uint.UInt
                    (typenum.uint.UInt
                      (typenum.uint.UInt
                        (typenum.uint.UInt
                          (typenum.uint.UInt
                            (typenum.uint.UInt
                              (typenum.uint.UInt
                                (typenum.uint.UInt
                                  (typenum.uint.UInt
                                    (typenum.uint.UInt
                                      (typenum.uint.UInt
                                        (typenum.uint.UInt
                                          (typenum.uint.UInt
                                            (typenum.uint.UInt
                                              typenum.uint.UTerm
                                              typenum.bit.B1)
                                            typenum.bit.B1)
                                          typenum.bit.B1)
                                        typenum.bit.B1)
                                      typenum.bit.B1)
                                    typenum.bit.B1)
                                  typenum.bit.B1)
                                typenum.bit.B1)
                              typenum.bit.B1)
                            typenum.bit.B1)
                          typenum.bit.B1)
                        typenum.bit.B1)
                      typenum.bit.B1)
                    typenum.bit.B1)
                  typenum.bit.B1)
                typenum.bit.B1)
              typenum.bit.B1)
            typenum.bit.B1)
          typenum.bit.B1)
        typenum.bit.B1)
      ]
    (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.legacy__traits__lib.typenum_perf


namespace new_tests.legacy__traits__lib.gat_bounds_issue_1907

class Bound.AssociatedTypes (Self : Type) where

class Bound (Self : Type)
  [associatedTypes : outParam (Bound.AssociatedTypes (Self : Type))]
  where

@[reducible] instance Impl.AssociatedTypes : Bound.AssociatedTypes u8 where

instance Impl : Bound u8 where

@[reducible] instance Impl_1.AssociatedTypes
  (A : Type)
  [trait_constr_Impl_1_associated_type_i0 : Bound.AssociatedTypes A]
  [trait_constr_Impl_1_i0 : Bound A ] :
  Bound.AssociatedTypes (rust_primitives.hax.Tuple2 u8 A)
  where

instance Impl_1
  (A : Type)
  [trait_constr_Impl_1_associated_type_i0 : Bound.AssociatedTypes A]
  [trait_constr_Impl_1_i0 : Bound A ] :
  Bound (rust_primitives.hax.Tuple2 u8 A)
  where

structure S where
  -- no fields

end new_tests.legacy__traits__lib.gat_bounds_issue_1907


namespace new_tests.legacy__traits__lib

class Foo.AssociatedTypes (Self : Type) where
  AssocType : Type

attribute [reducible] Foo.AssociatedTypes.AssocType

abbrev Foo.AssocType :=
  Foo.AssociatedTypes.AssocType

class Foo (Self : Type)
  [associatedTypes : outParam (Foo.AssociatedTypes (Self : Type))]
  where
  [trait_constr_AssocType_associated_type_i1 : SuperTrait.AssociatedTypes
    associatedTypes.AssocType]
  [trait_constr_AssocType_i1 : SuperTrait associatedTypes.AssocType ]
  N (Self) : usize
  assoc_f (Self) :
    (rust_primitives.hax.Tuple0 -> RustM rust_primitives.hax.Tuple0)
  method_f (Self) : (Self -> RustM rust_primitives.hax.Tuple0)
  assoc_type (Self)
    [trait_constr_assoc_type_associated_type_i1 :
      core_models.marker.Copy.AssociatedTypes
      associatedTypes.AssocType]
    [trait_constr_assoc_type_i1 : core_models.marker.Copy
      associatedTypes.AssocType
      ] :
    (associatedTypes.AssocType -> RustM rust_primitives.hax.Tuple0)

class Lang.AssociatedTypes (Self : Type) where
  Var : Type

attribute [reducible] Lang.AssociatedTypes.Var

abbrev Lang.Var :=
  Lang.AssociatedTypes.Var

class Lang (Self : Type)
  [associatedTypes : outParam (Lang.AssociatedTypes (Self : Type))]
  where
  s (Self) :
    (Self -> i32 -> RustM (rust_primitives.hax.Tuple2 Self associatedTypes.Var))

end new_tests.legacy__traits__lib


namespace new_tests.legacy__traits__lib.implicit_explicit_calling_conventions

class SubTrait.AssociatedTypes (Self : Type) (TypeArg : Type) (ConstArg : usize)
  where
  [trait_constr_SubTrait_i0 : Trait.AssociatedTypes Self TypeArg (ConstArg)]
  AssocType : Type

attribute [instance_reducible, instance]
  SubTrait.AssociatedTypes.trait_constr_SubTrait_i0

attribute [reducible] SubTrait.AssociatedTypes.AssocType

abbrev SubTrait.AssocType :=
  SubTrait.AssociatedTypes.AssocType

class SubTrait (Self : Type) (TypeArg : Type) (ConstArg : usize)
  [associatedTypes : outParam (SubTrait.AssociatedTypes (Self : Type) (TypeArg :
      Type) (ConstArg : usize))]
  where
  [trait_constr_SubTrait_i0 : Trait Self TypeArg (ConstArg)]
  [trait_constr_AssocType_associated_type_i1 : Trait.AssociatedTypes
    associatedTypes.AssocType
    TypeArg
    (ConstArg)]
  [trait_constr_AssocType_i1 : Trait
    associatedTypes.AssocType
    TypeArg
    (ConstArg)
    ]

attribute [instance_reducible, instance] SubTrait.trait_constr_SubTrait_i0

end new_tests.legacy__traits__lib.implicit_explicit_calling_conventions


namespace new_tests.legacy__traits__lib.recursive_trait_with_assoc_type

class Trait1.AssociatedTypes (Self : Type) where
  T : Type

attribute [reducible] Trait1.AssociatedTypes.T

abbrev Trait1.T :=
  Trait1.AssociatedTypes.T

class Trait1 (Self : Type)
  [associatedTypes : outParam (Trait1.AssociatedTypes (Self : Type))]
  where
  [trait_constr_T_associated_type_i1 : Trait1.AssociatedTypes associatedTypes.T]
  [trait_constr_T_i1 : Trait1 associatedTypes.T ]

end new_tests.legacy__traits__lib.recursive_trait_with_assoc_type


namespace new_tests.legacy__traits__lib.default_traits_parameters

class Foo.AssociatedTypes (Self : Type) where
  [trait_constr_Foo_i0 : Bar.AssociatedTypes Self associatedTypes.U]
  U : Type

attribute [instance_reducible, instance] Foo.AssociatedTypes.trait_constr_Foo_i0

attribute [reducible] Foo.AssociatedTypes.U

abbrev Foo.U :=
  Foo.AssociatedTypes.U

class Foo (Self : Type)
  [associatedTypes : outParam (Foo.AssociatedTypes (Self : Type))]
  where
  [trait_constr_Foo_i0 : Bar Self associatedTypes.U]

attribute [instance_reducible, instance] Foo.trait_constr_Foo_i0

end new_tests.legacy__traits__lib.default_traits_parameters


namespace new_tests.legacy__traits__lib.gat_bounds_issue_1907

class WithGat.AssociatedTypes (Self : Type) where
  Assoc : Type

attribute [reducible] WithGat.AssociatedTypes.Assoc

abbrev WithGat.Assoc :=
  WithGat.AssociatedTypes.Assoc

class WithGat (Self : Type)
  [associatedTypes : outParam (WithGat.AssociatedTypes (Self : Type))]
  where
  (A : Type)
  [trait_constr_Assoc_associated_type_i1 : Bound.AssociatedTypes
    associatedTypes.Assoc]
  [trait_constr_Assoc_i1 : Bound associatedTypes.Assoc ]

end new_tests.legacy__traits__lib.gat_bounds_issue_1907


namespace new_tests.legacy__traits__lib

@[reducible] instance Impl_1.AssociatedTypes :
  Foo.AssociatedTypes rust_primitives.hax.Tuple0
  where
  AssocType := i32

instance Impl_1 : Foo rust_primitives.hax.Tuple0 where
  N := (Impl_1.N_hoisted)
  assoc_f := (Impl_1.assoc_f_hoisted)
  method_f := (Impl_1.method_f_hoisted)
  assoc_type := (Impl_1.assoc_type_hoisted)

--  @fail(extraction): proverif(HAX0001, HAX0001)
@[spec]
def f
    (T : Type)
    [trait_constr_f_associated_type_i0 : Foo.AssociatedTypes T]
    [trait_constr_f_i0 : Foo T ]
    (x : T) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ ← (Foo.assoc_f T rust_primitives.hax.Tuple0.mk);
  (Foo.method_f T x)

--  @fail(extraction): proverif(HAX0001)
@[spec]
def g
    (T : Type)
    [trait_constr_g_associated_type_i0 : Foo.AssociatedTypes T]
    [trait_constr_g_i0 : Foo T ]
    (x : (Foo.AssocType T)) :
    RustM u32 := do
  (SuperTrait.function_of_super_trait (Foo.AssocType T) x)

end new_tests.legacy__traits__lib


namespace new_tests.legacy__traits__lib.recursive_trait_with_assoc_type

class Trait2.AssociatedTypes (Self : Type) where
  [trait_constr_Trait2_i0 : Trait1.AssociatedTypes Self]
  U : Type

attribute [instance_reducible, instance]
  Trait2.AssociatedTypes.trait_constr_Trait2_i0

attribute [reducible] Trait2.AssociatedTypes.U

abbrev Trait2.U :=
  Trait2.AssociatedTypes.U

class Trait2 (Self : Type)
  [associatedTypes : outParam (Trait2.AssociatedTypes (Self : Type))]
  where
  [trait_constr_Trait2_i0 : Trait1 Self]

attribute [instance_reducible, instance] Trait2.trait_constr_Trait2_i0

end new_tests.legacy__traits__lib.recursive_trait_with_assoc_type


namespace new_tests.legacy__traits__lib.gat_bounds_issue_1907

@[reducible] instance Impl_2.AssociatedTypes : WithGat.AssociatedTypes S where
  Assoc := (rust_primitives.hax.Tuple2 u8 A)

instance Impl_2 : WithGat S where

end new_tests.legacy__traits__lib.gat_bounds_issue_1907

