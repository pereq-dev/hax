//! @fls: fls_rpbhr0xukbx9, fls_8uWfFAsZeRCs, fls_bYWfGDAQcWfA
//! @title: Reading and writing through raw pointers
//! @lean(tier): unsupported
//! @lean(reason): The value-model boundary of the input-language reference: raw pointers reason about addresses, and there are no addresses in the value model.

pub fn read_through_const(x: &u32) -> u32 {
    let p: *const u32 = x;
    unsafe { *p }
}

pub fn write_through_mut(x: &mut u32, v: u32) {
    let p: *mut u32 = x;
    unsafe { *p = v };
}

#[hax_test::equiv]
pub fn round_trip() -> bool {
    let mut x = 1;
    write_through_mut(&mut x, 9);
    read_through_const(&x) == 9
}
