//! @fls: fls_nne91at3143t, fls_vbclxg9dq4yo, fls_gcszhqg6hnva
//! @title: Lifetime annotations on references and a struct field
//! @lean(tier): supported

pub struct View<'a> {
    pub data: &'a [u32],
}

pub fn first<'a>(v: &View<'a>) -> &'a u32 {
    &v.data[0]
}

pub fn bounded<'a, 'b: 'a, T: 'b>(x: &'b T) -> &'a T {
    x
}

#[hax_test::equiv]
pub fn annotated_references() -> bool {
    let data = [4u32, 5];
    let v = View { data: &data };
    *first(&v) == 4 && *bounded(&data[1]) == 5
}
