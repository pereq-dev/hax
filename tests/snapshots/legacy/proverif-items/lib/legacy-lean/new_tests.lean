
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


namespace new_tests.legacy__proverif_items__lib

structure Point where
  x : u32
  y : u32

@[spec]
def make (x : u32) (y : u32) : RustM Point := do
  (pure (Point.mk (x := x) (y := y)))

@[spec]
def get_x (p : Point) : RustM u32 := do (pure (Point.x p))

structure Counter where
  -- no fields

def Impl.BASE : u32 := (0 : u32)

@[spec]
def Impl.inner (self : Counter) : RustM u32 := do (pure Impl.BASE)

@[spec]
def Impl.outer (self : Counter) : RustM u32 := do (Impl.inner self)

end new_tests.legacy__proverif_items__lib

