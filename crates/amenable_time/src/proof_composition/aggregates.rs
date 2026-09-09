//! Aggregate temporal proof propositions — the composed half of the
//! `Standard` / `Evidence` split, ported from
//! `elicit_temporal::contracts::proof_composition` (93 `structural_prop!`
//! aggregates there).
//!
//! Each is an [`Evidence`](amenable_core::Evidence) claim: a proposition
//! a producer proves (a real `Witness<V>` in the backend crates, Phase 6)
//! and a consumer relies on. The `*Evidence` credential bundles and the
//! `Establish` edges that mint these land alongside in sibling modules
//! (`docs/AMENABLE_TIME_PLAN.md`, Phase 3).

temporal_evidence! {
    /// Aggregate proof that a complete calendar date is structurally valid.
    CalendarDateValid;
    /// Aggregate proof that a reduced-precision calendar date is structurally valid.
    ReducedCalendarDateValid;
    /// Aggregate proof that an ISO 8601 date representation is structurally valid.
    DateValid;
    /// Aggregate proof that an ISO 8601 time-of-day representation is structurally valid.
    TimeValid;
    /// Aggregate proof that a Gregorian calendar decade representation is structurally valid.
    DecadeValid;
    /// Aggregate proof that a Gregorian calendar century representation is structurally valid.
    CenturyValid;
    /// Aggregate proof that an ISO 8601-2 extended year form is structurally valid.
    ExtendedYearValid;
    /// Aggregate proof that a complete ordinal date is structurally valid.
    OrdinalDateValid;
    /// Aggregate proof that a complete week date is structurally valid.
    WeekDateValid;
    /// Aggregate proof that a local time representation is structurally valid.
    LocalTimeValid;
    /// Aggregate proof that a reduced-accuracy local time representation is structurally valid.
    ReducedLocalTimeValid;
    /// Aggregate proof that the UTC reference time scale semantics are established.
    UtcTimeScaleValid;
    /// Aggregate proof that a UTC-of-day representation is structurally valid.
    UtcOfDayValid;
    /// Aggregate proof that a numeric UTC offset is structurally valid.
    UtcOffsetValid;
    /// Aggregate proof that the UTC relationship does not use unknown-local-offset semantics.
    UtcOffsetKnown;
    /// Aggregate proof that standard-time semantics are established.
    StandardTimeValid;
    /// Aggregate proof that a standard-time-of-day representation is structurally valid.
    StandardTimeOfDayValid;
    /// Aggregate proof that a local time-scale interpretation is established.
    LocalTimeScaleValid;
    /// Aggregate proof that local-time semantics are established.
    LocalTimeSemanticsValid;
    /// Aggregate proof that a combined local date-time representation is structurally valid.
    LocalDateTimeValid;
    /// Aggregate proof that an offset date-time representation is structurally valid.
    OffsetDateTimeValid;
    /// Aggregate proof that a complete explicit-form date with shift is structurally valid.
    DateWithShiftValid;
    /// Aggregate proof that a complete explicit-form time of day with shift is structurally valid.
    TimeOfDayWithShiftValid;
    /// Aggregate proof that a timestamp denotes a single fixed instant.
    TimestampRepresentsFixedInstant;
    /// Aggregate proof that a local date-time does not, by itself, identify a fixed instant.
    LocalDateTimeDoesNotIdentifyFixedInstant;
    /// Aggregate proof that a standards-governed mutual-agreement authority is explicit and lawful.
    MutualAgreementAuthorityValid;
    /// Aggregate proof that explicit local-to-zone resolution authority is fully declared.
    ZoneTransitionResolutionAuthorityValid;
    /// Aggregate proof that ambiguous local-time resolution semantics are explicit and lawful.
    ZoneTransitionAmbiguitySemanticsValid;
    /// Aggregate proof that skipped local-time gap semantics are explicit and lawful.
    ZoneTransitionGapSemanticsValid;
    /// Aggregate proof that an RFC 3339 timestamp is structurally valid.
    Rfc3339TimestampValid;
    /// Aggregate proof that RFC 3339 lexical ordering preconditions are structurally valid.
    Rfc3339LexicalOrderingSemanticsValid;
    /// Aggregate proof that RFC 3339 generation guidance is structurally valid.
    Rfc3339GenerationGuidanceValid;
    /// Aggregate proof that RFC 3339 display-localization guidance is structurally valid.
    Rfc3339DisplayGuidanceValid;
    /// Aggregate proof that an RFC 9557 IXDTF timestamp is structurally valid.
    IxdtfTimestampValid;
    /// Aggregate proof that an IXDTF suffix-key registry entry carries the required field set.
    IxdtfSuffixKeyRegistryEntryValid;
    /// Aggregate proof that a permanent IXDTF suffix-key registration satisfies its reference semantics.
    IxdtfPermanentSuffixKeyRegistrationSemanticsValid;
    /// Aggregate proof that a provisional IXDTF suffix-key registration satisfies its reference semantics.
    IxdtfProvisionalSuffixKeyRegistrationSemanticsValid;
    /// Aggregate proof that IXDTF registry-policy semantics are structurally valid.
    IxdtfSuffixKeyRegistryPolicySemanticsValid;
    /// Aggregate proof that the initial `u-ca` registry entry semantics are structurally valid.
    IxdtfCalendarKeyRegistrySemanticsValid;
    /// Aggregate proof that an IXDTF timestamp declares a preferred presentation calendar.
    IxdtfTimestampHasPreferredPresentationCalendar;
    /// Aggregate proof that IXDTF additional-information semantics are structurally valid.
    IxdtfAdditionalInformationSemanticsValid;
    /// Aggregate proof that a named zone carries generic named-zone identity.
    NamedTimeZoneIdentityValid;
    /// Aggregate proof that a timestamp carries named-zone identity.
    ZonedDateTimeHasNamedZone;
    /// Aggregate proof that a critical time-zone suffix inconsistency is handled lawfully.
    CriticalTimeZoneInconsistencyHandlingValid;
    /// Aggregate proof that an elective time-zone suffix inconsistency is handled lawfully.
    ElectiveTimeZoneInconsistencyHandlingValid;
    /// Aggregate proof that a `Z`-based time-zone timestamp avoids offset inconsistency.
    ZuluTimeZoneInconsistencyAvoidanceValid;
    /// Aggregate proof that the timestamp offset agrees with the named-zone rules.
    OffsetConsistentWithNamedZone;
    /// Aggregate proof that offset-only semantics remain weaker than named-zone semantics.
    OffsetOnlyZoneSemanticsLimited;
    /// Aggregate proof that an offset time-zone annotation is consistent with the timestamp.
    OffsetTimeZoneAnnotationConsistentWithTimestamp;
    /// Aggregate proof that named-zone interpretation tracks current TZDB revision semantics.
    NamedTimeZoneInterpretationTracksTzdbRevision;
    /// Aggregate proof that subsecond precision was preserved.
    PrecisionPreserved;
    /// Aggregate proof that a conversion preserved ordering on the UTC timeline.
    TemporalOrderingPreserved;
    /// Aggregate proof that backend-conversion semantics are explicitly declared.
    BackendConversionSemanticsValid;
    /// Aggregate proof that lossy conversion authority is explicit and fully described.
    LossyConversionAuthorityValid;
    /// Aggregate proof that a conversion is lossless.
    ConversionLossless;
    /// Aggregate proof that a conversion truncates subseconds.
    ConversionTruncatesSubseconds;
    /// Aggregate proof that a duration representation is structurally valid.
    DurationFormValid;
    /// Aggregate proof that a duration representation family is explicit and lawful.
    DurationRepresentationSemanticsValid;
    /// Aggregate proof that a time interval representation is structurally valid.
    TimeIntervalValid;
    /// Aggregate proof that inherited end-component semantics are structurally valid.
    InheritedIntervalEndComponentsSemanticsValid;
    /// Aggregate proof that inherited trailing-zone semantics are structurally valid.
    InheritedIntervalZoneSemanticsValid;
    /// Aggregate proof that interval endpoints are chronologically ordered.
    IntervalEndpointsOrdered;
    /// Aggregate proof that complete-interval substitution semantics are structurally valid.
    CompleteIntervalSubstitutionSemanticsValid;
    /// Aggregate proof that a recurring interval representation is structurally valid.
    RecurringIntervalFormValid;
    /// Aggregate proof that complete recurring-interval representation semantics are established.
    CompleteRecurringIntervalRepresentationSemanticsValid;
    /// Aggregate proof that other-than-complete recurring-interval semantics are established.
    OtherThanCompleteRecurringIntervalRepresentationSemanticsValid;
    /// Aggregate proof that an extended temporal qualification is structurally valid.
    QualifiedTemporalExpressionValid;
    /// Aggregate proof that a temporal value and its qualification sidecar form a lawful exchange.
    QualifiedTemporalValueValid;
    /// Aggregate proof that a CalConnect explicit form is structurally valid.
    ExplicitTemporalFormValid;
    /// Aggregate proof that a CalConnect explicit local-time-of-day form is structurally valid.
    ExplicitTimeOfDayValid;
    /// Aggregate proof that a CalConnect explicit time-shift form is structurally valid.
    ExplicitTimeShiftValid;
    /// Aggregate proof that a CalConnect explicit date-time form is structurally valid.
    ExplicitDateTimeValid;
    /// Aggregate proof that a CalConnect explicit date-time-with-shift form is structurally valid.
    ExplicitDateTimeWithShiftValid;
    /// Aggregate proof that a CalConnect explicit duration form is structurally valid.
    ExplicitDurationValid;
    /// Aggregate proof that a CalConnect explicit time-interval form is structurally valid.
    ExplicitTimeIntervalValid;
    /// Aggregate proof that explicit-interval duration-substitution semantics are structurally valid.
    ExplicitIntervalDurationSubstitutionSemanticsValid;
    /// Aggregate proof that explicit-interval trailing-end inheritance semantics are structurally valid.
    ExplicitIntervalEndComponentInheritanceSemanticsValid;
    /// Aggregate proof that explicit-interval leading-shift propagation semantics are structurally valid.
    ExplicitIntervalShiftPropagationSemanticsValid;
    /// Aggregate proof that extended interval boundary semantics are structurally valid.
    ExtendedIntervalBoundarySemanticsValid;
    /// Aggregate proof that Level 1 enhanced-interval semantics are structurally valid.
    EnhancedIntervalLevelOneSemanticsValid;
    /// Aggregate proof that Level 2 enhanced-interval semantics are structurally valid.
    EnhancedIntervalLevelTwoSemanticsValid;
    /// Aggregate proof that a grouped time scale unit expression is structurally valid.
    GroupedTimeScaleUnitValid;
    /// Aggregate proof that a date-time formula is structurally valid.
    DateTimeFormulaValid;
    /// Aggregate proof that date-time formula evaluation semantics are fully declared.
    DateTimeFormulaEvaluationSemanticsValid;
    /// Aggregate proof that an explicit temporal result was lawfully produced by date-time formula evaluation.
    DateTimeFormulaEvaluationResultValid;
    /// Aggregate proof that a temporal set expression is structurally valid.
    TemporalSetExpressionValid;
    /// Aggregate proof that a seasonal temporal expression is structurally valid.
    SeasonalTemporalExpressionValid;
    /// Aggregate proof that a Level 2 sub-year grouping expression is structurally valid.
    SubYearGroupingExpressionValid;
    /// Aggregate proof that masked precision and unspecified-component semantics are structurally valid.
    UnspecifiedComponentExpressionValid;
    /// Aggregate proof that refined temporal-set range semantics are structurally valid.
    TemporalSetRangeSemanticsValid;
    /// Aggregate proof that a selection expression is structurally valid.
    SelectionExpressionValid;
    /// Aggregate proof that a repeat rule is structurally valid.
    RepeatRuleValid;
    /// Aggregate proof that a recurring interval with repeat-rule refinement is structurally valid.
    RecurringIntervalWithRepeatRuleValid;
}
