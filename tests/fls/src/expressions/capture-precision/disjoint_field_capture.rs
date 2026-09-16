//! @fls: fls_4TESOxGpEY2h, fls_7EXHdE2eOVek, fls_Fs12dmznjsMf, fls_Gj1znNpthHY6, fls_HMJUXHrvOmPl, fls_IFyJvb6mlFU4, fls_RaONmCLH2KGM, fls_TbfUxVf8PKPs, fls_Vt9C9mKxHOwo, fls_eNkZWskzznW6, fls_fATMTNUOHsfb, fls_gujpU7p5n9Zx, fls_iLH8X2U4ADHb, fls_j9WyKVyOLFon, fls_rdDT7jsaOMbs, fls_t8tFLUg8O83Q, fls_v8IFXHJnXhez
//! @title: Closures capturing disjoint fields and matching enums and arrays
//! @lean(tier): supported

pub struct State {
    pub count: u32,
    pub total: u32,
}

pub enum Mode {
    On,
    Off,
}

pub fn disjoint(mut s: State, mode: Mode, xs: [u32; 2]) -> u32 {
    let mut add = |k: u32| s.total += k;
    add(s.count);
    add(1);
    let is_on = || match mode {
        Mode::On => true,
        Mode::Off => false,
    };
    let first = || {
        let [head, _] = xs;
        head
    };
    let moved = move || {
        let State { count, .. } = s;
        count
    };
    let on = if is_on() { 100 } else { 0 };
    moved() + first() + on
}

#[hax_test::equiv]
pub fn field_paths_are_captured() -> bool {
    disjoint(State { count: 3, total: 0 }, Mode::On, [7, 8]) == 110
}
