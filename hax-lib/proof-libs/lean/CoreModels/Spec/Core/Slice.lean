module
public import CoreModels.MissingAeneas
public import CoreModels.Core.Funs

@[expose] public section

namespace CoreModels

open Aeneas
open Aeneas.Std hiding namespace core alloc
open Std.Do WP RustM

set_option mvcgen.warning false
set_option linter.deprecated.syntax false

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

end CoreModels
