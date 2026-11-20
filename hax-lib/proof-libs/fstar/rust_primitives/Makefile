# This is a generically useful Makefile for F* that is self-contained
#
# It is tempting to factor this out into multiple Makefiles but that
# makes it less portable, so resist temptation, or move to a more
# sophisticated build system.
#
# We expect FSTAR_HOME to be set to your FSTAR repo/install directory
# We expect HAX_PROOF_LIBS_HOME to be set to the folder containing core, rust_primitives etc.
# We expect HAX_LIBS_HOME to be set to the hax-lib folder
#
# ROOTS contains all the top-level F* files you wish to verify
# The default target `verify` verified ROOTS and its dependencies
# To lax-check instead, set `OTHERFLAGS="--lax"` on the command-line
#
#
# To make F* emacs mode use the settings in this file, you need to
# add the following lines to your .emacs
#
# (setq-default fstar-executable "<YOUR_FSTAR_HOME>/bin/fstar.exe")
# (setq-default fstar-smt-executable "<YOUR_Z3_HOME>/bin/z3")
#
# (defun my-fstar-compute-prover-args-using-make ()
#   "Construct arguments to pass to F* by calling make."
#   (with-demoted-errors "Error when constructing arg string: %S"
#     (let* ((fname (file-name-nondirectory buffer-file-name))
# 	   (target (concat fname "-in"))
# 	   (argstr (car (process-lines "make" "--quiet" target))))
#       (split-string argstr))))
# (setq fstar-subp-prover-args #'my-fstar-compute-prover-args-using-make)
#

HAX_HOME      ?= $(shell git rev-parse --show-toplevel)
FSTAR_HOME    ?= $(HAX_LIBS_HOME)/../../../FStar
FSTAR_BIN     ?= $(shell command -v fstar.exe 1>&2 2> /dev/null && echo "fstar.exe" || echo "$(FSTAR_HOME)/bin/fstar.exe")

HAX_PROOF_LIBS_HOME ?= $(HAX_HOME)/proof-libs/fstar
HAX_LIBS_HOME       ?= $(HAX_HOME)/hax-lib

CACHE_DIR     ?= $(HAX_LIBS_HOME)/.cache
HINT_DIR      ?= $(HAX_LIBS_HOME)/.hints

.PHONY: all verify clean

all:
	rm -f .depend && $(MAKE) .depend
	$(MAKE) verify

# By default, we process all the files in the current directory. Here, we
# *extend* the set of relevant files with the tests.
ROOTS = $(wildcard *.fst *.fsti)

FSTAR_INCLUDE_DIRS = $(HAX_PROOF_LIBS_HOME)/rust_primitives $(HAX_PROOF_LIBS_HOME)/core $(HAX_LIBS_HOME)/proofs/fstar/extraction/

FSTAR_FLAGS = --cmi \
  --warn_error -331 \
  --cache_checked_modules --cache_dir $(CACHE_DIR) \
  --already_cached "+Prims+FStar+LowStar+C+TestLib" \
  $(addprefix --include ,$(FSTAR_INCLUDE_DIRS))

FSTAR = $(FSTAR_BIN) $(FSTAR_FLAGS) $(OTHERFLAGS)


.depend: $(HINT_DIR) $(CACHE_DIR)
	$(info $(ROOTS))
	$(FSTAR) --cmi --dep full $(ROOTS) --extract '* -Prims -LowStar -FStar' > $@

# Regenerating `.depend` runs F*, which `clean` must not need.
ifneq ($(MAKECMDGOALS),clean)
include .depend
endif

$(HINT_DIR):
	mkdir -p $@

$(CACHE_DIR):
	mkdir -p $@

$(CACHE_DIR)/%.checked: | .depend $(HINT_DIR) $(CACHE_DIR)
	$(FSTAR) $< $(ENABLE_HINTS) --hint_file $(HINT_DIR)/$(notdir $*).hints

verify: $(addsuffix .checked, $(addprefix $(CACHE_DIR)/,$(ROOTS)))

# Targets for interactive mode

%.fst-in:
	$(info $(FSTAR_FLAGS) \
	  $(ENABLE_HINTS) --hint_file $(HINT_DIR)/$(basename $@).fst.hints)

%.fsti-in:
	$(info $(FSTAR_FLAGS) \
	  $(ENABLE_HINTS) --hint_file $(HINT_DIR)/$(basename $@).fsti.hints)


# Clean targets

SHELL=/usr/bin/env bash

clean:
	rm -rf $(CACHE_DIR)/*
	rm -f .depend
