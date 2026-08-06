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
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// Models `OnceCell`'s set-exactly-once rule — not `OnceCell` itself.
pub struct VerusOnceCellModel {
    pub value: Option<i32>,
}

impl VerusOnceCellModel {
    /// A fresh cell has no value.
    pub fn empty() -> (result: Self)
        ensures
            result.value is None,
    {
        Self { value: None }
    }

    /// Reads back the currently stored value, if any.
    pub fn get(&self) -> (result: Option<i32>)
        ensures
            result == self.value,
    {
        self.value
    }

    /// Succeeds and stores the value only while empty; a second `set`
    /// leaves the original value undisturbed.
    pub fn set(&mut self, new_value: i32) -> (result: bool)
        ensures
            old(self).value is None ==> result && final(self).value == Some(new_value),
            old(self).value is Some ==> !result && final(self).value == old(self).value,
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
        result.2 == Some(value),
        !result.3,
        result.4 == Some(value),
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
