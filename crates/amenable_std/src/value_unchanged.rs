//! The bookkeeping-only-write frame condition, named once instead of
//! restated per model.
//!
//! `RefCell`'s borrow-state operations (`try_borrow`, `try_borrow_mut`,
//! `release_shared`) and `Weak`'s `drop_strong` each mutate only their own
//! bookkeeping field (`borrow_state`/`strong_count`) and leave the payload
//! `value` field untouched — a frame condition each accommodation model
//! restates identically (`final(self).value == old(self).value`) rather
//! than naming once. `RefCell` already carries its own canonical
//! `Ensures<VerusVerifier>` claim (the write-through postcondition
//! `final(self).value == new_value`, homed as the seventh worklist item),
//! so this distinct claim — value stays put, rather than value becomes
//! something new — needs its own contract type instead of a second impl
//! on the same `RustStdStandard<RefCell<i32>>`.
//! `ValueUnchanged` is the ninth contract type in the
//! `amenable_core::Ensures`/`Requires` worklist — Verus-only, since this
//! frame condition is Verus's own explicit `old`/`final` idiom; Kani/
//! Creusot's own proofs over the same models don't state it this way.

use amenable_derive::Standard;

use crate::{RustStdProvenance, RustStdStandard, RustStdType};

/// An `i32` known to survive a bookkeeping-only operation unchanged — the
/// payload a `RefCell`/`Weak` accommodation model carries alongside state
/// it actually mutates.
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
pub struct ValueUnchanged {
    value: i32,
}

impl ValueUnchanged {
    /// Wrap an `i32` already known to survive the operation unchanged.
    pub const fn new(value: i32) -> Self {
        Self { value }
    }

    /// The wrapped value.
    pub const fn value(&self) -> i32 {
        self.value
    }
}
