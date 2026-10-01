//! Duration/interval/recurring-interval family semantic bundles.
//!
//!
//! The `elicit_temporal` `*Bundle` aggregate proof bundles, ported as
//! folded `#[derive(Evidence, Witness)]` composites (the same
//! structural closure as the rest of `proof_composition`). Each
//! `Established<X>` field collapses to `X`; a `<Foo>Evidence` field
//! already merged into its `*Valid` sibling is dropped; `*ProofBranch`
//! and standalone `*Evidence` fields are kept. These pair with the
//! Phase 5 `ProvenTemporalCarrier` carrier wrapper.

use crate::{
    BackendConversionSemanticBundle, CompleteIntervalSubstitutionProofBranch, DurationFormValid,
    DurationRepresentationProofBranch, ExplicitDurationMayBeNegative,
    ExplicitDurationMayUseFractionalLowestOrderUnit, ExplicitDurationRepresentationEvidence,
    ExplicitDurationSemanticEvidence, ExplicitDurationUsesDurationalUnitDesignators,
    ExplicitDurationValid, ExplicitIntervalDurationSubstitutionProofBranch,
    ExplicitIntervalEndComponentInheritanceProofBranch,
    ExplicitIntervalShiftPropagationProofBranch, ExplicitTimeIntervalValid,
    ExtendedIntervalBoundarySemanticsValid, IntervalEndComponentInheritanceProofBranch,
    IntervalEndpointsOrdered, IntervalZoneInheritanceProofBranch, RecurringIntervalFormValid,
    RecurringIntervalRepresentationProofBranch, TimeIntervalValid,
};

/// Aggregate semantic bundle for one duration carrier.
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
pub struct DurationSemanticBundle {
    /// The `validity` sub-claim.
    validity: DurationFormValid,
    /// The `representation` sub-claim.
    representation: DurationRepresentationProofBranch,
    /// The `backend_conversion` sub-claim.
    backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one time-interval carrier.
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
pub struct TimeIntervalSemanticBundle {
    /// The `validity` sub-claim.
    validity: TimeIntervalValid,
    /// The `extended_boundaries` sub-claim.
    extended_boundaries: ExtendedIntervalBoundarySemanticsValid,
    /// The `end_component_inheritance` sub-claim.
    end_component_inheritance: IntervalEndComponentInheritanceProofBranch,
    /// The `zone_inheritance` sub-claim.
    zone_inheritance: IntervalZoneInheritanceProofBranch,
    /// The `substitution` sub-claim.
    substitution: CompleteIntervalSubstitutionProofBranch,
    /// The `backend_conversion` sub-claim.
    backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one recurring-interval carrier.
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
pub struct RecurringIntervalSemanticBundle {
    /// The `validity` sub-claim.
    validity: RecurringIntervalFormValid,
    /// The `representation` sub-claim.
    representation: RecurringIntervalRepresentationProofBranch,
    /// The `backend_conversion` sub-claim.
    backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one explicit duration carrier.
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
pub struct ExplicitDurationSemanticBundle {
    /// The `validity` sub-claim.
    validity: ExplicitDurationValid,
    /// The `units` sub-claim.
    units: ExplicitDurationUsesDurationalUnitDesignators,
    /// The `representation` sub-claim.
    representation: ExplicitDurationRepresentationEvidence,
    /// The `sign` sub-claim.
    sign: ExplicitDurationMayBeNegative,
    /// The `fractional` sub-claim.
    fractional: ExplicitDurationMayUseFractionalLowestOrderUnit,
    /// The `semantics` sub-claim.
    semantics: ExplicitDurationSemanticEvidence,
    /// The `backend_conversion` sub-claim.
    backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one explicit time-interval carrier.
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
pub struct ExplicitTimeIntervalSemanticBundle {
    /// The `validity` sub-claim.
    validity: ExplicitTimeIntervalValid,
    /// The `duration_substitution` sub-claim.
    duration_substitution: ExplicitIntervalDurationSubstitutionProofBranch,
    /// The `end_component_inheritance` sub-claim.
    end_component_inheritance: ExplicitIntervalEndComponentInheritanceProofBranch,
    /// The `shift_propagation` sub-claim.
    shift_propagation: ExplicitIntervalShiftPropagationProofBranch,
    /// The `backend_conversion` sub-claim.
    backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one ordered fixed-instant interval result.
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
pub struct IntervalEndpointOrderingBundle {
    /// The `semantics` sub-claim.
    semantics: IntervalEndpointsOrdered,
}
