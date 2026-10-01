//! Duration-form/duration-representation evidence branches.
//!
//!
//! Shared `*Evidence` branch decompositions reused across several
//! aggregates, plus the per-form bundles behind the two
//! multi-credential aggregates. Same `#[derive(Evidence, Witness)]`
//! shape as the `proof_composition::composites` family.

use crate::{
    ContextDependentDurationSemanticsDeclared,
    DurationAlternativeFormCarriesCompleteCalendarAndClockComponents,
    DurationAlternativeFormRequiresPartnerAgreement,
    DurationAlternativeFormUsesDateAndTimeComponentSlots,
    DurationTimeComponentsFollowTimeDesignator, DurationWeekFormNotMixedWithCalendarOrClockUnits,
    DurationWeekFormUsesSingleWeekUnit, ExactDurationSemanticsDeclared,
    ExplicitDurationCompositeRepresentationDeclared,
    ExplicitDurationPrecedenceRepresentationCarriesEvaluationOrder,
    ExplicitDurationRepresentationKindDeclared, SpeculativeDurationSemanticsDeclared,
};

/// Evidence bundle for the alternative complete duration representation.
///
/// Normative source: ISO 8601-1:2019, 4.4.3.3
/// Informative cross-checks: ISO/WD 8601-1:2016(E), 4.4.4.2.2; 4.4.4.3;
/// 4.4.4.4; 4.4.5
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
pub struct DurationAlternativeFormEvidence {
    /// Use of the alternative duration form is explicitly agreed by the parties.
    agreement: DurationAlternativeFormRequiresPartnerAgreement,
    /// The payload uses calendar-date and time-of-day component slots.
    slots: DurationAlternativeFormUsesDateAndTimeComponentSlots,
    /// The payload carries a complete calendar-and-clock component set.
    complete: DurationAlternativeFormCarriesCompleteCalendarAndClockComponents,
}
/// Evidence bundle for the designator-based duration representation.
///
/// Normative source: ISO 8601-1:2019, 4.4.3.2
/// Informative cross-checks: CalConnect CC 18011:2018 §7.3 —
/// Representations; ISO/WD 8601-1:2016(E), 4.4.4.2.1; 4.4.4.3; 4.4.4.4;
/// 4.4.5
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
pub struct DurationDesignatorRepresentationEvidence {
    /// Time components, when present, are placed after the `T` designator.
    time_components: DurationTimeComponentsFollowTimeDesignator,
    /// Week-form semantics when the duration is expressed as `PnnW`.
    week_form: Option<DurationWeekFormEvidence>,
}
/// Evidence bundle for a valid duration form.
///
/// Normative sources: ISO 8601-1:2019, 4.4.2 b); 4.4.3.2; 4.4.3.3
/// Informative cross-checks: CalConnect CC 18011:2018 §7.3 — Representations;
/// ISO/WD 8601-1:2016(E), 4.4.4.2.2; 4.4.4.3; 4.4.4.4; 4.4.5
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
pub struct DurationWeekFormEvidence {
    /// Week-form durations use the week-unit shape.
    form: DurationWeekFormUsesSingleWeekUnit,
    /// Week-form durations are not mixed with calendar or clock units.
    exclusive: DurationWeekFormNotMixedWithCalendarOrClockUnits,
}
/// Representation-specific evidence branch for a CalConnect explicit duration.
///
/// Normative source: CalConnect CC 18011:2018 §7.3-§7.5 — Representations
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum ExplicitDurationRepresentationEvidence {
    /// Simple representation semantics.
    Simple {
        /// The duration explicitly declares its representation family.
        representation_kind: ExplicitDurationRepresentationKindDeclared,
    },
    /// Composite representation semantics.
    Composite {
        /// The duration explicitly declares its representation family.
        representation_kind: ExplicitDurationRepresentationKindDeclared,
        /// The duration uses the composite representation family.
        composite: ExplicitDurationCompositeRepresentationDeclared,
    },
    /// Precedence representation semantics.
    Precedence {
        /// The duration explicitly declares its representation family.
        representation_kind: ExplicitDurationRepresentationKindDeclared,
        /// The representation preserves the declared evaluation order.
        precedence: ExplicitDurationPrecedenceRepresentationCarriesEvaluationOrder,
    },
}

impl core::default::Default for ExplicitDurationRepresentationEvidence {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Simple {
            representation_kind: core::default::Default::default(),
        }
    }
}
/// Exactness-specific evidence branch for a CalConnect explicit duration.
///
/// Normative source: CalConnect CC 18011:2018 §7.8-§7.10 — Exact, context-dependent, and speculative duration
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum ExplicitDurationSemanticEvidence {
    /// No exactness family is explicitly declared.
    None,
    /// Exact-duration semantics are declared.
    Exact {
        /// The duration is explicitly classified as exact.
        exact: ExactDurationSemanticsDeclared,
    },
    /// Context-dependent-duration semantics are declared.
    ContextDependent {
        /// The duration is explicitly classified as context-dependent.
        context_dependent: ContextDependentDurationSemanticsDeclared,
    },
    /// Speculative-duration semantics are declared.
    Speculative {
        /// The duration is explicitly classified as speculative.
        speculative: SpeculativeDurationSemanticsDeclared,
    },
}

impl core::default::Default for ExplicitDurationSemanticEvidence {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::None
    }
}
