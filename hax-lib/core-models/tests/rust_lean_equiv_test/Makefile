# Rust↔Lean equivalence test

HAX ?= cargo hax
LEAN_DIR = source/proofs/lean
EXTRACTION_DIR = $(LEAN_DIR)/RustLeanTests/Extraction

.PHONY: all cargo-test extract gen-lean-tests build check-skipped clean

all: cargo-test build

# Rust-side half: every `#[hax_test::equiv]` generates a `#[cfg(test)]
# #[test]` wrapper that asserts the function returns `true`. The Lean-side
# half is the `#guard`s built below.
cargo-test:
	cd source && cargo test --lib

# 1. Extract Rust -> Lean with hax. Output
#    lands in `source/proofs/lean/RustLeanTests/Extraction/`.
extract:
	cd source && $(HAX) into lean

# 2. Generate `RustLeanTests/LeanTests.lean` — one `#guard <name> == .ok
#    true` per `#[hax_test::equiv]` function. Depends on `extract` so the gen
#    script can look up each test's fully-qualified Lean name in `Funs.lean`.
gen-lean-tests: extract
	python3 gen_lean_tests.py

# 3. Build the Lean library — this is where the `#guard`s fire.
build: gen-lean-tests
	cd $(LEAN_DIR) && lake exe cache get && lake build

# 4. `SkippedTests.lean` is outside the import graph, so `build` ignores it.
#    Here we elaborate it on purpose: every guard is expected to FAIL, and one
#    that passes means its blocker is gone. Hence `-` — success is the failure
#    case.
check-skipped: gen-lean-tests
	-cd $(LEAN_DIR) && lake env lean RustLeanTests/SkippedTests.lean > $(abspath skipped.log) 2>&1
	python3 gen_lean_tests.py --report-skipped skipped.log

# Remove the generated files, including the generated `LeanTests.lean`;
# keep the hand-written project files.
clean:
	rm -rf $(EXTRACTION_DIR)
	rm -f $(EXTRACTION_DIR).lean \
	      $(LEAN_DIR)/RustLeanTests/LeanTests.lean \
	      $(LEAN_DIR)/RustLeanTests/SkippedTests.lean \
	      skipped.log
	rm -rf $(LEAN_DIR)/llbc $(LEAN_DIR)/.lake
	rm -f $(LEAN_DIR)/.gitignore
