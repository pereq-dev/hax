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

      (* `impl` return types are lowered to anonymous associated types. Those
         surfaced as trait/impl members (the non-GAT case) translate like named
         associated types; a reference to one that has no such member (a GAT)
         cannot be stated and rejects its item. *)
      let reject_anon_assoc_ty declared =
        let is_declared item =
          List.mem declared item ~equal:Concrete_ident.equal
        in
        object
          inherit [_] Visitors.map as super

          method! visit_ty span t =
            match t with
            | TAssociatedType { item; _ }
              when Concrete_ident.is_anon_assoc_ty item && not (is_declared item)
              ->
                Error.unimplemented ~issue_id:1965
                  ~details:
                    "`impl` types are not supported in type signatures of \
                     associated items."
                  (Option.value_exn span)
            | _ -> super#visit_ty span t

          method! visit_item _ i =
            try super#visit_item (Some i.span) i
            with Diagnostics.SpanFreeError.Exn (Data (context, kind)) ->
              let error = Diagnostics.pretty_print_context_kind context kind in
              let cast_item : item -> Ast.Full.item = Stdlib.Obj.magic in
              let ast = cast_item i |> Print_rust.pitem_str in
              let msg =
                error ^ "\nLast available AST for this item:\n\n" ^ ast
              in
              make_hax_error_item i.span i.ident msg
        end

      let ditems items =
        let declared =
          List.concat_map items ~f:(fun i ->
              let idents get l =
                List.filter_map l ~f:(fun x ->
                    let ident = get x in
                    Option.some_if
                      (Concrete_ident.is_anon_assoc_ty ident)
                      ident)
              in
              match i.v with
              | Trait { items; _ } -> idents (fun ti -> ti.ti_ident) items
              | Impl { items; _ } -> idents (fun ii -> ii.ii_ident) items
              | _ -> [])
        in
        List.map ~f:((reject_anon_assoc_ty declared)#visit_item None) items
    end)
