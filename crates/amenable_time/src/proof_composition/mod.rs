//! Temporal proof composition — the composed (`Evidence`) half of the
//! contract graph, ported from
//! `elicit_temporal::contracts::proof_composition`.
//!
//! - [`aggregates`] — the 93 aggregate proof propositions, each an
//!   `Evidence` claim (`temporal_evidence!`).
//! - `*Evidence` credential bundles and the `Establish` edges that mint
//!   the aggregates from lower-order `Standard`s land here in later
//!   Phase 3 steps.

mod aggregates;

pub use aggregates::{
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
    ZoneTransitionResolutionAuthorityValid, ZonedDateTimeHasNamedZone,
    ZuluTimeZoneInconsistencyAvoidanceValid,
};
