//! Offset-only round-trip decomposition, baseline isolation, for the
//! CBMC round-trip timeout investigation -- see
//! `chrono_offset_date_time_round_trip.rs`'s own module doc comment
//! for the full narrative this file is one chapter of. Isolates which
//! piece of `offset_round_trips`'s own timeout is the cost: the real
//! equality postcondition (trivial ensures still times out -- not the
//! postcondition), the `reflect` direction alone (passes), the
//! `realize` direction added on top (passes), or DFCC's own
//! instrumentation regardless of contracts (a plain, uncontracted
//! `#[kani::proof]` -- passes). Continues in the sibling
//! `chrono_offset_date_time_round_trip_offset_fact_count.rs` (`reflect_
//! offset`'s own contract, decomposed by fact count) and
//! `chrono_offset_date_time_round_trip_offset_staged_fix.rs` (the
//! staged fix built from that decomposition).

mod kani_only {
    #![cfg(kani)]

    // ── Isolating the offset-only round trip's cost ─────────────────
    //
    // `amenable_kani::chrono::offset_date_time::offset_round_trips`
    // (the same two-function offset-only round trip, pure integer
    // arithmetic, no strings) timed out at 3 minutes under `#[kani::
    // proof_for_contract]` with a real, non-trivial `ensures`. These
    // four isolate which piece is actually expensive: the real
    // postcondition, the `reflect` direction alone, the `realize`
    // direction alone, or DFCC's own instrumentation regardless of
    // contract content.

    /// Same body as `offset_round_trips`, but a trivial `ensures` --
    /// isolates "the real equality postcondition is the cost" from
    /// "the body underneath is the cost regardless of what's asserted
    /// about it."
    #[kani::requires(true)]
    #[kani::ensures(|_result: &Result<chrono::FixedOffset, amenable_time::TemporalError>| true)]
    pub(super) fn offset_round_trips_trivial_ensures(
        offset: chrono::FixedOffset,
    ) -> Result<chrono::FixedOffset, amenable_time::TemporalError> {
        let descriptor = amenable_ext::fixed_offset_to_utc_offset_descriptor(offset)?;
        amenable_ext::utc_offset_descriptor_to_fixed_offset(descriptor)
    }

    /// The `reflect` direction alone: `fixed_offset_to_utc_offset_
    /// descriptor`, which goes through `UtcOffsetDescriptorBuilder`
    /// (`derive_builder`), trivial ensures. Isolates the builder-based
    /// "reflect" half's own cost from the "realize" half's.
    #[kani::requires(true)]
    #[kani::ensures(|_result: &Result<amenable_time::UtcOffsetDescriptor, amenable_time::TemporalError>| true)]
    pub(super) fn reflect_offset_only(
        offset: chrono::FixedOffset,
    ) -> Result<amenable_time::UtcOffsetDescriptor, amenable_time::TemporalError> {
        amenable_ext::fixed_offset_to_utc_offset_descriptor(offset)
    }

    /// Both directions, but the `realize` call's own cost is what this
    /// adds on top of `reflect_offset_only`'s -- same builder-
    /// construction step as that one, plus one more call. Comparing
    /// this harness's wall time against `reflect_offset_only`'s
    /// isolates `utc_offset_descriptor_to_fixed_offset`'s own
    /// incremental cost (no `derive_builder` on this side at all --
    /// just arithmetic and `FixedOffset::east_opt`).
    #[kani::requires(true)]
    #[kani::ensures(|_result: &Result<chrono::FixedOffset, amenable_time::TemporalError>| true)]
    pub(super) fn realize_offset_only(
        offset: chrono::FixedOffset,
    ) -> Result<chrono::FixedOffset, amenable_time::TemporalError> {
        let descriptor = amenable_ext::fixed_offset_to_utc_offset_descriptor(offset)?;
        amenable_ext::utc_offset_descriptor_to_fixed_offset(descriptor)
    }

    /// The identical round-trip body, with no Kani contract and no DFCC
    /// at all -- a plain `#[kani::proof]`. Isolates "DFCC's own
    /// instrumentation is the cost" from "the real body is the cost
    /// regardless of contracts."
    pub(super) fn offset_round_trip_uncontracted(
        offset: chrono::FixedOffset,
    ) -> Option<chrono::FixedOffset> {
        let descriptor = amenable_ext::fixed_offset_to_utc_offset_descriptor(offset).ok()?;
        amenable_ext::utc_offset_descriptor_to_fixed_offset(descriptor).ok()
    }
}

#[cfg(kani)]
use kani_only::{
    offset_round_trip_uncontracted, offset_round_trips_trivial_ensures, realize_offset_only,
    reflect_offset_only,
};

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::offset_round_trip_trivial_ensures_isolation".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::offset_round_trip_trivial_ensures_isolation".to_owned(),
            "amenable_kani".to_owned(),
            "the offset-only round trip's body, with a trivial ensures instead of the real equality postcondition -- confirmed still times out: the real equality postcondition was never the cost".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, OFFSET_ROUND_TRIP_TRIVIAL_ENSURES_ISOLATION_SRC, {
        /// `offset_round_trips_trivial_ensures`'s own contract: same
        /// body as the production `offset_round_trips`, trivial
        /// ensures.
        #[kani::proof_for_contract(offset_round_trips_trivial_ensures)]
        fn offset_round_trip_trivial_ensures_isolation() {
            let offset_seconds: i32 = kani::any();
            let Some(offset) = chrono::FixedOffset::east_opt(offset_seconds) else {
                return;
            };
            let _ = offset_round_trips_trivial_ensures(offset);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::reflect_offset_only_isolation".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::reflect_offset_only_isolation".to_owned(),
            "amenable_kani".to_owned(),
            "the reflect direction alone (fixed_offset_to_utc_offset_descriptor, the derive_builder-based half), trivial ensures -- confirmed passes (126.7s): the builder alone, under DFCC, is not the bottleneck".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, REFLECT_OFFSET_ONLY_ISOLATION_SRC, {
        /// `reflect_offset_only`'s own contract: `fixed_offset_to_utc_
        /// offset_descriptor` alone, trivial ensures.
        #[kani::proof_for_contract(reflect_offset_only)]
        fn reflect_offset_only_isolation() {
            let offset_seconds: i32 = kani::any();
            let Some(offset) = chrono::FixedOffset::east_opt(offset_seconds) else {
                return;
            };
            let _ = reflect_offset_only(offset);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::realize_offset_only_isolation".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::realize_offset_only_isolation".to_owned(),
            "amenable_kani".to_owned(),
            "both directions, trivial ensures -- confirmed times out (reflect_offset_only_isolation alone passes at 126.7s): adding the second, unstubbed real call tips DFCC over budget on its own".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, REALIZE_OFFSET_ONLY_ISOLATION_SRC, {
        /// `realize_offset_only`'s own contract: both directions,
        /// trivial ensures.
        #[kani::proof_for_contract(realize_offset_only)]
        fn realize_offset_only_isolation() {
            let offset_seconds: i32 = kani::any();
            let Some(offset) = chrono::FixedOffset::east_opt(offset_seconds) else {
                return;
            };
            let _ = realize_offset_only(offset);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::offset_round_trip_uncontracted_isolation".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::offset_round_trip_uncontracted_isolation".to_owned(),
            "amenable_kani".to_owned(),
            "the identical offset-only round-trip body with no Kani contract and no DFCC at all -- confirmed passes (~77s): the real body, uncontracted, is cheap; DFCC's own instrumentation on two chained real calls is the bottleneck".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, OFFSET_ROUND_TRIP_UNCONTRACTED_ISOLATION_SRC, {
        /// The identical offset-only round-trip body, no contract, no
        /// DFCC -- a plain `#[kani::proof]`.
        #[kani::proof]
        fn offset_round_trip_uncontracted_isolation() {
            let offset_seconds: i32 = kani::any();
            let Some(offset) = chrono::FixedOffset::east_opt(offset_seconds) else {
                return;
            };
            let _ = offset_round_trip_uncontracted(offset);
        }
    }
}
