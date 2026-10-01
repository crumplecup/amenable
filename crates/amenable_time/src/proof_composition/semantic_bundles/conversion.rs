//! Backend-conversion/precision/qualification family semantic bundles.
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
    BackendConversionSemanticsValid, ConversionLossless, ConversionTruncatesSubseconds,
    LossyConversionAuthorityValid, QualificationPlacementEvidence,
    QualifiedTemporalExpressionValid, QualifiedTemporalValueValid,
};

/// Aggregate semantic bundle for one backend conversion exchange.
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
pub struct BackendConversionSemanticBundle {
    /// The `semantics` sub-claim.
    semantics: BackendConversionSemanticsValid,
}

/// Aggregate semantic bundle for one qualified temporal value carrier.
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
pub struct QualifiedTemporalValueSemanticBundle {
    /// The `validity` sub-claim.
    validity: QualifiedTemporalValueValid,
    /// The `qualification` sub-claim.
    qualification: QualifiedTemporalExpressionValid,
    /// The `placement` sub-claim.
    placement: QualificationPlacementEvidence,
    /// The `backend_conversion` sub-claim.
    backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for explicit lossy-conversion authority.
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
pub struct LossyConversionAuthorityBundle {
    /// The `semantics` sub-claim.
    semantics: LossyConversionAuthorityValid,
}

/// Aggregate semantic bundle for one lossless conversion result.
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
pub struct LosslessConversionBundle {
    /// The `semantics` sub-claim.
    semantics: ConversionLossless,
}

/// Aggregate semantic bundle for one subsecond truncation result.
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
pub struct SubsecondTruncationBundle {
    /// The `semantics` sub-claim.
    semantics: ConversionTruncatesSubseconds,
}
