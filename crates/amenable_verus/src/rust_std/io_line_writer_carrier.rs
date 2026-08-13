//! Verus accommodation model for `std::io::LineWriter<Vec<u8>>`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harness uses a bounded
//! `KaniLineWriterObservation` rather than the real `LineWriter` path
//! directly. This carrier states the resulting law directly: a line
//! ending in a newline (byte value 10) reaches the underlying writer
//! immediately, a trailing partial line stays buffered until flush
//! (the same two bytes as after the newline write — nothing new has
//! been delivered yet), and flush then delivers the trailing byte too.
//! Not `LineWriter` itself — the proof is conditional: sound if the
//! real type refines this law, which `amenable_kani`'s own harness
//! (checking the real type via the bounded observation) already
//! confirms independently, for the identical claim.

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::observed_pair_matches_input;
use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// `(after_newline_write, buffered_before_flush, after_flush)`.
type LineWriterResult = ((u8, u8), (u8, u8), (u8, u8, u8));

/// For a non-newline `line_byte` followed by a newline, then a
/// non-newline `trailing_byte`: the newline-terminated write delivers
/// `(line_byte, 10)` immediately, the state just before flush is still
/// exactly `(line_byte, 10)` (the trailing byte hasn't arrived yet), and
/// after flush the underlying writer holds `(line_byte, 10,
/// trailing_byte)`.
pub fn verify_line_writer_model_flushes_on_a_newline_but_not_before_one(line_byte: u8, trailing_byte: u8) -> (result: LineWriterResult)
    requires
        line_byte != 10,
        trailing_byte != 10,
    ensures
        observed_pair_matches_input(result.0, (line_byte, 10u8)),
        observed_pair_matches_input(result.1, (line_byte, 10u8)),
        result.2 == (line_byte, 10u8, trailing_byte),
{
    ((line_byte, 10), (line_byte, 10), (line_byte, 10, trailing_byte))
}

} // verus!
