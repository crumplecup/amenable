//! Realize-direction counterpart to `chrono_offset_date_time_round_trip_
//! offset_staged_fix.rs` -- see that file and `chrono_offset_date_time_
//! round_trip.rs`'s own module doc for the full narrative.
//!
//! The reflect direction's staged fix (`offset_parts_2/3/4` plus
//! `build_descriptor_from_parts`, stubbing only the arithmetic chain and
//! leaving the builder transcription inlined) has since landed in real
//! production code: `amenable_ext::offset_descriptor_parts_2/3/4` plus
//! `build_utc_offset_descriptor_from_parts`, and `check_fixed_offset_to_
//! utc_offset_descriptor` (in `amenable_kani::chrono::offset_date_time`)
//! now passes with exactly that shape.
//!
//! The realize direction (`amenable_ext::utc_offset_descriptor_to_fixed_
//! offset`) got the same treatment: `signed_seconds_parts_2/3/4` (the
//! signed-seconds total, built up incrementally) plus `build_fixed_
//! offset_from_signed_seconds` (the one `FixedOffset::east_opt` call,
//! isolated). Each stage proves standalone, including the final build
//! step alone. But mirroring the reflect direction's exact pattern --
//! stub the arithmetic chain, inline the final step -- still times out
//! for the composed claim, confirmed directly against the real
//! production functions (not a reimplementation): `check_utc_offset_
//! descriptor_to_fixed_offset`'s own harness, stubbing `signed_seconds_
//! parts_4` and inlining `build_fixed_offset_from_signed_seconds`, fails
//! with `Runtime Symex: 133.816s`, `4816 VCC(s)` -- a genuine timeout,
//! not a compile artifact.
//!
//! This file isolates *why* the asymmetry exists before anything more
//! goes near production code again.

mod kani_only {
    #![cfg(kani)]

    // ── Hypothesis B: a refinement-type newtype, stubbing the final
    // step instead of inlining it ──
    //
    // `chrono::FixedOffset` can't get a manual `kani::Arbitrary` impl
    // directly -- both the trait and the type are foreign, so the
    // orphan rule blocks it in this crate. A local newtype wrapping
    // it isn't foreign, though, and if its own `any()` is proven to
    // range over *exactly* the real type's domain (not a subset, not
    // a superset), whatever a proof establishes about the wrapper
    // transfers to the real type it refines.
    //
    // Confirmed via chrono's own source (`fixed.rs:62-68`):
    // `FixedOffset::east_opt` succeeds iff `-86_400 < secs && secs <
    // 86_400`, i.e. exactly `secs.unsigned_abs() <= 86_399` -- and
    // `local_minus_utc` is a private field with `east_opt`/`west_opt`
    // the only constructors, so this is the type's *complete* domain,
    // not an approximation.
    //
    // `any()` is composed from the already-proven-cheap
    // `signed_seconds_parts_4` chain (sign + hours<=23 + minutes<=59
    // + seconds<=59) rather than a wide `kani::any::<i32>()` +
    // `assume` -- that mixed-radix decomposition is a bijection onto
    // `0..=86399` (every magnitude has a unique hours/minutes/seconds
    // triple), so every real offset is reachable, with no corners cut
    // and no wide-space-then-filter pattern reintroducing the exact
    // kind of cost this whole investigation has been fighting.
    #[derive(derive_new::new)]
    pub(super) struct RefinedFixedOffset(chrono::FixedOffset);

    impl kani::Arbitrary for RefinedFixedOffset {
        fn any() -> Self {
            let sign: amenable_time::UtcOffsetSign = kani::any();
            let hours: u8 = kani::any();
            kani::assume(hours <= 23);
            let minutes: u8 = kani::any();
            kani::assume(minutes <= 59);
            let seconds: u8 = kani::any();
            kani::assume(seconds <= 59);
            let signed_seconds =
                amenable_ext::signed_seconds_parts_4(sign, hours, minutes, seconds);
            Self::new(
                chrono::FixedOffset::east_opt(signed_seconds)
                    .expect("bounded by signed_seconds_parts_4's own proven range"),
            )
        }
    }

    /// Mirrors `build_fixed_offset_from_signed_seconds` exactly, but
    /// returns the newtype directly -- no `Result`, so no
    /// `TemporalError`-`Arbitrary` problem either -- making it
    /// directly `stub_verified`-able.
    #[kani::requires(seconds.unsigned_abs() <= 86_399)]
    #[kani::ensures(|result: &RefinedFixedOffset| result.0.local_minus_utc() == seconds)]
    pub(super) fn build_refined_fixed_offset_in_range(seconds: i32) -> RefinedFixedOffset {
        RefinedFixedOffset::new(
            chrono::FixedOffset::east_opt(seconds).expect("bounded by requires above"),
        )
    }

    /// The composed realize claim, stubbing *both* the arithmetic
    /// chain and the final build step -- the full "lean on stubs, not
    /// inlining" shape, mirroring the reflect direction's own
    /// composed fix exactly.
    #[kani::requires(hours <= 23 && minutes <= 59 && seconds <= 59)]
    #[kani::ensures(|result: &Result<chrono::FixedOffset, amenable_time::TemporalError>| {
        match result {
            Ok(offset) => {
                let magnitude =
                    i32::from(hours) * 3600 + i32::from(minutes) * 60 + i32::from(seconds);
                let expected = match sign {
                    amenable_time::UtcOffsetSign::Positive => magnitude,
                    amenable_time::UtcOffsetSign::Negative => -magnitude,
                };
                offset.local_minus_utc() == expected
            }
            Err(_) => false,
        }
    })]
    pub(super) fn realize_staged_stubbed(
        sign: amenable_time::UtcOffsetSign,
        hours: u8,
        minutes: u8,
        seconds: u8,
    ) -> Result<chrono::FixedOffset, amenable_time::TemporalError> {
        let signed_seconds = amenable_ext::signed_seconds_parts_4(sign, hours, minutes, seconds);
        Ok(build_refined_fixed_offset_in_range(signed_seconds).0)
    }

    // ── Which of the two remaining real differences is the cost? ──
    //
    // Removing `UtcOffsetDescriptorBuilder` from the production
    // harness (giving `UtcOffsetDescriptor` its own `Arbitrary` impl
    // instead) changed nothing: 104.119s/4760 VCCs vs 102.666s/4971
    // VCCs before -- within noise. That hypothesis is refuted. Two
    // real differences remain between `realize_staged_stubbed` (passes)
    // and the still-failing production proof: the relationship early-
    // return branch, and reading inputs via getters on a struct
    // instead of as bare arguments. Testing each alone, not together.

    /// Adds *only* the relationship branch to `realize_staged_stubbed`
    /// -- still bare primitive arguments, no descriptor/getters at all.
    #[kani::requires(
        relationship == amenable_time::UtcOffsetRelationship::Known
            && hours <= 23
            && minutes <= 59
            && seconds <= 59
    )]
    #[kani::ensures(|result: &Result<chrono::FixedOffset, amenable_time::TemporalError>| {
        match result {
            Ok(offset) => {
                let magnitude =
                    i32::from(hours) * 3600 + i32::from(minutes) * 60 + i32::from(seconds);
                let expected = match sign {
                    amenable_time::UtcOffsetSign::Positive => magnitude,
                    amenable_time::UtcOffsetSign::Negative => -magnitude,
                };
                offset.local_minus_utc() == expected
            }
            Err(_) => false,
        }
    })]
    pub(super) fn realize_staged_stubbed_with_relationship_check(
        relationship: amenable_time::UtcOffsetRelationship,
        sign: amenable_time::UtcOffsetSign,
        hours: u8,
        minutes: u8,
        seconds: u8,
    ) -> Result<chrono::FixedOffset, amenable_time::TemporalError> {
        if relationship == amenable_time::UtcOffsetRelationship::UnknownLocalOffset {
            return Err(amenable_time::TemporalError::new(
                amenable_time::TemporalErrorKind::Unsupported(
                    amenable_time::UnsupportedSource::closed(
                        amenable_time::UnsupportedCase::ChronoUnknownLocalOffset,
                    ),
                ),
            ));
        }
        let signed_seconds = amenable_ext::signed_seconds_parts_4(sign, hours, minutes, seconds);
        Ok(build_refined_fixed_offset_in_range(signed_seconds).0)
    }

    /// Adds *only* the struct/getter layer to `realize_staged_stubbed`
    /// -- a real `UtcOffsetDescriptor` (via its own `Arbitrary` impl),
    /// read through its getters, but no relationship branch at all.
    #[kani::requires(
        descriptor.hours() <= 23
            && descriptor.minutes().unwrap_or(0) <= 59
            && descriptor.seconds().unwrap_or(0) <= 59
    )]
    #[kani::ensures(|result: &Result<chrono::FixedOffset, amenable_time::TemporalError>| {
        match result {
            Ok(offset) => {
                let magnitude = i32::from(descriptor.hours()) * 3600
                    + i32::from(descriptor.minutes().unwrap_or(0)) * 60
                    + i32::from(descriptor.seconds().unwrap_or(0));
                let expected = match descriptor.sign() {
                    amenable_time::UtcOffsetSign::Positive => magnitude,
                    amenable_time::UtcOffsetSign::Negative => -magnitude,
                };
                offset.local_minus_utc() == expected
            }
            Err(_) => false,
        }
    })]
    pub(super) fn realize_staged_stubbed_via_descriptor_no_branch(
        descriptor: amenable_time::UtcOffsetDescriptor,
    ) -> Result<chrono::FixedOffset, amenable_time::TemporalError> {
        let signed_seconds = amenable_ext::signed_seconds_parts_4(
            descriptor.sign(),
            descriptor.hours(),
            descriptor.minutes().unwrap_or(0),
            descriptor.seconds().unwrap_or(0),
        );
        Ok(build_refined_fixed_offset_in_range(signed_seconds).0)
    }

    /// All three combined: relationship branch, struct/getters, AND
    /// the stubbed arithmetic -- exactly `utc_offset_descriptor_to_
    /// fixed_offset`'s own body shape, reconstructed here to confirm
    /// the compounding precisely, isolated from anything else that
    /// might differ between this reconstruction and the real function.
    #[kani::requires(
        descriptor.relationship() == amenable_time::UtcOffsetRelationship::Known
            && descriptor.hours() <= 23
            && descriptor.minutes().unwrap_or(0) <= 59
            && descriptor.seconds().unwrap_or(0) <= 59
    )]
    #[kani::ensures(|result: &Result<chrono::FixedOffset, amenable_time::TemporalError>| {
        match result {
            Ok(offset) => {
                let magnitude = i32::from(descriptor.hours()) * 3600
                    + i32::from(descriptor.minutes().unwrap_or(0)) * 60
                    + i32::from(descriptor.seconds().unwrap_or(0));
                let expected = match descriptor.sign() {
                    amenable_time::UtcOffsetSign::Positive => magnitude,
                    amenable_time::UtcOffsetSign::Negative => -magnitude,
                };
                offset.local_minus_utc() == expected
            }
            Err(_) => false,
        }
    })]
    pub(super) fn realize_staged_stubbed_via_descriptor_with_branch(
        descriptor: amenable_time::UtcOffsetDescriptor,
    ) -> Result<chrono::FixedOffset, amenable_time::TemporalError> {
        if descriptor.relationship() == amenable_time::UtcOffsetRelationship::UnknownLocalOffset {
            return Err(amenable_time::TemporalError::new(
                amenable_time::TemporalErrorKind::Unsupported(
                    amenable_time::UnsupportedSource::closed(
                        amenable_time::UnsupportedCase::ChronoUnknownLocalOffset,
                    ),
                ),
            ));
        }
        let signed_seconds = amenable_ext::signed_seconds_parts_4(
            descriptor.sign(),
            descriptor.hours(),
            descriptor.minutes().unwrap_or(0),
            descriptor.seconds().unwrap_or(0),
        );
        Ok(build_refined_fixed_offset_in_range(signed_seconds).0)
    }

    // ── Hypothesis A: is it the `UtcOffsetDescriptor`/builder layer? ──
    //
    // The production harness constructs its input via `UtcOffsetDescriptor
    // Builder` before calling the real function -- unavoidable harness
    // setup, but still real, unstubbed code running inside the same
    // proof. This raw-primitives version skips the descriptor/builder
    // entirely, calling the already-proven-standalone production stages
    // directly, to isolate whether *that* layer is the compounding cost
    // or whether it's genuinely `build_fixed_offset_from_signed_seconds`
    // under already-stubbed symbolic state.
    #[kani::requires(hours <= 23 && minutes <= 59 && seconds <= 59)]
    #[kani::ensures(|result: &Result<chrono::FixedOffset, amenable_time::TemporalError>| {
        match result {
            Ok(offset) => {
                let magnitude =
                    i32::from(hours) * 3600 + i32::from(minutes) * 60 + i32::from(seconds);
                let expected = match sign {
                    amenable_time::UtcOffsetSign::Positive => magnitude,
                    amenable_time::UtcOffsetSign::Negative => -magnitude,
                };
                offset.local_minus_utc() == expected
            }
            Err(_) => false,
        }
    })]
    pub(super) fn realize_staged_raw(
        sign: amenable_time::UtcOffsetSign,
        hours: u8,
        minutes: u8,
        seconds: u8,
    ) -> Result<chrono::FixedOffset, amenable_time::TemporalError> {
        let signed_seconds = amenable_ext::signed_seconds_parts_4(sign, hours, minutes, seconds);
        amenable_ext::build_fixed_offset_from_signed_seconds(signed_seconds)
    }
}

#[cfg(kani)]
use kani_only::{
    build_refined_fixed_offset_in_range, realize_staged_raw, realize_staged_stubbed,
    realize_staged_stubbed_via_descriptor_no_branch,
    realize_staged_stubbed_via_descriptor_with_branch, realize_staged_stubbed_with_relationship_check,
};

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::build_refined_fixed_offset_in_range_isolation".to_owned(),
            "gallery::chrono_offset_date_time_round_trip_realize_staged_fix::check_build_refined_fixed_offset_in_range".to_owned(),
            "amenable_kani".to_owned(),
            "the refinement-type newtype's infallible core, in isolation -- just the one east_opt call, returning the newtype directly so it can be stub_verified".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_BUILD_REFINED_FIXED_OFFSET_IN_RANGE_SRC, {
        /// `build_refined_fixed_offset_in_range`'s own contract.
        #[kani::proof_for_contract(build_refined_fixed_offset_in_range)]
        fn check_build_refined_fixed_offset_in_range() {
            let seconds: i32 = kani::any();
            kani::assume(seconds.unsigned_abs() <= 86_399);
            let _ = build_refined_fixed_offset_in_range(seconds);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::realize_staged_relationship_check_isolation".to_owned(),
            "gallery::chrono_offset_date_time_round_trip_realize_staged_fix::check_realize_staged_stubbed_with_relationship_check".to_owned(),
            "amenable_kani".to_owned(),
            "adds ONLY the relationship early-return branch to the already-passing realize_staged_stubbed, still bare primitive arguments -- isolates whether the branch alone reintroduces the production timeout".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_REALIZE_STAGED_STUBBED_WITH_RELATIONSHIP_CHECK_SRC, {
        /// `realize_staged_stubbed_with_relationship_check`'s own
        /// contract.
        #[kani::proof_for_contract(realize_staged_stubbed_with_relationship_check)]
        #[kani::stub_verified(amenable_ext::signed_seconds_parts_4)]
        #[kani::stub_verified(build_refined_fixed_offset_in_range)]
        fn check_realize_staged_stubbed_with_relationship_check() {
            let relationship: amenable_time::UtcOffsetRelationship = kani::any();
            let sign: amenable_time::UtcOffsetSign = kani::any();
            let hours: u8 = kani::any();
            kani::assume(hours <= 23);
            let minutes: u8 = kani::any();
            kani::assume(minutes <= 59);
            let seconds: u8 = kani::any();
            kani::assume(seconds <= 59);
            let _ = realize_staged_stubbed_with_relationship_check(
                relationship,
                sign,
                hours,
                minutes,
                seconds,
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::realize_staged_via_descriptor_no_branch_isolation".to_owned(),
            "gallery::chrono_offset_date_time_round_trip_realize_staged_fix::check_realize_staged_stubbed_via_descriptor_no_branch".to_owned(),
            "amenable_kani".to_owned(),
            "adds ONLY the struct/getter layer to the already-passing realize_staged_stubbed -- a real UtcOffsetDescriptor via its own Arbitrary impl, read through getters, no relationship branch -- isolates whether that layer alone reintroduces the production timeout".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_REALIZE_STAGED_STUBBED_VIA_DESCRIPTOR_NO_BRANCH_SRC, {
        /// `realize_staged_stubbed_via_descriptor_no_branch`'s own
        /// contract.
        #[kani::proof_for_contract(realize_staged_stubbed_via_descriptor_no_branch)]
        #[kani::stub_verified(amenable_ext::signed_seconds_parts_4)]
        #[kani::stub_verified(build_refined_fixed_offset_in_range)]
        fn check_realize_staged_stubbed_via_descriptor_no_branch() {
            let descriptor: amenable_time::UtcOffsetDescriptor = kani::any();
            let _ = realize_staged_stubbed_via_descriptor_no_branch(descriptor);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::realize_staged_via_descriptor_with_branch_confirms_compounding".to_owned(),
            "gallery::chrono_offset_date_time_round_trip_realize_staged_fix::check_realize_staged_stubbed_via_descriptor_with_branch".to_owned(),
            "amenable_kani".to_owned(),
            "all three combined (relationship branch + struct/getters + stubbed arithmetic), reconstructing utc_offset_descriptor_to_fixed_offset's own body shape exactly -- confirms whether the compounding is specifically the three together, isolated from anything else that might differ in the real function".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_REALIZE_STAGED_STUBBED_VIA_DESCRIPTOR_WITH_BRANCH_SRC, {
        /// `realize_staged_stubbed_via_descriptor_with_branch`'s own
        /// contract.
        #[kani::proof_for_contract(realize_staged_stubbed_via_descriptor_with_branch)]
        #[kani::stub_verified(amenable_ext::signed_seconds_parts_4)]
        #[kani::stub_verified(build_refined_fixed_offset_in_range)]
        fn check_realize_staged_stubbed_via_descriptor_with_branch() {
            let descriptor: amenable_time::UtcOffsetDescriptor = kani::any();
            let _ = realize_staged_stubbed_via_descriptor_with_branch(descriptor);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::realize_staged_stubbed_resolves_the_wall".to_owned(),
            "gallery::chrono_offset_date_time_round_trip_realize_staged_fix::check_realize_staged_stubbed".to_owned(),
            "amenable_kani".to_owned(),
            "the composed realize claim stubbing BOTH the arithmetic chain (signed_seconds_parts_4) and the final build step (via the RefinedFixedOffset newtype), the full 'lean on stubs, not inlining' shape -- tests whether this is what the production composed proof needs".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_REALIZE_STAGED_STUBBED_SRC, {
        /// `realize_staged_stubbed`'s own contract: stubs both halves.
        #[kani::proof_for_contract(realize_staged_stubbed)]
        #[kani::stub_verified(amenable_ext::signed_seconds_parts_4)]
        #[kani::stub_verified(build_refined_fixed_offset_in_range)]
        fn check_realize_staged_stubbed() {
            let sign: amenable_time::UtcOffsetSign = kani::any();
            let hours: u8 = kani::any();
            kani::assume(hours <= 23);
            let minutes: u8 = kani::any();
            kani::assume(minutes <= 59);
            let seconds: u8 = kani::any();
            kani::assume(seconds <= 59);
            let _ = realize_staged_stubbed(sign, hours, minutes, seconds);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::realize_staged_raw_isolates_descriptor_layer".to_owned(),
            "gallery::chrono_offset_date_time_round_trip_realize_staged_fix::check_realize_staged_raw".to_owned(),
            "amenable_kani".to_owned(),
            "mirrors the production composed realize proof exactly (stub signed_seconds_parts_4, inline build_fixed_offset_from_signed_seconds) but skips UtcOffsetDescriptor/its builder entirely -- isolates whether the descriptor/builder layer or the inlined build step is the compounding cost".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_REALIZE_STAGED_RAW_SRC, {
        /// `realize_staged_raw`'s own contract: stubs the arithmetic
        /// chain, inlines the final build step -- the exact shape of
        /// the production composed proof, minus the descriptor/builder
        /// layer.
        #[kani::proof_for_contract(realize_staged_raw)]
        #[kani::stub_verified(amenable_ext::signed_seconds_parts_4)]
        fn check_realize_staged_raw() {
            let sign: amenable_time::UtcOffsetSign = kani::any();
            let hours: u8 = kani::any();
            kani::assume(hours <= 23);
            let minutes: u8 = kani::any();
            kani::assume(minutes <= 59);
            let seconds: u8 = kani::any();
            kani::assume(seconds <= 59);
            let _ = realize_staged_raw(sign, hours, minutes, seconds);
        }
    }
}
