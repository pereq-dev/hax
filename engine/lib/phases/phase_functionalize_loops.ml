open! Prelude

module%inlined_contents Make
    (F :
      Features.T
        with type continue = Features.Off.continue
         and type early_exit = Features.Off.early_exit
         and type break = Features.Off.break) =
struct
  open Ast
  module FA = F

  module FB = struct
    include F
    include Features.Off.Loop
    include Features.Off.For_loop
    include Features.Off.While_loop
    include Features.Off.For_index_loop
    include Features.Off.State_passing_loop
    include Features.Off.Fold_like_loop
    include Features.Off.Continue
    include Features.Off.Early_exit
    include Features.Off.Break
  end

  include
    Phase_utils.MakeBase (F) (FB)
      (struct
        let phase_id = [%auto_phase_name auto]
      end)

  module Implem : ImplemT.T = struct
    let metadata = metadata

    module UA = Ast_utils.Make (F)
    module UB = Ast_utils.Make (FB)
    module Visitors = Ast_visitors.Make (F)

    module S = struct
      include Features.SUBTYPE.Id
    end

    type loop_annotation_kind =
      | LoopInvariant of { index_pat : B.pat option; invariant : B.expr }
      | LoopVariant of B.expr

    type loop_annotation = {
      body : B.expr;
      annotation : loop_annotation_kind option;
    }

    let extract_loop_annotation (body : B.expr) : loop_annotation =
      let rhs_body =
        let* (e_let : UB.D.expr_Let) = UB.D.expr_Let body in
        let*? _ = Option.is_none e_let.monadic in
        let* _ = UB.D.pat_PWild e_let.lhs in
        let* app = UB.D.expr_App e_let.rhs in
        let* f = UB.D.expr_GlobalVar app.f in
        Some (f, app.args, e_let.body)
      in
      match rhs_body with
      | Some
          ( f,
            [ { e = Closure { params = [ pat ]; body = invariant; _ }; _ } ],
            body )
        when Global_ident.eq_name Hax_lib___internal_loop_invariant f ->
          {
            body;
            annotation =
              Some (LoopInvariant { index_pat = Some pat; invariant });
          }
      | Some (f, [ invariant ], body)
        when Global_ident.eq_name Hax_lib___internal_while_loop_invariant f ->
          {
            body;
            annotation = Some (LoopInvariant { index_pat = None; invariant });
          }
      | Some (f, [ invariant ], body)
        when Global_ident.eq_name Hax_lib___internal_loop_decreases f ->
          { body; annotation = Some (LoopVariant invariant) }
      | _ -> { body; annotation = None }

    let expect_invariant_variant (annotation1 : loop_annotation_kind option)
        (annotation2 : loop_annotation_kind option) :
        loop_annotation_kind option * loop_annotation_kind option =
      match annotation1 with
      | Some (LoopVariant _) -> (annotation2, annotation1)
      | _ -> (annotation1, annotation2)

    type iterator =
      | Range of { start : B.expr; end_ : B.expr }
      | RangeInclusive of { start : B.expr; end_ : B.expr }
      | Slice of B.expr
      | ChunksExact of { size : B.expr; slice : B.expr }
      | Enumerate of iterator
      | StepBy of { n : B.expr; it : iterator }
    [@@deriving show]

    let rec as_iterator (e : B.expr) : iterator option =
      match e.e with
      | Construct
          {
            constructor = `Concrete range_ctor;
            is_record = true;
            is_struct = true;
            fields =
              [ (`Concrete start_field, start); (`Concrete end_field, end_) ];
            base = None;
          }
        when Concrete_ident.eq_name Core__ops__range__Range__start start_field
             && Concrete_ident.eq_name Core__ops__range__Range range_ctor
             && Concrete_ident.eq_name Core__ops__range__Range__end end_field ->
          Some (Range { start; end_ })
      | _ -> meth_as_iterator e

    and meth_as_iterator (e : B.expr) : iterator option =
      let* f, args =
        match e.e with
        | App { f = { e = GlobalVar f; _ }; args; _ } -> Some (f, args)
        | _ -> None
      in
      let f_eq n = Global_ident.eq_name n f in
      let one_arg () = match args with [ x ] -> Some x | _ -> None in
      let two_args () = match args with [ x; y ] -> Some (x, y) | _ -> None in
      if f_eq Core__ops__range__Impl_7__new then
        let* start, end_ = two_args () in
        match start.typ with
        | TInt _ -> Some (RangeInclusive { start; end_ })
        | _ -> None
      else if f_eq Core__iter__traits__iterator__Iterator__step_by then
        let* it, n = two_args () in
        let* it = as_iterator it in
        Some (StepBy { n; it })
      else if
        f_eq Core__iter__traits__collect__IntoIterator__into_iter
        || f_eq Core__slice__Impl__iter
      then
        let* iterable = one_arg () in
        match iterable.typ with
        | TSlice _ | TArray _ -> Some (Slice iterable)
        | _ -> as_iterator iterable
      else if f_eq Core__iter__traits__iterator__Iterator__enumerate then
        let* iterable = one_arg () in
        let* iterator = as_iterator iterable in
        Some (Enumerate iterator)
      else if f_eq Core__slice__Impl__chunks_exact then
        let* slice, size = two_args () in
        Some (ChunksExact { size; slice })
      else None

    let fn_args_of_iterator (cf : A.cf_kind option) (it : iterator) :
        (Concrete_ident.name * B.expr list * B.ty) option =
      let open Concrete_ident_generated in
      let usize = B.TInt { size = SSize; signedness = Unsigned } in
      match it with
      | Enumerate (ChunksExact { size; slice }) ->
          let fold_op =
            match cf with
            | Some BreakOrReturn ->
                Rust_primitives__hax__folds__fold_enumerated_chunked_slice_return
            | Some BreakOnly ->
                Rust_primitives__hax__folds__fold_enumerated_chunked_slice_cf
            | None -> Rust_primitives__hax__folds__fold_enumerated_chunked_slice
          in
          Some (fold_op, [ size; slice ], usize)
      | ChunksExact { size; slice } ->
          let fold_op =
            match cf with
            | Some BreakOrReturn ->
                Rust_primitives__hax__folds__fold_chunked_slice_return
            | Some BreakOnly ->
                Rust_primitives__hax__folds__fold_chunked_slice_cf
            | None -> Rust_primitives__hax__folds__fold_chunked_slice
          in
          Some (fold_op, [ size; slice ], usize)
      | Enumerate (Slice slice) ->
          let fold_op =
            match cf with
            | Some BreakOrReturn ->
                Rust_primitives__hax__folds__fold_enumerated_slice_return
            | Some BreakOnly ->
                Rust_primitives__hax__folds__fold_enumerated_slice_cf
            | None -> Rust_primitives__hax__folds__fold_enumerated_slice
          in
          Some (fold_op, [ slice ], usize)
      | StepBy { n; it = Range { start; end_ } } ->
          let fold_op =
            match cf with
            | Some BreakOrReturn ->
                Rust_primitives__hax__folds__fold_range_step_by_return
            | Some BreakOnly ->
                Rust_primitives__hax__folds__fold_range_step_by_cf
            | None -> Rust_primitives__hax__folds__fold_range_step_by
          in
          Some (fold_op, [ start; end_; n ], start.typ)
      | Range { start; end_ } ->
          let fold_op =
            match cf with
            | Some BreakOrReturn ->
                Rust_primitives__hax__folds__fold_range_return
            | Some BreakOnly -> Rust_primitives__hax__folds__fold_range_cf
            | None -> Rust_primitives__hax__folds__fold_range
          in
          Some (fold_op, [ start; end_ ], start.typ)
      | RangeInclusive { start; end_ } ->
          let fold_op =
            match cf with
            | Some BreakOrReturn ->
                Rust_primitives__hax__folds__fold_range_inclusive_return
            | Some BreakOnly ->
                Rust_primitives__hax__folds__fold_range_inclusive_cf
            | None -> Rust_primitives__hax__folds__fold_range_inclusive
          in
          Some (fold_op, [ start; end_ ], start.typ)
      | _ -> None

    [%%inline_defs dmutability + dsafety_kind]

    let rec dexpr_unwrapped (expr : A.expr) : B.expr =
      let span = expr.span in
      let module M = UB.M in
      let module MS = (val M.make span) in
      match expr.e with
      | Loop { body; kind = ForLoop { it; pat; _ }; state; control_flow; _ } ->
          let bpat, init =
            match state with
            | Some { bpat; init; _ } -> (dpat bpat, dexpr init)
            | None ->
                let unit = UB.unit_expr span in
                (M.pat_PWild ~span ~typ:unit.typ, unit)
          in
          let body = dexpr body in
          let { body; annotation } = extract_loop_annotation body in
          let it = dexpr it in
          let pat = dpat pat in
          let fn : B.expr = UB.make_closure [ bpat; pat ] body body.span in
          let cf = Option.map ~f:fst control_flow in
          let f, args =
            match as_iterator it |> Option.bind ~f:(fn_args_of_iterator cf) with
            | Some (f, args, typ) ->
                (* TODO what happens if there is control flow? *)
                let invariant : B.expr =
                  let default =
                    let pat = MS.pat_PWild ~typ in
                    (pat, MS.expr_Literal ~typ:TBool (Bool true))
                  in
                  let pat, invariant =
                    match annotation with
                    | Some (LoopInvariant { index_pat = Some pat; invariant })
                      ->
                        (pat, invariant)
                    | _ -> default
                  in
                  UB.make_closure [ bpat; pat ] invariant invariant.span
                in
                (f, args @ [ invariant; init; fn ])
            | None ->
                let fold : Concrete_ident.name =
                  match cf with
                  | Some BreakOrReturn ->
                      Rust_primitives__hax__folds__fold_return
                  | Some BreakOnly -> Rust_primitives__hax__folds__fold_cf
                  | None -> Core__iter__traits__iterator__Iterator__fold
                in
                (fold, [ it; init; fn ])
          in
          UB.call f args span (dty span expr.typ)
      | Loop { body; kind = WhileLoop { condition; _ }; state; control_flow; _ }
        ->
          let bpat, init =
            match state with
            | Some { bpat; init; _ } -> (dpat bpat, dexpr init)
            | None ->
                let unit = UB.unit_expr span in
                (M.pat_PWild ~span ~typ:unit.typ, unit)
          in
          let body = dexpr body in
          let { body; annotation = annotation1 } =
            extract_loop_annotation body
          in
          let { body; annotation = annotation2 } =
            extract_loop_annotation body
          in
          let invariant, variant =
            expect_invariant_variant annotation1 annotation2
          in
          let invariant =
            match invariant with
            | Some (LoopInvariant { index_pat = None; invariant }) -> invariant
            | _ -> MS.expr_Literal ~typ:TBool (Bool true)
          in
          let variant =
            match variant with
            | Some (LoopVariant variant) -> variant
            | _ ->
                let kind = { size = S32; signedness = Unsigned } in
                let e =
                  UB.M.expr_Literal ~typ:(TInt kind) ~span:body.span
                    (Int { value = "0"; negative = false; kind })
                in
                UB.call Rust_primitives__hax__int__from_machine [ e ] e.span
                  (TApp
                     {
                       ident =
                         `Concrete
                           (Concrete_ident.of_name ~value:false
                              Hax_lib__int__Int);
                       args = [];
                     })
          in
          let condition = dexpr condition in
          let condition : B.expr =
            M.expr_Closure ~params:[ bpat ] ~body:condition ~captures:[]
              ~span:condition.span
              ~typ:(TArrow ([ bpat.typ ], condition.typ))
          in
          let body : B.expr =
            M.expr_Closure ~params:[ bpat ] ~body ~captures:[]
              ~typ:(TArrow ([ bpat.typ ], body.typ))
              ~span:body.span
          in
          let fold_operator : Concrete_ident.name =
            match control_flow with
            | Some (BreakOrReturn, _) -> Rust_primitives__hax__while_loop_return
            | Some (BreakOnly, _) -> Rust_primitives__hax__while_loop_cf
            | None -> Rust_primitives__hax__while_loop
          in
          let invariant : B.expr =
            UB.make_closure [ bpat ] invariant invariant.span
          in
          let variant = UB.make_closure [ bpat ] variant variant.span in
          (* The invariant should come before the condition. This allows to use the invariant
             to prove panic freedom of the condition. *)
          UB.call fold_operator
            [ invariant; condition; variant; init; body ]
            span (dty span expr.typ)
      | Loop _ ->
          Error.unimplemented ~issue_id:933 ~details:"Unhandled loop kind" span
      | [%inline_arms "dexpr'.*" - Loop - Break - Continue - Return] ->
          map (fun e -> B.{ e; typ = dty expr.span expr.typ; span = expr.span })
      | _ -> .
    [@@inline_ands bindings_of dexpr - dexpr' - dloop_kind - dloop_state]

    [%%inline_defs "Item.*"]
  end

  include Implem
end
[@@add "subtype.ml"]
