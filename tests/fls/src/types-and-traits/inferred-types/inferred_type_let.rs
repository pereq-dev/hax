//! @fls: fls_xdtgr5toulpb, fls_9d8wbugmar1m
//! @title: The inferred type placeholder in a let statement
//! @lean(tier): supported

pub fn widen(x: u8) -> u32 {
    let y: _ = x;
    let z: Option<_> = Some(y as u32);
    match z {
        Some(v) => v,
        None => 0,
    }
}

#[hax_test::equiv]
pub fn inferred_from_initializer() -> bool {
    widen(200) == 200
}
