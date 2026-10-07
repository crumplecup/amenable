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

mod composites;
mod evidence;
mod proof_branches;
mod semantic_bundles;

pub use composites::{
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
    ExplicitIntervalShiftPropagationSemanticsValid, ExplicitTemporalFormValid,
    ExplicitTimeIntervalValid, ExplicitTimeOfDayValid, ExplicitTimeShiftValid,
    ExtendedIntervalBoundarySemanticsValid, ExtendedYearValid, GroupedTimeScaleUnitValid,
    InheritedIntervalEndComponentsSemanticsValid, InheritedIntervalZoneSemanticsValid,
    IntervalEndpointsOrdered, IxdtfAdditionalInformationSemanticsValid,
    IxdtfCalendarKeyRegistrySemanticsValid, IxdtfPermanentSuffixKeyRegistrationSemanticsValid,
    IxdtfProvisionalSuffixKeyRegistrationSemanticsValid, IxdtfSuffixKeyRegistryEntryValid,
    IxdtfSuffixKeyRegistryPolicySemanticsValid, IxdtfTimestampHasPreferredPresentationCalendar,
    IxdtfTimestampValid, LocalDateTimeDoesNotIdentifyFixedInstant, LocalDateTimeValid,
    LocalTimeScaleValid, LocalTimeSemanticsValid, LocalTimeValid, LossyConversionAuthorityValid,
    MutualAgreementAuthorityValid, NamedTimeZoneIdentityValid,
    NamedTimeZoneInterpretationTracksTzdbRevision, OffsetConsistentWithNamedZone,
    OffsetDateTimeValid, OffsetOnlyZoneSemanticsLimited,
    OffsetTimeZoneAnnotationConsistentWithTimestamp, OrdinalDateValid,
    OtherThanCompleteRecurringIntervalRepresentationSemanticsValid, PrecisionPreserved,
    QualifiedTemporalExpressionValid, QualifiedTemporalValueValid, RecurringIntervalFormValid,
    RecurringIntervalWithRepeatRuleValid, ReducedCalendarDateValid, ReducedLocalTimeValid,
    RepeatRuleValid, Rfc3339DisplayGuidanceValid, Rfc3339GenerationGuidanceValid,
    Rfc3339LexicalOrderingSemanticsValid, Rfc3339TimestampValid, SeasonalTemporalExpressionValid,
    SelectionExpressionValid, StandardTimeOfDayValid, StandardTimeValid,
    SubYearGroupingExpressionValid, TemporalOrderingPreserved, TemporalSetExpressionValid,
    TemporalSetRangeSemanticsValid, TimeIntervalValid, TimeOfDayWithShiftValid, TimeValid,
    TimestampRepresentsFixedInstant, UnspecifiedComponentExpressionValid, UtcOfDayValid,
    UtcOffsetKnown, UtcOffsetValid, UtcTimeScaleValid, WeekDateValid,
    ZoneTransitionAmbiguitySemanticsValid, ZoneTransitionGapSemanticsValid,
    ZoneTransitionResolutionAuthorityValid, ZonedDateTimeHasNamedZone, ZonedDateValid,
    ZuluTimeZoneInconsistencyAvoidanceValid,
};
pub use evidence::{
    CombinedDateTimeDateEvidence, CompleteDateEvidence,
    CompleteDurationEndIntervalSubstitutionEvidence,
    CompleteIntervalDurationRepresentationEvidence,
    CompleteStartDurationIntervalSubstitutionEvidence,
    CompleteStartEndIntervalSubstitutionEvidence, CompleteTimePointDateRepresentationEvidence,
    CompleteTimePointRepresentationEvidence, CompleteTimePointTimeRepresentationEvidence,
    DurationAlternativeFormEvidence, DurationDesignatorRepresentationEvidence,
    DurationWeekFormEvidence, ExplicitDurationRepresentationEvidence,
    ExplicitDurationSemanticEvidence, ExtendedYearBaseEvidence,
    ExtendedYearSignificantDigitsEvidence, MutualAgreementAuthorityScopeEvidence,
    NamedZoneAttachmentEvidence, QualificationPlacementEvidence,
    ReducedCalendarDatePrecisionEvidence, ReducedLocalTimePrecisionEvidence,
    SubYearGroupingKindEvidence, UtcOffsetPrecisionEvidence, ZonedTimestampEvidence,
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
pub use semantic_bundles::{
    BackendConversionSemanticBundle, DateTimeFormulaEvaluationResultBundle,
    DateTimeFormulaSemanticBundle, DurationSemanticBundle, ExplicitDurationSemanticBundle,
    ExplicitTemporalFormSemanticBundle, ExplicitTimeIntervalSemanticBundle,
    GroupedTimeScaleUnitSemanticBundle, IntervalEndpointOrderingBundle,
    LocalDateTimeSemanticBundle, LosslessConversionBundle, LossyConversionAuthorityBundle,
    NamedTimeZoneRevisionBundle, NamedTimeZoneSemanticBundle, OffsetDateTimeSemanticBundle,
    QualifiedTemporalValueSemanticBundle, RecurringIntervalSemanticBundle,
    SubsecondTruncationBundle, TemporalSetSemanticBundle, TimeIntervalSemanticBundle,
    ZoneTransitionResolutionAuthorityBundle, ZonedDateTimeSemanticBundle,
};
