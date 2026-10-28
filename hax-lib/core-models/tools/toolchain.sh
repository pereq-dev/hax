#!/usr/bin/env bash
# Print the Rust toolchain the models must build under:
#   hax     the one hax extracts with (the repository's `rust-toolchain.toml`)
#   charon  the one of the charon version hax pins by default, which the Lean
#           extraction goes through
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"

channel() { sed -n 's/^channel *= *"\(.*\)"$/\1/p'; }

case "${1:-}" in
  hax)
    toolchain="$(channel < "$ROOT/rust-toolchain.toml")"
    ;;
  charon)
    # Only the `[tools]` table: another table may have a `charon` key too.
    version="$(sed -n '/^\[tools\]/,/^\[/s/^charon *= *"\(.*\)"$/\1/p' "$ROOT/cli/cargo-hax/defaults.toml")"
    toolchain="$(curl -sSfL --retry 5 --retry-all-errors \
      "https://raw.githubusercontent.com/AeneasVerif/charon/$version/charon/rust-toolchain" \
      | channel)"
    ;;
  *)
    echo "usage: $0 hax|charon" >&2
    exit 2
    ;;
esac

[[ -n "$toolchain" ]] || { echo "error: could not determine the $1 toolchain" >&2; exit 1; }
echo "$toolchain"
