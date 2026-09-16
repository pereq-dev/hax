//! @fls: fls_97v4fnekrRXI
//! @title: A block expression without a tail expression typed by its break expressions
//! @lean(tier): supported

pub fn pick(flag: bool) -> u32 {
    'b: {
        if flag {
            break 'b 1;
        }
        break 'b 2;
    }
}

#[hax_test::equiv]
pub fn block_takes_break_value() -> bool {
    pick(true) == 1 && pick(false) == 2
}
