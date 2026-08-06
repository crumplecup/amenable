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
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// `Empty::read` always reports zero bytes read, regardless of the
/// length of the buffer offered to it.
pub fn verify_empty_model_read_reports_end_of_file(requested_len: u32) -> (result: u32)
    ensures
        result == 0,
{
    let _ = requested_len;
    0
}

/// `Repeat::read` fills every one of the four requested slots with the
/// given byte.
pub fn verify_repeat_model_fills_the_buffer_with_the_given_byte(byte: u8) -> (result: (u8, u8, u8, u8))
    ensures
        result == (byte, byte, byte, byte),
{
    (byte, byte, byte, byte)
}

/// `Sink::write` always reports the full requested length as written,
/// regardless of content.
pub fn verify_sink_model_write_reports_full_length_and_discards_content(requested_len: u32) -> (result: u32)
    ensures
        result == requested_len,
{
    requested_len
}

} // verus!
