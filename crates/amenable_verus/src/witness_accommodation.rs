//! A hand-trimmed accommodation mirror of `amenable_core::witness::
//! Witness`'s real trait definition -- not a probe, a permanent, real
//! necessity: `witness.rs` mixes the `Witness<V>` trait's own definition
//! (dependency-free) with `inventory`-based registry/export machinery
//! (`witness_exports()`, `WitnessExportRecord`, `inventory::collect!`)
//! in the same file, and `inventory` is one of the exact things this
//! crate can never resolve (`lib.rs`'s own doc comment: Verus never sees
//! `Cargo.toml`, so it cannot resolve any crate beyond `verus_builtin_
//! macros`/`vstd`). Every OTHER trait in the family (`Evidence`,
//! `Verifier`, `Provenance`, `Registry`, `Standard`, `ProofToken`,
//! `Sidecar`, `Establish`, `Exchange`) is dependency-free in its own real
//! file and gets mod-included verbatim in `lib.rs` -- this is the one
//! exception, and only because of `witness.rs`'s own file layout, not
//! because `Witness<V>`'s mechanics are themselves incompatible with
//! Verus (they aren't; see `gallery::evidence_self_referential_root` for
//! the confirmed, real, end-to-end story including this trait).
//!
//! If `witness.rs` in `amenable_core` is ever split (clean trait
//! mechanics vs. registry/export code, matching this file's own shape),
//! the real fix is deleting this file and mod-including the real one the
//! same way the other five are.

use crate::{Evidence, Verifier};

/// Verbatim copy of the real `Witness<V>` trait's own definition (no
/// inventory-dependent code included).
pub trait Witness<V: Verifier> {
    /// Evidence this witness backs.
    type SupportingEvidence: Evidence;

    /// Descriptor of the backend-facing proof for this verifier.
    type ProofArtifact;

    /// Identify the proof artifact relevant to this evidence, for this
    /// verifier.
    fn proof() -> Self::ProofArtifact;

    /// Describe what kind of support backs this witness.
    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::opaque_leaf()
    }

    /// Produce the basis behind this proof's supporting evidence.
    fn basis() -> <Self::SupportingEvidence as Evidence>::Basis {
        <Self::SupportingEvidence as Evidence>::basis()
    }
}

/// Stand-in for the real `WitnessSupportSummary` (a richer struct in
/// `amenable_core::witness`) -- only `support()`'s default return type
/// matters here, not its real fields.
pub struct WitnessSupportSummary;

impl WitnessSupportSummary {
    /// Mirrors the real `WitnessSupportSummary::opaque_leaf()`.
    pub fn opaque_leaf() -> Self {
        Self
    }
}
