//! Local/offset date-time family semantic bundles.
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
    BackendConversionSemanticBundle, LocalDateTimeDoesNotIdentifyFixedInstant, LocalDateTimeValid,
    OffsetDateTimeValid, TimestampRepresentsFixedInstant,
};

/// Aggregate semantic bundle for one local date-time carrier.
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
pub struct LocalDateTimeSemanticBundle {
    /// The `validity` sub-claim.
    validity: LocalDateTimeValid,
    /// The `local_semantics` sub-claim.
    local_semantics: LocalDateTimeDoesNotIdentifyFixedInstant,
    /// The `backend_conversion` sub-claim.
    backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one fixed-instant offset date-time carrier.
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
pub struct OffsetDateTimeSemanticBundle {
    /// The `validity` sub-claim.
    validity: OffsetDateTimeValid,
    /// The `fixed_instant` sub-claim.
    fixed_instant: TimestampRepresentsFixedInstant,
    /// The `backend_conversion` sub-claim.
    backend_conversion: BackendConversionSemanticBundle,
}
