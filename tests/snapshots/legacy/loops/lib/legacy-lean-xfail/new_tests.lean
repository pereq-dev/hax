
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


namespace new_tests.legacy__loops__lib.recognized_loops

@[spec]
def range (_ : rust_primitives.hax.Tuple0) :
    RustM (rust_primitives.hax.Tuple2 u64 rust_primitives.hax.Tuple0) := do
  let count : u64 := (0 : u64);
  (pure (rust_primitives.hax.Tuple2.mk
    (← (rust_primitives.hax.folds.fold_range
      (0 : u8)
      (10 : u8)
      (fun count i => (do (i <=? (10 : u8)) : RustM Bool))
      count
      (fun count i =>
        (do let count : u64 ← (count +? (1 : u64)); (pure count) : RustM u64))))
    rust_primitives.hax.Tuple0.mk))

@[spec]
def range_step_by (_ : rust_primitives.hax.Tuple0) :
    RustM (rust_primitives.hax.Tuple2 u64 rust_primitives.hax.Tuple0) := do
  let count : u64 := (0 : u64);
  (pure (rust_primitives.hax.Tuple2.mk
    (← (rust_primitives.hax.folds.fold_range_step_by
      (0 : u8)
      (10 : u8)
      (2 : usize)
      (fun count i => (do (i <=? (10 : u8)) : RustM Bool))
      count
      (fun count i =>
        (do let count : u64 ← (count +? (1 : u64)); (pure count) : RustM u64))))
    rust_primitives.hax.Tuple0.mk))

@[spec]
def enumerated_slice (T : Type) (slice : (RustSlice T)) :
    RustM (rust_primitives.hax.Tuple2 u64 rust_primitives.hax.Tuple0) := do
  let count : u64 := (0 : u64);
  (pure (rust_primitives.hax.Tuple2.mk
    (← (rust_primitives.hax.folds.fold_enumerated_slice
      slice
      (fun count i => (do (i <=? (10 : usize)) : RustM Bool))
      count
      (fun count i =>
        (do let count : u64 ← (count +? (2 : u64)); (pure count) : RustM u64))))
    rust_primitives.hax.Tuple0.mk))

@[spec]
def enumerated_chunked_slice (T : Type) (slice : (RustSlice T)) :
    RustM (rust_primitives.hax.Tuple2 u64 rust_primitives.hax.Tuple0) := do
  let count : u64 := (0 : u64);
  (pure (rust_primitives.hax.Tuple2.mk
    (← (rust_primitives.hax.folds.fold_enumerated_chunked_slice
      (3 : usize)
      slice
      (fun count i =>
        (do (hax_lib.prop.Impl.from_bool true) : RustM hax_lib.prop.Prop))
      count
      (fun count i =>
        (do let count : u64 ← (count +? (3 : u64)); (pure count) : RustM u64))))
    rust_primitives.hax.Tuple0.mk))

end new_tests.legacy__loops__lib.recognized_loops


namespace new_tests.legacy__loops__lib.for_loops

@[spec]
def range1 (_ : rust_primitives.hax.Tuple0) : RustM usize := do
  let acc : usize := (0 : usize);
  let acc : usize ←
    (rust_primitives.hax.folds.fold_range
      (0 : usize)
      (15 : usize)
      (fun acc _ => (do (pure true) : RustM Bool))
      acc
      (fun acc i => (do (acc +? i) : RustM usize)));
  (pure acc)

@[spec]
def range2 (n : usize) : RustM usize := do
  let acc : usize := (0 : usize);
  let acc : usize ←
    (rust_primitives.hax.folds.fold_range
      (0 : usize)
      (← (n +? (10 : usize)))
      (fun acc _ => (do (pure true) : RustM Bool))
      acc
      (fun acc i => (do ((← (acc +? i)) +? (1 : usize)) : RustM usize)));
  (pure acc)

@[spec]
def composed_range (n : usize) : RustM usize := do
  let acc : usize := (0 : usize);
  let acc : usize ←
    (core_models.iter.traits.iterator.Iterator.fold
      (← (core_models.iter.traits.collect.IntoIterator.into_iter
        (core_models.iter.adapters.chain.Chain
          (core_models.ops.range.Range usize)
          (core_models.ops.range.Range usize))
        (← (core_models.iter.traits.iterator.Iterator.chain
          (core_models.ops.range.Range usize)
          (core_models.ops.range.Range usize)
          (core_models.ops.range.Range.mk (start := (0 : usize)) (_end := n))
          (core_models.ops.range.Range.mk
            (start := (← (n +? (10 : usize))))
            (_end := (← (n +? (50 : usize)))))))))
      acc
      (fun acc i => (do ((← (acc +? i)) +? (1 : usize)) : RustM usize)));
  (pure acc)

@[spec]
def rev_range (n : usize) : RustM usize := do
  let acc : usize := (0 : usize);
  let acc : usize ←
    (core_models.iter.traits.iterator.Iterator.fold
      (← (core_models.iter.traits.collect.IntoIterator.into_iter
        (core_models.iter.adapters.rev.Rev (core_models.ops.range.Range usize))
        (← (core_models.iter.traits.iterator.Iterator.rev
          (core_models.ops.range.Range usize)
          (core_models.ops.range.Range.mk
            (start := (0 : usize))
            (_end := n))))))
      acc
      (fun acc i => (do ((← (acc +? i)) +? (1 : usize)) : RustM usize)));
  (pure acc)

@[spec]
def chunks (CHUNK_LEN : usize)
    (arr : (alloc.vec.Vec usize alloc.alloc.Global)) :
    RustM usize := do
  let acc : usize := (0 : usize);
  let chunks : (core_models.slice.iter.ChunksExact usize) ←
    (core_models.slice.Impl.chunks_exact usize
      (← (core_models.ops.deref.Deref.deref
        (alloc.vec.Vec usize alloc.alloc.Global) arr))
      CHUNK_LEN);
  let acc : usize ←
    (core_models.iter.traits.iterator.Iterator.fold
      (← (core_models.iter.traits.collect.IntoIterator.into_iter
        (core_models.slice.iter.ChunksExact usize)
        (← (core_models.clone.Clone.clone
          (core_models.slice.iter.ChunksExact usize) chunks))))
      acc
      (fun acc chunk =>
        (do
        let mean : usize := (0 : usize);
        let mean : usize ←
          (core_models.iter.traits.iterator.Iterator.fold
            (← (core_models.iter.traits.collect.IntoIterator.into_iter
              (RustSlice usize) chunk))
            mean
            (fun mean item => (do (mean +? item) : RustM usize)));
        let acc : usize ← (acc +? (← (mean /? CHUNK_LEN)));
        (pure acc) :
        RustM usize)));
  let acc : usize ←
    (core_models.iter.traits.iterator.Iterator.fold
      (← (core_models.iter.traits.collect.IntoIterator.into_iter
        (RustSlice usize)
        (← (core_models.slice.iter.Impl_88.remainder usize chunks))))
      acc
      (fun acc item => (do (acc -? item) : RustM usize)));
  (pure acc)

@[spec]
def iterator (arr : (alloc.vec.Vec usize alloc.alloc.Global)) :
    RustM usize := do
  let acc : usize := (0 : usize);
  let acc : usize ←
    (core_models.iter.traits.iterator.Iterator.fold
      (← (core_models.iter.traits.collect.IntoIterator.into_iter
        (core_models.slice.iter.Iter usize)
        (← (core_models.slice.Impl.iter usize
          (← (core_models.ops.deref.Deref.deref
            (alloc.vec.Vec usize alloc.alloc.Global) arr))))))
      acc
      (fun acc item => (do (acc +? item) : RustM usize)));
  (pure acc)

--  @fail(extraction): ssprove(HAX0001)
@[spec]
def nested (arr : (alloc.vec.Vec usize alloc.alloc.Global)) : RustM usize := do
  let acc : usize := (0 : usize);
  let acc : usize ←
    (core_models.iter.traits.iterator.Iterator.fold
      (← (core_models.iter.traits.collect.IntoIterator.into_iter
        (core_models.slice.iter.Iter usize)
        (← (core_models.slice.Impl.iter usize
          (← (core_models.ops.deref.Deref.deref
            (alloc.vec.Vec usize alloc.alloc.Global) arr))))))
      acc
      (fun acc item =>
        (do
        (core_models.iter.traits.iterator.Iterator.fold
          (← (core_models.iter.traits.collect.IntoIterator.into_iter
            (core_models.iter.adapters.rev.Rev
              (core_models.ops.range.Range usize))
            (← (core_models.iter.traits.iterator.Iterator.rev
              (core_models.ops.range.Range usize)
              (core_models.ops.range.Range.mk
                (start := (0 : usize))
                (_end := item))))))
          acc
          (fun acc i =>
            (do
            let acc : usize ← (acc +? (1 : usize));
            (core_models.iter.traits.iterator.Iterator.fold
              (← (core_models.iter.traits.collect.IntoIterator.into_iter
                (core_models.iter.adapters.zip.Zip
                  (core_models.slice.iter.Iter usize)
                  (core_models.ops.range.Range usize))
                (← (core_models.iter.traits.iterator.Iterator.zip
                  (core_models.slice.iter.Iter usize)
                  (core_models.ops.range.Range usize)
                  (← (core_models.slice.Impl.iter usize
                    (← (core_models.ops.deref.Deref.deref
                      (alloc.vec.Vec usize alloc.alloc.Global) arr))))
                  (core_models.ops.range.Range.mk
                    (start := (4 : usize))
                    (_end := i))))))
              acc
              (fun acc j =>
                (do
                ((← ((← ((← (acc +? item)) +? i))
                    +? (rust_primitives.hax.Tuple2._0 j)))
                  +? (rust_primitives.hax.Tuple2._1 j)) :
                RustM usize))) :
            RustM usize))) :
        RustM usize)));
  (pure acc)

@[spec]
def pattern
    (arr :
    (alloc.vec.Vec (rust_primitives.hax.Tuple2 usize usize) alloc.alloc.Global))
    :
    RustM usize := do
  let acc : usize := (0 : usize);
  let acc : usize ←
    (core_models.iter.traits.iterator.Iterator.fold
      (← (core_models.iter.traits.collect.IntoIterator.into_iter
        (alloc.vec.Vec
          (rust_primitives.hax.Tuple2 usize usize)
          alloc.alloc.Global) arr))
      acc
      (fun acc ⟨x, y⟩ => (do (acc +? (← (x *? y))) : RustM usize)));
  (pure acc)

@[spec]
def enumerate_chunks (arr : (alloc.vec.Vec usize alloc.alloc.Global)) :
    RustM usize := do
  let acc : usize := (0 : usize);
  let acc : usize ←
    (core_models.iter.traits.iterator.Iterator.fold
      (← (core_models.iter.traits.collect.IntoIterator.into_iter
        (core_models.iter.adapters.enumerate.Enumerate
          (core_models.slice.iter.Chunks usize))
        (← (core_models.iter.traits.iterator.Iterator.enumerate
          (core_models.slice.iter.Chunks usize)
          (← (core_models.slice.Impl.chunks usize
            (← (core_models.ops.deref.Deref.deref
              (alloc.vec.Vec usize alloc.alloc.Global) arr))
            (4 : usize)))))))
      acc
      (fun acc ⟨i, chunk⟩ =>
        (do
        (rust_primitives.hax.folds.fold_enumerated_slice
          chunk
          (fun acc _ => (do (pure true) : RustM Bool))
          acc
          (fun acc ⟨j, x⟩ => (do ((← (i +? j)) +? x) : RustM usize))) :
        RustM usize)));
  (pure acc)

@[spec]
def bool_returning (x : u8) : RustM Bool := do (x <? (10 : u8))

@[spec]
def f (_ : rust_primitives.hax.Tuple0) :
    RustM (rust_primitives.hax.Tuple2 u8 rust_primitives.hax.Tuple0) := do
  let acc : u8 := (0 : u8);
  (pure (rust_primitives.hax.Tuple2.mk
    (← (rust_primitives.hax.folds.fold_range
      (1 : u8)
      (10 : u8)
      (fun acc _ => (do (pure true) : RustM Bool))
      acc
      (fun acc i =>
        (do
        let acc : u8 ← (acc +? i);
        let _ ← (bool_returning i);
        (pure acc) :
        RustM u8))))
    rust_primitives.hax.Tuple0.mk))

end new_tests.legacy__loops__lib.for_loops


namespace new_tests.legacy__loops__lib.while_loops

--  @fail(extraction): coq(HAX0001, HAX0001), ssprove(HAX0001)
@[spec]
def f (_ : rust_primitives.hax.Tuple0) : RustM u8 := do
  let x : u8 := (0 : u8);
  let x : u8 ←
    (rust_primitives.hax.while_loop
      (fun x => (do (pure true) : RustM Bool))
      (fun x => (do (x <? (10 : u8)) : RustM Bool))
      (fun x =>
        (do
        (rust_primitives.hax.int.from_machine (0 : u32)) :
        RustM hax_lib.int.Int))
      x
      (fun x => (do let x : u8 ← (x +? (3 : u8)); (pure x) : RustM u8)));
  (x +? (12 : u8))

--  @fail(extraction): ssprove(HAX0001), coq(HAX0001, HAX0001)
@[spec]
def while_invariant_decr (_ : rust_primitives.hax.Tuple0) : RustM u8 := do
  let x : u8 := (0 : u8);
  let x : u8 ←
    (rust_primitives.hax.while_loop
      (fun x =>
        (do
        (hax_lib.prop.constructors.from_bool (← (x <=? (10 : u8)))) :
        RustM hax_lib.prop.Prop))
      (fun x => (do (x <? (10 : u8)) : RustM Bool))
      (fun x =>
        (do
        (rust_primitives.hax.int.from_machine (← ((10 : u8) -? x))) :
        RustM hax_lib.int.Int))
      x
      (fun x => (do let x : u8 ← (x +? (3 : u8)); (pure x) : RustM u8)));
  (x +? (12 : u8))

--  @fail(extraction): ssprove(HAX0001), coq(HAX0001, HAX0001)
@[spec]
def while_invariant_decr_rev (_ : rust_primitives.hax.Tuple0) : RustM u8 := do
  let x : u8 := (0 : u8);
  let x : u8 ←
    (rust_primitives.hax.while_loop
      (fun x =>
        (do
        (hax_lib.prop.constructors.from_bool (← (x <=? (10 : u8)))) :
        RustM hax_lib.prop.Prop))
      (fun x => (do (x <? (10 : u8)) : RustM Bool))
      (fun x =>
        (do
        (rust_primitives.hax.int.from_machine (← ((10 : u8) -? x))) :
        RustM hax_lib.int.Int))
      x
      (fun x => (do let x : u8 ← (x +? (3 : u8)); (pure x) : RustM u8)));
  (x +? (12 : u8))

end new_tests.legacy__loops__lib.while_loops


namespace new_tests.legacy__loops__lib.control_flow

--  @fail(extraction): coq(HAX0001), ssprove(HAX0001)
@[spec]
def double_sum (_ : rust_primitives.hax.Tuple0) : RustM i32 := do
  let sum : i32 := (0 : i32);
  let sum : i32 ←
    (rust_primitives.hax.folds.fold_range_cf
      (1 : i32)
      (10 : i32)
      (fun sum _ => (do (pure true) : RustM Bool))
      sum
      (fun sum i =>
        (do
        if (← (i <? (0 : i32))) then do
          (pure (core_models.ops.control_flow.ControlFlow.Break
            (rust_primitives.hax.Tuple2.mk rust_primitives.hax.Tuple0.mk sum)))
        else do
          (pure (core_models.ops.control_flow.ControlFlow.Continue
            (← (sum +? i)))) :
        RustM
        (core_models.ops.control_flow.ControlFlow
          (rust_primitives.hax.Tuple2 rust_primitives.hax.Tuple0 i32)
          i32))));
  (sum *? (2 : i32))

--  @fail(extraction): coq(HAX0001), ssprove(HAX0001)
@[spec]
def double_sum2 (_ : rust_primitives.hax.Tuple0) : RustM i32 := do
  let sum : i32 := (0 : i32);
  let sum2 : i32 := (0 : i32);
  let ⟨sum, sum2⟩ ←
    (rust_primitives.hax.folds.fold_range_cf
      (1 : i32)
      (10 : i32)
      (fun ⟨sum, sum2⟩ _ => (do (pure true) : RustM Bool))
      (rust_primitives.hax.Tuple2.mk sum sum2)
      (fun ⟨sum, sum2⟩ i =>
        (do
        if (← (i <? (0 : i32))) then do
          (pure (core_models.ops.control_flow.ControlFlow.Break
            (rust_primitives.hax.Tuple2.mk
              rust_primitives.hax.Tuple0.mk
              (rust_primitives.hax.Tuple2.mk sum sum2))))
        else do
          let sum : i32 ← (sum +? i);
          (pure (core_models.ops.control_flow.ControlFlow.Continue
            (rust_primitives.hax.Tuple2.mk sum (← (sum2 +? i))))) :
        RustM
        (core_models.ops.control_flow.ControlFlow
          (rust_primitives.hax.Tuple2
            rust_primitives.hax.Tuple0
            (rust_primitives.hax.Tuple2 i32 i32))
          (rust_primitives.hax.Tuple2 i32 i32)))));
  (sum +? sum2)

@[spec]
def double_sum_return (v : (RustSlice i32)) : RustM i32 := do
  let sum : i32 := (0 : i32);
  match
    (← (rust_primitives.hax.folds.fold_return
      (← (core_models.iter.traits.collect.IntoIterator.into_iter
        (RustSlice i32) v))
      sum
      (fun sum i =>
        (do
        if (← (i <? (0 : i32))) then do
          (pure (core_models.ops.control_flow.ControlFlow.Break
            (core_models.ops.control_flow.ControlFlow.Break (0 : i32))))
        else do
          (pure (core_models.ops.control_flow.ControlFlow.Continue
            (← (sum +? i)))) :
        RustM
        (core_models.ops.control_flow.ControlFlow
          (core_models.ops.control_flow.ControlFlow
            i32
            (rust_primitives.hax.Tuple2 rust_primitives.hax.Tuple0 i32))
          i32)))))
  with
    | (core_models.ops.control_flow.ControlFlow.Break  ret) => do (pure ret)
    | (core_models.ops.control_flow.ControlFlow.Continue  sum) => do
      (sum *? (2 : i32))

@[spec]
def double_sum2_return (v : (RustSlice i32)) : RustM i32 := do
  let sum : i32 := (0 : i32);
  let sum2 : i32 := (0 : i32);
  match
    (← (rust_primitives.hax.folds.fold_return
      (← (core_models.iter.traits.collect.IntoIterator.into_iter
        (RustSlice i32) v))
      (rust_primitives.hax.Tuple2.mk sum sum2)
      (fun ⟨sum, sum2⟩ i =>
        (do
        if (← (i <? (0 : i32))) then do
          (pure (core_models.ops.control_flow.ControlFlow.Break
            (core_models.ops.control_flow.ControlFlow.Break (0 : i32))))
        else do
          let sum : i32 ← (sum +? i);
          (pure (core_models.ops.control_flow.ControlFlow.Continue
            (rust_primitives.hax.Tuple2.mk sum (← (sum2 +? i))))) :
        RustM
        (core_models.ops.control_flow.ControlFlow
          (core_models.ops.control_flow.ControlFlow
            i32
            (rust_primitives.hax.Tuple2
              rust_primitives.hax.Tuple0
              (rust_primitives.hax.Tuple2 i32 i32)))
          (rust_primitives.hax.Tuple2 i32 i32))))))
  with
    | (core_models.ops.control_flow.ControlFlow.Break  ret) => do (pure ret)
    | (core_models.ops.control_flow.ControlFlow.Continue  ⟨sum, sum2⟩) => do
      (sum +? sum2)

--  @fail(extraction): coq(HAX0001, HAX0001, HAX0001), ssprove(HAX0001, HAX0001)
@[spec]
def bigger_power_2 (x : i32) : RustM i32 := do
  let pow : i32 := (1 : i32);
  (rust_primitives.hax.while_loop_cf
    (fun pow => (do (pure true) : RustM Bool))
    (fun pow => (do (pow <? (1000000 : i32)) : RustM Bool))
    (fun pow =>
      (do
      (rust_primitives.hax.int.from_machine (0 : u32)) : RustM hax_lib.int.Int))
    pow
    (fun pow =>
      (do
      let pow : i32 ← (pow *? (2 : i32));
      if (← (pow <? x)) then do
        let pow : i32 ← (pow *? (3 : i32));
        if true then do
          (pure (core_models.ops.control_flow.ControlFlow.Break
            (rust_primitives.hax.Tuple2.mk rust_primitives.hax.Tuple0.mk pow)))
        else do
          (pure (core_models.ops.control_flow.ControlFlow.Continue
            (← (pow *? (2 : i32)))))
      else do
        (pure (core_models.ops.control_flow.ControlFlow.Continue
          (← (pow *? (2 : i32))))) :
      RustM
      (core_models.ops.control_flow.ControlFlow
        (rust_primitives.hax.Tuple2 rust_primitives.hax.Tuple0 i32)
        i32))))

structure M where
  m : (alloc.vec.Vec u8 alloc.alloc.Global)

@[spec]
def Impl.decoded_message (self : M) :
    RustM
    (core_models.option.Option (alloc.vec.Vec u8 alloc.alloc.Global))
    := do
  match
    (← (rust_primitives.hax.folds.fold_range_return
      (0 : usize)
      (← (alloc.vec.Impl_1.len u8 alloc.alloc.Global (M.m self)))
      (fun _ _ => (do (pure true) : RustM Bool))
      rust_primitives.hax.Tuple0.mk
      (fun _ i =>
        (do
        if (← (i >? (5 : usize))) then do
          (pure (core_models.ops.control_flow.ControlFlow.Break
            (core_models.ops.control_flow.ControlFlow.Break
              core_models.option.Option.None)))
        else do
          (pure (core_models.ops.control_flow.ControlFlow.Continue
            rust_primitives.hax.Tuple0.mk)) :
        RustM
        (core_models.ops.control_flow.ControlFlow
          (core_models.ops.control_flow.ControlFlow
            (core_models.option.Option (alloc.vec.Vec u8 alloc.alloc.Global))
            (rust_primitives.hax.Tuple2
              rust_primitives.hax.Tuple0
              rust_primitives.hax.Tuple0))
          rust_primitives.hax.Tuple0)))))
  with
    | (core_models.ops.control_flow.ControlFlow.Break  ret) => do (pure ret)
    | (core_models.ops.control_flow.ControlFlow.Continue  _) => do
      (pure (core_models.option.Option.Some
        (← (core_models.clone.Clone.clone
          (alloc.vec.Vec u8 alloc.alloc.Global) (M.m self)))))

--  @fail(extraction): ssprove(HAX0001), coq(HAX0001)
@[spec]
def nested (_ : rust_primitives.hax.Tuple0) : RustM i32 := do
  let sum : i32 := (0 : i32);
  let sum : i32 ←
    (rust_primitives.hax.folds.fold_range
      (1 : i32)
      (10 : i32)
      (fun sum _ => (do (pure true) : RustM Bool))
      sum
      (fun sum i =>
        (do
        let sum : i32 ←
          (rust_primitives.hax.folds.fold_range_cf
            (1 : i32)
            (10 : i32)
            (fun sum _ => (do (pure true) : RustM Bool))
            sum
            (fun sum j =>
              (do
              if (← (j <? (0 : i32))) then do
                (pure (core_models.ops.control_flow.ControlFlow.Break
                  (rust_primitives.hax.Tuple2.mk
                    rust_primitives.hax.Tuple0.mk
                    sum)))
              else do
                (pure (core_models.ops.control_flow.ControlFlow.Continue
                  (← (sum +? j)))) :
              RustM
              (core_models.ops.control_flow.ControlFlow
                (rust_primitives.hax.Tuple2 rust_primitives.hax.Tuple0 i32)
                i32))));
        (sum +? i) :
        RustM i32)));
  (sum *? (2 : i32))

@[spec]
def nested_return (_ : rust_primitives.hax.Tuple0) : RustM i32 := do
  let sum : i32 := (0 : i32);
  match
    (← (rust_primitives.hax.folds.fold_range_return
      (1 : i32)
      (10 : i32)
      (fun sum _ => (do (pure true) : RustM Bool))
      sum
      (fun sum i =>
        (do
        match
          (← (rust_primitives.hax.folds.fold_range_return
            (1 : i32)
            (10 : i32)
            (fun sum _ => (do (pure true) : RustM Bool))
            sum
            (fun sum j =>
              (do
              if (← (j <? (0 : i32))) then do
                (pure (core_models.ops.control_flow.ControlFlow.Break
                  (core_models.ops.control_flow.ControlFlow.Break (0 : i32))))
              else do
                (pure (core_models.ops.control_flow.ControlFlow.Continue
                  (← (sum +? j)))) :
              RustM
              (core_models.ops.control_flow.ControlFlow
                (core_models.ops.control_flow.ControlFlow
                  i32
                  (rust_primitives.hax.Tuple2 rust_primitives.hax.Tuple0 i32))
                i32)))))
        with
          | (core_models.ops.control_flow.ControlFlow.Break  ret) => do
            (pure (core_models.ops.control_flow.ControlFlow.Break
              (core_models.ops.control_flow.ControlFlow.Break ret)))
          | (core_models.ops.control_flow.ControlFlow.Continue  sum) => do
            (pure (core_models.ops.control_flow.ControlFlow.Continue
              (← (sum +? i)))) :
        RustM
        (core_models.ops.control_flow.ControlFlow
          (core_models.ops.control_flow.ControlFlow
            i32
            (rust_primitives.hax.Tuple2 rust_primitives.hax.Tuple0 i32))
          i32)))))
  with
    | (core_models.ops.control_flow.ControlFlow.Break  ret) => do (pure ret)
    | (core_models.ops.control_flow.ControlFlow.Continue  sum) => do
      (sum *? (2 : i32))

--  @fail(extraction): ssprove(HAX0008), coq(HAX0008)
@[spec]
def continue_only (x : (RustSlice i32)) :
    RustM (rust_primitives.hax.Tuple2 i32 rust_primitives.hax.Tuple0) := do
  let product : i32 := (1 : i32);
  (pure (rust_primitives.hax.Tuple2.mk
    (← (core_models.iter.traits.iterator.Iterator.fold
      (← (core_models.iter.traits.collect.IntoIterator.into_iter
        (RustSlice i32) x))
      product
      (fun product i =>
        (do
        if (← (i ==? (0 : i32))) then do
          (pure product)
        else do
          (core_models.ops.arith.MulAssign.mul_assign i32 i32 product i) :
        RustM i32))))
    rust_primitives.hax.Tuple0.mk))

--  @fail(extraction): ssprove(HAX0001, HAX0008), coq(HAX0008, HAX0001)
@[spec]
def continue_and_break (x : (RustSlice i32)) :
    RustM (rust_primitives.hax.Tuple2 i32 rust_primitives.hax.Tuple0) := do
  let product : i32 := (1 : i32);
  (pure (rust_primitives.hax.Tuple2.mk
    (← (rust_primitives.hax.folds.fold_cf
      (← (core_models.iter.traits.collect.IntoIterator.into_iter
        (RustSlice i32) x))
      product
      (fun product i =>
        (do
        if (← (i ==? (0 : i32))) then do
          (pure (core_models.ops.control_flow.ControlFlow.Continue product))
        else do
          if (← (i <? (0 : i32))) then do
            (pure (core_models.ops.control_flow.ControlFlow.Break
              (rust_primitives.hax.Tuple2.mk
                rust_primitives.hax.Tuple0.mk
                product)))
          else do
            (pure (core_models.ops.control_flow.ControlFlow.Continue
              (← (core_models.ops.arith.MulAssign.mul_assign
                i32
                i32 product i)))) :
        RustM
        (core_models.ops.control_flow.ControlFlow
          (rust_primitives.hax.Tuple2 rust_primitives.hax.Tuple0 i32)
          i32)))))
    rust_primitives.hax.Tuple0.mk))

end new_tests.legacy__loops__lib.control_flow


namespace new_tests.legacy__loops__lib.and_mut_side_effect_loop

@[spec]
def looping (array : (RustArray u8 5)) : RustM (RustArray u8 5) := do
  let array : (RustArray u8 5) ←
    (rust_primitives.hax.folds.fold_range
      (0 : usize)
      (← (core_models.slice.Impl.len u8 (← (rust_primitives.unsize array))))
      (fun array _ => (do (pure true) : RustM Bool))
      array
      (fun array i =>
        (do
        (rust_primitives.hax.monomorphized_update_at.update_at_usize
          array
          i
          (← (rust_primitives.hax.cast_op i : RustM u8))) :
        RustM (RustArray u8 5))));
  (pure array)

@[spec]
def looping_2 (array : (RustArray u8 5)) : RustM (RustArray u8 5) := do
  let ⟨array, result⟩ :=
    (rust_primitives.hax.Tuple2.mk
      (← (rust_primitives.hax.folds.fold_range
        (0 : usize)
        (← (core_models.slice.Impl.len u8 (← (rust_primitives.unsize array))))
        (fun array _ => (do (pure true) : RustM Bool))
        array
        (fun array i =>
          (do
          (rust_primitives.hax.monomorphized_update_at.update_at_usize
            array
            i
            (← (rust_primitives.hax.cast_op i : RustM u8))) :
          RustM (RustArray u8 5)))))
      rust_primitives.hax.Tuple0.mk);
  let _ := rust_primitives.hax.Tuple0.mk;
  let _ := result;
  (pure array)

end new_tests.legacy__loops__lib.and_mut_side_effect_loop

