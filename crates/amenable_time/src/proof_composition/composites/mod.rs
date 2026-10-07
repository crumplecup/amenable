//! Aggregate temporal proof propositions, ported from
//! `elicit_temporal::contracts::proof_composition`. Each aggregate is
//! folded with its `*Evidence` bundle: the aggregate's fields ARE its
//! sub-claims, and `#[derive(Witness)]` makes the composite proof the
//! structural product of its members' proofs. See [`super`] for the
//! design; leaf `Witness<V>` impls land in the backend crates.
//!
//! Split by real domain rather than alphabetically: `calendar_date`
//! (calendar/ordinal/week-date), `clock` (local-time/standard-time),
//! `zone` (UTC-offset/named-zone/zone-transition), `datetime`
//! (local/offset/CalConnect-explicit date-time), `timestamp` (RFC 3339 /
//! IXDTF), `interval` (duration/interval/recurring-interval),
//! `extension` (the CalConnect/ISO 8601-2 explicit-form family), and
//! `conversion` (backend-conversion/precision/qualification) — matching
//! the domain split already used by `exchange::format`/`format_props`
//! and `traits::native_bridge`. `clock`/`zone`/`timestamp`/`interval`/
//! `extension` each hold more than 10 types, so each is further split
//! by sub-concern into its own directory (e.g. `zone::identity` /
//! `zone::suffix` / `zone::transition`).

mod calendar_date;
mod clock;
mod conversion;
mod datetime;
mod extension;
mod interval;
mod timestamp;
mod zone;

pub use calendar_date::{
    CalendarDateValid, CenturyValid, DateValid, DateWithShiftValid, DecadeValid, ExtendedYearValid,
    OrdinalDateValid, ReducedCalendarDateValid, WeekDateValid,
};
pub use clock::{
    ExplicitTimeOfDayValid, ExplicitTimeShiftValid, LocalTimeScaleValid, LocalTimeSemanticsValid,
    LocalTimeValid, ReducedLocalTimeValid, StandardTimeOfDayValid, StandardTimeValid,
    TimeOfDayWithShiftValid, TimeValid, UtcOfDayValid,
};
pub use conversion::{
    BackendConversionSemanticsValid, ConversionLossless, ConversionTruncatesSubseconds,
    LossyConversionAuthorityValid, MutualAgreementAuthorityValid, PrecisionPreserved,
    QualifiedTemporalExpressionValid, QualifiedTemporalValueValid, TemporalOrderingPreserved,
};
pub use datetime::{
    ExplicitDateTimeValid, ExplicitDateTimeWithShiftValid, LocalDateTimeValid, OffsetDateTimeValid,
};
pub use extension::{
    DateTimeFormulaEvaluationResultValid, DateTimeFormulaEvaluationSemanticsValid,
    DateTimeFormulaValid, ExplicitTemporalFormValid, GroupedTimeScaleUnitValid,
    SeasonalTemporalExpressionValid, SelectionExpressionValid, SubYearGroupingExpressionValid,
    TemporalSetExpressionValid, TemporalSetRangeSemanticsValid,
    UnspecifiedComponentExpressionValid,
};
pub use interval::{
    CompleteIntervalSubstitutionSemanticsValid,
    CompleteRecurringIntervalRepresentationSemanticsValid, DurationFormValid,
    DurationRepresentationSemanticsValid, EnhancedIntervalLevelOneSemanticsValid,
    EnhancedIntervalLevelTwoSemanticsValid, ExplicitDurationValid,
    ExplicitIntervalDurationSubstitutionSemanticsValid,
    ExplicitIntervalEndComponentInheritanceSemanticsValid,
    ExplicitIntervalShiftPropagationSemanticsValid, ExplicitTimeIntervalValid,
    ExtendedIntervalBoundarySemanticsValid, InheritedIntervalEndComponentsSemanticsValid,
    InheritedIntervalZoneSemanticsValid, IntervalEndpointsOrdered,
    OtherThanCompleteRecurringIntervalRepresentationSemanticsValid, RecurringIntervalFormValid,
    RecurringIntervalWithRepeatRuleValid, RepeatRuleValid, TimeIntervalValid,
};
pub use timestamp::{
    IxdtfAdditionalInformationSemanticsValid, IxdtfCalendarKeyRegistrySemanticsValid,
    IxdtfPermanentSuffixKeyRegistrationSemanticsValid,
    IxdtfProvisionalSuffixKeyRegistrationSemanticsValid, IxdtfSuffixKeyRegistryEntryValid,
    IxdtfSuffixKeyRegistryPolicySemanticsValid, IxdtfTimestampHasPreferredPresentationCalendar,
    IxdtfTimestampValid, Rfc3339DisplayGuidanceValid, Rfc3339GenerationGuidanceValid,
    Rfc3339LexicalOrderingSemanticsValid, Rfc3339TimestampValid, TimestampRepresentsFixedInstant,
};
pub use zone::{
    CriticalTimeZoneInconsistencyHandlingValid, ElectiveTimeZoneInconsistencyHandlingValid,
    LocalDateTimeDoesNotIdentifyFixedInstant, NamedTimeZoneIdentityValid,
    NamedTimeZoneInterpretationTracksTzdbRevision, OffsetConsistentWithNamedZone,
    OffsetOnlyZoneSemanticsLimited, OffsetTimeZoneAnnotationConsistentWithTimestamp,
    UtcOffsetKnown, UtcOffsetValid, UtcTimeScaleValid, ZoneTransitionAmbiguitySemanticsValid,
    ZoneTransitionGapSemanticsValid, ZoneTransitionResolutionAuthorityValid,
    ZonedDateTimeHasNamedZone, ZonedDateValid, ZuluTimeZoneInconsistencyAvoidanceValid,
};
