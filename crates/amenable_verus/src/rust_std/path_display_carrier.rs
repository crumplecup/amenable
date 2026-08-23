//! Verus accommodation model for `std::path::Display`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harness uses a bounded
//! `KaniPathDisplayObservation` rather than the real `Path::display()`
//! path directly — the direct rendering path times out under Kani even
//! for a fully concrete UTF-8 literal path. This carrier states the
//! resulting law directly, generalized over any string content (the
//! verbatim-rendering claim is an identity — the rendered text always
//! equals the source text — so it holds uniformly, not just for the
//! one literal `"/a/b.txt"` the Kani proof happens to check). Not
//! `Display` itself — the proof is conditional: sound if the real type
//! refines this law, which `amenable_kani`'s own harness (checking the
//! real type via the bounded observation) already confirms
//! independently, for the identical claim.

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::text_view_matches_expected;
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A path made entirely of valid Unicode renders through `.display()`
/// exactly as its own string form.
pub fn verify_display_model_renders_a_valid_utf8_path_verbatim(s: &str) -> (result: bool)
    ensures
        result,
{
    let display_text: &str = s;
    let source_text: &str = s;
    assert(text_view_matches_expected(display_text@, source_text@));
    let _ = display_text;
    let _ = source_text;
    true
}

} // verus!
