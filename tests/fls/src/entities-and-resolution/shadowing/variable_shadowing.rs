//! @fls: fls_ob0riinmitkl, fls_i0gp1y38lr73, fls_fslg89a70e3n
//! @title: Shadowing of variables by later bindings and inner blocks
//! @lean(tier): supported

pub fn shadowed(x: u32) -> u32 {
    let x = x + 1;
    let x = x * 2;
    let y = {
        let x = x + 100;
        x
    };
    x + y
}

#[hax_test::equiv]
pub fn latest_binding_wins() -> bool {
    shadowed(1) == 108
}
