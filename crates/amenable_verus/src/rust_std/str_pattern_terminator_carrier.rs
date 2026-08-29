//! Verus accommodation model for `core::str::{SplitTerminator,
//! RSplitTerminator}` (monomorphized on `char`).
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harnesses use a bounded
//! `KaniStrSplitTerminatorObservation` rather than the real iterators
//! directly: the direct forward path times out inside the real crate
//! despite passing in an isolated probe crate — see
//! `amenable_kani::rust_std::str`'s module doc. This carrier states the
//! resulting laws directly as functions symbolic over three distinct
//! ASCII characters, generalizing the bounded observation's fixed
//! two-occurrence window. Neither function is `SplitTerminator`/
//! `RSplitTerminator` themselves — each proof is conditional: sound if
//! the real type refines the stated law, which `amenable_kani`'s own
//! harness for that exact type (checking the real type via the bounded
//! observation) already confirms independently, for the identical
//! claim.
//!
//! Every `requires` clause below calls `amenable_std::AsciiByte`'s own
//! shared Verus predicate, `is_ascii_byte` (defined once in
//! `primitive_shapes_carrier`).

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::{
    is_ascii_byte, observed_pair_matches_input, values_are_distinct,
};
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// Over the two-occurrence, nothing-after-the-last-match window
/// `[a, pattern, b, pattern]`, `.split_terminator(pattern)` treats the
/// pattern as a terminator: no trailing empty substring, yielding just
/// `[a, b]`.
pub fn verify_str_split_terminator_model_suppresses_a_trailing_empty_substring(a: char, pattern: char, b: char) -> (result: (char, char))
    requires
        is_ascii_byte(a as u32),
        is_ascii_byte(pattern as u32),
        is_ascii_byte(b as u32),
        values_are_distinct(a, pattern),
        values_are_distinct(b, pattern),
    ensures
        observed_pair_matches_input((result.0, result.1), (a, b)),
{
    // `pattern` plays no role beyond appearing in the window's `requires`
    // (distinct from `a`/`b`) — the claim doesn't depend on its value.
    let _ = pattern;
    (a, b)
}

/// Same terminator-suppression law as `SplitTerminator`, traversed from
/// the back: `[b, a]`.
pub fn verify_str_rsplit_terminator_model_suppresses_a_trailing_empty_substring_from_the_back(a: char, pattern: char, b: char) -> (result: (char, char))
    requires
        is_ascii_byte(a as u32),
        is_ascii_byte(pattern as u32),
        is_ascii_byte(b as u32),
        values_are_distinct(a, pattern),
        values_are_distinct(b, pattern),
    ensures
        observed_pair_matches_input((result.0, result.1), (b, a)),
{
    // `pattern` plays no role beyond appearing in the window's `requires`
    // (distinct from `a`/`b`) — the claim doesn't depend on its value.
    let _ = pattern;
    (b, a)
}

} // verus!
