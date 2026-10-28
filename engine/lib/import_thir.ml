module Thir = struct
  include Types

  type item = item_for__thir_body
  type item_kind = item_kind_for__thir_body
  type impl_item = impl_item_for__thir_body
  type impl_item_kind = impl_item_kind_for__thir_body
  type generics = generics_for__thir_body
  type trait_item_kind = trait_item_kind_for__thir_body
  type generic_param = generic_param_for__thir_body
  type generic_param_kind = generic_param_kind_for__thir_body
  type trait_item = trait_item_for__thir_body
  type ty = node_for__ty_kind
  type item_ref = node_for__item_ref_contents
  type trait_ref = item_ref
end

open! Prelude
open Diagnostics

let assertion_failure (span : Thir.span list) (details : string) =
  let kind = T.AssertionFailure { details } in
  Diagnostics.SpanFreeError.raise ~span
    (Span.dummy () |> Span.owner_hint)
    ThirImport kind

let unimplemented ~issue_id (span : Thir.span list) (details : string) =
  let kind =
    T.Unimplemented
      {
        issue_id = Some (MyInt64.of_int issue_id);
        details = String.(if details = "" then None else Some details);
      }
  in
  Diagnostics.SpanFreeError.raise ~span
    (Span.dummy () |> Span.owner_hint)
    ThirImport kind

module Ast = struct
  include Ast
  include Rust
end

module U = Ast_utils.Make (Features.Rust)
module W = Features.On
module Ast_builder = Ast_builder.Make (Features.Rust)
open Ast

let def_id ~value (def_id : Thir.def_id) : global_ident =
  `Concrete (Concrete_ident.of_def_id ~value def_id)

let local_ident kind (ident : Thir.local_ident) : local_ident =
  {
    name = ident.name;
    id = Local_ident.mk_id kind (Int.of_string ident.id.local_id);
  }

let int_ty_to_size : Thir.int_ty -> size = function
  | Isize -> SSize
  | I8 -> S8
  | I16 -> S16
  | I32 -> S32
  | I64 -> S64
  | I128 -> S128

let uint_ty_to_size : Thir.uint_ty -> size = function
  | Usize -> SSize
  | U8 -> S8
  | U16 -> S16
  | U32 -> S32
  | U64 -> S64
  | U128 -> S128

let c_int_ty (ty : Thir.int_ty) : int_kind =
  { size = int_ty_to_size ty; signedness = Signed }

let c_uint_ty (ty : Thir.uint_ty) : int_kind =
  { size = uint_ty_to_size ty; signedness = Unsigned }

let csafety (safety : Types.safety) : safety_kind =
  match safety with Safe -> Safe | Unsafe -> Unsafe W.unsafe

let c_header_safety (safety : Types.header_safety) : safety_kind =
  match safety with
  | SafeTargetFeatures -> Safe
  | Normal safety -> csafety safety

let c_mutability (witness : 'a) : bool -> 'a Ast.mutability = function
  | true -> Mutable witness
  | false -> Immutable

let c_borrow_kind span : Thir.borrow_kind -> borrow_kind = function
  | Shared -> Shared
  | Fake _ ->
      assertion_failure [ span ]
        "Got a shallow borrow node (`BorrowKind::Fake`). Those are generated \
         by the borrow checker and should be discarded after borrow checking: \
         we should never see such borrows."
  | Mut _ -> Mut W.mutable_reference

let c_binding_mode : Thir.by_ref -> binding_mode = function
  | No -> ByValue
  | Yes (_, true) -> ByRef (Mut W.mutable_reference, W.reference)
  | Yes (_, false) -> ByRef (Shared, W.reference)

let unit_typ : ty = TApp { ident = `TupleType 0; args = [] }

let unit_expr span : expr =
  { typ = unit_typ; span; e = Ast.GlobalVar (`TupleCons 0) }

let wild_pat span : ty -> pat = fun typ -> { typ; span; p = PWild }

let c_logical_op : Thir.logical_op -> logical_op = function
  | And -> And
  | Or -> Or

(** The markers rustc leaves behind when it expands [#[cfg(..)]] and
    [#[cfg_attr(..)]] attributes. They exist only so that rustc can report
    better spans in its own diagnostics: they carry no information for us. Note
    their paths are purposefully invalid Rust identifiers, so that users cannot
    write them: keeping them around makes [Print_rust] produce Rust code that
    cannot be parsed back. *)
let cfg_trace_attributes = [ "<cfg_trace>"; "<cfg_attr_trace>" ]

let c_attr (attr : Thir.attribute) : attr option =
  match attr with
  | Unparsed { path; _ }
    when List.mem cfg_trace_attributes path ~equal:[%eq: string] ->
      None
  | Parsed (DocComment { kind; comment; span; _ }) ->
      let kind =
        match kind with Thir.Line -> DCKLine | Thir.Block -> DCKBlock
      in
      let kind = DocComment { kind; body = comment } in
      Some { kind; span = Span.of_thir span }
  | Parsed (AutomaticallyDerived span) ->
      (* Restore behavior before PR #1534 *)
      let kind = Tool { path = "automatically_derived"; tokens = "" } in
      Some { kind; span = Span.of_thir span }
  | Unparsed { args = Eq { expr = { symbol; _ }; _ }; path = "doc"; span; _ } ->
      (* Looks for `#[doc = "something"]` *)
      let kind = DocComment { kind = DCKLine; body = symbol } in
      Some { kind; span = Span.of_thir span }
  | Unparsed { args; path; span; _ } ->
      let args_tokens =
        match args with Delimited { tokens; _ } -> Some tokens | _ -> None
      in
      let tokens = Option.value ~default:"" args_tokens in
      let kind = Tool { path; tokens } in
      Some { kind; span = Span.of_thir span }
  | _ -> None

let c_attrs : Thir.attribute list -> attrs = List.filter_map ~f:c_attr

let c_item_attrs (attrs : Thir.item_attributes) : attrs =
  (* TODO: This is a quite coarse approximation, we need to reflect
     that parent/self structure in our AST. See
     https://github.com/pereq-dev/hax/issues/123. *)
  let self = c_attrs attrs.attributes in
  let parent =
    c_attrs attrs.parent_attributes
    |> List.filter ~f:([%matches? ({ kind = DocComment _; _ } : attr)] >> not)
    |>
    (* Repeating associateditem or uid is harmful, same for comments *)
    List.filter ~f:(fun payload ->
        match Attr_payloads.payloads [ payload ] with
        | [ ((Uid _ | AssociatedItem _), _) ] -> false
        | _ -> true)
  in
  self @ parent

type extended_literal =
  | EL_Lit of literal
  | EL_U8Array of literal list (* EL_U8Array only encodes arrays of [u8]s *)

let c_lit' span negative (lit : Thir.lit_kind) (ty : ty) : extended_literal =
  let mk l = EL_Lit l in
  let mku8 (n : int) =
    let kind = { size = S8; signedness = Unsigned } in
    Int { value = Int.to_string n; kind; negative = false }
  in
  let error kind =
    assertion_failure [ span ]
      ("[import_thir:literal] got a " ^ kind ^ " literal, expected " ^ kind
     ^ " type, got type ["
      ^ [%show: ty] ty
      ^ "] instead.")
  in
  match lit with
  | Err _ ->
      assertion_failure [ span ]
        "[import_thir:literal] got an error literal: this means the Rust \
         compiler or Hax's frontend probably reported errors above."
  | Str (str, _) -> mk @@ String str
  | CStr (l, _) | ByteStr (l, _) -> EL_U8Array (List.map ~f:mku8 l)
  | Byte n -> mk @@ mku8 n
  | Char s -> mk @@ Char s
  | Int (value, _kind) ->
      mk
      @@ Int
           {
             value;
             negative;
             kind = (match ty with TInt k -> k | _ -> error "integer");
           }
  | Float (value, _kind) ->
      mk
      @@ Float
           {
             value;
             negative;
             kind = (match ty with TFloat k -> k | _ -> error "float");
           }
  | Bool b -> mk @@ Bool b

let c_lit span neg (lit : Thir.spanned_for__lit_kind) : ty -> extended_literal =
  c_lit' span neg lit.node

let resugar_index_mut (e : expr) : (expr * expr) option =
  match (U.unbox_underef_expr e).e with
  | App
      {
        f = { e = GlobalVar (`Concrete meth); _ };
        args = [ { e = Borrow { e = x; _ }; _ }; index ];
        generic_args = _ (* TODO: see issue #328 *);
        trait = _ (* TODO: see issue #328 *);
        bounds_impls = _;
      }
    when Concrete_ident.eq_name Core__ops__index__IndexMut__index_mut meth ->
      Some (x, index)
  | App
      {
        f = { e = GlobalVar (`Concrete meth); _ };
        args = [ x; index ];
        generic_args = _ (* TODO: see issue #328 *);
        trait = _ (* TODO: see issue #328 *);
        bounds_impls = _;
      }
    when Concrete_ident.eq_name Core__ops__index__Index__index meth ->
      Some (x, index)
  | _ -> None

(** Name for the cast function from an ADT to its discriminant *)
let cast_name_for_type = Concrete_ident.with_suffix `Cast

module type EXPR = sig
  val c_expr : Thir.decorated_for__expr_kind -> expr
  val c_expr_drop_body : Thir.decorated_for__expr_kind -> expr
  val c_ty : Thir.span -> Thir.ty -> ty
  val c_generic_value : Thir.span -> Thir.generic_arg -> generic_value
  val c_generics : ?offset:int -> Thir.generics -> generics
  val c_param : Thir.span -> Thir.param -> param
  val c_fn_params : Thir.span -> Thir.param list -> param list
  val c_trait_item' : Thir.trait_item -> Thir.trait_item_kind -> trait_item'
  val c_trait_ref : Thir.span -> Thir.trait_ref -> trait_goal
  val c_impl_expr : Thir.span -> Thir.impl_expr -> impl_expr
  val c_clause : Thir.span -> int -> Thir.clause -> generic_constraint option

  val c_parent_bounds :
    (Thir.clause * Thir.impl_expr * Thir.span) list ->
    (impl_expr * impl_ident) list
end

(* BinOp to [core::ops::*] overloaded functions *)

module Make (CTX : sig
  val is_core_item : bool
end) : EXPR = struct
  let c_binop (op : Thir.bin_op) (lhs : expr) (rhs : expr) (span : span)
      (typ : ty) =
    let overloaded_names_of_binop : Thir.bin_op -> Concrete_ident.name =
      function
      | Add | AddUnchecked -> Core__ops__arith__Add__add
      | Sub | SubUnchecked -> Core__ops__arith__Sub__sub
      | Mul | MulUnchecked -> Core__ops__arith__Mul__mul
      | Div -> Core__ops__arith__Div__div
      | Rem -> Core__ops__arith__Rem__rem
      | BitXor -> Core__ops__bit__BitXor__bitxor
      | BitAnd -> Core__ops__bit__BitAnd__bitand
      | BitOr -> Core__ops__bit__BitOr__bitor
      | Shl | ShlUnchecked -> Core__ops__bit__Shl__shl
      | Shr | ShrUnchecked -> Core__ops__bit__Shr__shr
      | Lt -> Core__cmp__PartialOrd__lt
      | Le -> Core__cmp__PartialOrd__le
      | Ne -> Core__cmp__PartialEq__ne
      | Ge -> Core__cmp__PartialOrd__ge
      | Gt -> Core__cmp__PartialOrd__gt
      | Eq -> Core__cmp__PartialEq__eq
      | AddWithOverflow | SubWithOverflow | MulWithOverflow ->
          assertion_failure (Span.to_thir span)
            "Overflowing binary operators are not suppored"
      | Cmp ->
          assertion_failure (Span.to_thir span)
            "`Cmp` binary operator is not suppored"
      | Offset -> Core__ptr__const_ptr__Impl__offset
    in
    let primitive_names_of_binop : Thir.bin_op -> Concrete_ident.name = function
      | Add | AddUnchecked -> Rust_primitives__u128__add
      | Sub | SubUnchecked -> Rust_primitives__u128__sub
      | Mul | MulUnchecked -> Rust_primitives__u128__mul
      | Div -> Rust_primitives__u128__div
      | Rem -> Rust_primitives__u128__rem
      | BitXor -> Rust_primitives__u128__bit_xor
      | BitAnd -> Rust_primitives__u128__bit_and
      | BitOr -> Rust_primitives__u128__bit_or
      | Shl | ShlUnchecked -> Rust_primitives__u128__shl
      | Shr | ShrUnchecked -> Rust_primitives__u128__shr
      | Lt -> Rust_primitives__u128__lt
      | Le -> Rust_primitives__u128__le
      | Ne -> Rust_primitives__u128__ne
      | Ge -> Rust_primitives__u128__ge
      | Gt -> Rust_primitives__u128__gt
      | Eq -> Rust_primitives__u128__eq
      | AddWithOverflow | SubWithOverflow | MulWithOverflow ->
          assertion_failure (Span.to_thir span)
            "Overflowing binary operators are not suppored"
      | Cmp ->
          assertion_failure (Span.to_thir span)
            "`Cmp` binary operator is not suppored"
      | Offset -> Rust_primitives__offset
    in
    let name =
      if CTX.is_core_item then
        let assert_type_eq t1 t2 =
          if not (U.ty_equality t1 t2) then
            assertion_failure (Span.to_thir span)
              ("Binary operation: expected LHS and RHS to have the same type, \
                instead LHS has type ["
              ^ [%show: ty] t1
              ^ "] while RHS has type ["
              ^ [%show: ty] t2
              ^ "]")
        in
        let int =
          ("int", function TInt k -> Some (show_int_kind k) | _ -> None)
        in
        let float =
          ("float", function TFloat k -> Some (show_float_kind k) | _ -> None)
        in
        let bool = ("bool", function TBool -> Some "bool" | _ -> None) in
        let concat_tup sep (x, y) = x ^ sep ^ y in
        let ( <*> ) (x, f) (y, g) =
          ( x ^ "*" ^ y,
            f *** g >> uncurry Option.both >> Option.map ~f:(concat_tup "_") )
        in
        let both (e, f) =
          ( e ^ "*" ^ e,
            fun (t1, t2) ->
              assert_type_eq t1 t2;
              f t1 )
        in
        let ( <|> ) (x, f) (y, g) =
          (x ^ " or" ^ y, fun v -> match f v with None -> g v | v -> v)
        in
        let name = primitive_names_of_binop op in
        let expected, f =
          match op with
          | Add | Sub | Mul | AddWithOverflow | SubWithOverflow
          | MulWithOverflow | AddUnchecked | SubUnchecked | MulUnchecked | Div
            ->
              both int <|> both float
          | Rem | Cmp -> both int
          | BitXor | BitAnd | BitOr -> both int <|> both bool
          | Shl | Shr | ShlUnchecked | ShrUnchecked -> int <*> int
          | Lt | Le | Ne | Ge | Gt -> both int <|> both float
          | Eq -> both int <|> both float <|> both bool
          | Offset -> ("", fun _ -> Some "")
        in
        match f (lhs.typ, rhs.typ) with
        | Some with_ ->
            Concrete_ident.of_name ~value:true name
            |> (Concrete_ident.map_path_strings [@alert "-unsafe"]) ~f:(function
              | "u128" -> with_
              | s -> s)
        | None ->
            assertion_failure (Span.to_thir span)
              ("Binary operation: expected " ^ expected ^ " type, got "
              ^ [%show: ty] lhs.typ)
      else Concrete_ident.of_name ~value:true @@ overloaded_names_of_binop op
    in
    let needs_borrow =
      match op with Lt | Le | Ne | Ge | Gt | Eq -> true | _ -> false
    in
    let borrow_if_needed (e : expr) =
      if needs_borrow then
        match e.typ with
        | TRef _ -> e
        | _ ->
            {
              span = e.span;
              e = Borrow { e; kind = Shared; witness = W.reference };
              typ =
                TRef
                  {
                    witness = W.reference;
                    region = "unknown";
                    typ = e.typ;
                    mut = Immutable;
                  };
            }
      else e
    in
    let lhs = borrow_if_needed lhs in
    let rhs = borrow_if_needed rhs in
    U.call' (`Concrete name) [ lhs; rhs ] span typ

  let binop_of_assignop : Thir.assign_op -> Thir.bin_op = function
    | AddAssign -> Add
    | SubAssign -> Sub
    | MulAssign -> Mul
    | DivAssign -> Div
    | RemAssign -> Rem
    | BitXorAssign -> BitXor
    | BitAndAssign -> BitAnd
    | BitOrAssign -> BitOr
    | ShlAssign -> Shl
    | ShrAssign -> Shr

  let rec c_expr (e : Thir.decorated_for__expr_kind) : expr =
    try c_expr_unwrapped e
    with Diagnostics.SpanFreeError.Exn (Data (ctx, kind)) ->
      let typ : ty =
        try c_ty e.span e.ty
        with Diagnostics.SpanFreeError.Exn _ -> U.HaxFailure.Build.ty ""
      in
      let span = Span.of_thir e.span in
      U.hax_failure_expr' span typ (ctx, kind) ""

  (** Extracts an expression as the global name `dropped_body`: this drops the
      computational part of the expression, but keeps a correct type and span.
  *)
  and c_expr_drop_body (e : Thir.decorated_for__expr_kind) : expr =
    let typ = c_ty e.span e.ty in
    let span = Span.of_thir e.span in
    let v =
      Global_ident.of_name ~value:true Rust_primitives__hax__dropped_body
    in
    { span; typ; e = GlobalVar v }

  and c_block ~expr ~span ~stmts ~ty ~(safety_mode : Types.block_safety) : expr
      =
    let full_span = Span.of_thir span in
    let typ = c_ty span ty in
    let safety_mode =
      match safety_mode with
      | Safe -> Safe
      | BuiltinUnsafe | ExplicitUnsafe -> Unsafe W.unsafe
    in
    (* if there is no expression & the last expression is ⊥, just use that *)
    let lift_last_statement_as_expr_if_possible expr stmts (ty : Thir.ty) =
      match (ty.value, expr, List.drop_last stmts, List.last stmts) with
      | ( Thir.Never,
          None,
          Some stmts,
          Some ({ kind = Thir.Expr { expr; _ }; _ } : Thir.stmt) ) ->
          (stmts, Some expr)
      | _ -> (stmts, expr)
    in
    let o_stmts, o_expr =
      lift_last_statement_as_expr_if_possible expr stmts ty
    in
    let init =
      Option.map
        ~f:(fun e ->
          let e = c_expr e in
          { e with e = Block { e; safety_mode; witness = W.block } })
        o_expr
      |> Option.value ~default:(unit_expr full_span)
    in
    List.fold_right o_stmts ~init ~f:(fun { kind; _ } body ->
        match kind with
        | Expr { expr = rhs; _ } ->
            let rhs = c_expr rhs in
            let e =
              Let { monadic = None; lhs = wild_pat rhs.span rhs.typ; rhs; body }
            in
            { e; typ; span = Span.union rhs.span body.span }
        | Let
            {
              else_block = Some { expr; span; stmts; safety_mode; _ };
              pattern = lhs;
              initializer' = Some rhs;
              _;
            } ->
            let lhs = c_pat lhs in
            let rhs = c_expr rhs in
            let else_block = c_block ~expr ~span ~stmts ~ty ~safety_mode in
            let lhs_body_span = Span.union lhs.span body.span in
            let e =
              Match
                {
                  arms =
                    [
                      U.M.arm lhs body ~span:lhs_body_span;
                      U.M.arm
                        { p = PWild; span = else_block.span; typ = lhs.typ }
                        { else_block with typ = body.typ }
                        ~span:else_block.span;
                    ];
                  scrutinee = rhs;
                }
            in
            { e; typ; span = full_span }
        | Let { initializer' = None; _ } ->
            unimplemented ~issue_id:156 [ span ]
              "Sorry, Hax does not support declare-first let bindings (see \
               https://doc.rust-lang.org/rust-by-example/variable_bindings/declare.html) \
               for now."
        | Let { pattern = lhs; initializer' = Some rhs; _ } ->
            let lhs = c_pat lhs in
            let rhs = c_expr rhs in
            let e = Let { monadic = None; lhs; rhs; body } in
            { e; typ; span = Span.union rhs.span body.span })

  and c_expr_unwrapped (e : Thir.decorated_for__expr_kind) : expr =
    (* TODO: eliminate that `call`, use the one from `ast_utils` *)
    let call f args =
      App
        {
          f;
          args = List.map ~f:c_expr args;
          generic_args = [];
          trait = None;
          bounds_impls = [];
        }
    in
    let typ = c_ty e.span e.ty in
    let span = Span.of_thir e.span in
    let mk_global typ v : expr = { span; typ; e = GlobalVar v } in
    let ( ->. ) a b = TArrow (a, b) in
    let (v : expr') =
      match e.contents with
      | If
          {
            cond = { contents = Let { expr = scrutinee; pat }; _ };
            else_opt;
            then';
            _;
          } ->
          let scrutinee = c_expr scrutinee in
          let arm_pat = c_pat pat in
          let then_ = c_expr then' in
          let else_ =
            Option.value ~default:(U.unit_expr span)
            @@ Option.map ~f:c_expr else_opt
          in
          let arm_then = U.M.arm arm_pat then_ ~span:then_.span in
          let arm_else =
            let arm_pat = { arm_pat with p = PWild } in
            U.M.arm arm_pat else_ ~span:else_.span
          in
          Match { scrutinee; arms = [ arm_then; arm_else ] }
      | If { cond; else_opt; then'; _ } ->
          let cond = c_expr cond in
          let then_ = c_expr then' in
          let else_ = Option.map ~f:c_expr else_opt in
          If { cond; else_; then_ }
      | Call { args; fn_span = _; from_hir_call = _; fun'; ty = _ } -> (
          let args =
            if List.is_empty args then [ unit_expr span ]
            else List.map ~f:c_expr args
          in
          let f = c_expr fun' in
          match fun'.contents with
          | GlobalName
              {
                item =
                  {
                    value =
                      { def_id = id; generic_args; impl_exprs; in_trait; _ };
                    _;
                  };
                _;
              } ->
              let f = { f with e = GlobalVar (def_id ~value:true id) } in
              let bounds_impls = List.map ~f:(c_impl_expr e.span) impl_exprs in
              let generic_args =
                List.map ~f:(c_generic_value e.span) generic_args
              in
              let in_trait = Option.map ~f:(c_impl_expr e.span) in_trait in
              let trait =
                Option.map ~f:(fun ie -> (ie, ie.goal.args)) in_trait
              in
              App { f; args; generic_args; bounds_impls; trait }
          | _ ->
              App
                { f; args; generic_args = []; bounds_impls = []; trait = None })
      | Box { value } ->
          (U.call Rust_primitives__hax__box_new [ c_expr value ] span typ).e
      | Deref { arg } ->
          let inner_typ = c_ty arg.span arg.ty in
          call (mk_global ([ inner_typ ] ->. typ) @@ `Primitive Deref) [ arg ]
      | Binary { lhs; rhs; op } ->
          (c_binop op (c_expr lhs) (c_expr rhs) span typ).e
      | LogicalOp { lhs; rhs; op } ->
          let lhs_type = c_ty lhs.span lhs.ty in
          let rhs_type = c_ty rhs.span rhs.ty in
          call
            (mk_global ([ lhs_type; rhs_type ] ->. typ)
            @@ `Primitive (LogicalOp (c_logical_op op)))
            [ lhs; rhs ]
      | Unary { arg; op } ->
          (U.call
             (match op with
             | Not -> Core__ops__bit__Not__not
             | Neg -> Core__ops__arith__Neg__neg
             | PtrMetadata ->
                 assertion_failure (Span.to_thir span)
                   "Unsupported unary operator: `PtrMetadata`")
             [ c_expr arg ]
             span typ)
            .e
      | Cast { source } -> (
          let source_type = c_ty source.span source.ty in
          match source_type with
          (* Each inductive defines a cast function *)
          | TApp { ident = `Concrete ident; _ } ->
              (U.call'
                 (`Concrete (cast_name_for_type ident))
                 [ c_expr source ]
                 span typ)
                .e
          | _ ->
              call
                (mk_global ([ source_type ] ->. typ) @@ `Primitive Cast)
                [ source ])
      | Use { source } -> (c_expr source).e
      | NeverToAny { source } ->
          (U.call Rust_primitives__hax__never_to_any [ c_expr source ] span typ)
            .e
      (* TODO: this is incorrect (NeverToAny) *)
      | PointerCoercion { cast; source } -> c_pointer e typ span cast source
      | Loop { body } ->
          let body = c_expr body in
          Loop
            {
              body;
              kind = UnconditionalLoop;
              state = None;
              label = None;
              witness = W.loop;
              control_flow = None;
            }
      | Match { scrutinee; arms } ->
          let scrutinee = c_expr scrutinee in
          let arms = List.map ~f:c_arm arms in
          Match { scrutinee; arms }
      | Let _ ->
          unimplemented ~issue_id:2018 [ e.span ]
            "Let-chains (e.g. `if let .. && let ..`) are not supported."
      | Block { expr; span; stmts; safety_mode; _ } ->
          let { e; _ } = c_block ~expr ~span ~stmts ~ty:e.ty ~safety_mode in
          e
      | Assign { lhs; rhs } ->
          let lhs = c_expr lhs in
          let rhs = c_expr rhs in
          c_expr_assign lhs rhs
      | AssignOp { lhs; op; rhs } ->
          let lhs = c_expr lhs in
          c_expr_assign lhs
          @@ c_binop (binop_of_assignop op) lhs (c_expr rhs) span lhs.typ
      | VarRef { id } -> LocalVar (local_ident Expr id)
      | Field { lhs; field } ->
          let lhs = c_expr lhs in
          let projector =
            GlobalVar
              (`Projector
                 (`Concrete (Concrete_ident.of_def_id ~value:true field)))
          in
          let span = Span.of_thir e.span in
          App
            {
              f = { e = projector; typ = TArrow ([ lhs.typ ], typ); span };
              args = [ lhs ];
              generic_args = [];
              trait = None;
              bounds_impls = [];
            }
      | TupleField { lhs; field } ->
          (* TODO: refactor *)
          let lhs = c_expr lhs in
          let tuple_len =
            match lhs.typ with
            | TApp { ident = `TupleType len; _ } -> len
            | _ ->
                assertion_failure [ e.span ]
                  "LHS of tuple field projection is not typed as a tuple."
          in
          let projector =
            GlobalVar
              (`Projector (`TupleField (Int.of_string field, tuple_len)))
          in
          let span = Span.of_thir e.span in
          App
            {
              f = { e = projector; typ = TArrow ([ lhs.typ ], typ); span };
              args = [ lhs ];
              generic_args = [];
              trait = None;
              bounds_impls = [];
            }
      | GlobalName { item = { value = { def_id = id; _ }; _ }; constructor = _ }
        ->
          GlobalVar (def_id ~value:true id)
      | UpvarRef { var_hir_id = id; _ } -> LocalVar (local_ident Expr id)
      | Borrow { arg; borrow_kind = kind } ->
          let e' = c_expr arg in
          let kind = c_borrow_kind e.span kind in
          Borrow { kind; e = e'; witness = W.reference }
      | RawBorrow { arg; mutability = mut } ->
          let e = c_expr arg in
          AddressOf
            {
              e;
              mut = c_mutability W.mutable_pointer mut;
              witness = W.raw_pointer;
            }
      | Break { value; _ } ->
          (* TODO: labels! *)
          let e = Option.map ~f:c_expr value in
          let e = Option.value ~default:(unit_expr span) e in
          Break { e; acc = None; label = None; witness = (W.break, W.loop) }
      | Continue _ ->
          Continue { acc = None; label = None; witness = (W.continue, W.loop) }
      | Return { value } ->
          let e = Option.map ~f:c_expr value in
          let e = Option.value ~default:(unit_expr span) e in
          Return { e; witness = W.early_exit }
      | ConstBlock _ -> unimplemented ~issue_id:923 [ e.span ] "ConstBlock"
      | ConstParam { param = id; _ } (* TODO: shadowing? *) | ConstRef { id } ->
          LocalVar
            {
              name = id.name;
              id =
                Local_ident.mk_id Cnst
                  (MyInt64.to_int id.index
                  |> Option.value_or_thunk ~default:(fun _ ->
                      assertion_failure [ e.span ]
                        "Expected const id to fit in an OCaml native int"));
            }
      | Repeat { value; count } ->
          let value = c_expr value in
          let count = c_constant_expr count in
          let inner =
            U.call Rust_primitives__hax__repeat [ value; count ] span typ
          in
          (U.call Alloc__boxed__Impl__new [ inner ] span typ).e
      | Tuple { fields } ->
          (U.make_tuple_expr' ~span @@ List.map ~f:c_expr fields).e
      | Array { fields } -> Array (List.map ~f:c_expr fields)
      | Adt { info; base; fields; _ } ->
          let is_struct, is_record =
            match info.kind with
            | Struct { named } -> (true, named)
            | Enum { named; _ } -> (false, named)
            | Union ->
                unimplemented ~issue_id:998 [ e.span ]
                  "Construct union types: not supported"
          in
          let constructor = def_id ~value:true info.variant in
          let base =
            match base with
            | None' -> None
            | Base base -> Some (c_expr base.base, W.construct_base)
            | DefaultFields _ ->
                unimplemented ~issue_id:1386 [ e.span ]
                  "Default field values: not supported"
          in
          let fields =
            List.map
              ~f:(fun f ->
                let field = def_id ~value:true f.field in
                let value = c_expr f.value in
                (field, value))
              fields
          in
          Construct { is_record; is_struct; constructor; fields; base }
      | Literal { lit; neg; _ } -> (
          match c_lit e.span neg lit typ with
          | EL_Lit lit -> Literal lit
          | EL_U8Array l ->
              Array
                (List.map
                   ~f:(fun lit ->
                     {
                       e = Literal lit;
                       span;
                       typ = TInt { size = S8; signedness = Unsigned };
                     })
                   l))
      | NamedConst
          {
            item =
              { value = { def_id = id; generic_args; in_trait = impl; _ }; _ };
            _;
          } ->
          let f = GlobalVar (def_id ~value:true id) in
          let args = List.map ~f:(c_generic_value e.span) generic_args in
          let const_args =
            List.filter_map args ~f:(function GConst e -> Some e | _ -> None)
          in
          if List.is_empty const_args && Option.is_none impl then f
          else
            let f =
              {
                e = f;
                span;
                typ = TArrow (List.map const_args ~f:(fun e -> e.typ), typ);
              }
            in
            let trait =
              Option.map impl ~f:(c_impl_expr e.span &&& Fn.const args)
            in
            App
              {
                f;
                trait;
                args = const_args;
                generic_args = [];
                bounds_impls = [];
              }
      | Closure { body; params; upvars; _ } ->
          let params =
            List.filter_map ~f:(fun p -> Option.map ~f:c_pat p.pat) params
          in
          let params =
            if List.is_empty params then
              [ U.M.pat_PWild ~typ:U.M.ty_unit ~span ]
            else params
          in
          let body = c_expr body in
          let upvars = List.map ~f:c_expr upvars in
          Closure { body; params; captures = upvars }
      | Index { index; lhs } ->
          let index_type = c_ty index.span index.ty in
          let lhs_type = c_ty lhs.span lhs.ty in
          call
            (mk_global ([ lhs_type; index_type ] ->. typ)
            @@ Global_ident.of_name ~value:true Core__ops__index__Index__index)
            [ lhs; index ]
      | StaticRef { def_id = id; _ } -> GlobalVar (def_id ~value:true id)
      | PlaceTypeAscription _ ->
          assertion_failure [ e.span ]
            "Got a unexpected node `PlaceTypeAscription`. Please report, we \
             were not able to figure out an expression yielding that node: a \
             bug report would be very valuable here!"
      | ValueTypeAscription { source; _ } -> (c_expr source).e
      | ZstLiteral _ ->
          assertion_failure [ e.span ]
            "`ZstLiteral` are expected to be handled before-hand"
      | Yield _ ->
          unimplemented ~issue_id:924 [ e.span ]
            "Got expression `Yield`: coroutines are not supported by hax"
      | Todo payload ->
          assertion_failure [ e.span ] ("expression Todo\n" ^ payload)
    in
    { e = v; span; typ }

  and c_lhs lhs =
    match lhs.e with
    | LocalVar var -> LhsLocalVar { var; typ = lhs.typ }
    | _ -> (
        match resugar_index_mut lhs with
        | Some (e, index) ->
            LhsArrayAccessor
              { e = c_lhs e; typ = lhs.typ; index; witness = W.nontrivial_lhs }
        | None -> (
            match (U.unbox_underef_expr lhs).e with
            | App
                {
                  f =
                    {
                      e = GlobalVar (`Projector _ as field);
                      typ = TArrow ([ _ ], _);
                      span = _;
                    };
                  args = [ e ];
                  generic_args = _;
                  trait = _;
                  bounds_impls = _;
                (* TODO: see issue #328 *)
                } ->
                LhsFieldAccessor
                  {
                    e = c_lhs e;
                    typ = lhs.typ;
                    field;
                    witness = W.nontrivial_lhs;
                  }
            | _ -> LhsArbitraryExpr { e = lhs; witness = W.arbitrary_lhs }))

  and c_expr_assign lhs rhs =
    Assign { lhs = c_lhs lhs; e = rhs; witness = W.mutable_variable }

  and c_constant_expr (ce : Thir.decorated_for__constant_expr_kind) : expr =
    let rec constant_expr_to_expr (ce : Thir.decorated_for__constant_expr_kind)
        : Thir.decorated_for__expr_kind =
      {
        attributes = ce.attributes;
        contents = constant_expr_kind_to_expr_kind ce.contents ce.span;
        hir_id = ce.hir_id;
        span = ce.span;
        ty = ce.ty;
      }
    and constant_expr_kind_to_expr_kind (ce : Thir.constant_expr_kind) span :
        Thir.expr_kind =
      match ce with
      | Literal lit ->
          let lit, neg = constant_lit_to_lit lit span in
          Literal { lit = { node = lit; span }; neg }
      | Adt { fields; info; repr } ->
          let fields = List.map ~f:constant_field_expr fields in
          Adt { fields; info; repr; base = None'; user_ty = None }
      | Array { fields } ->
          Array { fields = List.map ~f:constant_expr_to_expr fields }
      | Tuple { fields } ->
          Tuple { fields = List.map ~f:constant_expr_to_expr fields }
      | GlobalName item -> GlobalName { item; constructor = None }
      | Borrow arg ->
          Borrow { arg = constant_expr_to_expr arg; borrow_kind = Thir.Shared }
      | ConstRef { id } -> ConstRef { id }
      | Cast _ | RawBorrow _ | TraitConst _ | FnPtr _ | Memory _ ->
          assertion_failure [ span ]
            "constant_lit_to_lit: TraitConst | FnPtr | RawBorrow | Cast | \
             Memory"
      | Todo _ -> assertion_failure [ span ] "ConstantExpr::Todo"
    and constant_lit_to_lit (l : Thir.constant_literal) span :
        Thir.lit_kind * bool =
      match l with
      | Bool v -> (Bool v, false)
      | Char v -> (Char v, false)
      | Int (Int (v, ty)) -> (
          match String.chop_prefix v ~prefix:"-" with
          | Some v -> (Int (v, Signed ty), true)
          | None -> (Int (v, Signed ty), false))
      | Int (Uint (v, ty)) -> (Int (v, Unsigned ty), false)
      | Float (v, ty) -> (
          match String.chop_prefix v ~prefix:"-" with
          | Some v -> (Float (v, Suffixed ty), true)
          | None -> (Float (v, Suffixed ty), false))
      | Str v -> (Str (v, Cooked), false)
      | ByteStr v -> (ByteStr (v, Cooked), false)
      | PtrNoProvenance _ ->
          assertion_failure [ span ] "constant_lit_to_lit: PtrNoProvenance"
    and constant_field_expr ({ field; value } : Thir.constant_field_expr) :
        Thir.field_expr =
      { field; value = constant_expr_to_expr value }
    in
    c_expr (constant_expr_to_expr ce)

  and c_pat (pat : Thir.decorated_for__pat_kind) : pat =
    let span = Span.of_thir pat.span in
    let typ = c_ty pat.span pat.ty in
    let v =
      match pat.contents with
      | Wild | Missing -> PWild
      | AscribeUserType { ascription = { annotation; _ }; subpattern } ->
          let typ, typ_span = c_canonical_user_type_annotation annotation in
          let pat = c_pat subpattern in
          PAscription { typ; typ_span; pat }
      | Binding { mode; subpattern; ty; var; _ } ->
          let mut = c_mutability W.mutable_variable mode.mutability in
          let subpat =
            Option.map ~f:(c_pat &&& Fn.const W.as_pattern) subpattern
          in
          let typ = c_ty pat.span ty in
          let mode = c_binding_mode mode.by_ref in
          let var = local_ident Expr var in
          PBinding { mut; mode; var; typ; subpat }
      | Variant { info; subpatterns; _ } ->
          let is_struct, is_record =
            match info.kind with
            | Struct { named } -> (true, named)
            | Enum { named; _ } -> (false, named)
            | Union ->
                unimplemented ~issue_id:998 [ pat.span ]
                  "Pattern match on union types: not supported"
          in
          let constructor = def_id ~value:true info.variant in
          let fields = List.map ~f:(c_field_pat info) subpatterns in
          PConstruct { constructor; fields; is_record; is_struct }
      | Tuple { subpatterns } ->
          (List.map ~f:c_pat subpatterns |> U.make_tuple_pat').p
      | Deref { subpattern } ->
          PDeref { subpat = c_pat subpattern; witness = W.reference }
      | Constant { value } ->
          let rec pat_of_expr (e : expr) =
            { p = pat'_of_expr' e.e e.span; span = e.span; typ = e.typ }
          and pat'_of_expr' (e : expr') span =
            match e with
            | Literal lit -> PConstant { lit }
            | Array l -> PArray { args = List.map ~f:pat_of_expr l }
            | Borrow { kind = _; e; witness } ->
                PDeref { subpat = pat_of_expr e; witness }
            | _ ->
                assertion_failure (Span.to_thir span)
                  ("expected a pattern, got " ^ [%show: expr'] e)
          in
          (c_constant_expr value |> pat_of_expr).p
      | ExpandedConstant { subpattern; _ } -> (c_pat subpattern).p
      | Array _ -> unimplemented ~issue_id:804 [ pat.span ] "Pat:Array"
      | Or { pats } -> POr { subpats = List.map ~f:c_pat pats }
      | Slice _ -> unimplemented ~issue_id:804 [ pat.span ] "pat Slice"
      | Range _ -> unimplemented ~issue_id:925 [ pat.span ] "pat Range"
      | DerefPattern _ ->
          unimplemented ~issue_id:926 [ pat.span ] "pat DerefPattern"
      | Never -> unimplemented ~issue_id:927 [ pat.span ] "pat Never"
      | Error _ ->
          assertion_failure [ pat.span ]
            "`Error` node: Rust compilation failed. If Rust compilation was \
             fine, please file an issue."
    in
    { p = v; span; typ }

  and c_field_pat _info (field_pat : Thir.field_pat) : field_pat =
    {
      field = def_id ~value:true field_pat.field;
      pat = c_pat field_pat.pattern;
    }

  and extended_literal_of_expr (e : expr) : extended_literal =
    let not_a_literal () =
      assertion_failure (Span.to_thir e.span)
        ("expected a literal, got " ^ [%show: expr] e)
    in
    match e.e with
    | Literal lit -> EL_Lit lit
    | Array lits ->
        EL_U8Array
          (List.map
             ~f:(function
               | {
                   e =
                     Literal
                       (Int { kind = { size = S8; signedness = Unsigned }; _ }
                        as lit);
                   _;
                 } ->
                   lit
               | _ -> not_a_literal ())
             lits)
    | _ -> not_a_literal ()

  and c_canonical_user_type_annotation
      (annotation : Thir.canonical_user_type_annotation) : ty * span =
    (c_ty annotation.span annotation.inferred_ty, Span.of_thir annotation.span)

  and c_pointer e typ span cast source =
    match cast with
    | ClosureFnPointer Safe | ReifyFnPointer ->
        (* we have arrow types, we do not distinguish between top-level functions and closures *)
        (c_expr source).e
    | Unsize _ ->
        (* https://doc.rust-lang.org/std/marker/trait.Unsize.html *)
        (U.call Rust_primitives__unsize [ c_expr source ] span typ).e
        (* let source = c_expr source in *)
        (* let from_typ = source.typ in *)
        (* let to_typ = typ in *)
        (* match (U.Box.Ty.destruct from_typ, U.Box.Ty.destruct to_typ) with *)
        (* | Some _from_typ, Some to_typ -> ( *)
        (*     match U.Box.Expr.destruct source with *)
        (*     | Some source -> *)
        (*         (U.Box.Expr.make *)
        (*         @@ U.call "dummy" "unsize_cast" [] [ source ] span to_typ) *)
        (*           .e *)
        (*     | _ -> *)
        (*         unimplemented e.span *)
        (*           "[Pointer(Unsize)] cast from not directly boxed expression") *)
        (* | _ -> *)
        (*     unimplemented e.span *)
        (*       ("[Pointer(Unsize)] cast\n • from type [" *)
        (*       ^ [%show: ty] from_typ *)
        (*       ^ "]\n • to type [" *)
        (*       ^ [%show: ty] to_typ *)
        (*       ^ "]\n\nThe expression is: " *)
        (*       ^ [%show: expr] source)) *)
    | _ ->
        assertion_failure [ e.span ]
          ("Pointer, with [cast] being " ^ [%show: Thir.pointer_coercion] cast)

  and c_ty (span : Thir.span) (ty : Thir.ty) : ty =
    match ty.value with
    | Bool -> TBool
    | Char -> TChar
    | Int k -> TInt (c_int_ty k)
    | Uint k -> TInt (c_uint_ty k)
    | Float k ->
        TFloat
          (match k with F16 -> F16 | F32 -> F32 | F64 -> F64 | F128 -> F128)
    | Arrow fn_sig | Closure { fn_sig; _ } | FnDef { fn_sig; _ } ->
        let ({ inputs; output; _ } : Thir.ty_fn_sig) = fn_sig.value in
        let inputs =
          if List.is_empty inputs then [ U.unit_typ ]
          else List.map ~f:(c_ty span) inputs
        in
        TArrow (inputs, c_ty span output)
    | Adt { value = { def_id = id; generic_args; _ }; _ } ->
        let ident = def_id ~value:false id in
        let args = List.map ~f:(c_generic_value span) generic_args in
        TApp { ident; args }
    | Foreign _ -> unimplemented ~issue_id:928 [ span ] "Foreign"
    | Str -> TStr
    | Array item_ref ->
        let ty, len =
          match item_ref.value.generic_args with
          | [ Type ty; Const len ] -> (ty, len)
          | _ ->
              assertion_failure [ span ]
                "Wrong generics for array: expected a type and a constant. See \
                 synthetic_items in hax frontend."
        in
        TArray { typ = c_ty span ty; length = c_constant_expr len }
    | Slice item_ref ->
        let ty =
          match item_ref.value.generic_args with
          | [ Type ty ] -> ty
          | _ ->
              assertion_failure [ span ]
                "Wrong generics for slice: expected a type. See \
                 synthetic_items in hax frontend."
        in
        let ty = c_ty span ty in
        TSlice { ty; witness = W.slice }
    | RawPtr _ -> TRawPointer { witness = W.raw_pointer }
    | Ref (_region, ty, mut) ->
        let typ = c_ty span ty in
        let mut = c_mutability W.mutable_reference mut in
        TRef { witness = W.reference; region = "todo"; typ; mut }
    | Never -> U.never_typ
    | Tuple item_ref ->
        let types =
          List.map
            ~f:(function Types.Type ty -> Some ty | _ -> None)
            item_ref.value.generic_args
          |> Option.all
          |> Option.value_or_thunk ~default:(fun _ ->
              assertion_failure [ span ]
                "Wrong generics for slice: expected a type. See \
                 synthetic_items in hax frontend.")
        in
        let types = List.map ~f:(fun ty -> GType (c_ty span ty)) types in
        TApp { ident = `TupleType (List.length types); args = types }
    | Alias { kind = Projection { assoc_item = _; impl_expr }; def_id; _ } ->
        let impl = c_impl_expr span impl_expr in
        let item = Concrete_ident.of_def_id ~value:false def_id in
        TAssociatedType { impl; item }
    | Alias { kind = Opaque _; def_id; _ } ->
        TOpaque (Concrete_ident.of_def_id ~value:false def_id)
    | Alias { kind = Inherent; _ } ->
        assertion_failure [ span ] "Ty::Alias with AliasTyKind::Inherent"
    | Alias { kind = Free; _ } ->
        assertion_failure [ span ] "Ty::Alias with AliasTyKind::Free"
    | Param { index; name } ->
        (* TODO: [id] might not unique *)
        TParam
          {
            name;
            id =
              Local_ident.mk_id Typ
                (MyInt64.to_int index
                |> Option.value_or_thunk ~default:(fun _ ->
                    assertion_failure [ span ]
                      "Expected param id to fit in an OCaml native int"));
          }
    | Error ->
        assertion_failure [ span ]
          "got type `Error`: Rust compilation probably failed."
    | Dynamic (_, predicates, _region) -> (
        let goals, non_traits =
          List.partition_map
            ~f:(fun ((clause, _span) : Types.clause * _) ->
              match clause.kind.value with
              | Trait { trait_ref; _ } ->
                  let goal : dyn_trait_goal =
                    {
                      trait =
                        Concrete_ident.of_def_id ~value:false
                          trait_ref.value.def_id;
                      non_self_args =
                        List.map ~f:(c_generic_value span)
                          (List.tl_exn trait_ref.value.generic_args);
                    }
                  in
                  First goal
              | _ -> Second ())
            predicates.predicates
        in
        match non_traits with
        | [] -> TDyn { witness = W.dyn; goals }
        | _ -> assertion_failure [ span ] "type Dyn with non trait predicate")
    | Coroutine _ ->
        unimplemented ~issue_id:924 [ span ]
          "Got type `Coroutine`: coroutines are not supported by hax"
    | Placeholder _ ->
        assertion_failure [ span ]
          "type Placeholder: should be gone after typechecking"
    | Bound _ ->
        assertion_failure [ span ]
          "type Bound: should be gone after typechecking"
    | Infer _ ->
        assertion_failure [ span ]
          "type Infer: should be gone after typechecking"
    | Todo _ -> assertion_failure [ span ] "type Todo"
  (* fun _ -> Ok Bool *)

  and c_impl_expr (span : Thir.span) (ie : Thir.impl_expr) : impl_expr =
    let goal = c_trait_ref span ie.trait.value in
    let impl = { kind = c_impl_expr_atom span ie.impl goal; goal } in
    match ie.impl with
    | Concrete { value = { impl_exprs = []; _ }; _ } -> impl
    | Concrete { value = { impl_exprs; _ }; _ } ->
        let args = List.map ~f:(c_impl_expr span) impl_exprs in
        { kind = ImplApp { impl; args }; goal }
    | _ -> impl

  and c_trait_ref span (tr : Thir.trait_ref) : trait_goal =
    let trait = Concrete_ident.of_def_id ~value:false tr.value.def_id in
    let args = List.map ~f:(c_generic_value span) tr.value.generic_args in
    { trait; args }

  and c_impl_expr_atom (span : Thir.span) (ie : Thir.impl_expr_atom) goal :
      impl_expr_kind =
    let browse_path (item_kind : impl_expr_kind)
        (chunk : Thir.impl_expr_path_chunk) =
      match chunk with
      | AssocItem
          { item; predicate = { value = { trait_ref; _ }; _ }; index; _ } ->
          let ident =
            { goal = c_trait_ref span trait_ref; name = "i" ^ index }
          in
          let item = Concrete_ident.of_def_id ~value:false item.value.def_id in
          let trait_ref = c_trait_ref span trait_ref in
          Projection
            { impl = { kind = item_kind; goal = trait_ref }; ident; item }
      | Parent { predicate = { value = { trait_ref; _ }; _ }; index; _ } ->
          let ident =
            { goal = c_trait_ref span trait_ref; name = "i" ^ index }
          in
          let trait_ref = c_trait_ref span trait_ref in
          Parent { impl = { kind = item_kind; goal = trait_ref }; ident }
    in
    match ie with
    | Concrete { value = { def_id; generic_args; _ }; _ } ->
        let trait = Concrete_ident.of_def_id ~value:false def_id in
        let args = List.map ~f:(c_generic_value span) generic_args in
        Concrete { trait; args }
    | LocalBound { index; path; _ } ->
        let init = LocalBound { id = "i" ^ index } in
        List.fold ~init ~f:browse_path path
    | Dyn -> Dyn
    | SelfImpl { path; _ } -> List.fold ~init:Self ~f:browse_path path
    | Builtin _ -> Builtin goal
    | Error str ->
        unimplemented ~issue_id:707 [ span ]
          ("Could not resolve trait reference: " ^ str)

  and c_generic_value (span : Thir.span) (ty : Thir.generic_arg) : generic_value
      =
    match ty with
    | Type ty -> GType (c_ty span ty)
    | Const e -> GConst (c_constant_expr e)
    | _ -> GLifetime { lt = "todo generics"; witness = W.lifetime }

  and c_arm (arm : Thir.arm) : arm =
    let arm_pat = c_pat arm.pattern in
    let body = c_expr arm.body in
    let span = Span.of_thir arm.span in
    let guard =
      Option.map
        ~f:(fun (e : Thir.decorated_for__expr_kind) ->
          let guard =
            match e.contents with
            | Let { expr; pat } ->
                IfLet
                  {
                    lhs = c_pat pat;
                    rhs = c_expr expr;
                    witness = W.match_guard;
                  }
            | _ ->
                IfLet
                  {
                    lhs =
                      { p = PConstant { lit = Bool true }; span; typ = TBool };
                    rhs = c_expr e;
                    witness = W.match_guard;
                  }
          in
          { guard; span = Span.of_thir e.span })
        arm.guard
    in
    { arm = { arm_pat; body; guard }; span }

  and c_param span (param : Thir.param) : param =
    {
      typ_span = Option.map ~f:Span.of_thir param.ty_span;
      typ = c_ty (Option.value ~default:span param.ty_span) param.ty;
      pat =
        c_pat
          (Option.value_or_thunk param.pat ~default:(fun _ ->
               assertion_failure [ span ]
                 "c_param: expected param.pat to be non-empty"));
      attrs = c_attrs param.attributes;
    }

  let c_fn_params span (params : Thir.param list) : param list =
    if List.is_empty params then [ U.make_unit_param (Span.of_thir span) ]
    else List.map ~f:(c_param span) params

  let c_generic_param (param : Thir.generic_param) : generic_param =
    let ident =
      let kind =
        match (param.kind : Thir.generic_param_kind) with
        | Lifetime _ -> Local_ident.LILifetime
        | Type _ -> Local_ident.Typ
        | Const _ -> Local_ident.Cnst
      in
      match param.name with
      | Fresh ->
          (* fail with ("[Fresh] ident? " ^ Thir.show_generic_param param) *)
          (* TODO might be wrong to just have a wildcard here *)
          ({ name = "_"; id = Local_ident.mk_id kind 123 } : local_ident)
      | Error -> assertion_failure [ param.span ] "[Error] ident"
      | Plain n -> local_ident kind n
    in
    let kind =
      match (param.kind : Thir.generic_param_kind) with
      | Lifetime _ -> GPLifetime { witness = W.lifetime }
      | Type _ -> GPType
      (* Rustc always fills in const generics on use. Thus we can drop this information. *)
      | Const { default = _; ty } -> GPConst { typ = c_ty param.span ty }
    in
    let span = Span.of_thir param.span in
    let attrs = c_attrs param.attributes in
    { ident; span; attrs; kind }

  let c_clause_kind span id (kind : Thir.clause_kind) :
      generic_constraint option =
    match kind with
    | Trait { is_positive = true; trait_ref } ->
        let args =
          List.map ~f:(c_generic_value span) trait_ref.value.generic_args
        in
        let trait =
          Concrete_ident.of_def_id ~value:false trait_ref.value.def_id
        in
        Some (GCType { goal = { trait; args }; name = "i" ^ Int.to_string id })
    | Projection { impl_expr; assoc_item; ty } ->
        let impl = c_impl_expr span impl_expr in
        let assoc_item =
          Concrete_ident.of_def_id ~value:false assoc_item.def_id
        in
        let typ = c_ty span ty in
        Some (GCProjection { impl; assoc_item; typ })
    | _ -> None

  let c_clause span (index : int) (p : Thir.clause) : generic_constraint option
      =
    let ({ kind; _ } : Thir.clause) = p in
    c_clause_kind span index kind.value

  let list_dedup (equal : 'a -> 'a -> bool) : 'a list -> 'a list =
    let rec aux (seen : 'a list) (todo : 'a list) : 'a list =
      match todo with
      | hd :: tl ->
          if List.mem ~equal seen hd then aux seen tl
          else hd :: aux (hd :: seen) tl
      | _ -> todo
    in
    aux []

  let c_bounds ?(offset : int = 0) span bounds =
    List.fold_left ~init:(offset, [])
      ~f:(fun (i, clauses) c ->
        match c_clause span i c with
        | Some (GCType _ as c) -> (i + 1, c :: clauses)
        | Some c -> (i, c :: clauses)
        | None -> (i, clauses))
      bounds
    |> snd |> List.rev

  (** Named like the bounds of [c_bounds], which they are matched with. *)
  let c_parent_bounds
      (parent_bounds : (Thir.clause * Thir.impl_expr * Thir.span) list) =
    List.fold_left ~init:(0, [])
      ~f:(fun (i, bounds) (clause, impl_expr, span) ->
        match c_clause span i clause with
        | Some (GCType trait_goal) ->
            (i + 1, (c_impl_expr span impl_expr, trait_goal) :: bounds)
        | _ -> (i, bounds))
      parent_bounds
    |> snd |> List.rev

  let c_generics ?(offset : int = 0) (generics : Thir.generics) : generics =
    let bounds = c_bounds ~offset generics.span generics.bounds in
    {
      params = List.map ~f:c_generic_param generics.params;
      constraints = bounds |> list_dedup equal_generic_constraint;
    }

  let c_trait_item' (super : Thir.trait_item) (item : Thir.trait_item_kind) :
      trait_item' =
    let span = super.span in
    match item with
    | Const (_, Some default) ->
        TIDefault
          {
            params = [];
            body = c_expr default.expr;
            witness = W.trait_item_default;
          }
    | Const (ty, None) -> TIFn (c_ty span ty)
    | RequiredFn (sg, _) ->
        let (Thir.{ inputs; output; _ } : Thir.fn_decl) = sg.decl in
        let output =
          match output with
          | DefaultReturn _span -> unit_typ
          | Return ty -> c_ty span ty
        in
        let inputs =
          if List.is_empty inputs then [ U.unit_typ ]
          else List.map ~f:(c_ty span) inputs
        in
        TIFn (TArrow (inputs, output))
    | ProvidedFn (_, { params; body; _ }) ->
        TIDefault
          {
            params = c_fn_params span params;
            body = c_expr body.expr;
            witness = W.trait_item_default;
          }
    | Type (bounds, None) ->
        let bounds =
          c_bounds span bounds
          |> List.filter_map ~f:(fun bound ->
              match bound with GCType impl -> Some impl | _ -> None)
        in
        TIType bounds
    | Type (_, Some _) ->
        unimplemented ~issue_id:929 [ span ]
          "Associated types defaults are not supported by hax yet (it is a \
           nightly feature)"
end

include struct
  open Make (struct
    let is_core_item = false
  end)

  let import_ty : Types.span -> Types.node_for__ty_kind -> Ast.Rust.ty = c_ty

  let import_trait_ref : Types.span -> Thir.trait_ref -> Ast.Rust.trait_goal =
    c_trait_ref

  let import_clause :
      Types.span -> int -> Types.clause -> Ast.Rust.generic_constraint option =
    c_clause
end

(** Instantiate the functor for translating expressions. The crate name can be
    configured (there are special handling related to `core`) *)
let make ~krate : (module EXPR) =
  let is_core_item = String.(krate = "core" || krate = "core_hax_model") in
  let module M : EXPR = Make (struct
    let is_core_item = is_core_item
  end)
  in
  (module M)

let c_trait_item (item : Thir.trait_item) : trait_item =
  let open (val make ~krate:item.owner_id.contents.value.krate : EXPR) in
  let { params; constraints } = c_generics ~offset:1 item.generics in
  (* TODO: see TODO in impl items *)
  let ti_ident = Concrete_ident.of_def_id ~value:false item.owner_id in
  {
    ti_span = Span.of_thir item.span;
    ti_generics = { params; constraints };
    ti_v = c_trait_item' item item.kind;
    ti_ident;
    ti_attrs = c_item_attrs item.attributes;
  }

let is_automatically_derived (attrs : Thir.attribute list) =
  List.exists (* We need something better here, see issue #108 *)
    ~f:(function
      (* This will break once these attributes get properly parsed. It will
          then be very easy to parse them correctly *)
      | Parsed (AutomaticallyDerived _) -> true
      | _ -> false)
    attrs

let should_skip (attrs : Thir.item_attributes) =
  let attrs = attrs.attributes @ attrs.parent_attributes in
  is_automatically_derived attrs

(** Converts a generic parameter to a generic value. This assumes the parameter
    is bound. *)
let generic_param_to_value ({ ident; kind; span; _ } : generic_param) :
    generic_value =
  match kind with
  | GPLifetime { witness } ->
      GLifetime { lt = [%show: local_ident] ident; witness }
  | GPType -> GType (TParam ident)
  | GPConst { typ } -> GConst { e = LocalVar ident; typ; span }

(** Generate a cast function from an inductive to its represantant type. *)
let cast_of_enum typ_name generics typ thir_span
    (variants : (variant * Types.variant_for__thir_body) list) : item =
  let span = Span.of_thir thir_span in
  let (module M) = Ast_builder.make span in
  let self =
    let args = List.map ~f:generic_param_to_value generics.params in
    TApp { ident = `Concrete typ_name; args }
  in
  let expr_of_int (n : Int64.t) : expr =
    let kind =
      match typ with
      | TInt kind -> kind
      | typ ->
          assertion_failure [ thir_span ]
            ("cast_of_enum: expected in type, got " ^ [%show: ty] typ)
    in
    let value = Int64.to_string n in
    M.expr_Literal ~typ (Int { value; negative = Int64.is_negative n; kind })
  in
  let arms =
    (* Each variant comes with a [rustc_middle::ty::VariantDiscr]. Some variant have [Explicit] discr (i.e. an expression)
       while other have [Relative] discr (the distance to the previous last explicit discr). *)
    List.folding_map variants ~init:None
      ~f:(fun previous_explicit_discriminator (variant, { discr; _ }) ->
        let pat =
          let mk_wild_field (cid, typ, _) =
            { field = `Concrete cid; pat = M.pat_PWild ~typ }
          in
          M.pat_PConstruct ~constructor:(`Concrete variant.name)
            ~is_struct:false ~typ ~is_record:variant.is_record
            ~fields:(List.map ~f:mk_wild_field variant.arguments)
        in
        match (previous_explicit_discriminator, discr) with
        | None, Relative m -> (None, (pat, expr_of_int m))
        | _, Explicit { def_id = did; _ } ->
            let e = M.expr_GlobalVar ~typ (def_id ~value:true did) in
            (Some e, (pat, e))
        | Some e, Relative n ->
            let n = expr_of_int n in
            let e = U.call Core__ops__arith__Add__add [ e; n ] span typ in
            (previous_explicit_discriminator, (pat, e)))
    |> List.map ~f:(fun (p, e) -> M.arm p e)
  in
  let scrutinee_var = Local_ident.{ name = "x"; id = mk_id Expr (-1) } in
  let scrutinee = M.expr_LocalVar ~typ:self scrutinee_var in
  let ident = cast_name_for_type typ_name in
  let params =
    let pat = U.make_var_pat scrutinee_var self span in
    [ { pat; typ = self; typ_span = None; attrs = [] } ]
  in
  let body = M.expr_Match ~typ ~scrutinee ~arms in
  M.item_Fn ~ident ~attrs:[] ~name:ident ~generics ~params ~safety:Safe ~body

let rec c_item ~ident ~type_only (item : Thir.item) : item list =
  try
    Span.with_owner_hint item.owner_id (fun _ ->
        c_item_unwrapped ~ident ~type_only item)
  with Diagnostics.SpanFreeError.Exn payload ->
    let context, kind = Diagnostics.SpanFreeError.payload payload in
    let error = Diagnostics.pretty_print_context_kind context kind in
    let span = Span.of_thir item.span in
    [ make_hax_error_item span ident error ]

and c_item_unwrapped ~ident ~type_only (item : Thir.item) : item list =
  let open (val make ~krate:item.owner_id.contents.value.krate : EXPR) in
  let span = Span.of_thir item.span in
  let attrs = c_item_attrs item.attributes in
  (* this is true if the user explicilty requested to erase using the `opaque` macro *)
  let erased_by_user attrs =
    Attr_payloads.payloads attrs
    |> List.exists ~f:(fst >> [%matches? (Erased : Types.ha_payload)])
  in
  let item_erased_by_user = erased_by_user attrs in
  let type_only =
    type_only
    && Attr_payloads.payloads attrs
       |> List.exists ~f:(fst >> [%matches? (NeverErased : Types.ha_payload)])
       |> not
  in
  (* This is true if the item should be erased because we are in type-only mode
     (Only certain kinds of items are erased in this case). *)
  let erased_by_hax =
    should_skip item.attributes
    || type_only
       &&
       match item.kind with
       | Fn _ | Static _ -> true
       | Impl { of_trait = Some _; items; _ }
         when List.exists items ~f:(fun item ->
                  match item.kind with Type _ -> true | _ -> false)
              |> not ->
           true
       | _ -> false
  in
  (* If the item is erased by hax we need to add the Erased attribute.
     It is already present if the item is erased by user. *)
  let attrs_with_erased erased_by_hax erased_by_user attrs =
    if erased_by_hax && not erased_by_user then
      Attr_payloads.to_attr Erased span :: attrs
    else attrs
  in
  let attrs = attrs_with_erased erased_by_hax item_erased_by_user attrs in
  let erased = item_erased_by_user || erased_by_hax in

  let mk_one v = { span; v; ident; attrs } in
  let mk v = [ mk_one v ] in
  let drop_body =
    erased
    && Attr_payloads.payloads attrs
       |> List.exists ~f:(fst >> [%matches? (NeverErased : Types.ha_payload)])
       |> not
  in
  let c_body = if drop_body then c_expr_drop_body else c_expr in
  let assert_item_def_id () =
    Option.value_or_thunk item.def_id ~default:(fun _ ->
        assertion_failure [ item.span ] "Expected this item to have a `def_id`")
  in
  (* TODO: things might be unnamed (e.g. constants) *)
  match (item.kind : Thir.item_kind) with
  | Const (_, generics, _, body) ->
      mk
      @@ Fn
           {
             name = Concrete_ident.of_def_id ~value:true (assert_item_def_id ());
             generics = c_generics generics;
             body = c_body body.expr;
             params = [];
             safety = Safe;
           }
  | Static (true, _, _, _) ->
      unimplemented ~issue_id:1343 [ item.span ]
        "Mutable static items are not supported."
  | Static (false, _, _ty, body) ->
      let name = Concrete_ident.of_def_id ~value:true (assert_item_def_id ()) in
      let generics = { params = []; constraints = [] } in
      mk
        (Fn
           {
             name;
             generics;
             body = c_body body.expr;
             params = [];
             safety = Safe;
           })
  | TyAlias (_, generics, ty) ->
      mk
      @@ TyAlias
           {
             name =
               Concrete_ident.of_def_id ~value:false (assert_item_def_id ());
             generics = c_generics generics;
             ty = c_ty item.span ty;
           }
  | Fn { generics; def = { body; params; header = { safety; _ }; _ }; _ } ->
      mk
      @@ Fn
           {
             name = Concrete_ident.of_def_id ~value:true (assert_item_def_id ());
             generics = c_generics generics;
             body = c_body body.expr;
             params = c_fn_params item.span params;
             safety = c_header_safety safety;
           }
  | (Enum (_, generics, _, _) | Struct (_, generics, _, _)) when erased ->
      let generics = c_generics generics in
      let is_struct = match item.kind with Struct _ -> true | _ -> false in
      let def_id = assert_item_def_id () in
      let name = Concrete_ident.of_def_id ~value:false def_id in
      mk @@ Type { name; generics; variants = []; is_struct }
  | Enum (_, generics, variants, repr) ->
      let def_id = assert_item_def_id () in
      let generics = c_generics generics in
      let is_struct = false in
      let discs =
        (* Each variant might introduce a anonymous constant defining its discriminant integer  *)
        List.filter_map ~f:(fun v -> v.disr_expr) variants
        |> List.map ~f:(fun Types.{ def_id; body; _ } ->
            let name = Concrete_ident.of_def_id ~value:true def_id in
            let generics = { params = []; constraints = [] } in
            let body = c_expr body.expr in
            {
              v = Fn { name; generics; body; params = []; safety = Safe };
              span;
              ident = name;
              attrs = [];
            })
      in
      let is_primitive =
        List.for_all
          ~f:(fun { data; _ } ->
            match data with
            | Unit _ | Tuple ([], _, _) | Struct { fields = []; _ } -> true
            | _ -> false)
          variants
      in
      let variants =
        List.map
          ~f:(fun ({ data; def_id = variant_id; attributes; _ } as original) ->
            let is_record =
              [%matches? (Struct { fields = _ :: _; _ } : Types.variant_data)]
                data
            in
            let name = Concrete_ident.of_def_id ~value:true variant_id in
            let arguments =
              match data with
              | Tuple (fields, _, _) | Struct { fields; _ } ->
                  List.map
                    ~f:(fun { def_id = id; ty; span; attributes; _ } ->
                      ( Concrete_ident.of_def_id ~value:true id,
                        c_ty span ty,
                        c_attrs attributes ))
                    fields
              | Unit _ -> []
            in
            let attrs = c_attrs attributes in
            ({ name; arguments; is_record; attrs }, original))
          variants
      in
      let name = Concrete_ident.of_def_id ~value:true def_id in
      let cast_fun =
        cast_of_enum name generics (c_ty item.span repr.typ) item.span variants
      in
      let variants, _ = List.unzip variants in
      let result =
        mk_one (Type { name; generics; variants; is_struct }) :: discs
      in
      if is_primitive then cast_fun :: result else result
  | Struct (_, generics, v, _repr) ->
      let generics = c_generics generics in
      let def_id = assert_item_def_id () in
      let is_struct = true in
      (* repeating the attributes of the item in the variant: TODO is that ok? *)
      let v =
        let name = Concrete_ident.of_def_id ~value:true def_id in
        (* let name = Concrete_ident.Create.move_under name ~new_parent:name in *)
        let mk fields is_record =
          let arguments =
            List.map
              ~f:(fun Thir.{ def_id = id; ty; span; attributes; _ } ->
                ( Concrete_ident.of_def_id ~value:true id,
                  c_ty span ty,
                  c_attrs attributes ))
              fields
          in
          { name; arguments; is_record; attrs }
        in
        match v with
        | Tuple (fields, _, _) -> mk fields false
        | Struct { fields = _ :: _ as fields; _ } -> mk fields true
        | _ -> { name; arguments = []; is_record = false; attrs }
      in
      let variants = [ v ] in
      let name = Concrete_ident.of_def_id ~value:false def_id in
      mk @@ Type { name; generics; variants; is_struct }
  | Trait (NotConst, No, safety, _, generics, _bounds, items) ->
      let items =
        List.filter
          ~f:(fun { attributes; _ } -> not (should_skip attributes))
          items
      in
      let name =
        Concrete_ident.of_def_id ~value:false (assert_item_def_id ())
      in
      let { params; constraints } = c_generics generics in
      let self =
        let id =
          Local_ident.mk_id Typ 0
          (* todo *)
        in
        let ident = Local_ident.{ name = "Self"; id } in
        { ident; span; attrs = []; kind = GPType }
      in
      let params = self :: params in
      let generics = { params; constraints } in
      let items = List.map ~f:c_trait_item items in
      let safety = csafety safety in
      mk @@ Trait { name; generics; items; safety }
  | Trait (_, Yes, _, _, _, _, _) ->
      unimplemented ~issue_id:930 [ item.span ] "Auto trait"
  | Trait (Const, _, _, _, _, _, _) ->
      unimplemented ~issue_id:930 [ item.span ] "Const trait"
  | Impl { of_trait = None; generics; items; _ } ->
      let items =
        List.filter
          ~f:(fun { attributes; _ } -> not (should_skip attributes))
          items
      in
      List.map
        ~f:(fun (item : Thir.impl_item) ->
          let item_def_id =
            Concrete_ident.of_def_id ~value:false item.owner_id
          in
          let attrs = c_item_attrs item.attributes in
          let sub_item_erased_by_user = erased_by_user attrs in
          let erased_by_type_only =
            type_only && match item.kind with Fn _ -> true | _ -> false
          in
          let sub_item_erased =
            sub_item_erased_by_user || erased_by_type_only
          in
          let attrs =
            attrs_with_erased erased_by_type_only sub_item_erased_by_user attrs
          in
          let c_body = if sub_item_erased then c_expr_drop_body else c_body in

          let generics = c_generics generics in
          let offset =
            List.count generics.constraints ~f:[%matches? GCType _]
          in

          let v =
            match (item.kind : Thir.impl_item_kind) with
            | Fn { body; params; header = { safety; _ }; _ } ->
                let params =
                  if List.is_empty params then [ U.make_unit_param span ]
                  else List.map ~f:(c_param item.span) params
                in
                Fn
                  {
                    name = item_def_id;
                    generics =
                      U.concat_generics generics
                        (c_generics ~offset item.generics);
                    body = c_body body.expr;
                    params;
                    safety = c_header_safety safety;
                  }
            | Const (_ty, e) ->
                Fn
                  {
                    name = item_def_id;
                    generics;
                    (* does that make sense? can we have `const<T>`? *)
                    body = c_body e.expr;
                    params = [];
                    safety = Safe;
                  }
            | Type _ty ->
                assertion_failure [ item.span ]
                  "Inherent implementations are not supposed to have \
                   associated types \
                   (https://doc.rust-lang.org/reference/items/implementations.html#inherent-implementations)."
          in
          let ident = Concrete_ident.of_def_id ~value:false item.owner_id in
          { span = Span.of_thir item.span; v; ident; attrs })
        items
  | Impl
      {
        of_trait = Some of_trait;
        generics;
        self_ty;
        items;
        safety;
        parent_bounds;
        _;
      } ->
      let items =
        List.filter
          ~f:(fun { attributes; _ } -> not (should_skip attributes))
          items
      in
      let generics = c_generics generics in
      let offset = List.count generics.constraints ~f:[%matches? GCType _] in
      let items =
        if erased then []
        else
          List.map
            ~f:(fun (item : Thir.impl_item) ->
              (* TODO: introduce a Kind.TraitImplItem or
                 something. Otherwise we have to assume every
                 backend will see traits and impls as
                 records. See https://github.com/pereq-dev/hax/issues/271. *)
              let ii_ident =
                Concrete_ident.of_def_id ~value:false item.owner_id
              in
              {
                ii_span = Span.of_thir item.span;
                ii_generics = c_generics ~offset item.generics;
                ii_v =
                  (match (item.kind : Thir.impl_item_kind) with
                  | Fn { body; params; _ } ->
                      let params =
                        if List.is_empty params then [ U.make_unit_param span ]
                        else List.map ~f:(c_param item.span) params
                      in
                      IIFn { body = c_expr body.expr; params }
                  | Const (_ty, e) -> IIFn { body = c_expr e.expr; params = [] }
                  | Type { ty; parent_bounds } ->
                      IIType
                        {
                          typ = c_ty item.span ty;
                          parent_bounds = c_parent_bounds parent_bounds;
                        });
                ii_ident;
                ii_attrs = c_item_attrs item.attributes;
              })
            items
      in
      mk
      @@ Impl
           {
             generics;
             self_ty = c_ty item.span self_ty;
             of_trait =
               ( Concrete_ident.of_def_id ~value:false of_trait.value.def_id,
                 List.map
                   ~f:(c_generic_value item.span)
                   of_trait.value.generic_args );
             items;
             parent_bounds = c_parent_bounds parent_bounds;
             safety = csafety safety;
           }
  | Use ({ span = _; res; segments; rename }, _) ->
      let v =
        Use
          {
            path = List.map ~f:(fun x -> fst x.ident) segments;
            is_external =
              List.exists
                ~f:(function None | Some Err -> true | _ -> false)
                res;
            (* TODO: this should represent local/external? *)
            rename;
          }
      in
      (* ident is supposed to always be an actual item, thus here we need to cheat a bit *)
      (* TODO: is this DUMMY thing really needed? there's a `Use` segment (see #272) *)
      let def_id = item.owner_id in
      (* let def_id : Types.def_id =
           let value =
             {
               def_id.contents.value with
               path =
                 def_id.contents.value.path
                 @ [
                     Types.
                       { data = ValueNs "DUMMY"; disambiguator = MyInt64.of_int 0 };
                   ];
             }
           in
           { contents = { def_id.contents with value } }
         in *)
      [
        { span; v; ident = Concrete_ident.of_def_id ~value:false def_id; attrs };
      ]
  | Union _ ->
      unimplemented ~issue_id:998 [ item.span ] "Union types: not supported"
  | GlobalAsm _ ->
      unimplemented ~issue_id:1344 [ item.span ]
        "Inline assembly blocks are not supported"
  | ExternCrate _ | Macro _ | Mod _ | ForeignMod _ | TraitAlias _ ->
      mk NotImplementedYet

let import_item ~type_only (item : Thir.item) :
    concrete_ident * (item list * Diagnostics.t list) =
  let ident = Concrete_ident.of_def_id ~value:false item.owner_id in
  let reports, r =
    let f = U.Reducers.disambiguate_local_idents in
    Diagnostics.Core.capture (fun _ ->
        c_item item ~ident ~type_only |> List.map ~f)
  in
  (ident, (r, reports))
