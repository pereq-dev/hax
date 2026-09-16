//! @fls: fls_5x5xykocwyiy, fls_ncg9etb3x7k0, fls_74nk389rk075, fls_t9mk8kasobea, fls_u52mx4xw8zod, fls_vl1qk0odouyb, fls_xbnki64un70v, fls_h9cvs854ae34, fls_p5o243hhe1y3, fls_huvo0mp2i6fb, fls_x8r0oppuc1t6, fls_96kczd4zhpco, fls_8sevg1sa82h4, fls_amoh8r4gghyj, fls_6o38qhbna46z, fls_kgt81m4f72ne, fls_nf4g82gi12ij
//! @title: Bindings, items, labels, generic parameters, and Self in their scopes
//! @lean(tier): supported

pub struct Acc(pub u32);

impl Acc {
    pub fn doubled(&self) -> Self {
        Self(self.0 * 2)
    }
}

pub fn scoped<T: Copy, F: Fn(T) -> u32>(param: T, pick: F) -> u32 {
    fn local_item() -> u32 {
        1
    }
    let outer = pick(param) + local_item();
    let total = {
        let inner = outer * 10;
        let add = |k: u32| inner + k;
        add(1)
    };
    let mut acc = 0;
    'scan: for i in 0..3u32 {
        if let Some(v) = Some(i) {
            acc += v;
        }
        if acc > 100 {
            break 'scan;
        }
    }
    let arm = match acc {
        n if n > 1 => n,
        _ => 0,
    };
    total + arm
}

#[hax_test::equiv]
pub fn scoped_names() -> bool {
    scoped(4u32, |x| x) == 54 && Acc(2).doubled().0 == 4
}
