
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


namespace new_tests.legacy__proverif_binding_subpatterns__lib

@[spec]
def in_match_arm
    (x : (rust_primitives.hax.Tuple2 u8 (rust_primitives.hax.Tuple2 u8 u8))) :
    RustM u8 := do
  match x with
    | ⟨0, pair@⟨a, b⟩⟩ => do
      ((← ((← (a +? b)) +? (rust_primitives.hax.Tuple2._0 pair)))
        +? (rust_primitives.hax.Tuple2._1 pair))
    | _ => do (pure (0 : u8))

@[spec]
def in_let (x : (rust_primitives.hax.Tuple2 u8 u8)) : RustM u8 := do
  let pair@⟨a, b⟩ := x;
  ((← (a +? b)) +? (rust_primitives.hax.Tuple2._0 pair))

structure Point where
  x : u8
  y : u8

@[spec]
def nested (p : Point) : RustM u8 := do
  let whole@{x := x, y := y} := p;
  ((← (x +? y)) +? (Point.x whole))

end new_tests.legacy__proverif_binding_subpatterns__lib

