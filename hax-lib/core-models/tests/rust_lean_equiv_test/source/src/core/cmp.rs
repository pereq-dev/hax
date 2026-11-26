//! Equivalence tests for `core::cmp::*`.

use crate::helpers::keyed;
use rust_lean_test_macro::rust_lean_test;

// ----- u8: PartialEq::eq -----------------------------------------------------

#[rust_lean_test]
pub fn test_int_eq_same() -> bool {
    (0u8 == 0u8) == true
}

#[rust_lean_test]
pub fn test_int_eq_diff() -> bool {
    (0u8 == u8::MAX) == false
}

#[rust_lean_test]
pub fn test_int_eq_max_max() -> bool {
    (u8::MAX == u8::MAX) == true
}

// ----- u8: != ----------------------------------------------------------------

#[rust_lean_test]
pub fn test_int_neq_same() -> bool {
    (0u8 != 0u8) == false
}

#[rust_lean_test]
pub fn test_int_neq_diff() -> bool {
    (0u8 != u8::MAX) == true
}

// ----- u8: < (lt) ------------------------------------------------------------

#[rust_lean_test]
pub fn test_int_lt_true() -> bool {
    (0u8 < u8::MAX) == true
}

#[rust_lean_test]
pub fn test_int_lt_equal() -> bool {
    (7u8 < 7u8) == false
}

#[rust_lean_test]
pub fn test_int_lt_false() -> bool {
    (u8::MAX < 0u8) == false
}

// ----- u8: <= (le) -----------------------------------------------------------

#[rust_lean_test]
pub fn test_int_le_true() -> bool {
    (0u8 <= u8::MAX) == true
}

#[rust_lean_test]
pub fn test_int_le_equal() -> bool {
    (7u8 <= 7u8) == true
}

#[rust_lean_test]
pub fn test_int_le_false() -> bool {
    (u8::MAX <= 0u8) == false
}

// ----- u8: > (gt) ------------------------------------------------------------

#[rust_lean_test]
pub fn test_int_gt_true() -> bool {
    (u8::MAX > 0u8) == true
}

#[rust_lean_test]
pub fn test_int_gt_equal() -> bool {
    (7u8 > 7u8) == false
}

#[rust_lean_test]
pub fn test_int_gt_false() -> bool {
    (0u8 > u8::MAX) == false
}

// ----- u8: >= (ge) -----------------------------------------------------------

#[rust_lean_test]
pub fn test_int_ge_true() -> bool {
    (u8::MAX >= 0u8) == true
}

#[rust_lean_test]
pub fn test_int_ge_equal() -> bool {
    (7u8 >= 7u8) == true
}

#[rust_lean_test]
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

#[rust_lean_test]
pub fn test_u8_cmp_less() -> bool {
    match 3u8.cmp(&7u8) {
        std::cmp::Ordering::Less => true,
        _ => false,
    }
}

#[rust_lean_test]
pub fn test_u8_cmp_greater() -> bool {
    match 9u8.cmp(&2u8) {
        std::cmp::Ordering::Greater => true,
        _ => false,
    }
}

#[rust_lean_test]
pub fn test_u8_cmp_equal() -> bool {
    match 5u8.cmp(&5u8) {
        std::cmp::Ordering::Equal => true,
        _ => false,
    }
}

// ----- clamp / Ordering::then / Ordering::then_with ---------------------------

#[rust_lean_test]
pub fn test_ordering_then_keeps_first() -> bool {
    match std::cmp::Ordering::Less.then(std::cmp::Ordering::Greater) {
        std::cmp::Ordering::Less => true,
        _ => false,
    }
}

#[rust_lean_test]
pub fn test_ordering_then_falls_through_on_equal() -> bool {
    match std::cmp::Ordering::Equal.then(std::cmp::Ordering::Greater) {
        std::cmp::Ordering::Greater => true,
        _ => false,
    }
}

#[rust_lean_test]
pub fn test_ordering_is_le() -> bool {
    std::cmp::Ordering::Less.is_le() && std::cmp::Ordering::Equal.is_le()
}

#[rust_lean_test]
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
#[rust_lean_test]
pub fn test_cmp_max_tie_returns_second() -> bool {
    core::cmp::max(keyed(5, 1), keyed(5, 2)).tag == 2
}

// std: "Returns the first argument if the comparison determines them to be
// equal."
#[rust_lean_test]
pub fn test_cmp_min_tie_returns_first() -> bool {
    core::cmp::min(keyed(5, 1), keyed(5, 2)).tag == 1
}

#[rust_lean_test]
pub fn test_cmp_max_picks_the_greater() -> bool {
    core::cmp::max(keyed(1, 1), keyed(9, 2)).tag == 2
}

#[rust_lean_test]
pub fn test_cmp_min_picks_the_lesser() -> bool {
    core::cmp::min(keyed(1, 1), keyed(9, 2)).tag == 1
}

// ----- dictionary-applying tests: PartialEq ----------------------------------

// `Keyed::eq` ignores `tag`, so a structural comparison would answer `false`.
#[rust_lean_test]
pub fn test_partial_eq_goes_through_the_dictionary() -> bool {
    keyed(5, 1) == keyed(5, 2)
}

#[rust_lean_test]
pub fn test_partial_ne_goes_through_the_dictionary() -> bool {
    (keyed(5, 1) != keyed(5, 2)) == false
}

// ----- Reverse ---------------------------------------------------------------

#[rust_lean_test]
pub fn test_reverse_cmp_is_flipped() -> bool {
    use core::cmp::Reverse;
    core::cmp::max(Reverse(keyed(1, 1)), Reverse(keyed(9, 2)))
        .0
        .tag
        == 1
}

#[rust_lean_test]
pub fn test_reverse_eq_goes_through_the_dictionary() -> bool {
    use core::cmp::Reverse;
    Reverse(keyed(5, 1)) == Reverse(keyed(5, 2))
}

// ----- PartialEq for Ordering ------------------------------------------------

#[rust_lean_test]
pub fn test_ordering_eq_same() -> bool {
    3u8.cmp(&7u8) == core::cmp::Ordering::Less
}

#[rust_lean_test]
pub fn test_ordering_eq_different() -> bool {
    (3u8.cmp(&7u8) == core::cmp::Ordering::Greater) == false
}

#[rust_lean_test]
pub fn test_ordering_ne() -> bool {
    5u8.cmp(&5u8) != core::cmp::Ordering::Less
}

// ----- Ord::max / min / clamp -----------------------------------------------

#[rust_lean_test]
pub fn test_ord_max_min() -> bool {
    3usize.max(5) == 5 && 3usize.min(5) == 3 && 7u8.max(7) == 7
}

#[rust_lean_test]
pub fn test_ord_clamp() -> bool {
    10u32.clamp(2, 5) == 5 && 1u32.clamp(2, 5) == 2 && 3u32.clamp(2, 5) == 3
}

#[rust_lean_test(panics)]
pub fn test_ord_clamp_inverted_panics() -> bool {
    3u32.clamp(5, 2) == 3
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Wrapped(pub u8);

#[rust_lean_test]
pub fn test_ord_min_on_derived() -> bool {
    Wrapped(4).min(Wrapped(2)) == Wrapped(2) && Wrapped(4).max(Wrapped(2)) == Wrapped(4)
}

// ----- Ord for arrays --------------------------------------------------------

#[rust_lean_test]
pub fn test_array_cmp() -> bool {
    [1u8, 2, 3].cmp(&[1u8, 3, 0]) == core::cmp::Ordering::Less
        && [1u8, 2, 3].cmp(&[1u8, 2, 3]) == core::cmp::Ordering::Equal
        && [2u8, 0, 0].cmp(&[1u8, 9, 9]) == core::cmp::Ordering::Greater
}

/// `PartialOrd` reversed against `Ord`: `max`, `min` and `clamp` follow the
/// former, as std's do.
#[derive(PartialEq, Eq)]
pub struct Skew(pub u8);

impl PartialOrd for Skew {
    fn partial_cmp(&self, other: &Skew) -> Option<core::cmp::Ordering> {
        Some(other.0.cmp(&self.0))
    }
}

impl Ord for Skew {
    fn cmp(&self, other: &Skew) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

#[rust_lean_test]
pub fn test_ord_defaults_follow_partial_ord() -> bool {
    Skew(1).max(Skew(2)) == Skew(1)
        && Skew(1).min(Skew(2)) == Skew(2)
        && Skew(5).clamp(Skew(4), Skew(2)) == Skew(4)
}

#[rust_lean_test]
pub fn test_cmp_max_min_follow_partial_ord() -> bool {
    core::cmp::max(Skew(1), Skew(2)) == Skew(1) && core::cmp::min(Skew(1), Skew(2)) == Skew(2)
}
