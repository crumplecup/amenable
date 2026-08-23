//! Verus accommodation model for `core::cell::RefMut<'static, i32>`.
//!
//! Same zero-`vstd`-coverage gap the other cell-family carriers document.
//! `RefMut`'s whole contract is "derefs to the guarded value, and a
//! write through it is visible once the borrow ends," which is already
//! exactly what `ref_cell_carrier.rs`'s `VerusRefCellModel::
//! try_borrow_mut`/`release_exclusive` model — this carrier reuses that
//! model directly and proves the specific deref/write-through claim
//! `RefMut` makes. Conditional on the real `RefMut` refining this law,
//! which `amenable_kani`'s own
//! `verify_ref_mut_derefs_and_writes_through_to_the_cell` harness
//! (checking the real `RefMut<'static, i32>`, produced by the real
//! `RefCell::borrow_mut`, directly) already confirms independently, for
//! the identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

use crate::rust_std::ref_cell_carrier::VerusRefCellModel;

verus! {

/// An exclusive borrow, once granted, exposes the cell's current value,
/// and a write made through it is visible once the borrow ends — the
/// deref/write-through claim `RefMut` is expected to refine.
pub fn verify_ref_mut_model_derefs_and_writes_through_to_the_cell(initial: i32, updated: i32) -> (result: bool)
    ensures
        result,
{
    let mut cell = VerusRefCellModel::new(initial);
    let borrowed = cell.try_borrow_mut();
    let derefs_to_initial = cell.value == initial;

    cell.release_exclusive(updated);
    let visible_after_release = cell.value == updated;

    borrowed && derefs_to_initial && visible_after_release
}

} // verus!
