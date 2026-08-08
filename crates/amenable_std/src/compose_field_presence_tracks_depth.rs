//! The `#[derive(KaniCompose)]` optional-field presence postcondition,
//! named once instead of restated per depth.
//!
//! `KaniCompose`'s derive macro grows an `Option<Box<_>>` field alongside
//! everything else as depth increases: absent at `kani_depth0()`, present
//! from `kani_depth1()` on. `amenable_kani::compose`'s own self-test checks
//! both directions of this the same relationship at two call sites
//! (`DerivedNode::maybe_child.is_none()` at depth 0,
//! `DerivedNode::maybe_child.is_some()` at depth 1) with no name tying them
//! together as the same claim.

use amenable_derive::Standard;

use crate::{RustStdProvenance, RustStdStandard, RustStdType};

/// Whether a `#[derive(KaniCompose)]`-generated optional field is present —
/// known to track construction depth: absent at depth 0, present from
/// depth 1 on.
///
/// A derived claim about `bool`, not a fresh root authority — its
/// evidence chain rests on `bool`'s own already-registered standard-
/// library provenance ([`RustStdStandard<bool>`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<bool>",
    basis_ctor = "RustStdStandard::<bool>::new()",
    provenance = "<bool as RustStdType>::provenance()",
    provenance_type = "RustStdProvenance"
)]
pub struct ComposeFieldPresenceTracksDepth {
    value: bool,
}

impl ComposeFieldPresenceTracksDepth {
    /// Wrap a presence flag already known to track construction depth.
    pub const fn new(value: bool) -> Self {
        Self { value }
    }

    /// The wrapped presence flag.
    pub const fn value(&self) -> bool {
        self.value
    }
}
