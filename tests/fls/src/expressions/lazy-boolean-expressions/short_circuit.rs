//! @fls: fls_40jya46h62yi, fls_gpbvus89iy4c, fls_jugckad775kq, fls_k8u77ow5bb6c, fls_p0rafjsridre, fls_srfv1d4idxy9, fls_tflikh8cmxvc, fls_tmfmu3syxp2q, fls_ufre0ko2cwh2, fls_xdgvrd58nkoa, fls_yffozo2vq5xz, fls_yg1348rlziw3, fls_zas0lizgq2hn
//! @title: Short-circuit evaluation of lazy and and lazy or
//! @lean(tier): supported

pub fn touch(counter: &mut u32, value: bool) -> bool {
    *counter += 1;
    value
}

pub fn evaluate(a: bool, b: bool, counter: &mut u32) -> [bool; 2] {
    let and = touch(counter, a) && touch(counter, b);
    let or = touch(counter, a) || touch(counter, b);
    [and, or]
}

#[hax_test::equiv]
pub fn right_operand_only_when_needed() -> bool {
    let mut counter = 0;
    let first = evaluate(false, true, &mut counter);
    let second = evaluate(true, false, &mut counter);
    let third = evaluate(true, true, &mut counter);
    first == [false, true] && second == [false, true] && third == [true, true] && counter == 9
}
