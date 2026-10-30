//! @fls: fls_5g508z6c7q5f, fls_knut10hoz6wc
//! @title: A trait bound on a type parameter
//! @lean(tier): supported

pub trait Half {
    fn half(&self) -> Self;
}

impl Half for u32 {
    fn half(&self) -> u32 {
        *self / 2
    }
}

pub fn quarter<T: Half>(x: T) -> T {
    x.half().half()
}

pub fn quarter_where<T>(x: T) -> T
where
    T: Half,
{
    x.half().half()
}

#[hax_test::equiv]
pub fn bounded_calls() -> bool {
    quarter(20u32) == 5 && quarter_where(8u32) == 2
}
