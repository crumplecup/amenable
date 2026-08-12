//! The three-element array-iterator initial-state postcondition, named
//! once instead of restated inline.
//!
//! The Verus `array_into_iter` accommodation model starts with the
//! supplied three values stored unchanged and the cursor at position
//! zero. This contract type gives that initialization law a registry
//! identity instead of leaving the helper predicate anonymous.
//! `ArrayIntoIterStartsAtFirstPosition` is the fifteenth contract type
//! in the `amenable_core::Ensures`/`Requires` worklist.

use amenable_derive::Standard;

use crate::{RustStdProvenance, RustStdStandard, RustStdType};

/// A cursor position known to be the initial position of the
/// three-element array `IntoIter` model.
///
/// A derived claim about `u8`, not a fresh root authority -- its
/// evidence chain rests on `u8`'s own already-registered standard-
/// library provenance ([`RustStdStandard<u8>`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<u8>",
    basis_ctor = "RustStdStandard::<u8>::new()",
    provenance = "<u8 as RustStdType>::provenance()",
    provenance_type = "RustStdProvenance"
)]
pub struct ArrayIntoIterStartsAtFirstPosition {
    value: u8,
}

impl ArrayIntoIterStartsAtFirstPosition {
    /// Wrap a cursor position already known to be the initial state.
    pub const fn new(value: u8) -> Self {
        Self { value }
    }

    /// The wrapped position.
    pub const fn value(&self) -> u8 {
        self.value
    }
}
