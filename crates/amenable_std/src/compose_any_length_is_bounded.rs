//! The `#[derive(KaniCompose)]` `kani_any()` bounded-length postcondition,
//! named once instead of restated per field.
//!
//! `KaniCompose`'s derive macro generates a bounded `kani_any()` for enums:
//! every collection-shaped field it produces stays within a small fixed
//! bound so the harness terminates. `amenable_kani::compose`'s own
//! self-test checks this three times for `DerivedChoice`'s
//! variant fields (a `String` bounded to 4, a `Vec<i32>` bounded to 3,
//! another `String` bounded to 4) with no name tying the three checks
//! together as the same claim.

use amenable_derive::Standard;

use crate::{RustStdProvenance, RustStdStandard, RustStdType};

/// A length known to stay within a `#[derive(KaniCompose)]`-generated
/// `kani_any()`'s declared bound.
///
/// A derived claim about `usize`, not a fresh root authority — its
/// evidence chain rests on `usize`'s own already-registered standard-
/// library provenance ([`RustStdStandard<usize>`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<usize>",
    basis_ctor = "RustStdStandard::<usize>::new()",
    provenance = "<usize as RustStdType>::provenance()",
    provenance_type = "RustStdProvenance"
)]
pub struct ComposeAnyLengthIsBounded {
    value: usize,
}

impl ComposeAnyLengthIsBounded {
    /// Wrap a length already known to stay within its declared bound.
    pub const fn new(value: usize) -> Self {
        Self { value }
    }

    /// The wrapped length.
    pub const fn value(&self) -> usize {
        self.value
    }
}
