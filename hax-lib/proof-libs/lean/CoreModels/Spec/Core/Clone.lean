module
public import CoreModels.MissingAeneas
public import CoreModels.Core.Funs

@[expose] public section

/-! # Specs for `core::clone` -/

namespace CoreModels

open Aeneas
open Aeneas.Std hiding namespace core alloc
open Std.Do WP RustM

set_option mvcgen.warning false
set_option linter.deprecated.syntax false

/-- Cloning every element of a list with an identity `clone` returns the list. -/
theorem mapM_clone_spec {T : Type} (inst : core.clone.Clone T)
    (hclone : ∀ y, ⦃ ⌜ True ⌝ ⦄ inst.clone y ⦃ ⇓ r => ⌜ r = y ⌝ ⦄) (l : List T) :
    ⦃ ⌜ True ⌝ ⦄ l.mapM inst.clone ⦃ ⇓ r => ⌜ r = l ⌝ ⦄ := by
  induction l with
  | nil => mvcgen
  | cons y l ih =>
    rw [List.mapM_cons]
    mvcgen [hclone, ih]
    subst_vars
    rfl

end CoreModels
