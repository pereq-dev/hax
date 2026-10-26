import Hax.core_models.core_models
import Hax.Tactic.SpecSet
open Std.Do

set_option mvcgen.warning false
set_option linter.unusedVariables false

/-

# Folds

Hax represents for-loops as folds over a range

-/
section Fold

open core_models.ops.control_flow
open rust_primitives.hax

class rust_primitives.hax.folds {int_type: Type} where
  /-- Encoding of Rust for-loops without early returns -/
  fold_range {α : Type}
    (s e : int_type)
    (inv : α -> int_type -> RustM Prop)
    (init: α)
    (body : α -> int_type -> RustM α)
    (pureInv:
        {i : α -> int_type -> Prop // ∀ a b, ⦃⌜ True ⌝⦄ inv a b ⦃⇓ r => ⌜ r = (i a b) ⌝⦄} := by
      set_option hax_mvcgen.specset "bv" in hax_construct_pure <;> bv_decide) :
    RustM α
  /-- Encoding of Rust for-loops with early returns -/
  fold_range_return  {α_acc α_ret : Type}
    (s e: int_type)
    (inv : α_acc -> int_type -> RustM Prop)
    (init: α_acc)
    (body : α_acc -> int_type ->
      RustM (ControlFlow (ControlFlow α_ret (Tuple2 Tuple0 α_acc)) α_acc ))
    (pureInv:
        {i : α_acc -> int_type -> Prop // ∀ a b, ⦃⌜ True ⌝⦄ inv a b ⦃⇓ r => ⌜ r = (i a b) ⌝⦄} := by
      set_option hax_mvcgen.specset "bv" in hax_construct_pure <;> bv_decide) :
    RustM (ControlFlow α_ret α_acc)
  /-- Encoding of Rust for-loops over `s..=e` without early returns -/
  fold_range_inclusive {α : Type}
    (s e : int_type)
    (inv : α -> int_type -> RustM Prop)
    (init: α)
    (body : α -> int_type -> RustM α)
    (pureInv:
        {i : α -> int_type -> Prop // ∀ a b, ⦃⌜ True ⌝⦄ inv a b ⦃⇓ r => ⌜ r = (i a b) ⌝⦄} := by
      set_option hax_mvcgen.specset "bv" in hax_construct_pure <;> bv_decide) :
    RustM α
  /-- Encoding of Rust for-loops over `s..=e` with early returns -/
  fold_range_inclusive_return  {α_acc α_ret : Type}
    (s e: int_type)
    (inv : α_acc -> int_type -> RustM Prop)
    (init: α_acc)
    (body : α_acc -> int_type ->
      RustM (ControlFlow (ControlFlow α_ret (Tuple2 Tuple0 α_acc)) α_acc ))
    (pureInv:
        {i : α_acc -> int_type -> Prop // ∀ a b, ⦃⌜ True ⌝⦄ inv a b ⦃⇓ r => ⌜ r = (i a b) ⌝⦄} := by
      set_option hax_mvcgen.specset "bv" in hax_construct_pure <;> bv_decide) :
    RustM (ControlFlow α_ret α_acc)

attribute [spec] rust_primitives.hax.folds.fold_range
attribute [spec] rust_primitives.hax.folds.fold_range_return
attribute [spec] rust_primitives.hax.folds.fold_range_inclusive
attribute [spec] rust_primitives.hax.folds.fold_range_inclusive_return

open Lean in
set_option hygiene false in
macro "declare_fold_specs" s:(&"signed" <|> &"unsigned") typeName:ident width:term : command => do
  let tyDot (n : Name) := mkIdent (typeName.getId ++ n)
  let tySimp (n : Name) : TSyntax _ := .mk
    (Syntax.node .none ``Lean.Parser.Tactic.simpLemma #[mkNullNode, mkNullNode, tyDot n])
  let tyRw (n : Name) : TSyntax `Lean.Parser.Tactic.rwRule := .mk
    (Syntax.node .none ``Lean.Parser.Tactic.rwRule #[mkNullNode, tyDot n])
  `(
    /-- Implementation of Rust for-loops without early returns -/
    def $(tyDot `fold_range) {α : Type}
        (s e : $typeName)
        (inv : α -> $typeName -> RustM Prop)
        (init: α)
        (body : α -> $typeName -> RustM α)
        (pureInv: {i : α -> $typeName -> Prop // ∀ a b, ⦃⌜ True ⌝⦄ inv a b ⦃⇓ r => ⌜ r = (i a b) ⌝⦄})
        : RustM α := do
        if s < e
        then fold_range (s + 1) e inv (← body init s) body pureInv
        else pure init
    termination_by (e - s)
    decreasing_by
      simp only [$(tySimp `sizeOf), Nat.add_lt_add_iff_right]
      exact $(tyDot `sub_succ_lt_self) _ _ (by assumption)

    /-- Implementation of Rust for-loops with early returns -/
    def $(tyDot `fold_range_return) {α_acc α_ret : Type}
        (s e: $typeName)
        (inv : α_acc -> $typeName -> RustM Prop)
        (init: α_acc)
        (body : α_acc -> $typeName ->
          RustM (ControlFlow (ControlFlow α_ret (Tuple2 Tuple0 α_acc)) α_acc ))
        (pureInv: {i : α_acc -> $typeName -> Prop // ∀ a b, ⦃⌜ True ⌝⦄ inv a b ⦃⇓ r => ⌜ r = (i a b) ⌝⦄}) := do
      if s < e
      then
        match (← body init s) with
        -- Rust: `return`
        | .Break (.Break res ) => pure (ControlFlow.Break res)
        -- Rust: `break`
        | .Break (.Continue ⟨ ⟨ ⟩, res⟩) => pure (ControlFlow.Continue res)
        -- Rust: `continue`
        | .Continue res => fold_range_return (s + 1) e inv res body pureInv
      else
        pure (ControlFlow.Continue init)
    termination_by (e - s)
    decreasing_by
      simp only [$(tySimp `sizeOf), Nat.add_lt_add_iff_right]
      exact $(tyDot `sub_succ_lt_self) _ _ (by assumption)

    /-- Implementation of Rust for-loops over `s..=e` without early returns -/
    def $(tyDot `fold_range_inclusive) {α : Type}
        (s e : $typeName)
        (inv : α -> $typeName -> RustM Prop)
        (init: α)
        (body : α -> $typeName -> RustM α)
        (pureInv: {i : α -> $typeName -> Prop // ∀ a b, ⦃⌜ True ⌝⦄ inv a b ⦃⇓ r => ⌜ r = (i a b) ⌝⦄})
        : RustM α := do
        if s < e
        then fold_range_inclusive (s + 1) e inv (← body init s) body pureInv
        else if s = e
        then body init s
        else pure init
    termination_by (e - s)
    decreasing_by
      simp only [$(tySimp `sizeOf), Nat.add_lt_add_iff_right]
      exact $(tyDot `sub_succ_lt_self) _ _ (by assumption)

    /-- Implementation of Rust for-loops over `s..=e` with early returns -/
    def $(tyDot `fold_range_inclusive_return) {α_acc α_ret : Type}
        (s e: $typeName)
        (inv : α_acc -> $typeName -> RustM Prop)
        (init: α_acc)
        (body : α_acc -> $typeName ->
          RustM (ControlFlow (ControlFlow α_ret (Tuple2 Tuple0 α_acc)) α_acc ))
        (pureInv: {i : α_acc -> $typeName -> Prop // ∀ a b, ⦃⌜ True ⌝⦄ inv a b ⦃⇓ r => ⌜ r = (i a b) ⌝⦄}) := do
      if s ≤ e
      then
        match (← body init s) with
        -- Rust: `return`
        | .Break (.Break res ) => pure (ControlFlow.Break res)
        -- Rust: `break`
        | .Break (.Continue ⟨ ⟨ ⟩, res⟩) => pure (ControlFlow.Continue res)
        -- Rust: `continue`
        | .Continue res =>
          if s < e
          then fold_range_inclusive_return (s + 1) e inv res body pureInv
          else pure (ControlFlow.Continue res)
      else
        pure (ControlFlow.Continue init)
    termination_by (e - s)
    decreasing_by
      simp only [$(tySimp `sizeOf), Nat.add_lt_add_iff_right]
      exact $(tyDot `sub_succ_lt_self) _ _ (by assumption)

    @[spec]
    instance : @rust_primitives.hax.folds $typeName where
      fold_range := $(tyDot `fold_range)
      fold_range_return := $(tyDot `fold_range_return)
      fold_range_inclusive := $(tyDot `fold_range_inclusive)
      fold_range_inclusive_return := $(tyDot `fold_range_inclusive_return)

    /-- Specification of Rust for-loops without early returns (for bv_decide) -/
    @[specset bv]
    theorem $(mkIdent (s!"rust_primitives.hax.folds.fold_range_spec_bv_{typeName.getId}").toName) {α}
      (s e : $typeName)
      (inv : α -> $typeName -> RustM Prop)
      (pureInv)
      (init: α)
      (body : α -> $typeName -> RustM α) :
      s ≤ e →
      pureInv.val init s →
      (∀ (acc : α) (i : $typeName),
        s ≤ i →
        i < e →
        pureInv.val acc i →
        ⦃ ⌜ True ⌝ ⦄
        (body acc i)
        ⦃ ⇓ res => ⌜ pureInv.val res (i+1) ⌝ ⦄) →
      ⦃ ⌜ True ⌝ ⦄
      ($(tyDot `fold_range) s e inv init body pureInv)
      ⦃ ⇓ r => ⌜ pureInv.val r e ⌝ ⦄
    := by
      intro h_le h_inv_s h_body
      unfold $(tyDot `fold_range)
      mvcgen
      · mstart
        mspec h_body _ _ ($(tyDot `le_refl) s) (by assumption) h_inv_s
        mspec $(mkIdent (s!"rust_primitives.hax.folds.fold_range_spec_bv_{typeName.getId}").toName)
          <;> grind
      · grind
    termination_by (e - s)
    decreasing_by
      simp only [$(tySimp `sizeOf), Nat.add_lt_add_iff_right]
      exact $(tyDot `sub_succ_lt_self) _ _ (by assumption)

    /-- Specification of Rust for-loops without early returns (for grind) -/
    @[specset int]
    theorem $(mkIdent (s!"rust_primitives.hax.folds.fold_range_spec_int_{typeName.getId}").toName) {α}
        (s e : $typeName)
        (inv : α -> $typeName -> RustM Prop)
        (pureInv)
        (init: α)
        (body : α -> $typeName -> RustM α) :
        s.toNat ≤ e.toNat →
        pureInv.val init s →
        (∀ (acc : α) (i : $typeName),
          s.toNat ≤ i.toNat →
          i.toNat < e.toNat →
          pureInv.val acc i →
          ⦃ ⌜ True ⌝ ⦄
          (body acc i)
          ⦃ ⇓ res => ⌜ pureInv.val res (i+1) ⌝ ⦄) →
        ⦃ ⌜ True ⌝ ⦄
        ($(tyDot `fold_range) s e inv init body pureInv)
        ⦃ ⇓ r => ⌜ pureInv.val r e ⌝ ⦄ := by
      apply $(mkIdent (s!"rust_primitives.hax.folds.fold_range_spec_bv_{typeName.getId}").toName)

    /-- Specification of Rust for-loops over `s..=e` without early returns (for
    bv_decide). The invariant after the loop is at `e + 1`, so nothing is known
    after a loop up to the largest value of the type. -/
    @[specset bv]
    theorem $(mkIdent (s!"rust_primitives.hax.folds.fold_range_inclusive_spec_bv_{typeName.getId}").toName) {α}
      (s e : $typeName)
      (inv : α -> $typeName -> RustM Prop)
      (pureInv)
      (init: α)
      (body : α -> $typeName -> RustM α) :
      s ≤ e →
      pureInv.val init s →
      (∀ (acc : α) (i : $typeName),
        s ≤ i →
        i ≤ e →
        pureInv.val acc i →
        ⦃ ⌜ True ⌝ ⦄
        (body acc i)
        ⦃ ⇓ res => ⌜ i + 1 ≠ 0 → pureInv.val res (i+1) ⌝ ⦄) →
      ⦃ ⌜ True ⌝ ⦄
      ($(tyDot `fold_range_inclusive) s e inv init body pureInv)
      ⦃ ⇓ r => ⌜ e + 1 ≠ 0 → pureInv.val r (e+1) ⌝ ⦄
    := by
      intro h_le h_inv_s h_body
      unfold $(tyDot `fold_range_inclusive)
      mvcgen
      · mstart
        mspec h_body _ _ ($(tyDot `le_refl) s) (by assumption) h_inv_s
        mspec $(mkIdent (s!"rust_primitives.hax.folds.fold_range_inclusive_spec_bv_{typeName.getId}").toName)
          <;> grind
      · mstart
        mspec h_body _ _ ($(tyDot `le_refl) s) (by assumption) h_inv_s
        grind
      · grind
    termination_by (e - s)
    decreasing_by
      simp only [$(tySimp `sizeOf), Nat.add_lt_add_iff_right]
      exact $(tyDot `sub_succ_lt_self) _ _ (by assumption)

    /-- Specification of Rust for-loops over `s..=e` without early returns (for grind) -/
    @[specset int]
    theorem $(mkIdent (s!"rust_primitives.hax.folds.fold_range_inclusive_spec_int_{typeName.getId}").toName) {α}
        (s e : $typeName)
        (inv : α -> $typeName -> RustM Prop)
        (pureInv)
        (init: α)
        (body : α -> $typeName -> RustM α) :
        s.toNat ≤ e.toNat →
        pureInv.val init s →
        (∀ (acc : α) (i : $typeName),
          s.toNat ≤ i.toNat →
          i.toNat ≤ e.toNat →
          pureInv.val acc i →
          ⦃ ⌜ True ⌝ ⦄
          (body acc i)
          ⦃ ⇓ res => ⌜ i + 1 ≠ 0 → pureInv.val res (i+1) ⌝ ⦄) →
        ⦃ ⌜ True ⌝ ⦄
        ($(tyDot `fold_range_inclusive) s e inv init body pureInv)
        ⦃ ⇓ r => ⌜ e + 1 ≠ 0 → pureInv.val r (e+1) ⌝ ⦄ := by
      apply $(mkIdent (s!"rust_primitives.hax.folds.fold_range_inclusive_spec_bv_{typeName.getId}").toName)

  )

declare_fold_specs unsigned UInt8 8
declare_fold_specs unsigned UInt16 16
declare_fold_specs unsigned UInt32 32
declare_fold_specs unsigned UInt64 64
declare_fold_specs unsigned USize64 64


end Fold
