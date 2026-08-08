//! The `KaniCompose::kani_depth0` postcondition, named once instead of
//! restated per carrier.
//!
//! `KaniCompose`'s own self-tests (`amenable_kani::compose`) check the same
//! claim about `kani_depth0()` for four different carriers — `String`, a
//! `Vec<u8>`, and two `#[derive(KaniCompose)]`-generated struct fields
//! (`DerivedNode::name`, `DerivedNode::flags`) — restated each time as a
//! bare `.is_empty()` on the depth-0 value. `ComposeDepthZeroIsEmpty` is
//! the first Kani-only contract type in the worklist that names a claim
//! about the `KaniCompose` derive/trait itself rather than a real Rust
//! standard-library type's own behavior.

use amenable_derive::Standard;

use crate::{RustStdProvenance, RustStdStandard, RustStdType};

/// A length known to be exactly zero — the postcondition every
/// `KaniCompose::kani_depth0()` self-test in this workspace checks about
/// its smallest meaningful inhabitant.
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
pub struct ComposeDepthZeroIsEmpty {
    value: usize,
}

impl ComposeDepthZeroIsEmpty {
    /// Wrap a length already known to be exactly zero.
    pub const fn new(value: usize) -> Self {
        Self { value }
    }

    /// The wrapped length.
    pub const fn value(&self) -> usize {
        self.value
    }
}
