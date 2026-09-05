//! The borrowed-value-iterator yield-once postcondition, named once
//! instead of restated per real type.
//!
//! `Option::iter`/`iter_mut` and `Result::iter`/`iter_mut` all guarantee
//! the identical shape: the borrowed iterator yields a reference to the
//! contained value exactly once, then ends, and any write made through
//! that first reference is reflected in the final value afterward.
//! Independently hand-written at four real Creusot proof sites
//! (`amenable_creusot::rust_std::{
//! verify_option_iter_yields_zero_or_one_reference,
//! verify_option_iter_mut_writes_through_to_the_option,
//! verify_result_iter_yields_a_reference_to_the_ok_value,
//! verify_result_iter_mut_writes_through_to_the_result}`) as the
//! identical three-clause `ensures` shape, differing only in whether the
//! final value is wrapped in `Some` or `Ok`.
//! `IterYieldsValueOnceThenEnds` names this contract type in the
//! `amenable_core::Ensures`/`Requires` worklist.

use amenable_derive::Standard;

use crate::{RustStdProvenance, RustStdStandard, RustStdType};

/// An `i32` known to be the sole value a borrowed value-iterator yields
/// before ending, with any write through that first reference reflected
/// in the underlying container afterward.
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
pub struct IterYieldsValueOnceThenEnds {
    value: i32,
}

impl IterYieldsValueOnceThenEnds {
    /// Wrap an `i32` already known to be the sole value yielded.
    pub const fn new(value: i32) -> Self {
        Self { value }
    }

    /// The wrapped value.
    pub const fn value(&self) -> i32 {
        self.value
    }
}
