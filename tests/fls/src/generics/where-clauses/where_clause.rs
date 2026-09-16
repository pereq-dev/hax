//! @fls: fls_3nqb7p5ifvio, fls_fhy4rsmmbvyy, fls_V4PKFqtCsAv6, fls_cslGPmVjujHD, fls_ytk74dyxuy6d
//! @title: Where clauses with type bound and lifetime bound predicates
//! @lean(tier): supported

pub trait Double {
    fn double(&self) -> Self;
}

impl Double for u32 {
    fn double(&self) -> u32 {
        *self * 2
    }
}

pub fn quad<T>(x: T) -> T
where
    T: Double,
{
    x.double().double()
}

pub fn pick<'a, 'b, T>(x: &'a T, _y: &'b T) -> &'a T
where
    'b: 'a,
    T: Double,
{
    x
}

#[hax_test::equiv]
pub fn predicates_hold() -> bool {
    quad(3u32) == 12 && *pick(&1u32, &2u32) == 1
}
