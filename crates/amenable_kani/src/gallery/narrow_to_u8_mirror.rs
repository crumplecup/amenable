//! Applies the proxy tactic (`gallery::boxed_error_variant_count`'s
//! `proxy_tactic_isolation`) to a mirror of the actual parked wall:
//! `amenable_ext::chrono::narrow_to_u8(value: u32, context:
//! &'static str) -> Result<u8, TemporalError>`, whose real `Err` arm
//! reaches `Box<TemporalErrorKind>` wrapping a `String`-bearing
//! source (confirmed OOM, `gallery::boxed_error_variant_count`'s
//! whole investigation).
//!
//! `narrow_to_u8`'s own contract never needs string content at all --
//! on success the narrowed value matches; on failure, `value`
//! genuinely didn't fit in a `u8`. Nothing downstream inspects the
//! error's detail (confirmed directly elsewhere this session: every
//! `TemporalErrorKind::` reference in this codebase is a constructor
//! call, never a `match`). So the mirror's `Err` type needs no
//! correspondence to the real `TemporalError`'s *content* -- only to
//! be cheap, and to carry a value-level contract identical in shape
//! to what the real function would have.
//!
//! The other half of this file is the modularity claim: a cheap
//! mirror function, once proven, is a legitimate `stub_verified`
//! target for another function -- exactly the composability
//! `gallery::string_depth_induction`'s `extend_by_one_char`/
//! `extend_by_two_chars` already demonstrated for a different
//! property. Here it's demonstrated for a mirror of a *real*
//! production function's contract, not a toy property.
//!
//! **What was actually confirmed:** all three pieces pass genuinely.
//! `mirror_narrow_to_u8`'s own contract (1.6s, 139/139 checks) holds
//! -- the real function's intended value-level claim, proven cheaply.
//! The exact call pattern that motivated this whole investigation --
//! three narrowing calls under one trivially-contracted caller --
//! now costs 58.2s/144 checks instead of never finishing. And the
//! modularity claim holds too (18.3s, 148/148 checks): a *different*
//! function can `stub_verified` the mirror rather than inlining it --
//! the same composability `gallery::string_depth_induction` already
//! proved for a toy property, now shown for a mirror of a real
//! production function's contract. (One real compile-time gap along
//! the way, not a proof issue: `MirrorTemporalError` needed its own
//! `kani::Arbitrary` impl before it could be a `stub_verified`
//! target's return type -- forgotten initially, free once added,
//! since its only field already has `Arbitrary`.)

mod kani_only {
    #![cfg(kani)]

    use crate::KaniUtf8Buffer;

    /// Cheap stand-in for `TemporalError`'s relevant shape: a boxed
    /// kind wrapping a buffer-backed detail, matching `gallery::
    /// boxed_error_variant_count::ProxyError`'s already-confirmed-
    /// cheap structure. `context` (the real function's static detail
    /// message) is deliberately never stored -- nothing downstream
    /// reads it, matching `caller_location()`'s own established
    /// "ignore real arguments under kani, construct something
    /// symbolic instead" pattern in `amenable_time::error`.
    pub(crate) struct MirrorTemporalError {
        detail: Box<KaniUtf8Buffer<8>>,
    }

    impl MirrorTemporalError {
        fn new() -> Self {
            Self {
                detail: Box::new(kani::any()),
            }
        }
    }

    /// Needed for `MirrorTemporalError` to be usable as part of a
    /// `stub_verified` target's return type (`Result<u8,
    /// MirrorTemporalError>`) -- free, since its only field,
    /// `Box<KaniUtf8Buffer<8>>`, already has `Arbitrary` (`Box<T>:
    /// Arbitrary` where `T: Arbitrary` is a standard Kani blanket
    /// impl).
    impl kani::Arbitrary for MirrorTemporalError {
        fn any() -> Self {
            Self { detail: kani::any() }
        }
    }

    /// Mirrors `narrow_to_u8`'s real signature and intended contract
    /// exactly: on success, the narrowed value round-trips; on
    /// failure, `value` genuinely exceeded `u8::MAX`. No claim about
    /// the error's content -- there is none to make.
    #[kani::requires(true)]
    #[kani::ensures(|result: &Result<u8, MirrorTemporalError>| match result {
        Ok(v) => u32::from(*v) == value,
        Err(_) => value > u32::from(u8::MAX),
    })]
    pub(crate) fn mirror_narrow_to_u8(
        value: u32,
        _context: &'static str,
    ) -> Result<u8, MirrorTemporalError> {
        u8::try_from(value).map_err(|_err| MirrorTemporalError::new())
    }

    /// The real function's own actual call pattern: three narrowing
    /// calls (hours, minutes, seconds) in one trivially-contracted
    /// caller -- the exact shape that made the real, unmirrored
    /// `narrow_to_u8` time out when combined under DFCC
    /// (`gallery::chrono_offset_date_time_round_trip`'s own
    /// `call_narrow_to_u8_thrice_contracted_isolation`).
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn call_mirror_narrow_to_u8_thrice_contracted() {
        let hours: u32 = kani::any();
        let minutes: u32 = kani::any();
        let seconds: u32 = kani::any();
        let _ = mirror_narrow_to_u8(hours, "hours");
        let _ = mirror_narrow_to_u8(minutes, "minutes");
        let _ = mirror_narrow_to_u8(seconds, "seconds");
    }

    /// The modularity claim: a *different* function, stubbing
    /// `mirror_narrow_to_u8` rather than inlining it -- demonstrating
    /// the mirror is a legitimate `stub_verified` target other
    /// mirrors can build on, not just a proof that terminates in
    /// isolation.
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn mirror_caller_stubbing_narrow_to_u8() {
        let value: u32 = kani::any();
        let _ = mirror_narrow_to_u8(value, "hours");
    }
}

#[cfg(kani)]
use kani_only::{call_mirror_narrow_to_u8_thrice_contracted, mirror_caller_stubbing_narrow_to_u8};
/// Exposed crate-wide, not just within this module: the modularity
/// claim this file demonstrates (a mirror is a legitimate
/// `stub_verified` target *other* mirrors can build on,
/// `check_mirror_caller_stubbing_narrow_to_u8` below) only means
/// something if other gallery modules can actually reach it --
/// `gallery::fixed_offset_descriptor_mirror` does exactly that.
#[cfg(kani)]
pub(crate) use kani_only::{mirror_narrow_to_u8, MirrorTemporalError};

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::narrow_to_u8_mirror::mirror_narrow_to_u8_contract".to_owned(),
            "gallery::narrow_to_u8_mirror::check_mirror_narrow_to_u8_contract".to_owned(),
            "amenable_kani".to_owned(),
            "confirmed PASSES, 1.6s, 139/139 checks -- the mirror's own contract (narrow_to_u8's real shape: success narrows correctly, failure means value genuinely exceeded u8::MAX), genuinely proven, cheap.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_MIRROR_NARROW_TO_U8_CONTRACT_SRC, {
        #[kani::proof_for_contract(mirror_narrow_to_u8)]
        fn check_mirror_narrow_to_u8_contract() {
            let _ = mirror_narrow_to_u8(kani::any(), "hours");
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::narrow_to_u8_mirror::call_mirror_narrow_to_u8_thrice_contracted".to_owned(),
            "gallery::narrow_to_u8_mirror::check_call_mirror_narrow_to_u8_thrice_contracted".to_owned(),
            "amenable_kani".to_owned(),
            "confirmed PASSES, 58.2s, 144/144 checks -- the exact call pattern (three narrowing calls under one trivially-contracted caller) that motivated this whole investigation's real wall, now cheap with the proxy swap applied.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CALL_MIRROR_NARROW_TO_U8_THRICE_CONTRACTED_SRC, {
        #[kani::proof_for_contract(call_mirror_narrow_to_u8_thrice_contracted)]
        fn check_call_mirror_narrow_to_u8_thrice_contracted() {
            call_mirror_narrow_to_u8_thrice_contracted();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::narrow_to_u8_mirror::mirror_caller_stubbing_narrow_to_u8".to_owned(),
            "gallery::narrow_to_u8_mirror::check_mirror_caller_stubbing_narrow_to_u8".to_owned(),
            "amenable_kani".to_owned(),
            "confirmed PASSES, 18.3s, 148/148 checks -- the modularity claim: a different function, stub_verified-ing mirror_narrow_to_u8 rather than inlining it, works. (Required adding kani::Arbitrary for MirrorTemporalError, forgotten initially -- a compile-time gap, not a proof issue.) Mirrors are legitimate stub targets other mirrors can build on.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_MIRROR_CALLER_STUBBING_NARROW_TO_U8_SRC, {
        #[kani::proof_for_contract(mirror_caller_stubbing_narrow_to_u8)]
        #[kani::stub_verified(mirror_narrow_to_u8)]
        fn check_mirror_caller_stubbing_narrow_to_u8() {
            mirror_caller_stubbing_narrow_to_u8();
        }
    }
}
