//! @fls: fls_gn1ngtx2tp2s, fls_927nfm5mkbsp, fls_bHwy8FLzEUi3, fls_bdx9gnnjxru3, fls_owdlsaaygtho, fls_uwuthzfgslif, fls_AAYJDCNMJgTq, fls_xCSsxYUZUFed, fls_kcAbTPZXQ5Y8, fls_EqJb3Jl3vK8K, fls_C7dvzcXcpQCy, fls_J8X8ahnJLrMo, fls_vljy4mm0zca2
//! @title: Function definitions with parameters, patterns, and explicit or unit return types
//! @lean(tier): supported

pub fn add((a, b): (u32, u32), c: u32) -> u32 {
    a + b + c
}

pub fn store(target: &mut u32, value: u32) {
    *target = value;
}

#[hax_test::equiv]
pub fn defined_functions() -> bool {
    let mut slot = 0;
    store(&mut slot, add((1, 2), 3));
    slot == 6
}
