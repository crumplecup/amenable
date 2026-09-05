//! A hand-trimmed accommodation mirror of `amenable_core::witness::
//! core_trait::Witness`'s real trait definition -- not a probe, a
//! permanent, real necessity: `inventory`-based registry/export
//! machinery (`witness_exports()`, `WitnessExportRecord`,
//! `inventory::collect!`, in `amenable_core::witness::registry`) is one
//! of the exact things this crate can never resolve (`lib.rs`'s own doc
//! comment: Verus never sees `Cargo.toml`, so it cannot resolve any
//! crate beyond `verus_builtin_macros`/`vstd`). Every OTHER trait in the
//! family (`Evidence`, `Verifier`, `Provenance`, `Registry`, `Standard`,
//! `ProofToken`, `Sidecar`, `Establish`, `Exchange`) is dependency-free
//! in its own real file and gets mod-included verbatim in `lib.rs` --
//! this is the one exception, not because `Witness<V>`'s mechanics are
//! themselves incompatible with Verus (they aren't; see `gallery::
//! evidence_self_referential_root` for the confirmed, real, end-to-end
//! story including this trait).
//!
//! **Update: `amenable_core::witness` has since been split** (clean
//! trait mechanics in `core_trait.rs`, separate from the
//! `inventory`-dependent `registry.rs`) -- the condition this file's own
//! doc comment used to describe as a future "if ever split" trigger for
//! deleting it. Not yet acted on, and not as simple as the other eight
//! `#[path]` inclusions once it is: `core_trait.rs` itself has one real
//! cross-file dependency the other eight don't, `use super::support::
//! WitnessSupportSummary;` -- a relative import assuming a sibling
//! `support` module, so mod-including it verbatim needs `support.rs`
//! `#[path]`-included alongside it, nested under a wrapping module (not
//! flat like the other eight), for `super::support` to resolve. `support.rs`
//! is itself small and dependency-free, so this looks achievable, but
//! hasn't been attempted or verified against the real `verus` binary --
//! a real next step, not performed here.

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
