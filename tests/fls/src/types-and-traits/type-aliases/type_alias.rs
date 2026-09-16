//! @fls: fls_bibigic4jjad
//! @title: A type alias substituted by the type it names
//! @lean(tier): supported

pub type Pair = (u32, u32);
pub type Grid<T> = [[T; 2]; 2];

pub fn swap(p: Pair) -> Pair {
    (p.1, p.0)
}

pub fn trace(g: Grid<u32>) -> u32 {
    g[0][0] + g[1][1]
}

#[hax_test::equiv]
pub fn alias_is_transparent() -> bool {
    swap((1, 2)) == (2, 1) && trace([[1, 2], [3, 4]]) == 5
}
