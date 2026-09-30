//! `parse_explicit_time_interval`/`format_explicit_time_interval` sidecars (CalConnect extension family).

use crate::{
    ExplicitIntervalDurationSubstitutionProofBranch,
    ExplicitIntervalEndComponentInheritanceProofBranch,
    ExplicitIntervalShiftPropagationProofBranch, ExplicitTimeIntervalDescriptor,
    ExplicitTimeIntervalValid, FormattedTemporalText,
};

/// Proof for [`ParsedExplicitTimeInterval`](crate::ParsedExplicitTimeInterval) — the 4 proof sidecars `parse_explicit_time_interval` returns, folded.
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
pub struct ExplicitTimeIntervalProof {
    /// The `explicit_time_interval_valid` sub-claim.
    explicit_time_interval_valid: ExplicitTimeIntervalValid,
    /// The `explicit_interval_duration_substitution_proof_branch` sub-claim.
    explicit_interval_duration_substitution_proof_branch:
        ExplicitIntervalDurationSubstitutionProofBranch,
    /// The `explicit_interval_end_component_inheritance_proof_branch` sub-claim.
    explicit_interval_end_component_inheritance_proof_branch:
        ExplicitIntervalEndComponentInheritanceProofBranch,
    /// The `explicit_interval_shift_propagation_proof_branch` sub-claim.
    explicit_interval_shift_propagation_proof_branch: ExplicitIntervalShiftPropagationProofBranch,
}

/// Lawful token: the folded [`ExplicitTimeIntervalProof`](crate::ExplicitTimeIntervalProof) was established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ExplicitTimeIntervalProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::ExplicitTimeIntervalProof"
)]
pub struct ExplicitTimeIntervalProofToken(());

/// Output sidecar for the `parse_explicit_time_interval` exchange: [`ExplicitTimeIntervalDescriptor`](crate::ExplicitTimeIntervalDescriptor) + a [`ExplicitTimeIntervalProof`](crate::ExplicitTimeIntervalProof) token.
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::ExplicitTimeIntervalProof", constructor = "pub")]
pub struct ParsedExplicitTimeInterval {
    #[sidecar(primary)]
    descriptor: ExplicitTimeIntervalDescriptor,
    #[sidecar(token)]
    token: ExplicitTimeIntervalProofToken,
}
/// Emission proof for [`FormattedExplicitTimeInterval`](crate::FormattedExplicitTimeInterval) — the `format_explicit_time_interval` output proof(s), folded.
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
pub struct ExplicitTimeIntervalFormatted {
    /// The re-issued `explicit_time_interval_valid` sub-claim.
    explicit_time_interval_valid: ExplicitTimeIntervalValid,
    /// The re-issued `explicit_interval_duration_substitution_proof_branch` sub-claim.
    explicit_interval_duration_substitution_proof_branch:
        ExplicitIntervalDurationSubstitutionProofBranch,
    /// The re-issued `explicit_interval_end_component_inheritance_proof_branch` sub-claim.
    explicit_interval_end_component_inheritance_proof_branch:
        ExplicitIntervalEndComponentInheritanceProofBranch,
    /// The re-issued `explicit_interval_shift_propagation_proof_branch` sub-claim.
    explicit_interval_shift_propagation_proof_branch: ExplicitIntervalShiftPropagationProofBranch,
}

/// Lawful token: [`ExplicitTimeIntervalFormatted`](crate::ExplicitTimeIntervalFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ExplicitTimeIntervalFormatted")]
#[amenable_derive::establish(
    credential = "crate::ExplicitTimeIntervalProofToken",
    proposition = "crate::ExplicitTimeIntervalFormatted"
)]
pub struct ExplicitTimeIntervalFormattedToken(());

/// Output sidecar for the `format_explicit_time_interval` exchange: the emitted text + a [`ExplicitTimeIntervalFormatted`](crate::ExplicitTimeIntervalFormatted) token.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::ExplicitTimeIntervalFormatted",
    constructor = "pub"
)]
pub struct FormattedExplicitTimeInterval {
    #[sidecar(primary)]
    text: FormattedTemporalText,
    #[sidecar(token)]
    token: ExplicitTimeIntervalFormattedToken,
}

impl FormattedExplicitTimeInterval {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}
