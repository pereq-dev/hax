module New_tests.Rustc_coverage__loops_branches
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

type t_DebugTest = | DebugTest : t_DebugTest

/// @fail(extraction): coq(HAX0001, HAX0001, HAX0001, HAX0001), ssprove(HAX0001, HAX0001)
[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl: Core_models.Fmt.t_Debug t_DebugTest =
  {
    f_fmt_pre = (fun (self: t_DebugTest) (f: Core_models.Fmt.t_Formatter) -> true);
    f_fmt_post
    =
    (fun
        (self: t_DebugTest)
        (f: Core_models.Fmt.t_Formatter)
        (out1:
          (Core_models.Fmt.t_Formatter &
            Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error))
        ->
        true);
    f_fmt
    =
    fun (self: t_DebugTest) (f: Core_models.Fmt.t_Formatter) ->
      if true
      then
        let _:Prims.unit =
          if false
          then
            Rust_primitives.Hax.while_loop (fun temp_0_ ->
                  let _:Prims.unit = temp_0_ in
                  true)
              (fun temp_0_ ->
                  let _:Prims.unit = temp_0_ in
                  true)
              (fun temp_0_ ->
                  let _:Prims.unit = temp_0_ in
                  Rust_primitives.Hax.Int.from_machine (mk_u32 0) <: Hax_lib.Int.t_Int)
              ()
              (fun temp_0_ ->
                  let _:Prims.unit = temp_0_ in
                  ())
        in
        let
        (tmp0: Core_models.Fmt.t_Formatter),
        (out: Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error) =
          Core_models.Fmt.impl_11__write_fmt f
            (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
                (let list = ["cool"] in
                  FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                  Rust_primitives.Hax.array_of_list 1 list)
              <:
              Core_models.Fmt.t_Arguments)
        in
        let f:Core_models.Fmt.t_Formatter = tmp0 in
        match out <: Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error with
        | Core_models.Result.Result_Ok _ ->
          (match
              Rust_primitives.Hax.Folds.fold_range_return (mk_i32 0)
                (mk_i32 10)
                (fun f temp_1_ ->
                    let f:Core_models.Fmt.t_Formatter = f in
                    let _:i32 = temp_1_ in
                    true)
                f
                (fun f i ->
                    let f:Core_models.Fmt.t_Formatter = f in
                    let i:i32 = i in
                    if true
                    then
                      let _:Prims.unit =
                        if false
                        then
                          Rust_primitives.Hax.while_loop (fun temp_0_ ->
                                let _:Prims.unit = temp_0_ in
                                true)
                            (fun temp_0_ ->
                                let _:Prims.unit = temp_0_ in
                                true)
                            (fun temp_0_ ->
                                let _:Prims.unit = temp_0_ in
                                Rust_primitives.Hax.Int.from_machine (mk_u32 0) <: Hax_lib.Int.t_Int
                            )
                            ()
                            (fun temp_0_ ->
                                let _:Prims.unit = temp_0_ in
                                ())
                      in
                      let
                      (tmp0: Core_models.Fmt.t_Formatter),
                      (out: Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error) =
                        Core_models.Fmt.impl_11__write_fmt f
                          (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
                              (let list = ["cool"] in
                                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                                Rust_primitives.Hax.array_of_list 1 list)
                            <:
                            Core_models.Fmt.t_Arguments)
                      in
                      let f:Core_models.Fmt.t_Formatter = tmp0 in
                      match
                        out <: Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error
                      with
                      | Core_models.Result.Result_Ok _ ->
                        Core_models.Ops.Control_flow.ControlFlow_Continue f
                        <:
                        Core_models.Ops.Control_flow.t_ControlFlow
                          (Core_models.Ops.Control_flow.t_ControlFlow
                              (Core_models.Fmt.t_Formatter &
                                Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
                              (Prims.unit & Core_models.Fmt.t_Formatter))
                          Core_models.Fmt.t_Formatter
                      | Core_models.Result.Result_Err err ->
                        Core_models.Ops.Control_flow.ControlFlow_Break
                        (Core_models.Ops.Control_flow.ControlFlow_Break
                          (f,
                            (Core_models.Result.Result_Err err
                              <:
                              Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
                            <:
                            (Core_models.Fmt.t_Formatter &
                              Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error))
                          <:
                          Core_models.Ops.Control_flow.t_ControlFlow
                            (Core_models.Fmt.t_Formatter &
                              Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
                            (Prims.unit & Core_models.Fmt.t_Formatter))
                        <:
                        Core_models.Ops.Control_flow.t_ControlFlow
                          (Core_models.Ops.Control_flow.t_ControlFlow
                              (Core_models.Fmt.t_Formatter &
                                Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
                              (Prims.unit & Core_models.Fmt.t_Formatter))
                          Core_models.Fmt.t_Formatter
                    else
                      Core_models.Ops.Control_flow.ControlFlow_Continue f
                      <:
                      Core_models.Ops.Control_flow.t_ControlFlow
                        (Core_models.Ops.Control_flow.t_ControlFlow
                            (Core_models.Fmt.t_Formatter &
                              Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
                            (Prims.unit & Core_models.Fmt.t_Formatter)) Core_models.Fmt.t_Formatter)
              <:
              Core_models.Ops.Control_flow.t_ControlFlow
                (Core_models.Fmt.t_Formatter &
                  Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
                Core_models.Fmt.t_Formatter
            with
            | Core_models.Ops.Control_flow.ControlFlow_Break ret -> ret
            | Core_models.Ops.Control_flow.ControlFlow_Continue f ->
              let hax_temp_output:Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error =
                Core_models.Result.Result_Ok (() <: Prims.unit)
                <:
                Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error
              in
              f, hax_temp_output
              <:
              (Core_models.Fmt.t_Formatter &
                Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error))
        | Core_models.Result.Result_Err err ->
          f,
          (Core_models.Result.Result_Err err
            <:
            Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
          <:
          (Core_models.Fmt.t_Formatter &
            Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
      else
        match
          Rust_primitives.Hax.Folds.fold_range_return (mk_i32 0)
            (mk_i32 10)
            (fun f temp_1_ ->
                let f:Core_models.Fmt.t_Formatter = f in
                let _:i32 = temp_1_ in
                true)
            f
            (fun f i ->
                let f:Core_models.Fmt.t_Formatter = f in
                let i:i32 = i in
                if true
                then
                  let _:Prims.unit =
                    if false
                    then
                      Rust_primitives.Hax.while_loop (fun temp_0_ ->
                            let _:Prims.unit = temp_0_ in
                            true)
                        (fun temp_0_ ->
                            let _:Prims.unit = temp_0_ in
                            true)
                        (fun temp_0_ ->
                            let _:Prims.unit = temp_0_ in
                            Rust_primitives.Hax.Int.from_machine (mk_u32 0) <: Hax_lib.Int.t_Int)
                        ()
                        (fun temp_0_ ->
                            let _:Prims.unit = temp_0_ in
                            ())
                  in
                  let
                  (tmp0: Core_models.Fmt.t_Formatter),
                  (out: Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error) =
                    Core_models.Fmt.impl_11__write_fmt f
                      (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
                          (let list = ["cool"] in
                            FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                            Rust_primitives.Hax.array_of_list 1 list)
                        <:
                        Core_models.Fmt.t_Arguments)
                  in
                  let f:Core_models.Fmt.t_Formatter = tmp0 in
                  match out <: Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error with
                  | Core_models.Result.Result_Ok _ ->
                    Core_models.Ops.Control_flow.ControlFlow_Continue f
                    <:
                    Core_models.Ops.Control_flow.t_ControlFlow
                      (Core_models.Ops.Control_flow.t_ControlFlow
                          (Core_models.Fmt.t_Formatter &
                            Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
                          (Prims.unit & Core_models.Fmt.t_Formatter)) Core_models.Fmt.t_Formatter
                  | Core_models.Result.Result_Err err ->
                    Core_models.Ops.Control_flow.ControlFlow_Break
                    (Core_models.Ops.Control_flow.ControlFlow_Break
                      (f,
                        (Core_models.Result.Result_Err err
                          <:
                          Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
                        <:
                        (Core_models.Fmt.t_Formatter &
                          Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error))
                      <:
                      Core_models.Ops.Control_flow.t_ControlFlow
                        (Core_models.Fmt.t_Formatter &
                          Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
                        (Prims.unit & Core_models.Fmt.t_Formatter))
                    <:
                    Core_models.Ops.Control_flow.t_ControlFlow
                      (Core_models.Ops.Control_flow.t_ControlFlow
                          (Core_models.Fmt.t_Formatter &
                            Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
                          (Prims.unit & Core_models.Fmt.t_Formatter)) Core_models.Fmt.t_Formatter
                else
                  Core_models.Ops.Control_flow.ControlFlow_Continue f
                  <:
                  Core_models.Ops.Control_flow.t_ControlFlow
                    (Core_models.Ops.Control_flow.t_ControlFlow
                        (Core_models.Fmt.t_Formatter &
                          Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
                        (Prims.unit & Core_models.Fmt.t_Formatter)) Core_models.Fmt.t_Formatter)
          <:
          Core_models.Ops.Control_flow.t_ControlFlow
            (Core_models.Fmt.t_Formatter &
              Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
            Core_models.Fmt.t_Formatter
        with
        | Core_models.Ops.Control_flow.ControlFlow_Break ret -> ret
        | Core_models.Ops.Control_flow.ControlFlow_Continue f ->
          let hax_temp_output:Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error =
            Core_models.Result.Result_Ok (() <: Prims.unit)
            <:
            Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error
          in
          f, hax_temp_output
          <:
          (Core_models.Fmt.t_Formatter &
            Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
  }

type t_DisplayTest = | DisplayTest : t_DisplayTest

/// @fail(extraction): coq(HAX0001, HAX0001, HAX0001, HAX0001), ssprove(HAX0001, HAX0001)
[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_1: Core_models.Fmt.t_Display t_DisplayTest =
  {
    f_fmt_pre = (fun (self: t_DisplayTest) (f: Core_models.Fmt.t_Formatter) -> true);
    f_fmt_post
    =
    (fun
        (self: t_DisplayTest)
        (f: Core_models.Fmt.t_Formatter)
        (out1:
          (Core_models.Fmt.t_Formatter &
            Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error))
        ->
        true);
    f_fmt
    =
    fun (self: t_DisplayTest) (f: Core_models.Fmt.t_Formatter) ->
      if false
      then
        match
          Rust_primitives.Hax.Folds.fold_range_return (mk_i32 0)
            (mk_i32 10)
            (fun f temp_1_ ->
                let f:Core_models.Fmt.t_Formatter = f in
                let _:i32 = temp_1_ in
                true)
            f
            (fun f i ->
                let f:Core_models.Fmt.t_Formatter = f in
                let i:i32 = i in
                if false
                then
                  Core_models.Ops.Control_flow.ControlFlow_Continue f
                  <:
                  Core_models.Ops.Control_flow.t_ControlFlow
                    (Core_models.Ops.Control_flow.t_ControlFlow
                        (Core_models.Fmt.t_Formatter &
                          Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
                        (Prims.unit & Core_models.Fmt.t_Formatter)) Core_models.Fmt.t_Formatter
                else
                  let _:Prims.unit =
                    if false
                    then
                      Rust_primitives.Hax.while_loop (fun temp_0_ ->
                            let _:Prims.unit = temp_0_ in
                            true)
                        (fun temp_0_ ->
                            let _:Prims.unit = temp_0_ in
                            true)
                        (fun temp_0_ ->
                            let _:Prims.unit = temp_0_ in
                            Rust_primitives.Hax.Int.from_machine (mk_u32 0) <: Hax_lib.Int.t_Int)
                        ()
                        (fun temp_0_ ->
                            let _:Prims.unit = temp_0_ in
                            ())
                  in
                  let
                  (tmp0: Core_models.Fmt.t_Formatter),
                  (out: Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error) =
                    Core_models.Fmt.impl_11__write_fmt f
                      (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
                          (let list = ["cool"] in
                            FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                            Rust_primitives.Hax.array_of_list 1 list)
                        <:
                        Core_models.Fmt.t_Arguments)
                  in
                  let f:Core_models.Fmt.t_Formatter = tmp0 in
                  match out <: Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error with
                  | Core_models.Result.Result_Ok _ ->
                    Core_models.Ops.Control_flow.ControlFlow_Continue f
                    <:
                    Core_models.Ops.Control_flow.t_ControlFlow
                      (Core_models.Ops.Control_flow.t_ControlFlow
                          (Core_models.Fmt.t_Formatter &
                            Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
                          (Prims.unit & Core_models.Fmt.t_Formatter)) Core_models.Fmt.t_Formatter
                  | Core_models.Result.Result_Err err ->
                    Core_models.Ops.Control_flow.ControlFlow_Break
                    (Core_models.Ops.Control_flow.ControlFlow_Break
                      (f,
                        (Core_models.Result.Result_Err err
                          <:
                          Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
                        <:
                        (Core_models.Fmt.t_Formatter &
                          Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error))
                      <:
                      Core_models.Ops.Control_flow.t_ControlFlow
                        (Core_models.Fmt.t_Formatter &
                          Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
                        (Prims.unit & Core_models.Fmt.t_Formatter))
                    <:
                    Core_models.Ops.Control_flow.t_ControlFlow
                      (Core_models.Ops.Control_flow.t_ControlFlow
                          (Core_models.Fmt.t_Formatter &
                            Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
                          (Prims.unit & Core_models.Fmt.t_Formatter)) Core_models.Fmt.t_Formatter)
          <:
          Core_models.Ops.Control_flow.t_ControlFlow
            (Core_models.Fmt.t_Formatter &
              Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
            Core_models.Fmt.t_Formatter
        with
        | Core_models.Ops.Control_flow.ControlFlow_Break ret -> ret
        | Core_models.Ops.Control_flow.ControlFlow_Continue f ->
          let hax_temp_output:Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error =
            Core_models.Result.Result_Ok (() <: Prims.unit)
            <:
            Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error
          in
          f, hax_temp_output
          <:
          (Core_models.Fmt.t_Formatter &
            Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
      else
        let _:Prims.unit =
          if false
          then
            Rust_primitives.Hax.while_loop (fun temp_0_ ->
                  let _:Prims.unit = temp_0_ in
                  true)
              (fun temp_0_ ->
                  let _:Prims.unit = temp_0_ in
                  true)
              (fun temp_0_ ->
                  let _:Prims.unit = temp_0_ in
                  Rust_primitives.Hax.Int.from_machine (mk_u32 0) <: Hax_lib.Int.t_Int)
              ()
              (fun temp_0_ ->
                  let _:Prims.unit = temp_0_ in
                  ())
        in
        let
        (tmp0: Core_models.Fmt.t_Formatter),
        (out: Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error) =
          Core_models.Fmt.impl_11__write_fmt f
            (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
                (let list = ["cool"] in
                  FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                  Rust_primitives.Hax.array_of_list 1 list)
              <:
              Core_models.Fmt.t_Arguments)
        in
        let f:Core_models.Fmt.t_Formatter = tmp0 in
        match out <: Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error with
        | Core_models.Result.Result_Ok _ ->
          (match
              Rust_primitives.Hax.Folds.fold_range_return (mk_i32 0)
                (mk_i32 10)
                (fun f temp_1_ ->
                    let f:Core_models.Fmt.t_Formatter = f in
                    let _:i32 = temp_1_ in
                    true)
                f
                (fun f i ->
                    let f:Core_models.Fmt.t_Formatter = f in
                    let i:i32 = i in
                    if false
                    then
                      Core_models.Ops.Control_flow.ControlFlow_Continue f
                      <:
                      Core_models.Ops.Control_flow.t_ControlFlow
                        (Core_models.Ops.Control_flow.t_ControlFlow
                            (Core_models.Fmt.t_Formatter &
                              Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
                            (Prims.unit & Core_models.Fmt.t_Formatter)) Core_models.Fmt.t_Formatter
                    else
                      let _:Prims.unit =
                        if false
                        then
                          Rust_primitives.Hax.while_loop (fun temp_0_ ->
                                let _:Prims.unit = temp_0_ in
                                true)
                            (fun temp_0_ ->
                                let _:Prims.unit = temp_0_ in
                                true)
                            (fun temp_0_ ->
                                let _:Prims.unit = temp_0_ in
                                Rust_primitives.Hax.Int.from_machine (mk_u32 0) <: Hax_lib.Int.t_Int
                            )
                            ()
                            (fun temp_0_ ->
                                let _:Prims.unit = temp_0_ in
                                ())
                      in
                      let
                      (tmp0: Core_models.Fmt.t_Formatter),
                      (out: Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error) =
                        Core_models.Fmt.impl_11__write_fmt f
                          (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
                              (let list = ["cool"] in
                                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                                Rust_primitives.Hax.array_of_list 1 list)
                            <:
                            Core_models.Fmt.t_Arguments)
                      in
                      let f:Core_models.Fmt.t_Formatter = tmp0 in
                      match
                        out <: Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error
                      with
                      | Core_models.Result.Result_Ok _ ->
                        Core_models.Ops.Control_flow.ControlFlow_Continue f
                        <:
                        Core_models.Ops.Control_flow.t_ControlFlow
                          (Core_models.Ops.Control_flow.t_ControlFlow
                              (Core_models.Fmt.t_Formatter &
                                Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
                              (Prims.unit & Core_models.Fmt.t_Formatter))
                          Core_models.Fmt.t_Formatter
                      | Core_models.Result.Result_Err err ->
                        Core_models.Ops.Control_flow.ControlFlow_Break
                        (Core_models.Ops.Control_flow.ControlFlow_Break
                          (f,
                            (Core_models.Result.Result_Err err
                              <:
                              Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
                            <:
                            (Core_models.Fmt.t_Formatter &
                              Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error))
                          <:
                          Core_models.Ops.Control_flow.t_ControlFlow
                            (Core_models.Fmt.t_Formatter &
                              Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
                            (Prims.unit & Core_models.Fmt.t_Formatter))
                        <:
                        Core_models.Ops.Control_flow.t_ControlFlow
                          (Core_models.Ops.Control_flow.t_ControlFlow
                              (Core_models.Fmt.t_Formatter &
                                Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
                              (Prims.unit & Core_models.Fmt.t_Formatter))
                          Core_models.Fmt.t_Formatter)
              <:
              Core_models.Ops.Control_flow.t_ControlFlow
                (Core_models.Fmt.t_Formatter &
                  Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
                Core_models.Fmt.t_Formatter
            with
            | Core_models.Ops.Control_flow.ControlFlow_Break ret -> ret
            | Core_models.Ops.Control_flow.ControlFlow_Continue f ->
              let hax_temp_output:Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error =
                Core_models.Result.Result_Ok (() <: Prims.unit)
                <:
                Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error
              in
              f, hax_temp_output
              <:
              (Core_models.Fmt.t_Formatter &
                Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error))
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
let main (_: Prims.unit) : Prims.unit =
  let debug_test:t_DebugTest = DebugTest <: t_DebugTest in
  let args:t_DebugTest = debug_test <: t_DebugTest in
  let args:t_Array Core_models.Fmt.Rt.t_Argument (mk_usize 1) =
    let list = [Core_models.Fmt.Rt.impl__new_debug #t_DebugTest args] in
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
  let display_test:t_DisplayTest = DisplayTest <: t_DisplayTest in
  let args:t_DisplayTest = display_test <: t_DisplayTest in
  let args:t_Array Core_models.Fmt.Rt.t_Argument (mk_usize 1) =
    let list = [Core_models.Fmt.Rt.impl__new_display #t_DisplayTest args] in
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
