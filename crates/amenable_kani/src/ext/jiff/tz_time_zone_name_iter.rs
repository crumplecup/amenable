//! `KaniWitness` for `amenable_ext::ExtStandard<jiff::tz::
//! TimeZoneNameIter<'static>>` — a real, checked property over jiff's
//! actual public API (`TimeZoneDatabase::available`/
//! `Iterator::next`), not a trusted stub.
//!
//! Scoped to `TimeZoneDatabase::none().available().next() == None`:
//! real jiff source confirms `available()` for the `Repr::Empty` case
//! (`TimeZoneDatabase::none()`) returns `TimeZoneNameIter::empty()`
//! (backed by a real, empty `Vec::new().into_iter()`), and its own
//! `next()` just delegates to that empty `IntoIter`'s own `next()` —
//! genuinely cheap and dispatch-free, a DIFFERENT (simpler) internal
//! repr from `TimeZone`'s own pointer-tagged union (the same
//! distinction already confirmed for `TimeZoneDatabase` itself).
//! Unlike `TimeZoneFollowingTransitions`, this is checkable on Kani:
//! nothing here ever touches `TimeZone::Repr`'s dispatch machinery at
//! all. Needs a genuine `&'static TimeZoneDatabase` (the checklist's
//! own registered type is `<'static>`, and `available<'d>(&'d self)`
//! ties its lifetime to the borrow) — a real, third confirmed
//! instance this session of the SAME E0716 "temporary value dropped
//! while borrowed" limitation (`TimeZoneDatabase::none()`, even
//! bound to a local `const`, still doesn't promote to `'static` when
//! used as a method-call receiver), fixed the same proven way:
//! `Box::leak(Box::new(TimeZoneDatabase::none()))`.
//!
//! Trusted on Creusot specifically, for a DIFFERENT, already-
//! confirmed reason: `extern_spec!` doesn't support third-party
//! `Iterator` trait impls at all (the same real "`IteratorSpec` is
//! not satisfied" error already confirmed for `TimeZoneFollowingTransitions`
//! — see `amenable_creusot::ext::jiff`'s own doc comment), and the
//! fallback accommodation-model pattern would be genuinely
//! content-free here too (the claim is an unconditional constant,
//! `None`, no formula to check consistency of).

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use super::super::macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::tz::TimeZoneNameIter<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_tz_time_zone_name_iter_next_is_none_for_empty_database".to_owned(),
            VERIFY_TZ_TIME_ZONE_NAME_ITER_NEXT_IS_NONE_FOR_EMPTY_DATABASE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::tz::TimeZoneNameIter<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::tz::TimeZoneNameIter<'static>>",
        "kani",
        || <ExtStandard<jiff::tz::TimeZoneNameIter<'static>> as crate::KaniWitness>::proof()
            .to_string(),
    )
}

kani_ensures_ext!(
    ExtStandard<jiff::tz::TimeZoneNameIter<'static>>,
    "amenable_ext::ExtStandard<jiff::tz::TimeZoneNameIter<'static>>",
    (),
    |()| {
        let db: &'static jiff::tz::TimeZoneDatabase =
            Box::leak(Box::new(jiff::tz::TimeZoneDatabase::none()));
        let mut it = db.available();
        it.next().is_none()
    }
);

amenable_derive::harness! {
    kani, VERIFY_TZ_TIME_ZONE_NAME_ITER_NEXT_IS_NONE_FOR_EMPTY_DATABASE_SRC, {
        /// `TimeZoneDatabase::none().available().next()` is always
        /// `None` — a fixed fact about jiff's real implementation,
        /// not an assumed one.
        #[kani::proof]
        fn verify_tz_time_zone_name_iter_next_is_none_for_empty_database() {
            let unit: () = kani::any();
            assert!(
                ExtStandard::<jiff::tz::TimeZoneNameIter<'static>>::ensures(unit),
                "TimeZoneDatabase::none().available() must yield no names"
            );
        }
    }
}
