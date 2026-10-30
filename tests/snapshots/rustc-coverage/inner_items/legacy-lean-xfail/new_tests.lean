
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


namespace new_tests.rustc_coverage__inner_items

def main.in_mod.IN_MOD_CONST : u32 := (1000 : u32)

--  @fail(extraction): ssprove(HAX0001)
@[spec]
def main.in_func (a : u32) : RustM rust_primitives.hax.Tuple0 := do
  let b : u32 := (1 : u32);
  let c : u32 ← (a +? b);
  let args : (rust_primitives.hax.Tuple1 u32) :=
    (rust_primitives.hax.Tuple1.mk c);
  let args : (RustArray core_models.fmt.rt.Argument 1) :=
    (RustArray.ofVec #v[(← (core_models.fmt.rt.Impl.new_display u32
                            (rust_primitives.hax.Tuple1._0 args)))]);
  let _ ←
    (std.io.stdio._print
      (← (core_models.fmt.rt.Impl_1.new_v1 ((2 : usize)) ((1 : usize))
        (RustArray.ofVec #v["c = ", "\n"])
        args)));
  (pure rust_primitives.hax.Tuple0.mk)

structure main.InStruct where
  in_struct_field : u32

def main.IN_CONST : u32 := (1234 : u32)

@[spec]
def main.Impl.trait_func_hoisted (self : main.InStruct) (incr : u32) :
    RustM main.InStruct := do
  let self : main.InStruct :=
    {self
    with in_struct_field := (← ((main.InStruct.in_struct_field self) +? incr))};
  let _ ← (main.in_func (main.InStruct.in_struct_field self));
  (pure self)

abbrev main.InType : Type := alloc.string.String

--  @fail(extraction): ssprove(HAX0008), fstar(HAX0008), coq(HAX0008)
class main.InTrait.AssociatedTypes (Self : Type) where

class main.InTrait (Self : Type)
  [associatedTypes : outParam (main.InTrait.AssociatedTypes (Self : Type))]
  where
  trait_func (Self) : (Self -> u32 -> RustM Self)
  default_trait_func (Self) (self : Self) :RustM Self := do
    let _ ← (main.in_func main.IN_CONST);
    let self : Self ← (main.InTrait.trait_func Self self main.IN_CONST);
    (pure self)

@[reducible] instance main.Impl.AssociatedTypes :
  main.InTrait.AssociatedTypes main.InStruct
  where

instance main.Impl : main.InTrait main.InStruct where
  trait_func := (main.Impl.trait_func_hoisted)

@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let is_true : Bool ←
    ((← (core_models.iter.traits.exact_size.ExactSizeIterator.len
        std.env.Args (← (std.env.args rust_primitives.hax.Tuple0.mk))))
      ==? (1 : usize));
  let countdown : u32 := (0 : u32);
  let countdown : u32 ←
    if is_true then do
      let countdown : u32 := (10 : u32);
      (pure countdown)
    else do
      (pure countdown);
  let _ ←
    if is_true then do
      let _ ← (main.in_func countdown);
      (pure rust_primitives.hax.Tuple0.mk)
    else do
      (pure rust_primitives.hax.Tuple0.mk);
  let val : main.InStruct :=
    (main.InStruct.mk (in_struct_field := (101 : u32)));
  let val : main.InStruct ← (main.InTrait.default_trait_func main.InStruct val);
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.rustc_coverage__inner_items

