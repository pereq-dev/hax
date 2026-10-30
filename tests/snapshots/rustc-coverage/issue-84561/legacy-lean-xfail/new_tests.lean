
-- Legacy lean backend for Hax
-- The Hax prelude library can be found in hax/proof-libs/legacy-lean
import Hax
import Std.Tactic.Do
import Std.Do.Triple
import Std.Tactic.Do.Syntax
open Std.Do
open Std.Tactic

set_option mvcgen.warning false
set_option linter.unusedVariables false


namespace new_tests.rustc_coverage__issue_84561

structure Foo where
  _0 : u32

@[instance] opaque Impl_1.AssociatedTypes :
  core_models.marker.StructuralPartialEq.AssociatedTypes Foo :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_1 :
  core_models.marker.StructuralPartialEq Foo :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_2.AssociatedTypes :
  core_models.cmp.PartialEq.AssociatedTypes Foo Foo :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_2 :
  core_models.cmp.PartialEq Foo Foo :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_3.AssociatedTypes :
  core_models.cmp.Eq.AssociatedTypes Foo :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_3 :
  core_models.cmp.Eq Foo :=
  by constructor <;> exact Inhabited.default

@[spec]
def Impl.fmt_hoisted (self : Foo) (f : core_models.fmt.Formatter) :
    RustM
    (rust_primitives.hax.Tuple2
      core_models.fmt.Formatter
      (core_models.result.Result
        rust_primitives.hax.Tuple0
        core_models.fmt.Error))
    := do
  let ⟨tmp0, out⟩ ←
    (core_models.fmt.Impl_11.write_fmt
      f
      (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
        (RustArray.ofVec #v["try and succeed"]))));
  let f : core_models.fmt.Formatter := tmp0;
  match out with
    | (core_models.result.Result.Ok  _) => do
      let
        hax_temp_output : (core_models.result.Result
          rust_primitives.hax.Tuple0
          core_models.fmt.Error) :=
        (core_models.result.Result.Ok rust_primitives.hax.Tuple0.mk);
      (pure (rust_primitives.hax.Tuple2.mk f hax_temp_output))
    | (core_models.result.Result.Err  err) => do
      (pure (rust_primitives.hax.Tuple2.mk
        f
        (core_models.result.Result.Err err)))

@[reducible] instance Impl.AssociatedTypes :
  core_models.fmt.Debug.AssociatedTypes Foo
  where

instance Impl : core_models.fmt.Debug Foo where
  fmt := (Impl.fmt_hoisted)

--  @fail(extraction): ssprove(HAX0001)
@[spec]
def test3 (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let is_true : Bool ←
    ((← (core_models.iter.traits.exact_size.ExactSizeIterator.len
        std.env.Args (← (std.env.args rust_primitives.hax.Tuple0.mk))))
      ==? (1 : usize));
  let bar : Foo := (Foo.mk (1 : u32));
  let _ ←
    match (rust_primitives.hax.Tuple2.mk bar (Foo.mk (1 : u32))) with
      | ⟨left_val, right_val⟩ => do
        (hax_lib.assert
          (← (core_models.cmp.PartialEq.eq Foo Foo left_val right_val)));
  let baz : Foo := (Foo.mk (0 : u32));
  let _ ←
    match (rust_primitives.hax.Tuple2.mk baz (Foo.mk (1 : u32))) with
      | ⟨left_val, right_val⟩ => do
        (hax_lib.assert
          (← (!? (← (core_models.cmp.PartialEq.eq
            Foo
            Foo left_val right_val)))));
  let args : (rust_primitives.hax.Tuple1 Foo) :=
    (rust_primitives.hax.Tuple1.mk (Foo.mk (1 : u32)));
  let args : (RustArray core_models.fmt.rt.Argument 1) :=
    (RustArray.ofVec #v[(← (core_models.fmt.rt.Impl.new_debug Foo
                            (rust_primitives.hax.Tuple1._0 args)))]);
  let _ ←
    (std.io.stdio._print
      (← (core_models.fmt.rt.Impl_1.new_v1 ((2 : usize)) ((1 : usize))
        (RustArray.ofVec #v["", "\n"])
        args)));
  let _ := rust_primitives.hax.Tuple0.mk;
  let args : (rust_primitives.hax.Tuple1 Foo) :=
    (rust_primitives.hax.Tuple1.mk bar);
  let args : (RustArray core_models.fmt.rt.Argument 1) :=
    (RustArray.ofVec #v[(← (core_models.fmt.rt.Impl.new_debug Foo
                            (rust_primitives.hax.Tuple1._0 args)))]);
  let _ ←
    (std.io.stdio._print
      (← (core_models.fmt.rt.Impl_1.new_v1 ((2 : usize)) ((1 : usize))
        (RustArray.ofVec #v["", "\n"])
        args)));
  let _ := rust_primitives.hax.Tuple0.mk;
  let args : (rust_primitives.hax.Tuple1 Foo) :=
    (rust_primitives.hax.Tuple1.mk baz);
  let args : (RustArray core_models.fmt.rt.Argument 1) :=
    (RustArray.ofVec #v[(← (core_models.fmt.rt.Impl.new_debug Foo
                            (rust_primitives.hax.Tuple1._0 args)))]);
  let _ ←
    (std.io.stdio._print
      (← (core_models.fmt.rt.Impl_1.new_v1 ((2 : usize)) ((1 : usize))
        (RustArray.ofVec #v["", "\n"])
        args)));
  let _ := rust_primitives.hax.Tuple0.mk;
  let _ ←
    match
      (rust_primitives.hax.Tuple2.mk (Foo.mk (1 : u32)) (Foo.mk (1 : u32)))
    with
      | ⟨left_val, right_val⟩ => do
        (hax_lib.assert
          (← (core_models.cmp.PartialEq.eq Foo Foo left_val right_val)));
  let _ ←
    match
      (rust_primitives.hax.Tuple2.mk (Foo.mk (0 : u32)) (Foo.mk (1 : u32)))
    with
      | ⟨left_val, right_val⟩ => do
        (hax_lib.assert
          (← (!? (← (core_models.cmp.PartialEq.eq
            Foo
            Foo left_val right_val)))));
  let _ ←
    match
      (rust_primitives.hax.Tuple2.mk (Foo.mk (2 : u32)) (Foo.mk (2 : u32)))
    with
      | ⟨left_val, right_val⟩ => do
        (hax_lib.assert
          (← (core_models.cmp.PartialEq.eq Foo Foo left_val right_val)));
  let bar : Foo := (Foo.mk (0 : u32));
  let _ ←
    match (rust_primitives.hax.Tuple2.mk bar (Foo.mk (3 : u32))) with
      | ⟨left_val, right_val⟩ => do
        (hax_lib.assert
          (← (!? (← (core_models.cmp.PartialEq.eq
            Foo
            Foo left_val right_val)))));
  let _ ←
    match
      (rust_primitives.hax.Tuple2.mk (Foo.mk (0 : u32)) (Foo.mk (4 : u32)))
    with
      | ⟨left_val, right_val⟩ => do
        (hax_lib.assert
          (← (!? (← (core_models.cmp.PartialEq.eq
            Foo
            Foo left_val right_val)))));
  let _ ←
    match
      (rust_primitives.hax.Tuple2.mk (Foo.mk (3 : u32)) (Foo.mk (3 : u32)))
    with
      | ⟨left_val, right_val⟩ => do
        (hax_lib.assert
          (← (core_models.cmp.PartialEq.eq Foo Foo left_val right_val)));
  let args : (rust_primitives.hax.Tuple1 Foo) :=
    (rust_primitives.hax.Tuple1.mk bar);
  let args : (RustArray core_models.fmt.rt.Argument 1) :=
    (RustArray.ofVec #v[(← (core_models.fmt.rt.Impl.new_debug Foo
                            (rust_primitives.hax.Tuple1._0 args)))]);
  let _ ←
    (std.io.stdio._print
      (← (core_models.fmt.rt.Impl_1.new_v1 ((2 : usize)) ((1 : usize))
        (RustArray.ofVec #v["", "\n"])
        args)));
  let _ := rust_primitives.hax.Tuple0.mk;
  let args : (rust_primitives.hax.Tuple1 Foo) :=
    (rust_primitives.hax.Tuple1.mk (Foo.mk (1 : u32)));
  let args : (RustArray core_models.fmt.rt.Argument 1) :=
    (RustArray.ofVec #v[(← (core_models.fmt.rt.Impl.new_debug Foo
                            (rust_primitives.hax.Tuple1._0 args)))]);
  let _ ←
    (std.io.stdio._print
      (← (core_models.fmt.rt.Impl_1.new_v1 ((2 : usize)) ((1 : usize))
        (RustArray.ofVec #v["", "\n"])
        args)));
  let _ := rust_primitives.hax.Tuple0.mk;
  let _ ←
    match
      (rust_primitives.hax.Tuple2.mk (Foo.mk (0 : u32)) (Foo.mk (5 : u32)))
    with
      | ⟨left_val, right_val⟩ => do
        (hax_lib.assert
          (← (!? (← (core_models.cmp.PartialEq.eq
            Foo
            Foo left_val right_val)))));
  let _ ←
    match
      (rust_primitives.hax.Tuple2.mk (Foo.mk (0 : u32)) (Foo.mk (5 : u32)))
    with
      | ⟨left_val, right_val⟩ => do
        (hax_lib.assert
          (← (!? (← (core_models.cmp.PartialEq.eq
            Foo
            Foo left_val right_val)))));
  let is_true : Bool ←
    ((← (core_models.iter.traits.exact_size.ExactSizeIterator.len
        std.env.Args (← (std.env.args rust_primitives.hax.Tuple0.mk))))
      ==? (1 : usize));
  let _ ←
    match
      (rust_primitives.hax.Tuple2.mk (Foo.mk (1 : u32)) (Foo.mk (1 : u32)))
    with
      | ⟨left_val, right_val⟩ => do
        (hax_lib.assert
          (← (core_models.cmp.PartialEq.eq Foo Foo left_val right_val)));
  let _ ←
    match
      (rust_primitives.hax.Tuple2.mk (Foo.mk (0 : u32)) (Foo.mk (1 : u32)))
    with
      | ⟨left_val, right_val⟩ => do
        (hax_lib.assert
          (← (!? (← (core_models.cmp.PartialEq.eq
            Foo
            Foo left_val right_val)))));
  let _ ←
    match
      (rust_primitives.hax.Tuple2.mk (Foo.mk (2 : u32)) (Foo.mk (2 : u32)))
    with
      | ⟨left_val, right_val⟩ => do
        (hax_lib.assert
          (← (core_models.cmp.PartialEq.eq Foo Foo left_val right_val)));
  let bar : Foo := (Foo.mk (1 : u32));
  let _ ←
    match (rust_primitives.hax.Tuple2.mk bar (Foo.mk (3 : u32))) with
      | ⟨left_val, right_val⟩ => do
        (hax_lib.assert
          (← (!? (← (core_models.cmp.PartialEq.eq
            Foo
            Foo left_val right_val)))));
  let _ ←
    if is_true then do
      let _ ←
        match
          (rust_primitives.hax.Tuple2.mk (Foo.mk (0 : u32)) (Foo.mk (4 : u32)))
        with
          | ⟨left_val, right_val⟩ => do
            (hax_lib.assert
              (← (!? (← (core_models.cmp.PartialEq.eq
                Foo
                Foo left_val right_val)))));
      (pure rust_primitives.hax.Tuple0.mk)
    else do
      let _ ←
        match
          (rust_primitives.hax.Tuple2.mk (Foo.mk (3 : u32)) (Foo.mk (3 : u32)))
        with
          | ⟨left_val, right_val⟩ => do
            (hax_lib.assert
              (← (core_models.cmp.PartialEq.eq Foo Foo left_val right_val)));
      (pure rust_primitives.hax.Tuple0.mk);
  let _ ←
    if is_true then do
      let _ ←
        match
          (rust_primitives.hax.Tuple2.mk (Foo.mk (0 : u32)) (Foo.mk (4 : u32)))
        with
          | ⟨left_val, right_val⟩ => do
            (hax_lib.assert
              (← (!? (← (core_models.cmp.PartialEq.eq
                Foo
                Foo left_val right_val)))));
      (pure rust_primitives.hax.Tuple0.mk)
    else do
      let _ ←
        match
          (rust_primitives.hax.Tuple2.mk (Foo.mk (3 : u32)) (Foo.mk (3 : u32)))
        with
          | ⟨left_val, right_val⟩ => do
            (hax_lib.assert
              (← (core_models.cmp.PartialEq.eq Foo Foo left_val right_val)));
      (pure rust_primitives.hax.Tuple0.mk);
  let _ ←
    match
      (rust_primitives.hax.Tuple2.mk
        (← if is_true then do
          (pure (Foo.mk (0 : u32)))
        else do
          (pure (Foo.mk (1 : u32))))
        (Foo.mk (5 : u32)))
    with
      | ⟨left_val, right_val⟩ => do
        (hax_lib.assert
          (← (!? (← (core_models.cmp.PartialEq.eq
            Foo
            Foo left_val right_val)))));
  let _ ←
    match
      (rust_primitives.hax.Tuple2.mk
        (Foo.mk (5 : u32))
        (← if is_true then do
          (pure (Foo.mk (0 : u32)))
        else do
          (pure (Foo.mk (1 : u32)))))
    with
      | ⟨left_val, right_val⟩ => do
        (hax_lib.assert
          (← (!? (← (core_models.cmp.PartialEq.eq
            Foo
            Foo left_val right_val)))));
  let _ ←
    match
      (rust_primitives.hax.Tuple2.mk
        (← if is_true then do
          let _ ←
            match
              (rust_primitives.hax.Tuple2.mk
                (Foo.mk (3 : u32))
                (Foo.mk (3 : u32)))
            with
              | ⟨left_val, right_val⟩ => do
                (hax_lib.assert
                  (← (core_models.cmp.PartialEq.eq
                    Foo
                    Foo left_val right_val)));
          (pure (Foo.mk (0 : u32)))
        else do
          let _ ←
            match
              (rust_primitives.hax.Tuple2.mk
                (← if is_true then do
                  (pure (Foo.mk (0 : u32)))
                else do
                  (pure (Foo.mk (1 : u32))))
                (Foo.mk (5 : u32)))
            with
              | ⟨left_val, right_val⟩ => do
                (hax_lib.assert
                  (← (!? (← (core_models.cmp.PartialEq.eq
                    Foo
                    Foo left_val right_val)))));
          (pure (Foo.mk (1 : u32))))
        (Foo.mk (5 : u32)))
    with
      | ⟨left_val, right_val⟩ => do
        (hax_lib.assert
          (← (!? (← (core_models.cmp.PartialEq.eq
            Foo
            Foo left_val right_val)))));
  let _ ←
    match
      (rust_primitives.hax.Tuple2.mk (Foo.mk (1 : u32)) (Foo.mk (3 : u32)))
    with
      | ⟨left_val, right_val⟩ => do
        (hax_lib.assert
          (← (core_models.cmp.PartialEq.eq Foo Foo left_val right_val)));
  let _ ←
    match
      (rust_primitives.hax.Tuple2.mk (Foo.mk (3 : u32)) (Foo.mk (3 : u32)))
    with
      | ⟨left_val, right_val⟩ => do
        (hax_lib.assert
          (← (core_models.cmp.PartialEq.eq Foo Foo left_val right_val)));
  (pure rust_primitives.hax.Tuple0.mk)

--  @fail(extraction): ssprove(HAX0002, HAX0008, HAX0008, HAX0008), coq(HAX0008, HAX0008, HAX0008, HAX0002), fstar(HAX0002, HAX0002, HAX0008, HAX0008, HAX0008), legacy-lean(HAX0002, HAX0002, HAX0008, HAX0008, HAX0008)
--  @fail(extraction): proverif(HAX0008, HAX0008, HAX0008, HAX0002)
@[spec]
def test1 (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ ←
    if sorry then do
      let _ ←
        (std.io.stdio._print
          (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
            (RustArray.ofVec #v["debug is enabled\n"]))));
      let _ := rust_primitives.hax.Tuple0.mk;
      (pure rust_primitives.hax.Tuple0.mk)
    else do
      (pure rust_primitives.hax.Tuple0.mk);
  let _ ←
    if sorry then do
      let _ ←
        (std.io.stdio._print
          (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
            (RustArray.ofVec #v["debug is enabled\n"]))));
      let _ := rust_primitives.hax.Tuple0.mk;
      (pure rust_primitives.hax.Tuple0.mk)
    else do
      (pure rust_primitives.hax.Tuple0.mk);
  let _ := (0 : i32);
  let _ ←
    if sorry then do
      let _ ←
        (std.io.stdio._print
          (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
            (RustArray.ofVec #v["debug is enabled\n"]))));
      let _ := rust_primitives.hax.Tuple0.mk;
      (pure rust_primitives.hax.Tuple0.mk)
    else do
      (pure rust_primitives.hax.Tuple0.mk);
  let _ := sorry;
  let _ := rust_primitives.hax.Tuple0.mk;
  let _ ←
    if sorry then do
      let _ ←
        (std.io.stdio._print
          (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
            (RustArray.ofVec #v["debug is enabled\n"]))));
      let _ := rust_primitives.hax.Tuple0.mk;
      (pure rust_primitives.hax.Tuple0.mk)
    else do
      (pure rust_primitives.hax.Tuple0.mk);
  (pure rust_primitives.hax.Tuple0.mk)

--  @fail(extraction): ssprove(HAX0001)
@[spec]
def test2.call_print (s : String) : RustM rust_primitives.hax.Tuple0 := do
  let args : (rust_primitives.hax.Tuple1 String) :=
    (rust_primitives.hax.Tuple1.mk s);
  let args : (RustArray core_models.fmt.rt.Argument 1) :=
    (RustArray.ofVec #v[(← (core_models.fmt.rt.Impl.new_display String
                            (rust_primitives.hax.Tuple1._0 args)))]);
  let _ ←
    (std.io.stdio._print
      (← (core_models.fmt.rt.Impl_1.new_v1 ((1 : usize)) ((1 : usize))
        (RustArray.ofVec #v[""])
        args)));
  let _ := rust_primitives.hax.Tuple0.mk;
  (pure rust_primitives.hax.Tuple0.mk)

--  @fail(extraction): ssprove(HAX0008), coq(HAX0008), legacy-lean(HAX0002, HAX0008), fstar(HAX0002, HAX0008)
--  @fail(extraction): proverif(HAX0008)
@[spec]
def test2 (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ ← (test2.call_print "called from call_debug: ");
  let _ ←
    if sorry then do
      let _ ←
        (std.io.stdio._print
          (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
            (RustArray.ofVec #v["debug is enabled\n"]))));
      let _ := rust_primitives.hax.Tuple0.mk;
      (pure rust_primitives.hax.Tuple0.mk)
    else do
      (pure rust_primitives.hax.Tuple0.mk);
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ ← (test1 rust_primitives.hax.Tuple0.mk);
  let _ ← (test2 rust_primitives.hax.Tuple0.mk);
  let _ ← (test3 rust_primitives.hax.Tuple0.mk);
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.rustc_coverage__issue_84561

