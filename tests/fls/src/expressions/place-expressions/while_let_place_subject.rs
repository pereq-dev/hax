//! @fls: fls_5yXuTLQOQ3cc, fls_9r7dopqf1nzl
//! @title: A place expression as the subject of a while let loop
//! @lean(tier): supported
//! @lean(reason): aeneas rejects the `Some(v)` pattern on the indexed place (`Not allowed to expand enumerations with several variants`); the control flow section of the input-language reference.

pub fn sum_until_none(queue: &[Option<u32>; 3]) -> u32 {
    let mut total = 0;
    let mut i = 0;
    while let Some(v) = queue[i] {
        total += v;
        i += 1;
    }
    total
}

#[hax_test::equiv]
pub fn place_subject_is_read() -> bool {
    sum_until_none(&[Some(1), Some(2), None]) == 3
}
