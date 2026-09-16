//! @fls: fls_42ijvuqqqlvh, fls_6sgj0ltt21i, fls_98qeczwv7fmy, fls_X9kdEAPTqsAe, fls_e5l4y3dy69xi, fls_ehamppbq4gmg, fls_esvpmh6razg3, fls_ff3sgpldn52o, fls_fouu0z3jwoad, fls_fza3omn8yw7s, fls_i68oxj659hc1, fls_issaykiuha75, fls_nw705fpon79b, fls_pc0c22asgzvw, fls_u9sl7h4i8hqu, fls_w96p9oyv5mqt, fls_y3sduoma6q9v
//! @title: Index expressions reading and writing array and slice elements
//! @lean(tier): supported

pub fn third(xs: &[u32]) -> u32 {
    xs[2]
}

pub fn bump_second(xs: &mut [u32; 3]) {
    xs[1] += 10;
}

#[hax_test::equiv]
pub fn indexed_elements() -> bool {
    let mut arr = [4, 5, 6];
    bump_second(&mut arr);
    third(&arr) == 6 && arr[1] == 15 && arr[0] == 4
}
