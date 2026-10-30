//! Discovery of the FLS tests: the directives in the `//!` header of every
//! test file and the functions marked `#[hax_test::equiv]`.

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{Context as _, Result, bail};
use hax_types::{cli_options::BackendName, lean::to_camel_case};
use syn::{ImplItem, Item, TraitItem};

use super::stage::Tier;
use crate::directives::{Directive, TestDirective};

/// A test of the FLS suite, with the metadata authored in its file.
#[derive(Clone, Debug)]
pub struct FlsTest {
    /// `<chapter>/<section>/<name>`, the key of the test in the results files.
    pub path: String,
    /// The test file.
    pub source: PathBuf,
    pub fls: Vec<String>,
    pub title: String,
    pub no_equiv: Option<String>,
    pub tiers: HashMap<BackendName, Tier>,
    pub reasons: HashMap<BackendName, String>,
    /// The guard functions, in source order.
    pub equiv: Vec<EquivFn>,
}

/// A function marked `#[hax_test::equiv]`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EquivFn {
    /// The inline modules enclosing the function, outermost first.
    pub module_path: Vec<String>,
    pub name: String,
}

impl FlsTest {
    /// Name of the generated cargo package. Two paths can give the same name,
    /// since `/` and `-` both map to `_`.
    pub fn crate_name(&self) -> String {
        format!("fls_{}", self.path.replace(['/', '-'], "_"))
    }
}

/// Every `*.rs` file under `fls_dir`, skipping hidden files and directories,
/// sorted by path.
pub fn discover(fls_dir: &Path) -> Result<Vec<FlsTest>> {
    let mut tests = vec![];
    let walk = walkdir::WalkDir::new(fls_dir)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|e| e.depth() == 0 || !e.file_name().to_string_lossy().starts_with('.'));
    for entry in walk {
        let entry = entry?;
        if !entry.file_type().is_file() || entry.path().extension().is_none_or(|e| e != "rs") {
            continue;
        }
        tests.push(read_test(fls_dir, entry.path())?);
    }
    tests.sort_by(|a, b| a.path.cmp(&b.path));
    // Each test has a Lean library and a `<Lib>Guard` library beside it.
    let mut libs: HashMap<String, &str> = HashMap::new();
    for test in &tests {
        let lib = to_camel_case(&test.crate_name());
        for name in [format!("{lib}Guard"), lib] {
            if let Some(other) = libs.insert(name.clone(), &test.path) {
                bail!(
                    "`{other}` and `{}` map to the same Lean library `{name}`",
                    test.path
                );
            }
        }
    }
    Ok(tests)
}

fn read_test(fls_dir: &Path, source: &Path) -> Result<FlsTest> {
    let path = source
        .strip_prefix(fls_dir)?
        .with_extension("")
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/");
    let text =
        fs::read_to_string(source).with_context(|| format!("cannot read {}", source.display()))?;
    let display = source.display();

    let mut test = FlsTest {
        path,
        source: source.to_path_buf(),
        fls: vec![],
        title: String::new(),
        no_equiv: None,
        tiers: HashMap::new(),
        reasons: HashMap::new(),
        equiv: vec![],
    };
    for (line, directive) in
        directives_of_file(&text).map_err(|e| e.context(format!("in {display}")))?
    {
        let at = |what: &str| format!("{display}:{line}: {what}");
        let Directive::Test(directive) = directive else {
            bail!("{}", at("`@fail` is not supported by the FLS suite"));
        };
        match directive {
            TestDirective::Fls { ids } => test.fls.extend(ids),
            TestDirective::Title { title } => {
                if title.is_empty() {
                    bail!("{}", at("empty `@title`"));
                }
                if !test.title.is_empty() {
                    bail!("{}", at("duplicate `@title`"));
                }
                test.title = title;
            }
            TestDirective::NoEquiv { reason } => {
                if reason.is_empty() {
                    bail!("{}", at("empty `@no-equiv`"));
                }
                if test.no_equiv.is_some() {
                    bail!("{}", at("duplicate `@no-equiv`"));
                }
                test.no_equiv = Some(reason);
            }
            TestDirective::Tier { backend, tier } => {
                if test.tiers.insert(backend, tier).is_some() {
                    bail!("{}", at(&format!("duplicate `@{backend}(tier)`")));
                }
            }
            TestDirective::Reason { backend, reason } => {
                if reason.is_empty() {
                    bail!("{}", at(&format!("empty `@{backend}(reason)`")));
                }
                if test.reasons.insert(backend, reason).is_some() {
                    bail!("{}", at(&format!("duplicate `@{backend}(reason)`")));
                }
            }
            TestDirective::SetCli { .. } | TestDirective::Off { .. } => {
                bail!(
                    "{}",
                    at("`@cli` and `@off` are not supported by the FLS suite")
                )
            }
        }
    }
    if test.fls.is_empty() {
        bail!("{display}: missing `@fls`");
    }
    if test.title.is_empty() {
        bail!("{display}: missing `@title`");
    }
    if test.tiers.is_empty() {
        bail!("{display}: a test needs a `@<backend>(tier)` line for at least one backend");
    }

    test.equiv = equiv_functions(&text).map_err(|e| e.context(format!("in {display}")))?;
    if test.no_equiv.is_some() && !test.equiv.is_empty() {
        bail!("{display}: has both `#[hax_test::equiv]` functions and a `@no-equiv` line");
    }
    let all_unsupported = test.tiers.values().all(|t| *t == Tier::Unsupported);
    if test.no_equiv.is_none() && test.equiv.is_empty() && !all_unsupported {
        bail!("{display}: a test needs #[hax_test::equiv] functions or a @no-equiv line");
    }
    Ok(test)
}

/// The directives of the `//!` header: the leading run of empty and `//!`
/// lines, of which the lines starting with `@` are parsed. Each directive is
/// returned with its 1-based line number.
pub fn directives_of_file(text: &str) -> Result<Vec<(usize, Directive)>> {
    let mut directives = vec![];
    for (i, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Some(doc) = line.strip_prefix("//!") else {
            break;
        };
        let Some(directive) = doc.trim().strip_prefix('@') else {
            continue;
        };
        let directive = Directive::from_str(directive.trim())
            .with_context(|| format!("line {}: cannot parse `{}`", i + 1, line))?;
        directives.push((i + 1, directive));
    }
    Ok(directives)
}

/// Whether the attribute is `#[hax_test::equiv]` or `#[equiv]`.
fn is_equiv(attr: &syn::Attribute) -> bool {
    let segments: Vec<String> = attr
        .path()
        .segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect();
    let segments: Vec<&str> = segments.iter().map(String::as_str).collect();
    matches!(segments.as_slice(), ["equiv"] | ["hax_test", "equiv"])
}

fn equiv_attribute(attrs: &[syn::Attribute]) -> Option<&syn::Attribute> {
    attrs.iter().find(|a| is_equiv(a))
}

/// The functions marked `#[hax_test::equiv]`, at the crate root or in inline
/// modules.
pub fn equiv_functions(text: &str) -> Result<Vec<EquivFn>> {
    let file = syn::parse_file(text).map_err(|e| {
        anyhow::anyhow!("does not parse ({e}); run cargo check in tests/ for the full diagnostic")
    })?;
    let mut found = vec![];
    visit_items(&file.items, &mut vec![], &mut found)?;
    Ok(found)
}

fn visit_items(
    items: &[Item],
    module_path: &mut Vec<String>,
    found: &mut Vec<EquivFn>,
) -> Result<()> {
    for item in items {
        match item {
            Item::Fn(f) => {
                if let Some(attr) = equiv_attribute(&f.attrs) {
                    if let syn::Meta::List(list) = &attr.meta {
                        if list.tokens.to_string().contains("skip_lean") {
                            bail!(
                                "`{}`: skip_lean is not allowed in FLS tests; record the stage instead",
                                f.sig.ident
                            );
                        }
                        bail!(
                            "`{}`: #[hax_test::equiv] takes no arguments in FLS tests",
                            f.sig.ident
                        );
                    }
                    found.push(EquivFn {
                        module_path: module_path.clone(),
                        name: f.sig.ident.to_string(),
                    });
                }
            }
            Item::Mod(m) => match &m.content {
                Some((_, items)) => {
                    module_path.push(m.ident.to_string());
                    visit_items(items, module_path, found)?;
                    module_path.pop();
                }
                None => bail!(
                    "`mod {};`: auxiliary Rust files are not allowed in FLS tests",
                    m.ident
                ),
            },
            Item::Impl(i) => {
                for item in &i.items {
                    if let ImplItem::Fn(f) = item
                        && equiv_attribute(&f.attrs).is_some()
                    {
                        bail!(
                            "`{}`: #[hax_test::equiv] applies to free functions, not to methods",
                            f.sig.ident
                        );
                    }
                }
            }
            Item::Trait(t) => {
                for item in &t.items {
                    if let TraitItem::Fn(f) = item
                        && equiv_attribute(&f.attrs).is_some()
                    {
                        bail!(
                            "`{}`: #[hax_test::equiv] applies to free functions, not to trait methods",
                            f.sig.ident
                        );
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_directives() {
        let text = "\n//! @fls: fls_a, fls_b\n//!\n//! Some doc text.\n//! @title: T\n//! @no-equiv: erased\n//! @lean(tier): supported\n//! @lean(reason): why\n\nuse core::mem;\n//! @title: not read\n";
        let directives = directives_of_file(text).unwrap();
        let lines: Vec<usize> = directives.iter().map(|(l, _)| *l).collect();
        assert_eq!(lines, [2, 5, 6, 7, 8]);
        assert_eq!(
            directives[0].1,
            Directive::Test(TestDirective::Fls {
                ids: vec!["fls_a".into(), "fls_b".into()]
            })
        );
        assert_eq!(
            directives[3].1,
            Directive::Test(TestDirective::Tier {
                backend: BackendName::Lean,
                tier: Tier::Supported
            })
        );
    }

    #[test]
    fn malformed_directive_names_its_line() {
        let err = directives_of_file("//! @fls: a\n//! @lean(tier): sometimes\n")
            .unwrap_err()
            .to_string();
        assert!(err.contains("line 2"), "{err}");
    }

    #[test]
    fn equiv_walk() {
        let text = r#"
            use hax_test::equiv;
            #[hax_test::equiv]
            pub fn root() -> bool { true }
            pub fn plain() -> bool { true }
            mod inner {
                mod deeper {
                    #[equiv]
                    pub fn nested() -> bool { true }
                }
            }
            #[equiv]
            pub fn after_use() -> bool { true }
        "#;
        let found = equiv_functions(text).unwrap();
        assert_eq!(
            found,
            [
                EquivFn {
                    module_path: vec![],
                    name: "root".into()
                },
                EquivFn {
                    module_path: vec!["inner".into(), "deeper".into()],
                    name: "nested".into()
                },
                EquivFn {
                    module_path: vec![],
                    name: "after_use".into()
                },
            ]
        );
    }

    #[test]
    fn equiv_walk_errors() {
        let skip = "#[hax_test::equiv(skip_lean = \"x\")] pub fn f() -> bool { true }";
        assert!(
            equiv_functions(skip)
                .unwrap_err()
                .to_string()
                .contains("skip_lean")
        );
        let panics = "#[hax_test::equiv(panics)] pub fn f() -> bool { true }";
        assert!(
            equiv_functions(panics)
                .unwrap_err()
                .to_string()
                .contains("takes no arguments")
        );
        let method = "struct S; impl S { #[equiv] pub fn f(&self) -> bool { true } }";
        assert!(
            equiv_functions(method)
                .unwrap_err()
                .to_string()
                .contains("free functions")
        );
        let file_mod = "mod x;";
        assert!(
            equiv_functions(file_mod)
                .unwrap_err()
                .to_string()
                .contains("auxiliary")
        );
        let broken = "pub fn f( -> bool {}";
        assert!(
            equiv_functions(broken)
                .unwrap_err()
                .to_string()
                .contains("cargo check")
        );
    }

    fn write_test(dir: &Path, name: &str, text: &str) {
        let path = dir.join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }

    #[test]
    fn discovery_consistency() {
        let dir = std::env::temp_dir().join(format!("fls-discover-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let head = "//! @fls: fls_a\n//! @title: T\n";
        let cases: [(&str, &str, bool); 7] = [
            ("a/b/no_tier.rs", "pub fn f() -> bool { true }\n", false),
            (
                "a/b/plain.rs",
                "//! @lean(tier): supported\npub fn f() -> bool { true }\n",
                false,
            ),
            (
                "a/b/unsupported.rs",
                "//! @lean(tier): unsupported\npub fn f() -> bool { true }\n",
                true,
            ),
            (
                "a/b/both.rs",
                "//! @lean(tier): supported\n//! @no-equiv: r\n#[hax_test::equiv]\npub fn f() -> bool { true }\n",
                false,
            ),
            (
                "a/b/empty_reason.rs",
                "//! @lean(tier): unsupported\n//! @lean(reason):\npub fn f() -> bool { true }\n",
                false,
            ),
            (
                "a/b/empty_no_equiv.rs",
                "//! @lean(tier): supported\n//! @no-equiv:   \npub fn f() -> bool { true }\n",
                false,
            ),
            (
                "a/b/ok.rs",
                "//! @lean(tier): supported\n#[hax_test::equiv]\npub fn f() -> bool { true }\n",
                true,
            ),
        ];
        for (name, body, ok) in cases {
            let case = dir.join(name.replace('/', "_"));
            write_test(&case, name, &format!("{head}{body}"));
            write_test(&case, "README.md", "not a test");
            let result = discover(&case);
            assert_eq!(result.is_ok(), ok, "{name}: {result:?}");
            if let Ok(tests) = result {
                assert_eq!(tests.len(), 1);
                assert_eq!(tests[0].path, name.strip_suffix(".rs").unwrap());
                assert_eq!(
                    tests[0].crate_name(),
                    format!(
                        "fls_{}",
                        name.strip_suffix(".rs").unwrap().replace(['/', '-'], "_")
                    )
                );
            }
        }
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn lean_library_collision() {
        let text = "//! @fls: fls_a\n//! @title: T\n//! @lean(tier): unsupported\n";
        for (i, (a, b)) in [
            ("a/b-c/t.rs", "a/b_c/t.rs"),
            ("a/b/t_2d.rs", "a/b/t2d.rs"),
            ("a/b/t.rs", "a/b/t_guard.rs"),
        ]
        .into_iter()
        .enumerate()
        {
            let dir = std::env::temp_dir().join(format!("fls-collide-{}-{i}", std::process::id()));
            let _ = fs::remove_dir_all(&dir);
            write_test(&dir, a, text);
            write_test(&dir, b, text);
            let err = discover(&dir).unwrap_err().to_string();
            assert!(err.contains("same Lean library"), "{err}");
            fs::remove_dir_all(&dir).unwrap();
        }
    }
}
