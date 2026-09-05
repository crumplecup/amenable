//! The owned-two-item-sequence postcondition, named once instead of
//! restated per real type.
//!
//! `LinkedList::into_iter` and `VecDeque::into_iter` both guarantee the
//! identical shape over a two-element accommodation model: the first
//! item, then the second, then `None`. Independently hand-written at
//! both real Creusot proof sites
//! (`amenable_creusot::rust_std::{
//! verify_linked_list_into_iter_yields_owned_values_in_order,
//! verify_vec_deque_into_iter_yields_owned_values_in_order}`) as a
//! byte-identical three-clause `ensures` shape.
//! `YieldsTwoValuesInOrderThenEnds` names this contract type in
//! the `amenable_core::Ensures`/`Requires` worklist.

use amenable_derive::Standard;

use crate::{RustStdProvenance, RustStdStandard, RustStdType};

/// An `i32` known to be the first of two values a consuming iterator
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
pub struct YieldsTwoValuesInOrderThenEnds {
    value: i32,
}

impl YieldsTwoValuesInOrderThenEnds {
    /// Wrap an `i32` already known to be yielded in order.
    pub const fn new(value: i32) -> Self {
        Self { value }
    }

    /// The wrapped value.
    pub const fn value(&self) -> i32 {
        self.value
    }
}
