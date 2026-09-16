//! @fls: fls_eso51epfofxb, fls_w1pw40phsv2o, fls_x8ylgxrf9ca, fls_bdcv6rwx0fsv
//! @title: A rest pattern bound by an identifier pattern in an array pattern
//! @lean(tier): supported
//! @lean(reason): aeneas has no builtin for the shared range index into an array that the binding of the rest pattern produces (`get_builtin_fun_info: not found: Index { is_array = true; mutability = RShared; is_range = true }`); the control flow section of the input-language reference.

pub fn tail_sum(a: [u32; 4]) -> u32 {
    let [_, rest @ ..] = a;
    rest[0] + rest[1] + rest[2]
}

#[hax_test::equiv]
pub fn bound_rest_is_the_remainder() -> bool {
    tail_sum([1, 2, 3, 4]) == 9
}
