open Hax_engine
open Utils
open Base
open Coq_ast

include
  Backend.Make
    (struct
      open Features
      include Off
      include On.Slice
      include On.Monadic_binding
      include On.Macro
      include On.Construct_base
      include On.Loop
      include On.For_loop
      include On.While_loop
      include On.For_index_loop
      include On.State_passing_loop
      include On.Fold_like_loop
    end)
    (struct
      let backend = Diagnostics.Backend.SSProve
    end)

module SubtypeToInputLanguage
    (FA :
      Features.T
        with type mutable_reference = Features.Off.mutable_reference
         and type continue = Features.Off.continue
         and type break = Features.Off.break
         and type mutable_pointer = Features.Off.mutable_pointer
         and type mutable_variable = Features.Off.mutable_variable
         and type reference = Features.Off.reference
         and type raw_pointer = Features.Off.raw_pointer
         and type early_exit = Features.Off.early_exit
         and type question_mark = Features.Off.question_mark
         and type as_pattern = Features.Off.as_pattern
         and type lifetime = Features.Off.lifetime
         and type monadic_action = Features.Off.monadic_action
         and type arbitrary_lhs = Features.Off.arbitrary_lhs
         and type nontrivial_lhs = Features.Off.nontrivial_lhs
         and type quote = Features.Off.quote
         and type block = Features.Off.block
         and type dyn = Features.Off.dyn
         and type match_guard = Features.Off.match_guard
         and type trait_item_default = Features.Off.trait_item_default
         and type unsafe = Features.Off.unsafe) =
struct
  module FB = InputLanguage

  include
    Subtype.Make (FA) (FB)
      (struct
        module A = FA
        module B = FB
        include Features.SUBTYPE.Id
        include Features.SUBTYPE.On.Monadic_binding
        include Features.SUBTYPE.On.Construct_base
        include Features.SUBTYPE.On.Slice
        include Features.SUBTYPE.On.Macro
        include Features.SUBTYPE.On.Loop
        include Features.SUBTYPE.On.For_loop
        include Features.SUBTYPE.On.While_loop
        include Features.SUBTYPE.On.For_index_loop
        include Features.SUBTYPE.On.State_passing_loop
        include Features.SUBTYPE.On.Fold_like_loop
      end)

  let metadata = Phase_utils.Metadata.make (Reject (NotInBackendLang backend))
end

module AST = Ast.Make (InputLanguage)
module BackendOptions = Backend.UnitBackendOptions
open Ast

module CoqNamePolicy = struct
  include Concrete_ident.DefaultNamePolicy

  let reserved_words = Hash_set.of_list (module String) [ "left"; "right" ]
  (* let temp = Hash_set.create (module String) in *)
  (* temp *)
  (* Hash_set.add temp "left" *)
end

module U = Ast_utils.Make (InputLanguage)
module RenderId = Concrete_ident.MakeRenderAPI (CoqNamePolicy)
open AST

module SSProveLibrary : Library = struct
  module Notation = struct
    let int_repr (_x : string) (i : string) : string = i
    let type_str : string = "choice_type"
    let bool_str : string = "'bool"
    let unit_str : string = "'unit"
  end
end

module SSP = Coq (SSProveLibrary)

module SSPExtraDefinitions (* : ANALYSIS *) = struct
  let wrap_type_in_both (a : SSP.AST.ty) =
    SSP.AST.AppTy (SSP.AST.NameTy "both", [ a ])

  let unit_term : SSP.AST.term =
    SSP.AST.TypedTerm (SSP.AST.UnitTerm, SSP.AST.Unit)

  let rec variables_of_ssp_pat (p : SSP.AST.pat) : string list =
    match p with
    | RecordPat (_, npl) -> List.concat_map ~f:(snd >> variables_of_ssp_pat) npl
    | ConstructorPat (_, pl) -> List.concat_map ~f:variables_of_ssp_pat pl
    | TuplePat pl -> List.concat_map ~f:variables_of_ssp_pat pl
    | AscriptionPat (p, _) -> variables_of_ssp_pat p
    | Ident x -> [ x ]
    | DisjunctivePat pl -> List.concat_map ~f:variables_of_ssp_pat pl
    | WildPat | UnitPat | Lit _ -> []

  let letb
      ({ pattern; mut; value; body; value_typ; monad_typ } : SSP.AST.let_args) :
      SSP.AST.term =
    match monad_typ with
    | Some (Exception _typ) ->
        SSP.AST.AppFormat
          ( [
              "letm[choice_typeMonad.result_bind_code ";
              (*typ*)
              "] ";
              (*p*)
              " := ";
              (*expr*)
              " in";
              "";
              (*body*)
              "";
            ],
            [
              SSP.AST.Typing (value_typ, true, 0);
              SSP.AST.Variable (pattern, 0);
              SSP.AST.Value (value, false, 0);
              SSP.AST.Newline 0;
              SSP.AST.Value (body, false, 0);
            ] )
    | Some (Result _typ) ->
        SSP.AST.AppFormat
          ( [
              "letm[choice_typeMonad.result_bind_code ";
              (*typ*)
              "] ";
              (*p*)
              " := ";
              (*expr*)
              " in";
              "";
              (*body*)
              "";
            ],
            [
              SSP.AST.Typing (value_typ, true, 0);
              SSP.AST.Variable (pattern, 0);
              SSP.AST.Value (value, false, 0);
              SSP.AST.Newline 0;
              SSP.AST.Value (body, false, 0);
            ] )
    | Some Option ->
        SSP.AST.AppFormat
          ( [
              "letm[choice_typeMonad.option_bind_code] ";
              (*p*)
              " := ";
              (*expr*)
              " in";
              "";
              (*body*)
              "";
            ],
            [
              SSP.AST.Variable (pattern, 0);
              SSP.AST.Value (value, false, 0);
              SSP.AST.Newline 0;
              SSP.AST.Value (body, false, 0);
            ] )
    | None ->
        if mut then
          SSP.AST.AppFormat
            ( [
                "letb ";
                (*p*)
                " loc(" (*p_loc*);
                ") := ";
                (*expr*)
                " in";
                "";
                (*body*)
                "";
              ],
              [
                SSP.AST.Variable (pattern, 0);
                SSP.AST.Variable
                  ( (match
                       List.map
                         ~f:(fun x -> SSP.AST.Ident (x ^ "_loc"))
                         (variables_of_ssp_pat pattern)
                     with
                    | [] -> SSP.AST.WildPat
                    | [ x ] -> x
                    | xs -> SSP.AST.TuplePat xs),
                    0 );
                SSP.AST.Value (value, false, 0);
                SSP.AST.Newline 0;
                SSP.AST.Value (body, false, 0);
              ] )
        else
          SSP.AST.AppFormat
            ( [ "letb "; (*p*) " := "; (*expr*) " in"; ""; (*body*) "" ],
              [
                SSP.AST.Variable (pattern, 0);
                SSP.AST.Value (value, false, 0);
                SSP.AST.Newline 0;
                SSP.AST.Value (body, false, 0);
              ] )

  let rec pat_as_expr (p : SSP.AST.pat) : (SSP.AST.pat * SSP.AST.term) option =
    match p with
    | WildPat | UnitPat -> None
    | Ident s -> Some (SSP.AST.Ident s, SSP.AST.Var s)
    | Lit l -> Some (SSP.AST.Lit l, Const l)
    | RecordPat (s, sps) ->
        let v =
          List.filter_map
            ~f:(fun (s, ps) ->
              Option.map ~f:(fun (p, t) -> ((s, p), (s, t))) (pat_as_expr ps))
            sps
        in
        Some
          ( SSP.AST.RecordPat (s, List.map ~f:fst v),
            SSP.AST.RecordConstructor (s, List.map ~f:snd v) )
    | ConstructorPat (_, ps) | TuplePat ps ->
        let pt_list = List.filter_map ~f:pat_as_expr ps in
        Some
          ( TuplePat (List.map ~f:fst pt_list),
            SSP.AST.Tuple (List.map ~f:snd pt_list) )
    | AscriptionPat (p, _) -> pat_as_expr p (* TypedTerm (, t) *)
    | DisjunctivePat ps ->
        let pt_list = List.filter_map ~f:pat_as_expr ps in
        Some
          ( TuplePat (List.map ~f:fst pt_list),
            SSP.AST.Tuple (List.map ~f:snd pt_list) )

  let ifb ((cond, then_, else_) : SSP.AST.term * SSP.AST.term * SSP.AST.term) :
      SSP.AST.term =
    SSP.AST.AppFormat
      ( [ "ifb "; (*expr*) ""; "then "; ""; "else "; "" ],
        [
          SSP.AST.Value (cond, false, 0);
          SSP.AST.Newline 0;
          SSP.AST.Value (then_, false, 0);
          SSP.AST.Newline 0;
          SSP.AST.Value (else_, false, 0);
        ] )

  let matchb ((expr, arms) : SSP.AST.term * (SSP.AST.pat * SSP.AST.term) list) :
      SSP.AST.term =
    SSP.AST.AppFormat
      ( [ "matchb "; (*expr*) " with" ]
        @ List.concat_map ~f:(fun _ -> [ "| "; " =>"; ""; "" ]) arms
        @ [ "end" ],
        [ SSP.AST.Value (expr, false, 0); SSP.AST.Newline 0 ]
        @ List.concat_map
            ~f:(fun (arm_pat, body) ->
              [
                SSP.AST.Variable (arm_pat, 0);
                SSP.AST.Newline 1;
                SSP.AST.Value (body, false, 1);
                SSP.AST.Newline 0;
              ])
            arms )

  let updatable_record
      ((name, arguments, variants) :
        string * SSP.AST.argument list * SSP.AST.record_field list) :
      SSP.AST.decl =
    let fields =
      List.concat_map
        ~f:(function
          | SSP.AST.Named (x, y) -> [ (x, y) ] | SSP.AST.Coercion _ -> [])
        variants
    in
    let ty_name =
      "("
      ^ String.concat ~sep:" "
          (name
          :: List.filter_map
               ~f:(fun x ->
                 match x with
                 | SSP.AST.Explicit (p, _t) ->
                     Some (SSP.pat_to_string p false 0)
                 | _ -> None)
               arguments)
      ^ ")"
    in
    SSP.AST.MultipleDecls
      ([
         SSP.AST.Definition
           ( name,
             arguments,
             SSP.AST.Type (SSP.AST.Product (List.map ~f:snd fields)),
             SSP.AST.TypeTy );
       ]
      @ List.mapi
          ~f:(fun i (x, y) ->
            SSP.AST.Equations
              ( x,
                List.map
                  ~f:(function
                    | SSP.AST.Explicit (a, b) -> SSP.AST.Implicit (a, b)
                    | v -> v)
                  arguments
                @ [
                    SSP.AST.Explicit
                      ( SSP.AST.Ident "s",
                        wrap_type_in_both (SSP.AST.NameTy name) );
                  ],
                SSP.AST.App
                  ( SSP.AST.Var "bind_both",
                    [
                      SSP.AST.Var "s";
                      SSP.AST.Lambda
                        ( [ SSP.AST.Ident "x" ],
                          (* SSP.AST.App *)
                          (*   ( SSP.AST.Var "solve_lift", *)
                          (* [ *)
                          SSP.AST.App
                            ( SSP.AST.Var "ret_both",
                              [
                                SSP.AST.TypedTerm
                                  ( List.fold_right ~init:(SSP.AST.Var "x")
                                      ~f:(fun x y ->
                                        SSP.AST.App (SSP.AST.Var x, [ y ]))
                                      ((if Stdlib.(i != 0) then [ "snd" ]
                                        else [])
                                      @ List.init
                                          (List.length fields - 1 - i)
                                          ~f:(fun _ -> "fst")),
                                    y );
                              ] )
                          (* ] ) *) );
                    ] ),
                wrap_type_in_both y ))
          fields
      @ [
          SSP.AST.Equations
            ( "Build_" ^ name,
              List.map
                ~f:(function
                  | SSP.AST.Explicit (a, b) -> SSP.AST.Implicit (a, b) | v -> v)
                arguments
              @ List.mapi
                  ~f:(fun i (x, y) ->
                    SSP.AST.Implicit (SSP.AST.Ident x, wrap_type_in_both y))
                  fields,
              List.fold_left
                ~init:
                  ((* SSP.AST.App *)
                   (*   ( SSP.AST.Var "solve_lift", *)
                   (* [ *)
                     SSP.AST.App
                     ( SSP.AST.Var "ret_both",
                       [
                         SSP.AST.TypedTerm
                           ( SSP.AST.Tuple
                               (List.map
                                  ~f:(fst >> fun x -> SSP.AST.Var x)
                                  fields),
                             SSP.AST.NameTy ty_name );
                       ] )
                     (* ] ) *))
                ~f:(fun z (x, _y) ->
                  SSP.AST.App
                    ( SSP.AST.Var "bind_both",
                      [ SSP.AST.Var x; SSP.AST.Lambda ([ SSP.AST.Ident x ], z) ]
                    ))
                fields,
              SSP.AST.NameTy ("both" ^ " " ^ ty_name) )
          (* :: SSP.AST.Arguments ("Build_" ^ pconcrete_ident name,) *);
        ]
      (* @ [SSP.AST.ProgramInstance *)
      (*      (\* (name, arguments, self_ty, ty_list, impl_list) *\) *)
      (*      ( "Settable", *)
      (*        [], *)
      (*        SSP.AST.NameTy name, *)
      (*        [wrap_type_in_both (SSP.AST.NameTy name)], *)
      (*        SSP.AST.InstanceDecls [SSP.AST.LetDef ("mkT", [], *)
      (*          SSP.AST.App (SSP.AST.Var "fun x => ", [ *)
      (*          List.fold_left *)
      (*           ~init: *)
      (*             ((\* SSP.AST.App *\) *)
      (*              (\*   ( SSP.AST.Var "solve_lift", *\) *)
      (*                  (\* [ *\) *)
      (*                    SSP.AST.App *)
      (*                      ( SSP.AST.Var "ret_both", *)
      (*                        [ *)
      (*                          SSP.AST.TypedTerm *)
      (*                            ( SSP.AST.Tuple *)
      (*                                (List.map *)
      (*                                   ~f:(fst >> fun x -> SSP.AST.Var x) *)
      (*                                   fields), *)
      (*                              SSP.AST.NameTy ty_name ); *)
      (*                        ] ); *)
      (*                    (\* ] ) *\)) *)
      (*           ~f:(fun z (x, _y) -> *)
      (*             SSP.AST.App *)
      (*               ( SSP.AST.Var "bind_both", *)
      (*                 [ SSP.AST.App (SSP.AST.Var x , [ (SSP.AST.Var "x") ] ); SSP.AST.Lambda ([ SSP.AST.Ident x ], z) ] *)
      (*               )) *)
      (*           fields]), *)
      (*          SSP.AST.WildTy)] *)
      (*      ) *)
      (*   ] *)
      @ List.mapi
          ~f:(fun i (x, _y) ->
            SSP.AST.Notation
              ( "'Build_" ^ name ^ "'" ^ " " ^ "'['" ^ " " ^ "x" ^ " " ^ "']'"
                ^ " " ^ "'('" ^ " " ^ "'" ^ x ^ "'" ^ " " ^ "':='" ^ " " ^ "y"
                ^ " " ^ "')'",
                SSP.AST.App
                  ( SSP.AST.Var ("Build_" ^ name),
                    List.mapi
                      ~f:(fun j (x, _y) ->
                        SSP.AST.AppFormat
                          ( [ x ^ " " ^ ":=" ^ " "; (*v*) "" ],
                            [
                              SSP.AST.Value
                                ( (if Stdlib.(j == i) then SSP.AST.Var "y"
                                   else
                                     SSP.AST.App
                                       (SSP.AST.Var x, [ SSP.AST.Var "x" ])),
                                  false,
                                  0 );
                            ] ))
                      fields ),
                None ))
          fields)

  let both_enum
      ((name, arguments, cases) :
        string * SSP.AST.argument list * SSP.AST.inductive_case list) :
      SSP.AST.decl =
    SSP.AST.MultipleDecls
      ((* Type definition *)
       SSP.AST.Definition
         ( (* "t_" ^ *) name,
           arguments,
           SSP.AST.Type
             (SSP.AST.Coproduct
                (List.map
                   ~f:(function
                     | BaseCase _ -> SSP.AST.Unit
                     | InductiveCase (_, typ) -> typ)
                   cases))
           (* (SSP.AST.NameTy ("chFin (mkpos " ^ number_of_cases ^ ")")) *),
           SSP.AST.TypeTy )
       (* Index names and constructors *)
      :: List.concat_mapi cases ~f:(fun i c ->
          let v_name, curr_typ =
            match c with
            | BaseCase v_name -> (v_name, [])
            | InductiveCase (v_name, typ) -> (v_name, [ typ ])
          in
          let injections inner_val =
            List.fold_left ~init:inner_val
              ~f:(fun y x -> SSP.AST.App (SSP.AST.Var x, [ y ]))
              ((if Stdlib.(i != 0) then [ "inr" ] else [])
              @ List.init (List.length cases - 1 - i) ~f:(fun _ -> "inl"))
          in
          let definition_body =
            let inject_argument inner_val =
              (* SSP.AST.App *)
              (*   ( SSP.AST.Var "solve_lift", *)
              (* [ *)
              SSP.AST.App
                ( SSP.AST.Var "ret_both",
                  [
                    SSP.AST.TypedTerm (injections inner_val, SSP.AST.NameTy name);
                  ] )
              (* ] ) *)
            in
            match curr_typ with
            | [] -> inject_argument unit_term
            | _ ->
                SSP.AST.App
                  ( SSP.AST.Var "bind_both",
                    [
                      SSP.AST.Var "x";
                      SSP.AST.Lambda
                        ( [ SSP.AST.Ident "x" ],
                          inject_argument (SSP.AST.Var "x") );
                    ] )
          in
          [
            (let arg, body =
               match curr_typ with
               | [] ->
                   ("", injections SSP.AST.UnitTerm)
                   (* TODO: Fix unit translation *)
               | _ -> (" " ^ "x", injections (SSP.AST.Var "x"))
             in
             SSP.AST.Notation
               ("'" ^ v_name ^ "_case" ^ "'" ^ arg, body, Some "at level 100"));
            SSP.AST.Equations
              ( v_name,
                List.map
                  ~f:(fun x ->
                    SSP.AST.Explicit (SSP.AST.Ident "x", wrap_type_in_both x))
                  curr_typ,
                definition_body,
                wrap_type_in_both (SSP.AST.NameTy name) );
          ]))
end

module StaticAnalysis (* : ANALYSIS *) = struct
  module FunctionDependency (* : ANALYSIS *) =
    [%functor_application
    Analyses.Function_dependency InputLanguage]

  module MutableVariables (* : ANALYSIS *) =
    [%functor_application
    Analyses.Mutable_variables InputLanguage]

  type analysis_data = { mut_var : MutableVariables.analysis_data }

  let analyse items =
    let func_dep = FunctionDependency.analyse items in
    let mut_var =
      MutableVariables.analyse (func_dep : MutableVariables.pre_data) items
    in
    { mut_var }
end

module Context = struct
  type t = {
    current_namespace : string list;
    analysis_data : StaticAnalysis.analysis_data;
  }
end

let primitive_to_string (id : Ast.primitive_ident) : string =
  match id with
  | Deref -> "(TODO: Deref)" (* failwith "Deref" *)
  | Cast -> "cast_int (WS2 := _)" (* failwith "Cast" *)
  | LogicalOp op -> ( match op with And -> "andb" | Or -> "orb")

open Phase_utils

module TransformToInputLanguage =
  [%functor_application
    Phases.Reject.Unsafe(Features.Rust)
    |> Phases.Rewrite_local_self
    |> Phases.Reject.RawOrMutPointer
    |> Phases.And_mut_defsite
    |> Phases.Reconstruct_asserts
    |> Phases.Reconstruct_for_loops
    |> Phases.Direct_and_mut
    |> Phases.Reject.Arbitrary_lhs
    |> Phases.Drop_blocks
    |> Phases.Drop_match_guards
    |> Phases.Reject.Continue
    |> Phases.Drop_references
    |> Phases.Trivialize_assign_lhs
    |> Phases.Reconstruct_question_marks
    |> Side_effect_utils.Hoist
    |> Phases.Local_mutation
    (* |> Phases.State_passing_loop *)
    |> Phases.Reject.Continue
    |> Phases.Cf_into_monads
    |> Phases.Reject.EarlyExit
    (* |> Phases.Functionalize_loops *)
    |> Phases.Reject.As_pattern
    |> Phases.Reject.Dyn
    |> Phases.Reject.Trait_item_default
    |> Phases.Bundle_cycles
    |> Phases.Sort_items_namespace_wise
    |> SubtypeToInputLanguage
    |> Identity
  ]
  [@ocamlformat "disable"]

(* let token_list (tokens : string) : string list list = *)
(*   List.map ~f:(split_str ~on:"=") (split_str ~on:"," tokens) *)

(* let get_argument (s : string) (token_list : string list list) = *)
(*   List.find_map *)
(*     ~f:(function *)
(*       | [ v; a ] when String.equal (String.strip v) s -> Some a | _ -> None) *)
(*     token_list *)

(* let strip (x : string) = *)
(*   String.strip *)
(*     ?drop:(Some (function '\"' -> true | _ -> false)) *)
(*     (String.strip x) *)

(* let strip_or_error (err : string) (s : string option) span = *)
(*   match s with *)
(*   | Some x -> strip x *)
(*   | None -> Error.unimplemented ~details:err span *)

let pconcrete_ident (id : Ast.concrete_ident) : string =
  (RenderId.render id).name

let plocal_ident (e : Local_ident.t) : string =
  RenderId.local_ident
    (match String.chop_prefix ~prefix:"impl " e.name with
    | Some name ->
        let name = "impl_" ^ Int.to_string ([%hash: string] name) in
        { e with name }
    | _ -> e)

module Make
    (Attrs : Attrs.WITH_ITEMS)
    (Ctx : sig
      val ctx : Context.t
    end) =
struct
  open Ctx

  let pglobal_ident (id : Ast.global_ident) : string =
    match id with
    | `Projector (`Concrete cid) | `Concrete cid -> pconcrete_ident cid
    | `Primitive p_id -> primitive_to_string p_id
    | `TupleType _i -> "TODO (global ident) tuple type"
    | `TupleCons _i -> "TODO (global ident) tuple cons"
    | `Projector (`TupleField (_i, _j)) | `TupleField (_i, _j) ->
        "TODO (global ident) tuple field"
    | _ -> .

  (* module TODOs_debug = struct *)
  (*   let __TODO_pat__ _ s = SSP.AST.Ident (s ^ " todo(pat)") *)
  (*   let __TODO_ty__ _ s : SSP.AST.ty = SSP.AST.NameTy (s ^ " todo(ty)") *)
  (*   let __TODO_item__ _ s = SSP.AST.Unimplemented (s ^ " todo(item)") *)

  (*   let __TODO_term__ _ s = *)
  (*     SSP.AST.Const (SSP.AST.Const_string (s ^ " todo(term)")) *)
  (* end *)

  module TODOs = struct
    let __TODO_ty__ span s : SSP.AST.ty =
      Error.unimplemented ~details:("[ty] node " ^ s) span

    let __TODO_pat__ span s =
      Error.unimplemented ~details:("[pat] node " ^ s) span

    let __TODO_term__ span s =
      Error.unimplemented ~details:("[expr] node " ^ s) span

    let __TODO_item__ _span s = SSP.AST.Unimplemented (s ^ " todo(item)")
  end

  open TODOs

  let pint_kind (k : Ast.int_kind) : SSP.AST.int_type =
    {
      size =
        (match k.size with
        | S8 -> U8
        | S16 -> U16
        | S32 -> U32
        | S64 -> U64
        | S128 -> U128
        | SSize -> USize);
      signed = Stdlib.(k.signedness == Signed);
    }

  let pliteral (e : Ast.literal) =
    match e with
    | String s -> SSP.AST.Const_string s
    (* This is wrong for non-ASCII chars: https://github.com/pereq-dev/hax/issues/1953 *)
    | Char c -> SSP.AST.Const_char (Char.to_int (String.get c 0))
    | Int { value; kind; _ } -> SSP.AST.Const_int (value, pint_kind kind)
    | Float _ -> failwith "Float: todo"
    | Bool b -> SSP.AST.Const_bool b

  let operators =
    let c = Ast.Global_ident.of_name ~value:true in
    [
      (c Rust_primitives__hax__array_of_list, (3, [ ""; ".a["; "]<-"; "" ]));
      (c Core__ops__index__Index__index, (2, [ ""; ".a["; "]" ]));
      (c Core__ops__bit__BitXor__bitxor, (2, [ ""; " .^ "; "" ]));
      (c Core__ops__bit__BitAnd__bitand, (2, [ ""; " .& "; "" ]));
      (c Core__ops__bit__BitOr__bitor, (2, [ ""; " .| "; "" ]));
      (c Core__ops__arith__Add__add, (2, [ ""; " .+ "; "" ]));
      (c Core__ops__arith__Sub__sub, (2, [ ""; " .- "; "" ]));
      (c Core__ops__arith__Mul__mul, (2, [ ""; " .* "; "" ]));
      (c Core__ops__arith__Div__div, (2, [ ""; " ./ "; "" ]));
      (c Core__cmp__PartialEq__eq, (2, [ ""; " =.? "; "" ]));
      (c Core__cmp__PartialOrd__lt, (2, [ ""; " <.? "; "" ]));
      (c Core__cmp__PartialOrd__le, (2, [ ""; " <=.? "; "" ]));
      (c Core__cmp__PartialOrd__ge, (2, [ ""; " >=.? "; "" ]));
      (c Core__cmp__PartialOrd__gt, (2, [ ""; " >.? "; "" ]));
      (c Core__cmp__PartialEq__ne, (2, [ ""; " <> "; "" ]));
      (c Core__ops__arith__Rem__rem, (2, [ ""; " .% "; "" ]));
      (c Core__ops__bit__Shl__shl, (2, [ ""; " shift_left "; "" ]));
      (c Core__ops__bit__Shr__shr, (2, [ ""; " shift_right "; "" ]));
    ]
    |> Map.of_alist_exn (module Ast.Global_ident)

  module LocalIdentOrLisIis =
  StaticAnalysis.MutableVariables.LocalIdentOrData (struct
    type ty = string list * string list [@@deriving compare, sexp]
  end)

  let rec pty span (t : ty) : SSP.AST.ty =
    match t with
    | TBool -> SSP.AST.Bool
    | TChar -> __TODO_ty__ span "char"
    | TInt k -> SSP.AST.Int (pint_kind k)
    | TStr -> SSP.AST.NameTy "chString"
    | TApp { ident = `TupleType 0; args = []; _ } -> SSP.AST.Unit
    | TApp { ident = `TupleType 1; args = [ GType ty ]; _ } -> pty span ty
    | TApp { ident = `TupleType n; args; _ } when n >= 2 ->
        SSP.AST.Product (args_ty span args)
    | TApp { ident; args; _ } ->
        SSP.AST.AppTy (SSP.AST.NameTy (pglobal_ident ident), args_ty span args)
    | TArrow ([ TApp { ident = `TupleType 0; args = []; _ } ], output) ->
        pty span output
    | TArrow (inputs, output) ->
        List.fold_right ~init:(pty span output)
          ~f:(fun x y -> SSP.AST.Arrow (x, y))
          (List.map ~f:(pty span) inputs)
    | TFloat _ -> __TODO_ty__ span "pty: Float"
    | TArray { typ; length = { e = Literal (Int { value; _ }); _ }; _ } ->
        SSP.AST.ArrayTy (pty span typ, value)
    | TArray { typ; length } ->
        SSP.AST.ArrayTy
          ( pty span typ,
            "(" ^ "is_pure" ^ " " ^ "("
            ^ SSP.term_to_string_with_paren
                (pexpr (Map.empty (module Local_ident)) false length)
                0
            ^ ")" ^ ")" )
        (* TODO: check int.to_string is correct! *)
    | TSlice { ty; _ } -> SSP.AST.SliceTy (pty span ty)
    | TParam i -> SSP.AST.NameTy (plocal_ident i)
    | TAssociatedType { item; _ } -> SSP.AST.NameTy (pconcrete_ident item)
    | TOpaque _ -> __TODO_ty__ span "pty: TAssociatedType/TOpaque"
    | _ -> .

  and args_ty span (args : generic_value list) : SSP.AST.ty list =
    List.map
      ~f:(function
        | GLifetime _ -> __TODO_ty__ span "lifetime"
        | GType typ -> pty span typ
        | GConst { typ; _ } ->
            SSPExtraDefinitions.wrap_type_in_both (pty span typ))
      args
  (* match args with *)
  (* | arg :: xs -> *)
  (*     (match arg with *)
  (*     | GLifetime { lt; witness } -> __TODO_ty__ span "lifetime" *)
  (*     | GType typ -> pty span typ *)
  (*     | GConst { typ; _ } -> *)
  (*         wrap_type_in_both "(fset [])" "(fset [])" (pty span typ)) *)
  (*     :: args_ty span xs *)
  (* | [] -> [] *)

  and ppat (p : pat) : SSP.AST.pat =
    match p.p with
    | PWild -> SSP.AST.WildPat
    | PAscription { typ; pat; _ } ->
        SSP.AST.AscriptionPat (ppat pat, pty p.span typ)
    | PBinding
        {
          mut = Immutable;
          mode = _;
          var;
          typ = _ (* we skip type annot here *);
          _;
        } ->
        SSP.AST.Ident (plocal_ident var)
    | PBinding
        {
          mut = Mutable _;
          mode = _;
          var;
          typ = _ (* we skip type annot here *);
          _;
        } ->
        SSP.AST.Ident (plocal_ident var) (* TODO Mutable binding ! *)
    | POr { subpats } -> SSP.AST.DisjunctivePat (List.map ~f:ppat subpats)
    | PArray _ -> __TODO_pat__ p.span "Parray?"
    | PConstruct { constructor = `TupleCons 0; fields = []; _ } ->
        SSP.AST.WildPat (* UnitPat *)
    | PConstruct { constructor = `TupleCons 1; fields = [ _ ]; _ } ->
        __TODO_pat__ p.span "tuple 1"
    | PConstruct { constructor = `TupleCons _n; fields = args; _ } ->
        SSP.AST.TuplePat (List.map ~f:(fun { pat; _ } -> ppat pat) args)
    (* Record *)
    | PConstruct { is_record = true; _ } -> __TODO_pat__ p.span "record pattern"
    (* (\* SSP.AST.Ident (pglobal_ident name) *\) *)
    (* SSP.AST.RecordPat (pglobal_ident name, List.map ~f:(fun {field; pat} -> (pglobal_ident field, ppat pat)) args) *)
    (*       (\* SSP.AST.ConstructorPat (pglobal_ident name ^ "_case", [SSP.AST.Ident "temp"]) *\) *)
    (*       (\* List.map ~f:(fun {field; pat} -> (pat, SSP.AST.App (SSP.AST.Var (pglobal_ident field), [SSP.AST.Var "temp"]))) args *\) *)
    (* Enum *)
    | PConstruct { constructor = name; fields = args; is_record = false; _ } ->
        SSP.AST.ConstructorPat
          ( pglobal_ident name,
            match args with
            | [] -> []
            | _ -> [ SSP.AST.TuplePat (List.map ~f:(fun p -> ppat p.pat) args) ]
          )
    | PConstant { lit } -> SSP.AST.Lit (pliteral lit)
    | _ -> .

  (* and analyse_fset (data : StaticAnalysis.MutableVariables.analysis_data) items = *)
  (*   (object *)
  (*      inherit [_] expr_reduce as super *)
  (*      inherit [_] U.Reducers.expr_list_monoid as m (\* TODO: Raname into list monoid *\) *)
  (*      method visit_t _ _ = m#zero *)
  (*      (\* method visit_mutability (_f : string -> _ -> _) (ctx : string) _ = m#zero *\) *)
  (*      method visit_mutability (f : string -> _ -> _) (ctx : string) mu = *)
  (*        match mu with Mutable wit -> f ctx wit | _ -> m#zero *)

  (*      method! visit_PBinding env mut _ var _typ subpat = *)
  (*        m#plus *)
  (*          (match mut with *)
  (*           | Mutable _ -> *)
  (*             var.name *)
  (*           | Immutable -> *)
  (*             (\* Set.singleton (module U.TypedLocalIdent) (var, typ) *\) *)
  (*             "") *)
  (*          (Option.value_map subpat ~default:m#zero *)
  (*             ~f:(fst >> super#visit_pat env)) *)

  (*      method! visit_global_ident (env : string) (x : Global_ident.t) = *)
  (*        match x with *)
  (*        | `Projector (`Concrete cid) | `Concrete cid -> *)
  (*          (match Map.find data (Uprint.Concrete_ident_view.to_definition_name cid) with *)
  (*           | Some (x,_) -> Set.of_list (module LocalIdent) x *)
  (*           | _ -> m#zero) *)
  (*        | _ -> m#zero *)

  (*      method visit_expr (env : string) e = [(e, env)] (\* :: super#visit_expr f e *\) *)
  (*   end) *)
  (*   #visit_expr *)
  (*     "" *)

  and pexpr (env : LocalIdentOrLisIis.W.t list Map.M(Local_ident).t)
      (add_solve : bool) (e : expr) : SSP.AST.term =
    let span = e.span in
    (* (match (add_solve, e.e) with *)
    (* | ( true, *)
    (*     ( Construct { is_record = true; _ } *)
    (*     | If _ (\* | Match _ *\) | Literal _ *)
    (*     | Construct { constructor = `TupleCons _; _ } *)
    (*     | App _ | GlobalVar _ | LocalVar _ ) ) -> *)
    (*     fun x -> x (\* SSP.AST.App (SSP.AST.Var "solve_lift", [ x ]) *\) *)
    (* | _ -> fun x -> x) *)
    match e.e with
    | Literal lit ->
        SSP.AST.App
          ( SSP.AST.Var "ret_both",
            [ SSP.AST.TypedTerm (SSP.AST.Const (pliteral lit), pty span e.typ) ]
          )
    | LocalVar local_ident -> SSP.AST.NameTerm (plocal_ident local_ident)
    | GlobalVar (`TupleCons 0)
    | Construct { constructor = `TupleCons 0; fields = []; _ } ->
        SSP.AST.App (SSP.AST.Var "ret_both", [ SSPExtraDefinitions.unit_term ])
    | GlobalVar global_ident -> SSP.AST.Var (pglobal_ident global_ident)
    | App
        {
          f = { e = GlobalVar (`Projector (`TupleField (i, j))); _ };
          args = [ _ ];
          _;
        } ->
        (* SSP.AST.App (SSP.AST.Var (Int.to_string i), [ SSP.AST.Var (Int.to_string j) ]) *)
        __TODO_term__ span "app global vcar projector tuple"
    | App
        {
          f;
          args =
            [
              {
                e =
                  ( GlobalVar (`TupleCons 0)
                  | Construct { constructor = `TupleCons 0; fields = []; _ } );
              };
            ];
          _;
        } ->
        (pexpr env false) f
    | App { f = { e = GlobalVar x; _ }; args; _ } when Map.mem operators x ->
        let arity, op = Map.find_exn operators x in
        if List.length args <> arity then failwith "Bad arity";
        let args =
          List.map
            ~f:(fun x -> SSP.AST.Value ((pexpr env false) x, true, 0))
            args
        in
        SSP.AST.AppFormat (op, args)
    (* | App { f = { e = GlobalVar x }; args } -> *)
    (*    __TODO_term__ span "GLOBAL APP?" *)
    | App { f; args; _ } ->
        let base = (pexpr env false) f in
        let args = List.map ~f:(pexpr env false) args in
        SSP.AST.App (base, args)
    | If { cond; then_; else_ } ->
        SSPExtraDefinitions.ifb
          ( (pexpr env false) cond,
            (pexpr env false) then_,
            Option.value_map else_ ~default:(SSP.AST.Literal "()")
              ~f:(pexpr env false) )
    | Array l -> SSP.AST.Array (List.map ~f:(pexpr env add_solve) l)
    | Let { lhs; rhs; body; monadic } ->
        let extra_set, _extra_env =
          LocalIdentOrLisIis.analyse_expr ctx.analysis_data.mut_var env rhs
        in
        let new_env =
          extend_env env
            (Map.of_alist_exn
               (module Local_ident)
               (List.map
                  ~f:(fun v -> (v, extra_set))
                  (Set.to_list (U.Reducers.variables_of_pat lhs))))
        in
        let new_env =
          match (monadic, is_mutable_pat lhs) with
          | None, true ->
              extend_env new_env
                (Map.of_alist_exn
                   (module Local_ident)
                   (List.map
                      ~f:(fun v -> (v, [ LocalIdentOrLisIis.W.Identifier v ]))
                      (Set.to_list (U.Reducers.variables_of_pat lhs))))
          | _, _ -> new_env
        in
        SSPExtraDefinitions.letb
          {
            pattern = ppat lhs;
            mut = is_mutable_pat lhs;
            value = (pexpr env false) rhs;
            body = (pexpr new_env add_solve) body;
            value_typ =
              (match monadic with
              | Some (MException typ, _) -> pty span typ
              | Some (MResult typ, _) -> pty span typ
              | _ ->
                  SSP.AST.WildTy
                  (* TODO : What should the correct type be here? `lhs.span lhs.typ` *));
            monad_typ =
              Option.map
                ~f:(fun (m, _) ->
                  match m with
                  | MException typ -> SSP.AST.Exception (pty span typ)
                  | MResult typ -> SSP.AST.Result (pty span typ)
                  | MOption -> SSP.AST.Option)
                monadic;
          }
    | EffectAction _ -> . (* __TODO_term__ span "monadic action" *)
    | Match
        {
          scrutinee;
          arms =
            [
              {
                arm =
                  {
                    arm_pat =
                      {
                        p =
                          PConstruct
                            {
                              fields = [ { pat; _ } ];
                              is_record = false;
                              is_struct = true;
                              _;
                            };
                        _;
                      };
                    body;
                  };
                _;
              };
            ];
        } ->
        (* Record match expressions *)
        (* (pexpr env true) body *)
        SSPExtraDefinitions.letb
          {
            pattern = ppat pat;
            mut = false;
            value = (pexpr env false) scrutinee;
            body = (pexpr env true) body;
            value_typ = pty pat.span pat.typ;
            monad_typ = None;
          }
    | Match { scrutinee; arms } ->
        SSPExtraDefinitions.matchb
          ( (pexpr env false) scrutinee,
            List.map
              ~f:(fun { arm = { arm_pat; body }; _ } ->
                match arm_pat.p with
                | PConstruct
                    {
                      constructor = name;
                      fields = args;
                      is_record = false;
                      is_struct = false;
                    } -> (
                    let arg_tuple =
                      SSP.AST.TuplePat (List.map ~f:(fun p -> ppat p.pat) args)
                    in
                    ( SSP.AST.ConstructorPat
                        ( pglobal_ident name ^ "_case",
                          match args with [] -> [] | _ -> [ arg_tuple ] ),
                      match
                        (args, SSPExtraDefinitions.pat_as_expr arg_tuple)
                      with
                      | _ :: _, Some (redefine_pat, redefine_expr) ->
                          SSPExtraDefinitions.letb
                            {
                              pattern = redefine_pat (* TODO *);
                              mut = false;
                              value =
                                SSP.AST.App
                                  ( SSP.AST.Var "ret_both",
                                    [
                                      SSP.AST.TypedTerm
                                        ( redefine_expr,
                                          SSP.AST.Product
                                            (List.map
                                               ~f:(fun x ->
                                                 pty arm_pat.span x.pat.typ)
                                               args) );
                                    ] );
                              body = (pexpr env true) body;
                              value_typ =
                                SSP.AST.Product
                                  (List.map
                                     ~f:(fun x -> pty arm_pat.span x.pat.typ)
                                     args);
                              monad_typ = None;
                            }
                      | _, _ -> (pexpr env true) body ))
                | _ -> (ppat arm_pat, (pexpr env true) body))
              arms )
    | Ascription _ -> __TODO_term__ span "asciption"
    | Construct { constructor = `TupleCons 1; fields = [ (_, e) ]; _ } ->
        (pexpr env false) e
    | Construct { constructor = `TupleCons _n; fields; _ } ->
        SSP.AST.App
          ( SSP.AST.Var "prod_b",
            [ SSP.AST.Tuple (List.map ~f:(snd >> pexpr env false) fields) ] )
    | Construct { is_record = true; constructor; fields; base = None; _ } ->
        SSP.AST.RecordConstructor
          ( "t_" ^ pglobal_ident constructor,
            List.map
              ~f:(fun (f, e) -> (pglobal_ident f, (pexpr env false) e))
              fields )
    | Construct { is_record = true; constructor; fields; base = Some (x, _); _ }
      ->
        SSP.AST.RecordUpdate
          ( pglobal_ident constructor,
            (pexpr env false) x,
            List.map
              ~f:(fun (f, e) -> (pglobal_ident f, (pexpr env false) e))
              fields )
    (* TODO: Is there only 1 field? *)
    | Construct { constructor; fields = [ (_f, e) ]; _ } ->
        SSP.AST.App
          (SSP.AST.Var (pglobal_ident constructor), [ (pexpr env add_solve) e ])
    | Construct { constructor; fields; _ } ->
        (* __TODO_term__ span "constructor" *)
        SSP.AST.App
          ( SSP.AST.Var (pglobal_ident constructor),
            List.map ~f:(snd >> pexpr env add_solve) fields )
    | Closure { params; body; _ } ->
        SSP.AST.Lambda
          ( List.map ~f:ppat params,
            (pexpr (extend_env_with_params env params) add_solve) body )
    | MacroInvokation { macro; _ } ->
        Error.raise
        @@ {
             kind = UnsupportedMacro { id = [%show: Ast.global_ident] macro };
             span = e.span;
           }
    | Assign _ ->
        SSP.AST.Const (SSP.AST.Const_string ("assign" ^ " todo(term)"))
    (* __TODO_term__ span "assign" *)
    | Loop { body; kind; state = None; label; witness } ->
        (pexpr env false)
          {
            e =
              Loop
                {
                  body;
                  kind;
                  state =
                    Some
                      {
                        init =
                          {
                            e =
                              Construct
                                {
                                  is_record = false;
                                  is_struct = false;
                                  base = None;
                                  constructor = `TupleCons 0;
                                  fields = [];
                                };
                            span = Span.dummy ();
                            typ = TApp { ident = `TupleType 0; args = [] };
                          };
                        bpat =
                          {
                            p =
                              PConstruct
                                {
                                  constructor = `TupleCons 0;
                                  fields = [];
                                  is_record = false;
                                  is_struct = false;
                                };
                            span = Span.dummy ();
                            typ = TApp { ident = `TupleType 0; args = [] };
                          };
                        witness =
                          Features.On.state_passing_loop
                          (* state_passing_loop *);
                      };
                  label;
                  witness;
                  control_flow = None;
                  (* TODO? *)
                };
            typ = e.typ;
            span = e.span;
          }
    | Loop
        {
          body;
          kind = ForIndexLoop { start; end_; var; _ };
          state = Some { init; bpat; _ };
          _;
        } ->
        SSP.AST.App
          ( SSP.AST.Var "foldi_both",
            [
              (pexpr env false) start;
              (pexpr env false) end_;
              SSP.AST.Lambda
                ( [
                    (* SSP.AST.Ident "{L I _ _}";  *)
                    SSP.AST.Ident (plocal_ident var);
                  ],
                  SSP.AST.App
                    ( SSP.AST.Var "ssp",
                      [
                        SSP.AST.Lambda
                          ( [ ppat bpat ],
                            both_type_expr
                              (extend_env env
                                 (Map.of_alist_exn
                                    (module Local_ident)
                                    ([
                                       ( var,
                                         [
                                           LocalIdentOrLisIis.W.Data
                                             ( [ plocal_ident var ^ "?" ],
                                               [ plocal_ident var ^ "?" ] );
                                         ] );
                                     ]
                                    @ List.map
                                        ~f:(fun v ->
                                          ( v,
                                            [
                                              LocalIdentOrLisIis.W.Data
                                                ( [ plocal_ident v ^ "!" ],
                                                  [ plocal_ident v ^ "!" ] );
                                            ] ))
                                        (vars_from_pat bpat))))
                              true [] body );
                      ] ) );
              (pexpr env false) init;
            ] )
    | Loop
        {
          body;
          kind = ForLoop { pat; it; _ };
          state = Some { init; bpat; _ };
          _;
        } ->
        let extra_set_init, _extra_env =
          LocalIdentOrLisIis.analyse_expr ctx.analysis_data.mut_var env init
        in
        let new_env =
          extend_env env
            (Map.of_alist_exn
               (module Local_ident)
               (List.map
                  ~f:(fun v -> (v, extra_set_init))
                  (Set.to_list (U.Reducers.variables_of_pat bpat))))
        in
        let extra_set_iter, _extra_env =
          LocalIdentOrLisIis.analyse_expr ctx.analysis_data.mut_var env it
        in
        let new_env =
          extend_env new_env
            (Map.of_alist_exn
               (module Local_ident)
               (List.map
                  ~f:(fun v -> (v, extra_set_iter))
                  (Set.to_list (U.Reducers.variables_of_pat bpat))))
        in
        SSP.AST.App
          ( SSP.AST.Var "foldi_both_list",
            [
              (pexpr env false) it;
              SSP.AST.Lambda
                ( [ (* SSP.AST.Ident "{L I _ _}";  *) ppat pat ],
                  SSP.AST.App
                    ( SSP.AST.Var "ssp",
                      [
                        SSP.AST.Lambda
                          ( [ ppat bpat ],
                            both_type_expr new_env true
                              (extra_set_iter @ extra_set_init)
                              body );
                      ] ) );
              (pexpr env false) init;
            ] )
    | Loop _ ->
        SSP.AST.Const (SSP.AST.Const_string ("other loop" ^ " todo(term)"))
    (* __TODO_term__ span "other loop" *)
    (* | Break { e; _ } -> *)
    (*     SSP.AST.Const (SSP.AST.Const_string ("break" ^ " todo(term)")) *)
    (*     (* __TODO_term__ span "break" *) *)
    | _ -> .

  and vars_from_pat : pat -> Local_ident.t list =
    U.Reducers.variables_of_pat >> Set.to_list

  and env_from_param (params : pat list) :
      LocalIdentOrLisIis.W.t list Map.M(Local_ident).t =
    Map.of_alist_exn
      (module Local_ident)
      (List.concat_mapi
         ~f:(fun i pat ->
           List.map
             ~f:(fun var ->
               ( var,
                 [
                   LocalIdentOrLisIis.W.Data
                     ( [ "L" ^ Int.to_string (i + 1) ],
                       [ "I" ^ Int.to_string (i + 1) ] );
                 ] ))
             (vars_from_pat pat))
         params)

  and extend_env (env : LocalIdentOrLisIis.W.t list Map.M(Local_ident).t)
      (env_ext : LocalIdentOrLisIis.W.t list Map.M(Local_ident).t) :
      LocalIdentOrLisIis.W.t list Map.M(Local_ident).t =
    Map.merge_skewed env env_ext ~combine:(fun ~key:_ a b -> a @ b)
  (* TODO: Just combine values? Should do this as sets! *)

  and extend_env_with_params
      (env : LocalIdentOrLisIis.W.t list Map.M(Local_ident).t)
      (params : pat list) : LocalIdentOrLisIis.W.t list Map.M(Local_ident).t =
    extend_env env (env_from_param params)

  and analyse_env_of_expr
      (env : LocalIdentOrLisIis.W.t list Map.M(Local_ident).t) (e : expr)
      extra_set =
    let expr_env, new_env =
      LocalIdentOrLisIis.analyse_expr ctx.analysis_data.mut_var env e
    in
    let expr_env = expr_env @ extra_set in
    let identifiers =
      List.filter_map
        ~f:(function Identifier x -> Some x | Data _ -> None)
        expr_env
    in
    let data =
      List.filter_map
        ~f:(function Identifier _ -> None | Data x -> Some x)
        expr_env
    in
    let lis, iis = (List.concat *** List.concat) (List.unzip data) in
    (identifiers, lis, iis, new_env)

  and both_type_expr (env : LocalIdentOrLisIis.W.t list Map.M(Local_ident).t)
      (add_solve : bool) (extra_set : LocalIdentOrLisIis.W.t list) (e : expr) =
    let identifiers, lis, iis, _new_env = analyse_env_of_expr env e extra_set in
    SSP.AST.TypedTerm
      ( (pexpr env add_solve) e,
        SSPExtraDefinitions.wrap_type_in_both (pty e.span e.typ) )

  and is_mutable_pat (pat : pat) =
    match pat.p with
    | PWild -> false
    | PAscription { pat; _ } -> is_mutable_pat pat
    | PConstruct { constructor = `TupleCons _; fields = args; _ } ->
        List.fold ~init:false ~f:( || )
          (List.map ~f:(fun p -> is_mutable_pat p.pat) args)
    | PConstruct _ -> false
    | PArray _ ->
        (* List.fold ~init:false ~f:(||) (List.map ~f:(fun p -> is_mutable_pat p) args) *)
        false
    | PConstant _ -> false
    | PBinding { mut = Mutable _; _ } -> true
    | PBinding _ -> false
    | POr _ ->
        (* List.fold ~init:false ~f:( || ) *)
        (*   (List.map ~f:(fun p -> is_mutable_pat p) subpats) *)
        false (* TODO? *)
    | _ -> .

  let pgeneric_param_as_argument span : AST.generic_param -> SSP.AST.argument =
    function
    | { ident; kind; _ } ->
        SSP.AST.Implicit
          ( SSP.AST.Ident (plocal_ident ident),
            match kind with
            | GPType (* { default = Some t } *) ->
                SSP.AST.NameTy (plocal_ident ident) (* pty span t *)
            | GPConst { typ = t } ->
                SSPExtraDefinitions.wrap_type_in_both (pty span t)
            (* | GPType { default = None } -> SSP.AST.WildTy *)
            | _ -> . )

  let pgeneric_constraints_as_argument span :
      generic_constraint -> SSP.AST.argument list = function
    | GCType { goal = { trait; args }; _ } ->
        [
          SSP.AST.Typeclass
            ( None,
              SSP.AST.AppTy
                ( SSP.AST.NameTy (pconcrete_ident trait),
                  List.map
                    ~f:(function
                      | GType typ -> pty span typ
                      | GConst { typ; _ } ->
                          SSPExtraDefinitions.wrap_type_in_both (pty span typ)
                      | _ -> .)
                    args ) );
        ]
    | GCProjection { impl; assoc_item; typ } ->
        []
        (* Error.unimplemented ~issue_id:549 *)
        (*   ~details:"Projections of an associated type is not yet supported." *)
        (*   span *)
    | _ -> .

  let pgeneric (span : Ast.span) (generics : AST.generics) :
      SSP.AST.argument list =
    List.map ~f:(pgeneric_param_as_argument span) generics.params
    @ List.concat_map
        ~f:(pgeneric_constraints_as_argument span)
        generics.constraints

  let rec split_arrow_in_args (a : SSP.AST.ty) : SSP.AST.ty list * SSP.AST.ty =
    match a with
    | SSP.AST.Arrow (x, y) ->
        let l, r = split_arrow_in_args y in
        (x :: l, r)
    | _ -> ([], a)

  let rec wrap_type_in_enumerator_helper (i : int) (a : SSP.AST.ty) =
    let l, r = split_arrow_in_args a in
    let size, t =
      List.fold_left
        ~f:(fun (yi, ys) x ->
          let size, x_val = wrap_type_in_enumerator_helper yi x in
          ( size,
            match ys with
            | Some v -> Some (SSP.AST.Arrow (v, x_val))
            | None -> Some x_val ))
        ~init:(i, None) l
    in
    match t with
    | Some v ->
        (size, SSP.AST.Arrow (v, SSPExtraDefinitions.wrap_type_in_both r))
    | None -> (size + 1, SSPExtraDefinitions.wrap_type_in_both r)

  let wrap_type_in_enumerator (a : SSP.AST.ty) =
    let size, v = wrap_type_in_enumerator_helper 0 a in
    (* Throw away anotation of last type, and replace with accumulation of all locations and imports *)
    let xs, a =
      match v with
      | SSP.AST.Arrow (x, SSP.AST.AppTy (SSP.AST.NameTy _, [ a ])) -> ([ x ], a)
      | SSP.AST.AppTy (SSP.AST.NameTy _, [ a ]) -> ([], a)
      | _ ->
          Error.unimplemented
            ~details:
              "SSProve: TODO: wrap_type_in_enumerator encountered an \
               unexpected type"
            (Span.dummy ())
    in
    let ret_ty =
      List.fold
        ~init:(SSPExtraDefinitions.wrap_type_in_both a)
        ~f:(fun y x -> SSP.AST.Arrow (x, y))
        xs
    in
    (size, ret_ty)

  let rec pitem (e : AST.item) : SSP.AST.decl list =
    try pitem_unwrapped e
    with Diagnostics.SpanFreeError.Exn _kind ->
      [ SSP.AST.Unimplemented "item error backend" ]

  and pitem_unwrapped (e : AST.item) : SSP.AST.decl list =
    let span = e.span in
    let decls_from_item =
      match e.v with
      | Fn { name = f_name; generics; body; params } ->
          [
            (let args, ret_typ =
               lift_definition_type_to_both f_name
                 (pgeneric span generics
                 @ List.map
                     ~f:(fun { pat; typ; _ } ->
                       SSP.AST.Explicit (ppat pat, pty span typ))
                     params)
                 (pty span body.typ)
             in
             if Attrs.lemma e.attrs then
               SSP.AST.Lemma
                 ( pconcrete_ident f_name,
                   args,
                   (pexpr
                      (extend_env_with_params
                         (Map.empty (module Local_ident))
                         (List.map ~f:(fun { pat; _ } -> pat) params))
                      true)
                     (Option.value ~default:body
                        (Attrs.associated_expr Ensures e.attrs)) )
             else
               SSP.AST.Equations
                 ( pconcrete_ident f_name,
                   args,
                   (pexpr
                      (extend_env_with_params
                         (Map.empty (module Local_ident))
                         (List.map ~f:(fun { pat; _ } -> pat) params))
                      true)
                     body,
                   ret_typ ));
          ]
      | TyAlias { name; generics; ty } ->
          let g = pgeneric span generics in
          [
            (if List.is_empty g then
               SSP.AST.Notation
                 ( "'" ^ pconcrete_ident name ^ "'",
                   SSP.AST.Type (pty span ty),
                   None )
             else
               SSP.AST.Definition
                 ( pconcrete_ident name,
                   g,
                   SSP.AST.Type (pty span ty),
                   SSP.AST.TypeTy ));
          ]
      (* record *)
      | Type
          {
            name;
            generics;
            variants = [ { name = _record_name; arguments; _ } ];
            is_struct = true;
          } ->
          [
            SSPExtraDefinitions.updatable_record
              ( pconcrete_ident name,
                pgeneric span generics,
                List.map
                  ~f:(fun (x, y) -> SSP.AST.Named (x, y))
                  (p_record_record span arguments) );
          ]
      (* enum *)
      | Type { name; generics; variants; _ } ->
          (* Define all record types in enums (no anonymous records) *)
          List.filter_map variants
            ~f:(fun { name = v_name; arguments; is_record; _ } ->
              if is_record then
                Some
                  (SSPExtraDefinitions.updatable_record
                     ( (match
                          String.chop_prefix ~prefix:"C_"
                            (pconcrete_ident v_name)
                        with
                       | Some name -> "t_" ^ name
                       | _ -> failwith "Incorrect prefix of record name in enum"),
                       pgeneric span generics,
                       List.map
                         ~f:(fun (x, y) -> SSP.AST.Named (x, y))
                         (p_record_record span arguments) ))
              else None)
          @ [
              SSPExtraDefinitions.both_enum
                ( pconcrete_ident name,
                  pgeneric span generics,
                  List.map variants
                    ~f:(fun { name = v_name; arguments; is_record; _ } ->
                      if is_record then
                        SSP.AST.InductiveCase
                          ( pconcrete_ident v_name,
                            SSP.AST.RecordTy
                              ( (match
                                   String.chop_prefix ~prefix:"C_"
                                     (pconcrete_ident v_name)
                                 with
                                | Some name -> "t_" ^ name
                                | _ ->
                                    failwith
                                      "Incorrect prefix of record name in enum"),
                                p_record_record span arguments ) )
                      else
                        match arguments with
                        | [] -> SSP.AST.BaseCase (pconcrete_ident v_name)
                        | [ (_arg_name, arg_ty, _attr) ] ->
                            SSP.AST.InductiveCase
                              (* arg_name = ?? *)
                              (pconcrete_ident v_name, pty span arg_ty)
                        | _ ->
                            SSP.AST.InductiveCase
                              ( pconcrete_ident v_name,
                                SSP.AST.Product
                                  (List.map
                                     ~f:((fun (_x, y, _z) -> y) >> pty span)
                                     arguments) )) );
            ]
      | IMacroInvokation { macro; argument; _ } -> (
          let unsupported () =
            let id = [%show: concrete_ident] macro in
            Error.raise { kind = UnsupportedMacro { id }; span = e.span }
          in
          match RenderId.render macro with
          | { path = "hacspec_lib" :: _; name } -> (
              match name with
              | "public_nat_mod" ->
                  let open Hacspeclib_macro_parser in
                  let o : PublicNatMod.t =
                    PublicNatMod.parse argument |> Result.ok_or_failwith
                  in
                  [
                    SSP.AST.Notation
                      ( "'" ^ "t_" ^ o.type_name ^ "'",
                        SSP.AST.Type
                          (SSP.AST.NatMod
                             ( o.type_of_canvas,
                               o.bit_size_of_field,
                               o.modulo_value )),
                        None );
                    SSP.AST.Definition
                      ( o.type_name,
                        [],
                        SSP.AST.Var "id",
                        SSP.AST.Arrow
                          ( SSPExtraDefinitions.wrap_type_in_both
                              (SSP.AST.NameTy ("t_" ^ o.type_name)),
                            SSPExtraDefinitions.wrap_type_in_both
                              (SSP.AST.NameTy ("t_" ^ o.type_name)) ) );
                  ]
              | "bytes" ->
                  let open Hacspeclib_macro_parser in
                  let o : Bytes.t =
                    Bytes.parse argument |> Result.ok_or_failwith
                  in
                  [
                    SSP.AST.Notation
                      ( "'" ^ "t_" ^ o.bytes_name ^ "'",
                        SSP.AST.Type
                          (SSP.AST.ArrayTy
                             ( SSP.AST.Int { size = SSP.AST.U8; signed = false },
                               (* int_of_string *) o.size )),
                        None );
                    SSP.AST.Definition
                      ( o.bytes_name,
                        [],
                        SSP.AST.Var "id",
                        SSP.AST.Arrow
                          ( SSPExtraDefinitions.wrap_type_in_both
                              (SSP.AST.NameTy ("t_" ^ o.bytes_name)),
                            SSPExtraDefinitions.wrap_type_in_both
                              (SSP.AST.NameTy ("t_" ^ o.bytes_name)) ) );
                  ]
              | "unsigned_public_integer" ->
                  let open Hacspeclib_macro_parser in
                  let o =
                    UnsignedPublicInteger.parse argument
                    |> Result.ok_or_failwith
                  in
                  [
                    SSP.AST.Notation
                      ( "'" ^ "t_" ^ o.integer_name ^ "'",
                        SSP.AST.Type
                          (SSP.AST.ArrayTy
                             ( SSP.AST.Int { size = SSP.AST.U8; signed = false },
                               Int.to_string ((o.bits + 7) / 8) )),
                        None );
                    SSP.AST.Definition
                      ( o.integer_name,
                        [],
                        SSP.AST.Var "id",
                        SSP.AST.Arrow
                          ( SSPExtraDefinitions.wrap_type_in_both
                              (SSP.AST.NameTy ("t_" ^ o.integer_name)),
                            SSPExtraDefinitions.wrap_type_in_both
                              (SSP.AST.NameTy ("t_" ^ o.integer_name)) ) );
                  ]
              | "public_bytes" ->
                  let open Hacspeclib_macro_parser in
                  let o : Bytes.t =
                    Bytes.parse argument |> Result.ok_or_failwith
                  in
                  let typ =
                    SSP.AST.ArrayTy
                      ( SSP.AST.Int { size = SSP.AST.U8; signed = false },
                        (* int_of_string *) o.size )
                  in
                  [
                    SSP.AST.Notation
                      ("'" ^ "t_" ^ o.bytes_name ^ "'", SSP.AST.Type typ, None);
                    SSP.AST.Definition
                      ( o.bytes_name,
                        [],
                        SSP.AST.Var "id",
                        SSP.AST.Arrow
                          ( SSPExtraDefinitions.wrap_type_in_both
                              (SSP.AST.NameTy ("t_" ^ o.bytes_name)),
                            SSPExtraDefinitions.wrap_type_in_both
                              (SSP.AST.NameTy ("t_" ^ o.bytes_name)) ) );
                  ]
              | "array" ->
                  let open Hacspeclib_macro_parser in
                  let o : Array.t =
                    Array.parse argument |> Result.ok_or_failwith
                  in
                  let typ =
                    match o.typ with
                    | "U128" -> SSP.AST.U128
                    | "U64" -> SSP.AST.U64
                    | "U32" -> SSP.AST.U32
                    | "U16" -> SSP.AST.U16
                    | "U8" -> SSP.AST.U8
                    | _usize -> SSP.AST.U32 (* TODO: usize? *)
                  in
                  [
                    SSP.AST.Notation
                      ( "'" ^ "t_" ^ o.array_name ^ "'",
                        SSP.AST.Type
                          (SSP.AST.ArrayTy
                             ( SSP.AST.Int { size = typ; signed = false },
                               (* int_of_string *) o.size )),
                        None );
                    SSP.AST.Definition
                      ( o.array_name,
                        [],
                        SSP.AST.Var "id",
                        SSP.AST.Arrow
                          ( SSPExtraDefinitions.wrap_type_in_both
                              (SSP.AST.NameTy ("t_" ^ o.array_name)),
                            SSPExtraDefinitions.wrap_type_in_both
                              (SSP.AST.NameTy ("t_" ^ o.array_name)) ) );
                  ]
              | _ -> unsupported ())
          | _ -> unsupported ())
      | Use { path; is_external; rename } ->
          let _ns_path = ctx.current_namespace in
          if is_external then []
          else
            [ SSP.AST.Require (None, (* ns_crate:: ns_path @ *) path, rename) ]
      | HaxError s -> [ __TODO_item__ span s ]
      | NotImplementedYet -> [ __TODO_item__ span "Not implemented yet?" ]
      | Alias _ -> [ __TODO_item__ span "Not implemented yet? alias" ]
      | Trait { name; items; generics } ->
          [
            SSP.AST.Class
              ( pconcrete_ident name,
                (match pgeneric span generics with
                | SSP.AST.Implicit (x, y) :: xs -> SSP.AST.Explicit (x, y) :: xs
                | x -> x),
                List.concat_map
                  ~f:(fun x ->
                    match x.ti_v with
                    | TIFn fn_ty ->
                        let size, value =
                          wrap_type_in_enumerator (pty x.ti_span fn_ty)
                        in
                        [
                          SSP.AST.Named
                            ( pconcrete_ident x.ti_ident,
                              SSP.AST.Forall ([], [], value) );
                        ]
                    | TIType impl_idents ->
                        SSP.AST.Named
                          (pconcrete_ident x.ti_ident, SSP.AST.TypeTy)
                        :: List.map
                             ~f:(fun { goal = tr; _ } ->
                               SSP.AST.Coercion
                                 ( pconcrete_ident x.ti_ident ^ "_"
                                   ^ pconcrete_ident tr.trait,
                                   SSP.AST.AppTy
                                     ( SSP.AST.NameTy (pconcrete_ident tr.trait),
                                       [
                                         SSP.AST.NameTy
                                           (pconcrete_ident x.ti_ident);
                                       ] ) ))
                             impl_idents
                    | _ -> .)
                  items );
          ]
      | Impl { generics; self_ty; of_trait = name, gen_vals; items } ->
          [
            SSP.AST.ProgramInstance
              ( pconcrete_ident name,
                pgeneric span generics,
                pty span self_ty,
                args_ty span gen_vals,
                SSP.AST.InstanceDecls
                  (List.concat_map
                     ~f:(fun x ->
                       match x.ii_v with
                       | IIFn { body; params } ->
                           [
                             (let args, ret_typ =
                                lift_definition_type_to_both x.ii_ident
                                  (List.map
                                     ~f:(fun { pat; typ; _ } ->
                                       SSP.AST.Explicit (ppat pat, pty span typ))
                                     params)
                                  (pty span body.typ)
                              in
                              SSP.AST.LetDef
                                ( pconcrete_ident x.ii_ident,
                                  args,
                                  (pexpr
                                     (extend_env_with_params
                                        (Map.empty (module Local_ident))
                                        (List.map
                                           ~f:(fun { pat; _ } -> pat)
                                           params))
                                     true)
                                    body,
                                  ret_typ ));
                           ]
                       | IIType { typ; _ } ->
                           [
                             SSP.AST.LetDef
                               ( pconcrete_ident x.ii_ident,
                                 [],
                                 SSP.AST.Type (pty span typ),
                                 SSP.AST.TypeTy );
                           ])
                     items) );
          ]
          @ [
              SSP.AST.HintUnfold (pconcrete_ident name, Some (pty span self_ty));
            ]
    in
    decls_from_item

  and new_arguments (arguments : SSP.AST.argument list) =
    snd
      (List.fold_left ~init:(0, [])
         ~f:(fun (i, y) arg ->
           let f = SSPExtraDefinitions.wrap_type_in_both in
           match arg with
           | Implicit (p, t) -> (i, y @ [ SSP.AST.Implicit (p, t) ])
           | Explicit (p, t) -> (i + 1, y @ [ SSP.AST.Explicit (p, f t) ])
           | Typeclass (so, t) -> (i, y @ [ SSP.AST.Typeclass (so, t) ]))
         arguments)

  and lift_definition_type_to_both (name : concrete_ident)
      (arguments : SSP.AST.argument list) (typ : SSP.AST.ty) :
      SSP.AST.argument list * SSP.AST.ty =
    let new_args = new_arguments arguments in
    let return_typ = both_return_type_from_name name typ in
    (new_args, return_typ)

  and both_return_type_from_name name typ =
    SSPExtraDefinitions.wrap_type_in_both typ

  and p_record_record span arguments : (string * SSP.AST.ty) list =
    List.map
      ~f:(function
        | arg_name, arg_ty, _arg_attrs ->
            (pconcrete_ident arg_name, pty span arg_ty))
      arguments
end

module type S = sig
  val pitem : AST.item -> SSP.AST.decl list
  (* val pgeneric : Ast.span -> AST.generics -> SSP.AST.argument list *)
end

let make (module M : Attrs.WITH_ITEMS) ctx =
  (module Make
            (M)
            (struct
              let ctx = ctx
            end) : S)

let decls_to_string (decls : SSP.AST.decl list) : string =
  String.concat ~sep:"\n" (List.map ~f:SSP.decl_to_string decls)

let print_item m (analysis_data : StaticAnalysis.analysis_data)
    (item : AST.item) : SSP.AST.decl list =
  let (module Print) =
    make m
      { current_namespace = (RenderId.render item.ident).path; analysis_data }
  in
  Print.pitem item

let cleanup_item_strings =
  List.map ~f:String.strip
  >> List.filter ~f:(String.is_empty >> not)
  >> String.concat ~sep:"\n\n"

(* module ConCert = struct *)
(*   let translate_concert_annotations *)
(*       (analysis_data : StaticAnalysis.analysis_data) (e : item) : *)
(*       SSP.AST.decl list = *)
(*     let (module Print) = *)
(*       make *)
(*         { *)
(*           current_namespace = U.Concrete_ident_view.to_namespace e.ident; *)
(*           analysis_data; *)
(*         } *)
(*     in *)
(*     match e.v with *)
(*     | Fn { name = f_name; generics; _ } -> *)
(*         List.concat_map *)
(*           ~f:(fun { kind; span } -> *)
(*             match kind with *)
(*             | Tool { path; tokens } -> ( *)
(*                 let token_list = token_list tokens in *)
(*                 match path with *)
(*                 | "hax::init" -> *)
(*                     let contract = *)
(*                       strip_or_error "contract argument missing" *)
(*                         (get_argument "contract" token_list) *)
(*                         e.span *)
(*                     in *)
(*                     [ *)
(*                       SSP.AST.Definition *)
(*                         ( "init_" ^ contract, *)
(*                           [ *)
(*                             SSP.AST.Explicit *)
(*                               (SSP.AST.Ident "chain", SSP.AST.NameTy "Chain"); *)
(*                             SSP.AST.Explicit *)
(*                               ( SSP.AST.Ident "ctx", *)
(*                                 SSP.AST.NameTy "ContractCallContext" ); *)
(*                             SSP.AST.Explicit *)
(*                               ( SSP.AST.Ident "st", *)
(*                                 SSP.AST.NameTy ("state_" ^ contract) ); *)
(*                           ], *)
(*                           SSP.AST.App *)
(*                             (SSP.AST.Var "ResultMonad.Ok", [ SSP.AST.Var "st" ]), *)
(*                           SSP.AST.AppTy *)
(*                             ( SSP.AST.NameTy "ResultMonad.result", *)
(*                               [ *)
(*                                 SSP.AST.NameTy ("state_" ^ contract); *)
(*                                 SSP.AST.NameTy "t_ParseError"; *)
(*                               ] ) ); *)
(*                     ] *)
(*                 | "hax::receive" -> *)
(*                     let contract = *)
(*                       strip_or_error "contract argument missing" *)
(*                         (get_argument "contract" token_list) *)
(*                         e.span *)
(*                     in *)
(*                     let name = *)
(*                       strip_or_error "name argument missing" *)
(*                         (get_argument "name" token_list) *)
(*                         e.span *)
(*                     in *)
(*                     let parameter = get_argument "parameter" token_list in *)
(*                     (\* let logger = get_argument "logger" token_list in *\) *)
(*                     (\* let payable = get_argument "payable" token_list in *\) *)
(*                     let param_instances, param_list, count, param_vars = *)
(*                       match parameter with *)
(*                       | Some x -> *)
(*                           ( [ *)
(*                               SSP.AST.ProgramInstance *)
(*                                 ( "t_HasReceiveContext", *)
(*                                   [], *)
(*                                   SSP.AST.NameTy ("t_" ^ strip x), *)
(*                                   [ *)
(*                                     SSP.AST.NameTy ("t_" ^ strip x); *)
(*                                     SSP.AST.Unit; *)
(*                                   ], *)
(*                                   SSP.AST.InstanceDecls *)
(*                                     [ *)
(*                                       SSP.AST.InlineDef *)
(*                                         ( "f_get", *)
(*                                           [ *)
(*                                             SSP.AST.Implicit *)
(*                                               ( SSP.AST.Ident "Ctx", *)
(*                                                 SSP.AST.WildTy ); *)
(*                                             SSP.AST.Implicit *)
(*                                               ( SSP.AST.Ident "L", *)
(*                                                 (SSP.AST.NameTy *)
(*                                                    "{fset Location}" *)
(*                                                   : SSP.AST.ty) ); *)
(*                                             SSP.AST.Implicit *)
(*                                               ( SSP.AST.Ident "I", *)
(*                                                 (SSP.AST.NameTy "Interface" *)
(*                                                   : SSP.AST.ty) ); *)
(*                                           ], *)
(*                                           SSP.AST.Var *)
(* "(solve_lift (@ret_both \ *)
   (*                                              (t_ParamType × t_Result Ctx \ *)
   (*                                              t_ParseError)) (tt, inr tt))", *)
(*                                           SSP.AST.WildTy ); *)
(*                                     ] ); *)
(*                               SSP.AST.ProgramInstance *)
(*                                 ( "t_Sized", *)
(*                                   [], *)
(*                                   SSP.AST.NameTy ("t_" ^ strip x), *)
(*                                   [ SSP.AST.NameTy ("t_" ^ strip x) ], *)
(*                                   SSP.AST.TermDef *)
(*                                     (SSP.AST.Lambda *)
(*                                        ([ SSP.AST.Ident "x" ], SSP.AST.Var "x")) *)
(*                                 ); *)
(*                             ], *)
(*                             [ *)
(*                               SSP.AST.Explicit *)
(*                                 ( SSP.AST.Ident "ctx", *)
(*                                   SSPExtraDefinitions.wrap_type_in_both "L0" *)
(*                                     "I0" *)
(*                                     (SSP.AST.NameTy ("t_" ^ strip x)) ); *)
(*                             ], *)
(*                             1, *)
(*                             [ SSP.AST.Var "ctx" ] ) *)
(*                       | _ -> ([], [], 0, []) *)
(*                     in *)
(*                     param_instances *)
(*                     @ [ *)
(*                         SSP.AST.Definition *)
(*                           ( "receive_" ^ contract ^ "_" ^ name, *)
(*                             Print.pgeneric span generics *)
(*                             @ List.map *)
(*                                 ~f:(fun x -> *)
(*                                   SSP.AST.Implicit *)
(*                                     ( SSP.AST.Ident x, *)
(*                                       (SSP.AST.NameTy "{fset Location}" *)
(*                                         : SSP.AST.ty) )) *)
(*                                 (List.map *)
(*                                    ~f:(fun i -> "L" ^ Int.to_string i) *)
(*                                    (List.range 0 (count + 1))) *)
(*                             @ List.map *)
(*                                 ~f:(fun x -> *)
(*                                   SSP.AST.Implicit *)
(*                                     ( SSP.AST.Ident x, *)
(*                                       (SSP.AST.NameTy "Interface" : SSP.AST.ty) *)
(*                                     )) *)
(*                                 (List.map *)
(*                                    ~f:(fun i -> "I" ^ Int.to_string i) *)
(*                                    (List.range 0 (count + 1))) *)
(*                             @ param_list *)
(*                             @ [ *)
(*                                 SSP.AST.Explicit *)
(*                                   ( SSP.AST.Ident "st", *)
(*                                     SSPExtraDefinitions.wrap_type_in_both *)
(*                                       ("L" ^ Int.to_string count) *)
(*                                       ("I" ^ Int.to_string count) *)
(*                                       (SSP.AST.NameTy ("state_" ^ contract)) ); *)
(*                                 (\* TODO: L, I *\) *)
(*                               ], *)
(*                             (\* Arguments *\) *)
(*                             SSP.AST.App *)
(*                               ( SSP.AST.Var (pconcrete_ident f_name) *)
(*                                 (\* contract *\), *)
(*                                 param_vars @ [ SSP.AST.Var "st" ] ), *)
(*                             SSPExtraDefinitions.wrap_type_in_both "_" "_" *)
(*                               (SSP.AST.NameTy *)
(*                                  ("t_Result ((v_A × state_" ^ contract *)
(*                                 ^ ")) (t_ParseError)")) ); *)
(*                         (\* TODO: L , I *\) *)
(*                       ] *)
(*                 | _ -> []) *)
(*             | _ -> []) *)
(*           e.attrs *)
(*     | Type { name; variants = [ _ ]; is_struct = true; _ } -> *)
(*         List.concat_map *)
(*           ~f:(fun { kind; _ } -> *)
(*             match kind with *)
(*             | Tool { path; tokens } when String.equal path "hax::contract_state" *)
(*               -> *)
(*                 let token_list = token_list tokens in *)
(*                 let contract = *)
(*                   strip_or_error "contract argument missing" *)
(*                     (get_argument "contract" token_list) *)
(*                     e.span *)
(*                 in *)
(*                 [ *)
(*                   SSP.AST.Definition *)
(*                     ( "state_" ^ contract, *)
(*                       [], *)
(*                       SSP.AST.Var (pconcrete_ident name), *)
(*                       SSP.AST.TypeTy ); *)
(*                 ] *)
(*             | _ -> []) *)
(*           e.attrs *)
(*     | _ -> [] *)

(*   let concert_contract_type_decls (items : item list) : SSP.AST.decl list list = *)
(*     let contract_items = *)
(*       List.filter_map *)
(*         ~f:(function *)
(*           | { kind = Tool { path; tokens }; _ } *)
(*             when String.equal path "hax::receive" -> *)
(*               let token_list = token_list tokens in *)
(*               let contract = *)
(*                 strip_or_error "contract argument missing" *)
(*                   (get_argument "contract" token_list) *)
(*                   (Span.dummy ()) *)
(*                 (\* TODO: carry span information *\) *)
(*               in *)
(*               let name = *)
(*                 strip_or_error "name argument missing" *)
(*                   (get_argument "name" token_list) *)
(*                   (Span.dummy ()) *)
(*                 (\* TODO: carry span information *\) *)
(*               in *)
(*               let parameter = get_argument "parameter" token_list in *)
(*               Some (contract, parameter, name) *)
(*           | _ -> None) *)
(*         (List.concat_map ~f:(fun x -> x.attrs) items) *)
(*     in *)
(*     if List.is_empty contract_items then [] *)
(*     else *)
(*       let contract_map = *)
(*         List.fold_left *)
(*           ~init:(Map.empty (module String)) *)
(*           ~f:(fun y (x_name, x_parameter, x_item) -> *)
(*             Map.set y ~key:x_name *)
(*               ~data: *)
(*                 (Option.value ~default:[] (Map.find y x_name) *)
(*                 @ [ (x_parameter, x_item) ])) *)
(*           contract_items *)
(*       in *)
(*       List.map *)
(*         ~f:(fun contract -> *)
(*           let receive_functions : (_ * string) list = *)
(*             Option.value ~default:[] (Map.find contract_map contract) *)
(*           in *)
(*           [ *)
(*             SSP.AST.Inductive *)
(*               ( "Msg_" ^ contract, *)
(*                 [], *)
(*                 List.map *)
(*                   ~f:(function *)
(*                     | Some param, x_item -> *)
(*                         SSP.AST.InductiveCase *)
(*                           ( "msg_" ^ contract ^ "_" ^ x_item, *)
(*                             SSP.AST.NameTy ("t_" ^ strip param) ) *)
(*                     | None, x_item -> *)
(*                         SSP.AST.BaseCase ("msg_" ^ contract ^ "_" ^ x_item)) *)
(*                   receive_functions ); *)
(*             SSP.AST.ProgramInstance *)
(*               ( "t_HasReceiveContext", *)
(*                 [], *)
(*                 SSP.AST.NameTy ("state_" ^ contract), *)
(*                 [ SSP.AST.NameTy ("state_" ^ contract); SSP.AST.Unit ], *)
(*                 SSP.AST.InstanceDecls *)
(*                   [ *)
(*                     SSP.AST.InlineDef *)
(*                       ( "f_get", *)
(*                         [ *)
(*                           SSP.AST.Explicit (SSP.AST.Ident "Ctx", SSP.AST.WildTy); *)
(*                           SSP.AST.Implicit *)
(*                             ( SSP.AST.Ident "L", *)
(*                               (SSP.AST.NameTy "{fset Location}" : SSP.AST.ty) ); *)
(*                           SSP.AST.Implicit *)
(*                             ( SSP.AST.Ident "I", *)
(*                               (SSP.AST.NameTy "Interface" : SSP.AST.ty) ); *)
(*                         ], *)
(*                         SSP.AST.Var *)
(* "(solve_lift (@ret_both (t_ParamType × t_Result Ctx \ *)
   (*                            t_ParseError)) (tt, inr tt))", *)
(*                         SSP.AST.WildTy ); *)
(*                   ] ); *)
(*             SSP.AST.ProgramInstance *)
(*               ( "t_Sized", *)
(*                 [], *)
(*                 SSP.AST.NameTy ("state_" ^ contract), *)
(*                 [ SSP.AST.NameTy ("state_" ^ contract) ], *)
(*                 SSP.AST.TermDef *)
(*                   (SSP.AST.Lambda ([ SSP.AST.Ident "x" ], SSP.AST.Var "x")) ); *)
(*             SSP.AST.ProgramInstance *)
(*               ( "t_HasActions", *)
(*                 [], *)
(*                 SSP.AST.NameTy ("state_" ^ contract), *)
(*                 [ SSP.AST.NameTy ("state_" ^ contract) ], *)
(*                 SSP.AST.TermDef (SSP.AST.Var "Admitted") ); *)
(*             SSP.AST.Equations *)
(*               ( "receive_" ^ contract, *)
(*                 [ *)
(*                   SSP.AST.Explicit *)
(*                     (SSP.AST.Ident "chain", SSP.AST.NameTy "Chain"); *)
(*                   SSP.AST.Explicit *)
(*                     (SSP.AST.Ident "ctx", SSP.AST.NameTy "ContractCallContext"); *)
(*                   SSP.AST.Explicit *)
(*                     (SSP.AST.Ident "st", SSP.AST.NameTy ("state_" ^ contract)); *)
(*                   SSP.AST.Explicit *)
(*                     ( SSP.AST.Ident "msg", *)
(*                       SSP.AST.NameTy ("Datatypes.option Msg_" ^ contract) ); *)
(*                 ], *)
(*                 SSP.AST.Match *)
(*                   ( SSP.AST.Var "msg", *)
(*                     List.map *)
(*                       ~f:(function *)
(*                         | Some _param, x_item -> *)
(*                             ( SSP.AST.Ident *)
(*                                 ("Some" ^ " " ^ "(" ^ "msg_" ^ contract ^ "_" *)
(*                                ^ x_item ^ " " ^ "val" ^ ")"), *)
(*                               SSP.AST.Var *)
(*                                 ("match (is_pure (both_prog (receive_" *)
(*                                ^ contract ^ "_" ^ x_item *)
(* ^ " (ret_both val) (ret_both st)))) with\n\ *)
   (*                                  \         | inl x => ResultMonad.Ok ((fst x), \ *)
   (*                                   [])\n\ *)
   (*                                  \         | inr x => ResultMonad.Err x\n\ *)
   (*                                  \         end") ) *)
(*                         | None, x_item -> *)
(*                             ( SSP.AST.Ident *)
(*                                 ("Some" ^ " " ^ "msg_" ^ contract ^ "_" ^ x_item), *)
(*                               SSP.AST.Var *)
(*                                 ("match (is_pure (both_prog (receive_" *)
(*                                ^ contract ^ "_" ^ x_item *)
(* ^ " (ret_both st)))) with\n\ *)
   (*                                  \         | inl x => ResultMonad.Ok ((fst x), \ *)
   (*                                   [])\n\ *)
   (*                                  \         | inr x => ResultMonad.Err x\n\ *)
   (*                                  \         end") )) *)
(*                       receive_functions *)
(*                     @ [ (SSP.AST.WildPat, SSP.AST.Var "ResultMonad.Err tt") ] ), *)
(*                 SSP.AST.NameTy *)
(*                   ("ResultMonad.result (state_" ^ contract *)
(*                  ^ " * list ActionBody) t_ParseError") ); *)
(*             SSP.AST.ProgramInstance *)
(*               ( "Serializable", *)
(*                 [], *)
(*                 SSP.AST.NameTy ("state_" ^ contract), *)
(*                 [ SSP.AST.NameTy ("state_" ^ contract) ], *)
(*                 SSP.AST.InstanceDecls [] ); *)
(*             SSP.AST.ProgramInstance *)
(*               ( "Serializable", *)
(*                 [], *)
(*                 SSP.AST.NameTy ("Msg_" ^ contract), *)
(*                 [ SSP.AST.NameTy ("Msg_" ^ contract) ], *)
(*                 SSP.AST.TermDef *)
(*                   (SSP.AST.Var *)
(*                      ("Derive Serializable Msg_OVN_rect<" *)
(*                      ^ String.concat ~sep:"," *)
(*                          (List.map *)
(*                             ~f:(fun x -> "msg_" ^ contract ^ "_" ^ snd x) *)
(*                             receive_functions) *)
(*                      ^ ">")) ); *)
(*             SSP.AST.Definition *)
(*               ( "contract_" ^ contract, *)
(*                 [], *)
(*                 SSP.AST.App *)
(*                   ( SSP.AST.Var "build_contract", *)
(*                     [ *)
(*                       SSP.AST.Var ("init_" ^ contract); *)
(*                       SSP.AST.Var ("receive_" ^ contract); *)
(*                     ] ), *)
(*                 SSP.AST.AppTy *)
(*                   ( SSP.AST.NameTy "Contract", *)
(*                     [ *)
(*                       SSP.AST.NameTy ("state_" ^ contract); *)
(*                       SSP.AST.NameTy ("Msg_" ^ contract); *)
(*                       SSP.AST.NameTy ("state_" ^ contract); *)
(*                       SSP.AST.NameTy "t_ParseError"; *)
(*                     ] ) ); *)
(*           ]) *)
(*         (Map.keys contract_map) *)

(*   let concert_header = *)
(*     [ *)
(*       SSP.AST.Comment "Concert lib part"; *)
(*       SSP.AST.Require (Some "ConCert.Utils", [ "Extras" ], None); *)
(*       SSP.AST.Require (Some "ConCert.Utils", [ "Automation" ], None); *)
(*       SSP.AST.Require (Some "ConCert.Execution", [ "Serializable" ], None); *)
(*       SSP.AST.Require (Some "ConCert.Execution", [ "Blockchain" ], None); *)
(*       SSP.AST.Require (Some "ConCert.Execution", [ "ContractCommon" ], None); *)
(*       SSP.AST.Require (Some "ConCert.Execution", [ "Serializable" ], None); *)
(*       SSP.AST.Require (None, [ "ConCertLib" ], None); *)
(*     ] *)
(* end *)

let process_annotation (x : 'a list) (f2 : ('b * ('a -> 'b)) list) : 'b list =
  List.concat_map
    ~f:(fun (d, f) ->
      let temp = List.map ~f x in
      if List.is_empty (List.concat temp) then [] else d :: temp)
    f2

let string_of_items m (x, y) =
  cleanup_item_strings
    (List.map ~f:decls_to_string
       (process_annotation x
          [
            ([], print_item m y);
            (* ConCert.(concert_header, translate_concert_annotations y); *)
          ]
          (* @ ConCert.concert_contract_type_decls x *)))

(* TODO move into string_of_items, as SSP.AST decl *)
let hardcoded_coq_headers =
  "(* File automatically generated by Hacspec *)\n\
   Set Warnings \"-notation-overridden,-ambiguous-paths\".\n\
   From Crypt Require Import choice_type Package Prelude.\n\
   Import PackageNotation.\n\
   From extructures Require Import ord fset.\n\
   From mathcomp Require Import word_ssrZ word.\n\
   (* From Jasmin Require Import word. *)\n\n\
   From Coq Require Import ZArith.\n\
   From Coq Require Import Strings.String.\n\
   Import List.ListNotations.\n\
   Open Scope list_scope.\n\
   Open Scope Z_scope.\n\
   Open Scope bool_scope.\n\n\
   From Hacspec Require Import ChoiceEquality.\n\
   From Hacspec Require Import LocationUtility.\n\
   From Hacspec Require Import Hacspec_Lib_Comparable.\n\
   From Hacspec Require Import Hacspec_Lib_Pre.\n\
   From Hacspec Require Import Hacspec_Lib.\n\n\
   Open Scope hacspec_scope.\n\
   Import choice.Choice.Exports.\n\n\
   From RecordUpdate Require Import RecordUpdate.\n\n\
   Import RecordSetNotations.\n\n\
   Obligation Tactic := (* try timeout 8 *) solve_ssprove_obligations.\n"

let translate m (_bo : BackendOptions.t) ~(bundles : AST.item list list)
    (items : AST.item list) : Types.file list =
  let analysis_data = StaticAnalysis.analyse items in
  U.group_items_by_namespace items
  |> Map.to_alist
  |> List.filter_map ~f:(fun (_, items) ->
      let* first_item = List.hd items in
      Some ((RenderId.render first_item.ident).path, items))
  |> List.map ~f:(fun (ns, items) ->
      let mod_name =
        String.concat ~sep:"_"
          (List.map ~f:(map_first_letter String.uppercase) ns)
      in
      let file_content =
        hardcoded_coq_headers ^ "\n"
        ^ string_of_items m (items, analysis_data)
        ^ "\n"
      in
      Types.
        { path = mod_name ^ ".v"; contents = file_content; sourcemap = None })

let apply_phases (_bo : BackendOptions.t) (i : Ast.Rust.item list) :
    AST.item list =
  TransformToInputLanguage.ditems i
