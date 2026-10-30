//! @fls: fls_8obn3dtzpe5f, fls_nayisy85kyq2, fls_qsbnl11be35s
//! @title: Line comments and nested block comments that hide code
//! @lean(tier): supported

pub fn sum() -> u32 {
    // + 100
    1 /* + 100 */ + 2 /* outer /* + 100 */
    + 100 */
}

#[hax_test::equiv]
pub fn comments_are_ignored() -> bool {
    sum() == 3
}
