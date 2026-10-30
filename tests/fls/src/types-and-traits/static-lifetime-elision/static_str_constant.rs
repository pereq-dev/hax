//! @fls: fls_l4RDXaFwnQZ6, fls_8irr97rZWfSC, fls_2GKCEI9MwMn9, fls_f3yZ31dRuTPG
//! @title: Constants and statics of reference type with the elided 'static lifetime
//! @lean(tier): supported

pub const FIRST: &u32 = &1;
pub static TABLE: &[u32] = &[10, 20, 30];

#[hax_test::equiv]
pub fn static_references() -> bool {
    *FIRST == 1 && TABLE[1] == 20 && TABLE.len() == 3
}
