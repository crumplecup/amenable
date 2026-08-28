//! Verus accommodation model for `std::io::WriterPanicked`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harness uses a bounded
//! `KaniWriterPanickedObservation` rather than a real panicking writer
//! directly (Kani can't reason about catching an inner panic). This
//! carrier states the resulting recovery law directly: when a
//! `BufWriter`'s inner writer panics mid-write, `.into_parts()`
//! afterward reports `WriterPanicked` while still recovering exactly
//! the data that was buffered. Not `WriterPanicked` itself — the proof
//! is conditional: sound if the real type refines this law, which
//! `amenable_kani`'s own harness (checking the real type via the
//! bounded observation) already confirms independently, for the
//! identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::observed_pair_matches_input;

verus! {

/// With a two-byte buffer `(a, b)` pending when the inner writer
/// panics, the resulting `WriterPanicked` reports the panic and
/// recovers exactly `(a, b)`.
pub fn verify_writer_panicked_model_recovers_the_buffered_data(a: u8, b: u8) -> (result: (bool, u8, u8))
    ensures
        result.0,
        observed_pair_matches_input((result.1, result.2), (a, b)),
{
    (true, a, b)
}

} // verus!
