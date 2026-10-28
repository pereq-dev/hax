
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


namespace new_tests.legacy__trait_param_bounds__lib

class Item.AssociatedTypes (Self : Type) where

class Item (Self : Type)
  [associatedTypes : outParam (Item.AssociatedTypes (Self : Type))]
  where
  get (Self) : (Self -> RustM u8)

class Other.AssociatedTypes (Self : Type) where

class Other (Self : Type)
  [associatedTypes : outParam (Other.AssociatedTypes (Self : Type))]
  where

@[spec]
def Impl.get_hoisted (self : u64) : RustM u8 := do (pure (0 : u8))

@[reducible] instance Impl.AssociatedTypes : Item.AssociatedTypes u64 where

instance Impl : Item u64 where
  get := (Impl.get_hoisted)

@[spec]
def Impl_1.get_hoisted (self : u16) : RustM u8 := do (pure (1 : u8))

@[reducible] instance Impl_1.AssociatedTypes : Item.AssociatedTypes u16 where

instance Impl_1 : Item u16 where
  get := (Impl_1.get_hoisted)

@[reducible] instance Impl_2.AssociatedTypes : Other.AssociatedTypes u32 where

instance Impl_2 : Other u32 where

--  Bound on a parameter.
class Squeeze.AssociatedTypes (Self : Type) (T : Type) where
  [trait_constr_Squeeze_i0 : Item.AssociatedTypes T]

attribute [instance_reducible, instance]
  Squeeze.AssociatedTypes.trait_constr_Squeeze_i0

class Squeeze (Self : Type) (T : Type)
  [associatedTypes : outParam (Squeeze.AssociatedTypes (Self : Type) (T :
      Type))]
  where
  [trait_constr_Squeeze_i0 : Item T]
  squeeze (Self) (T) : (Self -> T -> RustM u8)

attribute [instance_reducible, instance] Squeeze.trait_constr_Squeeze_i0

@[spec]
def Impl_3.squeeze_hoisted (self : u8) (x : u64) : RustM u8 := do
  (Item.get u64 x)

@[reducible] instance Impl_3.AssociatedTypes :
  Squeeze.AssociatedTypes u8 u64
  where

instance Impl_3 : Squeeze u8 u64 where
  squeeze := (Impl_3.squeeze_hoisted)

--  Generic impl.
structure W where
  -- no fields

@[spec]
def Impl_4.squeeze_hoisted
    (T : Type)
    [trait_constr_squeeze_hoisted_associated_type_i0 : Item.AssociatedTypes T]
    [trait_constr_squeeze_hoisted_i0 : Item T ]
    (self : W)
    (x : T) :
    RustM u8 := do
  (Item.get T x)

@[reducible] instance Impl_4.AssociatedTypes
  (T : Type)
  [trait_constr_Impl_4_associated_type_i0 : Item.AssociatedTypes T]
  [trait_constr_Impl_4_i0 : Item T ] :
  Squeeze.AssociatedTypes W T
  where

instance Impl_4
  (T : Type)
  [trait_constr_Impl_4_associated_type_i0 : Item.AssociatedTypes T]
  [trait_constr_Impl_4_i0 : Item T ] :
  Squeeze W T
  where
  squeeze := (Impl_4.squeeze_hoisted T)

--  Supertrait mixed with parameter bounds, inline and in a `where` clause.
class Mixed.AssociatedTypes (Self : Type) (A : Type) (B : Type) where
  [trait_constr_Mixed_i0 : Other.AssociatedTypes Self]
  [trait_constr_Mixed_i1 : Item.AssociatedTypes A]
  [trait_constr_Mixed_i2 : Item.AssociatedTypes B]
  [trait_constr_Mixed_i3 : core_models.clone.Clone.AssociatedTypes B]

attribute [instance_reducible, instance]
  Mixed.AssociatedTypes.trait_constr_Mixed_i0

attribute [instance_reducible, instance]
  Mixed.AssociatedTypes.trait_constr_Mixed_i1

attribute [instance_reducible, instance]
  Mixed.AssociatedTypes.trait_constr_Mixed_i2

attribute [instance_reducible, instance]
  Mixed.AssociatedTypes.trait_constr_Mixed_i3

class Mixed (Self : Type) (A : Type) (B : Type)
  [associatedTypes : outParam (Mixed.AssociatedTypes (Self : Type) (A : Type) (B
      : Type))]
  where
  [trait_constr_Mixed_i0 : Other Self]
  [trait_constr_Mixed_i1 : Item A]
  [trait_constr_Mixed_i2 : Item B]
  [trait_constr_Mixed_i3 : core_models.clone.Clone B]
  mixed (Self) (A) (B) : (Self -> A -> B -> RustM Bool)

attribute [instance_reducible, instance] Mixed.trait_constr_Mixed_i0

attribute [instance_reducible, instance] Mixed.trait_constr_Mixed_i1

attribute [instance_reducible, instance] Mixed.trait_constr_Mixed_i2

attribute [instance_reducible, instance] Mixed.trait_constr_Mixed_i3

@[spec]
def Impl_5.mixed_hoisted (self : u32) (a : u64) (b : u16) : RustM Bool := do
  ((← (Item.get u64 a)) ==? (← (Item.get u16 b)))

@[reducible] instance Impl_5.AssociatedTypes :
  Mixed.AssociatedTypes u32 u64 u16
  where

instance Impl_5 : Mixed u32 u64 u16 where
  mixed := (Impl_5.mixed_hoisted)

--  Supertrait in a `where` clause, after a parameter bound.
class WhereSuper.AssociatedTypes (Self : Type) (A : Type) where
  [trait_constr_WhereSuper_i0 : Item.AssociatedTypes A]
  [trait_constr_WhereSuper_i1 : Other.AssociatedTypes Self]

attribute [instance_reducible, instance]
  WhereSuper.AssociatedTypes.trait_constr_WhereSuper_i0

attribute [instance_reducible, instance]
  WhereSuper.AssociatedTypes.trait_constr_WhereSuper_i1

class WhereSuper (Self : Type) (A : Type)
  [associatedTypes : outParam (WhereSuper.AssociatedTypes (Self : Type) (A :
      Type))]
  where
  [trait_constr_WhereSuper_i0 : Item A]
  [trait_constr_WhereSuper_i1 : Other Self]
  where_super (Self) (A) : (Self -> A -> RustM u8)

attribute [instance_reducible, instance] WhereSuper.trait_constr_WhereSuper_i0

attribute [instance_reducible, instance] WhereSuper.trait_constr_WhereSuper_i1

@[spec]
def Impl_6.where_super_hoisted (self : u32) (a : u64) : RustM u8 := do
  (Item.get u64 a)

@[reducible] instance Impl_6.AssociatedTypes :
  WhereSuper.AssociatedTypes u32 u64
  where

instance Impl_6 : WhereSuper u32 u64 where
  where_super := (Impl_6.where_super_hoisted)

--  Bound on an associated type of a supertrait.
class HasAssoc.AssociatedTypes (Self : Type) where
  X : Type

attribute [reducible] HasAssoc.AssociatedTypes.X

abbrev HasAssoc.X :=
  HasAssoc.AssociatedTypes.X

class HasAssoc (Self : Type)
  [associatedTypes : outParam (HasAssoc.AssociatedTypes (Self : Type))]
  where

class AssocBound.AssociatedTypes (Self : Type) where
  [trait_constr_AssocBound_i0 : HasAssoc.AssociatedTypes Self]
  [trait_constr_AssocBound_i1 :
  core_models.clone.Clone.AssociatedTypes
  (HasAssoc.X Self)]

attribute [instance_reducible, instance]
  AssocBound.AssociatedTypes.trait_constr_AssocBound_i0

attribute [instance_reducible, instance]
  AssocBound.AssociatedTypes.trait_constr_AssocBound_i1

class AssocBound (Self : Type)
  [associatedTypes : outParam (AssocBound.AssociatedTypes (Self : Type))]
  where
  [trait_constr_AssocBound_i0 : HasAssoc Self]
  [trait_constr_AssocBound_i1 : core_models.clone.Clone (HasAssoc.X Self)]
  assoc_bound (Self) : (Self -> RustM u8)

attribute [instance_reducible, instance] AssocBound.trait_constr_AssocBound_i0

attribute [instance_reducible, instance] AssocBound.trait_constr_AssocBound_i1

@[reducible] instance Impl_7.AssociatedTypes :
  HasAssoc.AssociatedTypes u32
  where
  X := u64

instance Impl_7 : HasAssoc u32 where

@[spec]
def Impl_8.assoc_bound_hoisted (self : u32) : RustM u8 := do (pure (2 : u8))

@[reducible] instance Impl_8.AssociatedTypes :
  AssocBound.AssociatedTypes u32
  where

instance Impl_8 : AssocBound u32 where
  assoc_bound := (Impl_8.assoc_bound_hoisted)

--  Trait without items.
class Marker.AssociatedTypes (Self : Type) (T : Type) where
  [trait_constr_Marker_i0 : Item.AssociatedTypes T]

attribute [instance_reducible, instance]
  Marker.AssociatedTypes.trait_constr_Marker_i0

class Marker (Self : Type) (T : Type)
  [associatedTypes : outParam (Marker.AssociatedTypes (Self : Type) (T : Type))]
  where
  [trait_constr_Marker_i0 : Item T]

attribute [instance_reducible, instance] Marker.trait_constr_Marker_i0

@[reducible] instance Impl_9.AssociatedTypes :
  Marker.AssociatedTypes u8 u64
  where

instance Impl_9 : Marker u8 u64 where

@[spec]
def use_mixed
    (S : Type)
    (A : Type)
    (B : Type)
    [trait_constr_use_mixed_associated_type_i0 : Mixed.AssociatedTypes S A B]
    [trait_constr_use_mixed_i0 : Mixed S A B ]
    [trait_constr_use_mixed_associated_type_i1 : Item.AssociatedTypes A]
    [trait_constr_use_mixed_i1 : Item A ]
    [trait_constr_use_mixed_associated_type_i2 : Item.AssociatedTypes B]
    [trait_constr_use_mixed_i2 : Item B ]
    [trait_constr_use_mixed_associated_type_i3 :
      core_models.clone.Clone.AssociatedTypes
      B]
    [trait_constr_use_mixed_i3 : core_models.clone.Clone B ]
    (s : S)
    (a : A)
    (b : B) :
    RustM Bool := do
  (Mixed.mixed S A B s a b)

@[spec]
def call_use_mixed (_ : rust_primitives.hax.Tuple0) : RustM Bool := do
  (use_mixed u32 u64 u16 (0 : u32) (0 : u64) (0 : u16))

end new_tests.legacy__trait_param_bounds__lib

