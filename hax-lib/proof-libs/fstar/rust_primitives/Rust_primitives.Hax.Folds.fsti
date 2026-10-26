module Rust_primitives.Hax.Folds

open Rust_primitives
open FStar.Mul

(**** `s.chunks_exact(chunk_size).enumerate()` *)
/// Predicate that asserts a slice `s_chunk` is exactly the nth chunk
/// of the sequence `s`
let nth_chunk_of #t
  (s: Seq.seq t)
  (s_chunk: Seq.seq t {Seq.length s_chunk > 0})
  (chunk_nth: nat {chunk_nth < Seq.length s / Seq.length s_chunk})
  =  Seq.slice s (Seq.length s_chunk * chunk_nth) (Seq.length s_chunk * (chunk_nth + 1))
  == s_chunk

/// Fold function that is generated for `for` loops iterating on
/// `s.chunks_exact(chunk_size).enumerate()`-like iterators
val fold_enumerated_chunked_slice
  (#t: Type0) (#acc_t: Type0)
  (chunk_size: usize {v chunk_size > 0})
  (s: t_Slice t)
  (inv: acc_t -> (i:usize{v i <= Seq.length s / v chunk_size}) -> Type0)
  (init: acc_t {inv init (sz 0)})
  (f: ( acc:acc_t
      -> item:(usize & t_Slice t) {
        let (i, s_chunk) = item in
          v i < Seq.length s / v chunk_size
        /\ length s_chunk == chunk_size
        /\ nth_chunk_of s s_chunk (v i)
        /\ inv acc i
      }
      -> acc':acc_t {
        inv acc' (fst item +! sz 1)
      }
      )
  )
  : result: acc_t {inv result (mk_int (Seq.length s / v chunk_size))}

/// Fold function that is generated for `for` loops iterating on
/// `s.chunks_exact(chunk_size)`-like iterators
val fold_chunked_slice
  (#t: Type0) (#acc_t: Type0)
  (chunk_size: usize {v chunk_size > 0})
  (s: t_Slice t)
  (inv: acc_t -> (i:usize) -> Type0)
  (init: acc_t {inv init (sz 0)})
  (f: ( acc:acc_t
      -> item:(t_Slice t) {
        length item == chunk_size /\
        inv acc (sz 0)
      }
      -> acc':acc_t {
        inv acc' (sz 0)
      }
      )
  )
  : result: acc_t {inv result (mk_int 0)}

(**** `start..end_` *)
unfold let fold_range_wf_index (#u: inttype)
  (start: int_t u) (end_: int_t u)
  (strict: bool) (i: int)
  = i >= v start 
     /\ (if strict then i < v end_ else i <= v end_)

unfold let range_empty (#u: inttype)
  (start: int_t u) (end_: int_t u) = v start > v end_

let rec fold_range
  (#acc_t: Type0) (#u: inttype)
  (start: int_t u)
  (end_: int_t u)
  (inv: acc_t -> (i:int_t u{fold_range_wf_index start end_ false (v i)}) -> Type0)
  (init: acc_t {~(range_empty start end_) ==> inv init start})
  (f: (acc:acc_t -> i:int_t u  {v i <= v end_ /\ fold_range_wf_index start end_ true (v i) /\ inv acc i}
                 -> acc':acc_t {(inv acc' (mk_int (v i + 1)))}))
  : Tot (result: acc_t {if range_empty start end_ then result == init else inv result end_}) 
        (decreases v end_ - v start)
  = if v start < v end_
    then fold_range (start +! mk_int 1) end_ inv (f init start) f
    else init

let rec fold_range_cf
  (#acc_t: Type0) (#u: inttype)
  (start: int_t u)
  (end_: int_t u)
  (inv: acc_t -> (i:int_t u{fold_range_wf_index start end_ false (v i)}) -> Type0)
  (acc: acc_t {~(range_empty start end_) ==> inv acc start})
  (f: (acc:acc_t -> i:int_t u {v i <= v end_ /\ fold_range_wf_index start end_ true (v i) /\ inv acc i}
                  -> tuple:((Core_models.Ops.Control_flow.t_ControlFlow (unit & acc_t) acc_t))
                    {
                      let acc = match tuple with 
                        | Core_models.Ops.Control_flow.ControlFlow_Break ((), acc)
                        | Core_models.Ops.Control_flow.ControlFlow_Continue acc -> acc in
                      inv acc (mk_int (v i + 1))}))
: Tot (res: acc_t{if range_empty start end_ then res == acc else (exists (final: int_t u). v start <= v final /\ v final <= v end_ /\ inv res final)}) 
 (decreases v end_ - v start)
  =
  if v start < v end_
  then match f acc start with
       | Core_models.Ops.Control_flow.ControlFlow_Break ((), acc) -> acc
       | Core_models.Ops.Control_flow.ControlFlow_Continue acc ->
         fold_range_cf (start +! mk_int 1) end_ inv acc f
  else acc

let rec fold_range_return
  (#acc_t: Type0) (#ret_t: Type0) (#u: inttype)
  (start: int_t u)
  (end_: int_t u)
  (inv: acc_t -> (i:int_t u{fold_range_wf_index start end_ false (v i)}) -> Type0)
  (acc: acc_t )
  (f: (acc:acc_t -> i:int_t u {v i <= v end_ /\ fold_range_wf_index start end_ true (v i) }
                  -> tuple:((Core_models.Ops.Control_flow.t_ControlFlow (Core_models.Ops.Control_flow.t_ControlFlow ret_t (unit & acc_t))) acc_t)
                    ))
: Tot (Core_models.Ops.Control_flow.t_ControlFlow ret_t acc_t) (decreases v end_ - v start)
  =
  if v start < v end_
  then match f acc start with
       | Core_models.Ops.Control_flow.ControlFlow_Break (Core_models.Ops.Control_flow.ControlFlow_Break res)-> Core_models.Ops.Control_flow.ControlFlow_Break res
       
       | Core_models.Ops.Control_flow.ControlFlow_Break (Core_models.Ops.Control_flow.ControlFlow_Continue ((), res)) -> Core_models.Ops.Control_flow.ControlFlow_Continue res
       | Core_models.Ops.Control_flow.ControlFlow_Continue acc ->
         fold_range_return (start +! mk_int 1) end_ inv acc f
  else Core_models.Ops.Control_flow.ControlFlow_Continue acc

(**** `start..=end_` *)
/// `inv acc i` holds before the iteration on `i`, up to `end_ + 1` after the
/// loop. When `end_` is the largest value of its type, `end_ + 1` does not
/// exist, and the invariant gives nothing about the state after the loop.
unfold let fold_range_inclusive_wf_index (#u: inttype)
  (start: int_t u) (end_: int_t u) (i: int)
  = i >= v start /\ i <= v end_ + 1

let rec fold_range_inclusive
  (#acc_t: Type0) (#u: inttype)
  (start: int_t u)
  (end_: int_t u)
  (inv: acc_t -> (i:int_t u{fold_range_inclusive_wf_index start end_ (v i)}) -> Type0)
  (init: acc_t {~(range_empty start end_) ==> inv init start})
  (f: (acc:acc_t -> i:int_t u {v start <= v i /\ v i <= v end_ /\ inv acc i}
                 -> acc':acc_t {v i < maxint u ==> inv acc' (mk_int (v i + 1))}))
  : Tot (result: acc_t {if range_empty start end_ then result == init
                        else (v end_ < maxint u ==> inv result (mk_int (v end_ + 1)))})
        (decreases v end_ - v start)
  = if v start < v end_
    then fold_range_inclusive (start +! mk_int 1) end_ inv (f init start) f
    else if v start = v end_ then f init start
    else init

let rec fold_range_inclusive_cf
  (#acc_t: Type0) (#u: inttype)
  (start: int_t u)
  (end_: int_t u)
  (inv: acc_t -> (i:int_t u{fold_range_inclusive_wf_index start end_ (v i)}) -> Type0)
  (acc: acc_t {~(range_empty start end_) ==> inv acc start})
  (f: (acc:acc_t -> i:int_t u {v start <= v i /\ v i <= v end_ /\ inv acc i}
                  -> tuple:((Core_models.Ops.Control_flow.t_ControlFlow (unit & acc_t) acc_t))
                    {
                      let acc = match tuple with
                        | Core_models.Ops.Control_flow.ControlFlow_Break ((), acc)
                        | Core_models.Ops.Control_flow.ControlFlow_Continue acc -> acc in
                      v i < maxint u ==> inv acc (mk_int (v i + 1))}))
: Tot (res: acc_t{if range_empty start end_ then res == acc
                 else (v end_ < maxint u ==>
                       (exists (final: int_t u). v start <= v final /\ v final <= v end_ + 1 /\ inv res final))})
 (decreases v end_ - v start)
  =
  if v start <= v end_
  then match f acc start with
       | Core_models.Ops.Control_flow.ControlFlow_Break ((), acc) -> acc
       | Core_models.Ops.Control_flow.ControlFlow_Continue acc ->
         if v start < v end_
         then fold_range_inclusive_cf (start +! mk_int 1) end_ inv acc f
         else acc
  else acc

let rec fold_range_inclusive_return
  (#acc_t: Type0) (#ret_t: Type0) (#u: inttype)
  (start: int_t u)
  (end_: int_t u)
  (inv: acc_t -> (i:int_t u{fold_range_inclusive_wf_index start end_ (v i)}) -> Type0)
  (acc: acc_t )
  (f: (acc:acc_t -> i:int_t u {v start <= v i /\ v i <= v end_}
                  -> tuple:((Core_models.Ops.Control_flow.t_ControlFlow (Core_models.Ops.Control_flow.t_ControlFlow ret_t (unit & acc_t))) acc_t)
                    ))
: Tot (Core_models.Ops.Control_flow.t_ControlFlow ret_t acc_t) (decreases v end_ - v start)
  =
  if v start <= v end_
  then match f acc start with
       | Core_models.Ops.Control_flow.ControlFlow_Break (Core_models.Ops.Control_flow.ControlFlow_Break res) -> Core_models.Ops.Control_flow.ControlFlow_Break res
       | Core_models.Ops.Control_flow.ControlFlow_Break (Core_models.Ops.Control_flow.ControlFlow_Continue ((), res)) -> Core_models.Ops.Control_flow.ControlFlow_Continue res
       | Core_models.Ops.Control_flow.ControlFlow_Continue acc ->
         if v start < v end_
         then fold_range_inclusive_return (start +! mk_int 1) end_ inv acc f
         else Core_models.Ops.Control_flow.ControlFlow_Continue acc
  else Core_models.Ops.Control_flow.ControlFlow_Continue acc

(**** `s.enumerate()` *)
/// Fold function that is generated for `for` loops iterating on
/// `s.enumerate()`-like iterators
let fold_enumerated_slice
  (#t: Type0) (#acc_t: Type0)
  (s: t_Slice t)
  (inv: acc_t -> (i:usize{v i <= v (length s)}) -> Type0)
  (init: acc_t {inv init (sz 0)})
  (f: (acc:acc_t -> i:(usize & t) {v (fst i) < v (length s) /\ snd i == Seq.index s (v (fst i)) /\ inv acc  (fst i)}
                 -> acc':acc_t    {v (fst i) < v (length s) /\ inv acc' (fst i +! sz 1)}))
  : result: acc_t {inv result (length s)}
  = fold_range (sz 0) (length s) inv init (fun acc i -> f acc (i, Seq.index s (v i)))

val fold_enumerated_slice_return
  (#t: Type0) (#acc_t: Type0) (#ret: Type0)
  (s: t_Slice t)
  (inv: acc_t -> (i:usize{v i <= v (length s)}) -> Type0)
  (init: acc_t {inv init (sz 0)})
  (f: (acc:acc_t -> i:(usize & t) {v (fst i) < v (length s) /\ snd i == Seq.index s (v (fst i)) (*/\ inv acc  (fst i)*)}
                 -> Core_models.Ops.Control_flow.t_ControlFlow (Core_models.Ops.Control_flow.t_ControlFlow ret (unit & acc_t)) (acc':acc_t)    (*{v (fst i) < v (length s) /\ inv acc' (fst i)}*)))
  : result: Core_models.Ops.Control_flow.t_ControlFlow ret acc_t(* {inv result (length s)} *)

(**** `(start..end_).step_by(step)` *)
unfold let fold_range_step_by_wf_index (#u: inttype)
  (start: int_t u) (end_: int_t u)
  (step: usize {v step > 0}) (strict: bool) (i: int)
  = v start < v end_ ==> (let end_step = v end_ - 1 - ((v end_ - 1 - v start) % v step) in
                          i >= v start 
                        /\ (if strict then i <= end_step else i <= end_step + v step))
  // /\ i % v step == v start % v step

#push-options "--z3rlimit 80"
unfold let fold_range_step_by_upper_bound (#u: inttype)
  (start: int_t u) (end_: int_t u)
  (step: usize {v step > 0})
  : end':int {fold_range_step_by_wf_index start end_ step false end'}
  = if v end_ <= v start
    then v start
    else v end_ - 1 - ((v end_ - 1 - v start) % v step) + v step
#pop-options

let fold_range_step_by_bound_lemma (k: pos) (a: nat) (b: nat)
  : Lemma (requires a % k == 0 /\ a <= b)
          (ensures a <= b - b % k /\ (a + k > b ==> a == b - b % k))
  = FStar.Math.Lemmas.lemma_div_le a b k;
    FStar.Math.Lemmas.euclidean_division_definition a k;
    FStar.Math.Lemmas.euclidean_division_definition b k;
    FStar.Math.Lemmas.lemma_mult_le_right k (a / k) (b / k);
    if a / k + 1 <= b / k
    then FStar.Math.Lemmas.lemma_mult_le_right k (a / k + 1) (b / k)

#push-options "--z3rlimit 100"
let rec fold_range_step_by_from
  (#acc_t: Type0) (#u: inttype)
  (start: int_t u)
  (end_: int_t u)
  (step: usize {v step > 0 /\ range (v end_ + v step) u})
  (inv: acc_t -> (i:int_t u{fold_range_step_by_wf_index start end_ step false (v i)}) -> Type0)
  (i: int_t u {v i >= v start /\ (v i - v start) % v step == 0
            /\ (v start < v end_ ==> v i <= fold_range_step_by_upper_bound start end_ step)})
  (init: acc_t {inv init i})
  (f: (acc:acc_t -> j:int_t u  {v j < v end_ - ((v end_ - 1 - v start) % v step) /\ fold_range_step_by_wf_index start end_ step true (v j) /\ inv acc j}
                 -> acc':acc_t {(inv acc' (mk_int (v j + v step)))}))
  : Tot (result: acc_t {if v i < v end_
                        then inv result (mk_int (fold_range_step_by_upper_bound start end_ step))
                        else inv result i})
        (decreases (v end_ - v i + v step))
  = if v i < v end_
    then begin
      fold_range_step_by_bound_lemma (v step) (v i - v start) (v end_ - 1 - v start);
      FStar.Math.Lemmas.lemma_mod_plus (v i - v start) 1 (v step);
      fold_range_step_by_from start end_ step inv (mk_int (v i + v step)) (f init i) f
    end
    else init

/// Fold function that is generated for `for` loops iterating on
/// `(start..end_).step_by(step)`-like iterators
let fold_range_step_by
  (#acc_t: Type0) (#u: inttype)
  (start: int_t u)
  (end_: int_t u)
  (step: usize {v step > 0 /\ range (v end_ + v step) u})
  (inv: acc_t -> (i:int_t u{fold_range_step_by_wf_index start end_ step false (v i)}) -> Type0)
  (init: acc_t {inv init start})
  (f: (acc:acc_t -> i:int_t u  {v i < v end_ - ((v end_ - 1 - v start) % v step) /\ fold_range_step_by_wf_index start end_ step true (v i) /\ inv acc i}
                 -> acc':acc_t {(inv acc' (mk_int (v i + v step)))}))
  : result: acc_t {inv result (mk_int (fold_range_step_by_upper_bound start end_ step))}
  = (if v start < v end_
     then fold_range_step_by_bound_lemma (v step) 0 (v end_ - 1 - v start));
    fold_range_step_by_from start end_ step inv start init f
#pop-options

val fold_return #it #acc #ret #item (i: it) (init: acc) 
  (f: acc -> item -> 
    Core_models.Ops.Control_flow.t_ControlFlow  
    (Core_models.Ops.Control_flow.t_ControlFlow ret (unit & acc)) acc): 
  Core_models.Ops.Control_flow.t_ControlFlow ret acc
  