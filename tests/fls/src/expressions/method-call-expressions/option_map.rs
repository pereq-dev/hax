//! @fls: fls_b7i26954j1hc, fls_ljvj1f9fv085
//! @title: Method call on a core type through its model
//! @lean(tier): supported

pub fn double(x: Option<i32>) -> Option<i32> {
    x.map(|v| v * 2)
}

#[hax_test::equiv]
pub fn maps_some() -> bool {
    double(Some(21)) == Some(42)
}

#[hax_test::equiv]
pub fn maps_none() -> bool {
    double(None) == None
}
