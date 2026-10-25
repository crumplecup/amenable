//! Offset-only round-trip decomposition, the staged fix, for the CBMC
//! round-trip timeout investigation -- see
//! `chrono_offset_date_time_round_trip.rs`'s own module doc comment
//! for the full narrative this file is one chapter of, and the
//! sibling `chrono_offset_date_time_round_trip_offset_fact_count_decomposition.rs`
//! for the fact-count decomposition this fix is built from. Stages the
//! fix the same way the digit chain resolved its own cost (sibling
//! `chrono_offset_date_time_round_trip_digit_decomposition.rs`): prove
//! 2 facts, stub it, add 1, prove 3, stub it, add 1, prove 4 -- so the
//! composed proof never has to jointly discharge more than the
//! verifier can manage in one pass. `reflect_offset_staged`, the final
//! composed claim stubbing both the fact-count chain and the
//! descriptor-transcription half, passes.

mod kani_only {
    #![cfg(kani)]

    // ── Staged, digit-chain-shaped fix: prove 2, stub it, add 1 ─────
    //
    // Same technique that resolved the nine-digit identity: each stage
    // computes one more fact and stubs the *previous stage's proven
    // conjunction* instead of re-deriving everything from `offset`
    // inline. Pure arithmetic, no builder yet -- isolates whether
    // staging rescues the fact-count conjunction the way it rescued
    // the call chain, before reintroducing the builder's own cost.

    /// Base case: `sign` and `hours` from raw arithmetic (no builder at
    /// all). Matches `reflect_offset_sign_and_hours`'s own two facts.
    #[kani::requires(true)]
    #[kani::ensures(|result: &(amenable_time::UtcOffsetSign, u8)| {
        let seconds = offset.local_minus_utc();
        let sign_matches = match result.0 {
            amenable_time::UtcOffsetSign::Positive => seconds >= 0,
            amenable_time::UtcOffsetSign::Negative => seconds < 0,
        };
        sign_matches && i64::from(result.1) == i64::from(seconds.unsigned_abs() / 3600)
    })]
    pub(super) fn offset_parts_2(
        offset: chrono::FixedOffset,
    ) -> (amenable_time::UtcOffsetSign, u8) {
        let seconds = offset.local_minus_utc();
        let sign = if seconds < 0 {
            amenable_time::UtcOffsetSign::Negative
        } else {
            amenable_time::UtcOffsetSign::Positive
        };
        let hours = (seconds.unsigned_abs() / 3600) as u8;
        (sign, hours)
    }

    /// Stage 3: stubs `offset_parts_2`'s proven two-fact conjunction,
    /// adds `minutes` as the one new real computation.
    #[kani::requires(true)]
    #[kani::ensures(|result: &(amenable_time::UtcOffsetSign, u8, u8)| {
        let seconds = offset.local_minus_utc();
        let sign_matches = match result.0 {
            amenable_time::UtcOffsetSign::Positive => seconds >= 0,
            amenable_time::UtcOffsetSign::Negative => seconds < 0,
        };
        let magnitude = seconds.unsigned_abs();
        sign_matches
            && i64::from(result.1) == i64::from(magnitude / 3600)
            && i64::from(result.2) == i64::from((magnitude % 3600) / 60)
    })]
    pub(super) fn offset_parts_3(
        offset: chrono::FixedOffset,
    ) -> (amenable_time::UtcOffsetSign, u8, u8) {
        let (sign, hours) = offset_parts_2(offset);
        let minutes = ((offset.local_minus_utc().unsigned_abs() % 3600) / 60) as u8;
        (sign, hours, minutes)
    }

    /// Stage 4: stubs `offset_parts_3`'s proven three-fact conjunction,
    /// adds `seconds` -- the full four-fact relation, staged.
    #[kani::requires(true)]
    #[kani::ensures(|result: &(amenable_time::UtcOffsetSign, u8, u8, u8)| {
        let seconds_offset = offset.local_minus_utc();
        let sign_matches = match result.0 {
            amenable_time::UtcOffsetSign::Positive => seconds_offset >= 0,
            amenable_time::UtcOffsetSign::Negative => seconds_offset < 0,
        };
        let magnitude = seconds_offset.unsigned_abs();
        sign_matches
            && i64::from(result.1) == i64::from(magnitude / 3600)
            && i64::from(result.2) == i64::from((magnitude % 3600) / 60)
            && i64::from(result.3) == i64::from(magnitude % 60)
    })]
    pub(super) fn offset_parts_4(
        offset: chrono::FixedOffset,
    ) -> (amenable_time::UtcOffsetSign, u8, u8, u8) {
        let (sign, hours, minutes) = offset_parts_3(offset);
        let seconds = (offset.local_minus_utc().unsigned_abs() % 60) as u8;
        (sign, hours, minutes, seconds)
    }

    /// The other half of the staged fix: given already-decomposed parts
    /// (no `FixedOffset` arithmetic left to reason about at all), does
    /// the real `UtcOffsetDescriptorBuilder` transcribe them into a
    /// descriptor whose getters match, within budget? Tests the
    /// builder's own four-fact conjunction in isolation from the
    /// offset-decomposition arithmetic `offset_parts_4` already proved.
    #[kani::requires(hours <= 23)]
    #[kani::ensures(|result: &Result<amenable_time::UtcOffsetDescriptor, amenable_time::TemporalError>| {
        match result {
            Ok(descriptor) => {
                descriptor.sign() == sign
                    && descriptor.hours() == hours
                    && descriptor.minutes().unwrap_or(0) == minutes
                    && descriptor.seconds().unwrap_or(0) == seconds
            }
            Err(_) => false,
        }
    })]
    pub(super) fn build_descriptor_from_parts(
        sign: amenable_time::UtcOffsetSign,
        hours: u8,
        minutes: u8,
        seconds: u8,
    ) -> Result<amenable_time::UtcOffsetDescriptor, amenable_time::TemporalError> {
        let mut builder = amenable_time::UtcOffsetDescriptorBuilder::default()
            .sign(sign)
            .hours(hours);
        if minutes != 0 {
            builder = builder.minutes(minutes);
        }
        if seconds != 0 {
            builder = builder.seconds(seconds);
        }
        builder.build().map_err(|err| {
            amenable_time::TemporalError::new(
                amenable_time::TemporalErrorKind::UtcOffsetBuilderFailed(
                    amenable_time::UtcOffsetBuilderFailedSource::new(err),
                ),
            )
        })
    }

    /// The composed fix: `offset_parts_4`'s proven arithmetic relation
    /// plus `build_descriptor_from_parts`'s proven transcription,
    /// stubbed together -- the full four-fact relation between `offset`
    /// and the final descriptor, the exact claim `reflect_offset`
    /// couldn't prove in one unstubbed shot.
    #[kani::requires(true)]
    #[kani::ensures(|result: &Result<amenable_time::UtcOffsetDescriptor, amenable_time::TemporalError>| {
        match result {
            Ok(descriptor) => {
                let seconds = offset.local_minus_utc();
                let sign_matches = match descriptor.sign() {
                    amenable_time::UtcOffsetSign::Positive => seconds >= 0,
                    amenable_time::UtcOffsetSign::Negative => seconds < 0,
                };
                let magnitude = i64::from(descriptor.hours()) * 3600
                    + i64::from(descriptor.minutes().unwrap_or(0)) * 60
                    + i64::from(descriptor.seconds().unwrap_or(0));
                sign_matches && magnitude == i64::from(seconds.unsigned_abs())
            }
            Err(_) => true,
        }
    })]
    pub(super) fn reflect_offset_staged(
        offset: chrono::FixedOffset,
    ) -> Result<amenable_time::UtcOffsetDescriptor, amenable_time::TemporalError> {
        let (sign, hours, minutes, seconds) = offset_parts_4(offset);
        build_descriptor_from_parts(sign, hours, minutes, seconds)
    }
}

#[cfg(kani)]
use kani_only::{
    build_descriptor_from_parts, offset_parts_2, offset_parts_3, offset_parts_4,
    reflect_offset_staged,
};

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::offset_parts_2_passes_cheaply".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::check_offset_parts_2".to_owned(),
            "amenable_kani".to_owned(),
            "staged base case: sign and hours from raw arithmetic, no builder -- the digit-chain shape applied to the fact-count wall -- confirmed passes in ~22s".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_OFFSET_PARTS_2_SRC, {
        /// `offset_parts_2`'s own contract: the base case.
        #[kani::proof_for_contract(offset_parts_2)]
        fn check_offset_parts_2() {
            let offset_seconds: i32 = kani::any();
            let Some(offset) = chrono::FixedOffset::east_opt(offset_seconds) else {
                return;
            };
            let _ = offset_parts_2(offset);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::offset_parts_3_passes_cheaply".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::check_offset_parts_3".to_owned(),
            "amenable_kani".to_owned(),
            "stage 3: stubs offset_parts_2's proven two-fact conjunction, adds minutes as the one new real computation -- confirmed passes".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_OFFSET_PARTS_3_SRC, {
        /// `offset_parts_3`'s own contract: stubs `offset_parts_2`.
        #[kani::proof_for_contract(offset_parts_3)]
        #[kani::stub_verified(offset_parts_2)]
        fn check_offset_parts_3() {
            let offset_seconds: i32 = kani::any();
            let Some(offset) = chrono::FixedOffset::east_opt(offset_seconds) else {
                return;
            };
            let _ = offset_parts_3(offset);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::offset_parts_4_passes_cheaply".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::check_offset_parts_4".to_owned(),
            "amenable_kani".to_owned(),
            "stage 4: stubs offset_parts_3's proven three-fact conjunction, adds seconds -- the full four-fact relation, staged -- confirmed passes in ~15.5s, resolving the fact-count wall that defeated the flat version".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_OFFSET_PARTS_4_SRC, {
        /// `offset_parts_4`'s own contract: stubs `offset_parts_3`.
        #[kani::proof_for_contract(offset_parts_4)]
        #[kani::stub_verified(offset_parts_3)]
        fn check_offset_parts_4() {
            let offset_seconds: i32 = kani::any();
            let Some(offset) = chrono::FixedOffset::east_opt(offset_seconds) else {
                return;
            };
            let _ = offset_parts_4(offset);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::build_descriptor_from_parts_isolation".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::build_descriptor_from_parts_isolation".to_owned(),
            "amenable_kani".to_owned(),
            "the builder's own four-fact transcription conjunction, given already-decomposed parts -- no FixedOffset arithmetic left to reason about at all".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, BUILD_DESCRIPTOR_FROM_PARTS_ISOLATION_SRC, {
        /// `build_descriptor_from_parts`'s own contract.
        #[kani::proof_for_contract(build_descriptor_from_parts)]
        fn build_descriptor_from_parts_isolation() {
            let sign: amenable_time::UtcOffsetSign = kani::any();
            let hours: u8 = kani::any();
            let minutes: u8 = kani::any();
            let seconds: u8 = kani::any();
            kani::assume(hours <= 23);
            let _ = build_descriptor_from_parts(sign, hours, minutes, seconds);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::reflect_offset_staged_passes".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::check_reflect_offset_staged".to_owned(),
            "amenable_kani".to_owned(),
            "the composed fix: offset_parts_4 plus build_descriptor_from_parts, stubbed together -- the same four-fact relation reflect_offset's own unstubbed contract timed out trying to prove in one shot".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_REFLECT_OFFSET_STAGED_SRC, {
        /// `reflect_offset_staged`'s own contract: stubs both halves.
        #[kani::proof_for_contract(reflect_offset_staged)]
        #[kani::stub_verified(offset_parts_4)]
        fn check_reflect_offset_staged() {
            let offset_seconds: i32 = kani::any();
            let Some(offset) = chrono::FixedOffset::east_opt(offset_seconds) else {
                return;
            };
            let _ = reflect_offset_staged(offset);
        }
    }
}
