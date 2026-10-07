//! `KaniWitness` for `amenable_ext::ExtStandard<chrono::NaiveDate>`.
//!
//! The claim is a full characterization of `NaiveDate::from_ymd_opt` over every
//! `i32` year, `u32` month, and `u32` day. It has two directions:
//!
//! - `Some` exactly when the triple is a real proleptic Gregorian date, and
//! - a `Some` date reports back the year, month, and day it was built from.
//!
//! The validity rule is written out here, independently of chrono: the year is
//! inside chrono's supported range, the month is `1..=12`, and the day is
//! `1..=days_in_month(year, month)`, with the Gregorian leap rule. Nothing is
//! narrowed to a sub-range.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use chrono::Datelike;

use super::gregorian::days_in_month;
use crate::ext_macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<chrono::NaiveDate> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_naive_date_from_ymd_matches_gregorian_validity".to_owned(),
            VERIFY_NAIVE_DATE_FROM_YMD_MATCHES_GREGORIAN_VALIDITY_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<chrono::NaiveDate>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<chrono::NaiveDate>",
        "kani",
        || <ExtStandard<chrono::NaiveDate> as crate::KaniWitness>::proof().to_string(),
    )
}

kani_ensures_ext!(
    ExtStandard<chrono::NaiveDate>,
    "amenable_ext::ExtStandard<chrono::NaiveDate>",
    (i32, u32, u32),
    |(year, month, day)| {
        let in_range =
            (chrono::NaiveDate::MIN.year()..=chrono::NaiveDate::MAX.year()).contains(&year);
        let valid = in_range
            && (1..=12).contains(&month)
            && (1..=days_in_month(year, month)).contains(&day);
        match chrono::NaiveDate::from_ymd_opt(year, month, day) {
            Some(date) => {
                valid && date.year() == year && date.month() == month && date.day() == day
            }
            None => !valid,
        }
    }
);

amenable_derive::harness! {
    kani, VERIFY_NAIVE_DATE_FROM_YMD_MATCHES_GREGORIAN_VALIDITY_SRC, {
        /// `NaiveDate::from_ymd_opt(year, month, day)` is `Some` exactly when the
        /// triple is a real proleptic Gregorian date, over every `i32` year and
        /// every `u32` month and day, and a `Some` date reports back the same
        /// year, month, and day.
        #[kani::proof]
        fn verify_naive_date_from_ymd_matches_gregorian_validity() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            assert!(
                ExtStandard::<chrono::NaiveDate>::ensures((year, month, day)),
                "from_ymd_opt must be Some exactly for valid Gregorian dates, and round-trip them"
            );
        }
    }
}
