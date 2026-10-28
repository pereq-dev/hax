
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


namespace new_tests.legacy__literals__lib

def math_integers (x : hax_lib.int.Int) : RustM u8 := do
  let _ : hax_lib.int.Int ← (rust_primitives.hax.int.from_machine (3 : usize));
  let _neg_dec : hax_lib.int.Int ←
    (hax_lib.int.Impl_7._unsafe_from_str
      "-340282366920938463463374607431768211455000");
  let _pos_dec : hax_lib.int.Int ←
    (hax_lib.int.Impl_7._unsafe_from_str
      "340282366920938463463374607431768211455000");
  let _neg_hex : hax_lib.int.Int ←
    (hax_lib.int.Impl_7._unsafe_from_str
      "-340282366920938463463374607431768211455000");
  let _pos_hex : hax_lib.int.Int ←
    (hax_lib.int.Impl_7._unsafe_from_str
      "340282366920938463463374607431768211455000");
  let _neg_octal : hax_lib.int.Int ←
    (hax_lib.int.Impl_7._unsafe_from_str
      "-340282366920938463463374607431768211455000");
  let _pos_octal : hax_lib.int.Int ←
    (hax_lib.int.Impl_7._unsafe_from_str
      "340282366920938463463374607431768211455000");
  let _neg_bin : hax_lib.int.Int ←
    (hax_lib.int.Impl_7._unsafe_from_str
      "-340282366920938463463374607431768211455000");
  let _pos_bin : hax_lib.int.Int ←
    (hax_lib.int.Impl_7._unsafe_from_str
      "340282366920938463463374607431768211455000");
  let _ ←
    (rust_primitives.hax.int.gt
      (← (hax_lib.int.Impl_7._unsafe_from_str
        "-340282366920938463463374607431768211455000"))
      (← (hax_lib.int.Impl_7._unsafe_from_str
        "340282366920938463463374607431768211455000")));
  let _ ← (rust_primitives.hax.int.lt x x);
  let _ ← (rust_primitives.hax.int.ge x x);
  let _ ← (rust_primitives.hax.int.le x x);
  let _ ← (rust_primitives.hax.int.ne x x);
  let _ ← (rust_primitives.hax.int.eq x x);
  let _ ← (rust_primitives.hax.int.add x x);
  let _ ← (rust_primitives.hax.int.sub x x);
  let _ ← (rust_primitives.hax.int.mul x x);
  let _ ← (rust_primitives.hax.int.div x x);
  let _ : i16 ← (hax_lib.int.Impl_55.to_i16 x);
  let _ : i32 ← (hax_lib.int.Impl_57.to_i32 x);
  let _ : i64 ← (hax_lib.int.Impl_59.to_i64 x);
  let _ : i128 ← (hax_lib.int.Impl_61.to_i128 x);
  let _ : isize ← (hax_lib.int.Impl_63.to_isize x);
  let _ : u16 ← (hax_lib.int.Impl_43.to_u16 x);
  let _ : u32 ← (hax_lib.int.Impl_45.to_u32 x);
  let _ : u64 ← (hax_lib.int.Impl_47.to_u64 x);
  let _ : u128 ← (hax_lib.int.Impl_49.to_u128 x);
  let _ : usize ← (hax_lib.int.Impl_51.to_usize x);
  (hax_lib.int.Impl_41.to_u8
    (← (rust_primitives.hax.int.add x (← (rust_primitives.hax.int.mul x x)))))

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def math_integers.spec (x : hax_lib.int.Int) :
    Spec
      (requires := do
        ((← (rust_primitives.hax.int.gt
            x
            (← (hax_lib.int.Impl_7._unsafe_from_str "0"))))
          &&? (← (rust_primitives.hax.int.lt
            x
            (← (hax_lib.int.Impl_7._unsafe_from_str "16"))))))
      (ensures := fun _ => pure True)
      (math_integers (x : hax_lib.int.Int)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by hax_mvcgen [math_integers] <;> bv_decide
}

@[spec]
def panic_with_msg (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  (rust_primitives.hax.never_to_any
    (← (core_models.panicking.panic_fmt
      (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
        (RustArray.ofVec #v["with msg"]))))))

structure Foo where
  field : u8

@[instance] opaque Impl.AssociatedTypes :
  core_models.marker.StructuralPartialEq.AssociatedTypes Foo :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl :
  core_models.marker.StructuralPartialEq Foo :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_1.AssociatedTypes :
  core_models.cmp.PartialEq.AssociatedTypes Foo Foo :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_1 :
  core_models.cmp.PartialEq Foo Foo :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_2.AssociatedTypes :
  core_models.cmp.Eq.AssociatedTypes Foo :=
  by constructor <;> exact Inhabited.default

@[instance] opaque Impl_2 :
  core_models.cmp.Eq Foo :=
  by constructor <;> exact Inhabited.default

def CONSTANT : Foo :=
  RustM.of_isOk (do (pure (Foo.mk (field := (3 : u8))))) (by rfl)

@[spec]
def numeric (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ : usize := (123 : usize);
  let _ : isize := (-42 : isize);
  let _ : isize := (42 : isize);
  let _ : i32 := (-42 : i32);
  let _ : u128 := (22222222222222222222 : u128);
  (pure rust_primitives.hax.Tuple0.mk)

--  @fail(extraction): ssprove(HAX0001)
@[spec]
def patterns (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ ←
    match (1 : u8) with
      | 2 => do (pure rust_primitives.hax.Tuple0.mk)
      | _ => do (pure rust_primitives.hax.Tuple0.mk);
  let _ ←
    match
      (rust_primitives.hax.Tuple2.mk
        "hello"
        (rust_primitives.hax.Tuple2.mk
          (123 : i32)
          (RustArray.ofVec #v["a", "b"])))
    with
      | ⟨"hello", ⟨123, _todo⟩⟩ => do (pure rust_primitives.hax.Tuple0.mk)
      | _ => do (pure rust_primitives.hax.Tuple0.mk);
  let _ ←
    match (Foo.mk (field := (4 : u8))) with
      | {field := 3} => do (pure rust_primitives.hax.Tuple0.mk)
      | _ => do (pure rust_primitives.hax.Tuple0.mk);
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def casts (x8 : u8) (x16 : u16) (x32 : u32) (x64 : u64) (xs : usize) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ : u64 ←
    ((← ((← ((← ((← (rust_primitives.hax.cast_op x8 : RustM u64))
            +? (← (rust_primitives.hax.cast_op x16 : RustM u64))))
          +? (← (rust_primitives.hax.cast_op x32 : RustM u64))))
        +? x64))
      +? (← (rust_primitives.hax.cast_op xs : RustM u64)));
  let _ : u32 ←
    ((← ((← ((← ((← (rust_primitives.hax.cast_op x8 : RustM u32))
            +? (← (rust_primitives.hax.cast_op x16 : RustM u32))))
          +? x32))
        +? (← (rust_primitives.hax.cast_op x64 : RustM u32))))
      +? (← (rust_primitives.hax.cast_op xs : RustM u32)));
  let _ : u16 ←
    ((← ((← ((← ((← (rust_primitives.hax.cast_op x8 : RustM u16)) +? x16))
          +? (← (rust_primitives.hax.cast_op x32 : RustM u16))))
        +? (← (rust_primitives.hax.cast_op x64 : RustM u16))))
      +? (← (rust_primitives.hax.cast_op xs : RustM u16)));
  let _ : u8 ←
    ((← ((← ((← (x8 +? (← (rust_primitives.hax.cast_op x16 : RustM u8))))
          +? (← (rust_primitives.hax.cast_op x32 : RustM u8))))
        +? (← (rust_primitives.hax.cast_op x64 : RustM u8))))
      +? (← (rust_primitives.hax.cast_op xs : RustM u8)));
  let _ : i64 ←
    ((← ((← ((← ((← (rust_primitives.hax.cast_op x8 : RustM i64))
            +? (← (rust_primitives.hax.cast_op x16 : RustM i64))))
          +? (← (rust_primitives.hax.cast_op x32 : RustM i64))))
        +? (← (rust_primitives.hax.cast_op x64 : RustM i64))))
      +? (← (rust_primitives.hax.cast_op xs : RustM i64)));
  let _ : i32 ←
    ((← ((← ((← ((← (rust_primitives.hax.cast_op x8 : RustM i32))
            +? (← (rust_primitives.hax.cast_op x16 : RustM i32))))
          +? (← (rust_primitives.hax.cast_op x32 : RustM i32))))
        +? (← (rust_primitives.hax.cast_op x64 : RustM i32))))
      +? (← (rust_primitives.hax.cast_op xs : RustM i32)));
  let _ : i16 ←
    ((← ((← ((← ((← (rust_primitives.hax.cast_op x8 : RustM i16))
            +? (← (rust_primitives.hax.cast_op x16 : RustM i16))))
          +? (← (rust_primitives.hax.cast_op x32 : RustM i16))))
        +? (← (rust_primitives.hax.cast_op x64 : RustM i16))))
      +? (← (rust_primitives.hax.cast_op xs : RustM i16)));
  let _ : i8 ←
    ((← ((← ((← ((← (rust_primitives.hax.cast_op x8 : RustM i8))
            +? (← (rust_primitives.hax.cast_op x16 : RustM i8))))
          +? (← (rust_primitives.hax.cast_op x32 : RustM i8))))
        +? (← (rust_primitives.hax.cast_op x64 : RustM i8))))
      +? (← (rust_primitives.hax.cast_op xs : RustM i8)));
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def empty_array (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ : (RustSlice u8) ← (rust_primitives.unsize (RustArray.ofVec #v[]));
  (pure rust_primitives.hax.Tuple0.mk)

--  https://github.com/pereq-dev/hax/issues/500
@[spec]
def fn_pointer_cast (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let f : (u32 -> RustM u32) := (fun x => (do (pure x) : RustM u32));
  (pure rust_primitives.hax.Tuple0.mk)

--  @fail(extraction): ssprove(HAX0001)
def null : Char := ' '

@[spec]
def strings (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ : String := "hello";
  let _ : String := "hello\"world";
  let _ : String := "it\'s";
  let _ : String := "back\\slash";
  let _ : String := "line\nbreak";
  let _ : String := "carriage\rreturn";
  let _ : String := "tab\there";
  let _ : String := "null\x00byte";
  let _ : String := "bell\x07char";
  let _ : String := "\x1b[0m";
  let _ : String := "🦀";
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.legacy__literals__lib

