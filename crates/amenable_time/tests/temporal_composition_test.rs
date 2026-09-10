//! Every `proof_composition` `#[derive(Witness)]` aggregate resolves as a
//! real `ClassifiedWitness<VerusVerifier>` when the `verus` feature is on
//! — mirror of the Kani/Creusot `temporal_composition_test`s. The checked
//! Verus leaves live in `src/verus_witness.rs`, the structural ones in
//! `src/structural_witness.rs`.

#![cfg(feature = "verus")]
#![allow(
    clippy::too_many_lines,
    reason = "one assertion per composed aggregate — a flat manifest"
)]

use amenable_core::ClassifiedWitness;
use amenable_std::VerusVerifier;
use amenable_time::{
    BackendConversionSemanticBundle, BackendConversionSemanticsValid, CalendarDateValid,
    CenturyValid, CombinedDateTimeDateEvidence, CompleteDateEvidence,
    CompleteDurationEndIntervalSubstitutionEvidence,
    CompleteIntervalDurationRepresentationEvidence, CompleteIntervalSubstitutionProofBranch,
    CompleteIntervalSubstitutionSemanticsValid,
    CompleteRecurringIntervalRepresentationSemanticsValid,
    CompleteStartDurationIntervalSubstitutionEvidence,
    CompleteStartEndIntervalSubstitutionEvidence, CompleteTimePointDateRepresentationEvidence,
    CompleteTimePointRepresentationEvidence, CompleteTimePointTimeRepresentationEvidence,
    ConversionLossless, ConversionTruncatesSubseconds, CriticalTimeZoneInconsistencyHandlingValid,
    DateTimeFormulaEvaluationResultBundle, DateTimeFormulaEvaluationResultValid,
    DateTimeFormulaEvaluationSemanticsValid, DateTimeFormulaSemanticBundle, DateTimeFormulaValid,
    DateValid, DateWithShiftValid, DecadeValid, DurationAlternativeFormEvidence,
    DurationDesignatorRepresentationEvidence, DurationFormValid, DurationRepresentationProofBranch,
    DurationRepresentationSemanticsValid, DurationSemanticBundle, DurationWeekFormEvidence,
    ElectiveTimeZoneInconsistencyHandlingValid, EnhancedIntervalLevelOneSemanticsValid,
    EnhancedIntervalLevelTwoSemanticsValid, ExplicitDateTimeValid, ExplicitDateTimeWithShiftValid,
    ExplicitDurationRepresentationEvidence, ExplicitDurationSemanticBundle,
    ExplicitDurationSemanticEvidence, ExplicitDurationValid,
    ExplicitIntervalDurationSubstitutionProofBranch,
    ExplicitIntervalDurationSubstitutionSemanticsValid,
    ExplicitIntervalEndComponentInheritanceProofBranch,
    ExplicitIntervalEndComponentInheritanceSemanticsValid,
    ExplicitIntervalShiftPropagationProofBranch, ExplicitIntervalShiftPropagationSemanticsValid,
    ExplicitTemporalFormSemanticBundle, ExplicitTemporalFormValid,
    ExplicitTimeIntervalSemanticBundle, ExplicitTimeIntervalValid, ExplicitTimeOfDayValid,
    ExplicitTimeShiftValid, ExtendedIntervalBoundarySemanticsValid, ExtendedYearBaseEvidence,
    ExtendedYearSignificantDigitsEvidence, ExtendedYearValid, GroupedTimeScaleUnitSemanticBundle,
    GroupedTimeScaleUnitValid, InheritedIntervalEndComponentsSemanticsValid,
    InheritedIntervalZoneSemanticsValid, IntervalEndComponentInheritanceProofBranch,
    IntervalEndpointOrderingBundle, IntervalEndpointsOrdered, IntervalZoneInheritanceProofBranch,
    IxdtfAdditionalInformationProofBranch, IxdtfAdditionalInformationSemanticsValid,
    IxdtfCalendarAnnotationProofBranch, IxdtfCalendarKeyRegistrySemanticsValid,
    IxdtfPermanentSuffixKeyRegistrationSemanticsValid,
    IxdtfProvisionalSuffixKeyRegistrationSemanticsValid, IxdtfSuffixKeyRegistryEntryValid,
    IxdtfSuffixKeyRegistryPolicySemanticsValid, IxdtfTimeZoneAnnotationProofBranch,
    IxdtfTimestampHasPreferredPresentationCalendar, IxdtfTimestampValid,
    LocalDateTimeDoesNotIdentifyFixedInstant, LocalDateTimeSemanticBundle, LocalDateTimeValid,
    LocalTimeScaleValid, LocalTimeSemanticsValid, LocalTimeValid,
    LocalTimeZoneResolutionProofBranch, LosslessConversionBundle, LossyConversionAuthorityBundle,
    LossyConversionAuthorityValid, MutualAgreementAuthorityScopeEvidence,
    MutualAgreementAuthorityValid, NamedTimeZoneIdentityValid,
    NamedTimeZoneInterpretationTracksTzdbRevision, NamedTimeZoneRevisionBundle,
    NamedTimeZoneSemanticBundle, NamedZoneAttachmentEvidence, OffsetConsistentWithNamedZone,
    OffsetDateTimeSemanticBundle, OffsetDateTimeValid, OffsetOnlyZoneSemanticsLimited,
    OffsetTimeZoneAnnotationConsistentWithTimestamp, OrdinalDateValid,
    OtherThanCompleteRecurringIntervalRepresentationSemanticsValid, PrecisionPreserved,
    QualificationPlacementEvidence, QualifiedTemporalExpressionValid,
    QualifiedTemporalValueSemanticBundle, QualifiedTemporalValueValid, RecurringIntervalFormValid,
    RecurringIntervalRepresentationProofBranch, RecurringIntervalSemanticBundle,
    RecurringIntervalWithRepeatRuleIntervalProofBranch, RecurringIntervalWithRepeatRuleValid,
    ReducedCalendarDatePrecisionEvidence, ReducedCalendarDateValid,
    ReducedLocalTimePrecisionEvidence, ReducedLocalTimeValid, RepeatRuleValid,
    Rfc3339DisplayGuidanceValid, Rfc3339GenerationGuidanceValid,
    Rfc3339LexicalOrderingSemanticsValid, Rfc3339TimestampValid, SeasonalTemporalExpressionValid,
    SelectionExpressionValid, StandardTimeOfDayValid, StandardTimeValid,
    SubYearGroupingExpressionValid, SubYearGroupingKindEvidence, SubsecondTruncationBundle,
    TemporalOrderingPreserved, TemporalSetExpressionValid, TemporalSetRangeSemanticsValid,
    TemporalSetSemanticBundle, TimeIntervalSemanticBundle, TimeIntervalValid,
    TimeOfDayWithShiftValid, TimeValid, TimestampRepresentsFixedInstant,
    UnspecifiedComponentExpressionValid, UtcOfDayValid, UtcOffsetKnown, UtcOffsetPrecisionEvidence,
    UtcOffsetValid, UtcTimeScaleValid, WeekDateValid, ZoneTransitionAmbiguitySemanticsValid,
    ZoneTransitionGapSemanticsValid, ZoneTransitionResolutionAuthorityBundle,
    ZoneTransitionResolutionAuthorityValid, ZonedDateTimeHasNamedZone, ZonedDateTimeSemanticBundle,
    ZonedTimestampEvidence, ZuluTimeZoneInconsistencyAvoidanceValid,
};

fn assert_classified<T: ClassifiedWitness<VerusVerifier>>() {}

#[test]
fn every_temporal_composition_is_a_classified_witness() {
    assert_classified::<BackendConversionSemanticBundle>();
    assert_classified::<BackendConversionSemanticsValid>();
    assert_classified::<CalendarDateValid>();
    assert_classified::<CenturyValid>();
    assert_classified::<CombinedDateTimeDateEvidence>();
    assert_classified::<CompleteDateEvidence>();
    assert_classified::<CompleteDurationEndIntervalSubstitutionEvidence>();
    assert_classified::<CompleteIntervalDurationRepresentationEvidence>();
    assert_classified::<CompleteIntervalSubstitutionProofBranch>();
    assert_classified::<CompleteIntervalSubstitutionSemanticsValid>();
    assert_classified::<CompleteRecurringIntervalRepresentationSemanticsValid>();
    assert_classified::<CompleteStartDurationIntervalSubstitutionEvidence>();
    assert_classified::<CompleteStartEndIntervalSubstitutionEvidence>();
    assert_classified::<CompleteTimePointDateRepresentationEvidence>();
    assert_classified::<CompleteTimePointRepresentationEvidence>();
    assert_classified::<CompleteTimePointTimeRepresentationEvidence>();
    assert_classified::<ConversionLossless>();
    assert_classified::<ConversionTruncatesSubseconds>();
    assert_classified::<CriticalTimeZoneInconsistencyHandlingValid>();
    assert_classified::<DateTimeFormulaEvaluationResultBundle>();
    assert_classified::<DateTimeFormulaEvaluationResultValid>();
    assert_classified::<DateTimeFormulaEvaluationSemanticsValid>();
    assert_classified::<DateTimeFormulaSemanticBundle>();
    assert_classified::<DateTimeFormulaValid>();
    assert_classified::<DateValid>();
    assert_classified::<DateWithShiftValid>();
    assert_classified::<DecadeValid>();
    assert_classified::<DurationAlternativeFormEvidence>();
    assert_classified::<DurationDesignatorRepresentationEvidence>();
    assert_classified::<DurationFormValid>();
    assert_classified::<DurationRepresentationProofBranch>();
    assert_classified::<DurationRepresentationSemanticsValid>();
    assert_classified::<DurationSemanticBundle>();
    assert_classified::<DurationWeekFormEvidence>();
    assert_classified::<ElectiveTimeZoneInconsistencyHandlingValid>();
    assert_classified::<EnhancedIntervalLevelOneSemanticsValid>();
    assert_classified::<EnhancedIntervalLevelTwoSemanticsValid>();
    assert_classified::<ExplicitDateTimeValid>();
    assert_classified::<ExplicitDateTimeWithShiftValid>();
    assert_classified::<ExplicitDurationRepresentationEvidence>();
    assert_classified::<ExplicitDurationSemanticBundle>();
    assert_classified::<ExplicitDurationSemanticEvidence>();
    assert_classified::<ExplicitDurationValid>();
    assert_classified::<ExplicitIntervalDurationSubstitutionProofBranch>();
    assert_classified::<ExplicitIntervalDurationSubstitutionSemanticsValid>();
    assert_classified::<ExplicitIntervalEndComponentInheritanceProofBranch>();
    assert_classified::<ExplicitIntervalEndComponentInheritanceSemanticsValid>();
    assert_classified::<ExplicitIntervalShiftPropagationProofBranch>();
    assert_classified::<ExplicitIntervalShiftPropagationSemanticsValid>();
    assert_classified::<ExplicitTemporalFormSemanticBundle>();
    assert_classified::<ExplicitTemporalFormValid>();
    assert_classified::<ExplicitTimeIntervalSemanticBundle>();
    assert_classified::<ExplicitTimeIntervalValid>();
    assert_classified::<ExplicitTimeOfDayValid>();
    assert_classified::<ExplicitTimeShiftValid>();
    assert_classified::<ExtendedIntervalBoundarySemanticsValid>();
    assert_classified::<ExtendedYearBaseEvidence>();
    assert_classified::<ExtendedYearSignificantDigitsEvidence>();
    assert_classified::<ExtendedYearValid>();
    assert_classified::<GroupedTimeScaleUnitSemanticBundle>();
    assert_classified::<GroupedTimeScaleUnitValid>();
    assert_classified::<InheritedIntervalEndComponentsSemanticsValid>();
    assert_classified::<InheritedIntervalZoneSemanticsValid>();
    assert_classified::<IntervalEndComponentInheritanceProofBranch>();
    assert_classified::<IntervalEndpointOrderingBundle>();
    assert_classified::<IntervalEndpointsOrdered>();
    assert_classified::<IntervalZoneInheritanceProofBranch>();
    assert_classified::<IxdtfAdditionalInformationProofBranch>();
    assert_classified::<IxdtfAdditionalInformationSemanticsValid>();
    assert_classified::<IxdtfCalendarAnnotationProofBranch>();
    assert_classified::<IxdtfCalendarKeyRegistrySemanticsValid>();
    assert_classified::<IxdtfPermanentSuffixKeyRegistrationSemanticsValid>();
    assert_classified::<IxdtfProvisionalSuffixKeyRegistrationSemanticsValid>();
    assert_classified::<IxdtfSuffixKeyRegistryEntryValid>();
    assert_classified::<IxdtfSuffixKeyRegistryPolicySemanticsValid>();
    assert_classified::<IxdtfTimeZoneAnnotationProofBranch>();
    assert_classified::<IxdtfTimestampHasPreferredPresentationCalendar>();
    assert_classified::<IxdtfTimestampValid>();
    assert_classified::<LocalDateTimeDoesNotIdentifyFixedInstant>();
    assert_classified::<LocalDateTimeSemanticBundle>();
    assert_classified::<LocalDateTimeValid>();
    assert_classified::<LocalTimeScaleValid>();
    assert_classified::<LocalTimeSemanticsValid>();
    assert_classified::<LocalTimeValid>();
    assert_classified::<LocalTimeZoneResolutionProofBranch>();
    assert_classified::<LosslessConversionBundle>();
    assert_classified::<LossyConversionAuthorityBundle>();
    assert_classified::<LossyConversionAuthorityValid>();
    assert_classified::<MutualAgreementAuthorityScopeEvidence>();
    assert_classified::<MutualAgreementAuthorityValid>();
    assert_classified::<NamedTimeZoneIdentityValid>();
    assert_classified::<NamedTimeZoneInterpretationTracksTzdbRevision>();
    assert_classified::<NamedTimeZoneRevisionBundle>();
    assert_classified::<NamedTimeZoneSemanticBundle>();
    assert_classified::<NamedZoneAttachmentEvidence>();
    assert_classified::<OffsetConsistentWithNamedZone>();
    assert_classified::<OffsetDateTimeSemanticBundle>();
    assert_classified::<OffsetDateTimeValid>();
    assert_classified::<OffsetOnlyZoneSemanticsLimited>();
    assert_classified::<OffsetTimeZoneAnnotationConsistentWithTimestamp>();
    assert_classified::<OrdinalDateValid>();
    assert_classified::<OtherThanCompleteRecurringIntervalRepresentationSemanticsValid>();
    assert_classified::<PrecisionPreserved>();
    assert_classified::<QualificationPlacementEvidence>();
    assert_classified::<QualifiedTemporalExpressionValid>();
    assert_classified::<QualifiedTemporalValueSemanticBundle>();
    assert_classified::<QualifiedTemporalValueValid>();
    assert_classified::<RecurringIntervalFormValid>();
    assert_classified::<RecurringIntervalRepresentationProofBranch>();
    assert_classified::<RecurringIntervalSemanticBundle>();
    assert_classified::<RecurringIntervalWithRepeatRuleIntervalProofBranch>();
    assert_classified::<RecurringIntervalWithRepeatRuleValid>();
    assert_classified::<ReducedCalendarDatePrecisionEvidence>();
    assert_classified::<ReducedCalendarDateValid>();
    assert_classified::<ReducedLocalTimePrecisionEvidence>();
    assert_classified::<ReducedLocalTimeValid>();
    assert_classified::<RepeatRuleValid>();
    assert_classified::<Rfc3339DisplayGuidanceValid>();
    assert_classified::<Rfc3339GenerationGuidanceValid>();
    assert_classified::<Rfc3339LexicalOrderingSemanticsValid>();
    assert_classified::<Rfc3339TimestampValid>();
    assert_classified::<SeasonalTemporalExpressionValid>();
    assert_classified::<SelectionExpressionValid>();
    assert_classified::<StandardTimeOfDayValid>();
    assert_classified::<StandardTimeValid>();
    assert_classified::<SubYearGroupingExpressionValid>();
    assert_classified::<SubYearGroupingKindEvidence>();
    assert_classified::<SubsecondTruncationBundle>();
    assert_classified::<TemporalOrderingPreserved>();
    assert_classified::<TemporalSetExpressionValid>();
    assert_classified::<TemporalSetRangeSemanticsValid>();
    assert_classified::<TemporalSetSemanticBundle>();
    assert_classified::<TimeIntervalSemanticBundle>();
    assert_classified::<TimeIntervalValid>();
    assert_classified::<TimeOfDayWithShiftValid>();
    assert_classified::<TimeValid>();
    assert_classified::<TimestampRepresentsFixedInstant>();
    assert_classified::<UnspecifiedComponentExpressionValid>();
    assert_classified::<UtcOfDayValid>();
    assert_classified::<UtcOffsetKnown>();
    assert_classified::<UtcOffsetPrecisionEvidence>();
    assert_classified::<UtcOffsetValid>();
    assert_classified::<UtcTimeScaleValid>();
    assert_classified::<WeekDateValid>();
    assert_classified::<ZoneTransitionAmbiguitySemanticsValid>();
    assert_classified::<ZoneTransitionGapSemanticsValid>();
    assert_classified::<ZoneTransitionResolutionAuthorityBundle>();
    assert_classified::<ZoneTransitionResolutionAuthorityValid>();
    assert_classified::<ZonedDateTimeHasNamedZone>();
    assert_classified::<ZonedDateTimeSemanticBundle>();
    assert_classified::<ZonedTimestampEvidence>();
    assert_classified::<ZuluTimeZoneInconsistencyAvoidanceValid>();
}
