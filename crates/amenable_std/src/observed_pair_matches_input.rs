//! The pair-observation identity postcondition, named once instead of
//! restated per model.
//!
//! Several Verus accommodation models establish the same narrow law
//! over a two-element result: the pair they expose is exactly the pair
//! they started from. The surrounding stories differ (`Atomic*::new`/
//! `store`/`load` written out once per width, `IoSlice::new`'s
//! dereference, `Split`'s two-piece yield, and so on), and the pair's
//! own element types vary widely (`bool`, every integer width, `usize`)
//! — the shared Verus `spec fn` this contract type names,
//! `observed_pair_matches_input`, is generic over both slots rather
//! than fixed at one scalar type, unlike
//! [`ObservedValueMatchesInput`](crate::ObservedValueMatchesInput)'s
//! bare-scalar version. `ObservedPairMatchesInput` names this
//! contract type in the `amenable_core::Ensures`/`Requires` worklist.

use amenable_derive::Standard;

use crate::{RustStdProvenance, RustStdStandard, RustStdType};

/// An `i32` known to match exactly the input value an accommodation
/// model's pair-shaped observation started from.
///
/// A derived claim about `i32`, not a fresh root authority — its
/// evidence chain rests on `i32`'s own already-registered
/// standard-library provenance ([`RustStdStandard<i32>`]). The wrapped
/// value stands in for either slot of the pair the named Verus
/// predicate actually checks generically; this type exists to carry
/// registry evidence, not to model the pair itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as RustStdType>::provenance()",
    provenance_type = "RustStdProvenance"
)]
pub struct ObservedPairMatchesInput {
    value: i32,
}

impl ObservedPairMatchesInput {
    /// Wrap an `i32` already known to match one slot of the input pair
    /// observed by the model.
    pub const fn new(value: i32) -> Self {
        Self { value }
    }

    /// The wrapped value.
    pub const fn value(&self) -> i32 {
        self.value
    }
}
