module
public import CoreModels

@[expose] public section

/-! # `Iterator::next` on `RangeInclusive` and `RangeFrom`

std's calls are observable through a user `Step` impl: `next` yields the
original `start` (stepping a clone of it), and `RangeFrom` steps with
`Step::forward`, which an impl may override. -/

open CoreModels CoreModels.core
open Aeneas.Std (RustM)

namespace Hax.Tests.RangeNext

/-- A value whose `Clone` sets a tag that the comparisons ignore. -/
structure Tagged where
  val : Nat
  cloned : Bool

def ord : cmp.PartialOrd Tagged Tagged := {
  PartialEqInst := {
    eq := fun a b => .ok (a.val == b.val)
    ne := fun a b => .ok (a.val != b.val) }
  partial_cmp := fun a b => .ok (.some
    (if a.val < b.val then .Less else if a.val = b.val then .Equal else .Greater))
  lt := fun a b => .ok (decide (a.val < b.val))
  le := fun a b => .ok (decide (a.val ≤ b.val))
  gt := fun a b => .ok (decide (a.val > b.val))
  ge := fun a b => .ok (decide (a.val ≥ b.val))
}

/-- Steps by one up to `2`; `forward` saturates there, `forward_checked` fails. -/
def step : iter.range.Step Tagged := {
  cloneCloneInst := {
    clone := fun a => .ok { a with cloned := true }
    clone_from := fun _ b => .ok b }
  corecmpPartialOrdInst := ord
  steps_between := fun _ _ => .fail .panic
  forward_checked := fun a _ =>
    .ok (if a.val < 2 then .some { a with val := a.val + 1 } else .none)
  backward_checked := fun _ _ => .fail .panic
  forward := fun a _ => .ok { a with val := min (a.val + 1) 2 }
  forward_unchecked := fun _ _ => .fail .panic
  backward := fun _ _ => .fail .panic
  backward_unchecked := fun _ _ => .fail .panic
}

def t (v : Nat) : Tagged := ⟨v, false⟩
def c (v : Nat) : Tagged := ⟨v, true⟩

/-- `(t 0)..=(t 1)`: `t 0` itself, then a clone of the stepped clone. -/
example : IteratorRangeInclusive.next step ⟨t 0, t 1, false⟩ =
    .ok (.some (t 0), ⟨c 1, t 1, false⟩) := by
  simp [IteratorRangeInclusive.next, step, ord, t, c]
example : IteratorRangeInclusive.next step ⟨c 1, t 1, false⟩ =
    .ok (.some (c 1), ⟨c 1, t 1, true⟩) := by
  simp [IteratorRangeInclusive.next, step, ord, t, c]
example : IteratorRangeInclusive.next step ⟨t 1, t 0, false⟩ =
    .ok (.none, ⟨t 1, t 0, false⟩) := by
  simp [IteratorRangeInclusive.next, step, ord, t]

/-- `(t 2)..`: `t 2` itself; the saturating `forward` does not panic. -/
example : IteratorRangeFrom.next step ⟨t 2⟩ = .ok (.some (t 2), ⟨c 2⟩) := by
  simp [IteratorRangeFrom.next, step, t, c]
example : IteratorRangeFrom.next step ⟨t 0⟩ = .ok (.some (t 0), ⟨c 1⟩) := by
  simp [IteratorRangeFrom.next, step, t, c]

end Hax.Tests.RangeNext
