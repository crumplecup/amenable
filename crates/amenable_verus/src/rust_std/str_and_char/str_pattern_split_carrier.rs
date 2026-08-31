//! Verus accommodation model for `core::str::{Split, SplitN,
//! SplitInclusive}` (monomorphized on `char`).
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s module doc explains that of the 11
//! `Pattern`-generic split/match types, these three verify directly
//! against a fixed representative string (`"a,b,c"`) via `.collect()`,
//! unlike the other 8 which need a bounded observation model — this
//! carrier states each resulting split directly, asserted via view
//! equality (`@`) since `str`'s own `PartialEq::eq` has no `vstd` spec
//! support. None of these functions are `Split`/`SplitN`/
//! `SplitInclusive` themselves — each proof is conditional: sound if
//! the real type refines the stated split, which `amenable_kani`'s own
//! harness for that exact type (checking the real type directly)
//! already confirms independently, for the identical example.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::text_view_matches_expected;

verus! {

/// `"a,b,c".split(',')` yields the substrings between matches, forward:
/// `"a"`, `"b"`, `"c"`.
pub fn verify_str_split_model_yields_substrings_between_pattern_matches() -> (result: (&'static str, &'static str, &'static str))
    ensures
        text_view_matches_expected(result.0@, "a"@),
        text_view_matches_expected(result.1@, "b"@),
        text_view_matches_expected(result.2@, "c"@),
{
    ("a", "b", "c")
}

/// `"a,b,c".splitn(2, ',')` stops after 2 substrings, leaving the
/// remainder unsplit: `"a"`, `"b,c"`.
pub fn verify_str_splitn_model_limits_to_n_substrings() -> (result: (&'static str, &'static str))
    ensures
        text_view_matches_expected(result.0@, "a"@),
        text_view_matches_expected(result.1@, "b,c"@),
{
    ("a", "b,c")
}

/// `"a,b,c".split_inclusive(',')` keeps each matched delimiter attached
/// to the end of the substring that precedes it: `"a,"`, `"b,"`, `"c"`.
pub fn verify_str_split_inclusive_model_keeps_the_delimiter_attached() -> (result: (&'static str, &'static str, &'static str))
    ensures
        text_view_matches_expected(result.0@, "a,"@),
        text_view_matches_expected(result.1@, "b,"@),
        text_view_matches_expected(result.2@, "c"@),
{
    ("a,", "b,", "c")
}

} // verus!
