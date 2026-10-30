//! @fail(extraction): proverif(HAX0001, HAX0001, HAX0001, HAX0001)
//! @fail(tc): legacy-lean(1), fstar(2)
#![allow(unused_assignments, unused_variables, while_true)]

// This test confirms that (1) unexecuted infinite loops are handled correctly by the
// InstrumentCoverage MIR pass; and (2) Counter Expressions that subtract from zero can be dropped.

struct DebugTest;

/// @fail(extraction): coq(HAX0001, HAX0001, HAX0001, HAX0001), ssprove(HAX0001, HAX0001)
impl std::fmt::Debug for DebugTest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if true {
            if false {
                while true {}
            }
            write!(f, "cool")?;
        } else {
        }

        for i in 0..10 {
            if true {
                if false {
                    while true {}
                }
                write!(f, "cool")?;
            } else {
            }
        }
        Ok(())
    }
}

struct DisplayTest;

/// @fail(extraction): coq(HAX0001, HAX0001, HAX0001, HAX0001), ssprove(HAX0001, HAX0001)
impl std::fmt::Display for DisplayTest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if false {
        } else {
            if false {
                while true {}
            }
            write!(f, "cool")?;
        }
        for i in 0..10 {
            if false {
            } else {
                if false {
                    while true {}
                }
                write!(f, "cool")?;
            }
        }
        Ok(())
    }
}

/// @fail(extraction): ssprove(HAX0001)
fn main() {
    let debug_test = DebugTest;
    println!("{:?}", debug_test);
    let display_test = DisplayTest;
    println!("{}", display_test);
}
