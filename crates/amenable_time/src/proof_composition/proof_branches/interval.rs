//! Duration/interval/recurring-interval proof-branch discriminants.
//!
//!
//! Proof-branch discriminants -- the 13 `*ProofBranch` enums from
//! `elicit_temporal::types`, deferred from Phase 2 because they carry
//! `Established<T>` / `*Evidence` payloads. Folded the same way as the
//! `proof_composition::composites` family: `Established<X>` -> `X`, and
//! the elicit_temporal `evidence:` sidecar field is dropped where it is
//! type-identical to the aggregate it accompanies (the folded composite
//! already carries its own decomposition). Each is `#[derive(Evidence,
//! Witness)]` -- a proof-branch discriminant for the semantic bundles /
//! exchange outputs of Phases 4-5.

use crate::{
    CompleteDurationEndIntervalSubstitutionEvidence, CompleteIntervalSubstitutionSemanticsValid,
    CompleteRecurringIntervalRepresentationSemanticsValid,
    CompleteStartDurationIntervalSubstitutionEvidence,
    CompleteStartEndIntervalSubstitutionEvidence, DurationAlternativeFormEvidence,
    DurationDesignatorRepresentationEvidence, DurationRepresentationSemanticsValid,
    ExplicitIntervalDurationSubstitutionSemanticsValid,
    ExplicitIntervalEndComponentInheritanceSemanticsValid,
    ExplicitIntervalShiftPropagationSemanticsValid, ExplicitTimeIntervalValid,
    InheritedIntervalEndComponentsSemanticsValid, InheritedIntervalZoneSemanticsValid,
    OtherThanCompleteRecurringIntervalRepresentationSemanticsValid, TimeIntervalValid,
};

/// Explicit proof branch for inherited end-component interval semantics.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Default, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum IntervalEndComponentInheritanceProofBranch {
    /// The interval does not rely on inherited higher-order end components.
    #[default]
    None,
    /// The trailing interval component inherits omitted higher-order components.
    Inherited {
        /// Aggregate proof that inherited end-component semantics are explicit and lawful.
        semantics: InheritedIntervalEndComponentsSemanticsValid,
    },
}
/// Explicit proof branch for inherited trailing-zone interval semantics.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Default, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum IntervalZoneInheritanceProofBranch {
    /// The interval does not rely on inherited trailing-zone semantics.
    #[default]
    None,
    /// The trailing interval component inherits omitted zone or UTC semantics.
    Inherited {
        /// Aggregate proof that inherited trailing-zone semantics are explicit and lawful.
        semantics: InheritedIntervalZoneSemanticsValid,
    },
}
/// Explicit proof branch for CalConnect explicit-interval duration substitution semantics.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Default, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum ExplicitIntervalDurationSubstitutionProofBranch {
    /// The explicit interval uses concrete start/end boundaries, so duration substitution does not apply.
    #[default]
    NotApplicable,
    /// The explicit interval uses a start boundary plus an explicit duration.
    StartDuration {
        /// Aggregate proof that duration-substitution semantics are explicit and lawful.
        semantics: ExplicitIntervalDurationSubstitutionSemanticsValid,
    },
    /// The explicit interval uses an explicit duration plus an end boundary.
    DurationEnd {
        /// Aggregate proof that duration-substitution semantics are explicit and lawful.
        semantics: ExplicitIntervalDurationSubstitutionSemanticsValid,
    },
}
/// Explicit proof branch for CalConnect explicit-interval trailing-end inheritance semantics.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Default, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum ExplicitIntervalEndComponentInheritanceProofBranch {
    /// The explicit interval does not rely on trailing-end higher-order inheritance.
    #[default]
    None,
    /// The trailing explicit endpoint inherits omitted higher-order components from the start.
    Inherited {
        /// Aggregate proof that trailing-end inheritance semantics are explicit and lawful.
        semantics: ExplicitIntervalEndComponentInheritanceSemanticsValid,
    },
}
/// Explicit proof branch for CalConnect explicit-interval leading-shift propagation semantics.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Default, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum ExplicitIntervalShiftPropagationProofBranch {
    /// The explicit interval does not rely on leading-shift propagation.
    #[default]
    None,
    /// The leading explicit time shift applies to the trailing endpoint absent an explicit override.
    Propagated {
        /// Aggregate proof that leading-shift propagation semantics are explicit and lawful.
        semantics: ExplicitIntervalShiftPropagationSemanticsValid,
    },
}
/// Explicit proof branch for duration representation family semantics.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum DurationRepresentationProofBranch {
    /// The duration uses the designator-based representation family.
    Designator {
        /// Aggregate proof that duration representation-family semantics are explicit and lawful.
        semantics: DurationRepresentationSemanticsValid,
        /// Evidence bundle for the designator-based duration branch.
        evidence: DurationDesignatorRepresentationEvidence,
    },
    /// The duration uses the alternative complete representation family.
    Alternative {
        /// Aggregate proof that duration representation-family semantics are explicit and lawful.
        semantics: DurationRepresentationSemanticsValid,
        /// Evidence bundle for the alternative duration branch.
        evidence: DurationAlternativeFormEvidence,
    },
}

impl core::default::Default for DurationRepresentationProofBranch {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Designator {
            semantics: core::default::Default::default(),
            evidence: core::default::Default::default(),
        }
    }
}
/// Explicit proof branch for complete-interval substitution semantics.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Default, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum CompleteIntervalSubstitutionProofBranch {
    /// The interval is not a complete `4.4.4` representation, so `4.4.4.5` substitutions do not apply.
    #[default]
    NotApplicable,
    /// The interval is a complete start/end representation with explicit time-point substitution evidence.
    StartEnd {
        /// Aggregate proof that complete-interval substitution semantics are explicit and lawful.
        semantics: CompleteIntervalSubstitutionSemanticsValid,
        /// Evidence bundle for the complete start/end branch.
        evidence: CompleteStartEndIntervalSubstitutionEvidence,
    },
    /// The interval is a complete start/duration representation with explicit substitution evidence.
    StartDuration {
        /// Aggregate proof that complete-interval substitution semantics are explicit and lawful.
        semantics: CompleteIntervalSubstitutionSemanticsValid,
        /// Evidence bundle for the complete start/duration branch.
        evidence: CompleteStartDurationIntervalSubstitutionEvidence,
    },
    /// The interval is a complete duration/end representation with explicit substitution evidence.
    DurationEnd {
        /// Aggregate proof that complete-interval substitution semantics are explicit and lawful.
        semantics: CompleteIntervalSubstitutionSemanticsValid,
        /// Evidence bundle for the complete duration/end branch.
        evidence: CompleteDurationEndIntervalSubstitutionEvidence,
    },
}
/// Explicit proof branch for recurring-interval representation family semantics.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum RecurringIntervalRepresentationProofBranch {
    /// The recurring interval embeds a complete time-interval representation.
    Complete {
        /// Aggregate proof that complete recurring-interval semantics are explicit and lawful.
        semantics: CompleteRecurringIntervalRepresentationSemanticsValid,
    },
    /// The recurring interval embeds an other-than-complete time-interval representation.
    OtherThanComplete {
        /// Aggregate proof that other-than-complete recurring-interval semantics are explicit and lawful.
        semantics: OtherThanCompleteRecurringIntervalRepresentationSemanticsValid,
    },
}

impl core::default::Default for RecurringIntervalRepresentationProofBranch {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Complete {
            semantics: core::default::Default::default(),
        }
    }
}
/// Explicit proof branch for the embedded interval family of a recurring interval with repeat rule.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum RecurringIntervalWithRepeatRuleIntervalProofBranch {
    /// The recurring representation embeds a complete ISO 8601 interval.
    IsoComplete {
        /// The embedded ISO interval is structurally valid.
        interval: TimeIntervalValid,
        /// The embedded complete ISO interval carries explicit substitution-law sidecars.
        substitution: CompleteIntervalSubstitutionProofBranch,
    },
    /// The recurring representation embeds a CalConnect explicit interval.
    Explicit {
        /// The embedded CalConnect explicit interval is structurally valid.
        interval: ExplicitTimeIntervalValid,
        /// The embedded explicit interval carries duration-substitution semantics.
        duration_substitution: ExplicitIntervalDurationSubstitutionProofBranch,
        /// The embedded explicit interval carries trailing-end inheritance semantics.
        end_component_inheritance: ExplicitIntervalEndComponentInheritanceProofBranch,
        /// The embedded explicit interval carries leading-shift propagation semantics.
        shift_propagation: ExplicitIntervalShiftPropagationProofBranch,
    },
}

impl core::default::Default for RecurringIntervalWithRepeatRuleIntervalProofBranch {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::IsoComplete {
            interval: core::default::Default::default(),
            substitution: core::default::Default::default(),
        }
    }
}
