//! Boundary markers for the temporal exchange surface.
//!
//! [`RawTemporalText`] is the payload an input sidecar carries; it is
//! `Evidence` (every `Sidecar::Primary` must be) but carries no proof —
//! it is just the raw string. [`TemporalInputReceived`] is the
//! proposition an input sidecar asserts: "a caller handed us this text".
//! That is a boundary fact, asserted at the seam, not a checkable claim —
//! so it carries a trivial, **unconditional** `Witness<V>` for every
//! verifier (a backend must not also implement `Witness` for it — that
//! would be a coherence conflict, and there is nothing there to check).

use amenable_core::{Verifier, Witness, WitnessSupportSummary};
use derive_getters::Getters;
use derive_new::new;

/// The raw input text an exchange consumes — a `Sidecar::Primary`
/// payload, not a proof.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, amenable_derive::Evidence, Getters, new)]
#[evidence(basis = "Self")]
pub struct RawTemporalText {
    /// The verbatim input string.
    #[new(into)]
    value: String,
}

/// The proposition an input sidecar asserts: raw text was received at the
/// exchange boundary. Asserted, not proven — see the module docs.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub struct TemporalInputReceived;

impl<V: Verifier> Witness<V> for TemporalInputReceived {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {}

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::opaque_leaf()
    }
}
