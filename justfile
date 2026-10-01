@_default:
  just --list

# Build Rust and OCaml parts and install binaries in PATH. To build
# only OCaml parts or only Rust parts, set target to `rust` or
# `ocaml`.
@build target='rust+ocaml':
  ./.utils/rebuild.sh {{target}}

alias b := build

# alias for `build rust`
@rust:
  just build rust

# alias for `build ocaml`
@ocaml:
  just build ocaml

# `cargo expand` a crate, but sets flags and crate attributes so that the expansion is exactly what hax receives. This is useful to debug hax macros.
[no-cd]
expand *FLAGS:
  RUSTFLAGS='-Zcrate-attr=register_tool(_hax) -Zcrate-attr=feature(register_tool) --cfg hax_compilation --cfg _hax --cfg hax --cfg hax_backend_fstar --cfg hax' \
    cargo \
    $([[ "$(cargo --version)" == *nightly* ]] || echo "+nigthly") \
    expand {{FLAGS}}

# Show debug JSON emitted by the Rust engine
@debug-json N: (_ensure_command_in_path "jless" "jless (https://jless.io/)") (_ensure_command_in_path "jq" "jq (https://jqlang.github.io/jq/)")
  cat /tmp/hax-ast-debug.json | jq -s '.[{{N}}]' | jless

# Show the generated module `concrete_ident_generated.ml`, that contains all the Rust names the engine knows about. Those names are declared in the `./engine/names` crate.
@list-names:
  hax-engine-names-extract | sed '/include .val/,$d' | just _pager

# Show the Rust to OCaml generated types available to the engine.
@list-types:
  just _ensure_command_in_path ocamlformat ocamlformat
  cd engine && dune describe pp lib/types.ml \
    | sed -e '1,/open ParseError/ d' \
    | sed '/let rec pp_/,$d' \
    | ocamlformat --impl - \
    | just _pager

# Show the OCaml module `Generated_generic_printer_base`
@show-generated-printer-ml:
  just _ensure_command_in_path ocamlformat ocamlformat
  cd engine && dune describe pp lib/generated_generic_printer_base.ml \
    | ocamlformat --impl - \
    | just _pager

# Run core models tests
core-models-test:
  cargo test --manifest-path hax-lib/core-models/Cargo.toml --workspace

# Regenerate names in the Rust engine. Writes to `rust-engine/src/names/generated.rs`.
regenerate-names:
  #!/usr/bin/env bash
  OUTPUT_FILE=rust-engine/src/ast/identifiers/global_id/generated.rs
  cargo hax -C --manifest-path engine/names/Cargo.toml \; into --output-dir $(dirname -- $OUTPUT_FILE) generate-rust-engine-names
  rustfmt "$OUTPUT_FILE"

# Format all the code
fmt:
  cargo fmt
  cd engine && dune fmt

# Remove build artifacts and verification caches. A failing step (e.g. a
# missing tool) does not skip the remaining ones.
clean:
  -cargo clean
  -cargo clean --manifest-path examples/Cargo.toml
  -cargo clean --manifest-path hax-lib/core-models/Cargo.toml
  -cargo clean --manifest-path hax-lib/core-models/tests/Cargo.toml
  -cargo clean --manifest-path tests/Cargo.toml
  -cd engine && dune clean
  rm -rf .hax-tests-cache .fstar-cache .mypy_cache .ruff_cache \
    hax-lib/proof-libs/legacy-lean/.lake
  -make -C tests/verify/fstar clean
  -make -C tests/verify/lean clean
  -make -C hax-lib/core-models clean tests-clean
  -make -C hax-lib/proof-libs/fstar/core clean
  -make -C hax-lib/proof-libs/fstar/rust_primitives clean
  -make -C hax-lib/proof-libs/fstar/hax_lib clean
  -for makefile in examples/*/Makefile; do make -C "$(dirname "$makefile")" clean; done

# Type-check the committed F* snapshots. The optional argument narrows to
# snapshots whose path matches, e.g. `just verify-fstar legacy/tuples`.
verify-fstar MATCHING='':
  #!/usr/bin/env bash
  set -euo pipefail
  # `--keep-going` reports every failing snapshot, not just the first.
  make -C tests/verify/fstar -Otarget -j "$(nproc)" --keep-going MATCHING='{{MATCHING}}'

# Type-check the committed Lean snapshots. The optional argument narrows to
# snapshots whose path matches, e.g. `just verify-lean legacy/tuples`.
verify-lean MATCHING='':
  #!/usr/bin/env bash
  set -euo pipefail
  make -C tests/verify/lean -Otarget -j "$(nproc)" --keep-going MATCHING='{{MATCHING}}'

# Parse-check the committed ProVerif snapshots. The optional argument narrows to
# snapshots whose path matches, e.g. `just verify-proverif legacy/tuples`.
verify-proverif MATCHING='':
  #!/usr/bin/env bash
  set -euo pipefail
  make -C tests/verify/proverif -Otarget -j "$(nproc)" --keep-going MATCHING='{{MATCHING}}'

# Run hax tests
test *FLAGS:
  cargo run --release --bin test-driver -- ./tests {{FLAGS}}

# Check the tool version manifest against the artifacts it names, downloading the *default* versions only. Reaches the network.
test-tools-manifest:
  cargo test -p cargo-hax --bin cargo-hax -- --ignored manifest_artifacts \
    --skip every_listed_artifact_verifies

# Check the tool version manifest against the artifacts it names, downloading *every* listed version. Reaches the network.
test-tools-manifest-all:
  cargo test -p cargo-hax --bin cargo-hax -- --ignored every_listed_artifact_verifies

# Print the `tools-manifest.toml` entries for one version of a managed tool, e.g. `just add-tool-version aeneas nightly-2026.07.21-52fd438`. Each artifact is downloaded, hashed, and put through the checks a listed entry has to pass. Reaches the network.
add-tool-version tool version:
  #!/usr/bin/env bash
  set -euo pipefail
  # Only the entries reach stdout, so the output can be appended to the tool's
  # section of the manifest as it is. Making the version a default is a
  # separate edit to `defaults.toml`.
  entries=$(mktemp)
  trap 'rm -f "$entries"' EXIT
  HAX_ADD_TOOL_VERSION="{{tool}}@{{version}}" HAX_ADD_TOOL_VERSION_OUT="$entries" \
    cargo test -q -p cargo-hax --bin cargo-hax -- \
      --ignored --exact --nocapture tools::install::add_version::add_version >&2
  # A filter matching no test is not a test failure, so an empty result is.
  if [ ! -s "$entries" ]; then
    echo "no entries were generated" >&2
    exit 1
  fi
  cat "$entries"

# Install the managed tools from their real artifacts and run them. Reaches the network, and installs into the tool cache.
test-tools-install:
  cargo test -p cargo-hax --bin cargo-hax -- --ignored host_install

# Extract an example to Lean with the managed tools: a full charon and aeneas run, beyond the launch checks of `test-tools-install`. Reaches the network, and installs into the tool cache.
test-tools-extract:
  #!/usr/bin/env bash
  set -euo pipefail
  cargo build -q -p cargo-hax --bin cargo-hax
  # `examples/` is its own workspace, so the binary is invoked by path.
  HAX="$PWD/target/debug/cargo-hax"
  cd examples/barrett
  "$HAX" into lean

# Walk the documented tool setup flow from inside an example project, the way a user would: resolution, the `hax-lib` check, and the install of what the project resolves to. Reaches the network.
test-tools-cli:
  #!/usr/bin/env bash
  set -euo pipefail
  cargo build -q -p cargo-hax --bin cargo-hax
  # `examples/` is its own workspace, so the binary is invoked by path.
  HAX="$PWD/target/debug/cargo-hax"
  cd examples/barrett
  "$HAX" tools show
  "$HAX" tools install
  "$HAX" tools list --installed

# Serve documentation
docs: (_ensure_command_in_path "mkdocs" "mkdocs (https://www.mkdocs.org/)")
  mkdocs serve

# List the names of every example
list-examples:
  #!/usr/bin/env bash
  set -euo pipefail
  shopt -s nullglob
  makefiles=(examples/*/Makefile)
  [ ${#makefiles[@]} -gt 0 ] || { >&2 echo "no examples found"; exit 1; }
  for makefile in "${makefiles[@]}"; do
    basename "$(dirname "$makefile")"
  done

# Extract and verify one example, from scratch (e.g. `just check-example barrett`)
check-example NAME:
  make -C "examples/{{NAME}}" clean
  make -C "examples/{{NAME}}"

# Extract and verify every example, from scratch
check-examples:
  #!/usr/bin/env bash
  set -euo pipefail
  examples=$({{just_executable()}} list-examples)
  while IFS= read -r example; do
    {{just_executable()}} check-example "$example"
  done <<< "$examples"

# Check the coherency between issues labeled `marked-unimplemented` on GitHub and issues mentionned in the engine in the `Unimplemented {issue_id: ...}` errors.
@check-issues:
  just _ensure_command_in_path jq "jq (https://jqlang.github.io/jq/)"
  just _ensure_command_in_path gh "GitHub CLI (https://cli.github.com/)"
  just _ensure_command_in_path rg "ripgrep (https://github.com/BurntSushi/ripgrep)"
  just _ensure_command_in_path sd "sd (https://github.com/chmln/sd)"
  diff -U0 \
      <(gh issue -R pereq-dev/hax list --label 'marked-unimplemented' --json number,closed -L 200 \
           | jq '.[] | select(.closed | not) | .number' | sort -u) \
      <(rg 'issue_id:(\d+)' -Ior '$1' | sort -u) \
      | rg '^[+-]\d' \
      | sd '[-](\d+)' '#$1\t is labeled `marked-unimplemented`, but was not found in the code' \
      | sd '[+](\d+)' '#$1\t is *not* labeled `marked-unimplemented` or is closed'

# Check that the licenses of every crate and every package are compliant with `deny.toml`
check-licenses:
  #!/usr/bin/env bash
  just _ensure_command_in_path cargo-deny "cargo-deny (https://embarkstudios.github.io/cargo-deny/)"
  just _ensure_command_in_path toml2json "toml2json (https://github.com/woodruffw/toml2json)"
  echo "> Check licenses for Rust"
  cargo deny check licenses
  cd engine
  echo "> Check licenses for OCaml"
  # initialize opam if needed
  opam env >& /dev/null || opam init --no
  # pin package `hax-engine` if needed
  opam list --required-by=hax-engine --column=name,license: -s >& /dev/null || opam pin . --yes
  # Check that every pacakge matches licenses of `deny.toml`
  if opam list --required-by=hax-engine --column=name,license: -s \
     | grep -Pvi $(toml2json ../deny.toml| jq '.licenses.allow | join("|")'); then
     echo "Some licenses were non compliant to our policy (see `deny.toml`)"
  else
    echo "licenses ok"
  fi

_ensure_command_in_path BINARY NAME:
  #!/usr/bin/env bash
  command -v {{BINARY}} &> /dev/null || {
     >&2 echo -e "\033[0;31mSorry, the binary \033[1m{{BINARY}}\033[0m\033[0;31m is required for this command.\033[0m"
     >&2 echo -e "  \033[0;31m→ please install \033[1m{{NAME}}\033[0m"
     >&2 echo ""
     exit 1
  }

_pager:
  #!/usr/bin/env bash
  if command -v bat &> /dev/null; then
      bat -l ml
  else
      less
  fi

# Serve the book
[private]
@book:
  echo "We moved out from mdbook: please run 'just docs'"
  exit 1

# Runs hax twice: once with the Rust import thir, once with the OCaml one.
# Then it compares both.
diff-thir-importers DIR:
  #!/usr/bin/env bash
  # Ensures hax is built
  just b

  # Utils
  function readJSON() { cat proofs/debugger/extraction/ast.json; }
  BASE="$PWD"
  OUT="$BASE/diff-thir-importers"
  # Remove previous results (if any)
  rm -rf "$OUT"

  cd {{DIR}}
  cargo hax json -o old-thir.json
  cargo hax --experimental-full-def json -o thir.json
  cargo hax --experimental-full-def into debugger
  readJSON > rust-import-thir-ast.json
  cargo hax                         into debugger
  readJSON > ocaml-import-thir-ast.json

  mkdir "$OUT"
  mv thir.json old-thir.json *ast.json "$OUT"
  cd "$OUT"
  diff ocaml-import-thir-ast.json rust-import-thir-ast.json > diff.json
