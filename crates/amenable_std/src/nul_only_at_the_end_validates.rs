//! The nul-only-at-the-end validation disposition, named once instead of
//! restated per real type.
//!
//! `CString::from_vec_with_nul` and `CStr::from_bytes_with_nul` both
//! validate the identical shape: the input is accepted when its sole nul
//! byte is the final one, rejected when no nul is present at all, and
//! rejected when a nul appears before the end. Independently
//! hand-written at both real Creusot proof sites
//! (`amenable_creusot::rust_std::{
//! verify_from_vec_with_nul_requires_the_nul_only_at_the_end,
//! verify_from_bytes_with_nul_requires_the_nul_only_at_the_end}`) as the
//! identical three-flag `ensures` clause. `NulOnlyAtTheEndValidates` is
//! the tenth contract type in the `amenable_core::Ensures`/`Requires`
//! worklist.

use amenable_derive::Standard;

use crate::{RustStdProvenance, RustStdStandard, RustStdType};

/// A representative flag from the nul-only-at-the-end validation
/// disposition (`accepted`, `missing_nul_rejected`,
/// `interior_nul_rejected`) known to hold for the accommodation model's
/// chosen representative input.
///
/// A derived claim about `bool`, not a fresh root authority -- its
/// evidence chain rests on `bool`'s own already-registered standard-
/// library provenance ([`RustStdStandard<bool>`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<bool>",
    basis_ctor = "RustStdStandard::<bool>::new()",
    provenance = "<bool as RustStdType>::provenance()",
    provenance_type = "RustStdProvenance"
)]
pub struct NulOnlyAtTheEndValidates {
    holds: bool,
}

impl NulOnlyAtTheEndValidates {
    /// Wrap a validation-disposition flag already known to hold.
    pub const fn new(holds: bool) -> Self {
        Self { holds }
    }

    /// The wrapped flag.
    pub const fn holds(&self) -> bool {
        self.holds
    }
}
