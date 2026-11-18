module New_tests.Legacy__impl_trait_in_trait__lib
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

type t_Foo = | Foo : u8 -> t_Foo

(* item error backend: something is not implemented yet.
`impl` types are not supported in type signatures of associated items.

This is discussed in issue https://github.com/hacspec/hax/issues/1965.
Please upvote or comment this issue if you see this error message.
Note: the error was labeled with context `RejectImplTypeMethod`.

Last available AST for this item:

/// @fail(extraction): fstar(HAX0001), legacy-lean(HAX0001)
#[allow(dead_code)]
#[allow(dead_code, unused, unconditional_recursion)]
#[feature(register_tool, if_let_guard)]
#[feature(
    coverage_attribute,
    stmt_expr_attributes,
    custom_inner_attributes,
    test,
    yield_expr,
    coroutines,
    coroutine_trait,
    no_core,
    core_intrinsics
)]
#[register_tool(_hax)]
trait t_GenStreamer<Self_> {
    #[allow(dead_code)]
    #[allow(dead_code, unused, unconditional_recursion)]
    #[feature(register_tool, if_let_guard)]
    #[feature(
        coverage_attribute,
        stmt_expr_attributes,
        custom_inner_attributes,
        test,
        yield_expr,
        coroutines,
        coroutine_trait,
        no_core,
        core_intrinsics
    )]
    #[register_tool(_hax)]
    fn f_gstream<const N: int, Anonymous: 'unk>(_: &Self) -> proj_asso_type!();
}


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
                     (Types.TypeNs "legacy__impl_trait_in_trait__lib");
                     disambiguator = 0 }
                    ]
                  }
                }
              });
      path =
      [{ Types.data = (Types.TypeNs "legacy__impl_trait_in_trait__lib");
         disambiguator = 0 };
        { Types.data = (Types.TypeNs "GenStreamer"); disambiguator = 0 }]
      }
    };
  moved = None; suffix = None }) */
const _: () = ();
 *)

class t_Streamer (v_Self: Type0) = {
  [@@@ FStar.Tactics.Typeclasses.no_method]f_stream_impl_trait:Type0;
  f_stream_pre:v_Self -> Type0;
  f_stream_post:v_Self -> f_stream_impl_trait -> Type0;
  f_stream:x0: v_Self
    -> Prims.Pure f_stream_impl_trait (f_stream_pre x0) (fun result -> f_stream_post x0 result)
}

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl: t_Streamer t_Foo =
  {
    f_stream_impl_trait = u8;
    f_stream_pre = (fun (self: t_Foo) -> true);
    f_stream_post = (fun (self: t_Foo) (out: u8) -> true);
    f_stream = fun (self: t_Foo) -> self._0
  }
