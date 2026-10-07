//! `KaniWitness` for `amenable_ext::ExtStandard<chrono::IsoWeek>`.
//!
//! chrono gives `IsoWeek` no public constructor: every value comes from
//! `NaiveDate::iso_week`. So the claim is about the values a real date produces, over
//! every `i32` year and `u32` month and day.
//!
//! For every valid date, its ISO week's year and week number, together with the
//! date's weekday, must rebuild the same date through `NaiveDate::from_isoywd_opt`,
//! and the week number must lie in `1..=53`. Nothing is narrowed to a sub-range.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;
use chrono::{Datelike, IsoWeek};

use super::gregorian::days_in_month;
use crate::ext_macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<IsoWeek> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_iso_week_of_a_date_rebuilds_that_date".to_owned(),
            VERIFY_ISO_WEEK_OF_A_DATE_REBUILDS_THAT_DATE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<IsoWeek>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<chrono::IsoWeek>",
        "kani",
        || <ExtStandard<IsoWeek> as crate::KaniWitness>::proof().to_string(),
    )
}

kani_ensures_ext!(
    ExtStandard<IsoWeek>,
    "amenable_ext::ExtStandard<chrono::IsoWeek>",
    (i32, u32, u32),
    |(year, month, day)| {
        let in_range =
            (chrono::NaiveDate::MIN.year()..=chrono::NaiveDate::MAX.year()).contains(&year);
        let valid = in_range
            && (1..=12).contains(&month)
            && (1..=days_in_month(year, month)).contains(&day);
        match chrono::NaiveDate::from_ymd_opt(year, month, day) {
            Some(date) => {
                let week = date.iso_week();
                let rebuilt =
                    chrono::NaiveDate::from_isoywd_opt(week.year(), week.week(), date.weekday());
                valid && (1..=53).contains(&week.week()) && rebuilt == Some(date)
            }
            None => !valid,
        }
    }
);

amenable_derive::harness! {
    kani, VERIFY_ISO_WEEK_OF_A_DATE_REBUILDS_THAT_DATE_SRC, {
        /// For every valid date over every `i32` year and `u32` month and day, its ISO
        /// week's year and week, with its weekday, rebuild the same date, and the week
        /// lies in `1..=53`.
        #[kani::proof]
        fn verify_iso_week_of_a_date_rebuilds_that_date() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            assert!(
                ExtStandard::<IsoWeek>::ensures((year, month, day)),
                "a date's ISO week, with its weekday, must rebuild that date"
            );
        }
    }
}
