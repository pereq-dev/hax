#!/usr/bin/env bash

set -eu

SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"

opam_jobs=4
CLEANUP_WORKSPACE=on

# Parse command line arguments.
all_args=("$@")
while [ $# -gt 0 ]; do
    case "$1" in
    -j)
        opam_jobs=$2
        shift
        ;;
    --no-cleanup)
        CLEANUP_WORKSPACE=off
        ;;
    --help)
        echo "hax setup script"
        echo ""
        echo "Usage: $0 [OPTIONS]"
        echo ""
        echo "Options:"
        echo ' -j <JOBS>        The number of opam jobs to run in parallel'
        echo ' --no-cleanup     Disables the default behavior that runs `cargo clean` and `opam clean`'
        exit
        ;;
    *)
        printf '\e[31mError: unrecognized option \e[1m%s\e[0m\n' "$1"
        printf '\e[37mRun \e[1m%s --help\e[0m\e[37m for usage.\e[0m\n' "$0"
        exit 1
        ;;
    esac
    shift
done

# Cleanup the cargo and dune workspace, to make sure we are in a clean
# state
cleanup_workspace() {
    cargo clean
    (
        cd engine
        opam clean
    )
}

# Warns if we're building in a dirty checkout of hax: while hacking on
# hax, we should really be using `just build`.
warn_if_dirty() {
    (
        cd "$SCRIPTPATH"
        if ! git diff-index --quiet HEAD -- >& /dev/null; then
            printf '\e[33mWarning: This is a dirty checkout of hax!\n         If you are hacking on hax, please use the \e[1m`./.utils/rebuild.sh`\e[0m\e[33m script.\e[0m\n\n'
        fi
    )
}

# Ensures a given binary is available in PATH
ensure_binary_available() {
    command -v "$1" >/dev/null 2>&1 || {
        printf '\e[31mError: binary \e[1m%s\e[0m\e[31m was not found.\e[0m\n' "$1"
        printf '\e[37m(Did you look at \e[1mManual installation\e[0m\e[37m in \e[1mREADME.md\e[0m\e[37m?)\e[0m.\n'
        exit 1
    }
}

NODE_VERSION_MIN_MAJOR=17
ensure_node_is_recent_enough() {
    function strip_first_char () {
        cut -c2-
    }
    function get_major () {
        cut -d'.' -f1
    }
    VERSION=$(node --version)
    MAJOR=$(echo "$VERSION" | strip_first_char | get_major)
    if [[ "$MAJOR" -lt "$NODE_VERSION_MIN_MAJOR" ]]; then
        printf '\e[31mError: \e[1m%s\e[0m\e[31m appears to be too old.\e[0m\n' "NodeJS"
        printf '\e[37m(the minimal version required is \e[1m%s\e[0m\e[37m, yours is \e[1m%s\e[0m\e[37m)\e[0m.\n' "v${NODE_VERSION_MIN_MAJOR}.*.*" "$VERSION"
        exit 1
    fi
}

# Provides `install_rust_binaries`, installing `cargo-hax`, the frontend
# driver, and the other Rust binaries of the workspace
source "$SCRIPTPATH/.utils/install-rust-binaries.sh"

# Provides the `hax-engine` binary
install_ocaml_engine() {
    # Fixes out of memory issues (https://github.com/pereq-dev/hax/issues/197)
    {
        # Limit the number of thread spawned by opam
        export OPAMJOBS=$opam_jobs
        # Make the garbadge collector of OCaml more agressive (see
        # https://discuss.ocaml.org/t/how-to-limit-the-amount-of-memory-the-ocaml-compiler-is-allowed-to-use/797)
        export OCAMLRUNPARAM="o=20"
    }
    # Make opam show logs when an error occurs
    export OPAMERRLOGLEN=0
    # Make opam ignore system dependencies (it doesn't handle properly certain situations)
    export OPAMASSUMEDEPEXTS=1
    (
        set -x
        opam uninstall hax-engine || true
        # Lift the soft stack limit for ocamlopt: large preprocessed
        # files (e.g. `lib/types.pp.ml`) overflow the default stack on
        # recent GitHub Actions runner images. macOS rejects
        # `unlimited`, so try `hard` first.
        ulimit -s hard 2>/dev/null || ulimit -s unlimited 2>/dev/null || true
        opam install --yes ./engine
    )
}

warn_if_dirty

REQUIRED_OCAML_VERSION="5.4.1"
ensure_ocaml_version() {
    CURRENT_VERSION=$(opam exec -- ocamlc --version 2>/dev/null || echo "none")
    if [ "$CURRENT_VERSION" != "$REQUIRED_OCAML_VERSION" ]; then
        printf '\e[31mError: OCaml version \e[1m%s\e[0m\e[31m is required, but the current switch has \e[1m%s\e[0m\e[31m.\e[0m\n' \
            "$REQUIRED_OCAML_VERSION" "$CURRENT_VERSION"
        printf '\e[37mHint: run \e[1mopam switch create hax %s && eval $(opam env)\e[0m\e[37m\e[0m\n' "$REQUIRED_OCAML_VERSION"
        exit 1
    fi
}

for binary in opam node rustup jq; do
    ensure_binary_available $binary
done
ensure_node_is_recent_enough
ensure_ocaml_version

# Make sure the correct rust toolchain is installed
rustup show active-toolchain || rustup toolchain install 

if [ "$CLEANUP_WORKSPACE" = "on" ]; then
    cleanup_workspace
fi

install_rust_binaries --force
install_ocaml_engine
