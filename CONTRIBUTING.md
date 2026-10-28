# Engineering & Contributing Guidelines

The following is a set of guidelines for contributing to this repository.
These are mostly guidelines, not rules.
Use your best judgement, and feel free to propose changes to this document in a pull request.
The processes described here are not to pester you but to increase and maintain code quality.

## Working with this repository

We use issues to organise and prioritise work items.

**Assignee meaning in issues:** The assignee is the person responsible for following up on the issue (making sure it eventually gets addressed). It is usually (but not necessarily) the one working on it.

There can be any number of branches and pull requests for one issue.
But make sure that each issue is clearly linked to the pull request.
There must be one pull request that closes the issue.
If there are multiple PRs for an issue, make sure this is clear in the pull request.

## Development

The documentation of the internal crates of the hax frontend can be
found [here](https://hax.cryspen.com/frontend/index.html).

### Edit the sources (Nix)

Just clone & `cd` into the repo, then run `nix develop .`.
You can also just use [direnv](https://github.com/nix-community/nix-direnv), with [editor integration](https://github.com/direnv/direnv/wiki#editor-integration).

The flake provides several dev shells:

| Shell | Purpose |
|-------|---------|
| `nix develop .` | Hacking on hax itself: the toolchain to build the Rust CLI, the frontend and the OCaml engine. Provides no backend verifier. |
| `nix develop .#fstar` | The above plus F\*, for the F\* backend and the F\* proof libraries. Used by CI to check the proof libraries. |
| `nix develop .#examples` | The above plus ProVerif and Lean (through `elan`), for running `examples/` against a hax you build yourself. |
| `nix develop .#ci-examples` | Running `examples/` against a hax built by the flake, rather than one you build from source. Used by CI. |

The first three shells give you the toolchain to build hax, not a `cargo-hax` binary: run `just build` first (see [below](#building-testing-and-formatting)).

In any Nix command from the README's [*Installation*](README.md#installation) section, replace `github:pereq-dev/hax` by `./some-dir` to compile a local checkout of hax that lives in `./some-dir`.

### Structure of this repository

- `frontend/`: Rust library that hooks into the Rust compiler and
  extracts its internal typed abstract syntax tree
  [**THIR**](https://rustc-dev-guide.rust-lang.org/thir.html) as JSON.
- `engine/`: the simplification and elaboration engine that translates programs
  from the Rust language to various backends (see `engine/backends/`). Written
  in OCaml.
- `rust-engine/`: an on-going rewrite of our engine from OCaml to Rust.
- `cli/`: the `cargo hax` subcommand and the custom rustc drivers it
  uses to run the frontend.
- `hax-lib/`: helper crate providing hax-specific macros (e.g.
  `requires`, `ensures`) for annotating Rust programs.
- `hax-types/`: types shared between the frontend, the CLI, and the engine.
- `proof-libs/`: a symlink to `hax-lib/proof-libs/`, the per-backend
  proof libraries that the extracted code builds against.
- `examples/`: examples showing what hax can do.
- `tests/`: integration tests.
- `docs/`: sources of the [hax website](https://hax.cryspen.com/),
  including the manual and the blog.

### Building, testing, and formatting

We use the [`just` command runner](https://just.systems/). If you use
Nix, the dev shell provides it automatically, if you don't use Nix,
please [install `just`](https://just.systems/man/en/packages.html) on
your system.

The most important commands:

- `just build` builds the Rust and OCaml parts and installs the binaries in PATH (`just rust` and `just ocaml` build one part).
- `just test` runs the hax integration tests.
- `just check-example <name>` extracts and verifies one example, `just check-examples` all of them.
- `just fmt` formats all the code.

Run `just` or `just --list` to list all commands.

### Documentation

`mkdocs.yml` at the repository root configures the [mkdocs-material](https://squidfunk.github.io/mkdocs-material) site built from `docs/`.

`just docs` serves the site locally with live reload (it runs `mkdocs serve`). It requires mkdocs and the plugins:

```bash
pip install mkdocs-material mkdocs-glightbox mkdocs-nav-weight mkdocs-awesome-nav
```

Alternatively, `nix run .#serve-docs` builds the site with Nix and serves the result, without any Python setup (but also without live reload).

The [`pymdownx.snippets`](https://facelessuser.github.io/pymdown-extensions/extensions/snippets/) extension can include files from anywhere in the repository, resolved relative to the repository root. For example, `--8<-- "README.md:subcommands"` on its own line includes the part of `README.md` between the `<!-- --8<-- [start:subcommands] -->` and `<!-- --8<-- [end:subcommands] -->` markers. Prefer such named sections over the line-range form (`--8<-- "README.md:107:114"`), which silently includes the wrong content when the target file's lines shift.

## Pull Requests

We use the GitHub-based PR workflow.
When starting to work on an issue, create a branch and an according pull request that fixes the issue.
The changeset in a pull request must not be larger than 1000 lines (with some exceptions for test snapshots or generated code).
If an issue needs more work than that, split it into multiple pull requests.

By submitting a contribution, you agree it may be distributed under the project's license, and you warrant you have the right to contribute it.

After submitting the pull request, verify that all [status checks](https://help.github.com/articles/about-status-checks/) are passing before asking for review.

While the prerequisites above must be satisfied prior to having your pull request reviewed, the reviewer(s) may ask you to complete additional design work, tests, or other changes before your pull request can be ultimately accepted.

### PR & Commit Guidelines

- Split out mass-changes or mechanical changes into a separate PR from the substantive changes.
- Separate commits into conceptually-separate pieces for review purposes (even if you then later collapse them into a single changeset to merge), if technically possible.
- Address all comments from previous reviews (either by fixing as requested, or explaining why you haven't) before requesting another review.
- If your request only relates to part of the changes, say so clearly.

### Force pushing

It is fine to force-push either (1) before asking for a review or (2) after PR approval, just before merging. Otherwise, in between two reviews, please do not force-push.

### Regressions

When a PR introduces a regression, a fix should be submitted in a
window of 2 days, otherwise the PR will be reverted.

## Rules for the OCaml code
 - Avoid the OCaml standard library (`Stdlib`), prefer [`base`](https://v3.ocaml.org/p/base/latest/doc/index.html) or [`core`](https://v3.ocaml.org/p/core/latest/doc/index.html).
 - Avoid non-total functions (e.g. all the `_exn` functions in `base`).
 - Try to avoid exceptions, if possible.
 - Never use `==`, which is the physical equality, and almost never what you want.

## Contributing to `core-models`

Contributions to the model of Rust's `core` and `alloc` libraries are
welcome. See
[`hax-lib/core-models/README.md`](./hax-lib/core-models/README.md#contributing)
for the instructions.

## Changelog
Our changelog format is based on https://keepachangelog.com/.
Please add an entry under the `## [Unreleased]` section, in a subsection (`Added`, `Changed`, `Deprecated`, `Removed`, `Fixed` -- see https://keepachangelog.com/en/1.0.0/#how) for each notable change.
A release turns the `## [Unreleased]` heading into the version heading, so the section is absent right after a release; recreate it at the top of the file in that case.

Please prefix with `engine:`, `frontend:` or similar.

### Should I add an entry to `CHANGELOG.md`?

**Include in CHANGELOG.md:**
 - New features and enhancements
 - Bug fixes
 - Breaking changes
 - Security patches
 - Major documentation updates
 - Dependency updates that affect users

**Do not include:**
 - Code refactoring with no user impact
 - Minor doc fixes (typos, grammar)
 - CI/CD or tooling changes with no external effect
 - Linting, formatting, or style-only commits
 - Reverts or fixup commits
 - Dependency bumps with no behavioral impact

**Rule of thumb:**
If a user (developer or customer) wouldn’t notice or need to know, leave it out.

## Styleguides

### Optional Title Prefixes for Issues
To help quickly convey the focus of an issue, we sometimes add a short prefix in square brackets at the start of the title: `[prefix] Issue short title`. This is optional; you can just use it if the issue has a clear direction or goal.

Keep it short and intuitive, think of it as a lightweight hint, not a strict taxonomy or replacements for labels or milestones.

Use it when it helps. Leave it out when it doesn’t.

### Git Commit Messages

- Use the present tense
- Use the imperative mood
- Limit the first line to 80 characters
- Don't end the first line of the commit message with a period
- Reference issues and pull requests liberally after the first line
- If the patch is of nontrivial size, point to the important comments in the non-first lines of the commit message.

### Styleguide

Use `rustfmt` for every Rust code and `ocamlformat` for every OCaml
code. From the command line, run `cargo fmt` in the root of hax and
`dune fmt` in `engine`.

### Documentation Styleguide

Use [rustdoc](https://doc.rust-lang.org/rustdoc/index.html) comments
on Rust files and functions. Use
[`odoc`](https://ocaml.github.io/odoc/) comments on OCaml files. It is
mandatory on public functions and encouraged on internal functions.


## Reviews

As a reviewer always keep in mind the following principles

- Reviewing code is more valuable than writing code as it results in higher overall project activity. If you find you can't write code any more due to prioritizing reviews over coding, let's talk.
- You should respond to a review request within one working day of getting it, either with a review, a deadline by which you promise to do the review, or a polite refusal. If you think a patch is lower priority than your other work communicate that.

### Review Guidelines

- Check that the issue is assigned and linked.
- Commit title and message make sense and says what is being changed.
- Check that the PR applies cleanly on the target branch.
- Check new files for license and administrative issues.
- Check out code changes
  - Run automated tests
  - Manually verify changes if possible
- Code review
  - Does the change address the issue at hand?
  - Is the code well documented?
  - Do you understand the code changes?
    - If not, add a comment. The PR can't be accepted in this stage.
  - Is the public API changed?
    - Are the changes well documented for consumers?
    - Do the changes break backwards compatibility?
    - Is the new API sensible/needed?
  - Is the code maintainable after these changes?
  - Are there any security issues with these changes?
  - Are all code changes tested?
  - Do the changes effect performance?
  - Look at the interdiff for second and subsequent reviews.
- Ask if more information is needed to understand and judge the changes.

## AI guidelines

AI tools may be used to generate any contribution to hax (code, tests, documentation, commit messages, or issue text) under the conditions below.

### Disclosure

- The PR must clearly state that AI was used and for which parts of the contribution.
- The author must explain the methodology: how AI was used and how the result was tested.

### Responsibility

- A human must take ownership of the PR: review every AI-generated part carefully before requesting review, and be able to explain and debug it. Autonomous agents may draft or open changes, but a person takes responsibility for them, and reviewers may ask you to walk through any part of your patch.
- You remain fully responsible for AI-generated content as if you had written it yourself, including ensuring it does not infringe third-party rights and that the AI tool's terms permit contributing the output under the project's license.

### Commit trailers

- `Co-authored-by:` must not be used for AI tools. That trailer implies a human collaborator who can hold copyright and be held accountable. AI tools cannot.
- `Assisted-by:` is optional but encouraged. Use it to note which AI tool was used, e.g. `Assisted-by: Claude Fable 5` (include the tool name and version). This is a project-specific convention that Git and most tooling will not recognize, and it does not replace the disclosure required above.
