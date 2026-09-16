//! @fls: fls_yiPv3wKYkSRl, fls_AXOtKdSQR4AF
//! @title: An array of a visibly empty type is visibly empty
//! @lean(tier): supported
//! @lean(reason): aeneas fails on the array pattern `[v]` over an uninhabited element type (`Could not read a borrow`); the control flow section of the input-language reference.

pub enum Void {}

pub fn unwrap_array(x: Result<u32, [Void; 1]>) -> u32 {
    match x {
        Ok(v) => v,
        Err([v]) => match v {},
    }
}

#[hax_test::equiv]
pub fn inhabited_arm_only() -> bool {
    unwrap_array(Ok(4)) == 4
}
