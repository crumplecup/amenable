//! `KaniWitness` for `amenable_ext::ExtStandard<chrono::FixedOffset>`.
//!
//! `FixedOffset` is a thin wrapper around a bounded `i32`, exactly like jiff's
//! `Offset` (see `amenable_kani::jiff::offset`), with the same shape of claim: its
//! two checked constructors, `east_opt`/`west_opt`, succeed exactly on chrono's
//! documented bound (`-86_400 < secs < 86_400`), and whenever one succeeds,
//! `local_minus_utc()` reports back the value the constructor was given — `secs`
//! itself for `east_opt`, `-secs` for `west_opt`. Nothing is narrowed to a
//! sub-range: every `i32` second count is checked, and the `bool` selects which of
//! the two constructors that count is checked against.
//!
//! Unlike jiff's `Offset::from_seconds`, chrono's constructors return `Option`, not
//! a `Result` holding a heap-backed error chain, so the Drop-glue CBMC wall that
//! file documents does not apply here.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;
use chrono::FixedOffset;

use crate::ext_macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<FixedOffset> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_fixed_offset_east_and_west_round_trip".to_owned(),
            VERIFY_FIXED_OFFSET_EAST_AND_WEST_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<FixedOffset>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<chrono::FixedOffset>",
        "kani",
        || <ExtStandard<FixedOffset> as crate::KaniWitness>::proof().to_string(),
    )
}

/// chrono's own documented valid range for `FixedOffset::east_opt`/`west_opt`:
/// `-23:59:59..=23:59:59`, in seconds, as a strict open interval (`fixed.rs`'s own
/// `-86_400 < secs && secs < 86_400`).
const FIXED_OFFSET_SECONDS_MIN: i32 = -86_399;
const FIXED_OFFSET_SECONDS_MAX: i32 = 86_399;

kani_ensures_ext!(
    ExtStandard<FixedOffset>,
    "amenable_ext::ExtStandard<chrono::FixedOffset>",
    (i32, bool),
    |(secs, east)| {
        let in_range = (FIXED_OFFSET_SECONDS_MIN..=FIXED_OFFSET_SECONDS_MAX).contains(&secs);
        if east {
            match FixedOffset::east_opt(secs) {
                Some(offset) => in_range && offset.local_minus_utc() == secs,
                None => !in_range,
            }
        } else {
            match FixedOffset::west_opt(secs) {
                Some(offset) => in_range && offset.local_minus_utc() == -secs,
                None => !in_range,
            }
        }
    }
);

amenable_derive::harness! {
    kani, VERIFY_FIXED_OFFSET_EAST_AND_WEST_ROUND_TRIP_SRC, {
        /// `FixedOffset::east_opt(secs)` and `FixedOffset::west_opt(secs)` each
        /// succeed exactly when `secs` is in chrono's documented bound, and whenever
        /// one succeeds, `local_minus_utc()` reports back `secs` (east) or `-secs`
        /// (west) — checked for every `i32`, not an assumed slice of one.
        #[kani::proof]
        fn verify_fixed_offset_east_and_west_round_trip() {
            let secs: i32 = kani::any();
            let east: bool = kani::any();
            assert!(
                ExtStandard::<FixedOffset>::ensures((secs, east)),
                "east_opt/west_opt must succeed exactly in range, and local_minus_utc() must report back the value given"
            );
        }
    }
}
