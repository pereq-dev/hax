#!/usr/bin/env bash
# Generate the core/alloc model coverage report.
#
# Builds rustdoc JSON for the model crates and compares against the prebuilt
# rustdoc JSON of the real std crates (the `rust-docs-json` rustup component),
# producing COVERAGE.md + tools/core-coverage/coverage.json at the repo root.
#
# Both must come from the SAME nightly so their rustdoc `format_version` agrees;
# override the toolchain with TOOLCHAIN=nightly-YYYY-MM-DD.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$REPO_ROOT"

# The toolchain hax extracts with, so the report measures the std API that
# client code is resolved against.
TOOLCHAIN="${TOOLCHAIN:-$(tools/toolchain.sh hax)}"

# How to get the toolchain + the std rustdoc JSON it ships, shown on any
# toolchain-related failure below.
install_hint() {
  echo "  install the matching nightly and its rustdoc JSON of std with:" >&2
  echo "      rustup toolchain install $TOOLCHAIN" >&2
  echo "      rustup component add rust-docs-json --toolchain $TOOLCHAIN" >&2
  echo "  (override with: TOOLCHAIN=nightly-YYYY-MM-DD make coverage)" >&2
}

command -v rustc >/dev/null || {
  echo "error: 'rustc' not found on PATH (install rustup: https://rustup.rs)." >&2
  exit 1
}

# Locate the prebuilt std rustdoc JSON (denominator).
SYSROOT="$(rustc "+$TOOLCHAIN" --print sysroot 2>/dev/null)" || {
  echo "error: Rust toolchain '$TOOLCHAIN' is not installed." >&2
  install_hint
  exit 1
}
JSON_DIR="$SYSROOT/share/doc/rust/json"
if [[ ! -f "$JSON_DIR/core.json" ]]; then
  echo "error: the 'rust-docs-json' component (rustdoc JSON of real core/alloc)" >&2
  echo "       is not installed for toolchain '$TOOLCHAIN'." >&2
  install_hint
  exit 1
fi

# Build rustdoc JSON for the model crates (numerator). `--document-private-items`
# because the model deliberately puts most items in private modules (only `vec`
# is `pub mod`), and charon/aeneas extract them regardless of visibility.
RUSTDOC_FLAGS="-Z unstable-options --output-format json --document-private-items"
echo "building model rustdoc JSON ($TOOLCHAIN)..."
build_model_json() {
  local crate="$1" log
  # Build hermetically: unset any ambient RUSTFLAGS / RUSTDOCFLAGS /
  # CARGO_ENCODED_RUSTFLAGS so a stray `--cfg` in the caller's environment (or a
  # CI runner) can't add/remove `#[cfg(...)]`-gated items and make the report
  # non-reproducible. The model's coverage is measured in its plain form.
  # `--output-format json` needs a nightly (`-Z unstable-options`); surface the
  # real cargo error instead of letting `set -e` abort silently.
  if ! log="$(cd "$crate" && env -u RUSTFLAGS -u RUSTDOCFLAGS -u CARGO_ENCODED_RUSTFLAGS \
              cargo "+$TOOLCHAIN" rustdoc --lib -- $RUSTDOC_FLAGS 2>&1)"; then
    echo "error: failed to build rustdoc JSON for '$crate' with '$TOOLCHAIN':" >&2
    echo "$log" | sed 's/^/    /' >&2
    echo "  ('$TOOLCHAIN' must be a nightly — '--output-format json' is unstable.)" >&2
    exit 1
  fi
}
build_model_json core-models
build_model_json alloc

python3 tools/core-coverage/coverage.py \
  --core-den   "$JSON_DIR/core.json"   --core-model  target/doc/core_models.json \
  --alloc-den  "$JSON_DIR/alloc.json"  --alloc-model target/doc/alloc.json \
  --toolchain  "$TOOLCHAIN" \
  --markdown   COVERAGE.md \
  --json       tools/core-coverage/coverage.json
