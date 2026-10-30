//! @fls: fls_1bh2alh37frz, fls_2lrzrtjhsdes, fls_FkxLf91WKiIo, fls_bmTjhKdpfgCB, fls_fo6Aa6Td6rMA, fls_kuxo0on3vit6
//! @title: For loops over a range and over an array
//! @lean(tier): supported

pub fn sum_to(n: u32) -> u32 {
    let mut acc = 0;
    for i in 0..n {
        acc += i;
    }
    acc
}

pub fn sum_array(xs: [u32; 3]) -> u32 {
    let mut acc = 0;
    for x in xs {
        acc += x;
    }
    acc
}

#[hax_test::equiv]
pub fn iterated_sums() -> bool {
    sum_to(5) == 10 && sum_to(0) == 0 && sum_array([1, 2, 3]) == 6
}
