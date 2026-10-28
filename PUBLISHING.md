# Publishing

## OCaml

The OCaml engine is a single package, `hax-engine`, with a binary and a number of libraries. It is not published to opam and has no release step of its own.

## Rust

All published crates start with the `hax-` prefix, except `cargo-hax` (the entrypoint to the cargo `hax` subcommand). The published crates, in dependency order:

**cargo-hax**

1. `hax-frontend-exporter-options` (`frontend/exporter/options`)
2. `hax-adt-into` (`frontend/exporter/adt-into`)
3. `hax-frontend-exporter` (`frontend/exporter`)
4. `hax-types` (`hax-types`)
5. `cargo-hax` (`cli/cargo-hax`)
6. `hax-driver` (`cli/driver`)

**hax-lib**

1. `hax-lib-macros-types` (`hax-lib/macros/types`)
2. `hax-lib-macros` (`hax-lib/macros`)
3. `hax-lib` (`hax-lib`)
4. `hax-bounded-integers` (`hax-bounded-integers`)

`cargo-hax` accepts only the `hax-lib` of its own version, so every `cargo-hax` release must publish a matching `hax-lib`, even for changes that only touch the binary.

`hax-adt-into` is not listed as a workspace member in the root `Cargo.toml`; it is part of the workspace anyway, as a path dependency of `hax-frontend-exporter`, and is released with it.

**Rust engine**

1. `hax-rust-engine-macros` (`rust-engine/macros`)
2. `hax-rust-engine` (`rust-engine`)

Non-published crates set two flags in their `Cargo.toml`: `publish = false` makes cargo itself refuse to publish the crate, and `package.metadata.release.release = false` keeps it out of every `cargo release` step. These are `hax-engine-names` and `hax-engine-names-extract` (used only by the OCaml build of the engine), `hax-lib-protocol` and `hax-lib-protocol-macros`, and `test-driver` (runs only the repository's test suite).

The directories the root `Cargo.toml` excludes from the workspace (currently `tests` and `hax-lib/core-models`) need no flags: `cargo release --workspace` never touches them. The `hax-lib/core-models` workspace is versioned independently of the hax version and is not published.

## Procedure

1. Start the `Release PR` workflow with the bump level: `gh workflow run release_pr.yml -f level=patch` (or `minor`/`major`; add `-f rc=true` for a release candidate), or the "Run workflow" button under Actions.
2. Open the version-bump PR through the prefilled link in the workflow's run summary.
3. Work through the pre-merge checklist in the PR's description, then review and merge the PR.
4. Have a maintainer other than yourself approve, under Actions, the `publish` run that the merge queued.

The rest runs unattended: the crates are published, the release tags pushed, the `cargo-hax` binaries attached to the GitHub release, and the released version installed with `cargo binstall` as a check. A failed release run files an issue with recovery instructions.

## Details

### The version bump

The `Release PR` workflow runs the version bump on a runner and pushes it as a `release/vX.Y.Z` branch. The PR is opened by you rather than by the workflow so that its CI checks run (a PR opened with the workflow token raises no events). The bump is `cargo release LEVEL --workspace --no-publish --no-tag --no-push --execute`: it bumps the version of every crate of the workspace and the versions in `engine/dune-project` and `engine/hax-engine.opam`, and renames the `[Unreleased]` section of `CHANGELOG.md` to the target version. It does not publish anything.

### Release candidates

With `rc=true` the workflow targets a release candidate of the bumped version: `0.4.0-rc.1` for `minor` from `0.3.x`, or the next candidate when the current version already is one. On a pre-release version the changelog is left untouched, so the `[Unreleased]` section stays in place for the eventual stable release. A `patch` bump without `rc` finalizes a candidate: `0.4.0-rc.1` becomes `0.4.0`, and the changelog rotates then.

### The publish run

The merge starts the `publish` workflow on `main`, pinned to the PR's merge commit: neither commits that land on `main` while the run waits for approval nor a stale release PR merged out of order change what a queued release publishes. A version bump that reached `main` some other way, or a run that failed partway, can be dispatched by hand: `gh workflow run publish.yml -f sha=<commit>`, where the commit is the one carrying the bump (for a rerun, the same one as before, so the tags land on the commit the crates were published from). It publishes every releasable crate at its current version, pushes the release tags, and starts the `release` workflow on the `cargo-hax-v*` tag. A repeated run skips crates that are published already, which also completes a run whose 30-minute trusted-publishing token expired mid-publish.

### The release run

The `release` workflow attaches a `cargo-hax` archive per platform to the GitHub release it creates at the `cargo-hax-v*` tag. `cargo binstall cargo-hax` downloads those, at the names `package.metadata.binstall` in `cli/cargo-hax/Cargo.toml` declares, so both the archives and the published manifest have to be in place for a version to be binstallable. The `binstall` workflow verifies that pairing at the end of every `release` run; a manual dispatch re-checks a released version at any time. A failed run files an issue; it, or a run that never started, can be restarted with `gh workflow run release.yml --ref cargo-hax-vX.Y.Z`. The tag must be the ref: a dispatch on any other ref is recognized as not being a release and skipped.

The website serves the manual of the latest stable GitHub release rather than of `main`, so a stable release also runs the `Deploy to GH Pages` workflow.

### Trusted publishing

The `publish` workflow authenticates with [trusted publishing](https://crates.io/docs/trusted-publishing): every published crate lists repository `pereq-dev/hax`, workflow `publish.yml` and environment `crates-io` as a trusted publisher in its crates.io settings. A crate's first version cannot be published that way: publish it with a token once, then add the trusted publisher.

The `crates-io` GitHub environment makes a release a two-person action: it requires an approval by a maintainer other than the dispatcher (required reviewers, with self-review prevented) and deploys only from `main`. A publish queued by a release PR merge is dispatched by the workflow token, so self-review prevention does not bind the merger there; the PR's own review is the second pair of eyes in that path. Since crates.io issues tokens exclusively to runs inside that environment, dispatching a modified workflow on another ref cannot publish either. Running the `cargo release` steps `publish` (with `--no-verify`), `tag` and `push`, each with `--workspace --execute`, on a `main` checkout remains equivalent to the workflow, given a crates.io token with publish rights. The one-shot `cargo release --workspace --execute` does not work here: it re-applies the pre-release replacements, which fail once the changelog is rotated.
