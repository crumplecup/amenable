//! `parse_explicit_temporal_form`/`format_explicit_temporal_form` sidecars (CalConnect extension family).

use crate::{
    ExplicitTemporalFormDescriptor, ExplicitTemporalFormMayOmitZeroValuedComponents,
    ExplicitTemporalFormUsesDesignatorSymbols, ExplicitTemporalFormValid,
    ExplicitTemporalPrecisionUsesLowestDenotedComponent,
    ExplicitUtcRelationshipUsesZuluOrSignedShift, FormattedTemporalText,
};

/// Proof for [`ParsedExplicitTemporalForm`](crate::ParsedExplicitTemporalForm) — the 5 proof sidecars `parse_explicit_temporal_form` returns, folded.
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
pub struct ExplicitTemporalFormProof {
    /// The `explicit_temporal_form_valid` sub-claim.
    explicit_temporal_form_valid: ExplicitTemporalFormValid,
    /// The `explicit_temporal_form_uses_designator_symbols` sub-claim.
    explicit_temporal_form_uses_designator_symbols: ExplicitTemporalFormUsesDesignatorSymbols,
    /// The `explicit_temporal_form_may_omit_zero_valued_components` sub-claim.
    explicit_temporal_form_may_omit_zero_valued_components:
        ExplicitTemporalFormMayOmitZeroValuedComponents,
    /// The `explicit_temporal_precision_uses_lowest_denoted_component` sub-claim.
    explicit_temporal_precision_uses_lowest_denoted_component:
        ExplicitTemporalPrecisionUsesLowestDenotedComponent,
    /// The `explicit_utc_relationship_uses_zulu_or_signed_shift` sub-claim.
    explicit_utc_relationship_uses_zulu_or_signed_shift:
        ExplicitUtcRelationshipUsesZuluOrSignedShift,
}

/// Lawful token: the folded [`ExplicitTemporalFormProof`](crate::ExplicitTemporalFormProof) was established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ExplicitTemporalFormProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::ExplicitTemporalFormProof"
)]
pub struct ExplicitTemporalFormProofToken(());

/// Output sidecar for the `parse_explicit_temporal_form` exchange: [`ExplicitTemporalFormDescriptor`](crate::ExplicitTemporalFormDescriptor) + a [`ExplicitTemporalFormProof`](crate::ExplicitTemporalFormProof) token.
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::ExplicitTemporalFormProof", constructor = "pub")]
pub struct ParsedExplicitTemporalForm {
    #[sidecar(primary)]
    descriptor: ExplicitTemporalFormDescriptor,
    #[sidecar(token)]
    token: ExplicitTemporalFormProofToken,
}
/// Emission proof for [`FormattedExplicitTemporalForm`](crate::FormattedExplicitTemporalForm) — the `format_explicit_temporal_form` output proof(s), folded.
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
pub struct ExplicitTemporalFormFormatted {
    /// The re-issued `explicit_temporal_form_valid` sub-claim.
    explicit_temporal_form_valid: ExplicitTemporalFormValid,
    /// The re-issued `explicit_temporal_form_uses_designator_symbols` sub-claim.
    explicit_temporal_form_uses_designator_symbols: ExplicitTemporalFormUsesDesignatorSymbols,
    /// The re-issued `explicit_temporal_form_may_omit_zero_valued_components` sub-claim.
    explicit_temporal_form_may_omit_zero_valued_components:
        ExplicitTemporalFormMayOmitZeroValuedComponents,
    /// The re-issued `explicit_temporal_precision_uses_lowest_denoted_component` sub-claim.
    explicit_temporal_precision_uses_lowest_denoted_component:
        ExplicitTemporalPrecisionUsesLowestDenotedComponent,
    /// The re-issued `explicit_utc_relationship_uses_zulu_or_signed_shift` sub-claim.
    explicit_utc_relationship_uses_zulu_or_signed_shift:
        ExplicitUtcRelationshipUsesZuluOrSignedShift,
}

/// Lawful token: [`ExplicitTemporalFormFormatted`](crate::ExplicitTemporalFormFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ExplicitTemporalFormFormatted")]
#[amenable_derive::establish(
    credential = "crate::ExplicitTemporalFormProofToken",
    proposition = "crate::ExplicitTemporalFormFormatted"
)]
pub struct ExplicitTemporalFormFormattedToken(());

/// Output sidecar for the `format_explicit_temporal_form` exchange: the emitted text + a [`ExplicitTemporalFormFormatted`](crate::ExplicitTemporalFormFormatted) token.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::ExplicitTemporalFormFormatted",
    constructor = "pub"
)]
pub struct FormattedExplicitTemporalForm {
    #[sidecar(primary)]
    text: FormattedTemporalText,
    #[sidecar(token)]
    token: ExplicitTemporalFormFormattedToken,
}

impl FormattedExplicitTemporalForm {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}
