---
weight: 5
---

# Quick reference

An alphabetical index of the input language. Status meanings are in [the status vocabulary](index.md#status-vocabulary) (Supported / Partial / Unsupported). Many items marked *Supported* are not fully implemented today; those rows point to the [planned constructs](../lean/support.md#planned) of the Lean backend in the See column, where the FLS test suite lists the gaps it covers with their reasons. Gaps in the coverage of the modeled library are tracked in [model trust](trust.md#assumed-vs-proven).

| Feature | Status | See |
| --- | --- | --- |
| `?` operator (error propagation) | Supported | [control flow](language.md#control-flow), [planned](../lean/support.md#planned) |
| `@` bindings | Supported | [control flow](language.md#control-flow) |
| ABI / calling convention (`extern "C"`) | Supported | [the ownership model](language.md#ownership-model) (erased) |
| Arrays, fixed-size (`[T; N]`) | Supported | [types](language.md#types), [planned](../lean/support.md#planned) |
| Associated constants | Supported | [traits](language.md#traits) |
| Associated types | Supported | [traits](language.md#traits), [planned](../lean/support.md#planned) |
| `async` / `.await` | Unsupported | [outside the value model](language.md#outside-the-value-model) |
| Atomics / concurrency | Unsupported | [outside the value model](language.md#outside-the-value-model) |
| Attributes (`derive` / `cfg` / `repr` / `inline` / `track_caller`, diagnostic and linkage) | Supported | [the Rust subset](language.md) (transparent) |
| Blanket impls | Supported | [traits](language.md#traits) |
| `bool` | Supported | [types](language.md#types), [planned](../lean/support.md#planned) |
| `Box<T>` | Supported | [types](language.md#types) |
| Casts (`as`) | Supported | [arithmetic](language.md#arithmetic), [planned](../lean/support.md#planned) |
| `catch_unwind` (panic catching) | Unsupported | [the ownership model](language.md#ownership-model) |
| `char` | Supported | [types](language.md#types), [planned](../lean/support.md#planned) |
| `Clone` / `Copy` / move semantics | Supported | [the ownership model](language.md#ownership-model) |
| Closure traits (`Fn` / `FnMut` / `FnOnce`) | Supported | [functions and closures](language.md#functions), [planned](../lean/support.md#planned) |
| Closures | Supported | [functions and closures](language.md#functions), [planned](../lean/support.md#planned) |
| Const generics | Supported | [functions and closures](language.md#functions) |
| Constants (`const` / `const fn` / immutable `static`) | Supported | [the Rust subset](language.md) (transparent), [planned](../lean/support.md#planned) |
| Containers (`Vec`, `String`, `HashMap`) | Partial | [the library surface](language.md#library-surface), [assumed vs. proven models](trust.md#assumed-vs-proven) |
| Control flow (`if` / `match`) | Supported | [control flow](language.md#control-flow) |
| `core` library (`Option` / `Result` / `cmp` / `convert` / ...) | Partial | [the library surface](language.md#library-surface), [assumed vs. proven models](trust.md#assumed-vs-proven) |
| Coroutines / generators (`gen` / `yield`) | Unsupported | [loops and recursion](language.md#loops), [outside the value model](language.md#outside-the-value-model) |
| Default methods | Supported | [traits](language.md#traits) |
| Destructors (`Drop`) | Partial | [the ownership model](language.md#ownership-model) |
| Destructuring assignment (`(a, b) = ...`) | Supported | [control flow](language.md#control-flow) |
| Floating point (`f32` / `f64`) | Supported | [arithmetic](language.md#arithmetic), [model trust](trust.md), [planned](../lean/support.md#planned) |
| Foreign functions / statics (`extern`) | Supported | [making it opaque](idioms.md#opaque-and-model), [planned](../lean/support.md#planned) |
| Formatting (`format!` / `println!`, `Display` / `Debug`) | Unsupported | [the library surface](language.md#library-surface) |
| Function pointers (`fn`) | Supported | [functions and closures](language.md#functions), [planned](../lean/support.md#planned) |
| Generic functions | Supported | [functions and closures](language.md#functions), [planned](../lean/support.md#planned) |
| Higher-ranked trait bounds | Supported | [traits](language.md#traits) |
| I/O, clock, randomness, environment (`std::fs`, `SystemTime::now`, `std::env`) | Unsupported | [the library surface](language.md#library-surface), [outside the value model](language.md#outside-the-value-model) |
| `if let` / `while let` / `let else` | Supported | [control flow](language.md#control-flow) |
| `impl Trait` | Supported | [functions and closures](language.md#functions), [planned](../lean/support.md#planned) |
| Implicit coercions | Supported | [the Rust subset](language.md) (transparent), [planned](../lean/support.md#planned) |
| Inline assembly (`asm!` / `global_asm!` / naked) | Unsupported | [outside the value model](language.md#outside-the-value-model) |
| Integers, machine (scalar) | Supported | [types](language.md#types), [arithmetic](language.md#arithmetic) |
| Interior mutability (`Cell`, `RefCell`, `UnsafeCell`) | Unsupported | [outside the value model](language.md#outside-the-value-model) |
| Iterators | Supported | [loops and recursion](language.md#loops), [assumed vs. proven models](trust.md#assumed-vs-proven) |
| Labeled blocks / `break` value | Supported | [control flow](language.md#control-flow), [planned](../lean/support.md#planned) |
| Lifetimes (`'a` / `'static`) | Supported | [the ownership model](language.md#ownership-model), [planned](../lean/support.md#planned) |
| Literal patterns | Supported | [control flow](language.md#control-flow), [planned](../lean/support.md#planned) |
| Literals (integer / float / `char` / string / byte string / C string) | Supported | [types](language.md#types), [planned](../lean/support.md#planned) |
| Loops (`loop` / `while` / `for`) | Supported | [loops and recursion](language.md#loops), [planned](../lean/support.md#planned) |
| Macros (`macro_rules!` / procedural) | Supported | [the Rust subset](language.md) (transparent) |
| Marker / auto traits (`Sized` / `Send` / `Sync`) | Supported | [the ownership model](language.md#ownership-model) (erased), [planned](../lean/support.md#planned) |
| Match guards (`p if cond`) | Supported | [control flow](language.md#control-flow) |
| Methods and inherent impls (`impl Type`, `self` receivers) | Supported | [functions and closures](language.md#functions), [planned](../lean/support.md#planned) |
| Modules / `use` / visibility (`pub`) | Supported | [the Rust subset](language.md) (transparent), [planned](../lean/support.md#planned) |
| Mutable references (`&mut T`) | Supported | [the ownership model](language.md#ownership-model), [mutation](language.md#mutation), [planned](../lean/support.md#planned) |
| Mutable statics (`static mut`, `thread_local!`) | Unsupported | [outside the value model](language.md#outside-the-value-model) |
| Mutation through closure captures | Supported | [functions and closures](language.md#functions), [planned](../lean/support.md#planned) |
| Never type (`!`) | Supported | [types](language.md#types), [planned](../lean/support.md#planned) |
| `#[non_exhaustive]` | Supported | [the Rust subset](language.md) (transparent) |
| Operator overloading (`Add` / `Index` / `Deref`) | Supported | [traits](language.md#traits) |
| Or-patterns | Supported | [control flow](language.md#control-flow) |
| Panics (`panic!` / `assert!` / `unwrap` / overflow / `abort`) | Supported | [the ownership model](language.md#ownership-model), [arithmetic](language.md#arithmetic) |
| `PhantomData<T>` | Supported | [the ownership model](language.md#ownership-model) (erased) |
| `Pin<P>` / `Unpin` | Supported | [the ownership model](language.md#ownership-model) (erased), [planned](../lean/support.md#planned) |
| Range expressions (`0..n`) | Supported | [loops and recursion](language.md#loops), [planned](../lean/support.md#planned) |
| Range patterns (`0..12`) | Supported | [control flow](language.md#control-flow) |
| Raw pointers (`*const` / `*mut`) | Unsupported | [outside the value model](language.md#outside-the-value-model) |
| `Rc<T>` / `Arc<T>` | Partial | [types](language.md#types) |
| Recursion (with termination measure) | Supported | [loops and recursion](language.md#loops) |
| Reference patterns (`&` / `ref`) | Supported | [control flow](language.md#control-flow) |
| References, shared (`&T`) | Supported | [types](language.md#types), [planned](../lean/support.md#planned) |
| Runtime items (`main` / `#[global_allocator]` / `#[panic_handler]` / `no_std`) | Supported | [the Rust subset](language.md) (transparent) |
| Shared mutation | Unsupported | [mutation](language.md#mutation), [thread the state](idioms.md#shared-state) |
| Slice patterns (`[head, ..tail]`) | Supported | [control flow](language.md#control-flow), [planned](../lean/support.md#planned) |
| Slices (`[T]`) | Supported | [types](language.md#types) |
| String slices (`str` / `&str`) | Supported | [types](language.md#types), [planned](../lean/support.md#planned) |
| Structs / enums / tuples | Supported | [types](language.md#types) |
| Trait objects (`dyn`) | Supported | [traits](language.md#traits), [planned](../lean/support.md#planned) |
| Traits and impls | Supported | [traits](language.md#traits) |
| Type aliases (`type`) | Supported | [the Rust subset](language.md) (transparent) |
| `union` | Unsupported | [outside the value model](language.md#outside-the-value-model) |
| `unsafe` blocks and `unsafe fn` (the qualifier) | Supported | [outside the value model](language.md#outside-the-value-model) (erased) |
| `unsafe` operations (raw-pointer dereference, `transmute`, `union` reads) | Unsupported | [outside the value model](language.md#outside-the-value-model) |
| Variadic functions (C-variadic `extern`) | Unsupported | [outside the value model](language.md#outside-the-value-model), [making it opaque](idioms.md#opaque-and-model) |

**[Specification surface](specification.md).** The vocabulary for stating properties and supplying models is provided by `hax-lib` rather than being a Rust construct with a status, so it is indexed separately here. Entries that are not fully implemented today point to the planned constructs as above.

| Construct | See |
| --- | --- |
| `Abstraction` / `Concretization` | [refinement and abstraction](specification.md#refinement), [planned](../lean/support.md#planned) |
| `assert_prop!` | [logic and propositions](specification.md#logic), [planned](../lean/support.md#planned) |
| `assume!` | [logic and propositions](specification.md#logic), [planned](../lean/support.md#planned) |
| `#[attributes]` | [contracts](specification.md#contracts) |
| Connectives (`and` / `or` / `not` / `implies`) | [logic and propositions](specification.md#logic) |
| `#[decreases]` | [contracts](specification.md#contracts) |
| `#[ensures]` | [contracts](specification.md#contracts) |
| `#[ensures_ref]` | [contracts](specification.md#contracts) |
| `#[exclude]` / `#[include]` | [controlling extraction](specification.md#supplying-models) |
| `forall` / `exists` | [logic and propositions](specification.md#logic) |
| `future(...)` | [contracts](specification.md#contracts), [talking about a mutation](idioms.md#mutation-results) |
| `hax_lib::assert!` / `hax_lib::debug_assert!` | [logic and propositions](specification.md#logic), [planned](../lean/support.md#planned) |
| `implies!` | [logic and propositions](specification.md#logic) |
| `Int` / `int!` / `ToInt` | [refinement and abstraction](specification.md#refinement) |
| `#[lemma]` | [controlling extraction](specification.md#supplying-models) |
| `loop_decreases!` | [contracts](specification.md#contracts), [planned](../lean/support.md#planned) |
| `loop_invariant!` | [contracts](specification.md#contracts), [stating the invariant](idioms.md#loop-invariants), [planned](../lean/support.md#planned) |
| `#[opaque]` | [controlling extraction](specification.md#supplying-models), [planned](../lean/support.md#planned) |
| `Prop` | [logic and propositions](specification.md#logic) |
| `#[refinement_type]` / `Refinement` / `RefineAs` | [refinement and abstraction](specification.md#refinement), [planned](../lean/support.md#planned) |
| `#[requires]` | [contracts](specification.md#contracts) |
| `ToProp` | [logic and propositions](specification.md#logic) |
| `#[transparent]` | [controlling extraction](specification.md#supplying-models) |
