//! @fls: fls_6uzouesw2sod, fls_dimto84ifanr, fls_fovs9il2h9xg, fls_hr8qvwlhd9ts, fls_qqrconpa92i3, fls_r1b4n12i93pg, fls_s2vpn4ihenpe, fls_x27yayh4z787, fls_yeuayil6uxzx
//! @title: Field access on record structs, tuple structs, and tuples
//! @lean(tier): supported

pub struct Person {
    pub age: u32,
    pub id: (u32, u8),
}

pub struct Wrapper(pub Person);

pub fn read(w: &Wrapper) -> u32 {
    w.0.age + w.0.id.0 + w.0.id.1 as u32
}

pub fn write(w: &mut Wrapper) {
    w.0.age += 1;
    w.0.id.1 = 42;
}

#[hax_test::equiv]
pub fn selected_fields() -> bool {
    let mut alice = Wrapper(Person {
        age: 30,
        id: (7, 1),
    });
    write(&mut alice);
    read(&alice) == 80 && alice.0.id.1 == 42
}
