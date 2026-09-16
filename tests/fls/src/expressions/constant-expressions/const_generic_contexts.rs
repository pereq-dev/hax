//! @fls: fls_3of516eo0kkx, fls_6fq6bvxxvhsr, fls_yiks5bvojncc
//! @title: Constant expressions as a const generic argument and a const parameter default
//! @lean(tier): supported
//! @lean(reason): aeneas drops the const argument of `Width::get` at both call sites (`Width.get ()`), so Lean cannot infer it (`don't know how to synthesize implicit argument N`); the functions section of the input-language reference.

pub struct Width<const N: usize = 2>;

impl<const N: usize> Width<N> {
    pub const fn get(&self) -> usize {
        N * 10
    }
}

pub fn widths() -> usize {
    let default: Width = Width;
    let explicit = Width::<{ 1 + 2 }>;
    default.get() + explicit.get()
}

#[hax_test::equiv]
pub fn arguments_evaluate() -> bool {
    widths() == 50
}
