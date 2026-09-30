//! `KaniWitness` for `amenable_ext::ExtStandard<jiff::civil::Date>` — a
//! real, checked round-trip property over jiff's actual public API
//! (`Date::new`/`Date::year`/`Date::month`/`Date::day`).
//!
//! Unlike `Timestamp`/`Zoned` (which route through `TimeZone`'s
//! hand-rolled pointer-tagged `Repr` — confirmed to time out CBMC even
//! for a single, fully concrete call, see `gallery::
//! jiff_error_drop_cost`'s own doc comment), `civil::Date` is a pure
//! calendar value with no time zone involved at all (its private
//! `inner: JDate` field is a plain `jiff-core` proleptic day-count
//! representation, no pointer tagging) — checked directly by reading
//! jiff's real source, not assumed safe by resemblance. `Date::new`
//! does still route through the same `jiff::Error` recursive-Arc
//! Drop-glue wall on its `Err` arm (confirmed the same class of wall as
//! `Offset::from_seconds`'s own), fixed the identical way: an
//! independent, Drop-free bounds check gates the call so `.expect()`
//! never actually reaches `Err`.
//!
//! The gate here is a real, but intentionally narrowed, sufficient
//! condition, not jiff's full validity rule: day `1..=28` is valid for
//! *every* month in *every* year (even February in a non-leap year),
//! so restricting the day this way avoids needing to compute
//! days-in-month for a symbolic year/month combination while still
//! checking the full `year`/`month` range and a representative slice
//! of `day`. Property is vacuous outside that box, exactly like
//! `Offset::from_seconds`'s own out-of-range vacuous case.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use super::macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::civil::Date> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_civil_date_new_year_month_day_round_trips".to_owned(),
            VERIFY_CIVIL_DATE_NEW_YEAR_MONTH_DAY_ROUND_TRIPS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::civil::Date>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::civil::Date>",
        "kani",
        || <ExtStandard<jiff::civil::Date> as crate::KaniWitness>::proof().to_string(),
    )
}

/// jiff's own documented valid range for `civil::Date::new`'s year and
/// month (`-9999..=9999`, `1..=12`), and a deliberately narrowed
/// sufficient sub-range for day (`1..=28`, always valid regardless of
/// month/year — see this module's own doc comment).
const DATE_YEAR_MIN: i16 = -9999;
const DATE_YEAR_MAX: i16 = 9999;
const DATE_MONTH_MIN: i8 = 1;
const DATE_MONTH_MAX: i8 = 12;
const DATE_DAY_MIN: i8 = 1;
const DATE_DAY_MAX: i8 = 28;

kani_ensures_ext!(
    ExtStandard<jiff::civil::Date>,
    "amenable_ext::ExtStandard<jiff::civil::Date>",
    (i16, i8, i8),
    |(year, month, day)| {
        if !(DATE_YEAR_MIN..=DATE_YEAR_MAX).contains(&year)
            || !(DATE_MONTH_MIN..=DATE_MONTH_MAX).contains(&month)
            || !(DATE_DAY_MIN..=DATE_DAY_MAX).contains(&day)
        {
            true
        } else {
            let d = jiff::civil::Date::new(year, month, day)
                .expect("year/month/day are already checked to always be a valid civil::Date");
            d.year() == year && d.month() == month && d.day() == day
        }
    }
);

amenable_derive::harness! {
    kani, VERIFY_CIVIL_DATE_NEW_YEAR_MONTH_DAY_ROUND_TRIPS_SRC, {
        /// `Date::new(year, month, day)`, whenever it succeeds, always
        /// returns a `Date` whose own `year()`/`month()`/`day()` are
        /// exactly `year`/`month`/`day` back — checked over the full
        /// year/month range and a representative, always-valid day
        /// sub-range (`1..=28`); vacuous outside that box.
        #[kani::proof]
        fn verify_civil_date_new_year_month_day_round_trips() {
            let year: i16 = kani::any();
            let month: i8 = kani::any();
            let day: i8 = kani::any();
            assert!(
                ExtStandard::<jiff::civil::Date>::ensures((year, month, day)),
                "Date::new(year, month, day)'s year()/month()/day() must equal year/month/day whenever construction succeeds"
            );
        }
    }
}
