//! @fls: fls_1f5ibdkz3l51, fls_1l3rgtm6o54s, fls_41n6z85h1z47, fls_48br7odx6nke, fls_9qcm0dx9rolw, fls_bezkcuwp5qol, fls_cwt7afsbgs7w, fls_d7x817v8xzea, fls_djphr5mk0t6f, fls_f39rzauxrlcl, fls_f6in3h5cj8i6, fls_hish3qfmawd, fls_hr4kokysrjop, fls_hxa1avitfvrq, fls_jtdnf0vmn6xt, fls_k3ohh8k888c, fls_mk2yk99p6nvp, fls_qurz25skmryg, fls_ruy7e6yccsqk, fls_sif2aqky96j6, fls_sy2xzzq06i0x, fls_t1zqnab8a752, fls_ya23jjg5wjl, fls_ywqn5nixelkz
//! @title: Operator precedence and associativity in mixed expressions
//! @lean(tier): supported

pub struct P {
    pub v: u32,
}

pub fn double(x: u32) -> u32 {
    x * 2
}

pub fn mixed(a: u32, b: u32, c: u32) -> [u32; 6] {
    let p = P { v: a };
    let arr = [a, b, c];
    [
        a + b * c,
        (a + b) * c,
        a - b - c,
        a << 1 + 1,
        double(p.v) + arr[1] * (c as i32) as u32,
        a & b | c ^ a,
    ]
}

pub fn comparisons_and_lazy(a: u32, b: u32) -> bool {
    a < b && b < 10 || a == 0 && !(b == 0)
}

#[hax_test::equiv]
pub fn precedence_results() -> bool {
    mixed(8, 3, 2) == [14, 22, 3, 32, 22, 10]
        && comparisons_and_lazy(1, 2)
        && !comparisons_and_lazy(5, 2)
        && comparisons_and_lazy(0, 5)
}
