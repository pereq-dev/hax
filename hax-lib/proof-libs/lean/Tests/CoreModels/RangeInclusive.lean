module
public import Hax
meta import Hax

@[expose] public section

/-! # Tests for iteration over `RangeInclusive` -/

open CoreModels Aeneas
open Aeneas.Std hiding namespace core alloc

namespace Hax.Tests.RangeInclusive

def step := core.U8.Insts.CoreIterRangeStep

def range (s e : U8) : core.ops.range.RangeInclusive U8 := ⟨s, e, false⟩

def exhausted : core.ops.range.RangeInclusive U8 := ⟨3#u8, 3#u8, true⟩

/-- The elements `next` (or `next_back`, if `back`) yields, with at most `fuel` steps. -/
def drain (back : Bool) : Nat → core.ops.range.RangeInclusive U8 → RustM (List U8)
  | 0, _ => .ok []
  | fuel + 1, r => do
    let (o, r') ←
      if back then core.IteratorRangeInclusive.next_back step r
      else core.IteratorRangeInclusive.next step r
    match o with
    | none => .ok []
    | some x => do let xs ← drain back fuel r'; .ok (x :: xs)

def count (r : core.ops.range.RangeInclusive U8) : RustM Usize :=
  core.ops.range.RangeInclusive.Insts.CoreIterTraitsIteratorIterator.count step r

#guard (drain false 5 (range 253#u8 255#u8)).reducesTo [253#u8, 254#u8, 255#u8]
#guard (drain true 5 (range 253#u8 255#u8)).reducesTo [255#u8, 254#u8, 253#u8]
#guard (drain false 5 (range 0#u8 0#u8)).reducesTo [0#u8]
#guard (drain true 5 (range 0#u8 0#u8)).reducesTo [0#u8]
#guard (drain false 5 (range 5#u8 3#u8)).reducesTo []
#guard (drain true 5 (range 5#u8 3#u8)).reducesTo []
#guard (drain false 5 exhausted).reducesTo []
#guard (do let xs ← drain false 300 (range 0#u8 255#u8); .ok xs.length : RustM Nat).reducesTo 256

/-- `next` and `next_back` meet in the middle without yielding an element twice. -/
def meet : RustM (List (Option U8)) := do
  let (a, r) ← core.IteratorRangeInclusive.next step (range 1#u8 3#u8)
  let (b, r) ← core.IteratorRangeInclusive.next_back step r
  let (c, r) ← core.IteratorRangeInclusive.next step r
  let (d, _) ← core.IteratorRangeInclusive.next_back step r
  .ok [a, b, c, d]

#guard meet.reducesTo [some 1#u8, some 3#u8, some 2#u8, none]

#guard (count (range 0#u8 255#u8)).reducesTo 256#usize
#guard (count (range 7#u8 7#u8)).reducesTo 1#usize
#guard (count (range 5#u8 3#u8)).reducesTo 0#usize
#guard (count exhausted).reducesTo 0#usize

end Hax.Tests.RangeInclusive
