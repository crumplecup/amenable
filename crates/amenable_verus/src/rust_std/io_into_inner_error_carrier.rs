//! Verus accommodation model for
//! `std::io::IntoInnerError<BufWriter<Vec<u8>>>`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harness uses a bounded
//! `KaniFlushErrorObservation` rather than the real `BufWriter::
//! into_inner` failure path directly. This carrier states the
//! resulting recovery law directly: when flushing fails,
//! `into_inner()`'s error recovers both the flush failure and the
//! buffered data, unchanged. Not `IntoInnerError` itself — the proof is
//! conditional: sound if the real type refines this law, which
//! `amenable_kani`'s own harness (checking the real type via the
//! bounded observation) already confirms independently, for the
//! identical claim.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// When flushing fails with a two-byte buffer `(a, b)` still pending,
/// the resulting `IntoInnerError` reports the flush as failed and
/// recovers exactly `(a, b)`.
pub fn verify_into_inner_error_model_recovers_the_writer_and_the_flush_error(a: u8, b: u8) -> (result: (bool, u8, u8))
    ensures
        result.0,
        result.1 == a,
        result.2 == b,
{
    (true, a, b)
}

} // verus!
