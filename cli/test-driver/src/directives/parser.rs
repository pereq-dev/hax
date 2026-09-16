use anyhow::{Context, Result, bail};
use pest::Parser;
use pest::iterators::{Pair, Pairs};

#[derive(pest_derive::Parser)]
#[grammar_inline = r#"
WHITESPACE = _{ " " | "\t" }
NEWLINE = _{ "\n" | "\r\n" }

directive = { SOI ~ (NEWLINE)* ~ (cli | off | fail | backend_keyed | keyed) ~ (NEWLINE)* ~ EOI }

cli = { "cli" ~ ":" ~ line_text? }
off = { "off" ~ ":" ~ backend ~ ("," ~ backend)* }
fail = { "fail" ~ "(" ~ fail_kind ~ ")" ~ ":" ~ backend_fail ~ ("," ~ backend_fail)* }
backend_fail = { backend ~ "(" ~ errcode ~ ("," ~ errcode)* ~ ")" }
backend_keyed = { backend ~ "(" ~ key ~ ")" ~ ":" ~ line_text? }
keyed = { key ~ ":" ~ line_text? }

backend = @{ ASCII_ALPHANUMERIC+ ~ ("-" ~ ASCII_ALPHANUMERIC+)* }
errcode = @{ ASCII_ALPHANUMERIC+ }
key = @{ ASCII_ALPHA+ ~ ("-" ~ ASCII_ALPHA+)* }

fail_kind = @{ "tc" | "extraction" | "extract" }

line_text = { (!NEWLINE ~ ANY)* }
"#]
struct DirectivesParser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Directive {
    Cli {
        flags: String,
    },
    Off {
        backends: Vec<String>,
    },
    Fail {
        kind: FailureKind,
        entries: Vec<FailEntry>,
    },
    /// `@<key>: <value>`
    Keyed {
        key: String,
        value: String,
    },
    /// `@<backend>(<key>): <value>`
    BackendKeyed {
        backend: String,
        key: String,
        value: String,
    },
}

use super::FailureKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailEntry {
    pub backend: String,
    pub errcodes: Vec<String>,
}

/// Words that start a fixed-form directive. A `keyed` or `backend_keyed` parse
/// that begins with one of them is a malformed fixed-form directive.
const RESERVED: &[&str] = &["cli", "off", "fail"];

pub fn parse_directive(input: &str) -> Result<Directive> {
    let mut pairs = DirectivesParser::parse(Rule::directive, input)?;
    let directive_pair = pairs.next().context("directive parse produced no pairs")?;
    convert_directive(directive_pair)
}

fn convert_directive(pair: Pair<Rule>) -> Result<Directive> {
    let mut inner = pair.into_inner();
    let first = inner.next().context("empty directive")?;
    match first.as_rule() {
        Rule::cli => Ok(Directive::Cli {
            flags: extract_line_text(first.into_inner()),
        }),
        Rule::off => {
            let backends = first
                .into_inner()
                .filter(|p| p.as_rule() == Rule::backend)
                .map(|p| p.as_str().to_string())
                .collect();
            Ok(Directive::Off { backends })
        }
        Rule::fail => {
            let mut i = first.into_inner();
            let kind_pair = i.next().context("fail kind missing")?;
            let kind = match kind_pair.as_str() {
                "tc" => FailureKind::Typecheck,
                "extraction" | "extract" => FailureKind::Extract,
                _ => bail!("unknown fail kind"),
            };
            let entries = i
                .filter(|p| p.as_rule() == Rule::backend_fail)
                .map(parse_backend_fail)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Directive::Fail { kind, entries })
        }
        Rule::backend_keyed => {
            let mut i = first.into_inner();
            let backend = i.next().context("backend missing")?.as_str().to_string();
            let key = i.next().context("key missing")?.as_str().to_string();
            if RESERVED.contains(&backend.as_str()) {
                bail!("malformed `@{backend}` directive");
            }
            let value = extract_line_text(i);
            Ok(Directive::BackendKeyed {
                backend,
                key,
                value,
            })
        }
        Rule::keyed => {
            let mut i = first.into_inner();
            let key = i.next().context("key missing")?.as_str().to_string();
            if RESERVED.contains(&key.as_str()) {
                bail!("malformed `@{key}` directive");
            }
            let value = extract_line_text(i);
            Ok(Directive::Keyed { key, value })
        }
        _ => bail!("unexpected directive node"),
    }
}

fn extract_line_text(pairs: Pairs<Rule>) -> String {
    let mut s = String::new();
    for p in pairs {
        if p.as_rule() == Rule::line_text {
            s = p.as_str().trim().to_string();
        }
    }
    s
}

fn parse_backend_fail(pair: Pair<Rule>) -> Result<FailEntry> {
    let mut backend = None;
    let mut errcodes = Vec::new();
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::backend => backend = Some(p.as_str().to_string()),
            Rule::errcode => errcodes.push(p.as_str().to_string()),
            _ => {}
        }
    }
    Ok(FailEntry {
        backend: backend.context("backend missing in backend_fail")?,
        errcodes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyed_directives() {
        assert_eq!(
            parse_directive("fls: fls_a, fls_b").unwrap(),
            Directive::Keyed {
                key: "fls".into(),
                value: "fls_a, fls_b".into()
            }
        );
        assert_eq!(
            parse_directive("no-equiv: erased by the value model").unwrap(),
            Directive::Keyed {
                key: "no-equiv".into(),
                value: "erased by the value model".into()
            }
        );
        assert_eq!(
            parse_directive("lean(tier): unsupported").unwrap(),
            Directive::BackendKeyed {
                backend: "lean".into(),
                key: "tier".into(),
                value: "unsupported".into()
            }
        );
        assert!(parse_directive("fail(tc): fstar").is_err());
    }
}
