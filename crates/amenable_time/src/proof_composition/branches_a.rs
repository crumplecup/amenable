//! Shared `*Evidence` branch decompositions (C–D) reused across
//! several aggregates, plus the per-form bundles behind the two
//! multi-credential aggregates. Same `#[derive(Evidence, Witness)]`
//! shape as the [`super::composites_a`] family.

use crate::{
    CalendarDateValid, CombinedDateTimePermitsCalendarDateComponent,
    CombinedDateTimePermitsOrdinalDateComponent, CombinedDateTimePermitsWeekDateComponent,
    CompleteIntervalCalendarDateMayBeSubstitutedByOrdinalDate,
    CompleteIntervalCalendarDateMayBeSubstitutedByWeekDate,
    CompleteIntervalDurationMaySubstituteWeekForm,
    CompleteIntervalLocalTimeMayBeSubstitutedByUtcOfDay,
    DurationAlternativeFormCarriesCompleteCalendarAndClockComponents,
    DurationAlternativeFormRequiresPartnerAgreement,
    DurationAlternativeFormUsesDateAndTimeComponentSlots,
    DurationTimeComponentsFollowTimeDesignator, DurationWeekFormNotMixedWithCalendarOrClockUnits,
    DurationWeekFormUsesSingleWeekUnit, OrdinalDateValid, TimeIntervalValid, WeekDateValid,
};

/// Evidence branch for the complete date family carried by a combined date-time representation.
///
/// Normative source: ISO 8601-1:2019, 5.4.2 and 5.4.3.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum CombinedDateTimeDateEvidence {
    /// The combined representation uses the calendar-date family.
    Calendar {
        /// The calendar-date component is structurally valid.
        date: CalendarDateValid,
        /// Clause 4.3 permits the calendar-date family in combined date-time forms.
        family: CombinedDateTimePermitsCalendarDateComponent,
    },
    /// The combined representation uses the ordinal-date family.
    Ordinal {
        /// The ordinal-date component is structurally valid.
        date: OrdinalDateValid,
        /// Clause 4.3 permits the ordinal-date family in combined date-time forms.
        family: CombinedDateTimePermitsOrdinalDateComponent,
    },
    /// The combined representation uses the week-date family.
    Week {
        /// The week-date component is structurally valid.
        date: WeekDateValid,
        /// Clause 4.3 permits the week-date family in combined date-time forms.
        family: CombinedDateTimePermitsWeekDateComponent,
    },
}

impl core::default::Default for CombinedDateTimeDateEvidence {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Calendar {
            date: core::default::Default::default(),
            family: core::default::Default::default(),
        }
    }
}

/// Evidence branch for the complete date family carried by an explicit date-with-shift value.
///
/// Normative source: CalConnect CC 18011:2018 §4.3 — Date
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum CompleteDateEvidence {
    /// The complete date uses the calendar-date family.
    Calendar(CalendarDateValid),
    /// The complete date uses the ordinal-date family.
    Ordinal(OrdinalDateValid),
    /// The complete date uses the week-date family.
    Week(WeekDateValid),
}

impl core::default::Default for CompleteDateEvidence {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Calendar(core::default::Default::default())
    }
}

/// Evidence bundle for complete duration/end interval substitution semantics.
///
/// Normative source: ISO 8601-1:2019, 4.4.4.4; 4.4.4.5
/// Open-text cross-check: ISO/WD 8601-1:2016(E), 4.4.4.4; 4.4.4.5
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
pub struct CompleteDurationEndIntervalSubstitutionEvidence {
    /// The underlying interval representation is structurally valid.
    interval: TimeIntervalValid,
    /// Complete representation family used by the duration component.
    duration: CompleteIntervalDurationRepresentationEvidence,
    /// Complete representation family used by the end time point.
    end: CompleteTimePointRepresentationEvidence,
}

/// Evidence branch for the duration representation family used inside a complete interval.
///
/// Normative source: ISO 8601-1:2019, 4.4.4.5
/// Open-text cross-check: ISO/WD 8601-1:2016(E), 4.4.4.5
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum CompleteIntervalDurationRepresentationEvidence {
    /// The duration uses the baseline calendar-and-clock designator form.
    CalendarClockDesignator {
        /// Time components, when present, are placed after the `T` designator.
        time_components: DurationTimeComponentsFollowTimeDesignator,
    },
    /// The duration uses the complete week-form substitution permitted for complete intervals.
    WeekSubstitution {
        /// The substitution from calendar-and-clock duration to week form is explicit and lawful.
        substitution: CompleteIntervalDurationMaySubstituteWeekForm,
        /// The substituted duration obeys the week-form duration laws.
        week_form: DurationWeekFormEvidence,
    },
    /// The duration uses the alternative complete representation.
    Alternative(DurationAlternativeFormEvidence),
}

impl core::default::Default for CompleteIntervalDurationRepresentationEvidence {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::CalendarClockDesignator {
            time_components: core::default::Default::default(),
        }
    }
}

/// Evidence bundle for complete start/duration interval substitution semantics.
///
/// Normative source: ISO 8601-1:2019, 4.4.4.3; 4.4.4.5
/// Open-text cross-check: ISO/WD 8601-1:2016(E), 4.4.4.3; 4.4.4.5
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
pub struct CompleteStartDurationIntervalSubstitutionEvidence {
    /// The underlying interval representation is structurally valid.
    interval: TimeIntervalValid,
    /// Complete representation family used by the start time point.
    start: CompleteTimePointRepresentationEvidence,
    /// Complete representation family used by the duration component.
    duration: CompleteIntervalDurationRepresentationEvidence,
}

/// Evidence bundle for complete start/end interval substitution semantics.
///
/// Normative source: ISO 8601-1:2019, 4.4.4.1; 4.4.4.5
/// Open-text cross-check: ISO/WD 8601-1:2016(E), 4.4.4.1; 4.4.4.5
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
pub struct CompleteStartEndIntervalSubstitutionEvidence {
    /// The underlying interval representation is structurally valid.
    interval: TimeIntervalValid,
    /// Complete representation family used by the start time point.
    start: CompleteTimePointRepresentationEvidence,
    /// Complete representation family used by the end time point.
    end: CompleteTimePointRepresentationEvidence,
}

/// Evidence branch for the date representation family used within a complete interval time point.
///
/// Normative source: ISO 8601-1:2019, 4.4.4.5
/// Open-text cross-check: ISO/WD 8601-1:2016(E), 4.4.4.5
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum CompleteTimePointDateRepresentationEvidence {
    /// The time point uses the baseline calendar-date component shown in the standard.
    Calendar,
    /// The calendar-date slot is lawfully substituted by a complete ordinal date.
    Ordinal {
        /// The substitution from calendar date to ordinal date is explicit and lawful.
        substitution: CompleteIntervalCalendarDateMayBeSubstitutedByOrdinalDate,
    },
    /// The calendar-date slot is lawfully substituted by a complete week date.
    Week {
        /// The substitution from calendar date to week date is explicit and lawful.
        substitution: CompleteIntervalCalendarDateMayBeSubstitutedByWeekDate,
    },
}

impl core::default::Default for CompleteTimePointDateRepresentationEvidence {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Calendar
    }
}

/// Evidence bundle for one complete interval time-point representation.
///
/// Normative source: ISO 8601-1:2019, 4.4.4.5
/// Open-text cross-check: ISO/WD 8601-1:2016(E), 4.4.4.5
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
pub struct CompleteTimePointRepresentationEvidence {
    /// The date representation family used by the time point.
    date: CompleteTimePointDateRepresentationEvidence,
    /// The time representation family used by the time point.
    time: CompleteTimePointTimeRepresentationEvidence,
}

/// Evidence branch for the time representation family used within a complete interval time point.
///
/// Normative source: ISO 8601-1:2019, 4.4.4.5
/// Open-text cross-check: ISO/WD 8601-1:2016(E), 4.4.4.5
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum CompleteTimePointTimeRepresentationEvidence {
    /// The time point uses the baseline local-time component shown in the standard.
    Local,
    /// The local-time slot is lawfully substituted by a UTC representation.
    Utc {
        /// The substitution from local time to UTC is explicit and lawful.
        substitution: CompleteIntervalLocalTimeMayBeSubstitutedByUtcOfDay,
    },
    /// The local-time slot is lawfully substituted by local time with a difference from UTC.
    LocalWithUtcDifference {},
}

impl core::default::Default for CompleteTimePointTimeRepresentationEvidence {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Local
    }
}

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
