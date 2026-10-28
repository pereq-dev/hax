module New_tests.Rustc_coverage__issue_84561_
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Std.Env in
  ()

type t_Foo = | Foo : u32 -> t_Foo

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_1': Core_models.Marker.t_StructuralPartialEq t_Foo

unfold
let impl_1 = impl_1'

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_2': Core_models.Cmp.t_PartialEq t_Foo t_Foo

unfold
let impl_2 = impl_2'

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_3': Core_models.Cmp.t_Eq t_Foo

unfold
let impl_3 = impl_3'

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl: Core_models.Fmt.t_Debug t_Foo =
  {
    f_fmt_pre = (fun (self: t_Foo) (f: Core_models.Fmt.t_Formatter) -> true);
    f_fmt_post
    =
    (fun
        (self: t_Foo)
        (f: Core_models.Fmt.t_Formatter)
        (out1:
          (Core_models.Fmt.t_Formatter &
            Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error))
        ->
        true);
    f_fmt
    =
    fun (self: t_Foo) (f: Core_models.Fmt.t_Formatter) ->
      let
      (tmp0: Core_models.Fmt.t_Formatter),
      (out: Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error) =
        Core_models.Fmt.impl_11__write_fmt f
          (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
              (let list = ["try and succeed"] in
                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                Rust_primitives.Hax.array_of_list 1 list)
            <:
            Core_models.Fmt.t_Arguments)
      in
      let f:Core_models.Fmt.t_Formatter = tmp0 in
      match out <: Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error with
      | Core_models.Result.Result_Ok _ ->
        let hax_temp_output:Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error =
          Core_models.Result.Result_Ok (() <: Prims.unit)
          <:
          Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error
        in
        f, hax_temp_output
        <:
        (Core_models.Fmt.t_Formatter &
          Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
      | Core_models.Result.Result_Err err ->
        f,
        (Core_models.Result.Result_Err err
          <:
          Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
        <:
        (Core_models.Fmt.t_Formatter &
          Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
  }

/// @fail(extraction): ssprove(HAX0001)
let test3 (_: Prims.unit) : Prims.unit =
  let is_true:bool =
    (Core_models.Iter.Traits.Exact_size.f_len #Std.Env.t_Args
        #FStar.Tactics.Typeclasses.solve
        (Std.Env.args () <: Std.Env.t_Args)
      <:
      usize) =.
    mk_usize 1
  in
  let bar:t_Foo = Foo (mk_u32 1) <: t_Foo in
  let _:Prims.unit =
    match bar, (Foo (mk_u32 1) <: t_Foo) <: (t_Foo & t_Foo) with
    | left_val, right_val -> Hax_lib.v_assert (left_val =. right_val <: bool)
  in
  let baz:t_Foo = Foo (mk_u32 0) <: t_Foo in
  let _:Prims.unit =
    match baz, (Foo (mk_u32 1) <: t_Foo) <: (t_Foo & t_Foo) with
    | left_val, right_val -> Hax_lib.v_assert (~.(left_val =. right_val <: bool) <: bool)
  in
  let args:t_Foo = (Foo (mk_u32 1) <: t_Foo) <: t_Foo in
  let args:t_Array Core_models.Fmt.Rt.t_Argument (mk_usize 1) =
    let list = [Core_models.Fmt.Rt.impl__new_debug #t_Foo args] in
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
  let args:t_Foo = bar <: t_Foo in
  let args:t_Array Core_models.Fmt.Rt.t_Argument (mk_usize 1) =
    let list = [Core_models.Fmt.Rt.impl__new_debug #t_Foo args] in
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
  let args:t_Foo = baz <: t_Foo in
  let args:t_Array Core_models.Fmt.Rt.t_Argument (mk_usize 1) =
    let list = [Core_models.Fmt.Rt.impl__new_debug #t_Foo args] in
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
  let _:Prims.unit =
    match (Foo (mk_u32 1) <: t_Foo), (Foo (mk_u32 1) <: t_Foo) <: (t_Foo & t_Foo) with
    | left_val, right_val -> Hax_lib.v_assert (left_val =. right_val <: bool)
  in
  let _:Prims.unit =
    match (Foo (mk_u32 0) <: t_Foo), (Foo (mk_u32 1) <: t_Foo) <: (t_Foo & t_Foo) with
    | left_val, right_val -> Hax_lib.v_assert (~.(left_val =. right_val <: bool) <: bool)
  in
  let _:Prims.unit =
    match (Foo (mk_u32 2) <: t_Foo), (Foo (mk_u32 2) <: t_Foo) <: (t_Foo & t_Foo) with
    | left_val, right_val -> Hax_lib.v_assert (left_val =. right_val <: bool)
  in
  let bar:t_Foo = Foo (mk_u32 0) <: t_Foo in
  let _:Prims.unit =
    match bar, (Foo (mk_u32 3) <: t_Foo) <: (t_Foo & t_Foo) with
    | left_val, right_val -> Hax_lib.v_assert (~.(left_val =. right_val <: bool) <: bool)
  in
  let _:Prims.unit =
    match (Foo (mk_u32 0) <: t_Foo), (Foo (mk_u32 4) <: t_Foo) <: (t_Foo & t_Foo) with
    | left_val, right_val -> Hax_lib.v_assert (~.(left_val =. right_val <: bool) <: bool)
  in
  let _:Prims.unit =
    match (Foo (mk_u32 3) <: t_Foo), (Foo (mk_u32 3) <: t_Foo) <: (t_Foo & t_Foo) with
    | left_val, right_val -> Hax_lib.v_assert (left_val =. right_val <: bool)
  in
  let args:t_Foo = bar <: t_Foo in
  let args:t_Array Core_models.Fmt.Rt.t_Argument (mk_usize 1) =
    let list = [Core_models.Fmt.Rt.impl__new_debug #t_Foo args] in
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
  let args:t_Foo = (Foo (mk_u32 1) <: t_Foo) <: t_Foo in
  let args:t_Array Core_models.Fmt.Rt.t_Argument (mk_usize 1) =
    let list = [Core_models.Fmt.Rt.impl__new_debug #t_Foo args] in
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
  let _:Prims.unit =
    match (Foo (mk_u32 0) <: t_Foo), (Foo (mk_u32 5) <: t_Foo) <: (t_Foo & t_Foo) with
    | left_val, right_val -> Hax_lib.v_assert (~.(left_val =. right_val <: bool) <: bool)
  in
  let _:Prims.unit =
    match (Foo (mk_u32 0) <: t_Foo), (Foo (mk_u32 5) <: t_Foo) <: (t_Foo & t_Foo) with
    | left_val, right_val -> Hax_lib.v_assert (~.(left_val =. right_val <: bool) <: bool)
  in
  let is_true:bool =
    (Core_models.Iter.Traits.Exact_size.f_len #Std.Env.t_Args
        #FStar.Tactics.Typeclasses.solve
        (Std.Env.args () <: Std.Env.t_Args)
      <:
      usize) =.
    mk_usize 1
  in
  let _:Prims.unit =
    match (Foo (mk_u32 1) <: t_Foo), (Foo (mk_u32 1) <: t_Foo) <: (t_Foo & t_Foo) with
    | left_val, right_val -> Hax_lib.v_assert (left_val =. right_val <: bool)
  in
  let _:Prims.unit =
    match (Foo (mk_u32 0) <: t_Foo), (Foo (mk_u32 1) <: t_Foo) <: (t_Foo & t_Foo) with
    | left_val, right_val -> Hax_lib.v_assert (~.(left_val =. right_val <: bool) <: bool)
  in
  let _:Prims.unit =
    match (Foo (mk_u32 2) <: t_Foo), (Foo (mk_u32 2) <: t_Foo) <: (t_Foo & t_Foo) with
    | left_val, right_val -> Hax_lib.v_assert (left_val =. right_val <: bool)
  in
  let bar:t_Foo = Foo (mk_u32 1) <: t_Foo in
  let _:Prims.unit =
    match bar, (Foo (mk_u32 3) <: t_Foo) <: (t_Foo & t_Foo) with
    | left_val, right_val -> Hax_lib.v_assert (~.(left_val =. right_val <: bool) <: bool)
  in
  let _:Prims.unit =
    if is_true
    then
      let _:Prims.unit =
        match (Foo (mk_u32 0) <: t_Foo), (Foo (mk_u32 4) <: t_Foo) <: (t_Foo & t_Foo) with
        | left_val, right_val -> Hax_lib.v_assert (~.(left_val =. right_val <: bool) <: bool)
      in
      ()
    else
      let _:Prims.unit =
        match (Foo (mk_u32 3) <: t_Foo), (Foo (mk_u32 3) <: t_Foo) <: (t_Foo & t_Foo) with
        | left_val, right_val -> Hax_lib.v_assert (left_val =. right_val <: bool)
      in
      ()
  in
  let _:Prims.unit =
    if is_true
    then
      let _:Prims.unit =
        match (Foo (mk_u32 0) <: t_Foo), (Foo (mk_u32 4) <: t_Foo) <: (t_Foo & t_Foo) with
        | left_val, right_val -> Hax_lib.v_assert (~.(left_val =. right_val <: bool) <: bool)
      in
      ()
    else
      let _:Prims.unit =
        match (Foo (mk_u32 3) <: t_Foo), (Foo (mk_u32 3) <: t_Foo) <: (t_Foo & t_Foo) with
        | left_val, right_val -> Hax_lib.v_assert (left_val =. right_val <: bool)
      in
      ()
  in
  let _:Prims.unit =
    match
      (if is_true then Foo (mk_u32 0) <: t_Foo else Foo (mk_u32 1) <: t_Foo),
      (Foo (mk_u32 5) <: t_Foo)
      <:
      (t_Foo & t_Foo)
    with
    | left_val, right_val -> Hax_lib.v_assert (~.(left_val =. right_val <: bool) <: bool)
  in
  let _:Prims.unit =
    match
      (Foo (mk_u32 5) <: t_Foo),
      (if is_true then Foo (mk_u32 0) <: t_Foo else Foo (mk_u32 1) <: t_Foo)
      <:
      (t_Foo & t_Foo)
    with
    | left_val, right_val -> Hax_lib.v_assert (~.(left_val =. right_val <: bool) <: bool)
  in
  let _:Prims.unit =
    match
      (if is_true
        then
          let _:Prims.unit =
            match (Foo (mk_u32 3) <: t_Foo), (Foo (mk_u32 3) <: t_Foo) <: (t_Foo & t_Foo) with
            | left_val, right_val -> Hax_lib.v_assert (left_val =. right_val <: bool)
          in
          Foo (mk_u32 0) <: t_Foo
        else
          let _:Prims.unit =
            match
              (if is_true then Foo (mk_u32 0) <: t_Foo else Foo (mk_u32 1) <: t_Foo),
              (Foo (mk_u32 5) <: t_Foo)
              <:
              (t_Foo & t_Foo)
            with
            | left_val, right_val -> Hax_lib.v_assert (~.(left_val =. right_val <: bool) <: bool)
          in
          Foo (mk_u32 1) <: t_Foo),
      (Foo (mk_u32 5) <: t_Foo)
      <:
      (t_Foo & t_Foo)
    with
    | left_val, right_val -> Hax_lib.v_assert (~.(left_val =. right_val <: bool) <: bool)
  in
  let _:Prims.unit =
    match (Foo (mk_u32 1) <: t_Foo), (Foo (mk_u32 3) <: t_Foo) <: (t_Foo & t_Foo) with
    | left_val, right_val -> Hax_lib.v_assert (left_val =. right_val <: bool)
  in
  let _:Prims.unit =
    match (Foo (mk_u32 3) <: t_Foo), (Foo (mk_u32 3) <: t_Foo) <: (t_Foo & t_Foo) with
    | left_val, right_val -> Hax_lib.v_assert (left_val =. right_val <: bool)
  in
  ()

(* item error backend: something is not implemented yet.
Mutable static items are not supported.

This is discussed in issue https://github.com/pereq-dev/hax/issues/1343.
Please upvote or comment this issue if you see this error message.
Note: the error was labeled with context `AST import`.


Last AST:
/** print_rust: pitem: not implemented  (item: { Concrete_ident.T.def_id =
  { Explicit_def_id.T.is_constructor = false;
    def_id =
    { Types.index = (0, 0, None); is_local = true;
      kind =
      Types.Static {mutability = true; nested = false; safety = Types.Safe};
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
                     (Types.TypeNs "rustc_coverage__issue_84561");
                     disambiguator = 0 }
                    ]
                  }
                }
              });
      path =
      [{ Types.data = (Types.TypeNs "rustc_coverage__issue_84561");
         disambiguator = 0 };
        { Types.data = (Types.ValueNs "DEBUG_LEVEL_ENABLED");
          disambiguator = 0 }
        ]
      }
    };
  moved = None; suffix = None }) */
const _: () = ();
 *)

/// @fail(extraction): ssprove(HAX0002, HAX0008, HAX0008, HAX0008), coq(HAX0008, HAX0008, HAX0008, HAX0002), fstar(HAX0002, HAX0002, HAX0008, HAX0008, HAX0008), proverif(HAX0002, HAX0008, HAX0008, HAX0008), legacy-lean(HAX0002, HAX0002, HAX0008, HAX0008, HAX0008)
let test1 (_: Prims.unit) : Prims.unit =
  let _:Prims.unit =
    if
      Rust_primitives.Hax.failure "Fatal error: something we considered as impossible occurred! Please report this by submitting an issue on GitHub!\nDetails: expected an arrow type here\n\nNote: the error was labeled with context `DirectAndMut`.\n"
        "rust_primitives::hax::failure(\n \"Explicit rejection by a phase in the Hax engine:\na node of kind [Raw_pointer] have been found in the AST\n\nNote: the error was labeled with context `reject_RawOrMutP..."

    then
      let _:Prims.unit =
        Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
              (let list = ["debug is enabled\n"] in
                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                Rust_primitives.Hax.array_of_list 1 list)
            <:
            Core_models.Fmt.t_Arguments)
      in
      let _:Prims.unit = () in
      ()
  in
  let _:Prims.unit =
    if
      Rust_primitives.Hax.failure "Fatal error: something we considered as impossible occurred! Please report this by submitting an issue on GitHub!\nDetails: expected an arrow type here\n\nNote: the error was labeled with context `DirectAndMut`.\n"
        "rust_primitives::hax::failure(\n \"Explicit rejection by a phase in the Hax engine:\na node of kind [Raw_pointer] have been found in the AST\n\nNote: the error was labeled with context `reject_RawOrMutP..."

    then
      let _:Prims.unit =
        Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
              (let list = ["debug is enabled\n"] in
                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                Rust_primitives.Hax.array_of_list 1 list)
            <:
            Core_models.Fmt.t_Arguments)
      in
      let _:Prims.unit = () in
      ()
  in
  let _:i32 = mk_i32 0 in
  let _:Prims.unit =
    if
      Rust_primitives.Hax.failure "Fatal error: something we considered as impossible occurred! Please report this by submitting an issue on GitHub!\nDetails: expected an arrow type here\n\nNote: the error was labeled with context `DirectAndMut`.\n"
        "rust_primitives::hax::failure(\n \"Explicit rejection by a phase in the Hax engine:\na node of kind [Raw_pointer] have been found in the AST\n\nNote: the error was labeled with context `reject_RawOrMutP..."

    then
      let _:Prims.unit =
        Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
              (let list = ["debug is enabled\n"] in
                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                Rust_primitives.Hax.array_of_list 1 list)
            <:
            Core_models.Fmt.t_Arguments)
      in
      let _:Prims.unit = () in
      ()
  in
  let _:Prims.unit =
    Rust_primitives.Hax.failure "Explicit rejection by a phase in the Hax engine:\na node of kind [Arbitrary_lhs] have been found in the AST\n\nNote: the error was labeled with context `reject_ArbitraryLhs`.\n"
      "(rust_primitives::hax::failure(\n \"Fatal error: something we considered as impossible occurred! Please report this by submitting an issue on GitHub!\nDetails: expected an arrow type here\n\nNote: the e..."

  in
  let _:Prims.unit = () in
  let _:Prims.unit =
    if
      Rust_primitives.Hax.failure "Fatal error: something we considered as impossible occurred! Please report this by submitting an issue on GitHub!\nDetails: expected an arrow type here\n\nNote: the error was labeled with context `DirectAndMut`.\n"
        "rust_primitives::hax::failure(\n \"Explicit rejection by a phase in the Hax engine:\na node of kind [Raw_pointer] have been found in the AST\n\nNote: the error was labeled with context `reject_RawOrMutP..."

    then
      let _:Prims.unit =
        Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
              (let list = ["debug is enabled\n"] in
                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                Rust_primitives.Hax.array_of_list 1 list)
            <:
            Core_models.Fmt.t_Arguments)
      in
      let _:Prims.unit = () in
      ()
  in
  ()

/// @fail(extraction): ssprove(HAX0001)
let test2__call_print (s: string) : Prims.unit =
  let args:string = s <: string in
  let args:t_Array Core_models.Fmt.Rt.t_Argument (mk_usize 1) =
    let list = [Core_models.Fmt.Rt.impl__new_display #string args] in
    FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
    Rust_primitives.Hax.array_of_list 1 list
  in
  let _:Prims.unit =
    Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_v1 (mk_usize 1)
          (mk_usize 1)
          (let list = [""] in
            FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
            Rust_primitives.Hax.array_of_list 1 list)
          args
        <:
        Core_models.Fmt.t_Arguments)
  in
  let _:Prims.unit = () in
  ()

/// @fail(extraction): proverif(HAX0008), ssprove(HAX0008), coq(HAX0008), legacy-lean(HAX0002, HAX0008), fstar(HAX0002, HAX0008)
let test2 (_: Prims.unit) : Prims.unit =
  let _:Prims.unit = test2__call_print "called from call_debug: " in
  let _:Prims.unit =
    if
      Rust_primitives.Hax.failure "Fatal error: something we considered as impossible occurred! Please report this by submitting an issue on GitHub!\nDetails: expected an arrow type here\n\nNote: the error was labeled with context `DirectAndMut`.\n"
        "rust_primitives::hax::failure(\n \"Explicit rejection by a phase in the Hax engine:\na node of kind [Raw_pointer] have been found in the AST\n\nNote: the error was labeled with context `reject_RawOrMutP..."

    then
      let _:Prims.unit =
        Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
              (let list = ["debug is enabled\n"] in
                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                Rust_primitives.Hax.array_of_list 1 list)
            <:
            Core_models.Fmt.t_Arguments)
      in
      let _:Prims.unit = () in
      ()
  in
  ()

let main (_: Prims.unit) : Prims.unit =
  let _:Prims.unit = test1 () in
  let _:Prims.unit = test2 () in
  let _:Prims.unit = test3 () in
  ()
