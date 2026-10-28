open! Prelude

module%inlined_contents Make (F : Features.T) = struct
  module FA = F
  module A = Ast.Make (F)
  module U = Ast_utils.Make (F)
  open Ast

  (* TODO: Swap to Concrete_ident see: https://github.com/pereq-dev/hax/issues/375 *)
  type analysis_data = concrete_ident list Map.M(String).t
  type id_order = int

  let analyse (items : A.item list) : analysis_data =
    let temp_list = List.concat_map ~f:U.functions_of_item items in
    List.fold_left
      ~init:(Map.empty (module String))
      ~f:(fun y (name, body) ->
        Map.set y
          ~key:([%show: Concrete_ident.View.t] (Concrete_ident.to_view name))
          ~data:
            (Set.to_list
               (U.Reducers.collect_concrete_idents#visit_expr () body)))
      temp_list
end
