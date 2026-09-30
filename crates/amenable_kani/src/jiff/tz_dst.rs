//! `KaniWitness` for `amenable_ext::ExtStandard<jiff::tz::Dst>` — a
//! real, checked round-trip property over jiff's actual public API
//! (`From<bool>`/`is_dst`/`is_std`), not a trusted stub.
//!
//! `Dst` is a real, plain two-variant enum (`No`/`Yes`) — checked
//! directly rather than assumed trusted, per this checklist's own
//! "small enums are not automatically trusted" discipline.
//! `From<bool>` always builds `Yes` for `true`/`No` for `false`, and
//! `is_dst()`/`is_std()` are each other's exact complement.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use super::macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::tz::Dst> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_tz_dst_from_bool_round_trips".to_owned(),
            VERIFY_TZ_DST_FROM_BOOL_ROUND_TRIPS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::tz::Dst>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::tz::Dst>",
        "kani",
        || <ExtStandard<jiff::tz::Dst> as crate::KaniWitness>::proof().to_string(),
    )
}

kani_ensures_ext!(
    ExtStandard<jiff::tz::Dst>,
    "amenable_ext::ExtStandard<jiff::tz::Dst>",
    bool,
    |is_dst| {
        let dst = jiff::tz::Dst::from(is_dst);
        dst.is_dst() == is_dst && dst.is_std() == !is_dst
    }
);

amenable_derive::harness! {
    kani, VERIFY_TZ_DST_FROM_BOOL_ROUND_TRIPS_SRC, {
        /// `Dst::from(is_dst).is_dst()` always equals `is_dst`, and
        /// `.is_std()` is always its exact complement — checked for
        /// both booleans, not an assumed one.
        #[kani::proof]
        fn verify_tz_dst_from_bool_round_trips() {
            let is_dst: bool = kani::any();
            assert!(
                ExtStandard::<jiff::tz::Dst>::ensures(is_dst),
                "Dst::from(is_dst).is_dst()/.is_std() must round-trip is_dst exactly"
            );
        }
    }
}
