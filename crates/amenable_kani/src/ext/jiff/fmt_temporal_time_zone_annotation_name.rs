//! `KaniWitness` for `amenable_ext::ExtStandard<jiff::fmt::temporal::
//! TimeZoneAnnotationName<'static>>` — a real, checked round-trip
//! property over jiff's actual public API (`From<&str>`/`as_str`/
//! `into_owned`), not a trusted stub.
//!
//! `TimeZoneAnnotationName<'n>` wraps one private field (`name:
//! StringCow<'n>`) with a real, checkable round trip:
//! `TimeZoneAnnotationName::from(s).as_str() == s` for every `&str`.
//!
//! Scoped to a single-character name for the same reason
//! `fmt_temporal_time_zone_annotation.rs` documents: a `&'static
//! str` can't be constructed symbolically under Kani without the
//! real, documented UTF-8-validation cost wall, so this reuses
//! `char`'s own built-in `Arbitrary` via `c.to_string()`, promoted to
//! `'static` via `into_owned()`. Uses manual indexed byte comparison
//! (not `==`) for the same real CBMC-memcmp-unwind wall
//! `fmt_temporal_time_zone_annotation.rs` already found and fixed.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use super::super::macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::fmt::temporal::TimeZoneAnnotationName<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_fmt_temporal_time_zone_annotation_name_from_str_round_trips".to_owned(),
            VERIFY_FMT_TEMPORAL_TIME_ZONE_ANNOTATION_NAME_FROM_STR_ROUND_TRIPS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::fmt::temporal::TimeZoneAnnotationName<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::fmt::temporal::TimeZoneAnnotationName<'static>>",
        "kani",
        || <ExtStandard<jiff::fmt::temporal::TimeZoneAnnotationName<'static>> as crate::KaniWitness>::proof()
            .to_string(),
    )
}

kani_ensures_ext!(
    ExtStandard<jiff::fmt::temporal::TimeZoneAnnotationName<'static>>,
    "amenable_ext::ExtStandard<jiff::fmt::temporal::TimeZoneAnnotationName<'static>>",
    char,
    |c| {
        let name = c.to_string();
        let ann_name =
            jiff::fmt::temporal::TimeZoneAnnotationName::from(name.as_str()).into_owned();
        let got = ann_name.as_str().as_bytes();
        let want = name.as_bytes();
        got.len() == want.len()
            && got.first() == want.first()
            && got.get(1) == want.get(1)
            && got.get(2) == want.get(2)
            && got.get(3) == want.get(3)
    }
);

amenable_derive::harness! {
    kani, VERIFY_FMT_TEMPORAL_TIME_ZONE_ANNOTATION_NAME_FROM_STR_ROUND_TRIPS_SRC, {
        /// `TimeZoneAnnotationName::from(name).into_owned()` always
        /// round-trips the exact name given through `as_str()` —
        /// checked for every `char`, not an assumed slice of one.
        #[kani::proof]
        fn verify_fmt_temporal_time_zone_annotation_name_from_str_round_trips() {
            let c: char = kani::any();
            assert!(
                ExtStandard::<jiff::fmt::temporal::TimeZoneAnnotationName<'static>>::ensures(c),
                "TimeZoneAnnotationName::from(name).as_str() must equal the name given"
            );
        }
    }
}
