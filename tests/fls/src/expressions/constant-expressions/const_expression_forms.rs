//! @fls: fls_3bucpdj828bq, fls_b5fraqx07wuo, fls_fc62yaqyjpl2, fls_fobs8ebt7dhc, fls_idxf02p7jogu, fls_pbpzkfo1fgtz, fls_zcuzhw7qkzkr, fls_to4e7imq2c0w, fls_w4lpq9bs8tsc
//! @title: Constant expressions with ranges, struct expressions, assignments, let conditions, loops, unsafe blocks, dereferences, and method calls
//! @lean(tier): supported

pub struct Pair {
    pub a: u32,
    pub b: u32,
}

impl Pair {
    pub const fn sum(&self) -> u32 {
        self.a + self.b
    }
}

pub const RANGE: core::ops::Range<u32> = 1..4;
pub const PAIR: Pair = Pair { a: 2, b: 5 };
pub const SUM: u32 = PAIR.sum();
pub const DEREF: u32 = *&PAIR.a;
pub const LOOPED: u32 = {
    let mut acc = 0;
    acc = acc + RANGE.start;
    let mut i = 0;
    loop {
        if i == 3 {
            break;
        }
        acc += 1;
        i += 1;
    }
    acc
};
pub const LET_FORMS: u32 = {
    let mut opt = Some(3);
    let mut total = 0;
    if let Some(x) = opt {
        total += x;
    }
    while let Some(x) = opt {
        total += x;
        opt = if x > 1 { Some(x - 1) } else { None };
    }
    total
};
#[allow(unused_unsafe)]
pub const UNSAFE: u32 = unsafe { 7 };

#[hax_test::equiv]
pub fn constants_evaluate() -> bool {
    RANGE.end - RANGE.start == 3
        && SUM == 7
        && DEREF == 2
        && LOOPED == 4
        && LET_FORMS == 9
        && UNSAFE == 7
}
