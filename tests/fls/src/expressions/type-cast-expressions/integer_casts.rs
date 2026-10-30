//! @fls: fls_4s69s9pcvbn7, fls_76eq3bd6birr, fls_99kvyh4puy57, fls_a6midh2m0w0b, fls_al9f1t7vlsxi, fls_h9sxg3pxn7i2, fls_hhxawo12cndy, fls_jea17f39fmsg, fls_kc3gwj9x3jnr, fls_ldiritt32h2w, fls_le6bchl25ewz, fls_ltioqbhl14g0, fls_otaxe9okhdr1, fls_pcromhosmnf0, fls_shy6e0e30bco, fls_syk2li8ft3rx, fls_uqv32nthva6y, fls_uuayaksl8059, fls_vdxkpvmpwl3s
//! @title: Casts between integer types, from bool, and from an enum discriminant
//! @lean(tier): supported

pub enum Level {
    Low = 1,
    High = 7,
}

pub fn truncate(x: i16) -> u8 {
    x as u8
}

pub fn sign_extend(x: i16) -> i32 {
    x as i32
}

pub fn reinterpret_then_widen(x: i16) -> u32 {
    x as u32
}

pub fn zero_extend(x: i16) -> i64 {
    (x as u16) as i64
}

#[hax_test::equiv]
pub fn truncation_and_extension() -> bool {
    truncate(-2) == 254
        && truncate(300) == 44
        && sign_extend(-2) == -2
        && reinterpret_then_widen(-2) == 4294967294
        && zero_extend(-2) == 65534
        && zero_extend(300) == 300
}

#[hax_test::equiv]
pub fn bool_and_enum_casts() -> bool {
    true as u8 == 1
        && false as u32 == 0
        && Level::Low as u8 == 1
        && Level::High as i32 == 7
        && 7u8 as u8 == 7
}
