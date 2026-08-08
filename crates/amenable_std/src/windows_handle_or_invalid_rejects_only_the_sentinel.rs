//! The `HandleOrInvalid`-conversion postcondition, named once instead of
//! restated inline.
//!
//! `amenable_kani::os_windows_model`'s Linux-side accommodation model for
//! `std::os::windows::io::HandleOrInvalid` checks a single claim: converting
//! to an owned handle fails exactly when the wrapped raw value is the
//! `INVALID_HANDLE_VALUE` sentinel, and preserves the value unchanged
//! otherwise. Only the failure half is a raw, unnamed boolean at its one
//! real site (`os_windows_model::verify_windows_handle_or_invalid_rejects_only_the_sentinel`)
//! today; the success half is already a comparison against a concrete
//! reconstructed value, not a bare literal.

use amenable_derive::Standard;

use crate::{RustStdProvenance, RustStdStandard, RustStdType};

/// A raw handle value known to make `HandleOrInvalid`'s conversion fail
/// exactly when it equals the `INVALID_HANDLE_VALUE` sentinel — the
/// postcondition `amenable_kani`'s Windows accommodation model checks
/// about its one real proof site.
///
/// A derived claim about `isize` (the modeled raw handle representation,
/// see `amenable_kani::os_windows_model`'s own doc comment), not a fresh
/// root authority — its evidence chain rests on `isize`'s own already-
/// registered standard-library provenance ([`RustStdStandard<isize>`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<isize>",
    basis_ctor = "RustStdStandard::<isize>::new()",
    provenance = "<isize as RustStdType>::provenance()",
    provenance_type = "RustStdProvenance"
)]
pub struct WindowsHandleOrInvalidRejectsOnlyTheSentinel {
    value: isize,
}

impl WindowsHandleOrInvalidRejectsOnlyTheSentinel {
    /// Wrap a raw handle value already known to make conversion fail
    /// exactly at the sentinel.
    pub const fn new(value: isize) -> Self {
        Self { value }
    }

    /// The wrapped raw handle value.
    pub const fn value(&self) -> isize {
        self.value
    }
}
