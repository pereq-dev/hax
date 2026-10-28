open! Prelude

module Make (F : Features.T) = struct
  module AST = Ast.Make (F)
  module U = Ast_utils.Make (F)
  open Ast
  open AST

  (** Get the identifier of an item *)
  let ident_of (item : item) : Concrete_ident.t = item.ident

  (** Get all the identifiers declared under an item. This includes the
      identifier of the item itself, but also of any sub-item: for instance,
      associated items within an impl. *)
  let idents_of (item : item) : Concrete_ident.t list =
    let is_field_anonymous ident =
      match List.last (Concrete_ident.to_view ident).mod_path with
      | Some { data = n; _ } -> Option.is_some (Int.of_string_opt n)
      | _ -> false
    in
    ident_of item
    ::
    (match item.v with
    | Type { variants; _ } ->
        List.concat_map
          ~f:(fun variant ->
            let fields =
              List.map ~f:fst3 variant.arguments
              |> List.filter ~f:(not << is_field_anonymous)
            in

            variant.name :: fields)
          variants
    | Trait { items; _ } -> List.map ~f:(fun item -> item.ti_ident) items
    | Impl { items; _ } -> List.map ~f:(fun item -> item.ii_ident) items
    | _ -> (* No sub items *) [])

  module Namespace = struct
    include Concrete_ident.View.ModPath
    module Set = Set.M (Concrete_ident.View.ModPath)

    let of_concrete_ident ci : t = (Concrete_ident.to_view ci).mod_path

    let to_string ?(sep = "::") : t -> string =
      List.map ~f:(fun (o : Concrete_ident_view.DisambiguatedString.t) ->
          o.data)
      >> String.concat ~sep
  end

  module Error : Phase_utils.ERROR = Phase_utils.MakeError (struct
    let ctx = Diagnostics.Context.Dependencies
  end)

  module Attrs = Attr_payloads.Make (F) (Error)

  let uid_associated_items (items : item list) : attrs -> item list =
    let open Attrs.WithItems (struct
      let items = items
    end) in
    raw_associated_item >> List.filter_map ~f:(snd >> try_item_of_uid)

  module ItemGraph = struct
    module G = Graph.Persistent.Digraph.Concrete (Concrete_ident)

    module GInt = struct
      include Graph.Persistent.Digraph.Concrete (Int)

      let empty () = empty
    end

    module Topological = Graph.Topological.Make_stable (GInt)
    module Map_G_GInt = Graph.Gmap.Edge (G) (GInt)
    module Oper = Graph.Oper.P (G)

    let vertices_of_item (i : item) : G.V.t list =
      let ( @ ) = Set.union in
      let v = U.Reducers.collect_concrete_idents in
      let concat_map f =
        List.map ~f >> Set.union_list (module Concrete_ident)
      in
      let set =
        match i.v with
        | Fn { name = _; generics; body; params; _ } ->
            v#visit_generics () generics
            @ v#visit_expr () body
            @ concat_map (v#visit_param ()) params
        | TyAlias { name = _; generics; ty } ->
            v#visit_generics () generics @ v#visit_ty () ty
        | Type { name = _; generics; variants; is_struct = (_ : bool) } ->
            v#visit_generics () generics
            @ concat_map (v#visit_variant ()) variants
        | IMacroInvokation { macro; argument = (_ : string); span; witness = _ }
          ->
            v#visit_concrete_ident () macro @ v#visit_span () span
        | Trait { name = _; generics; items; safety = _ } ->
            v#visit_generics () generics
            @ concat_map (v#visit_trait_item ()) items
        | Impl { generics; self_ty; of_trait; items; parent_bounds; safety = _ }
          ->
            v#visit_generics () generics
            @ v#visit_ty () self_ty
            @ v#visit_concrete_ident () (fst of_trait)
            @ concat_map (v#visit_generic_value ()) (snd of_trait)
            @ concat_map (v#visit_impl_item ()) items
            @ concat_map
                (fun (ie, ii) ->
                  v#visit_impl_expr () ie @ v#visit_impl_ident () ii)
                parent_bounds
        | Alias { name = _; item } -> v#visit_concrete_ident () item
        | Use _ | Quote _ | HaxError _ | NotImplementedYet ->
            Set.empty (module Concrete_ident)
      in
      set |> Set.to_list

    let vertices_of_items ~uid_associated_items (items : item list) : G.E.t list
        =
      List.concat_map
        ~f:(fun i ->
          let attrs = U.Reducers.collect_attrs#visit_item () i in
          let assoc =
            uid_associated_items attrs |> List.map ~f:(fun i -> i.ident)
          in
          vertices_of_item i @ assoc |> List.map ~f:(Fn.const i.ident &&& Fn.id))
        items

    let of_items ~original_items (items : item list) : G.t =
      let init =
        List.fold ~init:G.empty ~f:(fun g -> ident_of >> G.add_vertex g) items
      in
      let uid_associated_items = uid_associated_items original_items in
      vertices_of_items ~uid_associated_items items
      |> List.fold ~init ~f:(G.add_edge >> uncurry)

    let transitive_dependencies_of (g : G.t) (selection : Concrete_ident.t list)
        : Concrete_ident.t Hash_set.t =
      let set = Hash_set.create (module Concrete_ident) in
      let rec visit vertex =
        if Hash_set.mem set vertex |> not then (
          Hash_set.add set vertex;
          G.iter_succ visit g vertex)
      in
      List.filter ~f:(G.mem_vertex g) selection |> List.iter ~f:visit;
      set

    let transitive_dependencies_of_items ~original_items (items : item list)
        ?(graph = of_items ~original_items items)
        (selection : Concrete_ident.t list) : item list =
      let set = transitive_dependencies_of graph selection in
      items |> List.filter ~f:(ident_of >> Hash_set.mem set)

    module MutRec = struct
      module Bundle = struct
        type t = concrete_ident list

        let namespaces_of : t -> Namespace.Set.t =
          List.map ~f:Namespace.of_concrete_ident
          >> Set.of_list (module Namespace)

        let homogeneous_namespace (ns : t) : bool =
          Set.length (namespaces_of ns) <= 1
      end

      type t = {
        mut_rec_bundles : Bundle.t list;
        non_mut_rec : concrete_ident list;
      }

      module SCC = Graph.Components.Make (G)

      let of_graph (g : G.t) : t =
        let is_mut_rec_with_itself x = G.mem_edge g x x in
        let mut_rec_bundles, non_mut_rec =
          SCC.scc_list g
          |> List.partition_map ~f:(function
            | [] -> failwith "scc_list returned empty cluster"
            | [ x ] when is_mut_rec_with_itself x |> not -> Second x
            | bundle -> First bundle)
        in
        { mut_rec_bundles; non_mut_rec }

      let all_homogeneous_namespace (g : G.t) =
        List.for_all ~f:Bundle.homogeneous_namespace
          (of_graph g).mut_rec_bundles
    end

    module CyclicDep = struct
      module Bundle = struct
        type t = Concrete_ident.t list

        module G = Graph.Persistent.Graph.Concrete (Concrete_ident)
        module CC = Graph.Components.Undirected (G)

        let cycles g = CC.components_list g
      end

      (* This is a solution that bundles together everything that belongs to the same module SCC.
         It results in bundles that are much bigger than they could be but is a simple solution
         to the problem described in https://github.com/pereq-dev/hax/issues/995#issuecomment-2411114404 *)
      let of_mod_sccs (items : item list)
          (mod_graph_cycles : Namespace.Set.t list) : Bundle.t list =
        let item_names = List.map items ~f:(fun x -> x.ident) in
        let cycles =
          List.filter mod_graph_cycles ~f:(fun set -> Set.length set > 1)
        in
        let bundles =
          List.map cycles ~f:(fun set ->
              List.filter item_names ~f:(fun item ->
                  Set.mem set (Namespace.of_concrete_ident item)))
        in
        bundles
    end

    open Graph.Graphviz.Dot (struct
      include G

      let graph_attributes _ = []
      let default_vertex_attributes _ = []
      let vertex_name i = "\"" ^ Concrete_ident.show i ^ "\""

      let vertex_attributes i =
        [ `Label (Concrete_ident.DefaultViewAPI.render i).name ]

      let get_subgraph i =
        let ns = Namespace.of_concrete_ident i in
        let sg_name = Namespace.to_string ~sep:"__" ns in
        let label = Namespace.to_string ~sep:"::" ns in
        let open Graph.Graphviz.DotAttributes in
        Some { sg_name; sg_attributes = [ `Label label ]; sg_parent = None }

      let default_edge_attributes _ = []
      let edge_attributes _ = []
    end)

    let print oc items = output_graph oc (of_items ~original_items:items items)
  end

  module ModGraph = struct
    module G = Graph.Persistent.Digraph.Concrete (Namespace)

    let of_items (items : item list) : G.t =
      let ig = ItemGraph.of_items ~original_items:items items in
      let vertices =
        List.fold items ~init:G.empty ~f:(fun g item ->
            G.add_vertex g (Namespace.of_concrete_ident item.ident))
      in
      List.map ~f:(ident_of >> (Namespace.of_concrete_ident &&& Fn.id)) items
      |> Map.of_alist_multi (module Namespace)
      |> Map.map
           ~f:
             (List.concat_map
                ~f:
                  (ItemGraph.G.succ ig
                  >> List.map ~f:Namespace.of_concrete_ident)
             >> Set.of_list (module Namespace)
             >> Set.to_list)
      |> Map.to_alist
      |> List.concat_map ~f:(fun (x, ys) -> List.map ~f:(fun y -> (x, y)) ys)
      |> List.fold ~init:vertices ~f:(G.add_edge >> uncurry)

    module SCC = Graph.Components.Make (G)

    let cycles g : Namespace.Set.t list =
      SCC.scc_list g |> List.map ~f:(Set.of_list (module Namespace))

    (** Returns the namespaces in topological order *)
    let order g : Namespace.t list =
      let module ModTopo = Graph.Topological.Make_stable (G) in
      ModTopo.fold List.cons g []

    open Graph.Graphviz.Dot (struct
      include G

      let graph_attributes _ = []
      let default_vertex_attributes _ = []
      let vertex_name ns = "\"" ^ Namespace.to_string ns ^ "\""
      let vertex_attributes _ = []
      let get_subgraph _ = None
      let default_edge_attributes _ = []
      let edge_attributes _ = []
    end)

    let print oc items =
      let g = of_items items in
      let complicated_ones =
        SCC.scc_list g
        |> List.concat_map ~f:(function [] | [ _ ] -> [] | bundle -> bundle)
      in
      let g =
        List.concat_map
          ~f:(fun ns ->
            List.map
              ~f:(fun y -> (ns, y))
              (G.succ g ns
              |> List.filter
                   ~f:(List.mem ~equal:[%equal: Namespace.t] complicated_ones)))
          complicated_ones
        |> List.fold ~init:G.empty ~f:(G.add_edge >> uncurry)
      in
      output_graph oc g
  end

  let ident_list_to_string =
    List.map ~f:Concrete_ident.DefaultViewAPI.show >> String.concat ~sep:", "

  let sort (items : item list) : item list =
    let g =
      ItemGraph.of_items ~original_items:items items |> ItemGraph.Oper.mirror
    in
    let stable_g =
      let to_index =
        items
        |> List.mapi ~f:(fun i item -> (item.ident, i))
        |> Map.of_alist_exn (module Concrete_ident)
        |> Map.find
      in
      ItemGraph.Map_G_GInt.filter_map
        (to_index *** to_index >> uncurry Option.both)
        g
    in
    let stable_g =
      List.foldi items ~init:stable_g ~f:(fun i g _ ->
          ItemGraph.GInt.add_vertex g i)
    in
    let items' =
      let items_array = Array.of_list items in
      let lookup (index : int) = items_array.(index) in
      ItemGraph.Topological.fold List.cons stable_g [] |> List.map ~f:lookup
    in
    (* Stable topological sort doesn't guarantee to group cycles together.
       We make this correction to ensure mutually recursive items are grouped. *)
    let items' =
      let cycles =
        ItemGraph.MutRec.SCC.scc_list g
        |> List.filter ~f:(fun cycle -> List.length cycle > 1)
      in
      (* TODO: This can be optimized by using a set or a map
         to avoid traversing all cycles at each iteration. *)
      List.fold items' ~init:[] ~f:(fun acc item ->
          match
            List.find cycles ~f:(fun cycle ->
                List.mem cycle item.ident ~equal:[%eq: concrete_ident])
          with
          | Some _
            when List.exists acc ~f:(fun els ->
                     List.mem els item ~equal:[%eq: item]) ->
              [] :: acc
          | Some cycle ->
              List.map cycle ~f:(fun ident ->
                  List.find_exn items ~f:(fun item ->
                      [%eq: concrete_ident] item.ident ident))
              :: acc
          | None -> [ item ] :: acc)
      |> List.concat
    in
    (* Quote items must be placed right before or after their origin *)
    let items' =
      let before_quotes, after_quotes, _ =
        List.partition3_map items' ~f:(fun item ->
            match item.v with
            | Quote { origin; _ } -> (
                match origin.position with
                | `Before -> `Fst (origin, item)
                | `After -> `Snd (origin, item)
                | `Replace -> `Trd ())
            | _ -> `Trd ())
      in
      let move_quote before origin quote_item =
        List.concat_map ~f:(fun item ->
            if [%eq: concrete_ident] origin.item_ident item.ident then
              if before then [ quote_item; item ] else [ item; quote_item ]
            else if [%eq: concrete_ident] quote_item.ident item.ident then []
            else [ item ])
      in
      let before_quotes = List.rev before_quotes in
      let items' =
        List.fold before_quotes ~init:items'
          ~f:(fun items' (origin, quote_item) ->
            move_quote true origin quote_item items')
      in
      List.fold after_quotes ~init:items' ~f:(fun items' (origin, quote_item) ->
          move_quote false origin quote_item items')
    in

    assert (
      let of_list =
        List.map ~f:ident_of >> Set.of_list (module Concrete_ident)
      in
      let items = of_list items in
      let items' = of_list items' in
      Set.equal items items');
    items'

  (** Sort within each namespaces: items are first grouped by namespace, then
      sorted topologically. *)
  let sort_namespace_wise (items : item list) : item list =
    let sorted_by_namespace =
      U.group_items_by_namespace items
      |> Map.data
      |> List.map ~f:(fun items -> sort items)
    in
    let sorted_namespaces = ModGraph.order (ModGraph.of_items items) in
    List.concat_map sorted_namespaces ~f:(fun namespace ->
        List.find sorted_by_namespace ~f:(fun items ->
            List.exists items ~f:(fun item ->
                Namespace.equal
                  (Namespace.of_concrete_ident item.ident)
                  namespace))
        |> Option.value ~default:[])

  let filter_by_inclusion_clauses' ~original_items
      (clauses : Types.inclusion_clause list) (items : item list) :
      item list * Concrete_ident.t Hash_set.t =
    let graph = ItemGraph.of_items ~original_items items in
    let of_list = Set.of_list (module Concrete_ident) in
    let selection = List.map ~f:ident_of items |> of_list in
    let deps_of =
      let to_set = Hash_set.to_list >> of_list in
      Set.to_list >> ItemGraph.transitive_dependencies_of graph >> to_set
    in
    let show_ident_set =
      Set.to_list
      >> List.map ~f:Concrete_ident.DefaultViewAPI.show
      >> List.map ~f:(fun s -> " - " ^ s)
      >> String.concat ~sep:"\n"
    in
    let show_inclusion_clause Types.{ kind; namespace } =
      (match kind with
        | Excluded -> "-"
        | SignatureOnly -> "+:"
        | Included deps_kind -> (
            match deps_kind with
            | Transitive -> "+"
            | Shallow -> "+~"
            | None' -> "+!"))
      ^ "["
      ^ (List.map
           ~f:(function Glob One -> "*" | Glob Many -> "**" | Exact s -> s)
           namespace.chunks
        |> String.concat ~sep:"::")
      ^ "]"
    in
    let hax_lib_include =
      let id_to_include =
        Hashtbl.of_alist_exn
          (module Concrete_ident)
          (List.map
             ~f:(fun it ->
               ( it.ident,
                 Attrs.find_unique_attr
                   ~f:(function Types.ItemStatus is -> Some is | _ -> None)
                   it.attrs ))
             items)
      in
      Hashtbl.find id_to_include >> Option.join
    in

    let items_drop_body = Hash_set.create (module Concrete_ident) in
    let apply_clause selection' (clause : Types.inclusion_clause) =
      let matches = Concrete_ident.matches_namespace clause.Types.namespace in
      let matched0 = Set.filter ~f:matches selection in
      let with_deps, drop_bodies =
        match clause.kind with
        | Included Transitive -> (true, false)
        | Included Shallow -> (true, true)
        | Included None' -> (false, false)
        | SignatureOnly -> (false, true)
        | Excluded -> (false, false)
      in
      let matched = matched0 |> if with_deps then deps_of else Fn.id in
      if drop_bodies then (
        Set.iter ~f:(Hash_set.add items_drop_body) matched;
        Set.iter ~f:(Hash_set.remove items_drop_body) matched0);
      Logs.info (fun m ->
          m "The clause [%s] will %s the following Rust items:\n%s"
            (show_inclusion_clause clause)
            (match clause.kind with Excluded -> "remove" | _ -> "add")
          @@ show_ident_set matched);
      let set_op =
        match clause.kind with
        | Included _ | SignatureOnly -> Set.union
        | Excluded -> Set.diff
      in
      let result = set_op selection' matched in
      let forced_include =
        selection'
        |> Set.filter
             ~f:
               (hax_lib_include
               >> [%eq: Types.ha_item_status option]
                    (Some (Included { late_skip = false })))
      in
      Set.union forced_include result
    in
    let selection = List.fold ~init:selection ~f:apply_clause clauses in
    Logs.info (fun m ->
        m "The following Rust items are going to be extracted:\n%s"
        @@ show_ident_set selection);
    (List.filter ~f:(ident_of >> Set.mem selection) items, items_drop_body)

  let filter_by_inclusion_clauses (clauses : Types.inclusion_clause list)
      (items : item list) : item list =
    let f = filter_by_inclusion_clauses' ~original_items:items clauses in
    let selection =
      let items', items_drop_body = f items in
      let items', _ =
        (* when one includes only shallow dependencies, we just remove bodies *)
        List.map
          ~f:(fun item ->
            if Hash_set.mem items_drop_body (ident_of item) then
              U.Mappers.drop_bodies#visit_item () item
            else item)
          items'
        |> f
      in
      List.map ~f:ident_of items' |> Set.of_list (module Concrete_ident)
    in
    List.filter ~f:(ident_of >> Set.mem selection) items

  let fresh_module_for (bundle : item list) =
    let fresh_module =
      Concrete_ident.fresh_module ~label:"bundle" (List.map ~f:ident_of bundle)
    in
    (* Items whose definition is supplied by a `Replace` quote living in this
       same bundle (i.e. `#[hax_lib::<backend>::replace(..)]`). Such an item is
       late-skipped — the quote's verbatim text is emitted in its stead — but the
       name it declares *is* still defined, and other modules reference it. So
       its re-export must survive even though the item it aliases does not; see
       the `attrs` computation below. *)
    let replaced_by_quote =
      List.filter_map bundle ~f:(fun item ->
          match item.v with
          | Quote { origin = { position = `Replace; item_ident; _ }; _ } ->
              Some item_ident
          | _ -> None)
      |> Set.of_list (module Concrete_ident)
    in
    let renamings =
      bundle
      (* Exclude `Use` items: we exclude those from bundling since they are only
         user hints. `Use` items don't have proper identifiers, and those
         identifiers are never referenced by other Rust items. *)
      |> List.filter ~f:(function { v = Use _; _ } -> false | _ -> true)
      (* Exclude `NotImplementedYet` items *)
      |> List.filter ~f:(function
        | { v = NotImplementedYet; _ } -> false
        | _ -> true)
      |> List.concat_map ~f:(fun item ->
          List.map
            ~f:(fun id ->
              (item, (id, Concrete_ident.move_to_fresh_module fresh_module id)))
            (idents_of item))
    in
    let aliases =
      let inspect_view_last id =
        List.last (Concrete_ident.to_view id).rel_path
      in
      List.filter_map renamings ~f:(fun (origin_item, (from_id, to_id)) ->
          let attrs =
            (* An alias inherits its origin's `late_skip` so that a skipped item
               does not leave a dangling re-export behind. A quote-replaced item
               is the exception: it is skipped precisely because the quote
               defines the name in its place, so the re-export has to stay. *)
            if Set.mem replaced_by_quote from_id then []
            else
              List.filter
                ~f:(fun att -> Attrs.late_skip [ att ])
                origin_item.attrs
          in
          let v = Alias { name = from_id; item = to_id } in
          match origin_item.v with
          (* We don't want to aliases for constructors of structs with named fields because
             they can't be imported in F*. Ideally this should be handled by the backend. *)
          | Type { variants; is_struct = true; _ }
            when List.for_all variants ~f:(fun variant -> variant.is_record)
                 && Concrete_ident.is_constructor from_id ->
              None
          (* We don't need aliases for fields of types. *)
          | Type _ when [%matches? Some (`Field _)] (inspect_view_last from_id)
            ->
              None
          (* We don't need aliases for methods of trait impls. *)
          | Impl _
            when [%matches? Some (`AssociatedItem _)]
                   (inspect_view_last from_id) ->
              None
          | Quote _ -> None
          | _ -> Some { attrs; span = origin_item.span; ident = from_id; v })
    in
    let rename =
      let renamings = List.map ~f:snd renamings in
      let renamings =
        match Map.of_alist (module Concrete_ident) renamings with
        | `Duplicate_key dup ->
            failwith
              [%string
                "Fatal error: in dependency analysis, we construct a renaming \
                 key-value list with a guarantee of unicity in keys. However, \
                 we found the following key twice:\n\
                 %{[%show: concrete_ident] dup}"]
        | `Ok value -> value
      in
      let renamer _lvl i = Map.find renamings i |> Option.value ~default:i in
      (U.Mappers.rename_concrete_idents renamer)#visit_item ExprLevel
    in
    List.map ~f:rename bundle @ aliases

  let bundle_cyclic_modules (items : item list) : item list =
    (* [module_level_scc] is a list of set of strongly connected modules. *)
    let module_level_scc = ModGraph.(of_items >> cycles) items in
    let items_per_ns =
      List.map ~f:(fun i -> (Namespace.of_concrete_ident i.ident, i)) items
      |> Map.of_alist_multi (module Namespace)
    in
    let items_of_ns = Map.find items_per_ns >> Option.value ~default:[] in
    module_level_scc
    |> List.concat_map ~f:(fun nss ->
        let multiple_heterogeneous_modules = Set.length nss > 1 in
        let items = Set.to_list nss |> List.concat_map ~f:items_of_ns in
        if multiple_heterogeneous_modules then fresh_module_for items else items)

  let recursive_bundles (items : item list) : item list list * item list =
    let g = ItemGraph.of_items ~original_items:items items in
    let bundles = ItemGraph.MutRec.of_graph g in
    let from_ident ident : item option =
      List.find ~f:(fun i -> [%equal: Concrete_ident.t] i.ident ident) items
    in
    let f = List.filter_map ~f:from_ident in
    (List.map ~f bundles.mut_rec_bundles, f bundles.non_mut_rec)
end
