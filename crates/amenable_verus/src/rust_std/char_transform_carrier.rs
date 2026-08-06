//! Verus accommodation model for `core::char::{ToLowercase, ToUppercase,
//! EscapeDebug, EscapeDefault, EscapeUnicode}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani::rust_std::char`'s own module doc comment
//! already explains why these adapters get fixed representative
//! examples rather than a symbolic law: `.collect()`ing even a
//! two-byte symbolic `EscapeAscii` iterator times out under Kani (see
//! `gallery::slice_escape_ascii`), so each Kani harness checks exactly
//! one concrete `char` at a time. This carrier mirrors that same
//! restricted shape for Verus — five small functions, one per adapter,
//! each returning the exact literal the corresponding Kani harness
//! checks (`'A'` lowercases to `"a"`, `'a'` uppercases to `"A"`, `'\n'`
//! debug/default-escapes to `"\\n"`, `'a'` unicode-escapes to
//! `"\\u{61}"`), asserted via view equality (`@`) rather than exec
//! `==` — `str`'s own `PartialEq::eq` has no `vstd` spec support either
//! (confirmed empirically: an exec `==` comparison between two `&str`
//! literals fails to verify), unlike `String`'s. None of these carry a
//! general algorithmic law (neither does the real Kani proof they're
//! conditional on) — the functions aren't `ToLowercase`/`ToUppercase`/
//! `EscapeDebug`/`EscapeDefault`/`EscapeUnicode` themselves, and the
//! proofs are conditional: sound if the real adapters refine these
//! fixed mappings, which `amenable_kani`'s own
//! `verify_to_lowercase_maps_an_uppercase_ascii_letter`/
//! `verify_to_uppercase_maps_a_lowercase_ascii_letter`/
//! `verify_char_escape_debug_escapes_a_newline`/
//! `verify_char_escape_default_escapes_a_newline`/
//! `verify_char_escape_unicode_renders_the_codepoint_escape` harnesses
//! (checking the real adapters directly) already confirm independently,
//! for the identical examples.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// `'A'.to_lowercase()` yields `"a"` — the fixed example `ToLowercase`
/// is expected to refine.
pub fn verify_to_lowercase_model_maps_an_uppercase_ascii_letter() -> (result: &'static str)
    ensures
        result@ =~= "a"@,
{
    "a"
}

/// `'a'.to_uppercase()` yields `"A"` — the fixed example `ToUppercase`
/// is expected to refine.
pub fn verify_to_uppercase_model_maps_a_lowercase_ascii_letter() -> (result: &'static str)
    ensures
        result@ =~= "A"@,
{
    "A"
}

/// `'\n'.escape_debug()` yields `"\n"` (the two-character escape,
/// backslash then `n`) — the fixed example `EscapeDebug` is expected to
/// refine.
pub fn verify_char_escape_debug_model_escapes_a_newline() -> (result: &'static str)
    ensures
        result@ =~= "\\n"@,
{
    "\\n"
}

/// `'\n'.escape_default()` yields `"\n"` (the two-character escape,
/// backslash then `n`) — the fixed example `EscapeDefault` is expected
/// to refine.
pub fn verify_char_escape_default_model_escapes_a_newline() -> (result: &'static str)
    ensures
        result@ =~= "\\n"@,
{
    "\\n"
}

/// `'a'.escape_unicode()` yields `"\u{61}"` — the fixed example
/// `EscapeUnicode` is expected to refine.
pub fn verify_char_escape_unicode_model_renders_the_codepoint_escape() -> (result: &'static str)
    ensures
        result@ =~= "\\u{61}"@,
{
    "\\u{61}"
}

} // verus!
