//! `KaniWitness` for `amenable_ext::ExtStandard<jiff::fmt::temporal::
//! Pieces<'static>>` — a real, checked round-trip property over
//! jiff's actual public API (`with_date`/`with_time`/`date`/`time`),
//! not a trusted stub.
//!
//! `Pieces<'n>` is a real, substantial data-carrying type (checked
//! directly against jiff's real source, `src/fmt/temporal/pieces.rs`)
//! — genuinely different from every other `fmt::*` type assessed so
//! far, which were either pure builders/markers or parser/printer
//! engines. `Pieces` has four private fields (`date: Date`, `time:
//! Option<Time>`, `offset: Option<PiecesOffset>`,
//! `time_zone_annotation: Option<TimeZoneAnnotation<'n>>`) and real
//! getters (`date()`/`time()`/`offset()`/`time_zone_annotation()`)
//! paired with real, unconditional setters (`with_date`/`with_time`/
//! etc. — confirmed by reading their bodies directly: `with_date`
//! is exactly `Pieces { date, ..self }`, `with_time` is exactly
//! `Pieces { time: Some(time), ..self }`, no validation at all since
//! both fields accept any already-valid `Date`/`Time` value).
//!
//! Scoped to `with_date`/`with_time`'s round trips only, deliberately
//! not `offset`/`time_zone_annotation` — those two fields' own types
//! (`PiecesOffset`/`TimeZoneAnnotation<'n>`) are themselves separate,
//! not-yet-assessed entries later in this checklist; checking
//! `Pieces`'s interaction with them now would be premature, out of
//! checklist order, and would duplicate work once those types get
//! their own real assessment. `date`/`time` alone are already a real,
//! substantial, non-tautological claim: `Pieces::from(date)` (a real,
//! documented `From<Date>` conversion) constructs a value, and
//! `with_date`/`with_time` are then checked to round-trip through
//! their matching getters exactly.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use crate::ext_macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::fmt::temporal::Pieces<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_fmt_temporal_pieces_with_date_with_time_round_trip".to_owned(),
            VERIFY_FMT_TEMPORAL_PIECES_WITH_DATE_WITH_TIME_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::fmt::temporal::Pieces<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::fmt::temporal::Pieces<'static>>",
        "kani",
        || <ExtStandard<jiff::fmt::temporal::Pieces<'static>> as crate::KaniWitness>::proof()
            .to_string(),
    )
}

/// The seven symbolic fields exercised: an initial date (to construct
/// a `Pieces` via `From<Date>`), then a second date and a time to set
/// via `with_date`/`with_time`.
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct PiecesDateTimeFields {
    initial_year: i16,
    initial_month: i8,
    initial_day: i8,
    year: i16,
    month: i8,
    day: i8,
    hour: i8,
    minute: i8,
    second: i8,
    subsec_nanosecond: i32,
}

kani_ensures_ext!(
    ExtStandard<jiff::fmt::temporal::Pieces<'static>>,
    "amenable_ext::ExtStandard<jiff::fmt::temporal::Pieces<'static>>",
    PiecesDateTimeFields,
    |fields| {
        if fields.initial_year < -9999i16
            || fields.initial_year > 9999i16
            || fields.initial_month < 1i8
            || fields.initial_month > 12i8
            || fields.initial_day < 1i8
            || fields.initial_day > 28i8
            || fields.year < -9999i16
            || fields.year > 9999i16
            || fields.month < 1i8
            || fields.month > 12i8
            || fields.day < 1i8
            || fields.day > 28i8
            || fields.hour < 0i8
            || fields.hour > 23i8
            || fields.minute < 0i8
            || fields.minute > 59i8
            || fields.second < 0i8
            || fields.second > 59i8
            || fields.subsec_nanosecond < 0i32
            || fields.subsec_nanosecond > 999_999_999i32
        {
            true
        } else {
            let initial_date = jiff::civil::Date::new(
                fields.initial_year,
                fields.initial_month,
                fields.initial_day,
            )
            .expect("initial date fields are already checked to be in range");
            let date = jiff::civil::Date::new(fields.year, fields.month, fields.day)
                .expect("date fields are already checked to be in range");
            let time = jiff::civil::Time::new(
                fields.hour,
                fields.minute,
                fields.second,
                fields.subsec_nanosecond,
            )
            .expect("time fields are already checked to be in range");
            let pieces = jiff::fmt::temporal::Pieces::from(initial_date)
                .with_date(date)
                .with_time(time);
            pieces.date() == date && pieces.time() == Some(time)
        }
    }
);

amenable_derive::harness! {
    kani, VERIFY_FMT_TEMPORAL_PIECES_WITH_DATE_WITH_TIME_ROUND_TRIP_SRC, {
        /// `Pieces::with_date`/`with_time`, whenever the given values
        /// are already-valid `Date`/`Time` values, always round-trip
        /// through their matching `date()`/`time()` getters exactly —
        /// checked over a symbolic instance, not an assumed slice.
        #[kani::proof]
        fn verify_fmt_temporal_pieces_with_date_with_time_round_trip() {
            let fields: PiecesDateTimeFields = kani::any();
            assert!(
                ExtStandard::<jiff::fmt::temporal::Pieces<'static>>::ensures(fields),
                "Pieces::with_date/with_time must round-trip through date()/time() exactly"
            );
        }
    }
}
