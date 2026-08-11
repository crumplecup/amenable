//! The `#[derive(KaniCompose)]` array-length postcondition, named once
//! instead of restated per depth.
//!
//! `KaniCompose`'s derive macro generates `kani_depth0()`/`kani_depth1()`/
//! `kani_depth2()` constructors for fixed-size arrays, each producing a
//! differently-populated array of the same declared length.
//! `amenable_kani::compose`'s own self-test checks this three times for
//! `[u8; 3]` (one call per depth) with no name tying the three checks
//! together as the same claim: an array's `.len()` is fixed at its
//! compile-time size regardless of which depth constructed it.

use amenable_derive::Standard;

use crate::{RustStdProvenance, RustStdStandard, RustStdType};

/// An array length known to match its fixed compile-time size, regardless
/// of which `#[derive(KaniCompose)]`-generated depth constructor produced
/// the array.
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
pub struct ComposeArrayLengthIsFixed {
    value: usize,
}

impl ComposeArrayLengthIsFixed {
    /// Wrap a length already known to match the array's fixed size.
    pub const fn new(value: usize) -> Self {
        Self { value }
    }

    /// The wrapped length.
    pub const fn value(&self) -> usize {
        self.value
    }
}
