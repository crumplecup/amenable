//! Emission-proof composites for the duration/interval family.
//!
//! Each formatter method's output proof(s) folded into one
//! `#[derive(Evidence, Witness)]` struct (the same folding as
//! `proof_composition`), so the matching `Formatted*` output sidecar
//! keeps its single-`token` shape.

use crate::{
    CompleteIntervalSubstitutionProofBranch, DurationFormValid, DurationRepresentationProofBranch,
    ExtendedIntervalBoundarySemanticsValid, IntervalEndComponentInheritanceProofBranch,
    IntervalZoneInheritanceProofBranch, RecurringIntervalFormValid,
    RecurringIntervalRepresentationProofBranch, TimeIntervalValid,
};

/// Emission proof for [`FormattedDuration`](crate::FormattedDuration) — the `format_duration` output proof(s), folded.
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
pub struct DurationFormatted {
    /// The `duration_form_valid` sub-claim.
    duration_form_valid: DurationFormValid,
    /// The `duration_representation_proof_branch` sub-claim.
    duration_representation_proof_branch: DurationRepresentationProofBranch,
}

/// Emission proof for [`FormattedRecurringInterval`](crate::FormattedRecurringInterval) — the `format_recurring_interval` output proof(s), folded.
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
pub struct RecurringIntervalFormatted {
    /// The `recurring_interval_form_valid` sub-claim.
    recurring_interval_form_valid: RecurringIntervalFormValid,
    /// The `recurring_interval_representation_proof_branch` sub-claim.
    recurring_interval_representation_proof_branch: RecurringIntervalRepresentationProofBranch,
}

/// Emission proof for [`FormattedTimeInterval`](crate::FormattedTimeInterval) — the `format_time_interval` output proof(s), folded.
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
pub struct TimeIntervalFormatted {
    /// The `time_interval_valid` sub-claim.
    time_interval_valid: TimeIntervalValid,
    /// The `extended_interval_boundary_semantics_valid` sub-claim.
    extended_interval_boundary_semantics_valid: ExtendedIntervalBoundarySemanticsValid,
    /// The `interval_end_component_inheritance_proof_branch` sub-claim.
    interval_end_component_inheritance_proof_branch: IntervalEndComponentInheritanceProofBranch,
    /// The `interval_zone_inheritance_proof_branch` sub-claim.
    interval_zone_inheritance_proof_branch: IntervalZoneInheritanceProofBranch,
    /// The `complete_interval_substitution_proof_branch` sub-claim.
    complete_interval_substitution_proof_branch: CompleteIntervalSubstitutionProofBranch,
}
