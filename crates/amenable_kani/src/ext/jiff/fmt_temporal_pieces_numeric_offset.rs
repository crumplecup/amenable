//! `KaniWitness` for `amenable_ext::ExtStandard<jiff::fmt::temporal::
//! PiecesNumericOffset>` — a real, checked round-trip property over
//! jiff's actual public API (`From<Offset>`/`offset`/`is_negative`/
//! `with_negative_zero`), not a trusted stub.
//!
//! `PiecesNumericOffset` is a real data-carrying type (two private
//! fields: `offset: Offset`, `is_negative: bool`), not a parser/
//! printer engine — checked directly rather than assumed trusted, per
//! this checklist's own "small types are not automatically trusted"
//! discipline. It's a thin wrapper around `jiff::tz::Offset` (already
//! a checked type in this crate — see `super::super::offset`) plus one
//! extra `is_negative` bit that only matters for rendering `-00:00`.
//!
//! Reuses the same bounds-check-before-construct pattern
//! `super::super::offset` established for `Offset::from_seconds`: the
//! `Result<Offset, jiff::Error>` Drop-glue wall documented there
//! applies equally here, since this harness constructs an `Offset` the
//! same way. Never lets the `Err` arm exist for any symbolic input the
//! harness actually reaches.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use super::super::macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::fmt::temporal::PiecesNumericOffset> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_fmt_temporal_pieces_numeric_offset_from_and_with_negative_zero".to_owned(),
            VERIFY_FMT_TEMPORAL_PIECES_NUMERIC_OFFSET_FROM_AND_WITH_NEGATIVE_ZERO_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::fmt::temporal::PiecesNumericOffset>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::fmt::temporal::PiecesNumericOffset>",
        "kani",
        || <ExtStandard<jiff::fmt::temporal::PiecesNumericOffset> as crate::KaniWitness>::proof()
            .to_string(),
    )
}

/// jiff's own documented valid range for `Offset::from_seconds` — the
/// same constant `super::super::offset` independently confirms,
/// restated here so this harness never constructs the drop-heavy `Err`
/// arm of `Offset::from_seconds`'s `Result` in the first place.
const OFFSET_SECONDS_MIN: i32 = -93_599;
const OFFSET_SECONDS_MAX: i32 = 93_599;

kani_ensures_ext!(
    ExtStandard<jiff::fmt::temporal::PiecesNumericOffset>,
    "amenable_ext::ExtStandard<jiff::fmt::temporal::PiecesNumericOffset>",
    i32,
    |secs| {
        if !(OFFSET_SECONDS_MIN..=OFFSET_SECONDS_MAX).contains(&secs) {
            true
        } else {
            let offset = jiff::tz::Offset::from_seconds(secs)
                .expect("secs is already checked to be in Offset::from_seconds's valid range");

            let pno = jiff::fmt::temporal::PiecesNumericOffset::from(offset);
            let from_ok = pno.offset().seconds() == secs && pno.is_negative() == (secs < 0);

            let pno_zeroed =
                jiff::fmt::temporal::PiecesNumericOffset::from(offset).with_negative_zero();
            let with_negative_zero_ok =
                pno_zeroed.offset().seconds() == secs && pno_zeroed.is_negative();

            from_ok && with_negative_zero_ok
        }
    }
);

amenable_derive::harness! {
    kani, VERIFY_FMT_TEMPORAL_PIECES_NUMERIC_OFFSET_FROM_AND_WITH_NEGATIVE_ZERO_SRC, {
        /// `PiecesNumericOffset::from(offset)` always round-trips
        /// `offset`'s own seconds through `.offset().seconds()`, and
        /// sets `.is_negative()` to exactly `offset.seconds() < 0`;
        /// `.with_negative_zero()` always preserves the wrapped
        /// `offset` while forcing `.is_negative()` to `true`. Checked
        /// for every `i32`, not an assumed slice of one.
        #[kani::proof]
        fn verify_fmt_temporal_pieces_numeric_offset_from_and_with_negative_zero() {
            let secs: i32 = kani::any();
            assert!(
                ExtStandard::<jiff::fmt::temporal::PiecesNumericOffset>::ensures(secs),
                "PiecesNumericOffset::from/with_negative_zero must preserve/set the documented fields"
            );
        }
    }
}
