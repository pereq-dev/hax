//! @fls: fls_7kpepal8ghuj, fls_t7i4n19qdgn4, fls_np8aghofjqhm, fls_qo0itr5il1kk, fls_aa4f3rvir9lm, fls_quzvhzpr0124, fls_tnh7o3pb4e22, fls_yymgpyi67dty, fls_knjruq5wppv, fls_utgjx6l5zwfl, fls_dm0xr424ine1
//! @title: Items and fields with every visibility modifier
//! @lean(tier): supported

pub mod outer {
    pub mod inner {
        pub fn public() -> u32 {
            1
        }
        pub(crate) fn crate_visible() -> u32 {
            2
        }
        pub(super) fn parent_visible() -> u32 {
            3
        }
        // rustfmt would shorten the path to `pub(super)`.
        #[rustfmt::skip]
        pub(in super) fn path_visible() -> u32 {
            4
        }
        pub(self) fn module_private() -> u32 {
            5
        }
        fn private() -> u32 {
            6
        }
        pub fn sum() -> u32 {
            module_private() + private()
        }
        pub struct Mixed {
            pub shown: u32,
            hidden: u32,
        }
        impl Mixed {
            pub fn new(shown: u32) -> Mixed {
                Mixed {
                    shown,
                    hidden: shown * 2,
                }
            }
            pub fn hidden(&self) -> u32 {
                self.hidden
            }
        }
        pub enum Choice {
            A(u32),
        }
    }
    pub fn from_parent() -> u32 {
        inner::parent_visible() + inner::path_visible()
    }
}

#[hax_test::equiv]
pub fn visible_items_compute() -> bool {
    let m = outer::inner::Mixed::new(5);
    let outer::inner::Choice::A(v) = outer::inner::Choice::A(9);
    outer::inner::public()
        + outer::inner::crate_visible()
        + outer::from_parent()
        + outer::inner::sum()
        == 21
        && m.shown == 5
        && m.hidden() == 10
        && v == 9
}
