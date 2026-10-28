# This is a generically useful Makefile for F* that is self-contained
#
# It is tempting to factor this out into multiple Makefiles but that
# makes it less portable, so resist temptation, or move to a more
# sophisticated build system.
#
# We expect:
#  1. `fstar.exe` to be in PATH (alternatively, you can also set
#     $FSTAR_HOME to be set to your F* repo/install directory)
#
#  2. `cargo`, `rustup`, `hax` and `jq` to be installed and in PATH.
#
#  3. the extracted Cargo crate to have "hax-lib" as a dependency:
#     `hax-lib = { version = "0.1.0-pre.1", git = "https://github.com/pereq-dev/hax"}`
#
# Optionally, you can set `HACL_HOME`.
#
# ROOTS contains all the top-level F* files you wish to verify
# The default target `verify` verified ROOTS and its dependencies
# To lax-check instead, set `OTHERFLAGS="--lax"` on the command-line
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

HACL_HOME     ?= $(HOME)/.hax/hacl_home
FSTAR_BIN     ?= $(shell command -v fstar.exe 1>&2 2> /dev/null && echo "fstar.exe" || echo "$(FSTAR_HOME)/bin/fstar.exe")

CACHE_DIR     ?= .cache
HINT_DIR      ?= .hints

SHELL ?= /usr/bin/env bash

EXECUTABLES = cargo cargo-hax jq
K := $(foreach bin,$(EXECUTABLES),\
        $(if $(shell command -v $(bin) 2> /dev/null),,$(error "No $(bin) in PATH")))

.PHONY: all verify clean

all:
	rm -f .depend && $(MAKE) .depend
	$(MAKE) verify

# The extraction configuration lives in the `chacha20` scenario in the
# crate's `hax.toml`.
HAX_CLI = "cargo hax extract chacha20"

# If $HACL_HOME doesn't exist, clone it
${HACL_HOME}:
	mkdir -p "${HACL_HOME}"
	git clone --depth 1 https://github.com/hacl-star/hacl-star.git "${HACL_HOME}"

# If no any F* file is detected, we run hax
ifeq "$(wildcard *.fst *fsti)" ""
_ := $(shell $(SHELL) -c $(HAX_CLI))
endif

# By default, we process all the files in the current directory
ROOTS = $(wildcard *.fst *fsti)

# Regenerate F* files via hax when Rust sources change
$(ROOTS): $(shell find ../../../../src -type f -name '*.rs')
	$(shell $(SHELL) -c $(HAX_CLI))

# The following is a bash script that discovers F* libraries
define FINDLIBS
    # Prints a path if and only if it exists. Takes one argument: the
    # path.
    function print_if_exists() {
        if [ -d "$$1" ]; then
            echo "$$1"
        fi
    }
    # Asks Cargo all the dependencies for the current crate or workspace,
    # and extract all "root" directories for each. Takes zero argument.
    function dependencies() {
        cargo metadata --format-version 1 |
            jq -r '.packages | .[] | .manifest_path | split("/") | .[:-1] | join("/")'
    }
    # Find hax libraries *around* a given path. Takes one argument: the
    # path.
    function find_hax_libraries_at_path() {
        path="$$1"
        # if there is a `proofs/fstar/extraction` subfolder, then that's a
        # F* library
        print_if_exists "$$path/proofs/fstar/extraction"
        # Maybe the `proof-libs` folder of hax is around?
        MAYBE_PROOF_LIBS=$$(realpath -q "$$path/../proof-libs/fstar")
        if [ $$? -eq 0 ]; then
            print_if_exists "$$MAYBE_PROOF_LIBS/core"
            print_if_exists "$$MAYBE_PROOF_LIBS/rust_primitives"
        fi
    }
    { while IFS= read path; do
          find_hax_libraries_at_path "$$path"
      done < <(dependencies)
    } | sort -u
endef
export FINDLIBS

FSTAR_INCLUDE_DIRS = $(HACL_HOME)/lib $(shell bash -c "$$FINDLIBS")

FSTAR_FLAGS = --cmi \
  --warn_error -331 \
  --cache_checked_modules --cache_dir $(CACHE_DIR) \
  --already_cached "+Prims+FStar+LowStar+C+Spec.Loops+TestLib" \
  $(addprefix --include ,$(FSTAR_INCLUDE_DIRS))

FSTAR = $(FSTAR_BIN) $(FSTAR_FLAGS)

.depend: $(HINT_DIR) $(CACHE_DIR) $(ROOTS)
	$(info $(ROOTS))
	$(FSTAR) --cmi --dep full $(ROOTS) --extract '* -Prims -LowStar -FStar' > $@

include .depend

$(HINT_DIR):
	mkdir -p $@

$(CACHE_DIR):
	mkdir -p $@

$(CACHE_DIR)/%.checked: | .depend $(HINT_DIR) $(CACHE_DIR)
	$(FSTAR) $(OTHERFLAGS) $< $(ENABLE_HINTS) --hint_file $(HINT_DIR)/$(notdir $*).hints

verify: $(addsuffix .checked, $(addprefix $(CACHE_DIR)/,$(ROOTS)))

# Targets for interactive mode

%.fst-in:
	$(info $(FSTAR_FLAGS) \
	  $(ENABLE_HINTS) --hint_file $(HINT_DIR)/$(basename $@).fst.hints)

%.fsti-in:
	$(info $(FSTAR_FLAGS) \
	  $(ENABLE_HINTS) --hint_file $(HINT_DIR)/$(basename $@).fsti.hints)


# Clean targets

clean:
	rm -rf $(CACHE_DIR)/*
	rm *.fst

# Special rule for `Chacha20.Hacspec_helper`
$(CACHE_DIR)/Chacha20.Hacspec_helper.fst.checked: | .depend $(HINT_DIR) $(CACHE_DIR)
	$(FSTAR) --lax $(OTHERFLAGS) $< $(ENABLE_HINTS) --hint_file $(HINT_DIR)/$(notdir $*).hints
