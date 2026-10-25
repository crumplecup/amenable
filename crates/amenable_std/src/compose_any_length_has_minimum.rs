//! The `#[derive(KaniCompose)]` `kani_depth1()`/`kani_depth2()` growing-
//! length postcondition, named once instead of restated per carrier as
//! a bare `!x.is_empty()`/`x.len() >= n`.
//!
//! `KaniCompose`'s derive macro generates a `kani_depth1()`/`kani_depth2()`
//! for growable carriers (`String`, a derived struct's `name` field) that
//! are guaranteed non-empty and strictly growing with depth. The three
//! self-tests in `amenable_kani::compose` restated this three times as a
//! bare boolean expression (`!depth1.is_empty()`, `depth2.len() >= 2`,
//! `!depth1.name.is_empty()`) with no name tying the claim to the one
//! `ComposeAnyLengthIsBounded` already names for the matching upper bound.

use amenable_derive::Standard;

use crate::{RustStdProvenance, RustStdStandard, RustStdType};

/// A length known to stay at or above a `#[derive(KaniCompose)]`-generated
/// `kani_depth1()`/`kani_depth2()`'s declared minimum.
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
pub struct ComposeAnyLengthHasMinimum {
    value: usize,
}

impl ComposeAnyLengthHasMinimum {
    /// Wrap a length already known to stay at or above its declared
    /// minimum.
    pub const fn new(value: usize) -> Self {
        Self { value }
    }

    /// The wrapped length.
    pub const fn value(&self) -> usize {
        self.value
    }
}
