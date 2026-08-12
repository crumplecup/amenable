//! Verus accommodation model for `core::cell::UnsafeCell<i32>`.
//!
//! Same zero-`vstd`-coverage gap `cell_carrier.rs` documents. `UnsafeCell`'s
//! only raw accessor (`.get()`, returning `*mut T`) needs `unsafe` to
//! dereference, which this crate forbids
//! (`#![forbid(unsafe_code)]` in `lib.rs`) — the same restriction
//! `amenable_kani::rust_std::cell`'s own module doc comment notes,
//! sticking to the safe accessors (`get_mut`, `into_inner`) instead, which
//! this model mirrors: a plain value box, read/write-through
//! (`read_through`/`write_through` standing in for `*cell.get_mut()` in
//! read/write position — a returned `&mut i32` itself hits Verus's
//! prophecy-based mutable-reference-return machinery, an unrelated
//! complication this model sidesteps rather than fights), extracted via
//! `into_inner`. `VerusUnsafeCellModel` isn't `UnsafeCell` and doesn't
//! claim to be — the proof is conditional: sound if the real `UnsafeCell`
//! refines this law, which `amenable_kani`'s own
//! `verify_unsafe_cell_get_mut_and_into_inner_round_trip` harness
//! (checking the real `UnsafeCell<i32>` directly, through its own safe
//! accessors) already confirms independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

// The shared write-through postcondition `amenable_std::verus_witness`
// registers for several plain value-box models. Imported only when
// Verus keeps ghost/spec items.
#[cfg(verus_keep_ghost)]
use crate::rust_std::cell_carrier::write_stores_new_value;

verus! {

/// Models `UnsafeCell`'s safe-accessor interface — a plain value box —
/// not `UnsafeCell` itself.
pub struct VerusUnsafeCellModel {
    pub value: i32,
}

impl VerusUnsafeCellModel {
    pub fn new(initial: i32) -> (result: Self)
        ensures
            result.value == initial,
    {
        Self { value: initial }
    }

    /// Reads the stored value, mirroring `*cell.get_mut()` in a read
    /// position.
    pub fn read_through(&self) -> (result: i32)
        ensures
            result == self.value,
    {
        self.value
    }

    /// Writes the stored value, mirroring `*cell.get_mut() = new_value`.
    pub fn write_through(&mut self, new_value: i32)
        ensures
            write_stores_new_value(new_value as int, final(self).value as int),
    {
        self.value = new_value;
    }

    /// Consumes the model and returns the current value.
    pub fn into_inner(self) -> (result: i32)
        ensures
            result == self.value,
    {
        self.value
    }
}

/// The stored value reads back correctly, a write through it is
/// visible, and `into_inner` returns the current value — the
/// get/write/extract round trip law `UnsafeCell<i32>` is expected to
/// refine through its own safe accessors.
pub fn verify_unsafe_cell_model_get_mut_and_into_inner_round_trip(initial: i32, updated: i32) -> (result: (bool, i32))
    ensures
        result.0,
        result.1 == updated,
{
    let mut cell = VerusUnsafeCellModel::new(initial);
    let exposes_initial = cell.read_through() == initial;

    cell.write_through(updated);

    (exposes_initial, cell.into_inner())
}

} // verus!
