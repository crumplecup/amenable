//! Backend-conversion/precision/qualification family aggregate proof propositions.
//!
//!
//! Ported from `elicit_temporal::contracts::proof_composition`. Each
//! aggregate is folded with its `*Evidence` bundle: the aggregate's
//! fields ARE its sub-claims, and `#[derive(Witness)]` makes the
//! composite proof the structural product of its members' proofs.
//! See [`super`] for the design; leaf `Witness<V>` impls land in the
//! backend crates.

use crate::{
    ApproximationQualificationDeclared, ConversionDropsSubsecondPrecision,
    ConversionPreservesRepresentedInstant, ConversionPreservesTemporalOrdering,
    ConversionRequiresExplicitAuthorityWhenLossy, ConversionSourceSemanticKindDeclared,
    ConversionTargetSemanticKindDeclared, FractionalSecondDigitsAreContiguous,
    FractionalSecondPrecisionDeclared, MutualAgreementAuthorityScopeEvidence,
    PrecisionReductionDeclared, QualificationPlacementEvidence, QualificationScopeDeclared,
    RoundingModeDeclared, SubsecondDigitsPreserved, TimestampRepresentsFixedInstant,
    UncertaintyAndApproximationMayBeCombined, UncertaintyQualificationDeclared,
    UtcTimelineOrderingAppliesToFixedInstants,
};

/// Aggregate proof that backend-conversion semantics are explicitly declared.
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
pub struct BackendConversionSemanticsValid {
    /// The source semantic kind is explicitly declared.
    source_kind: ConversionSourceSemanticKindDeclared,
    /// The target semantic kind is explicitly declared.
    target_kind: ConversionTargetSemanticKindDeclared,
    /// The conversion preserves the represented instant.
    instant: ConversionPreservesRepresentedInstant,
    /// The conversion preserves temporal ordering.
    ordering: TemporalOrderingPreserved,
}

/// Aggregate proof that a conversion is lossless.
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
pub struct ConversionLossless {
    /// Shared backend-conversion semantics are established.
    conversion: BackendConversionSemanticsValid,
    /// The conversion preserves subsecond precision.
    precision: PrecisionPreserved,
}

/// Aggregate proof that a conversion truncates subseconds.
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
pub struct ConversionTruncatesSubseconds {
    /// Shared backend-conversion semantics are established.
    conversion: BackendConversionSemanticsValid,
    /// Lossy conversion authority is explicit and fully described.
    lossy_authority: LossyConversionAuthorityValid,
}

/// Aggregate proof that lossy conversion authority is explicit and fully described.
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
pub struct LossyConversionAuthorityValid {
    /// Lossy conversion was explicitly authorized.
    explicit_lossy_authority: ConversionRequiresExplicitAuthorityWhenLossy,
    /// Precision reduction was declared.
    precision_reduction: PrecisionReductionDeclared,
    /// The rounding mode was declared.
    rounding: RoundingModeDeclared,
    /// The conversion dropped subsecond precision.
    dropped_digits: ConversionDropsSubsecondPrecision,
}

/// Aggregate proof that subsecond precision was preserved.
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
pub struct PrecisionPreserved {
    /// The source subsecond precision is explicitly declared.
    precision: FractionalSecondPrecisionDeclared,
    /// The source fraction digits form a contiguous decimal suffix.
    contiguous_digits: FractionalSecondDigitsAreContiguous,
    /// The exchange preserved all subsecond digits.
    preserved_digits: SubsecondDigitsPreserved,
}

/// Aggregate proof that a standards-governed mutual-agreement authority is explicit and lawful.
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
pub struct MutualAgreementAuthorityValid {
    /// The agreement-governed scope carried by the authority sidecar.
    scope: MutualAgreementAuthorityScopeEvidence,
}

/// Aggregate proof that an extended temporal qualification is structurally valid.
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
pub struct QualifiedTemporalExpressionValid {
    /// The expression explicitly declares uncertainty qualification.
    uncertainty: UncertaintyQualificationDeclared,
    /// The expression explicitly declares approximation qualification.
    approximation: ApproximationQualificationDeclared,
    /// The qualification scope is explicitly declared.
    scope: QualificationScopeDeclared,
    /// The placement law for the qualification marker is explicitly carried.
    placement: QualificationPlacementEvidence,
    /// Combined uncertainty and approximation semantics are available when needed.
    combined: UncertaintyAndApproximationMayBeCombined,
}

/// Aggregate proof that a temporal value and its qualification sidecar form a lawful exchange.
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
pub struct QualifiedTemporalValueValid {
    /// The qualification sidecar is structurally valid.
    qualification: QualifiedTemporalExpressionValid,
}

/// Aggregate proof that a conversion preserved ordering on the UTC timeline.
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
pub struct TemporalOrderingPreserved {
    /// The source timestamp denotes a fixed instant.
    fixed_instant: TimestampRepresentsFixedInstant,
    /// Ordering is evaluated on the UTC timeline.
    utc_timeline: UtcTimelineOrderingAppliesToFixedInstants,
    /// The conversion preserved temporal ordering.
    preserved_ordering: ConversionPreservesTemporalOrdering,
}
