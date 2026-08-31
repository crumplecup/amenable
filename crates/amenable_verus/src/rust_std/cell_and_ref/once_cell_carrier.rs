//! Verus accommodation model for `core::cell::OnceCell<i32>`.
//!
//! Same zero-`vstd`-coverage gap `cell_carrier.rs`/`ref_cell_carrier.rs`
//! document. `OnceCell`'s whole contract is narrower than either —
//! accepts exactly one `set`, `None` until then — so `VerusOnceCellModel`
//! models it directly as an `Option<i32>` with a set-only-when-empty
//! rule. `VerusOnceCellModel` isn't `OnceCell` and doesn't claim to be —
//! the proof is conditional: sound if the real `OnceCell` refines this
//! law, which `amenable_kani`'s own
//! `verify_once_cell_initializes_exactly_once` harness (checking the
//! real `OnceCell<i32>` directly) already confirms independently, for
//! the identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::primitive_shapes_carrier::{
    observed_option_matches_input, values_are_equal,
};

verus! {

/// Models `OnceCell`'s set-exactly-once rule — not `OnceCell` itself.
pub struct VerusOnceCellModel {
    pub value: Option<i32>,
}

/// A freshly-constructed model has no stored value.
pub open spec fn once_cell_empty_has_no_value(result: VerusOnceCellModel) -> bool {
    result.value is None
}

/// `set`'s success conjunct: an empty cell accepts the new value and
/// stores it.
pub open spec fn once_cell_set_succeeds_when_empty(old_value: Option<i32>, new_value: i32, result: bool, final_value: Option<i32>) -> bool {
    old_value is None ==> result && final_value == Some(new_value)
}

/// `set`'s rejection conjunct: an already-occupied cell rejects the new
/// value, leaving the original undisturbed.
pub open spec fn once_cell_set_rejected_when_occupied(old_value: Option<i32>, result: bool, final_value: Option<i32>) -> bool {
    old_value is Some ==> !result && final_value == old_value
}

impl VerusOnceCellModel {
    /// A fresh cell has no value.
    pub fn empty() -> (result: Self)
        ensures
            once_cell_empty_has_no_value(result),
    {
        Self { value: None }
    }

    /// Reads back the currently stored value, if any.
    pub fn get(&self) -> (result: Option<i32>)
        ensures
            values_are_equal(result, self.value),
    {
        self.value
    }

    /// Succeeds and stores the value only while empty; a second `set`
    /// leaves the original value undisturbed.
    pub fn set(&mut self, new_value: i32) -> (result: bool)
        ensures
            once_cell_set_succeeds_when_empty(old(self).value, new_value, result, final(self).value),
            once_cell_set_rejected_when_occupied(old(self).value, result, final(self).value),
    {
        if self.value.is_none() {
            self.value = Some(new_value);
            true
        } else {
            false
        }
    }
}

/// A fresh cell has no value, the first `set` succeeds and is
/// immediately visible through `get`, and a second `set` is rejected
/// without disturbing the value the first one stored — the
/// set-exactly-once law `OnceCell<i32>` is expected to refine.
pub fn verify_once_cell_model_initializes_exactly_once(value: i32, other: i32) -> (result: (bool, bool, Option<i32>, bool, Option<i32>))
    ensures
        result.0,
        result.1,
        observed_option_matches_input(result.2, value),
        !result.3,
        observed_option_matches_input(result.4, value),
{
    let mut cell = VerusOnceCellModel::empty();
    let starts_empty = cell.get().is_none();

    let first_set_succeeds = cell.set(value);
    let get_after_first_set = cell.get();

    let second_set_rejected = cell.set(other);
    let value_survives = cell.get();

    (starts_empty, first_set_succeeds, get_after_first_set, second_set_rejected, value_survives)
}

} // verus!
