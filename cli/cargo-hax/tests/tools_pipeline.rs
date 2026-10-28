//! End-to-end tests for the lean backend's tool resolution: `hax.toml`
//! path entries with the sibling rule, and on-demand installs from
//! `hax.toml` pins, exercised through the real binary with stub
//! charon/aeneas executables.

mod common;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use common::{make_archive, platform, serve, sha256_hex, stub, stub_pipeline_tools};

/// A minimal crate to run the backend on.
fn write_crate(dir: &Path) {
    common::write_crate(dir, "fixture");
}

/// Point a crate's `hax.toml` at the charon and aeneas binaries in `bin`.
fn write_path_entries(crate_dir: &Path, bin: &Path) {
    std::fs::write(crate_dir.join("hax.toml"), common::path_entries(bin)).unwrap();
}

fn run_backend(crate_dir: &Path, envs: &[(&str, &str)]) -> (String, bool) {
    common::run_hax(&["into", "lean"], crate_dir, envs)
}

#[test]
fn path_entry_runs_the_supplied_binaries_with_a_notice() {
    let dir = tempfile::tempdir().unwrap();
    let crate_dir = dir.path().join("crate");
    write_crate(&crate_dir);
    let bin = dir.path().join("bin");
    stub_pipeline_tools(&bin, &stub("aeneas-invoked"));
    write_path_entries(&crate_dir, &bin);

    let (output, success) = run_backend(&crate_dir, &[]);
    assert!(success, "{output}");
    // The stubs ran (charon records in the crate dir, aeneas too).
    assert!(crate_dir.join("charon-invoked").is_file(), "{output}");
    assert!(crate_dir.join("aeneas-invoked").is_file(), "{output}");
    // The non-default notice names both paths.
    assert!(output.contains("was tested with"), "{output}");
    assert!(
        output.contains(bin.join("charon").to_str().unwrap()),
        "{output}"
    );
}

#[test]
fn a_charon_run_that_produced_no_llbc_must_not_extract_from_a_stale_one() {
    let dir = tempfile::tempdir().unwrap();
    let crate_dir = dir.path().join("crate");
    write_crate(&crate_dir);
    let bin = dir.path().join("bin");
    stub_pipeline_tools(&bin, &stub("aeneas-invoked"));
    // A charon that exits 0 without writing its `--dest-file`.
    common::write_executable(&bin.join("charon"), &stub("charon-invoked"));
    write_path_entries(&crate_dir, &bin);
    // An LLBC file left by an earlier run.
    let llbc = crate_dir.join("proofs/lean/llbc/fixture.llbc");
    std::fs::create_dir_all(llbc.parent().unwrap()).unwrap();
    std::fs::write(&llbc, "stale").unwrap();

    let (output, success) = run_backend(&crate_dir, &[]);
    assert!(!success, "{output}");
    assert!(output.contains("did not produce"), "{output}");
    // The stale file is gone and aeneas never ran.
    assert!(!llbc.exists());
    assert!(!crate_dir.join("aeneas-invoked").exists());
}

#[test]
fn overriding_charons_dest_file_is_an_error() {
    let dir = tempfile::tempdir().unwrap();
    let crate_dir = dir.path().join("crate");
    write_crate(&crate_dir);
    let bin = dir.path().join("bin");
    stub_pipeline_tools(&bin, &stub("aeneas-invoked"));
    write_path_entries(&crate_dir, &bin);
    // An LLBC file left by an earlier, successful run.
    let llbc = crate_dir.join("proofs/lean/llbc/fixture.llbc");
    std::fs::create_dir_all(llbc.parent().unwrap()).unwrap();
    std::fs::write(&llbc, "previous").unwrap();

    let (output, success) = common::run_hax(
        &["into", "lean", "--charon-args=--dest-file /elsewhere.llbc"],
        &crate_dir,
        &[],
    );
    assert!(!success, "{output}");
    assert!(output.contains("--dest-file"), "{output}");
    assert!(output.contains("hax reserves"), "{output}");
    // The error precedes any tool run and any file removal.
    assert!(!crate_dir.join("charon-invoked").exists());
    assert!(llbc.exists());
}

#[test]
fn missing_sibling_executable_is_a_clear_error() {
    let dir = tempfile::tempdir().unwrap();
    let crate_dir = dir.path().join("crate");
    write_crate(&crate_dir);
    let bin = dir.path().join("bin");
    // charon without charon-driver next to it.
    common::write_executable(&bin.join("charon"), &stub("charon-invoked"));
    common::write_executable(&bin.join("aeneas"), &stub("aeneas-invoked"));
    write_path_entries(&crate_dir, &bin);

    let (output, success) = run_backend(&crate_dir, &[]);
    assert!(!success);
    assert!(output.contains("charon-driver"), "{output}");
    assert!(output.contains("was not found next to"), "{output}");
}

#[test]
fn hax_toml_pin_installs_on_demand_and_runs_from_the_cache() {
    // Fixture archives holding executable stubs.
    let charon = make_archive(&[
        ("charon", &common::charon_stub()),
        ("charon-driver", &stub("driver-invoked")),
    ]);
    let aeneas = make_archive(&[("aeneas", &stub("aeneas-invoked"))]);
    let (charon_sha, aeneas_sha) = (sha256_hex(&charon), sha256_hex(&aeneas));
    let server = serve(HashMap::from([
        ("/charon.tar.gz".to_string(), charon),
        ("/aeneas.tar.gz".to_string(), aeneas),
    ]));

    let dir = tempfile::tempdir().unwrap();
    let manifest_path: PathBuf = dir.path().join("manifest.toml");
    std::fs::write(
        &manifest_path,
        format!(
            r#"[tools.charon."stub-v1".{platform}]
url = "{base}/charon.tar.gz"
sha256 = "{charon_sha}"
entry_points = {{ charon = "charon", charon-driver = "charon-driver" }}

[tools.aeneas."stub-v1".{platform}]
url = "{base}/aeneas.tar.gz"
sha256 = "{aeneas_sha}"
entry_points = {{ aeneas = "aeneas" }}
"#,
            platform = platform(),
            base = server.base_url,
        ),
    )
    .unwrap();
    let cache = dir.path().join("cache");
    std::fs::create_dir(&cache).unwrap();

    let crate_dir = dir.path().join("crate");
    write_crate(&crate_dir);
    std::fs::write(
        crate_dir.join("hax.toml"),
        "[tools]\ncharon = \"stub-v1\"\naeneas = \"stub-v1\"\n",
    )
    .unwrap();

    let (output, success) = run_backend(
        &crate_dir,
        &[
            ("HAX_TOOLS_MANIFEST", manifest_path.to_str().unwrap()),
            ("XDG_CACHE_HOME", cache.to_str().unwrap()),
        ],
    );
    assert!(success, "{output}");
    // Downloaded on demand, with the note and the non-default notice.
    assert!(output.contains("Downloading charon stub-v1"), "{output}");
    assert!(output.contains("was tested with"), "{output}");
    // The cached stubs actually ran.
    assert!(crate_dir.join("charon-invoked").is_file(), "{output}");
    assert!(crate_dir.join("aeneas-invoked").is_file(), "{output}");
    // The cache holds both tools.
    assert!(cache.join("hax/tools/charon/stub-v1/charon").is_file());
    assert!(cache.join("hax/tools/aeneas/stub-v1/aeneas").is_file());

    // A second run is a pure cache hit: no download note.
    std::fs::remove_file(crate_dir.join("charon-invoked")).unwrap();
    let (output, success) = run_backend(
        &crate_dir,
        &[
            ("HAX_TOOLS_MANIFEST", manifest_path.to_str().unwrap()),
            ("XDG_CACHE_HOME", cache.to_str().unwrap()),
        ],
    );
    assert!(success, "{output}");
    assert!(!output.contains("Downloading"), "{output}");
    assert!(crate_dir.join("charon-invoked").is_file());
}

#[test]
fn lakefile_pins_come_from_the_resolved_versions() {
    let dir = tempfile::tempdir().unwrap();
    let crate_dir = dir.path().join("crate");
    write_crate(&crate_dir);
    let bin = dir.path().join("bin");
    stub_pipeline_tools(&bin, &stub("aeneas-invoked"));
    std::fs::write(
        crate_dir.join("hax.toml"),
        format!(
            r#"[tools]
charon = {{ path = "{charon}" }}
aeneas = {{ path = "{aeneas}" }}

[versions]
lean = "leanprover/lean4:v9.9.9-test"
hax-lean-lib = "v9.9.9"
"#,
            charon = bin.join("charon").display(),
            aeneas = bin.join("aeneas").display(),
        ),
    )
    .unwrap();

    let (all, success) = run_backend(&crate_dir, &[]);
    assert!(success, "{all}");

    let lean_dir = crate_dir.join("proofs/lean");
    let toolchain = std::fs::read_to_string(lean_dir.join("lean-toolchain")).unwrap();
    assert_eq!(toolchain.trim(), "leanprover/lean4:v9.9.9-test");
    let lakefile = std::fs::read_to_string(lean_dir.join("lakefile.toml")).unwrap();
    assert!(lakefile.contains("rev = \"v9.9.9\""), "{lakefile}");
    // aeneas resolves to a path: the lakefile pins the default rev, with
    // a warning naming the substitution.
    assert!(all.contains("pinning the aeneas Lean library"), "{all}");
    // Both declared versions deviate from the built-in defaults, so each
    // gets the non-default notice naming the tested version.
    assert!(
        all.contains("using lean leanprover/lean4:v9.9.9-test"),
        "{all}"
    );
    assert!(all.contains("using hax-lean-lib v9.9.9"), "{all}");
}

/// A crate with stub tools, overridden declared versions, and an existing
/// Lean project whose lakefile pins `hax` at `hax_rev` and whose
/// `lean-toolchain` holds `toolchain`.
fn write_crate_with_lean_project(dir: &Path, hax_rev: &str, toolchain: &str) -> PathBuf {
    let crate_dir = dir.join("crate");
    write_crate(&crate_dir);
    let bin = dir.join("bin");
    stub_pipeline_tools(&bin, &stub("aeneas-invoked"));
    std::fs::write(
        crate_dir.join("hax.toml"),
        format!(
            r#"[tools]
charon = {{ path = "{charon}" }}
aeneas = {{ path = "{aeneas}" }}

[versions]
lean = "leanprover/lean4:v9.9.9-test"
hax-lean-lib = "v9.9.9"
"#,
            charon = bin.join("charon").display(),
            aeneas = bin.join("aeneas").display(),
        ),
    )
    .unwrap();
    let lean_dir = crate_dir.join("proofs/lean");
    std::fs::create_dir_all(&lean_dir).unwrap();
    std::fs::write(
        lean_dir.join("lakefile.toml"),
        format!(
            r#"[[require]]
name = "aeneas"
git = "https://github.com/pereq-dev/aeneas"
rev = "nightly-old"
subDir = "backends/lean"

[[require]]
name = "hax"
git = "https://github.com/cryspen/hax-lean"
rev = "{hax_rev}"
"#
        ),
    )
    .unwrap();
    std::fs::write(lean_dir.join("lean-toolchain"), format!("{toolchain}\n")).unwrap();
    crate_dir
}

#[test]
fn stale_pins_in_an_existing_lean_project_are_warned_about() {
    let dir = tempfile::tempdir().unwrap();
    let crate_dir = write_crate_with_lean_project(dir.path(), "v0.1.0", "leanprover/lean4:v4.30.0");

    // The pin check runs on every extraction.
    let (output, success) = run_backend(&crate_dir, &[]);
    assert!(success, "{output}");
    assert!(
        output.contains("pins hax v0.1.0; the current configuration expects v9.9.9"),
        "{output}"
    );
    assert!(
        output.contains(
            "pins lean leanprover/lean4:v4.30.0; \
             the current configuration expects leanprover/lean4:v9.9.9-test"
        ),
        "{output}"
    );
    // aeneas resolves to a path, so its pin has nothing to be checked against.
    assert!(!output.contains("pins aeneas"), "{output}");
}

#[test]
fn matching_pins_in_an_existing_lean_project_pass_silently() {
    let dir = tempfile::tempdir().unwrap();
    let crate_dir =
        write_crate_with_lean_project(dir.path(), "v9.9.9", "leanprover/lean4:v9.9.9-test");

    let (output, success) = run_backend(&crate_dir, &[]);
    assert!(success, "{output}");
    assert!(
        !output.contains("the current configuration expects"),
        "{output}"
    );
}

#[test]
fn default_declared_versions_are_not_noticed() {
    let dir = tempfile::tempdir().unwrap();
    let crate_dir = dir.path().join("crate");
    write_crate(&crate_dir);
    let bin = dir.path().join("bin");
    stub_pipeline_tools(&bin, &stub("aeneas-invoked"));
    // No `[versions]` table: both declared versions resolve to the defaults.
    write_path_entries(&crate_dir, &bin);

    let (all, success) = run_backend(&crate_dir, &[]);
    assert!(success, "{all}");
    // The path-overridden tools are noticed, the declared versions are not.
    assert!(!all.contains("using lean "), "{all}");
    assert!(!all.contains("using hax-lean-lib "), "{all}");
}
