//! `KaniWitness` for `amenable_ext::ExtStandard<jiff::civil::Time>` —
//! a real, checked round-trip property over jiff's actual public API
//! (`Time::new`/`hour`/`minute`/`second`/`subsec_nanosecond`), not a
//! trusted stub.
//!
//! Unlike `civil::Date`/`civil::ISOWeekDate`, `Time::new`'s validity
//! is a fully rectangular, unconditional domain with no
//! interdependency between fields at all — checked directly against
//! jiff's real source: `0 <= hour <= 23`, `0 <= minute <= 59`, `0 <=
//! second <= 59`, `0 <= subsec_nanosecond <= 999_999_999`, each
//! independently, no days-in-month-style or leap-week-style
//! complication. So this witness states the FULL documented
//! validity condition exactly, not a narrowed sufficient sub-range —
//! confirmed by reading `Time::MIN`/`MAX` too (midnight /
//! `23:59:59.999999999`, exactly matching these same rectangular
//! bounds, no additional derived-elsewhere restriction the way
//! `ISOWeekDate::MIN`/`MAX` had).

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use super::macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::civil::Time> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_civil_time_new_hour_minute_second_subsec_round_trips".to_owned(),
            VERIFY_CIVIL_TIME_NEW_HOUR_MINUTE_SECOND_SUBSEC_ROUND_TRIPS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::civil::Time>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::civil::Time>",
        "kani",
        || <ExtStandard<jiff::civil::Time> as crate::KaniWitness>::proof().to_string(),
    )
}

/// jiff's own documented FULL valid range for `civil::Time::new`'s
/// four fields — no narrowing needed, unlike `civil_date.rs`/
/// `civil_iso_week_date.rs`, since each field's validity is fully
/// independent of the others.
const TIME_HOUR_MIN: i8 = 0;
const TIME_HOUR_MAX: i8 = 23;
const TIME_MINUTE_MIN: i8 = 0;
const TIME_MINUTE_MAX: i8 = 59;
const TIME_SECOND_MIN: i8 = 0;
const TIME_SECOND_MAX: i8 = 59;
const TIME_SUBSEC_NANOSECOND_MIN: i32 = 0;
const TIME_SUBSEC_NANOSECOND_MAX: i32 = 999_999_999;

kani_ensures_ext!(
    ExtStandard<jiff::civil::Time>,
    "amenable_ext::ExtStandard<jiff::civil::Time>",
    (i8, i8, i8, i32),
    |(hour, minute, second, subsec_nanosecond)| {
        if !(TIME_HOUR_MIN..=TIME_HOUR_MAX).contains(&hour)
            || !(TIME_MINUTE_MIN..=TIME_MINUTE_MAX).contains(&minute)
            || !(TIME_SECOND_MIN..=TIME_SECOND_MAX).contains(&second)
            || !(TIME_SUBSEC_NANOSECOND_MIN..=TIME_SUBSEC_NANOSECOND_MAX)
                .contains(&subsec_nanosecond)
        {
            true
        } else {
            let t = jiff::civil::Time::new(hour, minute, second, subsec_nanosecond).expect(
                "hour/minute/second/subsec_nanosecond are already checked to always be a valid civil::Time",
            );
            t.hour() == hour
                && t.minute() == minute
                && t.second() == second
                && t.subsec_nanosecond() == subsec_nanosecond
        }
    }
);

amenable_derive::harness! {
    kani, VERIFY_CIVIL_TIME_NEW_HOUR_MINUTE_SECOND_SUBSEC_ROUND_TRIPS_SRC, {
        /// `Time::new(hour, minute, second, subsec_nanosecond)`,
        /// whenever it succeeds, always returns a `Time` whose own
        /// `hour()`/`minute()`/`second()`/`subsec_nanosecond()` are
        /// exactly `hour`/`minute`/`second`/`subsec_nanosecond` back
        /// — checked over jiff's FULL documented validity range, not
        /// a narrowed sub-range.
        #[kani::proof]
        fn verify_civil_time_new_hour_minute_second_subsec_round_trips() {
            let hour: i8 = kani::any();
            let minute: i8 = kani::any();
            let second: i8 = kani::any();
            let subsec_nanosecond: i32 = kani::any();
            assert!(
                ExtStandard::<jiff::civil::Time>::ensures((hour, minute, second, subsec_nanosecond)),
                "Time::new(hour, minute, second, subsec_nanosecond)'s accessors must equal the inputs whenever construction succeeds"
            );
        }
    }
}
