---
weight: 1
---

# Proving properties

In the previous chapter, we proved one property of the `square` function:
panic freedom.

This contract stipulates that, given a small input, the function will
_return a value_: it will not panic or diverge. We could enrich the
contract of `square` with a post-condition about the fact it is an
increasing function:
```{.rust}
#[hax_lib::requires(x < 16)]
#[hax_lib::ensures(|res| res >= x)]
pub fn square(x: u8) -> u8 {
    x * x
}
```

Reextract:
```
cargo hax into lean
```

We will have to adjust the proof a little to make it work:
```
@[spec]
theorem square.spec.proof (x : Std.U8) : square.spec x := by
  unfold square.spec square
  hax_mvcgen
  all_goals grind [Nat.le_mul_self]
```
What we did here is to add the lemma `Nat.le_mul_self`, which states that `n <= n * n` for 
all natural numbers `n`.

There is no magic Lean proof that will prove all functions correct. The proofs need to be
tailored to the specific function and its specification.
In practice, AI can help you to generate such proofs. But be careful: the AI may produce Lean
proofs that seem to prove something that they don't actually prove. We plan to integrate
https://github.com/leanprover/comparator soon to prevent such pitfalls.



Finally, rerun `lake build` to verify the proof.

## See more examples
The property that we prove above demonstrates a very simple case of a proof using hax and Lean. For a more complex example, have a look at the
[`examples`](https://github.com/pereq-dev/hax/tree/main/examples) 
section of hax. 


