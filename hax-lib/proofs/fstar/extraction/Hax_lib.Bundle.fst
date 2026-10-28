module Hax_lib.Bundle
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Hax_lib.Abstraction in
  let open Num_bigint in
  let open Num_bigint.Bigint in
  let open Num_bigint.Bigint.Addition in
  let open Num_bigint.Bigint.Convert in
  let open Num_bigint.Bigint.Division in
  let open Num_bigint.Bigint.Multiplication in
  let open Num_bigint.Bigint.Subtraction in
  let open Num_traits.Cast in
  let open Num_traits.Ops.Euclid in
  ()

/// This function exists only when compiled with `hax`, and is not
/// meant to be used directly. It is called by `assert!` only in
/// appropriate situations.
let v_assert (e_formula: bool) : Prims.unit = ()

/// This function exists only when compiled with `hax`, and is not meant to be
/// used directly. It is called by `assert_prop!` only in appropriate
/// situations.
let assert_prop (e_formula: Hax_lib.Prop.t_Prop) : Prims.unit = ()

/// This function exists only when compiled with `hax`, and is not
/// meant to be used directly. It is called by `assume!` only in
/// appropriate situations.
let v_assume (e_formula: Hax_lib.Prop.t_Prop) : Prims.unit = ()

/// Dummy function that carries a string to be printed as such in the output language
let v_inline (_: string) : Prims.unit = ()

/// Similar to `inline`, but allows for any type. Do not use directly.
let inline_unsafe (#v_T: Type0) (_: string) : v_T =
  Rust_primitives.Hax.never_to_any (Core_models.Panicking.panic "internal error: entered unreachable code"

      <:
      Rust_primitives.Hax.t_Never)

/// Sink for any value into unit. This is used internally by hax to capture
/// value of any type. Specifically, this is useful for the `decreases` clauses
/// for the F* backend.
let any_to_unit (#v_T: Type0) (_: v_T) : Prims.unit =
  Rust_primitives.Hax.never_to_any (Core_models.Panicking.panic "internal error: entered unreachable code"

      <:
      Rust_primitives.Hax.t_Never)

/// A dummy function that holds a loop invariant.
let e_internal_loop_invariant
      (#v_T: Type0)
      (#v_R: Type0)
      (#v_P: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Core_models.Convert.t_Into v_R Hax_lib.Prop.t_Prop)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: Core_models.Ops.Function.t_FnOnce v_P v_T)
      (#_: unit{i1.Core_models.Ops.Function.f_Output == v_R})
      (_: v_P)
    : Prims.unit = ()

/// A dummy function that holds a while loop invariant.
let e_internal_while_loop_invariant (_: Hax_lib.Prop.t_Prop) : Prims.unit = ()

(* item error backend: The mutation of this &mut is not allowed here.

This is discussed in issue https://github.com/pereq-dev/hax/issues/420.
Please upvote or comment this issue if you see this error message.
Note: the error was labeled with context `DirectAndMut`.

Last available AST for this item:

/// A type that implements `Refinement` should be a newtype for a
/// type `T`. The field holding the value of type `T` should be
/// private, and `Refinement` should be the only interface to the
/// type.
/// Please never implement this trait yourself, use the
/// `refinement_type` macro instead.
#[no_std()]
#[feature(register_tool)]
#[register_tool(_hax)]
trait t_Refinement<Self_> {
    /// The base type
    #[no_std()]
    #[feature(register_tool)]
    #[register_tool(_hax)]
    type f_InnerType: TodoPrintRustBoundsTyp;
    /// Smart constructor capturing an invariant. Its extraction will
    /// yield a proof obligation.
    #[no_std()]
    #[feature(register_tool)]
    #[register_tool(_hax)]
    fn f_new(_: proj_asso_type!()) -> Self;
    /// Destructor for the refined type
    #[no_std()]
    #[feature(register_tool)]
    #[register_tool(_hax)]
    fn f_get(_: Self) -> proj_asso_type!();
    /// Gets a mutable reference to a refinement
    #[no_std()]
    #[feature(register_tool)]
    #[register_tool(_hax)]
    fn f_get_mut<Anonymous: 'unk>(_: Self) -> tuple2<Self, &mut proj_asso_type!()>;
    /// Tests wether a value satisfies the refinement
    #[no_std()]
    #[feature(register_tool)]
    #[register_tool(_hax)]
    fn f_invariant(_: proj_asso_type!()) -> hax_lib::prop::t_Prop;
}


Last AST:
/** print_rust: pitem: not implemented  (item: { Concrete_ident.T.def_id =
  { Explicit_def_id.T.is_constructor = false;
    def_id =
    { Types.index = (0, 0, None); is_local = true; kind = Types.Trait;
      krate = "hax_lib";
      parent =
      (Some { Types.contents =
              { Types.id = 0;
                value =
                { Types.index = (0, 0, None); is_local = true;
                  kind = Types.Mod; krate = "hax_lib"; parent = None;
                  path = [] }
                }
              });
      path =
      [{ Types.data = (Types.TypeNs "Refinement"); disambiguator = 0 }] }
    };
  moved =
  (Some { Concrete_ident.Fresh_module.id = 2;
          hints =
          [{ Explicit_def_id.T.is_constructor = false;
             def_id =
             { Types.index = (0, 0, None); is_local = true; kind = Types.Use;
               krate = "hax_lib";
               parent =
               (Some { Types.contents =
                       { Types.id = 0;
                         value =
                         { Types.index = (0, 0, None); is_local = true;
                           kind = Types.Mod; krate = "hax_lib";
                           parent = None; path = [] }
                         }
                       });
               path = [{ Types.data = Types.Use; disambiguator = 0 }] }
             };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.ExternCrate; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent = None; path = [] }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "core"); disambiguator = 0 }] }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Use; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent = None; path = [] }
                          }
                        });
                path = [{ Types.data = Types.Use; disambiguator = 1 }] }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Use; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent = None; path = [] }
                          }
                        });
                path = [{ Types.data = Types.Use; disambiguator = 2 }] }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Use; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent = None; path = [] }
                          }
                        });
                path = [{ Types.data = Types.Use; disambiguator = 3 }] }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Use; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent = None; path = [] }
                          }
                        });
                path = [{ Types.data = Types.Use; disambiguator = 4 }] }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = (Types.Macro Types.Bang); krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent = None; path = [] }
                          }
                        });
                path =
                [{ Types.data = (Types.MacroNs "proxy_macro_if_not_hax");
                   disambiguator = 0 }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = (Types.Macro Types.Bang); krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent = None; path = [] }
                          }
                        });
                path =
                [{ Types.data = (Types.MacroNs "debug_assert");
                   disambiguator = 0 }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = (Types.Macro Types.Bang); krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent = None; path = [] }
                          }
                        });
                path =
                [{ Types.data = (Types.MacroNs "assert"); disambiguator = 0 }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true; kind = Types.Fn;
                krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent = None; path = [] }
                          }
                        });
                path =
                [{ Types.data = (Types.ValueNs "assert"); disambiguator = 0 }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = (Types.Macro Types.Bang); krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent = None; path = [] }
                          }
                        });
                path =
                [{ Types.data = (Types.MacroNs "assert_prop");
                   disambiguator = 0 }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true; kind = Types.Fn;
                krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent = None; path = [] }
                          }
                        });
                path =
                [{ Types.data = (Types.ValueNs "assert_prop");
                   disambiguator = 0 }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true; kind = Types.Fn;
                krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent = None; path = [] }
                          }
                        });
                path =
                [{ Types.data = (Types.ValueNs "assume"); disambiguator = 0 }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = (Types.Macro Types.Bang); krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent = None; path = [] }
                          }
                        });
                path =
                [{ Types.data = (Types.MacroNs "assume"); disambiguator = 0 }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true; kind = Types.Fn;
                krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent = None; path = [] }
                          }
                        });
                path =
                [{ Types.data = (Types.ValueNs "inline"); disambiguator = 0 }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true; kind = Types.Fn;
                krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent = None; path = [] }
                          }
                        });
                path =
                [{ Types.data = (Types.ValueNs "inline_unsafe");
                   disambiguator = 0 }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true; kind = Types.Fn;
                krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent = None; path = [] }
                          }
                        });
                path =
                [{ Types.data = (Types.ValueNs "any_to_unit");
                   disambiguator = 0 }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true; kind = Types.Fn;
                krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent = None; path = [] }
                          }
                        });
                path =
                [{ Types.data = (Types.ValueNs "_internal_loop_invariant");
                   disambiguator = 0 }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true; kind = Types.Fn;
                krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent = None; path = [] }
                          }
                        });
                path =
                [{ Types.data =
                   (Types.ValueNs "_internal_while_loop_invariant");
                   disambiguator = 0 }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true; kind = Types.Fn;
                krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent = None; path = [] }
                          }
                        });
                path =
                [{ Types.data = (Types.ValueNs "_internal_loop_decreases");
                   disambiguator = 0 }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Trait; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent = None; path = [] }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "Refinement");
                   disambiguator = 0 }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Trait; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent = None; path = [] }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "RefineAs"); disambiguator = 0
                   }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Mod; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent = None; path = [] }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 }] }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Use; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Use; disambiguator = 0 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Use; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Use; disambiguator = 1 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Use; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Use; disambiguator = 2 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Use; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Use; disambiguator = 3 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Use; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Use; disambiguator = 4 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Use; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Use; disambiguator = 5 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Struct; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = (Types.TypeNs "Int"); disambiguator = 0 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 8 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 9 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 10 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 11 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 12 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 13 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 14 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 15 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 0 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.AssocFn; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Impl {of_trait = false};
                            krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib";
                                        parent =
                                        (Some { Types.contents =
                                                { Types.id = 0;
                                                  value =
                                                  { Types.index =
                                                    (0, 0, None);
                                                    is_local = true;
                                                    kind = Types.Mod;
                                                    krate = "hax_lib";
                                                    parent = None; path = []
                                                    }
                                                  }
                                                });
                                        path =
                                        [{ Types.data = (Types.TypeNs "int");
                                           disambiguator = 0 }
                                          ]
                                        }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 };
                              { Types.data = Types.Impl; disambiguator = 1 }]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 1 };
                  { Types.data = (Types.ValueNs "new"); disambiguator = 0 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.AssocFn; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Impl {of_trait = false};
                            krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib";
                                        parent =
                                        (Some { Types.contents =
                                                { Types.id = 0;
                                                  value =
                                                  { Types.index =
                                                    (0, 0, None);
                                                    is_local = true;
                                                    kind = Types.Mod;
                                                    krate = "hax_lib";
                                                    parent = None; path = []
                                                    }
                                                  }
                                                });
                                        path =
                                        [{ Types.data = (Types.TypeNs "int");
                                           disambiguator = 0 }
                                          ]
                                        }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 };
                              { Types.data = Types.Impl; disambiguator = 1 }]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 1 };
                  { Types.data = (Types.ValueNs "get"); disambiguator = 0 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 2 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 3 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 4 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 5 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 6 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.AssocFn; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Impl {of_trait = false};
                            krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib";
                                        parent =
                                        (Some { Types.contents =
                                                { Types.id = 0;
                                                  value =
                                                  { Types.index =
                                                    (0, 0, None);
                                                    is_local = true;
                                                    kind = Types.Mod;
                                                    krate = "hax_lib";
                                                    parent = None; path = []
                                                    }
                                                  }
                                                });
                                        path =
                                        [{ Types.data = (Types.TypeNs "int");
                                           disambiguator = 0 }
                                          ]
                                        }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 };
                              { Types.data = Types.Impl; disambiguator = 7 }]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 7 };
                  { Types.data = (Types.ValueNs "pow2"); disambiguator = 0 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.AssocFn; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Impl {of_trait = false};
                            krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib";
                                        parent =
                                        (Some { Types.contents =
                                                { Types.id = 0;
                                                  value =
                                                  { Types.index =
                                                    (0, 0, None);
                                                    is_local = true;
                                                    kind = Types.Mod;
                                                    krate = "hax_lib";
                                                    parent = None; path = []
                                                    }
                                                  }
                                                });
                                        path =
                                        [{ Types.data = (Types.TypeNs "int");
                                           disambiguator = 0 }
                                          ]
                                        }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 };
                              { Types.data = Types.Impl; disambiguator = 7 }]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 7 };
                  { Types.data = (Types.ValueNs "_unsafe_from_str");
                    disambiguator = 0 }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.AssocFn; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Impl {of_trait = false};
                            krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib";
                                        parent =
                                        (Some { Types.contents =
                                                { Types.id = 0;
                                                  value =
                                                  { Types.index =
                                                    (0, 0, None);
                                                    is_local = true;
                                                    kind = Types.Mod;
                                                    krate = "hax_lib";
                                                    parent = None; path = []
                                                    }
                                                  }
                                                });
                                        path =
                                        [{ Types.data = (Types.TypeNs "int");
                                           disambiguator = 0 }
                                          ]
                                        }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 };
                              { Types.data = Types.Impl; disambiguator = 7 }]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 7 };
                  { Types.data = (Types.ValueNs "rem_euclid");
                    disambiguator = 0 }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Use; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.AssocFn; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true;
                                        kind = Types.Impl {of_trait = false};
                                        krate = "hax_lib";
                                        parent =
                                        (Some { Types.contents =
                                                { Types.id = 0;
                                                  value =
                                                  { Types.index =
                                                    (0, 0, None);
                                                    is_local = true;
                                                    kind = Types.Mod;
                                                    krate = "hax_lib";
                                                    parent =
                                                    (Some { Types.contents =
                                                            { Types.id = 0;
                                                              value =
                                                              { Types.index =
                                                                (0, 0, None);
                                                                is_local =
                                                                true;
                                                                kind =
                                                                Types.Mod;
                                                                krate =
                                                                "hax_lib";
                                                                parent = None;
                                                                path = [] }
                                                              }
                                                            });
                                                    path =
                                                    [{ Types.data =
                                                       (Types.TypeNs "int");
                                                       disambiguator = 0 }
                                                      ]
                                                    }
                                                  }
                                                });
                                        path =
                                        [{ Types.data = (Types.TypeNs "int");
                                           disambiguator = 0 };
                                          { Types.data = Types.Impl;
                                            disambiguator = 7 }
                                          ]
                                        }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 };
                              { Types.data = Types.Impl; disambiguator = 7 };
                              { Types.data =
                                (Types.ValueNs "_unsafe_from_str");
                                disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 7 };
                  { Types.data = (Types.ValueNs "_unsafe_from_str");
                    disambiguator = 0 };
                  { Types.data = Types.Use; disambiguator = 0 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Use; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.AssocFn; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true;
                                        kind = Types.Impl {of_trait = false};
                                        krate = "hax_lib";
                                        parent =
                                        (Some { Types.contents =
                                                { Types.id = 0;
                                                  value =
                                                  { Types.index =
                                                    (0, 0, None);
                                                    is_local = true;
                                                    kind = Types.Mod;
                                                    krate = "hax_lib";
                                                    parent =
                                                    (Some { Types.contents =
                                                            { Types.id = 0;
                                                              value =
                                                              { Types.index =
                                                                (0, 0, None);
                                                                is_local =
                                                                true;
                                                                kind =
                                                                Types.Mod;
                                                                krate =
                                                                "hax_lib";
                                                                parent = None;
                                                                path = [] }
                                                              }
                                                            });
                                                    path =
                                                    [{ Types.data =
                                                       (Types.TypeNs "int");
                                                       disambiguator = 0 }
                                                      ]
                                                    }
                                                  }
                                                });
                                        path =
                                        [{ Types.data = (Types.TypeNs "int");
                                           disambiguator = 0 };
                                          { Types.data = Types.Impl;
                                            disambiguator = 7 }
                                          ]
                                        }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 };
                              { Types.data = Types.Impl; disambiguator = 7 };
                              { Types.data = (Types.ValueNs "rem_euclid");
                                disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 7 };
                  { Types.data = (Types.ValueNs "rem_euclid");
                    disambiguator = 0 };
                  { Types.data = Types.Use; disambiguator = 0 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Trait; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = (Types.TypeNs "ToInt"); disambiguator = 0 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = (Types.Macro Types.Bang); krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = (Types.MacroNs "implement_abstraction");
                    disambiguator = 0 }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 16 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 17 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 18 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 19 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 20 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 21 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 22 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 23 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 24 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 25 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 26 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 27 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 28 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 29 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 30 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 31 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 32 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 33 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 34 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 35 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 36 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 37 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 38 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 39 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = (Types.Macro Types.Bang); krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = (Types.MacroNs "implement_concretize");
                    disambiguator = 0 }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 40 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.AssocFn; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Impl {of_trait = false};
                            krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib";
                                        parent =
                                        (Some { Types.contents =
                                                { Types.id = 0;
                                                  value =
                                                  { Types.index =
                                                    (0, 0, None);
                                                    is_local = true;
                                                    kind = Types.Mod;
                                                    krate = "hax_lib";
                                                    parent = None; path = []
                                                    }
                                                  }
                                                });
                                        path =
                                        [{ Types.data = (Types.TypeNs "int");
                                           disambiguator = 0 }
                                          ]
                                        }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 };
                              { Types.data = Types.Impl; disambiguator = 41 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 41 };
                  { Types.data = (Types.ValueNs "to_u8"); disambiguator = 0 }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 42 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.AssocFn; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Impl {of_trait = false};
                            krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib";
                                        parent =
                                        (Some { Types.contents =
                                                { Types.id = 0;
                                                  value =
                                                  { Types.index =
                                                    (0, 0, None);
                                                    is_local = true;
                                                    kind = Types.Mod;
                                                    krate = "hax_lib";
                                                    parent = None; path = []
                                                    }
                                                  }
                                                });
                                        path =
                                        [{ Types.data = (Types.TypeNs "int");
                                           disambiguator = 0 }
                                          ]
                                        }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 };
                              { Types.data = Types.Impl; disambiguator = 43 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 43 };
                  { Types.data = (Types.ValueNs "to_u16"); disambiguator = 0
                    }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 44 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.AssocFn; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Impl {of_trait = false};
                            krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib";
                                        parent =
                                        (Some { Types.contents =
                                                { Types.id = 0;
                                                  value =
                                                  { Types.index =
                                                    (0, 0, None);
                                                    is_local = true;
                                                    kind = Types.Mod;
                                                    krate = "hax_lib";
                                                    parent = None; path = []
                                                    }
                                                  }
                                                });
                                        path =
                                        [{ Types.data = (Types.TypeNs "int");
                                           disambiguator = 0 }
                                          ]
                                        }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 };
                              { Types.data = Types.Impl; disambiguator = 45 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 45 };
                  { Types.data = (Types.ValueNs "to_u32"); disambiguator = 0
                    }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 46 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.AssocFn; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Impl {of_trait = false};
                            krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib";
                                        parent =
                                        (Some { Types.contents =
                                                { Types.id = 0;
                                                  value =
                                                  { Types.index =
                                                    (0, 0, None);
                                                    is_local = true;
                                                    kind = Types.Mod;
                                                    krate = "hax_lib";
                                                    parent = None; path = []
                                                    }
                                                  }
                                                });
                                        path =
                                        [{ Types.data = (Types.TypeNs "int");
                                           disambiguator = 0 }
                                          ]
                                        }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 };
                              { Types.data = Types.Impl; disambiguator = 47 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 47 };
                  { Types.data = (Types.ValueNs "to_u64"); disambiguator = 0
                    }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 48 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.AssocFn; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Impl {of_trait = false};
                            krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib";
                                        parent =
                                        (Some { Types.contents =
                                                { Types.id = 0;
                                                  value =
                                                  { Types.index =
                                                    (0, 0, None);
                                                    is_local = true;
                                                    kind = Types.Mod;
                                                    krate = "hax_lib";
                                                    parent = None; path = []
                                                    }
                                                  }
                                                });
                                        path =
                                        [{ Types.data = (Types.TypeNs "int");
                                           disambiguator = 0 }
                                          ]
                                        }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 };
                              { Types.data = Types.Impl; disambiguator = 49 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 49 };
                  { Types.data = (Types.ValueNs "to_u128"); disambiguator = 0
                    }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 50 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.AssocFn; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Impl {of_trait = false};
                            krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib";
                                        parent =
                                        (Some { Types.contents =
                                                { Types.id = 0;
                                                  value =
                                                  { Types.index =
                                                    (0, 0, None);
                                                    is_local = true;
                                                    kind = Types.Mod;
                                                    krate = "hax_lib";
                                                    parent = None; path = []
                                                    }
                                                  }
                                                });
                                        path =
                                        [{ Types.data = (Types.TypeNs "int");
                                           disambiguator = 0 }
                                          ]
                                        }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 };
                              { Types.data = Types.Impl; disambiguator = 51 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 51 };
                  { Types.data = (Types.ValueNs "to_usize");
                    disambiguator = 0 }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 52 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.AssocFn; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Impl {of_trait = false};
                            krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib";
                                        parent =
                                        (Some { Types.contents =
                                                { Types.id = 0;
                                                  value =
                                                  { Types.index =
                                                    (0, 0, None);
                                                    is_local = true;
                                                    kind = Types.Mod;
                                                    krate = "hax_lib";
                                                    parent = None; path = []
                                                    }
                                                  }
                                                });
                                        path =
                                        [{ Types.data = (Types.TypeNs "int");
                                           disambiguator = 0 }
                                          ]
                                        }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 };
                              { Types.data = Types.Impl; disambiguator = 53 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 53 };
                  { Types.data = (Types.ValueNs "to_i8"); disambiguator = 0 }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 54 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.AssocFn; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Impl {of_trait = false};
                            krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib";
                                        parent =
                                        (Some { Types.contents =
                                                { Types.id = 0;
                                                  value =
                                                  { Types.index =
                                                    (0, 0, None);
                                                    is_local = true;
                                                    kind = Types.Mod;
                                                    krate = "hax_lib";
                                                    parent = None; path = []
                                                    }
                                                  }
                                                });
                                        path =
                                        [{ Types.data = (Types.TypeNs "int");
                                           disambiguator = 0 }
                                          ]
                                        }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 };
                              { Types.data = Types.Impl; disambiguator = 55 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 55 };
                  { Types.data = (Types.ValueNs "to_i16"); disambiguator = 0
                    }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 56 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.AssocFn; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Impl {of_trait = false};
                            krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib";
                                        parent =
                                        (Some { Types.contents =
                                                { Types.id = 0;
                                                  value =
                                                  { Types.index =
                                                    (0, 0, None);
                                                    is_local = true;
                                                    kind = Types.Mod;
                                                    krate = "hax_lib";
                                                    parent = None; path = []
                                                    }
                                                  }
                                                });
                                        path =
                                        [{ Types.data = (Types.TypeNs "int");
                                           disambiguator = 0 }
                                          ]
                                        }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 };
                              { Types.data = Types.Impl; disambiguator = 57 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 57 };
                  { Types.data = (Types.ValueNs "to_i32"); disambiguator = 0
                    }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 58 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.AssocFn; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Impl {of_trait = false};
                            krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib";
                                        parent =
                                        (Some { Types.contents =
                                                { Types.id = 0;
                                                  value =
                                                  { Types.index =
                                                    (0, 0, None);
                                                    is_local = true;
                                                    kind = Types.Mod;
                                                    krate = "hax_lib";
                                                    parent = None; path = []
                                                    }
                                                  }
                                                });
                                        path =
                                        [{ Types.data = (Types.TypeNs "int");
                                           disambiguator = 0 }
                                          ]
                                        }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 };
                              { Types.data = Types.Impl; disambiguator = 59 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 59 };
                  { Types.data = (Types.ValueNs "to_i64"); disambiguator = 0
                    }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 60 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.AssocFn; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Impl {of_trait = false};
                            krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib";
                                        parent =
                                        (Some { Types.contents =
                                                { Types.id = 0;
                                                  value =
                                                  { Types.index =
                                                    (0, 0, None);
                                                    is_local = true;
                                                    kind = Types.Mod;
                                                    krate = "hax_lib";
                                                    parent = None; path = []
                                                    }
                                                  }
                                                });
                                        path =
                                        [{ Types.data = (Types.TypeNs "int");
                                           disambiguator = 0 }
                                          ]
                                        }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 };
                              { Types.data = Types.Impl; disambiguator = 61 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 61 };
                  { Types.data = (Types.ValueNs "to_i128"); disambiguator = 0
                    }
                  ]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.Impl {of_trait = true}; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Mod; krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib"; parent = None;
                                        path = [] }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 62 }]
                }
              };
            { Explicit_def_id.T.is_constructor = false;
              def_id =
              { Types.index = (0, 0, None); is_local = true;
                kind = Types.AssocFn; krate = "hax_lib";
                parent =
                (Some { Types.contents =
                        { Types.id = 0;
                          value =
                          { Types.index = (0, 0, None); is_local = true;
                            kind = Types.Impl {of_trait = false};
                            krate = "hax_lib";
                            parent =
                            (Some { Types.contents =
                                    { Types.id = 0;
                                      value =
                                      { Types.index = (0, 0, None);
                                        is_local = true; kind = Types.Mod;
                                        krate = "hax_lib";
                                        parent =
                                        (Some { Types.contents =
                                                { Types.id = 0;
                                                  value =
                                                  { Types.index =
                                                    (0, 0, None);
                                                    is_local = true;
                                                    kind = Types.Mod;
                                                    krate = "hax_lib";
                                                    parent = None; path = []
                                                    }
                                                  }
                                                });
                                        path =
                                        [{ Types.data = (Types.TypeNs "int");
                                           disambiguator = 0 }
                                          ]
                                        }
                                      }
                                    });
                            path =
                            [{ Types.data = (Types.TypeNs "int");
                               disambiguator = 0 };
                              { Types.data = Types.Impl; disambiguator = 63 }
                              ]
                            }
                          }
                        });
                path =
                [{ Types.data = (Types.TypeNs "int"); disambiguator = 0 };
                  { Types.data = Types.Impl; disambiguator = 63 };
                  { Types.data = (Types.ValueNs "to_isize");
                    disambiguator = 0 }
                  ]
                }
              }
            ];
          label = "bundle" });
  suffix = None }) */
const _: () = ();
 *)

/// A utilitary trait that provides a `into_checked` method on traits
/// that have a refined counter part. This trait is parametrized by a
/// type `Target`: a base type can be refined in multiple ways.
/// Please never implement this trait yourself, use the
/// `refinement_type` macro instead.
class t_RefineAs (v_Self: Type0) (v_RefinedType: Type0) = {
  f_into_checked_pre:v_Self -> Type0;
  f_into_checked_post:v_Self -> v_RefinedType -> Type0;
  f_into_checked:x0: v_Self
    -> Prims.Pure v_RefinedType
        (f_into_checked_pre x0)
        (fun result -> f_into_checked_post x0 result)
}

/// Mathematical integers for writting specifications. Mathematical
/// integers are unbounded and arithmetic operation on them never over
/// or underflow.
type t_Int = | Int : Hax_lib.Int.Bigint.t_BigInt -> t_Int

/// A dummy function that holds a loop variant.
let e_internal_loop_decreases (_: t_Int) : Prims.unit = ()

let impl_9: Core_models.Clone.t_Clone t_Int =
  { f_clone = (fun x -> x); f_clone_pre = (fun _ -> True); f_clone_post = (fun _ _ -> True) }

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_8': Core_models.Marker.t_Copy t_Int

unfold
let impl_8 = impl_8'

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_11': Core_models.Marker.t_StructuralPartialEq t_Int

unfold
let impl_11 = impl_11'

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_12': Core_models.Cmp.t_PartialEq t_Int t_Int

unfold
let impl_12 = impl_12'

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_10': Core_models.Cmp.t_Eq t_Int

unfold
let impl_10 = impl_10'

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_14': Core_models.Cmp.t_PartialOrd t_Int t_Int

unfold
let impl_14 = impl_14'

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_13': Core_models.Cmp.t_Ord t_Int

unfold
let impl_13 = impl_13'

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_15': Core_models.Fmt.t_Debug t_Int

unfold
let impl_15 = impl_15'

let impl_1__new
      (#iimpl_637761304_: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Core_models.Convert.t_Into iimpl_637761304_ Num_bigint.Bigint.t_BigInt)
      (x: iimpl_637761304_)
    : t_Int =
  Int
  (Hax_lib.Int.Bigint.impl_BigInt__new (Core_models.Convert.f_into #iimpl_637761304_
          #Num_bigint.Bigint.t_BigInt
          #FStar.Tactics.Typeclasses.solve
          x
        <:
        Num_bigint.Bigint.t_BigInt))
  <:
  t_Int

let impl_1__get (self: t_Int) : Num_bigint.Bigint.t_BigInt =
  Hax_lib.Int.Bigint.impl_BigInt__get self._0

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl: Core_models.Fmt.t_Display t_Int =
  {
    f_fmt_pre = (fun (self: t_Int) (f: Core_models.Fmt.t_Formatter) -> true);
    f_fmt_post
    =
    (fun
        (self: t_Int)
        (f: Core_models.Fmt.t_Formatter)
        (out1:
          (Core_models.Fmt.t_Formatter &
            Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error))
        ->
        true);
    f_fmt
    =
    fun (self: t_Int) (f: Core_models.Fmt.t_Formatter) ->
      let args:t_Array Core_models.Fmt.Rt.t_Argument (mk_usize 1) =
        let list =
          [
            Core_models.Fmt.Rt.impl__new_display #Num_bigint.Bigint.t_BigInt
              (impl_1__get self <: Num_bigint.Bigint.t_BigInt)
          ]
        in
        FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
        Rust_primitives.Hax.array_of_list 1 list
      in
      let tmp0, out:(Core_models.Fmt.t_Formatter &
        Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error) =
        Core_models.Fmt.impl_11__write_fmt f
          (Core_models.Fmt.Rt.impl_1__new_v1 (mk_usize 1)
              (mk_usize 1)
              (let list = [""] in
                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                Rust_primitives.Hax.array_of_list 1 list)
              args
            <:
            Core_models.Fmt.t_Arguments)
      in
      let f:Core_models.Fmt.t_Formatter = tmp0 in
      let hax_temp_output:Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error = out in
      f, hax_temp_output
      <:
      (Core_models.Fmt.t_Formatter & Core_models.Result.t_Result Prims.unit Core_models.Fmt.t_Error)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_2: Core_models.Ops.Arith.t_Add t_Int t_Int =
  {
    f_Output = t_Int;
    f_add_pre = (fun (self: t_Int) (other: t_Int) -> true);
    f_add_post = (fun (self: t_Int) (other: t_Int) (out: t_Int) -> true);
    f_add
    =
    fun (self: t_Int) (other: t_Int) ->
      impl_1__new #Num_bigint.Bigint.t_BigInt
        (Core_models.Ops.Arith.f_add #Num_bigint.Bigint.t_BigInt
            #Num_bigint.Bigint.t_BigInt
            #FStar.Tactics.Typeclasses.solve
            (impl_1__get self <: Num_bigint.Bigint.t_BigInt)
            (impl_1__get other <: Num_bigint.Bigint.t_BigInt)
          <:
          Num_bigint.Bigint.t_BigInt)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_3: Core_models.Ops.Arith.t_Neg t_Int =
  {
    f_Output = t_Int;
    f_neg_pre = (fun (self: t_Int) -> true);
    f_neg_post = (fun (self: t_Int) (out: t_Int) -> true);
    f_neg
    =
    fun (self: t_Int) ->
      impl_1__new #Num_bigint.Bigint.t_BigInt
        (Core_models.Ops.Arith.f_neg #Num_bigint.Bigint.t_BigInt
            #FStar.Tactics.Typeclasses.solve
            (impl_1__get self <: Num_bigint.Bigint.t_BigInt)
          <:
          Num_bigint.Bigint.t_BigInt)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_4: Core_models.Ops.Arith.t_Sub t_Int t_Int =
  {
    f_Output = t_Int;
    f_sub_pre = (fun (self: t_Int) (other: t_Int) -> true);
    f_sub_post = (fun (self: t_Int) (other: t_Int) (out: t_Int) -> true);
    f_sub
    =
    fun (self: t_Int) (other: t_Int) ->
      impl_1__new #Num_bigint.Bigint.t_BigInt
        (Core_models.Ops.Arith.f_sub #Num_bigint.Bigint.t_BigInt
            #Num_bigint.Bigint.t_BigInt
            #FStar.Tactics.Typeclasses.solve
            (impl_1__get self <: Num_bigint.Bigint.t_BigInt)
            (impl_1__get other <: Num_bigint.Bigint.t_BigInt)
          <:
          Num_bigint.Bigint.t_BigInt)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_5: Core_models.Ops.Arith.t_Mul t_Int t_Int =
  {
    f_Output = t_Int;
    f_mul_pre = (fun (self: t_Int) (other: t_Int) -> true);
    f_mul_post = (fun (self: t_Int) (other: t_Int) (out: t_Int) -> true);
    f_mul
    =
    fun (self: t_Int) (other: t_Int) ->
      impl_1__new #Num_bigint.Bigint.t_BigInt
        (Core_models.Ops.Arith.f_mul #Num_bigint.Bigint.t_BigInt
            #Num_bigint.Bigint.t_BigInt
            #FStar.Tactics.Typeclasses.solve
            (impl_1__get self <: Num_bigint.Bigint.t_BigInt)
            (impl_1__get other <: Num_bigint.Bigint.t_BigInt)
          <:
          Num_bigint.Bigint.t_BigInt)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_6: Core_models.Ops.Arith.t_Div t_Int t_Int =
  {
    f_Output = t_Int;
    f_div_pre = (fun (self: t_Int) (other: t_Int) -> true);
    f_div_post = (fun (self: t_Int) (other: t_Int) (out: t_Int) -> true);
    f_div
    =
    fun (self: t_Int) (other: t_Int) ->
      impl_1__new #Num_bigint.Bigint.t_BigInt
        (Core_models.Ops.Arith.f_div #Num_bigint.Bigint.t_BigInt
            #Num_bigint.Bigint.t_BigInt
            #FStar.Tactics.Typeclasses.solve
            (impl_1__get self <: Num_bigint.Bigint.t_BigInt)
            (impl_1__get other <: Num_bigint.Bigint.t_BigInt)
          <:
          Num_bigint.Bigint.t_BigInt)
  }

/// Raises `2` at the power `self`
let impl_7__pow2 (self: t_Int) : t_Int =
  let exponent:u32 =
    Core_models.Option.impl__expect #u32
      (Num_traits.Cast.f_to_u32 #Num_bigint.Bigint.t_BigInt
          #FStar.Tactics.Typeclasses.solve
          (impl_1__get self <: Num_bigint.Bigint.t_BigInt)
        <:
        Core_models.Option.t_Option u32)
      "Exponent doesn't fit in a u32"
  in
  impl_1__new #Num_bigint.Bigint.t_BigInt
    (Num_bigint.Bigint.impl_BigInt__pow (Core_models.Convert.f_from #Num_bigint.Bigint.t_BigInt
            #u8
            #FStar.Tactics.Typeclasses.solve
            (mk_u8 2)
          <:
          Num_bigint.Bigint.t_BigInt)
        exponent
      <:
      Num_bigint.Bigint.t_BigInt)

/// Constructs a `Int` out of a string literal. This function
/// assumes its argument consists only of decimal digits, with
/// optionally a minus sign prefix.
let impl_7__e_unsafe_from_str (s: string) : t_Int =
  impl_1__new #Num_bigint.Bigint.t_BigInt
    (Core_models.Result.impl__unwrap #Num_bigint.Bigint.t_BigInt
        #Num_bigint.t_ParseBigIntError
        (Core_models.Str.Traits.f_from_str #Num_bigint.Bigint.t_BigInt
            #FStar.Tactics.Typeclasses.solve
            s
          <:
          Core_models.Result.t_Result Num_bigint.Bigint.t_BigInt Num_bigint.t_ParseBigIntError)
      <:
      Num_bigint.Bigint.t_BigInt)

let impl_7__rem_euclid (self v: t_Int) : t_Int =
  impl_1__new #Num_bigint.Bigint.t_BigInt
    (Num_traits.Ops.Euclid.f_rem_euclid #Num_bigint.Bigint.t_BigInt
        #FStar.Tactics.Typeclasses.solve
        (impl_1__get self <: Num_bigint.Bigint.t_BigInt)
        (impl_1__get v <: Num_bigint.Bigint.t_BigInt)
      <:
      Num_bigint.Bigint.t_BigInt)

class t_ToInt (v_Self: Type0) = {
  f_to_int_pre:v_Self -> Type0;
  f_to_int_post:v_Self -> t_Int -> Type0;
  f_to_int:x0: v_Self -> Prims.Pure t_Int (f_to_int_pre x0) (fun result -> f_to_int_post x0 result)
}

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_16: Hax_lib.Abstraction.t_Abstraction u8 =
  {
    f_AbstractType = t_Int;
    f_lift_pre = (fun (self: u8) -> true);
    f_lift_post = (fun (self: u8) (out: t_Int) -> true);
    f_lift
    =
    fun (self: u8) ->
      impl_1__new #Num_bigint.Bigint.t_BigInt
        (Core_models.Convert.f_from #Num_bigint.Bigint.t_BigInt
            #u8
            #FStar.Tactics.Typeclasses.solve
            self
          <:
          Num_bigint.Bigint.t_BigInt)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_17: t_ToInt u8 =
  {
    f_to_int_pre = (fun (self: u8) -> true);
    f_to_int_post = (fun (self: u8) (out: t_Int) -> true);
    f_to_int
    =
    fun (self: u8) -> Hax_lib.Abstraction.f_lift #u8 #FStar.Tactics.Typeclasses.solve self
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_18: Hax_lib.Abstraction.t_Abstraction u16 =
  {
    f_AbstractType = t_Int;
    f_lift_pre = (fun (self: u16) -> true);
    f_lift_post = (fun (self: u16) (out: t_Int) -> true);
    f_lift
    =
    fun (self: u16) ->
      impl_1__new #Num_bigint.Bigint.t_BigInt
        (Core_models.Convert.f_from #Num_bigint.Bigint.t_BigInt
            #u16
            #FStar.Tactics.Typeclasses.solve
            self
          <:
          Num_bigint.Bigint.t_BigInt)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_19: t_ToInt u16 =
  {
    f_to_int_pre = (fun (self: u16) -> true);
    f_to_int_post = (fun (self: u16) (out: t_Int) -> true);
    f_to_int
    =
    fun (self: u16) -> Hax_lib.Abstraction.f_lift #u16 #FStar.Tactics.Typeclasses.solve self
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_20: Hax_lib.Abstraction.t_Abstraction u32 =
  {
    f_AbstractType = t_Int;
    f_lift_pre = (fun (self: u32) -> true);
    f_lift_post = (fun (self: u32) (out: t_Int) -> true);
    f_lift
    =
    fun (self: u32) ->
      impl_1__new #Num_bigint.Bigint.t_BigInt
        (Core_models.Convert.f_from #Num_bigint.Bigint.t_BigInt
            #u32
            #FStar.Tactics.Typeclasses.solve
            self
          <:
          Num_bigint.Bigint.t_BigInt)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_21: t_ToInt u32 =
  {
    f_to_int_pre = (fun (self: u32) -> true);
    f_to_int_post = (fun (self: u32) (out: t_Int) -> true);
    f_to_int
    =
    fun (self: u32) -> Hax_lib.Abstraction.f_lift #u32 #FStar.Tactics.Typeclasses.solve self
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_22: Hax_lib.Abstraction.t_Abstraction u64 =
  {
    f_AbstractType = t_Int;
    f_lift_pre = (fun (self: u64) -> true);
    f_lift_post = (fun (self: u64) (out: t_Int) -> true);
    f_lift
    =
    fun (self: u64) ->
      impl_1__new #Num_bigint.Bigint.t_BigInt
        (Core_models.Convert.f_from #Num_bigint.Bigint.t_BigInt
            #u64
            #FStar.Tactics.Typeclasses.solve
            self
          <:
          Num_bigint.Bigint.t_BigInt)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_23: t_ToInt u64 =
  {
    f_to_int_pre = (fun (self: u64) -> true);
    f_to_int_post = (fun (self: u64) (out: t_Int) -> true);
    f_to_int
    =
    fun (self: u64) -> Hax_lib.Abstraction.f_lift #u64 #FStar.Tactics.Typeclasses.solve self
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_24: Hax_lib.Abstraction.t_Abstraction u128 =
  {
    f_AbstractType = t_Int;
    f_lift_pre = (fun (self: u128) -> true);
    f_lift_post = (fun (self: u128) (out: t_Int) -> true);
    f_lift
    =
    fun (self: u128) ->
      impl_1__new #Num_bigint.Bigint.t_BigInt
        (Core_models.Convert.f_from #Num_bigint.Bigint.t_BigInt
            #u128
            #FStar.Tactics.Typeclasses.solve
            self
          <:
          Num_bigint.Bigint.t_BigInt)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_25: t_ToInt u128 =
  {
    f_to_int_pre = (fun (self: u128) -> true);
    f_to_int_post = (fun (self: u128) (out: t_Int) -> true);
    f_to_int
    =
    fun (self: u128) -> Hax_lib.Abstraction.f_lift #u128 #FStar.Tactics.Typeclasses.solve self
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_26: Hax_lib.Abstraction.t_Abstraction usize =
  {
    f_AbstractType = t_Int;
    f_lift_pre = (fun (self: usize) -> true);
    f_lift_post = (fun (self: usize) (out: t_Int) -> true);
    f_lift
    =
    fun (self: usize) ->
      impl_1__new #Num_bigint.Bigint.t_BigInt
        (Core_models.Convert.f_from #Num_bigint.Bigint.t_BigInt
            #usize
            #FStar.Tactics.Typeclasses.solve
            self
          <:
          Num_bigint.Bigint.t_BigInt)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_27: t_ToInt usize =
  {
    f_to_int_pre = (fun (self: usize) -> true);
    f_to_int_post = (fun (self: usize) (out: t_Int) -> true);
    f_to_int
    =
    fun (self: usize) -> Hax_lib.Abstraction.f_lift #usize #FStar.Tactics.Typeclasses.solve self
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_28: Hax_lib.Abstraction.t_Abstraction i8 =
  {
    f_AbstractType = t_Int;
    f_lift_pre = (fun (self: i8) -> true);
    f_lift_post = (fun (self: i8) (out: t_Int) -> true);
    f_lift
    =
    fun (self: i8) ->
      impl_1__new #Num_bigint.Bigint.t_BigInt
        (Core_models.Convert.f_from #Num_bigint.Bigint.t_BigInt
            #i8
            #FStar.Tactics.Typeclasses.solve
            self
          <:
          Num_bigint.Bigint.t_BigInt)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_29: t_ToInt i8 =
  {
    f_to_int_pre = (fun (self: i8) -> true);
    f_to_int_post = (fun (self: i8) (out: t_Int) -> true);
    f_to_int
    =
    fun (self: i8) -> Hax_lib.Abstraction.f_lift #i8 #FStar.Tactics.Typeclasses.solve self
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_30: Hax_lib.Abstraction.t_Abstraction i16 =
  {
    f_AbstractType = t_Int;
    f_lift_pre = (fun (self: i16) -> true);
    f_lift_post = (fun (self: i16) (out: t_Int) -> true);
    f_lift
    =
    fun (self: i16) ->
      impl_1__new #Num_bigint.Bigint.t_BigInt
        (Core_models.Convert.f_from #Num_bigint.Bigint.t_BigInt
            #i16
            #FStar.Tactics.Typeclasses.solve
            self
          <:
          Num_bigint.Bigint.t_BigInt)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_31: t_ToInt i16 =
  {
    f_to_int_pre = (fun (self: i16) -> true);
    f_to_int_post = (fun (self: i16) (out: t_Int) -> true);
    f_to_int
    =
    fun (self: i16) -> Hax_lib.Abstraction.f_lift #i16 #FStar.Tactics.Typeclasses.solve self
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_32: Hax_lib.Abstraction.t_Abstraction i32 =
  {
    f_AbstractType = t_Int;
    f_lift_pre = (fun (self: i32) -> true);
    f_lift_post = (fun (self: i32) (out: t_Int) -> true);
    f_lift
    =
    fun (self: i32) ->
      impl_1__new #Num_bigint.Bigint.t_BigInt
        (Core_models.Convert.f_from #Num_bigint.Bigint.t_BigInt
            #i32
            #FStar.Tactics.Typeclasses.solve
            self
          <:
          Num_bigint.Bigint.t_BigInt)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_33: t_ToInt i32 =
  {
    f_to_int_pre = (fun (self: i32) -> true);
    f_to_int_post = (fun (self: i32) (out: t_Int) -> true);
    f_to_int
    =
    fun (self: i32) -> Hax_lib.Abstraction.f_lift #i32 #FStar.Tactics.Typeclasses.solve self
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_34: Hax_lib.Abstraction.t_Abstraction i64 =
  {
    f_AbstractType = t_Int;
    f_lift_pre = (fun (self: i64) -> true);
    f_lift_post = (fun (self: i64) (out: t_Int) -> true);
    f_lift
    =
    fun (self: i64) ->
      impl_1__new #Num_bigint.Bigint.t_BigInt
        (Core_models.Convert.f_from #Num_bigint.Bigint.t_BigInt
            #i64
            #FStar.Tactics.Typeclasses.solve
            self
          <:
          Num_bigint.Bigint.t_BigInt)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_35: t_ToInt i64 =
  {
    f_to_int_pre = (fun (self: i64) -> true);
    f_to_int_post = (fun (self: i64) (out: t_Int) -> true);
    f_to_int
    =
    fun (self: i64) -> Hax_lib.Abstraction.f_lift #i64 #FStar.Tactics.Typeclasses.solve self
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_36: Hax_lib.Abstraction.t_Abstraction i128 =
  {
    f_AbstractType = t_Int;
    f_lift_pre = (fun (self: i128) -> true);
    f_lift_post = (fun (self: i128) (out: t_Int) -> true);
    f_lift
    =
    fun (self: i128) ->
      impl_1__new #Num_bigint.Bigint.t_BigInt
        (Core_models.Convert.f_from #Num_bigint.Bigint.t_BigInt
            #i128
            #FStar.Tactics.Typeclasses.solve
            self
          <:
          Num_bigint.Bigint.t_BigInt)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_37: t_ToInt i128 =
  {
    f_to_int_pre = (fun (self: i128) -> true);
    f_to_int_post = (fun (self: i128) (out: t_Int) -> true);
    f_to_int
    =
    fun (self: i128) -> Hax_lib.Abstraction.f_lift #i128 #FStar.Tactics.Typeclasses.solve self
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_38: Hax_lib.Abstraction.t_Abstraction isize =
  {
    f_AbstractType = t_Int;
    f_lift_pre = (fun (self: isize) -> true);
    f_lift_post = (fun (self: isize) (out: t_Int) -> true);
    f_lift
    =
    fun (self: isize) ->
      impl_1__new #Num_bigint.Bigint.t_BigInt
        (Core_models.Convert.f_from #Num_bigint.Bigint.t_BigInt
            #isize
            #FStar.Tactics.Typeclasses.solve
            self
          <:
          Num_bigint.Bigint.t_BigInt)
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_39: t_ToInt isize =
  {
    f_to_int_pre = (fun (self: isize) -> true);
    f_to_int_post = (fun (self: isize) (out: t_Int) -> true);
    f_to_int
    =
    fun (self: isize) -> Hax_lib.Abstraction.f_lift #isize #FStar.Tactics.Typeclasses.solve self
  }

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_40: Hax_lib.Abstraction.t_Concretization t_Int u8 =
  {
    f_concretize_pre = (fun (self: t_Int) -> true);
    f_concretize_post = (fun (self: t_Int) (out: u8) -> true);
    f_concretize
    =
    fun (self: t_Int) ->
      let concretized:Core_models.Option.t_Option u8 =
        Num_traits.Cast.f_to_u8 #Num_bigint.Bigint.t_BigInt
          #FStar.Tactics.Typeclasses.solve
          (impl_1__get self <: Num_bigint.Bigint.t_BigInt)
      in
      let _:Prims.unit =
        if true
        then
          let _:Prims.unit = v_assert (Core_models.Option.impl__is_some #u8 concretized <: bool) in
          ()
      in
      Core_models.Option.impl__unwrap #u8 concretized
  }

let impl_41__to_u8 (self: t_Int) : u8 =
  Hax_lib.Abstraction.f_concretize #t_Int #u8 #FStar.Tactics.Typeclasses.solve self

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_42: Hax_lib.Abstraction.t_Concretization t_Int u16 =
  {
    f_concretize_pre = (fun (self: t_Int) -> true);
    f_concretize_post = (fun (self: t_Int) (out: u16) -> true);
    f_concretize
    =
    fun (self: t_Int) ->
      let concretized:Core_models.Option.t_Option u16 =
        Num_traits.Cast.f_to_u16 #Num_bigint.Bigint.t_BigInt
          #FStar.Tactics.Typeclasses.solve
          (impl_1__get self <: Num_bigint.Bigint.t_BigInt)
      in
      let _:Prims.unit =
        if true
        then
          let _:Prims.unit = v_assert (Core_models.Option.impl__is_some #u16 concretized <: bool) in
          ()
      in
      Core_models.Option.impl__unwrap #u16 concretized
  }

let impl_43__to_u16 (self: t_Int) : u16 =
  Hax_lib.Abstraction.f_concretize #t_Int #u16 #FStar.Tactics.Typeclasses.solve self

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_44: Hax_lib.Abstraction.t_Concretization t_Int u32 =
  {
    f_concretize_pre = (fun (self: t_Int) -> true);
    f_concretize_post = (fun (self: t_Int) (out: u32) -> true);
    f_concretize
    =
    fun (self: t_Int) ->
      let concretized:Core_models.Option.t_Option u32 =
        Num_traits.Cast.f_to_u32 #Num_bigint.Bigint.t_BigInt
          #FStar.Tactics.Typeclasses.solve
          (impl_1__get self <: Num_bigint.Bigint.t_BigInt)
      in
      let _:Prims.unit =
        if true
        then
          let _:Prims.unit = v_assert (Core_models.Option.impl__is_some #u32 concretized <: bool) in
          ()
      in
      Core_models.Option.impl__unwrap #u32 concretized
  }

let impl_45__to_u32 (self: t_Int) : u32 =
  Hax_lib.Abstraction.f_concretize #t_Int #u32 #FStar.Tactics.Typeclasses.solve self

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_46: Hax_lib.Abstraction.t_Concretization t_Int u64 =
  {
    f_concretize_pre = (fun (self: t_Int) -> true);
    f_concretize_post = (fun (self: t_Int) (out: u64) -> true);
    f_concretize
    =
    fun (self: t_Int) ->
      let concretized:Core_models.Option.t_Option u64 =
        Num_traits.Cast.f_to_u64 #Num_bigint.Bigint.t_BigInt
          #FStar.Tactics.Typeclasses.solve
          (impl_1__get self <: Num_bigint.Bigint.t_BigInt)
      in
      let _:Prims.unit =
        if true
        then
          let _:Prims.unit = v_assert (Core_models.Option.impl__is_some #u64 concretized <: bool) in
          ()
      in
      Core_models.Option.impl__unwrap #u64 concretized
  }

let impl_47__to_u64 (self: t_Int) : u64 =
  Hax_lib.Abstraction.f_concretize #t_Int #u64 #FStar.Tactics.Typeclasses.solve self

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_48: Hax_lib.Abstraction.t_Concretization t_Int u128 =
  {
    f_concretize_pre = (fun (self: t_Int) -> true);
    f_concretize_post = (fun (self: t_Int) (out: u128) -> true);
    f_concretize
    =
    fun (self: t_Int) ->
      let concretized:Core_models.Option.t_Option u128 =
        Num_traits.Cast.f_to_u128 #Num_bigint.Bigint.t_BigInt
          #FStar.Tactics.Typeclasses.solve
          (impl_1__get self <: Num_bigint.Bigint.t_BigInt)
      in
      let _:Prims.unit =
        if true
        then
          let _:Prims.unit =
            v_assert (Core_models.Option.impl__is_some #u128 concretized <: bool)
          in
          ()
      in
      Core_models.Option.impl__unwrap #u128 concretized
  }

let impl_49__to_u128 (self: t_Int) : u128 =
  Hax_lib.Abstraction.f_concretize #t_Int #u128 #FStar.Tactics.Typeclasses.solve self

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_50: Hax_lib.Abstraction.t_Concretization t_Int usize =
  {
    f_concretize_pre = (fun (self: t_Int) -> true);
    f_concretize_post = (fun (self: t_Int) (out: usize) -> true);
    f_concretize
    =
    fun (self: t_Int) ->
      let concretized:Core_models.Option.t_Option usize =
        Num_traits.Cast.f_to_usize #Num_bigint.Bigint.t_BigInt
          #FStar.Tactics.Typeclasses.solve
          (impl_1__get self <: Num_bigint.Bigint.t_BigInt)
      in
      let _:Prims.unit =
        if true
        then
          let _:Prims.unit =
            v_assert (Core_models.Option.impl__is_some #usize concretized <: bool)
          in
          ()
      in
      Core_models.Option.impl__unwrap #usize concretized
  }

let impl_51__to_usize (self: t_Int) : usize =
  Hax_lib.Abstraction.f_concretize #t_Int #usize #FStar.Tactics.Typeclasses.solve self

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_52: Hax_lib.Abstraction.t_Concretization t_Int i8 =
  {
    f_concretize_pre = (fun (self: t_Int) -> true);
    f_concretize_post = (fun (self: t_Int) (out: i8) -> true);
    f_concretize
    =
    fun (self: t_Int) ->
      let concretized:Core_models.Option.t_Option i8 =
        Num_traits.Cast.f_to_i8 #Num_bigint.Bigint.t_BigInt
          #FStar.Tactics.Typeclasses.solve
          (impl_1__get self <: Num_bigint.Bigint.t_BigInt)
      in
      let _:Prims.unit =
        if true
        then
          let _:Prims.unit = v_assert (Core_models.Option.impl__is_some #i8 concretized <: bool) in
          ()
      in
      Core_models.Option.impl__unwrap #i8 concretized
  }

let impl_53__to_i8 (self: t_Int) : i8 =
  Hax_lib.Abstraction.f_concretize #t_Int #i8 #FStar.Tactics.Typeclasses.solve self

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_54: Hax_lib.Abstraction.t_Concretization t_Int i16 =
  {
    f_concretize_pre = (fun (self: t_Int) -> true);
    f_concretize_post = (fun (self: t_Int) (out: i16) -> true);
    f_concretize
    =
    fun (self: t_Int) ->
      let concretized:Core_models.Option.t_Option i16 =
        Num_traits.Cast.f_to_i16 #Num_bigint.Bigint.t_BigInt
          #FStar.Tactics.Typeclasses.solve
          (impl_1__get self <: Num_bigint.Bigint.t_BigInt)
      in
      let _:Prims.unit =
        if true
        then
          let _:Prims.unit = v_assert (Core_models.Option.impl__is_some #i16 concretized <: bool) in
          ()
      in
      Core_models.Option.impl__unwrap #i16 concretized
  }

let impl_55__to_i16 (self: t_Int) : i16 =
  Hax_lib.Abstraction.f_concretize #t_Int #i16 #FStar.Tactics.Typeclasses.solve self

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_56: Hax_lib.Abstraction.t_Concretization t_Int i32 =
  {
    f_concretize_pre = (fun (self: t_Int) -> true);
    f_concretize_post = (fun (self: t_Int) (out: i32) -> true);
    f_concretize
    =
    fun (self: t_Int) ->
      let concretized:Core_models.Option.t_Option i32 =
        Num_traits.Cast.f_to_i32 #Num_bigint.Bigint.t_BigInt
          #FStar.Tactics.Typeclasses.solve
          (impl_1__get self <: Num_bigint.Bigint.t_BigInt)
      in
      let _:Prims.unit =
        if true
        then
          let _:Prims.unit = v_assert (Core_models.Option.impl__is_some #i32 concretized <: bool) in
          ()
      in
      Core_models.Option.impl__unwrap #i32 concretized
  }

let impl_57__to_i32 (self: t_Int) : i32 =
  Hax_lib.Abstraction.f_concretize #t_Int #i32 #FStar.Tactics.Typeclasses.solve self

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_58: Hax_lib.Abstraction.t_Concretization t_Int i64 =
  {
    f_concretize_pre = (fun (self: t_Int) -> true);
    f_concretize_post = (fun (self: t_Int) (out: i64) -> true);
    f_concretize
    =
    fun (self: t_Int) ->
      let concretized:Core_models.Option.t_Option i64 =
        Num_traits.Cast.f_to_i64 #Num_bigint.Bigint.t_BigInt
          #FStar.Tactics.Typeclasses.solve
          (impl_1__get self <: Num_bigint.Bigint.t_BigInt)
      in
      let _:Prims.unit =
        if true
        then
          let _:Prims.unit = v_assert (Core_models.Option.impl__is_some #i64 concretized <: bool) in
          ()
      in
      Core_models.Option.impl__unwrap #i64 concretized
  }

let impl_59__to_i64 (self: t_Int) : i64 =
  Hax_lib.Abstraction.f_concretize #t_Int #i64 #FStar.Tactics.Typeclasses.solve self

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_60: Hax_lib.Abstraction.t_Concretization t_Int i128 =
  {
    f_concretize_pre = (fun (self: t_Int) -> true);
    f_concretize_post = (fun (self: t_Int) (out: i128) -> true);
    f_concretize
    =
    fun (self: t_Int) ->
      let concretized:Core_models.Option.t_Option i128 =
        Num_traits.Cast.f_to_i128 #Num_bigint.Bigint.t_BigInt
          #FStar.Tactics.Typeclasses.solve
          (impl_1__get self <: Num_bigint.Bigint.t_BigInt)
      in
      let _:Prims.unit =
        if true
        then
          let _:Prims.unit =
            v_assert (Core_models.Option.impl__is_some #i128 concretized <: bool)
          in
          ()
      in
      Core_models.Option.impl__unwrap #i128 concretized
  }

let impl_61__to_i128 (self: t_Int) : i128 =
  Hax_lib.Abstraction.f_concretize #t_Int #i128 #FStar.Tactics.Typeclasses.solve self

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_62: Hax_lib.Abstraction.t_Concretization t_Int isize =
  {
    f_concretize_pre = (fun (self: t_Int) -> true);
    f_concretize_post = (fun (self: t_Int) (out: isize) -> true);
    f_concretize
    =
    fun (self: t_Int) ->
      let concretized:Core_models.Option.t_Option isize =
        Num_traits.Cast.f_to_isize #Num_bigint.Bigint.t_BigInt
          #FStar.Tactics.Typeclasses.solve
          (impl_1__get self <: Num_bigint.Bigint.t_BigInt)
      in
      let _:Prims.unit =
        if true
        then
          let _:Prims.unit =
            v_assert (Core_models.Option.impl__is_some #isize concretized <: bool)
          in
          ()
      in
      Core_models.Option.impl__unwrap #isize concretized
  }

let impl_63__to_isize (self: t_Int) : isize =
  Hax_lib.Abstraction.f_concretize #t_Int #isize #FStar.Tactics.Typeclasses.solve self
