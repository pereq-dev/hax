module
public import Hax.Tactic.HaxMvcgenAt

@[expose] public section

/-!

# `hax_mvcgen`

This file implements a tactic `hax_mvcgen` that automatically runs `mvcgen` on
both goals and hypotheses containing `Triple`s.

-/

set_option autoImplicit true

open Lean Std.Do Elab Parser Tactic Meta

private meta def isTripleExpr (e : Expr) : MetaM Bool := do
  forallTelescope (cleanupAnnotations := true) (← instantiateMVars e) fun _ body =>
    return (← whnfR body).isAppOfArity' ``Triple 7

private meta def goalNeedsMvcgen (e : Expr) : Bool :=
  if e.isAppOfArity' ``Triple 7 then true
  else if e.isAppOfArity' ``SPred.entails 3 then
    (e.appArg!.find? (·.isAppOf ``WP.wp)).isSome
  else if e.isAppOf ``ULift.down then
    (e.appArg!.find? (·.isAppOf ``PredTrans.apply)).isSome
  else false

meta partial def haxMvcgenLoop (mainGoal : MVarId)
    (cfgStx : TSyntax `Lean.Parser.Tactic.optConfig) (argStx : Syntax)
    (visited : Std.HashSet FVarId := {}) :
    TacticM (List MVarId) := do
  Core.checkMaxHeartbeats "hax_mvcgen"
  let (_, mainGoal) ← mainGoal.intros
  mainGoal.withContext do
    let lctx ← getLCtx
    -- Look for a hypothesis containing a `Triple`. If found, run `hax_mvcgen at` on it and
    -- make work on the resulting subgoals recursively.
    for hyp in lctx.decls.toArray.filterMap id do
      if !hyp.isImplementationDetail && !visited.contains hyp.fvarId &&
         (← isTripleExpr hyp.type) then
        trace `Hax.hax_mvcgen fun () => m!"hax_mvcgen at {hyp.userName}: {hyp.type}"
        -- Mark this FVar as visited before recursing so the same hypothesis is not
        -- processed again on the same branch (avoids infinite recursion).
        let visited' := visited.insert hyp.fvarId
        let goals ← haxMvcgenAt mainGoal hyp cfgStx argStx
        return (← goals.flatMapM (haxMvcgenLoop · cfgStx argStx visited'))
    -- Otherwise, check whether we should run `mvcgen` on the goal.
    -- Work recursively on the resulting subgoals.
    let goalType ← whnfR (← instantiateMVars (← mainGoal.getType))
    if goalNeedsMvcgen goalType then
      trace `Hax.hax_mvcgen fun () => m!"hax_mvcgen at goal: {mainGoal}"
      let goals ← evalMvcgenAt cfgStx argStx mainGoal
      return (← goals.flatMapM (haxMvcgenLoop · cfgStx argStx visited))
    -- No `Triple`s in hyptheses or goal: The `mainGoal` is a finished verification condition.
    return [mainGoal]

syntax (name := hax_mvcgen) "hax_mvcgen" optConfig
  (" [" withoutPosition((simpStar <|> simpErase <|> simpLemma),*,?) "] ")? : tactic

/-- `hax_mvcgen` runs `mvcgen` on both goals and hypotheses containing `Triple`s. -/
@[tactic hax_mvcgen]
meta def elabHaxMvcgenLoop : Tactic := fun stx => do
  let cfgStx : TSyntax `Lean.Parser.Tactic.optConfig := ⟨stx[1]⟩
  let argStx := stx[2]
  replaceMainGoal (← haxMvcgenLoop (← getMainGoal) cfgStx argStx)
