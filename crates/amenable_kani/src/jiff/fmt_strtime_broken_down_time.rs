//! `KaniWitness` for `amenable_ext::ExtStandard<jiff::fmt::strtime::
//! BrokenDownTime>` — a real, checked round-trip property over jiff's
//! actual public API, not a trusted stub.
//!
//! `BrokenDownTime` is a much larger type than most of this checklist
//! (checked directly against jiff's real source,
//! `src/fmt/strtime/mod.rs`): 12 numeric fields, each with a real,
//! independently-validated setter/getter pair (`set_year`/`year`,
//! `set_month`/`month`, etc. — `Result`-returning setters, unlike
//! `Span`'s own plain builder setters). Confirmed by reading
//! `set_day`'s own doc comment directly (not assumed from `civil::
//! Date`'s resemblance): "setting a day to a value that is legal in
//! any context is always valid, even if it isn't valid for the year,
//! month" — every field here validates against its own UNCONDITIONAL
//! range, with zero cross-field interdependency, the same
//! `civil::Time`-style "fully rectangular" shape, not `civil::Date`'s
//! month-dependent day range.
//!
//! Real bounds, confirmed against `jiff-core`'s own `bounds.rs` (the
//! single source of truth every `jiff::util::b` boundary type mirrors,
//! not inferred from doc-comment examples alone): year/iso_week_year
//! `-9999..=9999`, month `1..=12`, day `1..=31`, day_of_year
//! `1..=366`, iso_week `1..=53`, sunday/monday-based week `0..=53`,
//! hour `0..=23`, minute `0..=59`, second `0..=59` (genuinely
//! DIFFERENT from `civil::Time`'s own `0..=60` leap-second allowance —
//! confirmed by reading the real bound, not assumed identical),
//! subsec_nanosecond `0..=999_999_999`.
//!
//! Scoped to exactly these 12 numeric round trips, deliberately not
//! the 5 reference-type fields (`offset`/`weekday`/`meridiem`/
//! `timestamp`/`iana_time_zone`) — their setters are plain,
//! unconditional field assignments with no `Result`/validation logic
//! at all (confirmed by reading each signature: `fn set_offset(&mut
//! self, offset: Option<Offset>)`, no `Result`), so the real
//! validated behavior this type provides is exhaustively covered by
//! the 12 numeric fields alone.
//!
//! **A real, confirmed CBMC wall, a new manifestation of the already-
//! documented `jiff::Error` Drop-glue cost.** A first harness using
//! `!(range).contains(&x) || { ...; setter(x).is_ok() && ... }` (the
//! exact shape `civil_date.rs`'s own harness already uses
//! successfully) timed out even for `year` ALONE. Isolated via the
//! standard build-up technique: `BrokenDownTime::default()` alone
//! (immediately `mem::forget`-ed) and calling only the read-only
//! getter both verify in under a second; calling the SETTER is what
//! reintroduces the timeout, confirmed via a probe showing CBMC
//! unwinding `drop_in_place::<[jiff_core::util::SmallStr<6>]>` — a
//! DIFFERENT internal representation of jiff's error-construction cost
//! than the recursive `Arc<ErrorInner>` chain documented elsewhere in
//! this crate's `gallery::jiff_error_drop_cost`, but the same
//! underlying class of cost (every `Result<_, jiff::Error>`-returning
//! call pays it, regardless of which internal representation the
//! `Error` value happens to use). Fixed by combining three changes
//! together (confirmed necessary as a set, not attributed to just
//! one): (1) a real `if in_range { .. }` branch rather than a `||`
//! short-circuit expression, (2) `.expect(..)` on the already-range-
//! checked call rather than `.is_ok()`, and (3) wrapping the
//! `BrokenDownTime` in `ManuallyDrop` afterward rather than letting it
//! drop normally at scope end — matching this crate's own established
//! "gate the fallible call behind an independent bounds check so
//! `.expect()` never actually reaches `Err`" technique, just needing
//! the extra drop-suppression this time since the containing struct's
//! own Drop glue (triggered by `set_X`'s `&mut self` receiver,
//! confirmed via a probe that the read-only getter alone does NOT
//! trigger) pays the same cost independently of the `Result` value
//! itself. `ManuallyDrop::new`, not `mem::forget` — a real, separate
//! clippy lint (`clippy::mem_forget`, "usage of `mem::forget` on type
//! with `Drop` fields") flags the latter outright, confirmed via a
//! real `just check-all-package` failure.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use crate::ext_macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::fmt::strtime::BrokenDownTime> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_fmt_strtime_broken_down_time_numeric_setters_round_trip".to_owned(),
            VERIFY_FMT_STRTIME_BROKEN_DOWN_TIME_NUMERIC_SETTERS_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::fmt::strtime::BrokenDownTime>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::fmt::strtime::BrokenDownTime>",
        "kani",
        || <ExtStandard<jiff::fmt::strtime::BrokenDownTime> as crate::KaniWitness>::proof()
            .to_string(),
    )
}

/// The 12 numeric fields checked, in `BrokenDownTime`'s own
/// declaration order.
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct BrokenDownTimeNumericFields {
    year: i16,
    month: i8,
    day: i8,
    day_of_year: i16,
    iso_week_year: i16,
    iso_week: i8,
    week_sun: i8,
    week_mon: i8,
    hour: i8,
    minute: i8,
    second: i8,
    subsec_nanosecond: i32,
}

kani_ensures_ext!(
    ExtStandard<jiff::fmt::strtime::BrokenDownTime>,
    "amenable_ext::ExtStandard<jiff::fmt::strtime::BrokenDownTime>",
    BrokenDownTimeNumericFields,
    |fields| {
        let year_ok = if (-9999i16..=9999i16).contains(&fields.year) {
            let mut tm = jiff::fmt::strtime::BrokenDownTime::default();
            tm.set_year(Some(fields.year))
                .expect("year already checked to be in range");
            let matches = tm.year() == Some(fields.year);
            let _ = std::mem::ManuallyDrop::new(tm);
            matches
        } else {
            true
        };
        let month_ok = if (1i8..=12i8).contains(&fields.month) {
            let mut tm = jiff::fmt::strtime::BrokenDownTime::default();
            tm.set_month(Some(fields.month))
                .expect("month already checked to be in range");
            let matches = tm.month() == Some(fields.month);
            let _ = std::mem::ManuallyDrop::new(tm);
            matches
        } else {
            true
        };
        let day_ok = if (1i8..=31i8).contains(&fields.day) {
            let mut tm = jiff::fmt::strtime::BrokenDownTime::default();
            tm.set_day(Some(fields.day))
                .expect("day already checked to be in range");
            let matches = tm.day() == Some(fields.day);
            let _ = std::mem::ManuallyDrop::new(tm);
            matches
        } else {
            true
        };
        let day_of_year_ok = if (1i16..=366i16).contains(&fields.day_of_year) {
            let mut tm = jiff::fmt::strtime::BrokenDownTime::default();
            tm.set_day_of_year(Some(fields.day_of_year))
                .expect("day_of_year already checked to be in range");
            let matches = tm.day_of_year() == Some(fields.day_of_year);
            let _ = std::mem::ManuallyDrop::new(tm);
            matches
        } else {
            true
        };
        let iso_week_year_ok = if (-9999i16..=9999i16).contains(&fields.iso_week_year) {
            let mut tm = jiff::fmt::strtime::BrokenDownTime::default();
            tm.set_iso_week_year(Some(fields.iso_week_year))
                .expect("iso_week_year already checked to be in range");
            let matches = tm.iso_week_year() == Some(fields.iso_week_year);
            let _ = std::mem::ManuallyDrop::new(tm);
            matches
        } else {
            true
        };
        let iso_week_ok = if (1i8..=53i8).contains(&fields.iso_week) {
            let mut tm = jiff::fmt::strtime::BrokenDownTime::default();
            tm.set_iso_week(Some(fields.iso_week))
                .expect("iso_week already checked to be in range");
            let matches = tm.iso_week() == Some(fields.iso_week);
            let _ = std::mem::ManuallyDrop::new(tm);
            matches
        } else {
            true
        };
        let week_sun_ok = if (0i8..=53i8).contains(&fields.week_sun) {
            let mut tm = jiff::fmt::strtime::BrokenDownTime::default();
            tm.set_sunday_based_week(Some(fields.week_sun))
                .expect("week_sun already checked to be in range");
            let matches = tm.sunday_based_week() == Some(fields.week_sun);
            let _ = std::mem::ManuallyDrop::new(tm);
            matches
        } else {
            true
        };
        let week_mon_ok = if (0i8..=53i8).contains(&fields.week_mon) {
            let mut tm = jiff::fmt::strtime::BrokenDownTime::default();
            tm.set_monday_based_week(Some(fields.week_mon))
                .expect("week_mon already checked to be in range");
            let matches = tm.monday_based_week() == Some(fields.week_mon);
            let _ = std::mem::ManuallyDrop::new(tm);
            matches
        } else {
            true
        };
        let hour_ok = if (0i8..=23i8).contains(&fields.hour) {
            let mut tm = jiff::fmt::strtime::BrokenDownTime::default();
            tm.set_hour(Some(fields.hour))
                .expect("hour already checked to be in range");
            let matches = tm.hour() == Some(fields.hour);
            let _ = std::mem::ManuallyDrop::new(tm);
            matches
        } else {
            true
        };
        let minute_ok = if (0i8..=59i8).contains(&fields.minute) {
            let mut tm = jiff::fmt::strtime::BrokenDownTime::default();
            tm.set_minute(Some(fields.minute))
                .expect("minute already checked to be in range");
            let matches = tm.minute() == Some(fields.minute);
            let _ = std::mem::ManuallyDrop::new(tm);
            matches
        } else {
            true
        };
        let second_ok = if (0i8..=59i8).contains(&fields.second) {
            let mut tm = jiff::fmt::strtime::BrokenDownTime::default();
            tm.set_second(Some(fields.second))
                .expect("second already checked to be in range");
            let matches = tm.second() == Some(fields.second);
            let _ = std::mem::ManuallyDrop::new(tm);
            matches
        } else {
            true
        };
        let subsec_nanosecond_ok = if (0i32..=999_999_999i32).contains(&fields.subsec_nanosecond) {
            let mut tm = jiff::fmt::strtime::BrokenDownTime::default();
            tm.set_subsec_nanosecond(Some(fields.subsec_nanosecond))
                .expect("subsec_nanosecond already checked to be in range");
            let matches = tm.subsec_nanosecond() == Some(fields.subsec_nanosecond);
            let _ = std::mem::ManuallyDrop::new(tm);
            matches
        } else {
            true
        };
        year_ok
            && month_ok
            && day_ok
            && day_of_year_ok
            && iso_week_year_ok
            && iso_week_ok
            && week_sun_ok
            && week_mon_ok
            && hour_ok
            && minute_ok
            && second_ok
            && subsec_nanosecond_ok
    }
);

amenable_derive::harness! {
    kani, VERIFY_FMT_STRTIME_BROKEN_DOWN_TIME_NUMERIC_SETTERS_ROUND_TRIP_SRC, {
        /// Each of `BrokenDownTime`'s 12 numeric setters, whenever it
        /// succeeds on an in-range value, has its matching getter
        /// return exactly that value back — checked independently
        /// for all 12 fields on a single symbolic instance, not an
        /// assumed slice.
        #[kani::proof]
        fn verify_fmt_strtime_broken_down_time_numeric_setters_round_trip() {
            let fields: BrokenDownTimeNumericFields = kani::any();
            assert!(
                ExtStandard::<jiff::fmt::strtime::BrokenDownTime>::ensures(fields),
                "BrokenDownTime's numeric setters must round-trip through their matching getters when in range"
            );
        }
    }
}
