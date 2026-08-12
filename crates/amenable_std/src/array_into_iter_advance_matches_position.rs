//! The three-step array-iterator transition postcondition, named once
//! instead of restated inline.
//!
//! The Verus `array_into_iter` accommodation model has one step law:
//! depending on the pre-state position, `advance()` yields the matching
//! element or `None`, updates the cursor accordingly, and preserves the
//! stored elements. This contract type gives that law a registry
//! identity instead of leaving the helper predicate anonymous.
//! `ArrayIntoIterAdvanceMatchesPosition` is the sixteenth contract type
//! in the `amenable_core::Ensures`/`Requires` worklist.

use amenable_derive::Standard;

use crate::{RustStdProvenance, RustStdStandard, RustStdType};

/// A cursor position known to follow the three-element array `IntoIter`
/// model's one-step transition law.
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
pub struct ArrayIntoIterAdvanceMatchesPosition {
    value: u8,
}

impl ArrayIntoIterAdvanceMatchesPosition {
    /// Wrap a cursor position already known to satisfy the transition
    /// law.
    pub const fn new(value: u8) -> Self {
        Self { value }
    }

    /// The wrapped position.
    pub const fn value(&self) -> u8 {
        self.value
    }
}
