//! @fls: fls_7x9pi2o7pee7, fls_tsqvr3fmcel, fls_wcf5mxrzbujn, fls_fckoj1jh5mrc, fls_8kqb8754e6p4
//! @title: Shared and mutable references active until their last use
//! @lean(tier): supported

pub fn use_references() -> u32 {
    let mut value = 1;
    let shared = &value;
    let read = *shared;
    let exclusive = &mut value;
    *exclusive += read;
    let again = &value;
    *again * 10
}

#[hax_test::equiv]
pub fn referents_are_read_and_written() -> bool {
    use_references() == 20
}
