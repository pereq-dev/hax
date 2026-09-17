# Orchestration Makefile for the two test surfaces under `tests/`.
# Each subdirectory has its own self-contained Makefile that drives a
# charon → Aeneas → Lake pipeline against its own crate.

.PHONY: all client-test equiv-test clean

all: client-test equiv-test

# Smoke test: extract the items a downstream client uses and check the
# result elaborates against our `CoreModels` shims.
client-test:
	$(MAKE) -C client_test

# Rust↔Lean equivalence test: cargo runs the `#[hax_test::equiv]`
# functions on the Rust side; Lake evaluates the same functions via the
# Aeneas extraction and the `#guard`s emitted by `gen_lean_tests.py`.
equiv-test:
	$(MAKE) -C rust_lean_equiv_test

clean:
	$(MAKE) -C client_test clean
	$(MAKE) -C rust_lean_equiv_test clean
