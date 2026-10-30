//! @fls: fls_opt7v0mopxc8, fls_n5l17mlglq11, fls_2jujsujpjp3w, fls_dekskhk4g895, fls_cax6fe4em23k, fls_y0iqr5ibnbfe
//! @title: Send and Sync bounds satisfied automatically by a struct of scalars
//! @lean(tier): supported

pub struct Message {
    pub id: u32,
    pub body: [u8; 4],
}

pub fn share<T: Send + Sync>(m: &T) -> &T {
    m
}

pub fn send<T: Send>(m: T) -> T {
    m
}

#[hax_test::equiv]
pub fn bounds_are_erased() -> bool {
    let m = send(Message {
        id: 1,
        body: [1, 2, 3, 4],
    });
    let r = share(&m);
    r.id + r.body[3] as u32 == 5
}
