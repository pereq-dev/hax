# The extracted `lib.pvl` has no `process`: `analysis.pv` supplies it, after
# the shipped libraries and `missingdecl.pvl` (symbols the extraction
# references but does not define).
PROOF_LIB := ../../hax-lib/proof-libs/proverif
EXTRACTION := ./proofs/psk/proverif/extraction

# The verdicts of `analysis.pv`'s queries, in order: both roles complete (a
# reachability query is `false` when the event is reachable), and neither the
# PSK nor the payload leaks. ProVerif exits 0 whatever the verdicts, so they
# are compared here. An incomplete extraction adds a failing query of its own.
EXPECTED := false false true true

check: $(EXTRACTION)/lib.pvl
	@out=$$(timeout 30 proverif \
	  -lib $(PROOF_LIB)/primitives \
	  -lib $(PROOF_LIB)/result \
	  -lib $(PROOF_LIB)/cryptolib \
	  -lib $(EXTRACTION)/missingdecl \
	  -lib $(EXTRACTION)/lib \
	  ./proofs/psk/proverif/analysis.pv) || { printf '%s\n' "$$out"; exit 1; }; \
	printf '%s\n' "$$out" | grep '^RESULT'; \
	got=$$(printf '%s\n' "$$out" | grep '^RESULT' | grep -oE 'is (true|false)' | cut -d' ' -f2 | xargs); \
	[ "$$got" = "$(EXPECTED)" ] || { echo "expected verdicts: $(EXPECTED)"; echo "got:               $$got"; exit 1; }

$(EXTRACTION)/lib.pvl:
	cargo hax extract psk

clean:
	rm -f $(EXTRACTION)/lib.pvl $(EXTRACTION)/missingdecl.pvl

.PHONY: check clean
