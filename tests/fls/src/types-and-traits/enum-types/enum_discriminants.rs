//! @fls: fls_gbdd37seqoab, fls_wQTFwl88VujQ, fls_t4yeovFm83Wo, fls_w7sggezgq9o4, fls_93l5o6qar5p2, fls_t36rk3wikq28, fls_8ajw5trd23wi, fls_pijczoq4k9ij, fls_duqbzvpuehvv, fls_x7nh42on06bg
//! @title: Enum variants with implicit and explicit discriminants
//! @lean(tier): supported

pub enum Level {
    Low,
    Mid = 5,
    High,
}

#[repr(u8)]
pub enum Small {
    A = 200,
    B,
}

#[hax_test::equiv]
pub fn implicit_and_explicit_discriminants() -> bool {
    Level::Low as isize == 0 && Level::Mid as isize == 5 && Level::High as isize == 6
}

#[hax_test::equiv]
pub fn repr_discriminants() -> bool {
    Small::A as u8 == 200 && Small::B as u8 == 201
}
