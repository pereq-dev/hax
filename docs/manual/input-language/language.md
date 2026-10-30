---
weight: 1
---

# The Rust subset

The input language is a subset of **safe Rust**, shaped by one design decision: Rust's ownership discipline lets us model programs functionally, with no heap and no pointer reasoning. A construct is signature-expressible when it has a place in that functional model; whether its *implementation* also translates is a separate question ([signature and implementation support](index.md#signature-and-implementation)). The target is stable safe Rust: a not-yet-stable feature is mentioned only where naming it helps place a boundary (`gen` blocks, [loops and recursion](#loops)), while experimental features whose semantics are still in flux (trait aliases, `try` blocks, explicit tail calls with `become`, specialization) are outside the scope of this section.

The unit of translation is a crate's items, its functions, types, traits, and their impls, rather than a whole-program execution, so the program entry point (`main`) plays no special role and you verify library code in place. The other whole-program and runtime items go the same way: `#[global_allocator]`, `#[panic_handler]`, `#[start]`, and the `no_std` / `no_main` configuration select runtime scaffolding rather than a value computation, so a per-item model has nowhere to attach them. They are transparent: the runtime role is ignored, and an item they mark is judged as an ordinary item. The test and benchmark harness (`#[test]`, `#[bench]`, `#[cfg(test)]`) is compiled out of the build hax extracts, so the items it marks are not extracted; to verify their logic, move it into ordinary items.

Several whole-language mechanics are transparent to the value model and so are not tabulated as features. Each is resolved or desugared, before or during translation, into a construct already covered, leaving no independent trace; they are collected here.

| Mechanic | How it is handled |
| --- | --- |
| Macros (`macro_rules!`, procedural) and `#[derive(...)]` | Expanded before the program reaches Charon, so the subset applies to their *output*, not the macros themselves. |
| Configuration and codegen attributes (`cfg`, `#[repr]`, `#[inline]`) | Resolved or discarded before the value model sees them. |
| Diagnostic and linkage attributes (`#[allow]` / `#[warn]` / `#[deny]` / `#[forbid]`, `#[cfg_attr]`, `#[must_use]`, `#[deprecated]`, `#[no_mangle]` / `#[export_name]` / `#[cold]` / `#[target_feature]`) | Describe no value computation; resolved or discarded on the same ground as the codegen attributes. |
| `#[non_exhaustive]` | Only obliges a downstream `match` to carry a wildcard arm and forbids out-of-crate literal construction; a constraint the compiler checks and discards before the model. |
| `#[track_caller]` | Semantic rather than a lint, but erases: the caller `Location` it threads is observable only in a panic message, which folds into the monad's `fail` outcome ([the ownership model](#ownership-model)), so the model sees the annotated function as its unannotated counterpart. |
| Module system (modules, `use` imports, visibility `pub`, paths, crate structure) | Bookkeeping the frontend resolves away: it decides which names a program may refer to, not what the resolved program means. Items may be declared at any nesting, including inside a function body or impl block, and resolve to ordinary items wherever they appear. |
| Type aliases (including associated type aliases) | Substituted by the type they name before the model sees them. |
| Constants (`const` items, `const fn`, inline `const { }` blocks, immutable `static`) | Ordinary constant values that inline into the model; only `static mut`, a mutable global, falls outside it ([outside the value model](#outside-the-value-model)). |
| Implicit coercions | Each desugars to a construct already covered: a deref coercion to a `Deref` / `DerefMut` call ([traits](#traits)), an unsizing coercion to the trait object ([traits](#traits)) or slice ([types](#types)) it names, a trait-upcasting coercion (`&dyn Sub` to `&dyn Super`) into that same `dyn` existential ([traits](#traits)), the never coercion to the `!` reading ([types](#types)), and the remaining pointer coercions (mutability weakening `&mut T` to `&T`, and a function item or non-capturing closure to a function pointer) to their targets. |

## The ownership model {#ownership-model}

Everything about the shape of the input language follows from one move: programs are modeled as pure functions over *values*, with no heap and no addresses. Rust makes this possible because its ownership discipline already guarantees the absence of the aliasing that would otherwise force memory reasoning. With this move understood, the rest of the subset is a set of consequences rather than a list of arbitrary restrictions.

**Values, not addresses.** A parameter of type `&mut T` is modeled as a plain `T`: no pointer, no location, nothing to alias. This is sound precisely because the borrow checker guarantees that a live `&mut T` is the *unique* way to reach that value.

**Returned mutable borrows: backward functions.** The interesting case is a function that hands a mutable borrow *back* to its caller:

```rust
fn first_mut(v: &mut Vec<u32>) -> &mut u32 { &mut v[0] }
```

In the model this becomes two things: a *forward* result (the `u32` value at index 0) and a *backward function*, a continuation that takes the possibly updated `u32` and writes it back into the `Vec`, yielding the updated `Vec`. Together they behave like a lens: the forward direction is the "get", the backward direction the "put". The caller mutates through the borrow exactly as in Rust; the model just makes the write-back explicit. Because returned mutable borrows have this clean functional reading, they are part of the input language, including `&mut` in return types.

**Effects are explicit: the `Result` monad.** Rust functions can panic, overflow, index out of bounds, or diverge. The model makes each explicit by wrapping results in a `Result`-like monad with `ok`, `fail`, and `div` outcomes. This effect monad is distinct from Rust's own `Result<T, E>`, an ordinary enum that translates directly as an algebraic data type: the monad wraps *every* function to track panics and divergence, whether or not it returns a `Result`. "This function does not panic" is therefore not an assumption; it is a property you state and prove. A panic is the `fail` outcome, never a value a running program observes, so *catching* one (`catch_unwind`, or unwinding across a boundary) has no faithful reading and stays outside the input language, the same carve-out an observable `Drop` runs into below. An `abort`, a non-unwinding halt whether from `panic = "abort"`, a double panic, or `process::abort`, is the same `fail` outcome and needs no separate treatment: it is termination that is excluded from being caught, not termination itself.

**Destructors: drop glue is erased.** Because a value is data with no location, dropping it is not an observable event: the model simply stops mentioning the value, and the compiler-generated drop glue that would recursively free a `Vec` or a `Box` has nothing to do in a world with no heap. Types therefore carry no destructor in the model. A custom `Drop` impl used only for cleanup is sound under this reading, but one whose `drop` body has an *observable* effect, or whose ordering across scopes is load-bearing, has no faithful reading, because the value model has no effect at scope exit to attach it to. That single case aside, `Drop` is transparent. The [Lean support page](../lean/support.md) records how the pipeline treats such a destructor today.

**Lifetimes, layout, ABI, move-vs-copy, and marker traits are erased.** All of these are bookkeeping the borrow checker, the type checker, and the code generator use to license the value model, and none survives into it.

- *Lifetimes.* Explicit lifetime parameters and bounds (`'a`, `'static`, `T: 'a`) govern which borrows the compiler accepts, but once a program type-checks the model works over plain values and drops them; the one place a lifetime stays visible is a higher-ranked bound ([traits](#traits)), which reads as quantification.
- *Layout.* Because a value is data and not bytes at an address, `#[repr(...)]`, alignment, and padding have no counterpart in the model, which is also the deeper reason the layout-dependent constructs of [outside the value model](#outside-the-value-model) (`union`, `transmute`) have no value reading.
- *ABI.* `extern "C"`, `"system"`, and the other calling conventions fix how arguments cross the machine boundary, not what the call computes, so an `extern "C" fn` reads as the same arrow type its default-ABI counterpart does, and only the consequences that reach the type survive (an unbounded C-variadic list has no arrow type at all, [outside the value model](#outside-the-value-model)).
- *Move vs. copy.* A value in the model can be named as often as needed, so whether Rust would *move* or *copy* it, and whether the type is `Copy`, is a borrow-check question with no counterpart in the model; `Clone` and `Copy` impls are then just ordinary methods. `Pin<P>` rests on the same erasure of moves: with no observable move to prevent, its no-move guarantee constrains nothing, so it reads as the pointer `P` it wraps and `Unpin` erases like the other marker traits.
- *Marker traits.* `Sized` and `?Sized` bounds only decide which types the compiler admits in a given position, and the auto traits `Send` and `Sync` track the thread-safety of the concurrency the model excludes ([outside the value model](#outside-the-value-model)), so neither the bounds nor the traits leave a trace in a model over plain values. `PhantomData<T>` is a field-level marker of the same kind: it carries no value and exists only to record variance and ownership for the borrow checker, so it erases along with the lifetimes it stands in for.

**Two consequences.**

- Aliasing the borrow checker *already* rejects is a non-issue: if it compiles as safe Rust, the model can represent its data flow.
- Every restriction later in this section traces back to this one requirement, *a functional model.* Interior mutability, raw pointers, and concurrency are out ([outside the value model](#outside-the-value-model)) because they break the "unique owner, no addresses" premise.

## Types {#types}

The algebraic core of Rust's type system carries over directly, because each construct has an obvious value in the functional model. The signature/implementation split applies at the type level too: a type whose *representation* cannot be modeled can still be used as an opaque type, its operations supplied by models, even though its internals do not translate.

| Feature | Status | Notes |
| --- | --- | --- |
| Scalar integer types | Supported | Modeled as checked machine integers; see [arithmetic](#arithmetic). |
| `bool` | Supported | Some forms are not yet translated; see the [planned constructs](../lean/support.md#planned) of the Lean backend. |
| `char` | Supported | A Unicode scalar value. Some forms are not yet translated; see the [planned constructs](../lean/support.md#planned) of the Lean backend. |
| Never type `!` | Supported | The uninhabited type of a diverging computation; read through the monad's `fail` / `div` outcome ([the ownership model](#ownership-model)). Some forms are not yet translated; see the [planned constructs](../lean/support.md#planned) of the Lean backend. |
| Structs / enums / tuples | Supported | Algebraic data types, translated directly. |
| Fixed-size arrays `[T; N]` | Supported | Some forms are not yet translated; see the [planned constructs](../lean/support.md#planned) of the Lean backend. |
| Slices `[T]` | Supported | Safe slice API is modeled; the unsafe indexing core is in the [trusted kernel](trust.md#trusted-kernel). |
| String slices `str` / `&str` | Supported | Safe `str` API is modeled; the unsafe core is in the [trusted kernel](trust.md#trusted-kernel). Owned `String` is a container ([the library surface](#library-surface)). Some forms are not yet translated; see the [planned constructs](../lean/support.md#planned) of the Lean backend. |
| `&T` / `&mut T` | Supported | Modeled by value; `&mut` via backward functions ([the ownership model](#ownership-model)). Some forms are not yet translated; see the [planned constructs](../lean/support.md#planned) of the Lean backend. |
| `Box<T>` | Supported | Indirection is erased in the value model. |
| `Rc<T>` / `Arc<T>` | Partial | Shared *immutable* ownership has a value reading: the value is simply named more than once, as with `Box`, and the model is supplied on that basis. The reference-count and identity operations (`Rc::strong_count`, `Rc::ptr_eq`) have no value reading. Shared *mutation* via `Rc<RefCell<T>>` is the interior-mutability case ([outside the value model](#outside-the-value-model)); [thread the state](idioms.md#shared-state) instead. |
| Floating point `f32` / `f64` | Supported | The operations rest on an assumed model, so numeric facts about float results cannot be proven; see [arithmetic](#arithmetic) and [model trust](trust.md). |
| Literals | Supported | Integer, float, `char`, string, byte-string, and C-string literals are constant values of their types; float literals inherit the assumed float model ([arithmetic](#arithmetic)). Some forms are not yet translated; see the [planned constructs](../lean/support.md#planned) of the Lean backend. |
| Raw pointers `*const/*mut` | Unsupported | No functional model (see [outside the value model](#outside-the-value-model)); the `&raw const` / `&raw mut` borrow operator that produces one is out on the same ground. |
| Interior mutability (`Cell`, `RefCell`) | Unsupported | Breaks the unique-owner premise ([outside the value model](#outside-the-value-model)). |
| `union` | Unsupported | Overlapping representations have no value semantics; reads are `unsafe` ([outside the value model](#outside-the-value-model)). |

## Functions, closures, generics {#functions}

Functions, generics, and closures are all part of the input language. Generics are resolved the way the backend expects and impose no restriction of their own. Closures are modeled including their captured environment; a closure that mutates through a captured `&mut` is handled by threading the captured state, the same mechanism as returned mutable borrows ([the ownership model](#ownership-model)). The callable interface behind them, the `Fn` / `FnMut` / `FnOnce` family, adds nothing of its own: such a bound is an ordinary trait bound ([traits](#traits)), and the three traits differ only in how the body reaches its captures (by `&`, `&mut`, or move), the capture distinction just described. Function pointers and `impl Trait` round out the ways to pass behavior: a function pointer is an arrow type with a direct functional reading, while `impl Trait` is generics in argument position and an existential (like `dyn`, [traits](#traits)) in return position.

| Feature | Status | Notes |
| --- | --- | --- |
| Generic functions | Supported | Some forms are not yet translated; see the [planned constructs](../lean/support.md#planned) of the Lean backend. |
| Closures | Supported | Captured environment is part of the model. Some forms are not yet translated; see the [planned constructs](../lean/support.md#planned) of the Lean backend. |
| Mutation through closure captures | Supported | Captured state is threaded through the closure. Some forms are not yet translated; see the [planned constructs](../lean/support.md#planned) of the Lean backend. |
| `Fn` / `FnMut` / `FnOnce` bounds | Supported | The callable interface behind closures; an ordinary trait bound ([traits](#traits)). Some forms are not yet translated; see the [planned constructs](../lean/support.md#planned) of the Lean backend. |
| Const generics | Supported | |
| Function pointers (`fn`) | Supported | Arrow types with a clean functional reading. Some forms are not yet translated; see the [planned constructs](../lean/support.md#planned) of the Lean backend. |
| Methods and inherent impls | Supported | A method is a function whose first parameter is its receiver (`self`, `&self`, `&mut self`, or a wrapper such as `Pin<&mut Self>`), so it reads as that function. Some forms are not yet translated; see the [planned constructs](../lean/support.md#planned) of the Lean backend. |
| `impl Trait` | Supported | Argument position is generics; return position is an existential (cf. `dyn`, [traits](#traits)). Some forms are not yet translated; see the [planned constructs](../lean/support.md#planned) of the Lean backend. |

## Traits and trait resolution {#traits}

Traits map onto the target's type-class machinery, and the full resolution story carries over: associated types and constants, default methods, blanket impls, and higher-ranked trait bounds. Trait objects are supported as well, building on the `dyn` coverage in the Aeneas basis. Which traits may form a `dyn` object is settled upstream by Rust's own object-safety (dyn-compatibility) rules, not by the model: a trait with a generic method, or a method that takes or returns `Self` by value, is not object-safe and never reaches the model as a trait object in the first place. Operator overloading needs no separate treatment: the operators and coercions defined by traits (`Add` and its siblings, the comparison and equality operators `==` / `!=` / `<` / `<=` through `PartialEq` / `Eq` / `PartialOrd` / `Ord`, `Index`, `Deref`) desugar to ordinary trait-method calls and inherit the status of the impl behind them.

| Feature | Status | Notes |
| --- | --- | --- |
| Trait definitions / impls | Supported | Modeled as type classes. |
| Associated types | Supported | The generic case (GATs) is not yet translated; see the [planned constructs](../lean/support.md#planned) of the Lean backend. |
| Associated constants | Supported | Constant values carried by a trait or impl; inline like other constants ([the Rust subset](language.md)). |
| Default methods | Supported | |
| Blanket impls | Supported | |
| Higher-ranked trait bounds | Supported | |
| Trait objects (`dyn`) | Supported | Modeled by the Aeneas `dyn` existential; dispatch through a trait object is not yet translated, see the [planned constructs](../lean/support.md#planned) of the Lean backend. |

## Control flow and pattern matching {#control-flow}

Conditionals and pattern matching translate directly, including the pattern forms that reach into structure; each has a clean reading as a match in the target logic. The binding conditionals (`if let`, `while let`, `let else`), including the `&&`-chained let-chain form, and labeled blocks are further sugar over `match` and carry no separate model. A destructuring assignment (`(a, b) = (b, a)`), together with the underscore-place assignment (`_ = f()`) it generalizes, likewise desugars to ordinary assignments to the individual places and needs no separate model. Error propagation with `?` is included as well, being sugar for a match that returns early on the error case, converting the error through `From` on the way out, an ordinary trait-method call ([traits](#traits)).

| Feature | Status | Notes |
| --- | --- | --- |
| `if` / `match` | Supported | |
| `if let` / `while let` / `let else` | Supported | Sugar for a `match`; the non-matching arm of `let else` diverges (`!`, [types](#types)). |
| Labeled blocks and `break` with a value | Supported | A labeled block reads as an expression yielding its `break` value; multi-level jumps are not yet translated, see the [planned constructs](../lean/support.md#planned) of the Lean backend. |
| `?` operator (error propagation) | Supported | Sugar for a match on the real `Result` / `Option` enum, with the error converted via `From` ([traits](#traits)), distinct from the effect monad ([the ownership model](#ownership-model)); inside nested loops its early return is a multi-level jump (see the previous row). |
| Literal patterns | Supported | A literal compared against the scrutinee. Some forms are not yet translated; see the [planned constructs](../lean/support.md#planned) of the Lean backend. |
| Range patterns (`0..12`) | Supported | |
| Match guards (`p if cond`) | Supported | An extra boolean condition on a match arm; reads as a guard in the target match. |
| `@` bindings | Supported | |
| Reference patterns (`&`, `ref`, `ref mut`) | Supported | Match through or into a borrow; the default binding modes desugar to these. |
| Slice patterns (`[head, ..tail]`) | Supported | Some forms are not yet translated; see the [planned constructs](../lean/support.md#planned) of the Lean backend. |
| Or-patterns | Supported | |

## Loops and recursion {#loops}

All loop forms and recursion are part of the input language. Aeneas translates loops by joining loop states, and recursion is admitted once a termination measure discharges well-foundedness (`#[decreases]`, `loop_decreases!`; see [contracts](specification.md#contracts)). Some multi-level control-flow forms are not yet translated by the current implementation; the [planned constructs](../lean/support.md#planned) of the Lean backend record them.

A `for` loop is sugar rather than a primitive: it desugars to a `loop` that calls `Iterator::next` on the `IntoIterator` and matches the resulting `Some` / `None`, so it inherits the loop reading above and the `loop_invariant!` / `loop_decreases!` idiom ([contracts](specification.md#contracts)) attaches to that desugared loop. The range expressions that drive the common `0..n` form (and the `0..=n`, `n..`, `..n`, and `..` variants) build ordinary `core` range structs, algebraic data types ([types](#types)) like any other, which reach the model either as the iterator a `for` loop drives or as the index argument to a slicing operation ([constraining the inputs](idioms.md#panicking-operations)); they are distinct from the range *patterns* of [control flow](#control-flow). Iterator *adapters* (`map`, `filter`, `enumerate`, `zip`, `fold`, and the rest) are ordinary trait methods returning adapter values; each advance drives the underlying `next` through the same backward-function model any `&mut self` uses ([the ownership model](#ownership-model)), and their laziness is just the ordinary reading of a value that yields its elements on demand. An adapter is supported whether its body translates or it is reached through the modeled `core` iterator surface ([the library surface](#library-surface)). The index-based invariant in the `0..n` examples of [contracts](specification.md#contracts) and [stating the invariant](idioms.md#loop-invariants) is the simplest case of this; over an adapter chain the invariant is stated over the desugared loop's state rather than a numeric index.

*Producing* an iterator by suspension is the unsupported exception: a `gen` block or generator function `yield`s its elements from a suspended state machine, which needs an execution model and so lands [outside the value model](#outside-the-value-model), the same reading as `async`.

| Feature | Status | Notes |
| --- | --- | --- |
| `loop` / `while` / `for` | Supported | Jumps out of nested loops are not yet translated; see the [planned constructs](../lean/support.md#planned) of the Lean backend. |
| Iterators | Supported | Includes adapter chains (`map` / `filter` / `fold`, and so on); a `for` loop desugars to `Iterator::next` over `IntoIterator` (see above). |
| Recursion (with termination measure) | Supported | Measure via `#[decreases]` ([contracts](specification.md#contracts)). |

## Mutation and the `&mut` model {#mutation}

The practical rules for mutation all follow from the [ownership model](#ownership-model), including the point made there that any aliasing the borrow checker accepts is expressible.

| Feature | Status | Notes |
| --- | --- | --- |
| Passing / returning `&mut` | Supported | The argument is a value; a returned `&mut T` carries its backward function ([the ownership model](#ownership-model)). |
| Shared mutation | Unsupported | Requires interior mutability ([outside the value model](#outside-the-value-model)). [Thread the state explicitly](idioms.md#shared-state) instead. |

## Arithmetic, machine integers, and floating point {#arithmetic}

**Machine integers.** The scalar integer types are modeled as checked machine integers: every operation that can overflow produces a `fail` in the `Result` monad ([the ownership model](#ownership-model)) unless you prove it cannot. To reason without the noise of overflow, lift into the mathematical-integer model (`Int`, `int!`, `ToInt`; see [refinement and abstraction](specification.md#refinement)), prove the arithmetic there, and connect back at the boundary.

**Casts (`as`).** The `as` operator converts between scalar types. Casts among the integer types, between `bool` and an integer, between `char` and `u32` (a `char` is always a valid `u32`, and the sole integer-to-`char` cast Rust permits is from `u8`), and from a fieldless enum to an integer (its discriminant) are modeled directly: they are *total* and follow Rust's truncation and sign-extension rules, so unlike a checked `TryFrom` conversion an `as` cast never fails but may silently lose information, and that lossiness is visible in the model rather than hidden. Casts to or from floating point are float operations and inherit the assumed float model (below and [the trusted kernel](trust.md#trusted-kernel)). Casts that leave the scalar world are governed by their own rules rather than by this paragraph: an unsizing cast to a trait object by the trait-object rules ([traits](#traits)), and a raw-pointer cast by the raw-pointer rules ([outside the value model](#outside-the-value-model)).

**Floating point.** This is the concrete case of a supported construct whose meaning rests on an assumed model. The `f32` and `f64` *types* are signature-expressible, so float-using code passes through and everything around the float operations can be reasoned about. The *operations* have a functional reading, but only an assumed opaque model of it is provided ([model trust](trust.md)). Concretely, you can prove the shape and control flow of a computation carrying floats, but not numeric facts about the float results. Faithful IEEE-754 reasoning (rounding, NaN, signed zero) is not provided: the semantics are hard and SMT support is thin. The [planned constructs](../lean/support.md#planned) of the Lean backend record what does not extract today.

## Outside the value model {#outside-the-value-model}

These constructs have no faithful reading in the value-based [ownership model](#ownership-model), because they contradict it rather than merely lacking a translated body; most of them are not even signature-expressible. That makes them different in kind from floating point, which *is* expressible and simply needs a model. The **Prospects** column records whether an item is on the *separation-logic horizon* (outside today's value model, but targeted by ongoing separation-logic and ghost-state research aimed at unsafe and aliasing) or has *no anticipated model* (not on any current research path).

| Construct | Why it is out | Prospects |
| --- | --- | --- |
| Raw-pointer manipulation (`*const`/`*mut`, the `&raw` borrow) | Reasons about addresses; there are no addresses in the value model. | Separation-logic horizon |
| Operations that need `unsafe` and have no value semantics (raw-pointer dereference and arithmetic, `transmute`, uninitialized memory, `union` reads) | Each reads or writes memory outside the ownership discipline that makes the functional translation sound. | Separation-logic horizon |
| Interior mutability (`UnsafeCell`, `Cell`, `RefCell`) | Mutation through a shared reference destroys the unique-owner guarantee the value model depends on. | Separation-logic horizon |
| Concurrency and atomics | Observable interleavings require a memory model the sequential value semantics does not provide. | Separation-logic horizon |
| `async` / `.await` | Compiles to a poll-driven state machine whose suspension points need an execution model, not just a value model. | No anticipated model |
| Generators and `gen` blocks (`yield`) | Compile to a resume-driven state machine whose suspension points need an execution model, the same shape as `async`. | No anticipated model |
| `union` | Overlapping fields have no single value representation, and every read is `unsafe`. | No anticipated model |
| Mutable statics (`static mut`, `thread_local!`) | A mutable global is shared mutable state with no owner to thread, the same premise break as interior mutability; [thread the state explicitly](idioms.md#shared-state) instead. | Separation-logic horizon |
| Inline assembly (`asm!`, `global_asm!`, `#[naked]` + `naked_asm!`) | Arbitrary machine instructions have no Rust-level value semantics to translate. | No anticipated model |
| C-variadic functions (`extern "C" fn(..., ...)`) | An unbounded, untyped argument list has no fixed arrow type to express. | No anticipated model |

The `unsafe` qualifier itself, on a function or a block, is not among them: the pipeline erases it, since MIR has no unsafe blocks and the flag on a signature is never read, so an `unsafe fn` whose body stays clear of the operations above translates like a safe one. A modeled unsafe library function such as `get_unchecked` is a checked call under its model's precondition, so a call that violates it is a `fail` outcome ([the ownership model](#ownership-model)) rather than undefined behavior.

The value model this section describes is sequential and address-free, so none of these has a faithful reading in it today. Where a *safe*, value-compatible facade exists over one of these (for example, the safe slice API over an unsafe indexing core), the facade is built on the [trusted kernel](trust.md#trusted-kernel): the unsafe core stays trusted, and the facade is proven on top of it.

## The library surface {#library-surface}

The input language includes a library surface, not just language constructs. Most of it is usable through hand-written models rather than by extracting the real (often unsafe) bodies, and that is how the library is provided, not a limitation to work around. Library surface beyond the in-scope part of `core` and the modeled containers is out of scope. Formatting is the common example of surface that falls outside this: the `Display` / `Debug` machinery reached through `format!` and `println!` is out of scope rather than something a model supplies. A distinct class sits below even that: surface whose meaning is an effect on the outside world or a nondeterministic result, file and network I/O, the system clock (`SystemTime::now`), randomness, and environment or process access other than `abort` (the `fail` outcome of [the ownership model](#ownership-model)), is not merely unmodeled but impure, with no functional reading at all. It is excluded on the same ground as the constructs of [outside the value model](#outside-the-value-model), not awaiting a model, because there is no pure value for a model to denote.

| Surface | Status | Notes |
| --- | --- | --- |
| `core` | Partial | The safe API with a value reading, provided through hand-written models rather than by extracting real bodies. Out of scope by design: raw pointers (`ptr`), interior mutability (`cell`), atomics, and `async` ([outside the value model](#outside-the-value-model)), and formatting (`fmt`, see above). `transmute` and the intrinsics are not callable from the input language; models use them only as items of the [trusted kernel](trust.md#trusted-kernel), the bounded residue that can never become a faithful extraction and on which everything else is modeled. The current coverage of the models is discussed in [assumed vs. proven models](trust.md#assumed-vs-proven). |
| Modeled `alloc` / `std` containers | Partial | `Vec`, `String`, `HashMap`, and the like. Their value operations (construction, lookup, insertion, removal) are in scope; the iteration order of a randomly seeded map or set (`HashMap` / `HashSet` with the default `RandomState`) is not, since randomness has no functional reading. See [assumed vs. proven models](trust.md#assumed-vs-proven) for the current state of the container models. |
