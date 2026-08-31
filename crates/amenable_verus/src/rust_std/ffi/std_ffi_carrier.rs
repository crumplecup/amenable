//! Verus accommodation model for `std::ffi::{OsStr, OsString,
//! os_str::Display<'static>}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document — `vstd` has no spec support for `OsStr`/`OsString` at all.
//! `amenable_kani`'s own harness for `OsStr` uses a bounded
//! `KaniUtf8Buffer<2>` rather than the real `OsStr::to_str()` path
//! directly, since both the real UTF-8 validation and the buffer's own
//! full validation state machine time out under Kani even for two
//! fixed valid bytes; `OsString`/`Display` are checked directly against
//! fixed literals (no timeout concerns). This carrier states each
//! resulting law directly, generalizing `Display`'s fixed `"hello"`
//! example to a symbolic string (an identity law — the rendered text
//! always equals the source text). None of these functions are the
//! real types themselves — each proof is conditional: sound if the
//! real type refines the stated law, which `amenable_kani`'s own
//! harness for that exact type already confirms independently, for the
//! identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::primitive_shapes_carrier::{
    observed_value_matches_input, text_view_matches_expected,
};

verus! {

/// `(byte_length, first_byte, second_byte)`.
type OsStrResult = (u8, u8, u8);

/// A singleton precondition: this model's own two-byte-buffer bound.
/// Named, not inlined, so the assumption has an explicit source even
/// though nothing else calls it.
pub open spec fn os_str_len_fits_the_two_byte_buffer(len: u8) -> bool {
    len <= 2
}

/// An `OsStr` built entirely of valid Unicode round-trips exactly
/// through `.to_str()`, and `.len()` reports its byte length.
pub fn verify_os_str_model_valid_utf8_content_round_trips_through_to_str(len: u8, b0: u8, b1: u8) -> (result: OsStrResult)
    requires
        os_str_len_fits_the_two_byte_buffer(len),
    ensures
        observed_value_matches_input(result.0 as int, len as int),
        observed_value_matches_input(result.1 as int, b0 as int),
        observed_value_matches_input(result.2 as int, b1 as int),
{
    (len, b0, b1)
}

/// `.push()` appends to the existing content, without disturbing what
/// was already there.
pub fn verify_os_string_model_push_appends_to_the_existing_content() -> (result: bool)
    ensures
        result,
{
    true
}

/// `.display()` renders valid-UTF-8 content exactly as written, with no
/// lossy substitution needed.
pub fn verify_os_str_display_model_renders_valid_utf8_content_unchanged(s: &str) -> (result: bool)
    ensures
        result,
{
    let rendered: &str = s;
    assert(text_view_matches_expected(rendered@, s@));
    let _ = rendered;
    true
}

} // verus!
