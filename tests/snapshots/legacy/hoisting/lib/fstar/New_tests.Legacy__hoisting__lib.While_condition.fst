module New_tests.Legacy__hoisting__lib.While_condition
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

/// @fail(extraction): coq(HAX0001, HAX0001), ssprove(HAX0001), proverif(HAX0008)
let local_let (_: Prims.unit) : u8 =
  let n:u8 = mk_u8 0 in
  let n:u8 =
    Rust_primitives.Hax.while_loop (fun n ->
          let n:u8 = n in
          true)
      (fun n ->
          let n:u8 = n in
          let y:u8 = Core_models.Num.impl_u8__wrapping_add n (mk_u8 1) in
          y <. mk_u8 4)
      (fun n ->
          let n:u8 = n in
          Rust_primitives.Hax.Int.from_machine (mk_u32 0) <: Hax_lib.Int.t_Int)
      n
      (fun n ->
          let n:u8 = n in
          let n:u8 = n +! mk_u8 1 in
          n)
  in
  n

/// @fail(extraction): coq(HAX0001, HAX0001), ssprove(HAX0001), proverif(HAX0008)
let local_mut (_: Prims.unit) : u8 =
  let n:u8 = mk_u8 0 in
  let n:u8 =
    Rust_primitives.Hax.while_loop (fun n ->
          let n:u8 = n in
          true)
      (fun n ->
          let n:u8 = n in
          let y:u8 = n in
          let y:u8 = Core_models.Num.impl_u8__wrapping_add y (mk_u8 1) in
          y <. mk_u8 4)
      (fun n ->
          let n:u8 = n in
          Rust_primitives.Hax.Int.from_machine (mk_u32 0) <: Hax_lib.Int.t_Int)
      n
      (fun n ->
          let n:u8 = n in
          let n:u8 = n +! mk_u8 1 in
          n)
  in
  n

/// @fail(extraction): coq(HAX0001, HAX0001, HAX0008), ssprove(HAX0008, HAX0001), proverif(HAX0008, HAX0008)
let inner_loop (n: u8) : u8 =
  let i:u8 = mk_u8 0 in
  Rust_primitives.Hax.while_loop (fun i ->
        let i:u8 = i in
        true)
    (fun i ->
        let i:u8 = i in
        let k:u8 = mk_u8 0 in
        let k:u8 =
          Rust_primitives.Hax.Folds.fold_range (mk_u8 0)
            i
            (fun k temp_1_ ->
                let k:u8 = k in
                let _:u8 = temp_1_ in
                true)
            k
            (fun k z ->
                let k:u8 = k in
                let z:u8 = z in
                if (z %! mk_u8 2 <: u8) =. mk_u8 0 <: bool
                then k
                else Core_models.Num.impl_u8__wrapping_add k (mk_u8 1) <: u8)
        in
        k <. n)
    (fun i ->
        let i:u8 = i in
        Rust_primitives.Hax.Int.from_machine (mk_u32 0) <: Hax_lib.Int.t_Int)
    i
    (fun i ->
        let i:u8 = i in
        let i:u8 = Core_models.Num.impl_u8__wrapping_add i (mk_u8 1) in
        i)

let dec (x: u8) : (u8 & bool) =
  let (x: u8), (hax_temp_output: bool) =
    if x =. mk_u8 0
    then x, false <: (u8 & bool)
    else
      let x:u8 = x -! mk_u8 1 in
      x, true <: (u8 & bool)
  in
  x, hax_temp_output <: (u8 & bool)

(* item error backend: something is not implemented yet.
Side effects in the condition of a while loop are not supported

This is discussed in issue https://github.com/hacspec/hax/issues/2337.
Please upvote or comment this issue if you see this error message.
Note: the error was labeled with context `HoistSideEffects`.

Last available AST for this item:

/// @fail(extraction): fstar(HAX0001), legacy-lean(HAX0001), coq(HAX0001, HAX0001), ssprove(HAX0001), proverif(HAX0008)
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
fn mut_ref(_: tuple0) -> int {
    {
        let mut x: int = { 3 };
        {
            let mut n: int = { 0 };
            {
                let _: tuple0 = {
                    {
                        while {
                            let Tuple2(tmp0, out): tuple2<int, bool> = {
                                new_tests::legacy__hoisting__lib::while_condition::dec(x)
                            };
                            {
                                let _: tuple0 = { (x = tmp0) };
                                out
                            }
                        } {
                            {
                                let _: tuple0 = {
                                    (n = rust_primitives::hax::machine_int::add(n, 1))
                                };
                                Tuple0
                            }
                        }
                    }
                };
                n
            }
        }
    }
}


Last AST:
/** print_rust: pitem: not implemented  (item: { Concrete_ident.T.def_id =
  { Explicit_def_id.T.is_constructor = false;
    def_id =
    { Types.index = (0, 0, None); is_local = true; kind = Types.Fn;
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
                              parent =
                              (Some { Types.contents =
                                      { Types.id = 0;
                                        value =
                                        { Types.index = (0, 0, None);
                                          is_local = true; kind = Types.Mod;
                                          krate = "new_tests"; parent = None;
                                          path = [] }
                                        }
                                      });
                              path =
                              [{ Types.data =
                                 (Types.TypeNs "legacy__hoisting__lib");
                                 disambiguator = 0 }
                                ]
                              }
                            }
                          });
                  path =
                  [{ Types.data = (Types.TypeNs "legacy__hoisting__lib");
                     disambiguator = 0 };
                    { Types.data = (Types.TypeNs "while_condition");
                      disambiguator = 0 }
                    ]
                  }
                }
              });
      path =
      [{ Types.data = (Types.TypeNs "legacy__hoisting__lib");
         disambiguator = 0 };
        { Types.data = (Types.TypeNs "while_condition"); disambiguator = 0 };
        { Types.data = (Types.ValueNs "mut_ref"); disambiguator = 0 }]
      }
    };
  moved = None; suffix = None }) */
const _: () = ();
 *)

(* item error backend: something is not implemented yet.
Side effects in the condition of a while loop are not supported

This is discussed in issue https://github.com/hacspec/hax/issues/2337.
Please upvote or comment this issue if you see this error message.
Note: the error was labeled with context `HoistSideEffects`.

Last available AST for this item:

/// @fail(extraction): fstar(HAX0001), legacy-lean(HAX0001), coq(HAX0001, HAX0001), ssprove(HAX0001), proverif(HAX0008)
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
fn assign(_: tuple0) -> int {
    {
        let mut n: int = { 0 };
        {
            let _: tuple0 = {
                {
                    while {
                        let _: tuple0 = {
                            (n = rust_primitives::hax::machine_int::add(n, 1))
                        };
                        rust_primitives::hax::machine_int::lt(n, 4)
                    } {
                        Tuple0
                    }
                }
            };
            n
        }
    }
}


Last AST:
/** print_rust: pitem: not implemented  (item: { Concrete_ident.T.def_id =
  { Explicit_def_id.T.is_constructor = false;
    def_id =
    { Types.index = (0, 0, None); is_local = true; kind = Types.Fn;
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
                              parent =
                              (Some { Types.contents =
                                      { Types.id = 0;
                                        value =
                                        { Types.index = (0, 0, None);
                                          is_local = true; kind = Types.Mod;
                                          krate = "new_tests"; parent = None;
                                          path = [] }
                                        }
                                      });
                              path =
                              [{ Types.data =
                                 (Types.TypeNs "legacy__hoisting__lib");
                                 disambiguator = 0 }
                                ]
                              }
                            }
                          });
                  path =
                  [{ Types.data = (Types.TypeNs "legacy__hoisting__lib");
                     disambiguator = 0 };
                    { Types.data = (Types.TypeNs "while_condition");
                      disambiguator = 0 }
                    ]
                  }
                }
              });
      path =
      [{ Types.data = (Types.TypeNs "legacy__hoisting__lib");
         disambiguator = 0 };
        { Types.data = (Types.TypeNs "while_condition"); disambiguator = 0 };
        { Types.data = (Types.ValueNs "assign"); disambiguator = 0 }]
      }
    };
  moved = None; suffix = None }) */
const _: () = ();
 *)

let fallible (b: bool) : Core_models.Result.t_Result bool u8 =
  if b
  then Core_models.Result.Result_Ok true <: Core_models.Result.t_Result bool u8
  else Core_models.Result.Result_Err (mk_u8 1) <: Core_models.Result.t_Result bool u8

(* item error backend: something is not implemented yet.
Side effects in the condition of a while loop are not supported

This is discussed in issue https://github.com/hacspec/hax/issues/2337.
Please upvote or comment this issue if you see this error message.
Note: the error was labeled with context `HoistSideEffects`.

Last available AST for this item:

/// @fail(extraction): fstar(HAX0001), legacy-lean(HAX0001), coq(HAX0001, HAX0001), ssprove(HAX0001), proverif(HAX0008)
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
fn question_mark(_: tuple0) -> core_models::result::t_Result<int, int> {
    {
        let mut n: int = { 0 };
        {
            let _: tuple0 = {
                {
                    while (match (new_tests::legacy__hoisting__lib::while_condition::fallible(
                        rust_primitives::hax::machine_int::lt(n, 4),
                    )) {
                        core_models::result::Result_Ok(ok) => ok,
                        core_models::result::Result_Err(err) => {
                            (return core_models::result::Result_Err(err))
                        }
                    }) {
                        {
                            let _: tuple0 = {
                                (n = rust_primitives::hax::machine_int::add(n, 1))
                            };
                            Tuple0
                        }
                    }
                }
            };
            core_models::result::Result_Ok(n)
        }
    }
}


Last AST:
/** print_rust: pitem: not implemented  (item: { Concrete_ident.T.def_id =
  { Explicit_def_id.T.is_constructor = false;
    def_id =
    { Types.index = (0, 0, None); is_local = true; kind = Types.Fn;
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
                              parent =
                              (Some { Types.contents =
                                      { Types.id = 0;
                                        value =
                                        { Types.index = (0, 0, None);
                                          is_local = true; kind = Types.Mod;
                                          krate = "new_tests"; parent = None;
                                          path = [] }
                                        }
                                      });
                              path =
                              [{ Types.data =
                                 (Types.TypeNs "legacy__hoisting__lib");
                                 disambiguator = 0 }
                                ]
                              }
                            }
                          });
                  path =
                  [{ Types.data = (Types.TypeNs "legacy__hoisting__lib");
                     disambiguator = 0 };
                    { Types.data = (Types.TypeNs "while_condition");
                      disambiguator = 0 }
                    ]
                  }
                }
              });
      path =
      [{ Types.data = (Types.TypeNs "legacy__hoisting__lib");
         disambiguator = 0 };
        { Types.data = (Types.TypeNs "while_condition"); disambiguator = 0 };
        { Types.data = (Types.ValueNs "question_mark"); disambiguator = 0 }]
      }
    };
  moved = None; suffix = None }) */
const _: () = ();
 *)

(* item error backend: something is not implemented yet.
Side effects in the condition of a while loop are not supported

This is discussed in issue https://github.com/hacspec/hax/issues/2337.
Please upvote or comment this issue if you see this error message.
Note: the error was labeled with context `HoistSideEffects`.

Last available AST for this item:

/// @fail(extraction): fstar(HAX0001), legacy-lean(HAX0001), coq(HAX0001, HAX0001), ssprove(HAX0001), proverif(HAX0008)
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
fn cclosure_return(n: int) -> int {
    {
        let mut i: int = { 0 };
        {
            let _: tuple0 = {
                {
                    while {
                        let c: arrow!(int -> bool) = {
                            (|x| {
                                {
                                    let _: tuple0 = {
                                        (if rust_primitives::hax::machine_int::eq(x, 3) {
                                            rust_primitives::hax::never_to_any((return false))
                                        })
                                    };
                                    true
                                }
                            })
                        };
                        BinOp::Ast
                            .And(
                                core_models::ops::function::f_call(c, Tuple1(i)),
                                rust_primitives::hax::machine_int::lt(i, n),
                            )
                    } {
                        {
                            let _: tuple0 = {
                                (i = rust_primitives::hax::machine_int::add(i, 1))
                            };
                            Tuple0
                        }
                    }
                }
            };
            i
        }
    }
}


Last AST:
/** print_rust: pitem: not implemented  (item: { Concrete_ident.T.def_id =
  { Explicit_def_id.T.is_constructor = false;
    def_id =
    { Types.index = (0, 0, None); is_local = true; kind = Types.Fn;
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
                              parent =
                              (Some { Types.contents =
                                      { Types.id = 0;
                                        value =
                                        { Types.index = (0, 0, None);
                                          is_local = true; kind = Types.Mod;
                                          krate = "new_tests"; parent = None;
                                          path = [] }
                                        }
                                      });
                              path =
                              [{ Types.data =
                                 (Types.TypeNs "legacy__hoisting__lib");
                                 disambiguator = 0 }
                                ]
                              }
                            }
                          });
                  path =
                  [{ Types.data = (Types.TypeNs "legacy__hoisting__lib");
                     disambiguator = 0 };
                    { Types.data = (Types.TypeNs "while_condition");
                      disambiguator = 0 }
                    ]
                  }
                }
              });
      path =
      [{ Types.data = (Types.TypeNs "legacy__hoisting__lib");
         disambiguator = 0 };
        { Types.data = (Types.TypeNs "while_condition"); disambiguator = 0 };
        { Types.data = (Types.ValueNs "closure_return"); disambiguator = 0 }]
      }
    };
  moved = None; suffix = None }) */
const _: () = ();
 *)
