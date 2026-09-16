//! @fls: fls_bn7wmf681ngt
//! @title: A heterogeneous tuple and its fields
//! @lean(tier): supported

pub fn pair(n: u32, flag: bool) -> (u32, bool, i8) {
    (n, flag, -1)
}

#[hax_test::equiv]
pub fn tuple_fields() -> bool {
    let t = pair(7, true);
    t.0 == 7 && t.1 && t.2 == -1
}

#[hax_test::equiv]
pub fn unit_tuple() -> bool {
    let u: () = ();
    u == ()
}
