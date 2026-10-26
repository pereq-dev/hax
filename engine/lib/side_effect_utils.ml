open! Prelude

module MakeSI
    (F :
      Features.T
        with type monadic_binding = Features.Off.monadic_binding
         and type for_index_loop = Features.Off.for_index_loop) =
struct
  module AST = Ast.Make (F)
  module U = Ast_utils.Make (F)
  include Ast
  include AST
  module Visitors = Ast_visitors.Make (F)

  module SideEffects = struct
    (* TODO: consider non-terminaison and closed-mutation *)
    type t = {
      reads_local_mut : U.Sets.TypedLocalIdent.t;  (** only free variables *)
      writes_local_mut : U.Sets.TypedLocalIdent.t;  (** only free variables *)
      deep_mutation : bool;
      return : ty option;
      continue : ty option option; (* TODO: continue with labels *)
      break : ty option; (* TODO: break with labels *)
    }
    [@@deriving show]

    let zero : t =
      {
        reads_local_mut = Set.empty (module U.TypedLocalIdent);
        writes_local_mut = Set.empty (module U.TypedLocalIdent);
        deep_mutation = false;
        return = None;
        continue = None;
        break = None;
      }

    let plus : t -> t -> t =
      let merge_ty x y =
        if not @@ U.ty_equality x y then
          Diagnostics.failure ~context:(Other "side_effect_utils.ml")
            ~span:(Span.dummy ())
            (AssertionFailure
               {
                 details =
                   "Expected two exact same types, got x="
                   ^ (x |> U.LiftToFullAst.ty |> Print_rust.pty_str)
                   ^ " and y="
                   ^ (y |> U.LiftToFullAst.ty |> Print_rust.pty_str);
               })
        else x
      in
      let merge_opts (type x) (f : x -> x -> x) (a : x option) (b : x option) =
        match (a, b) with
        | Some a, Some b -> Some (f a b)
        | Some a, None | None, Some a -> Some a
        | None, None -> None
      in
      fun x y ->
        {
          reads_local_mut = Set.union x.reads_local_mut y.reads_local_mut;
          writes_local_mut = Set.union x.writes_local_mut y.writes_local_mut;
          deep_mutation = x.deep_mutation || y.deep_mutation;
          return = merge_opts merge_ty x.return y.return;
          continue =
            merge_opts
              (fun x y ->
                match (x, y) with
                | Some x, Some y -> Some (merge_ty x y)
                | _ -> None)
              x.continue y.continue;
          break = merge_opts merge_ty x.break y.break;
        }

    let reads (var : Local_ident.t) (ty : ty) =
      {
        zero with
        reads_local_mut = Set.singleton (module U.TypedLocalIdent) (var, ty);
      }

    let writes (var : Local_ident.t) (ty : ty) =
      {
        zero with
        writes_local_mut = Set.singleton (module U.TypedLocalIdent) (var, ty);
      }

    let no_deep_mut_or_cf : t -> bool =
      [%matches?
        {
          deep_mutation = false;
          return = None;
          continue = None;
          break = None;
          _;
        }]

    let reads_local_mut_only : t -> bool =
     fun x -> no_deep_mut_or_cf x && Set.is_empty x.writes_local_mut

    let commute : t -> t -> bool =
      curry @@ function
      | ( ({ reads_local_mut = xr; writes_local_mut = xw; _ } as x),
          ({ reads_local_mut = yr; writes_local_mut = yw; _ } as y) )
        when no_deep_mut_or_cf x && no_deep_mut_or_cf y ->
          let open Set in
          let x = union xw xr in
          let y = union yw yr in
          is_empty @@ union (inter xw y) (inter yw x)
      | x, y when reads_local_mut_only x || reads_local_mut_only y -> true
      | _ -> false

    class ['s] monoid =
      object
        method private zero = zero
        method private plus = plus
      end

    let without_rw_vars (vars : U.Sets.Local_ident.t) (effects : t) =
      let without = Set.filter ~f:(fst >> Set.mem vars >> not) in
      {
        effects with
        writes_local_mut = without effects.writes_local_mut;
        reads_local_mut = without effects.reads_local_mut;
      }
  end

  module Hoist = struct
    type binding = pat * expr [@@deriving show]
    type t = { lbs : binding list; effects : SideEffects.t } [@@deriving show]

    let plus x y : t =
      let effects = SideEffects.plus x.effects y.effects in
      { lbs = x.lbs @ y.lbs; effects }

    let zero : t = { lbs = []; effects = SideEffects.zero }
    let flbs { lbs; _ } = lbs
    let feff { effects; _ } = effects
    let no_lbs effects = { lbs = []; effects }

    class ['s] monoid =
      object
        method private zero = zero
        method private plus = plus
      end

    class ['s] bool_monoid =
      object
        method private zero = false
        method private plus = ( && )
      end

    module CollectContext = struct
      type t = { mutable fresh_id : int }

      let fresh_local_ident (self : t) : Local_ident.t =
        self.fresh_id <- self.fresh_id + 1;
        {
          name = "hoist" ^ Int.to_string self.fresh_id;
          id = Local_ident.mk_id SideEffectHoistVar (-1) (* todo *);
        }

      let empty = { fresh_id = 0 }
    end

    module HoistSeq = struct
      let ( let* ) x f = Option.bind ~f x

      let many (ctx : CollectContext.t) (l : (expr * t) list)
          (next : expr list -> t -> expr * t) =
        let fresh () = CollectContext.fresh_local_ident ctx in
        let effects, l =
          List.fold_right l ~init:(SideEffects.zero, [])
            ~f:(fun (expr, { lbs; effects = effects0 }) (effects, l) ->
              ( SideEffects.plus effects0 effects,
                (if
                   SideEffects.reads_local_mut_only effects0
                   && SideEffects.commute effects0 effects
                 then (lbs, expr)
                 else
                   let var =
                     (* if the body is a local variable, use that,
                        otherwise get a fresh one *)
                     match snd @@ U.collect_let_bindings expr with
                     (* TODO: this optimization is disabled because it fails in cases like f(x, {x = 3; x}) *)
                     | { e = LocalVar var; _ } when false -> var
                     | _ -> fresh ()
                   in
                   ( lbs @ [ (U.make_var_pat var expr.typ expr.span, expr) ],
                     { expr with e = LocalVar var } ))
                :: l ))
        in
        let lbs = List.concat_map ~f:fst l in
        next (List.map ~f:snd l) { lbs; effects }

      let err_hoist_invariant span (type r) (location : string) : r =
        Diagnostics.failure ~context:(Other "HoistSeq") ~span
          (AssertionFailure
             {
               details =
                 "[HoistSeq.many] broke its invariant (location:" ^ location
                 ^ ")";
             })

      let one (ctx : CollectContext.t) (e : expr * t)
          (next : expr -> t -> expr * t) =
        many ctx [ e ] (function
          | [ e ] -> next e
          | _ -> err_hoist_invariant (fst e).span Stdlib.__LOC__)
    end

    let let_of_binding ((pat, rhs) : pat * expr) (body : expr) : expr =
      U.make_let pat rhs body

    let lets_of_bindings (bindings : (pat * expr) list) (body : expr) : expr =
      List.fold_right ~init:body ~f:let_of_binding bindings

    let collect_and_hoist_effects_object ~reject_effectful_while_conditions =
      object (self)
        (* inherit [_] expr_mapreduce *)
        inherit [_] Visitors.mapreduce as super
        inherit [_] monoid as m

        (* method visit_t _ x = (x, m#zero) *)
        (* method visit_mutability _ _ x = (x, m#zero) *)

        method private hoist_app (env : CollectContext.t) (e : expr)
            ~generic_args ~trait ~bounds_impls (visited : (expr * t) list) =
          HoistSeq.many env visited (fun l effects ->
              let f, args =
                match l with
                | f :: args -> (f, args)
                | _ -> HoistSeq.err_hoist_invariant e.span Stdlib.__LOC__
              in
              ( { e with e = App { f; args; generic_args; trait; bounds_impls } },
                effects ))

        method private hoist_if (env : CollectContext.t) (e : expr)
            (cond : expr * t) (then_ : unit -> expr * t)
            (else_ : (unit -> expr * t) option) =
          HoistSeq.one env cond (fun cond effects ->
              let then_, { lbs = lbs_then; effects = ethen } = then_ () in
              let else_, { lbs = lbs_else; effects = eelse } =
                match Option.map ~f:(fun else_ -> else_ ()) else_ with
                | Some (else_, eelse) -> (Some else_, eelse)
                | None -> (None, m#zero)
              in
              let then_ = lets_of_bindings lbs_then then_ in
              let else_ = Option.map ~f:(lets_of_bindings lbs_else) else_ in
              let effects =
                m#plus (m#plus (no_lbs ethen) (no_lbs eelse)) effects
              in
              ({ e with e = If { cond; then_; else_ } }, effects))

        (* Collecting effects bottom up *)
        method! visit_lhs (env : CollectContext.t) lhs =
          match lhs with
          | LhsLocalVar { var; typ } ->
              (LhsLocalVar { var; typ }, no_lbs @@ SideEffects.writes var typ)
          | LhsArbitraryExpr { e; witness } ->
              let deep_mutation =
                object
                  inherit [_] Visitors.reduce as _super
                  inherit [_] bool_monoid as _m

                  (* method visit_t _ _ = m#zero *)
                  (* method visit_mutability _ _ _ = m#zero *)
                  (* method! visit_Deref _ _ _ = true *)
                  method! visit_item () _ = false
                end
                  #visit_expr () e
              in
              ( LhsArbitraryExpr { e; witness },
                no_lbs { SideEffects.zero with deep_mutation } )
          | _ -> super#visit_lhs env lhs

        method! visit_expr (env : CollectContext.t) e =
          match e.e with
          | LocalVar v -> (e, no_lbs (SideEffects.reads v e.typ))
          | QuestionMark { e = e'; return_typ; witness } ->
              HoistSeq.one env (self#visit_expr env e') (fun e' effects ->
                  let effects =
                    m#plus effects
                      (no_lbs
                         { SideEffects.zero with return = Some return_typ })
                  in
                  ( { e with e = QuestionMark { e = e'; return_typ; witness } },
                    effects ))
          | Return { e = e'; witness } ->
              HoistSeq.one env (self#visit_expr env e') (fun e' effects ->
                  ( { e with e = Return { e = e'; witness } },
                    m#plus effects
                      (no_lbs { SideEffects.zero with return = Some e'.typ }) ))
          | Break { e = e'; label; acc; witness } ->
              HoistSeq.one env (self#visit_expr env e') (fun e' effects ->
                  ( { e with e = Break { e = e'; acc; label; witness } },
                    m#plus effects
                      (no_lbs { SideEffects.zero with break = Some e'.typ }) ))
          | Continue { acc = e'; label; witness } -> (
              let ceffect =
                no_lbs
                  {
                    SideEffects.zero with
                    continue = Some (Option.map ~f:(fun (e, _) -> e.typ) e');
                  }
              in
              match e' with
              | Some (e', witness') ->
                  HoistSeq.one env (self#visit_expr env e') (fun e' effects ->
                      ( {
                          e with
                          e =
                            Continue
                              { acc = Some (e', witness'); label; witness };
                        },
                        m#plus ceffect effects ))
              | None -> (e, ceffect))
          | Loop { body; kind; state; label; witness; control_flow } ->
              (* a while condition is evaluated at each iteration: its
                 bindings stay in it *)
              let kind, condition_effects =
                match kind with
                | WhileLoop { condition; witness } ->
                    let span = condition.span in
                    let locals =
                      object
                        inherit [_] Visitors.reduce
                        inherit [_] U.Sets.Local_ident.monoid as _m
                        method! visit_pat () p = U.Reducers.variables_of_pat p
                      end
                        #visit_expr () condition
                    in
                    let has_return =
                      object
                        inherit [_] Visitors.reduce as super
                        method zero = false
                        method plus = ( || )

                        method! visit_expr' () e =
                          match e with
                          | Return _ -> true
                          | _ -> super#visit_expr' () e
                      end
                        #visit_expr
                        () condition
                    in
                    let condition, { lbs; effects } =
                      self#visit_expr env condition
                    in
                    let effects = SideEffects.without_rw_vars locals effects in
                    if
                      reject_effectful_while_conditions
                      && (has_return
                         || not (SideEffects.reads_local_mut_only effects))
                    then
                      Diagnostics.SpanFreeError.raise ~span:(Span.to_thir span)
                        (Span.owner_hint span) (Phase HoistSideEffects)
                        (Unimplemented
                           {
                             issue_id = Some (MyInt64.of_int 2337);
                             details =
                               Some
                                 "Side effects in the condition of a while \
                                  loop are not supported";
                           });
                    ( WhileLoop
                        { condition = lets_of_bindings lbs condition; witness },
                      no_lbs effects )
                | kind -> (kind, m#zero)
              in
              let kind' =
                match kind with
                | ForLoop { it; _ } -> [ self#visit_expr env it ]
                | _ -> []
              in
              let state' =
                Option.map
                  ~f:(fun { init; _ } -> self#visit_expr env init)
                  state
              in
              let kind_state = kind' @ Option.to_list state' in
              (* effects to realize before the loop *)
              (* let effects_before = List.fold ~init:zero ~f:plus kind_state in *)
              HoistSeq.many env kind_state (fun l effects ->
                  let kind =
                    match (l, kind) with
                    | it :: ([ _ ] | []), ForLoop { pat; witness; _ } ->
                        ForLoop { pat; witness; it }
                    | ([ _ ] | []), ((UnconditionalLoop | WhileLoop _) as kind)
                      ->
                        kind
                    | _, ForIndexLoop _ -> .
                    | _ -> HoistSeq.err_hoist_invariant e.span Stdlib.__LOC__
                  in
                  let state =
                    match (l, state) with
                    | (_ :: [ state ] | [ state ]), Some { witness; bpat; _ } ->
                        Some { witness; bpat; init = state }
                    | ([ _ ] | []), None -> None
                    | _ -> HoistSeq.err_hoist_invariant e.span Stdlib.__LOC__
                  in
                  (* by now, the "inputs" of the loop are hoisted as let if needed *)
                  let body, { lbs; effects = body_effects } =
                    self#visit_expr env body
                  in
                  (* the loop construction **handles** the effect continue and break *)
                  let body_effects =
                    no_lbs { body_effects with continue = None; break = None }
                  in
                  let effects =
                    m#plus (m#plus effects condition_effects) body_effects
                  in
                  let body = lets_of_bindings lbs body in
                  ( {
                      e with
                      e =
                        Loop { body; kind; state; label; witness; control_flow };
                    },
                    effects ))
          | If { cond; then_; else_ } ->
              self#hoist_if env e (self#visit_expr env cond)
                (fun () -> self#visit_expr env then_)
                (Option.map else_ ~f:(fun else_ () -> self#visit_expr env else_))
          | App
              {
                f = { e = GlobalVar (`Primitive (LogicalOp op)); _ } as f;
                args = [ lhs; rhs ];
                generic_args;
                trait;
                bounds_impls;
              } ->
              let lhs = self#visit_expr env lhs in
              let ((rhs', { lbs; effects }) as rhs) = self#visit_expr env rhs in
              if List.is_empty lbs && SideEffects.reads_local_mut_only effects
              then
                self#hoist_app env e ~generic_args ~trait ~bounds_impls
                  [ (f, m#zero); lhs; rhs ]
              else
                (* `rhs` is only evaluated when `lhs` does not decide *)
                let lit b = ({ rhs' with e = Literal (Bool b) }, m#zero) in
                let then_, else_ =
                  match op with And -> (rhs, lit false) | Or -> (lit true, rhs)
                in
                self#hoist_if env e lhs
                  (fun () -> then_)
                  (Some (fun () -> else_))
          | App { f; args; generic_args; trait; bounds_impls } ->
              self#hoist_app env e ~generic_args ~trait ~bounds_impls
                (List.map ~f:(self#visit_expr env) (f :: args))
          | Literal _ -> (e, m#zero)
          | Block { e; safety_mode; witness } ->
              HoistSeq.one env (self#visit_expr env e) (fun e effects ->
                  ({ e with e = Block { e; safety_mode; witness } }, effects))
          | Array l ->
              HoistSeq.many env
                (List.map ~f:(self#visit_expr env) l)
                (fun l effects -> ({ e with e = Array l }, effects))
          | Construct
              { constructor; is_record; is_struct; fields = []; base = None } ->
              ( {
                  e with
                  e =
                    Construct
                      {
                        constructor;
                        is_record;
                        is_struct;
                        fields = [];
                        base = None;
                      };
                },
                m#zero )
          | Construct { constructor; is_struct; is_record; fields; base } ->
              HoistSeq.many env
                (List.map ~f:(self#visit_expr env)
                   (Option.to_list (Option.map ~f:fst base)
                   @ List.map ~f:snd fields))
                (fun l effects ->
                  let base, fields_expr =
                    match (l, base) with
                    | hd :: tl, Some (_, witness) -> (Some (hd, witness), tl)
                    | _, None -> (None, l)
                    | _ -> HoistSeq.err_hoist_invariant e.span Stdlib.__LOC__
                  in
                  let fields =
                    match List.zip (List.map ~f:fst fields) fields_expr with
                    | Ok fields -> fields
                    | Unequal_lengths ->
                        HoistSeq.err_hoist_invariant e.span Stdlib.__LOC__
                  in
                  ( {
                      e with
                      e =
                        Construct
                          { constructor; is_struct; is_record; fields; base };
                    },
                    effects ))
          | Match { scrutinee; arms } ->
              let arms, eff_arms =
                let arms =
                  List.map ~f:(self#visit_arm env) arms
                  (* materialize letbindings in each arms *)
                  |> List.map ~f:(fun ({ arm; span }, ({ lbs; effects } : t)) ->
                      let arm =
                        { arm with body = lets_of_bindings lbs arm.body }
                      in
                      (({ arm; span } : arm), { lbs = []; effects }))
                    (* cancel effects that concern variables introduced in pats  *)
                  |> List.map ~f:(fun (arm, { lbs; effects }) ->
                      let vars = U.Reducers.variables_of_pat arm.arm.arm_pat in
                      let effects = SideEffects.without_rw_vars vars effects in
                      (arm, { lbs; effects }))
                in
                ( List.map ~f:fst arms,
                  List.fold ~init:m#zero ~f:m#plus (List.map ~f:snd arms) )
              in
              HoistSeq.one env (self#visit_expr env scrutinee)
                (fun scrutinee effects ->
                  ( { e with e = Match { scrutinee; arms } },
                    m#plus eff_arms effects ))
          | Let { monadic = Some _; _ } -> .
          | Let { monadic = None; lhs; rhs; body } ->
              let rhs, { lbs = rhs_lbs; effects = rhs_effects } =
                self#visit_expr env rhs
              in
              let body, { lbs = body_lbs; effects = body_effects } =
                self#visit_expr env body
              in
              let lbs = rhs_lbs @ ((lhs, rhs) :: body_lbs) in
              let effects = SideEffects.plus rhs_effects body_effects in
              (body, { lbs; effects })
          | GlobalVar _ -> (e, m#zero)
          | Ascription { e = e'; typ } ->
              HoistSeq.one env (self#visit_expr env e') (fun e' eff ->
                  ({ e with e = Ascription { e = e'; typ } }, eff))
          | MacroInvokation _ -> (e, m#zero)
          | Assign { lhs; e = e'; witness } ->
              (* TODO: here, LHS should really have no effect... This is not fine *)
              let lhs, lhs_effects = self#visit_lhs env lhs in
              HoistSeq.one env (self#visit_expr env e') (fun e' effects ->
                  let effects = m#plus effects lhs_effects in
                  ({ e with e = Assign { e = e'; lhs; witness } }, effects))
          | Borrow { kind; e = e'; witness } ->
              let kind, kind_effects = self#visit_borrow_kind env kind in
              HoistSeq.one env (self#visit_expr env e') (fun e' effects ->
                  let effects = m#plus kind_effects effects in
                  ({ e with e = Borrow { kind; e = e'; witness } }, effects))
          | AddressOf { mut; e = e'; witness } ->
              let mut, mut_effects = (mut, m#zero) in
              HoistSeq.one env (self#visit_expr env e') (fun e' effects ->
                  let effects = m#plus mut_effects effects in
                  ({ e with e = AddressOf { mut; e = e'; witness } }, effects))
          | Closure { params; body; captures } ->
              let body, body_effects =
                let body, { lbs; effects } = self#visit_expr env body in
                let vars =
                  Set.union_list (module Local_ident)
                  @@ List.map ~f:U.Reducers.variables_of_pat params
                in
                let body = lets_of_bindings lbs body in
                let effects =
                  {
                    (SideEffects.without_rw_vars vars effects) with
                    return = None;
                  }
                in
                (body, { lbs = []; effects })
              in
              ({ e with e = Closure { params; body; captures } }, body_effects)
              (* HoistSeq.many env *)
              (*   (List.map ~f:(super#visit_expr env) captures) *)
              (*   (fun captures effects -> *)
              (*     let effects = m#plus body_effects effects in *)
              (*     ({ e with e = Closure { params; body; captures } }, effects)) *)
          | EffectAction _ ->
              Diagnostics.failure
                ~context:(Other "collect_and_hoist_effects_object") ~span:e.span
                (Unimplemented
                   { issue_id = None; details = Some "EffectAction" })
          | Quote _ -> (e, m#zero)
      end

    (** [reject_effectful_while_conditions] raises an error on a while loop
        whose condition has side effects. *)
    let collect_and_hoist_effects ?(reject_effectful_while_conditions = false)
        (e : expr) : expr * SideEffects.t =
      let e, { lbs; effects } =
        (collect_and_hoist_effects_object ~reject_effectful_while_conditions)
          #visit_expr CollectContext.empty e
      in
      (lets_of_bindings lbs e, effects)
  end
end

module%inlined_contents Hoist
    (F :
      Features.T
        with type monadic_binding = Features.Off.monadic_binding
         and type for_index_loop = Features.Off.for_index_loop) =
struct
  module FA = F

  module FB = struct
    include F
  end

  module UA = Ast_utils.Make (F)
  module UB = Ast_utils.Make (FB)
  module A = Ast.Make (F)
  module B = Ast.Make (FB)

  module S = struct
    include Features.SUBTYPE.Id
  end

  open MakeSI (F)

  [%%inline_defs dmutability + dsafety_kind]

  module ID = struct
    (* OCaml is not able to understand A.expr is the same as B.expr........... *)
    [%%inline_defs dexpr]
  end

  open ID

  let dexpr (expr : A.expr) : B.expr =
    Hoist.collect_and_hoist_effects ~reject_effectful_while_conditions:true expr
    |> fst |> dexpr

  [%%inline_defs "Item.*"]

  let metadata = Phase_utils.Metadata.make HoistSideEffects
end
[@@add "subtype.ml"]
