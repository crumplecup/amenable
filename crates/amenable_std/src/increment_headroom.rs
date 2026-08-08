//! The arithmetic-overflow-headroom precondition, named once instead of
//! restated per model.
//!
//! Several Verus accommodation models increment a symbolic `i32` (once, or
//! a fixed small number of times) to produce their next representative
//! value, and Verus checks arithmetic overflow by default — so each one
//! states its own "there's enough room below `i32::MAX` to do this safely"
//! precondition. The exact margin varies with how much headroom each
//! model actually needs (bare `i32::MAX` for a single-step transform that
//! reads the value without re-deriving a next one, `i32::MAX - 1` for a
//! single increment, `i32::MAX - 10` for a chunk of a few), but it's the
//! identical *kind* of precondition each time, restated across
//! `iter_sequence_carrier`, `iter_stateful_carrier`, `iter_transform_carrier`,
//! `primitive_shapes_carrier`, and `slice_chunks_carrier` with no name
//! tying them together.
//! `IncrementHeadroom` is the eighth contract type in the
//! `amenable_core::Ensures`/`Requires` worklist — Verus-only, since this
//! precondition exists specifically because Verus checks overflow by
//! default; Kani/Creusot's own proofs over the same models don't need it.

use amenable_derive::Standard;

use crate::{RustStdProvenance, RustStdStandard, RustStdType};

/// An `i32` known to have enough headroom below `i32::MAX` for a Verus
/// accommodation model to increment safely without triggering an overflow
/// check.
///
/// A derived claim about `i32`, not a fresh root authority — its evidence
/// chain rests on `i32`'s own already-registered standard-library
/// provenance ([`RustStdStandard<i32>`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as RustStdType>::provenance()",
    provenance_type = "RustStdProvenance"
)]
pub struct IncrementHeadroom {
    value: i32,
}

impl IncrementHeadroom {
    /// Wrap an `i32` already known to have enough increment headroom.
    pub const fn new(value: i32) -> Self {
        Self { value }
    }

    /// The wrapped value.
    pub const fn value(&self) -> i32 {
        self.value
    }
}
