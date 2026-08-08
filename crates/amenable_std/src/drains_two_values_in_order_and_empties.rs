//! The draining-two-item-sequence postcondition, named once instead of
//! restated per real type.
//!
//! `LinkedList`'s FIFO back/front removal and `VecDeque::drain(..)` both
//! guarantee the identical shape over a two-element accommodation
//! model: the first item, then the second, then `None`, leaving the
//! container reporting empty. Independently hand-written at both real
//! Creusot proof sites (`amenable_creusot::rust_std::{
//! verify_linked_list_is_fifo_through_back_and_front,
//! verify_vec_deque_drain_removes_and_yields_in_order}`) as a
//! byte-identical four-clause `ensures` shape.
//! `DrainsTwoValuesInOrderAndEmpties` is the fourteenth contract type in
//! the `amenable_core::Ensures`/`Requires` worklist.

use amenable_derive::Standard;

use crate::{RustStdProvenance, RustStdStandard, RustStdType};

/// An `i32` known to be the first of two values a draining operation
/// yields, in order, before leaving the container empty.
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
pub struct DrainsTwoValuesInOrderAndEmpties {
    value: i32,
}

impl DrainsTwoValuesInOrderAndEmpties {
    /// Wrap an `i32` already known to be drained in order.
    pub const fn new(value: i32) -> Self {
        Self { value }
    }

    /// The wrapped value.
    pub const fn value(&self) -> i32 {
        self.value
    }
}
