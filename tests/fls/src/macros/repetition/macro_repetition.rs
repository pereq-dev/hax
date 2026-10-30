//! @fls: fls_V1WRuzZUWUGj, fls_8byjmlgum2f3, fls_ltdp3zs60dzr, fls_u86j0zm2jshf, fls_h5f8x4jdnvbu, fls_10lsg5212ffb, fls_hf4gj5pfl437, fls_tm0w0680wf4x, fls_4ps4x4513xau, fls_n2dx4ug5nd5w, fls_9n46ugmcqmix, fls_ctzthi6keit2, fls_g3dtpw4rtgdr, fls_bd673n5awwbz, fls_pvp6dxykuv66, fls_95rn4cvgznmd, fls_o9rwz9z0a2h4, fls_JinrPA0pMZCr, fls_g445ovedgo4q
//! @title: Macro repetitions with the star, plus, and question mark operators
//! @lean(tier): supported

macro_rules! sum {
    ($($x:expr),*) => {
        0 $(+ $x)*
    };
}

macro_rules! max_of {
    ($first:expr $(, $rest:expr)+) => {{
        let mut m = $first;
        $(if $rest > m { m = $rest; })+
        m
    }};
}

macro_rules! with_default {
    ($x:expr $(, $d:expr)?) => {{
        let mut v = $x;
        $(if v == 0 { v = $d; })?
        v
    }};
}

macro_rules! pairs {
    ($($a:expr => $b:expr);*) => {
        [$(($a, $b)),*]
    };
}

#[hax_test::equiv]
pub fn repeated_transcriptions() -> bool {
    let empty: u32 = sum!();
    let three = sum!(1u32, 2);
    let ps = pairs!(1u32 => 2u32; 3 => 4);
    empty == 0
        && three == 3
        && max_of!(3u32, 9, 4) == 9
        && with_default!(0u32, 7) == 7
        && with_default!(5u32) == 5
        && ps[1].1 == 4
}
