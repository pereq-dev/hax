module
public import CoreModels.Core.Types
public import CoreModels.Alloc.Types
public import CoreModels.RustPrimitives.Funs

@[expose] public section

/-!

# Funs Prologue

This file contains workarounds required to be present **before** `Funs.lean` runs. The file
`Funs.lean` contains the functions automatically generated from our Rust implementation of core.
Since it's automatically generated, we cannot move this material there.

-/

namespace CoreModels.core

open Aeneas.Std RustM

/-! ## Scalar PartialEq / PartialOrd instances -/

def U8.Insts.CoreCmpPartialEqU8       : cmp.PartialEq U8    U8    := { eq := fun x y => ok (x == y), ne := fun x y => ok (x != y) }
def U16.Insts.CoreCmpPartialEqU16     : cmp.PartialEq U16   U16   := { eq := fun x y => ok (x == y), ne := fun x y => ok (x != y) }
def U32.Insts.CoreCmpPartialEqU32     : cmp.PartialEq U32   U32   := { eq := fun x y => ok (x == y), ne := fun x y => ok (x != y) }
def U64.Insts.CoreCmpPartialEqU64     : cmp.PartialEq U64   U64   := { eq := fun x y => ok (x == y), ne := fun x y => ok (x != y) }
def U128.Insts.CoreCmpPartialEqU128   : cmp.PartialEq U128  U128  := { eq := fun x y => ok (x == y), ne := fun x y => ok (x != y) }
def Usize.Insts.CoreCmpPartialEqUsize : cmp.PartialEq Usize Usize := { eq := fun x y => ok (x == y), ne := fun x y => ok (x != y) }
def I8.Insts.CoreCmpPartialEqI8       : cmp.PartialEq I8    I8    := { eq := fun x y => ok (x == y), ne := fun x y => ok (x != y) }
def I16.Insts.CoreCmpPartialEqI16     : cmp.PartialEq I16   I16   := { eq := fun x y => ok (x == y), ne := fun x y => ok (x != y) }
def I32.Insts.CoreCmpPartialEqI32     : cmp.PartialEq I32   I32   := { eq := fun x y => ok (x == y), ne := fun x y => ok (x != y) }
def I64.Insts.CoreCmpPartialEqI64     : cmp.PartialEq I64   I64   := { eq := fun x y => ok (x == y), ne := fun x y => ok (x != y) }
def I128.Insts.CoreCmpPartialEqI128   : cmp.PartialEq I128  I128  := { eq := fun x y => ok (x == y), ne := fun x y => ok (x != y) }
def Isize.Insts.CoreCmpPartialEqIsize : cmp.PartialEq Isize Isize := { eq := fun x y => ok (x == y), ne := fun x y => ok (x != y) }
def Bool.Insts.CoreCmpPartialEqBool    : cmp.PartialEq Bool  Bool  := { eq := fun x y => ok (x == y), ne := fun x y => ok (x != y) }

/-! ## Tuple PartialEq

`(A, B) == (C, D)` — reached by e.g. comparing the `(T, bool)` an
`overflowing_*` returns. Short-circuits on the first component, as Rust does. -/

def Pair.Insts.CoreCmpPartialEqPair.eq {A B C D : Type _}
    (PartialEqInst : cmp.PartialEq A C) (PartialEqInst1 : cmp.PartialEq B D) :
    A × B → C × D → RustM Bool := fun (a, b) (c, d) => do
  let eqFst ← PartialEqInst.eq a c
  if eqFst then PartialEqInst1.eq b d else ok false

def Pair.Insts.CoreCmpPartialEqPair.ne {A B C D : Type _}
    (PartialEqInst : cmp.PartialEq A C) (PartialEqInst1 : cmp.PartialEq B D) :
    A × B → C × D → RustM Bool := fun p q => do
  let eq ← Pair.Insts.CoreCmpPartialEqPair.eq PartialEqInst PartialEqInst1 p q
  ok (!eq)

def Pair.Insts.CoreCmpPartialEqPair {A B C D : Type _}
    (PartialEqInst : cmp.PartialEq A C) (PartialEqInst1 : cmp.PartialEq B D) :
    cmp.PartialEq (A × B) (C × D) := {
  eq := Pair.Insts.CoreCmpPartialEqPair.eq PartialEqInst PartialEqInst1
  ne := Pair.Insts.CoreCmpPartialEqPair.ne PartialEqInst PartialEqInst1
}

def mkUPartialOrd {ty} : cmp.PartialOrd (UScalar ty) (UScalar ty) := {
  PartialEqInst := { eq := fun x y => ok (x == y), ne := fun x y => ok (x != y) }
  partial_cmp := fun x y =>
    ok (option.Option.Some
      (match compare x.val y.val with
       | .lt => cmp.Ordering.Less
       | .eq => cmp.Ordering.Equal
       | .gt => cmp.Ordering.Greater))
  lt := fun x y => ok (match compare x.val y.val with | .lt => true | _ => false)
  le := fun x y => ok (match compare x.val y.val with | .gt => false | _ => true)
  gt := fun x y => ok (match compare x.val y.val with | .gt => true | _ => false)
  ge := fun x y => ok (match compare x.val y.val with | .lt => false | _ => true)
}

/-- The `Iterator::next` implementation for `core::ops::range::Range<A>`,
    parameterised over the `Step` dictionary. -/
def IteratorRange.next {A : Type _} (StepInst : iter.range.Step A) :
    ops.range.Range A → Aeneas.Std.RustM ((Option A) × ops.range.Range A) := fun range => do
  let cmp ← StepInst.corecmpPartialOrdInst.partial_cmp range.start range.«end»
  let isLess : Bool := match cmp with
    | Option.some o => match o with
                       | core.cmp.Ordering.Less => true
                       | _ => false
    | _ => false
  if isLess then
    let cur ← StepInst.cloneCloneInst.clone range.start
    let next? ← StepInst.forward_checked cur 1#usize
    match next? with
    | Option.none      => .fail .panic
    | Option.some next => .ok (Option.some cur, { range with start := next })
  else .ok (Option.none, range)

def mkIPartialOrd {ty} : cmp.PartialOrd (IScalar ty) (IScalar ty) := {
  PartialEqInst := { eq := fun x y => ok (x == y), ne := fun x y => ok (x != y) }
  partial_cmp := fun x y =>
    ok (option.Option.Some
      (match compare x.val y.val with
       | .lt => cmp.Ordering.Less
       | .eq => cmp.Ordering.Equal
       | .gt => cmp.Ordering.Greater))
  lt := fun x y => ok (match compare x.val y.val with | .lt => true | _ => false)
  le := fun x y => ok (match compare x.val y.val with | .gt => false | _ => true)
  gt := fun x y => ok (match compare x.val y.val with | .gt => true | _ => false)
  ge := fun x y => ok (match compare x.val y.val with | .lt => false | _ => true)
}

def U8.Insts.CoreCmpPartialOrdU8       : cmp.PartialOrd U8    U8    := mkUPartialOrd
def U16.Insts.CoreCmpPartialOrdU16     : cmp.PartialOrd U16   U16   := mkUPartialOrd
def U32.Insts.CoreCmpPartialOrdU32     : cmp.PartialOrd U32   U32   := mkUPartialOrd
def U64.Insts.CoreCmpPartialOrdU64     : cmp.PartialOrd U64   U64   := mkUPartialOrd
def U128.Insts.CoreCmpPartialOrdU128   : cmp.PartialOrd U128  U128  := mkUPartialOrd
def Usize.Insts.CoreCmpPartialOrdUsize : cmp.PartialOrd Usize Usize := mkUPartialOrd
def I8.Insts.CoreCmpPartialOrdI8       : cmp.PartialOrd I8    I8    := mkIPartialOrd
def I16.Insts.CoreCmpPartialOrdI16     : cmp.PartialOrd I16   I16   := mkIPartialOrd
def I32.Insts.CoreCmpPartialOrdI32     : cmp.PartialOrd I32   I32   := mkIPartialOrd
def I64.Insts.CoreCmpPartialOrdI64     : cmp.PartialOrd I64   I64   := mkIPartialOrd
def I128.Insts.CoreCmpPartialOrdI128   : cmp.PartialOrd I128  I128  := mkIPartialOrd
def Isize.Insts.CoreCmpPartialOrdIsize : cmp.PartialOrd Isize Isize := mkIPartialOrd

/-! ## Scalar `Ord` instances

`core::cmp::Ord for <int>` is `hax_lib::exclude`d in `cmp.rs` (like `PartialEq`
/ `PartialOrd`), so — to match the excluded `PartialOrd` instances above — we
re-provide it here. Without these, any model code requiring `T: Ord` on a
scalar (e.g. `<[T]>::cmp`, sorting, `BinaryHeap`) references an undefined
`<int>.Insts.CoreCmpOrd`. -/

def mkUOrd {ty} : cmp.Ord (UScalar ty) := {
  EqInst := { PartialEqInst := { eq := fun x y => ok (x == y), ne := fun x y => ok (x != y) } }
  PartialOrdInst := mkUPartialOrd
  cmp := fun x y =>
    ok (match compare x.val y.val with
        | .lt => cmp.Ordering.Less
        | .eq => cmp.Ordering.Equal
        | .gt => cmp.Ordering.Greater)
}

def mkIOrd {ty} : cmp.Ord (IScalar ty) := {
  EqInst := { PartialEqInst := { eq := fun x y => ok (x == y), ne := fun x y => ok (x != y) } }
  PartialOrdInst := mkIPartialOrd
  cmp := fun x y =>
    ok (match compare x.val y.val with
        | .lt => cmp.Ordering.Less
        | .eq => cmp.Ordering.Equal
        | .gt => cmp.Ordering.Greater)
}

def U8.Insts.CoreCmpOrd    : cmp.Ord U8    := mkUOrd
def U16.Insts.CoreCmpOrd   : cmp.Ord U16   := mkUOrd
def U32.Insts.CoreCmpOrd   : cmp.Ord U32   := mkUOrd
def U64.Insts.CoreCmpOrd   : cmp.Ord U64   := mkUOrd
def U128.Insts.CoreCmpOrd  : cmp.Ord U128  := mkUOrd
def Usize.Insts.CoreCmpOrd : cmp.Ord Usize := mkUOrd
def I8.Insts.CoreCmpOrd    : cmp.Ord I8    := mkIOrd
def I16.Insts.CoreCmpOrd   : cmp.Ord I16   := mkIOrd
def I32.Insts.CoreCmpOrd   : cmp.Ord I32   := mkIOrd
def I64.Insts.CoreCmpOrd   : cmp.Ord I64   := mkIOrd
def I128.Insts.CoreCmpOrd  : cmp.Ord I128  := mkIOrd
def Isize.Insts.CoreCmpOrd : cmp.Ord Isize := mkIOrd

/-! ## Scalar `Eq` instances

`core::cmp::Eq for <int>` is `hax_lib::exclude`d in `cmp.rs` alongside PartialEq/
PartialOrd/Ord, so re-provide it here (a downstream `==`/derived-Eq on a scalar
references `<int>.Insts.CoreCmpEq`). `cmp.Eq` is just the `PartialEq` marker. -/
def mkUEq {ty} : cmp.Eq (UScalar ty) :=
  { PartialEqInst := { eq := fun x y => ok (x == y), ne := fun x y => ok (x != y) } }
def mkIEq {ty} : cmp.Eq (IScalar ty) :=
  { PartialEqInst := { eq := fun x y => ok (x == y), ne := fun x y => ok (x != y) } }

def U8.Insts.CoreCmpEq    : cmp.Eq U8    := mkUEq
def U16.Insts.CoreCmpEq   : cmp.Eq U16   := mkUEq
def U32.Insts.CoreCmpEq   : cmp.Eq U32   := mkUEq
def U64.Insts.CoreCmpEq   : cmp.Eq U64   := mkUEq
def U128.Insts.CoreCmpEq  : cmp.Eq U128  := mkUEq
def Usize.Insts.CoreCmpEq : cmp.Eq Usize := mkUEq
def I8.Insts.CoreCmpEq    : cmp.Eq I8    := mkIEq
def I16.Insts.CoreCmpEq   : cmp.Eq I16   := mkIEq
def I32.Insts.CoreCmpEq   : cmp.Eq I32   := mkIEq
def I64.Insts.CoreCmpEq   : cmp.Eq I64   := mkIEq
def I128.Insts.CoreCmpEq  : cmp.Eq I128  := mkIEq
def Isize.Insts.CoreCmpEq : cmp.Eq Isize := mkIEq

abbrev ops.range.Range.Insts.CoreIterTraitsIteratorIterator.next :=
  @IteratorRange.next

/-- `Iterator::count` for `core::ops::range::Range<A>`. Driving `next` to
    exhaustion advances `start` one step at a time until it reaches `end`, which
    is what `Step::steps_between` reports (`0` when `start > end`). -/
def ops.range.Range.Insts.CoreIterTraitsIteratorIterator.count {A : Type _}
    (StepInst : iter.range.Step A) (range : ops.range.Range A) : RustM Usize := do
  let (steps, _) ← StepInst.steps_between range.start range.«end»
  ok steps

/-- `next_back` for `Range<A>`, parameterised over `Step` — mirrors Aeneas.Std's
    `RangeIter`. Consume from the high end: if `start < end`, decrement `end` by one
    and yield the new `end`; otherwise `none`. -/
def IteratorRange.next_back {A : Type _} (StepInst : iter.range.Step A) :
    ops.range.Range A → RustM ((Option A) × ops.range.Range A) := fun range => do
  let lt ← StepInst.corecmpPartialOrdInst.lt range.start range.«end»
  if lt then do
    let b ← StepInst.backward_checked range.«end» 1#usize
    match b with
    | Option.none      => .fail .panic
    | Option.some e'   => .ok (Option.some e', { range with «end» := e' })
  else .ok (Option.none, range)

/-! ## Full generic `Range<A>` iterator instance dicts

aeneas models `Range` iteration GENERICALLY (`Range<A: Step>`) and emits
`core.ops.range.Range.Insts.CoreIterTraits…(StepInst)` at downstream `for`/`.map`/
`.rev`/`.collect` sites. Core-models' Rust source instead defines Range iteration
PER-SCALAR-TYPE (the `impl_iterator_range_int!` macro → `RangeUsize.Insts.…`), so
the generic instance dict is missing. We provide it here (delegating to the generic
`IteratorRange.next`/`.next_back` above), mirroring Aeneas.Std's `RangeIter`. The
`.next` abbrev above is the function form the same-crate generated code calls; this
is the full dict a downstream extraction passes as an `Iterator`/`DoubleEnded`
dictionary. -/
def ops.range.Range.Insts.CoreIterTraitsIteratorIterator {A : Type _}
    (StepInst : iter.range.Step A) :
    iter.traits.iterator.Iterator (ops.range.Range A) A := {
  next := IteratorRange.next StepInst
}

def ops.range.Range.Insts.CoreIterTraitsDouble_endedDoubleEndedIterator {A : Type _}
    (StepInst : iter.range.Step A) :
    iter.traits.double_ended.DoubleEndedIterator (ops.range.Range A) A := {
  iteratorIteratorInst := ops.range.Range.Insts.CoreIterTraitsIteratorIterator StepInst
  next_back := IteratorRange.next_back StepInst
}

/-- [core::cmp::impls::{core::cmp::PartialOrd<&0 (B)> for &1 (A)}::lt]:
    Source: '/rustc/library/core/src/cmp.rs', lines 2133:8-2133:40
    Name pattern: [core::cmp::impls::{core::cmp::PartialOrd<&'1 @A, &'0 @B>}::lt]
    Visibility: public -/
@[rust_fun "core::cmp::impls::{core::cmp::PartialOrd<&'1 @A, &'0 @B>}::lt"]
def Shared1A.Insts.CoreCmpPartialOrdShared0B.lt
  {A : Type _} {B : Type _} (PartialOrdInst : cmp.PartialOrd A B) :
  A → B → RustM Bool := fun a b => do
  let o ← PartialOrdInst.partial_cmp a b
  match o with
  | some cmp.Ordering.Less => ok true
  | _ => ok false

/-- [core::cmp::impls::{core::cmp::PartialOrd<&0 (B)> for &1 (A)}::gt]:
    Source: '/rustc/library/core/src/cmp.rs', lines 2141:8-2141:40
    Name pattern: [core::cmp::impls::{core::cmp::PartialOrd<&'1 @A, &'0 @B>}::gt]
    Visibility: public -/
@[rust_fun "core::cmp::impls::{core::cmp::PartialOrd<&'1 @A, &'0 @B>}::gt"]
def Shared1A.Insts.CoreCmpPartialOrdShared0B.gt
  {A : Type _} {B : Type _} (PartialOrdInst : cmp.PartialOrd A B) :
  A → B → RustM Bool := fun a b => do
  let o ← PartialOrdInst.partial_cmp a b
  match o with
  | some cmp.Ordering.Greater => ok true
  | _ => ok false


/-! ## Formatting arguments

`fmt::Arguments::new` is `hax_lib::exclude`d in `core-models/src/core/fmt.rs`
(aeneas fails with "There should be no bottoms in the value" on any body that
builds an `Arguments`, which is why every other constructor there is
`hax_lib::opaque`), so its model lives here. `fmt.Arguments` is `Unit`. -/

def fmt.Arguments.new {N M : Usize}
    (_template : Array U8 N) (_args : Array fmt.rt.Argument M) :
    RustM fmt.Arguments :=
  ok ()

/-! ## Comparing and cloning references

Real core's `impl PartialEq<&B> for &A` and `impl Clone for &T`, which the model
cannot provide from Rust: an impl whose self type is a reference gets the impl's
lifetimes baked into its extracted name (see the `PartialOrd` pair above), and
`Clone for &T` would overlap the model's own primitive instances. The names below
are the ones an extracted client actually references -- read off
`tests/client_test`'s extraction, like the `PartialOrd` pair. -/

def Shared1A.Insts.CoreCmpPartialEqShared0B.eq {A B : Type _}
    (PartialEqInst : cmp.PartialEq A B) : A → B → RustM Bool :=
  PartialEqInst.eq

def Shared1A.Insts.CoreCmpPartialEqShared0B.ne {A B : Type _}
    (PartialEqInst : cmp.PartialEq A B) : A → B → RustM Bool :=
  PartialEqInst.ne

def Shared1A.Insts.CoreCmpPartialEqShared0B {A B : Type _}
    (PartialEqInst : cmp.PartialEq A B) : cmp.PartialEq A B :=
  PartialEqInst

/-- `Clone` for a shared reference: cloning `&T` copies the reference, which
    extraction erases, so this is the identity even when `T` is not `Clone`. -/
def Shared0T.Insts.CoreCloneClone.clone {T : Type _} : T → RustM T := ok

def Shared0T.Insts.CoreCloneClone (T : Type _) : clone.Clone T := {
  clone := Shared0T.Insts.CoreCloneClone.clone
  -- `Clone` gained a `clone_from` provided method (see `clone.rs`); cloning a
  -- shared reference is the identity, so overwriting is just the source.
  clone_from := fun _ source => ok source
}

/-! ## Option -/

def option.Option.take :=
  fun {T} x => Aeneas.Std.RustM.ok (@Aeneas.Std.core.option.Option.take T x)

/-! ## Mem -/

def mem.swap :=
  fun {T} x y => Aeneas.Std.RustM.ok (@Aeneas.Std.core.mem.swap T x y)

def mem.replace :=
  fun {T} x y => Aeneas.Std.RustM.ok (@Aeneas.Std.core.mem.replace T x y)

/-! ## Redirects to Aeneas's library

The `num.<X>.{MIN,MAX,BITS}` re-exports below stand in for the definitions the
model would otherwise generate (`patch_lean.py` comments those out). They have
to be pure values rather than `RustM _`, because that is how an extracted
consumer of `<int>::{MIN,MAX,BITS}` refers to them; a monadic definition here
would not unify at any such use site. -/

export Aeneas.Std.core (
  num.U8.MIN num.U8.MAX num.U8.BITS num.I8.MIN num.I8.MAX num.I8.BITS
  num.U16.MIN num.U16.MAX num.U16.BITS num.I16.MIN num.I16.MAX num.I16.BITS
  num.U32.MIN num.U32.MAX num.U32.BITS num.I32.MIN num.I32.MAX num.I32.BITS
  num.U64.MIN num.U64.MAX num.U64.BITS num.I64.MIN num.I64.MAX num.I64.BITS
  num.U128.MIN num.U128.MAX num.U128.BITS num.I128.MIN num.I128.MAX num.I128.BITS
  num.Usize.MIN num.Usize.MAX num.Usize.BITS
  num.Isize.MIN num.Isize.MAX num.Isize.BITS
  convert.num.FromU16U8.from
  convert.num.FromU32U8.from
  convert.num.FromU32U16.from
  convert.num.FromU64U8.from
  convert.num.FromU64U16.from
  convert.num.FromU64U32.from
  convert.num.FromU128U8.from
  convert.num.FromU128U16.from
  convert.num.FromU128U32.from
  convert.num.FromU128U64.from
  convert.num.FromUsizeU8.from
  convert.num.FromUsizeU16.from
  convert.num.FromI16I8.from
  convert.num.FromI32I8.from
  convert.num.FromI32I16.from
  convert.num.FromI64I8.from
  convert.num.FromI64I16.from
  convert.num.FromI64I32.from
  convert.num.FromI128I8.from
  convert.num.FromI128I16.from
  convert.num.FromI128I32.from
  convert.num.FromI128I64.from
  convert.num.FromIsizeI8.from
  convert.num.FromIsizeI16.from
)

end CoreModels.core
