open Hax_engine
open Utils
open Base

include
  Backend.Make
    (struct
      open Features
      include Off
      include On.Monadic_binding
      include On.Slice
      include On.Macro
      include On.Construct_base
      include On.Quote
      include On.Dyn
      include On.Unsafe
    end)
    (struct
      let backend = Diagnostics.Backend.FStar
    end)

module SubtypeToInputLanguage
    (FA :
      Features.T
        with type mutable_reference = Features.Off.mutable_reference
         and type continue = Features.Off.continue
         and type break = Features.Off.break
         and type mutable_reference = Features.Off.mutable_reference
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
         and type loop = Features.Off.loop
         and type block = Features.Off.block
         and type for_loop = Features.Off.for_loop
         and type while_loop = Features.Off.while_loop
         and type for_index_loop = Features.Off.for_index_loop
         and type state_passing_loop = Features.Off.state_passing_loop
         and type fold_like_loop = Features.Off.fold_like_loop
         and type match_guard = Features.Off.match_guard
         and type trait_item_default = Features.Off.trait_item_default) =
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
        include Features.SUBTYPE.On.Quote
        include Features.SUBTYPE.On.Dyn
        include Features.SUBTYPE.On.Unsafe
      end)

  let metadata = Phase_utils.Metadata.make (Reject (NotInBackendLang backend))
end

module AST = Ast.Make (InputLanguage)

module BackendOptions = struct
  type t = Hax_engine.Types.f_star_options
end

open Ast

module FStarNamePolicy = struct
  include Concrete_ident.DefaultNamePolicy

  [@@@ocamlformat "disable"]

  let anonymous_field_transform index = "_" ^ index

  let reserved_words = Hash_set.of_list (module String) ["attributes";"noeq";"unopteq";"and";"assert";"assume";"begin";"by";"calc";"class";"default";"decreases";"b2t";"effect";"eliminate";"else";"end";"ensures";"exception";"exists";"false";"friend";"forall";"fun";"λ";"function";"if";"in";"include";"inline";"inline_for_extraction";"instance";"introduce";"irreducible";"let";"logic";"match";"returns";"as";"module";"new";"new_effect";"layered_effect";"polymonadic_bind";"polymonadic_subcomp";"noextract";"of";"open";"opaque";"private";"quote";"range_of";"rec";"reifiable";"reify";"reflectable";"requires";"set_range_of";"sub_effect";"synth";"then";"total";"true";"try";"type";"unfold";"unfoldable";"val";"when";"with";"_";"__SOURCE_FILE__";"__LINE__";"match";"if";"let";"and";"string"]
end

module RenderId = Concrete_ident.MakeRenderAPI (FStarNamePolicy)
module U = Ast_utils.Make (InputLanguage)
module Visitors = Ast_visitors.Make (InputLanguage)
open AST
module F = Fstar_ast
module Destruct = Ast_destruct.Make (InputLanguage)

module Context = struct
  type t = {
    current_namespace : string list;
    items : item list;
    interface_mode : bool;
    line_width : int;
  }
end

(** Convers a namespace to a module name *)
let module_name (ns : string list) : string =
  String.concat ~sep:"." (List.map ~f:(map_first_letter String.uppercase) ns)

(** Set to true when extracting core_models (HAX_CORE_MODELS_EXTRACTION_MODE set
    to 'on') *)
let hax_core_models_extraction =
  Sys.getenv "HAX_CORE_MODELS_EXTRACTION_MODE"
  |> [%equal: string option] (Some "on")

module Make
    (Attrs : Attrs.WITH_ITEMS)
    (Ctx : sig
      val ctx : Context.t
    end) =
struct
  open Ctx

  module StringToFStar = struct
    let catch_parsing_error (type a b) kind span (f : a -> b) x =
      try f x
      with e ->
        let kind =
          Types.FStarParseError
            {
              fstar_snippet = "";
              details =
                "While parsing a " ^ kind ^ ", error: "
                ^ Base.Error.to_string_hum (Base.Error.of_exn e);
            }
        in
        Error.raise { span; kind }

    let term span = catch_parsing_error "term" span F.term_of_string
  end

  let doc_to_string : PPrint.document -> string =
    FStar_Pprint.pretty_string 1.0 (Z.of_int ctx.line_width)

  let term_to_string : F.AST.term -> string =
    FStar_Parser_ToDocument.term_to_document >> doc_to_string

  let pat_to_string : F.AST.pattern -> string =
    FStar_Parser_ToDocument.pat_to_document >> doc_to_string

  let decl_to_string : F.AST.decl -> string =
    FStar_Parser_ToDocument.decl_to_document >> doc_to_string

  let pprim_ident (span : span) (id : primitive_ident) =
    match id with
    | Deref -> Error.assertion_failure span "pprim_ident Deref"
    | Cast -> F.lid [ "cast" ]
    | LogicalOp op -> (
        match op with
        | And -> F.lid [ "Prims"; "op_AmpAmp" ]
        | Or -> F.lid [ "Prims"; "op_BarBar" ])

  let pnegative = function true -> "-" | false -> ""

  let dummy_clone_impl =
    StringToFStar.term Span.default
      {fstar|{
        f_clone = (fun x -> x);
        f_clone_pre = (fun _ -> True);
        f_clone_post = (fun _ _ -> True);
      }|fstar}

  (* Print a literal as an F* constant *)
  let rec pliteral_as_const span (e : literal) =
    match e with
    | String s -> F.Const.Const_string (s, F.dummyRange)
    (* This is wrong for non-ASCII chars: https://github.com/pereq-dev/hax/issues/1953 *)
    | Char c -> F.Const.Const_char (Char.to_int (String.get c 0))
    | Int { value; kind = { size; signedness }; negative } ->
        Error.unimplemented
          ~details:
            "Integers cannot be printed as constants, they can only be printed \
             as expressions."
          span
    | Float _ ->
        Error.unimplemented
          ~details:
            "Floats cannot be printed as constants, they can only be printed \
             as expressions."
          span
    | Bool b -> F.Const.Const_bool b

  (* Print a literal appearing in a pattern as an F* pattern *)
  let rec pliteral_as_pat span (e : literal) =
    match e with
    | Int { value; kind = { size; signedness }; negative } ->
        let pat_name =
          F.pat
          @@ F.AST.PatName (F.lid [ "Rust_primitives"; "Integers"; "MkInt" ])
        in
        let mk_const c = F.AST.PatConst c |> F.pat in
        let mk_int value negative =
          mk_const (F.Const.Const_int (pnegative negative ^ value, None))
        in
        F.pat_app pat_name @@ [ mk_int value negative ]
    | Float _ ->
        Error.unimplemented ~issue_id:464
          ~details:"Pattern matching on floats is not yet supported." span
    | _ -> F.pat @@ F.AST.PatConst (pliteral_as_const span e)

  let pliteral_as_expr span (e : literal) =
    let mk_const c = F.AST.Const c |> F.term in
    let mk_int value negative =
      mk_const (F.Const.Const_int (pnegative negative ^ value, None))
    in
    match e with
    | Int { value; kind = { size; signedness }; negative = n } ->
        let f =
          match (size, signedness) with
          | S8, Signed -> F.lid [ "mk_i8" ]
          | S16, Signed -> F.lid [ "mk_i16" ]
          | S32, Signed -> F.lid [ "mk_i32" ]
          | S64, Signed -> F.lid [ "mk_i64" ]
          | S128, Signed -> F.lid [ "mk_i128" ]
          | SSize, Signed -> F.lid [ "mk_isize" ]
          | S8, Unsigned -> F.lid [ "mk_u8" ]
          | S16, Unsigned -> F.lid [ "mk_u16" ]
          | S32, Unsigned -> F.lid [ "mk_u32" ]
          | S64, Unsigned -> F.lid [ "mk_u64" ]
          | S128, Unsigned -> F.lid [ "mk_u128" ]
          | SSize, Unsigned -> F.lid [ "mk_usize" ]
        in
        F.mk_e_app (F.term @@ F.AST.Name f) [ mk_int value n ]
    | Float { value; negative; _ } ->
        F.mk_e_app
          (F.term_of_lid [ "mk_float" ])
          [
            mk_const
              (F.Const.Const_string (pnegative negative ^ value, F.dummyRange));
          ]
    | _ -> mk_const @@ pliteral_as_const span e

  let pconcrete_ident (id : concrete_ident) =
    let id = RenderId.render id in
    let path = ctx.current_namespace in
    if [%eq: string list] path id.path then F.lid [ id.name ]
    else F.lid (id.path @ [ id.name ])

  let rec pglobal_ident (span : span) (id : global_ident) =
    match id with
    | `Concrete cid -> pconcrete_ident cid
    | `Primitive prim_id -> pprim_ident span prim_id
    | `TupleType 0 -> F.lid [ "prims"; "unit" ]
    | `TupleCons n when n <= 1 ->
        Error.assertion_failure span
          ("Got a [TupleCons " ^ string_of_int n ^ "]")
    | `TupleType n when n <= 14 ->
        F.lid [ "FStar"; "Pervasives"; "tuple" ^ string_of_int n ]
    | `TupleCons n when n <= 14 ->
        F.lid [ "FStar"; "Pervasives"; "Mktuple" ^ string_of_int n ]
    | `TupleType n | `TupleCons n ->
        let reason = "F* doesn't support tuple of size greater than 14" in
        Error.raise
          {
            kind = UnsupportedTupleSize { tuple_size = Int64.of_int n; reason };
            span;
          }
    | `TupleField _ | `Projector _ ->
        Error.assertion_failure span
          ("pglobal_ident: expected to be handled somewhere else: "
         ^ show_global_ident id)

  let plocal_ident_str (e : Local_ident.t) =
    RenderId.local_ident
      (match String.chop_prefix ~prefix:"impl " e.name with
      | Some name ->
          let name = "impl_" ^ Int.to_string ([%hash: string] name) in
          { e with name }
      | _ -> e)

  let plocal_ident = plocal_ident_str >> F.id

  let pfield_ident span (f : global_ident) : F.Ident.lident =
    match f with
    | `Concrete cid -> pconcrete_ident cid
    | `Projector (`TupleField (n, len)) | `TupleField (n, len) ->
        F.lid [ "_" ^ Int.to_string (n + 1) ]
    | `Projector (`Concrete cid) -> pconcrete_ident cid
    | _ ->
        Error.assertion_failure span
          ("pfield_ident: not a valid field name in F* backend: "
         ^ show_global_ident f)

  let index_of_field_concrete id =
    try Some (Int.of_string @@ (RenderId.render id).name) with _ -> None

  let index_of_field = function
    | `Concrete id -> index_of_field_concrete id
    | `TupleField (nth, _) -> Some nth
    | _ -> None

  let is_field_an_index = index_of_field >> Option.is_some

  let operators =
    let c = Global_ident.of_name ~value:true in
    [
      (c Rust_primitives__hax__array_of_list, (3, ".[]<-"));
      (c Core__ops__index__Index__index, (2, ".[]"));
      (c Core__ops__bit__Not__not, (1, "~."));
      (c Rust_primitives__hax__machine_int__not, (1, "~."));
      (c Rust_primitives__hax__machine_int__add, (2, "+!"));
      (c Rust_primitives__hax__machine_int__sub, (2, "-!"));
      (c Rust_primitives__hax__machine_int__div, (2, "/!"));
      (c Rust_primitives__hax__machine_int__mul, (2, "*!"));
      (c Rust_primitives__hax__machine_int__rem, (2, "%!"));
      (c Rust_primitives__hax__machine_int__shl, (2, "<<!"));
      (c Rust_primitives__hax__machine_int__shr, (2, ">>!"));
      (c Rust_primitives__hax__machine_int__bitxor, (2, "^."));
      (c Rust_primitives__hax__machine_int__bitor, (2, "|."));
      (c Rust_primitives__hax__machine_int__bitand, (2, "&."));
      (c Core__cmp__PartialEq__eq, (2, "=."));
      (c Rust_primitives__hax__machine_int__eq, (2, "=."));
      (c Core__cmp__PartialEq__ne, (2, "<>."));
      (c Rust_primitives__hax__machine_int__ne, (2, "<>."));
      (c Rust_primitives__hax__machine_int__le, (2, "<=."));
      (c Rust_primitives__hax__machine_int__lt, (2, "<."));
      (c Rust_primitives__hax__machine_int__gt, (2, ">."));
      (c Rust_primitives__hax__machine_int__ge, (2, ">=."));
      (`Primitive (LogicalOp And), (2, "&&"));
      (`Primitive (LogicalOp Or), (2, "||"));
      (c Rust_primitives__hax__int__add, (2, "+"));
      (c Rust_primitives__hax__int__sub, (2, "-"));
      (c Rust_primitives__hax__int__mul, (2, "*"));
      (* Neither `Int` division nor `Int` remainder has an infix form here:
         the bigint operations truncate towards zero, while F*'s `/` and `%`
         on `int` are Euclidean. Division prints as a call to
         `Rust_primitives.Hax.Int.div`, which spells out the difference. *)
      (c Rust_primitives__hax__int__neg, (1, "-"));
      (c Rust_primitives__hax__int__ge, (2, ">="));
      (c Rust_primitives__hax__int__le, (2, "<="));
      (c Rust_primitives__hax__int__gt, (2, ">"));
      (c Rust_primitives__hax__int__lt, (2, "<"));
      (c Rust_primitives__hax__int__ne, (2, "<>"));
      (c Rust_primitives__hax__int__eq, (2, "="));
      (c Hax_lib__prop__constructors__and, (2, "/\\"));
      (c Hax_lib__prop__constructors__or, (2, "\\/"));
      (c Hax_lib__prop__constructors__not, (1, "~"));
      (c Hax_lib__prop__constructors__eq, (2, "=="));
      (c Hax_lib__prop__constructors__ne, (2, "=!="));
      (c Hax_lib__prop__constructors__implies, (2, "==>"));
    ]
    |> Map.of_alist_exn (module Global_ident)

  let rec pty span (t : ty) =
    match t with
    | TBool -> F.term_of_lid [ "bool" ]
    | TChar -> F.term_of_lid [ "FStar"; "Char"; "char" ]
    | TInt k -> F.term_of_lid [ show_int_kind k ]
    | TStr -> F.term_of_lid [ "string" ]
    | TSlice { ty; _ } ->
        F.mk_e_app (F.term_of_lid [ "t_Slice" ]) [ pty span ty ]
    | TApp { ident = `TupleType 0 as ident; args = [] } ->
        F.term @@ F.AST.Name (pglobal_ident span ident)
    | TApp { ident = `TupleType 1; args = [ GType ty ] } -> pty span ty
    | TApp { ident = `TupleType n; args } when n >= 2 -> (
        let args =
          List.filter_map
            ~f:(function GType t -> Some (pty span t) | _ -> None)
            args
        in
        let mk_star a b = F.term @@ F.AST.Op (F.id "&", [ a; b ]) in
        match args with
        | hd :: tl ->
            F.term @@ F.AST.Paren (List.fold_left ~init:hd ~f:mk_star tl)
        | _ -> Error.assertion_failure span "Tuple type: bad arity")
    | TApp { ident; args } ->
        let base = F.term @@ F.AST.Name (pglobal_ident span ident) in
        let args = List.map ~f:(pgeneric_value span) args in
        F.mk_e_app base args
    | TArrow (inputs, output) ->
        F.mk_e_arrow (List.map ~f:(pty span) inputs) (pty span output)
    | TFloat _ -> F.term_of_lid [ "float" ]
    | TArray { typ; length } ->
        F.mk_e_app (F.term_of_lid [ "t_Array" ]) [ pty span typ; pexpr length ]
    | TParam i -> F.term @@ F.AST.Var (F.lid_of_id @@ plocal_ident i)
    | TAssociatedType { impl = { kind = Self; _ }; item } ->
        F.term @@ F.AST.Var (F.lid [ (RenderId.render item).name ])
    | TAssociatedType { impl; item } -> (
        match pimpl_expr span impl with
        | Some impl ->
            F.term @@ F.AST.Project (impl, F.lid [ (RenderId.render item).name ])
        | None -> F.term @@ F.AST.Wild)
    | TOpaque s -> F.term @@ F.AST.Wild
    | TDyn { goals; _ } ->
        let traits = List.map ~f:(pdyn_trait_goal span) goals in
        let dyn = F.AST.Var (F.lid [ "dyn" ]) |> F.term in
        let length =
          F.AST.Const
            (FStar_Const.Const_int (List.length goals |> Int.to_string, None))
          |> F.term
        in
        F.mk_e_app dyn (length :: traits)
    | _ -> .

  and pdyn_trait_goal span (goal : dyn_trait_goal) =
    (* This introduces a potential shadowing *)
    let type_var = "z" in
    let pat = F.pat @@ F.AST.PatVar (F.id type_var, None, []) in
    let trait = F.AST.Var (pconcrete_ident goal.trait) |> F.term in
    let args =
      (F.AST.Var (F.lid [ type_var ]) |> F.term)
      :: List.map ~f:(pgeneric_value span) goal.non_self_args
    in
    F.mk_e_abs [ pat ] (F.mk_e_app trait args)

  and pimpl_expr span (ie : impl_expr) =
    let some = Option.some in
    match ie.kind with
    | Concrete tr -> c_trait_goal span tr |> some
    | LocalBound { id } ->
        let local_ident =
          Local_ident.{ name = id; id = Local_ident.mk_id Expr 0 }
        in
        F.term @@ F.AST.Var (F.lid_of_id @@ plocal_ident local_ident) |> some
    | Parent { impl; ident } when [%matches? Self _] impl.kind ->
        let trait = "_super_" ^ ident.name in
        F.term_of_lid [ trait ] |> some
    | Parent { impl; ident } ->
        let* impl = pimpl_expr span impl in
        let trait = "_super_" ^ ident.name in
        F.term @@ F.AST.Project (impl, F.lid [ trait ]) |> some
    | ImplApp { impl; args = [] } -> pimpl_expr span impl
    | ImplApp { impl; args } ->
        let* impl = pimpl_expr span impl in
        let* args = List.map ~f:(pimpl_expr span) args |> Option.all in
        F.mk_e_app impl args |> some
    | Projection { impl; item; ident } when [%matches? Self _] impl.kind ->
        F.term_of_lid
          [ (pconcrete_ident item |> F.Ident.text_of_lid) ^ "_" ^ ident.name ]
        |> some
    | Dyn _ -> F.term_of_lid [ "_Dyn" ] |> some
    | Builtin _ -> F.term_of_lid [ "_Builtin" ] |> some
    | _ -> None

  and c_trait_goal span trait_goal =
    let trait = F.term @@ F.AST.Name (pconcrete_ident trait_goal.trait) in
    List.map ~f:(pgeneric_value span) trait_goal.args |> F.mk_e_app trait

  and pgeneric_value span (g : generic_value) =
    match g with
    | GType ty -> pty span ty
    | GConst e -> pexpr e
    | GLifetime _ -> .

  and ppat (p : pat) = ppat' true p

  and ppat' (shallow : bool) (p : pat) =
    let ppat = ppat' false in
    match p.p with
    | PWild -> F.wild
    | PAscription { typ; pat = { p = PBinding _; _ } as pat } ->
        F.pat @@ F.AST.PatAscribed (ppat pat, (pty p.span typ, None))
    | PAscription { pat; _ } -> ppat pat
    | PBinding
        {
          mut = Immutable;
          mode = _;
          subpat = None;
          var;
          typ = _ (* we skip type annot here *);
        } ->
        F.pat @@ F.AST.PatVar (plocal_ident var, None, [])
    | POr { subpats } when shallow ->
        F.pat @@ F.AST.PatOr (List.map ~f:ppat subpats)
    | POr _ ->
        Error.assertion_failure p.span
          "Nested disjuntive patterns should have been eliminated by phase \
           `HoistDisjunctions` (see PR #830)."
    | PArray { args } -> F.pat @@ F.AST.PatList (List.map ~f:ppat args)
    | PConstruct { constructor = `TupleCons 0; fields = [] } ->
        F.pat @@ F.AST.PatConst F.Const.Const_unit
    | PConstruct { constructor = `TupleCons 1; fields = [ { pat } ] } ->
        ppat pat
    | PConstruct { constructor = `TupleCons n; fields } ->
        F.pat
        @@ F.AST.PatTuple (List.map ~f:(fun { pat } -> ppat pat) fields, false)
    | PConstruct { constructor; fields; is_record; is_struct } ->
        let pat_rec () =
          F.pat @@ F.AST.PatRecord (List.map ~f:pfield_pat fields)
        in
        if is_struct && is_record then pat_rec ()
        else
          let pat_name =
            F.pat @@ F.AST.PatName (pglobal_ident p.span constructor)
          in
          F.pat_app pat_name
          @@
          if is_record then [ pat_rec () ]
          else List.map ~f:(fun { field; pat } -> ppat pat) fields
    | PConstant { lit } -> pliteral_as_pat p.span lit
    | _ -> .

  and pfield_pat ({ field; pat } : field_pat) =
    (pglobal_ident pat.span field, ppat pat)

  and pexpr (e : expr) =
    try pexpr_unwrapped e
    with Diagnostics.SpanFreeError.Exn _ ->
      (* let typ = *)
      (* try pty e.span e.typ *)
      (* with Diagnostics.SpanFreeError _ -> U.hax_failure_typ *)
      (* in *)
      F.term @@ F.AST.Const (F.Const.Const_string ("failure", F.dummyRange))

  and fun_application ~span f args ~trait_generic_args ~generic_args =
    let pgeneric_args ?qualifier =
      let qualifier_or default = Option.value ~default qualifier in
      List.map ~f:(function
        | GConst const -> (pexpr const, qualifier_or F.AST.Nothing)
        | GLifetime _ -> .
        | GType ty -> (pty span ty, qualifier_or F.AST.Hash))
    in
    let args = List.map ~f:(pexpr &&& Fn.const F.AST.Nothing) args in
    let trait_generic_args =
      Option.map
        ~f:
          (pgeneric_args ~qualifier:F.AST.Hash
          >> Fn.flip ( @ ) [ (F.solve, F.AST.Hash) ])
        trait_generic_args
      |> Option.value ~default:[]
    in
    F.mk_app f (trait_generic_args @ pgeneric_args generic_args @ args)

  and pexpr_unwrapped (e : expr) =
    match e.e with
    | Literal l -> pliteral_as_expr e.span l
    | LocalVar local_ident ->
        F.term @@ F.AST.Var (F.lid_of_id @@ plocal_ident local_ident)
    | GlobalVar (`TupleCons 0)
    | Construct { constructor = `TupleCons 0; fields = [] } ->
        F.AST.unit_const F.dummyRange
    | GlobalVar global_ident ->
        F.term @@ F.AST.Var (pglobal_ident e.span @@ global_ident)
    | App { f = { e = GlobalVar f; _ }; args = [ x ] }
      when Global_ident.eq_name Hax_lib__prop__constructors__from_bool f ->
        let x = pexpr x in
        F.mk_e_app (F.term_of_lid [ "b2t" ]) [ x ]
    | App
        {
          f = { e = GlobalVar f; _ };
          args = [ { e = Closure { params = [ x ]; body = phi; _ }; _ } ];
        }
      when Global_ident.eq_name Hax_lib__prop__constructors__forall f ->
        let phi = pexpr phi in
        let binders =
          let b = Destruct.pat_PBinding x |> Option.value_exn in
          [
            F.AST.
              {
                b = F.AST.Annotated (plocal_ident b.var, pty x.span b.typ);
                brange = F.dummyRange;
                blevel = Un;
                aqual = None;
                battributes = [];
              };
          ]
        in
        F.term @@ F.AST.QForall (binders, ([], []), phi)
    | App
        {
          f = { e = GlobalVar f; _ };
          args = [ { e = Closure { params = [ x ]; body = phi; _ }; _ } ];
        }
      when Global_ident.eq_name Hax_lib__prop__constructors__exists f ->
        let phi = pexpr phi in
        let binders =
          let b = Destruct.pat_PBinding x |> Option.value_exn in
          [
            F.AST.
              {
                b = F.AST.Annotated (plocal_ident b.var, pty x.span b.typ);
                brange = F.dummyRange;
                blevel = Un;
                aqual = None;
                battributes = [];
              };
          ]
        in
        F.term @@ F.AST.QExists (binders, ([], []), phi)
    | App
        {
          f = { e = GlobalVar (`Projector (`TupleField (_, 1))) };
          args = [ arg ];
        } ->
        pexpr arg
    | App
        {
          f = { e = GlobalVar (`Projector (`TupleField (n, len))) };
          args = [ arg ];
        } ->
        F.term
        @@ F.AST.Project (pexpr arg, F.lid [ "_" ^ string_of_int (n + 1) ])
    | App { f = { e = GlobalVar (`Projector (`Concrete cid)) }; args = [ arg ] }
      ->
        F.term @@ F.AST.Project (pexpr arg, pconcrete_ident cid)
    | App { f = { e = GlobalVar x }; args } when Map.mem operators x ->
        let arity, op = Map.find_exn operators x in
        if List.length args <> arity then
          Error.assertion_failure e.span
            ("pexpr: bad arity for operator application (" ^ op ^ ")");
        F.term @@ F.AST.Op (F.Ident.id_of_text op, List.map ~f:pexpr args)
    | App
        {
          f = { e = GlobalVar f; _ };
          args = [ { e = Literal (String s); _ } ];
          generic_args = _;
        }
      when Global_ident.eq_name Hax_lib__int__Impl_7___unsafe_from_str f ->
        (match
           String.chop_prefix ~prefix:"-" s
           |> Option.value ~default:s
           |> String.filter ~f:([%matches? '0' .. '9'] >> not)
         with
        | "" -> ()
        | s ->
            Error.assertion_failure e.span
            @@ "pexpr: expected a integer, found the following non-digit \
                chars: '" ^ s ^ "'");
        F.AST.Const (F.Const.Const_int (s, None)) |> F.term
    | App { f; args; generic_args; bounds_impls = _; trait } ->
        let trait_generic_args = Option.map ~f:snd trait in
        fun_application (pexpr f) args ~span:e.span ~trait_generic_args
          ~generic_args
    | If { cond; then_; else_ } ->
        F.term
        @@ F.AST.If
             ( pexpr cond,
               None,
               None,
               pexpr then_,
               Option.value_map else_ ~default:F.unit ~f:pexpr )
    | Array l ->
        let len = List.length l in
        let body = F.AST.mkConsList F.dummyRange (List.map ~f:pexpr l) in
        let array_of_list =
          let id =
            Concrete_ident.of_name ~value:true
              Rust_primitives__hax__array_of_list
          in
          F.term @@ F.AST.Name (pconcrete_ident id)
        in
        let list_ident = F.id "list" in
        let list = F.term_of_lid [ "list" ] in
        let assert_norm =
          F.term_of_lid [ "FStar"; "Pervasives"; "assert_norm" ]
        in
        let equality = F.term_of_lid [ "Prims"; "eq2" ] in
        let length = F.term_of_lid [ "List"; "Tot"; "length" ] in
        let length = F.mk_e_app length [ list ] in
        let len =
          F.term @@ F.AST.Const (F.Const.Const_int (Int.to_string len, None))
        in
        let array = F.mk_e_app array_of_list [ len; list ] in
        let formula = F.mk_e_app equality [ length; len ] in
        let assertion = F.mk_e_app assert_norm [ formula ] in
        let pat = F.AST.PatVar (list_ident, None, []) |> F.pat in
        let pat =
          match l with
          | [] ->
              let list_ty =
                let prims_list = F.term_of_lid [ "Prims"; "list" ] in
                let inner_typ =
                  match e.typ with
                  | TArray { typ; _ } -> pty e.span typ
                  | _ ->
                      Error.assertion_failure e.span
                        "Malformed type for array literal"
                in
                F.mk_e_app prims_list [ inner_typ ]
              in
              F.pat @@ F.AST.PatAscribed (pat, (list_ty, None))
          | _ -> pat
        in
        F.term
        @@ F.AST.Let
             ( NoLetQualifier,
               [ (None, (pat, body)) ],
               F.term @@ F.AST.Seq (assertion, array) )
    | Let { lhs; rhs; body; monadic = Some (monad, _) } ->
        let p =
          F.pat @@ F.AST.PatAscribed (ppat lhs, (pty lhs.span lhs.typ, None))
        in
        let op =
          "let"
          ^
          match monad with
          | MResult _ -> "|"
          | MOption -> "?"
          | MException _ -> "!"
        in
        F.term @@ F.AST.LetOperator ([ (F.id op, p, pexpr rhs) ], pexpr body)
    | Let { lhs; rhs; body; monadic = None } ->
        let rec ascribe_tuple_components pattern =
          match pattern with
          | { p = PConstruct { constructor = `TupleCons n1; fields; _ }; _ }
            when n1 > 1 ->
              (* F* type inference works better if the ascription is on each component intead of the whole tuple. *)
              F.pat
              @@ F.AST.PatTuple
                   ( List.map
                       ~f:(fun { pat } -> ascribe_tuple_components pat)
                       fields,
                     false )
          | _ ->
              (* TODO: temp patch that remove annotation when we see an associated type *)
              if [%matches? TAssociatedType _] @@ U.remove_tuple1 pattern.typ
              then ppat pattern
              else
                F.pat
                @@ F.AST.PatAscribed
                     (ppat pattern, (pty pattern.span pattern.typ, None))
        in
        F.term
        @@ F.AST.Let
             ( NoLetQualifier,
               [ (None, (ascribe_tuple_components lhs, pexpr rhs)) ],
               pexpr body )
    | EffectAction _ -> .
    | Match { scrutinee; arms } ->
        F.term
        @@ F.AST.Match (pexpr scrutinee, None, None, List.map ~f:parm arms)
    | Ascription { e; typ } ->
        F.term @@ F.AST.Ascribed (pexpr e, pty e.span typ, None, false)
    | Construct { constructor = `TupleCons 1; fields = [ (_, e') ]; base } ->
        pexpr e'
    | Construct { constructor = `TupleCons n; fields; base = None } ->
        F.AST.mkTuple (List.map ~f:(snd >> pexpr) fields) F.dummyRange
    | Construct
        { is_record = true; is_struct = true; constructor; fields; base } ->
        F.term
        @@ F.AST.Record
             ( Option.map ~f:(fst >> pexpr) base,
               List.map
                 ~f:(fun (f, e) -> (pfield_ident e.span f, pexpr e))
                 fields )
    | Construct { is_record = false; constructor; fields; base } ->
        if [%matches? Some _] base then
          Diagnostics.failure ~context:(Backend FStar) ~span:e.span
            (AssertionFailure { details = "non-record type with base present" });
        F.mk_e_app (F.term @@ F.AST.Name (pglobal_ident e.span constructor))
        @@ List.map ~f:(snd >> pexpr) fields
    | Construct { is_record = true; constructor; fields; base } ->
        let r =
          F.term
          @@ F.AST.Record
               ( Option.map ~f:(fst >> pexpr) base,
                 List.map
                   ~f:(fun (f, e') -> (pglobal_ident e.span f, pexpr e'))
                   fields )
        in
        F.mk_e_app
          (F.term @@ F.AST.Name (pglobal_ident e.span constructor))
          [ r ]
    | Closure { params; body } ->
        let params =
          List.mapi
            ~f:(fun i p ->
              match p.p with
              | PBinding { var; subpat = None; _ } -> (var, p)
              | _ ->
                  ( Local_ident.
                      { name = "temp_" ^ Int.to_string i; id = mk_id Expr (-1) },
                    p ))
            params
        in
        let body =
          let f (lid, (pat : pat)) =
            let rhs = { e = LocalVar lid; span = pat.span; typ = pat.typ } in
            U.make_let pat rhs
          in
          List.fold_right ~init:body ~f params
        in
        let mk_pat ((lid, pat) : local_ident * pat) =
          ppat (U.make_var_pat lid pat.typ pat.span)
        in
        F.mk_e_abs (List.map ~f:mk_pat params) (pexpr body)
    | Return { e } ->
        F.term @@ F.AST.App (F.term_of_lid [ "RETURN_STMT" ], pexpr e, Nothing)
    | MacroInvokation { macro; args; witness } ->
        Error.raise
        @@ {
             kind = UnsupportedMacro { id = [%show: global_ident] macro };
             span = e.span;
           }
    | Quote quote -> pquote e.span quote |> StringToFStar.term e.span
    | _ -> .

  (** Prints a `quote` to a string *)
  and pquote span { contents; _ } =
    List.map
      ~f:(function
        | Verbatim code -> code
        | Expr e -> pexpr e |> term_to_string
        | Pattern p -> ppat p |> pat_to_string
        | Typ p -> pty span p |> term_to_string)
      contents
    |> String.concat

  and parm { arm = { arm_pat; body } } = (ppat arm_pat, None, pexpr body)

  module FStarBinder = struct
    type kind = Implicit | Tcresolve | Explicit
    type t = { kind : kind; ident : F.Ident.ident; typ : F.AST.term }

    let make_explicit x = { x with kind = Explicit }

    let implicit_to_explicit x =
      if [%matches? Tcresolve] x.kind then x else make_explicit x

    let of_generic_param span (p : generic_param) : t =
      let ident = plocal_ident p.ident in
      match p.kind with
      | GPLifetime _ -> Error.assertion_failure span "pgeneric_param:LIFETIME"
      | GPType -> { kind = Implicit; typ = F.type0_term; ident }
      | GPConst { typ } -> { kind = Explicit; typ = pty span typ; ident }

    let of_generic_constraint span (nth : int) (c : generic_constraint) =
      match c with
      | GCLifetime _ -> .
      | GCType { goal; name } ->
          let typ = c_trait_goal span goal in
          Some { kind = Tcresolve; ident = F.id name; typ }
      | GCProjection { impl = { kind = LocalBound { id }; _ }; assoc_item; typ }
        ->
          let proj =
            F.term
            @@ F.AST.Project
                 (F.term @@ F.AST.Var (F.lid [ id ]), pconcrete_ident assoc_item)
          in
          let typ =
            F.mk_refined "_" (F.term_of_string "unit") (fun ~x ->
                F.term
                @@ F.AST.Op (FStar_Ident.id_of_text "==", [ proj; pty span typ ]))
          in
          Some { kind = Implicit; typ; ident = FStar_Ident.id_of_text "_" }
      | _ -> None

    let of_generics span generics : t list =
      List.map ~f:(of_generic_param span) generics.params
      @ (generics.constraints
        |> List.sort ~compare:(fun c1 c2 ->
            match (c1, c2) with
            | GCType _, GCProjection _ -> -1
            | GCProjection _, GCType _ -> 1
            | _ -> 0)
        |> List.filter_mapi ~f:(of_generic_constraint span))

    let of_typ span (nth : int) typ : t =
      let ident = F.id ("x" ^ Int.to_string nth) in
      { kind = Explicit; ident; typ = pty span typ }

    (** Makes an F* binder from a name and an F* type *)
    let of_named_fstar_typ span name typ : t =
      let ident = plocal_ident name in
      { kind = Explicit; ident; typ }

    (** Makes an F* binder from a name and an hax type *)
    let of_named_typ span name = pty span >> of_named_fstar_typ span name

    let to_pattern (x : t) : F.AST.pattern =
      let subpat =
        match x.kind with
        | Tcresolve ->
            let tcresolve =
              Some
                (F.AST.Meta
                   (F.term @@ F.AST.Var FStar_Parser_Const.tcresolve_lid))
            in
            F.pat @@ F.AST.PatVar (x.ident, tcresolve, [])
        | _ ->
            let aqual =
              match x.kind with Implicit -> Some F.AST.Implicit | _ -> None
            in
            F.pat @@ F.AST.PatVar (x.ident, aqual, [])
      in
      F.pat @@ F.AST.PatAscribed (subpat, (x.typ, None))

    let to_typ (x : t) : F.AST.term = x.typ
    let to_ident (x : t) : F.Ident.ident = x.ident

    let to_term (x : t) : F.AST.term =
      F.term @@ F.AST.Var (FStar_Ident.lid_of_ns_and_id [] (to_ident x))

    let to_imp (x : t) : F.AST.imp =
      match x.kind with Tcresolve | Implicit -> Hash | Explicit -> Nothing

    let to_qualified_term : t -> F.AST.term * F.AST.imp = to_term &&& to_imp

    let to_qualifier (x : t) : F.AST.arg_qualifier option =
      match x.kind with
      | Tcresolve -> Some TypeClassArg
      | Implicit -> Some Implicit
      | Explicit -> None

    let to_binder (x : t) : F.AST.binder =
      F.AST.
        {
          b = F.AST.Annotated (x.ident, x.typ);
          brange = F.dummyRange;
          blevel = Un;
          aqual = to_qualifier x;
          battributes = [];
        }
  end

  let rec pgeneric_constraint_type span (c : generic_constraint) =
    match c with
    | GCLifetime _ ->
        Error.assertion_failure span "pgeneric_constraint_bd:LIFETIME"
    | GCType { goal; name = _ } -> c_trait_goal span goal

  let pmaybe_refined_ty span (free_variables : string list) (attrs : attrs)
      (binder_name : string) (ty : ty) : F.AST.term =
    match Attrs.associated_refinement_in_type span free_variables attrs with
    | Some refinement ->
        F.mk_refined binder_name (pty span ty) (fun ~x -> pexpr refinement)
    | None -> pty span ty

  let add_clauses_effect_type ~self ~no_tot_abbrev (attrs : attrs) typ :
      F.AST.typ =
    let attr_term ?keep_last_args ?map_expr kind f =
      (* A clause on a method with a `self` produces a function whose first argument is `self_`.
         `subst_self` will substitute that first argument `self_` into the provided local identifier `self`.
      *)
      let subst_self : (expr -> expr) option =
        (* If `self` was present on the original function.  *)
        let* self = self in
        (* Lookup the pre/post/decreases function, get the first argument: that is `self`. *)
        let* self' =
          let* _, params, _ = Attrs.associated_fn kind attrs in
          let* first_param = List.hd params in
          let* { var; _ } = Destruct.pat_PBinding first_param.pat in
          Some var
        in
        let f id = if [%eq: local_ident] self' id then self else id in
        Some ((U.Mappers.rename_local_idents f)#visit_expr ())
      in
      Attrs.associated_expr ?keep_last_args kind attrs
      |> Option.map
           ~f:
             (Option.value ~default:Fn.id subst_self
             >> Option.value ~default:Fn.id map_expr
             >> pexpr >> f >> F.term)
    in
    let extract_any_to_unit_payload =
      let visitor =
        object
          inherit [_] U.Visitors.map as super

          method! visit_expr () e =
            match e.e with
            | App { f = { e = GlobalVar f; _ }; args = [ e ]; _ }
              when Global_ident.eq_name Hax_lib__any_to_unit f ->
                e
            | _ -> super#visit_expr () e
        end
      in
      visitor#visit_expr ()
    in
    let decreases =
      attr_term Decreases ~map_expr:extract_any_to_unit_payload (fun t ->
          F.AST.Decreases (t, None))
    in
    let smtpat =
      let smt_pat = F.term_of_lid [ "SMTPat" ] in
      attr_term SMTPat ~map_expr:extract_any_to_unit_payload (fun t ->
          let payload = F.mk_e_app smt_pat [ t ] in
          (F.AST.mkConsList F.dummyRange [ payload ]).tm)
    in
    let is_lemma = Attrs.lemma attrs in
    let prepost_bundle =
      let trivial_pre = F.term_of_lid [ "Prims"; "l_True" ] in
      let trivial_post =
        if is_lemma then trivial_pre
        else F.mk_e_abs [ F.pat @@ F.AST.PatWild (None, []) ] trivial_pre
      in
      let pre = attr_term Requires (fun t -> F.AST.Requires (t, None)) in
      let post =
        let keep_last_args = if is_lemma then 0 else 1 in
        attr_term ~keep_last_args Ensures (fun t -> F.AST.Ensures (t, None))
      in
      if is_lemma || no_tot_abbrev || Option.is_some pre || Option.is_some post
      then
        Some
          ( Option.value ~default:trivial_pre pre,
            Option.value ~default:trivial_post post )
      else None
    in
    let args =
      (Option.map ~f:(fun (req, ens) -> [ req; ens ]) prepost_bundle
      |> Option.value ~default:[])
      @ Option.to_list decreases @ Option.to_list smtpat
    in
    match args with
    | [] -> typ
    | _ ->
        let mk namespace eff = F.term_of_lid (namespace @ [ eff ]) in
        let prims = mk [ "Prims" ] in
        let eff =
          if Option.is_some prepost_bundle then
            if is_lemma then mk [] "Lemma" else prims "Pure"
          else prims "Tot"
        in
        F.mk_e_app eff (if is_lemma then List.drop args 1 else typ :: args)

  (** Prints doc comments out of a list of attributes *)
  let pdoc_comments attrs =
    attrs
    |> List.filter_map ~f:(fun (attr : attr) ->
        match attr.kind with
        | DocComment { kind; body } -> Some (kind, body)
        | _ -> None)
    |> List.map ~f:(fun (kind, string) ->
        match kind with
        | DCKLine ->
            String.split_lines string
            |> List.map ~f:(fun s -> "///" ^ s)
            |> String.concat_lines
        | DCKBlock -> "(**" ^ string ^ "*)")
    |> List.map ~f:(fun s -> `VerbatimIntf (s, `NoNewline))

  let rec pitem (e : item) :
      [> `Impl of F.AST.decl
      | `Intf of F.AST.decl
      | `VerbatimImpl of string * [ `NoNewline | `Newline ]
      | `VerbatimIntf of string * [ `NoNewline | `Newline ]
      | `Comment of string ]
      list =
    try
      match pitem_unwrapped e with
      | [] -> []
      | printed_items ->
          (* Print comments only for items that are being printed *)
          pdoc_comments e.attrs @ printed_items
    with Diagnostics.SpanFreeError.Exn error ->
      let error = Diagnostics.SpanFreeError.payload error in
      let error = [%show: Diagnostics.Context.t * Diagnostics.kind] error in
      [
        `Comment
          ("item error backend: " ^ error ^ "\n\nLast AST:\n"
          ^ (U.LiftToFullAst.item e |> Print_rust.pitem_str));
      ]

  and pitem_unwrapped (e : item) :
      [> `Impl of F.AST.decl
      | `Intf of F.AST.decl
      | `VerbatimImpl of string * [ `NoNewline | `Newline ]
      | `VerbatimIntf of string * [ `NoNewline | `Newline ]
      | `Comment of string ]
      list =
    let is_erased = Attrs.is_erased e.attrs in
    let erased_impl name ty attrs binders =
      let name' = F.id_prime name in
      let pat = F.AST.PatVar (name, None, []) in
      let term = F.term @@ F.AST.Var (F.lid_of_id @@ name') in
      let pat, term =
        match binders with
        | [] -> (pat, term)
        | _ ->
            ( F.AST.PatApp
                (F.pat pat, List.map ~f:FStarBinder.to_pattern binders),
              List.fold_left binders ~init:term ~f:(fun term binder ->
                  let binder_term, binder_imp =
                    FStarBinder.to_qualified_term binder
                  in
                  F.term @@ F.AST.App (term, binder_term, binder_imp)) )
      in
      [
        F.decl ~quals:[ Assumption ] ~fsti:false ~attrs
        @@ F.AST.Assume (name', ty);
        F.decl
          ~quals:
            (if ctx.interface_mode then []
             else [ Unfold_for_unification_and_vcgen ])
          ~fsti:false
        @@ F.AST.TopLevelLet (NoLetQualifier, [ (F.pat @@ pat, term) ]);
      ]
    in
    match e.v with
    | Alias { name; item } ->
        (* These should come from bundled items (in the case of cyclic module dependencies).
           We make use of this f* feature: https://github.com/FStarLang/FStar/pull/3369 *)
        let bundle = (RenderId.render item).path |> module_name in
        [
          `VerbatimImpl
            ( Printf.sprintf "include %s {%s as %s}" bundle
                (RenderId.render item).name (RenderId.render name).name,
              `Newline );
        ]
    | Fn { name; generics; body; params } ->
        let name = F.id @@ (RenderId.render name).name in
        let pat = F.pat @@ F.AST.PatVar (name, None, []) in
        let generics = FStarBinder.of_generics e.span generics in
        let pat_args =
          List.map ~f:FStarBinder.to_pattern generics
          @ List.map
              ~f:(fun { pat; typ_span; typ } ->
                let span = Option.value ~default:e.span typ_span in
                F.pat @@ F.AST.PatAscribed (ppat pat, (pty span typ, None)))
              params
        in
        let pat = F.pat @@ F.AST.PatApp (pat, pat_args) in
        let qualifier = F.AST.(NoLetQualifier) in
        let impl =
          F.decl ~fsti:false
          @@ F.AST.TopLevelLet (qualifier, [ (pat, pexpr body) ])
        in
        let is_const = List.is_empty params in
        let ty =
          add_clauses_effect_type
            ~self:
              (let* hd = List.hd params in
               let* { var; _ } = Destruct.pat_PBinding hd.pat in
               let*? () = String.equal var.name "self" in
               Some var)
            ~no_tot_abbrev:(ctx.interface_mode && not is_const)
            e.attrs (pty body.span body.typ)
        in
        let arrow_typ =
          F.term
          @@ F.AST.Product
               ( List.map ~f:FStarBinder.to_binder generics
                 @ List.mapi
                     ~f:(fun i { pat; typ_span; typ } ->
                       let name =
                         match pat.p with
                         | PBinding { var; _ } ->
                             Some (RenderId.local_ident var)
                         | _ ->
                             (* TODO: this might generate bad code,
                                see
                                https://github.com/pereq-dev/hax/issues/402
                             *)
                             None
                       in
                       let name = Option.map ~f:F.id name in
                       let span = Option.value ~default:e.span typ_span in
                       pty span typ |> F.binder_of_term ?name)
                     params,
                 ty )
        in
        let pat = F.pat @@ F.AST.PatAscribed (pat, (ty, None)) in
        let full =
          F.decl @@ F.AST.TopLevelLet (qualifier, [ (pat, pexpr body) ])
        in

        let intf = F.decl ~fsti:true (F.AST.Val (name, arrow_typ)) in

        let erased = erased_impl name arrow_typ [] generics in
        let impl, full =
          if is_erased then (erased, erased) else ([ impl ], [ full ])
        in
        if ctx.interface_mode && ((not is_const) || is_erased) then intf :: impl
        else full
    | TyAlias { name; generics; ty } ->
        let pat =
          F.pat @@ F.AST.PatVar (F.id @@ (RenderId.render name).name, None, [])
        in
        let ty, quals =
          (* Adds a refinement if a refinement attribute is detected *)
          match Attrs.associated_expr ~keep_last_args:1 Ensures e.attrs with
          | Some { e = Closure { params = [ binder ]; body; _ }; _ } ->
              let binder, _ =
                U.Expect.pbinding_simple binder |> Option.value_exn
              in
              let ty =
                F.mk_refined (plocal_ident_str binder) (pty e.span ty)
                  (fun ~x -> pexpr body)
              in
              (ty, [])
          | _ -> (pty e.span ty, [ F.AST.Unfold_for_unification_and_vcgen ])
        in
        F.decls ~quals
        @@ F.AST.TopLevelLet
             ( NoLetQualifier,
               [
                 ( F.pat
                   @@ F.AST.PatApp
                        ( pat,
                          FStarBinder.(
                            of_generics e.span generics
                            |> List.map ~f:to_pattern) ),
                   ty );
               ] )
    | Type { name; generics; _ } when is_erased ->
        let generics =
          FStarBinder.of_generics e.span generics
          |> List.map ~f:FStarBinder.implicit_to_explicit
        in
        let ty = F.eqtype_term in
        let arrow_typ =
          F.term
          @@ F.AST.Product (List.map ~f:FStarBinder.to_binder generics, ty)
        in
        let name = F.id @@ (RenderId.render name).name in
        let erased = erased_impl name arrow_typ [] generics in
        let intf = F.decl ~fsti:true (F.AST.Val (name, arrow_typ)) in
        if ctx.interface_mode then intf :: erased else erased
    | Type
        {
          name;
          generics;
          variants = [ { arguments; is_record = true; _ } ];
          is_struct = true;
        } ->
        F.decls
        @@ F.AST.Tycon
             ( false,
               false,
               [
                 F.AST.TyconRecord
                   ( F.id @@ (RenderId.render name).name,
                     FStarBinder.of_generics e.span generics
                     |> List.map ~f:FStarBinder.implicit_to_explicit
                     |> List.map ~f:FStarBinder.to_binder,
                     None,
                     [],
                     List.map
                       ~f:(fun (prev, (field, ty, attrs)) ->
                         let fname : string = (RenderId.render field).name in
                         let fvars =
                           List.map prev ~f:(fun (field, _, _) ->
                               (RenderId.render field).name)
                         in
                         ( F.id fname,
                           None,
                           [],
                           pmaybe_refined_ty e.span fvars attrs fname ty ))
                       (inits arguments) );
               ] )
    | Type { name; generics; variants; _ } ->
        let self =
          F.mk_e_app
            (F.term_of_lid [ (RenderId.render name).name ])
            (List.map
               ~f:FStarBinder.(of_generic_param e.span >> to_ident)
               generics.params
            |> List.map ~f:(fun id -> F.term @@ F.AST.Name (F.lid_of_id id)))
        in

        let constructors =
          List.map
            ~f:(fun { name; arguments; is_record; _ } ->
              ( F.id (RenderId.render name).name,
                Some
                  (let field_indexes =
                     List.map ~f:(fst3 >> index_of_field_concrete) arguments
                   in
                   if is_record then
                     F.AST.VpRecord
                       ( List.map
                           ~f:(fun (field, ty, attrs) ->
                             let fname : string =
                               (RenderId.render field).name
                             in
                             (F.id fname, None, [], pty e.span ty))
                           arguments,
                         Some self )
                   else
                     F.AST.VpArbitrary
                       (F.term
                       @@ F.AST.Product
                            ( List.map
                                ~f:(fun (_, ty, _) ->
                                  F.mk_e_binder @@ F.AST.NoName (pty e.span ty))
                                arguments,
                              self ))),
                [] ))
            variants
        in
        F.decls
        @@ F.AST.Tycon
             ( false,
               false,
               [
                 F.AST.TyconVariant
                   ( F.id @@ (RenderId.render name).name,
                     FStarBinder.of_generics e.span generics
                     |> List.map ~f:FStarBinder.implicit_to_explicit
                     |> List.map ~f:FStarBinder.to_binder,
                     None,
                     constructors );
               ] )
    | IMacroInvokation { macro; argument; span } -> (
        let open Hacspeclib_macro_parser in
        let unsupported_macro () =
          Error.raise
          @@ {
               kind = UnsupportedMacro { id = [%show: concrete_ident] macro };
               span = e.span;
             }
        in
        match RenderId.render macro with
        | { path = "hacspec_lib" :: _; name } -> (
            let unwrap r =
              match r with
              | Ok r -> r
              | Error details ->
                  let macro_id = [%show: concrete_ident] macro in
                  Error.raise
                    {
                      kind = ErrorParsingMacroInvocation { macro_id; details };
                      span = e.span;
                    }
            in
            let mk_typ_name name = "t_" ^ String.lowercase name in
            match name with
            | "public_nat_mod" ->
                let o = PublicNatMod.parse argument |> unwrap in
                (F.decls_of_string @@ "unfold type " ^ mk_typ_name o.type_name
               ^ "  = nat_mod 0x" ^ o.modulo_value)
                @ F.decls_of_string @@ "unfold type "
                ^ mk_typ_name o.type_of_canvas
                ^ "  = lseq pub_uint8 "
                ^ string_of_int o.bit_size_of_field
            | "bytes" ->
                let o = Bytes.parse argument |> unwrap in
                F.decls_of_string @@ "unfold type " ^ mk_typ_name o.bytes_name
                ^ "  = lseq uint8 " ^ o.size
            | "public_bytes" ->
                let o = Bytes.parse argument |> unwrap in
                F.decls_of_string @@ "unfold type " ^ mk_typ_name o.bytes_name
                ^ "  = lseq uint8 " ^ o.size
            | "array" ->
                let o = Array.parse argument |> unwrap in
                let typ =
                  match o.typ with
                  | "U32" -> "uint32"
                  | "U16" -> "uint16"
                  | "U8" -> "uint8"
                  | usize -> "uint_size"
                in
                let size = o.size in
                let array_def =
                  F.decls_of_string @@ "unfold type " ^ mk_typ_name o.array_name
                  ^ "  = lseq " ^ typ ^ " " ^ size
                in
                let index_def =
                  match o.index_typ with
                  | Some index ->
                      F.decls_of_string @@ "unfold type "
                      ^ mk_typ_name (o.array_name ^ "_idx")
                      ^ " = nat_mod " ^ size
                  | None -> []
                in
                array_def @ index_def
            | "unsigned_public_integer" ->
                let o = UnsignedPublicInteger.parse argument |> unwrap in
                F.decls_of_string @@ "unfold type " ^ mk_typ_name o.integer_name
                ^ "  = lseq uint8 ("
                ^ (Int.to_string @@ ((o.bits + 7) / 8))
                ^ ")"
            | _ -> unsupported_macro ())
        | _ -> unsupported_macro ())
    | Trait { name; generics; items } ->
        let name_str = (RenderId.render name).name in
        let name_id = F.id @@ name_str in
        let fields =
          List.concat_map
            ~f:(fun i ->
              let name = (RenderId.render i.ti_ident).name in
              let generics = FStarBinder.of_generics i.ti_span i.ti_generics in
              let bds = generics |> List.map ~f:FStarBinder.to_binder in
              let fields =
                match i.ti_v with
                | TIType bounds ->
                    let t = F.type0_term in
                    (* let constraints = *)
                    (*   List.map *)
                    (*     ~f:(fun implements -> *)
                    (*       { typ = TApp { ident = i.ti_ident } }) *)
                    (*     bounds *)
                    (* in *)
                    ( F.id name,
                      None,
                      [ F.term @@ F.AST.Var FStar_Parser_Const.no_method_lid ],
                      t )
                    :: List.map
                         ~f:(fun
                             { goal = { trait; args }; name = impl_ident_name }
                           ->
                           let base =
                             F.term @@ F.AST.Name (pconcrete_ident trait)
                           in
                           let args =
                             List.map ~f:(pgeneric_value e.span) args
                           in
                           ( F.id (name ^ "_" ^ impl_ident_name),
                             (* Dodgy concatenation *)
                             None,
                             [],
                             F.mk_e_app base args ))
                         bounds
                | TIFn ty
                  when Attrs.find_unique_attr i.ti_attrs ~f:(function
                         | TraitMethodNoPrePost -> Some ()
                         | _ -> None)
                       |> Option.is_some ->
                    let weakest =
                      let h kind =
                        Attrs.associated_fns kind i.ti_attrs
                        |> List.hd
                        |> Option.map ~f:(fun attr ->
                            ( attr,
                              [%eq: Attr_payloads.AssocRole.t] kind Requires ))
                      in
                      Option.first_some (h Ensures) (h Requires)
                      |> Option.map
                           ~f:(fun ((generics, params, expr), is_req) ->
                             let dummy_self =
                               List.find generics.params
                                 ~f:[%matches? { kind = GPType _; _ }]
                               |> Option.value_or_thunk ~default:(fun () ->
                                   Error.assertion_failure i.ti_span
                                     ("Expected a first generic of type \
                                       `Self`. Instead generics params are: "
                                     ^ [%show: generic_param list]
                                         generics.params))
                               |> fun x -> x.ident
                             in
                             let self =
                               Local_ident.{ name = "Self"; id = mk_id Typ 0 }
                             in
                             let renamer =
                               let f (id : local_ident) =
                                 if [%eq: string] dummy_self.name id.name then
                                   self
                                 else id
                               in
                               U.Mappers.rename_local_idents f
                             in
                             let generics =
                               renamer#visit_generics () generics
                             in
                             let params =
                               List.map ~f:(renamer#visit_param ()) params
                             in
                             let expr = renamer#visit_expr () expr in
                             (generics, params, expr, is_req))
                    in
                    let ty =
                      let variables =
                        let idents_visitor = U.Reducers.collect_local_idents in
                        idents_visitor#visit_trait_item () i
                        :: (Option.map
                              ~f:(fun (generics, params, expr, _) ->
                                [
                                  idents_visitor#visit_generics () generics;
                                  idents_visitor#visit_expr () expr;
                                ]
                                @ List.map
                                    ~f:(idents_visitor#visit_param ())
                                    params)
                              weakest
                           |> Option.value ~default:[])
                        |> Set.union_list (module Local_ident)
                        |> Set.to_list |> ref
                      in
                      let mk_fresh prefix =
                        let v = U.fresh_local_ident_in !variables prefix in
                        variables := v :: !variables;
                        v
                      in
                      let bindings = ref [] in
                      let f (p : param) =
                        let name =
                          match p.pat.p with
                          | PBinding { var; _ } -> var
                          | _ ->
                              let name = mk_fresh "x" in
                              let ({ span; typ; _ } : pat) = p.pat in
                              let expr = { e = LocalVar name; span; typ } in
                              bindings := (p.pat, expr) :: !bindings;
                              name
                        in
                        FStarBinder.of_named_typ p.pat.span name p.typ
                      in
                      weakest
                      |> Option.map ~f:(fun (generics, binders, expr, is_req) ->
                          (generics, List.map ~f binders, expr, is_req))
                      |> Option.map
                           ~f:(fun (generics, binders, (expr : expr), is_req) ->
                             let result_ident = mk_fresh "pred" in
                             let result_bd =
                               FStarBinder.of_named_fstar_typ expr.span
                                 result_ident F.type0_term
                             in
                             let expr = U.make_lets !bindings expr in
                             let expr = pexpr expr in
                             let result =
                               F.term
                               @@ F.AST.Var
                                    (plocal_ident result_ident |> F.lid_of_id)
                             in
                             let result =
                               F.AST.Refine
                                 ( FStarBinder.to_binder result_bd,
                                   (if is_req then Fn.flip else Fn.id)
                                     F.implies result expr )
                               |> F.term
                             in
                             F.AST.Product
                               ( List.map ~f:FStarBinder.to_binder binders,
                                 result )
                             |> F.term)
                      |> Option.value_or_thunk ~default:(fun _ ->
                          let ty = pty e.span ty in
                          match ty.tm with
                          | F.AST.Product (inputs, _) ->
                              {
                                ty with
                                tm = F.AST.Product (inputs, F.type0_term);
                              }
                          | _ -> F.type0_term)
                    in

                    let ty =
                      F.term
                      @@ F.AST.Product
                           (generics |> List.map ~f:FStarBinder.to_binder, ty)
                    in
                    [ (F.id name, None, [], ty) ]
                | TIFn (TArrow (inputs, output)) ->
                    let inputs =
                      List.mapi ~f:(FStarBinder.of_typ e.span) inputs
                    in
                    let inputs = generics @ inputs in
                    let output = pty e.span output in
                    let ty_pre_post =
                      let inputs =
                        List.map ~f:FStarBinder.to_qualified_term inputs
                      in
                      let add_pre n = n ^ "_pre" in
                      let pre_name_str =
                        (RenderId.render
                           (Concrete_ident.with_suffix `Pre i.ti_ident))
                          .name
                      in
                      let pre =
                        F.mk_app (F.term_of_lid [ pre_name_str ]) inputs
                      in
                      let result = F.term_of_lid [ "result" ] in
                      let add_post n = n ^ "_post" in
                      let post_name_str =
                        (RenderId.render
                           (Concrete_ident.with_suffix `Post i.ti_ident))
                          .name
                      in
                      let post =
                        F.mk_app
                          (F.term_of_lid [ post_name_str ])
                          (inputs @ [ (result, Nothing) ])
                      in
                      let post =
                        F.mk_e_abs
                          [ F.pat @@ F.AST.PatVar (F.id "result", None, []) ]
                          post
                      in
                      F.mk_e_app
                        (F.term_of_lid [ "Prims"; "Pure" ])
                        [ output; pre; post ]
                    in
                    let inputs = List.map ~f:FStarBinder.to_binder inputs in
                    let ty = F.term @@ F.AST.Product (inputs, ty_pre_post) in
                    [ (F.id name, None, [], ty) ]
                | TIFn non_arrow_ty ->
                    let inputs = generics in
                    let output = pty e.span non_arrow_ty in
                    let inputs = List.map ~f:FStarBinder.to_binder inputs in
                    let ty = F.term @@ F.AST.Product (inputs, output) in
                    [ (F.id name, None, [], ty) ]
                | _ -> .
              in
              List.map ~f:Fn.id
                (* ~f:(fun (n, q, a, ty) -> (n, q, a, F.mk_e_app bds ty)) *)
                fields)
            items
        in
        let constraints_fields : FStar_Parser_AST.tycon_record =
          generics.constraints
          |> List.filter_map ~f:(fun c ->
              match c with
              | GCType { goal = bound; name = id } ->
                  let name = "_super_" ^ id in
                  let typ = pgeneric_constraint_type e.span c in
                  Some (F.id name, None, [ F.Attrs.no_method ], typ)
              | GCProjection _ ->
                  (* TODO: Not yet implemented, see https://github.com/pereq-dev/hax/issues/785 *)
                  None
              | _ -> .)
        in
        let fields : FStar_Parser_AST.tycon_record =
          constraints_fields @ fields
        in
        let fields : FStar_Parser_AST.tycon_record =
          if List.is_empty fields then
            let marker_field = "__marker_trait_" ^ name_str in
            [ (F.id marker_field, None, [], pty e.span U.unit_typ) ]
          else fields
        in
        (* Binders are explicit on class definitions *)
        let bds =
          List.map
            ~f:
              FStarBinder.(
                of_generic_param e.span >> implicit_to_explicit >> to_binder)
            generics.params
        in
        let tcdef = F.AST.TyconRecord (name_id, bds, None, [], fields) in
        let d = F.AST.Tycon (false, true, [ tcdef ]) in
        (* This helps f* in type class resolution *)
        let constraints_export =
          constraints_fields
          |> List.map ~f:(fun (super_name, _, _, typ) ->
              let super_name = FStar_Ident.string_of_id super_name in
              let tc_name = FStar_Ident.string_of_id name_id in
              let typ = FStar_Parser_AST.term_to_string typ in
              let binders = FStar_Parser_AST.binders_to_string ") (" bds in
              let tc_instance =
                name_id
                :: FStar_Parser_AST.idents_of_binders bds
                     FStar_Compiler_Range.dummyRange
                |> List.map ~f:FStar_Ident.string_of_id
                |> String.concat ~sep:" "
              in
              `VerbatimIntf
                ( "[@@ FStar.Tactics.Typeclasses.tcinstance]\nlet _ = fun ("
                  ^ binders ^ ") {|i: " ^ tc_instance ^ "|} -> i." ^ super_name,
                  `Newline ))
        in
        `Intf { d; drange = F.dummyRange; quals = []; attrs = [] }
        :: constraints_export
    | Impl
        {
          generics;
          self_ty = _;
          of_trait = trait, generic_args;
          items;
          parent_bounds;
        } ->
        let name = (RenderId.render e.ident).name |> F.id in
        let pat = F.pat @@ F.AST.PatVar (name, None, []) in
        let generics = FStarBinder.of_generics e.span generics in
        let pat =
          F.pat
          @@ F.AST.PatApp (pat, List.map ~f:FStarBinder.to_pattern generics)
        in
        let typ =
          F.mk_e_app
            (F.term @@ F.AST.Name (pconcrete_ident trait))
            (List.map ~f:(pgeneric_value e.span) generic_args)
        in
        let pat = F.pat @@ F.AST.PatAscribed (pat, (typ, None)) in
        let fields =
          List.concat_map
            ~f:(fun { ii_span; ii_generics; ii_v; ii_ident } ->
              let name = (RenderId.render ii_ident).name in
              let ii_generics =
                {
                  ii_generics with
                  constraints =
                    List.filter ~f:[%matches? GCType _] ii_generics.constraints;
                }
              in
              match ii_v with
              | IIFn { body; params } ->
                  let pats =
                    FStarBinder.(
                      of_generics ii_span ii_generics |> List.map ~f:to_pattern)
                    @ List.map
                        ~f:(fun { pat; typ_span; typ } ->
                          let span = Option.value ~default:ii_span typ_span in
                          F.pat
                          @@ F.AST.PatAscribed (ppat pat, (pty span typ, None)))
                        params
                  in
                  [ (F.lid [ name ], F.mk_e_abs pats (pexpr body)) ]
              | IIType { typ; parent_bounds } ->
                  (F.lid [ name ], pty ii_span typ)
                  :: List.map
                       ~f:(fun (_impl_expr, impl_ident) ->
                         (F.lid [ name ^ "_" ^ impl_ident.name ], F.tc_solve))
                       parent_bounds)
            items
        in
        let parent_bounds_fields =
          List.map
            ~f:(fun (_impl_expr, impl_ident) ->
              (F.lid [ "_super_" ^ impl_ident.name ], F.tc_solve))
            parent_bounds
        in
        let fields = parent_bounds_fields @ fields in
        let fields =
          if List.is_empty fields then
            [
              ( F.lid
                  [
                    "__marker_trait_"
                    ^ List.last_exn
                        (FStar_Ident.path_of_lid (pconcrete_ident trait));
                  ],
                pexpr (U.unit_expr e.span) );
            ]
          else fields
        in
        let body = F.term @@ F.AST.Record (None, fields) in
        let tcinst = F.term @@ F.AST.Var FStar_Parser_Const.tcinstance_lid in
        let has_type =
          List.exists items ~f:(fun { ii_v; _ } ->
              match ii_v with IIType _ -> true | _ -> false)
        in
        let let_impl = F.AST.TopLevelLet (NoLetQualifier, [ (pat, body) ]) in
        let generics_binders = List.map ~f:FStarBinder.to_binder generics in
        let val_type = F.term @@ F.AST.Product (generics_binders, typ) in
        let v = F.AST.Val (name, val_type) in
        let intf = F.decls ~fsti:true ~attrs:[ tcinst ] v in
        let impl =
          if is_erased then erased_impl name val_type [ tcinst ] generics
          else
            F.decls
              ~fsti:(ctx.interface_mode && has_type)
              ~attrs:[ tcinst ] let_impl
        in
        let is_auto_clone =
          List.exists
            ~f:(function
              | { kind = Tool { path = "automatically_derived"; _ }; _ } -> true
              | _ -> false)
            e.attrs
          && Concrete_ident.eq_name Core__clone__Clone trait
        in
        let intf = if has_type && not is_erased then [] else intf in
        if is_erased && is_auto_clone then
          F.decls ~fsti:ctx.interface_mode
            (F.AST.TopLevelLet (NoLetQualifier, [ (pat, dummy_clone_impl) ]))
        else if ctx.interface_mode then intf @ impl
        else impl
    | Quote { quote; _ } ->
        let fstar_opts =
          Attrs.find_unique_attr e.attrs ~f:(function
            | ItemQuote q -> Some q.fstar_options
            | _ -> None)
          |> Option.value_or_thunk ~default:(fun _ ->
              Error.assertion_failure e.span
                "Malformed `Quote` item: could not find a ItemQuote payload")
          |> Option.value ~default:Types.{ intf = false; impl = true }
        in
        let payload = (pquote e.span quote, `Newline) in
        if ctx.interface_mode then
          (if fstar_opts.intf then [ `VerbatimIntf payload ] else [])
          @ if fstar_opts.impl then [ `VerbatimImpl payload ] else []
        else [ `VerbatimImpl payload ]
    | HaxError details ->
        [
          `Comment
            ("item error backend: " ^ details ^ "\n\nLast AST:\n"
            ^ (U.LiftToFullAst.item e |> Print_rust.pitem_str));
        ]
    | Use _ (* TODO: Not Yet Implemented *) | NotImplementedYet -> []
    | _ -> .
end

module type S = sig
  val decl_to_string : F.AST.decl -> string

  val pitem :
    item ->
    [> `Impl of F.AST.decl
    | `Intf of F.AST.decl
    | `VerbatimImpl of string * [ `NoNewline | `Newline ]
    | `VerbatimIntf of string * [ `NoNewline | `Newline ]
    | `Comment of string ]
    list
end

let make (module M : Attrs.WITH_ITEMS) ctx =
  (module Make
            (M)
            (struct
              let ctx = ctx
            end) : S)

let strings_of_item (bo : BackendOptions.t) m items (item : item) :
    ([> `Impl of string | `Intf of string ] * [ `NoNewline | `Newline ]) list =
  let interface_mode' : Types.inclusion_kind =
    List.rev bo.interfaces
    |> List.find ~f:(fun (clause : Types.inclusion_clause) ->
        let namespace = clause.namespace in
        (* match anything under that **module** namespace *)
        let namespace =
          { namespace with chunks = namespace.chunks @ [ Glob One; Glob Many ] }
        in
        Concrete_ident.matches_namespace namespace item.ident)
    |> Option.map ~f:(fun (clause : Types.inclusion_clause) -> clause.kind)
    |> Option.value ~default:(Types.Excluded : Types.inclusion_kind)
  in
  let interface_mode =
    not ([%matches? (Types.Excluded : Types.inclusion_kind)] interface_mode')
  in
  let (module Print) =
    make m
      {
        current_namespace = (RenderId.render item.ident).path;
        interface_mode;
        items;
        line_width = bo.line_width;
      }
  in
  let mk_impl i = `Impl i in
  let mk_intf = if interface_mode then fun i -> `Intf i else fun i -> `Impl i in
  let no_impl =
    [%matches? (Types.Included None' : Types.inclusion_kind)] interface_mode'
  in
  Print.pitem item
  |> List.concat_map ~f:(function
    | `Impl i -> [ (mk_impl (Print.decl_to_string i), `Newline) ]
    | `Intf i -> [ (mk_intf (Print.decl_to_string i), `Newline) ]
    | `VerbatimIntf (s, nl) -> [ (mk_intf s, nl) ]
    | `VerbatimImpl (s, nl) -> [ (`Impl s, nl) ]
    | `Comment s ->
        let s = "(* " ^ s ^ " *)" in
        if interface_mode then [ (`Impl s, `Newline); (`Intf s, `Newline) ]
        else [ (`Impl s, `Newline) ])
  |> List.filter ~f:(function `Impl _, _ when no_impl -> false | _ -> true)

type rec_prefix = NonRec | FirstMutRec | MutRec

let string_of_items ~mod_name ~bundles (bo : BackendOptions.t) m items :
    string * string =
  let collect_trait_goal_idents =
    object
      inherit [_] Visitors.reduce as super
      inherit [_] U.Sets.Concrete_ident.monoid as _m

      method! visit_trait_goal (_env : unit) x =
        Set.singleton (module Concrete_ident) x.trait
    end
  in
  let header =
    let lines =
      List.map ~f:(collect_trait_goal_idents#visit_item ()) items
      |> Set.union_list (module Concrete_ident)
      |> Set.map
           (module String)
           ~f:(fun i -> (RenderId.render i).path |> module_name)
      |> Fn.flip Set.remove mod_name
      |> Set.to_list
      |> List.filter ~f:(fun m ->
          (* Special treatment for modules handled specifically in our F* libraries *)
          String.is_prefix ~prefix:"Core_models." m |> not
          && String.is_prefix ~prefix:"Alloc." m |> not
          && String.equal "Hax_lib.Int" m |> not)
      |> List.map ~f:(fun mod_path -> "let open " ^ mod_path ^ " in")
    in
    match lines with
    | [] -> ""
    | _ ->
        "let _ ="
        ^ ([
             "(* This module has implicit dependencies, here we make them \
              explicit. *)";
             "(* The implicit dependencies arise from typeclasses instances. *)";
           ]
           @ lines @ [ "()" ]
          |> List.map ~f:(( ^ ) "\n  ")
          |> String.concat ~sep:"")
        ^ "\n\n"
  in
  let map_string ~f ?(map_intf = true) (str, space) =
    ( (match str with
      | `Impl s -> `Impl (f s)
      | `Intf s -> `Intf (if map_intf then f s else s)),
      space )
  in
  let replace_in_strs ~pattern ~with_ =
    List.map
      ~f:
        (map_string ~map_intf:false ~f:(fun str ->
             String.substr_replace_first ~pattern ~with_ str))
  in

  (* Each of these bundles contains recursive items (mutually if the bundle has more than one element).
     We know that these items will already be grouped together but we need to add the `rec` qualifier
     to the first one (in the case of functions). And to replace the `let`/`type` keyword by `and`
     for the other elements coming after. *)
  let first_in_bundles = Array.create (List.length bundles) None in
  let get_recursivity_prefix it =
    match
      List.findi bundles ~f:(fun _ bundle ->
          List.mem bundle it ~equal:[%eq: item])
    with
    | Some (i, _) -> (
        match first_in_bundles.(i) with
        | Some first_it when [%eq: item] first_it it -> FirstMutRec
        | Some _ -> MutRec
        | None ->
            first_in_bundles.(i) <- Some it;
            FirstMutRec)
    | None -> NonRec
  in
  let strings its =
    List.concat_map
      ~f:(fun item ->
        let recursivity_prefix = get_recursivity_prefix item in
        let strs = strings_of_item bo m items item in
        match (recursivity_prefix, item.v) with
        | FirstMutRec, Fn _ ->
            replace_in_strs ~pattern:"let" ~with_:"let rec" strs
        | MutRec, Fn _ -> replace_in_strs ~pattern:"let" ~with_:"and" strs
        | MutRec, Type _ -> replace_in_strs ~pattern:"type" ~with_:"and" strs
        | _ -> strs)
      its
    |> List.map ~f:(map_string ~f:String.strip)
    |> List.filter
         ~f:(fst >> ( function `Impl s | `Intf s -> String.is_empty s ) >> not)
  in
  let string_for filter =
    let l =
      List.filter_map
        ~f:(fun (s, space) ->
          let* s = filter s in
          Some (s, space))
        (strings items)
    in
    let n = List.length l - 1 in
    let lines =
      List.mapi
        ~f:(fun i (s, space) ->
          s
          ^ if [%matches? `NoNewline] space || [%eq: int] i n then "" else "\n")
        l
    in
    match lines with [] -> "" | _ -> header ^ String.concat ~sep:"\n" lines
  in
  let replace =
    String.substr_replace_all ~pattern:"_hax_panic_freedom_admit_"
      ~with_:"admit () (* Panic freedom *)"
  in
  ( string_for (function `Impl s -> Some (replace s) | _ -> None),
    string_for (function `Intf s -> Some (replace s) | _ -> None) )

let fstar_headers (bo : BackendOptions.t) (mod_name : string) =
  let opts =
    Printf.sprintf {|#set-options "--fuel %Ld --ifuel %Ld --z3rlimit %Ld"|}
      bo.fuel bo.ifuel bo.z3rlimit
  in

  List.append [ opts; "open FStar.Mul" ]
    (if hax_core_models_extraction then [ "open Rust_primitives" ]
     else [ "open Core_models" ])
  |> String.concat ~sep:"\n"

(** Rewrites `unsize x` to `x <: τ` when `τ` is in the allowlist described by
    `unsize_identity_typ` *)
let unsize_as_identity =
  (* Tells if a unsize should be treated as identity by type *)
  let rec unsize_identity_typ = function
    | TArray _ -> true
    | TRef { typ; _ } -> unsize_identity_typ typ
    | _ -> false
  in
  let visitor =
    object
      inherit [_] U.Visitors.map as super

      method! visit_expr () e =
        match e.e with
        | App { f = { e = GlobalVar f; _ }; args = [ x ]; _ }
          when Global_ident.eq_name Rust_primitives__unsize f
               && unsize_identity_typ x.typ ->
            let x = super#visit_expr () x in
            { e with e = Ascription { e = x; typ = e.typ } }
        | _ -> super#visit_expr () e
    end
  in
  visitor#visit_item ()

(** Translate as F* (the "legacy" printer) *)
let translate_as_fstar m (bo : BackendOptions.t) ~(bundles : AST.item list list)
    (items : AST.item list) : Types.file list =
  U.group_items_by_namespace items
  |> Map.to_alist
  |> List.filter_map ~f:(fun (_, items) ->
      let* first_item = List.hd items in
      Some ((RenderId.render first_item.ident).path, items))
  |> List.concat_map ~f:(fun (ns, items) ->
      let mod_name = module_name ns in
      let impl, intf = string_of_items ~mod_name ~bundles bo m items in
      let make ~ext body =
        if String.is_empty body then None
        else
          Some
            Types.
              {
                path = mod_name ^ "." ^ ext;
                contents =
                  "module " ^ mod_name ^ "\n" ^ fstar_headers bo mod_name
                  ^ "\n\n" ^ body ^ "\n";
                sourcemap = None;
              }
      in
      List.filter_map ~f:Fn.id [ make ~ext:"fst" impl; make ~ext:"fsti" intf ])

let translate =
  if
    Sys.getenv "HAX_ENGINE_EXPERIMENTAL_RUST_PRINTER_INSTEAD_OF_FSTAR"
    |> Option.is_some
  then failwith "todo"
  else translate_as_fstar

open Phase_utils
module DepGraphR = Dependencies.Make (Features.Rust)

module TransformToInputLanguage =
  [%functor_application
    Phases.Reject.RawOrMutPointer(Features.Rust)
  |> Phases.Reject_impl_type_method
  |> Phases.Rewrite_local_self
  |> Phases.Transform_hax_lib_inline
  |> Phases.Specialize
  |> Phases.Drop_sized_trait
  |> Phases.Simplify_question_marks
  |> Phases.And_mut_defsite
  |> Phases.Reconstruct_asserts
  |> Phases.Reconstruct_for_loops
  |> Phases.Reconstruct_while_loops
  |> Phases.Direct_and_mut
  |> Phases.Reject.Arbitrary_lhs
  |> Phases.Drop_blocks
  |> Phases.Drop_match_guards
  |> Phases.Drop_references
  |> Phases.Explicit_conversions
  |> Phases.Trivialize_assign_lhs
  |> Side_effect_utils.Hoist
  |> Phases.Hoist_disjunctive_patterns
  |> Phases.Simplify_match_return
  |> Phases.Local_mutation
  |> Phases.Rewrite_control_flow
  |> Phases.Drop_return_break_continue
  |> Phases.Functionalize_loops
  |> Phases.Reject.Question_mark
  |> Phases.Reject.As_pattern
  |> Phases.Traits_specs
  |> Phases.Simplify_hoisting
  |> Phases.Newtype_as_refinement
  |> Phases.Reject.Trait_item_default
  |> Phases.Bundle_cycles
  |> Phases.Reorder_fields
  |> Phases.Sort_items_namespace_wise
  |> SubtypeToInputLanguage
  |> Identity
  ]
  [@ocamlformat "disable"]

let post_process_items =
  List.map ~f:unsize_as_identity
  >> List.map ~f:unsize_as_identity
  >> List.map ~f:U.Mappers.add_typ_ascription

let apply_phases (bo : BackendOptions.t) (items : Ast.Rust.item list) :
    AST.item list =
  let items =
    (* let hax_core_extraction = *)
    (*   Sys.getenv "HAX_CORE_EXTRACTION_MODE" *)
    (*   |> [%equal: string option] (Some "on") *)
    (* in *)
    (* if hax_core_extraction then *)
    (*   let names = *)
    (*     Core_names.names |> List.map ~f:(Concrete_ident.of_def_id Value) *)
    (*   in *)
    (*   DepGraphR.ItemGraph.transitive_dependencies_of_items names items *)
    (* else *)
    items
  in
  let items = TransformToInputLanguage.ditems items |> post_process_items in
  items
