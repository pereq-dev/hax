module Core_models.Bool
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Rust_primitives

/// See [`std::primitive::bool::then_some`]
let impl_bool__then_some (#v_T: Type0) (b: bool) (t: v_T) : Core_models.Option.t_Option v_T =
  if b
  then Core_models.Option.Option_Some t <: Core_models.Option.t_Option v_T
  else Core_models.Option.Option_None <: Core_models.Option.t_Option v_T

/// See [`std::primitive::bool::then`]
assume
val impl_bool__then':
    #v_T: Type0 ->
    #v_F: Type0 ->
    {| i0: Core_models.Ops.Function.t_FnOnce v_F Prims.unit |} ->
    #_: unit{i0.Core_models.Ops.Function.f_Output == v_T} ->
    b: bool ->
    f: v_F
  -> Core_models.Option.t_Option v_T

unfold
let impl_bool__then
      (#v_T #v_F: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Core_models.Ops.Function.t_FnOnce v_F Prims.unit)
      (#_: unit{i0.Core_models.Ops.Function.f_Output == v_T})
     = impl_bool__then' #v_T #v_F #i0 #_
