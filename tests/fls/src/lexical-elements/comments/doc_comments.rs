//! @fls: fls_8obn3dtzpe5f, fls_7n6d3jx61ose, fls_KZp0yiFLTqxb, fls_RYVL9KgaxKvl, fls_scko7crha0um, fls_63gzofa9ktic, fls_tspijl68lduc, fls_k3hj30hjkdhw, fls_nWtKuPi8Fw6v, fls_ok0zvo9vcmzo
//! @title: Outer and inner line and block doc comments on items
//! @lean(tier): supported

/// An outer line doc.
pub fn line_documented() -> u32 {
    1
}

/** An outer block doc. */
pub fn block_documented() -> u32 {
    2
}

pub mod inner {
    //! An inner line doc.
    /*! An inner block doc. */

    pub fn value() -> u32 {
        3
    }
}

#[hax_test::equiv]
pub fn documented_items_extract() -> bool {
    line_documented() + block_documented() + inner::value() == 6
}
