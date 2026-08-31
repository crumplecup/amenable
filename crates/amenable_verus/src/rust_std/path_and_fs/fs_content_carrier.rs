//! Verus accommodation model for `std::fs::{File, FileTimes, Metadata}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document — the direct `std::fs` tempdir path crosses real OS-backed
//! filesystem state that Kani itself times out on. `amenable_kani`'s
//! own harnesses use an Amenable-owned filesystem model rather than a
//! real temp directory; this carrier states each resulting numeric law
//! directly. None of these functions are `File`/`FileTimes`/`Metadata`
//! themselves — each proof is conditional: sound if the real type
//! refines the stated law, which `amenable_kani`'s own harness for
//! that exact type (checking the real type via the bounded filesystem
//! model) already confirms independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

pub open spec fn file_model_write_then_read_round_trips_the_bytes(
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    result: (u8, u8, u8, u8),
) -> bool {
    result == (a, b, c, d)
}

/// Bytes written to a file and flushed are read back unchanged through
/// a fresh handle.
pub fn verify_file_model_write_then_read_round_trips_the_bytes(a: u8, b: u8, c: u8, d: u8) -> (result: (u8, u8, u8, u8))
    ensures
        file_model_write_then_read_round_trips_the_bytes(a, b, c, d, result),
{
    (a, b, c, d)
}

pub open spec fn file_times_model_sets_the_recorded_modification_time(
    target_unix_seconds: u64,
    result: u64,
) -> bool {
    result == target_unix_seconds
}

/// `.set_modified()` applied via `File::set_times()` is reflected
/// exactly in the file's metadata.
pub fn verify_file_times_model_sets_the_recorded_modification_time(target_unix_seconds: u64) -> (result: u64)
    ensures
        file_times_model_sets_the_recorded_modification_time(target_unix_seconds, result),
{
    target_unix_seconds
}

pub open spec fn metadata_model_reports_the_written_length(
    byte_count: u8,
    result: (u64, bool),
) -> bool {
    &&& result.0 == byte_count as u64
    &&& result.1 == (byte_count == 0)
}

/// `.len()` reports exactly the number of bytes written, and
/// `.is_empty()` agrees with whether that count is zero.
pub fn verify_metadata_model_reports_the_written_length(byte_count: u8) -> (result: (u64, bool))
    ensures
        metadata_model_reports_the_written_length(byte_count, result),
{
    (byte_count as u64, byte_count == 0)
}

} // verus!
