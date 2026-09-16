//! @fls: fls_sye3d17l9bf5, fls_95eooah0vcqx, fls_vpcqgec83ybt, fls_wmcp0n36jlbr, fls_pi6eukz7kc99, fls_h42kg56vsefx, fls_z311nxou9yi3, fls_372h3oevejih, fls_ahCqtkh0m5sR, fls_3SjMBlc0b7qo, fls_56jq9k9l31rt, fls_byqjs5fvy2bj, fls_prbwj1pmng6k, fls_sh669lnc5o1b, fls_h6kx8dxh5u96, fls_5r7ontjlmgwj, fls_m0bzw4jap6sg, fls_vo7mgm34hwg2, fls_hidfwkwr2r73, fls_mDgq5XjzKAl3, fls_YufUgB25ovh3, fls_OhVxhJ23x7W2
//! @title: Generic structs, enums, traits, aliases, and implementations with type and const parameters
//! @lean(tier): supported

pub struct Buffer<T = u8, const N: usize = 2> {
    pub items: [T; N],
}

pub enum Maybe<T> {
    Just(T),
    Nothing,
}

pub trait Fill<T> {
    const LEN: usize;
    type Items;
    fn fill(value: T) -> Self;
}

pub type Pair<T> = Buffer<T>;

impl<T: Copy, const N: usize> Fill<T> for Buffer<T, N> {
    const LEN: usize = N;
    type Items = [T; N];
    fn fill(value: T) -> Self {
        Buffer { items: [value; N] }
    }
}

pub fn first<T: Copy, const N: usize>(b: &Buffer<T, N>) -> Maybe<T> {
    if N == 0 {
        Maybe::Nothing
    } else {
        Maybe::Just(b.items[0])
    }
}

#[hax_test::equiv]
pub fn generic_values() -> bool {
    let p: Pair<u8> = Buffer::fill(7);
    let long: Buffer<u8, 5> = Buffer::fill(1);
    let d: Buffer = Buffer::fill(3);
    let got = match first(&p) {
        Maybe::Just(v) => v,
        Maybe::Nothing => 0,
    };
    got == 7
        && p.items.len() == 2
        && <Buffer<u8, 5> as Fill<u8>>::LEN == 5
        && long.items.len() == 5
        && d.items.len() == 2
        && d.items[0] == 3
}
