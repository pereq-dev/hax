//! @off: fstar, lean, coq, ssprove, legacy-lean
//! @fail(extraction): proverif(HAX0001)
//!
//! Pattern forms beyond a flat constructor: nested constructors, literals
//! inside tuples, or-patterns under a constructor, rest patterns and
//! `matches!`.
#![allow(dead_code)]

fn nested_option(x: Option<Option<u8>>) -> u8 {
    match x {
        Some(Some(v)) => v,
        Some(None) => 1,
        None => 2,
    }
}

fn tuple_literals(x: (u8, u8)) -> u8 {
    match x {
        (0, y) => y,
        (y, 1) => y,
        _ => 3,
    }
}

fn or_under_constructor(x: Option<u8>) -> u8 {
    match x {
        Some(1 | 2) => 0,
        Some(_) => 1,
        None => 2,
    }
}

fn bool_scrutinee(b: bool) -> u8 {
    match b {
        true => 1,
        false => 0,
    }
}

fn negative_literal(x: i8) -> u8 {
    match x {
        -1 => 0,
        _ => 1,
    }
}

fn char_literal(c: char) -> u8 {
    match c {
        'a' => 0,
        '\n' => 1,
        _ => 2,
    }
}

fn str_literal(s: &str) -> u8 {
    match s {
        "yes" => 1,
        _ => 0,
    }
}

struct Header {
    kind: u8,
    len: u8,
    flags: u8,
}

fn struct_rest(h: Header) -> u8 {
    match h {
        Header { kind: 0, len, .. } => len,
        Header { flags, .. } => flags,
    }
}

fn tuple_rest(x: (u8, u8, u8)) -> u8 {
    let (a, ..) = x;
    a
}

fn is_some(x: Option<u8>) -> bool {
    matches!(x, Some(_))
}

// Later arms are unreachable once a binder matches everything.
fn binder_before_wildcard(x: Option<u8>) -> Option<u8> {
    match x {
        y => y,
        _ => None,
    }
}

// A failing guard would have to retry the next alternative (`guard_with_or((1,
// 5), 5)` is 0), which the model does not do: rejected.
fn guard_with_or(x: (u8, u8), y: u8) -> u8 {
    match x {
        (v, _) | (_, v) if v == y => 0,
        _ => 1,
    }
}

enum Message {
    Ping,
    Data(u8, u8),
    Close { code: u8 },
}

fn message_code(m: Message) -> u8 {
    match m {
        Message::Data(a, _) if a == 0 => 0,
        Message::Data(_, b) => b,
        Message::Close { code } => code,
        _ => 255,
    }
}
