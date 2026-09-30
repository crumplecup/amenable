//! Plain duration and time-interval propositions.
//!
//!
//! Ported from `elicit_temporal::contracts::proof_composition`. Each
//! aggregate is folded with its `*Evidence` bundle: the aggregate's
//! fields ARE its sub-claims, and `#[derive(Witness)]` makes the
//! composite proof the structural product of its members' proofs.
//! See [`super`] for the design; leaf `Witness<V>` impls land in the
//! backend crates.

use crate::{
    DurationAlternativeFormEvidence, DurationDesignatorRepresentationEvidence,
    DurationUsesPeriodDesignator, ExplicitDurationMayBeNegative,
    ExplicitDurationMayUseFractionalLowestOrderUnit, ExplicitDurationRepresentationEvidence,
    ExplicitDurationSemanticEvidence, ExplicitDurationUsesDurationalUnitDesignators,
    ExplicitTimeIntervalUsesDateTimeEndpointFamily, IntervalDurationIsNonNegative,
    IntervalStartPrecedesEnd, TimeIntervalBoundaryOrDurationFormDeclared,
    TimeIntervalHasTwoComponents, TimeIntervalUsesSolidusSeparator,
};

/// Aggregate proof that a duration representation is structurally valid.
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
pub struct DurationFormValid {
    /// The representation begins with the duration designator.
    period: DurationUsesPeriodDesignator,
    /// The concrete representation family and its associated laws.
    representation: DurationRepresentationSemanticsValid,
}

/// Aggregate proof that a duration representation family is explicit and lawful.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum DurationRepresentationSemanticsValid {
    /// The duration uses the designator-based representation.
    Designator(DurationDesignatorRepresentationEvidence),
    /// The duration uses the alternative complete representation.
    Alternative(DurationAlternativeFormEvidence),
}

impl core::default::Default for DurationRepresentationSemanticsValid {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Designator(core::default::Default::default())
    }
}

/// Aggregate proof that a CalConnect explicit duration form is structurally valid.
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
pub struct ExplicitDurationValid {
    /// Durational unit designators are used.
    units: ExplicitDurationUsesDurationalUnitDesignators,
    /// The explicit duration declares its representation family.
    representation: ExplicitDurationRepresentationEvidence,
    /// Signed negative-duration semantics are explicitly available.
    sign: ExplicitDurationMayBeNegative,
    /// Fractional lowest-order unit semantics are explicitly available.
    fractional: ExplicitDurationMayUseFractionalLowestOrderUnit,
    /// Exactness-family semantics are explicitly carried when needed.
    semantics: ExplicitDurationSemanticEvidence,
}

/// Aggregate proof that a time interval representation is structurally valid.
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
pub struct TimeIntervalValid {
    /// The interval uses the solidus separator between its top-level components.
    separator: TimeIntervalUsesSolidusSeparator,
    /// The interval has exactly two top-level components.
    components: TimeIntervalHasTwoComponents,
    /// The interval declares its endpoint-or-duration form.
    form: TimeIntervalBoundaryOrDurationFormDeclared,
}

/// Aggregate proof that a CalConnect explicit time-interval form is structurally valid.
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
pub struct ExplicitTimeIntervalValid {
    /// The explicit interval still satisfies the shared two-component interval skeleton.
    interval: TimeIntervalValid,
    /// Both interval boundaries use the CalConnect `[datetimeE]` endpoint family.
    endpoints: ExplicitTimeIntervalUsesDateTimeEndpointFamily,
}

/// Aggregate proof that interval endpoints are chronologically ordered.
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
pub struct IntervalEndpointsOrdered {
    /// The interval representation is structurally valid.
    interval: TimeIntervalValid,
    /// The start endpoint precedes the end endpoint.
    ordering: IntervalStartPrecedesEnd,
    /// Any duration component denotes a non-negative span.
    duration: IntervalDurationIsNonNegative,
}
