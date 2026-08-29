//! Verus accommodation model for `core::cell::RefCell<i32>`.
//!
//! Same zero-`vstd`-coverage gap `cell_carrier.rs` documents for `Cell`,
//! but `RefCell`'s defining behavior — a dynamically checked borrow rule
//! layered over `Cell`'s get/set semantics — is a real state machine
//! (shared-borrow count vs. exclusive borrow), not a plain value box, so
//! it earns its own model rather than reusing `VerusCellModel`.
//! `VerusRefCellModel` tracks borrow state as a plain `i32` (`0` =
//! unborrowed, positive = that many live shared borrows, `-1` = one live
//! exclusive borrow) instead of RAII guards — Verus proof functions
//! aren't `Drop`-glue-sensitive the way `amenable_kani`'s guard-scoped
//! harness is, so explicit `release_*` calls stand in for a guard going
//! out of scope. `VerusRefCellModel` isn't `RefCell` and doesn't claim to
//! be — the proof is conditional: sound if the real `RefCell` refines
//! this borrow-state law, which `amenable_kani`'s own
//! `verify_ref_cell_dynamic_borrow_rules` harness (checking the real
//! `RefCell<i32>` directly, through actual `Ref`/`RefMut` guards) already
//! confirms independently, for the identical rule.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

// The shared write-through postcondition `amenable_std::verus_witness`
// registers for several plain interior-mutability models. Imported only
// when Verus keeps ghost/spec items.
#[cfg(verus_keep_ghost)]
use crate::rust_std::cell_carrier::write_stores_new_value;
#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::{observed_value_matches_input, value_unchanged};

verus! {

/// Models `RefCell`'s dynamically checked borrow rule — a shared-borrow
/// count vs. exclusive borrow state machine layered over a plain value —
/// not `RefCell` itself.
pub struct VerusRefCellModel {
    pub value: i32,
    /// `0` = unborrowed, positive = that many live shared borrows, `-1`
    /// = one live exclusive borrow.
    pub borrow_state: i32,
}

/// `try_borrow`'s precondition: the live shared-borrow count has room
/// for one more without overflowing `i32`.
pub open spec fn try_borrow_headroom_holds(borrow_state: i32) -> bool {
    borrow_state < i32::MAX
}

/// `try_borrow`'s whole postcondition: succeeds and increments the
/// shared-borrow count unless an exclusive borrow is live, in which case
/// it fails and leaves the state untouched.
pub open spec fn try_borrow_result_matches(old_state: i32, result: bool, new_state: i32) -> bool {
    (old_state >= 0 ==> result && new_state == old_state + 1)
        && (old_state < 0 ==> !result && new_state == old_state)
}

/// `try_borrow_mut`'s whole postcondition: succeeds and enters the
/// exclusive state only while completely unborrowed, otherwise fails and
/// leaves the state untouched.
pub open spec fn try_borrow_mut_result_matches(old_state: i32, result: bool, new_state: i32) -> bool {
    (old_state == 0 ==> result && new_state == -1)
        && (old_state != 0 ==> !result && new_state == old_state)
}

/// `release_shared`'s precondition: at least one shared borrow is live
/// to release.
pub open spec fn release_shared_requires_a_live_shared_borrow(borrow_state: i32) -> bool {
    borrow_state > 0
}

/// `release_shared`'s borrow-count postcondition.
pub open spec fn release_shared_decrements_borrow_state(old_state: i32, new_state: i32) -> bool {
    new_state == old_state - 1
}

impl VerusRefCellModel {
    pub fn new(initial: i32) -> (result: Self)
        ensures
            observed_value_matches_input(result.value as int, initial as int),
            observed_value_matches_input(result.borrow_state as int, 0int),
    {
        Self { value: initial, borrow_state: 0 }
    }

    /// Succeeds (incrementing the shared-borrow count) unless an
    /// exclusive borrow is live. Bounds the live shared-borrow count
    /// below `i32::MAX` — a real ceiling no real program approaches, kept
    /// explicit rather than modeling unbounded borrow counts.
    pub fn try_borrow(&mut self) -> (result: bool)
        requires
            try_borrow_headroom_holds(old(self).borrow_state),
        ensures
            try_borrow_result_matches(old(self).borrow_state, result, final(self).borrow_state),
            value_unchanged(old(self).value as int, final(self).value as int),
    {
        if self.borrow_state >= 0 {
            self.borrow_state += 1;
            true
        } else {
            false
        }
    }

    /// Succeeds (entering the exclusive state) only while completely
    /// unborrowed.
    pub fn try_borrow_mut(&mut self) -> (result: bool)
        ensures
            try_borrow_mut_result_matches(old(self).borrow_state, result, final(self).borrow_state),
            value_unchanged(old(self).value as int, final(self).value as int),
    {
        if self.borrow_state == 0 {
            self.borrow_state = -1;
            true
        } else {
            false
        }
    }

    /// Releases one live shared borrow.
    pub fn release_shared(&mut self)
        requires
            release_shared_requires_a_live_shared_borrow(old(self).borrow_state),
        ensures
            release_shared_decrements_borrow_state(old(self).borrow_state, final(self).borrow_state),
            value_unchanged(old(self).value as int, final(self).value as int),
    {
        self.borrow_state -= 1;
    }

    /// Releases the live exclusive borrow, optionally writing through
    /// first (modeling `*borrow_mut() = new_value` before the guard
    /// drops).
    pub fn release_exclusive(&mut self, new_value: i32)
        requires
            observed_value_matches_input(old(self).borrow_state as int, -1int),
        ensures
            observed_value_matches_input(final(self).borrow_state as int, 0int),
            write_stores_new_value(new_value as int, final(self).value as int),
    {
        self.value = new_value;
        self.borrow_state = 0;
    }
}

/// A shared borrow blocks a mutable borrow until dropped, an exclusive
/// borrow blocks both shared and further exclusive borrows until
/// dropped, and a write made through an exclusive borrow is visible
/// afterward — the dynamic borrow-rule law `RefCell<i32>` is expected to
/// refine.
pub fn verify_ref_cell_model_dynamic_borrow_rules(initial: i32, updated: i32) -> (result: (bool, bool, bool, bool, bool, i32))
    ensures
        result.0,
        !result.1,
        result.2,
        !result.3,
        !result.4,
        observed_value_matches_input(result.5 as int, updated as int),
{
    let mut model = VerusRefCellModel::new(initial);

    let shared_ok = model.try_borrow();
    let mut_rejected_during_shared = model.try_borrow_mut();
    model.release_shared();

    let mut_ok_after_release = model.try_borrow_mut();
    let shared_rejected_during_exclusive = model.try_borrow();
    let second_mut_rejected_during_exclusive = model.try_borrow_mut();
    model.release_exclusive(updated);

    (
        shared_ok,
        mut_rejected_during_shared,
        mut_ok_after_release,
        shared_rejected_during_exclusive,
        second_mut_rejected_during_exclusive,
        model.value,
    )
}

} // verus!
