open Hax_engine
open Base
open Stdio
open Utils

let setup_logs (options : Types.engine_options) =
  let level : Logs.level option =
    match options.backend.verbose with
    | 0 -> None
    | 1 -> Some Info
    | _ -> Some Debug
  in
  Logs.set_level level;
  Logs.set_reporter @@ Logs.format_reporter ()

module Deps = Dependencies.Make (Features.Rust)

module Error : Phase_utils.ERROR = Phase_utils.MakeError (struct
  let ctx = Diagnostics.Context.ThirImport
end)

module Attrs = Attr_payloads.MakeBase (Error)

let import_thir_items (include_clauses : Types.inclusion_clause list)
    (items : Types.item_for__thir_body list) : Ast.Rust.item list =
  let imported_items =
    List.map
      ~f:(fun item ->
        let ident = Concrete_ident.(of_def_id ~value:true item.owner_id) in
        let most_precise_clause =
          (* Computes the include clause that apply to `item`, if any *)
          List.filter
            ~f:(fun clause ->
              Concrete_ident.matches_namespace clause.Types.namespace ident)
            include_clauses
          |> List.last
        in
        let type_only =
          (* Shall we drop the body? *)
          Option.map
            ~f:(fun clause -> [%matches? Types.SignatureOnly] clause.kind)
            most_precise_clause
          |> Option.value ~default:false
        in
        Import_thir.import_item ~type_only item)
      items
    |> List.map ~f:snd
  in
  Logs.info (fun m -> m "Items translated");
  let items = List.concat_map ~f:fst imported_items in
  let associated_items =
    let assoc_items = Deps.uid_associated_items items in
    fun (item : Deps.AST.item) -> assoc_items item.attrs
  in
  (* Build a map from idents to error reports *)
  let ident_to_reports =
    List.concat_map
      ~f:(fun (items, reports) ->
        List.map ~f:(fun (item : Ast.Rust.item) -> (item.ident, reports)) items)
      imported_items
    |> Map.of_alist_exn (module Concrete_ident)
  in
  let items = Deps.filter_by_inclusion_clauses include_clauses items in
  let items =
    items
    @ (List.concat_map ~f:associated_items items
      |> List.filter ~f:(List.mem ~equal:[%eq: Deps.AST.item] items >> not))
  in
  let items =
    List.filter
      ~f:(fun i ->
        match Attrs.status i.attrs with Included _ -> true | _ -> false)
      items
  in
  Hax_io.write
    (ItemProcessed (List.filter_map ~f:(fun i -> Span.owner_hint i.span) items));
  (* Extract error reports for the items we actually extract *)
  let reports =
    List.concat_map
      ~f:(fun (item : Ast.Rust.item) ->
        Map.find_exn ident_to_reports item.ident)
      items
    |> List.dedup_and_sort ~compare:Diagnostics.compare
  in
  (* Report every error *)
  List.iter ~f:Diagnostics.Core.report reports;
  items

let run (options : Types.engine_options) : Types.output =
  setup_logs options;
  if options.backend.debug_engine |> Option.is_some then
    Phase_utils.DebugBindPhase.enable ();
  let run (type options_type)
      (module M : Backend.T with type BackendOptions.t = options_type)
      (backend_options : options_type) : Types.file list =
    let open M in
    Concrete_ident.ImplInfoStore.init
      (Concrete_ident_generated.impl_infos @ options.impl_infos);
    let include_clauses =
      options.backend.translation_options.include_namespaces
    in
    let input =
      match options.input with
      | Types.FullDef _ ->
          failwith "Internal error: the ocaml engine does not support FullDef"
      | Types.Legacy i -> i
    in
    let items =
      Profiling.profile ThirImport (List.length input) (fun _ ->
          import_thir_items include_clauses input)
    in
    let items =
      if options.backend.extract_type_aliases then items
      else
        List.filter
          ~f:(function { v = TyAlias _; _ } -> false | _ -> true)
          items
    in
    Logs.info (fun m ->
        m "Applying phase for backend %s"
          ([%show: Diagnostics.Backend.t] M.backend));
    let items = apply_phases backend_options items in
    let with_items = Attrs.with_items items in
    let bundles, _ =
      let module DepGraph = Dependencies.Make (InputLanguage) in
      DepGraph.recursive_bundles items
    in
    let items =
      List.filter items ~f:(fun (i : AST.item) ->
          Attrs.late_skip i.attrs |> not)
    in
    Logs.info (fun m ->
        m "Translating items with backend %s"
          ([%show: Diagnostics.Backend.t] M.backend));
    let items =
      Profiling.profile (Backend M.backend) (List.length items) (fun _ ->
          translate with_items backend_options items ~bundles)
    in
    items
  in
  let diagnostics, files =
    Diagnostics.try_ (fun () ->
        match options.backend.backend with
        | ProVerif opts -> run (module Proverif_backend) opts
        | Fstar opts -> run (module Fstar_backend) opts
        | Coq -> run (module Coq_backend) ()
        | Ssprove -> run (module Ssprove_backend) ()
        | Easycrypt -> run (module Easycrypt_backend) ()
        | GenerateRustEngineNames ->
            failwith
              "The OCaml hax engine should never be called with \
               `GenerateRustEngineNames`, it is an rust engine only internal \
               command."
        | backend ->
            failwith
              ("The OCaml hax engine should never be called with backend `"
              ^ [%show: Types.backend] backend
              ^ "`. This backend uses the newer rust engine. Please report \
                 this issue on our GitHub repository: \
                 https://github.com/pereq-dev/hax."))
  in
  {
    diagnostics = List.map ~f:Diagnostics.to_thir_diagnostic diagnostics;
    files = Option.value ~default:[] files;
    debug_json = [];
  }

(** Shallow parses a `id_table::Node<T>` (or a raw `T`) JSON *)
let parse_id_table_node (json : Yojson.Safe.t) :
    (int64 * Yojson.Safe.t) list * Yojson.Safe.t =
  let expect_uint64 = function
    | `Intlit str -> Some (Int64.of_string str)
    | `Int id -> Some (Int.to_int64 id)
    | _ -> None
  in
  let table, value =
    match json with
    | `List [ table; value ] -> (table, value)
    | _ -> failwith "parse_id_table_node: expected a tuple at top-level"
  in
  let table =
    match table with
    | `List json_list -> json_list
    | _ -> failwith "parse_id_table_node: `map` is supposed to be a list"
  in
  let table =
    List.map
      ~f:(function
        | `List [ id; `Assoc [ (_, contents) ] ] ->
            let id =
              expect_uint64 id
              |> Option.value_exn
                   ~message:"parse_id_table_node: id: expected int64"
            in
            (id, contents)
        | _ -> failwith "parse_id_table_node: expected a list of size two")
      table
  in
  (table, value)

let load_table ?(check_version = true) : Yojson.Safe.t =
  let table, json =
    Hax_io.read_json () |> Option.value_exn |> parse_id_table_node
  in
  (if check_version then
     let version =
       try Yojson.Safe.Util.(member "hax_version" json |> to_string)
       with _ -> "unknown"
     in
     if String.equal version Types.hax_version |> not then (
       prerr_endline
         [%string
           {|
The versions of `hax-engine` and of `cargo-hax` are different:
  - `hax-engine` version: %{Types.hax_version}
  - `cargo-hax`  version: %{version}

Please reinstall hax.
|}];
       Stdlib.exit 1));
  table
  |> List.iter ~f:(fun (id, json) ->
      Hashtbl.add_exn Types.cache_map ~key:id ~data:(`JSON json));
  json

let parse_options () =
  let json = load_table ~check_version:true in
  let options = [%of_yojson: Types.engine_options] json in
  Profiling.enabled := options.backend.profile;
  options

let send_debug_strings =
  Phase_utils.DebugBindPhase.export
  >> List.iter ~f:(fun json -> DebugString json |> Hax_io.write)

(** Entrypoint of the engine. Assumes `Hax_io.init` was called. *)
let engine () =
  let options = Profiling.profile (Other "parse_options") 1 parse_options in
  Printexc.record_backtrace true;
  let result =
    try Ok (run options) with
    | Hax_engine.Diagnostics.SpanFreeError.Exn exn ->
        Error
          ( Failure
              ("Uncatched hax exception (please report, this should not \
                appear): "
              ^ [%show: Hax_engine.Diagnostics.SpanFreeError.t] exn),
            Printexc.get_raw_backtrace () )
    | e -> Error (e, Printexc.get_raw_backtrace ())
  in
  match result with
  | Ok results ->
      List.iter
        ~f:(fun diag -> Diagnostic diag |> Hax_io.write)
        results.diagnostics;
      List.iter ~f:(fun file -> File file |> Hax_io.write) results.files;

      send_debug_strings ();
      Hax_io.close ();

      Logs.info (fun m -> m "Exiting Hax engine (success)")
  | Error (exn, bt) ->
      Logs.info (fun m -> m "Exiting Hax engine (with an unexpected failure)");
      Printexc.raise_with_backtrace exn bt

module ExportFullAst = Export_ast.Make (Features.Full)
module ExportRustAst = Export_ast.Make (Features.Rust)
module ExportLeanAst = Export_ast.Make (Lean_backend.InputLanguage)

let driver_for_rust_engine_inner (query : Rust_engine_types.query) :
    Rust_engine_types.response =
  Profiling.enabled := query.profiling;
  if query.debug_bind_phase then Phase_utils.DebugBindPhase.enable ();
  match query.kind with
  | Types.ImportThir { input; translation_options } ->
      let diagnostics, imported_items =
        Diagnostics.try_ (fun () ->
            import_thir_items translation_options.include_namespaces input)
      in
      let imported_items = Option.value ~default:[] imported_items in
      let rust_ast_items =
        List.concat_map ~f:ExportRustAst.ditem imported_items
      in
      Rust_engine_types.ImportThir
        {
          output = rust_ast_items;
          diagnostics = List.map ~f:Diagnostics.to_thir_diagnostic diagnostics;
        }
  | Types.ApplyPhases { input; phases } ->
      let items = List.concat_map ~f:Import_ast.ditem input in
      let module Phase =
        (val List.map
               ~f:(fun name ->
                 Untyped_phases.phase_of_name name |> Option.value_exn)
               phases
             |> Untyped_phases.bind_list)
      in
      let diagnostics, items =
        Diagnostics.capture (fun () -> Phase.ditems items)
      in
      let output = List.concat_map ~f:ExportFullAst.ditem items in
      Rust_engine_types.ApplyPhases
        {
          output;
          diagnostics = List.map ~f:Diagnostics.to_thir_diagnostic diagnostics;
        }
  | Types.Print { printer = Fstar backend_options; input } ->
      let open Fstar_backend in
      let items = List.concat_map ~f:Import_ast.ditem input in

      let items : AST.item list = Stdlib.Obj.magic items in
      let items = post_process_items items in
      let with_items = Attrs.with_items items in
      let bundles, _ =
        let module DepGraph = Dependencies.Make (InputLanguage) in
        DepGraph.recursive_bundles items
      in
      let items =
        List.filter items ~f:(fun (i : AST.item) ->
            Attrs.late_skip i.attrs |> not)
      in
      Logs.info (fun m ->
          m "Translating items with backend %s"
            ([%show: Diagnostics.Backend.t] Fstar_backend.backend));
      let files =
        Profiling.profile (Backend Fstar_backend.backend) (List.length items)
          (fun _ -> translate with_items backend_options items ~bundles)
      in
      List.iter ~f:(fun file -> File file |> Hax_io.write) files;
      Rust_engine_types.PrintOk
  | Types.Print _ ->
      failwith
        "Using the Ocaml engine for Printing only is reserved to the F* backend"

(** Entry point for interacting with the Rust hax engine *)
let driver_for_rust_engine () : unit =
  let query : Rust_engine_types.query =
    let json = load_table ~check_version:false in
    [%of_yojson: Rust_engine_types.query] json
  in
  Concrete_ident.ImplInfoStore.init
    (Concrete_ident_generated.impl_infos @ query.impl_infos);
  let response = driver_for_rust_engine_inner query in
  send_debug_strings ();
  Hax_io.write_json ([%yojson_of: Rust_engine_types.response] response);
  Hax_io.write_json ([%yojson_of: Types.from_engine] Exit)
