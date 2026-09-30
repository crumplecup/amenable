//! `parse_explicit_duration`/`format_explicit_duration` sidecars (CalConnect extension family).

use crate::{
    ExplicitDurationDescriptor, ExplicitDurationMayBeNegative,
    ExplicitDurationMayUseFractionalLowestOrderUnit, ExplicitDurationRepresentationEvidence,
    ExplicitDurationSemanticEvidence, ExplicitDurationUsesDurationalUnitDesignators,
    ExplicitDurationValid, FormattedTemporalText,
};

/// Proof for [`ParsedExplicitDuration`](crate::ParsedExplicitDuration) — the 6 proof sidecars `parse_explicit_duration` returns, folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct ExplicitDurationProof {
    /// The `explicit_duration_valid` sub-claim.
    explicit_duration_valid: ExplicitDurationValid,
    /// The `explicit_duration_uses_durational_unit_designators` sub-claim.
    explicit_duration_uses_durational_unit_designators:
        ExplicitDurationUsesDurationalUnitDesignators,
    /// The `explicit_duration_representation_evidence` sub-claim.
    explicit_duration_representation_evidence: ExplicitDurationRepresentationEvidence,
    /// The `explicit_duration_may_be_negative` sub-claim.
    explicit_duration_may_be_negative: ExplicitDurationMayBeNegative,
    /// The `explicit_duration_may_use_fractional_lowest_order_unit` sub-claim.
    explicit_duration_may_use_fractional_lowest_order_unit:
        ExplicitDurationMayUseFractionalLowestOrderUnit,
    /// The `explicit_duration_semantic_evidence` sub-claim.
    explicit_duration_semantic_evidence: ExplicitDurationSemanticEvidence,
}

/// Lawful token: the folded [`ExplicitDurationProof`](crate::ExplicitDurationProof) was established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ExplicitDurationProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::ExplicitDurationProof"
)]
pub struct ExplicitDurationProofToken(());

/// Output sidecar for the `parse_explicit_duration` exchange: [`ExplicitDurationDescriptor`](crate::ExplicitDurationDescriptor) + a [`ExplicitDurationProof`](crate::ExplicitDurationProof) token.
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::ExplicitDurationProof", constructor = "pub")]
pub struct ParsedExplicitDuration {
    #[sidecar(primary)]
    descriptor: ExplicitDurationDescriptor,
    #[sidecar(token)]
    token: ExplicitDurationProofToken,
}
/// Emission proof for [`FormattedExplicitDuration`](crate::FormattedExplicitDuration) — the `format_explicit_duration` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct ExplicitDurationFormatted {
    /// The re-issued `explicit_duration_valid` sub-claim.
    explicit_duration_valid: ExplicitDurationValid,
    /// The re-issued `explicit_duration_uses_durational_unit_designators` sub-claim.
    explicit_duration_uses_durational_unit_designators:
        ExplicitDurationUsesDurationalUnitDesignators,
    /// The re-issued `explicit_duration_representation_evidence` sub-claim.
    explicit_duration_representation_evidence: ExplicitDurationRepresentationEvidence,
    /// The re-issued `explicit_duration_may_be_negative` sub-claim.
    explicit_duration_may_be_negative: ExplicitDurationMayBeNegative,
    /// The re-issued `explicit_duration_may_use_fractional_lowest_order_unit` sub-claim.
    explicit_duration_may_use_fractional_lowest_order_unit:
        ExplicitDurationMayUseFractionalLowestOrderUnit,
    /// The re-issued `explicit_duration_semantic_evidence` sub-claim.
    explicit_duration_semantic_evidence: ExplicitDurationSemanticEvidence,
}

/// Lawful token: [`ExplicitDurationFormatted`](crate::ExplicitDurationFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ExplicitDurationFormatted")]
#[amenable_derive::establish(
    credential = "crate::ExplicitDurationProofToken",
    proposition = "crate::ExplicitDurationFormatted"
)]
pub struct ExplicitDurationFormattedToken(());

/// Output sidecar for the `format_explicit_duration` exchange: the emitted text + a [`ExplicitDurationFormatted`](crate::ExplicitDurationFormatted) token.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::ExplicitDurationFormatted", constructor = "pub")]
pub struct FormattedExplicitDuration {
    #[sidecar(primary)]
    text: FormattedTemporalText,
    #[sidecar(token)]
    token: ExplicitDurationFormattedToken,
}

impl FormattedExplicitDuration {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}
