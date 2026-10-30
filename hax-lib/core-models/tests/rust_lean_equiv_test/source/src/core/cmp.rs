//! Equivalence tests for `core::cmp::*`.

use crate::helpers::keyed;

// ----- u8: PartialEq::eq -----------------------------------------------------

#[hax_test::equiv]
pub fn test_int_eq_same() -> bool {
    (0u8 == 0u8) == true
}

#[hax_test::equiv]
pub fn test_int_eq_diff() -> bool {
    (0u8 == u8::MAX) == false
}

#[hax_test::equiv]
pub fn test_int_eq_max_max() -> bool {
    (u8::MAX == u8::MAX) == true
}

// ----- u8: != ----------------------------------------------------------------

#[hax_test::equiv]
pub fn test_int_neq_same() -> bool {
    (0u8 != 0u8) == false
}

#[hax_test::equiv]
pub fn test_int_neq_diff() -> bool {
    (0u8 != u8::MAX) == true
}

// ----- u8: < (lt) ------------------------------------------------------------

#[hax_test::equiv]
pub fn test_int_lt_true() -> bool {
    (0u8 < u8::MAX) == true
}

#[hax_test::equiv]
pub fn test_int_lt_equal() -> bool {
    (7u8 < 7u8) == false
}

#[hax_test::equiv]
pub fn test_int_lt_false() -> bool {
    (u8::MAX < 0u8) == false
}

// ----- u8: <= (le) -----------------------------------------------------------

#[hax_test::equiv]
pub fn test_int_le_true() -> bool {
    (0u8 <= u8::MAX) == true
}

#[hax_test::equiv]
pub fn test_int_le_equal() -> bool {
    (7u8 <= 7u8) == true
}

#[hax_test::equiv]
pub fn test_int_le_false() -> bool {
    (u8::MAX <= 0u8) == false
}

// ----- u8: > (gt) ------------------------------------------------------------

#[hax_test::equiv]
pub fn test_int_gt_true() -> bool {
    (u8::MAX > 0u8) == true
}

#[hax_test::equiv]
pub fn test_int_gt_equal() -> bool {
    (7u8 > 7u8) == false
}

#[hax_test::equiv]
pub fn test_int_gt_false() -> bool {
    (0u8 > u8::MAX) == false
}

// ----- u8: >= (ge) -----------------------------------------------------------

#[hax_test::equiv]
pub fn test_int_ge_true() -> bool {
    (u8::MAX >= 0u8) == true
}

#[hax_test::equiv]
pub fn test_int_ge_equal() -> bool {
    (7u8 >= 7u8) == true
}

#[hax_test::equiv]
pub fn test_int_ge_false() -> bool {
    (0u8 >= u8::MAX) == false
}

// ----- u8::partial_cmp -------------------------------------------------------

// TODO(partial-cmp-option): partial_cmp on integers returns
// `Option<Ordering>` whose `Some(Ordering::_)` shape involves both the
// option type (fine, helpers exist) AND the Ordering variant. We test the
// downstream `is_lt` / `is_eq` / `is_gt` predicates above instead; matching
// on `Option<Ordering>` directly needs more care to keep types pinned.

// ----- u8: Ord::cmp ----------------------------------------------------------
// Directly exercises the scalar `Ord` instance (`U8.Insts.CoreCmpOrd`)
// re-provided in `FunsPrologue`.

#[hax_test::equiv]
pub fn test_u8_cmp_less() -> bool {
    match 3u8.cmp(&7u8) {
        std::cmp::Ordering::Less => true,
        _ => false,
    }
}

#[hax_test::equiv]
pub fn test_u8_cmp_greater() -> bool {
    match 9u8.cmp(&2u8) {
        std::cmp::Ordering::Greater => true,
        _ => false,
    }
}

#[hax_test::equiv]
pub fn test_u8_cmp_equal() -> bool {
    match 5u8.cmp(&5u8) {
        std::cmp::Ordering::Equal => true,
        _ => false,
    }
}

// ----- clamp / Ordering::then / Ordering::then_with ---------------------------

#[hax_test::equiv]
pub fn test_ordering_then_keeps_first() -> bool {
    match std::cmp::Ordering::Less.then(std::cmp::Ordering::Greater) {
        std::cmp::Ordering::Less => true,
        _ => false,
    }
}

#[hax_test::equiv]
pub fn test_ordering_then_falls_through_on_equal() -> bool {
    match std::cmp::Ordering::Equal.then(std::cmp::Ordering::Greater) {
        std::cmp::Ordering::Greater => true,
        _ => false,
    }
}

#[hax_test::equiv]
pub fn test_ordering_is_le() -> bool {
    std::cmp::Ordering::Less.is_le() && std::cmp::Ordering::Equal.is_le()
}

#[hax_test::equiv]
pub fn test_ordering_is_gt() -> bool {
    std::cmp::Ordering::Greater.is_gt() && !std::cmp::Ordering::Equal.is_gt()
}

// Rust-only: the model's `Ord` has only `cmp`, no `clamp`.
#[cfg(test)]
mod ord_clamp {
    #[test]
    fn test_clamp_below() {
        assert_eq!(1u8.clamp(3, 7), 3);
    }

    #[test]
    fn test_clamp_inside() {
        assert_eq!(5u8.clamp(3, 7), 5);
    }

    #[test]
    fn test_clamp_above() {
        assert_eq!(9u8.clamp(3, 7), 7);
    }
}

// ----- dictionary-applying tests: Ord tie-breaking ---------------------------

// std: "Returns the second argument if the comparison determines them to be
// equal."
#[hax_test::equiv]
pub fn test_cmp_max_tie_returns_second() -> bool {
    core::cmp::max(keyed(5, 1), keyed(5, 2)).tag == 2
}

// std: "Returns the first argument if the comparison determines them to be
// equal."
#[hax_test::equiv]
pub fn test_cmp_min_tie_returns_first() -> bool {
    core::cmp::min(keyed(5, 1), keyed(5, 2)).tag == 1
}

#[hax_test::equiv]
pub fn test_cmp_max_picks_the_greater() -> bool {
    core::cmp::max(keyed(1, 1), keyed(9, 2)).tag == 2
}

#[hax_test::equiv]
pub fn test_cmp_min_picks_the_lesser() -> bool {
    core::cmp::min(keyed(1, 1), keyed(9, 2)).tag == 1
}

// ----- dictionary-applying tests: PartialEq ----------------------------------

// `Keyed::eq` ignores `tag`, so a structural comparison would answer `false`.
#[hax_test::equiv]
pub fn test_partial_eq_goes_through_the_dictionary() -> bool {
    keyed(5, 1) == keyed(5, 2)
}

#[hax_test::equiv]
pub fn test_partial_ne_goes_through_the_dictionary() -> bool {
    (keyed(5, 1) != keyed(5, 2)) == false
}

// ----- Reverse ---------------------------------------------------------------

#[hax_test::equiv]
pub fn test_reverse_cmp_is_flipped() -> bool {
    use core::cmp::Reverse;
    core::cmp::max(Reverse(keyed(1, 1)), Reverse(keyed(9, 2)))
        .0
        .tag
        == 1
}

#[hax_test::equiv]
pub fn test_reverse_eq_goes_through_the_dictionary() -> bool {
    use core::cmp::Reverse;
    Reverse(keyed(5, 1)) == Reverse(keyed(5, 2))
}

// ----- PartialEq for Ordering ------------------------------------------------

#[hax_test::equiv]
pub fn test_ordering_eq_same() -> bool {
    3u8.cmp(&7u8) == core::cmp::Ordering::Less
}

#[hax_test::equiv]
pub fn test_ordering_eq_different() -> bool {
    (3u8.cmp(&7u8) == core::cmp::Ordering::Greater) == false
}

#[hax_test::equiv]
pub fn test_ordering_ne() -> bool {
    5u8.cmp(&5u8) != core::cmp::Ordering::Less
}
