//! @fls: fls_ecteot716j8j, fls_4lb6yh12w1cv, fls_tKj18krZ1pSt, fls_98vyqh9bzigx, fls_ppnj8bcncdp9, fls_wmay1vd8u0da, fls_e613hpr50t9, fls_bic6iyd1nvfm, fls_w5a6rybvptn6, fls_vesfzh8h6qzu, fls_rr8rsy7oilf0, fls_5ruu8n6r9mvp, fls_7gxb7ztpuofj, fls_on0i2cpk254y, fls_455dmnp4cxqv, fls_5a3vfresnv5z, fls_sqs5to20p0te, fls_9hhtcey2d4t6, fls_8aynifgq02gt, fls_j9XOoXDmN5Dq, fls_aniw4ehsn2kb, fls_2ekwpx2bwj1b, fls_n85sjh925x, fls_4x3w50w7qm8w, fls_6cne58tlquze, fls_tel7kogaqytg, fls_aw61psz5drg8, fls_sv2x3x81b32j, fls_2d05gcixjrzt, fls_4jr7eg6e0g4w, fls_xugsn2ghh73c, fls_i21l6t3vn95t, fls_quer8ltdwnf2, fls_eka6chp3hapa, fls_ejuap3kkvs57
//! @title: An asm! block moving an input register to an output register
//! @lean(tier): unsupported
//! @lean(reason): The value-model boundary of the input-language reference: arbitrary machine instructions have no Rust-level value semantics to translate.

pub fn copy(x: u64) -> u64 {
    let y: u64;
    // `mov dst, src` is valid on x86_64 (Intel syntax) and on aarch64.
    unsafe {
        core::arch::asm!("mov {}, {}", out(reg) y, in(reg) x, options(pure, nomem, nostack));
    }
    y
}

#[hax_test::equiv]
pub fn copied() -> bool {
    copy(7) == 7
}
