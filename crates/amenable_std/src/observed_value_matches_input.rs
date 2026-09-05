//! The simple observed-value identity postcondition, named once instead
//! of restated per model.
//!
//! Several Verus accommodation models establish the same narrow law:
//! the `i32` they expose, dereference, receive, or read back is exactly
//! the `i32` they started from. The surrounding stories differ
//! (`Arc`/`Rc` deref, `Ready` resolution, channel delivery, tuple-field
//! roundtrip wrappers like `Wrapping`, and so on), but the observed
//! postcondition itself is identical and now has one shared Verus
//! `spec fn` plus one named witness in the registry.
//! `ObservedValueMatchesInput` names this contract type in the
//! `amenable_core::Ensures`/`Requires` worklist.

use amenable_derive::Standard;

use crate::{RustStdProvenance, RustStdStandard, RustStdType};

/// An `i32` known to match exactly the input value the accommodation
/// model started from.
///
/// A derived claim about `i32`, not a fresh root authority — its
/// evidence chain rests on `i32`'s own already-registered
/// standard-library provenance ([`RustStdStandard<i32>`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as RustStdType>::provenance()",
    provenance_type = "RustStdProvenance"
)]
pub struct ObservedValueMatchesInput {
    value: i32,
}

impl ObservedValueMatchesInput {
    /// Wrap an `i32` already known to match the input value observed by
    /// the model.
    pub const fn new(value: i32) -> Self {
        Self { value }
    }

    /// The wrapped value.
    pub const fn value(&self) -> i32 {
        self.value
    }
}
