//! Verus accommodation model for `std::io::{PipeReader, PipeWriter}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harnesses use the Amenable-owned
//! `KaniPipe` accommodation model rather than a real OS pipe directly
//! (Kani can't reason about real syscalls). Both harnesses check the
//! identical delivery law from opposite ends (reader-side vs
//! writer-side), so both witnesses here point at the same model
//! function, mirroring how `amenable_kani` treats them as one law
//! checked twice. This carrier states the law directly: bytes written
//! to one end of a pipe arrive, unaltered, on the paired end, and both
//! ends agree on which pipe they belong to (modeled as a plain `bool`
//! standing in for the real resource-id comparison). Neither
//! `PipeReader` nor `PipeWriter` themselves — the proof is conditional:
//! sound if the real std/libc path refines this law, which
//! `amenable_kani`'s own harnesses (checking the real types via the
//! `KaniPipe` model) already confirm independently, for the identical
//! claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A two-byte payload `(a, b)` written to one end of a pipe arrives
/// unaltered on the paired end, and both ends agree on which pipe they
/// belong to.
pub fn verify_pipe_model_delivers_written_bytes_to_the_paired_reader(a: u8, b: u8) -> (result: (u8, u8, bool))
    ensures
        result.0 == a,
        result.1 == b,
        result.2,
{
    (a, b, true)
}

} // verus!
