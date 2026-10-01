module New_tests.Rustc_coverage__attr__trait_impl_inherit
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

(* item error backend: Explicit rejection by a phase in the Hax engine:
a node of kind [Trait_item_default] have been found in the AST

Note: the error was labeled with context `reject_TraitItemDefault`.

Last available AST for this item:

/** @fail(extraction): ssprove(HAX0008), fstar(HAX0008), coq(HAX0008)*/#[feature(coverage_attribute)]#[allow(dead_code, unused, unconditional_recursion)]#[feature(register_tool, if_let_guard)]#[feature(coverage_attribute, stmt_expr_attributes, custom_inner_attributes, test,
yield_expr, coroutines, coroutine_trait, no_core, core_intrinsics)]#[register_tool(_hax)]trait t_T<Self_>{fn f_f((self: Self)) -> tuple0{{let _: tuple0 = {std::io::stdio::e_print(core_models::fmt::rt::impl_1__new_const::<generic_value!(todo)>(["default\n"]))};{let _: tuple0 = {Tuple0};Tuple0}}}}

Last AST:
/** print_rust: pitem: not implemented  (item: { Concrete_ident.T.def_id =
  { Explicit_def_id.T.is_constructor = false;
    def_id =
    { Types.index = (0, 0, None); is_local = true; kind = Types.Trait;
      krate = "new_tests";
      parent =
      (Some { Types.contents =
              { Types.id = 0;
                value =
                { Types.index = (0, 0, None); is_local = true;
                  kind = Types.Mod; krate = "new_tests";
                  parent =
                  (Some { Types.contents =
                          { Types.id = 0;
                            value =
                            { Types.index = (0, 0, None); is_local = true;
                              kind = Types.Mod; krate = "new_tests";
                              parent = None; path = [] }
                            }
                          });
                  path =
                  [{ Types.data =
                     (Types.TypeNs "rustc_coverage__attr__trait_impl_inherit");
                     disambiguator = 0 }
                    ]
                  }
                }
              });
      path =
      [{ Types.data =
         (Types.TypeNs "rustc_coverage__attr__trait_impl_inherit");
         disambiguator = 0 };
        { Types.data = (Types.TypeNs "T"); disambiguator = 0 }]
      }
    };
  moved = None; suffix = None }) */
const _: () = ();
 *)

type t_S = | S : t_S

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl: t_T t_S =
  {
    f_f_pre = (fun (self: t_S) -> true);
    f_f_post = (fun (self: t_S) (out: Prims.unit) -> true);
    f_f
    =
    fun (self: t_S) ->
      let _:Prims.unit =
        Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
              (let list = ["impl S\n"] in
                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                Rust_primitives.Hax.array_of_list 1 list)
            <:
            Core_models.Fmt.t_Arguments)
      in
      let _:Prims.unit = () in
      ()
  }

let main (_: Prims.unit) : Prims.unit =
  let _:Prims.unit = f_f #t_S #FStar.Tactics.Typeclasses.solve (S <: t_S) in
  ()
