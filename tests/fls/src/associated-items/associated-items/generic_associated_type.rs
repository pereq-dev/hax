//! @fls: fls_PeD0DzjK57be, fls_3foYUch29ZtF, fls_SnQc0zZS57Cz, fls_6Z05BK2JSzpP, fls_AtItgS1UvwiX, fls_znfADVeOvXHD
//! @title: A generic associated type with a lifetime parameter
//! @lean(tier): supported
//! @lean(reason): aeneas fails on the generic associated type (`Unimplemented` in `translate_fwd_ty`); the traits section of the input-language reference.

pub trait Lender {
    type Loan<'a>
    where
        Self: 'a;
    fn lend(&self) -> Self::Loan<'_>;
}

pub struct Bag(pub [u32; 2]);

impl Lender for Bag {
    type Loan<'a> = &'a u32;
    fn lend(&self) -> &u32 {
        &self.0[1]
    }
}

#[hax_test::equiv]
pub fn lent_value() -> bool {
    *Bag([1, 2]).lend() == 2
}
