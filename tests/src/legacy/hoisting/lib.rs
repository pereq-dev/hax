#![allow(dead_code)]

/// Issue #2336: the right operand of `&&` and `||` is evaluated only if the
/// left one does not determine the result.
mod issue_2336 {
    fn change(value: &mut bool) -> bool {
        *value = true;
        false
    }

    fn or_mut_ref(first: bool, value: &mut bool) -> bool {
        first || change(value)
    }

    fn and_mut_ref(first: bool, value: &mut bool) -> bool {
        first && change(value)
    }

    fn or_assign(first: bool) -> u8 {
        let mut x = 0;
        let _ = first || {
            x = 1;
            false
        };
        x
    }

    fn fallible(b: bool) -> Result<bool, u8> {
        if b { Ok(true) } else { Err(1) }
    }

    fn or_question_mark(first: bool) -> Result<bool, u8> {
        Ok(first || fallible(false)?)
    }

    fn or_return(first: bool) -> u8 {
        let _ = first || return 1;
        0
    }

    fn and_local_let(i: usize, v: &[u8], value: &mut bool) -> bool {
        i < v.len() && {
            let x = v[i];
            x > 0 || change(value)
        }
    }

    fn if_condition(first: bool, value: &mut bool) -> u8 {
        if first || change(value) { 1 } else { 0 }
    }

    /// @fail(extraction): proverif(HAX0008)
    fn match_guard(o: Option<u8>, value: &mut bool) -> u8 {
        match o {
            Some(x) if x > 3 || change(value) => x,
            _ => 0,
        }
    }

    fn pure_operands(a: bool, b: bool, c: u8) -> bool {
        a && (b || c > 3)
    }

    fn left_mut_ref(second: bool, value: &mut bool) -> bool {
        change(value) || second
    }

    fn both_mut_refs(a: &mut bool, b: &mut bool) -> bool {
        change(a) && change(b)
    }

    fn chained(first: bool, second: bool, value: &mut bool) -> bool {
        first || second || change(value)
    }
}

/// Issue #2337: the condition of a `while` loop is evaluated at each iteration.
mod while_condition {
    /// @fail(extraction): coq(HAX0001, HAX0001), ssprove(HAX0001), proverif(HAX0008)
    fn local_let() -> u8 {
        let mut n = 0u8;
        while {
            let y = n.wrapping_add(1);
            y < 4
        } {
            n += 1;
        }
        n
    }

    /// @fail(extraction): coq(HAX0001, HAX0001), ssprove(HAX0001), proverif(HAX0008)
    fn local_mut() -> u8 {
        let mut n = 0u8;
        while {
            let mut y = n;
            y = y.wrapping_add(1);
            y < 4
        } {
            n += 1;
        }
        n
    }

    /// @fail(extraction): coq(HAX0001, HAX0001, HAX0008), ssprove(HAX0008, HAX0001), proverif(HAX0008, HAX0008)
    // Lean cannot prove automatically that this condition does not panic
    #[cfg_attr(hax_backend_legacy_lean, hax_lib::opaque)]
    fn inner_loop(n: u8) -> u8 {
        let mut i = 0u8;
        while {
            let mut k = 0u8;
            for z in 0..i {
                if z % 2 == 0 {
                    continue;
                }
                k = k.wrapping_add(1);
            }
            k < n
        } {
            i = i.wrapping_add(1);
        }
        i
    }

    fn dec(x: &mut u8) -> bool {
        if *x == 0 {
            false
        } else {
            *x -= 1;
            true
        }
    }

    /// @fail(extraction): fstar(HAX0001), legacy-lean(HAX0001), coq(HAX0001, HAX0001), ssprove(HAX0001), proverif(HAX0008)
    fn mut_ref() -> u8 {
        let mut x = 3u8;
        let mut n = 0u8;
        while dec(&mut x) {
            n += 1;
        }
        n
    }

    /// @fail(extraction): fstar(HAX0001), legacy-lean(HAX0001), coq(HAX0001, HAX0001), ssprove(HAX0001), proverif(HAX0008)
    fn assign() -> u8 {
        let mut n = 0u8;
        while {
            n += 1;
            n < 4
        } {}
        n
    }

    fn fallible(b: bool) -> Result<bool, u8> {
        if b { Ok(true) } else { Err(1) }
    }

    /// @fail(extraction): fstar(HAX0001), legacy-lean(HAX0001), coq(HAX0001, HAX0001), ssprove(HAX0001), proverif(HAX0008)
    fn question_mark() -> Result<u8, u8> {
        let mut n = 0u8;
        while fallible(n < 4)? {
            n += 1;
        }
        Ok(n)
    }

    /// @fail(extraction): fstar(HAX0001), legacy-lean(HAX0001), coq(HAX0001, HAX0001), ssprove(HAX0001), proverif(HAX0008)
    fn closure_return(n: u8) -> u8 {
        let mut i = 0u8;
        while {
            let c = |x: u8| -> bool {
                if x == 3 {
                    return false;
                }
                true
            };
            c(i) && i < n
        } {
            i += 1;
        }
        i
    }
}
