//! Temporal proof composition — the composed (`Evidence`) half of the
//! contract graph, ported from `elicit_temporal::contracts::proof_composition`.
//!
//! Each aggregate `*Valid` proposition is folded with its
//! `elicit_temporal` `*Evidence` bundle: the aggregate's fields ARE its
//! sub-claims (a `ProvableFrom<FooEvidence> for FooValid` edge becomes
//! field containment), and `#[derive(Evidence, Witness)]` makes the
//! composite `Witness<V>` proof the structural product of its members'
//! proofs. No proof tokens are carried — the aggregate is *named*, its
//! decomposition lives in its fields. Leaf `Witness<V>` impls for the
//! member `Standard`s land in the backend crates (orphan rules); genuine
//! *transitions* become `Establish` / `Exchange` edges in Phase 4.

mod branches_a;
mod branches_b;
mod composites_a;
mod composites_b;
mod composites_c;
mod composites_d;
mod proof_branches;

pub use branches_a::{
    CombinedDateTimeDateEvidence, CompleteDateEvidence,
    CompleteDurationEndIntervalSubstitutionEvidence,
    CompleteIntervalDurationRepresentationEvidence,
    CompleteStartDurationIntervalSubstitutionEvidence,
    CompleteStartEndIntervalSubstitutionEvidence, CompleteTimePointDateRepresentationEvidence,
    CompleteTimePointRepresentationEvidence, CompleteTimePointTimeRepresentationEvidence,
    DurationAlternativeFormEvidence, DurationDesignatorRepresentationEvidence,
    DurationWeekFormEvidence,
};
pub use branches_b::{
    ExplicitDurationRepresentationEvidence, ExplicitDurationSemanticEvidence,
    ExtendedYearBaseEvidence, ExtendedYearSignificantDigitsEvidence,
    MutualAgreementAuthorityScopeEvidence, NamedZoneAttachmentEvidence,
    QualificationPlacementEvidence, ReducedCalendarDatePrecisionEvidence,
    ReducedLocalTimePrecisionEvidence, SubYearGroupingKindEvidence, UtcOffsetPrecisionEvidence,
    ZonedTimestampEvidence,
};
pub use composites_a::{
    BackendConversionSemanticsValid, CalendarDateValid, CenturyValid,
    CompleteIntervalSubstitutionSemanticsValid,
    CompleteRecurringIntervalRepresentationSemanticsValid, ConversionLossless,
    ConversionTruncatesSubseconds, CriticalTimeZoneInconsistencyHandlingValid,
    DateTimeFormulaEvaluationResultValid, DateTimeFormulaEvaluationSemanticsValid,
    DateTimeFormulaValid, DateValid, DateWithShiftValid, DecadeValid, DurationFormValid,
    DurationRepresentationSemanticsValid, ElectiveTimeZoneInconsistencyHandlingValid,
    EnhancedIntervalLevelOneSemanticsValid, EnhancedIntervalLevelTwoSemanticsValid,
    ExplicitDateTimeValid, ExplicitDateTimeWithShiftValid, ExplicitDurationValid,
    ExplicitIntervalDurationSubstitutionSemanticsValid,
    ExplicitIntervalEndComponentInheritanceSemanticsValid,
};
pub use composites_b::{
    ExplicitIntervalShiftPropagationSemanticsValid, ExplicitTemporalFormValid,
    ExplicitTimeIntervalValid, ExplicitTimeOfDayValid, ExplicitTimeShiftValid,
    ExtendedIntervalBoundarySemanticsValid, ExtendedYearValid, GroupedTimeScaleUnitValid,
    InheritedIntervalEndComponentsSemanticsValid, InheritedIntervalZoneSemanticsValid,
    IntervalEndpointsOrdered, IxdtfAdditionalInformationSemanticsValid,
    IxdtfCalendarKeyRegistrySemanticsValid, IxdtfPermanentSuffixKeyRegistrationSemanticsValid,
    IxdtfProvisionalSuffixKeyRegistrationSemanticsValid, IxdtfSuffixKeyRegistryEntryValid,
    IxdtfSuffixKeyRegistryPolicySemanticsValid, IxdtfTimestampHasPreferredPresentationCalendar,
    IxdtfTimestampValid, LocalDateTimeDoesNotIdentifyFixedInstant, LocalDateTimeValid,
    LocalTimeScaleValid, LocalTimeSemanticsValid, LocalTimeValid,
};
pub use composites_c::{
    LossyConversionAuthorityValid, MutualAgreementAuthorityValid, NamedTimeZoneIdentityValid,
    NamedTimeZoneInterpretationTracksTzdbRevision, OffsetConsistentWithNamedZone,
    OffsetDateTimeValid, OffsetOnlyZoneSemanticsLimited,
    OffsetTimeZoneAnnotationConsistentWithTimestamp, OrdinalDateValid,
    OtherThanCompleteRecurringIntervalRepresentationSemanticsValid, PrecisionPreserved,
    QualifiedTemporalExpressionValid, QualifiedTemporalValueValid, RecurringIntervalFormValid,
    RecurringIntervalWithRepeatRuleValid, ReducedCalendarDateValid, ReducedLocalTimeValid,
    RepeatRuleValid, Rfc3339DisplayGuidanceValid, Rfc3339GenerationGuidanceValid,
    Rfc3339LexicalOrderingSemanticsValid, Rfc3339TimestampValid, SeasonalTemporalExpressionValid,
};
pub use composites_d::{
    SelectionExpressionValid, StandardTimeOfDayValid, StandardTimeValid,
    SubYearGroupingExpressionValid, TemporalOrderingPreserved, TemporalSetExpressionValid,
    TemporalSetRangeSemanticsValid, TimeIntervalValid, TimeOfDayWithShiftValid, TimeValid,
    TimestampRepresentsFixedInstant, UnspecifiedComponentExpressionValid, UtcOfDayValid,
    UtcOffsetKnown, UtcOffsetValid, UtcTimeScaleValid, WeekDateValid,
    ZoneTransitionAmbiguitySemanticsValid, ZoneTransitionGapSemanticsValid,
    ZoneTransitionResolutionAuthorityValid, ZonedDateTimeHasNamedZone,
    ZuluTimeZoneInconsistencyAvoidanceValid,
};
pub use proof_branches::{
    CompleteIntervalSubstitutionProofBranch, DurationRepresentationProofBranch,
    ExplicitIntervalDurationSubstitutionProofBranch,
    ExplicitIntervalEndComponentInheritanceProofBranch,
    ExplicitIntervalShiftPropagationProofBranch, IntervalEndComponentInheritanceProofBranch,
    IntervalZoneInheritanceProofBranch, IxdtfAdditionalInformationProofBranch,
    IxdtfCalendarAnnotationProofBranch, IxdtfTimeZoneAnnotationProofBranch,
    LocalTimeZoneResolutionProofBranch, RecurringIntervalRepresentationProofBranch,
    RecurringIntervalWithRepeatRuleIntervalProofBranch,
};
