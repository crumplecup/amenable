//! Verus accommodation model for `std::io::{Empty, Repeat, Sink}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harnesses check the real
//! `std::io::empty()`/`std::io::repeat(byte)`/`std::io::sink()` handles
//! directly (no bounded observation needed — these are trivial,
//! self-contained laws), and this carrier states the identical laws:
//! `Empty::read` always reports zero bytes read regardless of the
//! buffer requested; `Repeat::read` always fills every requested slot
//! with the given byte; `Sink::write` always reports the full length
//! written and discards the content. None of these functions are
//! `Empty`/`Repeat`/`Sink` themselves — each proof is conditional:
//! sound if the real type refines the stated law, which
//! `amenable_kani`'s own harness for that exact type (checking the real
//! type directly) already confirms independently, for the identical
//! claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::{
    observed_quad_matches_input, observed_value_matches_input,
};

verus! {

/// A singleton claim: `Empty::read` always reports the literal `0`
/// bytes read. Named, not inlined, so the assumption has an explicit
/// source even though nothing else calls it.
pub open spec fn empty_read_reports_zero_bytes(bytes_read: u32) -> bool {
    bytes_read == 0
}

/// `Empty::read` always reports zero bytes read, regardless of the
/// length of the buffer offered to it.
pub fn verify_empty_model_read_reports_end_of_file(requested_len: u32) -> (result: u32)
    ensures
        empty_read_reports_zero_bytes(result),
{
    let _ = requested_len;
    0
}

/// `Repeat::read` fills every one of the four requested slots with the
/// given byte.
pub fn verify_repeat_model_fills_the_buffer_with_the_given_byte(byte: u8) -> (result: (u8, u8, u8, u8))
    ensures
        observed_quad_matches_input(result, (byte, byte, byte, byte)),
{
    (byte, byte, byte, byte)
}

/// `Sink::write` always reports the full requested length as written,
/// regardless of content.
pub fn verify_sink_model_write_reports_full_length_and_discards_content(requested_len: u32) -> (result: u32)
    ensures
        observed_value_matches_input(result as int, requested_len as int),
{
    requested_len
}

} // verus!
