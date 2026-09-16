//! Declares every test file under `src/` as a module, nested by chapter and
//! section, with the hyphens of the directory names replaced by underscores.
//! Hidden entries are skipped, and the module names are raw identifiers so
//! that a test may be named after a keyword.

use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
};

#[derive(Default)]
struct Dir {
    dirs: BTreeMap<String, Dir>,
    files: BTreeMap<String, PathBuf>,
}

fn collect(path: &Path, dir: &mut Dir) {
    for entry in fs::read_dir(path).unwrap() {
        let path = entry.unwrap().path();
        if path.file_name().unwrap().to_str().unwrap().starts_with('.') {
            continue;
        }
        if path.is_dir() {
            let name = module_name(&path, path.file_name().unwrap());
            collect(&path, dir.dirs.entry(name).or_default());
        } else if path.extension().is_some_and(|e| e == "rs") {
            let name = module_name(&path, path.file_stem().unwrap());
            dir.files.insert(name, path);
        }
    }
}

/// `stem` with hyphens replaced by underscores, which must be an identifier
/// that can be raw.
fn module_name(path: &Path, stem: &std::ffi::OsStr) -> String {
    let name = stem.to_str().unwrap().replace('-', "_");
    let identifier = name.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
    if !identifier || ["_", "self", "super", "crate", "Self"].contains(&name.as_str()) {
        panic!("{}: `{name}` cannot name a module", path.display());
    }
    name
}

fn render(dir: &Dir, out: &mut String) {
    for (name, sub) in &dir.dirs {
        *out += &format!("pub mod r#{name} {{\n");
        render(sub, out);
        *out += "}\n";
    }
    for (name, path) in &dir.files {
        *out += &format!("#[path = {path:?}]\npub mod r#{name};\n");
    }
}

fn main() {
    // Read at run time: a build script compiled for another checkout sharing
    // the target directory would otherwise generate paths into that one.
    let src = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src");
    println!("cargo::rerun-if-changed={}", src.display());
    let mut root = Dir::default();
    collect(&src, &mut root);
    let mut out = String::new();
    render(&root, &mut out);
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::write(out_dir.join("modules.rs"), out).unwrap();
}
