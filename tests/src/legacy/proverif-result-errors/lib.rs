//! @fail(extraction): proverif(HAX0001)
//! @off: fstar, lean, coq, ssprove, legacy-lean
//! @cli: into proverif --result-errors
//!
//! The `--result-errors` Result model: `Ok` and `Err` are constructors and
//! every arm is kept. `proverif-result` extracts the same items with the
//! default model.

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

fn err_arms_first(r: Result<u8, Error>) -> u8 {
    match r {
        Err(Error::Invalid) => 1,
        Err(_) => 2,
        Ok(v) => v,
    }
}

fn nested_err(o: Option<Result<u8, Error>>) -> u8 {
    match o {
        Some(Err(_)) => 0,
        Some(Ok(v)) => v,
        None => 1,
    }
}

fn is_ok(r: Result<u8, Error>) -> bool {
    r.is_ok()
}

fn map_err(r: Result<u8, u8>) -> Result<u8, u16> {
    r.map_err(|e| e as u16)
}

// The model of a std fallible conversion always succeeds, so under
// `--result-errors` its error path would be lost: rejected.
fn narrow(x: u32) -> Result<u8, core::num::TryFromIntError> {
    x.try_into()
}

// `transpose` swaps the `Option` and the `Result`.
fn transpose(x: Option<Result<u8, u8>>) -> Result<Option<u8>, u8> {
    x.transpose()
}
