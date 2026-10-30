//! @fls: fls_2d4aqspw0wlt, fls_msjo2zd67zn1, fls_w1oti03tm1y6, fls_9gqd7eevbknt, fls_r4o2rmhqg4br, fls_1n7p6ij1dpm, fls_2j659ns8wop4, fls_me6bf9m2ypt, fls_d09l2rl0161l, fls_7nd5tixyqir8
//! @title: The ordering operations on bool
//! @lean(tier): supported
//! @lean(reason): The ordering operators on `bool` extract to Lean propositions on `Bool` without a decision procedure (`failed to synthesize Decidable ((a ≥ b) = b1)`), so the extraction fails to elaborate; the types section of the input-language reference.

pub fn gt(a: bool, b: bool) -> bool {
    a > b
}

pub fn derived(a: bool, b: bool) -> bool {
    ((a < b) == !(a >= b)) && ((a >= b) == (a == b || a > b)) && ((a <= b) == (a == b || a < b))
}

#[hax_test::equiv]
pub fn gt_table() -> bool {
    !gt(true, true) && gt(true, false) && !gt(false, true) && !gt(false, false)
}

#[hax_test::equiv]
pub fn derived_comparisons() -> bool {
    derived(false, false) && derived(false, true) && derived(true, false) && derived(true, true)
}
