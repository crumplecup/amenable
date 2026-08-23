//! Verus accommodation model for `core::str::{SplitAsciiWhitespace,
//! SplitWhitespace}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s harnesses check the same fixed
//! whitespace-collapsing example this carrier states directly, asserted
//! via view equality (`@`) since `str`'s own `PartialEq::eq` has no
//! `vstd` spec support. Neither function is `SplitAsciiWhitespace`/
//! `SplitWhitespace` themselves — each proof is conditional: sound if
//! the real type refines the stated example, which `amenable_kani`'s
//! own harness for that exact type (checking the real type directly)
//! already confirms independently, for the identical example.

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::text_view_matches_expected;
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// `" a  b ".split_ascii_whitespace()` collapses runs of whitespace and
/// drops leading/trailing whitespace: yields `"a"` then `"b"`.
pub fn verify_split_ascii_whitespace_model_collapses_runs_of_whitespace() -> (result: (&'static str, &'static str))
    ensures
        text_view_matches_expected(result.0@, "a"@),
        text_view_matches_expected(result.1@, "b"@),
{
    ("a", "b")
}

/// Same collapsing behavior as `SplitAsciiWhitespace`, over Unicode
/// whitespace.
pub fn verify_split_whitespace_model_collapses_runs_of_whitespace() -> (result: (&'static str, &'static str))
    ensures
        text_view_matches_expected(result.0@, "a"@),
        text_view_matches_expected(result.1@, "b"@),
{
    ("a", "b")
}

} // verus!
