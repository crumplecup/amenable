//! Extended/enhanced-interval (Level 1/2) boundary and substitution propositions.
//!
//!
//! Ported from `elicit_temporal::contracts::proof_composition`. Each
//! aggregate is folded with its `*Evidence` bundle: the aggregate's
//! fields ARE its sub-claims, and `#[derive(Witness)]` makes the
//! composite proof the structural product of its members' proofs.
//! See [`super`] for the design; leaf `Witness<V>` impls land in the
//! backend crates.

use crate::{
    BeforeOrAfterQualificationIsLevelTwoOnly, BeforeOrOnDateUsesLeadingDoubleDotQualifier,
    CompleteDurationEndIntervalSubstitutionEvidence,
    CompleteStartDurationIntervalSubstitutionEvidence,
    CompleteStartEndIntervalSubstitutionEvidence,
    ExplicitTimeIntervalDurationSubstitutionInfersMissingBoundary,
    ExplicitTimeIntervalTrailingEndMayInheritHigherOrderComponents, ExplicitTimeIntervalValid,
    IntervalEndOmittedHigherOrderComponentsInheritFromStart,
    OnOrAfterDateUsesTrailingDoubleDotQualifier, OpenIntervalBoundaryDeclared,
    QualifiedTemporalExpressionValid, TimeIntervalValid, UnknownIntervalBoundaryDeclared,
    UnspecifiedComponentExpressionValid,
};

/// Aggregate proof that extended interval boundary semantics are structurally valid.
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
pub struct ExtendedIntervalBoundarySemanticsValid {
    /// The underlying interval representation is structurally valid.
    interval: TimeIntervalValid,
    /// Open-boundary semantics are explicitly declared.
    open_boundary: OpenIntervalBoundaryDeclared,
    /// Unknown-boundary semantics are explicitly declared.
    unknown_boundary: UnknownIntervalBoundaryDeclared,
}

/// Aggregate proof that inherited end-component semantics are structurally valid.
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
pub struct InheritedIntervalEndComponentsSemanticsValid {
    /// The underlying interval representation is structurally valid.
    interval: TimeIntervalValid,
    /// Omitted higher-order end components inherit from the leading component.
    inheritance: IntervalEndOmittedHigherOrderComponentsInheritFromStart,
}

/// Aggregate proof that inherited trailing-zone semantics are structurally valid.
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
pub struct InheritedIntervalZoneSemanticsValid {
    /// The underlying interval representation is structurally valid.
    interval: TimeIntervalValid,
}

/// Aggregate proof that explicit-interval duration-substitution semantics are structurally valid.
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
pub struct ExplicitIntervalDurationSubstitutionSemanticsValid {
    /// The underlying explicit interval representation is structurally valid.
    interval: ExplicitTimeIntervalValid,
    /// A missing boundary is inferable from the substituted explicit duration.
    substitution: ExplicitTimeIntervalDurationSubstitutionInfersMissingBoundary,
}

/// Aggregate proof that explicit-interval trailing-end inheritance semantics are structurally valid.
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
pub struct ExplicitIntervalEndComponentInheritanceSemanticsValid {
    /// The underlying explicit interval representation is structurally valid.
    interval: ExplicitTimeIntervalValid,
    /// Omitted higher-order trailing-end components inherit from the leading endpoint.
    inheritance: ExplicitTimeIntervalTrailingEndMayInheritHigherOrderComponents,
}
/// Aggregate proof that explicit-interval leading-shift propagation semantics are structurally valid.
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
pub struct ExplicitIntervalShiftPropagationSemanticsValid {
    /// The underlying explicit interval representation is structurally valid.
    interval: ExplicitTimeIntervalValid,
}

/// Aggregate proof that complete-interval substitution semantics are structurally valid.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum CompleteIntervalSubstitutionSemanticsValid {
    /// Established via the `CompleteStartEndIntervalSubstitutionEvidence` decomposition.
    CompleteStartEndIntervalSubstitution(CompleteStartEndIntervalSubstitutionEvidence),
    /// Established via the `CompleteStartDurationIntervalSubstitutionEvidence` decomposition.
    CompleteStartDurationIntervalSubstitution(CompleteStartDurationIntervalSubstitutionEvidence),
    /// Established via the `CompleteDurationEndIntervalSubstitutionEvidence` decomposition.
    CompleteDurationEndIntervalSubstitution(CompleteDurationEndIntervalSubstitutionEvidence),
}

impl core::default::Default for CompleteIntervalSubstitutionSemanticsValid {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::CompleteStartEndIntervalSubstitution(core::default::Default::default())
    }
}

/// Aggregate proof that Level 1 enhanced-interval semantics are structurally valid.
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
pub struct EnhancedIntervalLevelOneSemanticsValid {
    /// The underlying extended-interval boundary semantics are structurally valid.
    boundaries: ExtendedIntervalBoundarySemanticsValid,
    /// Boundary-date qualification semantics are available.
    qualification: QualifiedTemporalExpressionValid,
}

/// Aggregate proof that Level 2 enhanced-interval semantics are structurally valid.
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
pub struct EnhancedIntervalLevelTwoSemanticsValid {
    /// Level 2 semantics extend the Level 1 enhanced-interval surface.
    level_one: EnhancedIntervalLevelOneSemanticsValid,
    /// Unspecified-component semantics are available for boundary dates.
    unspecified_components: UnspecifiedComponentExpressionValid,
    /// Before-or-after qualification is a Level 2 feature.
    before_or_after_level: BeforeOrAfterQualificationIsLevelTwoOnly,
    /// The start boundary may use the leading `..` before-or-on qualifier.
    before_or_on_start: BeforeOrOnDateUsesLeadingDoubleDotQualifier,
    /// The end boundary may use the trailing `..` on-or-after qualifier.
    on_or_after_end: OnOrAfterDateUsesTrailingDoubleDotQualifier,
}
