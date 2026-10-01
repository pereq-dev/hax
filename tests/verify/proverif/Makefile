# Parse-checks the committed ProVerif snapshots with `proverif -parse-only`,
# without running hax or an analysis.

HAX_ROOT  ?= $(shell git rev-parse --show-toplevel)
SNAPSHOTS := $(HAX_ROOT)/tests/snapshots
PROOF_LIB := $(HAX_ROOT)/hax-lib/proof-libs/proverif

PROVERIF ?= proverif

CACHE_DIR ?= $(CURDIR)/.cache

# A model declares no `process`, which ProVerif requires.
MAIN := $(CACHE_DIR)/main.pv

# The snapshots ProVerif is not expected to accept live in
# `<test>/proverif-xfail/`, so matching `*/proverif/*` is the whole selection.
# `MATCHING` narrows the run locally: `just verify-proverif legacy/tuples`.
MATCHING ?=
MODELS := $(shell find $(SNAPSHOTS) -path '*/proverif/lib.pvl' | grep -e '$(MATCHING)' | sort)
CHECKS := $(patsubst $(SNAPSHOTS)/%/lib.pvl,check/%,$(MODELS))

# An empty selection would make every goal below succeed without checking
# anything, so refuse it. `clean` and `describe` still work, to debug just that.
ifeq ($(strip $(MODELS)),)
ifeq ($(filter clean describe,$(MAKECMDGOALS)),)
$(error no ProVerif snapshot under $(SNAPSHOTS) matches '$(MATCHING)')
endif
endif

# Set explicitly: the default goal is otherwise whichever target appears first,
# which a later edit can silently change.
.DEFAULT_GOAL := all
.DELETE_ON_ERROR:

.PHONY: all verify describe clean $(CHECKS)

all verify: $(CHECKS)

$(MAIN):
	$(Q)mkdir -p $(dir $@)
	$(Q)printf 'process\n  0\n' > $@

# A model extracted with `--result-errors` names a constant that only
# `result_errors.pvl` declares. ProVerif warns copiously on models it accepts,
# so its output is reported only when it rejects one.
$(CHECKS): check/%: | $(MAIN)
	$(Q)printf '[PV] %s\n' "$(patsubst %/proverif,%,$*)"
	$(Q)out=$$($(PROVERIF) -parse-only \
	  -lib $(PROOF_LIB)/primitives \
	  -lib $(PROOF_LIB)/$$(grep -q hax_result_model__result_errors $(SNAPSHOTS)/$*/lib.pvl && echo result_errors || echo result) \
	  -lib $(PROOF_LIB)/cryptolib \
	  $(if $(wildcard $(SNAPSHOTS)/$*/missingdecl.pvl),-lib $(SNAPSHOTS)/$*/missingdecl) \
	  -lib $(SNAPSHOTS)/$*/lib $(MAIN) 2>&1) \
	  || { printf '%s\n' "$$out"; exit 1; }

describe:
	@echo "snapshots: $(words $(MODELS))"
	@echo "proverif:  $(PROVERIF)"

clean:
	rm -rf $(CACHE_DIR)

Q ?= @
