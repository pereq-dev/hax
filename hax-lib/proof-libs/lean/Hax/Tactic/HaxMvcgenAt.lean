module
public import Lean
public import Aeneas
public import Hax.MissingLean

@[expose] public section

/-!

# `hax_mvcgen at`

This file implements a tactic `hax_mvcgen at h` that runs `mvcgen` on `Triple`s
occurring in a hypothesis `h`.

-/

set_option autoImplicit true

open Lean Std.Do Elab Parser Tactic Meta Aeneas Std

initialize Lean.registerTraceClass `Hax.hax_mvcgen_at

meta register_option hax_mvcgen.warnings : Bool := {
  defValue := true
  descr := "Enable or disable warnings produced by hax_mvcgen"
}

/-- Try to guess which function is missing a `@[spec]` lemma from a verification condition that
`mvcgen` could not discharge: `mvcgen` gets stuck on the first program it has no specification
for, leaving a goal that mentions `wp⟦f a₁ ... aₙ⟧`. Returns `none` when the head of that program
is not a constant, or is mere monadic plumbing, which says nothing about the missing spec. -/
private meta def missingSpecCandidate? (e : Expr) : Option Name := Id.run do
  let some wpApp := e.find? (·.isAppOfArity' ``Std.Do.WP.wp 5) | return none
  -- Look through `bind`s (`Bind.bind .. x f` or `Aeneas.Std.bind .. x f`): the missing
  -- specification is the one of the program bound first, i.e. the second-to-last argument.
  let mut prog := wpApp.appArg!
  while (prog.isAppOf ``Bind.bind || prog.isAppOf ``Aeneas.Std.bind) && prog.getAppNumArgs ≥ 2 do
    prog := prog.getAppArgs[prog.getAppNumArgs - 2]!
  let .const n _ := prog.getAppFn | return none
  return some n

/-- Run `mvcgen` on `goal`. Lean deprecates the `mvcgen` syntax in favour of `vcgen`, which is not a
drop-in replacement for the `Std.Do.Triple` goals hax works with. The deprecation warning would be
reported at every `hax_mvcgen` call, where users cannot disable it, so it is silenced here. -/
meta def evalMvcgenAt (cfgStx : TSyntax `Lean.Parser.Tactic.optConfig) (argStx : Syntax)
    (goal : MVarId) : TacticM (List MVarId) := do
  let inner : TSyntax `tactic := ⟨Syntax.node .none ``Lean.Parser.Tactic.mvcgen
    #[Syntax.atom .none "mvcgen", cfgStx.raw, argStx]⟩
  withOptions (·.setBool `linter.deprecated.syntax false) (evalTacticAt inner goal)

/-- Main entry point of `hax_mvcgen at` -/
meta def haxMvcgenAt (mainGoal : MVarId) (hyp : LocalDecl) (cfgStx : TSyntax `Lean.Parser.Tactic.optConfig) (argStx : Syntax) : TacticM (List MVarId) := do
  forallTelescope (cleanupAnnotations := true) (← instantiateMVars hyp.type) fun xs hbody => do

    trace `Hax.hax_mvcgen_at fun () => m!"hax_mvcgen at {mkFVar hyp.fvarId}: {mainGoal}"
    let hbody ← whnfR hbody
    unless hbody.isAppOfArity' ``Triple 7 do
      Lean.Meta.throwTacticEx `hax_mvcgen mainGoal (m!"Expected `Std.Do.Triple`, got {hbody}")

    -- Create an MVar `newHyp` of type `Prop` representing the expression that `hyp` will be
    -- replaced with. We will determin what `newHyp` states later.
    let newHyp ← mkFreshExprMVar (kind := .syntheticOpaque) (mkSort .zero)

    -- To prove `newHyp`, we apply `triple_in_hypothesis`, followed by `hax_mvcgen`.
    trace `Hax.hax_mvcgen_at fun () => m!"apply `triple_in_hypthesis`"
    let newHypProof ← mkFreshExprMVar newHyp
    let lem ← mkAppM ``Aeneas.Std.triple_in_hypothesis #[newHyp, mkAppN (mkFVar hyp.fvarId) xs]
    let goals ← newHypProof.mvarId!.applyN lem 1
    let [goal] := goals
      | Lean.Meta.throwTacticEx `hax_mvcgen mainGoal
          (m!"Unexpected number of goals after `triple_in_hypothesis`: {goals}")
    let previousLctxSize ← goal.withContext do pure (← getLCtx).decls.size
    trace `Hax.hax_mvcgen_at fun () => m!"run `mvcgen`"
    let cfgStx : TSyntax `Lean.Parser.Tactic.optConfig :=
      Parser.Tactic.appendConfig cfgStx (← `(Lean.Parser.Tactic.optConfig| -trivial))
    let goals ← evalMvcgenAt cfgStx argStx goal

    -- We partition the resulting goals into `newHypGoals` and `sideGoals`: If the conclusion
    -- of a goal is exactly `newHyp`, then we put it into `newHypGoals`, otherwise into `sideGoals`.
    trace `Hax.hax_mvcgen_at fun () => m!"partioning into newHypGoals/sideGoals"
    let mut newHypGoals : Array MVarId := #[]
    let mut sideGoals : Array MVarId := #[]
    for goal in goals do
      let (_, goal) ← goal.withContext do goal.intros
      let target ← goal.withContext do instantiateMVars (← goal.getType)
      if target == newHyp then
        newHypGoals := newHypGoals.push goal
      else
        if (target.find? (· == newHyp)).isSome then
          let mut msg :=
            m!"Failed to process hypothesis {mkFVar hyp.fvarId}. Usually this error is due to \
               missing specs for functions contained in the program."
          if let some fnName := missingSpecCandidate? target then
            msg := msg ++
              m!" This is likely because {.ofConstName fnName} is missing a @[spec] lemma."
          msg := msg ++ m!"\n\nRemaining goal:\n{target}"
          goal.withContext do Lean.Meta.throwTacticEx `hax_mvcgen mainGoal msg
        sideGoals := sideGoals.push goal

    -- For each `newHypGoal`, we collect the local decls `newDecls` that have been introduced
    -- by the `hax_mvcgen` call above.
    trace `Hax.hax_mvcgen_at fun () => m!"collect new declarations"
    let mut newDecls : Array (Array LocalDecl) := #[]
    for newHypGoal in newHypGoals do
      let lctx ← newHypGoal.withContext getLCtx
      let decls := (lctx.decls.toArray.drop previousLctxSize).filterMap id
      newDecls := newDecls.push decls

    -- For each newHypGoal `i`, build `newHypGoalProofsᵢ`:
    --   `fun (p : Prop) (f₁ : fType₁[p]) ... (fₙ : fTypeₙ[p]) => fᵢ newFVarsᵢ₁ ... newFVarsᵢₘ`
    -- where `fTypeⱼ` = `∀ newFVarsⱼ₁ ... newFVarsⱼₘ, p`
    -- All `newHypGoalProofs` have the same type: `∀ (p : Prop), fType₁ → ... → fTypeₙ → p`
    let mut newHypGoalProofs := #[]
    for i in [0:newHypGoals.size] do
      newHypGoals[i]!.withContext do
        trace `Hax.hax_mvcgen_at fun () => m!"build proofs {newDecls[i]!.map (mkFVar ·.fvarId)}"
      let newHypGoalProof ← newHypGoals[i]!.withContext do
        withLocalDeclD `p (mkSort .zero) fun p => do
          let fDeclsNamed ← (Array.range newDecls.size).mapM fun j => do
            let fType ← newHypGoals[j]!.withContext
              (mkForallFVars (newDecls[j]!.map (mkFVar ·.fvarId)) p)
            pure (Name.mkSimple s!"f{j + 1}", fun _ : Array Expr => pure fType)
          withLocalDeclsD fDeclsNamed fun fs => do
            mkLambdaFVars (#[p] ++ fs)
              (mkAppN fs[i]! (newDecls[i]!.filter (!·.isLet)|>.map (mkFVar ·.fvarId)))
      newHypGoalProofs := newHypGoalProofs.push newHypGoalProof

    -- Assign proofs to goals
    trace `Hax.hax_mvcgen_at fun () => m!"assign proofs to goals"
    if !newHypGoals.isEmpty then
      let newHypInst ← inferType newHypGoalProofs[0]!
      trace `Hax.hax_mvcgen_at fun () => m!"assign newHyp {newHypInst}"
      newHyp.mvarId!.assign newHypInst
      for i in [0:newHypGoals.size] do
        trace `Hax.hax_mvcgen_at fun () => m!"assign newHypGoal {i}"
        newHypGoals[i]!.assign newHypGoalProofs[i]!
    else
      trace `Hax.hax_mvcgen_at fun () =>  m!"hax_mvcgen at: no mvar VCs generated, only side conditions: {sideGoals}"
      if hax_mvcgen.warnings.get (← getOptions) then
        logWarning m!"hax_mvcgen at: no mvar VCs generated, only side conditions."

    -- Discharge side goals with `mvcgen`'s trivial discharger:
    trace `Hax.hax_mvcgen_at fun () => m!"discharge side goals"
    let sideGoalsList ← sideGoals.toList.flatMapM
      fun sideGoal => do evalTacticAt (←  `(tactic| mvcgen_trivial)) sideGoal

    if !sideGoalsList.isEmpty then
      trace `Hax.hax_mvcgen_at fun () => m!"hax_mvcgen at: nontrivial side goals generated: {sideGoalsList}"
      if hax_mvcgen.warnings.get (← getOptions) then
        logWarning m!"hax_mvcgen at: nontrivial side goals generated."

    -- TODO: if everything worked correctly, this should not happen:
    if newHypGoals.isEmpty then
      return [← mainGoal.clear hyp.fvarId]



    -- Replace old `hyp` with `newHyp`, using `newHypProof`.
    mainGoal.withContext do
      let fvar := mkFVar hyp.fvarId
      trace `Hax.hax_mvcgen_at fun () => m!"replace hypothesis {fvar}: {mainGoal}"
    let newHypProof ← mkLambdaFVars xs newHypProof
    mainGoal.withContext do
      trace `Hax.hax_mvcgen_at fun () => m!"newHypProof: {newHypProof}"
    let {mvarId, fvarId, ..} ← mainGoal.replace hyp.fvarId newHypProof
    let mainGoals ←
      if xs.size == 0 then
        let fvar := mkFVar fvarId
        mvarId.withContext do
          trace `Hax.hax_mvcgen_at fun () => m!"apply new hypothesis {fvar}: {mvarId}"
        let r ← mvarId.apply (mkFVar fvarId)
        r.mapM (·.tryClear fvarId)
      else
        pure [mvarId]
    return (mainGoals ++ sideGoalsList)

syntax (name := hax_mvcgen_at_hyp) "hax_mvcgen" optConfig
  (" [" withoutPosition((simpStar <|> simpErase <|> simpLemma),*,?) "] ")? "at" ident : tactic

@[tactic hax_mvcgen_at_hyp]
meta def elabHaxMvcgenAtHyp : Tactic := fun stx => do
  let cfgStx : TSyntax `Lean.Parser.Tactic.optConfig := ⟨stx[1]⟩
  let argStx := stx[2]
  let mainGoal ← getMainGoal
  mainGoal.withContext do
    let lctx ← getLCtx
    let .some hyp := lctx.findFromUserName? (Syntax.getId stx[4])
      | Lean.Meta.throwTacticEx `hax_mvcgen mainGoal (m!"Cannot find local assumption {stx[4]}")
    replaceMainGoal (← haxMvcgenAt mainGoal hyp cfgStx argStx)
