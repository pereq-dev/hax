//! @fls: fls_qqiu594hki8g, fls_r78zzw7yyg34, fls_9yuobz1jsehf, fls_x10AKxoXXbs8, fls_69bnxrtj0nar, fls_zAdtysiuUwBX, fls_SezcYXcSlEq7, fls_6xRXEt2pGnZi, fls_w1pw40phsv2o, fls_zgoke73xrhk3
//! @title: Slice patterns of several lengths over a slice
//! @lean(tier): supported
//! @lean(reason): aeneas fails on the length check of a slice pattern over a slice (`Inconsistent projection: pe: PtrMetadata`); the control flow section of the input-language reference.

pub fn describe(s: &[u32]) -> u32 {
    match s {
        [] => 0,
        [only] => *only,
        [first, .., last] => first + last,
    }
}

#[hax_test::equiv]
pub fn slice_matches_by_length() -> bool {
    describe(&[]) == 0 && describe(&[7]) == 7 && describe(&[1, 5, 9]) == 10
}
