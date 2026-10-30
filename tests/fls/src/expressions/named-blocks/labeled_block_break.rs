//! @fls: fls_B4NBv2jfZLuy, fls_J8wJNfcSAYrS, fls_YxvAUUYAPkaq, fls_kKZPKvJ902cw, fls_ltEygvWDtHXE
//! @title: A labeled block expression yielding its break value
//! @lean(tier): supported

pub fn classify(x: u32) -> u32 {
    let v = 'blk: {
        if x == 0 {
            break 'blk 100;
        }
        if x > 10 {
            break 'blk 200;
        }
        x
    };
    v
}

#[hax_test::equiv]
pub fn break_values() -> bool {
    classify(0) == 100 && classify(5) == 5 && classify(50) == 200
}
