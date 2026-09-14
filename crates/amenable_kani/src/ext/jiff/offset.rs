//! `KaniWitness` for `amenable_ext::ExtStandard<jiff::tz::Offset>` — a
//! real, checked round-trip property over jiff's actual public API
//! (`Offset::from_seconds`/`Offset::seconds`), not a trusted stub.
//!
//! `Offset` is a good first checked candidate: unlike `Timestamp`/
//! `Zoned`/`civil::DateTime` (opaque composite types with no simple
//! black-box property), it's a thin wrapper around a bounded `i32` with
//! one clear round-trip to state, entirely through jiff's public
//! constructors/accessors — Kani never needs to see `jiff`'s private
//! representation to check it.
//!
//! One real CBMC wall found and worked around here, not hidden: `jiff::
//! Error` is `Arc<ErrorInner>`-backed and `ErrorInner` holds `cause:
//! Option<Error>` — a recursive, heap-backed error chain. Any harness
//! that lets a symbolic `Result<Offset, jiff::Error>` drop *normally* on
//! its `Err` arm (`if let Ok(..) = .. { }`, `match`, `.is_ok()`,
//! `.map_or(..)`) times out at 3 minutes even for a tiny assumed range —
//! confirmed by isolating down to a bare `Offset::from_seconds(secs)`
//! call with the result immediately discarded, and confirmed to be Drop
//! glue specifically (not general call-graph complexity) because
//! `std::mem::forget`-ing the same call passes instantly. `.unwrap()`
//! doesn't hit this: Kani compiles with `panic = "abort"`, so the `Err`
//! arm aborts without ever running Drop. The fix below never calls
//! `Offset::from_seconds` on a value that might be out of range in the
//! first place — an independent, drop-free `i32` bounds check gates the
//! call, so `.unwrap()` is a safe assertion under a proven precondition,
//! not a workaround, and the property stays correct for every `i32`, not
//! just an assumed slice of it.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use super::super::macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::tz::Offset> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_offset_from_seconds_round_trips".to_owned(),
            VERIFY_OFFSET_FROM_SECONDS_ROUND_TRIPS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::tz::Offset>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::tz::Offset>",
        "kani",
        || <ExtStandard<jiff::tz::Offset> as crate::KaniWitness>::proof().to_string(),
    )
}

/// jiff's own documented valid range for `Offset::from_seconds`
/// (`-25:59:59..=25:59:59`, in seconds) — checked independently of
/// `from_seconds` itself so the drop-heavy `Err` arm of its `Result`
/// never has to be constructed at all when `secs` is out of range (see
/// this module's own doc comment for why that matters).
const OFFSET_SECONDS_MIN: i32 = -93_599;
const OFFSET_SECONDS_MAX: i32 = 93_599;

kani_ensures_ext!(
    ExtStandard<jiff::tz::Offset>,
    "amenable_ext::ExtStandard<jiff::tz::Offset>",
    i32,
    |secs| {
        if !(OFFSET_SECONDS_MIN..=OFFSET_SECONDS_MAX).contains(&secs) {
            true
        } else {
            jiff::tz::Offset::from_seconds(secs)
                .expect("secs is already checked to be in Offset::from_seconds's valid range")
                .seconds()
                == secs
        }
    }
);

amenable_derive::harness! {
    kani, VERIFY_OFFSET_FROM_SECONDS_ROUND_TRIPS_SRC, {
        /// `Offset::from_seconds(secs)`, whenever it succeeds, always
        /// returns an `Offset` whose own `seconds()` is exactly `secs`
        /// back — and when it fails (out of jiff's documented
        /// -93,599..=93,599 range), the property holds vacuously. Checked
        /// for every `i32`, not an assumed slice of one.
        #[kani::proof]
        fn verify_offset_from_seconds_round_trips() {
            let secs: i32 = kani::any();
            assert!(
                ExtStandard::<jiff::tz::Offset>::ensures(secs),
                "Offset::from_seconds(secs).seconds() must equal secs whenever construction succeeds"
            );
        }
    }
}
