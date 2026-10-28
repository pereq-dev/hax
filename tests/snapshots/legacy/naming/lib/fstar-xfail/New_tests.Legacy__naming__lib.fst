module New_tests.Legacy__naming__lib
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

type t_Foo =
  | Foo_A : t_Foo
  | Foo_B { f_x:usize }: t_Foo

type t_Foo2 =
  | Foo2_A : t_Foo2
  | Foo2_B { f_x:usize }: t_Foo2

type t_B = | B : t_B

type t_C = { f_x:usize }

type t_X = | X : t_X

let mk_c (_: Prims.unit) : t_C =
  let _:t_Foo = Foo_B ({ f_x = mk_usize 3 }) <: t_Foo in
  let _:t_X = X <: t_X in
  { f_x = mk_usize 3 } <: t_C

let impl_Foo__f (self: t_Foo) : t_Foo = Foo_A <: t_Foo

let impl_B__f (self: t_B) : t_B = B <: t_B

type t_Foobar = { f_a:t_Foo }

let f__g (_: Prims.unit) : Prims.unit = ()

let f__g__impl_B__g (self: t_B) : usize = mk_usize 0

type f__g__impl_B__g__t_Foo =
  | C_f__g__impl_B__g__Foo_A : f__g__impl_B__g__t_Foo
  | C_f__g__impl_B__g__Foo_B { f__g__impl_B__g__f_x:usize }: f__g__impl_B__g__t_Foo

let f__g__impl_Foo__g (self: t_Foo) : usize = mk_usize 1

let f (x: t_Foobar) : usize = f__g__impl_Foo__g x.f_a

let f__g__impl_Foo__g__t_hello__h (_: Prims.unit) : Prims.unit = ()

let reserved_names (v_val v_noeq v_of: u8) : u8 = (v_val +! v_noeq <: u8) +! v_of

type t_Arity1 (v_T: Type0) = | Arity1 : v_T -> t_Arity1 v_T

class t_T1 (v_Self: Type0) = { __marker_trait_t_T1:Prims.unit }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_T1_for_Foo: t_T1 t_Foo = { __marker_trait_t_T1 = () }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_T1_for_tuple_Foo_u8: t_T1 (t_Foo & u8) = { __marker_trait_t_T1 = () }

class t_T2_for_a (v_Self: Type0) = { __marker_trait_t_T2_for_a:Prims.unit }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_T2_ee_for_a_for_Arity1_of_tuple_Foo_u8: t_T2_for_a (t_Arity1 (t_Foo & u8)) =
  { __marker_trait_t_T2_for_a = () }

class t_T3_ee_for_a (v_Self: Type0) = { __marker_trait_t_T3_ee_for_a:Prims.unit }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_T3_ee_e_for_a_for_Foo: t_T3_ee_for_a t_Foo = { __marker_trait_t_T3_ee_for_a = () }

type t_StructA = { f_a:usize }

type t_StructB = {
  f_a:usize;
  f_b:usize
}

type t_StructC = { f_a:usize }

type t_StructD = {
  f_a:usize;
  f_b:usize
}

let construct_structs (a b: usize) : Prims.unit =
  let _:t_StructA = { f_a = a } <: t_StructA in
  let _:t_StructB = { f_a = a; f_b = b } <: t_StructB in
  let _:t_StructC = { f_a = a } <: t_StructC in
  let _:t_StructD = { f_a = a; f_b = b } <: t_StructD in
  ()

let v_INHERENT_CONSTANT: usize = mk_usize 3

class t_FooTrait (v_Self: Type0) = { f_ASSOCIATED_CONSTANT:usize }

let constants
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_FooTrait v_T)
      (_: Prims.unit)
    : usize =
  (f_ASSOCIATED_CONSTANT #FStar.Tactics.Typeclasses.solve <: usize) +! v_INHERENT_CONSTANT

/// From issue https://github.com/pereq-dev/hax/issues/839
let string_shadows (v_string n: string) : Prims.unit = ()

/// From issue https://github.com/pereq-dev/hax/issues/1450
let items_under_closures (_: Prims.unit) : Prims.unit =
  let _: Prims.unit -> Prims.unit =
    fun temp_0_ ->
      let _:Prims.unit = temp_0_ in
      ()
  in
  ()

let items_under_closures__anon_const_0__nested_function (_: Prims.unit) : Prims.unit = ()

type items_under_closures__anon_const_0__t_NestedStruct =
  | C_items_under_closures__anon_const_0__NestedStruct : items_under_closures__anon_const_0__t_NestedStruct

let items_under_closures__nested_function (_: Prims.unit) : Prims.unit = ()

type items_under_closures__t_NestedStruct =
  | C_items_under_closures__NestedStruct : items_under_closures__t_NestedStruct
