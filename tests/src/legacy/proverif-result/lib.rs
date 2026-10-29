//! @off: fstar, lean, coq, ssprove, legacy-lean
//!
//! The default Result model: a `Result` is its `Ok` payload, producing an
//! `Err` ends the trace, and arms that need an `Err` are dropped. Code that
//! goes on after an `Err` is rejected. `proverif-result-errors` extracts the
//! same items with `--result-errors`.

#[derive(PartialEq)]
enum Error {
    Invalid,
    Other,
}

fn check(x: u8) -> Result<u8, Error> {
    if x == 0 { Err(Error::Invalid) } else { Ok(x) }
}

fn question_mark(x: u8) -> Result<u8, Error> {
    let y = check(x)?;
    Ok(y)
}

/// @fail(extraction): proverif(HAX0001, HAX0001)
fn err_arms_first(r: Result<u8, Error>) -> u8 {
    match r {
        Err(Error::Invalid) => 1,
        Err(_) => 2,
        Ok(v) => v,
    }
}

/// @fail(extraction): proverif(HAX0001)
fn nested_err(o: Option<Result<u8, Error>>) -> u8 {
    match o {
        Some(Err(_)) => 0,
        Some(Ok(v)) => v,
        None => 1,
    }
}

/// @fail(extraction): proverif(HAX0001)
fn is_ok(r: Result<u8, Error>) -> bool {
    r.is_ok()
}

fn map_err(r: Result<u8, u8>) -> Result<u8, u16> {
    r.map_err(|e| e as u16)
}

// `transpose` swaps the `Option` and the `Result`.
fn transpose(x: Option<Result<u8, u8>>) -> Result<Option<u8>, u8> {
    x.transpose()
}

// Going on after an `Err`.

/// @fail(extraction): proverif(HAX0001)
fn if_let_else(r: Result<u8, Error>) -> u8 {
    if let Ok(v) = r { v } else { 0 }
}

/// @fail(extraction): proverif(HAX0001)
fn wildcard_arm(r: Result<u8, Error>) -> u8 {
    match r {
        Ok(v) => v,
        _ => 0,
    }
}

/// @fail(extraction): proverif(HAX0001)
fn discarded(x: u8) -> u8 {
    let _ = check(x);
    x
}

/// @fail(extraction): proverif(HAX0001)
fn unused_param(_r: Result<u8, Error>, x: u8) -> u8 {
    x
}

/// @fail(extraction): proverif(HAX0001)
fn used_on_one_path(c: bool, x: u8) -> Result<u8, Error> {
    let r = check(x);
    if c { r } else { Ok(0) }
}

// Propagating an `Err`.

fn propagating_arms(r: Result<u8, Error>) -> Result<u8, Error> {
    match r {
        Ok(v) => Ok(v + 1),
        Err(e) => Err(e),
    }
}

fn panicking_arm(r: Result<u8, Error>) -> u8 {
    match r {
        Ok(v) => v,
        Err(_) => panic!(),
    }
}

fn wildcard_err(r: Result<u8, Error>) -> Result<u8, Error> {
    match r {
        Ok(v) => Ok(v),
        _ => Err(Error::Other),
    }
}

fn bound_then_propagated(x: u8) -> Result<u8, Error> {
    let r = check(x);
    let v = r?;
    Ok(v)
}

fn in_tuple(x: u8) -> (u8, Result<u8, Error>) {
    (x, check(x))
}

fn from_tuple(x: u8) -> Result<u8, Error> {
    let (y, r) = in_tuple(x);
    let v = r?;
    Ok(y + v)
}

fn passed(x: u8) -> Result<u8, Error> {
    question_mark_of(check(x))
}

fn question_mark_of(r: Result<u8, Error>) -> Result<u8, Error> {
    let v = r?;
    Ok(v)
}

// Errors of hand-written models.

#[hax_lib::proverif::replace_body("hax_result_ok(x)")]
fn modelled(x: u8) -> Result<u8, Error> {
    check(x)
}

/// @fail(extraction): proverif(HAX0001)
fn match_any(x: u8) -> u8 {
    match modelled(x) {
        Ok(v) => v,
        Err(_) => 0,
    }
}

/// @fail(extraction): proverif(HAX0001, HAX0001)
fn match_variant(x: u8) -> u8 {
    match modelled(x) {
        Ok(v) => v,
        Err(Error::Invalid) => 1,
        Err(_) => 2,
    }
}

fn forwarded(x: u8) -> Result<u8, Error> {
    let v = modelled(x)?;
    Ok(v)
}

/// @fail(extraction): proverif(HAX0001, HAX0001)
fn match_forwarded(x: u8) -> u8 {
    match forwarded(x) {
        Ok(v) => v,
        Err(Error::Invalid) => 1,
        Err(Error::Other) => 2,
    }
}

fn error_code(e: Error) -> u8 {
    match e {
        Error::Invalid => 1,
        Error::Other => 2,
    }
}

/// @fail(extraction): proverif(HAX0001)
fn inspected(x: u8) -> u8 {
    match modelled(x) {
        Ok(v) => v,
        Err(e) => error_code(e),
    }
}

#[hax_lib::opaque]
fn opaque_result(x: u8) -> Result<u8, Error> {
    check(x)
}

fn use_opaque(x: u8) -> Result<u8, Error> {
    let v = opaque_result(x)?;
    Ok(v)
}

#[hax_lib::pv_constructor]
fn encode(x: u8) -> u8 {
    x
}

#[hax_lib::pv_inverse_of(encode)]
fn decode(b: u8) -> Result<u8, Error> {
    check(b)
}

fn round_trip(x: u8) -> Result<u8, Error> {
    decode(encode(x))
}

/// @fail(extraction): proverif(HAX0001)
fn decode_or_zero(b: u8) -> u8 {
    match decode(b) {
        Ok(v) => v,
        Err(_) => 0,
    }
}

// Observable calls while an `Err` is pending.

#[hax_lib::proverif::replace_body("x")]
fn record(x: u8) -> u8 {
    x
}

/// @fail(extraction): proverif(HAX0001)
fn err_arm_effect(r: Result<u8, Error>) -> Result<u8, Error> {
    match r {
        Ok(v) => Ok(v),
        Err(e) => {
            record(0);
            Err(e)
        }
    }
}

/// @fail(extraction): proverif(HAX0001)
fn effect_before_propagation(x: u8) -> Result<u8, Error> {
    let r = check(x);
    record(x);
    let v = r?;
    Ok(v)
}

/// @fail(extraction): proverif(HAX0001)
fn effect_after_result(x: u8) -> (Result<u8, Error>, u8) {
    (check(x), record(x))
}

fn effect_after_propagation(x: u8) -> Result<u8, Error> {
    let v = check(x)?;
    Ok(record(v))
}

struct Out {
    n: u8,
    r: Result<u8, Error>,
}

fn produce(x: u8) -> Out {
    Out { n: x, r: check(x) }
}

/// @fail(extraction): proverif(HAX0001)
fn rest_pattern(x: u8) -> u8 {
    let Out { n, .. } = produce(x);
    n
}

fn all_fields(x: u8) -> Result<u8, Error> {
    let Out { n, r } = produce(x);
    let v = r?;
    Ok(n + v)
}

// Error values of hand-written models, kept through other values.

/// @fail(extraction): proverif(HAX0001, HAX0001)
fn equal_err(x: u8) -> u8 {
    if modelled(x) == Err(Error::Invalid) { 1 } else { 0 }
}

/// @fail(extraction): proverif(HAX0001, HAX0001)
fn repacked(x: u8) -> u8 {
    let swapped: Result<Error, ()> = match modelled(x) {
        Ok(_) => Err(()),
        Err(e) => Ok(e),
    };
    match swapped {
        Ok(Error::Invalid) => 1,
        Ok(Error::Other) => 2,
        Err(_) => 3,
    }
}

#[hax_lib::pv_extern]
fn derive(k: u8) -> Result<u8, Error> {
    check(k)
}

fn use_derive(k: u8) -> Result<u8, Error> {
    let v = derive(k)?;
    Ok(v)
}

#[hax_lib::proverif::replace_body("crypto__hash(x)")]
fn hashed(x: u8) -> Result<u8, Error> {
    check(x)
}

fn use_hashed(x: u8) -> Result<u8, Error> {
    let v = hashed(x)?;
    Ok(v)
}

enum Wrapped {
    E(Error),
}

fn map_err_constructor(x: u8) -> Result<u8, Wrapped> {
    let v = check(x).map_err(Wrapped::E)?;
    Ok(v)
}
