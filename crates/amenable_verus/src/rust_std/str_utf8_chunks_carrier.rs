//! Verus accommodation model for `core::str::{Utf8Chunks, Utf8Chunk,
//! Utf8Error}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s harnesses check fixed representative
//! byte sequences for `Utf8Chunks`/`Utf8Chunk`, and a symbolic invalid
//! lead byte (`0xF5..=0xFF`, never a valid UTF-8 lead byte anywhere,
//! against `amenable_kani`'s own UTF-8 accommodation model rather than
//! the real `std::str::from_utf8` path, which times out even for a
//! fixed two-byte invalid vector) for `Utf8Error`. This carrier states
//! each resulting law directly: the wholly-valid case for `Utf8Chunks`
//! (one chunk, no invalid bytes, no more chunks), the split for
//! `Utf8Chunk` (valid prefix plus the one bad byte), and the
//! valid-prefix-length/error-span claim for `Utf8Error`, generalized
//! over any lead byte in the invalid range the same way the Kani proof
//! is. None of these functions are `Utf8Chunks`/`Utf8Chunk`/
//! `Utf8Error` themselves — each proof is conditional: sound if the
//! real type refines the stated law, which `amenable_kani`'s own
//! harness for that exact type already confirms independently, for the
//! identical claim.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::text_view_matches_expected;

verus! {

/// Re-validating wholly valid UTF-8 bytes (`b"ab"`) yields exactly one
/// chunk: `valid() == "ab"`, `invalid()` is empty, and there is no
/// second chunk.
pub fn verify_utf8_chunks_model_yields_one_chunk_for_wholly_valid_input() -> (result: (&'static str, bool, bool))
    ensures
        text_view_matches_expected(result.0@, "ab"@),
        result.1,
        !result.2,
{
    ("ab", true, false)
}

/// For `b"ab\xFFcd"`, the first chunk's `valid()` is the UTF-8 prefix
/// `"ab"` before the bad byte, and `invalid()` is that one bad byte
/// (`0xFF`).
pub fn verify_utf8_chunk_model_separates_the_valid_prefix_from_invalid_bytes() -> (result: (&'static str, u8))
    ensures
        text_view_matches_expected(result.0@, "ab"@),
        result.1 == 0xFFu8,
{
    ("ab", 0xFFu8)
}

/// For `[b'a', b'b', invalid, b'c']` with `invalid` in `0xF5..=0xFF`
/// (never a valid UTF-8 lead byte anywhere, so it's a lone one-byte
/// error regardless of its neighbors): `valid_up_to() == 2` and
/// `error_len() == Some(1)`.
pub fn verify_utf8_error_model_reports_the_valid_prefix_length_and_error_span(invalid: u8) -> (result: (u8, u8))
    requires
        invalid >= 0xF5u8,
    ensures
        result.0 == 2,
        result.1 == 1,
{
    // `invalid` plays no role beyond appearing in the `requires` bound
    // (`0xF5..=0xFF` is a lone one-byte error regardless of its exact
    // value) — the claim doesn't depend on which byte in that range it is.
    let _ = invalid;
    (2, 1)
}

} // verus!
