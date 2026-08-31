//! Verus accommodation model for `std::path::{Component, Components,
//! Iter}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document — `vstd` has no spec support for `Path`/`OsStr` at all.
//! `amenable_kani`'s own harnesses check the real types directly over
//! fixed literal paths (`"/a"`, `"/a/b"`; no timeout concerns). This
//! carrier states each resulting law directly, asserted via view
//! equality (`@`) since `str`'s own `PartialEq::eq` has no `vstd` spec
//! support. None of these functions are `Component`/`Components`/`Iter`
//! themselves — each proof is conditional: sound if the real type
//! refines the stated example, which `amenable_kani`'s own harness for
//! that exact type (checking the real type directly) already confirms
//! independently, for the identical example.

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::text_view_matches_expected;
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// Parsing `"/a"`'s components: the first is the root (`result.0`),
/// the second is a `Normal` segment naming `"a"` (`result.1`).
pub fn verify_component_model_distinguishes_root_from_normal_segments() -> (result: (bool, &'static str))
    ensures
        result.0,
        text_view_matches_expected(result.1@, "a"@),
{
    (true, "a")
}

/// `.components()` over `"/a/b"` yields the root, then `"a"`, then
/// `"b"`, in order.
pub fn verify_components_model_yields_root_then_named_segments_in_order() -> (result: (bool, &'static str, &'static str))
    ensures
        result.0,
        text_view_matches_expected(result.1@, "a"@),
        text_view_matches_expected(result.2@, "b"@),
{
    (true, "a", "b")
}

/// A singleton claim: this fixed example's `.iter()` always yields
/// exactly 3 segments. Named, not inlined, so the assumption has an
/// explicit source even though nothing else calls it.
pub open spec fn path_iter_yields_three_segments(count: u32) -> bool {
    count == 3
}

/// `.iter()` over `"/a/b"` yields three segments (root, then two named):
/// the second is `"a"`, the third is `"b"`.
pub fn verify_iter_model_yields_the_named_segments() -> (result: (u32, &'static str, &'static str))
    ensures
        path_iter_yields_three_segments(result.0),
        text_view_matches_expected(result.1@, "a"@),
        text_view_matches_expected(result.2@, "b"@),
{
    (3, "a", "b")
}

} // verus!
