//! Verus accommodation model for `core::cell::Ref<'static, i32>`.
//!
//! Same zero-`vstd`-coverage gap the other cell-family carriers document.
//! `Ref`'s whole contract is "derefs to the value the shared borrow it
//! guards was taken from," which is already exactly what
//! `ref_cell_carrier.rs`'s `VerusRefCellModel::try_borrow` grants access
//! to — this carrier reuses that model directly rather than
//! reimplementing borrow-state tracking, and proves the specific
//! deref claim `Ref` makes. Conditional on the real `Ref` refining this
//! law, which `amenable_kani`'s own
//! `verify_ref_derefs_to_the_borrowed_value` harness (checking the real
//! `Ref<'static, i32>`, produced by the real `RefCell::borrow`, directly)
//! already confirms independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

use crate::rust_std::cell_and_ref::VerusRefCellModel;

verus! {

/// A shared borrow, once granted, exposes the same value the cell was
/// holding — the deref claim `Ref` is expected to refine.
pub fn verify_ref_model_derefs_to_the_borrowed_value(value: i32) -> (result: bool)
    ensures
        result,
{
    let mut cell = VerusRefCellModel::new(value);
    let borrowed = cell.try_borrow();
    let derefs_to_value = cell.value == value;

    borrowed && derefs_to_value
}

} // verus!
