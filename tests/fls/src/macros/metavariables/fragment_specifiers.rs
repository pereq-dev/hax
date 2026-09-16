//! @fls: fls_g93r3teei8wo, fls_4zdait30exvn, fls_8zypylq60zba, fls_ephlmLsGTMgw, fls_2HguXbL7DjKH
//! @title: Metavariables with the expr, ident, ty, literal, pat, block, and tt fragment specifiers
//! @lean(tier): supported

macro_rules! define {
    ($name:ident : $t:ty = $init:literal) => {
        let $name: $t = $init;
    };
}

macro_rules! when {
    ($scrutinee:expr, $p:pat => $body:block) => {
        match $scrutinee {
            $p => $body,
            _ => 0,
        }
    };
}

macro_rules! passthrough {
    ($($t:tt)*) => {
        $($t)*
    };
}

pub fn run(flag: Option<u32>) -> u32 {
    define!(base: u32 = 5);
    let matched = when!(flag, Some(v) => { v * 2 });
    passthrough!(base + matched)
}

#[hax_test::equiv]
pub fn fragments_expand() -> bool {
    run(Some(3)) == 11 && run(None) == 5
}
