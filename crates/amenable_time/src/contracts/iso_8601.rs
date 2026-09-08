//! ISO 8601 temporal propositions — the largest contract module (109 in
//! `elicit_temporal`).
//!
//! Sources: ISO 8601-1:2019 (+ Amd 1:2022), ISO 8601-2:2019, and the
//! publicly-circulated working draft ISO/WD 8601-1:2016(E). All three are
//! tier C (`docs/AMENABLE_TIME_PLAN.md`): paywalled or draft, so
//! `NormativeQuotation::ParaphraseOnly` — no ISO prose is reproduced.
//! Contracts citing the published standard carry `status: Normative`;
//! those citing only the working draft carry `status: OpenTextCrossCheck`.
//!
//! Ported group by group (`docs/AMENABLE_TIME_COVERAGE.md`). This file
//! grows as the groups land.

use crate::NormativeQuotation;

temporal_standard! {
    /// The time axis is the single axis along which instantaneous events
    /// are ordered by when they occur.
    TimeAxisOrdersTimePointsByTemporalPosition => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.4",
        body: Iso,
        status: Normative,
        summary: "the time axis is one axis ordering instantaneous events by their succession in time",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// A time scale attaches an ordered set of marks to instants on the
    /// time axis, giving each a measured position.
    TimeScaleAssociatesTimePointsWithOrderedMeasure => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.5",
        body: Iso,
        status: Normative,
        summary: "a time scale is an ordered system of marks attributed to instants on the time axis",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// A time value is a mark placed on a named time scale, identifying
    /// one instant or interval on it.
    TimeIsMarkOnSpecifiedTimeScale => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.2",
        body: Iso,
        status: Normative,
        summary: "a time is a mark, on a specified time scale, attributed to an instant or a time interval",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// An instant is a single point on the time axis — it has position
    /// but no extent.
    InstantIsPointOnTimeAxis => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.3",
        body: Iso,
        status: Normative,
        summary: "an instant is a point on the time axis",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// A date names a position on the calendar time scale.
    DateIdentifiesPositionWithinCalendar => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.1",
        body: Iso,
        status: Normative,
        summary: "a date is a time on the calendar time scale — a position within the calendar",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// A time of day is a time that falls within one calendar day.
    TimeOfDayOccursWithinCalendarDay => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.16",
        body: Iso,
        status: Normative,
        summary: "a time of day is a time occurring within a calendar day",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );
}
