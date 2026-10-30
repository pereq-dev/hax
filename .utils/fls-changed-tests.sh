#!/usr/bin/env bash
# Selects the FLS tests affected by the changes between a base commit and HEAD
# (or a second commit).
#
#   .utils/fls-changed-tests.sh <base> [<head>]
#
# Prints one of:
#   all     a file the whole suite depends on changed, or the tool versions
#           of a results file changed; run without --matching
#   none    no test file and no test entry of a results file changed
#   some    followed by one `-m <chapter>/<section>/<name>` line per test whose
#           file was added, modified, or renamed, or whose results entry was
#           added or changed, to be passed to the driver
set -euo pipefail

base="${1:?usage: $0 <base-commit> [<head-commit>]}"
head="${2:-HEAD}"
range="$base...$head"
tests_dir='tests/fls/src/'

# Paths outside the test files whose change can move the observed stages.
full_run_paths=(
  'cli/'
  'hax-lib/'
  'hax-types/'
  'tests/hax-test-macros/'
  'Cargo.toml'
  'Cargo.lock'
  '.cargo/'
  'rust-toolchain.toml'
  'flake.nix'
  'flake.lock'
  '.github/workflows/fls_tests.yml'
  '.utils/fls-changed-tests.sh'
)
# Paths under `full_run_paths` that the Lean pipeline never reads.
exempt_paths=(
  'hax-lib/core-models/'
  'hax-lib/proof-libs/coq/'
  'hax-lib/proof-libs/fstar/'
  'hax-lib/proof-libs/fstar-secret-integers/'
  'hax-lib/proof-libs/legacy-lean/'
  'hax-lib/proofs/fstar/'
)

# Without rename detection, a file moved out of a full-run path is listed
# under its old path too.
changed="$(git diff --name-only --no-renames "$range" --)"
while IFS= read -r file; do
  for path in "${exempt_paths[@]}"; do
    [[ "$file" == "$path"* ]] && continue 2
  done
  for path in "${full_run_paths[@]}"; do
    if [[ "$file" == "$path"* ]]; then
      echo all
      exit 0
    fi
  done
done <<<"$changed"

# A results file selects the tests whose entry was added or changed. A change
# outside the entries changes the tool versions and is verified on every test.
merge_base="$(git merge-base "$base" "$head")"
# The two versions go through files: a results file outgrows the size limit
# of a single command-line argument.
old_file="$(mktemp)"
new_file="$(mktemp)"
trap 'rm -f "$old_file" "$new_file"' EXIT
compare_results() {
  python3 -I -c '
import sys, tomllib
old, new = (tomllib.load(open(path, "rb")) for path in (sys.argv[1], sys.argv[2]))
def header(results):
    return {k: v for k, v in results.items() if k != "tests"}
if header(old) != header(new):
    print("all")
else:
    old_tests = old.get("tests", {})
    for path, entry in new.get("tests", {}).items():
        if old_tests.get(path) != entry:
            print(path)
' "$1" "$2"
}
results_selected=""
for file in $(grep -- '^tests/fls-results/.*\.toml$' <<<"$changed" || true); do
  git show "$merge_base:$file" >"$old_file" 2>/dev/null || : >"$old_file"
  git show "$head:$file" >"$new_file" 2>/dev/null || : >"$new_file"
  compared="$(compare_results "$old_file" "$new_file")"
  if [ "$compared" = all ]; then
    echo all
    exit 0
  fi
  results_selected+="$compared"$'\n'
done

# A test file `<chapter>/<section>/<name>.rs` selects itself.
selected="$({
  git diff --name-only --diff-filter=AMR -M "$range" -- "$tests_dir" \
    | sed -nE "s#^${tests_dir}([^/]+/[^/]+/[^/]+)\.rs\$#\1#p"
  printf '%s' "$results_selected"
} | grep -v '^$' | sort -u || true)"
if [ -z "$selected" ]; then
  echo none
  exit 0
fi
echo some
while IFS= read -r test; do
  echo "-m $test"
done <<<"$selected"
