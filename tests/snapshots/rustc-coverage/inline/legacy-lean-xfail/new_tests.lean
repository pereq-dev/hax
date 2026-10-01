
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


namespace new_tests.rustc_coverage__inline

@[spec]
def length (T : Type) (xs : (RustSlice T)) : RustM usize := do
  (core_models.slice.Impl.len T xs)

@[spec]
def swap
    (T : Type)
    [trait_constr_swap_associated_type_i0 :
      core_models.marker.Copy.AssociatedTypes
      T]
    [trait_constr_swap_i0 : core_models.marker.Copy T ]
    (xs : (RustSlice T))
    (i : usize)
    (j : usize) :
    RustM (RustSlice T) := do
  let t : T ← xs[i]_?;
  let xs : (RustSlice T) ←
    (rust_primitives.hax.monomorphized_update_at.update_at_usize
      xs
      i
      (← xs[j]_?));
  let xs : (RustSlice T) ←
    (rust_primitives.hax.monomorphized_update_at.update_at_usize xs j t);
  (pure xs)

--  @fail(extraction): ssprove(HAX0001)
@[spec]
def display
    (T : Type)
    [trait_constr_display_associated_type_i0 :
      core_models.fmt.Display.AssociatedTypes
      T]
    [trait_constr_display_i0 : core_models.fmt.Display T ]
    (xs : (RustSlice T)) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ ←
    (core_models.iter.traits.iterator.Iterator.fold
      (← (core_models.iter.traits.collect.IntoIterator.into_iter
        (RustSlice T) xs))
      rust_primitives.hax.Tuple0.mk
      (fun _ x =>
        (do
        let args : (rust_primitives.hax.Tuple1 T) :=
          (rust_primitives.hax.Tuple1.mk x);
        let args : (RustArray core_models.fmt.rt.Argument 1) :=
          (RustArray.ofVec #v[(← (core_models.fmt.rt.Impl.new_display T
                                  (rust_primitives.hax.Tuple1._0 args)))]);
        let _ ←
          (std.io.stdio._print
            (← (core_models.fmt.rt.Impl_1.new_v1 ((1 : usize)) ((1 : usize))
              (RustArray.ofVec #v[""])
              args)));
        (pure rust_primitives.hax.Tuple0.mk) :
        RustM rust_primitives.hax.Tuple0)));
  let _ ←
    (std.io.stdio._print
      (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
        (RustArray.ofVec #v["\n"]))));
  let _ := rust_primitives.hax.Tuple0.mk;
  (pure rust_primitives.hax.Tuple0.mk)

@[spec]
def error (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  (rust_primitives.hax.never_to_any
    (← (core_models.panicking.panic_fmt
      (← (core_models.fmt.rt.Impl_1.new_const ((1 : usize))
        (RustArray.ofVec #v["error"]))))))

@[spec]
def permutate
    (T : Type)
    [trait_constr_permutate_associated_type_i0 :
      core_models.marker.Copy.AssociatedTypes
      T]
    [trait_constr_permutate_i0 : core_models.marker.Copy T ]
    [trait_constr_permutate_associated_type_i1 :
      core_models.fmt.Display.AssociatedTypes
      T]
    [trait_constr_permutate_i1 : core_models.fmt.Display T ]
    (xs : (RustSlice T))
    (k : usize) :
    RustM (RustSlice T) := do
  let n : usize ← (length T xs);
  let xs : (RustSlice T) ←
    if (← (k ==? n)) then do
      let _ ← (display T xs);
      (pure xs)
    else do
      if (← (k <? n)) then do
        (rust_primitives.hax.folds.fold_range
          k
          n
          (fun xs _ => (do (pure true) : RustM Bool))
          xs
          (fun xs i =>
            (do
            let xs : (RustSlice T) ← (swap T xs i k);
            let xs : (RustSlice T) ← (permutate T xs (← (k +? (1 : usize))));
            let xs : (RustSlice T) ← (swap T xs i k);
            (pure xs) :
            RustM (RustSlice T))))
      else do
        let _ ← (error rust_primitives.hax.Tuple0.mk);
        (pure xs);
  (pure xs)
partial_fixpoint

@[spec]
def permutations
    (T : Type)
    [trait_constr_permutations_associated_type_i0 :
      core_models.marker.Copy.AssociatedTypes
      T]
    [trait_constr_permutations_i0 : core_models.marker.Copy T ]
    [trait_constr_permutations_associated_type_i1 :
      core_models.fmt.Display.AssociatedTypes
      T]
    [trait_constr_permutations_i1 : core_models.fmt.Display T ]
    (xs : (RustSlice T)) :
    RustM rust_primitives.hax.Tuple0 := do
  let ys : (alloc.vec.Vec T alloc.alloc.Global) ←
    (alloc.borrow.ToOwned.to_owned (RustSlice T) xs);
  let ys : (alloc.vec.Vec T alloc.alloc.Global) ←
    (alloc.slice.Impl.to_vec
      (← (permutate T (← (alloc.vec.Impl_1.as_slice ys)) (0 : usize))));
  (pure rust_primitives.hax.Tuple0.mk)

--  @fail(extraction): ssprove(HAX0001)
@[spec]
def main (_ : rust_primitives.hax.Tuple0) :
    RustM rust_primitives.hax.Tuple0 := do
  let _ ←
    (permutations Char
      (← (rust_primitives.unsize (RustArray.ofVec #v['a', 'b', 'c']))));
  (pure rust_primitives.hax.Tuple0.mk)

end new_tests.rustc_coverage__inline

