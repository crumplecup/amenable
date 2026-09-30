//! `KaniWitness` for `amenable_ext::ExtStandard<jiff::fmt::strtime::
//! Meridiem>` — a real, checked property over jiff's actual public
//! API (`From<jiff::civil::Time> for Meridiem`), not a trusted stub.
//!
//! Unlike `Extension`/`DefaultCustom`, `Meridiem` has one real public
//! conversion (checked directly against jiff's real source,
//! `src/fmt/strtime/mod.rs`): `impl From<Time> for Meridiem { fn
//! from(t: Time) -> Meridiem { if t.hour() < 12 { AM } else { PM } } }`
//! — a real, documented threshold, the same "small enums aren't
//! automatically trusted" lesson `civil::Era`/`civil::Weekday` already
//! established. Only `hour` is inspected by the real conversion (not
//! assumed — read directly from the body), so this witness varies
//! only `hour` over `civil::Time`'s own full valid range
//! (`0..=23`, already established by `civil_time.rs`), fixing
//! minute/second/subsec_nanosecond at `0`.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use super::macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::fmt::strtime::Meridiem> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_fmt_strtime_meridiem_from_time_matches_hour_threshold".to_owned(),
            VERIFY_FMT_STRTIME_MERIDIEM_FROM_TIME_MATCHES_HOUR_THRESHOLD_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::fmt::strtime::Meridiem>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::fmt::strtime::Meridiem>",
        "kani",
        || <ExtStandard<jiff::fmt::strtime::Meridiem> as crate::KaniWitness>::proof().to_string(),
    )
}

kani_ensures_ext!(
    ExtStandard<jiff::fmt::strtime::Meridiem>,
    "amenable_ext::ExtStandard<jiff::fmt::strtime::Meridiem>",
    i8,
    |hour| {
        if !(0i8..=23i8).contains(&hour) {
            true
        } else {
            let t = jiff::civil::Time::new(hour, 0, 0, 0)
                .expect("hour is already checked to be in civil::Time's valid range");
            let meridiem: jiff::fmt::strtime::Meridiem = t.into();
            if hour < 12 {
                meridiem == jiff::fmt::strtime::Meridiem::AM
            } else {
                meridiem == jiff::fmt::strtime::Meridiem::PM
            }
        }
    }
);

amenable_derive::harness! {
    kani, VERIFY_FMT_STRTIME_MERIDIEM_FROM_TIME_MATCHES_HOUR_THRESHOLD_SRC, {
        /// `Meridiem::from(time)` matches jiff's own documented
        /// threshold exactly (`AM` for `hour < 12`, `PM` otherwise) —
        /// checked over `civil::Time`'s full valid hour range, not an
        /// assumed slice.
        #[kani::proof]
        fn verify_fmt_strtime_meridiem_from_time_matches_hour_threshold() {
            let hour: i8 = kani::any();
            assert!(
                ExtStandard::<jiff::fmt::strtime::Meridiem>::ensures(hour),
                "Meridiem::from(time) must match jiff's documented hour < 12 threshold"
            );
        }
    }
}
