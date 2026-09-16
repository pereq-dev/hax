//! @fls: fls_3x6qd8vt5uus, fls_d4vdvpihoeb1, fls_i3z9ueoe99zd, fls_10k9gdxlpuls, fls_9pda3ja0ihks, fls_mcUMWsYcxzmZ, fls_l88o2snx9qbt, fls_thpj9io9tyuy
//! @title: Type, constant, lifetime, and binding arguments
//! @lean(tier): supported

pub trait Producer {
    type Output;
    fn produce(&self) -> Self::Output;
}

pub struct Const<const K: u32>;

impl<const K: u32> Producer for Const<K> {
    type Output = u32;
    fn produce(&self) -> u32 {
        K
    }
}

pub fn run<P: Producer<Output = u32>>(p: &P) -> u32 {
    p.produce()
}

pub fn run_bounded<P: Producer<Output: Copy + Into<u64>>>(p: &P) -> u64 {
    p.produce().into()
}

pub fn borrow<'a, T>(x: &'a T) -> &'a T
where
    T: 'a,
{
    x
}

#[hax_test::equiv]
pub fn supplied_arguments() -> bool {
    let c = Const::<5>;
    run(&c) == 5 && run_bounded(&c) == 5 && *borrow::<'_, u32>(&9) == 9
}
