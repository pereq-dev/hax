//! @fls: fls_il9a1olqmu38, fls_gbdd37seqoab
//! @title: A zero-variant enum has no values
//! @lean(tier): supported

pub enum Never {}

pub fn from_never(n: Never) -> u32 {
    match n {}
}

pub fn unwrap_or_zero(x: Result<u32, Never>) -> u32 {
    match x {
        Ok(v) => v,
        Err(n) => from_never(n),
    }
}

#[hax_test::equiv]
pub fn only_ok_is_inhabited() -> bool {
    unwrap_or_zero(Ok(3)) == 3
}
