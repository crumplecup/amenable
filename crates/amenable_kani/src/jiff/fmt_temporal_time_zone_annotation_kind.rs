//! `KaniWitness` for `amenable_ext::ExtStandard<jiff::fmt::temporal::
//! TimeZoneAnnotationKind<'static>>` — a real, checked round-trip
//! property over jiff's actual public API (`From<&str>`/
//! `From<Offset>`/`into_owned`), not a trusted stub.
//!
//! `TimeZoneAnnotationKind<'n>` is a real, `#[non_exhaustive]`
//! two-variant enum (`Named(TimeZoneAnnotationName<'n>)`,
//! `Offset(Offset)`) — checked directly rather than assumed trusted.
//! Unlike `TimeZoneAnnotation<'static>` (whose `kind`/`critical`
//! fields are `pub(crate)` to jiff, not visible here), this enum's
//! own variants ARE public, so `From<&'n str>`'s exact name content
//! is checkable directly by matching, no private-field indirection
//! needed.
//!
//! Scoped to a single-character name for the same reason
//! `fmt_temporal_time_zone_annotation.rs` documents: a `&'static
//! str` can't be constructed symbolically under Kani without the
//! real, documented UTF-8-validation cost wall, so this reuses
//! `char`'s own built-in `Arbitrary` via `c.to_string()`. Reuses
//! `super::offset`'s bounds-check-before-construct pattern
//! for the `Offset` variant.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use crate::ext_macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::fmt::temporal::TimeZoneAnnotationKind<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_fmt_temporal_time_zone_annotation_kind_from_name_and_from_offset".to_owned(),
            VERIFY_FMT_TEMPORAL_TIME_ZONE_ANNOTATION_KIND_FROM_NAME_AND_FROM_OFFSET_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::fmt::temporal::TimeZoneAnnotationKind<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::fmt::temporal::TimeZoneAnnotationKind<'static>>",
        "kani",
        || <ExtStandard<jiff::fmt::temporal::TimeZoneAnnotationKind<'static>> as crate::KaniWitness>::proof()
            .to_string(),
    )
}

/// jiff's own documented valid range for `Offset::from_seconds` — the
/// same constant `super::offset` independently confirms.
const OFFSET_SECONDS_MIN: i32 = -93_599;
const OFFSET_SECONDS_MAX: i32 = 93_599;

kani_ensures_ext!(
    ExtStandard<jiff::fmt::temporal::TimeZoneAnnotationKind<'static>>,
    "amenable_ext::ExtStandard<jiff::fmt::temporal::TimeZoneAnnotationKind<'static>>",
    (char, i32),
    |(c, secs)| {
        let name = c.to_string();
        let kind = jiff::fmt::temporal::TimeZoneAnnotationKind::from(name.as_str()).into_owned();
        let named_ok = match kind {
            jiff::fmt::temporal::TimeZoneAnnotationKind::Named(ref n) => {
                let got = n.as_str().as_bytes();
                let want = name.as_bytes();
                got.len() == want.len()
                    && got.first() == want.first()
                    && got.get(1) == want.get(1)
                    && got.get(2) == want.get(2)
                    && got.get(3) == want.get(3)
            }
            _ => false,
        };

        let offset_ok = if !(OFFSET_SECONDS_MIN..=OFFSET_SECONDS_MAX).contains(&secs) {
            true
        } else {
            let offset = jiff::tz::Offset::from_seconds(secs)
                .expect("secs is already checked to be in Offset::from_seconds's valid range");
            let kind2 = jiff::fmt::temporal::TimeZoneAnnotationKind::from(offset);
            match kind2 {
                jiff::fmt::temporal::TimeZoneAnnotationKind::Offset(o) => o.seconds() == secs,
                _ => false,
            }
        };

        named_ok && offset_ok
    }
);

amenable_derive::harness! {
    kani, VERIFY_FMT_TEMPORAL_TIME_ZONE_ANNOTATION_KIND_FROM_NAME_AND_FROM_OFFSET_SRC, {
        /// `TimeZoneAnnotationKind::from(name).into_owned()` always
        /// builds the `Named` variant round-tripping the exact name
        /// given; `TimeZoneAnnotationKind::from(offset)` always
        /// builds the `Offset` variant carrying `offset`'s own
        /// seconds — checked for every `char`/`i32` pair, not an
        /// assumed slice of one.
        #[kani::proof]
        fn verify_fmt_temporal_time_zone_annotation_kind_from_name_and_from_offset() {
            let c: char = kani::any();
            let secs: i32 = kani::any();
            assert!(
                ExtStandard::<jiff::fmt::temporal::TimeZoneAnnotationKind<'static>>::ensures((
                    c, secs
                )),
                "TimeZoneAnnotationKind::from(name)/from(offset) must build the documented variant"
            );
        }
    }
}
