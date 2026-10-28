# Architecture

hax is a software pipeline designed to transform Rust code into various formal verification backends such as **Lean**, **F\***, **Rocq**, **ProVerif**, and **EasyCrypt**. The Lean backend runs the external [Charon](https://github.com/AeneasVerif/charon) + [Aeneas](https://github.com/AeneasVerif/aeneas) pipeline: `cargo-hax` orchestrates it, and none of the components below are involved.

The remaining backends, including the legacy Lean backend, run the legacy pipeline this page describes. It comprises two main components:

1. **The Frontend** (written in Rust)
2. **The Engine** (written in OCaml, with a partial rewrite in Rust)

The frontend hooks into the Rust compiler, producing an abstract syntax tree for a given crate. The engine then takes this AST as input and applies various transformations, to reach in the end the language of the backend: F*, Rocq...

## The Frontend (Rust)

The frontend is responsible for extracting and exporting Rust code's abstract syntax trees (ASTs) in a format suitable for processing by the engine (or by other tools).

### [`hax-frontend-exporter` Library](https://hax.cryspen.com/frontend/docs/hax_frontend_exporter/)

This library mirrors the internal types of the Rust compiler (`rustc`) that constitute the **HIR** (High-Level Intermediate Representation), **THIR** (Typed High-Level Intermediate Representation), and **MIR** (Mid-Level Intermediate Representation) ASTs. It extends them with additional information such as attributes, trait implementations, and removes ID indirections.

**`SInto` Trait:** The library defines an entry point for translating a given `rustc` value to its mirrored hax version using the [`SInto`](https://hax.cryspen.com/frontend/docs/hax_frontend_exporter/trait.SInto.html) trait (stateful `into`). For a value `x` of type `T` from `rustc`, if `T` is mirrored by hax, then `x.sinto(s)` produces an augmented and simplified "hax-ified" AST for `x`. Here, `s` represents the state holding information about the translation process.

### `hax-driver` Binary

`hax-driver` (crate name; the binary is called `driver-hax-frontend-exporter`) is a custom Rust compiler driver that behaves like `rustc` but performs additional tasks:

1. **Item Enumeration:** Lists all items in a crate.
2. **AST Transformation:** Applies `sinto` on each item to generate the hax-ified AST.
3. **Output Generation:** Outputs the mirrored items into a `haxmeta` file within the `target` directory.

### `cargo-hax` Binary

`cargo-hax` provides a `hax` subcommand for Cargo, accessible via `cargo hax --help`. It serves as the command-line interface for hax, orchestrating both the frontend and the engine.

**Workflow:**

1. **Custom Build Execution:** Runs `cargo check`, instructing Cargo to use `hax-driver` instead of `rustc`.
2. **Multiple Compiler Invocations:** `cargo check` invokes `hax-driver` multiple times with various options.
3. **Inter-Process Communication:** `hax-driver` communicates with `cargo-hax` via `stderr` using JSON lines.
4. **Metadata Generation:** Produces `haxmeta` files containing the transformed ASTs.
5. **Engine Invocation (Optional):** If requested, runs the engine, passing options and `haxmeta` information via `stdin` serialized as JSON.
6. **Interactive Communication:** Engages in interactive communication with the engine.
7. **User Reporting:** Outputs results and diagnostics to the user.

## The Engine (OCaml)

The engine processes the transformed ASTs and options provided via JSON input from `stdin`. It performs several key functions to convert the hax-ified Rust code into the target backend language.

The engine is written in OCaml, and parts of it have been rewritten in Rust in the [`rust-engine`](https://github.com/pereq-dev/hax/tree/main/rust-engine) crate: the legacy Lean backend runs its phases and printer there, and the F\* backend can run its phases there (behind an experimental flag) while delegating printing to the OCaml engine. This section describes the OCaml engine; the concepts (import, internal AST, phases, printers) carry over to the Rust rewrite.

### Importing and Simplifying ASTs

- **AST Importation:** Imports the hax-ified Rust THIR AST. This is module `Import_thir`.
- **Internal AST Conversion:** Converts the imported AST into a simplified and opinionated internal AST designed for ease of transformation and analysis. This is mostly the functor `Ast.Make`.

### Internal AST and Features

The internal AST is defined using a **functor** that takes a list of type-level booleans, referred to as **features**, and produces the AST types accordingly.

Features are for instances, mutation, loops, unsafe code. The enumeration `Features.Enumeration` lists all those features.

**Feature Witnesses:** On relevant AST nodes, feature witnesses are included to enforce constraints at the type level. For example, in the `loop` expression constructor, a witness of type `F.loop` is used, where `F` represents the current feature set. If `F.loop` is an empty type, constructing a `loop` expression is prohibited, ensuring that loops are disallowed in contexts where they are not supported.

### Transformation Phases

The engine executes a sequence of **phases**, which are determined based on the target backend. Each phase:

1. **Input:** Takes a list of items from an AST with specific feature constraints.
2. **Output:** Transforms these items into a new AST type, potentially enabling or disabling features through type-level changes.

The phases can be found in the `Phases` module.

### Backend Code Generation

After completing the transformation phases:

1. **Backend Printer Invocation:** Calls the printer associated with the selected backend to generate the target code.
2. **File Map Creation:** Produces a map from file names to their contents, representing the generated code.
3. **Output Serialization:** Outputs the file map and additional information (e.g., errors) as JSON to `stderr`.

### Communication Protocol

The engine communicates asynchronously with the frontend using a protocol defined in [`hax_types::engine_api::protocol`](https://hax.cryspen.com/frontend/docs/hax_types/engine_api/protocol/index.html). This communication includes:

- **Diagnostic Data:** Sending error messages, warnings, and other diagnostics.
- **Profiling Information:** Providing performance metrics and profiling data.
- **Pretty-Printing Requests:** Requesting formatted versions of Rust source code or diagnostics for better readability.

