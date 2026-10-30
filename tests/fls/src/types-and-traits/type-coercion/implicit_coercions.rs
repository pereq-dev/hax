//! @fls: fls_w5pjcj9qmgbv, fls_5v0n2a32bk95, fls_h8dkehit8rza, fls_aujb44849tq1, fls_jte6n2js32af, fls_fwy2z11c1sji, fls_sf0c3fbx8z57, fls_iiiu2q7pym4p, fls_j3kbaf43sgpj, fls_sp794uzfiofr, fls_wxrugvlazy6v, fls_apstt4elv2k7, fls_eu4bt3dw1b8c
//! @title: Implicit coercions at let statements, call arguments, results, and fields
//! @lean(tier): supported

pub struct Holder<'a> {
    pub view: &'a [u32],
}

pub fn read(x: &u32) -> u32 {
    *x
}

pub fn first(xs: &[u32]) -> u32 {
    xs[0]
}

pub fn unsized_result(a: &[u32; 2]) -> &[u32] {
    a
}

pub fn never_to_u32(x: Option<u32>) -> u32 {
    match x {
        Some(v) => v,
        None => panic!("no value"),
    }
}

#[hax_test::equiv]
pub fn coercions_preserve_values() -> bool {
    let mut x = 4;
    let through_mut = read(&mut x);
    let shared: &u32 = &mut x;
    let arr = [1u32, 2];
    let boxed = Box::new(9u32);
    let deref: &u32 = &boxed;
    let h = Holder { view: &arr };
    through_mut == 4
        && *shared == 4
        && first(&arr) == 1
        && unsized_result(&arr)[1] == 2
        && *deref == 9
        && h.view.len() == 2
        && never_to_u32(Some(3)) == 3
}
