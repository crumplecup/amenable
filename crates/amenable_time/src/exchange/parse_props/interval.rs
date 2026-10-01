//! Time-interval parse composite proof.
//!
//!
//! Per-method composite proof propositions -- for the parse methods
//! whose `elicit_temporal` return tuple carries 2+ proof sidecars, the
//! sidecars folded into one `#[derive(Evidence, Witness)]` struct (the
//! same folding as `proof_composition`) so the output `Sidecar` keeps
//! its single-`token` shape.

use crate::{
    CompleteIntervalSubstitutionProofBranch, ExtendedIntervalBoundarySemanticsValid,
    IntervalEndComponentInheritanceProofBranch, IntervalZoneInheritanceProofBranch,
    TimeIntervalValid,
};

/// Proof for [`ParsedTimeInterval`](crate::ParsedTimeInterval) — the 5 proof sidecars `elicit_temporal`'s `parse_time_interval` returns, folded into one composite proposition.
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
pub struct TimeIntervalProof {
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
