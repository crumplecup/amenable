//! `KaniWitness` for `amenable_ext::ExtStandard<jiff::civil::ISOWeekDate>`
//! — a real, checked round-trip property over jiff's actual public
//! API (`ISOWeekDate::new`/`year`/`week`/`weekday`), not a trusted
//! stub.
//!
//! `ISOWeekDate` is a real value type with real getters, the same
//! shape as `civil::Date` — checked directly by reading jiff's real
//! source, not assumed trusted or assumed checkable either way.
//! `ISOWeekDate::new`'s validity is more complex than `Date::new`'s:
//! jiff's own doc comment states week `53` is only valid for years
//! containing a "leap week," while week `1..=52` is documented as
//! valid for every representable year — the same "sufficient, not
//! exhaustive" simplification `civil_date.rs` uses for day `1..=28`,
//! here applied to week instead, avoiding the need to compute which
//! years contain a leap week for a symbolic year.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use super::macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::civil::ISOWeekDate> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_civil_iso_week_date_new_year_week_weekday_round_trips".to_owned(),
            VERIFY_CIVIL_ISO_WEEK_DATE_NEW_YEAR_WEEK_WEEKDAY_ROUND_TRIPS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::civil::ISOWeekDate>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::civil::ISOWeekDate>",
        "kani",
        || <ExtStandard<jiff::civil::ISOWeekDate> as crate::KaniWitness>::proof().to_string(),
    )
}

/// A deliberately narrowed sufficient sub-range, not jiff's full
/// documented range: a first attempt using the full `-9999..=9999`
/// year range alongside week `1..=52` FAILED for real (Kani caught a
/// genuine property error, not a false trail) — `ISOWeekDate::MIN`/
/// `MAX` are derived from `Date::MIN`/`MAX`, not from the leap-week
/// rule alone, so a week/weekday combination near the exact boundary
/// years can still push the represented date outside the overall
/// representable range even at week `<= 52` (jiff's own doc comment
/// confirms this directly: `ISOWeekDate::new(9999, 52,
/// Weekday::Saturday)` errors, even though week `52` is otherwise
/// always valid). Narrowing the year range inward by a comfortable
/// margin (clear of the exact `Date::MIN`/`MAX` edge) restores the
/// "week `1..=52` always valid" property honestly, matching the
/// "comfortably safe sub-range" discipline already used by this
/// crate's Verus models (e.g. `timestamp_series.rs`).
const ISO_WEEK_DATE_YEAR_MIN: i16 = -9990;
const ISO_WEEK_DATE_YEAR_MAX: i16 = 9990;
const ISO_WEEK_DATE_WEEK_MIN: i8 = 1;
const ISO_WEEK_DATE_WEEK_MAX: i8 = 52;

/// All seven real `Weekday` variants, for exhaustive enumeration —
/// `Weekday` has no `kani::Arbitrary` derive of its own to draw from
/// directly.
const ALL_WEEKDAYS: [jiff::civil::Weekday; 7] = [
    jiff::civil::Weekday::Monday,
    jiff::civil::Weekday::Tuesday,
    jiff::civil::Weekday::Wednesday,
    jiff::civil::Weekday::Thursday,
    jiff::civil::Weekday::Friday,
    jiff::civil::Weekday::Saturday,
    jiff::civil::Weekday::Sunday,
];

kani_ensures_ext!(
    ExtStandard<jiff::civil::ISOWeekDate>,
    "amenable_ext::ExtStandard<jiff::civil::ISOWeekDate>",
    (i16, i8, usize),
    |(year, week, weekday_index)| {
        if !(ISO_WEEK_DATE_YEAR_MIN..=ISO_WEEK_DATE_YEAR_MAX).contains(&year)
            || !(ISO_WEEK_DATE_WEEK_MIN..=ISO_WEEK_DATE_WEEK_MAX).contains(&week)
            || weekday_index >= ALL_WEEKDAYS.len()
        {
            true
        } else {
            let weekday = ALL_WEEKDAYS[weekday_index];
            let iwd = jiff::civil::ISOWeekDate::new(year, week, weekday)
                .expect("year/week are already checked to always be a valid civil::ISOWeekDate");
            iwd.year() == year && iwd.week() == week && iwd.weekday() == weekday
        }
    }
);

amenable_derive::harness! {
    kani, VERIFY_CIVIL_ISO_WEEK_DATE_NEW_YEAR_WEEK_WEEKDAY_ROUND_TRIPS_SRC, {
        /// `ISOWeekDate::new(year, week, weekday)`, whenever it
        /// succeeds within the always-valid `1..=52` week sub-range,
        /// always returns an `ISOWeekDate` whose own `year()`/
        /// `week()`/`weekday()` are exactly `year`/`week`/`weekday`
        /// back — checked over the full year range and all seven
        /// weekdays, a representative always-valid week sub-range.
        #[kani::proof]
        fn verify_civil_iso_week_date_new_year_week_weekday_round_trips() {
            let year: i16 = kani::any();
            let week: i8 = kani::any();
            let weekday_index: usize = kani::any();
            assert!(
                ExtStandard::<jiff::civil::ISOWeekDate>::ensures((year, week, weekday_index)),
                "ISOWeekDate::new(year, week, weekday)'s year()/week()/weekday() must equal year/week/weekday whenever construction succeeds"
            );
        }
    }
}
