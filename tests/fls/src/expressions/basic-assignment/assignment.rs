//! @fls: fls_9i0ctuo099bp, fls_b0mqcn5fejhk, fls_esn5ceoldta, fls_hc01gtvlxba, fls_t8eqzc64ivin, fls_uhcodvq75nlr, fls_bsjw6f4a3wol, fls_bwwtgqprbxrm, fls_nhgexeu2h6wi, fls_qengy157fa4a, fls_uinh05sslxeo
//! @title: Basic assignment to variables, fields, and indexed places
//! @lean(tier): supported

pub struct Slot {
    pub v: u32,
}

pub fn assign() -> [u32; 3] {
    let mut a = 1;
    let mut c = Slot { v: 0 };
    let mut arr = [0u32; 2];
    a = 5;
    c.v = a + 1;
    arr[1] = c.v * 2;
    let unit: () = (a = 9);
    let () = unit;
    [a, c.v, arr[1]]
}

#[hax_test::equiv]
pub fn assigned_values() -> bool {
    assign() == [9, 6, 12]
}
