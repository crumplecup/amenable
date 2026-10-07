//! `KaniWitness` for `amenable_ext::ExtStandard<chrono::NaiveTime>`.
//!
//! The claim is a full characterization of `NaiveTime::from_hms_nano_opt` over
//! every `u32` hour, minute, second, and nanosecond. It has two directions:
//!
//! - `Some` exactly when the fields name a real time of day, including a leap
//!   second (nanoseconds at or above `1_000_000_000`, on second `59` only, and
//!   below `2_000_000_000`), and
//! - a `Some` time reports back the hour, minute, second, and nanosecond it was
//!   built from.
//!
//! The validity rule is written out here independently of chrono. Nothing is
//! narrowed to a sub-range.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;
use chrono::Timelike;

use crate::ext_macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<chrono::NaiveTime> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_naive_time_from_hms_nano_matches_time_of_day_validity".to_owned(),
            VERIFY_NAIVE_TIME_FROM_HMS_NANO_MATCHES_TIME_OF_DAY_VALIDITY_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<chrono::NaiveTime>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<chrono::NaiveTime>",
        "kani",
        || <ExtStandard<chrono::NaiveTime> as crate::KaniWitness>::proof().to_string(),
    )
}

kani_ensures_ext!(
    ExtStandard<chrono::NaiveTime>,
    "amenable_ext::ExtStandard<chrono::NaiveTime>",
    (u32, u32, u32, u32),
    |(hour, minute, second, nano)| {
        let valid = hour < 24
            && minute < 60
            && second < 60
            && !(nano >= 1_000_000_000 && second != 59)
            && nano < 2_000_000_000;
        match chrono::NaiveTime::from_hms_nano_opt(hour, minute, second, nano) {
            Some(time) => {
                valid
                    && time.hour() == hour
                    && time.minute() == minute
                    && time.second() == second
                    && time.nanosecond() == nano
            }
            None => !valid,
        }
    }
);

amenable_derive::harness! {
    kani, VERIFY_NAIVE_TIME_FROM_HMS_NANO_MATCHES_TIME_OF_DAY_VALIDITY_SRC, {
        /// `NaiveTime::from_hms_nano_opt` is `Some` exactly when the fields name a
        /// real time of day, including a leap second, over every `u32` hour,
        /// minute, second, and nanosecond. A `Some` time reports back the same
        /// fields.
        #[kani::proof]
        fn verify_naive_time_from_hms_nano_matches_time_of_day_validity() {
            let hour: u32 = kani::any();
            let minute: u32 = kani::any();
            let second: u32 = kani::any();
            let nano: u32 = kani::any();
            assert!(
                ExtStandard::<chrono::NaiveTime>::ensures((hour, minute, second, nano)),
                "from_hms_nano_opt must be Some exactly for valid times of day, and round-trip them"
            );
        }
    }
}
