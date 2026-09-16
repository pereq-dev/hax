//! @fls: fls_fHDXn6CafgU7, fls_3szBrTEI2Rzo, fls_ftE05Blr5Esv, fls_csdjfwczlzfd, fls_jriT46yWgIR0, fls_jnCncQ9PpqWf, fls_qpARFqqnKGMn
//! @title: Shared and mutable references as thin and fat pointer values
//! @lean(tier): supported

pub fn sum(thin: &u32, fat: &[u32]) -> u32 {
    let copy = thin;
    let mut total = *thin + *copy;
    let mut i = 0;
    while i < fat.len() {
        total += fat[i];
        i += 1;
    }
    total
}

pub fn set(target: &mut u32) {
    *target = 7;
}

#[hax_test::equiv]
pub fn reference_values() -> bool {
    let mut x = 1;
    {
        let shared = &x;
        let _released = *shared;
    }
    set(&mut x);
    sum(&x, &[1, 2]) == 17
}
