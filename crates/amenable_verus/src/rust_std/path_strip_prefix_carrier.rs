//! Verus accommodation model for `std::path::StripPrefixError`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document — `vstd` has no spec support for `Path`/`OsStr` at all.
//! `amenable_kani`'s own harness checks the real `Path::strip_prefix`
//! directly over fixed literal paths (no timeout concerns): it fails
//! (producing this error) on a non-matching prefix, and succeeds,
//! removing the matching prefix, when it does match. This carrier
//! states the same fixed example directly, asserted via view equality
//! (`@`) since `str`'s own `PartialEq::eq` has no `vstd` spec support.
//! Not `StripPrefixError` itself — the proof is conditional: sound if
//! the real type refines this example, which `amenable_kani`'s own
//! `verify_strip_prefix_error_reports_a_non_matching_prefix` harness
//! (checking the real type directly) already confirms independently,
//! for the identical example.

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::text_view_matches_expected;
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// `Path::new("/a/b").strip_prefix("/x")` fails (`result.0`); `Path::
/// new("/a/b").strip_prefix("/a")` succeeds, leaving `"b"` (`result.1`).
pub fn verify_strip_prefix_error_model_reports_a_non_matching_prefix() -> (result: (bool, &'static str))
    ensures
        result.0,
        text_view_matches_expected(result.1@, "b"@),
{
    (true, "b")
}

} // verus!
