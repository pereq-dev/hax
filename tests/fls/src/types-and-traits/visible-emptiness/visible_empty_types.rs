//! @fls: fls_SD4yUEQ9hHa3, fls_GeoneCP5TYwf, fls_AXOtKdSQR4AF, fls_TE2GBnZX2YFW, fls_ZfnYORORz40y, fls_R65LmGTvSlIm, fls_fSNrRsgzLd0E, fls_GNFzLMH8RBUC, fls_S9QL6yVF5LFI, fls_A2W4v53ihTGx
//! @title: Visibly empty enums, structs, and tuples make their match arms unnecessary
//! @lean(tier): supported

pub enum Void {}

pub struct Wrapped(pub Void);

pub enum Outcome {
    Value(u32),
    Impossible(Wrapped),
}

pub fn value(o: Outcome) -> u32 {
    match o {
        Outcome::Value(v) => v,
        Outcome::Impossible(Wrapped(v)) => match v {},
    }
}

pub fn unwrap_pair(x: Result<u32, (u8, Void)>) -> u32 {
    match x {
        Ok(v) => v,
        Err((_, v)) => match v {},
    }
}

#[hax_test::equiv]
pub fn inhabited_arms_only() -> bool {
    value(Outcome::Value(2)) == 2 && unwrap_pair(Ok(3)) == 3
}
