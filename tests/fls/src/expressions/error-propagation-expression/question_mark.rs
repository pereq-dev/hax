//! @fls: fls_1nnhjcgy8kdh, fls_8df018q7y6g, fls_8q59wbumrt5s, fls_a09614kgsspt, fls_ab4vhq4nwn7f, fls_alk4qvfprnvy, fls_z4zikxy2b1em
//! @title: The question mark operator on Result and Option
//! @lean(tier): supported

pub fn parse_digit(c: u8) -> Result<u32, u8> {
    if c >= b'0' && c <= b'9' {
        Ok((c - b'0') as u32)
    } else {
        Err(c)
    }
}

pub fn two_digits(a: u8, b: u8) -> Result<u32, u8> {
    let high = parse_digit(a)?;
    let low = parse_digit(b)?;
    Ok(high * 10 + low)
}

pub fn first_plus_one(xs: &[u32]) -> Option<u32> {
    let first = xs.first()?;
    Some(*first + 1)
}

#[hax_test::equiv]
pub fn propagated_and_unwrapped() -> bool {
    two_digits(b'4', b'2') == Ok(42)
        && two_digits(b'4', b'x') == Err(b'x')
        && first_plus_one(&[1]) == Some(2)
        && first_plus_one(&[]) == None
}
