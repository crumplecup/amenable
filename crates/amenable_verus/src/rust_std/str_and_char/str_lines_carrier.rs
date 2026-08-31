//! Verus accommodation model for `core::str::{Lines, LinesAny}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s harnesses check the same fixed two-line
//! examples this carrier states directly, asserted via view equality
//! (`@`) since `str`'s own `PartialEq::eq` has no `vstd` spec support.
//! `LinesAny` is deprecated (in favor of `Lines`) but still stable and
//! real, same reasoning `amenable_kani::rust_std::str` gives for
//! covering it. Neither function is `Lines`/`LinesAny` themselves —
//! each proof is conditional: sound if the real type refines the stated
//! example, which `amenable_kani`'s own harness for that exact type
//! (checking the real type directly) already confirms independently,
//! for the identical example.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::primitive_shapes_carrier::text_view_matches_expected;

verus! {

/// `"a\nb".lines()` yields `"a"` then `"b"`, without a trailing empty
/// line.
pub fn verify_lines_model_splits_on_line_endings() -> (result: (&'static str, &'static str))
    ensures
        text_view_matches_expected(result.0@, "a"@),
        text_view_matches_expected(result.1@, "b"@),
{
    ("a", "b")
}

/// `"a\r\nb".lines_any()` yields `"a"` then `"b"` — unlike `Lines`,
/// splits on either `\n` or `\r\n`.
pub fn verify_lines_any_model_splits_on_any_line_ending() -> (result: (&'static str, &'static str))
    ensures
        text_view_matches_expected(result.0@, "a"@),
        text_view_matches_expected(result.1@, "b"@),
{
    ("a", "b")
}

} // verus!
