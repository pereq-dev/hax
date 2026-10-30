//! @fls: fls_1e8qer6bh2f3, fls_2i4fbxbbvpf1, fls_5azwlk7hav1k, fls_5udx9zyeg5ga, fls_67l4j48n6p7o, fls_e8gd5lzcaifw, fls_i9sxf2q5jjqt, fls_iv9t4nfs4f6w, fls_p5pjxk5xfcbx, fls_w7lq4dkoyuf7, fls_yhlyzef9h97q
//! @title: If expressions with else and else-if branches
//! @lean(tier): supported

pub fn sign(x: i32) -> i32 {
    if x < 0 {
        -1
    } else if x == 0 {
        0
    } else {
        1
    }
}

pub fn clamp_to_ten(x: &mut u32) {
    if *x > 10 {
        *x = 10;
    }
}

#[hax_test::equiv]
pub fn branch_values() -> bool {
    let mut big = 50;
    let mut small = 3;
    clamp_to_ten(&mut big);
    clamp_to_ten(&mut small);
    sign(-4) == -1 && sign(0) == 0 && sign(9) == 1 && big == 10 && small == 3
}
