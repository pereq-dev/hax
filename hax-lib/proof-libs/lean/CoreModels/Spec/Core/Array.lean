module
public import CoreModels.Core.Funs
public import CoreModels.MissingAeneas
public import CoreModels.Spec.RustPrimitives.Sequence

@[expose] public section

/-! # Specs for `core::array` -/

namespace CoreModels

open Aeneas
open Aeneas.Std hiding namespace core alloc
open Std.Do WP RustM

set_option mvcgen.warning false
set_option linter.deprecated.syntax false

/-! ## `repeat` -/

/-- `array_from_fn_go` with the `repeat` closure pops `n` copies of `x` off the front of a
sequence of `n + k` copies. Needs no assumption on `clone`: the closure never calls it. -/
theorem core.array.repeat.array_from_fn_go_replicate {T : Type} (inst : core.clone.Clone T)
    (N : Std.Usize) (x : T) (n k : Nat)
    (s : rust_primitives.sequence.Seq T) (hs : s.val = List.replicate (n + k) x) :
    ∃ s' : rust_primitives.sequence.Seq T,
      rust_primitives.slice.array_from_fn_go
        (core.array.repeat.closure.Insts.CoreOpsFunctionFnMutTupleUsizeT N inst) s n
        = ok (List.replicate n x, s')
      ∧ s'.val = List.replicate k x := by
  induction n generalizing k with
  | zero => exact ⟨s, rfl, by simpa using hs⟩
  | succ n ih =>
    obtain ⟨s1, h1, hs1⟩ := ih (k + 1) (by rw [hs]; congr 1; omega)
    unfold rust_primitives.slice.array_from_fn_go
    rw [h1, bind_ok]
    have hlen : 0 < s1.val.length := by simp [hs1]
    refine ⟨Slice.from (s1.val.take 0 ++ s1.val.drop 1)
      (by have := s1.property; simp; omega), ?_, ?_⟩
    · show (do
          let q ← rust_primitives.sequence.seq_remove s1 0#usize >>= fun (t, s) => ok (t, s)
          ok (List.replicate n x ++ [q.1], q.2)) = _
      unfold rust_primitives.sequence.seq_remove
      have hlen' : (0#usize : Std.Usize).val < s1.val.length := by simpa using hlen
      simp only [hlen', ↓reduceDIte]
      simp only [bind_tc_ok]
      have hhead : s1.val.get ⟨(0#usize : Std.Usize).val, hlen'⟩ = x := by
        simp [hs1]
      rw [hhead, List.replicate_succ']
      rfl
    · simp [hs1]

/-- `[x; N]` when `clone` returns its argument on `x`. Not `@[spec]` because `mvcgen`
cannot discharge `hc`; see the per-type instances below. -/
theorem core.array.repeat_spec {T : Type} (inst : core.clone.Clone T)
    (N : Std.Usize) (x : T) (hc : inst.clone x = ok x) :
    ⦃ ⌜ True ⌝ ⦄
    core.array.repeat N inst x
    ⦃ ⇓ a => ⌜ a.val = List.replicate N.val x ⌝ ⦄ := by
  obtain ⟨s, hc', hsv⟩ := rust_primitives.sequence.seq_create_replicate inst x hc N
  obtain ⟨s', hg, _⟩ :=
    core.array.repeat.array_from_fn_go_replicate inst N x N.val 0 s (by simpa using hsv)
  unfold core.array.repeat rust_primitives.slice.array_from_fn
  simp only [hc', bind_ok]
  simp only [hg, bind_ok, List.length_replicate, ↓reduceDIte]
  mvcgen
  simp [Array.from_val]

/-- Instantiates `core.array.repeat_spec` as a `@[spec]` for a primitive `Clone` instance. -/
local macro "repeat_spec_for" thm:ident ty:term:max cloneInst:term:max : command =>
  `(@[spec] theorem $thm (N : Std.Usize) (x : $ty) :
      ⦃ ⌜ True ⌝ ⦄ core.array.repeat N $cloneInst x
      ⦃ ⇓ a => ⌜ a.val = List.replicate N.val x ⌝ ⦄ :=
    core.array.repeat_spec _ N x rfl)

repeat_spec_for core.array.repeat_Bool_spec Bool core.Bool.Insts.CoreCloneClone
repeat_spec_for core.array.repeat_U8_spec Std.U8 core.U8.Insts.CoreCloneClone
repeat_spec_for core.array.repeat_U16_spec Std.U16 core.U16.Insts.CoreCloneClone
repeat_spec_for core.array.repeat_U32_spec Std.U32 core.U32.Insts.CoreCloneClone
repeat_spec_for core.array.repeat_U64_spec Std.U64 core.U64.Insts.CoreCloneClone
repeat_spec_for core.array.repeat_U128_spec Std.U128 core.U128.Insts.CoreCloneClone
repeat_spec_for core.array.repeat_Usize_spec Std.Usize core.Usize.Insts.CoreCloneClone
repeat_spec_for core.array.repeat_I8_spec Std.I8 core.I8.Insts.CoreCloneClone
repeat_spec_for core.array.repeat_I16_spec Std.I16 core.I16.Insts.CoreCloneClone
repeat_spec_for core.array.repeat_I32_spec Std.I32 core.I32.Insts.CoreCloneClone
repeat_spec_for core.array.repeat_I64_spec Std.I64 core.I64.Insts.CoreCloneClone
repeat_spec_for core.array.repeat_I128_spec Std.I128 core.I128.Insts.CoreCloneClone
repeat_spec_for core.array.repeat_Isize_spec Std.Isize core.Isize.Insts.CoreCloneClone

end CoreModels
