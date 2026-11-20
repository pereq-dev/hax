# Type-checks the committed F* snapshots: no hax, no engine, nothing
# regenerated. One `--dep full` over every root, so the `proof-libs` closure is
# one set of nodes in one graph, checked once, rather than once per snapshot.

HAX_ROOT  ?= $(shell git rev-parse --show-toplevel)
SNAPSHOTS := $(HAX_ROOT)/tests/snapshots

# `--lax` makes F* write `<module>.checked.lax`, not `<module>.checked`, hence
# a cache and a target name per mode, and a `.depend` generated with
# `$(OTHERFLAGS)`: make's targets have to be the files F* writes, or dependents
# start before their dependencies exist and concurrent runs tear the cache.
CHECKED   = $(if $(findstring --lax,$(OTHERFLAGS)),.checked.lax,.checked)
CACHE_ROOT ?= $(HAX_ROOT)/.fstar-cache/verify
CACHE_DIR  ?= $(CACHE_ROOT)/$(if $(findstring --lax,$(OTHERFLAGS)),lax,full)

# `.depend` belongs to one mode and one set of roots: `--dep full` emits rules
# only for the roots it was given, under the names the mode writes. Keeping it
# in $(CACHE_DIR) stops a mode switch from reading the other mode's rules, and
# $(SOURCES_STAMP) rebuilds it when that set changes -- otherwise a narrower
# earlier run leaves targets with no prerequisite, which F* reports as
# `Error 5: No file provided`.
DEPEND        = $(CACHE_DIR)/depend.mk
SOURCES_STAMP = $(CACHE_DIR)/sources

FSTAR_BIN_DETECT := $(if $(shell command -v fstar.exe),fstar.exe,$(FSTAR_HOME)/bin/fstar.exe)
FSTAR_BIN ?= $(FSTAR_BIN_DETECT)

# The snapshots no prover is expected to accept live in `<test>/fstar-xfail/`,
# so matching `*/fstar/*` is the whole selection. `MATCHING` narrows the run
# locally: `just verify-fstar legacy/tuples`.
MATCHING ?=
ROOTS := $(shell find $(SNAPSHOTS) -path '*/fstar/*.fst' -o -path '*/fstar/*.fsti' \
           | grep -e '$(MATCHING)' | sort)
DIRS  := $(sort $(dir $(ROOTS)))

# An empty selection would make every goal below succeed without checking
# anything, so refuse it. `clean` and `describe` still work, to debug just that.
ifeq ($(strip $(ROOTS)),)
ifeq ($(filter clean describe,$(MAKECMDGOALS)),)
$(error no F* snapshot under $(SNAPSHOTS) matches '$(MATCHING)')
endif
endif

LIB_DIRS := $(HAX_ROOT)/hax-lib/proof-libs/fstar/core \
  $(HAX_ROOT)/hax-lib/proof-libs/fstar/rust_primitives \
  $(HAX_ROOT)/hax-lib/proofs/fstar/extraction
LIB_ROOTS := $(shell find $(LIB_DIRS) -name '*.fst' -o -name '*.fsti' | sort)

INCLUDE_DIRS := $(DIRS) $(LIB_DIRS)

# `--lax`, as `test-driver` has always done here: checked, not proved.
# `OTHERFLAGS=` gives a full run.
OTHERFLAGS ?= --lax

FSTAR_FLAGS = \
  --warn_error -321-331-241-274-239-271 \
  --ext context_pruning --z3version 4.13.3 \
  --cache_checked_modules --cache_dir $(CACHE_DIR) \
  --already_cached "+Prims+FStar+LowStar+C+TestLib" \
  $(addprefix --include ,$(INCLUDE_DIRS))

FSTAR := $(FSTAR_BIN) $(FSTAR_FLAGS)

# Set explicitly: the default goal is otherwise whichever target appears first,
# which a later edit can silently change -- adding `FORCE` below once made
# `make` do nothing at all and report success.
.DEFAULT_GOAL := all
.DELETE_ON_ERROR:

.PHONY: all verify unique-modules describe clean FORCE

all: unique-modules
	$(Q)$(MAKE) -f $(firstword $(MAKEFILE_LIST)) verify

# One cache and one include path make module names a global namespace, shared
# with the proof libraries, where a collision would check one and silently skip
# the other. Compared on
# the module name, not the file name, so that `Foo.fst` in one snapshot and
# `Foo.fsti` in another do not slip through; the two together in one snapshot
# are an ordinary interface pair, hence the pairing with the directory.
unique-modules:
	$(Q)dupes=$$(for root in $(ROOTS) $(LIB_ROOTS); do \
	  printf '%s %s\n' "$$(basename $$root | sed 's/\.fsti\{0,1\}$$//')" "$$(dirname $$root)"; \
	done | sort -u | awk '{print $$1}' | uniq -d); \
	if [ -n "$$dupes" ]; then \
	  echo "error: several snapshots extract the same F* module:"; \
	  echo "$$dupes" | sed 's/^/  /'; \
	  exit 1; \
	fi

$(CACHE_DIR):
	$(Q)mkdir -p $@

# Rewritten only when the selected set changes, so that `.depend` is rebuilt
# then rather than on every run.
FORCE:

$(SOURCES_STAMP): FORCE | $(CACHE_DIR)
	$(Q)printf '%s\n' $(ROOTS) $(LIB_ROOTS) | cmp -s - $@ 2> /dev/null \
	  || printf '%s\n' $(ROOTS) $(LIB_ROOTS) > $@

$(DEPEND): $(ROOTS) $(LIB_ROOTS) $(SOURCES_STAMP) | $(CACHE_DIR)
	$(Q)$(FSTAR) $(OTHERFLAGS) --dep full $(ROOTS) --extract '* -Prims -LowStar -FStar' > $@

# Regenerating $(DEPEND) runs F*, which `clean` and `describe` must not need.
ifneq ($(if $(MAKECMDGOALS),$(filter-out clean describe,$(MAKECMDGOALS)),all),)
include $(DEPEND)
endif

$(CACHE_DIR)/%$(CHECKED): | $(DEPEND) $(CACHE_DIR)
	$(Q)printf '[CHECK] %s\n' "$(basename $(notdir $*))"
	$(Q)$(FSTAR) $(OTHERFLAGS) $<

verify: $(addsuffix $(CHECKED),$(addprefix $(CACHE_DIR)/,$(notdir $(ROOTS))))

describe:
	@echo "roots:     $(words $(ROOTS))"
	@echo "snapshots: $(words $(DIRS))"
	@echo "fstar:     $(FSTAR_BIN)"
	@echo "cache:     $(CACHE_DIR)"
	@echo "flags:     $(OTHERFLAGS)"

clean:
	rm -rf $(CACHE_ROOT) $(CACHE_DIR)

Q ?= @
