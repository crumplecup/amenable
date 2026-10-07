//! `KaniWitness` for `amenable_ext::ExtStandard<jiff::fmt::temporal::
//! PiecesOffset>` — a real, checked round-trip property over jiff's
//! actual public API (`Zulu`/`Numeric`/`to_numeric_offset`/
//! `From<Offset>`), not a trusted stub.
//!
//! `PiecesOffset` is a real, `#[non_exhaustive]` two-variant enum
//! (`Zulu`, `Numeric(PiecesNumericOffset)`), not a parser/printer —
//! checked directly rather than assumed trusted. `to_numeric_offset`
//! maps `Zulu` to `Offset::UTC` and unwraps `Numeric` to its own
//! wrapped offset; `From<Offset>` always builds the `Numeric` variant
//! via `PiecesNumericOffset::from`.
//!
//! Reuses `super::offset`'s bounds-check-before-construct
//! pattern for the same `Result<Offset, jiff::Error>` Drop-glue wall
//! documented there.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use crate::ext_macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::fmt::temporal::PiecesOffset> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_fmt_temporal_pieces_offset_zulu_and_from_offset_round_trip".to_owned(),
            VERIFY_FMT_TEMPORAL_PIECES_OFFSET_ZULU_AND_FROM_OFFSET_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::fmt::temporal::PiecesOffset>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::fmt::temporal::PiecesOffset>",
        "kani",
        || <ExtStandard<jiff::fmt::temporal::PiecesOffset> as crate::KaniWitness>::proof()
            .to_string(),
    )
}

/// jiff's own documented valid range for `Offset::from_seconds` — the
/// same constant `super::offset` independently confirms.
const OFFSET_SECONDS_MIN: i32 = -93_599;
const OFFSET_SECONDS_MAX: i32 = 93_599;

kani_ensures_ext!(
    ExtStandard<jiff::fmt::temporal::PiecesOffset>,
    "amenable_ext::ExtStandard<jiff::fmt::temporal::PiecesOffset>",
    i32,
    |secs| {
        let zulu_ok = jiff::fmt::temporal::PiecesOffset::Zulu
            .to_numeric_offset()
            .seconds()
            == 0;

        if !(OFFSET_SECONDS_MIN..=OFFSET_SECONDS_MAX).contains(&secs) {
            zulu_ok
        } else {
            let offset = jiff::tz::Offset::from_seconds(secs)
                .expect("secs is already checked to be in Offset::from_seconds's valid range");
            let from_ok = jiff::fmt::temporal::PiecesOffset::from(offset)
                .to_numeric_offset()
                .seconds()
                == secs;

            zulu_ok && from_ok
        }
    }
);

amenable_derive::harness! {
    kani, VERIFY_FMT_TEMPORAL_PIECES_OFFSET_ZULU_AND_FROM_OFFSET_ROUND_TRIP_SRC, {
        /// `PiecesOffset::Zulu.to_numeric_offset()` is always
        /// `Offset::UTC`, and `PiecesOffset::from(offset)
        /// .to_numeric_offset()` always round-trips `offset`'s own
        /// seconds back — checked for every `i32`, not an assumed
        /// slice of one.
        #[kani::proof]
        fn verify_fmt_temporal_pieces_offset_zulu_and_from_offset_round_trip() {
            let secs: i32 = kani::any();
            assert!(
                ExtStandard::<jiff::fmt::temporal::PiecesOffset>::ensures(secs),
                "PiecesOffset::Zulu/From<Offset> must map to the documented numeric offset"
            );
        }
    }
}
