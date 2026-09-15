//! `KaniWitness` for `amenable_ext::ExtStandard<jiff::Span>` — a real,
//! checked round-trip property over jiff's actual public API (each
//! unit setter paired with its own getter), not a trusted stub.
//!
//! `Span` stores ten independent signed unit quantities (years/months/
//! weeks/days/hours/minutes/seconds/milliseconds/microseconds/
//! nanoseconds) plus one shared `sign` covering the whole value —
//! confirmed by reading jiff's real source (`src/span.rs`): each setter
//! (e.g. `years`) stores the unsigned magnitude and recomputes the
//! shared sign via a private `resign` step, and each getter (e.g.
//! `get_years`) returns `sign * magnitude`. Calling exactly one setter
//! on a fresh `Span::new()` — the shape this harness checks — never
//! exercises `resign`'s "combine with an already-nonzero span" case, so
//! the getter reproduces the setter's input exactly for any value
//! jiff's own documented range accepts (`Span::years`/`months`/etc.'s
//! own `# Panics` sections; the panicking setters are used directly,
//! not `try_*`, since each is only ever called from inside its own
//! range guard — mirroring `offset.rs`'s pattern of never handing Kani
//! a value that could reach jiff's `Err`/panic path).
//!
//! One property, ten independent checks: rather than ten separate
//! `ExtStandard<jiff::Span>` proofs (there is exactly one carrier, not
//! ten), this harness takes all ten fields as one symbolic input and
//! asserts each field's own round trip on its own freshly-constructed
//! `Span::new()`, matching the one-`ensures`-per-type convention used
//! everywhere else in this family.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use super::super::macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::Span> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_span_unit_setters_round_trip".to_owned(),
            VERIFY_SPAN_UNIT_SETTERS_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::Span>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::Span>",
        "kani",
        || <ExtStandard<jiff::Span> as crate::KaniWitness>::proof().to_string(),
    )
}

/// jiff's own documented valid ranges for each `Span` unit setter (each
/// setter's own `# Panics` section), checked independently of the
/// setters themselves so no symbolic value ever reaches the panicking
/// path.
const SPAN_YEARS_MIN: i16 = -19_998;
const SPAN_YEARS_MAX: i16 = 19_998;
const SPAN_MONTHS_MIN: i32 = -239_976;
const SPAN_MONTHS_MAX: i32 = 239_976;
const SPAN_WEEKS_MIN: i32 = -1_043_497;
const SPAN_WEEKS_MAX: i32 = 1_043_497;
const SPAN_DAYS_MIN: i32 = -7_304_484;
const SPAN_DAYS_MAX: i32 = 7_304_484;
const SPAN_HOURS_MIN: i32 = -175_307_616;
const SPAN_HOURS_MAX: i32 = 175_307_616;
const SPAN_MINUTES_MIN: i64 = -10_518_456_960;
const SPAN_MINUTES_MAX: i64 = 10_518_456_960;
const SPAN_SECONDS_MIN: i64 = -631_107_417_600;
const SPAN_SECONDS_MAX: i64 = 631_107_417_600;
const SPAN_MILLISECONDS_MIN: i64 = -631_107_417_600_000;
const SPAN_MILLISECONDS_MAX: i64 = 631_107_417_600_000;
const SPAN_MICROSECONDS_MIN: i64 = -631_107_417_600_000_000;
const SPAN_MICROSECONDS_MAX: i64 = 631_107_417_600_000_000;
const SPAN_NANOSECONDS_MIN: i64 = -9_223_372_036_854_775_807;
const SPAN_NANOSECONDS_MAX: i64 = 9_223_372_036_854_775_807;

/// One symbolic tuple per `Span` unit field mirrors the type's own ten
/// independent setter/getter pairs.
type SpanUnitFields = (i16, i32, i32, i32, i32, i64, i64, i64, i64, i64);

kani_ensures_ext!(
    ExtStandard<jiff::Span>,
    "amenable_ext::ExtStandard<jiff::Span>",
    SpanUnitFields,
    |(
        years,
        months,
        weeks,
        days,
        hours,
        minutes,
        seconds,
        milliseconds,
        microseconds,
        nanoseconds,
    )| {
        let years_ok = !(SPAN_YEARS_MIN..=SPAN_YEARS_MAX).contains(&years)
            || jiff::Span::new().years(years).get_years() == years;
        let months_ok = !(SPAN_MONTHS_MIN..=SPAN_MONTHS_MAX).contains(&months)
            || jiff::Span::new().months(months).get_months() == months;
        let weeks_ok = !(SPAN_WEEKS_MIN..=SPAN_WEEKS_MAX).contains(&weeks)
            || jiff::Span::new().weeks(weeks).get_weeks() == weeks;
        let days_ok = !(SPAN_DAYS_MIN..=SPAN_DAYS_MAX).contains(&days)
            || jiff::Span::new().days(days).get_days() == days;
        let hours_ok = !(SPAN_HOURS_MIN..=SPAN_HOURS_MAX).contains(&hours)
            || jiff::Span::new().hours(hours).get_hours() == hours;
        let minutes_ok = !(SPAN_MINUTES_MIN..=SPAN_MINUTES_MAX).contains(&minutes)
            || jiff::Span::new().minutes(minutes).get_minutes() == minutes;
        let seconds_ok = !(SPAN_SECONDS_MIN..=SPAN_SECONDS_MAX).contains(&seconds)
            || jiff::Span::new().seconds(seconds).get_seconds() == seconds;
        let milliseconds_ok = !(SPAN_MILLISECONDS_MIN..=SPAN_MILLISECONDS_MAX)
            .contains(&milliseconds)
            || jiff::Span::new()
                .milliseconds(milliseconds)
                .get_milliseconds()
                == milliseconds;
        let microseconds_ok = !(SPAN_MICROSECONDS_MIN..=SPAN_MICROSECONDS_MAX)
            .contains(&microseconds)
            || jiff::Span::new()
                .microseconds(microseconds)
                .get_microseconds()
                == microseconds;
        let nanoseconds_ok = !(SPAN_NANOSECONDS_MIN..=SPAN_NANOSECONDS_MAX).contains(&nanoseconds)
            || jiff::Span::new().nanoseconds(nanoseconds).get_nanoseconds() == nanoseconds;
        years_ok
            && months_ok
            && weeks_ok
            && days_ok
            && hours_ok
            && minutes_ok
            && seconds_ok
            && milliseconds_ok
            && microseconds_ok
            && nanoseconds_ok
    }
);

amenable_derive::harness! {
    kani, VERIFY_SPAN_UNIT_SETTERS_ROUND_TRIP_SRC, {
        /// Setting exactly one unit on a fresh `Span::new()` and reading
        /// it back through that unit's own getter reproduces the
        /// original value exactly, for every one of `Span`'s ten unit
        /// fields independently, whenever the value is within jiff's own
        /// documented range for that setter (out of range panics, so is
        /// excluded rather than assumed away). Checked for every value
        /// in each field's own integer width, not an assumed slice.
        #[kani::proof]
        fn verify_span_unit_setters_round_trip() {
            let years: i16 = kani::any();
            let months: i32 = kani::any();
            let weeks: i32 = kani::any();
            let days: i32 = kani::any();
            let hours: i32 = kani::any();
            let minutes: i64 = kani::any();
            let seconds: i64 = kani::any();
            let milliseconds: i64 = kani::any();
            let microseconds: i64 = kani::any();
            let nanoseconds: i64 = kani::any();
            assert!(
                ExtStandard::<jiff::Span>::ensures((
                    years,
                    months,
                    weeks,
                    days,
                    hours,
                    minutes,
                    seconds,
                    milliseconds,
                    microseconds,
                    nanoseconds,
                )),
                "each Span unit setter, paired with its own getter on a fresh Span::new(), must \
                 round-trip exactly whenever the value is within jiff's documented range"
            );
        }
    }
}
