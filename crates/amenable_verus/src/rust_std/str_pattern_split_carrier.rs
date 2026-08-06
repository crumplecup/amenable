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
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// `"a,b,c".split(',')` yields the substrings between matches, forward:
/// `"a"`, `"b"`, `"c"`.
pub fn verify_str_split_model_yields_substrings_between_pattern_matches() -> (result: (&'static str, &'static str, &'static str))
    ensures
        result.0@ =~= "a"@,
        result.1@ =~= "b"@,
        result.2@ =~= "c"@,
{
    ("a", "b", "c")
}

/// `"a,b,c".splitn(2, ',')` stops after 2 substrings, leaving the
/// remainder unsplit: `"a"`, `"b,c"`.
pub fn verify_str_splitn_model_limits_to_n_substrings() -> (result: (&'static str, &'static str))
    ensures
        result.0@ =~= "a"@,
        result.1@ =~= "b,c"@,
{
    ("a", "b,c")
}

/// `"a,b,c".split_inclusive(',')` keeps each matched delimiter attached
/// to the end of the substring that precedes it: `"a,"`, `"b,"`, `"c"`.
pub fn verify_str_split_inclusive_model_keeps_the_delimiter_attached() -> (result: (&'static str, &'static str, &'static str))
    ensures
        result.0@ =~= "a,"@,
        result.1@ =~= "b,"@,
        result.2@ =~= "c"@,
{
    ("a,", "b,", "c")
}

} // verus!
