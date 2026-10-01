//! @off: fstar, lean, coq, ssprove, legacy-lean
//!
//! Constructs whose model must keep the Rust evaluation order: a failure in a
//! condition ends the trace, `&&` and `||` short-circuit, and a guarded arm
//! falls through to the next one. Also patterns ProVerif restricts (`()`, an
//! external constructor, nested `@` bindings) and locals named like the binders
//! the printer generates.

fn check(x: u8) -> Result<u8, ()> {
    if x == 0 { Err(()) } else { Ok(x) }
}

fn reject_on_error(x: u8) -> u8 {
    if check(x).is_err() { 0 } else { 1 }
}

fn short_circuit_or(o: Option<u8>, x: u8) -> bool {
    o.is_none() || o.unwrap() != x
}

fn short_circuit_and(o: Option<u8>, x: u8) -> bool {
    o.is_some() && o.unwrap() == x
}

fn unit_patterns(r: Result<(), u8>) -> u8 {
    let n = match r {
        Ok(()) => 1,
        Err(_) => 0,
    };
    if let Ok(()) = r { n } else { 2 }
}

fn external_constructors(a: u8, b: u8) -> u8 {
    match a.cmp(&b) {
        core::cmp::Ordering::Less => 0,
        core::cmp::Ordering::Equal => 1,
        core::cmp::Ordering::Greater => 2,
    }
}

fn nested_bindings(x: (u8, (u8, u8))) -> u8 {
    let t @ (a, b @ (c, _)) = x;
    c
}

fn guarded(x: Option<u8>, y: u8) -> u8 {
    match x {
        Some(v) if v == y => 0,
        Some(v) if v > y => 1,
        Some(v) if check(v).is_ok() => 2,
        _ => 3,
    }
}

fn generated_names(wildcard: u8, _: u8, hax__scrut0: (u8, u8)) -> u8 {
    let bl__seq0 = wildcard;
    match hax__scrut0 {
        (_, y) => wildcard + y + bl__seq0,
    }
}

fn keyword(public_vars: u8) -> u8 {
    public_vars
}

fn five_pushes(a: u8, b: u8, c: u8, d: u8, e: u8) -> u8 {
    let mut v = Vec::new();
    v.push(a);
    v.push(b);
    v.push(c);
    v.push(d);
    v.push(e);
    let mut acc = 0;
    for x in v {
        acc = acc ^ x;
    }
    acc
}
