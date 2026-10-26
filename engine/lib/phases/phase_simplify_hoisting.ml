open! Prelude

module Make (F : Features.T) =
  Phase_utils.MakeMonomorphicPhase
    (F)
    (struct
      let phase_id = [%auto_phase_name auto]

      open Ast.Make (F)
      module U = Ast_utils.Make (F)
      module Visitors = Ast_visitors.Make (F)

      module Error = Phase_utils.MakeError (struct
        let ctx = Diagnostics.Context.Phase phase_id
      end)

      (** Whether a use of [var] in [body] is under a binding of a name of
          [names]. *)
      let captured (var : Local_ident.t) (names : (string, _) Set.t)
          (body : expr) =
        let bind bound vars =
          Set.fold vars ~init:bound ~f:(fun bound (v : Local_ident.t) ->
              Set.add bound v.name)
        in
        let bind_pats bound pats =
          bind bound (U.Reducers.variables_of_pats pats)
        in
        object (self)
          inherit [_] Visitors.reduce as super
          method zero = false
          method plus = ( || )

          method! visit_expr bound e =
            match e.e with
            | LocalVar v when [%eq: Local_ident.t] v var ->
                not (Set.are_disjoint bound names)
            | Let { lhs; rhs; body; _ } ->
                self#visit_expr bound rhs
                || self#visit_expr (bind_pats bound [ lhs ]) body
            | Closure { params; body; _ } ->
                self#visit_expr (bind_pats bound params) body
            | Loop { kind; state; _ } ->
                let pats =
                  (match kind with ForLoop { pat; _ } -> [ pat ] | _ -> [])
                  @ (Option.map ~f:(fun s -> s.bpat) state |> Option.to_list)
                in
                let bound = bind_pats bound pats in
                let bound =
                  match kind with
                  | ForIndexLoop { var; _ } -> Set.add bound var.name
                  | _ -> bound
                in
                super#visit_expr bound e
            | _ -> super#visit_expr bound e

          method! visit_arm bound arm =
            super#visit_arm (bind_pats bound [ arm.arm.arm_pat ]) arm
        end
          #visit_expr
          (Set.empty (module String))
          body

      let inline_matches =
        object (self)
          inherit [_] Visitors.map as super

          method! visit_expr () e =
            match e with
            | {
             e =
               Let
                 {
                   monadic = None;
                   lhs =
                     {
                       p =
                         PBinding
                           {
                             mut = Immutable;
                             mode = ByValue;
                             var;
                             subpat = None;
                             _;
                           };
                       _;
                     };
                   rhs;
                   body;
                 };
             _;
            }
              when Local_ident.is_side_effect_hoist_var var ->
                let body, count =
                  object
                    inherit [_] Visitors.mapreduce as super
                    method zero = 0
                    method plus = ( + )

                    method! visit_expr () e =
                      match e.e with
                      | LocalVar v when [%eq: Local_ident.t] v var -> (rhs, 1)
                      | _ -> super#visit_expr () e
                  end
                    #visit_expr
                    () body
                in
                let names =
                  U.Reducers.collect_local_idents#visit_expr () rhs
                  |> Set.map
                       (module String)
                       ~f:(fun (v : Local_ident.t) -> v.name)
                in
                if [%eq: int] count 1 && not (captured var names e) then
                  self#visit_expr () body
                else super#visit_expr () e
            | _ -> super#visit_expr () e
        end

      let ditems = List.map ~f:(inline_matches#visit_item ())
    end)
