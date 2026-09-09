//! CalConnect CC 18011:2018 explicit temporal forms (§4.3) and explicit
//! time intervals (§6).
//!
//! Tier B (`docs/AMENABLE_TIME_PLAN.md`): CalConnect specs are publicly
//! available but copyrighted, so the primary quotation is
//! `NormativeQuotation::ParaphraseOnly` for now — the `SemanticSummary`
//! paraphrase plus the exact clause pointer and the CalConnect CSD URL
//! carry the citation. Upgrading these to a verbatim single clause is a
//! follow-on.

use crate::NormativeQuotation;

temporal_standard! {
    /// Explicit forms use designator symbols between components.
    ExplicitTemporalFormUsesDesignatorSymbols => (
        document: "CalConnect CC 18011:2018", section: "4.3", body: CalConnect, status: Normative,
        summary: "an explicit temporal form delimits its time-scale components with designator symbols rather than fixed-width fields",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// Zero-valued components may be omitted.
    ExplicitTemporalFormMayOmitZeroValuedComponents => (
        document: "CalConnect CC 18011:2018", section: "4.3.6", body: CalConnect, status: Normative,
        summary: "an explicit form may drop a zero-valued component as long as the remaining representation is still valid",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// The lowest denoted component sets precision.
    ExplicitTemporalPrecisionUsesLowestDenotedComponent => (
        document: "CalConnect CC 18011:2018", section: "4.3.7", body: CalConnect, status: Normative,
        summary: "the lowest-order component actually written in an explicit form declares that form's precision",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// The UTC relationship is `Z` or a signed shift.
    ExplicitUtcRelationshipUsesZuluOrSignedShift => (
        document: "CalConnect CC 18011:2018", section: "4.3.5", body: CalConnect, status: Normative,
        summary: "an explicit form states its UTC relationship as either the Z designator or a signed explicit time shift",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// An explicit time of day leads with `T`.
    ExplicitTimeOfDayUsesTimeDesignator => (
        document: "CalConnect CC 18011:2018", section: "4.3.3", body: CalConnect, status: Normative,
        summary: "an explicit local time of day is introduced by the leading time designator T",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// An explicit time of day uses H/M/S unit designators.
    ExplicitTimeOfDayUsesHourMinuteSecondUnitDesignators => (
        document: "CalConnect CC 18011:2018", section: "4.3.3", body: CalConnect, status: Normative,
        summary: "an explicit local time of day writes its hour, minute, and second with unit designators",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// An explicit time of day has no end-of-day form.
    ExplicitTimeOfDayForbidsEndOfDayRepresentation => (
        document: "CalConnect CC 18011:2018", section: "4.3.3.2", body: CalConnect, status: Normative,
        summary: "the explicit local-time-of-day family has no 24:00 end-of-day representation",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// An explicit time shift leads with `Z`.
    ExplicitTimeShiftUsesZuluDesignator => (
        document: "CalConnect CC 18011:2018", section: "4.3.4", body: CalConnect, status: Normative,
        summary: "an explicit time shift is introduced by a leading Z designator",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// A leading minus means behind UTC.
    ExplicitTimeShiftUsesLeadingMinusOnlyWhenBehindUtc => (
        document: "CalConnect CC 18011:2018", section: "4.3.4", body: CalConnect, status: Normative,
        summary: "a leading minus sign appears on an explicit time shift only when local time is behind UTC",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// A non-empty time-shift payload is an explicit time of day.
    ExplicitTimeShiftPayloadUsesExplicitTimeOfDay => (
        document: "CalConnect CC 18011:2018", section: "4.3.4", body: CalConnect, status: Normative,
        summary: "when an explicit time shift carries a payload it is drawn from the explicit local-time-of-day family",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// A bare `Z` shift is UTC with zero offset.
    ExplicitTimeShiftBareZuluRepresentsUtcZero => (
        document: "CalConnect CC 18011:2018", section: "4.3.4", body: CalConnect, status: Normative,
        summary: "an explicit time shift consisting of Z alone denotes UTC with a zero shift",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// An explicit date-time is date then time.
    ExplicitDateTimeUsesDateThenTimeConcatenation => (
        document: "CalConnect CC 18011:2018", section: "4.3.4.3", body: CalConnect, status: Normative,
        summary: "an explicit date-time writes a complete explicit date immediately followed by an explicit time",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// An explicit date-time's time portion may be reduced precision.
    ExplicitDateTimeTimePortionMayBeReducedPrecision => (
        document: "CalConnect CC 18011:2018", section: "4.3.4.3", body: CalConnect, status: Normative,
        summary: "the time portion of an explicit date-time may be written at reduced precision",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// An explicit date-with-shift is date then shift.
    ExplicitDateWithShiftUsesDateThenShiftConcatenation => (
        document: "CalConnect CC 18011:2018", section: "4.3.4.1", body: CalConnect, status: Normative,
        summary: "an explicit date with shift writes a complete explicit date immediately followed by an explicit time shift",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// An explicit time-of-day-with-shift is time then shift.
    ExplicitTimeOfDayWithShiftUsesTimeThenShiftConcatenation => (
        document: "CalConnect CC 18011:2018", section: "4.3.4.2", body: CalConnect, status: Normative,
        summary: "an explicit time of day with shift writes an explicit time immediately followed by an explicit time shift",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// An explicit date-time-with-shift is date-time then shift.
    ExplicitDateTimeWithShiftUsesDateTimeThenShiftConcatenation => (
        document: "CalConnect CC 18011:2018", section: "4.3.4.3", body: CalConnect, status: Normative,
        summary: "an explicit date-time with shift writes an explicit date-time immediately followed by an explicit time shift",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// An explicit time interval uses the datetimeE endpoint family.
    ExplicitTimeIntervalUsesDateTimeEndpointFamily => (
        document: "CalConnect CC 18011:2018", section: "6", body: CalConnect, status: Normative,
        summary: "a complete explicit time interval uses CalConnect's [datetimeE]/[datetimeE] endpoint family",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// An explicit interval may substitute a duration for a boundary.
    ExplicitTimeIntervalDurationSubstitutionInfersMissingBoundary => (
        document: "CalConnect CC 18011:2018", section: "6", body: CalConnect, status: Normative,
        summary: "an explicit time interval may write an explicit duration in place of one boundary when the missing endpoint stays inferable",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// An explicit interval's trailing end may inherit higher-order components.
    ExplicitTimeIntervalTrailingEndMayInheritHigherOrderComponents => (
        document: "CalConnect CC 18011:2018", section: "6", body: CalConnect, status: Normative,
        summary: "an explicit time interval may omit higher-order trailing-end components when inheritance from the start is unambiguous",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// A leading explicit shift propagates across the separator.
    ExplicitTimeIntervalLeadingShiftAppliesToTrailingComponentUnlessOverridden => (
        document: "CalConnect CC 18011:2018", section: "6", body: CalConnect, status: Normative,
        summary: "a leading explicit time shift applies to the trailing interval component too, unless that component supplies its own",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
}
