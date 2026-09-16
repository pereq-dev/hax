//! @fls: fls_bi82rusji8g0, fls_1gc436ee1nzm, fls_5xw4opkbxhsc, fls_8sfjw83irpre, fls_lh9my7g8oflq, fls_livflk52xaj9, fls_vj213j9bj61y
//! @title: Inclusive range expressions constructing RangeInclusive and RangeToInclusive
//! @lean(tier): supported
//! @lean(reason): The Lean core model has no `Iterator` instance for `RangeInclusive` (`Unknown constant CoreModels.core.ops.range.RangeInclusive.Insts.CoreIterTraitsIteratorIterator.next`); the model-trust section of the input-language reference.

use core::ops::{RangeInclusive, RangeToInclusive};

pub fn inclusive_sum(lo: u32, hi: u32) -> u32 {
    let r: RangeInclusive<u32> = lo..=hi;
    let to: RangeToInclusive<u32> = ..=hi;
    let mut acc = 0;
    for i in r {
        acc += i;
    }
    acc + to.end
}

#[hax_test::equiv]
pub fn inclusive_bounds() -> bool {
    inclusive_sum(1, 3) == 9
}
