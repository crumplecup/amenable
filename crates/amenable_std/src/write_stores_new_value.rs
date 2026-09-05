//! The write-through postcondition for plain value-box models, named once
//! instead of restated per model.
//!
//! Several Verus accommodation models perform the same simple write law:
//! after writing a new `i32` into the modeled slot, the subsequently
//! observed stored value is exactly that new value. `Cell` states it for
//! `set`/`replace`; `RefCell` and `UnsafeCell` state it for write-through
//! operations; ordered-pair mutable-iterator carriers state the same law
//! on the field they just wrote. The proof sites now share one Verus
//! `spec fn` for that claim, and this contract type is the named witness
//! that owns the bound in the registry.
//! `WriteStoresNewValue` names this contract type in the
//! `amenable_core::Ensures`/`Requires` worklist — Verus-only, since this
//! exact write-through idiom is expressed in Verus's explicit `final(...)`
//! style here.

use amenable_derive::Standard;

use crate::{RustStdProvenance, RustStdStandard, RustStdType};

/// An `i32` known to have been written through to the modeled storage
/// slot.
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
pub struct WriteStoresNewValue {
    value: i32,
}

impl WriteStoresNewValue {
    /// Wrap an `i32` already known to be the value just written through.
    pub const fn new(value: i32) -> Self {
        Self { value }
    }

    /// The wrapped value.
    pub const fn value(&self) -> i32 {
        self.value
    }
}
