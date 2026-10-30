//! @fls: fls_klcltwcwrw6i, fls_y1z7kougmahd, fls_chtj3hcfe3ap, fls_wm61yeclairz, fls_YnUsdSM4x9eq, fls_n77icl6idazp, fls_h2zikgmazoxx, fls_7sm3206va03c, fls_nRgjCLYZL3iX, fls_huynsyx13gsz, fls_ybv0tdu7dnj5, fls_e65q3iz50j6a, fls_RZvIsApi4WQm, fls_7kb6ltajgiou, fls_28c21rzc6rsp, fls_4s2n95h4rd1q, fls_cul31g1kkz5c, fls_no853u27p4f3, fls_ojdntg5i79pb, fls_8q8nqfpSz7Ly, fls_kv5bpq8rf1j9, fls_nj7s6xmzx55f, fls_tvvycup09b51
//! @title: Simple paths, type paths, qualified paths, and a global path
//! @lean(tier): supported

pub trait Named {
    type Id;
    fn id() -> Self::Id;
}

pub struct S;

impl Named for S {
    type Id = u32;
    fn id() -> u32 {
        7
    }
}

pub mod util {
    pub fn double(x: u32) -> u32 {
        x * 2
    }
}

pub fn projected() -> <S as Named>::Id {
    <S as Named>::id()
}

#[hax_test::equiv]
pub fn resolved_paths() -> bool {
    let generic: Option<u32> = Some::<u32>(1);
    let global = ::core::cmp::max(1u32, 2u32);
    self::util::double(projected()) == 14 && generic == Some(1) && global == 2
}
