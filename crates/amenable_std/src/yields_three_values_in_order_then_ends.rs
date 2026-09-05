//! The owned-three-item-sequence postcondition, named once instead of
//! restated per real type.
//!
//! `array::IntoIter` over `[i32; 3]` guarantees the same fixed shape its
//! accommodation model states: the first item, then the second, then the
//! third, then `None`. The current machine-checked proof site is the
//! Verus `array_into_iter` carrier, and this contract type is the named
//! witness that owns that bound in the registry instead of leaving the
//! four-clause postcondition anonymous.
//! `YieldsThreeValuesInOrderThenEnds` names this contract type in
//! the `amenable_core::Ensures`/`Requires` worklist.

use amenable_derive::Standard;

use crate::{RustStdProvenance, RustStdStandard, RustStdType};

/// An `i32` known to be the first of three values a consuming iterator
/// yields, in order, before ending.
///
/// A derived claim about `i32`, not a fresh root authority -- its
/// evidence chain rests on `i32`'s own already-registered standard-
/// library provenance ([`RustStdStandard<i32>`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as RustStdType>::provenance()",
    provenance_type = "RustStdProvenance"
)]
pub struct YieldsThreeValuesInOrderThenEnds {
    value: i32,
}

impl YieldsThreeValuesInOrderThenEnds {
    /// Wrap an `i32` already known to be yielded first in order.
    pub const fn new(value: i32) -> Self {
        Self { value }
    }

    /// The wrapped value.
    pub const fn value(&self) -> i32 {
        self.value
    }
}
