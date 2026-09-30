//! `KaniWitness` for `amenable_ext::ExtStandard<jiff::fmt::temporal::
//! TimeZoneAnnotation<'static>>` — a real, checked round-trip
//! property over jiff's actual public API (`From<&str>`/
//! `From<Offset>`/`kind`/`is_critical`/`into_owned`), not a trusted
//! stub.
//!
//! `TimeZoneAnnotation<'n>` is a real data-carrying type (`kind:
//! TimeZoneAnnotationKind<'n>`, `critical: bool`, both `pub(crate)`
//! to jiff, not `pub`), not a parser/printer — checked directly
//! rather than assumed trusted, per this checklist's own "small types
//! are not automatically trusted" discipline. `From<&'n str>` always
//! builds the `Named` variant with `is_critical() == false`;
//! `From<Offset>` always builds the `Offset` variant (with the
//! wrapped offset's own seconds) and `is_critical() == false`.
//!
//! Scoped to a single-character name: a `&'static str` can't be
//! constructed symbolically under Kani (no `Arbitrary` for owned
//! strings that avoids the real, documented UTF-8-validation cost
//! wall — see `rust_std::primitives::char_string`'s own doc comment),
//! so this reuses `char`'s own built-in `Arbitrary` (already
//! Unicode-scalar-constrained by the type itself) via
//! `c.to_string()`, which needs no UTF-8 validation at all since a
//! `char` is already guaranteed valid — checked for every `char`, not
//! an assumed slice of one. The borrowed name is promoted to
//! `'static` via `into_owned()`, exercising that real conversion too.
//!
//! Reuses `super::offset`'s bounds-check-before-construct
//! pattern for the `Offset` variant, avoiding the same
//! `Result<Offset, jiff::Error>` Drop-glue wall documented there.
//!
//! One real CBMC wall found and worked around here, not hidden:
//! comparing `n.as_str() == name.as_str()` directly timed out at 3
//! minutes — the same CBMC-memcmp-unwind wall already documented for
//! `Vec<u8>`/slice `==` comparisons (`str`'s `PartialEq` delegates to
//! slice comparison under the hood). Fixed with manual indexed byte
//! comparison instead, bounded to 4 positions (the maximum UTF-8
//! width of a single `char`).

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use super::macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::fmt::temporal::TimeZoneAnnotation<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_fmt_temporal_time_zone_annotation_from_name_and_from_offset".to_owned(),
            VERIFY_FMT_TEMPORAL_TIME_ZONE_ANNOTATION_FROM_NAME_AND_FROM_OFFSET_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::fmt::temporal::TimeZoneAnnotation<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::fmt::temporal::TimeZoneAnnotation<'static>>",
        "kani",
        || <ExtStandard<jiff::fmt::temporal::TimeZoneAnnotation<'static>> as crate::KaniWitness>::proof()
            .to_string(),
    )
}

/// jiff's own documented valid range for `Offset::from_seconds` — the
/// same constant `super::offset` independently confirms.
const OFFSET_SECONDS_MIN: i32 = -93_599;
const OFFSET_SECONDS_MAX: i32 = 93_599;

kani_ensures_ext!(
    ExtStandard<jiff::fmt::temporal::TimeZoneAnnotation<'static>>,
    "amenable_ext::ExtStandard<jiff::fmt::temporal::TimeZoneAnnotation<'static>>",
    (char, i32),
    |(c, secs)| {
        let name = c.to_string();
        let ann = jiff::fmt::temporal::TimeZoneAnnotation::from(name.as_str()).into_owned();
        let named_ok = !ann.is_critical()
            && match ann.kind() {
                jiff::fmt::temporal::TimeZoneAnnotationKind::Named(n) => {
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
            let ann2 = jiff::fmt::temporal::TimeZoneAnnotation::from(offset);
            !ann2.is_critical()
                && match ann2.kind() {
                    jiff::fmt::temporal::TimeZoneAnnotationKind::Offset(o) => o.seconds() == secs,
                    _ => false,
                }
        };

        named_ok && offset_ok
    }
);

amenable_derive::harness! {
    kani, VERIFY_FMT_TEMPORAL_TIME_ZONE_ANNOTATION_FROM_NAME_AND_FROM_OFFSET_SRC, {
        /// `TimeZoneAnnotation::from(name).into_owned()` always builds
        /// the `Named` variant round-tripping the exact name given,
        /// with `is_critical() == false`; `TimeZoneAnnotation::
        /// from(offset)` always builds the `Offset` variant carrying
        /// `offset`'s own seconds, with `is_critical() == false` —
        /// checked for every `char`/`i32` pair, not an assumed slice
        /// of one.
        #[kani::proof]
        fn verify_fmt_temporal_time_zone_annotation_from_name_and_from_offset() {
            let c: char = kani::any();
            let secs: i32 = kani::any();
            assert!(
                ExtStandard::<jiff::fmt::temporal::TimeZoneAnnotation<'static>>::ensures((
                    c, secs
                )),
                "TimeZoneAnnotation::from(name)/from(offset) must build the documented variant"
            );
        }
    }
}
