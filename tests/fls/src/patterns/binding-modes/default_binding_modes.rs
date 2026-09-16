//! @fls: fls_vnh9wfrvumdz, fls_7xby6d1903kw, fls_RViC5UEZPQUV, fls_6lXtoxebD5It, fls_xNxQN8sgpZ3O, fls_55jtzh6a292x, fls_y3wuvj1y5j20, fls_dqe75i8h2fie, fls_qcaf2kup7zn0, fls_6acdqz8rwnn, fls_tv0avib387bv, fls_dbgmwldye42e, fls_t34oqarwcusu, fls_pxvtqxke1enp, fls_7gxb74u1np36, fls_7y56d0ulxomf, fls_X4y3QltM208J
//! @title: Default binding modes when matching through shared and mutable references
//! @lean(tier): supported

pub struct Slot {
    pub value: u32,
}

pub fn read(x: &Option<u32>) -> u32 {
    match x {
        Some(v) => *v,
        None => 0,
    }
}

pub fn increment(x: &mut Option<Slot>) {
    match x {
        Some(cell) => cell.value += 1,
        None => {}
    }
}

pub fn nested(x: &&(u32, u32)) -> u32 {
    let (a, b) = x;
    *a + *b
}

pub fn by_value(x: Option<u32>) -> u32 {
    match x {
        Some(v) => v,
        None => 0,
    }
}

#[hax_test::equiv]
pub fn binding_modes_follow_the_reference() -> bool {
    let mut c = Some(Slot { value: 1 });
    increment(&mut c);
    let after = match &c {
        Some(cell) => cell.value,
        None => 0,
    };
    read(&Some(4)) == 4 && after == 2 && nested(&&(1, 2)) == 3 && by_value(Some(9)) == 9
}
