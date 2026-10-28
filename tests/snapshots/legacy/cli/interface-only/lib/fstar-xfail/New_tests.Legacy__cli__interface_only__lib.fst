module New_tests.Legacy__cli__interface_only__lib
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

/// This item contains unsafe blocks and raw references, two features
/// not supported by hax. Thanks to the `-i` flag and the `+:`
/// modifier, `f` is still extractable as an interface.
/// Expressions within type are still extracted, as well as pre- and
/// post-conditions.
/// @fail(extraction): proverif(HAX0008, HAX0008, HAX0008, HAX0008), ssprove(HAX0008, HAX0008, HAX0008, HAX0008), coq(HAX0008, HAX0008, HAX0008, HAX0008), fstar(HAX0008, HAX0008, HAX0008, HAX0008)
/// @fail(extraction): legacy-lean(HAX0008, HAX0008, HAX0008, HAX0008)
let f (x: u8)
    : Prims.Pure (t_Array u8 (mk_usize 4))
      (requires x <. mk_u8 254)
      (ensures
        fun r ->
          let r:t_Array u8 (mk_usize 4) = r in
          (r.[ mk_usize 0 ] <: u8) >. x) =
  Rust_primitives.Hax.failure "Explicit rejection by a phase in the Hax engine:\na node of kind [Raw_pointer] have been found in the AST\n\nNote: the error was labeled with context `reject_RawOrMutPointer`.\n"
    "{\n let y: raw_pointer!() = { cast(x) };\n {\n let _: tuple0 = {\n {\n let _: tuple0 = {\n {\n let _: tuple0 = {\n std::io::stdio::e_print({\n let args: tuple1<&int> = { Tuple1(&(deref(y))) };\n {\n let args: [c..."

(* item error backend: Explicit rejection by a phase in the Hax engine:
a node of kind [Raw_pointer] have been found in the AST

Note: the error was labeled with context `reject_RawOrMutPointer`.

Last available AST for this item:

/// This struct contains a field which uses raw pointers, which are
/// not supported by hax. This item cannot be extracted at all: we
/// need to exclude it with `-i '-*::Foo'`.
/// @fail(extraction): proverif(HAX0008), fstar(HAX0008), coq(HAX0008), ssprove(HAX0008)
/// @fail(extraction): legacy-lean(HAX0008)
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
struct t_Foo {
    f_unsupported_field: raw_pointer!(),
}


Last AST:
/** print_rust: pitem: not implemented  (item: { Concrete_ident.T.def_id =
  { Explicit_def_id.T.is_constructor = false;
    def_id =
    { Types.index = (0, 0, None); is_local = true; kind = Types.Struct;
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
                     (Types.TypeNs "legacy__cli__interface_only__lib");
                     disambiguator = 0 }
                    ]
                  }
                }
              });
      path =
      [{ Types.data = (Types.TypeNs "legacy__cli__interface_only__lib");
         disambiguator = 0 };
        { Types.data = (Types.TypeNs "Foo"); disambiguator = 0 }]
      }
    };
  moved = None; suffix = None }) */
const _: () = ();
 *)

type t_Bar = | Bar : t_Bar

/// Non-inherent implementations are extracted, their bodies are not
/// dropped. This might be a bit surprising: see
/// https://github.com/pereq-dev/hax/issues/616.
[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl: Core_models.Convert.t_From t_Bar Prims.unit =
  {
    f_from_pre = (fun ((): Prims.unit) -> true);
    f_from_post = (fun ((): Prims.unit) (out: t_Bar) -> true);
    f_from = fun ((): Prims.unit) -> Bar <: t_Bar
  }

let f_from__impl_1__from (_: u8) : t_Bar = Bar <: t_Bar

/// If you need to drop the body of a method, please hoist it:
[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_1: Core_models.Convert.t_From t_Bar u8 =
  {
    f_from_pre = (fun (x: u8) -> true);
    f_from_post = (fun (x: u8) (out: t_Bar) -> true);
    f_from = fun (x: u8) -> f_from__impl_1__from x
  }

type t_Holder (v_T: Type0) = { f_value:Alloc.Vec.t_Vec v_T Alloc.Alloc.t_Global }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_2 (#v_T: Type0) : Core_models.Convert.t_From (t_Holder v_T) Prims.unit =
  {
    f_from_pre = (fun ((): Prims.unit) -> true);
    f_from_post = (fun ((): Prims.unit) (out: t_Holder v_T) -> true);
    f_from = fun ((): Prims.unit) -> { f_value = Alloc.Vec.impl__new #v_T () } <: t_Holder v_T
  }

type t_Param (v_SIZE: usize) = { f_value:t_Array u8 v_SIZE }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_3 (v_SIZE: usize) : Core_models.Convert.t_From (t_Param v_SIZE) Prims.unit =
  {
    f_from_pre = (fun ((): Prims.unit) -> true);
    f_from_post = (fun ((): Prims.unit) (out: t_Param v_SIZE) -> true);
    f_from
    =
    fun ((): Prims.unit) ->
      { f_value = Rust_primitives.Hax.repeat (mk_u8 0) v_SIZE } <: t_Param v_SIZE
  }

let ff_generic (v_X: usize) (#v_U: Type0) (e_x: v_U) : t_Param v_X =
  { f_value = Rust_primitives.Hax.repeat (mk_u8 0) v_X } <: t_Param v_X

class t_T (v_Self: Type0) = {
  [@@@ FStar.Tactics.Typeclasses.no_method]f_Assoc:Type0;
  f_d_pre:Prims.unit -> Type0;
  f_d_post:Prims.unit -> Prims.unit -> Type0;
  f_d:x0: Prims.unit -> Prims.Pure Prims.unit (f_d_pre x0) (fun result -> f_d_post x0 result)
}

/// Impls with associated types are not erased
[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_T_for_u8: t_T u8 =
  {
    f_Assoc = u8;
    f_d_pre = (fun (_: Prims.unit) -> true);
    f_d_post = (fun (_: Prims.unit) (out: Prims.unit) -> true);
    f_d = fun (_: Prims.unit) -> ()
  }

class t_T2 (v_Self: Type0) = {
  f_d_pre:Prims.unit -> Type0;
  f_d_post:Prims.unit -> Prims.unit -> Type0;
  f_d:x0: Prims.unit -> Prims.Pure Prims.unit (f_d_pre x0) (fun result -> f_d_post x0 result)
}

/// Items can be forced to be transparent
[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_T2_for_u8: t_T2 u8 =
  {
    f_d_pre = (fun (_: Prims.unit) -> false);
    f_d_post = (fun (_: Prims.unit) (out: Prims.unit) -> true);
    f_d = fun (_: Prims.unit) -> ()
  }

let rec padlen (b: t_Slice u8) (n: usize)
    : Prims.Pure usize
      (requires (Core_models.Slice.impl__len #u8 b <: usize) >=. n)
      (ensures
        fun out ->
          let out:usize = out in
          out <=. n) =
  if n >. mk_usize 0 && (b.[ n -! mk_usize 1 <: usize ] <: u8) =. mk_u8 0
  then mk_usize 1 +! (padlen b (n -! mk_usize 1 <: usize) <: usize)
  else mk_usize 0
