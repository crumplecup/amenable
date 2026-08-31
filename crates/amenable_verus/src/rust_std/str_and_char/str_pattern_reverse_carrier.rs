//! Verus accommodation model for `core::str::{RSplit, RSplitN}`
//! (monomorphized on `char`).
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harnesses use a bounded
//! `KaniStrRSplitObservation`/`KaniStrRSplitNObservation` rather than
//! the real iterators directly, since reverse pattern search
//! (`memchr::memrchr` under the hood) times out under Kani even for a
//! single `.next()` call on a fixed five-byte str — see
//! `amenable_kani::rust_std::str`'s module doc. This carrier states the
//! resulting laws directly as functions symbolic over three (or four)
//! distinct ASCII characters, generalizing the bounded observation's
//! fixed window shape. Neither function is `RSplit`/`RSplitN`
//! themselves — each proof is conditional: sound if the real type
//! refines the stated law, which `amenable_kani`'s own harness for that
//! exact type (checking the real type via the bounded observation)
//! already confirms independently, for the identical claim.
//!
//! Every `requires` clause below calls `amenable_std::AsciiByte`'s own
//! shared Verus predicate, `is_ascii_byte` (defined once in
//! `primitive_shapes_carrier`).

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::primitive_shapes_carrier::{
    is_ascii_byte, observed_pair_matches_input, observed_triple_matches_input, values_are_distinct,
};
#[cfg(verus_keep_ghost)]
use crate::rust_std::str_and_char::char_carrier::char_roundtrip_preserves_value;
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// Over the one-occurrence window `[before, pattern, after]`,
/// `.rsplit(pattern)` yields the piece after the match, then the piece
/// before it.
pub fn verify_str_rsplit_model_yields_substrings_from_the_back(before: char, pattern: char, after: char) -> (result: (char, char))
    requires
        is_ascii_byte(before as u32),
        is_ascii_byte(pattern as u32),
        is_ascii_byte(after as u32),
        values_are_distinct(before, pattern),
        values_are_distinct(after, pattern),
    ensures
        observed_pair_matches_input((result.0, result.1), (after, before)),
{
    // `pattern` plays no role beyond appearing in the window's `requires`
    // (distinct from `before`/`after`) — the claim doesn't depend on its
    // value.
    let _ = pattern;
    (after, before)
}

/// Over the two-occurrence window `[a, pattern, b, pattern, c]`,
/// `.rsplitn(2, pattern)` yields the piece after the last match (`c`),
/// then everything before it uncut (`[a, pattern, b]`).
pub fn verify_str_rsplitn_model_limits_to_n_substrings_from_the_back(a: char, pattern: char, b: char, c: char) -> (result: (char, (char, char, char)))
    requires
        is_ascii_byte(a as u32),
        is_ascii_byte(pattern as u32),
        is_ascii_byte(b as u32),
        is_ascii_byte(c as u32),
        values_are_distinct(a, pattern),
        values_are_distinct(b, pattern),
        values_are_distinct(c, pattern),
    ensures
        char_roundtrip_preserves_value(result.0, c),
        observed_triple_matches_input(result.1, (a, pattern, b)),
{
    (c, (a, pattern, b))
}

} // verus!
