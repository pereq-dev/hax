{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-26.05";
    flake-utils.url = "github:numtide/flake-utils";
    crane = { url = "github:ipetkov/crane"; };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fstar.url = "github:FStarLang/FStar/v2025.10.06";
    hacl-star = {
      url = "github:hacl-star/hacl-star";
      flake = false;
    };
    rust-by-examples = {
      url = "github:rust-lang/rust-by-example";
      flake = false;
    };
  };

  outputs =
    { flake-utils, nixpkgs, rust-overlay, crane, hacl-star, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        toolchain =
          (fromTOML (pkgs.lib.readFile ./rust-toolchain.toml)).toolchain;
        rustc = pkgs.rust-bin.fromRustupToolchain toolchain;
        rustc-docs = (let
          # Only x86 linux has the component rustc-docs, see https://github.com/nix-community/fenix/issues/51
          # system = "x86_64-linux";
          n = toolchain // {
            components = toolchain.components ++ [ "rustc-docs" ];
          };
          rustc = builtins.trace n.components
            ((pkgs.rust-bin.fromRustupToolchain n).override {
              targets = [ "x86_64-unknown-linux-gnu" ];
            });
        in rustc);
        craneLib = (crane.mkLib pkgs).overrideToolchain rustc;
        rustfmt = pkgs.rustfmt;
        fstar = inputs.fstar.packages.${system}.default;
        hax-env-file = pkgs.writeText "hax-env-file" ''
          HAX_PROOF_LIBS_HOME="${./proof-libs/fstar}"
          HAX_LIBS_HOME="${./hax-lib}"/proofs/fstar/extraction
          HACL_HOME="${hacl-star}"
        '';
        hax-env = pkgs.writeScriptBin "hax-env" ''
          if [[ "$1" == "no-export" ]]; then
            cat "${hax-env-file}"
          else
            cat "${hax-env-file}" | xargs -I{} echo "export {}"
          fi
        '';
        ocamlPackages = pkgs.ocamlPackages;
        ocamlformat = ocamlPackages.ocamlformat;
        proverif = pkgs.proverif.overrideDerivation
          (_: { patches = [ examples/proverif-psk/pv_div_by_zero_fix.diff ]; });
      in rec {
        packages = {
          inherit rustc ocamlformat rustfmt fstar hax-env rustc-docs proverif;
          docs = pkgs.python3Packages.callPackage ./docs {
            hax-frontend-docs = packages.hax-rust-frontend.docs;
          };
          hax-engine = pkgs.callPackage ./engine {
            hax-rust-frontend = packages.hax-rust-frontend.unwrapped;
            # `hax-engine-names-extract` extracts Rust names but also
            # some informations about `impl`s when names are `impl`
            # blocks. That includes some span information, which
            # includes full paths to Rust sources. Sometimes those
            # Rust sources happens to be in the Nix store. That
            # creates useless dependencies, this wrapper below takes
            # care of removing those extra depenedencies.
            hax-engine-names-extract =
              pkgs.writeScriptBin "hax-engine-names-extract" ''
                #!${pkgs.stdenv.shell}
                ${packages.hax-rust-frontend.hax-engine-names-extract}/bin/hax-engine-names-extract | sed 's|/nix/store/\(.\{6\}\)|/nix_store/\1-|g'
              '';
            inherit rustc ocamlPackages;
          };
          hax-rust-frontend = pkgs.callPackage ./cli {
            inherit rustc craneLib rustc-docs;
            inherit (packages) hax-engine;
          };
          hax = packages.hax-rust-frontend;
          default = packages.hax;

          check-toolchain = checks.toolchain;
          check-readme-coherency = checks.readme-coherency;

          rust-by-example-hax-extraction = pkgs.stdenv.mkDerivation {
            name = "rust-by-example-hax-extraction";
            phases = [ "installPhase" ];
            buildInputs = [ packages.hax pkgs.cargo ];
            installPhase = ''
              cp --no-preserve=mode -rf ${inputs.rust-by-examples} workdir
              cd workdir
              ${pkgs.nodejs}/bin/node ${./.utils/rust-by-example.js}
              mv rust-by-examples-crate/proofs $out
            '';
          };

          # The commit that corresponds to our nightly pin, helpful when updating rusrc.
          toolchain_commit = pkgs.runCommand "hax-toolchain-commit" { } ''
            # This is sad but I don't know a better way.
            cat ${rustc}/share/doc/rust/html/version_info.html \
              | grep 'github.com' \
              | sed 's#.*"https://github.com/rust-lang/rust/commit/\([^"]*\)".*#\1#' \
              > $out
          '';
        };
        checks = {
          toolchain = packages.hax.tests;
          readme-coherency =
            let src = pkgs.lib.sourceFilesBySuffices ./. [ ".md" ];
            in pkgs.stdenv.mkDerivation {
              name = "readme-coherency";
              inherit src;
              buildPhase = ''
                ${apps.replace-fstar-versions-md.program}
                diff -r . ${src}
              '';
              installPhase = "touch $out";
            };
        };
        apps = {
          replace-fstar-versions-md = {
            type = "app";
            program = "${pkgs.writeScript "replace-fstar-versions-md" ''
              #!${pkgs.bash}/bin/bash
              FSTAR_VERSION=$(cat ${
                ./flake.lock
              } | ${pkgs.jq}/bin/jq '.nodes.fstar.original.ref' -r)
              ${pkgs.fd}/bin/fd \
                 -X ${pkgs.sd}/bin/sd '`.*?`(<!---FSTAR_VERSION-->)' '`'"$FSTAR_VERSION"'`$1' **/*.md \
                 ";" --glob '*.md'
            ''}";
          };
          serve-rustc-docs = {
            type = "app";
            program = "${pkgs.writeScript "serve-rustc-docs" ''
              #!${pkgs.bash}/bin/bash
              cd ${rustc-docs}/share/doc/rust/html/rustc
              ${pkgs.python3}/bin/python -m http.server "$@"
            ''}";
          };
          serve-docs = {
            type = "app";
            program = "${pkgs.writeScript "serve-docs" ''
              #!${pkgs.bash}/bin/bash
              cd ${packages.docs}
              ${pkgs.python3}/bin/python -m http.server "$@"
            ''}";
          };
        };
        devShells = let
          inputsFrom = [
            packages.hax-rust-frontend.unwrapped
            # `hax-engine`'s build requires `hax-rust-frontend` and
            # `hax-engine-names-extract`, but in a dev environment,
            # those two packages are supposed to be built locally,
            # thus we kill them here
            (packages.hax-engine.override {
              hax-rust-frontend = pkgs.hello;
              hax-engine-names-extract = pkgs.hello;
            })
            packages.docs
          ];
          utils = pkgs.stdenv.mkDerivation {
            name = "hax-dev-scripts";
            phases = [ "installPhase" ];
            installPhase = ''
              mkdir -p $out/bin
              cp ${./.utils/rebuild.sh} $out/bin/rebuild
            '';
          };
          defaultPackages = [
            ocamlformat
            ocamlPackages.ocaml-lsp
            ocamlPackages.ocamlformat-rpc-lib
            ocamlPackages.ocaml-print-intf
            ocamlPackages.odoc
            ocamlPackages.utop

            pkgs.just
            pkgs.cargo-expand
            pkgs.cargo-release
            pkgs.openssl.dev
            pkgs.zlib.dev
            pkgs.pkg-config
            pkgs.rust-analyzer
            pkgs.toml2json
            pkgs.zstd
            rustfmt
            utils

            pkgs.go-grip
          ];
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          DYLD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.zlib rustc ];
        in {
          examples = pkgs.mkShell {
            inherit LIBCLANG_PATH DYLD_LIBRARY_PATH;
            inputsFrom = [ devShells.fstar ];
            HACL_HOME = "${hacl-star}";
            shellHook = ''
              HAX_ROOT=$(git rev-parse --show-toplevel)
              export HAX_PROOF_LIBS_HOME="$HAX_ROOT/proof-libs/fstar"
              export HAX_LIBS_HOME="$HAX_ROOT/hax-lib"
            '';
            packages = [
              proverif
              pkgs.elan
              pkgs.jq
            ];
          };
          ci-examples = pkgs.mkShell {
            shellHook = ''
              eval $(hax-env)
              export CACHE_DIR=$(mktemp -d)
              export HINT_DIR=$(mktemp -d)
              export SHELL=${pkgs.bash}/bin/bash
              # Ensure locally-compiled crates (e.g. the test-driver)
              # embed the same HAX_VERSION as the Nix-built cargo-hax.
              export HAX_VERSION=${(builtins.fromTOML (builtins.readFile ./Cargo.toml)).workspace.package.version}
            '';
            packages = [
              packages.hax
              packages.hax-env
              packages.rustc
              packages.fstar
              packages.proverif
              pkgs.jq
              pkgs.just
              pkgs.elan
              # `rust_lean_equiv_test`'s `gen_lean_tests.py` runs inside this shell.
              pkgs.python3
            ];
          };
          default = pkgs.mkShell {
            inherit inputsFrom LIBCLANG_PATH DYLD_LIBRARY_PATH;
            packages = defaultPackages;
          };
          # For `tests/verify`: the provers and nothing else, since verifying
          # the committed snapshots needs no hax, rustc or engine.
          verify-fstar = pkgs.mkShell {
            packages = [ packages.fstar pkgs.gnumake pkgs.just pkgs.git ];
          };
          verify-lean = pkgs.mkShell {
            packages = [ pkgs.elan pkgs.gnumake pkgs.just pkgs.git ];
          };
          verify-proverif = pkgs.mkShell {
            packages = [ proverif pkgs.gnumake pkgs.just pkgs.git ];
          };
          fstar = pkgs.mkShell {
            inherit inputsFrom LIBCLANG_PATH DYLD_LIBRARY_PATH;
            shellHook = ''
              export HAX_HOME=$(git rev-parse --show-toplevel)
              export FSTAR_HOME="${fstar}"
            '';
            packages = defaultPackages ++ [ fstar ];
          };
        };
      });
}
