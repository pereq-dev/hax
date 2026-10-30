//! @fls: fls_1223lwh4nq49, fls_1r29rtnjlkql, fls_2j132xueobfv, fls_361q9ljc6ybz, fls_a243nclqqjlu, fls_gwgttltgjma4, fls_h5o6tgul4yor, fls_p15oeage4j0e, fls_pwut2jbmk66k, fls_qxdpyf4u3hbz, fls_xmklb3070sp
//! @title: Operands, subject expressions, and expressions with and without a block
//! @lean(tier): supported

pub fn combine(a: u32, b: u32, flag: Option<bool>) -> u32 {
    let with_block = { a + b };
    let without_block = a * b;
    let chosen = if with_block > without_block {
        with_block
    } else {
        without_block
    };
    if let Some(true) = flag { chosen } else { 0 }
}

#[hax_test::equiv]
pub fn evaluated_values() -> bool {
    combine(2, 3, Some(true)) == 6 && combine(1, 1, Some(true)) == 2 && combine(2, 3, None) == 0
}
