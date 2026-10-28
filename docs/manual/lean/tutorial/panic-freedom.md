---
weight: 0
---

# Panic freedom

Let's start with a simple example: a function that squares a `u8`
integer. Set up a fresh Rust project to follow along:
```
cargo new hax-tutorial --lib
cd hax-tutorial
```

Replace the content of `src/lib.rs` with our new squaring function:
```{.rust}
pub fn square(x: u8) -> u8 {
    x * x
}
```

## Extracting Lean code

Run the following command to extract Lean code from this Rust project:
```
cargo hax into lean
```
Besides extracting the code, this sets up the Lean scaffolding around it, so
the result is a working Lean project. It can be found in the `proofs/lean`
directory.

The file `proofs/lean/HaxTutorial/Extraction/Funs.lean` contains (among some boilerplate code) the translation of our `square` function:
```{.lean}
def square (x : Std.U8) : RustM Std.U8 := do
  x * x
```
To type-check the extraction, run
```
cd proofs/lean
lake exe cache get
lake build
```
The `lake exe cache get` step downloads prebuilt files for
[mathlib](https://github.com/leanprover-community/mathlib4), a dependency
of the generated project. Without it, `lake build` compiles mathlib from
source, which takes much longer.
The first time you run this, it might take a while to download dependencies.
Ultimately, the type checking succeeds. If you have used the F* backend before,
this might come as a surprise because the function can panic when `x` is so large that the
multiplication causes an integer overflow.
In Lean, our translation is wrapped in a monad called `RustM` that allows us
to model functions that panic.

## Specifying panic-freedom

To try to prove panic-freedom, we have to 
specify that `square` is expected not to panic.
To add a specification, we need to add `hax_lib` as a dependency of our Rust package:
```
cargo add --git https://github.com/pereq-dev/hax hax-lib
```
Now we can add a simple specification as follows:
```{.rust}
#[hax_lib::ensures(|res| true)]
pub fn square(x: u8) -> u8 {
    x * x
}
```
Adding a `hax_lib::ensures` annotation will make hax generate a Lean specification of the function, asserting panic freedom as well as the postcondition. Here, we used the trivial postcondition `true`, so we only assert panic freedom.

To generate the Lean specification, we need to reextract:
```
cargo hax into lean
```
Now, hax produces the additional file `proofs/lean/HaxTutorial/Extraction/Specs.lean` and
rewrites `proofs/lean/HaxTutorial/Extraction.lean` to import it, so it is part of the build
through the root module's `import HaxTutorial.Extraction`. The new file contains the
specification for our `square` function:
```{.lean}
/-- [hax_tutorial::square::post]:
    Source: 'src/lib.rs', lines 1:0-1:31 -/
@[reducible]
def square.post (x : Std.U8) (res : Std.U8) : RustM Bool := do
  ok true

def square.spec (x : Std.U8) : Prop :=
  ⦃ ⌜ True ⌝ ⦄
  square x
  ⦃ ⇓ res => ⌜ (square.post x res).holds ⌝ ⦄
```
This file defines 
- the postcondition `square.post` that we have provided via the `hax_lib::ensures`
clause and
- a definition `square.spec` of our overall specification.

## Proving the specification correct

Now, we'd like to
prove that specification correct. To this end, hax already generated a template
`proofs/lean/HaxTutorial/Extraction/ProofObligations.lean` for us, which states
the proof obligation. The template is never imported by the root module: files
under `Extraction/` are regenerated on every extraction, so the proof work goes
into `proofs/lean/HaxTutorial/Verification/ProofObligations.lean`, which the
scaffolding created for exactly this purpose and which the root module already
imports. Replace its contents with a copy of the template.
Let's type-check all of this:
```
lake build
```
Again, this succeeds. But this time, we get a warning:
```
warning: HaxTutorial/Verification/ProofObligations.lean:25:8: declaration uses `sorry`
```
Everything type-checks, but we haven't proved anything yet. The template just contains the
keyword `sorry` where the proof is supposed to be. This tells Lean: "Sorry, I don't have a proof
for this yet".

We need to replace the `sorry` with an actual proof. A typical proof pattern is the following:
```{.lean}
@[spec]
theorem square.spec.proof (x : Std.U8) : square.spec x := by
  unfold square.spec square
  hax_mvcgen
  all_goals grind
```
This unfolds the definitions of the specification `square.spec` and of the function `square`,
runs our tactic `hax_mvcgen` to generate verification conditions, and finally calls `grind` to
discharge the verification conditions.

Replace the `sorry` in `HaxTutorial/Verification/ProofObligations.lean` with the proof above and rerun `lake build`.
We get the following error:
```
error: HaxTutorial/Verification/ProofObligations.lean:28:12: `grind` failed
case grind
x : U8
h : U8.max < ↑x * ↑x
h_1 : ¬(ExceptConds.false.1 { down := Error.panic }).down
⊢ False
```
The error is hard to read, but what Lean is trying to tell us is that it failed to prove
that `x * x` will never exceed the maximum `u8` value. 
Indeed, as stated, our specification is not correct.
The reason is that the multiplication can overflow when `x` is too large, and this will
cause a panic in Rust's debug mode.

Multiplication is not the only panicking function provided by the Rust
library: most of the other integer arithmetic operation have such
informal assumptions.
Another source of panics is indexing. Indexing in an array, a slice or
a vector is a partial operation: the index might be out of range.

## Fixing the specification
There is an informal assumption to the
multiplication operator in Rust: the inputs should be small enough so
that the multiplication doesn't overflow.

Note that Rust also provides `wrapping_mul`, a non-panicking variant
of the multiplication on `u8` that wraps when the result is bigger
than `255`. Replacing the common multiplication with `wrapping_mul` in
`square` would fix the panic, but then, `square(16)` returns zero.
Semantically, this is not what one would expect from `square`.

Our problem is that our function `square` is well-defined only when
its input is within `0` and `15`.
The solution is to add a precondition to restrict the inputs and
stay in the domain where the multiplication fits in a `u8`.
We can do so using the `hax_lib::requires` annotation: 
```{.rust}
#[hax_lib::requires(x < 16)]
#[hax_lib::ensures(|res| true)]
pub fn square(x: u8) -> u8 {
    x * x
}
```

To generate the Lean specification, we need to reextract:
```
cargo hax into lean
```
Now, the Lean specification also contains this precondition:
```{.lean}
/-- [hax_tutorial::square::pre]:
    Source: 'src/lib.rs', lines 1:0-1:28 -/
@[reducible] def square.pre (x : Std.U8) : RustM Bool := do
               ok (x < 16#u8)

/-- [hax_tutorial::square::post]:
    Source: 'src/lib.rs', lines 2:0-2:31 -/
@[reducible]
def square.post (x : Std.U8) (res : Std.U8) : RustM Bool := do
  ok true

def square.spec (x : Std.U8) : Prop :=
  (square.pre x).holds →
  ⦃ ⌜ True ⌝ ⦄
  square x
  ⦃ ⇓ res => ⌜ (square.post x res).holds ⌝ ⦄
```
Finally, run `lake build` again. This time Lean accepts the proof!

However, we now get a couple of warnings of the form
```
warning: HaxTutorial/Verification/ProofObligations.lean:27:2: The `mvcgen` tactic is experimental and still under development. Avoid using it in production projects.
```
They are harmless. You can deactivate them by adding the lines
```
[leanOptions]
mvcgen.warning = false
```
to the file `proofs/lean/lakefile.toml`.

