//! The next mirror up from `gallery::narrow_to_u8_mirror`: the actual
//! parked target, `amenable_ext::chrono::
//! fixed_offset_to_utc_offset_descriptor`, which calls `narrow_to_u8`
//! up to three times (hours, then conditionally minutes and seconds)
//! with real `?`-propagation between them, then builds a
//! `UtcOffsetDescriptor` via a fallible builder on a fourth error
//! path. Every one of those four error sites reaches the same
//! `Box<TemporalErrorKind>` wrapping a `String`-bearing source.
//!
//! This mirrors the real function's *control flow* almost exactly
//! (three conditional narrowing calls, early-return via `?`), not
//! just its data shape -- the real function's own `ensures` already
//! states the claim this chases: on success, sign and magnitude
//! reconstruct exactly; on error, no claim (nothing inspects content).
//! The success value is the raw `(UtcOffsetSign, u8, u8, u8)` tuple
//! rather than a real `UtcOffsetDescriptor` (a third-party builder
//! type not worth mirroring for this question), matching `gallery::
//! chrono_offset_date_time_round_trip`'s own `offset_parts_4`
//! convention -- but unlike that earlier pure-arithmetic proof (which
//! skipped error machinery entirely, confirmed cheap on its own), this
//! one goes through the real control-flow shape, calling `gallery::
//! narrow_to_u8_mirror::mirror_narrow_to_u8` (stubbed) at each site,
//! testing whether the arithmetic success path and the now-cheap
//! error-path machinery stay cheap *together*, in the real shape --
//! which was never confirmed before now (prior attempts used either
//! pure arithmetic with no error type at all, or the real error
//! machinery, which OOM'd).
//!
//! **What was actually confirmed:** both variants pass. Unstubbed
//! (58.4s, 246/246 checks) -- the real target's exact control-flow
//! shape, three conditional narrowing calls with real
//! `?`-propagation, all inlined, is cheap on its own once the proxy
//! swap is applied. This is the actual parked wall resolved: this
//! exact shape, with the real `String`/`Box<TemporalErrorKind>`
//! machinery, never finished. Stubbed (112.6s, 250/250 checks) also
//! passes, but is slower, not faster -- a real, worth-recording
//! finding: `stub_verified`'s own postcondition-checking overhead
//! cost more here than it saved, since the inlined body was already
//! cheap. Stubbing is a tool for when the inlined cost is the actual
//! problem (as it was for `narrow_to_u8`'s own real version, chained
//! three times under DFCC), not something to reach for by default.

mod kani_only {
    #![cfg(kani)]

    use amenable_time::UtcOffsetSign;

    use crate::gallery::narrow_to_u8_mirror::MirrorTemporalError;
    use crate::gallery::narrow_to_u8_mirror::mirror_narrow_to_u8;

    /// Mirrors `fixed_offset_to_utc_offset_descriptor`'s real control
    /// flow: one unconditional narrowing call, then two conditional
    /// ones, each with real `?`-propagation. `sign`/`magnitude` stand
    /// in for `fixed_offset_sign_and_magnitude`'s own already-cheap
    /// output (confirmed separately, `amenable_kani::chrono::
    /// offset_date_time::check_fixed_offset_sign_and_magnitude`) --
    /// not re-derived here, since that function's own cost was never
    /// in question.
    #[kani::requires(true)]
    #[kani::ensures(|result: &Result<(UtcOffsetSign, u8, u8, u8), MirrorTemporalError>| {
        match result {
            Ok((result_sign, hours, minutes, seconds)) => {
                let sign_matches = *result_sign == sign;
                let computed_magnitude = u32::from(*hours) * 3600
                    + u32::from(*minutes) * 60
                    + u32::from(*seconds);
                sign_matches && computed_magnitude == magnitude
            }
            Err(_) => true,
        }
    })]
    pub(super) fn mirror_fixed_offset_to_utc_offset_descriptor(
        sign: UtcOffsetSign,
        magnitude: u32,
    ) -> Result<(UtcOffsetSign, u8, u8, u8), MirrorTemporalError> {
        let hours = mirror_narrow_to_u8(magnitude / 3600, "hours")?;
        let remainder_minutes = (magnitude % 3600) / 60;
        let remainder_seconds = magnitude % 60;

        let minutes = if remainder_minutes != 0 {
            mirror_narrow_to_u8(remainder_minutes, "minutes")?
        } else {
            0
        };
        let seconds = if remainder_seconds != 0 {
            mirror_narrow_to_u8(remainder_seconds, "seconds")?
        } else {
            0
        };

        Ok((sign, hours, minutes, seconds))
    }
}

#[cfg(kani)]
use kani_only::mirror_fixed_offset_to_utc_offset_descriptor;
#[cfg(kani)]
use crate::gallery::narrow_to_u8_mirror::mirror_narrow_to_u8;

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::fixed_offset_descriptor_mirror::mirror_fixed_offset_to_utc_offset_descriptor_unstubbed".to_owned(),
            "gallery::fixed_offset_descriptor_mirror::check_mirror_fixed_offset_to_utc_offset_descriptor_unstubbed".to_owned(),
            "amenable_kani".to_owned(),
            "confirmed PASSES, 58.4s, 246/246 checks -- the real target's own full control-flow shape (three conditional narrowing calls, real ?-propagation), inlining mirror_narrow_to_u8 at every site, no stub_verified at all. The actual parked wall, resolved: this exact shape never finished with the real String/Box<TemporalErrorKind> machinery.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_MIRROR_FIXED_OFFSET_TO_UTC_OFFSET_DESCRIPTOR_UNSTUBBED_SRC, {
        /// Inlines `mirror_narrow_to_u8` at all three call sites --
        /// no `stub_verified`. Tests whether the real function's full
        /// control-flow shape is cheap even without composing on an
        /// already-proven piece.
        #[kani::proof_for_contract(mirror_fixed_offset_to_utc_offset_descriptor)]
        fn check_mirror_fixed_offset_to_utc_offset_descriptor_unstubbed() {
            let sign: amenable_time::UtcOffsetSign = kani::any();
            let magnitude: u32 = kani::any();
            kani::assume(magnitude <= 86_399);
            let _ = mirror_fixed_offset_to_utc_offset_descriptor(sign, magnitude);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::fixed_offset_descriptor_mirror::mirror_fixed_offset_to_utc_offset_descriptor_stubbed".to_owned(),
            "gallery::fixed_offset_descriptor_mirror::check_mirror_fixed_offset_to_utc_offset_descriptor_stubbed".to_owned(),
            "amenable_kani".to_owned(),
            "confirmed PASSES, 112.6s, 250/250 checks -- slower than the unstubbed version (58.4s), not faster. Worth recording honestly: stub_verified's own postcondition-checking overhead, applied three times here, cost more than it saved once the inlined body was already cheap on its own. Stubbing isn't free; it's a tool for when the inlined cost is the actual problem, not a default to reach for.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_MIRROR_FIXED_OFFSET_TO_UTC_OFFSET_DESCRIPTOR_STUBBED_SRC, {
        /// The same claim, this time stubbing the already-proven
        /// `mirror_narrow_to_u8` at all three call sites -- the real
        /// composability test for this specific real-function shape.
        #[kani::proof_for_contract(mirror_fixed_offset_to_utc_offset_descriptor)]
        #[kani::stub_verified(mirror_narrow_to_u8)]
        fn check_mirror_fixed_offset_to_utc_offset_descriptor_stubbed() {
            let sign: amenable_time::UtcOffsetSign = kani::any();
            let magnitude: u32 = kani::any();
            kani::assume(magnitude <= 86_399);
            let _ = mirror_fixed_offset_to_utc_offset_descriptor(sign, magnitude);
        }
    }
}
