//! Verus accommodation model for `core::cell::Cell<i32>`.
//!
//! `vstd` gives `Cell` no spec support at all — the same kind of
//! zero-coverage gap `binary_heap_carrier.rs`/`linked_list_carrier.rs`
//! document. `Cell`'s real implementation goes through `UnsafeCell`
//! (interior mutability via raw-pointer access), which this crate can't
//! exercise directly anyway (`#![forbid(unsafe_code)]` in `lib.rs`, same
//! restriction `amenable_kani`'s own `cell.rs` module doc comment notes
//! for its real-`Cell` proof). Writing `assume_specification`s for
//! `Cell::get`/`set`/`replace` ourselves would still be a new, unaudited
//! trust assumption about real interior-mutability internals. Instead
//! this carrier models `Cell`'s whole interface — a plain get/set-by-value
//! box, nothing more — in real code Verus checks completely.
//! `VerusCellModel` isn't `Cell` and doesn't claim to be — the proof is
//! conditional: sound if the real `Cell` refines this law, which
//! `amenable_kani`'s own `verify_cell_get_set_replace_take_round_trip`
//! harness (checking the real `Cell<i32>` directly) already confirms
//! independently, for the identical get/set/replace round trip.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// The shared write-through law for plain value-box models: after a
/// write, the observed stored slot is exactly the value that was
/// written.
pub open spec fn write_stores_new_value(new_value: int, observed: int) -> bool {
    observed == new_value
}

/// `Cell::new` stores the initial value in the model.
pub open spec fn cell_model_new_stores_initial_value(observed: int, initial: int) -> bool {
    observed == initial
}

/// `Cell::get` reads back the current modeled value.
pub open spec fn cell_model_get_reads_current_value(observed: int, current: int) -> bool {
    observed == current
}

/// `Cell::replace` returns the previous modeled value.
pub open spec fn cell_model_replace_returns_previous_value(observed: int, previous: int) -> bool {
    observed == previous
}

/// The get/set/replace round-trip law the `Cell` accommodation model
/// establishes.
pub open spec fn cell_model_get_set_replace_round_trip_holds(
    stores_initial: bool,
    set_overwrites: bool,
    replace_returns_old: bool,
    replace_stores_new: bool,
) -> bool {
    stores_initial && set_overwrites && replace_returns_old && replace_stores_new
}

/// Models `Cell`'s whole interface — get/set-by-value, nothing more —
/// not `Cell` itself.
pub struct VerusCellModel {
    pub value: i32,
}

impl VerusCellModel {
    /// `new` stores the initial value.
    pub fn new(initial: i32) -> (result: Self)
        ensures
            cell_model_new_stores_initial_value(result.value as int, initial as int),
    {
        Self { value: initial }
    }

    /// `get` reads back the currently stored value.
    pub fn get(&self) -> (result: i32)
        ensures
            cell_model_get_reads_current_value(result as int, self.value as int),
    {
        self.value
    }

    /// `set` overwrites the stored value.
    pub fn set(&mut self, new_value: i32)
        ensures
            write_stores_new_value(new_value as int, final(self).value as int),
    {
        self.value = new_value;
    }

    /// `replace` overwrites the stored value and hands back the old one.
    pub fn replace(&mut self, new_value: i32) -> (result: i32)
        ensures
            cell_model_replace_returns_previous_value(result as int, old(self).value as int),
            write_stores_new_value(new_value as int, final(self).value as int),
    {
        let old_value = self.value;
        self.value = new_value;
        old_value
    }
}

/// `new` stores the initial value, `set` overwrites it, and `replace`
/// overwrites it while handing back the previous value — the
/// get/set/replace round trip law the real `Cell<i32>` is expected to
/// refine.
pub fn verify_cell_model_get_set_replace_round_trip(initial: i32, updated: i32, replacement: i32) -> (result: (bool, bool, bool, bool))
    ensures
        cell_model_get_set_replace_round_trip_holds(result.0, result.1, result.2, result.3),
{
    let mut cell = VerusCellModel::new(initial);
    let stores_initial = cell.get() == initial;

    cell.set(updated);
    let set_overwrites = cell.get() == updated;

    let old = cell.replace(replacement);
    let replace_returns_old = old == updated;
    let replace_stores_new = cell.get() == replacement;

    (stores_initial, set_overwrites, replace_returns_old, replace_stores_new)
}

} // verus!
