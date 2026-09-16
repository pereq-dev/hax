//! @fls: fls_4e7EE4a8Yvmy, fls_DJxQDBsO9hc7, fls_Ki4FOzJMqtvJ, fls_TS6DvMon5h27, fls_Twkre8IzUa8S, fls_UtjWrE2qeplQ, fls_WlXB0AHifCdd, fls_dTABiwAPGhdZ, fls_gOXUWePymgGV, fls_qQrV8QuGGcVO
//! @title: Immutable and mutable raw borrow expressions
//! @lean(tier): unsupported
//! @lean(reason): The value-model boundary of the input-language reference: the `&raw` borrow produces a raw pointer, and there are no addresses in the value model.

pub fn through_raw(x: &mut u32) -> u32 {
    let p = &raw mut *x;
    let q = &raw const *x;
    unsafe {
        *p = 5;
        *q
    }
}

#[hax_test::equiv]
pub fn raw_round_trip() -> bool {
    let mut x = 1;
    through_raw(&mut x) == 5
}
