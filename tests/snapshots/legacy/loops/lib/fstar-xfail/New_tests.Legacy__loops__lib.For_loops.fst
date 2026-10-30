module New_tests.Legacy__loops__lib.For_loops
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let range1 (_: Prims.unit) : usize =
  let acc:usize = mk_usize 0 in
  let acc:usize =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      (mk_usize 15)
      (fun acc temp_1_ ->
          let acc:usize = acc in
          let _:usize = temp_1_ in
          true)
      acc
      (fun acc i ->
          let acc:usize = acc in
          let i:usize = i in
          acc +! i <: usize)
  in
  acc

let range2 (n: usize) : usize =
  let acc:usize = mk_usize 0 in
  let acc:usize =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      (n +! mk_usize 10 <: usize)
      (fun acc temp_1_ ->
          let acc:usize = acc in
          let _:usize = temp_1_ in
          true)
      acc
      (fun acc i ->
          let acc:usize = acc in
          let i:usize = i in
          (acc +! i <: usize) +! mk_usize 1 <: usize)
  in
  acc

let composed_range (n: usize) : usize =
  let acc:usize = mk_usize 0 in
  let acc:usize =
    Core_models.Iter.Traits.Iterator.f_fold (Core_models.Iter.Traits.Collect.f_into_iter #(Core_models.Iter.Adapters.Chain.t_Chain
              (Core_models.Ops.Range.t_Range usize) (Core_models.Ops.Range.t_Range usize))
          #FStar.Tactics.Typeclasses.solve
          (Core_models.Iter.Traits.Iterator.f_chain #(Core_models.Ops.Range.t_Range usize)
              #FStar.Tactics.Typeclasses.solve
              #(Core_models.Ops.Range.t_Range usize)
              ({ Core_models.Ops.Range.f_start = mk_usize 0; Core_models.Ops.Range.f_end = n }
                <:
                Core_models.Ops.Range.t_Range usize)
              ({
                  Core_models.Ops.Range.f_start = n +! mk_usize 10 <: usize;
                  Core_models.Ops.Range.f_end = n +! mk_usize 50 <: usize
                }
                <:
                Core_models.Ops.Range.t_Range usize)
            <:
            Core_models.Iter.Adapters.Chain.t_Chain (Core_models.Ops.Range.t_Range usize)
              (Core_models.Ops.Range.t_Range usize))
        <:
        Core_models.Iter.Adapters.Chain.t_Chain (Core_models.Ops.Range.t_Range usize)
          (Core_models.Ops.Range.t_Range usize))
      acc
      (fun acc i ->
          let acc:usize = acc in
          let i:usize = i in
          (acc +! i <: usize) +! mk_usize 1 <: usize)
  in
  acc

let rev_range (n: usize) : usize =
  let acc:usize = mk_usize 0 in
  let acc:usize =
    Core_models.Iter.Traits.Iterator.f_fold (Core_models.Iter.Traits.Collect.f_into_iter #(Core_models.Iter.Adapters.Rev.t_Rev
            (Core_models.Ops.Range.t_Range usize))
          #FStar.Tactics.Typeclasses.solve
          (Core_models.Iter.Traits.Iterator.f_rev #(Core_models.Ops.Range.t_Range usize)
              #FStar.Tactics.Typeclasses.solve
              ({ Core_models.Ops.Range.f_start = mk_usize 0; Core_models.Ops.Range.f_end = n }
                <:
                Core_models.Ops.Range.t_Range usize)
            <:
            Core_models.Iter.Adapters.Rev.t_Rev (Core_models.Ops.Range.t_Range usize))
        <:
        Core_models.Iter.Adapters.Rev.t_Rev (Core_models.Ops.Range.t_Range usize))
      acc
      (fun acc i ->
          let acc:usize = acc in
          let i:usize = i in
          (acc +! i <: usize) +! mk_usize 1 <: usize)
  in
  acc

let chunks (v_CHUNK_LEN: usize) (arr: Alloc.Vec.t_Vec usize Alloc.Alloc.t_Global) : usize =
  let acc:usize = mk_usize 0 in
  let chunks:Core_models.Slice.Iter.t_ChunksExact usize =
    Core_models.Slice.impl__chunks_exact #usize
      (Alloc.Vec.impl_1__as_slice arr <: t_Slice usize)
      v_CHUNK_LEN
  in
  let acc:usize =
    Core_models.Iter.Traits.Iterator.f_fold (Core_models.Iter.Traits.Collect.f_into_iter #(Core_models.Slice.Iter.t_ChunksExact
            usize)
          #FStar.Tactics.Typeclasses.solve
          (Core_models.Clone.f_clone #(Core_models.Slice.Iter.t_ChunksExact usize)
              #FStar.Tactics.Typeclasses.solve
              chunks
            <:
            Core_models.Slice.Iter.t_ChunksExact usize)
        <:
        Core_models.Slice.Iter.t_ChunksExact usize)
      acc
      (fun acc chunk ->
          let acc:usize = acc in
          let chunk:t_Slice usize = chunk in
          let mean:usize = mk_usize 0 in
          let mean:usize =
            Core_models.Iter.Traits.Iterator.f_fold (Core_models.Iter.Traits.Collect.f_into_iter #(t_Slice
                    usize)
                  #FStar.Tactics.Typeclasses.solve
                  chunk
                <:
                Core_models.Slice.Iter.t_Iter usize)
              mean
              (fun mean item ->
                  let mean:usize = mean in
                  let item:usize = item in
                  mean +! item <: usize)
          in
          let acc:usize = acc +! (mean /! v_CHUNK_LEN <: usize) in
          acc)
  in
  let acc:usize =
    Core_models.Iter.Traits.Iterator.f_fold (Core_models.Iter.Traits.Collect.f_into_iter #(t_Slice
            usize)
          #FStar.Tactics.Typeclasses.solve
          (Core_models.Slice.Iter.impl_88__remainder #usize chunks <: t_Slice usize)
        <:
        Core_models.Slice.Iter.t_Iter usize)
      acc
      (fun acc item ->
          let acc:usize = acc in
          let item:usize = item in
          acc -! item <: usize)
  in
  acc

let iterator (arr: Alloc.Vec.t_Vec usize Alloc.Alloc.t_Global) : usize =
  let acc:usize = mk_usize 0 in
  let acc:usize =
    Core_models.Iter.Traits.Iterator.f_fold (Core_models.Iter.Traits.Collect.f_into_iter #(Core_models.Slice.Iter.t_Iter
            usize)
          #FStar.Tactics.Typeclasses.solve
          (Core_models.Slice.impl__iter #usize (Alloc.Vec.impl_1__as_slice arr <: t_Slice usize)
            <:
            Core_models.Slice.Iter.t_Iter usize)
        <:
        Core_models.Slice.Iter.t_Iter usize)
      acc
      (fun acc item ->
          let acc:usize = acc in
          let item:usize = item in
          acc +! item <: usize)
  in
  acc

/// @fail(extraction): ssprove(HAX0001)
let nested (arr: Alloc.Vec.t_Vec usize Alloc.Alloc.t_Global) : usize =
  let acc:usize = mk_usize 0 in
  let acc:usize =
    Core_models.Iter.Traits.Iterator.f_fold (Core_models.Iter.Traits.Collect.f_into_iter #(Core_models.Slice.Iter.t_Iter
            usize)
          #FStar.Tactics.Typeclasses.solve
          (Core_models.Slice.impl__iter #usize (Alloc.Vec.impl_1__as_slice arr <: t_Slice usize)
            <:
            Core_models.Slice.Iter.t_Iter usize)
        <:
        Core_models.Slice.Iter.t_Iter usize)
      acc
      (fun acc item ->
          let acc:usize = acc in
          let item:usize = item in
          Core_models.Iter.Traits.Iterator.f_fold (Core_models.Iter.Traits.Collect.f_into_iter #(Core_models.Iter.Adapters.Rev.t_Rev
                  (Core_models.Ops.Range.t_Range usize))
                #FStar.Tactics.Typeclasses.solve
                (Core_models.Iter.Traits.Iterator.f_rev #(Core_models.Ops.Range.t_Range usize)
                    #FStar.Tactics.Typeclasses.solve
                    ({
                        Core_models.Ops.Range.f_start = mk_usize 0;
                        Core_models.Ops.Range.f_end = item
                      }
                      <:
                      Core_models.Ops.Range.t_Range usize)
                  <:
                  Core_models.Iter.Adapters.Rev.t_Rev (Core_models.Ops.Range.t_Range usize))
              <:
              Core_models.Iter.Adapters.Rev.t_Rev (Core_models.Ops.Range.t_Range usize))
            acc
            (fun acc i ->
                let acc:usize = acc in
                let i:usize = i in
                let acc:usize = acc +! mk_usize 1 in
                Core_models.Iter.Traits.Iterator.f_fold (Core_models.Iter.Traits.Collect.f_into_iter
                      #(Core_models.Iter.Adapters.Zip.t_Zip (Core_models.Slice.Iter.t_Iter usize)
                          (Core_models.Ops.Range.t_Range usize))
                      #FStar.Tactics.Typeclasses.solve
                      (Core_models.Iter.Traits.Iterator.f_zip #(Core_models.Slice.Iter.t_Iter usize)
                          #FStar.Tactics.Typeclasses.solve
                          #(Core_models.Ops.Range.t_Range usize)
                          (Core_models.Slice.impl__iter #usize
                              (Alloc.Vec.impl_1__as_slice arr <: t_Slice usize)
                            <:
                            Core_models.Slice.Iter.t_Iter usize)
                          ({
                              Core_models.Ops.Range.f_start = mk_usize 4;
                              Core_models.Ops.Range.f_end = i
                            }
                            <:
                            Core_models.Ops.Range.t_Range usize)
                        <:
                        Core_models.Iter.Adapters.Zip.t_Zip (Core_models.Slice.Iter.t_Iter usize)
                          (Core_models.Ops.Range.t_Range usize))
                    <:
                    Core_models.Iter.Adapters.Zip.t_Zip (Core_models.Slice.Iter.t_Iter usize)
                      (Core_models.Ops.Range.t_Range usize))
                  acc
                  (fun acc j ->
                      let acc:usize = acc in
                      let j:(usize & usize) = j in
                      (((acc +! item <: usize) +! i <: usize) +! j._1 <: usize) +! j._2 <: usize))
          <:
          usize)
  in
  acc

let pattern (arr: Alloc.Vec.t_Vec (usize & usize) Alloc.Alloc.t_Global) : usize =
  let acc:usize = mk_usize 0 in
  let acc:usize =
    Core_models.Iter.Traits.Iterator.f_fold (Core_models.Iter.Traits.Collect.f_into_iter #(Alloc.Vec.t_Vec
              (usize & usize) Alloc.Alloc.t_Global)
          #FStar.Tactics.Typeclasses.solve
          arr
        <:
        Alloc.Vec.Into_iter.t_IntoIter (usize & usize) Alloc.Alloc.t_Global)
      acc
      (fun acc temp_1_ ->
          let acc:usize = acc in
          let (x: usize), (y: usize) = temp_1_ in
          acc +! (x *! y <: usize) <: usize)
  in
  acc

let enumerate_chunks (arr: Alloc.Vec.t_Vec usize Alloc.Alloc.t_Global) : usize =
  let acc:usize = mk_usize 0 in
  let acc:usize =
    Core_models.Iter.Traits.Iterator.f_fold (Core_models.Iter.Traits.Collect.f_into_iter #(Core_models.Iter.Adapters.Enumerate.t_Enumerate
            (Core_models.Slice.Iter.t_Chunks usize))
          #FStar.Tactics.Typeclasses.solve
          (Core_models.Iter.Traits.Iterator.f_enumerate #(Core_models.Slice.Iter.t_Chunks usize)
              #FStar.Tactics.Typeclasses.solve
              (Core_models.Slice.impl__chunks #usize
                  (Alloc.Vec.impl_1__as_slice arr <: t_Slice usize)
                  (mk_usize 4)
                <:
                Core_models.Slice.Iter.t_Chunks usize)
            <:
            Core_models.Iter.Adapters.Enumerate.t_Enumerate (Core_models.Slice.Iter.t_Chunks usize))
        <:
        Core_models.Iter.Adapters.Enumerate.t_Enumerate (Core_models.Slice.Iter.t_Chunks usize))
      acc
      (fun acc temp_1_ ->
          let acc:usize = acc in
          let (i: usize), (chunk: t_Slice usize) = temp_1_ in
          Rust_primitives.Hax.Folds.fold_enumerated_slice chunk
            (fun acc temp_1_ ->
                let acc:usize = acc in
                let _:usize = temp_1_ in
                true)
            acc
            (fun acc temp_1_ ->
                let acc:usize = acc in
                let (j: usize), (x: usize) = temp_1_ in
                (i +! j <: usize) +! x <: usize)
          <:
          usize)
  in
  acc

let bool_returning (x: u8) : bool = x <. mk_u8 10

let f (_: Prims.unit) : (u8 & Prims.unit) =
  let acc:u8 = mk_u8 0 in
  Rust_primitives.Hax.Folds.fold_range (mk_u8 1)
    (mk_u8 10)
    (fun acc temp_1_ ->
        let acc:u8 = acc in
        let _:u8 = temp_1_ in
        true)
    acc
    (fun acc i ->
        let acc:u8 = acc in
        let i:u8 = i in
        let acc:u8 = acc +! i in
        let _:bool = bool_returning i in
        acc),
  ()
  <:
  (u8 & Prims.unit)
