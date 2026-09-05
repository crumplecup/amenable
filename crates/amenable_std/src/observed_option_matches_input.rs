//! The `Option`-wrapped observed-value identity postcondition, named
//! once instead of restated per model.
//!
//! Several Verus accommodation models for `core::iter` adapters whose
//! `.next()` returns `Option<i32>` establish the same narrow law: the
//! yielded `Option<i32>` is exactly `Some` of the value the model
//! started from. [`ObservedValueMatchesInput`](crate::ObservedValueMatchesInput)
//! already names the bare-scalar version of this claim; this is its
//! `Option`-wrapped counterpart, sharing one Verus `spec fn` plus one
//! named witness in the registry. `ObservedOptionMatchesInput` names this
//! contract type in the `amenable_core::Ensures`/`Requires` worklist.

use amenable_derive::Standard;

use crate::{RustStdProvenance, RustStdStandard, RustStdType};

/// An `Option<i32>` known to match exactly `Some` of the input value the
/// accommodation model started from.
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
pub struct ObservedOptionMatchesInput {
    value: i32,
}

impl ObservedOptionMatchesInput {
    /// Wrap an `i32` already known to be the value the model's
    /// `Option<i32>` observation matches.
    pub const fn new(value: i32) -> Self {
        Self { value }
    }

    /// The wrapped value.
    pub const fn value(&self) -> i32 {
        self.value
    }
}
