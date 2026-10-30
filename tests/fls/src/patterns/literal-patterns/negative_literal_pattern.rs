//! @fls: fls_pah15qa54irs, fls_COQKJC0dvtNO, fls_m01eo9sa55s
//! @title: Negative integer literal patterns
//! @lean(tier): supported
//! @lean(reason): The negative literal pattern extracts to a Lean `match` alternative that is not a constructor pattern, and the extraction fails to elaborate (`Failed to compile pattern matching: Stuck at ... (-1)#iscalar`); the control flow section of the input-language reference.

pub fn sign_name(n: i32) -> u8 {
    match n {
        -1 => 1,
        0 => 2,
        _ => 0,
    }
}

#[hax_test::equiv]
pub fn negative_literal_matches() -> bool {
    sign_name(-1) == 1 && sign_name(0) == 2 && sign_name(5) == 0
}
