//! `KaniWitness` for `amenable_ext::ExtStandard<chrono::NaiveDateTime>`.
//!
//! The claim covers construction from a date and a time over every `i32` year,
//! `u32` month, day, hour, minute, second, and nanosecond. It has two directions:
//!
//! - the date-time is `Some` exactly when both the date and the time are valid, and
//! - a `Some` date-time reports back the same date and time it was built from.
//!
//! Validity of the parts is the one defined by chrono's own `from_ymd_opt` and
//! `from_hms_nano_opt` specs. The validity rule is restated here, independently of
//! chrono, in the same form as the `NaiveDate` and `NaiveTime` witnesses. Nothing is
//! narrowed to a sub-range.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;
use chrono::{Datelike, Timelike};

use super::gregorian::days_in_month;
use crate::ext_macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<chrono::NaiveDateTime> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_naive_date_time_new_matches_date_and_time_validity".to_owned(),
            VERIFY_NAIVE_DATE_TIME_NEW_MATCHES_DATE_AND_TIME_VALIDITY_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<chrono::NaiveDateTime>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<chrono::NaiveDateTime>",
        "kani",
        || <ExtStandard<chrono::NaiveDateTime> as crate::KaniWitness>::proof().to_string(),
    )
}

kani_ensures_ext!(
    ExtStandard<chrono::NaiveDateTime>,
    "amenable_ext::ExtStandard<chrono::NaiveDateTime>",
    (i32, u32, u32, u32, u32, u32, u32),
    |(year, month, day, hour, minute, second, nano)| {
        let date_valid = (chrono::NaiveDate::MIN.year()..=chrono::NaiveDate::MAX.year())
            .contains(&year)
            && (1..=12).contains(&month)
            && (1..=days_in_month(year, month)).contains(&day);
        let time_valid = hour < 24
            && minute < 60
            && second < 60
            && !(nano >= 1_000_000_000 && second != 59)
            && nano < 2_000_000_000;
        let valid = date_valid && time_valid;
        let date = chrono::NaiveDate::from_ymd_opt(year, month, day);
        let time = chrono::NaiveTime::from_hms_nano_opt(hour, minute, second, nano);
        match (date, time) {
            (Some(date), Some(time)) => {
                let combined = chrono::NaiveDateTime::new(date, time);
                valid
                    && combined.date() == date
                    && combined.time() == time
                    && combined.year() == year
                    && combined.month() == month
                    && combined.day() == day
                    && combined.hour() == hour
                    && combined.minute() == minute
                    && combined.second() == second
                    && combined.nanosecond() == nano
            }
            _ => !valid,
        }
    }
);

amenable_derive::harness! {
    kani, VERIFY_NAIVE_DATE_TIME_NEW_MATCHES_DATE_AND_TIME_VALIDITY_SRC, {
        /// `NaiveDateTime::new` over a date and a time built from the same fields is
        /// `Some` exactly when both parts are valid, over every `i32` year and `u32`
        /// month, day, hour, minute, second, and nanosecond. A built date-time reports
        /// back the same date and time.
        #[kani::proof]
        fn verify_naive_date_time_new_matches_date_and_time_validity() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            let hour: u32 = kani::any();
            let minute: u32 = kani::any();
            let second: u32 = kani::any();
            let nano: u32 = kani::any();
            assert!(
                ExtStandard::<chrono::NaiveDateTime>::ensures((
                    year, month, day, hour, minute, second, nano
                )),
                "a date-time built from valid parts must round-trip them, and an invalid part must yield no date-time"
            );
        }
    }
}
