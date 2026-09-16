//! @fls: fls_po4gw6t2ptwu, fls_pbc7ktlu0pl, fls_frjv68kqqxfh, fls_qsyorqjkdh2t, fls_aolj6abvp9sa, fls_rf6a2ae3y7vu, fls_n4102qskkmz2
//! @title: Names of the language and core preludes used without imports
//! @lean(tier): supported

pub fn prelude_values(flag: bool, c: char, s: &str) -> Option<u64> {
    let n: u8 = 1;
    let m: i128 = -1;
    let _text: &str = s;
    if flag && c == 'x' {
        Some(n as u64 + (-m) as u64)
    } else {
        None
    }
}

#[derive(Clone, Copy)]
pub struct Marker;

#[hax_test::equiv]
pub fn prelude_entities_resolve() -> bool {
    let _m: Marker = Marker;
    prelude_values(true, 'x', "y") == Some(2) && prelude_values(false, 'x', "y").is_none()
}
