module
public import CoreModels.Core.Funs
public import CoreModels.MissingAeneas
public import CoreModels.Spec.Core.Clone

@[expose] public section

/-! # Specs for `rust_primitives::sequence` -/

namespace CoreModels

open Aeneas
open Aeneas.Std hiding namespace core alloc
open Std.Do WP RustM

set_option mvcgen.warning false
set_option linter.deprecated.syntax false

/-! ## `seq_create` -/

theorem rust_primitives.sequence.seq_create_replicate {T : Type} (inst : core.clone.Clone T)
    (x : T) (hc : inst.clone x = ok x) (n : Std.Usize) :
    ∃ s, rust_primitives.sequence.seq_create inst x n = ok s
      ∧ s.val = List.replicate n.val x := by
  unfold rust_primitives.sequence.seq_create
  by_cases h0 : n.val = 0
  · exact ⟨Slice.new _, by simp [h0], by simp [h0, Slice.new_val]⟩
  · simp only [h0, ite_false, core.clone.Clone.mapM_clone_of_ok inst _
      (fun y hy => (List.eq_of_mem_replicate hy) ▸ hc), RustM.match.ok]
    have hl : (List.replicate (n.val - 1) x ++ [x]).length ≤ Usize.max := by
      simp only [List.length_append, List.length_replicate, List.length_singleton]
      scalar_tac
    simp only [hl, _root_.dite_true]
    refine ⟨_, rfl, ?_⟩
    simp only [Slice.from_val, ← List.replicate_succ']
    congr 1; omega

end CoreModels
