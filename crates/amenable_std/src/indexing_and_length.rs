//! The fixed-length-container indexing/length postcondition, named once
//! instead of restated per real type.
//!
//! An `[i32; 3]` array and the `&[i32]` slice viewing it both guarantee
//! the identical shape: `.len()` reports the compile-time-known element
//! count, and each index recovers the element actually stored there.
//! Independently hand-written at both real Creusot proof sites
//! (`amenable_creusot::rust_std::{verify_array_indexing_and_length,
//! verify_slice_indexing_and_length}`) as four identical `ensures`
//! clauses. `IndexingAndLength` names this contract type in the
//! `amenable_core::Ensures`/`Requires` worklist.

use amenable_derive::Standard;

use crate::{RustStdProvenance, RustStdStandard, RustStdType};

/// A container length known to match both `.len()`'s report and every
/// index recovering the element it was constructed with.
///
/// A derived claim about `usize`, not a fresh root authority -- its
/// evidence chain rests on `usize`'s own already-registered standard-
/// library provenance ([`RustStdStandard<usize>`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<usize>",
    basis_ctor = "RustStdStandard::<usize>::new()",
    provenance = "<usize as RustStdType>::provenance()",
    provenance_type = "RustStdProvenance"
)]
pub struct IndexingAndLength {
    value: usize,
}

impl IndexingAndLength {
    /// Wrap a length already known to match the container's real
    /// indexing/length behavior.
    pub const fn new(value: usize) -> Self {
        Self { value }
    }

    /// The wrapped length.
    pub const fn value(&self) -> usize {
        self.value
    }
}
