{ craneLib, stdenv, makeWrapper, lib, rustc, rustc-docs, gcc, hax-engine
, doCheck ? true, zlib, just, libiconv }:
let
  pname = "hax";
  is-webapp-static-asset = path:
    builtins.match ".*(script[.]js|index[.]html)" path != null;
  # Crate readmes are compiled in as rustdoc crate docs via
  # `#![doc = include_str!("../README.md")]`.
  is-crate-readme = path: builtins.match ".*/README[.]md" path != null;
  workspace = (builtins.fromTOML (builtins.readFile ../Cargo.toml)).workspace;
  is-in = dirs: relative:
    lib.any (dir: relative == dir || lib.hasPrefix "${dir}/" relative) dirs;
  # The workspace members and the files at the repository root, without the
  # trees the workspace excludes.
  is-workspace-path = relative:
    (is-in workspace.members relative || !(lib.hasInfix "/" relative)
      || is-in [ ".cargo" ] relative) && !(is-in workspace.exclude relative);
  source-filter = path: type:
    is-workspace-path (lib.removePrefix "${toString ./..}/" path)
    && ((builtins.isNull
      (builtins.match ".*/(tests|examples|docs|proof-libs)/.*" path)
      && (builtins.isNull (builtins.match ".*[.](md|svg)" path)
        || is-crate-readme path)
      && (craneLib.filterCargoSources path type || is-webapp-static-asset path
        || is-crate-readme path))
      || !(builtins.isNull (builtins.match ".*/renamings" path)));
  buildInputs = lib.optionals stdenv.isDarwin [ libiconv zlib.dev ];
  binaries = [ hax hax-engine.bin rustc gcc hax_rust_engine ] ++ buildInputs;
  commonArgs = {
    version = "0.0.1";
    # Filesets hold no empty directories, so that adding a directory outside
    # the kept trees does not affect the source hash. The result is passed as
    # a plain store path, as crane otherwise collects the dependency build's
    # `Cargo.toml` files from the unfiltered tree.
    src = craneLib.path {
      name = "source";
      path = "${lib.fileset.toSource {
        root = ./..;
        fileset = lib.fileset.fromSource (lib.cleanSourceWith {
          src = ./..;
          filter = source-filter;
        });
      }}";
    };
    inherit buildInputs doCheck;
    cargoExtraArgs = "--locked";
    doNotRemoveReferencesToRustToolchain = true;
  } // (if doCheck then {
    # [cargo test] builds independent workspaces. Each time another
    # workspace is added, it's corresponding lockfile should be added
    # in the [cargoLockList] list below.
    cargoVendorDir = craneLib.vendorMultipleCargoDeps {
      cargoLockList = [ ../Cargo.lock ../tests/Cargo.lock ];
    };
  } else
    { });
  # hax dependencies (without hax itself)
  cargoArtifacts = craneLib.buildDepsOnly (commonArgs // { pname = pname; });
  # `cargo-hax` alone, matching a plain `cargo install cargo-hax`: built in its
  # own invocation, scoped to just that package. `hax-driver` (below) needs
  # `hax-frontend-exporter/rustc` (`rustc_private`); building it in the same
  # `cargo build` as `cargo-hax` unifies that feature onto the copy of
  # `hax-frontend-exporter` `cargo-hax` also links, even though `cargo-hax`'s
  # own code isn't written against the `rustc_private` sysroot crates it would
  # pull in, which rustc reports as duplicated crates.
  hax_bin = craneLib.buildPackage (commonArgs // {
    inherit cargoArtifacts;
    pname = "cargo-hax";
    doInstallCargoArtifacts = true;
    cargoExtraArgs = "--locked -p cargo-hax";
  });
  # The rest of the workspace: `hax-driver`, the custom rustc driver
  # `cargo-hax` shells out to (needs `rustc_private`), plus the other
  # default-member libraries and binaries. The selection is `default-members`
  # of the workspace manifest minus `cargo-hax`; there is no cargo flag
  # expressing that, so it is derived here. Package names are read from each
  # member's own manifest, as they differ from the directory names.
  driver-and-libs-packages = map (member:
    (builtins.fromTOML
      (builtins.readFile (../. + "/${member}/Cargo.toml"))).package.name)
    (builtins.filter (member: member != "cli/cargo-hax")
      workspace.default-members);
  hax_driver_and_libs = craneLib.buildPackage (commonArgs // {
    inherit cargoArtifacts pname;
    doInstallCargoArtifacts = true;
    cargoExtraArgs = "--locked "
      + lib.concatMapStringsSep " " (p: "-p ${p}") driver-and-libs-packages;
  });
  # `hax-export-json-schemas`, which the OCaml engine's build consumes. Built
  # apart from the rest, scoped to `cargo-hax` alone for the same reason as
  # `hax_bin`, on whose artifacts it builds so that only what the feature
  # changes is recompiled.
  hax_export_json_schemas = craneLib.buildPackage (commonArgs // {
    cargoArtifacts = hax_bin;
    pname = "hax-export-json-schemas";
    cargoExtraArgs =
      "--locked -p cargo-hax --bin hax-export-json-schemas --features cargo-hax/legacy-engine";
  });
  # hax without cargo artifacts: only binaries
  hax = stdenv.mkDerivation {
    name = "hax-${commonArgs.version}";
    unpackPhase = "true";
    buildPhase = "true";
    installPhase = ''
      mkdir -p $out/bin
      cp ${hax_bin}/bin/cargo-hax $out/bin/
      cp -r ${hax_driver_and_libs}/bin/. $out/bin/
      cp ${hax_export_json_schemas}/bin/hax-export-json-schemas $out/bin/
      # Copied as a whole above, so a package the selection no longer covers
      # would otherwise go missing unnoticed.
      for binary in driver-hax-frontend-exporter test-driver; do
        test -x $out/bin/$binary || {
          echo "not built by hax_driver_and_libs: $binary"
          exit 1
        }
      done
    '';
  };
  hax_rust_engine = craneLib.buildPackage (commonArgs // {
    inherit cargoArtifacts;
    buildInputs = buildInputs ++ [ makeWrapper ];
    pname = "hax-rust-engine";
    cargoExtraArgs = "--manifest-path rust-engine/Cargo.toml --locked";
  });
  docs = craneLib.cargoDoc (commonArgs // {
    # preBuildPhases = [ "addRustcDocs" ];
    cargoDocExtraArgs = "--document-private-items";
    # addRustcDocs = ''
    #   mkdir -p target/doc
    #   cp --no-preserve=mode -rf ${rustc-docs}/share/doc/rust/html/rustc/* target/doc/
    # '';
    inherit cargoArtifacts pname;
  });
  tests = craneLib.buildPackage (commonArgs // {
    inherit cargoArtifacts;
    pname = "hax-tests";
    doCheck = true;
    CI = "true";
    cargoBuildCommand = "true";
    checkPhaseCargoCommand = ''
      cp -r --no-preserve=mode ${../tests} tests

      cp ${../justfile} justfile

      mv tests/snapshots tests/old-snapshots
      just test --no-verify
      diff -rN -x coq tests/old-snapshots tests/snapshots || {
        echo ""
        echo "The committed snapshots do not match what the engine produces."
        echo "Regenerate them with \`just test --no-verify\` and commit the result."
        exit 1
      }
    '';
    buildInputs = binaries ++ [ just ];
  });
in stdenv.mkDerivation {
  name = hax.name;
  buildInputs = [ makeWrapper ];
  phases = [ "installPhase" ];
  installPhase = ''
    mkdir -p $out/bin
    makeWrapper ${hax}/bin/cargo-hax $out/bin/cargo-hax \
      --prefix PATH : ${lib.makeBinPath binaries} \
      ${
        lib.optionalString stdenv.isDarwin ''
          --prefix RUSTFLAGS : "-C link-arg=-L${libiconv}/lib" \
          --suffix DYLD_LIBRARY_PATH : ${lib.makeLibraryPath [ zlib rustc ]}
        ''
      } \
      ${
        lib.optionalString stdenv.isLinux ''
          --suffix LD_LIBRARY_PATH : ${lib.makeLibraryPath [ zlib rustc ]}
        ''
      }
  '';
  meta.mainProgram = "cargo-hax";
  passthru = {
    unwrapped = hax;
    hax-engine-names-extract = craneLib.buildPackage (commonArgs // {
      pname = "hax_engine_names_extract";
      # This builds from `engine/names/extract`, where `cargo-hax` is not a
      # selectable package.
      cargoExtraArgs = "--locked";
      cargoLock = ../Cargo.lock;
      cargoToml = ../engine/names/extract/Cargo.toml;
      cargoArtifacts = hax_driver_and_libs;
      # `build.rs` here shells out to `cargo-hax`, which in turn needs
      # `hax-driver` on `PATH`: both are needed, not just `hax_driver_and_libs`.
      nativeBuildInputs = [ hax ];
      postUnpack = ''
        cd $sourceRoot/engine/names/extract
        sourceRoot="."
      '';
    });
    inherit docs tests;
  };
}
