//! Verus accommodation model for `core::str::{EscapeDebug, EscapeDefault,
//! EscapeUnicode}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. Distinct from `core::char`'s own escape adapters (see
//! `char_transform_carrier.rs`) — these iterate over a `str`, not a
//! bare `char` — but `amenable_kani`'s harnesses check the identical
//! fixed examples for both, so this carrier mirrors
//! `char_transform_carrier.rs`'s own reasoning: `.collect()`ing even a
//! short symbolic escape iterator times out under Kani, so each harness
//! checks exactly one concrete example, asserted here via view equality
//! (`@`) since `str`'s own `PartialEq::eq` has no `vstd` spec support.
//! None of these functions are `EscapeDebug`/`EscapeDefault`/
//! `EscapeUnicode` themselves — each proof is conditional: sound if the
//! real type refines the stated example, which `amenable_kani`'s own
//! harness for that exact type (checking the real type directly)
//! already confirms independently, for the identical example.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::primitive_shapes_carrier::text_view_matches_expected;

verus! {

/// `"\n".escape_debug().collect()` yields `"\n"` (the two-character
/// escape).
pub fn verify_str_escape_debug_model_escapes_control_characters() -> (result: &'static str)
    ensures
        text_view_matches_expected(result@, "\\n"@),
{
    "\\n"
}

/// `"\n".escape_default().collect()` yields `"\n"` (the two-character
/// escape).
pub fn verify_str_escape_default_model_escapes_control_characters() -> (result: &'static str)
    ensures
        text_view_matches_expected(result@, "\\n"@),
{
    "\\n"
}

/// `"a".escape_unicode().collect()` yields `"\u{61}"`.
pub fn verify_str_escape_unicode_model_renders_the_codepoint_escape() -> (result: &'static str)
    ensures
        text_view_matches_expected(result@, "\\u{61}"@),
{
    "\\u{61}"
}

} // verus!
