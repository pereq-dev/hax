//! @fls: fls_2eheo4yo2orm, fls_ajbdn54qe6wc, fls_dd62w8c9n9sd, fls_du5eybf8mocy, fls_fa14yfvxsbx3, fls_g80a92tr2ser, fls_hj6srmzbobid, fls_jqu2u6mdccgi, fls_l4u5hhw8tnvs, fls_ll6h6qcaos65, fls_n7nuj1lvpspc, fls_q90ikfi7ewoi, fls_qb8u5skn8bc4, fls_u0iqhbw37xvq, fls_uydzlfc4hjbx, fls_vqb89rkkjw81, fls_vqj7ljrrd7wi, fls_wsfhd3udt6fq, fls_z8c3b9s9de3x, fls_1smb3tj9pxsq, fls_1tsdlpgkgb2u, fls_hier3b8knpuq, fls_horl3qcfdb0k, fls_oqj7s9fi3j3j, fls_skopz71arbwa, fls_vxrg6preh46x, fls_yso6dmog0an2, fls_pydmv629vfuu
//! @title: Destructuring assignment to tuples, arrays, structs, and underscore places
//! @lean(tier): supported

pub struct P {
    pub x: u32,
    pub y: u32,
}

pub struct W(pub u32);

pub fn swap_and_split() -> [u32; 6] {
    let (mut a, mut b) = (1, 2);
    (a, b) = (b, a);
    let mut first = 0;
    let mut rest_last = 0;
    [first, .., rest_last] = [10, 20, 30];
    let mut x = 0;
    let mut y = 0;
    P { x, y } = P { x: 7, y: 8 };
    let mut w = 0;
    W(w) = W(9);
    _ = a + b;
    [a, b, first, rest_last, x + y, w]
}

#[hax_test::equiv]
pub fn assigned_by_destructuring() -> bool {
    swap_and_split() == [2, 1, 10, 30, 15, 9]
}
