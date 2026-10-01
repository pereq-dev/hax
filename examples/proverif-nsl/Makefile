# The extracted `lib.pvl` has no `process`: `analysis.pv` supplies it, after
# the shipped libraries, the model of the public-key encryption
# (`nsl_crypto.pvl`) and `missingdecl.pvl` (symbols the extraction references
# but does not define).
PROOF_LIB := ../../hax-lib/proof-libs/proverif
EXTRACTION := ./proofs/nsl/proverif/extraction

# The verdicts of `analysis.pv`'s queries, in order: both roles complete (a
# reachability query is `false` when the event is reachable), each party
# authenticates the other, and the session keys of the honest parties stay
# secret. ProVerif exits 0 whatever the verdicts, so they are compared here. An
# incomplete extraction adds a failing query of its own.
EXPECTED := false false true true true true

check: $(EXTRACTION)/missingdecl.model.pvl
	@out=$$(timeout 60 proverif \
	  -lib $(PROOF_LIB)/primitives \
	  -lib $(PROOF_LIB)/result \
	  -lib $(PROOF_LIB)/cryptolib \
	  -lib ./nsl_crypto \
	  -lib $(EXTRACTION)/missingdecl.model \
	  -lib $(EXTRACTION)/lib \
	  ./proofs/nsl/proverif/analysis.pv) || { printf '%s\n' "$$out"; exit 1; }; \
	printf '%s\n' "$$out" | grep '^RESULT'; \
	got=$$(printf '%s\n' "$$out" | grep '^RESULT' | grep -oE 'is (true|false)' | cut -d' ' -f2 | xargs); \
	[ "$$got" = "$(EXPECTED)" ] || { echo "expected verdicts: $(EXPECTED)"; echo "got:               $$got"; exit 1; }

# `missingdecl.pvl` declares the trait items that `nsl_crypto.pvl` defines.
$(EXTRACTION)/missingdecl.model.pvl: $(EXTRACTION)/lib.pvl
	grep -v '^fun proverif_nsl__PublicKeyEncryption__' $(EXTRACTION)/missingdecl.pvl > $@

$(EXTRACTION)/lib.pvl:
	cargo hax extract nsl

clean:
	rm -f $(EXTRACTION)/lib.pvl $(EXTRACTION)/missingdecl.pvl $(EXTRACTION)/missingdecl.model.pvl

.PHONY: check clean
