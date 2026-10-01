module New_tests.Legacy__dyn__lib
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

class t_Printable (v_Self: Type0) (v_S: Type0) = {
  f_stringify_pre:v_Self -> Type0;
  f_stringify_post:v_Self -> v_S -> Type0;
  f_stringify:x0: v_Self
    -> Prims.Pure v_S (f_stringify_pre x0) (fun result -> f_stringify_post x0 result)
}

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl: t_Printable i32 Alloc.String.t_String =
  {
    f_stringify_pre = (fun (self: i32) -> true);
    f_stringify_post = (fun (self: i32) (out: Alloc.String.t_String) -> true);
    f_stringify
    =
    fun (self: i32) -> Alloc.String.f_to_string #i32 #FStar.Tactics.Typeclasses.solve self
  }

/// @fail(extraction): ssprove(HAX0008), coq(HAX0008)
/// @fail(extraction): proverif(HAX0008)
let print (a: dyn 1 (fun z -> t_Printable z Alloc.String.t_String)) : Prims.unit =
  let args:Alloc.String.t_String =
    f_stringify #(dyn 1 (fun z -> t_Printable z Alloc.String.t_String))
      #Alloc.String.t_String
      #FStar.Tactics.Typeclasses.solve
      a
    <:
    Alloc.String.t_String
  in
  let args:t_Array Core_models.Fmt.Rt.t_Argument (mk_usize 1) =
    let list = [Core_models.Fmt.Rt.impl__new_display #Alloc.String.t_String args] in
    FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
    Rust_primitives.Hax.array_of_list 1 list
  in
  let _:Prims.unit =
    Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_v1 (mk_usize 2)
          (mk_usize 1)
          (let list = [""; "\n"] in
            FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 2);
            Rust_primitives.Hax.array_of_list 2 list)
          args
        <:
        Core_models.Fmt.t_Arguments)
  in
  let _:Prims.unit = () in
  ()
