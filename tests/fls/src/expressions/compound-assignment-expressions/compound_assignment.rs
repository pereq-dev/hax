//! @fls: fls_3bu3g8o5nopc, fls_48i245an2449, fls_4nnqz4etisgw, fls_69wr03rt0ali, fls_6x7j9x354pkb, fls_7l7v7vigw3fu, fls_8j408kckzzud, fls_9d970yfwmj2d, fls_9v09ayi2azpe, fls_a2g4hs15jpiu, fls_ak4g5112jkl, fls_d1cxq1zbt5fq, fls_dvy201zd6oym, fls_eesn9kuylim, fls_fbp5dojti27r, fls_h2cpbz2t74hy, fls_hovju0sni9gr, fls_ijfmnnrdlu8n, fls_kuet16jp6ps9, fls_lkjwyy78m2vx, fls_ndlv3k9uclz2, fls_oy9ur11k78t, fls_p9687v3xckps, fls_pkzj0uigfcgm, fls_s7rey2bndfei, fls_w2hbhb989yr4, fls_whj50spxz3bh, fls_xmgcdw9yhb55, fls_yeh6mvyvb4dp
//! @title: The ten compound assignment operators on integers
//! @lean(tier): supported

pub fn apply(mut x: u32) -> [u32; 10] {
    let mut out = [0; 10];
    x += 3;
    out[0] = x;
    x -= 1;
    out[1] = x;
    x *= 4;
    out[2] = x;
    x /= 3;
    out[3] = x;
    x %= 5;
    out[4] = x;
    x &= 0b110;
    out[5] = x;
    x |= 0b1001;
    out[6] = x;
    x ^= 0b11;
    out[7] = x;
    x <<= 2;
    out[8] = x;
    x >>= 1;
    out[9] = x;
    out
}

#[hax_test::equiv]
pub fn compound_results() -> bool {
    apply(1) == [4, 3, 12, 4, 4, 4, 13, 14, 56, 28]
}
