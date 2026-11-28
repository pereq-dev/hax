module
public import CoreModels.MissingAeneas
public import CoreModels.Core.Funs
public import CoreModels.Spec.Core.Clone

@[expose] public section

namespace CoreModels

open Aeneas
open Aeneas.Std hiding namespace core alloc
open Std.Do WP RustM

set_option mvcgen.warning false
set_option linter.deprecated.syntax false

open ScalarElab

attribute [spec]
  CoreModels.core.slice.Slice.len

@[spec]
theorem core.Shared0Slice.Insts.CoreOpsIndexIndexRangeUsizeSlice.index_spec
    {T : Type} (s : Slice T) (r : core.ops.range.Range Std.Usize)
    (h0 : r.start.val ≤ r.end.val)
    (h1 : r.end.val ≤ s.val.length) :
    ⦃ ⌜ True ⌝ ⦄
    core.Shared0Slice.Insts.CoreOpsIndexIndexRangeUsizeSlice.index s r
    ⦃ ⇓ r' => ⌜ r'.val = s.val.slice r.start.val r.end.val ∧
                r.start.val + r'.val.length = r.end.val ⌝ ⦄ := by
  mvcgen [core.Shared0Slice.Insts.CoreOpsIndexIndexRangeUsizeSlice.index,
    rust_primitives.slice.slice_slice, -Slice.subslice_spec.mvcgen_spec, Slice.subslice]
    <;> grind

/-! ## `copy_from_slice` -/

/-- `s.copy_from_slice(src)`: for an element type whose `clone` returns its argument, the result
is `src`. The lengths must match, since `copy_from_slice` panics otherwise. -/
theorem core.slice.Slice.copy_from_slice_spec {T : Type} (inst : core.marker.Copy T)
    (hclone : ∀ y, ⦃ ⌜ True ⌝ ⦄ inst.cloneCloneInst.clone y ⦃ ⇓ r => ⌜ r = y ⌝ ⦄)
    (s src : Slice T) (h : s.val.length = src.val.length) :
    ⦃ ⌜ True ⌝ ⦄
    core.slice.Slice.copy_from_slice inst s src
    ⦃ ⇓ r => ⌜ r = src ⌝ ⦄ := by
  mvcgen [core.slice.Slice.copy_from_slice, rust_primitives.slice.slice_clone_from_slice]
  all_goals
    -- Cloning `src` returns `src`, which rules out the failing and diverging branches.
    obtain ⟨_, hmap, rfl⟩ :=
      triple_iff_exists_ok.1 (mapM_clone_spec _ hclone src.val)
    rename_i hm
    simp only [hmap, RustM.match.ok] at hm
    simp_all [Slice.eq_iff, Slice.from_val]

uscalar @[spec] theorem core.slice.Slice.«%S».copy_from_slice_spec (s src : Slice «%S»)
    (h : s.val.length = src.val.length) :
    ⦃ ⌜ True ⌝ ⦄
    core.slice.Slice.copy_from_slice core.«%S».Insts.CoreMarkerCopy s src
    ⦃ ⇓ r => ⌜ r = src ⌝ ⦄ :=
  core.slice.Slice.copy_from_slice_spec _
    (fun _ => by mvcgen [core.«%S».Insts.CoreMarkerCopy, core.«%S».Insts.CoreCloneClone.clone])
    s src h

iscalar @[spec] theorem core.slice.Slice.«%S».copy_from_slice_spec (s src : Slice «%S»)
    (h : s.val.length = src.val.length) :
    ⦃ ⌜ True ⌝ ⦄
    core.slice.Slice.copy_from_slice core.«%S».Insts.CoreMarkerCopy s src
    ⦃ ⇓ r => ⌜ r = src ⌝ ⦄ :=
  core.slice.Slice.copy_from_slice_spec _
    (fun _ => by mvcgen [core.«%S».Insts.CoreMarkerCopy, core.«%S».Insts.CoreCloneClone.clone])
    s src h

end CoreModels
