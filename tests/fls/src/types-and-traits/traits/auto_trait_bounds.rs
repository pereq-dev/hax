//! @fls: fls_YynbrIceKmsJ, fls_02D6ku4Sd6yL, fls_RLFIzQeAPhG6, fls_avSxO0LEka2x, fls_ft8axGGOe3aL, fls_M4EoT1NMyxJS, fls_Tir2kpKNP1KC, fls_WxHiKr8BGGvz, fls_Bd4HwdrRuXMm, fls_UzfG5ic8PUIH
//! @title: Auto trait bounds satisfied by a plain struct
//! @lean(tier): supported
//! @lean(reason): aeneas fails on a bound by an auto trait other than `Send` and `Sync`, here `Unpin`, `UnwindSafe`, and `RefUnwindSafe` (`Unhandled BuiltinOrAuto for trait ...; builtin_impl_data: BuiltinAuto`); the ownership model section of the input-language reference.

use core::panic::{RefUnwindSafe, UnwindSafe};

pub struct Pair {
    pub a: u32,
    pub b: u32,
}

pub struct Empty;

pub fn through<T: Send + Sync + Unpin + UnwindSafe + RefUnwindSafe>(p: T) -> T {
    p
}

#[hax_test::equiv]
pub fn bounds_are_transparent() -> bool {
    let p = through(Pair { a: 1, b: 2 });
    let Empty = through(Empty);
    p.a + p.b == 3
}
