module
public import CoreModels.Core.Funs
public import CoreModels.MissingAeneas

@[expose] public section

/-! # Specs for `core::clone` -/

namespace CoreModels

open Aeneas
open Aeneas.Std hiding namespace core alloc
open Std.Do WP RustM

set_option mvcgen.warning false
set_option linter.deprecated.syntax false

/-- If `clone` returns its argument on every element of `l`, cloning `l` elementwise is the
identity. -/
theorem core.clone.Clone.mapM_clone_of_ok {T : Type} (inst : core.clone.Clone T)
    (l : List T) (hc : ∀ x ∈ l, inst.clone x = ok x) : l.mapM inst.clone = ok l := by
  induction l with
  | nil => rfl
  | cons x xs ih =>
    simp only [List.mapM_cons, ih (fun y hy => hc y (List.mem_cons_of_mem x hy)),
      hc x List.mem_cons_self, bind_tc_ok]
    rfl

end CoreModels
