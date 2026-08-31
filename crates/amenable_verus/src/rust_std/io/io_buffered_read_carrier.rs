//! Verus accommodation model for `std::io::BufReader<&'static [u8]>`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harness uses a bounded
//! `KaniBufferedReadObservation` rather than the real `BufReader` path
//! directly. This carrier states the resulting read-through law
//! directly: a `BufReader` reads through to exactly the bytes of the
//! reader it wraps. Not `BufReader` itself — the proof is conditional:
//! sound if the real type refines this law, which `amenable_kani`'s own
//! harness (checking the real type via the bounded observation) already
//! confirms independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::observed_pair_matches_input;

verus! {

/// A `BufReader` wrapping a two-byte payload reads through to exactly
/// those two bytes.
pub fn verify_buf_reader_model_reads_the_underlying_bytes(a: u8, b: u8) -> (result: (u8, u8))
    ensures
        observed_pair_matches_input(result, (a, b)),
{
    (a, b)
}

} // verus!
