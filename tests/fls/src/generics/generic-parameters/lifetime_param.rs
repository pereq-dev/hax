//! @fls: fls_s0nrjwqg2wox, fls_ixmgqupxvf73
//! @title: Function with an explicit lifetime parameter
//! @lean(tier): supported
//! @no-equiv: Lifetimes are erased by the value model and have no observable value.

pub fn pick_first<'a>(x: &'a u32, _y: &'a u32) -> &'a u32 {
    x
}
