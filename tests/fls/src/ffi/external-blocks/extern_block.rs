//! @fls: fls_4dje9t5y2dia, fls_8ltVLtAfvy0m, fls_Nz0l16hMxqTd, fls_4XOoiFloXM7t, fls_v24ino4hix3m, fls_w00qi1gx204e
//! @title: An external block declaring a safe foreign function
//! @lean(tier): supported
//! @lean(reason): The foreign function extracts as an axiom (`axiom llabs : Std.I64 → RustM Std.I64`) without a model, so the guard cannot evaluate; the idioms section of the input-language reference asks for an opaque item with a hand-written model.

unsafe extern "C" {
    pub safe fn llabs(x: i64) -> i64;
}

pub fn magnitude(x: i64) -> i64 {
    llabs(x)
}

#[hax_test::equiv]
pub fn foreign_call() -> bool {
    magnitude(-4) == 4 && magnitude(3) == 3
}
