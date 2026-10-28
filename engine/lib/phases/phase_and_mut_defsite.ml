open! Prelude

module%inlined_contents Make
    (FA :
      Features.T
        with type mutable_variable = Features.On.mutable_variable
         and type mutable_reference = Features.On.mutable_reference
         and type nontrivial_lhs = Features.On.nontrivial_lhs
         and type arbitrary_lhs = Features.On.arbitrary_lhs
         and type reference = Features.On.reference) =
struct
  open Ast
  module FB = FA

  include
    Phase_utils.MakeBase (FA) (FB)
      (struct
        let phase_id = [%auto_phase_name auto]
      end)

  module A = Ast.Make (FA)
  module B = Ast.Make (FB)
  module BVisitors = Ast_visitors.Make (FB)

  module Implem : ImplemT.T = struct
    let metadata = metadata

    module S = struct
      include Features.SUBTYPE.Id
    end

    module UB = Ast_utils.Make (FB)

    module M = struct
      open B
      open UB

      (* given `ty`, produces type `&mut ty` *)
      let mut_ref (typ : ty) : ty =
        let mut = Mutable Features.On.mutable_reference in
        TRef { witness = Features.On.reference; region = ""; typ; mut }

      (* given `e`, produces well-typed expr `&mut e` *)
      let mut_borrow (e : expr) : expr =
        let kind = Mut Features.On.mutable_reference in
        let witness = Features.On.reference in
        let e' = Borrow { kind; e; witness } in
        { e with e = e'; typ = mut_ref e.typ }

      let expect_mut_ref_param (all_vars : local_ident list) (i : int)
          (param : param) : (local_ident * ty * span) option =
        let* typ = Expect.mut_ref param.typ in
        match param.pat.p with
        | PBinding
            { mut = Immutable; mode = ByValue; var; typ = _; subpat = None } ->
            Some (var, typ, param.pat.span)
        | PWild ->
            let var =
              fresh_local_ident_in all_vars ("arg_" ^ Int.to_string i ^ "_wild")
            in
            Some (var, typ, param.pat.span)
        | _ ->
            Error.raise
              { kind = NonTrivialAndMutFnInput; span = param.pat.span }

      let rewrite_fn_sig (all_vars : local_ident list) (params : param list)
          (output : ty) :
          (param list * ty * (local_ident * ty * span) list) option =
        let and_muts =
          List.filter_mapi ~f:(expect_mut_ref_param all_vars) params
        in
        match and_muts with
        | [] -> None
        | _ ->
            let params =
              List.mapi
                ~f:(fun i param ->
                  match expect_mut_ref_param all_vars i param with
                  | None -> param
                  | Some (var, typ, span) ->
                      let p : pat' =
                        let mut = Mutable Features.On.mutable_variable in
                        PBinding
                          { mut; mode = ByValue; var; typ; subpat = None }
                      in
                      { param with pat = { p; span; typ }; typ })
                params
            in
            let output_components =
              List.map ~f:snd3 and_muts
              @ if UB.is_unit_typ output then [] else [ output ]
            in
            let output = UB.make_tuple_typ output_components in
            Some (params, output, and_muts)

      (* visit an expression and replace all `Return e` nodes by `Return (f e)` *)
      let map_returns ~(f : expr -> expr) : expr -> expr =
        let visitor =
          object (self)
            inherit [_] Visitors.map as super

            method! visit_expr' () e =
              match e with
              | Return { e; witness } ->
                  let e = self#visit_expr () e in
                  Return { e = f e; witness }
              | _ -> super#visit_expr' () e
          end
        in
        visitor#visit_expr ()

      (* transforms
          `(let … = … in)* expr`
         into
          `(let … = … in)* let output = expr in output` *)
      let wrap_in_identity_let (e : expr) : expr =
        let var = Local_ident.{ id = mk_id Expr 0; name = "hax_temp_output" } in
        let f (e : expr) : expr =
          match e.e with
          | GlobalVar (`TupleCons 0) -> e
          | _ ->
              let rhs = e in
              let lhs, body =
                if [%eq: ty] e.typ UB.unit_typ then
                  (* This case has been added to fix https://github.com/pereq-dev/hax/issues/720.
                     It might need a better solution. *)
                  ( UB.M.pat_PWild ~span:e.span ~typ:e.typ,
                    UB.M.expr_unit ~span:e.span )
                else
                  (UB.make_var_pat var e.typ e.span, { e with e = LocalVar var })
              in
              { body with e = Let { monadic = None; lhs; rhs; body } }
        in
        UB.map_body_of_nested_lets f e

      let mutref_to_mut_expr (vars : local_ident list) : expr -> expr =
        let ( <|?> ) (type a) (x : a option) (f : unit -> a option) : a option =
          x |> Option.map ~f:Option.some |> Option.value_or_thunk ~default:f
        in
        let in_vars = List.mem vars ~equal:[%equal: local_ident] in
        let expect_in_vars_local_var (x : expr) : local_ident option =
          match x.e with LocalVar v when in_vars v -> Some v | _ -> None
        in
        let retyped_local_var_in_vars e =
          let* var = expect_in_vars_local_var e in
          (* var is supposed to be typed `&mut _` *)
          let typ =
            Expect.mut_ref e.typ
            |> Option.value_or_thunk ~default:(fun () ->
                Error.assertion_failure e.span @@ "Expect.mut_ref: got `None`")
          in
          (* we reconstruct `e` to type it correctly *)
          Some { e = LocalVar var; typ; span = e.span }
        in
        let visitor =
          object
            inherit [_] Visitors.map as super

            method! visit_expr () e =
              (let* e = Expect.deref e in
               retyped_local_var_in_vars e)
              <|?> (fun _ -> retyped_local_var_in_vars e)
              |> Option.value_or_thunk ~default:(fun _ -> super#visit_expr () e)
          end
        in
        visitor#visit_expr ()

      let convert_lhs =
        (* TODO: refactor (see #316) *)
        let rec place_to_lhs (p : Place.t) : lhs =
          let typ = p.typ in
          match p.place with
          | LocalVar var -> LhsLocalVar { var; typ }
          | FieldProjection { place; projector } ->
              let e = place_to_lhs place in
              LhsFieldAccessor
                {
                  witness = Features.On.nontrivial_lhs;
                  field = projector;
                  typ;
                  e;
                }
          | IndexProjection { place; index } ->
              let e = place_to_lhs place in
              LhsArrayAccessor
                { e; typ; index; witness = Features.On.nontrivial_lhs }
          | _ ->
              let e = Place.to_expr p in
              LhsArbitraryExpr { witness = Features.On.arbitrary_lhs; e }
        in

        let visitor =
          object
            inherit [_] Visitors.map as super

            method! visit_expr () e =
              try super#visit_expr () e
              with Diagnostics.SpanFreeError.Exn (Data (context, kind)) ->
                UB.hax_failure_expr e.span e.typ (context, kind)
                  (UB.LiftToFullAst.expr e)

            method! visit_expr' () e =
              match e with
              | Assign { lhs; e; witness } ->
                  let span = e.span in
                  let lhs = UB.expr_of_lhs span lhs in
                  let lhs =
                    lhs |> Place.of_expr
                    |> Option.value_or_thunk ~default:(fun () ->
                        Error.assertion_failure span
                        @@ "Place.of_expr: got `None` for: "
                        ^ Print_rust.pexpr_str (UB.LiftToFullAst.expr lhs))
                    |> place_to_lhs
                  in
                  Assign { lhs; e; witness }
              | _ -> super#visit_expr' () e
          end
        in
        visitor#visit_expr ()

      let rewrite_function (params : param list) (body : expr) :
          (param list * expr) option =
        let all_vars =
          UB.Reducers.collect_local_idents#visit_expr () body
          :: List.map ~f:(Reducers.collect_local_idents#visit_param ()) params
          |> Set.union_list (module Local_ident)
          |> Set.to_list
        in
        let* params, _, vars = rewrite_fn_sig all_vars params body.typ in
        let idents = List.map ~f:fst3 vars in
        let vars =
          List.map
            ~f:(fun (var, typ, span) -> B.{ span; typ; e = LocalVar var })
            vars
        in
        let f (e : B.expr) : B.expr =
          UB.make_tuple_expr ~span:e.span
            (vars @ if UB.is_unit_typ e.typ then [] else [ e ])
        in
        let body =
          body |> mutref_to_mut_expr idents |> convert_lhs |> map_returns ~f
          |> wrap_in_identity_let
          |> UB.map_body_of_nested_lets f
        in
        Some (params, body)
    end

    include M

    let ditems (items : A.item list) : B.item list =
      let items : B.item list = Stdlib.Obj.magic items in
      let visitor =
        object
          inherit [_] BVisitors.map as super

          method! visit_impl_item' () item' =
            (match item' with
              | IIFn { params; body } ->
                  let* params, body = rewrite_function params body in
                  Some (B.IIFn { body; params })
              | _ -> None)
            |> Option.value_or_thunk
                 ~default:(Fn.flip super#visit_impl_item' item')

          method! visit_trait_item () item =
            let span = item.ti_span in
            let ti_v =
              (match item.ti_v with
                | TIFn (TArrow (inputs, output)) ->
                    (* Here, we craft a dummy function so that we can
                     call `rewrite_function` *)
                    let var =
                      Local_ident.{ id = mk_id Expr 0; name = "dummy" }
                    in
                    let params =
                      List.map
                        ~f:(fun typ ->
                          let pat = UB.make_var_pat var typ span in
                          (* let pat : B.pat = { typ; p; span } in *)
                          B.{ pat; typ; typ_span = None; attrs = [] })
                        inputs
                    in
                    let body =
                      B.
                        {
                          e =
                            (* this is wrongly typed, though it's fine,
                             we throw this away before returning *)
                            (UB.unit_expr span).e;
                          typ = output;
                          span;
                        }
                    in
                    let* params, body = rewrite_function params body in
                    let inputs = List.map ~f:(fun p -> p.typ) params in
                    let output = body.typ in
                    let ty = B.TArrow (inputs, output) in
                    Some (B.TIFn ty)
                | TIDefault { params; body; witness } ->
                    let* params, body = rewrite_function params body in
                    let witness = S.trait_item_default span witness in
                    Some (B.TIDefault { params; body; witness })
                | _ -> None)
              |> Option.value_or_thunk
                   ~default:(Fn.flip super#visit_trait_item' item.ti_v)
            in
            { item with ti_v }

          method! visit_item () i =
            try super#visit_item () i
            with Diagnostics.SpanFreeError.Exn (Data (context, kind)) ->
              let error = Diagnostics.pretty_print_context_kind context kind in
              let cast_item : B.item -> Ast.Full.item = Stdlib.Obj.magic in
              let ast = cast_item i |> Print_rust.pitem_str in
              let msg =
                error ^ "\nLast available AST for this item:\n\n" ^ ast
              in
              B.make_hax_error_item i.span i.ident msg

          method! visit_item' () item' =
            (match item' with
              | Fn { name; generics; body; params; safety } ->
                  let* params, body = rewrite_function params body in
                  Some (B.Fn { name; generics; body; params; safety })
              | _ -> None)
            |> Option.value_or_thunk ~default:(Fn.flip super#visit_item' item')
        end
      in
      List.map ~f:(visitor#visit_item ()) items

    let dexpr (_e : A.expr) : B.expr =
      Stdlib.failwith "Should not be called directly"
  end

  include Implem
  module FA = FA
end
[@@add "subtype.ml"]
