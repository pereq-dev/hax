//! @fls: fls_1kj8nlc5eb8a, fls_2gto5kp9bjw8, fls_2l9objtb23zn, fls_3y69y9ga4at7, fls_5cs68nm54l31, fls_81jf78m5uga4, fls_aj6tbe54v5jl, fls_f3izbkm8607z, fls_fwtd3b10veiw, fls_guop34ayjw2, fls_izlpt6100gvk, fls_p7hcqryal5cm, fls_t52in1kkyli3, fls_wmsekin1gd2y, fls_ya9res33oxt6
//! @title: Array element constructors and array repetition constructors
//! @lean(tier): supported

pub const NINE: [u32; 9] = [42; 9];

pub fn grid() -> [[u8; 2]; 2] {
    [[1, 2], [3, 4]]
}

pub fn repeated(n: u32) -> [u32; 3] {
    [n * 2; 3]
}

#[hax_test::equiv]
pub fn constructed_arrays() -> bool {
    let g = grid();
    NINE[8] == 42 && NINE.len() == 9 && g[1][0] == 3 && repeated(5) == [10, 10, 10]
}
